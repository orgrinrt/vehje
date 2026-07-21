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

## Measured results

Ratio to baseline (ei_thermo), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | ei_branchmax (ratio) | ei_thermo (base) |
|---|---|---|
| 64 | 5.67x | 1115 ns |
| 256 | 5.01x | 5026 ns |
| 1024 | 5.10x | 19325 ns |
| 4096 | 4.60x | 77911 ns |
| 16384 | 5.08x | 278129 ns |

## Cost-model sanity line

At n=16384, the baseline (ei_thermo) median is 278129 ns for thermo: N DAG nodes, one OR per interior node. Treating n as the work-item count, that is 16.98 ns/item, about 54.3 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

Thermometer inference (bitwise OR to accumulate grades up the DAG) is ~5x faster than branch-max (per-lane max via a family loop), stable across sizes. Less dramatic than inclusion (69x) because inference's per-node work is a single OR either way; the 5x is branch-max's family loop overhead, not a vectorization gap.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
