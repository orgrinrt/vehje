# BN2 findings: node record width (24-byte inline beats 16-byte + spill)

**Date:** 2026-07-21
**Type:** runtime bench, Zig-native, isolated ReleaseFast binaries, 4M-node program, 9-pass median, monotonic
clock, backward-local (L1-hot) operand reads (the design's forward-scan property). Zig 0.16.0, aarch64.
**Settles:** the wire-format node record width (Cluster B/C provisional 16 vs 24 bytes), and flips the lean.
**Data:** `recwidth.csv`.

## Results (ns per op, median)

| call fraction | rec16 (16B, 2 inline + pool spill for arity>2) | rec24 (24B, 3 inline) |
|---|---|---|
| 5% | 4.43 | **3.82** |
| 20% | 5.31 | **4.22** |
| 40% | 5.65 | **4.28** |

Same logical program laid out both ways; both walk it identically except for the record layout and the arity>2
handling (16B spills all call operands to a `[]u32` pool and loops over `arity`; 24B inlines the three operands
and does a branchless fixed add).

## The finding: go wider (24 bytes)

The 24-byte record is faster at EVERY call fraction, including 5% where the arithmetic (arity-2) case dominates
and the 16-byte record's better node-array density (4 nodes per 64B line vs ~2.7) was expected to win. It does
not, for two reasons:

- The node array is streamed strictly front-to-back, so the hardware prefetcher absorbs almost all of the
  wider record's density penalty; both layouts stream from memory and the extra 8 bytes per node cost little on
  a sequential walk.
- The 16-byte record pays a real per-call penalty that the 24-byte record avoids entirely: a pool base
  indirection plus a variable-length `while (k < arity)` loop plus (for call-heavy programs) cache-cold pool
  reads (at 40% calls the pool is ~19MB). Inlining the third operand removes the indirection, the loop, and the
  pool miss. The gap widens with call fraction (16B 4.43->5.65 as calls go 5->40%; 24B 3.82->4.28), exactly the
  cost of the spill.

So the provisional Cluster B/C lean toward 16 bytes (which assumed arity<=2 dominance and node-density savings)
is flipped by measurement: **use the 24-byte record with three inline operands.** Inlining the common call arity
(3) and avoiding the spill beats the density savings even in the arithmetic-heavy regime.

## Caveat and boundary

This is the design-representative regime: backward-local (L1-hot) operand reads on a sequentially-streamed node
array. If operand reads were fully random (cache-bound on `res[]`), node-array density would matter more and the
balance could shift; and an IR with essentially no arity>2 ops at all would make the two equal (24B just wastes
8 bytes). But for any realistic mix with calls/record-construction, 24 bytes wins. The wire format should fix the
record at 24 bytes, three inline operands, arity>3 spilling to the child-index pool.

## Artifacts
- `common_rw.zig` (shared logical-program gen), `rec16.zig` / `rec24.zig` (the two layouts), `recwidth.csv`.

## Addendum: 32-byte (4 inline) variant confirms 24 is optimal
Adding rec32 (32 bytes, 4 inline operands) to the sweep:
| call fraction | rec16 | rec24 | rec32 |
|---|---|---|---|
| 5% | 4.43 | 3.82 | 3.82 |
| 20% | 5.31 | 4.22 | 4.22 |
| 40% | 5.65 | 4.28 | 4.31 |

rec32 is IDENTICAL to rec24 (and marginally slower at 40% due to the density cost). The 4th inline operand buys
nothing because arity>=4 is only 1.3% of calls (SP2), so inlining it just wastes 8 bytes/node. The record-width
decision is thus precise: **24 bytes (3 inline operands) is optimal**. 16 is worse (pool spill on arity-3 calls),
32 is no better (the 4th operand is unused density). Lock the wire format at 24 bytes, 3 inline operands, arity>3
spilling to the child-index pool.
