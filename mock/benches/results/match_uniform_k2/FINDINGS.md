# Match lowering: arm-selection strategies, K=2 arms, uniform hits

Companion benches: `match_uniform_k2` / `k8` / `k64` (the arm-count sweep) and
`match_hot_k64` (the profile-hot-arm case).

## What this measures

The `Match` core form's arm selection. op folded Match into the core set precisely
because it does not lower to `If`s; the compiler owns the arm-selection strategy per
match. This bench races three strategies at K=2 arms with arbitrary sparse keys and
a uniform hit distribution (each arm equally likely):

- `ml_ifchain_u2`: linear if-chain in index order, O(K).
- `ml_jumptable_u2`: masked direct-index table, O(1). (Baseline.)
- `ml_tree_u2`: binary decision tree over sorted keys, O(log K).

The keys are arbitrary and sparse but carry distinct low-12-bit slots, so the jump
table cannot collide and every strategy returns the SAME body for a given scrutinee;
the harness cross-validates all three byte-for-byte. The scrutinee stream comes from
the FFI input and folds into the accumulator (no hoist). Arm keys/bodies/table are
built once via `OnceLock`; work scales as `N * REP` (REP = 64). The hot-first
if-chain is omitted here because under a uniform stream it degenerates to the plain
if-chain; it is measured in `match_hot_k64`.

## Measured results

Ratio to baseline (ml_jumptable_u2), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | ml_ifchain_u2 (ratio) | ml_jumptable_u2 (base) | ml_tree_u2 (ratio) |
|---|---|---|---|
| 64 | 1.02x | 4780 ns | 1.34x |
| 256 | 1.05x | 18075 ns | 1.33x |
| 1024 | 0.98x | 65246 ns | 1.31x |
| 4096 | 1.00x | 258562 ns | 1.30x |
| 16384 | 0.99x | 1021074 ns | 1.30x |

## Cost-model sanity line

At n=16384, the baseline (ml_jumptable_u2) median is 1021074 ns for N dispatches over 2 arms. Treating n as the work-item count, that is 62.32 ns/item, about 199.4 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

At 2 uniform arms, if-chain and jump-table tie; the tree costs 1.3x. Everything is cheap at k=2.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
