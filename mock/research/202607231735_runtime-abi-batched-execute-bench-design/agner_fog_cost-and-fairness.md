# Runtime C ABI batched-execute bench arc: cost and fairness

**Date:** 2026-07-23
**Lens:** instruction-level and microarchitectural cost of crossing the host-to-runtime C ABI, and whether a
given bench measures the boundary crossing it claims rather than an artifact.
**Read against:** `00_context.md` (the brief), the carrier at `mock/benches/carrier/`, and the harness at
`~/Dev/clause-dev/mockspace/{bench-core,bench-matrix,bench-harness}`.

Notation for evidence provenance, used throughout: **[M]** = measured or read directly from source I cite by
`file:line`; **[I]** = inferred from established aarch64 microarchitecture and to be confirmed by a local
disasm or PMU probe before any number is trusted. I do not let an **[I]** stand as a fact.

## One-line verdict on the candidate decomposition

The axes are the right axes, but the candidate is built on a boundary the current harness does not actually
time: as wired today the C ABI is crossed **once per batch of internal work**, not once per record, so the
existing `bridge_ns` column measures scaffold bookkeeping, not the crossing. The arc's first and load-bearing
task is to move the crossing **into** the calibrated timed region as a genuine cross-object call, which needs a
new upstream `boundary` regime plus runtime-artifact staging (the same shape of upstream addition the `stream`
regime was). Two further defects: candidate benches (1) and (2) ride the same 1/W curve and will alias FFI
amortization with SoA amortization unless (1) is pinned to a W-invariant scalar payload as the isolator; and the
candidate under-weights the **return path** (output-arena decode, per-call environment establishment), which are
two whole missing categories. Fix those three and the decomposition is sound.

## Why the existing harness does not measure the boundary (the structural finding)

The scaffold cell is a generic `FnMut(&mut St, u64) -> u64`, monomorphized and inlined into the variant cdylib
by fat LTO (`bench-matrix/src/scaffold.rs:94-155`, decl rationale `bench-matrix/src/decl.rs:1-11`). The
calibrated loop (`timed_calibrated!`, `bench-core/src/lib.rs:556-597`) runs entirely **inside** the cdylib: it
repeats the cell `reps` times, all within one `extern "C"` entry invocation.

The harness crosses the FFI boundary in `run_worker` exactly once per `on_algo_call`
(`bench-harness/src/harness.rs:355-380`): `entry(input, output, n)` is called, and `bridge` is computed as
`call_accum - algo_accum` (`harness.rs:409-413`). `call_accum` brackets that single `entry()` call, whose body is
the whole scaffold: setup timing, a cold first-touch pass of `ITERS=16`
(`scaffold.rs:117-127`, `ITERS` at `scaffold.rs:44`), a digest pass of `ITERS`
(`scaffold.rs:132-139`), a probe pass, and the calibrated `reps` loop. `algo_accum` is one calibrated per-pass I
term (`scaffold.rs:144-154`).

So `bridge_ns` is dominated by the scaffold's internal first-touch and digest passes, not by the call
instruction. **It is not the boundary cost and cannot be used as the boundary metric.** The `on_algo_call` closure
fires only a handful of times per batch sample (once per `AlgoCall` item per selected program,
`bench-harness/src/workload.rs:432-441`), each crossing running an entire `ITERS * reps` interpret loop. The
crossing is amortized to invisibility by construction.

The correct realization is the one the brief floats (lines 83-87): a two-object shape where the timed region
itself performs the cross-object calls. The crossing becomes the measured op, crossed `N/W` times per calibrated
pass, through a fn pointer the optimizer cannot devirtualize.

## Measurement backbone for a per-call boundary bench

Hold the total record count **N** fixed per bench (N is the matrix `sizes` sweep, one value per bench). Sweep the
batch width **W** as the matrix axis. Per calibrated pass the cell issues `N/W` cross-object calls, each feeding W
records. The per-pass cost decomposes as

```
pass(N, W) = (N/W) * C_cross(W) + N * I_payload(W)
```

