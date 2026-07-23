# The carrier matrix through the microarchitecture + benchmark-validity lens (Agner Fog)

Scope note: I reviewed the CURRENT run (`git_commit: d772801-dirty`, `carrier_optimize_madd_n64.meta.json`),
not the pre-fix state the two prior reviews (`202607230030_full-matrix-review/`) audited. Their headline
defects (uncalibrated `timed!`, in-region SoA allocation, 12 panicking JIT variants, native-ceiling O(N)
per-input) are FIXED in this tree: the template now uses `timed_calibrated!` (`gen_matrix.rs:54`), vertical
uses the non-allocating `..._into` with pre-allocated scratch (`gen_matrix.rs:314,328,335`), the JIT and
native-ceiling families cap at N<=1024 (`gen_matrix.rs:297,361`). Everything below is about the fixed state,
and the novel angles are consequences of the calibration that did not exist when they reviewed. Machine is an
**Apple M1** (`meta.json`), single-thread pinned to a Firestorm P-core (`counter.rs:139-168`).

## Verdict in one line

The harness is honest and the ratios are directionally trustworthy in two clean cache regimes (L1-resident
N<=1024 and L2-resident N=16384), but the reported numbers are warm-resident 16-execution throughput (not
per-call latency in the advertised realistic context), the N=4096 column sits on the M1 L1D capacity cliff and
its cross-variant ordering is a cache artifact, and two headline magnitudes (SIMD 6x, dispatch "2-15%") are
inflated by a shared checksum tax and a wire-vs-predecoded baseline asymmetry; correct the decision to the
directions, not the exact multipliers.

## What is methodologically sound (specific, cite file:line)

- **Timer choice and the calibration principle.** `CNTVCT_EL0` at 24 MHz, 41.67 ns/tick, read with
  `mrs` under `nomem`/`nostack` (`counter.rs:38-44`), frequency confirmed live (`meta.json counter_freq:
  24000000`). At 41.67 ns/tick a raw 2 us region is ~48 ticks, so `+/-1` tick is 2% quantization; the fix is
  right in principle: `CALIBRATION_FLOOR_TICKS = 2048` (~85.3 us) (`bench-core/lib.rs:435`) and
  `calibrate_reps` repeats the run block until the aggregate clears the floor, then divides
  (`bench-core/lib.rs:447-454`, `harness` expansion `:537-554`). This recovers sub-tick resolution on the
  aggregate, which is the correct way to beat a coarse free-running counter.
- **Anti-DCE without `black_box`, via real data dependencies.** The hot path uses NO `black_box`
  (`gen_matrix.rs:54-63`); it seeds `acc` from `output[0]`, folds it, and writes `acc` back to the
  harness-visible `output` (`:55,62`). Because the next calibration rep reads the prior rep's `output[0]`,
  the reps form a genuine WAR/RAW chain the optimizer cannot collapse, and the terminal `output` write is
  read by the harness (`harness.rs:341`). This is stronger than a `black_box` fence: it forbids collapse
  through a real dependency instead of an opaque barrier that can also inhibit the optimizations you want to
  measure. Program node 0 is `INPUT` (`gen.rs:245`), so the DAG depends on the per-call seed and cannot be
  constant-folded away.
- **The dispatch axis is isolated to dispatch.** Every scalar cell shares one op-semantics source
  (`ops.rs:27-40`), one unchecked operand path (`access.rs:24-32`), and post-pass checksum
  (`access.rs:39-45`); only the dispatch skeleton differs (`interp.rs:19-60` switch vs `:118-127` fntable
  vs the tail-threaded `interp_threaded.rs:1-16`). The prior bounds-check confound is genuinely gone.
- **Process hygiene.** Per-variant release cdylib (fat LTO, one CGU, `matrix.rs:256`), fresh subprocess
  per (variant, mode, pass) with ABI-hash check on load (`harness.rs:100-109`), randomized variant order per
  pass (`harness.rs:559-565`), warmup before timing (`harness.rs:244-257`), P-core QoS pin
  (`counter.rs:139-151`). Cross-variant cross-validation is real WHERE reps are equal: within one bench all
  cells run the same program and should produce the same `output`.
- **Native-ceiling correctly quarantined.** Its O(N^2) byte-stream idiom (`gen_matrix.rs:348-361`) is
  labelled non-comparable and capped, and the summary never normalizes it against siblings.

## Findings (numbered)

