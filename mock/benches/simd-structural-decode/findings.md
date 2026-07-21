# BN4 findings: SIMD structural decode throughput

**Date:** 2026-07-21 | **Type:** runtime bench, Zig-native | Zig 0.16.0 | data `simd.csv`
**Settles:** the simdjson-style SIMD structural pass throughput claim (2001), enabled by the aligned format (SK20).

## Result (validate 64M structural kind bytes, best of 7)
| variant | GB/s |
|---|---|
| scalar loop | 2.31 |
| SIMD (@Vector(32,u8) + reduce) | **33.07** |

SIMD is 14.3x the scalar loop, ~33 GB/s (near memory bandwidth). The scalar loop's early-return defeats full
auto-vectorisation; the explicit SIMD compare + `@reduce(.Or, v >= lim)` runs at bandwidth. The
alignment-and-stride-predictable arena (SK20's finding: 64-aligned, contiguous structural bytes) is exactly what
admits this. Confirms the untrusted structural decode can run at simdjson throughput.

## Design impact
The typed structural decode (SK18) validates structure at ~33 GB/s via SIMD over the aligned format, so the
untrusted-load path is throughput-bound, not a bottleneck. The format must keep the structural bytes contiguous
and aligned (SK20), which this bench and SK20 jointly require.

## Addendum: optimal SIMD vector width (aarch64)
Sweeping the vector width: 16-byte 38.2 GB/s, 32-byte 32.9 GB/s, 64-byte 33.0 GB/s. The 16-byte vector (128-bit,
the NATIVE aarch64 NEON register width) is fastest; 32/64-byte vectors lower to multiple NEON ops with overhead
and no benefit (the decode is memory-bandwidth-bound anyway). Use 16-byte vectors for the structural decode on
aarch64 (match the platform's native SIMD width). ~38 GB/s confirms the decode is bandwidth-bound, not a
bottleneck.
