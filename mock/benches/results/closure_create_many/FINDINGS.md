# Closure representation: create-many-call-once (flat vs linked)

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

## Measured results

Ratio to baseline (closure_create_many_linked), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | closure_create_many_flat (ratio) | closure_create_many_linked (base) |
|---|---|---|
| 64 | 2.08x | 5358 ns |
| 256 | 2.14x | 18602 ns |
| 1024 | 2.01x | 80232 ns |
| 4096 | 2.07x | 290679 ns |
| 16384 | 2.12x | 1162862 ns |

## Cost-model sanity line

At n=16384, the baseline (closure_create_many_linked) median is 1162862 ns for linked env: N creations, one alloc+link per creation. Treating n as the work-item count, that is 70.98 ns/item, about 227.1 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

Creation INVERTS the call result: a linked environment is 2.0x to 2.1x faster to create than a flat one, because flattening copies the captured set upfront. Create-heavy closures favor linked; call-heavy favor flat. Both directions of the tradeoff are now measured, not asserted.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
