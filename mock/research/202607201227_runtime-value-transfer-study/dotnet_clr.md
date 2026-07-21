# .NET CLR: value transfer across the embed boundary

## One-line summary

.NET's internal value model (a moving garbage-collected object graph with boxing) is the exact
thing that makes crossing to native code expensive, and its whole interop and buffer stack is a
30-year campaign to claw that cost back: make the wire form blittable so there is nothing to
marshal, pin so the GC cannot move it, and for large output abandon "return one buffer" entirely
in favor of an incremental pull-based writer (`IBufferWriter<T>` / `PipeWriter`) with backpressure
that streams an arbitrarily large nested value while only one segment is ever resident. vehje starts
on the far side of that campaign already won (flat arena, no GC, blittable by construction), so the
CLR is more cautionary tale than template, but three of its mechanisms map cleanly.

## The internal value model

The CLR splits every type into two kinds, and this split is the root of everything downstream.

**Reference types** (`class`) live on the managed heap. A variable of a reference type holds an
`object reference` (a pointer the runtime is free to change), not the object. The garbage collector
owns these objects: it traces reachability, reclaims unreachable objects, and, being a
*compacting/moving* collector, physically *relocates* live objects to defragment the heap. That
relocation is the pivotal fact. Any raw pointer into a heap object is invalid the instant the GC
runs, unless the object is pinned.

**Value types** (`struct`, the primitives `int`/`double`/`bool`, enums) are stored inline wherever
their container lives: a local lives on the stack, a `struct` field of a heap object lives inline
inside that heap object, an array element lives inline in the array's payload. The common shorthand
"value types live on the stack" is imprecise; the accurate rule is "a value type has no independent
identity or heap allocation of its own, it is stored inline in its container." A value-type variable
*is* the bytes; a reference-type variable is a pointer to the bytes.

**Boxing** bridges the two. When a value type must be treated as `System.Object` (or any interface),
the CLR *boxes* it: it allocates a fresh object on the managed heap, copies the value's bytes into
it, and returns a reference. **Unboxing** checks the boxed object's type and copies the bytes back
out into a value-type slot. Boxing is implicit, unboxing explicit. Every box is a heap allocation
plus a copy, so boxing in a hot loop is a classic GC-pressure and latency source
([Boxing and Unboxing, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/csharp/programming-guide/types/boxing-and-unboxing)).
Boxing is what lets "a value of any type be treated as an object" in the unified type system, and it
is precisely the indirection tax vehje never pays.

So "what is a value" in the CLR: either inline bytes with no identity (value type) or a movable,
GC-owned, traced heap object addressed by a mutable reference (reference type). A large nested output
in idiomatic .NET is the latter: a tree of heap objects connected by references, each independently
allocated, each movable, the whole graph owned by the GC. That representation is not directly
transferable to anything outside the runtime, which is the entire reason the marshalling layer exists.

## Crossing the boundary (mechanisms, named APIs, ownership contract)

The managed representation and the native representation differ, so crossing requires
**marshalling**: "the process of transforming types when they need to cross between managed and
native code" ([Type marshalling, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/type-marshalling)).
The core distinction is blittable vs non-blittable.

**Blittable types** have an identical bit representation in managed and native memory, so marshalling
them is *nothing*: the runtime hands over the address, no transformation. The blittable set is the
fixed-width primitives (`System.Byte`/`SByte`/`Int16`/`UInt16`/`Int32`/`UInt32`/`Int64`/`UInt64`
mapping to `uint8_t`..`int64_t`), `IntPtr`/`UIntPtr`, pointers, and `struct`s composed entirely of
blittable fields laid out with `[StructLayout(LayoutKind.Sequential)]` (or `Explicit`). A blittable
array marshals as "a pointer to the start of an array of native representations."

