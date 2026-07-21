# Multi-shot enumeration throughput (expansion): SK2 perf

**Date:** 2026-07-21 | Zig 0.16.0 | data `ms.csv` | expands SK2.

## Result
| choice space | resumptions | ns/resumption | M-resume/s |
|---|---|---|---|
| 100 | 100 | ~0 | (below clock res) |
| 1,000 | 1,000 | 2.00 | 500 |
| 8,000 | 8,000 | 2.25 | 444 |
| 65,536 | 65,536 | 2.47 | 405 |
| 262,144 | 262,144 | 2.90 | 345 |

## Reading
The bounded multi-shot handler enumerates its choice space at ~2-2.9 ns per resumption (400-500 M-resumptions/s),
roughly constant, rising slightly with space size (cache effects on the larger budget array). Each resumption is
the continuation body plus a mixed-radix increment. The enumeration OVERHEAD (the increment + dispatch) is ~2.5
ns and negligible; a real continuation body would dominate. So bounded multi-shot (SK2) is not just feasible
no-alloc but cheap: a pattern with thousands of combinations resolves in microseconds.

## Design impact
Bounded multi-shot handlers (CR1) are fast as well as no-alloc: ~2.5 ns/resumption enumeration overhead, so the
cost is essentially the continuation body times the (statically-bounded, budget-fit-checked) number of
resumptions. The Handle form's multi-shot support is cheap for the deterministic bounded patterns it targets.
