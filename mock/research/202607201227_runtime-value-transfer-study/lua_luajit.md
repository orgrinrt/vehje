# Lua / LuaJIT: value transfer across the embed boundary

## One-line summary

Lua's answer to "how does a produced value cross to the host" is a hard refusal to let it cross at all as memory: every value stays VM-owned and GC-managed, and the host manipulates it only through a virtual stack addressed by integer index (or by integer registry reference for long-lived holds), never by pointer, so the collector stays free to move, free, or intern anything between any two API calls. LuaJIT keeps that exact boundary but compresses the value representation to a single NaN-boxed 64-bit word, and adds a second, separate door (the FFI) that gives up the safety and the reference-Lua portability of the stack API in exchange for pointer-direct, JIT-inlinable access to C memory the VM treats as opaque.

## The internal value model (TValue, NaN-boxing, where values live)

Reference Lua's value is a tagged union. From `lobject.h` (Lua 5.4), a `Value` is a union over the representations a slot can hold, and a `TValue` pairs it with a one-byte type tag:

```c
typedef union Value {
  struct GCObject *gc;    /* collectable objects */
  void *p;               /* light userdata */
  lua_CFunction f;       /* light C functions */
  lua_Integer i;         /* integer numbers */
  lua_Number n;          /* float numbers */
  lu_byte ub;
} Value;

#define TValuefields  Value value_; lu_byte tt_
typedef struct TValue { TValuefields; } TValue;
```

The tag byte `tt_` uses bits 0 to 3 for the base type (a `LUA_T*` constant), bits 4 to 5 for variant bits (integer vs float, short vs long string, light C function vs Lua closure), and bit 6 for the "is collectable" flag. So a `TValue` is 16 bytes on a 64-bit target (8-byte union + tag + padding). The two fundamentally different residencies are visible right in the union: `i` and `n` are stored *inline* (the number is the value, no heap object), whereas `gc` is a pointer to a separately allocated, GC-tracked `GCObject` (strings, tables, closures, full userdata, threads). Every collectable object begins with a common header (`CommonHeader`: a `next` link, the `tt` type tag, and a `marked` GC-color byte), so the collector can walk them uniformly.

Where values live: numbers and booleans and nil live inline inside whatever `TValue` slot holds them (a stack slot, a table array part, an upvalue). Everything else lives on the VM-owned GC heap, reachable only through `gc` pointers that the collector owns and may relocate or free. There is no arena the host can point into and trust.

LuaJIT keeps the semantics but changes the representation to save space and branch cost. Its `TValue` is an 8-byte union, and it NaN-boxes: it treats a double as the base type and packs every non-double value into the payload bits of an IEEE-754 NaN. IEEE doubles have a large space of bit patterns that all mean "NaN"; LuaJIT steals that space for tagged non-numbers. Two layouts exist:

- **32-bit / non-GC64 mode.** The 64-bit word splits into a most-significant word carrying the internal type tag `it` and a least-significant word carrying a 32-bit GC reference, an `int32_t`, or the low word of a double. Real doubles have MSW `< 0xfff80000`; anything with MSW `>= 0xfff80000` is a tagged non-double, and the tags are arranged as negative numbers so a single sign-extended compare classifies the value.
- **GC64 mode (64-bit pointers).** Bits 63 to 51 are the NaN marker (`0x1fff8`, 13 bits), bits 50 to 47 are a 4-bit internal type tag (16 types), and bits 46 to 0 are a 47-bit payload: a GC pointer masked to its low 47 bits (`LJ_GCVMASK`), a zero-extended 32-bit integer, all-ones for primitives, or a segmented light-userdata pointer. `itype()` is just `(uint32_t)(o->it64 >> 47)`.

