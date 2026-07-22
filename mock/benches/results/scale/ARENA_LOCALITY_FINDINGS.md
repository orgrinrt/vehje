# Arena locality: interpreter throughput vs backward-read window

**Strength: measurement** (CNTVCT_EL0, 5 runs median; the interp cross-validated against the fn-table interp
on each program). Ported from the old standalone Zig bench onto the carrier so it shares the IR and validation.

## The question

The arena layout can keep a value's operands physically near it (a tight backward-read window), which should
keep operand reads in cache-hot recently-written results. Does that help interpreter throughput? The workload
is a 4M-node carrier program whose operands are drawn from the last `window` nodes; `window` is swept.

## Result (4M-node program, switch interp)

| backward window | window bytes | ns/op | cyc/op |
|---|---|---|---|
| 2       | 16 B   | 10.01 | 32.0 |
| 128     | 1 KB   | 9.73  | 31.1 |
| 4096    | 32 KB  | 10.07 | 32.2 |
| 65536   | 512 KB | 9.68  | 31.0 |
| 262144  | 2 MB   | 9.85  | 31.5 |
| 1000000 | 8 MB   | 9.81  | 31.4 |
| 2000000 | 16 MB  | 11.41 | 36.5 |

## The finding

Interpreter throughput is INSENSITIVE to operand backward-read locality until the window exceeds the L2 cache,
and only modestly sensitive past it. From a 16-byte window to an 8 MB window (spanning the whole L2), ns/op is
flat at ~9.8 (31 cycles/op). Only at a 16 MB window (2M live values back, exceeding the M1's ~12 MB L2) does it
rise, and only by ~16% (to 11.4 ns/op), not the order of magnitude a naive "one DRAM miss per operand" would
predict.

The mechanism: the per-node cost is dominated by the sequential 24-byte wire read (prefetched) plus the fixed
decode, compute, and result write. The two operand loads per node execute in the shadow of that work, so
whether they hit L1 (tight window), L2 (up to 8 MB), or start missing to DRAM (past 12 MB), the out-of-order
engine hides most of the latency. The old standalone bench swept only to a 512-node window (all L1-resident)
and reported flatness; extending the window five orders of magnitude finds the L2 cliff it could not see, and
shows the cliff is shallow.

Design implication (op's call): the arena's operand-locality optimization buys at most ~16%, and only for
programs whose operand references span more than the L2 cache in live values (>1M nodes back). Real programs
reference recent values, so the interpreter gets essentially nothing from operand-locality tuning; the wire
stream is the cost. Do not over-invest in operand locality for the interpreter tier.

## Cost-model / boundary

9.8 ns/op is ~31 cycles/op for a 24-byte wire read, two operand loads, a compute, and a result write over a
96 MB wire past cache; memory-bound and physically consistent. Boundary: single interp shape (switch); the
locality axis is the program's operand window, not the layout stride (that is the record-width bench). The
cliff at 16 MB is where the live-value working set exceeds L2, which no realistic program's operand references
reach.