### 1. The calibration re-warm defeats the "realistic surrounding workload" for the reported metric (design/interpretation)

The problem: `algo_ns` is the INNER calibrated time returned by `entry` (`harness.rs:343` reads
`result.run_ticks`), and the realistic 6-item shuffle workload (`main.rs:19-28`) wraps `entry` but its cost
lands in `e2e_ns`/`bridge_ns`, not `algo_ns` (`harness.rs:376-385`). Now add the calibration: the reps loop
re-runs the same N-node program tens to thousands of times inside `entry` (`bench-core/lib.rs:547-550`), so
after the first rep the interpreter's own working set is L1/L2-resident and the branch predictor is trained on
this program. The surrounding workload perturbs caches/BP only for the probe and the first rep; its influence
on the reps mean is ~1/reps, i.e. negligible. Why it distorts: the whole "measured call sees a real calling
environment rather than an empty loop" premise (`main.rs:13-16`) is true for `e2e_ns` (unreported) and
effectively FALSE for the reported `algo_ns`. The headline numbers are warm, steady-state, same-program
throughput, not cold-context latency. That is a legitimate thing to measure, but it is not the thing the
harness advertises, and it changes what the matrix establishes: it ranks warm per-op throughput of dispatch
shapes, which is the right question for a hot column-eval inner loop and the WRONG question for a cold,
first-touch, one-shot residual execution. Concrete fix: report the ratios as warm-throughput explicitly, and
if cold-context latency matters to vehje, add a non-calibrated single-pass mode whose `algo_ns` is the first
rep only. File: `gen_matrix.rs:47-64`, `harness.rs:337-346`.

### 2. The O(N) fidelity checksum is co-timed and dilutes every dispatch ratio (measurement)

The problem: each body folds `acc ^= c::checksum(&r)` INSIDE the `k` loop, once per interpret
(`gen_matrix.rs:171`), and `checksum` is an O(N) fold over the whole results array
(`access.rs:39-45`). So each timed pass is 16 interprets PLUS 16 full-array folds. The brief claims the
checksum is "out of the timed region" (`00_context.md:64`); it is not, it is inside the timed run block.
Why it distorts: the fold is uniform across variants, so absolute differences are preserved but RATIOS are
compressed toward 1.0 by the shared O(N) tax co-resident in the denominator. Magnitude: the `nullfloor`
control (`interp.rs:413+`: two loads + one fixed add + store per node, no opcode branch) plus its checksum
runs at 0.22x of switch at N=16384 real (`RUN_SUMMARY.md:106`). The non-dispatch floor is ~22% of the total,
so a true dispatch delta on the remaining ~78% shows on the total scaled by 0.78. Finding 1 of the summary
("dispatch shape barely matters, 2-15%") is therefore directionally right but understates the pure-dispatch
effect by ~1/0.78 ~ 1.3x. Concrete fix: to isolate dispatch, difference against `nullfloor` (which does the
same memory traffic and a representative ALU op), not against `switch`; the harness even has `subtract` mode
wired (`gen_matrix.rs:118`) but the summary tables report raw median ratios vs switch. Note the checksum is
also serving anti-DCE duty; a single-element keep-alive plus the existing `output` anchor would cut the tax
without weakening fidelity, but would change the interpret's own dead-store DCE profile, so it is a real
tradeoff, not a free win.

### 3. The N=4096 column sits on the M1 L1D capacity cliff; its cross-variant ordering is a cache artifact (measurement, HIGH)

