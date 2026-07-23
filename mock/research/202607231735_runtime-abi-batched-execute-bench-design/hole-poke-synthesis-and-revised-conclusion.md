# Hole-poke synthesis and the revised, honest conclusion

**Date:** 2026-07-23 (after the second expert pass)
**What this is:** three experts poked holes in the first-look conclusion (agner-fog on measurement,
linus-torvalds on modelling, john-carmack on causal inference; their files sit beside this one). They converged
hard, and the first-look conclusion in `first-look-results.md` is corrected here. That file is left as the audit
trail of the overstated first claim; this document supersedes its conclusion.

## What the three converged on

1. **The "W >= 8 threshold" is circular.** `carrier-runtime`'s `soa_batch` runs entirely through the scalar
   remainder below `LANES = 8` (`lib.rs:118-133`), so at W = 1, 2, 4 the SoA cell IS the scalar path. "No win
   below 8, win at 8" is the bench reading its own hardcoded lane constant back out; it is arithmetic ("a vector
   needs a vector's worth of lanes"), not an ABI finding. The ABI-relevant "does a column help" knee is really
   W >= 2.
2. **The ~2.4x is dispatch-amortisation plus partial NEON, not isolated vectorisation, and not a property of the
   ABI entry.** The SoA cell walks the program 32 times (8 lanes each) where the scalar cell walks it 256 times,
   so most of the gain is amortised loop control and opcode dispatch; `Simd<u64,8>` lowers to four 128-bit `.2d`
   ops and `u64` multiply has no NEON form. It is a property of how `soa_batch` chunks, and there is no
   horizontal-scalar-batch control to separate SIMD from dispatch-amortisation.
3. **The decisive regime was never built.** The design itself says the decision lives where `C_cross / I_payload`
   is large (the native / copy-and-patch tier). But every boundary family pinned a heavy ~256-node tree-walk
   (`common.rs` `PROG_NODES = 256`), so a ~9 ns crossing is invisible against a ~microsecond-per-record payload,
   and the full run that was in flight could not answer the question. This is the load-bearing miss.
4. **The quoted "2-4 us crossing" is not the crossing.** That is the `fill_seeds` intercept (a constant O(N) fill
   and fold). The actual crossing, isolated from the W-slope, is **~9 ns per warm crossing**, which both bench 1
   and bench 3 agree on independently. `null_entry` is dominated by the fill, not the k crossings, so its raw
   value does not isolate `C_cross`.
5. **Sequential thermal drift confounds every cross-W comparison.** The W-invariant scalar payload rose ~12.5%
   monotonically across the 371 s sweep; the "2.56x" headline used the noisiest point (scalar at W=8), and the
   honest plateau ratio is ~2.42x. At W=256 the FFI cell even came out faster than in-process, a drift artefact
   that also voids any entry-form conclusion from this payload class.
6. **The cold regime (as first built) did not actually mispredict.** It indexed the cycled target off a
   deterministic loop counter an ITTAGE predictor learns after warmup, and the warm baseline lacked the
   per-call `black_box` the cold targets carry. Both are fixed (commit `1c38104a`): the index is now derived from
   the batch's seed data (unknown to the predictor at branch time), and the warm baseline is `cold_target_0` so
   the delta is pure misprediction.
7. **The record is a toy relative to the round's own settled contracts** (value-arena tree outputs, per-op field
   reads, effects/faults). The one-u64-in / one-checksum-out record and the XOR-prefolded marshalling do not
   model a real residual; the masked-SoA cost the settled error contract names was not built.

## The revised, honest conclusion

The batched-column-entry decision is right, but the bench **confirms a largely foregone conclusion rather than
deciding an open one**, and its real value is a set of constants and one genuinely new result, not the "2.4x
enablement" the first look claimed. Precisely:

- **The crossing is nearly free: `C_cross` ~= 9 ns warm.** This is the citable result, and its value is
  *negative*: it kills "FFI is expensive, so the ABI must batch to amortise the crossing" and it kills "the ABI
  must ship per-W monomorphised symbols" (a runtime-W entry with an internal loop pays essentially nothing over
  the monomorphised forms at this crossing cost). The ABI can expose one runtime-W column entry and stop there.
- **The SoA vectorisation win survives the boundary intact, byte-exact.** Crossing an object boundary does not
  cost the vectorisation anything: the same ~2.4x SoA-over-scalar throughput holds whether the payload runs
  in-process or across a real dlopen'd `blr`. This is the new, real result (the "preservation" result), and it is
  what justifies letting the runtime own a resident column and vectorise it.
