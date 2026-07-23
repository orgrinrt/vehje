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

`abi_payload_cost` sweeps the residual size (the payload cost) at fixed W=64, so `null_entry / scalar_payload` is
the `C_cross / I_payload` curve measured directly, from a 1-node residual (cheapest, closest to the native tier)
to 1024 nodes. This is the experiment the design named and the earlier families omitted. (Data appended when the
run lands; the cheap-residual end is where, if anywhere, the crossing is a non-trivial fraction and batching pays
for crossing amortisation rather than only for the vectorisation it enables.)

## What genuinely holds (all three agree)

The two-object cdylib discipline, the resolve-in-setup, the byte-exact cross-validation, and the ICF hygiene are
exemplary and verified (the 16 cold targets are genuinely distinct addresses; the NEON kernel is real, reached by
`bl`, shared by every SoA form). The infrastructure is sound; it was aimed at the wrong payload regime and its
conclusion was over-claimed. The corrections above are the honest read.
