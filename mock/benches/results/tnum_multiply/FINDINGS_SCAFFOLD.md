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

## Result (fill after run)

| n | tm_fastpath (baseline) | tm_loop (excess) |
|---|---|---|
| 64 | | |
| 256 | | |
| 1024 | | |
| 4096 | | |
| 16384 | | |

Cross-validation: expect pass.

## Cost-model sanity line (fill after run)

At n=16384, `tm_loop` is <T> for 16384 multiplies times 2 passes = <ns>/multiply,
about <cycles> cycles, consistent with 64 abstract adds per loop multiply. State
the fast-path number against it (the ~80% both-known share collapsing to a
concrete multiply) and the speedup on the realistic mix. Note the worst-case
unknown-times-unknown cost and that real programs hit it rarely.
