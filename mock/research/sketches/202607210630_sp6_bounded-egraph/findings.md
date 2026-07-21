# SP6 findings: no-alloc bounded equality-saturation e-graph

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifact `egraph.zig`
**Settles:** 2001 original-work item 1 (the no-alloc bounded streaming eqsat e-graph), the biggest flagged
novelty. Feasibility confirmed on the pinned toolchain.

## Result
A fixed-capacity (CAP=256, no heap) e-graph with union-find over e-classes, hash-cons congruence, rewrite
saturation, and min-cost extraction:
- **Saturation:** building `((x + 0) * 1)` and applying `(add ?a 0)->?a`, `(mul ?a 1)->?a` with a rebuild
  (re-canonicalise + union congruent) reaches fixpoint in **2 rounds**, and `find(root) == find(x)` is true, so
  the rewrites proved `(x+0)*1 == x` via congruence closure.
- **Extraction:** an iterative cost fixpoint over classes extracts root's min-cost representative: cost 1, op
  `vr` = `x`. Correct saturate-then-extract, no heap.

## Load-bearing implementation finding: extraction must be an iterative fixpoint, not recursive
A recursive cost/extraction over the e-graph HANGS (it timed out at 2 minutes). E-graphs contain cycles after
merges (a class can hold an e-node referencing that class), so recursive extraction loops or explores
exponentially even with a depth guard. The fix is the standard egg-style approach: compute class costs as an
ITERATIVE FIXPOINT (cost[cl] = min over its e-nodes of 1 + sum child costs, iterated to stability, cyclic
e-nodes never win because they cost more than the base case), then follow the min-cost e-node per class. This is
cycle-safe and terminates fast (2 passes here). The design's extraction pass must be the iterative fixpoint form.

## Scope (honest)
Minimal e-graph: linear hash-cons (O(n) per add) and O(n^2) rebuild, fine for a bounded window but a production
version needs the hash-table congruence index and the slotted + colored extensions (binders + graded context,
2055). The streaming-window discipline (process a bounded region, saturate, extract, advance) is not exercised
here; this proves the core saturate-then-extract is no-alloc-buildable and bounded. The graded well-founded PE
unfold (terminating by binding-time grade) is the additional layer over this base.

## Design impact
The no-alloc bounded e-graph is feasible; the compile-stage lowering (macro-expand + const-fold + CSE + algebraic
identities as one saturate-then-extract) can be built no-alloc. Extraction is an iterative cost fixpoint (not
recursive). The slotted/colored/graded extensions and the streaming window are the remaining engineering over
this confirmed base. Per BN0, the full e-graph stays dev-time (its cost is superlinear); the cheap runtime
const-fold+CSE subset is a bounded single pass.