where `C_cross(W)` is the fixed per-crossing cost (call + return + prologue + argument marshalling that scales
with W), and `I_payload(W)` is the per-record execution cost, which for the column-SoA form itself falls with W
as dispatch amortizes over lanes. Fit each variant's `total(k) = S + k*I` with the harness OLS fit
(`bench-harness/src/analysis.rs:513-557`, `CostModel` at `:495-501`) over a geometric ladder, taking `k = N/W`
for the crossing-count axis at fixed N, or `k = N` across the size sweep for the payload axis. Report `r2`; a poor
`r2` is the signal that a regime change is hiding under a single number (`analysis.rs:483-494`).

The counter quantum sets the floor discipline. **[M]** `CNTVCT_EL0` runs at 24 MHz, 41.67 ns/tick
(`bench-core/src/counter.rs:2-3`, `read_counter` at `:37-77`); the calibration floor is 2048 ticks, about 85 us
per pass (`CALIBRATION_FLOOR_TICKS`, `bench-core/src/lib.rs:464`, `calibrate_reps` at `:476-483`). **[I]** A single
well-predicted cross-object crossing is order 1 to 3 ns on an M1 P-core (~3.2 GHz, ~0.31 ns/cycle): roughly
1/20th of one counter tick, pure quantization noise on its own. Calibration is what makes it measurable: the pass
repeats until it clears 2048 ticks, and the W-sweep plus the boundary-free floor separate the crossing term from
the payload term. No new timing mechanism is needed, but the cell at each W must issue enough crossings that the
crossing term is a real fraction of the pass. The W-sweep does this automatically: at W=1 the `N` crossings
dominate the pass; at W=N a single crossing is negligible and the pass is all payload. That sweep from
crossing-dominated to payload-dominated **is** the amortization curve, read directly, not asserted.

### The floors that make the numbers legible

Four floors, each isolating one term by subtraction. The first two are specific to the boundary and are the load-bearing additions.

- **Boundary-free floor `F_bf` (in-process, same-object, inlinable).** The identical payload over identical wire
  bytes, called as a directly-named or generic-`FnMut` Rust function the optimizer inlines, no cross-object edge.
  Then `ffi(W) - F_bf(W) = (N/W) * C_cross(W)` isolates the crossing. Discipline: `F_bf` must be a genuine direct
  inlinable call, not a same-object `fn` pointer the optimizer keeps indirect, or the floor already pays a call
  and the delta undershoots.
- **Null-entry floor `F_null` (cross-object, empty payload).** A cross-object entry that returns W zeros and does
  no interpret work. `F_null(W) = (N/W) * (C_cross(W) + C_marshal(W))`: the pure crossing plus marshalling with
  zero payload. This is the input-side twin of the candidate's null-sink and is the single number the ABI decision
  most needs. Exempt from cross-validation (its output is not the payload output), per the floors-exempt rule
  (brief line 68).
- **Native ceiling `F_nat`.** The shape-specialized native madd (`native_madd_over_input`,
  `carrier/src/bench/native_ceiling.rs:48`), crossed. The payload floor: interpret dispatch removed, so
  `C_cross / F_nat` is the crossing as a fraction of the cheapest real payload, which is exactly the ratio that
  decides whether batching pays (see category D).
- **Null-dispatch `F_nd`.** The existing `interpret_nulldispatch` (`carrier/src/bench/dispatch.rs:52`), the
  in-payload dispatch floor.

Cross-validate `ffi` against `F_bf` on the reps-invariant digest (`scaffold.rs:132-139`, digest field
`bench-core/src/lib.rs:417-424`): both run byte-identical payload, so digests must match; only the call edge
differs. A digest mismatch means the two sides are not running the same computation and no timing is trustworthy.

## Confounds and fairness traps specific to an FFI-boundary bench

Each: the trap, why it costs, the check that defeats it, and where it applies.

