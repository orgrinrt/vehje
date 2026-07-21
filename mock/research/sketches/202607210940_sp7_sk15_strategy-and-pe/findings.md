# SP7 + SK15 findings: proof-directed strategy selection + PE-as-extraction

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifact `strat.zig`
**Settles:** SP7 (the graded evaluator portfolio) and SK15 (partial evaluation as the extraction/selection
preference).

## Result
A region carries a grade (straight-line? hot? stencil-permitted? static-predicate?), and a constant-time
selector picks the optimal evaluator from a bounded portfolio, no runtime search:
- straight-line arithmetic -> linear scan (the L1-hot forward scan; BN1 says switch dispatch)
- hot loop, stencil-permitted -> stencil native (copy-and-patch, ONLY where the grade proves it wins)
- hot loop, iOS (no stencil) -> linear scan (graceful: native is one strategy, absent here, the floor takes over)
- branchy region -> block transfer (CFG-of-blocks, SK17)
- static-predicate branch -> folded (SK15: PE folds the known predicate to one arm)
All five select correctly.

## Reading
- **SP7:** native/copy-and-patch is one strategy in a bounded portfolio, chosen in the regions where the grade
  proves it wins, not a ruling path. The selection is a constant-time grade dispatch (no search), so it is
  no-alloc and off the hot path. This is the recenter's "keystone-among-many" made structural, and it degrades
  gracefully on platforms that forbid the native strategy (iOS/consoles fall to the linear-scan floor).
- **SK15:** partial evaluation is the static-predicate-folds case of the same selection (a known predicate picks
  the `folded` strategy = residualise one arm), which in the e-graph (SP6) is the extraction cost preference for
  the maximally-residualised form. Confirmed as one coordinate of the graded selection.

## Design impact
Evaluation strategy (interpret / block-transfer / native-stencil / fold) is a graded selection per region,
constant-time, bounded portfolio, native chosen only where proven to win, degrading to the floor elsewhere. PE is
the fold case. This is the structural form of the recenter's native-is-a-ceiling-not-the-purpose stance.
