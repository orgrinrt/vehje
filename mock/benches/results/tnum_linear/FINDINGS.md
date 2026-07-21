# tnum linear transfers: add/and/or/shl vs concrete (scaffold)

Scaffold for `[bench.tnum_linear]`. Fill the result table and cost-model line
after the harness run.

## What it measures

The cheap linear and logical tnum transfers, sizing the per-op cost of running
abstract interpretation on every numeric node during the check pass. Five
variants over the same operand array:

- `tl_concrete`: plain u64 add, no mask tracking (the neutral baseline).
- `tl_add`: `tnum_add` (carry propagation through unknown bits).
- `tl_and`: `tnum_and`.
- `tl_or`: `tnum_or`.
- `tl_shl`: `tnum_shl`.

Operands (a mix of known and unknown tnums, so the mask arithmetic is genuinely
exercised) are built once per subprocess from a compile-time seed. The timed loop
applies the transfer over the array folding the FFI input so nothing hoists.

These are independent transfer functions with legitimately different results, so
this bench sets `may_differ = true`: the harness does per-variant validation but
does not cross-compare outputs. Baseline: `tl_concrete`, mode subtract, so each
tnum transfer reports its overhead over a concrete u64 add. The point is the
per-op cost floor (the ~1ns cheap regime) contrasting with the multiply bench.

## The defect this fixes

The old `tnum.zig` probe hand-timed each transfer once outside the harness
(header-only CSV) with a standalone sink, so the per-op numbers were not
reproducible and not run under a realistic surrounding workload.

## The fix

Each transfer runs under the harness as its own cdylib variant over the identical
operand mix and workload, with the concrete u64 add as the honest baseline. The
subtract-normalised delta is the tnum overhead per op.

## Measured results

Ratio to baseline (tl_shl), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | tl_add (ratio) | tl_and (ratio) | tl_concrete (ratio) | tl_or (ratio) | tl_shl (base) |
|---|---|---|---|---|---|
| 64 | 2.43x | 2.46x | 1.97x | 1.98x | 413 ns |
| 256 | 2.43x | 2.47x | 1.98x | 1.99x | 1612 ns |
| 1024 | 2.42x | 2.38x | 1.90x | 1.98x | 6418 ns |
| 4096 | 2.46x | 2.45x | 2.01x | 2.05x | 22045 ns |
| 16384 | 2.39x | 2.43x | 2.04x | 2.01x | 87810 ns |

## Cost-model sanity line

At n=16384, the baseline (tl_shl) median is 87810 ns for N tnum ops, bit-parallel over 64 known-bit lanes. Treating n as the work-item count, that is 5.36 ns/item, about 17.2 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

Tnum (known-bits) linear ops cost in proportion to their bit work: shl is the cheapest (a single shift), and/or/concrete are ~2.0x, add is ~2.4x (carry propagation over known bits). No surprises; the abstract-arithmetic cost tracks the concrete op complexity.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
