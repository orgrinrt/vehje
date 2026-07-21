# tnum abstract arithmetic: is the numeric residual affordable at compile time?

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 20M abstract ops, mixed known/unknown tnums, 5-run best. Zig 0.16.0,
aarch64. Artifacts: `tnum.zig`, `tnum.csv`.
**Settles:** the compile-side cost of the tnum numeric residual, a named design mechanism (the eBPF-verifier-style
tristate-number abstract domain the compiler uses to track numeric values, prove bounds, and fold constants).
Sizes whether tnum abstract interpretation is cheap enough to run on every numeric op during the check pass.

## Why this probe

The design tracks numeric values through the program with tnums (a tnum is `{value, mask}`: `value` holds the
known bits, `mask` marks which bits are unknown). This is the eBPF verifier's abstract domain, used to prove that
an index is in range, that a check is always true or always false (eliminating it), and to fold constants. The
question is affordability: if abstract interpretation runs on every numeric operation at compile time, each op
does tnum arithmetic instead of concrete arithmetic, so the per-op cost decides whether the analysis can be
pervasive or must be selective. The tnum add (with its carry propagation through unknown bits) and the tnum
multiply (a bit-iteration) are the operations to size.

## Results (ns per abstract op)

| op | ns/op | vs concrete |
|---|---|---|
| concrete u64 add (baseline) | 0.68 | 1.0x |
| tnum_add | 0.95 | 1.4x |
| tnum_and | 0.87 | 1.3x |
| tnum_or | 0.68 | 1.0x |
| tnum_shl | 0.69 | 1.0x |
| tnum_mul (64-bit loop) | 149.76 | 220x |
| tnum_mul fast-path (80% both-known) | 30.56 | 45x |

## The finding: the linear domain is free, so track it pervasively; multiply needs a known-operand fast path

Two regimes, cleanly separated:

**Linear and logical tnum ops are essentially free (~1 ns, within 1.4x of concrete).** tnum_add (0.95 ns),
tnum_and (0.87 ns), tnum_or (0.68 ns), and tnum_shl (0.69 ns) are all a handful of bit operations (the carry
propagation for add is `sigma = sm + sv; chi = sigma ^ sv; mu = chi | a.mask | b.mask`, a few instructions). So
running tnum abstract interpretation over the whole program's additions, subtractions, shifts, and bitwise
operations is negligible: at ~1 ns per numeric node, a 500K-node program's linear numeric analysis is ~0.5 ms,
lost in the compile budget. Track the linear domain everywhere.

**tnum_mul is expensive (149 ns, 220x concrete) and needs a fast path.** The general tnum multiply is a 64-bit
long-multiplication loop (iterate the bits of one operand, shift-and-add-abstract the other), and it costs 149 ns.
That is because it does 64 abstract adds internally. The fast path is the both-operands-known case: if both are
fully known, the product is known (a concrete multiply, 0.68 ns), which is exactly the multiply-by-constant and
constant-folded case that dominates real programs (`index * stride` with a known stride, `x * 2`, arithmetic on
literals). With 80% of multiplies both-known, the average drops to 30.6 ns (the 20% genuine unknown-times-unknown
still hit the loop and dominate the average). Even the worst case is affordable at realistic counts: 10K genuine
unknown-times-unknown multiplies is ~1.5 ms, fine for compile time, and real programs have far fewer (most
multiplies have a known operand).

So the design rule for the tnum check pass:

1. **Track the linear/logical domain (add, sub, shift, and, or, xor) on every numeric node.** It is ~1 ns per op,
   pervasive tracking is free, and it is where most bound proofs come from (an index built by adds and shifts).
2. **Multiply uses a known-operand fast path: both-known lowers to a concrete multiply (0.68 ns).** This handles
   multiply-by-constant and constant-folded multiplies, the overwhelming majority. Only genuine
   unknown-times-unknown falls to the 149 ns bit loop.
3. **Track tnum demand-driven where the full multiply would otherwise dominate.** A numeric op whose result never
   feeds a bound, check, or fold does not need its tnum computed, so the rare expensive unknown-times-unknown
   multiply is only paid when its result actually feeds a proof obligation. This keeps the 149 ns loop off the hot
   path even for the multiply-heavy numeric consumer.

## Design impact

- The tnum numeric residual is affordable as a pervasive compile-time analysis: the linear domain is ~1 ns per
  op, so the compiler can track numeric known-bits over the whole program cheaply, discharging bound and check
  proofs (the certified-generation model's numeric obligations) at negligible cost.
- Multiply is the one operation needing care: a both-known fast path (concrete) covers the common
  multiply-by-constant case at 0.68 ns; the general unknown-times-unknown loop (149 ns) is reserved for the rare
  case and paid only demand-driven where the result feeds a proof. This is the standard abstract-interpretation
  discipline (cheap transfer functions for the common ops, a fast path for the expensive one, demand-driven for
  the rest).
- This sits on the compile side (dev-time), so its cost is a dev-iteration concern, not a runtime one (SK1
  confirmed the resulting inclusion/bound checks fold away to nothing at runtime). At ~1 ns/op for the linear
  domain, tnum analysis does not meaningfully slow the compile.

## Boundary

64-bit tnums; a language with wider or narrower numeric types scales the multiply loop by bit width (a 32-bit
tnum multiply is half the loop). The bench measures per-op cost in isolation; real analysis interleaves tnum ops
with the graph walk (the lease-fixpoint bench sized the walk itself), so the total check-pass cost is the walk
plus ~1 ns per linear numeric node plus the multiply fast/slow split. Division and modulo (not benched) are
similar to or more expensive than multiply and take the same known-operand fast path plus demand-driven
treatment. The tnum algorithms here are the standard eBPF-verifier ones; a coarser numeric domain (interval or
sign only) would be cheaper but proves less, a separate design axis.

## Artifacts
- `tnum.zig` (tnum add/and/or/shl/mul plus the mul fast path, over mixed known/unknown inputs), `tnum.csv`.
