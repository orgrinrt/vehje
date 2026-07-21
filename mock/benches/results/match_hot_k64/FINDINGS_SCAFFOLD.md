# Match lowering: hot-first if-chain, K=64 arms, 90% hit one arm

Scaffold. Numbers are filled by the harness run (`vehje-benches match_hot_k64`).
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

## Expected result (from the design)

A hot arm changes everything for the if-chain. Plain if-chain is expected worst here
(it finds the hot arm last, ~K comparisons). Hot-first if-chain is expected to drop to
near the jump table (one comparison almost every time) and to beat the decision tree,
because the tree and table gain nothing from a hot arm (they do the same work
regardless). So the ordering should be roughly hot-first ~ jump-table << tree << plain
if-chain.

This settles the profile-hot branch of the cluster's lowering rule: a statically- or
profile-known hot arm lowers to a hot-first if-chain, which also handles guards inline
and is competitive with the jump table without needing dense keys.

## Cost-model sanity (fill from run)

Plain if-chain at K=64 under the hot stream is ~64 comparisons on 90% of matches;
hot-first is ~1. The gap between the two variants at the same K is the whole value of
hot-arm ordering. Times should scale close to linearly with `N * REP`.

## Boundary

The hot arm is fixed (arm K-1) and the bias is a flat 90/10; real profiles are noisier
and the arm may drift. Bodies are trivial (isolating dispatch). The finding is the
regime (hot arm -> reorder the chain), not the exact multiplier.
