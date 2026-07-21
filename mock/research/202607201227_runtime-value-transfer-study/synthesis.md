# Synthesis: the vehje runtime output value-transfer model

Five runtime families (.NET CLR, Lua/LuaJIT, V8/Deno, WebAssembly core + Component Model, and the
zero-copy format family Cap'n Proto/FlatBuffers/Arrow/rkyv/Py_buffer/JNI) were studied independently
for one question: how a runtime hands a produced value, possibly enormous and deeply nested, to an
embedding host with bounded residency and copy-free where possible. They converge, hard, on one
answer, and the answer is a design vehje is already halfway into by construction. This document pulls
the convergent lessons, audits where the convergence could be overselling, and then pins the concrete
vehje design, including the sink contract that the earlier seed left vague.

## The one-paragraph judgement

The output is not "returned" and not "held behind accessors." It is **written, as a flat,
index-referenced value-arena (the same structural family as vehje's residual), into a pull-based sink
the host drives, under an explicit single-release ownership contract, with backpressure bounding
residency and a bounds-checked validation pass on the untrusted (subprocess) path.** Scalars skip the
arena and cross by value; bulk byte runs cross as a shared region, not re-serialized. The same
value-arena bytes are the zero-copy in-proc representation and the verbatim subprocess wire form, so
one format serves both boundaries and every future tier. Both rejected designs were half-right: the
out-buffer had the right "runtime writes bytes" instinct but no backpressure and a fixed size; the
retained-handle had the right "hand back a region" instinct but no lease contract and (fatally) a
per-field accessor model. The synthesis keeps the correct half of each and drops the broken half.

## The convergence: nine cross-cutting lessons

Each is asserted by at least three of the five, most by all five.

1. **The value graph crosses as ONE flat, self-describing, index/offset-referenced buffer, not as a
   materialized host object graph and not as per-field accessor calls.** Cap'n Proto/rkyv/FlatBuffers
   make the wire bytes the in-memory form; V8's `ValueSerializer` is a linear tagged stream; the wasm
   Canonical ABI lays nested records contiguously and walks them by offset; .NET's fast path is
   blittable (identical repr, nothing to marshal); Lua's lesson is that its per-leaf copy-out exists
   only because its values are scattered `TValue`s on a GC heap with no contiguous form, which vehje
   can simply avoid by being born flat. vehje's residual (`wire.rs`: fixed seven-word records + flat
   child-index pool + self-contained string blob) is exactly this shape. The output reuses it.

2. **Relative links (indices/offsets), never absolute pointers, make one format serve both
   boundaries.** Cap'n Proto offsets are relative to the pointer word; rkyv `RelPtr`; FlatBuffers
   `uoffset`; wasm arena indices. Position-independence is what lets the same bytes be read in place
   in-proc (hand back a base pointer) and written verbatim to a pipe/file/shared region out-of-proc
   with zero fixups. vehje's child-index pool is already index-form, so this is free.

3. **Three channels by value-shape (V8/Deno's sharpest lesson): scalars by value, bulk byte runs as a
   shared `(ptr,len)` region, structure as the flat record arena.** Deno's `serde_v8` field-by-field
   object materialization was the dominant cost, and the fix was structural (stop crossing object
   graphs: unfurl fields to positional args, return primitive arrays, pass the handle through
   untouched), not a faster mapper. Do not put on the arena channel what is a scalar, and never
   re-serialize a megabyte of bytes field-by-field.

4. **The handle was never the problem; the UNCONTRACTED handle was.** Every system rescues the
   retained-region idea with an explicit ownership/release contract: Arrow's `release` callback
   (producer owns pointed-to memory, consumer calls release exactly once, NULL sentinel marks
   released, move-semantics for single-owner transfer), wasm's `realloc` + `post-return` two-phase
   handshake, Py_buffer's refcount-lock + mandatory `PyBuffer_Release`, Lua's `luaL_ref`/`luaL_unref`
   integer keys, .NET's `Memory<T>` lease rules (3/4/7/8), V8's refcounted `BackingStore`. vehje's
   rejected retained-handle failed on two counts: (a) unspecified lease, (b) per-field accessor
   round-trips. Fix both: state the lease AND read the flat arena in place with no accessor chatter.

5. **Bounded residency is streaming with backpressure, never a bigger buffer.** .NET
   `PipeWriter`/`PauseWriterThreshold`+`ResumeWriterThreshold` (the producer is forced to wait for the
   consumer, so only an O(window) slice is resident regardless of total size; `Utf8JsonWriter` over a
   pipe is the existence proof for arbitrarily nested trees); wasm `stream<T>`/`future<T>`
   completion-based reads/writes over a caller-supplied buffer with backpressure; Arrow's C Stream
   Interface `get_next`; V8's detach/transfer; Lua's transient-window-vs-pin. The producer writes into
   a bounded window the consumer drains; residency caps at the window.

6. **Untrusted input demands a bounds-checked traversal.** Cap'n Proto's mandatory traversal limit
   (64 MiB default) and pointer-depth limit (64), rkyv's opt-in `bytecheck`. Bytes that crossed a
   process boundary are untrusted; the reader validates every index/offset is in range and the graph
   is depth-bounded before/while walking. This also bounds residency against a runaway or malicious
   output. The in-proc path (runtime produced the bytes) may skip it.

7. **The moving-GC handle-table apparatus does NOT transfer, and that is vehje's single biggest
   simplification.** V8 `Local`/`Persistent`/`HandleScope`, Lua's stack-by-index, JNI's
   critical-region, .NET's pinning (`GCHandle`/`fixed`) all exist because a *moving/compacting
   collector* relocates live objects, so the host can never hold a stable address. vehje's arena
   never moves; a stable offset is inherently non-dangling while the arena lives. Adopt the *contract*
   (address by index, borrow narrowly, explicit release) and refuse the *apparatus* (GC roots,
   barriers, finalizers, pinning). Corollary: do not introduce a moving value store.

8. **Copy is irreducible across disjoint memories; zero-copy exists only within one.** In-proc = shared
   address space = read in place, zero copy. Subprocess = disjoint = exactly one copy (the pipe
   write/read). Because the arena is index-based and relocatable, the SAME bytes are the in-proc live
   form and the wire form, so the subprocess path has no separate serialization step; it writes the
   arena it already built. Do not pretend the cross-process copy away; make it single and safe.

9. **Encoding-match kills copies; encoding-mismatch forces them, and vehje owns both ends.** V8 strings
   copy on every op because JS is UTF-8-vs-Latin-1; .NET's blittable ideal is "the fastest marshal is
   no marshal." The wasm Canonical ABI's elaborate per-type lift/lower exists ONLY to bridge two
   languages with different in-memory layouts. vehje's runtime and its host reader are one design over
   one arena format, so there is no cross-language layout gap: the value crosses as its own bytes, with
   no per-type marshalling protocol. A foreign-language host needs a reader for the one documented
   format, not an N-by-M marshalling matrix.

## Honest audit: where convergence could oversell

Unanimity across five sources is strong only if the escape valves are real. They are.

- **"Everything is the arena" is wrong for trivial outputs, and the design already exempts them.**
  Lesson 3 is the guard: a scalar result crosses by value, never wrapped in an arena. The arena is for
  *structured* outputs. A program returning `42` returns `42`.
- **Zero-copy in-proc read is not free: the host must speak the arena format.** This is a genuine
  API-surface cost the accessor model was trying to dodge. The answer the zero-copy family gives
  (rkyv `ArchivedT`, capnp reader views, Arrow readers) is a safe reader over the flat buffer, shipped
  once. So `vehje-runtime-driver` must ship a safe Rust reader over the value-arena, and a foreign
  host gets one documented format to read. This is a real deliverable, not a hand-wave, and it is
  strictly cheaper than N accessor FFI round-trips per value.
- **Pointer-chasing a tree is cache-unfriendly versus a linear copy** (the zero-copy family is explicit
  about this). Zero-copy wins when the host touches a small part of a large output or memory-maps it;
  a host that will linearly consume the whole thing once is exactly the streaming case, where the pipe
  copy is the right call anyway. The two-mode design (below) puts each on its best path.
- **Convergence-as-corroboration check.** The five did not share framing; they studied disjoint
  systems and independently reached a shape that also matches vehje's existing residual. That is
  triangulation, not shared drift. The one thing all five inherit from the brief is the vehje problem
  statement itself, which is fine (it is the question, not a leading answer).

## The concrete vehje design

### The payload: a value-arena, symmetric with the residual

The produced output is a **value-arena**: fixed-width value-node records + a flat child-index pool +
a self-contained byte blob, position-independent (child links are indices, blob refs are offset+len).
Structurally identical to the residual arena the runtime already consumes; the difference is the node
schema (value kinds, not IR kinds). Closed, versioned record layout, no vtables (lesson 9: vehje owns
both ends, so FlatBuffers/Cap'n Proto schema-evolution machinery is overkill). Tier-orthogonal: a
future bytecode or native tier emits the same value-arena a tier-0 interpreter does.

The node-kind schema itself (what value forms exist: unit, bool, int, the float question, bytes/string,
list, record, tagged/variant, and whether any lazy/thunk form is a node or is disallowed by the
effect proof) is the ONE genuinely open sub-question and belongs to op; it is the "what is in the
arena" axis, orthogonal to "how the arena crosses," which is what this study settled.

### The sink: a two-function reserve/commit contract (this is what the seed left vague)

The sink is not "the receiver figures it out." It is a fixed contract with exactly the
`IBufferWriter`/wasm-`stream.write` shape, mirrored across both boundaries:

- `reserve(hint) -> (ptr, cap)`: the sink lends the runtime a writable window of at least `hint` bytes
  (or as much as it can give). Blocks under backpressure until the consumer has drained enough.
- `commit(n)`: the runtime declares it wrote `n` bytes of value-arena into the current window.

The runtime serializes the value-arena depth-first (leaves before parents, the natural evaluation
order, so a parent records already-known child indices with no back-patching, per the FlatBuffers/rkyv
lesson) through reserve/commit. Only O(window) bytes are resident regardless of total value size
(lesson 5). In-proc, the sink is a `#[repr(C)]` struct of two fn pointers plus an opaque `userdata`
(FFI, not `dyn`; a wire shape at the C ABI boundary, compliant with the no-dyn framework rule and the
WorkUnit-mental-model "FFI structs are wire shapes" allowance). Out-of-proc, the sink is the exe's
stdout pipe: reserve/commit map to `write`s and the OS pipe buffer IS the backpressure (a full pipe
blocks the write, the direct analogue of wasm `stream.write` returning a blocked sentinel).

Under no_alloc this is not merely preferred, it is FORCED: the runtime cannot grow or own an output
heap, so it must write into memory the host lends. That inverts Arrow's "producer owns" to "host owns,"
which makes the release contract trivial (the host owns the bytes throughout; the runtime holds no
output memory after `_execute` returns).

### Two modes, one protocol

- **Whole-value mode (default when the output fits a residency budget):** the host lends one window
  large enough to hold the whole value-arena; the runtime fills it; the host reads the value-arena in
  place by index (zero-copy in-proc). This is the contracted retained-handle, fixed: the region is
  host-owned, the lease is "until the host is done reading," and there is no accessor chatter (the host
  reads the flat arena directly through the driver's reader).
- **Streaming mode (when the output exceeds the budget, or is consumed once linearly):** the host lends
  a small window repeatedly; the runtime streams value-arena chunks at safe subtree/record boundaries
  (each chunk independently walkable); the host drains each; backpressure bounds residency.

They are the same reserve/commit protocol; whole-value is streaming with a window big enough to need
only one reserve. This unifies the two originally-separate open questions: the exe's stdout and the
in-proc host-supplied sink are the same contract, and "the result" is whatever the sink received, whole
or chunked. There is no separate "result contract" versus "exe I/O" design; there is one sink.

### Ownership and release

Host owns the output bytes (it lent them). The lease is explicit and named: in whole-value mode the
runtime must not touch the region after `_execute` returns (the .NET `Memory<T>` Rule-3 lease); in
streaming mode the runtime touches only the current window between a `reserve` and its `commit`. No
`release` of runtime-held memory is needed because the runtime holds none; the in-proc borrow window
is enforced by Rust lifetimes in the driver (turning Lua's `lua_tolstring` runtime footgun into a
compile error), and the cross-process case is a plain owned buffer the host allocated.

### Validation

The subprocess reader (and any host reading bytes that crossed a process boundary) runs a
bounds-checked traversal (every child-index in range, every blob offset+len in range, depth capped)
before/while walking, per lesson 6. The in-proc trusted path may skip it. This is also the residency
guard against a runaway output.

### Both boundaries, one format

In-proc `.a` (path 2): scalars by value in return/out-params; structured output via the sink vtable;
whole-value mode gives zero-copy in-place read through the driver's reader. Subprocess exe (path 1):
the exe writes the value-arena to stdout through the pipe-sink; the host reads it back and validates.
Same bytes, same format, same node schema. Future tiers emit the same value-arena.

### The named deliverables this implies

1. A value-arena format spec + encoder (mirror of `wire.rs`/`encode.rs`, for values), in
   `vehje-runtime-abi`.
2. The `VehjeSink` `#[repr(C)]` reserve/commit contract in `vehje-runtime-abi`, plus the stdout-pipe
   sink for the exe.
3. A safe Rust reader over the value-arena in `vehje-runtime-driver` (the rkyv-`ArchivedT`/capnp-reader
   analogue), lifetime-scoped to the borrow.
4. A bounds-checked validation pass over the value-arena for the untrusted path.
5. The value-node-kind schema (op's call).

## The genuine remaining decisions for the together-session

1. **The value-node-kind schema.** What value forms exist in the arena. op's call; needs the "what can
   a vehje program produce" answer, which touches the effect model (does anything lazy/deferred survive
   into a runtime value, or does the effect proof guarantee fully-reduced values only).

2. **Whole-value region: host-lent up front, or runtime-scratch handed back.** Two shapes of the
   whole-value mode. (A) Host lends one big region, runtime fills it: airtight no_alloc, host owns
   throughout, but the host must size it without knowing the output size (over-provision, or fall to
   streaming when it overflows). (B) Runtime builds into its own construction-time scratch region and
   hands back a descriptor + release: host need not guess size, true zero-copy, but reintroduces a
   runtime-owned region and an Arrow-style release. Recommendation: (A) as the model, because it keeps
   the no_alloc/host-owns story airtight and streaming already handles the overflow case; (B) only if
   profiling shows the over-provision waste matters. Both are the same value-arena payload.

3. **Chunk boundary policy for streaming mode.** Chunk at whole-subtree boundaries (each chunk a
   self-contained mini value-arena, simplest to validate and walk, some fragmentation) versus a flat
   record-window with cross-chunk index references (denser, but the reader must buffer across chunks).
   Recommendation: whole-subtree chunks first (matches the depth-first leaves-first emission and keeps
   each chunk independently validatable), revisit only if fragmentation shows up in a bench.

Decisions 2 and 3 are bench-decidable if they ever become contentious (per the workspace's
bench-driven fork rule); 1 is op's design call.