1. **The optimizer inlines or partial-evaluates across the boundary.** If the "runtime" is a function in the same
   cdylib as the host cell, fat LTO (`lto=fat`, the isolation premise, `scaffold.rs:9-15`) inlines it and there is
   no boundary at all. Cost: the whole bench measures native-vs-native, the exact failure the carrier README
   already names for the earlier native-ceiling bench (`carrier/src/lib.rs:16-19`). Check: the runtime is a
   **separate** cdylib, resolved via `libloading::Symbol` at cell-setup and called through the stored fn pointer;
   disassemble the cell and confirm a `blr` (register-indirect branch) to the resolved pointer, not a `bl` to a
   named symbol and not an inlined body. The disasm gate already exists (`bench-harness/src/disasm.rs`). Applies
   at the new cell's `St`, which holds the `libloading::Library` plus resolved pointers, mirroring the way
   `carrier/src/bench/dispatch.rs:16-19` holds decoded state in `St`.

2. **The runtime partial-evaluates its interpreter over the program.** Even across a real dylib edge, if the
   runtime cdylib holds the program as a compile-time constant, its own LTO specializes the interpret loop over it
   and the payload collapses. Cost: `I_payload` is understated, `C_cross` is overstated by subtraction. Check:
   the program crosses into the runtime as wire bytes (a `*const u8` argument or opaque runtime state built from
   bytes at setup), never as a generic const. This is already the carrier's structural property
   (`carrier/src/lib.rs:16-19`, `program_at` at `carrier/src/lib.rs:94-98`); the runtime cdylib must preserve it.

3. **A batch shape that hides per-record cost under memory latency.** If a batch of W records is W **distinct**
   programs each chased through pool indirection, per-record cost is dominated by cache-miss latency (the 24-byte
   3-operand node, pool indirection the expensive access shape, brief line 60, `carrier/src/ir.rs:300-311`), and
   the 1/W FFI term is unmeasurable under the stalls. Cost: the amortization knee vanishes into memory noise and
   the bench reports "batching does not help" for the wrong reason. Fair shape: the payload is the **same**
   predecoded, cache-resident program run over W distinct **seeds** (the vertical shape,
   `carrier/src/bench/vertical.rs:65-105`, `carrier/src/vertical.rs:35-78`), so per-record cost is compute-bound
   and the crossing term is visible against it. The distinct-programs shape is a separate, deliberately-labeled
   cold regime (see confound 7), never the default. Applies at every cell's per-record loop.

4. **The boundary-free floor is not actually boundary-free (either direction).** If `F_bf` calls the payload
   through a `fn` pointer the optimizer keeps indirect, the floor already pays a call and `C_cross` undershoots.
   If `F_bf` is over-inlined because the same-object optimizer sees and const-folds the program, the floor
   undershoots the payload and `C_cross` overshoots. Cost: the headline crossing number is wrong in one direction
   or the other. Check: `F_bf` runs the same carrier interpret over the same opaque wire bytes as the runtime
   (confound 2 discipline on both sides), called directly; symmetry confirmed by the digest match (confound 1
   cross-validation). Applies at the `F_bf` cell body.

5. **The runtime sees W at compile time when it should not.** For the per-W monomorphized entries (`execute_w8`),
   W as a const inside the runtime is correct: that is the vehicle (candidate 3, brief lines 99-101). But for the
   runtime-W entry (W as argument), if the bench lets the runtime specialize on a constant W it measures the
   monomorphized shape mislabeled as runtime-W. Cost: the entry-form comparison collapses and the arc "concludes"
   there is no cost to runtime W. Check: the host passes W as a runtime value from the swept size, and disasm of
   the runtime-W entry shows a data-dependent loop bound in a register, not an unrolled constant. Applies at
   bench IB-3's two runtime entries.

6. **Counter quantum vs a single crossing (already handled, stated so it is not re-broken).** One crossing is far
   below the 41.67 ns tick **[M]**. Calibration handles it (`bench-core/src/lib.rs:568-592`). The trap is
   authoring a cell whose single pass is dominated by one crossing at large W: at W=N the pass is essentially one
   crossing plus payload, and the crossing is genuinely negligible there, which is the correct answer, not a
   defect. Do not "fix" it by forcing extra crossings; the W-sweep is the instrument.

7. **The indirect predictor makes a monomorphic crossing near-free.** A call site that always targets the same
   address is perfectly predicted after warmup, so the steady-state crossing is the call and return micro-ops plus
   argument setup, not a mispredict **[I]**. The warm regime measures exactly this best case (`scaffold::warm`,
   `scaffold.rs:94`). The many-residuals-per-frame deployment thrashes the indirect predictor; that mispredicted
   crossing is **[I]** several times more expensive (order 8 to 16+ cycles on a wide OoO aarch64 core, to be PMU
   confirmed). Cost: reporting the warm number alone understates the boundary in the real deployment. Check: a
   boundary-specific cold regime that cycles distinct **entry points / distinct runtime targets** by iteration
   index, not just distinct programs, so it stresses the indirect predictor at the boundary. This is a new cell
   shape on top of `scaffold::cold_cycle` (`scaffold.rs:163`, which today cycles programs, not call targets).
   Report both regimes; never let warm stand as "the boundary cost."

8. **Argument marshalling charged to the wrong side, and the sink is two crossings not one.** The
   `reserve(hint) / commit(n)` sink is a `#[repr(C)]` struct of two function pointers plus userdata (brief lines
   27-28), so each output batch is **two** additional cross-object calls. At W=1 that is two extra crossings per
   record, potentially larger than the execute crossing itself. Cost: a per-record reserve/commit sink can
   dominate the whole boundary and be mis-attributed to execute. Check: isolate the sink on its own axis (bench
   IB-4) with a null-sink floor, and count sink crossings explicitly (2 per batch, amortized over W outputs).
   Applies at the output-side cells.

9. **PMU asymmetry, used as a second witness.** perf reads instructions and cycles around the batch region
   (`bench-harness/src/harness.rs:343,399-402`, `bench-harness/src/perf.rs`). Because the crossing is now inside
   the timed cell, instructions-retired per pass includes the call, return, and marshalling. Check: the
   instruction-count slope across the W-sweep should track the timing slope (`~(N/W)*cross_insns + N*payload_insns`).
   If timing and instruction slopes disagree, the timing carries a confound the instruction count does not
   (frequency scaling, a memory stall). This cross-check is free from `perf.rs` and should be a mandatory gate on
   any `C_cross` claim, not an optional column.

## Category gaps in the candidate decomposition

Whole classes of boundary-cost question the candidate does not name, versus permutations of its listed axes.

- **A. Output-arena decode and validation.** The value crosses back as a region-structured value-arena with a
  **typed structural decode on the untrusted path** (brief lines 26-29). The candidate benches the sink's
  reserve/commit crossings (axis 4) but never the host-side **decode** of the returned W-output arena. A batched
  execute that produces W outputs the host must structurally validate can be dominated by that per-output decode.
  Missing bench: output-arena decode cost as a function of W and region structure.
- **B. Per-call environment establishment vs a persistent runtime handle.** Each entry may re-establish the
  runtime-environment interface descriptor (the FIXME deferred in the shipping ABI, brief line 34). If every
  batched call rebuilds env context, that is a fixed per-call cost that does **not** amortize over W within a call
  but **does** amortize if the host holds a runtime handle across calls. This is a first-order ABI shape question
  (does `execute` take a fresh env or a held handle?) with a measurable per-crossing cost, and the candidate never
  raises it.
- **C. Partial-batch and error signaling on the return path.** A W-record batch where K records fail needs an ABI
  shape to report it: a per-record status array (W status writes/reads, a real per-W cost), a first-error index,
  or an out-param. The perf numbers must not silently assume the all-success path. At minimum floor the
  per-record-status-write cost; the shape choice is a correctness-arc question but its per-W cost belongs on this
  arc's ledger.
- **D. The batched-ABI win scales with the crossing-to-payload ratio, and that is tier-dependent (the headline the
  candidate buries).** The Tier tag narrows to interpret-arena vs run-bytecode, and native is a per-region
  strategy (brief lines 24-25, 32). The boundary matters most exactly where the payload is cheapest: a native-tier
  region makes the crossing a large fraction of total, an interpret-arena region makes it negligible. So "does a
  batched entry pay" is not a yes/no; it is `C_cross / I_payload`, largest at the native tier, which is precisely
  the case a scalar-per-record ABI most penalizes. The vertical/SoA result the brief calls the biggest win (lines
  45-49) is one instance of this law. The candidate pins "payload interp" as axis 4 but never frames the ratio as
  the decision variable; it should be the matrix's headline plot (win vs `C_cross / I_payload`), not a footnote.
- **E. Cross-call pipelining.** A synchronous per-call bench cannot see a deployment that issues W-batches back to
  back and overlaps the next crossing with the prior batch's payload under out-of-order execution. The `stream`
  regime (`scaffold.rs:226`) measures throughput over a byte stream but not cross-call overlap. Name it as a
  category the synchronous measurement understates, so the selector does not over-penalize small W on a throughput
  deployment.

## The individual benches, as I would build them

Realization: build **one** shared `carrier-runtime` cdylib exposing every candidate entry point
(`scalar_execute`, `execute_w4/w8/w16/...` monomorphized, `execute_runtime_w(w)`, `null_entry`, and the
`reserve`/`commit` sink pointers). The boundary under test is fixed; only the **host driver** varies across
variants. Each host variant dlopens the one runtime artifact at cell-setup and calls the subset it measures. The
per-W monomorphization lives once in the runtime cdylib, compiled under its own fat LTO. This preserves standard
per-variant host-cdylib isolation while making every crossing a genuine cross-object call. The one upstream
addition: a `scaffold::boundary` regime whose setup returns a state that has dlopen'd the runtime and resolved
the pointers, and a harness/generator capability to build and stage the single runtime `.dylib` as a sibling and
hand its path to workers (`bench-harness/src/driver/staging.rs` is where staging lives). This is the same class of
upstream change as the `stream` regime (brief lines 78-79), and it goes into `mockspace-bench-matrix`, not the
carrier.

- **IB-1: boundary amortization over W, payload pinned scalar.** Regime: `boundary`/warm. Sweep W in
  {1,2,4,8,16,32,64,128}; fixed N per bench across `sizes`. Cells: `ffi_batched_scalar` (runtime loops internally,
  scalar interp per record, W-invariant payload), `inproc_batched_scalar` = `F_bf`, `null_entry` = `F_null`.
  Baseline `inproc`. Isolates `C_cross` as `ffi - F_bf`, and pure crossing+marshalling as `F_null`. This is the
  FFI-amortization isolator; its W-invariant payload is what makes IB-2 attributable.
- **IB-2: column-SoA win across the boundary.** Same W-sweep. Cells: `ffi_batched_scalar` (baseline, payload
  W-invariant), `ffi_column_soa` (payload amortizes dispatch over W lanes via the real SoA interpret,
  `carrier/src/vertical.rs:35-78` behind the runtime entry). The delta `soa - batched_scalar` isolates the
  vectorization term **on top of** the shared FFI term. Because IB-1's control has W-invariant payload, the two
  1/W knees (FFI amortization, SoA amortization) are separated by construction rather than aliased. This is the
  fix to the candidate's benches (1)+(2) riding one axis.
- **IB-3: entry-point form.** Fixed payload (column-SoA), W-sweep restricted to the compiled monomorphizations.
  Cells: `execute_wN_mono` (per-W const entry, unrolled/NEON-packed in the runtime), `execute_runtime_w` (W as
  argument, internal data-dependent loop capped at Wmax). Delta is the cost of losing compile-time W at the
  boundary. Fairness gate: disasm both runtime entries (confound 5), confirm `execute_wN` is unrolled/vectorized
  and `execute_runtime_w` has a register loop bound.
- **IB-4: output-sink shape (with the return-path decode, gap A).** Fixed execute (column-SoA, one W), sweep the
  sink. Cells: `null_sink` (no output crossing, baseline), `per_record_reserve_commit` (2 sink crossings per
  record), `batched_reserve_commit` (2 sink crossings per batch of W), `batched_reserve_commit_decode` (adds the
  host-side structural decode/validation of the returned W-output arena). Delta null->per_record is the per-record
  sink crossing; per_record->batched is the sink batch amortization; batched->decode puts the untrusted-path
  decode on the ledger.
- **IB-5: call-scoped vs session-scoped environment (gap B).** Cells: `fresh_env_per_call` (each execute
  establishes a runtime-env descriptor), `held_handle` (env established once at setup, reused). Delta is the
  per-call context-establishment cost and whether it amortizes over W. Small, but it answers whether `execute`
  must take a persistent runtime handle, which is an ABI-shape decision.

All five run over the six designed profiles (P_real / P_madd / P_tight / P_scatter / P_wideselect / P_leaf,
`carrier/src/lib.rs:127`, `carrier/src/bench/dispatch.rs:27`) and both a warm regime and the boundary-specific
cold regime (distinct call targets, confound 7).

## The composition matrix, as I would build it

After the individuals teach the terms, cross: **boundary_form** {scalar_per_record, batched_scalar, column_soa}
x **W** {1..128} x **entry_form** {mono_wN, runtime_w} x **payload** {scalar, soa, native} x **sink** {null,
per_record_rc, batched_rc, batched_rc_decode} over the six profiles x {warm, cold-distinct-target}. Prune to
coherent cells: `scalar_per_record` exists only at W=1; `column_soa` only with `soa` payload; `mono_wN` only at
its compiled W. Report, per cell, the `(S, I)` cost line and `r2` from `fit_cost_model`
(`bench-harness/src/analysis.rs:513-557`); floor-differenced ratios against `F_bf` and `F_null` using the
harness's floor-and-ratio machinery (`with_floor` / `floor_mean` / `normalise_mode = "ratio"`,
`analysis.rs:275-333`); an oracle envelope (min per-record cost over forms, per profile per W); and a selector
that emits, for each (profile, tier/payload, deployment regime), the boundary form and W that minimize per-record
cost. The selector's output **is** the ABI decision: whether a batched/column entry pays, at what W it pays, and
in which entry-point form.

The matrix's headline is not a table but a plot: batched-ABI advantage against `C_cross / I_payload` (category
D). It shows the win is largest at the native tier and smallest at heavy interpret, which is the general law the
vertical result is one point on. That plot, plus the two-knee separation from IB-1/IB-2, is what makes the
decision evidence-backed rather than a single ratio at a single W.

## Open questions for the synthesiser

- **Realization.** Confirm the single-shared-runtime-cdylib shape (one artifact, all entries; host variants vary
  only the driver) over a per-variant runtime. The shared shape fixes the boundary and isolates the host-driver
  axis cleanly; it needs the harness to stage one sibling `.dylib` and pass its path to workers. Is the
  `scaffold::boundary` regime plus staging the right upstream unit, mirroring how `stream` was added?
- **Wmax for the runtime-W entry.** The ABI's maximum batch width sets the runtime-W internal buffer sizing and
  the top of the W-sweep. What is it?
- **Return-path scope.** Is the output-arena decode (gap A) and per-call env establishment (gap B) in scope for
  this perf arc, or does the arc floor them and defer the shape choice to a correctness arc? I would floor both
  now; the numbers are cheap and the ABI shape hinges on them.
- **Cold regime definition for the boundary.** Confirm the boundary-specific cold regime cycles distinct call
  targets (stresses the indirect predictor at the edge, confound 7) rather than distinct programs (stresses the
  payload predictor). Both are worth having; they answer different questions and must not be conflated under one
  "cold" label.
- **PMU slope as a gate.** Make the instruction-count slope cross-check (confound 9) a mandatory acceptance gate
  on every `C_cross` figure, not a reported-and-ignored column. Agreed?

## Sources

Web search was unavailable this session (budget exhausted), so every hardware figure here is either read from the
counter source I cite (`bench-core/src/counter.rs:2-3`, the 24 MHz / 41.67 ns/tick CNTVCT figure) or marked
**[I]** as a microarchitectural inference to be confirmed by a local disasm (`bench-harness/src/disasm.rs`) and
PMU probe (`bench-harness/src/perf.rs`) before it is trusted. No external number is asserted as fact.
