# Record-width bandwidth past L2: the regime the harness cap excluded

**Strength: measurement** (CNTVCT_EL0, 5 runs median; all layouts cross-validated to the identical checksum).

## The question

In the harness (`carrier_record_width`, L1/L2-resident) all wire layouts 12-32B tied, and 16B was a live
contender against the specced 24B. The audit noted the bandwidth regime past L2 was owed: does a wider stride
lose to memory bandwidth once the wire exceeds cache? This interprets a 1M and 4M-node program at each layout
(wire 12.7 MB to 128 MB, well past the M1's ~12 MB L2), all interpreting the identical program and folding the
identical checksum.

## Result

| n | layout | stride | wire | ns/node | GiB/s streamed |
|---|---|---|---|---|---|
| 1M | rec12 | 12B | 12.7 MB | 10.52 | 1.1 |
| 1M | rec16 | 16B | 16.0 MB | 9.70  | 1.5 |
| 1M | rec24 | 24B | 24.0 MB | 9.69  | 2.3 |
| 1M | rec32 | 32B | 32.0 MB | 9.91  | 3.0 |
| 4M | rec12 | 12B | 50.8 MB | 10.29 | 1.1 |
| 4M | rec16 | 16B | 64.0 MB | 10.38 | 1.4 |
| 4M | rec24 | 24B | 96.0 MB | 9.84  | 2.3 |
| 4M | rec32 | 32B | 128.0 MB | 9.95 | 3.0 |

All layouts tie at ~10 ns/node regardless of stride, even at 128 MB (10x the L2). The GiB/s figure scales
directly with stride (1.1 to 3.0), confirming wider layouts do stream proportionally more bytes, but the TIME
per node does not move.

## The finding

Record width is not the interpreter's bottleneck at any scale. The interp is latency-bound, not
bandwidth-bound: each node depends on earlier results (a dependency chain), and its operands are read from the
results array within a 64-node locality window, so they stay L1-hot. The per-node cost (~10 ns, ~32 cycles) is
set by that dependency latency plus the decode and the op, not by streaming the wire. The wire is read
sequentially and the hardware prefetcher hides its latency; at 2.3 GiB/s for rec24, the interp uses about 3% of
the M1's ~68 GB/s, nowhere near bandwidth-bound.

So the harness result extends to scale, and more strongly: 12B through 32B tie in time even past L2, and the
wider layouts merely waste memory bandwidth headroom for no speed benefit. The "24 bytes precisely optimal"
claim has no support at any scale for this workload; 16B (the true 3-operand layout) is fully viable and reads
less memory. The wire format can go to 16 bytes.

## Boundary

This is specific to the dependent-chain interpreter with good operand locality. A width-sensitive regime would
be a bandwidth-saturated pass, many parallel interpreters contending for DRAM, or a poor-locality workload
whose operand reads miss cache, where the results-array footprint (not the wire stride) would dominate. None of
those is the interpreter's regime here. When the real runtime lands with its actual access pattern, re-run this
before finalizing the wire stride, but nothing in the interpreter's behavior favors a wider record.
