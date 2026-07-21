# V8 / Deno: value transfer across the embed boundary

## One-line summary

V8 never lets the host hold a raw pointer to a runtime value (a moving GC forbids it), so
every value crosses through one of three disjoint channels: scalars cross by value inline in
the call, bulk bytes cross as a shared `BackingStore` (owned refcount, copy-free, moved not
copied via detach), and whole object graphs cross only through an explicit tag-based
serializer (`ValueSerializer`, the structured-clone format) that walks the graph once with an
identity map for cycles. The hard-won Deno lesson on top of that: a generic
Rust-struct-to-JS-object mapper (`serde_v8`) was the dominant cost, and the fix was to stop
materialising object graphs across the boundary at all (unfurl fields into positional args,
return primitive arrays, pass `v8::Local` through untouched) rather than to make the mapper
faster.

## The internal value model (Local/Persistent handles, HandleScope, Isolate)

V8's runtime values live in a per-`Isolate` garbage-collected heap. An `Isolate` "is a VM
instance with its own heap" ([v8.dev/docs/embed](https://v8.dev/docs/embed)); values from one
isolate must never be used in another. The heap uses a **moving/compacting** collector: "during
the garbage collection process the garbage collector often moves objects to different locations
in the heap" (V8 embedder wiki). This single fact dictates the entire boundary design.

