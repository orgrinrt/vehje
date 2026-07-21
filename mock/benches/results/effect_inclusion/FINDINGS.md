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

## Measured results

Ratio to baseline (eg_thermo), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | eg_branchmax (ratio) | eg_thermo (base) |
|---|---|---|
| 64 | 24.63x | 175 ns |
| 256 | 35.56x | 457 ns |
| 1024 | 41.34x | 1479 ns |
| 4096 | 62.04x | 5539 ns |
| 16384 | 69.43x | 23026 ns |

## Cost-model sanity line

At n=16384, the baseline (eg_thermo) median is 23026 ns for thermo: ITERS(16) x N effects, one AND+compare each. Treating n as the work-item count, that is 1.41 ns/item, about 4.5 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass (both encodings compute the identical inclusion predicate; accumulators agree).

## Verdict

Thermometer-encoded inclusion (one bitwise AND per effect) is 24x to 69x faster than branch-max (a per-family loop with a data-dependent early return), and the gap WIDENS with n. Cost-model attributes it: thermo streams at ~3.5 effective IPC (LLVM auto-vectorizes the data-parallel reduction) while branch-max runs ~24 scalar branchy family-ops per effect at ~1.2 IPC; 24x(3.5/1.2) reproduces the ~70x. Strong evidence for the thermometer encoding.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
