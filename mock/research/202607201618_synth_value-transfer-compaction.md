# Value-transfer study: compacted, one section per runtime

**Date:** 2026-07-20
**What this is:** a compaction of the five runtime docs in `202607201227_runtime-value-transfer-study/`
(the study's own `synthesis.md` excluded). One section per document, conveying the whole picture of each while
dropping the exact API names, numbers, and verbatim code. For the specifics, read the named source. For the
cross-cutting domain view, read
`202607201618_synth_embeddable-runtime-value-model-and-host-boundary.md`.

## .NET CLR (`dotnet_clr.md`)

The CLR is presented as the cautionary template: its internal value model is exactly what makes crossing to
native code expensive, and its whole interop and buffer stack is a decades-long campaign to claw that cost
back. Every type is either a reference type living on a garbage-collected, relocating heap and addressed by a
mutable object reference, or a value type stored inline with no independent identity; boxing bridges them by
allocating a fresh heap object and copying, so a large nested output is a tree of individually-allocated,
movable objects that cannot be handed outside the runtime without transformation. That transformation is
marshalling, and its core distinction is blittable (identical bit layout on both sides, nothing to do) versus
non-blittable (allocate, copy, convert). Because the collector relocates, handing native code a pointer into
managed memory requires pinning, which defeats the collector and fragments the heap. The modern answer to
transferring a large value is not a bigger buffer but a pull-based, backpressured, streaming writer: the
consumer supplies buffer space on demand, the producer writes bounded chunks, and the writer blocks once the
consumer's backlog crosses a high-water mark, so only a bounded window is ever resident regardless of total
size, and an arbitrarily nested structure serializes this way. Alongside this sit a stack-only borrowed-view
type whose safety the compiler enforces at compile time, a heap-storable companion for when that view must
outlive a synchronous call, a multi-segment sequence so a value need not be contiguous, and pooled buffers so
streaming does not allocate per chunk. The hard-won lessons: runtime-generated marshalling stubs were
abandoned for compile-time source generation because resolving the boundary ahead of time into inspectable
static code beats generating it at run time; views beat defensive copies; the storable-handle versus
borrowed-window distinction must be two types because only one is a compiler-checked lifetime; and a
buffer-crossing API is not done until its ownership, consumption, and lease contract is stated. The doc's own
mapping notes that a flat, no-GC, no-alloc arena runtime designs away the GC, boxing, pinning, and marshalling
taxes entirely, and should borrow the pull-based writer, the backpressure, and the explicit lease contract.

## Lua and LuaJIT (`lua_luajit.md`)

Lua is the canonical embeddable-VM design, and its boundary is a hard refusal to let a value cross as memory:
every value stays VM-owned and collector-managed, and the host manipulates it only through a virtual stack
addressed by integer index, never by a pointer it can keep. Values move by copy-in and copy-out through that
stack. The reason for the indirection is stated as the whole point: because the host names a slot rather than
an address, the collector stays free to move objects, run incrementally between any two calls, or intern a
string, and none of it can dangle a host reference. To hold a value beyond a single call, the host receives an
integer key into a registry (a collector root), released explicitly; the durable handle is an integer,
stable across collection precisely because it is not a pointer. The reverse direction and opaque host objects
go through userdata: a full userdata is a VM-owned block whose finalizer lets the collector drive a host
object's teardown, and a light userdata is a raw host pointer the VM stores as an inert opaque token it never
dereferences. The sharpest embedding footgun is that a borrowed pointer into a VM string is valid only while
that string stays on the stack, a deliberately-scoped peek that must be copied out immediately or pinned;
bulk data is therefore routed through userdata rather than the string type, and LuaJIT adds a separate,
clearly-labelled fast door (its FFI) that gives pointer-direct, JIT-inlined access to C memory the VM treats
as opaque, winning large multiples on bulk numeric data by skipping the per-element copy, at the cost of the
collector management, portability, and classic-API compatibility the stack API existed to provide. LuaJIT's
other change, packing every value into a single word by stealing NaN bit-patterns, is a pure representation
win that changes nothing about the boundary contract. The durable lessons: never let the host hold a pointer
into the live graph, address everything by index or integer reference; separate the transient per-call
boundary from the explicitly-held durable reference; make any borrowed peek's lifetime explicit and hard to
misuse; and let a safe by-index boundary and a fast by-pointer boundary coexist as two labelled doors. The
doc's mapping: adopt the contract (address the output by index into a flat arena, borrow narrowly under a
Rust-lifetime-enforced window, offer a zero-copy bulk path) but drop the whole collector apparatus, the string
interning, and the per-leaf copy, because a born-flat arena lets the host read leaves in place.