The tag ordering is chosen so range checks are cheap: `LJ_TNIL` (`~0u`) through `LJ_TNUMX` (`~13u`), with boundary constants (`LJ_TISNUM`, `LJ_TISTRUECOND`, `LJ_TISPRI`) letting "is it a number", "is it truthy", "is it a primitive" each become one comparison. GC-tracked references still carry a write-barrier contract (`setgcref*` must be paired with a barrier so a black object never points to a white one); non-GC raw pointers use a separate `MRef` with no barrier. The point for us: LuaJIT halved the slot from 16 to 8 bytes and turned type dispatch into sign tricks, but the residency story is identical to reference Lua. Doubles and small integers are inline; strings, tables, closures, userdata are GC-heap objects behind a reference the collector owns.

The load-bearing invariant across both: **the host never holds a raw pointer into the live value graph.** It holds stack slots and integer keys, and the VM hands out short-lived borrowed pointers only under a narrow, documented lifetime.

## The stack C API as the whole boundary (why by-index, not by-pointer)

The entire host-to-VM boundary in reference Lua is a virtual stack of `TValue` slots that the host manipulates by index. There is no other door in the standard API. You never receive a `TValue*` you can keep; you receive a position.

Indexing: a positive index is an absolute position from the bottom (1 is the first slot), a negative index is an offset from the top (-1 is the top, -n is the nth from the top). Stack shape is managed with `lua_gettop(L)` (current height), `lua_settop(L, idx)` (grow with nils or truncate to an exact height), `lua_pushvalue(L, idx)` (duplicate a slot onto the top), and `lua_absindex(L, idx)` (normalize a relative index). Pseudo-indices name positions "accessible to C code but not on the stack", used for the registry (`LUA_REGISTRYINDEX`) and C-closure upvalues.

Values move across the boundary by copy-in / copy-out through this stack:

- **Host to VM (push):** `lua_pushnumber`, `lua_pushinteger`, `lua_pushboolean`, `lua_pushnil`, `lua_pushlstring(L, s, len)` and `lua_pushstring(L, s)` (which *copy* the bytes into an interned VM string), `lua_pushlightuserdata(L, p)`, `lua_pushcclosure` / `lua_pushcfunction`.
- **VM to host (read):** `lua_tonumberx`, `lua_tointegerx`, `lua_toboolean`, `lua_tolstring(L, idx, &len)`, `lua_touserdata`, `lua_topointer`. Each reads the slot at `idx` and returns a C-representable scalar or a borrowed pointer.

Why by-index and not by-pointer, stated plainly by the design: the stack indirection buys the collector total freedom. Because the host only ever names a slot, the GC is free to move objects, run incremental collection between any two API calls, trigger a metamethod that reallocates, or intern/dedup a string, and none of it can dangle a host reference, because the host holds no reference into the graph. A slot index stays valid across GC; a pointer would not. This is the whole reason the API is shaped the way it is. It trades a small per-access indirection (index into the stack array, then read the `TValue`) for the guarantee that host code and the collector can never race over ownership of a live object. The stack is also the argument-and-return protocol for calls: `lua_call` / `lua_pcall` consume the function and arguments pushed onto the stack and leave the results on the stack, again by position, never by handing back a pointer.

The cost side is real and worth naming for vehje: this is a copy-in / copy-out marshalling boundary. Pushing a string copies bytes into the VM; reading a number copies a scalar out. There is no zero-copy pass of a large aggregate through the standard stack API. A big nested value does not "cross"; it is walked slot by slot, each leaf copied at the moment the host reads it.

## Holding values across calls (registry, refs, userdata)

The stack is transient: it is unwound when a C function returns, and slots are reused. To hold a VM value *across* calls (a callback the host keeps, a config table the host reads later), Lua gives two mechanisms, both of which preserve the no-raw-pointer invariant by handing back an integer, not an address.

- **The registry** is a predefined table at pseudo-index `LUA_REGISTRYINDEX` that any C code can store values into. It is a normal Lua table, so anything in it is a GC root and stays alive. Reserved integer keys `LUA_RIDX_MAINTHREAD` and `LUA_RIDX_GLOBALS` live there.
- **References**: `int luaL_ref(L, LUA_REGISTRYINDEX)` pops the top value, stores it in the registry, and returns a fresh integer key (the "reference") that names it. `void luaL_unref(L, LUA_REGISTRYINDEX, ref)` releases it (returns the integer to the free list and drops the GC root). To use a held value you push it back onto the stack with `lua_rawgeti(L, LUA_REGISTRYINDEX, ref)`. The durable handle the host holds is an `int`. It is stable across GC and across calls precisely because it is not a pointer; the VM resolves it to a live slot each time.