The problem, measured: per-node cost for switch/real is 2.5 ns at N=1024 (41.27us / 16 / 1024), jumps to
9.0 ns at N=4096 (588us / 16 / 4096), then is flat at 10.0 ns at N=16384 (`RUN_SUMMARY.md:108-109` and raw
CSVs). A 4x per-node step between 1024 and 4096, then near-nothing to 16384, is the signature of falling out
of L1D. The working set is the REC24 program bytes streamed per node plus the results array: at N=4096 that is
~96 KB program + ~32 KB results = ~128 KB, exactly M1 Firestorm's 128 KB L1D (L1D size is public
reverse-engineering, not my measurement); at N=1024 it is ~32 KB (L1-resident), at N=16384 ~512 KB
(L2-resident, L2 is 12 MB shared per P-cluster). The reproducible part is fine. The artifact is the
CROSS-VARIANT reshuffle AT the cliff: at N=4096 real the order is nullfloor(117us) < bittree(262) <
ifchainlin(545) < fntable/ifchain/switch(~560-590) < threaded(639); at N=16384 it is nullfloor(585) <
fntable(2343) < switch(2612) < bittree(2653) < threaded(2923) < ifchainlin(3255). `bittree` moves from
2nd-fastest to 6th, `ifchainlin` from mid-pack to last, ACROSS the residency boundary. Within-variant CV at
N=4096 is only ~5% (raw switch rows span 530k-630k ns), so this is not random noise; it is associativity /
code-and-static placement sensitivity that differs per cdylib right at the capacity knee. Why it invalidates:
a reader taking the N=4096 sub-table at face value gets a dispatch ranking that contradicts both neighbors.
The summary dodges this by headlining "vs base (N max)" = N=16384 only, which is defensible, but the matrix
ships the 4096 column with no flag. Concrete fix: mark N=4096 as the cache-cliff size for this record width
and exclude it from ranking, OR add the two columns (`instructions`, `cycles`) so an IPC/miss story confirms
the L1->L2 transition rather than leaving it inferred. Files: `RUN_SUMMARY.md` per-size tables; raw
`carrier_dispatch_*/`.

### 4. Vertical's scalar baseline is the wire interpreter; the vert cells are predecoded, inflating the SIMD win (measurement, HIGH)

The problem, measured: the vertical scalar cell runs `c::interpret(&d, ...)` over the wire form
(`gen_matrix.rs:320-322`), while vert4/vert8 run over cached `Predecoded` `pd` (`gen_matrix.rs:311-336`).
So the scalar baseline pays per-node wire decode that the SIMD cells do not. Confirmed in the raw: vertical
scalar real N=16384 is 2.609 ms per single interpret, matching the WIRE switch (2.640 ms), not the predecoded
switch (2.094 ms). vert8 is 440 us/input, so the reported ratio is 2608/440 = 5.9x ("up to 6x",
`RUN_SUMMARY.md:64-70`). Against a fair predecoded scalar baseline it is 2094/440 = 4.8x. Why it distorts:
~20% of the headline SIMD speedup is the scalar baseline re-decoding the wire form, an axis that has nothing
to do with vectorization; the vert cells got predecode for free out of the timed region. The per-8-inputs
equalization (scalar 8x1, vert4 2x4, vert8 1x8, `gen_matrix.rs:316-337`) is genuinely well-designed and IS
fair; the ONLY unfairness is decoded-vs-wire. Concrete fix: baseline vertical against
`interpret_predecoded` (the predecoded scalar interpreter, which exists, `predecode.rs`), so the only
difference measured is scalar-lanes vs SIMD-lanes. The corrected 4.8x is still the biggest win in the matrix,
so the conclusion holds; the number is off by ~20%.

### 5. Reported values are per-16-execution and carry a constant integer-truncation bias (measurement, LOW-MEDIUM)

Two second-order artifacts of the calibration. (a) The template fixes `ITERS = 16` and the reps mean is per
PASS, never divided by 16 (`gen_matrix.rs:48,53`; `bench-core/lib.rs:553` divides by reps only), so every
reported `algo_ns` is the time for 16 interprets + 16 checksums, ~16x a single program execution. Uniform
across cells, so ratios are unaffected, but the absolute microsecond figures are not "one execution" and
should not be read as such. (b) `run_ticks = (__end - __start) / __reps` is u64 integer division
(`bench-core/lib.rs:553`), truncating ~0.5 tick (~20.8 ns) on average per call. For the sub-microsecond cells
(optimize `all` madd N=64 = 420 ns, `RUN_SUMMARY.md:462`) that is a ~5% downward bias; for the millisecond
cells it vanishes. Constant across variants, so it cancels in differences but slightly perturbs small-value
ratios (e.g. native-tier 3-4x ratios shift ~3%). Fix: accumulate `__end-__start` across the batch and divide
once, or carry a fractional (f64) per-pass tick before averaging.

### 6. The 8-byte output is timing-dependent, so cross-variant/determinism validation is not the guarantee it looks like (correctness of the fidelity claim, MEDIUM)

