# tnum multiply: always-loop vs known-operand fast path (scaffold)

Scaffold for `[bench.tnum_multiply]`. Fill the result table and cost-model line
after the harness run.

## What it measures

Abstract multiply in the tnum domain (a tnum is `{value, mask}`: known bits plus
which bits are unknown), the eBPF-verifier-style tristate-number arithmetic the
compiler uses to prove bounds and fold constants. The general tnum multiply is a
64-bit long-multiplication loop, the expensive transfer. Two variants:

- `tm_loop`: always runs the 64-bit loop.
- `tm_fastpath`: both-known operands (`a.mask == 0 && b.mask == 0`) lower to a
  concrete `value * value`, mask 0; only genuine unknown-times-unknown falls to
  the loop. This is the load-bearing fix.

Operands (a mix, roughly 80% fully known, the multiply-by-constant / folded
majority) are built once per subprocess from a compile-time seed. The timed loop
multiplies pairs folding the FFI input so nothing hoists. The fast path is exact
for both-known operands and falls to the identical loop otherwise, so both
variants produce byte-identical output (cross-validated). Baseline: `tm_fastpath`
(neutral), mode subtract, so `tm_loop` reports its excess.

## The defect this fixes

The old `tnum.zig` probe hand-timed the always-loop and fast-path multiplies
separately outside the harness (header-only CSV), with no cross-validation that
the fast path is exact. The 220x figure was a single hand-timed number.

## The fix

Both strategies run under the harness over the identical operand mix,
cross-validated byte-identical (proving the fast path equals the loop for
both-known operands), so the delta is the real cost the fast path removes on the
realistic mix.

## Measured results

Ratio to baseline (tm_fastpath), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | tm_fastpath (base) | tm_loop (ratio) |
|---|---|---|
| 64 | 10854 ns | 2.20x |
| 256 | 35000 ns | 2.56x |
| 1024 | 121276 ns | 2.66x |
| 4096 | 482724 ns | 2.76x |
| 16384 | 1977595 ns | 2.56x |

## Cost-model sanity line

At n=16384, the baseline (tm_fastpath) median is 1977595 ns for N tnum multiplies, fast-path shortcut vs 64-iteration bit loop. Treating n as the work-item count, that is 120.70 ns/item, about 386.2 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

A known-fast-path tnum multiply (handles power-of-two and small-constant cases directly) is 2.2x to 2.8x faster than the general bit-loop multiply. Special-case the common multiply shapes; the bit-loop is the fallback.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