**Userdata** is the mechanism for the reverse direction and for opaque host-owned objects living *inside* the VM:

- **Full userdata** (`void *lua_newuserdatauv(L, size_t sz, int nuvalue)`) is a GC-managed block of `sz` bytes that the VM allocates and owns; the host gets a pointer to the block valid while the userdata is alive on the stack or reachable. It can carry a metatable, and if that metatable has a `__gc` metamethod, Lua calls it at finalization. This is the standard way to give the VM an opaque host handle: allocate a full userdata sized to hold your host-side struct (or a pointer/id into host storage), attach a metatable with `__gc` that tears down the host resource, and the VM's own collector now drives the host object's lifetime.
- **Light userdata** (`void lua_pushlightuserdata(L, void *p)`) is just a raw C pointer stored inline in a `TValue` (the `p` field of the `Value` union). It is not GC-tracked, has no metatable of its own, and has no finalizer. It is the escape hatch for passing a host pointer *through* the VM as an opaque token: the VM stores it, compares it by identity, and hands it back untouched, but never dereferences it and never owns it. Lifetime is entirely the host's problem.

The asymmetry is the lesson: VM values reach the host as indices or integer refs (VM owns memory, host owns nothing), and host objects reach the VM as full userdata (VM owns the block, `__gc` bridges lifetime) or light userdata (host owns everything, VM treats it as an opaque token). At no point does either side hold a durable raw pointer into the other's live, collectable graph.

## Large/bulk data and the string-pointer lifetime contract

Strings are the closest Lua gets to a bulk-data transfer, and they show both the copy cost and the sharpest footgun.

Lua strings are immutable and (for short strings) interned. Reference Lua interns short strings (default cutoff `LUAI_MAXSHORTLEN`, 40 bytes) into a global string table so identical short strings share one allocation and compare by pointer. Long strings are *not* eagerly interned; a long string is hashed lazily and only interned when used as a table key. Pushing bytes in with `lua_pushlstring(L, s, len)` computes a hash, looks up the intern table, and either reuses an existing `TString` or allocates a new one and *copies the bytes in*. So a large blob crossing host-to-VM as a string is a full copy into a VM-owned, immutable object; you cannot hand the VM a borrowed buffer as a string and expect zero copy.

Reading a string out is where the lifetime contract bites. `const char *lua_tolstring(L, idx, &len)` returns a pointer to the VM's internal bytes, and the manual's guarantee is exact and narrow: *the pointer is valid only while the string value at that index is not removed from the stack.* Outside that window the collector is free to invalidate it. Concretely: read the pointer, and if you then pop the slot, run any API call that can trigger GC or a metamethod, or let the string become unreachable, the pointer may dangle. The idiomatic safe pattern is copy-out immediately (memcpy the `len` bytes into host memory while the slot is still on the stack) or keep the string pinned on the stack / in the registry for as long as you need the pointer. This is the canonical embedding footgun, and it exists *because* of the no-raw-pointer rule: `lua_tolstring` is a deliberately scoped, borrowed peek into VM memory, not a transfer of ownership.

For genuinely large or binary bulk buffers, the mature-embedding answer is usually *not* to route them through the string type at all, but through userdata: a full userdata is a VM-owned block you can `memcpy` into and address directly for its lifetime, and light userdata lets you pass a raw host pointer to an off-heap `malloc`/`mmap` buffer that the VM never copies and never touches. That is the "move a big buffer without the VM copying it" path: the bytes live in host memory, the VM holds only an opaque pointer token. Reference Lua has no zero-copy transfer of a bulk buffer *as a first-class value*; you either copy it into a VM object or you keep it host-side and pass a token.