The problem: the final `output` is `acc` after R calibration reps, and R depends on measured timing
(`bench-core/lib.rs:543` sizes reps from the probe). Two variants with identical per-op semantics but
different reps counts produce different final `output` bytes; the validate path runs `entry` twice and
compares (`harness.rs:775-789`), but the two calls can themselves take different reps and diverge. So
`may_differ = false` byte-equality across dispatch shapes (the advertised fidelity witness) cannot be relied
on to hold, and where it appears to pass it may be passing loosely or the cross-check is effectively inert.
The genuine fidelity anchor is instead the in-loop `acc ^= checksum(&r)` chain, which does force each
interpret to compute the same per-pass hash regardless of reps; that is real. But the output-level
cross-variant comparison advertised by `MAY_DIFFER = false` (`byte_routine.rs:47-51`) is not doing what it
claims on a calibrated, reps-variable output. Fix: make `output` a reps-invariant digest (e.g. write the
first pass's acc, or a running xor that is reps-idempotent), so output-level cross-validation becomes
meaningful again.

### 7. Provenance: dirty tree (LOW, note only)

`git_commit: d772801-dirty` (`meta.json`). The run was taken against an uncommitted working tree, so the
exact source that produced these numbers is not pinned. Re-run from a clean commit before treating any
figure as citable.

## The measurement-validity ledger (per axis)

- **Dispatch shape (wire, 6 profiles).** Real at the metal in the L1 (N<=1024) and L2 (N=16384) regimes; the
  shape-vs-shape comparison holds compute constant and is the valid part. Effect magnitude UNDERSTATED by the
  co-timed checksum (Finding 2). N=4096 ranking is a cache ARTIFACT (Finding 3). Net: "dispatch barely
  matters on straight-line code" is a real microarchitectural result (Firestorm's ~8-wide OoO front end and
  good indirect-branch prediction on a correlated op stream hide dispatch cost), trustworthy in direction,
  soft in magnitude.
- **Predecode family.** Same regime caveats. `null`/`direct` winning is real (removing the opcode branch and
  keeping handlers register-resident). The N=4096 blowups in `_scatter`/`_real` (e.g. direct 378us then
  1.709ms, `RUN_SUMMARY.md:203`) are the same L1 cliff, not technique.
- **Record width (REC12..REC32).** The null result is REAL ONLY in the re-warmed, L1/L2-resident regime.
  Decode is not throughput-bound when the program is cache-resident, so width does not move the number. This
  does NOT establish width-irrelevance for a runtime that streams programs from DRAM: at 2.6x size spread
  (REC12 vs REC32) a memory-bandwidth-bound cold execution WOULD separate them, and the calibration re-warm
  guarantees this bench cannot see that. Unprovable here without a cold-stream mode. Treat finding 3 as
  "width is free in-cache," not "width never matters."
- **Value representation (static/tagged/nanbox).** Ratio is real within the one program shape; correctly
  hedged in the summary as one-program (`RUN_SUMMARY.md:45-46`). The nanbox/tagged win at N=16384 lands right
  at the L2 regime; whether it survives a cold path is unproven.
- **Residual (register vs stack).** Register-beats-stack is REAL and robust: the stack machine does strictly
  more memory traffic (push/pop through a stack array) per op, which is genuine on any OoO core. 1.91x on
  madd/tight is credible at the metal.
- **Optimize.** Real. The `all` 15-142x on madd is the folder collapsing the chain to almost nothing
  (`RUN_SUMMARY.md:462-468`), and eqsat-alone pessimizing is a real property of AC-reassociation without
  folding. `checksum_at` over live-outs keeps node counts comparable (`access.rs:52-58`). Trustworthy.
- **Native (interp vs copypatch vs stencil).** ~3x and copypatch==stencil is real; both eliminate the
  dispatch branch entirely. Capped at N<=1024 (L1-resident), so the 3x is the in-cache figure. Integer-bias
  (Finding 5b) perturbs the ratio ~3%.
- **Vertical SIMD.** Real and the biggest win, but the 6x is INFLATED to ~4.8x by the wire-vs-predecoded
  baseline (Finding 4). Mechanism is richer than "dispatch amortized over eight": on M1 the `Simd<u64,8>`
  lowers to 4x 128-bit NEON ops (M1 has no SVE; NEON is 128-bit, ~4 pipes), and the scalar `SELECT`
  data-dependent branch becomes a branchless `.select()` blend (`ops.rs:68-69`), so on scatter/real part of
  the win is eliminated branch mispredicts, not lane count. Direction certain, magnitude ~4.8x not 6x.
