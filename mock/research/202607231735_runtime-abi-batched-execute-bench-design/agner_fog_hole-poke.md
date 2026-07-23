# Holes in the measurement (Agner Fog)

**Date:** 2026-07-23. Scope: the first-look numbers, the ABI conclusion drawn from them, and the two
refinements (boundary-cold regime, Zig tail cell). Verified against the committed TSVs, the source, and
`otool -tV` of the built `carrier-runtime` release dylib. No timing runs performed.

## Verdict in one line

The machinery is honestly built and the SoA arithmetic is real NEON, but the headline "~2.4x at W>=8" bundles
two mechanisms the bench cannot separate and pins the knee to a hardcoded constant, the cross-W amortisation
story is confounded by sequential thermal drift rather than the reps-starvation the doc blames, and the
new boundary-cold regime as written almost certainly does not defeat the predictor.

## Real holes

1. **The "SoA vectorisation win" conflates SIMD arithmetic with interpreter dispatch-amortisation; there is no
   control to separate them.** At W>=8 the scalar cell walks the 256-node program 256 times (one per record);
   the SoA cell walks it 32 times, 8 lanes per walk (`soa_batch`, carrier-runtime/src/lib.rs:118-133). So the
   loop control, per-node opcode dispatch, and operand-index loads are amortised 8x *before any lane of
   arithmetic runs*. Predecoding already happened at `cr_init`, so the scalar path re-pays only the walk, not
   decode, yet the walk is exactly what the vertical form amortises. The measured 2.42x is a blend of (a) 8x
   fewer program walks and (b) partial 128-bit NEON (Simd<u64,8> lowers to 4x `.2d` ops, and u64 MUL has no
   NEON form so it scalarises). The report names it "the SoA vectorisation win" (first-look-results.md:47,
   line 68) and attributes the whole factor to vectorisation. It cannot: no horizontal-scalar-batch cell
   exists (decode once, run 8 seeds through the scalar kernel) to isolate the dispatch-amortisation term.
   *Fix:* add a `scalar_batch8` control that runs 8 seeds per decoded walk with the scalar kernel; the
   SoA-minus-scalar8 delta is the true SIMD contribution, scalar8-minus-scalar1 the dispatch-amortisation.

2. **The "W>=8 threshold" is an artefact of the remainder fallback, not a measured ABI property.** Below 8,
   `soa_batch`'s `while i + LANES <= w` never fires and the cell runs the pure scalar remainder
   (carrier-runtime/src/lib.rs:128-132), so soa_payload at W=1,2,4 *is literally the scalar interpreter*
   (data: soa_win_real_n4 soa 2,362,750 vs scalar 2,413,990, ratio 1.02). The knee at 8 is `LANES = 8`
   (lib.rs:43) reappearing in the output, not a discovery. The report elevates it to "the ABI's minimum
   useful batch is the SIMD width" (first-look-results.md:85). By hole 1, the dispatch-amortisation term has
   its knee at W>=2, not W>=8; only the SIMD-arithmetic slice has a hard floor at 8. The measurement cannot
   support "a column entry needs 8 records to be useful"; it supports "filling the NEON vector needs 8." A
   real ABI that shipped a scalar-column entry at W=2 would already capture most of the win, which the current
   data hides.

3. **Every cross-W claim is confounded by sequential (thermal/DVFS) drift.** The cells run one profile-size
   per process, sequentially, over minutes: `abi_cross_scalar_real_n32` timestamp 1784827579,
   `..._n256` 1784827728, 149s later; the whole family spans 371s (n1 1784827357 to n256 1784827728). The
   scalar payload interprets the same 256 records at every W and must be W-invariant, yet `inproc_direct`
   rises monotonically 2,160,452 (n32) to 2,374,961 (n64) to 2,387,171 (n128) to 2,429,625 (n256), +12.5%,
   with CIs ballooning (n256 inproc_direct 2,388,856-2,761,592). That drift is time-correlated, not
   batch-correlated. Any C_cross amortisation curve or crossing-vs-W read fitted across these points inherits
   it. The within-W SoA/scalar ratios (same TSV, same timestamp, seconds apart) are drift-immune and are the
   only trustworthy comparison; the cross-W amortisation narrative is not.

4. **`null_entry` does not isolate C_cross; it measures the constant fill+fold floor.** The timed cell is the
   whole column: `fill_seeds` over 256 elements plus `cross_column`'s per-crossing fold, both O(N_TOTAL) and
   W-independent (common.rs `fill_seeds`, `cross_column`). If the k crossings dominated, W=1 (256 crossings)
   would tower over W=256 (1 crossing). They do not: the sequence is 5189, 3401, 4055, 3056, 2541, 2283,
   2907, 2958, 3501 ns (cross_scalar_real_n1..n256) - non-monotonic, and W=256 (3501) *exceeds* W=16 (2541).
   The per-crossing term is buried under the fill+fold constant and inside sample noise; an OLS slope on
   `k = N/W` here reads mostly noise. The "crossing negligible vs payload" *direction* survives (it is even
   more negligible than claimed), but "amortisation is visible in null_entry at low W falling toward high W"
   (first-look-results.md:43) is reading a trend into a flat noisy series. The stated "~2-5us crossing" is the
   fill+fold floor, not the crossing.

