# Closure representation: create-once-call-many (flat vs linked)

Scaffold. Numbers are filled by the harness run (`vehje-benches closure_call_many`).
Sibling bench: `closure_create_many` (the inverting usage pattern).

## What this measures

The `Lambda` + `Apply` closure representation fork under create-once-call-many: a
closure is built once (captures fixed) and invoked many times, so captured-variable
ACCESS cost dominates and creation amortises to nothing. Two variants race, differing
only in representation:

- `closure_call_many_flat`: captures copied into flat storage once; each call reads a
  slot by input-derived index (O(1) access).
- `closure_call_many_linked`: the closure points into a 4-frame environment chain;
  each call walks the chain to the root frame and reads the slot (O(depth) access).

Both hold the same capture values, so the accumulator is representation-independent
and the harness cross-validates flat and linked byte-for-byte. The FFI input drives
the read index and folds into the accumulator, so the access cannot hoist. State (env
+ chain) is built once via `OnceLock` and outside the timed loop; only the repeated
access is timed. Work scales as `N * REP` (REP = 64), a volume axis (the working set
is a handful of frames at every size, not a cache-regime axis).

## Expected result (from the design)

Flat wins or ties: O(1) flat access beats the O(depth) chain walk once depth exceeds
1 (chain depth here is 4). Normalise baseline is `closure_call_many_flat`; linked
should show a positive (slower) delta that grows with the walked depth.

Read alongside `closure_create_many`, where the winner INVERTS (linked wins, creation
dominates). The two benches together are the finding: representation is a consequence
of the escape verdict the reachability-binder / region analysis already produces, not
a free global choice. A call-many closure typically escapes, and an escaping closure
must copy captures into a promoted region anyway, so it is flat, and flat's O(1)
access is a bonus rather than a cost.

## Cost-model sanity (fill from run)

Per call: one input-derived index plus, for linked, up to 3 pointer hops and a load;
for flat, one load. Expect flat near a single dependent load and linked higher by the
per-hop chase latency times depth. Times should scale close to linearly with
`N * REP`, confirming the timed work is the access, not a hoisted constant.

## Boundary

Captures are scalar `i64` slots (a closure over a large record captures a handle). The
chain is a linear parent chain. The escape analysis itself is not benched here (it is
the design's reachability-binder rule); this sizes the runtime cost of each
representation the analysis selects between. The sweep does not reach a memory-bound
regime; the access-vs-creation inversion, not a cache effect, is the point.
