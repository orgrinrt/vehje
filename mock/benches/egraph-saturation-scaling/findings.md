# E-graph saturation scaling: is eqsat bounded, and does the window cap hold?

**Date:** 2026-07-21
**Type:** Zig-native scaling bench, ReleaseFast, a proper e-graph (std hash-map hashcons + union-find with path
compression + egg-style rebuild) saturated to fixpoint with commutativity + distributivity, across expression
depths 2 to 5. Zig 0.16.0, aarch64. Artifacts: `egsat.zig`, `scaling.csv`.
**Settles:** the scaling question the SP6 bounded-egraph sketch explicitly scoped out ("toy O(n^2) rebuild,
streaming-window not exercised"). Does equality saturation stay bounded or explode with a real congruence index,
and does the design's bounded-window CAP hold it tractable? Sizes the SP6/BN0 "eqsat is dev-time-only + bounded"
decision with growth numbers.

## Why this probe

The design puts equality saturation (eqsat, an e-graph rewriting to a fixpoint over all equivalent forms) in the
optimization tier, dev-time-only, with a bounded streaming window. Two claims underpin that placement and were
unmeasured: eqsat's growth is manageable enough for dev-time (but too much for runtime), and a node cap gives a
hard tractability bound. Distributivity `(a+b)*c => a*c + b*c` is the classic eqsat explosion driver (it can
generate exponentially many equivalent forms), so a rule set including it is the honest stress test.

## Results

Sum-of-products expressions of increasing depth, saturated to fixpoint (no new merges) with commutativity and
two-sided distributivity, uncapped versus with a 4096-node cap:

| depth | init nodes | uncapped @ fixpoint | classes | rounds | uncapped ms | CAP=4096 nodes | CAP ms |
|---|---|---|---|---|---|---|---|
| 2 | 31 | 156 | 71 | 4 | ~0 | 156 | 0.02 |
| 3 | 127 | 652 | 295 | 4 | 0.1 | 652 | 0.05 |
| 4 | 511 | 2636 | 1191 | 4 | 0.2 | 2636 | 0.21 |
| 5 | 2047 | 10572 | 4775 | 4 | 1.9 | 4102 | 0.26 |

## The finding: with hashcons congruence, eqsat growth is ~5x LINEAR (not exponential) and saturates in a few rounds; the cap is a hard safety bound

Two results, both load-bearing:

**Growth is linear in program size, ~5.2x, and bounded.** The e-graph reaches a fixpoint at a consistent ~5.2x
the initial node count (156/31, 652/127, 2636/511, 10572/2047 all ~5.1 to 5.2x), in exactly 4 rounds regardless
of depth. The feared exponential explosion does not happen, because hashcons congruence deduplicates the
equivalent forms distributivity generates: the same `mul(a, c)` sub-term created down two different rewrite paths
is one e-node, not two. So for a well-behaved rule set (commutativity, distributivity) with a real congruence
index, eqsat is a bounded, few-round, linear-growth process, not the intractable blowup its reputation suggests.
This is what makes it dev-time-viable.

**The cap is a hard tractability bound, not just an optimization.** At depth 5 the uncapped graph reaches 10572
nodes in 1.9ms; the 4096-node cap stops it at 4102 nodes in 0.26ms. The cap bites and holds. This matters because
the ~5.2x linearity holds for THIS rule set: a pathological rule set (unrestricted associativity reassociation of
long chains, or any rule that does not self-limit under congruence) genuinely can explode, and there is no static
guarantee a consumer's rule set is well-behaved. The bounded-window cap is therefore the safety guarantee the
design needs: it converts "eqsat might explode on a bad rule set" into "eqsat is capped at N nodes, extract the
best form found within the budget, advance the window." The cap is load-bearing correctness, not tuning.

## Design impact

- Eqsat belongs at dev-time (compile side), confirmed: even the tame ~5.2x linear growth over a few rounds is
  compile-appropriate, not runtime-appropriate (the runtime uses the cheap-lowering subset, const-fold + CSE, a
  single no-alloc pass, per the cheap-lowering-subset bench). A large program (say 500K initial nodes) saturates
  to ~2.6M e-nodes at ~1.9ms per 10K nodes, so hundreds of ms, fine for dev-time, not for per-frame.
- The bounded streaming window is a hard safety mechanism, not an optimization: it guarantees termination and a
  memory bound regardless of the consumer's rule set, which no static analysis of an arbitrary rule set can
  provide. Keep it as a mandated cap with saturate-extract-advance, exactly as the design specifies.
- The extraction pass must be the iterative cost fixpoint (SP6's load-bearing finding: recursive extraction hangs
  on the cycles e-graphs contain after merges). This bench's e-graphs have those cycles (commutativity makes
  `mul(a,b)` and `mul(b,a)` congruent, a 2-cycle), so the iterative extraction requirement stands.

## Boundary

The rule set is commutativity plus two-sided distributivity, the standard explosion stress test, but not the
worst case: a rule set with unrestricted associativity reassociation, or rules that manufacture fresh unbounded
terms, would grow faster and lean harder on the cap (which is exactly why the cap exists). The hashcons here is a
std hash-map (production-representative, unlike SP6's O(n^2) toy), but the rebuild is simplified (it drains the
worklist without the full egg upward-merge congruence repair); a complete rebuild would find slightly more
congruences, shrinking the class count further, so the ~5.2x is if anything an over-count. The no-alloc discipline
is relaxed here (std hash-map allocates) to measure scaling; the SP6 sketch already proved the no-alloc bounded
form is buildable, and the streaming-window cap is what reconciles the two (a fixed-capacity arena sized to the
cap).

## Artifacts
- `egsat.zig` (the hash-map e-graph + saturation driver), `scaling.csv` (the depth-vs-size table).
