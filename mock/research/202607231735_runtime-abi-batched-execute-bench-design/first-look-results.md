# First-look results: the runtime C ABI batched-execute benches

**Date:** 2026-07-23
**What this is:** the first timing data off the built machinery, after the panel review and the blocker fixes.
It proves the run pipeline end-to-end and gives the first evidence-backed read on the ABI decision. It is a
first look, not the final decision: it covers a subset of families and profiles, and carries the fairness doc's
caveats (warm crossing, high-W reps noise). Raw data is committed under `mock/benches/.bench_history/`.

## The pipeline works

`gen_matrix` (post-fix) generates all boundary variants; each variant builds as an isolated fat-LTO cdylib in
~8s; the driver (`--only <section>`) dlopens them, times under the realistic workload, and writes per-size TSVs.
One family-profile (bench 1, `abi_cross_scalar_real`, 4 cells x 9 W) took ~7 minutes, dominated by the high-W
floor cells' large rep counts (the reps effect below). The full 22-family, 6-profile matrix is therefore a
multi-hour job; the families below are the first-look subset.

## Bench 1: crossing amortisation, scalar payload (`abi_cross_scalar_real`)

Median ns for the whole 256-record column, per W (`n` = batch width):

| W | ffi_batched_scalar | inproc_direct | inproc_fnptr | null_entry (crossing) |
|---|---|---|---|---|
| 4 | 2,176,659 | 2,165,159 | 2,179,285 | 4,055 |
| 32 | ~2,170,000 | ~2,160,000 | ~2,180,000 | 2,280 |
| 256 | 2,376,329 | 2,429,625 | 2,423,527 | 3,501 |

Reads:

1. **The crossing is negligible against a scalar-interpret payload.** `null_entry` (pure crossing plus
   marshalling, no interpret) is ~2-4 us for the whole column; the payload (`inproc_direct`, 256 in-process
   scalar interprets) is ~2.17 ms. The crossing is ~0.15% of the payload.
2. **A real cross-object crossing is nearly free over in-process.** `ffi_batched_scalar` (the genuine dlopen'd
   cross-object call per batch) sits within ~0.5% of `inproc_direct` at low W. Crossing an object boundary costs
   almost nothing next to the scalar interpret.
3. **The indirect-call rung is also negligible.** `inproc_fnptr` (rung b, black-boxed pointer) tracks
   `inproc_direct` (rung a) within noise.
4. **The consequence for the ABI decision:** when the payload is an expensive scalar interpret, NONE of the
   boundary or entry-form choices move the number measurably (the payload dominates ~500x). This is agner's
   `C_cross / I_payload` law with data: the scalar-payload point sits at the "batching barely helps" end. The
   decision lives at the cheap-payload end (SoA, and especially the native tier), where `C_cross / I_payload` is
   large.

The amortisation is visible in `null_entry` at low W (W=4, 64 crossings: 4.06 us) falling toward high W (fewer
crossings), but the high-W points are noisy (W=256 CI 3.44-3.73 us, W=256 payload CI widening to 2.76 ms): the
sub-tick reps-starvation the fairness doc flagged. The low-W anchor is the reliable part of the curve.

## Bench 3: SoA-win, cheaper payload (`abi_soa_win_real`) — the headline

Median ns for the whole 256-record column, scalar vs SoA-8 payload, per W:

| W | scalar_payload | soa_payload | SoA speedup |
|---|---|---|---|
| 1 | 2,460,000 | 2,370,000 | ~1.0x (no win) |
| 2 | 2,420,000 | 2,440,000 | ~1.0x (no win) |
| 4 | 2,410,000 | 2,360,000 | ~1.0x (no win) |
| 8 | 2,490,000 | 971,740 | 2.56x |
| 16 | 2,200,000 | 904,290 | 2.43x |
| 32 | 2,180,000 | 904,290 | 2.41x |
| 64 | 2,170,000 | 897,310 | 2.42x |
| 128 | 2,170,000 | 897,240 | 2.42x |
| 256 | 2,150,000 | 883,340 | 2.43x |

The result that answers the ABI question:

1. **The batched entry is required to unlock the SoA vectorisation win.** Below the SIMD width (W = 1, 2, 4) the
   SoA payload equals the scalar payload: a batch too small to fill an 8-lane vector runs entirely as the scalar
   remainder, so there is no win. At W >= 8 the SoA payload drops to ~900 us, a stable **~2.4x speedup** that
   holds through W = 256.
2. **A scalar per-record boundary (W = 1) forecloses that win.** The SoA payload at W = 1 is 2.37 ms, no better
   than scalar. Only a boundary that takes a column of W >= 8 records per call lets the runtime vectorise across
   records. This is the design premise, now measured: to vectorise across records the boundary must take a
   column.
3. **The crossing cost is not why it matters.** `null_entry` (the crossing) stays ~2-5 us across the whole
   sweep, negligible against both payloads. The batched entry matters as the ENABLER of vectorisation, not
   because the crossing is expensive. The crossing being cheap (bench 1) and the batched entry being decisive
   (bench 3) are consistent: batching pays through what it unlocks downstream, not through crossing amortisation
   at this payload cost.

## The first-look ABI answer

**Expose a batched / column entry that takes W >= the SIMD lane width (8) records per call.** It unlocks a ~2.4x
SoA vectorisation win that a scalar per-record entry forecloses; the batch must reach the SIMD width for the win
to appear, and the win is stable for all larger W. The crossing cost itself is negligible at any interpret
payload, so the batched entry is justified by the vectorisation it enables, not by crossing amortisation. The
`W >= 8` threshold is a direct design input: the ABI's minimum useful batch is the SIMD width, and Wmax = 128
(op's setting) sits well past the point where the win saturates.

This is a first look on one profile (`real`) and the scalar-vs-SoA payload axis. The full decision (does the
optimum entry form or W move across the six profiles; the native tier where `C_cross / I_payload` is largest;
the sink and marshalling costs) needs the remaining families across profiles, the multi-hour full run. The
qualitative headline (a batched column entry is required for the vectorisation win, threshold at the SIMD width)
is robust to the fairness-doc caveats, which touch the crossing magnitude, not the payload win.

## What this first look establishes

The measurement backbone behaves as designed and the theoretical framing holds against data: the batched-ABI
question is a payload-relative one, and for expensive payloads the answer is "the crossing does not matter."
The interesting regime is cheap payloads, which the SoA, native-ceiling, entry-form, and composition families
target. The full decision needs those families across the six profiles (the selector-regret over `abi_boundary_w`
and the `C_cross / I_payload` plot), which is the multi-hour full run; this note will be extended as those land.

## Caveats carried (from the fairness doc)

Warm crossing (a conservative lower bound on C_cross; the boundary-cold regime would raise it, strengthening not
inverting any "batch" conclusion); high-W floor points reps-noisy (the min-reps floor is a follow-up); the
absolute C_cross magnitude and the headline ratio's precision are what the named follow-ups tighten. The
qualitative reads above (crossing negligible vs scalar payload; the decision is payload-relative) are robust to
all of these.
