# Closure representation: create-many-call-once (flat vs linked)

Scaffold. Numbers are filled by the harness run (`vehje-benches closure_create_many`).
Sibling bench: `closure_call_many` (the inverting usage pattern).

## What this measures

The `Lambda` + `Apply` closure representation fork under create-many-call-once: a
closure is created per iteration and invoked once, so CREATION cost dominates. This is
the `Iter`-pipeline-callback case (`map(|x| x + offset)`). Two variants race,
differing only in creation:

- `closure_create_many_flat`: each iteration builds the parent frame (ambient work,
  equal for both variants), then copies its 8 capture values into a fresh flat closure
  (the extra creation cost) and calls it once.
- `closure_create_many_linked`: each iteration builds the same parent frame, then
  creates the closure as a single pointer to it (no capture copy) and calls it once.

Both read the same value (`base[idx] ^ seed`), so the accumulator is
representation-independent and the harness cross-validates flat and linked
byte-for-byte. Frame and closure are `black_box`ed each iteration so the per-iteration
construction is real and cannot hoist; the FFI input folds into the captured values
and the accumulator. Work scales as `N * REP` (REP = 64).

## Expected result (from the design)

Linked wins: its creation is one pointer write (O(1)), while flat copies all captures
(O(capture-count)). Normalise baseline is `closure_create_many_flat`; linked should
show a negative (faster) delta. This is the exact sign FLIP of the linked delta in
`closure_call_many` (positive there), and the flip IS the inversion: the winner is
usage-pattern dependent, so the choice is per-closure, decided by escape. A
non-escaping closure (the common callback: consumed and dropped by the pipeline) can
point at the still-live parent frame, so it is linked (cheap creation, no copy).

## Cost-model sanity (fill from run)

Both variants pay the ambient frame construction (8 seeded xors). Flat adds an 8-wide
copy; linked adds one pointer store. The measured delta is that copy versus that
store. Times should scale close to linearly with `N * REP`.

## Boundary

Captures are scalar `i64` slots; a real non-escaping closure points at its enclosing
frame's live variables (modeled here as the ambient parent frame). The flat copy cost
scales with capture count; at very small capture counts the two converge. The escape
analysis that selects the representation is the design's reachability-binder rule, not
benched here; this sizes the creation cost each representation pays.
