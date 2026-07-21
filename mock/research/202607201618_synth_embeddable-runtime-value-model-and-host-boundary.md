# The embeddable runtime value model and the host boundary

**Date:** 2026-07-20
**Domain:** how a runtime holds a produced value and hands it across the boundary to an embedding host. The
internal value model (moving collectors and handle tables versus never-moving arenas), the boundary mechanisms
(marshalling, by-index versus by-offset, the three channels by value-shape), ownership and lease contracts,
streaming with backpressure for bounded residency, and zero-copy with untrusted-buffer validation.
**Synthesised from:** `runtime-value-transfer-study/dotnet_clr.md`, `.../lua_luajit.md`, `.../v8_deno.md`,
`.../wasm_canonical_abi.md`, `.../zerocopy_formats.md`, with the refcounted value model in
`prior_art/05_typst.md` and the embedding boundary in `prior_art/08_terra_procmacro_comptime.md`.
(The study's own `synthesis.md` is deliberately not used here; this is an independent domain write-up over the
five raw runtime docs.)

## The single fact that organises the whole domain

The elaborate machinery every dynamic runtime carries at its host boundary exists for one reason: a
moving/compacting garbage collector relocates live objects, so the host can never hold a stable address into
the value graph. V8 relocates and "updates all handles that refer to the object with the object's new
location"; the CLR's collector relocates and any raw pointer into a heap object is invalid the instant the GC
runs unless pinned; Lua's collector is why the host addresses values by stack index, never by pointer. Every
handle table, every pin, every `luaL_ref`, every marshalling copy is downstream of relocation. The corollary
is the load-bearing design lever: **a runtime whose value store never moves inherits none of it.** A stable
offset into a never-relocated arena is inherently non-dangling while the arena lives, so the entire handle/pin/
root apparatus is absent by construction. This is the biggest structural simplification available to an
arena-based, no-GC runtime, and it is derived from first principles across four disjoint systems, not
borrowed.

## The internal value models, compared

- **.NET CLR** splits every type into reference types (on the managed heap, addressed by a mutable object
  reference the GC relocates) and value types (stored inline in their container, no independent identity).
  Boxing bridges them by allocating a fresh heap object and copying the value's bytes in; every box is a heap
  allocation plus a copy. A large nested output in idiomatic .NET is a tree of individually-allocated, movable
  heap objects, which is precisely why crossing to native code needs marshalling.
- **Lua** is a 16-byte tagged union (`Value` union plus a `tt_` tag byte); numbers/booleans/nil live inline,
  everything else is a GC-heap `GCObject` behind a pointer. **LuaJIT** NaN-boxes the same semantics into 8
  bytes (GC64: a 13-bit NaN marker, a 4-bit internal type tag, a 47-bit payload), with type dispatch as a sign
  compare. Representation differs; the no-raw-pointer invariant is identical.
- **V8** values live in a per-`Isolate` GC heap; the host holds `Local<T>` (scoped to a `HandleScope`) or
  `Persistent`/`Global` (cross-scope), both indirecting through a handle table the GC updates on relocation.
  `v8::External` wraps a host `void*` for the reverse direction.
- **Typst** (`prior_art/05`) is the interesting middle: not a GC but hand-rolled atomic refcounting, `Content`
  as a `#[repr(transparent)]` type-erased `Arc` with clone-on-write via `make_unique`. It shows that "cheap
  clone" without a collector still means refcount-sharing a heap allocation, not avoiding the heap, and that
  refcounting is the fallback when you drop the collector but keep a shared mutable value graph.

The lesson for a produced-once, immutable, arena-held value: none of these models is the template. The value
should be born flat in a never-moving arena, so it needs neither a collector (V8/Lua/CLR), nor refcounting
(Typst), nor pinning, and the host reads it in place by offset.

## The boundary is marshalling, and blittable is the ideal

