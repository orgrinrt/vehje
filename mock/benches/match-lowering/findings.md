# Match lowering strategy: if-chain vs jump-table vs decision-tree

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 50M matches, arm counts 2 to 64, arbitrary sparse keys, two hit
distributions, 5-run best. Zig 0.16.0, aarch64. Artifacts: `matchbench.zig`, `lowering.csv`.
**Settles:** the `Match` core form's lowering strategy. op folded Match into the core set precisely because it
does not translate to `If`s properly and must lower on its own; this bench gives the compiler the concrete
per-node lowering rule (which arm-selection strategy, by arm count and key density).

## Why this probe

`Match` is a core form (not sugar over `If`), so the compiler owns its lowering. Arm selection has three classic
strategies with different asymptotics, and which one a given `match` should compile to depends on arm count, key
density, and the hit distribution. Getting this rule right matters because pattern matching is pervasive in real
scripts (dispatching on a value's kind/tag). The three strategies:

- **Linear if-chain:** compare the scrutinee against each arm's key in order. O(arms) worst case, but O(1) when an
  early arm hits, and trivially handles guards and arbitrary keys.
- **Jump table:** index the arm directly by the scrutinee (dense small-range keys, e.g. an enum tag). O(1)
  regardless of arm count, but needs dense keys or the table is huge and holey.
- **Binary decision tree:** binary-search sorted keys. O(log arms), the right fallback for many sparse arms where
  a jump table would be too large.

The arm keys here are arbitrary and sparse (an earlier version had bodies that were a closed form of the arm
index, which LLVM strength-reduced the entire if-chain scan into a direct computation, giving a false flat
result; real match arms have arbitrary keys and bodies, which this version models).

## Results (ns per match)

| K arms | if-chain | jump-table | decision-tree |
|---|---|---|---|
| **uniform hits** | | | |
| 2 | 0.79 | 0.51 | 0.96 |
| 4 | 1.79 | 0.50 | 1.34 |
| 8 | 3.05 | 0.51 | 1.80 |
| 16 | 6.17 | 0.51 | 2.31 |
| 32 | 9.99 | 0.51 | 3.35 |
| 64 | 16.33 | 0.51 | 5.35 |
| **hot-first (90% arm 0)** | | | |
| 8 | 0.60 | 0.51 | 2.77 |
| 32 | 1.08 | 0.51 | 5.13 |
| 64 | 2.14 | 0.51 | 6.48 |

## The finding: a clean per-match lowering rule falls out

The three strategies have exactly their textbook asymptotics, and the crossovers are sharp:

- **Jump table is flat at 0.51 ns, O(1), and wins for any K >= 4 when keys are dense.** This is the enum/tag-match
  common case (matching on a value's kind), and it is the fastest strategy by a wide margin at scale. When the
  scrutinee is a small-range discriminant, lower to a jump table, full stop.
- **If-chain grows linearly (O(K/2) average): 0.79 ns at K=2 up to 16.3 ns at K=64.** It wins only at very few
  arms (K<=3, where it beats the decision tree's setup) or when a hot arm dominates.
- **Decision tree grows as O(log K): 0.96 ns at K=2 up to 5.35 ns at K=64.** It beats the if-chain from K=4 upward
  and is the correct fallback for many SPARSE arms (keys too wide for a jump table).
- **A hot arm changes everything for the if-chain.** With 90% of matches hitting the first-tested arm, the
  if-chain drops to 0.60 ns at K=8 (competitive with the jump table) and stays cheap even at K=64 (2.14 ns, versus
  16.3 ns uniform), because the hit is found on the first comparison almost every time. The decision tree and jump
  table gain nothing from a hot arm (they do the same work regardless), so a known-hot arm is the if-chain's
  domain.

### The lowering rule for the compiler

For each match (or each node of a nested-pattern decision tree):

1. **Dense keys (small-range discriminant / enum tag):** jump table. The common tag-match case, O(1), fastest at
   any arm count >= 4.
2. **A profile-known or statically-obvious hot arm:** if-chain with the hot arm tested first. Competitive with the
   jump table and better than the tree, and it handles guards inline.
3. **Sparse keys, few arms (K <= 3):** if-chain (no setup cost, wins at tiny K).
4. **Sparse keys, many arms (K >= 4):** binary decision tree. O(log K), the fallback when a jump table would be
   too large or holey.

## Design impact

- The `Match` lowering is not a single strategy; it is this per-node choice driven by key density, arm count, and
  hot-arm knowledge. The compiler picks per match. This is the Maranget-style pattern-matrix compilation applied
  to vehje's Core: build a decision tree over the scrutinees, and at each node select jump-table / if-chain /
  binary-tree by the rule above.
- Guards (arm conditions beyond the key) force a local if-chain within a matched bucket (test the key with the
  chosen strategy, then the guard sequentially); nested patterns (matching sub-structure) compile to a decision
  tree over multiple scrutinees, each node using this same per-node strategy choice. The bench validates the
  per-node arm-selection cost that the full algorithm composes.
- The jump-table's flat 0.51 ns confirms that tag-dispatch (the overwhelmingly common match shape: match on a
  value's kind) is essentially free, which is what makes a kind-tagged value model (the NaN-box tag, the record
  discriminant) cheap to branch on.

## Boundary

Single-scrutinee matches with equality-keyed arms; real matches add guards and nested/structural patterns, which
compose as decision-tree nodes over this per-node choice (noted above), not a different mechanism. The jump-table
variant assumes dense-enough keys to index directly; a sparse-key jump table would need a perfect-hash or a
two-level table (a known technique, more setup, still O(1) lookup), which sits between the direct table and the
decision tree and is worth it only for large sparse arm sets that are hot. Arm bodies here are trivial (isolating
dispatch cost); real bodies dominate for expensive arms, at which point the selection strategy matters less.

## Artifacts
- `matchbench.zig` (the three strategies over arbitrary sparse keys, uniform and hot-first distributions),
  `lowering.csv`.
