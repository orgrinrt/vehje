# Match lowering: hot-first if-chain, K=64 arms, 90% hit one arm

Companion: `match_uniform_k2` / `k8` / `k64` (the uniform arm-count sweep).

## What this measures

The profile-hot-arm case for `Match` lowering: a K=64 match where 90% of scrutinees
hit ONE arm. Four strategies race, all returning the same body for a given scrutinee
(cross-validated byte-for-byte):

- `ml_ifchain_h64`: plain if-chain in index order. The hot arm is the LAST tested
  (arm K-1), so under the hot stream it pays ~K comparisons on 90% of matches.
- `ml_hotfirst_h64`: if-chain that tests the hot arm FIRST, then the rest. Under the
  hot stream it resolves on the first comparison 90% of the time.
- `ml_jumptable_h64`: masked O(1) table. (Baseline.)
- `ml_tree_h64`: binary decision tree, O(log K), indifferent to the hit distribution.

The 90/10 hot bias is baked into the shared arm-selection so all four see the same
scrutinee stream. Keys have distinct low-12-bit slots (no jump-table collision). The
FFI input folds into the stream and the accumulator; arms built once via `OnceLock`;
work scales as `N * REP` (REP = 64).

## Measured results

Ratio to baseline (ml_hotfirst_h64), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | ml_hotfirst_h64 (base) | ml_ifchain_h64 (ratio) | ml_jumptable_h64 (ratio) | ml_tree_h64 (ratio) |
|---|---|---|---|---|
| 64 | 4481 ns | 1.02x | 1.07x | 7.65x |
| 256 | 17686 ns | 1.00x | 1.00x | 7.21x |
| 1024 | 65051 ns | 1.00x | 0.98x | 7.41x |
| 4096 | 255176 ns | 1.00x | 1.02x | 7.59x |
| 16384 | 1011119 ns | 1.00x | 1.01x | 7.67x |

## Cost-model sanity line

At n=16384, the baseline (ml_hotfirst_h64) median is 1011119 ns for N dispatches, hot arm hit ~most of the time. Treating n as the work-item count, that is 61.71 ns/item, about 197.5 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

With a hot-skewed key distribution over 64 arms, hot-first, if-chain, and jump-table all tie (~1.0x): the branch predictor nails the hot arm regardless of structure. The balanced binary tree is 7.6x slower (its interior branches are the ones that mispredict). Hot-first ordering buys nothing over a plain if-chain when the predictor already learns the hot arm.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