The CLR names the axis precisely: crossing managed-to-native is *marshalling*, and the core distinction is
blittable versus non-blittable. Blittable types have an identical bit representation on both sides, so
marshalling them is nothing (the runtime hands over the address). Non-blittable types (managed UTF-16 `String`
versus a native `char*`, `System.Boolean` 1 byte versus Win32 `BOOL` 4 bytes) need an allocate-copy-convert
step. The design ideal reads straight off this: make the transferred value's wire form so that the managed and
native representations are identical and there is nothing to marshal. A flat, fixed-width-record, C-ABI-shaped
buffer is blittable in the CLR's own sense, and "the fastest possible marshal is no marshal."

The same encoding-match-kills-copies fact appears in V8/Deno as its opposite: strings copy on every op because
a JS string may be Latin-1 while the host wants UTF-8, so "strings in ops always require a copy (at least)."
Encoding mismatch forces the copy; encoding agreement removes it. A runtime whose value format both sides
already speak has no cross-language layout gap and no marshalling protocol, unlike WebAssembly's Canonical ABI
whose elaborate per-type lift/lower exists *only* because it bridges two languages with different in-memory
layouts.

## The by-index / by-offset discipline, and never a raw pointer

Lua's stack C API is the canonical statement: the entire host-to-VM boundary is a virtual stack of `TValue`
slots addressed by integer index (positive from bottom, negative from top), never a keepable `TValue*`. Values
move by copy-in/copy-out (`lua_push*` / `lua_to*`). The design's own reasoning: because the host names a slot,
the collector is free to move objects, run incrementally between any two calls, or intern a string, and none
of it can dangle a host reference. To hold a value across calls, `luaL_ref(L, LUA_REGISTRYINDEX)` returns an
*integer* key (a GC root), released by `luaL_unref`; the durable handle is an `int`, stable across GC
precisely because it is not a pointer. WebAssembly's Canonical ABI makes the identical choice for opaque
entities: `resource` types use `own`/`borrow` handles that are indices into a per-instance table, never raw
pointers, so they stay valid across the boundary. This is the same discipline the arena IR uses (an
`EntityRef` is an index), and it is why the arena value and the wire value can share one representation.

## The three channels by value-shape

V8/Deno's hardest-won lesson is the sharpest generalizable one: cross a value by its *shape*, on three
disjoint channels, never one uniform "return the value" call.

1. **Scalars cross by value inline.** `#[op2]` markers `#[smi]` (tagged small int), `#[bigint]`; the fast-call
   ABI passes primitives with no marshalling.
2. **Bulk bytes cross as a shared region, never re-serialized.** An `ArrayBuffer` shares a refcounted
   `BackingStore`; `new_backing_store_from_vec` *takes ownership* of host bytes without copying and frees them
   when the last referencing buffer is collected. Detach/transfer *moves* a large buffer (zeroing the source's
   length, re-wrapping the same store on the far side) instead of copying, which is the engine primitive under
   `postMessage` transferables. A zero-copy `&[u8]` view carries the sharp caveat "JS may modify the contents
   if V8 is called re-entrantly," so aliasing live VM memory is sound only while the VM is quiescent, and the
   API offers explicit `(copy)` variants for when it is not.
3. **Object graphs cross only through an explicit serializer.** `v8::ValueSerializer` writes a tag-based
   linear structured-clone stream; cycles and shared subgraphs are handled by an identity map (`id_map_`,
   emitting `kObjectReference` + a varint id on a hit) rather than structural recursion, so each distinct
   object is serialized once and back-edges are compact references.

The load-bearing correction Deno learned: the generic Rust-struct-to-JS-object mapper (`serde_v8`) was the
dominant boundary cost (the URLPattern op serialized an 8-9 key struct that JS immediately destructured and
GC'd), and the fix was structural, "stop crossing object graphs" (unfurl fields into positional args, return
primitive arrays, pass the handle through untouched), not a faster mapper. The rule: put nothing on the
object-graph channel that could cross as a scalar or a shared buffer.

