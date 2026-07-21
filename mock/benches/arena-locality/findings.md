# Arena locality sweep (expansion): interpreter throughput vs backward-read window

**Date:** 2026-07-21 | Zig 0.16.0 | data `loc.csv` | expands BN1/BN2 local-vs-random.

## Result (switch interp, 4M nodes, ns/op, median of 7)
| backward window | ns/op |
|---|---|
| 2 | 4.56 | 8 | 4.39 | 32 | 4.45 | 128 | 4.54 | 512 | 4.53 |
| 2048 | 4.54 | 16384 | 4.45 | 131072 | 4.40 | 1048576 | 4.47 |
| 4000000 (full random) | **5.99** |

## Reading
Throughput is FLAT (~4.4-4.5 ns/op) across backward windows from 2 up to 1,048,576, and only rises to 5.99
(+33%) when operand reads span the entire 4M-node (32MB) result array uniformly. The cache cliff is soft and far
out: temporal locality (recently-written results stay cache-warm) keeps even large windows fast; only genuinely
uniform reads across the whole large array fall out of L2/L3.

So the backward-local emission discipline (children-before-parents, SK17/SK20) gives a real but MODEST benefit
(~33% worst case), and the interpreter stays fast for moderately-non-local programs. The design's L1-hot cache
story holds but is not fragile: it does not require tight locality, only that value dependencies not be uniformly
spread across millions of nodes (rare). This refines BN1's local-vs-random gap (which was the full-random
extreme); intermediate localities are all fast.

## Design impact
The arena walk is robust to operand-read locality up to very large spans. Keep the backward-local emission (it is
free and gives the modest win), but the per-frame budget does not hinge on tight locality. A program with
uniformly-non-local value dependencies across millions of nodes pays ~33%, which is the realistic worst case.