- **A batched / column entry is the right shape**, but because the runtime vectorises across a *resident column*
  and because the contract is stateless-per-call with synchronous results (the round's settled shape), not
  because a scalar entry was measured to "foreclose" a win. Under that contract the conclusion was true before
  the bench ran; the alternatives (a runtime-resident column fed record-at-a-time, accumulate-and-flush) are
  ruled out by the contract, not by these numbers. The `W >= 8` figure is `soa_batch`'s software chunk width, not
  an ABI parameter; the ABI's minimum useful batch is 2 (any column beats one record), and the runtime, not the
  ABI, owns the SIMD width.

## What the payload-cost family adds (the decisive axis, now built)

`abi_payload_cost` sweeps the residual size (the payload cost) at fixed W=64, so the crossing floor is constant
and the payload moves. Measured (profile `real`, median ns for the 256-record column at W=64):

| residual nodes | null_entry (crossing+fill) | scalar_payload | soa_payload | SoA vs scalar |
|---|---|---|---|---|
| 1 | 2,630 | 9,710 | 2,640 | 3.7x faster |
| 4 | 2,670 | 26,790 | 6,520 | 4.1x |
| 16 | 2,610 | 114,020 | 33,000 | 3.5x |
| 64 | 2,540 | 481,890 | 208,780 | 2.3x |
| 256 | 2,530 | 2,150,000 | 887,950 | 2.4x |
| 1024 | 2,500 | 9,040,000 | 3,560,000 | 2.5x |

The decisive reads:

1. **The crossing is negligible against ANY interpret payload, including the cheapest.** `null_entry` is ~2.5 us
   flat across the whole sweep, but that is dominated by the constant `fill_seeds` (256 seeds), not the crossing:
   the actual crossing is ~9 ns x 4 = ~36 ns for the column. Even the 1-node residual (scalar 9.71 us = ~2.5 us
   fill plus ~28 ns/record interpret) dwarfs the crossing ~200:1. The crossing only becomes a live term BELOW
   interpret cost, at the native / copy-and-patch tier (~ns/record), which no interpret-based bench reaches. So
   the crossing-amortisation justification for batching is real ONLY at the native tier, and that tier is still
   unmeasured; for every interpret tier the crossing does not matter.
2. **The SoA win holds across the whole payload range** (2.3x to 4.1x, largest at tiny residuals), so a batched
   column entry is justified by the vectorisation it enables at every interpret size, not by crossing cost. This
   is the load-bearing, robust result.
3. So the corrected conclusion tightens to: **expose a batched column entry because the runtime vectorises the
   resident column (a 2.3-4x throughput win that survives the boundary byte-exact); the crossing itself is free
   (~9 ns) at every interpret tier, so the ABI needs no per-W symbol zoo and no crossing-amortisation
   cleverness.** Whether crossing amortisation ever matters is a native-tier question left open, to be answered
   by wiring the copy-and-patch payload across the boundary (a named follow-up), not by these interpret numbers.

Raw data committed under `.bench_history/abi_payload_cost_{real,leaf}_n*.tsv`. The `leaf` profile agrees (SoA
2.5-4x, crossing flat). One `n=1024 leaf scalar` outlier warning is a large-program slowdown, not a defect.

## The native tier answers the open crossing-amortisation question (option 1)

`abi_native_cross` runs the copy-and-patch compiled residual across the boundary. A first pass at a 256-node
native residual showed even native-compiled code costs ~860 us/column (per-node work plus the result reduction),
so the crossing was still negligible (native_ffi_w ~= inproc_native, flat across W). Shrinking the residual to a
4-node kernel (~ns/record, the true cheap-payload regime) makes the crossing the dominant term, and the
amortisation is unmistakable (profile `real`, median us for the 256-record column):

| W | native_ffi_w (across boundary) | inproc_native (no crossing) | null_entry (crossing floor) |
|---|---|---|---|
| 1 | 34.70 | 11.59 | 4.87 |
| 2 | 17.49 | 12.09 | 3.47 |
| 4 | 13.93 | 11.66 | 4.05 |
| 8 | 13.54 | 11.69 | 3.08 |
| 16 | 12.25 | 11.45 | 2.54 |
| 64 | 12.06 | 11.39 | 2.45 |
| 256 | 11.60 | 11.41 | 3.17 |

**At the native tier, batching amortises the crossing ~3x.** `native_ffi_w` falls from 34.7 us at W=1 (256
per-record crossings) to 11.6 us at W=256 (one crossing), converging onto `inproc_native` (~11.4 us, the identical
native payload with no crossing). So a scalar per-record entry at native cost pays ~23 us of crossing over the
column (~66% of its time); a column entry removes it. The per-record ABI cost here is ~90 ns (23 us / 256), not
the ~9 ns raw `blr`, because a real per-record call also pays argument marshalling and the handle dereference,
not just the branch; the batched entry amortises all of it.

So the crossing-amortisation justification for a batched entry is REAL at the native tier (a ~3x win, and the
larger the per-record ABI overhead the more it matters), and absent at the interpret tiers (where the payload
dwarfs it). The batched column entry is justified at every tier, for complementary reasons: vectorisation
everywhere, plus crossing amortisation once the payload is cheap enough (the native / copy-and-patch endgame the
framework is aiming at). This closes the one open question the first hole-poke left.

## What genuinely holds (all three agree)

The two-object cdylib discipline, the resolve-in-setup, the byte-exact cross-validation, and the ICF hygiene are
exemplary and verified (the 16 cold targets are genuinely distinct addresses; the NEON kernel is real, reached by
`bl`, shared by every SoA form). The infrastructure is sound; it was aimed at the wrong payload regime and its
conclusion was over-claimed. The corrections above are the honest read.
