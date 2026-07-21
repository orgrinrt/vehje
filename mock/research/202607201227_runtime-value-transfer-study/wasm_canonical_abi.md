# WebAssembly: value transfer across the module/host boundary

## One-line summary

WebAssembly's answer, refined across three layers (core linear memory, the
Component Model's Canonical ABI, and the async `stream`/`future` types), is:
everything that is not a machine scalar crosses as an offset into a flat
byte region, and the hard-won part was not the offset idea but the formal
ownership contract (`realloc` + `post-return`) that makes "return a pointer
and length" safe, plus streaming as the escape hatch for values too large
to hold resident. Vehje already sits on the winning side of this design
because its residual is a flat, index-addressed arena, but it has not yet
adopted the ownership-contract and streaming pieces, which is exactly the
open problem.

## Core wasm: linear memory, offsets, and the pointer+len idiom

Core WebAssembly functions can only take and return the four numeric value
types `i32`, `i64`, `f32`, `f64` (the MVP; later proposals add `v128`,
`funcref`, `externref`). The "why component model" design doc states it
plainly: functions are "restricted, essentially, to using integer (`i32` or
`i64`) or floating-point (`f32` or `f64`) types."
([why-component-model](https://component-model.bytecodealliance.org/design/why-component-model.html))

A module's memory is "a list of raw uninterpreted bytes" organized in pages
of 64 KiB, with a minimum size and an optional maximum, grown at runtime via
`memory.grow` and queried via `memory.size`. In the MVP a module has a single
memory (memory index 0), which most instructions reference implicitly.
([core spec / modules](https://webassembly.github.io/spec/core/syntax/modules.html))
Load and store instructions address that memory by an `i32` offset.

The consequence is the load-bearing fact of the whole WebAssembly value story:
**anything that is not one of the four scalars crosses the boundary as an
offset (a pointer) into linear memory, plus whatever companion scalars
describe its shape.** A string is passed as two `i32`s: "an integer offset
into a memory and an integer representing the length"
([why-component-model](https://component-model.bytecodealliance.org/design/why-component-model.html)).
The host (or another module that imports the same memory) then reads those
bytes directly out of the shared linear memory. Within a single shared memory
this is genuinely zero-copy: the reader dereferences the offset in place, no
marshalling.

It is also completely unsafe and untyped. The same doc notes "nothing in the
type system to prevent the returned length from being confused with the
returned offset, since both are integers," and that the memory
export/import convention lives "outside the type system entirely." The
pointer is just a number; its validity, its layout, and its lifetime are
conventions the two sides agree on out of band.

That out-of-band agreement is the classic **"return pointer + length, caller
reads linear memory" idiom**, and its unsolved half is allocation ownership:
who frees the region. In practice the guest module exports allocation entry
points (`malloc`/`free`, or a `cabi_realloc`/`dealloc` pair; TinyGo and Rust
guests export exactly these), and the host must call the guest's own
deallocator, exactly once, on a pointer the guest itself allocated. Two
failure modes recur: calling `free` on a region the guest did not allocate
with the expected layout corrupts the allocator, and caching a host-side
pointer into linear memory across a `memory.grow` leaves it dangling, because
growth can relocate the backing store and the cached offset now points into
garbage.
([radu-matei practical guide](https://radu-matei.com/blog/practical-guide-to-wasm-memory/),
[component-model issue #314, efficient memory passing](https://github.com/WebAssembly/component-model/issues/314))

So core wasm gives the mechanism (offset into flat memory, zero-copy read)
but not the safety (no types, no ownership contract). The next two layers add
each of those.

## The Canonical ABI: lifting/lowering nested typed values, realloc/post-return ownership

The Component Model targets the harder case: two separately-compiled
components with **disjoint** linear memories. A component "may not export a
memory and thus it cannot indirectly communicate to others by writing to its
memory" ([why-component-model](https://component-model.bytecodealliance.org/design/why-component-model.html)).
There is no shared address space, so a pointer from one component is
meaningless in the other. The Canonical ABI is the formal contract for moving
typed values (`record`, `variant`, `list`, `string`, and arbitrarily nested
aggregates thereof, described in WIT) across that gap.

Two operations bracket every call. `canon lift` wraps a core function
(`CoreFuncInst`) as a component-level function (`FuncInst`); `canon lower`
does the inverse, exposing a component function to core code. Lowering
decomposes a component value into core scalars and memory writes; lifting
reconstructs a component value from core scalars and memory reads.
([CanonicalABI.md](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md))

**Flattening and the spill to a pointer.** The ABI first tries to pass a value
as a flat sequence of core scalars. `flatten_functype` computes the flattened
param and result lists and spills to linear memory past fixed budgets. The
constants are exact
([definitions.py](https://github.com/WebAssembly/component-model/blob/main/design/mvp/canonical-abi/definitions.py)):

```python
MAX_FLAT_PARAMS = 16
MAX_FLAT_RESULTS = 1

def flatten_functype(opts, ft, context):
  flat_params = flatten_types(ft.param_types())
  flat_results = flatten_types(ft.result_type())
  if not opts.async_:
    if len(flat_params) > MAX_FLAT_PARAMS:
      flat_params = [opts.memory.ptr_type()]         # spill args to one pointer
    if len(flat_results) > MAX_FLAT_RESULTS:
      match context:
        case 'lift':
          flat_results = [opts.memory.ptr_type()]    # return a pointer to a return area
        case 'lower':
          flat_params += [opts.memory.ptr_type()]    # caller passes an out-pointer
          flat_results = []
```

The lesson embedded here: a handful of scalars pass in registers, but the
moment the value is structured or dynamically sized, it is passed as a single
`i32` pointer into linear memory, with the aggregate laid out there. A caller
lowering a big result passes an out-pointer (the return area); a callee
lifting a big result returns a pointer. This is the core-wasm offset idiom,
now type-directed instead of hand-agreed.

**Layout of nested aggregates.** `store()` and `load()` recurse structurally
over the WIT type. Records are laid out field by field with alignment padding;
lists and strings are `(ptr, len)` pairs where the elements are stored
contiguously at `ptr` respecting element alignment; variants are a
discriminant (the smallest integer type covering the case count) followed by
the payload union aligned to the largest case
([CanonicalABI.md](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md),
[definitions.py](https://github.com/WebAssembly/component-model/blob/main/design/mvp/canonical-abi/definitions.py)):

```python
case ListType(t, l)     : return load_list(cx, ptr, t, l)
case RecordType(fields) : return load_record(cx, ptr, fields)
# record loading: record[field.label] = load(cx, ptr, field.t)
```

A deeply nested value is thus a tree of contiguous records whose list/string
fields point elsewhere in the same memory. Walking it is a recursive descent
over offsets. This is precisely vehje's "structs upon structs upon structs"
problem, formally specified.

**The copy across disjoint memories.** Because the two components do not share
memory, lifting-then-lowering across a component call is a genuine copy:
source memory to a freshly allocated buffer in destination memory. It is
zero-copy only within one memory (the core-wasm host case above), never across
the component boundary. The whole aggregate is walked and re-serialized into
the target's memory.

**The ownership contract (`realloc` + `post-return`).** This is the part core
wasm lacked. To allocate the destination buffer, the ABI calls a wasm-exported
`realloc` (conventionally `cabi_realloc`) with the signature `(original_ptr,
original_size, alignment, new_size) -> new_ptr`; a fresh allocation passes
`realloc(0, 0, align, size)`
([definitions.py](https://github.com/WebAssembly/component-model/blob/main/design/mvp/canonical-abi/definitions.py)).
The **callee's** module provides `realloc`, so the destination owns the memory
it writes into. After the results have been lifted out and handed to the
caller, the ABI invokes the callee's optional `post-return`:

```python
if opts.post_return is not None:
  inst.may_leave = False
  [] = call_and_trap_on_throw(opts.post_return, flat_results)
  inst.may_leave = True
```

`post-return` runs once, after the return values are safely lifted, to free
the temporary buffers the callee allocated to hand results out. This closes
the "who frees" hole: the lifetime is a defined two-phase handshake
(allocate via `realloc`, return, then reclaim via `post-return`), not an
out-of-band convention. Out-of-memory inside `realloc` (a `memory.grow`
returning -1) traps rather than corrupting state.
([CanonicalABI.md](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md))

For opaque, non-serializable entities the model adds `resource` types with
`own<T>` / `borrow<T>` handles: `own` transfers cleanup responsibility to the
consumer, `borrow` grants temporary access while the owner keeps it. Handles
are indices into a per-instance table, never raw pointers, so they stay valid
across the boundary.
([Explainer.md](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md))

## Streaming (`stream<T>` / `future<T>`) for values too large to materialize

The Canonical ABI above still materializes the whole value on each side. The
async additions (stabilized in WASI 0.3.0, February 2026) are the model's
answer to values too large or too incremental to hold resident.
([Bytecode Alliance, WASI 0.3](https://bytecodealliance.org/articles/WASI-0.3))

A `stream<T>` carries an ordered sequence of `T`; a `future<T>` is the
single-value degenerate case. Each has two ends, a **readable end** and a
**writable end**. Passing a stream/future across the boundary transfers
ownership of the readable end; the writable end is sticky and stays with the
component that created it and cannot be transferred.
([component-model issue #185](https://github.com/WebAssembly/component-model/issues/185))

Core wasm code drives them with completion-based built-ins: `stream.read` and
`stream.write` (and `future.read` / `future.write`), each passing a
linear-memory buffer. A call either copies elements into or out of that buffer
immediately, or returns a "blocked" sentinel meaning the transfer continues
concurrently and the task will be rescheduled when it can progress. There is
built-in flow control (backpressure) so a fast producer does not overrun a
slow consumer, and the host runs a single shared event loop that schedules
whichever task a delivered value unblocks, even across multiple component
boundaries.
([WASI 0.3 async search synthesis](https://bytecodealliance.org/articles/WASI-0.3))

The load-bearing property for vehje: the producer writes elements into a
bounded buffer the consumer supplies, and the whole value is never resident at
once. Residency is capped at the buffer window, not at the value size.
Streaming beats materialization exactly when the value can exceed the memory
budget.

## Wasm GC as the alternative to hand-laid linear-memory structs

The GC proposal adds engine-managed heap types: `struct` (heterogeneous
fixed-field aggregate), `array` (homogeneous), and typed references
`(ref $t)` / `(ref null $t)`, plus `i31ref`, `eqref`, `anyref`. These let a
producer build nested typed objects the engine garbage-collects, instead of
hand-laying records with `store`/`load` in linear memory.
([gc Overview.md](https://github.com/WebAssembly/gc/blob/main/proposals/gc/Overview.md))

The decisive tradeoff for a value-transfer boundary: GC objects are
**fundamentally separate from linear memory.** They cannot be addressed by a
memory offset, cannot be stored as raw bytes in linear memory, and cannot be
reinterpreted between representations. A GC reference therefore cannot be
written into a flat buffer, cannot be sent over a pipe, and cannot cross a
plain module boundary as bytes. You gain automatic collection and type
checking; you lose the flat, relocatable, byte-addressable representation that
makes offsets-into-a-buffer work.

This is why the Component Model built interop on flat linear memory + the
Canonical ABI rather than on GC references. Flat data serializes and relocates;
managed references do not. Any runtime whose value must cross to a host as
bytes (in memory or over a pipe) is on the linear-memory side of this line, not
the GC side.

## Hard-won lessons (why the formal ownership contract and Interface Types exist)

The Explainer and design docs are unusually explicit about what failed before.
([Explainer.md](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md),
[why-component-model](https://component-model.bytecodealliance.org/design/why-component-model.html))

1. **Everything non-scalar crosses as an offset into a flat region.** This is
   the one idea the entire stack is built on. Scalars pass directly; a string,
   list, or nested aggregate passes as a pointer (plus length) into a flat byte
   region the other side reads. It is load-bearing because it is the only
   representation that is simultaneously compact, walkable, relocatable, and
   serializable.

2. **The naive "return pointer + length" was unsafe until a formal ownership
   contract wrapped it.** Bare pointer+len has no lifetime and no owner; the
   host caching a stale offset across `memory.grow`, or freeing a region the
   guest did not allocate, are real corruptions. The Canonical ABI's fix was
   not to abandon pointer+len but to formalize its lifetime: allocate through
   the destination's `realloc`, hand results out, then reclaim through
   `post-return`, once, deterministically. The pointer became safe by acquiring
   a contract, not by disappearing.

3. **Interface Types / the Canonical ABI exist because every language pair was
   hand-rolling its own marshalling.** With only ints and a shared memory, "a
   string in C is represented entirely differently from a string in Rust or in
   JavaScript," so each pairing wrote bespoke, mutually-incompatible,
   type-unsafe serialization. The Canonical ABI replaced that N-times-M
   fragmentation with one canonical, type-directed lowering/lifting described
   by a single WIT contract. One owned format, not one per consumer.

4. **Streaming beats materialization when the value is too big to hold.** The
   sync ABI materializes the whole value on both sides; `stream`/`future` exist
   precisely so a producer can feed a bounded consumer buffer incrementally,
   capping residency at the window rather than the value.

5. **Copy is irreducible across disjoint memories; zero-copy exists only within
   one memory.** The core-wasm host reading the guest's memory in place is
   zero-copy. The component-to-component crossing is always a copy, because
   there is no shared address space to point into. The design does not pretend
   otherwise; it makes the copy safe and typed rather than eliminating it.

## Mapping to vehje (what transfers, what does not, with reasoning)

Vehje's boundary is narrower than the general component case but the same
physics apply. The runtime produces one possibly-enormous nested output value
that must reach the host either in-process over the 3-fn C ABI (shared address
space) or as a subprocess writing a pipe/file (disjoint address spaces). Vehje
already owns a flat, index-addressed arena format (the tier-0 residual: fixed
seven-word records, a flat child-index pool, a self-contained string blob).
That format is not incidental; it is the same design WebAssembly converged on,
and it puts vehje on the correct side of every lesson above.

**What transfers directly:**

- **Lesson 1 (offset into a flat region) is already vehje's model, and the
  output value should reuse it.** The output value should be a flat arena of
  value records where nested references are indices into the same arena, not
  pointers. This is the residual format applied to values instead of IR. It is
  compact, walkable by recursive descent (exactly like the ABI's `load()` over
  `RecordType`/`ListType`), relocatable, and serializable. Do not invent a
  second, pointer-based value representation; the arena is the representation.

- **Lesson 5 splits vehje's two paths cleanly, and one format serves both.**
  In-proc over the C ABI the host and runtime share an address space, so the
  host reads the arena in place: genuinely zero-copy, the core-wasm-host case.
  Over the subprocess pipe the memories are disjoint, so exactly one copy is
  irreducible (serialize to the pipe, the host reads it back). Because the arena
  uses indices not pointers, it is relocatable and can be written to the pipe
  verbatim; the same bytes are both the in-proc live representation and the wire
  format. This is the unification the open problem is looking for: not two value
  models, one arena that is zero-copy in-proc and one-copy across the pipe.

- **Lesson 2 is the fix for the rejected "retained handle" design.** The
  retained-handle-plus-accessors option was rejected for its lifetime and
  ownership concerns, but those concerns are the same "who frees the pointer"
  hole core wasm had, and the Canonical ABI shows the fix is a contract, not
  avoidance. The C ABI already has the shape: `_execute` returns an `i32`
  status and the value comes back as an offset+length into a runtime-owned
  arena (the MAX_FLAT spill pattern: structured results return a pointer, not
  marshalled scalars). Give that returned region a `post-return`-style
  contract: the runtime owns the arena, the host reads it while it is valid,
  and the host calls a single `vehje_runtime_release` (the existing `_free`, or
  a value-scoped variant) exactly once. That is the two-phase handshake
  (produce, read, reclaim) that made pointer+len safe. The handle is fine once
  it has this contract; the rejection was of the uncontracted handle.

- **Lesson 4 answers the bounded-residency requirement for enormous outputs.**
  Neither rejected design bounds residency: the out-buffer is unbounded and the
  retained graph keeps everything alive. The stream answer is a pull-based
  cursor over the value: the runtime materializes a bounded window of the arena
  (a chunk, or a subtree), the host consumes it, the runtime advances and
  reuses the window. Residency caps at the window, not the value. For the
  subprocess path the pipe already is this stream (write a chunk, the host
  reads, repeat, with the OS pipe buffer as backpressure, the direct analogue
  of `stream.write` returning a blocked sentinel). For the in-proc path it is a
  cursor over the C ABI: an advance-and-read entry point that fills a bounded
  region. Modeling both as one streamed-cursor-over-a-flat-arena, with the
  ownership contract from lesson 2 on each window, is the sophisticated design
  the two naive options failed to reach.

- **Lesson 3 is a discipline constraint on vehje's tiers and consumers.** The
  value encoding must be one owned format across tier-0 (flat arena), future
  bytecode and native tiers, and every consumer/host. The fragmentation the
  Canonical ABI exists to prevent is each tier or each host inventing its own
  value marshalling. The residual is already tier-tagged and self-contained;
  the value format should be equally canonical and tier-orthogonal, so a native
  tier produces the same arena shape a tier-0 interpreter does.

**What does not transfer:**

- **Wasm GC is the wrong model for vehje and confirms the arena choice.** Vehje
  is `no_std`, no alloc, flat arena; GC-managed nested references are the
  opposite, and their defining property (not addressable by offset, not
  storable as bytes, not serializable) is exactly what would break both the
  in-proc zero-copy read and the subprocess pipe. The lesson is negative and
  reassuring: the reason WebAssembly built interop on flat memory rather than GC
  is the reason vehje's arena is right. Do not reach for a managed value graph.

- **The full lift/lower type-directed marshalling machinery does not transfer,
  because vehje controls both ends.** The Canonical ABI's elaborate structural
  lowering exists to bridge two languages with different in-memory layouts (C
  strings versus Rust strings). Vehje's runtime and its arena format are one
  design; the host reads vehje's own layout. There is no cross-language layout
  gap to marshal across, so vehje needs the arena and the ownership contract
  but not a WIT-style per-type lift/lower pass. The value crosses as the arena's
  own bytes, not re-encoded into a neutral ABI. (A host in a foreign language
  still needs a reader for the arena format, but that is one documented format,
  not a per-type marshalling protocol.)

- **`resource` / `own` / `borrow` handle tables do not transfer as such.** They
  solve opaque non-serializable entities crossing between untrusting instances.
  Vehje's output is data (a value tree), fully serializable, so it needs the
  value-arena path, not an opaque-handle table. The one idea worth borrowing is
  that a handle is an index into a table, never a raw pointer, which vehje
  already honors by using arena indices.

The synthesis: vehje should keep the flat arena as the single value
representation (lesson 1, 3), return it as a runtime-owned region with a
`post-return`-style release-exactly-once contract rather than an uncontracted
handle (lesson 2), unify the in-proc zero-copy read and the subprocess
one-copy write on that one relocatable format (lesson 5), and add a bounded
pull-cursor / stream over it so residency is capped for enormous outputs
(lesson 4). GC and full type-directed lift/lower stay out.

## Sources (links)

- WebAssembly core spec, modules and linear memory: https://webassembly.github.io/spec/core/syntax/modules.html
- Component Model, Canonical ABI (CanonicalABI.md): https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
- Component Model, Canonical ABI reference implementation (definitions.py, `MAX_FLAT_PARAMS=16`, `MAX_FLAT_RESULTS=1`, `flatten_functype`, `realloc`, `post_return`): https://github.com/WebAssembly/component-model/blob/main/design/mvp/canonical-abi/definitions.py
- Component Model, Explainer (canon lift/lower, WIT, resources/own/borrow): https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md
- Why the Component Model? (offset+length string idiom, no type safety, hand-rolled marshalling, components may not export memory): https://component-model.bytecodealliance.org/design/why-component-model.html
- Component Model, async streams/futures rationale (issue #185): https://github.com/WebAssembly/component-model/issues/185
- Bytecode Alliance, WASI 0.3 launch (streams/futures, backpressure, shared event loop): https://bytecodealliance.org/articles/WASI-0.3
- Component Model, efficient memory passing between wasm and host (issue #314): https://github.com/WebAssembly/component-model/issues/314
- Wasm GC proposal Overview (struct/array/ref heap types, separation from linear memory): https://github.com/WebAssembly/gc/blob/main/proposals/gc/Overview.md
- A practical guide to WebAssembly memory (pointer+len ownership, malloc/free exports, grow invalidation): https://radu-matei.com/blog/practical-guide-to-wasm-memory/
