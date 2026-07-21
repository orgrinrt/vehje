# Match lowering: arm-selection strategies, K=8 arms, uniform hits

Companion benches: `match_uniform_k2` / `k8` / `k64` (the arm-count sweep) and
`match_hot_k64` (the profile-hot-arm case).

## What this measures

The `Match` core form's arm selection. op folded Match into the core set precisely
because it does not lower to `If`s; the compiler owns the arm-selection strategy per
match. This bench races three strategies at K=8 arms with arbitrary sparse keys and
a uniform hit distribution (each arm equally likely):

- `ml_ifchain_u8`: linear if-chain in index order, O(K).
- `ml_jumptable_u8`: masked direct-index table, O(1). (Baseline.)
- `ml_tree_u8`: binary decision tree over sorted keys, O(log K).

The keys are arbitrary and sparse but carry distinct low-12-bit slots, so the jump
table cannot collide and every strategy returns the SAME body for a given scrutinee;
the harness cross-validates all three byte-for-byte. The scrutinee stream comes from
the FFI input and folds into the accumulator (no hoist). Arm keys/bodies/table are
built once via `OnceLock`; work scales as `N * REP` (REP = 64). The hot-first
if-chain is omitted here because under a uniform stream it degenerates to the plain
if-chain; it is measured in `match_hot_k64`.

## Measured results

Ratio to baseline (ml_jumptable_u8), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | ml_ifchain_u8 (ratio) | ml_jumptable_u8 (base) | ml_tree_u8 (ratio) |
|---|---|---|---|
| 64 | 1.00x | 4754 ns | 3.18x |
| 256 | 1.05x | 16020 ns | 3.40x |
| 1024 | 0.99x | 64784 ns | 3.18x |
| 4096 | 1.01x | 256342 ns | 3.20x |
| 16384 | 1.00x | 1021627 ns | 3.25x |

## Cost-model sanity line

At n=16384, the baseline (ml_jumptable_u8) median is 1021627 ns for N dispatches over 8 arms. Treating n as the work-item count, that is 62.36 ns/item, about 199.5 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

At 8 uniform arms, if-chain and jump-table still tie; the tree costs 3.2x. The jump-table's indirect branch does not beat the if-chain even at k=8.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
