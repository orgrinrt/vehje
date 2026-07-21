# Zero-copy formats & handle-table patterns: reading huge nested data without deserialization

## One-line summary

Every mature system that reads arbitrarily-nested data without a decode step converges on the same
shape: a flat, self-describing buffer whose internal links are relative offsets you pointer-chase in
place under bounds checks, paired with an explicit ownership-and-release contract for the backing
memory. Cap'n Proto and rkyv make the wire bytes literally the in-memory representation; FlatBuffers
adds a vtable indirection for schema evolution; Arrow's C Data Interface and CPython's buffer protocol
and JNI's critical/reference machinery are the boundary contracts that govern who owns the bytes and
for how long. vehje already has the buffer half of this (a flat bump-arena residual with relative
child-index pools); what the survey adds is the discipline for the second half, the ownership/release
contract and the bounds-checked traversal.

## Cap'n Proto (wire==memory, segments, relative-offset pointers, ownership)

Cap'n Proto's founding claim is that it is "INFINITY TIMES faster than Protocol Buffers" because
"there is no encoding/decoding step" ([capnproto.org](https://capnproto.org/)). The wire format serves
double duty as the in-memory representation: data is "arranged like a compiler would arrange a struct,
with fixed widths, fixed offsets, and proper alignment," so "once your structure is built, you can
simply write the bytes straight out to disk." Reading is the inverse: mmap the file and the OS pages in
only the parts you actually touch; field access is random, needing no whole-message parse.

The link primitive is a 64-bit **wire pointer** whose two least-significant bits tag the kind
([encoding.html](https://capnproto.org/encoding.html)): `00` struct, `01` list, `10` far, `11` other
(capabilities). The struct pointer packs: 2 bits kind, a **30-bit signed word offset** from the end of
the pointer word to the struct's data section, 16 bits data-section size (words), 16 bits
pointer-section size (count). A struct is therefore two contiguous regions: a fixed-width data section
(scalars) and a pointer section (links to sub-objects). The list pointer packs 2 bits kind, 30-bit
signed offset to the first element, a 3-bit element-size code (code 7 = composite, i.e. a list of
structs), and 29 bits of element count.

The load-bearing detail for position independence: **all intra-segment offsets are relative to the end
of the pointer word itself**, not to any absolute base. Resolving an address is
`addr = (pointer_word_end) + offset * 8`. That is what lets a message be copied, mmap'd, or relocated
without any pointer fixups, and what lets you read a deeply-nested tree by pure pointer-chasing:
dereference the root struct pointer, index into its pointer section, dereference that pointer, repeat.

A message is not one flat buffer but an **arena of segments**. Objects are bump-allocated sequentially
into a segment "until there is no more room, in which case a new segment is allocated and objects
continue to be allocated sequentially there." Cross-segment links cannot use a simple offset, so the
**far pointer** (kind `10`) carries: a 1-bit landing-pad-size flag, a 29-bit offset to a landing pad,
and a 32-bit target segment id. A single far pointer's landing pad is a normal pointer sitting
immediately before the target object; a double-far pointer (used when the target segment is full and
you must point into the middle of another segment) uses a two-word landing pad whose second word tags
the actual object. This far/double-far machinery is the real complexity cost of the segmented arena.

Ownership: a `MessageBuilder` owns the arena it bump-allocates into; a `MessageReader` (e.g.
`FlatArrayMessageReader` over an mmap'd `[]word`) borrows the segments and hands out cheap reader views
that are just `(segment, offset)` cursors. The reader never copies; it validates as it walks.

What it costs. First, **pointer indirection**: every nested field is a dereference, so a pointer-heavy
tree is a chain of dependent loads (cache-unfriendly next to a contiguous copy). Second, **alignment**:
"all objects are aligned to word boundaries" and "primitive types must always be aligned to a multiple
of their size," so the producer must respect 8-byte alignment and the consumer must be able to assume
it. Third, **untrusted-input safety**: because a malicious message can make pointers loop or point into
enormous zero-sized lists, every implementation MUST enforce a **traversal (read) limit** ("each time a
pointer is dereferenced, a counter is incremented by the size of the data it points to"; C++ default 64
MiB) and a **pointer-depth limit** (default 64), and must "count a list of zero-sized elements as if
each element were one word" so an attacker cannot force unbounded iteration over a tiny buffer. Those
limits are not optional polish; they are the price of pointer-chasing bytes you did not produce.

## FlatBuffers (vtables, in-place read, builder model, contrast with Cap'n Proto)

FlatBuffers targets the same zero-copy goal but was built for a sharper constraint: game programmers in
C++ who "want to avoid heap allocations at all costs" and read levels/animations directly from an mmap
([white_paper](https://flatbuffers.dev/white_paper/), [flatbuffers.dev](https://flatbuffers.dev/)).
"The only memory needed to access your data is that of the buffer. It requires 0 additional
allocations." It is "very suitable for use with mmap (or streaming), requiring only part of the buffer
to be in memory." That is why latency-critical and mobile systems chose it: no parse, no unpack, no
allocation, random field access, mmap-friendly.

The offset types ([internals](https://flatbuffers.dev/internals/)): `uoffset_t` is always `uint32_t`,
an unsigned **forward** offset used for tables, unions, strings, and vectors; `soffset_t` is its signed
form, used only for the vtable link (which can sit anywhere relative to the object); `voffset_t` is a
`uint16_t`, the type of vtable slots. Fixing offsets at 32 bits keeps buffers binary-compatible between
32- and 64-bit machines.

The distinguishing mechanism is the **vtable**. A table object begins with an `soffset_t` that is
*subtracted* from the object start to find its vtable. A vtable is: `voffset_t[0]` = size of the vtable
in bytes, `voffset_t[1]` = size of the object in bytes, then one `voffset_t` per schema field giving
that field's byte offset within the object (0 meaning "absent"). Reading a field: the generated
accessor holds the field's vtable index as a constant, checks it against the vtable's first element
(the size, i.e. the count of known fields) "to protect against newer code reading older data," and if
the index is out of range or the slot is 0, returns the schema **default value**; otherwise it adds the
slot's offset to the object base and reads in place. That check is exactly what buys **forward and
backward compatibility**: old readers skip fields they do not know, new readers reading old data get
defaults for fields that did not exist. Vtables are deduplicated: objects with identical field layouts
share one vtable. Structs (as opposed to tables) are fixed-layout, inline, aligned to their largest
member, with no vtable and no versioning, the fast path when you never need evolution. Strings and
vectors are "contiguous aligned scalar elements prefixed by a 32-bit element count," reached through a
`uoffset_t`, never inlined.

The **builder writes back-to-front**: "the buffer is constructed backwards, starting at the highest
memory address," so leaf objects (sub-tables, strings, vectors) are written first and the root last and
early in the buffer. Writing children before parents means a parent already knows the final offsets of
its children when it is written, which "significantly reduces the amount of bookkeeping and simplifies
the construction API." (A recurring consequence, noted on the mailing lists, is that you cannot easily
stream-build front-to-back into a fixed region; the root's position is only known at the end.)

Contrast with Cap'n Proto. Both are flat, offset-linked, zero-copy, mmap-friendly. Cap'n Proto makes
the struct layout itself the schema (fixed data+pointer sections, no per-object indirection table), so
field access is one fewer indirection but schema evolution is governed by ordinal/offset rules baked
into the layout; it also carries the segment/far-pointer arena machinery for unbounded builders.
FlatBuffers adds the vtable indirection on every table field (one extra load) and in return gets
cleaner optional-field/versioning semantics and a single contiguous buffer with no segment concept. For
a runtime that values the absolute minimum indirection and can accept the layout-is-the-schema
contract, Cap'n Proto is closer; for one that needs robust field-level evolution across versions,
FlatBuffers' vtable is the reason to pay the extra hop.

## Apache Arrow (columnar zero-copy, the C Data Interface release contract)

Arrow is the other axis: not a tree-of-structs wire format but a **columnar in-memory layout** that
two libraries in the same or different processes agree on, so a typed, possibly-nested dataset can be
handed across a language/library boundary with no serialization at all. The transfer vehicle is the
**C Data Interface**: two POD structs, `ArrowSchema` (the type) and `ArrowArray` (the data), defined in
a single `abi.h` with no Arrow library dependency ([CDataInterface](https://arrow.apache.org/docs/format/CDataInterface.html)).

`ArrowSchema` carries `format` (a null-terminated string encoding the datatype, e.g. `i` for int32,
`+s` for struct), optional `name` and `metadata`, `flags`, `n_children` + `children` (a C array of
child-schema pointers, which is how nested/struct/list types are described), an optional `dictionary`,
plus `release` and `private_data`. `ArrowArray` carries `length`, `null_count`, `offset`, `n_buffers` +
`buffers` (a C array of raw `void*` into the actual column memory: validity bitmap, offsets buffer,
values buffer, depending on type), `n_children` + `children` (child arrays, mirroring the schema's
nesting), an optional `dictionary`, plus `release` and `private_data`.

The ownership contract is the part worth stealing. **The consumer allocates and owns the base struct
itself** (on its own stack or heap) and passes a pointer to the producer to fill in. **The producer
allocates and owns everything the struct points at**: the format/metadata strings, the buffers, the
children arrays. The consumer "MUST not try to interfere with the producer's handling of these
members' lifetime." The `private_data` is opaque producer state the consumer must never touch. The
**release callback** is the entire lifetime mechanism: the consumer's only influence over lifetime is
calling `array->release(array)` exactly once when done. The spec pins its behaviour precisely: the
release callback MUST walk all children (and the dictionary) and call their releases, MUST free any
data area owned by the struct (buffers, children arrays), and MUST mark the struct released by setting
its own `release` member to NULL. A NULL `release` is the sentinel for "already released." There is
also **move semantics**: a consumer may bitwise/shallow-copy the struct and then mark the source
released by zeroing its `release` *without* calling it, transferring the single live responsibility to
the copy; only one live copy exists at a time.

"Zero-copy across the boundary" here means exactly: both sides implement the same documented columnar
layout, so the buffers are passed by pointer and read in place; nothing is marshalled. The catch is the
precondition, both sides must agree on the layout a priori. When they do, an arbitrarily large,
arbitrarily nested typed column crosses for the cost of filling in two small structs. When the data is
a stream of such batches rather than one, Arrow defines a sibling **C Stream Interface** (`get_next`
pulling successive `ArrowArray`s), which is the "stream instead of hold-in-place" answer for unbounded
data.

## Host-side in-place access patterns (CPython buffer protocol, JNI critical, rkyv)

These three are the canonical designs for "lend a view into my memory to another component without
copying, with a bounded, explicit lifetime."

**CPython buffer protocol (PEP 3118 / `Py_buffer`)** is the textbook "lend a view" contract
([c-api/buffer](https://docs.python.org/3/c-api/buffer.html)). The `Py_buffer` struct is a plain C
descriptor (not a `PyObject`): `buf` (pointer to the logical start), `obj` (the exporting object),
`len`, `itemsize`, `readonly`, `ndim`, `format` (a `struct`-module type string), and for
multidimensional data `shape`, `strides`, `suboffsets`, plus `internal` (exporter-private, consumer
must not touch). A consumer requests a view with `PyObject_GetBuffer(exporter, &view, flags)`; the
`flags` negotiate what the consumer can handle and what it needs: `PyBUF_SIMPLE` (flat bytes),
`PyBUF_WRITABLE` (must be writable or the request fails), `PyBUF_FORMAT` (fill in the type string),
`PyBUF_ND` (shape), `PyBUF_STRIDES` (shape+strides), `PyBUF_INDIRECT` (suboffsets), and the
`PyBUF_C_CONTIGUOUS`/`PyBUF_F_CONTIGUOUS`/`PyBUF_ANY_CONTIGUOUS` contiguity requests. The lifetime
mechanism is a **reference lock**: on success `GetBuffer` sets `view.obj` to a *new reference* to the
exporter (increments its refcount), which pins the exporter, it cannot be freed or resized while the
view is outstanding. The consumer MUST pair every successful `GetBuffer` with exactly one
`PyBuffer_Release(&view)`, which decrements the refcount and releases the lock. The whole design exists
so the consumer can "access that buffer directly and without intermediate copying" while the
refcount-lock guarantees the memory stays alive and stable for the borrow.

**JNI array-critical and reference tables** is the same idea under a moving garbage collector, and it
teaches the handle-table lesson ([JNI functions](https://docs.oracle.com/en/java/javase/21/docs/specs/jni/functions.html)).
`GetPrimitiveArrayCritical(env, arr, &isCopy)` returns a raw pointer into a Java array "if possible;
otherwise a copy is made," and while the pointer is held the VM "cannot bring the VM to a state that
allows garbage collection" (some VMs disable GC outright). The price is a hard restriction: between
`GetPrimitiveArrayCritical` and `ReleasePrimitiveArrayCritical` the native code is in a **critical
region** where it "must not call other JNI functions, or any system call that may cause the current
thread to block," and must not run long. That is the archetypal bounded-in-place-access window: you get
a copy-free pointer, but only inside a tightly scoped section with strict rules, because the owner (the
GC) needs to know exactly when it can move memory again. The reference-table lesson is separate and
complementary: JNI handles to Java objects come as **local references**, which are "valid for the
duration of a native method call and freed automatically after the native method returns," backed by a
per-frame table (the VM guarantees capacity for at least 16, extendable via `EnsureLocalCapacity` /
`PushLocalFrame`/`PopLocalFrame`); a reference that must outlive the call must be promoted to a
**global reference** via `NewGlobalRef` and explicitly released with `DeleteGlobalRef`. The design
distinguishes cheap, auto-reclaimed, call-scoped handles from expensive, explicitly-managed,
long-lived ones. That is the exact axis a retained-handle value model has to decide.

**rkyv** is the Rust "the archive is directly usable" approach and the closest analogue to vehje's own
residual ([rkyv/rkyv](https://github.com/rkyv/rkyv), [docs.rs/rkyv](https://docs.rs/rkyv)). "rkyv
guarantees the in-memory representation matches the serialized format," so "if you wrote your data to
disk, you can just mmap your file into memory, cast a pointer, and your data is ready to use." It solves
the pointer problem with **relative pointers** (`RelPtr`, an offset not an address), making archived
data relocatable and position-independent, exactly Cap'n Proto's offset trick expressed in Rust types;
`ArchivedVec<T>` is a `Vec` whose elements sit in the buffer and are reached by a relative pointer.
Layout mirrors FlatBuffers/Cap'n Proto: "objects are laid out in reverse order, the root near the end
of the buffer, leaf objects near the beginning," because the root is written last once its children's
offsets are known. Access without deserialization is the default; **validation is opt-in** via
`bytecheck` / the `check_bytes` machinery, which walks the buffer and proves every relative pointer,
length, and enum discriminant is in-bounds and well-formed before you are handed a typed reference.
rkyv's own framing of the tradeoff is the useful one: "because checking serialized data can generally
be done without allocations, the cost of checking and zero-copy access can be much lower than that of
traditional deserialization," so validated zero-copy is still cheaper than deserializing, and the check
is the thing that makes an *untrusted* buffer safe to pointer-chase.

## Hard-won lessons (the recurring copy-free pattern and its costs)

The recurring pattern, stated once. Copy-free access to huge nested data is always **a flat,
self-describing buffer with relative-offset internal pointers that you read in place under bounds
checks, plus an explicit ownership/release contract for the backing memory.** Cap'n Proto, FlatBuffers,
and rkyv are three spellings of the buffer half (offsets relative to the pointer, or forward `uoffset`,
or `RelPtr`; a tree laid leaves-first so parents know child offsets). Arrow's C Data Interface,
CPython's buffer protocol, and JNI are three spellings of the contract half (the release callback, the
refcount lock, the critical region + reference tables).

The costs, equally recurring:

- **You give up the freedom to relocate mid-flight and to grow in place.** Everything internal is an
  offset from a fixed anchor; that is what makes it position-independent, but the producer must finalize
  layout before handing it over (hence back-to-front construction, hence Cap'n Proto's segment-and-far-
  pointer machinery for builders that outgrow one region).
- **Alignment is a contract, not a detail.** Word/element alignment must be produced and may be assumed
  by the reader; violate it and the in-place cast is UB or slow.
- **Pointer-chase versus copy is a real tradeoff, not a free win.** A pointer-heavy nested read is a
  chain of dependent, cache-unfriendly loads. Zero-copy wins decisively when the consumer touches a
  small fraction of a large buffer (mmap + random access) and loses to a bulk copy when the consumer
  will stream the whole thing linearly anyway.
- **An untrusted buffer is an attack surface.** The same relative pointers that make it fast make it a
  weapon: loops, out-of-bounds offsets, and tiny buffers describing enormous logical structures. Every
  serious system answers this the same way, with a validation/limit pass: Cap'n Proto's mandatory
  traversal and depth limits, rkyv's `bytecheck`. In-place access to bytes you did not produce is only
  safe behind a bounds-checked walk.
- **Someone must own the bytes, and the borrow must be explicitly scoped.** The release callback
  (Arrow), the refcount lock (Py_buffer), the critical region and local/global reference split (JNI):
  all encode the same rule. The consumer borrows; the producer owns; the borrow ends at a named,
  mandatory release. There is no zero-copy handoff without a lifetime contract, because "who frees this
  and when" cannot be answered by the bytes alone.

When streaming beats in-place access: when the consumer will read the whole result sequentially exactly
once, when the result does not fit in a residency budget, or when the producer cannot finalize the
whole layout before the consumer needs the first bytes. That is why Arrow ships a C Stream Interface
next to the C Data Interface, and why a subprocess pipe is a natural fit for produce-once-consume-once.
In-place access wins when the consumer needs random access to a small part of a large structure, or
will re-read it, or wants it memory-mapped and paged on demand.

## Mapping to vehje (what transfers, what does not; note vehje already has a flat arena + child-index pool)

vehje already owns the buffer half of the pattern. The tier-0 residual at
`mock/crates/vehje-runtime-abi/src/wire.rs` / `encode.rs` is exactly a flat serialized arena: fixed
seven-word node records, a flat child-index pool (relative indices into that arena, which is the same
position-independent relative-link idea as Cap'n Proto offsets, rkyv `RelPtr`, and FlatBuffer
`uoffset`s, just index-form rather than byte-offset-form), and a self-contained string blob. The
runtime is a dumb index evaluator over pre-resolved indices, so there is no name resolution to do at
read time. This is the correct foundation; the survey confirms the shape rather than contradicting it.
The **produced output value** should be represented as the same kind of thing: a second flat arena of
value records with a relative child-index pool, so the value graph is a position-independent buffer the
host reads in place. Call it the value-arena, symmetric with the residual-arena the runtime already
consumes.

What transfers directly:

- **Relative links, not pointers, for the value graph.** vehje's child-index pool is already this. Keep
  the value graph as records + a flat index pool so the whole value-arena is one relocatable, mmap-able,
  pipe-able blob with zero fixups, working identically for the in-proc `.a` (hand back a base pointer)
  and the subprocess exe (write the blob to a pipe/file/shared region). This is the single design that
  satisfies "must work for both boundaries" without a second mechanism, because the boundary becomes
  "where does the one blob live," not "how do we serialize differently per path."
- **A leaves-first / append-only build order.** The runtime produces value nodes bottom-up (children
  before parents, the natural evaluation order), and FlatBuffers/rkyv show that writing children before
  parents means a parent records already-known child indices with no back-patching. This fits a
  bump-arena and vehje's no-alloc discipline: append records, never relocate.
- **An explicit ownership/release contract at the C ABI, modelled on Arrow's release callback.** This
  is the piece the rejected "retained handle + accessors" design was missing and the piece that makes
  it work. Extend the 3-fn ABI so `_execute` yields a value-arena descriptor (base pointer + length +
  an opaque `private_data`) plus a `release` function the host calls exactly once. The runtime owns the
  arena's backing memory (a caller-or-runtime-provided region, consistent with no-alloc: the arena
  lives in a bump region the runtime was given or reserved); the host borrows it read-only in place and
  signals "done" via release. That is Arrow's contract, minus the columnar layout, and it dissolves the
  lifetime/ownership objection to handles: the lifetime is not open-ended, it is "until release," named
  and mandatory. Crucially this is NOT the rejected accessor-call model: the host reads the flat arena
  directly (no per-field FFI round-trips), the handle is only the ownership token for the backing bytes.
- **Bounds-checked in-place traversal for the subprocess/untrusted path.** In-proc over the `.a`, the
  runtime produced the bytes and the host can trust them. Across a subprocess pipe or a file, the bytes
  are effectively untrusted input, and Cap'n Proto's traversal/depth limits and rkyv's `bytecheck` are
  the non-negotiable lesson: the host-side reader validates every child-index is in range and the graph
  is acyclic/depth-bounded before or during the walk. vehje should ship a validation pass over the
  value-arena (indices in-bounds, string-blob offsets in-bounds, depth capped) that the untrusted path
  runs and the trusted in-proc path may skip. This also directly bounds residency against a malicious
  or runaway output.
- **Bounded residency via the arena budget, and streaming as the escape hatch.** The value-arena is a
  fixed region; if a full-program output exceeds the budget, the answer is the Arrow-C-Stream / pipe
  lesson, not a bigger buffer: emit the value graph as a sequence of arena chunks over the pipe
  (subprocess path) or as successive filled regions the host drains and releases (in-proc path). The
  in-place value-arena is the default for outputs that fit a residency budget; chunked streaming is the
  fallback for ones that do not. This is exactly the "in-place vs stream" line the survey draws, and it
  is extensible to future tiers (bytecode, native) because the value-arena format is orthogonal to how
  the value was computed.

What does not transfer, or transfers only as a lesson:

- **FlatBuffers/Cap'n Proto vtables and schema-evolution machinery are overkill here.** Their vtable and
  ordinal-versioning complexity exists to evolve a *schema* across independently-versioned producers and
  consumers. vehje's runtime and its residual are produced and consumed by the same versioned toolchain
  (the tier is orthogonal to correctness; the effect proof is discharged in Rust before lowering), so
  the value node format can be a closed, versioned record layout without per-field vtable indirection.
  Take the flat-buffer + relative-offset lesson; skip the vtable tax.
- **Arrow's columnar layout does not fit an arbitrary nested value tree.** Arrow is optimal for
  homogeneous columns of a known type; vehje's output is a heterogeneous struct-upon-struct tree. Take
  Arrow's *contract* (the release callback, producer-owns / consumer-borrows, move semantics), not its
  *layout* (columnar buffers). The value-arena stays tree-shaped like Cap'n Proto/rkyv, governed by an
  Arrow-style release contract.
- **Cap'n Proto's segmented multi-segment arena with far/double-far pointers is more than vehje needs**
  for a single produced value. A single contiguous value-arena (FlatBuffers-style, one buffer) is
  simpler and sufficient; the segment/far-pointer machinery only earns its complexity when a builder
  must grow unboundedly across disjoint regions, which the chunked-streaming fallback handles more
  cleanly for vehje's case. Keep one contiguous arena per value (or per stream chunk); do not import
  segments.
- **JNI's GC-critical-region restriction does not apply** (vehje has no moving collector), but its
  reference-table lesson does: distinguish the cheap call-scoped borrow (the in-proc host reads the
  arena within one `_execute`/`release` span, like a local reference auto-freed at return) from a
  promoted long-lived hold (a host that wants the value to outlive the call must take explicit
  ownership, like `NewGlobalRef`/`DeleteGlobalRef`, i.e. copy it out or keep the arena and the release
  token alive deliberately). Encode that split in the ABI so the default is the cheap scoped borrow and
  the long-lived hold is an explicit, host-driven choice.

The net design the survey points to: represent the output as a flat, contiguous **value-arena** of
records with a relative child-index pool (reusing vehje's existing residual-arena shape), hand it across
the C ABI as a base-pointer + length + opaque token descriptor read **in place** by the host, govern its
lifetime with an **Arrow-style release callback** (producer owns, consumer borrows, one mandatory
release), **validate** it with a bounds-checked traversal on the untrusted subprocess path (Cap'n
Proto/rkyv limits), and fall back to **chunked streaming over the pipe** when a single output exceeds the
residency budget. That satisfies every constraint in the brief: no_std/no_alloc (bump-arena, no heap),
bounded residency (fixed arena budget + streaming escape), copy-free where possible (in-place read),
one mechanism for both the `.a` and the exe boundary (the arena is just bytes; the boundary only decides
where they live), and tier-extensibility (the value-arena format is orthogonal to bytecode/native
tiers).

## Sources (links)

- Cap'n Proto encoding spec: https://capnproto.org/encoding.html
- Cap'n Proto introduction / "infinitely faster" and arena model: https://capnproto.org/
- FlatBuffers internals (offsets, vtables, back-to-front builder): https://flatbuffers.dev/internals/
- FlatBuffers white paper (design goals, why games): https://flatbuffers.dev/white_paper/
- Apache Arrow C Data Interface (ArrowSchema/ArrowArray, release contract, move semantics): https://arrow.apache.org/docs/format/CDataInterface.html
- Apache Arrow C Stream Interface (streaming successor): https://arrow.apache.org/docs/format/CStreamInterface.html
- Apache Arrow C ABI header (`abi.h`): https://github.com/apache/arrow/blob/main/cpp/src/arrow/c/abi.h
- CPython buffer protocol (`Py_buffer`, PyBUF flags, GetBuffer/Release lock): https://docs.python.org/3/c-api/buffer.html
- PEP 3118, Revising the buffer protocol: https://peps.python.org/pep-3118/
- JNI functions spec (GetPrimitiveArrayCritical, local/global references): https://docs.oracle.com/en/java/javase/21/docs/specs/jni/functions.html
- JVM Anatomy Quark #9, JNI Critical and GC Locker: https://shipilev.net/jvm/anatomy-quarks/9-jni-critical-gclocker/
- rkyv, zero-copy deserialization framework for Rust: https://github.com/rkyv/rkyv
- rkyv docs (Archive, relative pointers, validation): https://docs.rs/rkyv
