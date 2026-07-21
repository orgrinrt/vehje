# Effect inference: thermometer OR-join vs naive branch-max (scaffold)

Scaffold for `[bench.effect_inference]`. Fill the result table and the cost-model
line after the harness run.

## What it measures

The bottom-up effect-lattice join over a fixed node DAG, the load-bearing
inference in the certified-generation model (every construct carries an effect;
a node's effect is the join of its children's; the whole-program effect must be
included in the target's permits). Two variants compute the identical lattice:

- `ei_thermo`: thermometer grades (none=00, read=01, write=11) so the per-family
  lattice join is a single bitwise OR, and the graded bind is the same OR.
- `ei_branchmax`: the naive form, binary grade encoding (0/1/2) with a per-family
  max/compare loop over 24 two-bit lanes.

The DAG is generated once per (variant, size) subprocess from a compile-time
seed. The timed loop recomputes the join with an input-derived extra effect
folded into the root leaf, so it depends on the FFI input and does not hoist.
Both encodings decode to the identical canonical per-node grade total, so the
folded outputs are byte-identical and the harness cross-validates them. Baseline:
`ei_thermo`, mode subtract, so branch-max reports its excess over the OR.

## The defect this fixes

The old `effect.zig` probe ran standalone (hand-timed, `effect.csv` header-only)
with no cross-validated alternative: the "ordered lattice costs exactly a flat
bitwise op" claim rested on one hand-timed number, with no naive join measured
beside it and no proof the two forms compute the same lattice.

## The fix

Both encodings run under the harness over the identical DAG, cross-validated to
byte-identical output (proving they compute the same lattice), with the naive
per-family branch-max as the honest comparison. The delta isolates the
branch-free win.

## Result (fill after run)

| n | ei_thermo (baseline) | ei_branchmax (excess) |
|---|---|---|
| 64 | | |
| 256 | | |
| 1024 | | |
| 4096 | | |
| 16384 | | |

Cross-validation: expect pass.

## Cost-model sanity line (fill after run)

At n=16384, `ei_thermo` is <T> for 16384 nodes times 8 inference passes =
<ns>/node-join, about <cycles> cycles at 3 GHz for a child gather plus one OR
plus the grade-total fold. Confirm near-linear scaling with N, and state the
branch-max excess as the per-node cost of the 24-lane max loop the thermometer
encoding removes.