## Ownership and lease contracts: the retained handle was never the problem

The rejected-in-many-designs "retained handle plus accessors" idea is rescued everywhere by pairing the handle
with an explicit ownership/release contract. The corpus gives five spellings of the same contract:

- **Arrow C Data Interface** (`zerocopy_formats`): the consumer allocates and owns the base `ArrowArray`/
  `ArrowSchema` struct; the producer owns everything pointed-to; a single `release(array)` callback is the
  entire lifetime mechanism, and it must recursively release children, free owned buffers, and null its own
  `release` pointer (a null `release` is the "already released" sentinel). Move semantics: a consumer may
  shallow-copy the struct and zero the source's `release` without calling it, transferring the single live
  responsibility.
- **WebAssembly Canonical ABI**: the callee-provided `cabi_realloc(orig_ptr, orig_size, align, new_size)`
  allocates the destination buffer, and `post-return` runs once after results are lifted to free the temporary
  buffers. This is a defined two-phase handshake (allocate, return, reclaim), the fix for the classic "return
  pointer + length, but who frees" hole that bare core-wasm left open.
- **CPython buffer protocol**: `PyObject_GetBuffer` sets a new reference to the exporter (a refcount lock
  pinning it so it cannot be freed or resized while the view is outstanding), paired with exactly one
  `PyBuffer_Release`.
- **JNI**: `GetPrimitiveArrayCritical`/`Release...` bracket a critical region where no other JNI call may run
  (the GC-locker), and local references auto-free at native-method return while a value that must outlive the
  call is promoted with `NewGlobalRef`/`DeleteGlobalRef`, the explicit cheap-scoped-versus-durable split.
- **.NET `Memory<T>`**: the lease rules (a `void`-returning method taking `Memory<T>` must not touch it after
  returning; the async analogue ends at the returned `Task`; an `IMemoryOwner<T>` holder must `Dispose` or
  transfer, never both) codify ownership/consumption/lease as a stated contract because the compiler only
  catches the stack-only `Span<T>` subset.

The synthesis: a handle crossing an API is safe when its ownership (single owner, transferable), its consumer
(one at a time), and its *lease* (the exact window it may be touched) are specified. The retained handle fails
only when the lease is unspecified; the fix is a named, mandatory release, not avoidance. The `Span<T>` versus
`Memory<T>` split donates the type-level version of the same lesson: keep "storable buffer handle" and
"borrowed access window" as distinct types because only one is expressible as a compiler-checked stack
lifetime.

## Bounded residency is streaming with backpressure, not a bigger buffer

For a value larger than a residency budget, the answer across the corpus is a pull-based writer with
backpressure, not a larger allocation.

- **.NET** `IBufferWriter<T>` is the pull-based sink: the consumer supplies space on demand
  (`GetSpan(sizeHint)`/`GetMemory` + `Advance(count)`); `Utf8JsonWriter` serializes an arbitrarily large
  nested graph by borrowing a chunk, writing tokens, advancing, and flushing, never holding the whole tree.
  `System.IO.Pipelines` adds backpressure: `FlushAsync` blocks the writer once buffered-but-unconsumed data
  crosses `PauseWriterThreshold` and resumes below `ResumeWriterThreshold`, so only an O(window) slice is
  resident regardless of total size. `ReadOnlySequence<T>` + `AdvanceTo(consumed, examined)` lets the value
  be a chain of segments the reader walks and releases, no flattening.
- **WebAssembly** `stream<T>`/`future<T>` (WASI 0.3, Feb 2026) are the same shape at the ABI level:
  completion-based `stream.read`/`stream.write` over a caller-supplied buffer with built-in flow control, the
  producer feeding a bounded consumer buffer incrementally, residency capped at the window.
- **Arrow** ships a C Stream Interface (`get_next` pulling successive `ArrowArray`s) next to the C Data
  Interface for exactly the unbounded-batches case.