**Non-blittable types** need transformation because the representations diverge: `System.String`
(managed UTF-16, heap, immutable) vs a native `char*`/`char16_t*` with a chosen encoding and
null-termination; `System.Boolean` (1 byte) vs Win32 `BOOL` (4 bytes); `System.Array` vs a native
pointer or `SAFEARRAY`; a class marshalled by native-representation pointer. For these the runtime
allocates a native buffer, copies and converts, calls, and (for `in`/`out`/`ref`) copies back.
`[MarshalAs(UnmanagedType.LPUTF8Str)]` and `StringMarshalling.Utf8`/`Utf16`/`Custom` name the target
shape.

**P/Invoke** is the call mechanism. The classic form is `[DllImport]` on an `extern` method; the
runtime synthesizes an **IL stub** at call time that performs the marshalling and the transition
([P/Invoke source generation, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation)).
The modern form is `[LibraryImport]` on a `static partial` method, where a Roslyn source generator
emits the marshalling code at *compile* time (covered under Hard-won lessons).

**Pinning** is the bridge over the moving-GC problem. Because the collector relocates heap objects,
you cannot hand a native function a pointer into managed memory and expect it to stay valid. `fixed`
(a scoped statement) and `GCHandle` (a heap-allocated, explicitly-freed handle) both *pin*: they tell
the GC "do not move this object." `GCHandle.Alloc(obj, GCHandleType.Pinned)` returns a handle;
`AddrOfPinnedObject()` yields the stable address; `Free()` unpins and re-enables collection
([GCHandle, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.gchandle)).
`GCHandleType` has four values: `Normal` (keep alive, movable, for keeping a managed object reachable
while native code holds only an opaque token, e.g. a callback delegate), `Pinned` (keep alive *and*
fixed in place), `Weak` and `WeakTrackResurrection` (do *not* keep alive; observe without extending
lifetime). Pinning exists solely to defeat GC relocation for the duration of a native access, and it
hurts the collector: a pinned object is an immovable rock the compactor must work around, fragmenting
the heap. The ownership contract at this boundary is manual and unforgiving: the managed side must
keep the handle alive and `Free()` it exactly once, or leak; the native side must not retain the
pointer past the unpin.

**The managed/unmanaged ownership contract**, generalized: managed memory is GC-owned and movable;
native memory is caller-owned and manually freed; marshalling either copies across the divide (each
side owns its copy) or pins (managed side retains ownership, lends a stable address for a bounded
window). Bugs manifest as use-after-unpin (native pointer dereferenced after the GC moved or freed
the object), leaked `GCHandle`/`IMemoryOwner`, and double-marshalling copies that quietly dominate a
hot path.

## Zero-copy and streaming of large/nested values

This is the part that maps hardest onto vehje's problem, because it is .NET's answer to exactly
"transfer a possibly-enormous value without unbounded copies or unbounded residency."