LuaJIT's FFI is the exception that proves the rule. `ffi.new` allocates a cdata buffer, `ffi.cast` reinterprets pointers, and cdata can wrap `malloc`/`mmap` memory directly; the host and JIT-compiled Lua both address that memory by pointer with no stack marshalling. The documented benchmark (an image processing loop) shows 20x speed and 35x memory improvement over pushing the same data through Lua values, precisely because the FFI path skips the copy-in/copy-out and the per-element boxing. But that memory, when it comes from C-land allocators, is not GC-tracked, is not accessible from the classic Lua/C stack API ("there's no support for creating/accessing cdata objects in the old Lua/C API"), and is LuaJIT-only. The FFI buys pointer-direct bulk access by giving up exactly the three things the stack API existed to provide: automatic lifetime, cross-API portability, and safety.

## Hard-won lessons (the durable principles and the footguns)

1. **Never let the host hold a VM pointer into the live graph; address everything by stack index or integer reference.** This is the single durable design decision, and everything else follows from it. It is what makes an incremental, moving-friendly, metamethod-reentrant collector safe to combine with arbitrary host C code. The moment you hand out a durable raw pointer, you have coupled the host to the collector's internal layout and the design collapses.

2. **Separate the transient boundary (stack, unwound per call) from the durable hold (registry / `luaL_ref`, an integer key).** Values that outlive a call need an explicit, named place to live and an explicit release (`luaL_unref`). Implicit "the pointer stays good" is the bug; explicit "this integer names a GC root until you unref it" is the fix.

3. **The string-pointer lifetime footgun (`lua_tolstring` valid only while on the stack) is the recurring embedding bug.** It is a direct consequence of principle 1: any borrowed peek into VM memory must be either copied out immediately or pinned. A transfer design should make the borrow window explicit and hard to misuse, not leave it to a sentence in the manual.

4. **The stack API's copy-in/copy-out has a real, sometimes dominant cost.** Boxing every leaf into a `TValue` and marshalling scalars one at a time is why LuaJIT's FFI exists and why it wins 20x on bulk numeric data. Safety-by-copy is the right default, but it does not scale to large aggregates, and a mature system needs a second, opt-in path for bulk data.

5. **LuaJIT's two changes are instructive and orthogonal.** NaN-boxing (16 bytes to 8, dispatch by sign tricks) is a pure representation win that changes nothing about the boundary contract. The FFI is a deliberate second door that trades the stack API's safety/portability/GC-management for pointer-direct, JIT-inlinable, zero-marshalling access, and it is explicitly incompatible with the classic API. The lesson is that the safe by-index boundary and the fast by-pointer boundary can coexist as *separate, clearly-labelled doors* with different contracts, rather than one door trying to be both.

6. **Userdata is the clean pattern for opaque cross-ownership.** Full userdata + `__gc` gives the VM's collector authority over a host object's teardown; light userdata passes a host pointer the VM treats as an inert token. Both keep the no-shared-mutable-graph invariant intact.

## Mapping to vehje (what transfers, what does not, with reasoning)

vehje's problem is adjacent but not identical: a small runtime interprets a residual and produces one possibly-enormous nested output value that must cross to a host, either in-process over the 3-fn C ABI or out-of-process over a pipe/file. There is no long-lived interactive VM the host keeps calling into; there is (in the common case) one big result. That difference reshapes which Lua lessons carry.

**Transfers well:**

- **The core invariant: the host never holds a raw pointer into the runtime's live value graph.** This is exactly vehje's rejection of the naive "retained handle + accessors" design's ownership hazard, and it is the right north star. Whatever vehje builds, the host should address the output by *position/index into a stable serialized form*, not by chasing runtime-owned pointers. vehje's residual already thinks in indices (the runtime is a dumb index evaluator; `Var` is pre-resolved to indices, child pools are index arrays); extending "everything is an index" from the residual to the *output* is the same discipline Lua applies to the stack. An index into a flat output arena is vehje's `lua_gettop`-addressed slot: stable across whatever the runtime does internally, dereferenced only by the runtime's own accessor.

