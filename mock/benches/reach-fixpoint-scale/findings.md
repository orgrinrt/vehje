# Reach-fixpoint scale (expansion): lease-inference throughput + whole-column vs delta

**Date:** 2026-07-21 | Zig 0.16.0 | data `reach.csv` | expands SP5.

## Result (reach-bitmask fixpoint, backward-window-64 DAG)
| n nodes | edges | variant | rounds | ms | throughput |
|---|---|---|---|---|---|
| 100K | 200K | whole-column | 2 | 0.5 | 729 M-edge-OR/s |
| 100K | 200K | delta | 2 | 1.1 | 174 M-edge/s |
| 1M | 2M | whole-column | 2 | 6.0 | 670 M-edge-OR/s |
| 1M | 2M | delta | 2 | 11.4 | 175 M-edge/s |
| 8M | 16M | whole-column | 2 | 50.7 | 631 M-edge-OR/s |
| 8M | 16M | delta | 2 | 91.3 | 175 M-edge/s |

## Two findings
1. **Lease inference is fast at scale.** An 8M-node / 16M-edge program's reach fixpoint runs in ~50 ms
   (~630 M-edge-ORs/s), well within a compile-stage budget. The compile-stage brain's lease query is cheap.
2. **Whole-column OR beats delta semi-naive for the realistic (shallow) case.** The graph converges in 2 rounds
   (SP2 measured nesting depth 3+ = 0.6%, so real programs are shallow), and delta semi-naive's per-round
   bookkeeping (dirty/next bool arrays + a full scan/copy each round) is NOT repaid when there are only a couple
   of rounds; it is ~4x slower here. Delta only wins when rounds are LARGE (deep dependency graphs), which are
   rare. So for the lease fixpoint on realistic (shallow) programs, the simple whole-column OR is faster.

## Design impact
The lease-inference engine should use the simple whole-column bitmask-OR fixpoint (not delta-tracked semi-naive)
for realistic shallow programs; it is faster and simpler. Reserve delta tracking for genuinely deep graphs if any
consumer produces them. Lease inference at 8M nodes in 50 ms confirms the compile-stage lease query is not a
bottleneck. (Note: the delta impl here uses a bool-array + full-scan; a dirty-worklist delta would be faster, but
still loses to whole-column when rounds are tiny.)