## V8 and Deno (`v8_deno.md`)

V8 is the most directly relevant embedding shape because Deno is a Rust host embedding a C++ VM over an FFI,
which is exactly the in-process case. V8 never hands the host a raw pointer to a runtime value, because its
collector moves objects; instead the host holds handles that indirect through a table the collector updates on
relocation, in two lifetime classes (scoped and cross-scope), with a separate wrapper for carrying a host
pointer into script. This indirection is a correctness mechanism under a moving collector, not an
optimization, and it is the first thing to check when deciding whether the pattern transfers: a non-moving
store does not inherit it. The Rust-to-V8 call surface historically routed every non-buffer value through a
generic serde-style struct-to-object mapper, and the hard-won lesson is that this mapper was the dominant cost
(each returned object was per-field allocation, per-field copy, then immediate garbage), and the fix was
structural, to stop crossing object graphs at all: unfurl struct fields into positional arguments, return
primitive arrays, and pass raw handles through untouched, backed by a fast-call path that handles primitives,
handles, and buffers with no marshalling. Bulk bytes never cross as serialized values; they cross as a
refcounted shared backing store behind a typed array, copy-free, with a constructor that takes ownership of
host bytes and frees them when the last referencing buffer dies, and a detach-and-transfer operation that
moves a large buffer by re-wrapping the same store on the far side rather than copying. Aliasing that live
memory is sound only while the VM is not re-entered, so the API offers explicit copying variants. When an
arbitrary object graph genuinely must cross a boundary that cannot share memory, a dedicated serializer writes
a tag-based linear structured-clone stream, handling cycles and shared subgraphs through an identity map that
emits a compact back-reference on a repeated object rather than re-serializing it. The convergent principle
across the whole system: scalars cross by value, bulk data crosses as a shared or transferred backing store,
and object graphs cross only through an explicit serializer, three channels chosen by value-shape, never one
uniform return call. The doc's mapping: the handle-table machinery is vehje's biggest simplification to shed
(the arena never moves), the serde anti-pattern is the thing to avoid by construction, the three-channel split
maps cleanly onto a flat index-referenced output arena that reads copy-free in-process and writes verbatim to
a pipe out-of-process, the identity map for shared subtrees is free once children are indices, and
move-not-copy ownership transfer is the bounded-residency primitive.

## WebAssembly Canonical ABI (`wasm_canonical_abi.md`)

WebAssembly's answer, refined across three layers, is that everything not a machine scalar crosses as an
offset into a flat byte region, and the genuinely hard-won part was not the offset idea but the formal
ownership contract that makes returning a pointer and length safe, plus streaming as the escape hatch for
values too large to hold resident. In core wasm a module's memory is raw bytes addressed by integer offset,
so a string or aggregate passes as an offset plus companion scalars, and the reader dereferences it in place,
genuinely zero-copy within one shared memory but completely untyped: nothing in the type system distinguishes
the returned offset from the returned length, the memory convention lives outside the type system, and the
unsolved half of the classic pointer-plus-length idiom is who frees the region and the hazard of caching a
stale offset across a memory growth that relocates the backing store. The Component Model targets the harder
case of two components with disjoint memories and no shared address space, and its Canonical ABI is the formal
contract for moving nested typed values across that gap: a lifting and lowering step brackets every call, a
handful of scalars pass in registers but a structured or dynamically-sized value spills to a single pointer
into linear memory, nested aggregates are laid out as contiguous records whose list and string fields point
elsewhere in the same memory and are walked by recursive descent over offsets, and crossing two disjoint
memories is always a real copy, never zero-copy. The part core wasm lacked is a two-phase ownership handshake:
the destination's own allocator provides the buffer, results are lifted out, and a post-return hook then runs
once to reclaim the temporaries, closing the who-frees hole with a defined lifetime rather than an out-of-band
convention; opaque non-serializable entities use owning and borrowing handles that are indices into a
per-instance table, never raw pointers. The async additions add stream and future types with readable and
writable ends and completion-based reads and writes over a caller-supplied buffer with built-in flow control,
so a producer feeds a bounded consumer buffer incrementally and residency caps at the window, not the value.
The GC proposal's managed heap types are deliberately not addressable by offset, not storable as bytes, and
not serializable, which is precisely why WebAssembly built interop on flat linear memory rather than on GC
references. The lessons: everything non-scalar crosses as an offset into a flat region; the naive
pointer-plus-length became safe by acquiring a contract, not by disappearing; the Canonical ABI exists because
every language pair was otherwise hand-rolling incompatible marshalling; streaming beats materialization when
the value is too big; and copy is irreducible across disjoint memories while zero-copy exists only within one.
The doc's mapping: vehje's flat, index-addressed arena already is this winning design, the two paths unify on
one relocatable format that is zero-copy in-process and one-copy over a pipe, the rejected retained handle is
fixed by a post-return-style release-exactly-once contract, bounded residency is answered by a pull cursor or
stream over the arena, and both GC and the full type-directed lift-lower machinery stay out because vehje owns
both ends and has no cross-language layout gap to bridge.