**`Span<T>` / `ReadOnlySpan<T>`** are the copy-free primitive: a `(pointer, length)` view over *any*
contiguous memory (managed array, `stackalloc`, native buffer, a slice of a `string`). They are
`ref struct`s: stack-only, cannot be boxed, cannot be a field of a class, cannot cross an `await` or
`yield` ([System.Memory design guidelines, dotnet/runtime](https://github.com/dotnet/runtime/blob/main/docs/coding-guidelines/api-guidelines/System.Memory.md)).
That restriction is deliberate: the compiler's lifetime rules then *statically* guarantee the view
never outlives the buffer it borrows, which is the zero-copy safety contract enforced at compile time.

**`Memory<T>` / `ReadOnlyMemory<T>`** exist *because* `Span<T>` is stack-only. When a buffer view must
survive an async call or an iterator (be stored on the heap), `Span<T>` is illegal, so .NET 2.1 added
`Memory<T>`, which *can* live on the managed heap, and you project a `Span<T>` out of it via
`.Span` only at the synchronous moment of access
([Memory<T> usage guidelines, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/memory-and-spans/memory-t-usage-guidelines)).
`Memory<T>.Pin()` returns a `MemoryHandle` for the async-pinvoke case where `fixed` cannot span the
operation. The `Span`/`Memory` split is the single cleanest lesson here: separate the *storable
capability* (`Memory<T>`) from the *access capability* (`Span<T>`), because their lifetime contracts
differ and only one is expressible as a compiler-checked stack lifetime.

**`ReadOnlySequence<T>`** is the multi-segment answer to "the value is not contiguous." It represents
a logically-single buffer physically composed of a linked list of segments. A large or nested output
never has to be flattened into one allocation; it is a chain of pooled segments the reader walks. This
is the structural insight for enormous values: contiguity is not required, only sequential access plus
a way to say "I have consumed up to here, release those segments."

**`IBufferWriter<T>`** is the pull-based sink and the direct rebuttal of the naive out-buffer design.
Instead of the producer allocating its own output buffer and returning it, the *consumer* supplies
buffer space on demand: the producer calls `GetSpan(sizeHint)` / `GetMemory(sizeHint)` to borrow
writable space, writes into it, and calls `Advance(count)` to commit. `Utf8JsonWriter` is built on
`IBufferWriter<byte>`: it serializes an arbitrarily large, arbitrarily nested object graph by
repeatedly borrowing a chunk, writing tokens, advancing, and flushing, so it *never holds the whole
JSON tree in memory* ([System.Buffers, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/io/buffers)).

**`System.IO.Pipelines`** (`PipeWriter` / `PipeReader`) is `IBufferWriter<T>` plus lifecycle plus
backpressure, and it is the complete streaming-transfer machine
([System.IO.Pipelines, .NET Blog](https://devblogs.microsoft.com/dotnet/system-io-pipelines-high-performance-io-in-net/)).
`PipeWriter` implements `IBufferWriter<byte>`: the producer does `GetMemory(int)` /
`Advance(int)` / `FlushAsync()`. `PipeReader.ReadAsync()` returns a `ReadOnlySequence<byte>` the
consumer inspects *without committing*, then `AdvanceTo(consumed, examined)` tells the pipe which
segments can be recycled (partial consumption: peek a partial message, consume nothing, ask for more).
Buffers come from a pool, so streaming does not allocate per chunk. **Backpressure** is the residency
bound: `FlushAsync` blocks the writer once buffered-but-unconsumed data crosses `PauseWriterThreshold`,
and unblocks when the reader drains below `ResumeWriterThreshold`. That single mechanism is how .NET
transfers a value larger than memory: the producer is *forced to wait* for the consumer, so only a
bounded window is ever resident, regardless of total output size.

**`ArrayPool<T>` / `MemoryPool<T>`** underlie all of the above by recycling the transfer buffers
themselves, so a long stream reuses a small set of buffers instead of allocating (and GC-collecting)
one per chunk. `MemoryPool<T>.Shared.Rent()` returns an `IMemoryOwner<T>` carrying the lifetime.

## Hard-won lessons (what was abandoned/regretted and why)

**Runtime-generated marshalling IL stubs were abandoned for source generation.** `[DllImport]`
synthesizes its marshalling stub as IL *at runtime*, which the docs now call out as three concrete
problems: it is incompatible with Native AOT and IL trimming (no runtime code generation allowed
there), it costs JIT time and cannot be inlined, and "debugging the marshalling logic in `DllImport`
scenarios is a non-trivial exercise" because the stub is invisible generated IL. `[LibraryImport]`
(.NET 7) moves the marshalling into *compile-time* source-generated C# you can read and step through,
removes the runtime stub, and lets the P/Invoke inline. The lesson: resolve the boundary shape
*ahead of time* into inspectable, static code; runtime codegen of the boundary was a mistake paid for
in AOT-hostility, opacity, and cost.

**The `Span<T>` revolution reframed buffers as views, not copies.** Before `Span<T>`, slicing meant
`Array.Copy` or `string.Substring` (an allocation and a copy); passing a sub-buffer meant
`(array, offset, count)` triples threaded by hand and unchecked. `Span<T>` made the *view* a
first-class, bounds-checked, allocation-free primitive over any memory kind. The regret it corrected
was pervasive defensive copying and offset-bookkeeping bugs.

**`Memory<T>` exists because `Span<T>` alone was too restrictive.** The stack-only `ref struct`
constraint that makes `Span<T>` safe also makes it unusable across `await`/`yield`/heap-storage. Rather
than weaken `Span<T>`'s safety, they added a second, heap-storable type and kept the strong type
distinction. The lesson: do not collapse "storable buffer handle" and "borrowed access window" into
one type when their lifetime contracts differ; the difference *is* the design.

**The `Memory<T>` lease/ownership rules are the codified scar tissue of buffer-lifetime bugs.** The
usage guidelines are an explicit ruleset because the failure mode (a consumer retaining a buffer after
its lease ends, or two consumers racing) is real and silent. The rules, verbatim in spirit: Rule 3, a
`void`-returning method that takes `Memory<T>` must not touch it after returning (its lease is exactly
the call); Rule 4, the async analogue (the lease ends when the returned `Task` completes); Rule 7, an
`IMemoryOwner<T>` holder must `Dispose` it or transfer it, never both; Rule 8, accepting an
`IMemoryOwner<T>` parameter means accepting ownership. The three named concepts are *ownership* (single
owner, transferable), *consumption* (one consumer at a time absent external sync), and *lease* (the
window a consumer may touch the buffer). The hard-won point: a buffer-crossing API is not done until
its ownership/lease contract is stated at the type-and-convention level, because the compiler only
catches the `Span<T>` subset.

## Mapping to vehje (what transfers, what does not, with reasoning)

vehje's constraints (no_std, no_alloc, bounded residency, copy-free where possible, both an in-proc
`.a` and a subprocess exe, extensible to future tiers) intersect the CLR stack sharply. The CLR is
useful here mostly by showing which of its problems vehje has already designed away, and then donating
three mechanisms wholesale.

**Does not transfer (and mostly should not):**

- *The GC / managed-heap object-graph value model, boxing/unboxing, object references.* vehje has no
  collector and no heap; its value lives in a caller-provided bump arena as flat records. The entire
  reference-type/boxing indirection tax is absent by construction. This is the CLR's central cost and
  vehje never pays it, so none of the machinery built to manage it applies. Treat the CLR value model
  as the anti-pattern, not the template.
- *Pinning (`GCHandle` Pinned, `fixed`, `Memory<T>.Pin`).* Pinning exists *only* because a moving GC
  relocates objects. vehje's arena memory does not move, so it is permanently "pinned" by
  construction. In-proc, the host can read pointers directly into vehje's residual/value arena with
  zero pinning ceremony. This is a real advantage to state explicitly in the design: the
  stable-address property that .NET buys back at GC-fragmentation cost, vehje has for free.
- *Marshalling (`Marshal`, `MarshalAs`, `LibraryImport` string/struct conversion).* Marshalling exists
  because managed and native *representations differ*. vehje's tier-0 residual is already a flat,
  C-ABI-shaped, fixed-width-record blob: it is *blittable* in .NET's own sense. The correct lesson to
  steal is the blittable ideal itself: design the transferred value's wire form so managed/native
  representations are identical and there is nothing to marshal. vehje's `wire.rs` seven-word node
  records plus flat child-index pool plus self-contained string blob already satisfy this; the study
  confirms the shape is right and names why (blittable = the fastest possible marshal is no marshal).

**Transfers directly (the load-bearing borrowings):**

1. *Pull-based writer over caller-supplied buffers (`IBufferWriter<T>` / `PipeWriter.GetMemory` +
   `Advance`), replacing "producer returns one buffer."* This is the precise correction of vehje's
   rejected out-buffer design. Keep the incremental "runtime writes into buffer space it is handed,
   chunk by chunk" shape; drop the "the whole result must fit one caller buffer" assumption. The
   producer requests space, writes a bounded chunk of the value tree, signals how much, repeats. The
   same abstraction serves both vehje paths: in-proc, the host supplies a vehje-analogue of
   `IBufferWriter` (a `get_space`/`advance` callback pair over the C ABI, no_alloc, no dyn on the Rust
   side); subprocess, the sink is the pipe/file and the exe writes into pooled segments.

2. *Backpressure as the residency bound (`PauseWriterThreshold` / `ResumeWriterThreshold`).* This is
   the answer to "bounded memory residency even for enormous outputs." The runtime serializes the
   value depth-first into the sink and *blocks* when the sink's unconsumed backlog crosses a high-water
   mark, resuming when the host drains below a low-water mark. Only a bounded window of the enormous
   nested value is ever materialized at once, independent of total size. For the subprocess path this
   is literally OS pipe backpressure (a full pipe blocks `write`); for the in-proc path it is the
   host's `advance`/`consume` callback returning "buffer full, drain first." Either way the runtime
   never holds the whole tree. `Utf8JsonWriter`-over-`PipeWriter` is the existence proof that an
   arbitrarily large nested structure serializes this way with O(window) residency.

3. *Multi-segment sequence + explicit lease/ownership contract (`ReadOnlySequence<T>` +
   `AdvanceTo`, and the `Memory<T>` Rule 3/4/7/8 lease discipline).* Two joined lessons. First, the
   transferred value need not be one contiguous blob: a chain of segments the host walks, releasing
   consumed segments via an `advance-to` signal, bounds residency without flattening. Second, and this
   is the direct rescue of vehje's rejected "retained handle + accessors" design: .NET did *not*
   conclude "handles are bad." It concluded "a buffer/handle crossing an API is only safe when its
   ownership model (single owner, transferable), its consumer model (one at a time), and its *lease*
   (the exact window the consumer may touch it) are specified as a contract." vehje's handle idea
   failed because the lease was unspecified, not because handles are wrong. If vehje keeps any
   handle-shaped transfer, it must ship the lease contract with it: who owns the arena, for how long
   the host may read through the handle, when the runtime may reclaim or overwrite it, enforced by the
   Rust type system on the compile side (lifetime-checked) exactly where vehje can, and by documented
   C-ABI convention where it cannot (the unavoidable analogue of the `Memory<T>` rules the CLR could
   not put in the type system).

The synthesis-relevant conclusion: the optimal vehje boundary is not "out-buffer" and not "retained
handle," it is a *streamed, backpressured, pull-based serialization of the value into host-supplied
segments*, with a compile-time-resolved blittable wire form so no marshalling occurs, no pinning is
needed (arena is stable), and residency is bounded by the backpressure window rather than by the
output size. That is the `IBufferWriter` + Pipelines pattern, minus the GC/marshalling/pinning tax
that only the CLR has to pay, plus an explicitly stated lease contract on whatever crosses the C ABI.

## Sources (links)

- [Boxing and Unboxing, Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/csharp/programming-guide/types/boxing-and-unboxing)
- [Type marshalling (blittable vs non-blittable, default rules), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/type-marshalling)
- [P/Invoke source generation (LibraryImport vs DllImport IL stub, AOT/trimming, debuggability), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation)
- [GCHandle Struct (GCHandleType Normal/Pinned/Weak/WeakTrackResurrection, AddrOfPinnedObject, Alloc/Free), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.gchandle)
- [Memory<T> and Span<T> usage guidelines (owners/consumers/lease, Rules 1-10, IMemoryOwner, MemoryPool, Pin/MemoryHandle), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/memory-and-spans/memory-t-usage-guidelines)
- [System.Memory API design guidelines (Span<T> as ref struct, why Memory<T>), dotnet/runtime](https://github.com/dotnet/runtime/blob/main/docs/coding-guidelines/api-guidelines/System.Memory.md)
- [Memory-related and span types (overview), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/memory-and-spans/)
- [System.Buffers (IBufferWriter<T>, ArrayPool, Utf8JsonWriter over IBufferWriter), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/io/buffers)
- [System.IO.Pipelines: High performance IO in .NET (PipeWriter/PipeReader, GetMemory/Advance/FlushAsync, ReadOnlySequence, AdvanceTo, PauseWriterThreshold/ResumeWriterThreshold, backpressure, pooling), .NET Blog](https://devblogs.microsoft.com/dotnet/system-io-pipelines-high-performance-io-in-net/)
- [System.IO.Pipelines (PipeWriter/PipeReader reference), Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/standard/io/pipelines)
