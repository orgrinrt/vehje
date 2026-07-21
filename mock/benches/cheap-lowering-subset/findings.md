# Cheap lowering subset (expansion): const-fold + CSE throughput + reduction

**Date:** 2026-07-21 | Zig 0.16.0 | data `lower.csv` | expands D6 / the three-way lowering seam.

## Result (redundancy-heavy IR, single no-alloc bottom-up pass: const-fold + CSE via open-addressing hash-cons)
- input 2,000,000 nodes -> output **484,338 nodes (75.8% reduction)**
- 59.5 ms = **33.6 M-nodes/s (29.7 ns/node)**

## Reading
The cheap runtime lowering subset (const-fold + common-subexpression-elimination as one bounded no-alloc pass,
the three-way-lowering-seam's load-time stage for arriving scripts) folds constants and deduplicates repeated
subexpressions, shrinking a redundancy-heavy IR by 76% at ~34 M-nodes/s. A realistic arriving script (thousands
of nodes) lowers in sub-millisecond, well within a load-time budget. The ~30 ns/node is dominated by the
hash-cons probes; a 2M-node input is the stress case, not a realistic per-script size.

The 76% reduction directly serves the authoring/templating consumers (the recenter's majority): their config /
document / template-shard IR is exactly the redundancy-heavy case (repeated shards, constant config, folded
build-time values), so the cheap lowering both shrinks what must be emitted AND folds the build-staged
computation. This is the concrete mechanism behind D6's "the cheap load-time const-fold+CSE subset directly
serves the templating consumers."

## Design impact
The cheap lowering subset (const-fold + CSE, single no-alloc hash-cons pass) is buildable and fast (~34 M-nodes/s,
76% reduction on redundant IR). It is the load-time compile stage for arriving scripts and the primary compile
benefit for the authoring/templating majority. The full equality-saturation lowering (SP6) stays dev-time (BN0);
this cheap subset ships in the runtime.