- **The transient-boundary vs durable-hold split, reframed as streaming-cursor vs pinned-region.** Lua's "stack slot is valid only during the call, use a ref to hold longer" maps onto vehje's need for bounded residency: the runtime can expose a *cursor* over the output (a bounded window, like a morsel) that is valid only until the host advances it, plus an explicit "pin this subtree" operation for the rare case the host needs a stable hold. The default is the cheap transient window; the pin is the explicit, released-by-the-host durable hold. This is `luaL_ref`/`luaL_unref` in structural clothing and directly answers "bounded memory residency even for enormous outputs": the runtime keeps resident only the pinned regions plus the current window.

- **The explicit, narrow borrow-lifetime contract (the `lua_tolstring` lesson, applied as a warning).** Any pointer vehje hands into runtime memory (a `&[u8]` at a leaf, a slice of the output arena) must have a lifetime that is spelled out and enforced, ideally in the type system for the in-proc Rust driver rather than left as a doc sentence. vehje has a real advantage Lua lacks: Rust lifetimes can make the borrow window a compile error to outlive, turning Lua's runtime footgun into a static guarantee for the in-proc path.

- **The two-door insight: a safe default boundary and a fast bulk boundary, separately labelled.** vehje should not try to make one mechanism serve both "read this small scalar leaf" and "move this 500MB blob leaf". Lua's split (copy-through-stack by default, FFI/userdata pointer-direct for bulk) says: let the common nested-struct traversal be the safe indexed cursor, and give bulk leaves (large strings, byte arrays, arrays of fixed-width numbers) a zero-copy path where the host maps the runtime's flat region (or a shared memory region / mmap'd file) directly. The residual's tier-0 format is already a flat arena with a self-contained string blob; the output can be the same shape, and a flat arena is directly mmap-transferable across the subprocess boundary with zero copy, which is strictly better than anything Lua can do across a process boundary.

**Does not transfer (or transfers only with heavy adaptation):**

- **The interned, immutable string model.** Lua interns short strings into a global table because it is an interactive VM where the same identifiers recur constantly and pointer-equality of strings pays off. vehje produces an output once; interning a produced blob buys nothing and the copy-into-intern-table cost is pure waste. vehje's output strings should be spans into the flat blob (offset + length), not interned VM objects. The residual already does this (self-contained string blob); the output should too. Keep the immutability (an output value is read-only once produced, which simplifies the borrow contract enormously) and drop the interning.

- **The GC entirely.** Lua's whole by-index boundary exists to make a *moving, incremental garbage collector* safe against host C code. vehje is `no_std`, no-alloc, arena-based; there is no collector, nothing moves, values live in a caller-provided bump region. Much of the *machinery* Lua needs (registry as GC root, write barriers, `__gc` finalizers, "GC may run between any two calls") is answering a problem vehje does not have. vehje gets the invariant's *benefit* (host can't dangle) almost for free from the arena model: an offset into a stable arena is inherently non-dangling as long as the arena lives. So adopt Lua's *contract* (address by index, borrow narrowly) but not Lua's *apparatus* (GC roots, barriers, finalizers). The reason Lua's boundary is so ceremonious is the collector; without a collector, vehje's version is much lighter.

- **The per-leaf copy-out marshalling as the default.** Lua copies each scalar out on read because its values are boxed `TValue`s scattered on a GC heap, so there is no contiguous form to hand over. vehje's output can be *born* as a contiguous, index-addressed arena (the same discipline as the residual), which means the host reads leaves in place with no per-leaf copy at all. This is the point where vehje should beat Lua rather than imitate it: Lua marshals because it must; vehje can lay the output out flat so the traversal is pointer-arithmetic over a shared region. The subprocess path then transfers the whole arena as one mmap/pipe blob (bounded by writing it out incrementally if it exceeds a residency cap), and the in-proc path hands back a base pointer + length the host indexes into under a Rust-lifetime-enforced borrow.

- **Full userdata + `__gc` as a lifetime bridge.** This solves "a host object living inside the VM with VM-driven teardown", which is the opposite direction from vehje's problem (a runtime output flowing *out*). vehje's boundary is largely one-directional (produce, hand out, done), so the bidirectional-ownership machinery is not needed for the core value-transfer. It may become relevant only if vehje later lets host callbacks inject host objects into a running evaluation, at which point the light-userdata-as-opaque-token pattern (pass a host pointer the runtime never dereferences) is the clean minimal answer.

**Net for the design.** Lua endorses vehje's instinct to reject both naive designs: the "out-buffer" is Lua's copy-everything-out taken to its unbounded worst case, and the "retained handle + accessors" is exactly the raw-pointer-into-the-live-graph hazard Lua's entire API exists to forbid. The synthesis Lua points at is a third shape: the output is a *flat, immutable, index-addressed arena* (same family as vehje's residual), the host addresses it by offset/index (never by runtime-owned pointer), reads leaves in place under a narrow explicit borrow (Rust-lifetime-enforced in-proc, mmap-region-scoped cross-proc), holds bounded residency via a transient window plus explicit pins (the registry/ref split, structurally), and gets a genuine zero-copy bulk path for large leaves and for the whole cross-process transfer that Lua's stack API cannot offer but its FFI shows is worth having as a separate, clearly-contracted door. Keep Lua's contract; drop Lua's collector-driven apparatus; exploit the arena to skip the marshalling copy Lua is forced into.

## Sources (links)

- [Lua 5.4 Reference Manual](https://www.lua.org/manual/5.4/manual.html) (C API: stack indices, `lua_gettop`/`lua_settop`/`lua_pushvalue`/`lua_absindex`, pseudo-indices, `lua_push*`/`lua_to*`, `lua_tolstring` lifetime guarantee, registry + `LUA_REGISTRYINDEX`, `luaL_ref`/`luaL_unref`, full vs light userdata, `lua_newuserdatauv`, `lua_pushlightuserdata`, `__gc`)
- [Lua 5.4 source: lobject.h](https://www.lua.org/source/5.4/lobject.h.html) (the `Value` union, `TValue`, `TValuefields`, tag byte layout)
- [Value & TValue - Notes on the Implementation of Lua 5.3](https://poga.github.io/lua53-notes/value_tvalue.html) (tagged-union tag-bit breakdown)
- [String - Notes on the Implementation of Lua 5.3](https://poga.github.io/lua53-notes/string.html) (short vs long strings, interning, `LUAI_MAXSHORTLEN`)
- [LuaJIT Object System and Type Representation - DeepWiki](https://deepwiki.com/LuaJIT/LuaJIT/5.1-garbage-collection) (NaN-boxing `TValue`, GC64 vs 32-bit layouts, itype tags, GCRef/MRef, write barriers, GCHeader)
- [The LuaJIT GC64 Mode - OpenResty Blog](https://blog.openresty.com/en/luajit-gc64-mode/) (GC64 bit layout: 13-bit NaN marker, 4-bit tag, 47-bit payload)
- [LuaJIT FFI](https://luajit.org/ext_ffi.html) and [FFI Tutorial](https://luajit.org/ext_ffi_tutorial.html) (`ffi.cdef`/`ffi.new`/`ffi.cast`/cdata, JIT-inlined C calls, the 20x/35x image benchmark)
- [ffi.* API Functions](https://luajit.org/ext_ffi_api.html) (cdata construction and GC vs non-GC memory ownership)
- [Foreign Function Interface - DeepWiki](https://deepwiki.com/openresty/luajit2/4-foreign-function-interface) (cdata not accessible from the classic Lua/C stack API; GC tracking only for `ffi.new` memory, not `malloc`/`mmap`)
- [LuaJIT Source Code Analysis (Part 2): Data Type](https://medium.com/@eclipseflowernju/luajit-source-code-analysis-part-2-data-type-59b501d59e7f) (64-bit `TValue` union, nil/false/true as `~0u`/`~1u`/`~2u`)