5. **The fairness doc blames the wrong high-W contaminant.** It caveats `null_entry` as reps-starved at high W
   ("per-pass time falls below the calibration tick floor", fairness:79-81). But the timed unit is the full
   column (256 fills + 256 folds + k crossings), never a single crossing, so at W=256 the pass is still
   microseconds, far above `CALIBRATION_FLOOR_TICKS = 2048` (bench-core/src/lib.rs:461-482): reps stays 1 at
   every W. The actual high-W contaminant is the sequential drift of hole 3, which the doc does not name. The
   named follow-up (min-reps floor) would not fix what is actually contaminating the high-W points.

6. **The boundary-cold regime as written likely does not defeat the indirect predictor.** The target index is
   `idx = (cross * 0x9e37...) >> 60 & 15` (cold.rs:77), a pure deterministic function of the loop counter.
   The sequence of 16 targets is byte-identical on every calibration rep. Apple Silicon's indirect predictor
   is history-based (ITTAGE-class); a fixed deterministic sequence of 16 targets is exactly what such a
   predictor learns after a short warmup, most easily at low W where each rep issues 256 crossings and reps
   repeat. So the cold cell probably measures a warmed-up predictor, i.e. near-warm cost, after the first rep.
   The comment claims it "mispredicts" (cold.rs:66-68); that is asserted, not demonstrated. To actually thrash
   the predictor the index must be *reps-variant* (a function of the seed/record bytes, unpredictable across
   reps) while the fold stays reps-invariant. As built it measures BTB *capacity* pressure over 16 sites, not
   misprediction. *Fix:* index off `seeds[off]` (data-dependent, changes every rep via the seed) rather than
   the loop counter; verify with a PMU branch-misprediction counter, not by assertion.

7. **cold-minus-warm is not a clean misprediction delta.** Each cold target executes an extra
   `black_box($tag as u64)` (carrier-runtime/src/lib.rs:252; cold_null:288) that the warm target
   (`cr_execute_scalar_runtime_w`) does not. The cold-warm difference therefore carries one black_box barrier
   per crossing on top of any prediction effect. Small, but it means the isolation is not pure. *Fix:* give
   the warm comparator an equivalent discarded `black_box` so the bodies differ only in address.

8. **One profile, one column shape.** All numbers are `real` at N=256 nodes / 256 records. Both scratches are
   L1-resident (scalar 2KB, SoA `Simd<u64,8>` 16KB, well under 128KB L1), so no memory-hierarchy effect
   distinguishes the payloads, and a MUL-heavy program (no NEON u64 mul) would vectorise far worse. The
   headline 2.42x is a single point in (profile x size x locality) space presented as *the* SoA win. The
   report concedes "one profile" (first-look-results.md:88) but still states the ratio to three significant
   figures and calls it "robust."

9. **The reported 2.56x at W=8 is a co-elevation artefact, not the peak win.** At the n8 sample both cells sit
   above their W>=16 plateaus (scalar 2,489,486 vs plateau ~2,196,746; soa 971,735 vs plateau ~904,290),
   a time-correlated bump at that sample. The ratio 2.56 (first-look-results.md:56) is two co-elevated
   numbers; the true plateau ratio is 2.42. Presenting 2.56x as "the win first appears" overstates the knee.

## What actually holds

- The two-object split forces a genuine un-inlinable cross-object call: `cr_execute_soa_runtime_w` disassembles
  to a thin body that `bl`s into `interpret_vertical_checksum_into` (verified), and the runtime is a separate
  dylib the host dlopens. The crossing is real, not elided.
- The SoA arithmetic is real NEON, and the fairness doc's correction #2 is accurate: the kernel is a 283-line
  callee dense with `.2d` / `q`-register ops (2371 vector instructions in the dylib), reached by `bl`, shared
  by every SoA entry form. The SoA cell does genuine 8-lane work, cross-validated byte-exact
  (`soa_crossing_matches_in_process_byte_exact`).
- The 16 cold targets are genuinely distinct addresses; ICF did not fold them (disasm shows 0xe78 / 0xf20 /
  0x13d4, and the `cold_targets_are_distinct_addresses_with_identical_fold` test enforces it). The black_box
  trick works for its stated purpose; hole 6 is about the *access sequence*, not ICF.
- The within-W SoA/scalar ratio is drift-immune (same timestamp) and is a real throughput gain of ~2.4x,
  whatever its internal split. The *qualitative* ABI conclusion (expose a column entry so the runtime can
  evaluate records vertically) is sound and survives all the caveats; it is only the *magnitude*, the *cause*,
  and the *threshold value* that the measurement overstates.
- The crossing-is-cheap-vs-expensive-payload direction holds by orders of magnitude (single-us crossing floor
  vs ~2.17ms payload) even though C_cross itself is not cleanly isolated.

## The one thing most likely to be an artefact

The **W>=8 threshold reported as a measured ABI property.** It is `LANES = 8` (carrier-runtime/src/lib.rs:43)
and the scalar-remainder fallback (lib.rs:128-132) reappearing in the output: below 8 the SoA entry *is* the
scalar entry, so of course the win switches on at 8. The general claim the ABI cares about, "batch the column
so the runtime amortises per-record work," has its knee at W>=2 (the dispatch-amortisation term of hole 1),
not W>=8. Absent a horizontal-scalar-batch control, the bench cannot see that knee, and the report promotes a
hardcoded SIMD constant into a discovered minimum-useful-batch. That single number, more than any other, is
built into the experiment rather than measured by it.
