# BN5 findings: ReleaseFast vs ReleaseSafe (safety-check cost)

**Date:** 2026-07-21 | **Type:** runtime bench, Zig-native | Zig 0.16.0 | data `safe.csv`
**Settles:** the 1845 ReleaseFast-vs-ReleaseSafe axis (can the shipped runtime keep safety checks on).

## Result (bounds + overflow-check-heavy workload: random-index dependency chain, 1M u32, median of 7)
| mode | median ns |
|---|---|
| ReleaseFast | 35,620,000 |
| ReleaseSafe | 35,591,000 |

**-0.1% (identical within noise).** ReleaseSafe's bounds and overflow checks are effectively FREE on this
memory-latency-bound workload: the checks hide under the memory-access latency of the random-index dependency
chain (the dominant cost is the cache-miss load, not the check).

## Reading and design impact
The interpreter's hot loop is memory-bound (an arena walk with data-dependent operand reads, per BN1/BN2), which
is exactly this workload shape, so shipping the composed runtime in ReleaseSafe (safety checks ON) costs nothing
measurable. A compute-bound tight loop with no memory stalls WOULD show real overhead (the checks cannot hide),
so this is not "safety is always free," but for the runtime's dominant memory-bound shape it is. Recommendation:
ship the runtime with ReleaseSafe (keep the safety net) for the interpreter path; profile any compute-bound
kernel (e.g. a stenciled native tier) separately if one is added.
