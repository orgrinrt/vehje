# Match lowering: arm-selection strategies, K=64 arms, uniform hits

Scaffold. Numbers are filled by the harness run (`vehje-benches match_uniform_k64`).
Companion benches: `match_uniform_k2` / `k8` / `k64` (the arm-count sweep) and
`match_hot_k64` (the profile-hot-arm case).

## What this measures

The `Match` core form's arm selection. op folded Match into the core set precisely
because it does not lower to `If`s; the compiler owns the arm-selection strategy per
match. This bench races three strategies at K=64 arms with arbitrary sparse keys and
a uniform hit distribution (each arm equally likely):

- `ml_ifchain_u64`: linear if-chain in index order, O(K).
- `ml_jumptable_u64`: masked direct-index table, O(1). (Baseline.)
- `ml_tree_u64`: binary decision tree over sorted keys, O(log K).

The keys are arbitrary and sparse but carry distinct low-12-bit slots, so the jump
table cannot collide and every strategy returns the SAME body for a given scrutinee;
the harness cross-validates all three byte-for-byte. The scrutinee stream comes from
the FFI input and folds into the accumulator (no hoist). Arm keys/bodies/table are
built once via `OnceLock`; work scales as `N * REP` (REP = 64). The hot-first
if-chain is omitted here because under a uniform stream it degenerates to the plain
if-chain; it is measured in `match_hot_k64`.

## Expected result (from the design)

The three strategies show their textbook asymptotics. Jump table is flat and, being
O(1), is expected fastest for K >= 4 (this bench models it as always-available via a
masked table, so it wins even at K=2). If-chain grows linearly and is competitive only
at very few arms. Decision tree grows as O(log K) and is expected to beat the if-chain
from K=4 upward. So at K=2 the three are close; at K=8 and K=64 jump-table dominates
and tree pulls clearly ahead of the chain.

The per-node lowering rule the cluster settles: dense small-range discriminant ->
jump table; sparse many arms -> decision tree; sparse very few arms -> if-chain (its
niche is when keys are too sparse to table at all, which the always-masked jump table
here does not exercise); a profile-hot arm -> hot-first if-chain (`match_hot_k64`).

## Cost-model sanity (fill from run)

Per match: one input-derived arm select, a key load, and the strategy's comparisons
(chain ~K/2, tree ~log2 K, table 1). Times should scale close to linearly with
`N * REP`.

## Boundary

Single-scrutinee, equality-keyed arms with trivial bodies (isolating dispatch cost).
Guards and nested patterns compose as decision-tree nodes over this same per-node
choice, not a different mechanism. The jump table assumes maskable keys; a genuinely
sparse un-maskable key set is the decision tree's domain and is not separately raced.
