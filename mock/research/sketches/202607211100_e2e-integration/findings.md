# End-to-end integration (capstone): the pieces compose

**Date:** 2026-07-21 | **Outcome:** WORKS (composition confirmed) | Zig 0.16.0 | artifact `e2e.zig`
**Settles:** that the validated pieces COMPOSE into a working load+run pipeline, and the full-pipeline cost.

## Result (50,000-node script through the full pipeline)
| stage | time | note |
|---|---|---|
| 1 lease inference (reach bitmask fixpoint) | 1488 us | compile-stage |
| 2 cheap lowering (const-fold + CSE) | 1752 us | 50000 -> 16738 nodes, 67% reduction |
| 3 interpretation (switch dispatch) | 90 us | on the lowered (smaller) IR |
| 4 value-arena emit | 25 us | |
| 5 typed validate | 12 us | ok=true |
| **TOTAL load+run** | **3367 us** | 67 us / 1k-nodes |

## Reading
The full pipeline (lease inference -> cheap lowering -> interpretation -> value-arena emission -> typed validate)
composes and runs end to end. A 50k-node script (a substantial mod) loads and runs in 3.4 ms; a realistic mod
script of a few thousand nodes is sub-millisecond. With the incremental caching (D6/BN3), unchanged mods in a
stack are cache hits, so a large stack loads in reasonable time.

The cost profile matches op's characterization of the authoring/templating majority exactly: the COMPILE stage
(lease + lower = 3.24 ms of the 3.37 ms) dominates, and the RUNTIME (interp + emit + validate = 127 us) is cheap.
These consumers are compile-stage-heavy with a trivial execute stage, which is what the whole architecture is
shaped for. Each stage's per-node cost is consistent with its standalone bench (lease ~30 ns/node, lowering
~35 ns/node, interp cheap on the reduced IR), so the pieces compose with predictable, additive costs.

## Design impact
The architecture is not just feasible piece-by-piece, it composes into a working pipeline with a realistic
sub-millisecond-per-script cost. The compile-stage-heavy / cheap-runtime profile is confirmed. This is the
strongest single validation that the design holds as a whole.
