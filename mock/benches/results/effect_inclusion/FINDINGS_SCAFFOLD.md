# Effect inclusion gate: thermometer subset test vs naive per-family compare (scaffold)

Scaffold for `[bench.effect_inclusion]`. Fill the result table and cost-model
line after the harness run.

## What it measures

The whole-program inclusion gate: scan every node's effect against the target
permit-set and count violations. This is the certified-generation correctness
boundary (reject any construct whose effect exceeds the target's permits). Two
variants count identical violations:

- `eg_thermo`: inclusion is the subset test `(script & ~target) == 0`, one
  AND-NOT plus a compare per node, branch-free.
- `eg_branchmax`: the naive per-family form, a loop comparing each of 24 lane
  grades against the target's.

Effects are computed once per subprocess (from the fixed DAG) into the variant's
encoding, so the timed loop runs only the inclusion scan. Each iteration
perturbs one target family from the FFI input, so the scan is not loop-invariant.
The thermometer subset test equals per-family `grade_script <= grade_target`, so
violation counts are identical and outputs byte-identical (cross-validated).
Baseline: `eg_thermo`, mode subtract.

## The defect this fixes

The old `effect.zig` probe hand-timed one thermometer-only inclusion scan outside
the harness (header-only CSV), so "the inclusion gate is essentially free" had no
reproducible number and no naive per-family comparison showing what the subset
test saves.

## The fix

Both forms run under the harness over the identical effect array, cross-validated
to identical violation counts, with the naive per-family compare as the honest
baseline. The delta isolates the branch-free saving.

## Result (fill after run)

| n | eg_thermo (baseline) | eg_branchmax (excess) |
|---|---|---|
| 64 | | |
| 256 | | |
| 1024 | | |
| 4096 | | |
| 16384 | | |

Cross-validation: expect pass.

## Cost-model sanity line (fill after run)

At n=16384, `eg_thermo` is <T> for 16384 nodes times 16 inclusion passes =
<ns>/node, about <cycles> cycles for one AND-NOT plus a compare plus the count
fold. State the per-whole-program inclusion-proof cost and confirm the branch-max
excess is the 24-lane comparison loop the subset test collapses.
