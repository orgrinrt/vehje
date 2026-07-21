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

## Result (fill after run, raw ns/op and overhead vs concrete)

| n | tl_concrete | tl_add | tl_and | tl_or | tl_shl |
|---|---|---|---|---|---|
| 64 | | | | | |
| 256 | | | | | |
| 1024 | | | | | |
| 4096 | | | | | |
| 16384 | | | | | |

Validation: per-variant only (may_differ); no cross-compare.

## Cost-model sanity line (fill after run)

State the per-op cost of each linear transfer (expect all within a small factor
of the concrete add, the cheap regime), and the compile-budget implication: at
<ns>/op a program of <k> numeric nodes carries pervasive tnum tracking for the
linear domain in <ms>, negligible against the compile budget. Contrast with the
multiply bench: the linear domain is free, so track it everywhere; only multiply
needs the known-operand fast path.
