# The full carrier matrix, measurement-honesty lens + audit of review 1 (Agner Fog)

**Date:** 2026-07-23. Matrix state as generated on `feat/extension-point-contract`. Harness pinned at
mockspace rev `49ff5f5` (`~/.cargo/git/checkouts/mockspace-725094cc58de1ed7/49ff5f5`).
**Method:** read the generator and every family's prep/body (`gen_matrix.rs`), the carrier cells
(`native.rs`, `vertical.rs`, `access.rs`, `checksum.rs`, `cfg.rs`, `cfg_threaded.rs`), the harness
(`driver/mod.rs`, `driver/worker.rs`, `harness.rs`, `validation.rs`, `matrix.rs`, `report.rs`,
`analysis.rs`), the `timed!` / `timed_calibrated!` macros and the CNTVCT counter in `bench-core`, the
`bench_variant` proc-macro, and `bench.toml`'s carrier config. Ran `cargo test --features
threaded,vertical,jit --lib` in `carrier` (49/49 green; the Zig crossval is a separate feature-gated test I
did not enable). I traced each of review 1's open questions to a definitive answer in the harness source
rather than leaving them open.

## Verdict in one line

Closer to ready than review 1's headline implies on the JIT-panic axis (the driver provably tolerates a
crashed variant and does not stall), but NOT ready for a different reason review 1 missed: the vertical
family allocates its result scratch inside the timed region while its baseline does not, which corrupts the
one measurement the design docs call the likely headline. Land three carrier-side fixes (vertical
scratch-hoist + equal-inputs-per-call, native-ceiling size cap, JIT size cap) and the matrix is sound; two
more (`timed_calibrated!`, prep caching) are strongly advisable and cheap.

## Audit of review 1 (per finding, with evidence, and the resolved open questions)

### Resolved open question 1: does a crashed variant subprocess stall the run? NO. Definitively.

Review 1 left this open and correctly flagged it as the hinge for Finding 1's severity. It resolves against
the "stall" reading, on three independent guards I read in full:

- The variant `run<N>` is called through `bench_entry`, which the proc-macro emits as `pub unsafe extern
  "C" fn` (`bench-macro/src/lib.rs:321`). A `.expect("jit")` panic unwinds into that `extern "C"` frame;
  on the pinned nightly (well past 1.81) unwinding out of a non-`-unwind` `extern "C"` fn aborts the
  process (SIGABRT, non-zero exit). It is a hard process exit, not a caught error.
- Validation runs first and spawns each variant in its own `--mode validate` subprocess
  (`validation.rs:278-303`). A non-success exit is caught: `if !out.status.success() { ... SKIPPING ...
  slow_variants.insert(vi); continue; }`. The crashing variant is excluded from `safe_paths`
  (`validation.rs:500-513`) before any timing. Note the crash-catch here is NOT gated on `max_call_us`
  (only the earlier preflight probe loop at `validation.rs:153` is); the validate-worker crash-catch runs
  unconditionally, so it fires even though the carrier benches set no `max_call_us`.
- Even if a crash slipped past validation, the timing orchestrator checks the same thing:
  `if !output.status.success() { eprintln!(" ... FAILED ..."); continue; }` (`harness.rs:647-655`). One
  dead worker is one skipped `(variant, mode, pass)`, then the loop proceeds.

So a JIT panic at N=4096 loses that variant's points and prints a `SKIPPING`/`FAILED` line; it does not halt
the matrix. **Finding 1 HOLDS on substance (real defect, 24 lost near-native data points), OVERSTATED on
urgency.** The verdict's "blocking two of five sizes across twelve variants" reads as run-stopping; it is
data-loss + log-noise, not a stall. It is still worth fixing, on the mandate's own terms ("a cell that
cannot be made representative is named, not silently dropped"): a size the imm12 window cannot serve should
be declared out of range, not discovered by a crash log. The fix review 1 names is right, cap
`native_family` sizes to `[64, 256, 1024]` (`gen_matrix.rs:259-271`), and it is cheap, so land it, but
reclassify its severity from blocking to should-fix.

### Resolved open question 2: native-ceiling's O(N)-over-input shape, intentional or accident? INTENTIONAL. Review 1's alternative fix is wrong.

Review 1 saw both readings and punted. The source settles it: the shape is deliberate. `native.rs:1-31`
documents the ceiling as a throughput-over-a-byte-stream measurement (the corrected ~10x figure), and
`native_madd_over_input` / `interp::run_over_input` are the crate's established "fold one checksum per pass
over the whole input" idiom, mirrored symmetrically by both cells (`native.rs:117-130`; `gen_matrix.rs:299`).
This is not an accidental carryover; it is the ceiling's designed question.

Therefore review 1's proposed alternative fix, "change the body to one call per outer iteration, matching
its nine siblings", is **wrong and would destroy the measurement.** Collapsing the input sweep turns the
throughput ceiling into a single-execution latency probe, which is a different quantity the nine sibling
families already cover. The correct resolution is review 1's OTHER option: document the different cost basis
next to the family title, and never normalize `carrier_native_ceiling`'s raw ns against a sibling family's.
On that comparability axis, **Finding 3 HOLDS**, and note the report does not auto-compare across benches
(each `[bench.*]` renders its own findings file against its own baseline), so the distortion only bites a
human eyeballing two findings files, which is a documentable caveat, not a wrong number.

But review 1 UNDERSTATED the *practical* half, and it is the more urgent one. The ceiling's O(N)-over-input
body sits inside the shared template's `while k < ITERS` loop (`gen_matrix.rs:46-54`, ITERS=16), and the two
ceiling cells ignore the per-iteration `seed` entirely (interp reads `b` from its own inner loop; native
reads `input` directly). So the 16 outer iterations re-sweep the entire N-byte input 16 times identically:
per `run()` call the work is `16 * N` interpret calls over an `~N`-node program, i.e. `~16*N^2` node-steps
(plus an equal `~16*N^2` checksum-fold steps, one fold per inner pass). At N=16384 that is `~4.3e9` node-steps
+ `~4.3e9` fold-steps per single `run()` call. With `runs_per_pass=100`, `batch_size=100`, `warmup=20`
(`bench.toml:5-8`), one worker executes ~120 such calls, and there is **no `max_call_us` anywhere in
`bench.toml`**, so the only backstop is the 300s subprocess timeout (`harness.rs:577-584`,
`.unwrap_or(300)`). The N=16384 ceiling worker will blow past 300s and be killed producing zero samples,
repeated for warm+cold across 6 passes = 12 kills at 300s each = **~1 hour of wall time spent producing
nothing for that one size point.** N=4096 (`~5.4e8` steps/call, ~0.5s/call, ~65s/worker) survives but is
slow. This is a concrete overnight-run hazard review 1 flagged only as "the slowest cell to gather stats."
The fix: cap the ceiling family's sizes (drop 16384, and 4096 is questionable) OR set a `max_call_us` so it
aborts fast OR stop multiplying the sweep by ITERS. The 16x re-sweep is pure redundancy (the cells are
deterministic and seed-independent), so it buys nothing.

### Resolved open question 3: vertical's per-input divisor. The clean fix needs no harness change and no manual recompute.

Review 1's two options were (a) an upstream `AxisValue` weight field, (b) documented manual recomputation.
Both are worse than a third that the task hint points at and that I confirm is correct: **make every
vertical cell process the same fixed number of inputs per timed call.** Pick a batch B = 8 (the widest
lane): the `scalar` cell loops 8 scalar interprets, `vert4` does two W=4 passes, `vert8` does one W=8 pass.
Then every cell's `algo_ns` is "time to process 8 inputs", the raw numbers are directly comparable, the
report's `Fastest`/`Δ mean`/subtract framing all become correct with **zero harness change**, and there is
no `/W` anywhere for a reader to forget. This is strictly better than (a) and (b) for this bench. Keep (a),
an upstream weight field, as a genuine future generalization for W-wide benches, but it is not needed to
unblock this run.

On the finding itself: **Finding 2 HOLDS, and is slightly UNDERSTATED.** Review 1 framed the risk as "a
reader comparing raw times could misread." It is stronger than that: the report's automated verdict WILL
state the wrong conclusion. `report.rs:45-56` computes `Fastest` from raw `v.algo_all.median` and
`report.rs:73-99` counts "significantly slower" variants from raw paired samples, both independent of
`normalise_mode` (subtract mode only reframes the Δ column, `analysis.rs:138`). So `vert8` doing 8x the work
per call is machine-reported as ~8x slower and flagged "significantly slower than baseline", the exact
inverse of the per-input truth, on the family the design docs call the likely headline. This is not a
latent reader-error; it is an actively wrong printed headline.

### Finding 4 (CFG ops hand-transcribed): HOLDS exactly, confirmed at all three sites.

I grepped rather than trusted the claim. `cfg.rs:73-76` hand-writes ADD/SUB/AND/MUL in the switch match;
`cfg.rs:111-120` hand-writes the identical four again in `c_add`/`c_sub`/`c_mul`/`c_and`;
`cfg_threaded.rs:94-96` writes them a third time in `h_add`/`h_sub`/`h_mul` (h_and follows). None defer to
`ops::binop_body!`. As review 1 says, no current number is wrong (the three copies agree, tests green), so
this is a drift-risk/maintainability gap, correctly rated minor. Confirmed.

### Finding 5 (untimed prep not cached): HOLDS, correctly scoped, one caveat.

The `timed!` region measures only between the two counter reads bracketing the `run { }` block
(`bench-core/lib.rs:490-497`); everything in `{prep}` is textually before it and never counted. So the
non-cached preps (`stackbc::compile` `gen_matrix.rs:222`, `optimize::optimize` `:247`, JIT codegen
`:266-267`, `trace::select_trace` `:190`) do not corrupt any measurement, exactly as review 1 says. They
DO rerun on every `entry()` call, because prep runs once per `run()` and each timed sample is one `run()`;
the OnceLock caches across calls within a worker process, the non-cached preps do not. With ~120 calls/worker
that is 120x redundant O(N) trace-selection / eqsat / JIT-codegen per worker, all wall-clock, all avoidable.
The framework's own doc mandates the OnceLock pattern for exactly this (`bench-core/lib.rs:37-45`), so the
non-cached preps violate stated discipline. Review 1's fix (extend the OnceLock) is right, with one caveat
it did not name: `OnceLock<T>` requires `T: Sync`, and `copypatch::JitCode` / `stencil::StencilCode` hold a
raw executable-memory pointer that likely is not `Sync`; caching those needs a `thread_local!` or a
`Sync`-wrapper, not a bare OnceLock. The plain-data artifacts (`opt`, `trace`, `blocks`, `sp`) cache
trivially.

### The "what is strong" section: independently spot-checked, holds where I looked.

I re-derived, not re-trusted: `native_madd` writes `results[i]` with no inline fold matching `interpret`
(`native.rs:79-115`); `access::checksum` folds once post-pass, uniform across variants and
null-floor-differenced (`access.rs:38-45`); cfg register-file is a stack `[0u64; NREG]`
(`cfg.rs:61,130`), so all four cfg cells share the same allocation shape and there is no intra-cfg alloc
asymmetry; the `ops::binop_simd!` single-source is real in `vertical.rs:47-73`; the matrix generator cleanly
separates harness-owned expansion from consumer template (`matrix.rs`), with brace/multibyte tests. Tests
green 49/49. I did NOT re-disassemble the vertical NEON claim (prior Finding 5); I take the ISA-audit
artifact on that one point.

## Additional findings review 1 missed (numbered)

### A1. Vertical allocates its SoA scratch INSIDE the timed region; the scalar baseline does not. (HIGH, correctness)

**Problem.** The vertical body calls `c::vertical::interpret_vertical_checksum::<W>(pd, &seeds)`
(`gen_matrix.rs:283`), and that function heap-allocates its result scratch on every call:
`let mut results = vec![Simd::<u64, W>::splat(0); p.nodes.len()];` (`vertical.rs:88`). This call sits inside
the template's timed `run { while k < ITERS { {body} } }` block, so the allocation is executed 16 times
per timed sample, INSIDE the measured region. Every other family, the scalar baseline included, allocates
its result buffer once in `{prep}` (`let mut r = vec![0u64; d.node_count];`, `gen_matrix.rs:117`) and
reuses it; the scalar cell's body is `c::interpret(&d, seed, &mut r)` with `r` pre-allocated. So the
vertical cells pay a `malloc`+`free` of an `N * W * 8`-byte buffer per iteration that the baseline never
pays (at W=8, N=16384 that is ~1MB alloc/free per iteration, x16 per sample).

**Why it distorts.** This is the measurement bug the whole review exists to catch, and it is worse than the
`/W` presentation issue because it corrupts the timed number itself, not its framing. It loads the vertical
cells with allocator latency the baseline excludes, inflating their measured time and pushing the SIMD
result to look slower, on top of the `/W` mis-ranking (Finding 2) which pushes the same direction. Two
independent artifacts both biasing the headline family against SIMD. It also distorts the intra-family
`vert4`-vs-`vert8` comparison (vert8 allocates a 2x-wider buffer). `access.rs`'s null-floor differencing
does not rescue this: the baseline is `scalar`, which has no matching allocation to difference against.

**Concrete fix.** Hoist the scratch into `vprep` and call the non-allocating `interpret_vertical`
(`vertical.rs:35`, which takes `&mut results`) in the body, then reduce, exactly the scalar family's
prep-once/reuse discipline. This folds naturally into the Finding-2 restructure (equal inputs per call):
one `vprep` allocates both the SoA scratch and the scalar `r`, and each cell reuses its own. File:
`gen_matrix.rs:273-292` (family), `vertical.rs:35,87-98` (the allocating wrapper is the trap; the
non-allocating form already exists).

### A2. The template uses `timed!`, not `timed_calibrated!`, with a fixed ITERS=16; at N=64 and N=256 the timed region is below the counter's own quantization floor. (MEDIUM)

**Problem.** The counter is `CNTVCT_EL0` at **24 MHz, 41.67 ns/tick** on Apple Silicon (`counter.rs:2-4`,
confirmed by reading the `mrs CNTVCT_EL0` read and the `CNTFRQ_EL0` frequency source). The framework defines
`CALIBRATION_FLOOR_TICKS = 2048` (~85 us) as the minimum region for the `+/-1` tick quantization to be
negligible (`bench-core/lib.rs:433-435`), and ships `timed_calibrated!` (`:524-559`) to auto-repeat a run
block up to that floor. The carrier template uses plain `timed!` with a hardcoded `ITERS=16`
(`gen_matrix.rs:43-57`), which does not calibrate. One interpret pass over an N-node program is ~N node-steps
at ~1 to 3 ns each, so the timed region is roughly `16 * N * (1..3) ns`. At N=64 that is ~1 to 5 us =
**~25 to 120 ticks**; at N=256, ~100 to 480 ticks. Both are well below the framework's own 2048-tick floor;
the counter resolves the N=64 region into only tens of distinct values.

**Why it distorts.** The mean over 600 samples per (variant, size, mode) (`passes=6 x runs_per_pass=100`,
`bench.toml:5-8`) averages the `+/-1` tick quantization down, so the reported *mean* survives at small N.
The casualty is *delta resolution*: the paired sign test differences carry `+/-2` ticks (~83 ns) of
quantization, so a genuinely-small dispatch delta (switch vs fntable at N=64 might be a few ns/call, i.e. a
few ticks over 16 iters) collapses into ties. The report's own ties warning (`report.rs:390-401`, "HIGH"
above 10%) will fire and correctly reduce effective sample size, so nothing lies, but the smallest-N points
lose the ability to resolve exactly the small dispatch-shape deltas the wire and predecode families exist to
measure. `timed_calibrated!` is the framework's prescribed answer and the run block is safe to repeat (it is
the fold-the-input loop the macro doc names as the safe case). For large N (region already above the floor)
it degrades to `timed!` plus one probe pass, so it is strictly better. Fix: swap the template to
`timed_calibrated!` and let it size the reps (the manual `ITERS=16` becomes redundant). File:
`gen_matrix.rs:43-57`.

### A3. Minor: several carrier modules are unbenched, so the matrix does not exercise them. (LOW, note only)

`thermo.rs`, `reach.rs`, `retract.rs`, `incr.rs`, `fusion.rs`, `liveness.rs`, `sharded_intern.rs`,
`output_building.rs` carry passing unit tests but no matrix family generates them. Not a defect in the
matrix under review (the mandate's family list is fully covered), just a note that the composition matrix
and these modules are disjoint; if any were meant to appear as a family, it is missing.

## The measurement-honesty question (does each family measure what it claims, at the metal)

- **Anti-DCE / keep-alive: sound, and better than `black_box`.** The folded `acc` is written to the FFI
  `output: &mut [u8; 8]` via `output.copy_from_slice(&acc.to_le_bytes())` inside the timed block
  (`gen_matrix.rs:55`). Because `output` is a caller-owned pointer the harness reads and cross-validates,
  the whole `acc` chain, hence every `checksum`, hence every `interpret`, is anchored and cannot be
  eliminated. The 16 iterations form a serial XOR dependency chain through `acc` plus a reuse-`r`
  WAR/RAW chain, so they cannot be reordered or collapsed. No `black_box` is needed and none is used;
  this is the correct discipline. The one cost inside the region is the single 8-byte store, uniform and
  negligible.
- **Wire / predecode / layout / residual / valrepr: honest.** One call to the function under test per
  outer iteration on the identical per-family program with identical seed derivation; scratch pre-allocated
  in prep; subtract-vs-baseline is the right framing given matched per-call cost bases. These measure
  dispatch/layout/representation and nothing else.
- **CFG: honest at the measured level.** All four cells run the identical `blocks` to the identical
  termination, stack register file, cross-validated. Finding 4 is a source-hygiene gap, not a measurement
  asymmetry.
- **Optimize: honest.** Downstream interp cost over the resulting program, `checksum_at` over live-outs so
  the changed node count stays comparable; subtract-vs-`none` is the right question. The prep is heavy and
  uncached (Finding 5), a wall-clock cost, not a measurement error.
- **Native tier (interp/copypatch/stencil): honest where it runs, but incomplete.** Byte-exact
  cross-validated; the program crosses as opaque bytes so no cell sees another's work. Finding 1 removes
  copypatch/stencil at N>=4096; cap the sizes so the tier is honest about its range.
- **Native ceiling: honest within the family, on a different (intentional) cost basis than its siblings.**
  interp-vs-native is a fair throughput comparison; do not reshape it. Its raw ns are not on the sibling
  scale, and its N=16384 point is a run-time hazard (see the Finding 3 audit).
- **Vertical: the one family that does NOT currently measure what it claims.** Two compounding artifacts,
  the in-timed-region allocation (A1, corrupts the number) and the unequal inputs-per-call (Finding 2,
  mis-ranks the report), both bias the headline SIMD result to look slower than the metal actually is. This
  is the family to fix before trusting any number from it.

## The bottom line for op (minimal set before the run; and the caveats)

Must land (each is carrier-side, no harness change):

1. **Vertical: hoist the SoA scratch into prep and equalize inputs-per-call.** Fixes A1 (allocation in the
   timed region, a real corruption) and Finding 2 (report verdict inversion) together. Use B=8 inputs per
   timed call for scalar/vert4/vert8 with pre-allocated scratch and the non-allocating `interpret_vertical`.
   This is the single most important change; the headline family is currently untrustworthy.
2. **Native ceiling: cap sizes (drop 16384; reconsider 4096) or set a `max_call_us`.** Otherwise the top
   size blows past the 300s subprocess timeout and burns ~1 hour producing no data. The intentional
   throughput shape stays; only the runaway size is removed.
3. **Native tier (copypatch/stencil): cap `native_family` sizes to `[64, 256, 1024]`.** Below the imm12
   window with margin; declares the technique's real range instead of discovering it via 24 crash logs. The
   run survives without this (the driver tolerates the crashes, resolved above), but the fix is trivial and
   satisfies the "name it, don't silently drop it" mandate.

Strongly advised, cheap, not strictly blocking:

4. **Swap the template to `timed_calibrated!`** so N=64/256 regions clear the 24 MHz counter's quantization
   floor and small dispatch deltas stay resolvable (A2).
5. **Cache the heavy preps** (`optimize`, `trace`/`blocks`, `sp`) via the existing OnceLock pattern; use a
   `thread_local!` for the non-`Sync` JIT artifacts (Finding 5). Wall-clock only, but it compounds with the
   native-ceiling and JIT costs to make the large-N points needlessly expensive.

Documented caveats (no code change required, but state them in the findings so nobody misreads):

6. **`carrier_native_ceiling`'s raw ns are throughput-over-a-byte-stream, not per-program-execution
   latency.** Never normalize them against a sibling family. Do NOT "fix" this by reshaping the body.
7. **CFG's ADD/SUB/MUL/AND are transcribed at three sites** (`cfg.rs:73-76`, `cfg.rs:111-120`,
   `cfg_threaded.rs:94-96`); a future op-semantics change must touch all three plus `ops::binop_body!`. No
   current number is affected.

On review 1 overall: its ten re-verified strengths and its Findings 4 and 5 hold as written; Finding 1 holds
on substance but its stall-urgency framing is overstated (the driver tolerates crashed workers, proven);
Finding 2 holds and is slightly understated (the report actively prints the wrong verdict) with a cleaner
fix than either option it offered; Finding 3 holds on comparability, its intentional-vs-accident question
resolves to intentional (so its alternative "reshape the body" fix is wrong), and it understated the
concrete 300s-timeout hazard. The one class review 1 missed is the in-timed-region allocation asymmetry (A1),
which is the most serious measurement defect in the matrix, and the counter-quantization-vs-region-size issue
(A2). None of these block the *mechanics* of a run; A1, Finding 2, and the native-ceiling timeout block
*trusting the numbers it would produce*.