- **Native ceiling.** ~1.5x, correctly quarantined as O(N^2) throughput, memory-bound not dispatch-bound.
  Fine as a standalone ceiling, never comparable to siblings. The -0.65 lag-1 autocorrelation
  (`RUN_SUMMARY.md:95`, `summary.rs:55-56`) is a real thermal alternation on a long single-thread run, not a
  code effect; the median absorbs it.
- **instructions / cycles columns.** All zero (`meta.json`, PMU off). Every "at the metal" claim above about
  L1/L2 residency, IPC, mispredict counts, and NEON issue is INFERENCE from timing shape plus public M1
  microarchitecture, not measurement. The PMU re-run is what would move Findings 2, 3, and the vertical
  branch-elimination claim from inferred to proven.

## Novel angles the prior reviewers missed (>=1)

1. **The calibration they recommended adopting now defeats the workload realism they credited.** The prior
   agner review's A2 argued FOR `timed_calibrated!`; it is now in, and the consequence (Finding 1) is that
   the reps re-warm makes the realistic 6-item shuffle irrelevant to `algo_ns`. This is a new interaction that
   could not exist in the pre-fix state they reviewed.
2. **The N=4096 L1D capacity cliff and the cross-N rank instability (Finding 3).** Neither prior review
   framed the N sweep against the M1 cache hierarchy or noticed that `bittree`/`ifchainlin` swap regimes
   across the residency boundary. This is the sharpest decision-grade caveat and is a pure cache-placement
   artifact.
3. **The record-width null result is regime-bound (ledger).** Prior finding-3 discussion was about
   native-ceiling comparability; the point that the re-warm forbids this bench from ever seeing a
   DRAM-streaming width effect (so finding 3 must be read as "free in-cache," not "irrelevant") is new and
   directly governs whether vehje can pick REC width for density without a throughput worry in a cold-load
   deployment.
4. **The wire-vs-predecoded vertical baseline (Finding 4), quantified at 5.9x -> 4.8x.** The prior A1 was
   about in-region SoA allocation (now fixed); the decode-in-baseline asymmetry is a different, still-present
   inflation.
5. **The reps-variable output makes MAY_DIFFER=false validation inert (Finding 6).** A correctness-of-fidelity
   angle the prior reviews (which predate calibration) did not raise.

## Open questions for the synthesiser (calls I cannot make alone)

1. **Which regime is vehje's runtime?** If residuals execute warm and repeatedly over resident programs (hot
   column-eval inner loop), the L2-resident N=16384 ratios are the right oracle and finding 3 ("width is
   free") stands. If residuals are cold-loaded and streamed once from DRAM, the matrix cannot answer the
   width question and a cold-stream bench is owed. Tradeoff: adding a cold-stream mode is real harness work
   vs shipping a width decision the current data does not actually support outside cache. No preference.
2. **Warm-throughput vs cold-latency as the canonical metric.** Finding 1 means the matrix ranks warm
   throughput. If vehje cares about first-touch latency of a freshly emitted residual, the ranking could
   differ (threading and predecode change relative to a cold front end). Tradeoff: a single-pass uncalibrated
   mode costs counter resolution at small N (the exact problem calibration solved) vs measures the real cold
   path. No preference.
3. **Is the ~20% checksum tax acceptable, or should the fidelity keep-alive be lightened?** Lightening it
   (Finding 2) sharpens the dispatch ratios but changes the interpret's own dead-store elimination and
   weakens the O(N) cross-validation. Tradeoff: measurement fidelity of the dispatch axis vs semantic
   fidelity of the interpret. This is a values call about what the bench is FOR. No preference.
4. **Do the two headline magnitudes need correcting in the writeup, or only the directions?** SIMD 6x->~4.8x
   (Finding 4) and dispatch "2-15%"->~1.3x wider (Finding 2) are both direction-preserving. If the synthesis
   only carries directions into the canonical design, the numbers can stand with a caveat; if it carries
   multipliers (e.g. "SIMD buys 6x, budget for it"), they should be corrected. No preference on which.
5. **PMU re-run before or after the design decision?** The `instructions`/`cycles` columns would convert the
   L1/L2-cliff, checksum-tax, and NEON-branch-elimination claims from inferred to measured. Tradeoff: the
   sudo-gated re-run is cheap and reproduces the identical matrix (`RUN_SUMMARY.md:10-13`) vs deciding now on
   timing-shape inference. No preference, but note every "at the metal" claim in the ledger is currently
   inference, not measurement.