The subprocess analogue is free: an OS pipe *is* this stream, and a full pipe blocking `write` is the
backpressure, the direct analogue of `stream.write` returning a blocked sentinel.

## Zero-copy in place, and the untrusted-buffer tax

`zerocopy_formats` establishes the recurring copy-free shape (covered structurally in the IR-representation
synthesis): a flat, self-describing buffer with relative-offset internal links read in place under bounds
checks, plus an ownership/release contract for the backing memory. The domain-boundary facts to carry here:
zero-copy exists only *within one address space* (in-process, the host reads the arena in place); across
disjoint memories (a subprocess pipe, two wasm components) exactly one copy is irreducible, and the design
makes that copy safe and single rather than pretending it away. And an untrusted buffer (one that crossed a
process boundary) is an attack surface: the same relative pointers that make it fast make loops, out-of-bounds
offsets, and tiny-buffer-huge-logical-structure attacks possible, so every serious system runs a bounds-checked
validation (Cap'n Proto's mandatory traversal and pointer-depth limits, rkyv's `bytecheck`) before or during
the walk. In-process, where the producer made the bytes, that pass can be skipped.

## The embedding-artifact boundary: what ships

Two prior arts frame the "runtime is embeddable" end. Lua's whole C API is shaped so the host stays in control
and the VM stays swappable; the `lua_tolstring` footgun (a pointer valid only while the string is on the
stack) is the canonical embedding bug, a deliberately-scoped borrowed peek, and a Rust host can turn that
runtime footgun into a compile-time lifetime error, which is the in-process advantage a typed driver has over
a C API. LuaJIT's FFI is the two-door insight: a safe by-index boundary and a fast pointer-direct boundary
coexist as separate, clearly-labelled doors with different contracts (the FFI wins 20x/35x on bulk numeric
data by skipping copy-in/copy-out, at the cost of GC-management, classic-API compatibility, and portability).
Terra (`prior_art/08`) is the clean statement of "the generator does not ship": Terra runs in a separate
environment with no runtime dependency on Lua, and `saveobj` emits a standalone artifact linkable into a C
executable, so the staging host is dev-time only.

## The design the domain points at, consolidated

Born-flat, index-referenced, immutable value-arena in a never-moving region (no collector, no refcount, no
pinning, none of the handle-table apparatus); scalars cross by value; bulk runs cross as a shared region whose
ownership transfers rather than copies; the structured value crosses as the arena's own bytes, read in place
by offset in-process and written verbatim to a pipe or file out-of-process (one format, both boundaries);
governed by an Arrow-style single-release lease contract where a region is lent, with a `Span`/`Memory`-style
distinction between a borrowed window and a stored handle; bounded for enormous outputs by a reserve/commit
pull sink with backpressure (the pipe is that sink for the subprocess path); and validated by a bounds-checked
traversal on the untrusted path only.

## Sources

Primary synthesised docs:
`202607201227_runtime-value-transfer-study/dotnet_clr.md`, `.../lua_luajit.md`, `.../v8_deno.md`,
`.../wasm_canonical_abi.md`, `.../zerocopy_formats.md`;
`202607190650_mockspace-procedural-docs-arc/prior_art/05_typst.md` (refcounted value model),
`.../08_terra_procmacro_comptime.md` (embedding-artifact boundary).

Key underlying citations carried by those docs: .NET type-marshalling, `GCHandle`, `Span`/`Memory` usage
guidelines, `System.Buffers`, and `System.IO.Pipelines` docs; the Lua 5.4 reference manual and LuaJIT FFI
docs; the V8 embedder guide, rusty_v8/`deno_core` `#[op2]`, `serde_v8`, and `ValueSerializer`; the WebAssembly
core spec and Component Model Canonical ABI (`realloc`/`post-return`, `stream`/`future`); the Apache Arrow C
Data/Stream Interface, CPython buffer protocol (PEP 3118), JNI functions spec, Cap'n Proto, FlatBuffers, and
rkyv.