## Zero-copy formats and handle-table patterns (`zerocopy_formats.md`)

Every mature system that reads arbitrarily-nested data without a decode step converges on one shape: a flat,
self-describing buffer whose internal links are relative offsets read in place under bounds checks, paired
with an explicit ownership-and-release contract for the backing memory. The surveyed systems split into two
halves of that pattern. The buffer half is spelled three ways that are the same idea: one format makes the
wire bytes literally the in-memory representation, laid out like a compiler would lay out a struct, with
data and pointer sections linked by offsets relative to the pointer word itself so the whole message can be
memory-mapped or relocated with no pointer fixups and read by pure pointer-chasing, organized as an arena of
segments with special cross-segment pointers as the real complexity cost; a second adds a vtable indirection
on every table field, buying clean optional-field and forward/backward-compatible versioning through a
default-value fallback in exchange for one extra load, in a single contiguous buffer built back-to-front so a
parent knows its children's final offsets when it is written; and a Rust one expresses the same relative
pointer as a type, laying objects out root-last for the same reason, with opt-in validation. The costs are
uniform: you give up the freedom to relocate mid-flight and to grow in place, alignment becomes a contract
rather than a detail, pointer-chasing a nested read is cache-unfriendly next to a bulk copy so zero-copy wins
only when the consumer touches a small fraction or memory-maps it, and an untrusted buffer is an attack
surface whose relative pointers can loop or describe an enormous logical structure in a tiny buffer, so every
serious implementation enforces a traversal and pointer-depth limit or a validation pass. The contract half is
also three spellings of one rule: a columnar interchange passes two small structs across a language boundary
where the consumer allocates the base struct, the producer owns everything it points at, and a single release
callback that recursively frees and nulls itself is the entire lifetime mechanism, with move semantics
transferring the single live responsibility; a buffer-view protocol lends a view while a reference lock pins
the exporter so it cannot be freed or resized, paired with a mandatory release; and a critical-region API
returns a raw pointer only inside a tightly scoped section with strict rules because the collector needs to
know when it can move memory again, alongside a reference-table split between cheap call-scoped handles freed
automatically and expensive explicitly-managed long-lived ones. The recurring lesson: copy-free access to huge
nested data is always a flat buffer with relative-offset links read in place under bounds checks plus an
explicit scoped ownership contract, and streaming beats in-place access when the consumer reads the whole
result once, when it does not fit a residency budget, or when the producer cannot finalize the layout before
the consumer needs the first bytes. The doc's mapping: vehje already owns the buffer half (its residual
arena's flat child-index pool is the same position-independent relative link in index form), the output
becomes a symmetric value-arena handed across as base pointer plus length plus an opaque token read in place
(not a per-field accessor model), governed by a release callback that dissolves the retained-handle lifetime
objection, validated by a bounds-checked traversal on the untrusted subprocess path, with chunked streaming as
the fallback when a single output exceeds the budget; the vtable and schema-evolution machinery, the columnar
layout, and the multi-segment far-pointer arena are all more than a single-versioned-toolchain, single-value,
one-contiguous-arena case needs.

## Sources

The five compacted documents: `202607201227_runtime-value-transfer-study/dotnet_clr.md`, `.../lua_luajit.md`,
`.../v8_deno.md`, `.../wasm_canonical_abi.md`, `.../zerocopy_formats.md`. Each carries its own primary
citations (Microsoft Learn and dotnet/runtime; the Lua reference manual and LuaJIT docs; the V8 embedder guide
and rusty_v8/deno_core; the WebAssembly core spec and Component Model Canonical ABI; Cap'n Proto, FlatBuffers,
Apache Arrow, CPython buffer protocol, JNI, and rkyv).