Because objects move, the host is never handed a raw object address. It is handed a **handle**,
which is an indirection: a `Local<T>` is effectively a pointer to a slot, and the slot holds the
current object address. The GC, when it relocates an object, "also updates all handles that
refer to the object with the object's new location"
([v8.dev/docs/embed](https://v8.dev/docs/embed)). The reverberate.org analysis states the
principle plainly: handles provide "a layer of indirection that lets V8 move objects around in
memory without breaking references from C++," and C++ code "cannot safely hold direct pointers
to garbage-collected objects"
([blog.reverberate.org](https://blog.reverberate.org/2016/10/17/native-extensions-memory-management-part2-javascript-v8.html)).
A raw pointer read across a GC point is a dangling pointer. The handle table is not an
optimisation; it is the correctness mechanism under a moving heap.

Handles come in two lifetime classes:

- `Local<T>` (`v8::Local<SomeType>`): a temporary handle "held on a stack and deleted when the
  appropriate destructor is called." Its lifetime is bounded by an enclosing `HandleScope`,
  which "can be thought of as a container for any number of handles"; when the scope's
  destructor runs, every `Local` created inside it is released
  ([v8.dev/docs/embed](https://v8.dev/docs/embed)). Returning a `Local` from a function that
  opens its own scope requires `EscapableHandleScope::Escape()`, which copies the handle up into
  the caller's scope before the inner scope tears down. `Local` is the default currency of the
  API; almost every V8 call takes and returns `Local`s.
- `Persistent<T>` / `Global<T>` (and `UniquePersistent<T>` in modern headers): a handle that
  "remains valid until it is explicitly disposed" and is not tied to a `HandleScope`. It is used
  to hold a value alive across scopes, across turns of the event loop, or inside a host struct.
  A persistent handle "contains a reference to a storage cell within the v8 engine which holds
  an object value and which is updated by the garbage collector whenever the object is moved"
  (Persistent class reference). It can be made weak via `PersistentBase::SetWeak` to receive a
  GC callback when the object would otherwise die, which is how the host attaches finalizers.

A `Context` "is an execution environment that allows separate, unrelated, JavaScript code to
run in a single instance of V8" ([v8.dev/docs/embed](https://v8.dev/docs/embed)); it holds the
global object and builtins. Values are always serialized or interpreted relative to a
`Local<Context>`.

For the reverse direction (wrap a host pointer so JS can carry it), V8 offers `v8::External`,
"simply a wrapper around a `void*`," and internal fields on objects
(`SetInternalField`/`GetInternalField`). An `External` lets a runtime hand the host a token that
JS holds opaquely and passes back later; the host dereferences it. This is the mechanism behind
the "retained handle" pattern (an opaque host-side value referenced from script), and it is worth
noting V8 supports it directly rather than exposing raw runtime addresses.

## The Rust<->V8 boundary (ops, serde_v8, the fast path)

Deno is the load-bearing data point here: a Rust host (`deno_core`) embedding the C++ V8 through
the `rusty_v8` (`v8` crate) FFI bindings. The Rust-to-JS call surface is the **ops** layer. An op
is a Rust function exposed to JS; arguments and return values cross at that call.

Historically every non-buffer value crossed through **`serde_v8`**, a `serde` data format that
bijects Rust values and `v8::Value`s: it "aims to provide an expressive but ~maximally efficient
encoding layer to biject rust & v8/js values" and is "used to encode/decode all non-buffer
values" ([lib.rs/crates/serde_v8](https://lib.rs/crates/serde_v8)). It carries a passthrough
escape hatch, `serde_v8::Value`, "which will passthrough the original v8 value untouched when
encoding/decoding" ([docs.rs/serde_v8](https://docs.rs/serde_v8)) so a raw handle can cross
without materialisation.

The current surface is the **`#[op2]`** macro
([docs.rs/deno_core op2](https://docs.rs/deno_core/latest/deno_core/attr.op2.html)), which
replaced the older generic path and makes the crossing explicit and typed per argument. Its
attribute markers name exactly how each value crosses:

- `#[smi]`: a small integer passed as a V8 SMI (tagged inline, no heap object, no allocation).
- `#[bigint]`: `i64`/`u64`/`usize` as a JS `BigInt`.
- `#[string]`: UTF-8 `String`/`&str`/`Cow<str>`. Strings "always require a copy (at least) to
  ensure that we are not incorrectly passing Latin-1 data to methods that expect a UTF-8 string,"
  and the fast path is available "only if string is Latin-1."
- `#[arraybuffer]` / `#[buffer]`: byte buffers, covered in the next section (the zero-copy path).
- `#[serde]`: the legacy JSON-shaped conversion, explicitly "marked as potentially slow."
- bare `v8::Local` arguments/returns: pass through "without copying where possible."

By default `#[op2]` uses `FromV8Scopeless` to avoid even opening a V8 scope for the conversion,
adding `#[scoped]` only when scope access is needed. This is a deliberate stripping of overhead
that the old serde path always paid.

**The fast-call path** is the second key mechanism. An op annotated `#[op2(fast)]` becomes
eligible for V8's fastcall ABI: when V8's optimising JIT has specialised the call site, it
"will switch the implementation over if the parameters are compatible," calling a C ABI that
skips the general argument-marshalling machinery entirely. Fastcall supports "primitive types,
`v8::Local` references, and most buffer types." A slow op can name an alternate fast body with
`#[op2(fast(op_XYZ))]`. The design intent is that the hot, monomorphic, primitive-and-buffer
ops never touch the generic conversion path.

## Zero-copy bulk data (ArrayBuffer/BackingStore, transferables/detach)

Bulk bytes do not cross as serialized JS values. They cross as an `ArrayBuffer` sharing a
**`BackingStore`**, which is the copy-free channel and the model most directly relevant to
vehje.

A `BackingStore` is the raw memory behind an `ArrayBuffer`, with its lifetime managed by a
shared refcount. `ArrayBuffer::GetBackingStore()` "returns a shared pointer to the backing store
... This pointer coordinates the lifetime management of the internal storage with any live
ArrayBuffers on the heap, even across isolates. The embedder should not attempt to manage
lifetime of the storage through other means"
([v8.github.io ArrayBuffer](https://v8.github.io/api/head/classv8_1_1ArrayBuffer.html)). The
refcounted `SharedRef<BackingStore>` is the ownership token.

The rusty_v8 API lets a Rust host construct a backing store *from host-owned bytes without
copying* ([docs.rs/v8 ArrayBuffer](https://docs.rs/v8/latest/v8/struct.ArrayBuffer.html)):

```rust
pub fn new_backing_store_from_vec(data: Vec<u8>) -> UniqueRef<BackingStore>
pub fn new_backing_store_from_boxed_slice(data: Box<[u8]>) -> UniqueRef<BackingStore>
pub fn new_backing_store_from_bytes<T: Rawable>(bytes: T) -> UniqueRef<BackingStore>
// then:
pub fn with_backing_store(scope, backing_store: &SharedRef<BackingStore>) -> Local<ArrayBuffer>
```

Ownership semantics are explicit: such a call "returns a new standalone BackingStore that takes
over the ownership of the given buffer. The destructor of the BackingStore frees owned buffer
memory." So the host `Vec<u8>` is *moved into* V8; V8 frees it when the last `ArrayBuffer`
referencing that store is collected. No byte is copied on the way in. The reverse query surface
is `data() -> Option<NonNull<c_void>>`, `byte_length()`, `get_backing_store()`.

**Detach / transfer** is the mechanism for *moving* (not copying) a large buffer across a
sub-boundary (e.g. host to worker). `ArrayBuffer::Detach(key)` "sets the byte length of the
buffer and all typed arrays to zero, preventing JavaScript from ever accessing underlying
backing store" ([v8.github.io](https://v8.github.io/api/head/classv8_1_1ArrayBuffer.html)); the
store itself survives (its refcount is held elsewhere) and is re-wrapped on the far side. This
is the engine-level primitive under JS `postMessage` transferables and
`ArrayBuffer.prototype.transfer()`: "when an ArrayBuffer is transferred ... the original
ArrayBuffer becomes detached," enabling "zero-copy semantics by moving ownership rather than
duplicating data" ([MDN ArrayBuffer.transfer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer)).
Deno's `#[op2]` exposes this as `#[buffer(detach)]` yielding a `JsBuffer` "for safe detachment."

The guarantee attached to the zero-copy slice path (`&[u8]`/`&mut [u8]` via `#[arraybuffer]`)
carries a sharp warning: "JS may modify the contents of the slice if V8 is called re-entrantly"
([docs.rs/deno_core op2](https://docs.rs/deno_core/latest/deno_core/attr.op2.html)). Copy-free
aliasing of live VM memory is only sound while the VM is not re-entered. Where that cannot be
guaranteed, the API offers explicit `#[arraybuffer(copy)]` / `#[buffer(copy)]` variants. This is
the exact aliasing-vs-copy tradeoff, made a per-call type decision rather than a global default.

The load-bearing principle: **bulk data crosses as typed arrays over a shared backing store, not
as serialized JS objects,** because a megabyte of bytes serialized field-by-field into a JS
object graph would allocate a megabyte of GC-managed cells and copy every byte, where the
backing store shares the same bytes at a refcount's cost.

## Serializing a whole value graph (ValueSerializer / structured clone)

When an *arbitrary object graph* (not a scalar, not a flat byte buffer) must genuinely cross a
boundary that cannot share memory (`postMessage` between isolates, persisting to disk,
`structuredClone`), V8 uses `v8::ValueSerializer`, "value serialization compatible with the HTML
structured clone algorithm"
([v8.github.io ValueSerializer](https://v8.github.io/api/head/classv8_1_1ValueSerializer.html)).

The format is **tag-based and linear**: every value is prefixed with a single-byte
`SerializationTag`; the deserializer reads a tag, learns the type, and reads the payload. It
handles Map, Set, Date, RegExp, ArrayBuffer, TypedArray, and Error-with-stack, and "crucially,
circular references." The format "is backward-compatible (i.e. safe to store to disk)."

The C++ surface is small:

```cpp
void WriteHeader();                                  // writes the format version
Maybe<bool> WriteValue(Local<Context>, Local<Value>);// serializes one value graph
std::pair<uint8_t*, size_t> Release();               // "Ownership of the buffer is transferred to the caller"
void TransferArrayBuffer(uint32_t id, Local<ArrayBuffer>);
```

`Release()` hands the caller the owned byte buffer; the serializer is then unusable
([v8.github.io](https://v8.github.io/api/head/classv8_1_1ValueSerializer.html)). The rusty_v8
Rust binding wraps this as `ValueSerializer::new(scope, Box<impl ValueSerializerImpl>)`,
`write_header()`, `write_value(context, value) -> Option<bool>`, `release() -> Vec<u8>`, plus
`transfer_array_buffer()` and low-level `write_uint32/uint64/double/raw_bytes`
([docs.rs/v8 ValueSerializer](https://docs.rs/v8/latest/v8/struct.ValueSerializer.html)).

**Cycles and shared subgraphs** are handled by an identity map, not by structural recursion.
Reading V8's `value-serializer.cc`
([github.com/v8/v8](https://github.com/v8/v8/blob/main/src/objects/value-serializer.cc)): the
serializer holds an `id_map_` (an `IdentityMap` from object to serial id) and a `next_id_`
counter. When `WriteJSReceiver()` hits an object it calls `id_map_.FindOrInsert(receiver)`; on a
*hit* it does not recurse, it emits `SerializationTag::kObjectReference` plus the varint id. So
each distinct object is serialized once; every later occurrence (including a back-edge that forms
a cycle) is a compact back-reference. This is the mechanism that makes the walk terminate on
cyclic graphs and deduplicate shared subgraphs.

**Transfer vs clone** for buffers is a separate map. `array_buffer_transfer_map_` records buffers
marked for out-of-band transfer; a transferred buffer "bypasses deep copying" (its bytes move via
the detached backing store, coordinated by matching `ValueDeserializer::TransferArrayBuffer`
ids), while an untransferred buffer is *cloned* by serializing its raw bytes under a `kArrayBuffer`
tag. Shared array buffers go through the delegate's `GetSharedArrayBufferId`. So even inside the
serializer, bulk bytes get the move-not-copy treatment when the caller opts in; only genuinely
cloned buffers pay the byte copy.

The **delegate** (`ValueSerializer::Delegate`, with `WriteHostObject`, `GetSharedArrayBufferId`,
`ThrowDataCloneError`) is the extension seam: host-defined object types serialize through
`WriteHostObject` into the same linear stream, so a runtime can teach the format about its own
value kinds.

Cost model: `WriteValue` is a single depth-first pass over the graph, O(nodes + edges + bytes),
with one identity-map lookup per object and a linear output buffer that the caller then owns. It
is the general answer, and correspondingly the expensive one relative to sharing a backing store,
because it *materialises the whole graph into bytes*. It is used only when memory cannot be
shared (cross-isolate, cross-process, disk).

## Hard-won lessons (what was slow/wrong and what fixed it)

1. **Handle-table indirection is non-negotiable under a moving GC, and it is a correctness
   property, not a tax.** The host cannot be given a raw runtime-object pointer, because the GC
   relocates objects and would leave that pointer dangling
   ([blog.reverberate.org](https://blog.reverberate.org/2016/10/17/native-extensions-memory-management-part2-javascript-v8.html)).
   Every design decision downstream (scoped `Local`s, `Persistent` for cross-scope holds,
   `External` for host pointers into JS) exists to preserve this invariant. A runtime with a
   *non-moving* value store does not inherit this constraint, which is the first thing to check
   when deciding whether the pattern transfers.

2. **A generic Rust-struct-to-JS-object mapper was the dominant boundary cost, and the fix was
   to stop crossing object graphs, not to speed up the mapper.** `serde_v8` is on a removal path:
   "in most cases using serde_v8 is a big footgun in terms of performance"
   ([deno_core#716](https://github.com/denoland/deno_core/issues/716)). The canonical example:
   URLPattern ops returned an 8-9 key struct; "on each op invocation the key is serialized and
   copied from Rust to V8. Then in JavaScript, this object is immediately destructured and GCed."
   Every returned object is per-field allocation, per-field copy, then immediate garbage. The
   remediation is structural: return a **primitive array** instead of an object ("return an array
   of values, which would alleviate a lot of pressure on GC"), **unfurl struct fields into
   positional op arguments** rather than serializing a struct, and construct the object in JS
   where one is actually needed. Even before removal, the crate's own guidance is "directly using
   rust structs/tuples or primitives, since mapping to serde_json::Value will add extra overhead"
   ([docs.rs/serde_v8](https://docs.rs/serde_v8)). The lesson: the object-graph channel is the
   costly one; avoid putting anything on it that could cross as scalars or a buffer.

3. **The fast path is a monomorphic scalar/buffer ABI; strings are the awkward middle.** Fastcall
   handles primitives, `v8::Local` refs, and buffers with no marshalling. Strings resist zero-copy
   because JS strings may be Latin-1 while the host wants UTF-8, so "strings in ops always require
   a copy (at least)," fastcall only when Latin-1
   ([op2 docs](https://docs.rs/deno_core/latest/deno_core/attr.op2.html)). Deno chipped at even
   this: a thread-local reusable buffer plus `v8::String::write_utf8_into`
   ([PR #32688](https://github.com/denoland/deno/pull/32688)) and true zero-copy `&str` for the
   ASCII case via `ValueView` and earlier `SeqOneByteString`
   ([PR #16777](https://github.com/denoland/deno/pull/16777)). The takeaway: encoding mismatches
   (like UTF-8-vs-Latin-1) are exactly where copy-free breaks down; a runtime whose wire encoding
   already matches the host's avoids this class of cost.

4. **Transferable/detach is the primitive for moving big data cheaply.** A large buffer moves by
   detaching the source `ArrayBuffer` (zeroing its length) and re-wrapping the same
   `BackingStore` on the far side, coordinated by transfer id, instead of copying the bytes
   ([ValueSerializer TransferArrayBuffer](https://v8.github.io/api/head/classv8_1_1ValueSerializer.html);
   [MDN transfer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer)).
   The general principle, stated at the level the whole system converges on: **scalars cross by
   value, bulk data crosses as a shared/transferred backing store, object graphs cross only
   through an explicit serializer.** Three channels, chosen by shape, never one uniform "return
   the value" call.

5. **Aliasing live VM memory is sound only while the VM is quiescent.** The zero-copy `&[u8]`
   slice into an `ArrayBuffer` is valid until V8 is re-entered, at which point JS could mutate or
   the GC could act, hence the explicit `(copy)` variants for when that guarantee is unavailable.
   Copy-free is a conditional optimisation gated on a lifetime/quiescence contract, not a free
   default.

## Mapping to vehje (what transfers, what does not, with reasoning)

**What does not transfer (and is a relief that it does not):**

- **The handle-table / moving-GC machinery is vehje's biggest simplification.** V8's entire
  `Local`/`Persistent`/`HandleScope` apparatus exists because the GC relocates live objects, so
  the host can never see a stable address. Vehje's runtime interprets a residual and produces an
  output value; there is no requirement that the output value store be a *moving* heap. If the
  runtime materialises its output into a **bump/arena region that never relocates** (which the
  no-alloc, caller-provided-arena discipline already implies), then stable offsets into that arena
  are valid host-visible references, and none of the handle-table indirection is needed. The
  lesson to carry: the indirection is a *consequence of a moving store*; do not adopt it, and
  correspondingly do not adopt a moving store, unless a real requirement forces one.

- **`serde_v8`-style whole-graph field-by-field marshalling is the anti-pattern to avoid by
  construction.** The "out-buffer value" rejected design and the "retained handle + accessors"
  rejected design are the two poles V8/Deno also live between; Deno's verdict is that the object
  materialisation cost dominates and the answer is to not materialise. Vehje should not define a
  boundary that serializes an arbitrary nested struct into per-field host allocations.

**What transfers directly:**

- **Three-channel-by-shape is the core transferable idea.** Vehje's output is "structs upon
  structs upon structs, arbitrarily large trees," but that graph decomposes into scalars, byte
  runs, and structure. The V8/Deno convergent answer maps cleanly: scalar leaves cross by value;
  large homogeneous byte/element runs cross as a shared arena region (vehje's analog of a
  `BackingStore`, a `(ptr, len)` into the runtime arena that the host reads in place); and the
  *shape* (the tree of records and child indices) crosses as an explicit, tag-based, linear
  serialization. Notably, vehje's tier-0 residual format is already "fixed seven-word node
  records + a flat child-index pool + a self-contained string blob" (`wire.rs`) which is
  structurally the same idea as V8's linear tagged stream: a flat, offset-referenced,
  copy-free-to-walk layout. The output value can use the *same* shape as the input residual: a
  flat arena of fixed records plus a child-index pool, which the host reads by offset without a
  deserialization pass.

- **`ValueSerializer`'s identity map for cycles/sharing is the one graph-walk lesson to keep.**
  If vehje outputs can share subtrees or (ever) form cycles, the flat-arena encoding needs the
  same `id_map_`/`kObjectReference` trick: assign each record an index, and a repeated/back
  reference is just that index in the child pool rather than a re-emitted subtree. Vehje's
  index-based residual already has this shape (children are indices, not inlined), so shared
  subtrees are free and cycles are expressible; this is the right call and V8 confirms it is the
  right call for arbitrary graphs.

- **The move-not-copy / detach primitive maps onto bounded residency.** V8 bounds memory for huge
  buffers by *transferring ownership* of the backing store rather than copying. Vehje's bounded-
  residency requirement for enormous outputs is the same problem: the runtime should be able to
  hand the host ownership of an output arena region (in-proc: transfer the arena pointer + len +
  a free callback over the C ABI, the exact analog of `new_backing_store_from_vec` taking over
  ownership and freeing on drop) so the value crosses without a copy and residency is bounded by
  who holds it, not by duplicating it. For the subprocess path this becomes: stream the flat
  arena down the pipe / write it to an mmap-able file the host maps, which is copy-free-on-read
  the way a `BackingStore` is copy-free-on-share.

**The in-proc `.a` vs subprocess split, mapped:**

- **In-proc (path 2, C ABI):** this is Deno's exact shape (Rust host, foreign VM, C ABI). The
  transferable lessons apply almost literally. Cross the boundary with (1) scalars by value in the
  return/out-params, (2) a `(ptr, len)` view into the runtime arena for bulk regions with an
  explicit ownership+free contract (the `BackingStore`-takes-ownership model: either the host
  borrows while the runtime is quiescent, or ownership transfers and the host later calls
  `vehje_runtime_free_value`), and (3) the flat record arena as the graph encoding, read by
  offset. The aliasing-quiescence warning applies: a borrowed view is valid only while the
  runtime is not re-entered; if the host may re-enter, transfer ownership instead of lending.

- **Subprocess (path 1, pipe/file):** here memory genuinely cannot be shared across the process
  boundary, which is exactly V8's cross-isolate/`postMessage`/disk case, and the answer there is
  the serializer, not a shared pointer. Vehje's flat arena *is* the serialized form already (it is
  designed to be written and read as bytes), so the "serialize the graph" step is free: the
  runtime writes the same arena it built, the host maps or reads it. The one thing to preserve
  from `ValueSerializer` is that the encoding is self-contained and offset/index-referenced (no
  absolute pointers), so it survives the copy across the pipe intact, precisely V8's
  "backward-compatible, safe to store to disk" property.

**Net recommendation the V8/Deno data point supports:** do not pick one of the two rejected
designs; pick the shape by value-shape. Scalars and small results cross by value across the C
ABI; a single **flat, index-referenced, self-contained output arena** (the same family as
vehje's residual wire format) is the graph channel, read copy-free by offset in-proc and
written verbatim to the pipe/file out-of-proc; large homogeneous data inside it is a `(ptr, len)`
region whose *ownership* transfers (bounding residency) rather than being copied. This is the
minimal design that is simultaneously copy-free where possible (in-proc borrow/transfer),
bounded (ownership transfer, not duplication), works for both boundary kinds (same bytes, mapped
or piped), and extends to future tiers (bytecode/native runtimes emit the same output-arena
contract). The retained-handle idea is not wrong in the abstract (V8 supports it via `External`),
but V8/Deno's own experience says the graph should cross as one self-describing flat encoding, not
as an opaque handle chattered at through N accessor calls.

## Sources (links)

- V8 embedder guide (Local/Persistent handles, HandleScope, Isolate, Context, External, moving GC updates handles): https://v8.dev/docs/embed
- Why handle indirection under a moving GC; no raw pointers to GC objects across a GC: https://blog.reverberate.org/2016/10/17/native-extensions-memory-management-part2-javascript-v8.html
- V8 ArrayBuffer / BackingStore lifetime (`GetBackingStore` shared pointer, detach, externalization): https://v8.github.io/api/head/classv8_1_1ArrayBuffer.html
- rusty_v8 ArrayBuffer / BackingStore Rust API (`new_backing_store_from_vec`, ownership-takeover, `data`, `detach`, `with_backing_store`): https://docs.rs/v8/latest/v8/struct.ArrayBuffer.html
- `#[op2]` macro: argument/return markers, fastcall path, zero-copy buffers, string copy semantics: https://docs.rs/deno_core/latest/deno_core/attr.op2.html
- `serde_v8` crate (bijection, `serde_v8::Value` passthrough, perf caveats): https://docs.rs/serde_v8 and https://lib.rs/crates/serde_v8
- "Road to removal of serde_v8" (URLPattern GC-pressure example, unfurl fields / return arrays): https://github.com/denoland/deno_core/issues/716
- Deno zero-copy string work (`ValueView`, `write_utf8_into`, `SeqOneByteString`): https://github.com/denoland/deno/pull/32688 and https://github.com/denoland/deno/pull/16777
- v8::ValueSerializer C++ reference (`WriteHeader`/`WriteValue`/`Release` ownership, `TransferArrayBuffer`, Delegate): https://v8.github.io/api/head/classv8_1_1ValueSerializer.html
- rusty_v8 ValueSerializer Rust API (`new` with `ValueSerializerImpl`, `write_value`, `release -> Vec<u8>`): https://docs.rs/v8/latest/v8/struct.ValueSerializer.html
- V8 value-serializer.cc (identity map `id_map_`/`next_id_`, `kObjectReference`, `array_buffer_transfer_map_`, `SerializationTag`): https://github.com/v8/v8/blob/main/src/objects/value-serializer.cc
- ArrayBuffer.prototype.transfer / transferable zero-copy move semantics: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer
