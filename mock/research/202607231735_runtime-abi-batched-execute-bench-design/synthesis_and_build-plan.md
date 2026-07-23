# Synthesis and build plan: the runtime C ABI batched-execute bench arc

**Date:** 2026-07-23
**What this is:** the synthesis of the three design forks (agner-fog cost+fairness, chris-fallin
entry-forms+codegen, haoran-xu breadth+honesty) into one buildable arc, plus the genuine open decisions for op.
It supersedes the candidate decomposition in `00_context.md` where they conflict. The build follows op's directive
verbatim: build ALL of it (individuals then a composition matrix), fairness-audit every cell, hand the built
machinery to an expert panel, and run NO bench until the panel clears it.

## The convergence, and the four verified structural facts

Three independent lenses converged on one shape, which is the signal the shape is right. Before the design, the
four load-bearing facts, each read directly from source (not taken on a fork's word):

1. **The current harness does not time the boundary crossing.** `run_worker` calls `entry(input, output, n)` once
   per `on_algo_call`, times the outer call as `call_accum`, reads the variant's own internal `run_ticks` as
   `algo_accum`, and reports `bridge = call_accum.saturating_sub(algo_accum)` (`bench-harness/src/harness.rs:346-421`).
   The whole interpret loop runs inside that one `entry()` call, so `bridge_ns` is scaffold setup/first-touch/digest
   plus a single crossing, never a per-record or per-batch crossing cost. **Consequence:** the arc's first task is a
   new upstream regime that puts genuine cross-object calls INSIDE the calibrated timed region.
2. **The dispatch-table entry form already exists and runs.** `#[bench_variant]` generates
   `bench_entry(input, output, n)` whose body is `match n { N => { const N = ...; func::<N>(...) } }`
   (`bench-macro/src/lib.rs:280-335`): a single exported symbol, a runtime `n`, dispatched by a match to statically
   monomorphised `::<N>` bodies. **Consequence:** op's "generic `execute<const W>` monomorphised and mapped at the C
   boundary" vehicle has a working precedent, and it has TWO distinct C-boundary lowerings, not one.
3. **`MatrixDecl` carries exactly one swept axis** (`bench-matrix/src/decl.rs:64-89`: one `sweep: SweepAxis`, flat
   `sizes`, flat `cells`). **Consequence:** the composition matrix is a curated flat cell list (the `entgrid`
   precedent) unless a multi-axis sweep is added upstream.
4. **`BenchEntryFn`'s `n` is the size slot** (`bench-core/src/lib.rs:429-430`). **Consequence:** batch width W and
   program size collide if both ride `n`/`sizes`; they must be separated.

## The reconciled axis model: boundary form is a derived label, not a swept axis

The candidate's "boundary form" bundled three independent things (records-per-call W, wire layout AoS vs SoA, and
whether the runtime vectorises) that both chris-fallin and haoran-xu independently unbundled. It is a report-time
label over a coordinate, not a dimension. The real independent axes:

- **Entry-point form** (four, not two). Scalar per-record (the W=1 anchor); runtime-W (one symbol, W a runtime
  argument, internal loop); dispatch-table (one symbol, runtime W, `match W` to N monomorphised bodies, fact 2);
  per-W monomorphised set (N symbols `execute_wK`, W baked into symbol identity, caller resolves once). The vehicle
  question op raised is precisely dispatch-table versus per-W-set: same monomorphised bodies, differing only in
  where the W-to-body binding happens (callee-side one branch per call, versus caller-side once then zero branches).
- **Payload** (scalar / vertical-SoA / native-ceiling). **Structurally coupled to entry form** (chris-fallin's
  sharpest catch): `Simd<u64, W>` is a distinct concrete type per W (`carrier/src/vertical.rs:35`, gated by
  `LaneCount<W>: SupportedLaneCount`), so the runtime-W entry can ONLY wrap the scalar interpreter; the vertical
  payload is reachable only through the dispatch-table or per-W-set forms. Several candidate cells are therefore
  structurally impossible and must be pruned, not measured.
- **Batch width W** (the sweep: 1,2,4,8,16,32,64,128), dedicated to its own slot, never sharing `n` with program
  size (fact 4).
- **Marshalling layout** (AoS / SoA-native / SoA-transposed), split from vectorisation (haoran, Arrow C Data
  Interface precedent): AoS-W64 and SoA-W64 are different marshalling costs at identical W before anything
  downstream vectorises, and the AoS-to-SoA transpose is charged as an explicit timed stage, never hidden in the
  vectorisation win.
- **Output sink** (per-record reserve/commit / batched columnar / null-floor / +host-side decode), built as the
  real settled `#[repr(C)]` two-function-pointer struct called indirect, deliberately outside the scaffold's
  `FnMut` convention (haoran trap 5; the sink is TWO crossings per batch, agner confound 8).
- **Instance lifecycle regime** (haoran): at the ABI layer "cold" means a fresh runtime instance (fresh residual
  load, region reservation, tier warmup) versus a long-lived instance taking many batched calls, NOT the
  interpreter arc's program-identity warm/cold. Named `instance_lifecycle` to avoid the mislabel. Plus agner's
  boundary-specific cold: cycle distinct CALL TARGETS to stress the indirect predictor at the edge.
- **Language** (Rust-stand-in / Zig): the Rust cdylib answers the language-agnostic ISA-level crossing cost; it does
  NOT answer the Zig callee-side codegen cost (chris, haoran). A real Zig cdylib cell is a distinct category the
  five candidate axes cannot express.

## The measurement backbone

Per calibrated pass, holding total record count N fixed and sweeping W (agner):

```
pass(N, W) = (N/W) * C_cross(W)  +  N * I_payload(W)
```

`C_cross(W)` is the fixed per-crossing cost (call + return + marshalling that scales with W); `I_payload(W)` is
per-record execution, which for the SoA form itself falls with W as dispatch amortises over lanes. Fit each
variant's `total(k) = S + k*I` with the harness OLS fit (`bench-harness/src/analysis.rs:513-557`), `k = N/W` for
the crossing axis, report `r2` (a poor fit flags a hidden regime change). The W-sweep from crossing-dominated
(W=1) to payload-dominated (W=N) IS the amortisation curve, read not asserted. The 41.67 ns CNTVCT quantum
(`bench-core/src/counter.rs:2-3`) makes a single ~1-3 ns crossing sub-tick noise; the 2048-tick calibration floor
(`bench-core/src/lib.rs:464`) is what lifts it into measurability, so no new timing mechanism is needed, only
enough crossings per pass, which the W-sweep supplies.

**The floor ladder** (merging agner's four with chris's indirect-call middle rung): (a) direct in-process
inlinable call to the monomorphised body, the true zero; (b) in-process call through an already-resolved `fn`
pointer, no dylib boundary, isolating indirect-call cost alone; (c) null-entry cross-object call, empty payload,
isolating pure crossing + marshalling (the single number the ABI decision most needs); (d) native ceiling, crossed
(`carrier/src/bench/native_ceiling.rs:48`); (e) null-dispatch, the in-payload dispatch floor
(`carrier/src/bench/dispatch.rs:52`). Rung (b) is what separates "indirect call" (paid in every real deployment)
from "crossing an object boundary," which a single boundary-free floor conflates.

**The headline is a ratio, not a yes/no** (agner category D): the batched-ABI win scales with `C_cross /
I_payload`, largest exactly where the payload is cheapest (the native tier), which is the case a scalar-per-record
ABI most penalises. The vertical/SoA win the brief calls the biggest result is one point on that law. The matrix's
headline output is a plot of batched-ABI advantage against `C_cross / I_payload` across the six profiles, not a
single ratio at a single W.

## The realization: one shared runtime cdylib, thin host variants

All three forks converge (agner, chris explicitly; haoran via the harness dlopen): build ONE shared
`carrier-runtime` cdylib exposing every entry point (`execute1`, `execute_wK` monomorphised, one `execute`
dispatch-table symbol, `execute_runtime_w`, `null_entry`, and the `reserve`/`commit` sink pointers), all wrapping
the UNMODIFIED `carrier/src/interp.rs` and `vertical.rs` bodies (zero new payload logic), built once under its own
fat-LTO. N thin `#[bench_variant]` host variants each `dlopen` the one runtime artifact at `setup` (resolving the
symbol OUTSIDE the timed region, satisfying chris trap 3 for free) and drive the subset they measure in `cell`.
This fixes the boundary and isolates the host-driver axis; it reuses the `carrier-zig` shared-object precedent
(chris) and avoids the per-cell host/runtime crate-pair duplication smell. The boundary under test is a genuine
cross-object `blr` through a resolved pointer the optimiser cannot devirtualise, crossed N/W times per pass, which
the current harness's single once-per-`bench_entry` dlopen cannot provide (fact 1).

**Why the two-object split, precisely** (chris): not to prove FFI costs something (known), but to make the
per-batch-call COUNT itself a real, un-elidable, ISA-visible cost. A same-crate loop calling the payload W times
under `lto=fat` inlines "W calls" and "one call over W records" into indistinguishable code, and the axis under
test vanishes into the optimiser. The separate cdylib is what keeps each simulated crossing a genuine call.

## The upstream work (to mockspace-bench-matrix / -harness, per everything-upstream)

- **`scaffold::boundary` regime** (required, the analogue of how `stream` was added): `setup` returns a state that
  has `dlopen`'d the shared runtime and resolved the entry pointers; `cell` issues N/W cross-object calls in the
  timed region. Plus a harness capability to build and stage the single sibling runtime `.dylib` and hand its path
  to workers (staging lives at `bench-harness/src/driver/`).
- **A boundary-cold regime** that cycles distinct call targets (agner confound 7), distinct from the existing
  `cold_cycle` which cycles programs, and an `instance_lifecycle` regime (haoran) that re-establishes runtime state
  per invocation.
- **A minimum-reps floor across the W sweep** (haoran trap 1): the calibration picks fewer reps for large-W calls
  to hit the same wall-clock floor, so the high end of the sweep (the region the "does the win survive" question
  cares about most) gets the least statistical power and most first-touch contamination, and reads as false
  "diminishing returns." Enforce comparable reps, report `batch_count` (already a worker column,
  `harness.rs:134`).
- **The multi-axis sweep** is NOT built. `MatrixDecl`'s one-axis constraint (fact 3) is worked around with the
  curated `entgrid`-style flat cell list, which already expresses a cross in this tree
  (`carrier/src/bench/entgrid.rs`). A native multi-axis declaration is a real harness capability gap but orthogonal
  to the ABI question; flagged for a future harness arc, not built here (see op-fork C).

## The individual benches (the reconciled union)

Each isolates one axis; downstream benches pin the upstream ones' best point rather than re-sweeping.

0. **ISA-shape gate** (not a timing bench, chris IB-0): disassemble the call site and callee body for every entry
   form and confirm each is the shape it claims (per-W-set = direct call through a resolved pointer, no branch;
   dispatch-table = jump-table/predictable branch on W; runtime-W = scalar loop, zero NEON; vertical = real NEON;
   W=1 vertical lowers to scalar GPR ops or is excluded as the anchor). Extends `disasm-probe`/`isa_audit.py` and
   the `di_ifchain` precedent (`carrier/src/interp.rs:129-144`). Gating: two forms can compile identically once one
   arm dominates a warm run, and a spurious delta between identical sequences is worse than no number.
1. **Call-crossing amortisation over W**, payload pinned scalar (the FFI isolator, so bench 3's SoA term separates
   by construction; agner IB-1, haoran 1). Cells: the direct/resolved-fnptr/null-entry floors, `ffi_batched_scalar`.
   Yields `C_cross` and the amortisation knee.
2. **Marshalling layout**, payload held scalar (haoran 2): AoS / SoA-native / SoA-transposed (transpose an explicit
   timed stage). Answers what SoA costs at the boundary before it buys anything.
3. **Column-SoA vectorisation win across the boundary** (the headline), vary payload scalar vs
   `interpret_vertical::<W>` over the same W-sweep (agner IB-2, haoran 3). Delta on top of bench 1's shared FFI
   term isolates the vectorisation win, un-aliased.
4. **Entry-point form**, payload pinned scalar for the fair comparison (all four forms wrap `interp.rs:20`; chris
   trap 1), W-sweep. Cells: scalar-anchor, runtime-W, dispatch-table, per-W-set. Disasm-confirmed (bench 0). This
   is op's vehicle bench: dispatch-table vs per-W-set is the C-boundary-mapping decision.
5. **Output-sink shape** (agner IB-4, haoran 5, +decode gap A): null-sink / per-record reserve+commit (2 crossings
   per record) / batched columnar (2 per batch) / +host-side structural decode of the returned W-output arena.
   Built as the real `#[repr(C)]` two-fn-ptr struct, indirect.
6. **Call-scoped vs session-scoped environment** (agner gap B / IB-5): fresh runtime-env descriptor per call vs a
   held handle. Answers whether `execute` must take a persistent runtime handle.
7. **Residency and ownership** (haoran 6, the design round's own "bench-decidable if contentious" open question,
   `202607201315_topic...md` closing para): host-lent buffer registered/pinned once and reused vs a fresh region
   per call (io_uring registered-buffers / JNI critical / CPython buffer-protocol precedent). Orthogonal to W;
   crossed against a W subset in the matrix.
8. **Cross-language floor** (chris IB-4, haoran 7): extend `carrier-zig/interp.zig` with `execute1`/`execute_wK`/
   dispatch-table exports (the byte layout, opcode table, and checksum reduction already exist; only the batch loop
   and `@Vector` lane wrapper are new), cross-validated byte-exact per the existing `zig_crossval` discipline,
   reported with the stub-fidelity caveat until the real M3 runtime entry points land.
9. **Error-path / partial-batch fault** (haoran 9, agner gap C): NOT a timing bench. The return-struct shape for a
   batch where record K faults (per-record status vector / first-error-index / masked selection) is the contract the
   whole cost-model fit is measured against, and it is undecided. This is op-fork A.

## The composition matrix (curated, entgrid-style)

After the individuals teach the terms, cross the real independent axes as explicitly-named coherent cells (the
`entgrid` precedent, not a literal cartesian product, per fact 3), pruned by the structural constraints
(scalar-per-record only at W=1; vertical payload only via dispatch-table/per-W-set; per-W-set only at its compiled
W). Three families:

- `abi_boundary_w`: sweep W, cells = {direct floor, resolved-fnptr floor, null-entry floor, Zig floor,
  scalar-anchor, runtime-W, dispatch-table, per-W-set} x the winning layout/payload from benches 1-4, over the six
  profiles. The headline family.
- `abi_sink_w`: sweep W, cells = {per-record, batched, null} against the winning boundary/entry/payload.
- `abi_residency`: cells = {host-lent-reused, fresh-per-call} crossed against a W subset.

Reported (mirroring the interpreter composition matrix): the `(S, I, r2)` cost line per cell per profile; floor
decomposition against the ladder using the harness `with_floor`/`normalise_mode = "ratio"` machinery
(`analysis.rs:275-333`); an oracle envelope over everything measured; a selector-regret report (does one fixed ABI
choice, one W and one entry form and one sink, lose much regret across the six profiles and the W range, or does
the optimum change per profile, which would be a real design consequence, a runtime-selectable batch width or
entry form); and the `C_cross / I_payload` headline plot. The selector's output IS the ABI decision.

## The fairness gates (every cell, before any run)

One shared unchecked access primitive per axis (already the carrier discipline). Every entry/dispatch label
ISA-confirmed by disassembly (bench 0), any mislabelled cell relabelled not shipped. Per-cell cdylib isolation
(`lto=fat`, `codegen-units=1`, subprocess dlopen) so no cross-variant partial-evaluation; the program crosses as
opaque wire bytes so no cell specialises the interpreter over it. Byte-exact digest cross-validation before any
timing is trusted; floors exempt. The PMU instruction-slope cross-check (agner confound 9,
`bench-harness/src/perf.rs`) is a MANDATORY acceptance gate on every `C_cross` figure: the instruction slope across
W must track the timing slope, or the timing carries a confound the count does not. The known traps and their
checks: optimiser inlining across the boundary (separate cdylib + `blr` disasm); program const-fold in the runtime
(opaque bytes); memory-latency hiding per-record cost (same cache-resident program over W distinct seeds, the
vertical shape, not W distinct pool-chased programs); runtime-W seeing a const W (data-dependent register loop
bound confirmed by disasm); dlsym in the timed region (resolve in `setup` into `St`); fixed-W flattering the
dispatch table (run it under the boundary-cold varying-W regime too); auto-vectorisation granting/suppressing SIMD
nobody asked for (disasm the runtime-W scalar cell for zero NEON); W=1 vertical not a fair scalar anchor (disasm or
exclude); the sink built as an inlinable closure instead of the real struct (build the struct); the AoS-to-SoA
transpose charged to the vectorisation win (explicit timed stage); reps-starvation at high W (reps floor);
Rust/Rust cdylib reported as "the ABI cost" (labelled a same-toolchain lower bound, Zig cell as the real comparator).

## The build sequence (all of it, commit as we go, no run until the panel)

1. Upstream: the `scaffold::boundary` regime + runtime `.dylib` staging + the boundary-cold/instance-lifecycle
   regimes + the reps floor, into `mockspace-bench-matrix`/`-harness`, with tests, suite green. Commit.
2. The shared `carrier-runtime` cdylib exposing all entry points over the unmodified carrier bodies. Commit.
3. The `carrier-runtime` Zig sibling (bench 8 exports). Commit.
4. The individual benches 0-8 as host variants + `matrix_decls`, cross-validated by `cargo test` (byte-exact), each
   ISA-confirmed. Commit per family.
5. The curated composition-matrix families. Commit.
6. The fairness-audit evidence doc (disasm per cell, the trap checklist ticked), mirroring
   `202607221230_fairness-audit-disassembly.md`. Commit.
7. Hand the built machinery to the expert panel (op's gate). Fix findings. THEN run.

The build stays red where a piece is not yet realised; `cargo test` (correctness) is green as cells land. This is
the strict-by-design healthy mid-build state.

## The genuine open decisions for op

Most scope questions resolve to "build all" under op's mandate; these are the ones that genuinely need op because
they are design inputs or contract decisions the benches depend on, not scope.

**A. The error-path / partial-batch return contract — SETTLED (op 2026-07-23): fallibility is a value property,
not an ABI mechanism.** op directed the most sound / future-proof / ideal shape rather than a menu pick. Reasoned
from the round's value-form discipline (`202607201316`: absence is an explicit value form, notko's ladder applied
to the runtime value model) and the algebraic-effects `Handle` direction (`202607210120`: an error is an effect a
handler services), the settled contract:

1. A fallible record's output is an `Outcome<T, E>`-shaped value in the value-arena (a tagged value-node, the
   introduction/constructor projection of the signature, so `E` is an arbitrarily rich value subtree, never a fixed
   status code). Whether a record can fault is compile-time-known from its effect/type signature; an infallible
   record produces a plain value and carries NO fault representation (illegal-states-unrepresentable). An error
   surfaces as the value a handler determined (an unhandled `raise` is in the effect set the target `Permits`).
2. Batch completion is a stream property: the reserve/commit sink carries a completion status (all W committed, or
   truncated-at-K with a reason such as budget-exhausted). The one genuinely out-of-band signal, belonging to the
   value-arena stream, consistent with streaming-with-backpressure. Not any record's value.
3. The lane-mask is an INTERNAL SoA mechanism (mask a faulting lane and continue the column, mask-not-trap), with
   an optional "any-fault" summary as a value-arena header field for a host fast-branch. Denormalized convenience,
   not an ABI channel. The load-bearing truth is the per-record `Outcome` values plus the stream completion status.

This is more sound than any menu option: not a rigid status vector, not SIMD-hostile fail-fast, and it drops the
bolted-on mask+status of the earlier option (a) as vehicle-cruft. Bench consequence: the error-path cost is (a) the
`Outcome`-tag width in fallible output value-nodes (a value-shape variant in the sink/value-arena bench) and (b) the
internal SoA mask-and-continue cost (all-success SoA vs masked SoA in the payload bench); there is NO separate
status channel to bench. This decision feeds the eventual `vehje-runtime-abi` topic; it is recorded here as the
contract the benches lock against.

**B. Wmax, the ABI's maximum batch width — SETTLED (op 2026-07-23): Wmax = 128, sweep W to 256.** The per-W
monomorphised symbol set is `execute_w{2,4,8,16,32,64,128}`; the runtime-W internal buffer sizes to 128; the
W-sweep runs {1,2,4,8,16,32,64,128,256} so the knee past Wmax is evidence-visible (a W=256 request is handled by
the runtime-W path or as 2x128 chunking, and the sweep shows whether crossing Wmax costs anything).

**C. The multi-axis harness sweep (a should-it-go-upstream call).** The composition matrix is genuinely
multi-dimensional and `MatrixDecl` gives one axis. The arc ships fine with the curated `entgrid`-flat approach
(zero upstream change, existing precedent). A native multi-axis sweep declaration is a real reusable harness
capability but orthogonal to the ABI question and a non-trivial design in itself. Recommendation: curated-entgrid
now, flag the multi-axis sweep as a separate future harness arc, rather than fold a harness-ergonomics redesign
into the ABI arc.

**D. Zig cross-language cell in the first pass, or fast-follow?** op's "build all" implies the first pass, and the
vehicle's real target is the Zig runtime, so the Zig callee-side codegen matters. The Zig port is mechanical (the
byte layout and opcode table exist; only the batch loop and `@Vector` wrapper are new). Recommendation: build it in
the first pass as bench 8, with the stub-fidelity caveat, since the alternative leaves the headline entry-form
conclusion resting on a Rust-only artefact.
