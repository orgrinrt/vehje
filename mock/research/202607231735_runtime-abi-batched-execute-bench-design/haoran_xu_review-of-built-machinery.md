# The built abi-bench machinery through the breadth-and-honesty lens (Haoran Xu)

**Date:** 2026-07-23
**Scope:** pre-run review of the built machinery only. No timing numbers exist; none were taken. Every claim
below is either a direct citation of source I read, or a result of a command I ran myself (`cargo test`,
`cargo run --bin gen_matrix`, `otool -tV` disassembly). Where I could not verify a claim from source or a run,
I say so and route it to the open-questions section instead of asserting it.

## Verdict in one line

The correctness discipline (byte-exact cross-validation, ISA-shape disassembly, the trap checklist) is real
and mostly holds up under my own re-derivation, but the pipeline cannot currently produce a single number: the
shared generator rejects every declared family on its very first size, the fairness-audit document under
review is stale against the actual source state on two whole benches, and two of the nine families have no
executed correctness check at all. None of this is a reason to distrust the cross-validated cells; it is a
reason not to hand this specific audit document to the panel as a status report, and not to run anything
until the generation blocker is fixed.

## What is sound (specific, cite file:line)

- **The two-object split is real, not aspirational.** `mock/benches/carrier-runtime/src/lib.rs` exports every
  batched entry over the unmodified carrier bodies (`c::interpret_predecoded`, `c::vertical::interpret_vertical_checksum_into`),
  and the host families dlopen it via `mockspace_bench_matrix::boundary::Runtime::open`/`resolve`
  (`mock/benches/carrier/src/bench/boundary/common.rs:94-148`). I disassembled the release cdylib myself
  (`otool -tV target/release/libcarrier_runtime.dylib`) and confirmed `cr_execute_scalar_runtime_w`
  (`/tmp` extract, lines 1068-1103 of the disasm) calls `interpret_predecoded` via a plain intra-object `bl`,
  not an inlined body; the crossing is genuinely a separate compiled object the host resolves a pointer into.
- **The scalar-vs-SoA ISA contrast holds.** `cr_execute_scalar_runtime_w`'s body (39 instructions) contains
  zero vector-register traffic; `cr_execute_soa_runtime_w`'s body loads a `Simd<u64,8>` chunk via
  `ldp q0,q1` / `ldp q2,q3` (4 NEON 128-bit register-pair loads, exactly the 8 lanes `LANES=8` implies) before
  calling `interpret_vertical_checksum_into::<8>`, whose own body (lines 29-313 of the disasm) is dense with
  `cmeq.2d` / `cmhi.2d` / `and.16b` / `sub.2d` / `neg.2d` vector ops. The claim "scalar payload does not
  accidentally gain SIMD nobody asked for" is directly verified, not asserted.
- **The dispatch-table disassembly numbers check out almost exactly.** I disassembled
  `cr_execute_scalar_dispatch` independently: 383 lines total, 77 lines matching `cmp`/`cbz`/`cbnz`/`b.<cc>`.
  The audit doc's claim ("77 branch-shaped lines... over a 382-line body") is accurate to within a
  labelling-convention off-by-one. `cr_execute_scalar_w64` disassembles to 33 lines, zero NEON, no width
  comparison against 64, matching the "compact 33-line const-bound body, no match-W branch" claim exactly.
  This tells me the ISA-shape gate's authors did real, careful work on at least these two entries.
- **The cross-validation that exists is real and exercises the actual timed entry points, not a parallel
  replica compiled differently.** `mock/benches/carrier/src/bench/boundary/cross.rs:165-193` resolves
  `cr_execute_scalar_runtime_w` (the identical symbol `common::open_and_init` resolves for the timed cell) and
  asserts byte-equality against `inproc_column`/`inproc_scalar_batch`. `soa.rs`, `sink.rs`, `marshal.rs`,
  `zig.rs`, `entry.rs` all do the equivalent for their respective entries. I ran the full suite myself
  (`cargo test --features boundary,vertical`, dylibs staged via `VEHJE_CARRIER_RUNTIME`/`VEHJE_CARRIER_ZIG`):
  53 unit tests plus the zig-crossval integration test, all green.
- **The floor ladder's rung-b construction is correct as written.** `cross.rs:117-129`'s `inproc_fnptr` cell
  wraps `s.f` (a genuine `fn` pointer field) in `core::hint::black_box` before calling it, which is the right
  mechanism to force an indirect call the optimiser cannot devirtualise; I cannot confirm the generated
  variant's disassembly shows a real `blr` (that artifact does not exist until generation succeeds, see
  Finding 1), but the source-level construction is the correct one.
- **The composition matrix does not reimplement anything.** `matrix.rs` imports and calls
  `common::{open_and_init, cross_column}`, `entry::{open_anchor, anchor_column, open_mono, mono_column}`, and
  `zig::open_zig_cross` verbatim; every cell is the same measured operation the individual bench already
  validated, recombined. This is exactly the "not a reimplementation" claim the design's own comment makes,
  and it is true on inspection.

## Findings (numbered)

**1. The generator rejects every declared family; nothing can currently be run.**
The problem: `mockspace-bench-matrix`'s shared generator hardcodes the only `N` values a generated variant is
monomorphized over: `const TEMPLATE_SIZES: &[usize] = &[64, 256, 1024, 4096, 16384]`
(`mockspace/bench-matrix/src/generate.rs:46`), inherited from the byte-buffer-size axis every non-boundary
carrier bench sweeps. Every boundary family's `sizes` field is the batch width `W`, swept `[1, 2, 4, 8, 16,
32, 64, 128, 256]` (or `[1,2,4,8,16]` for marshal's field count), and `generate_all` rejects the whole decl if
any single value is not in the allow-list (`generate.rs:96-106`). I ran the actual generator:
```
cd mock/benches && cargo run --bin gen_matrix --features "vehje-bench-carrier/boundary,vehje-bench-carrier/vertical"
FAILED: matrix 'abi_cross_scalar' requests size 1, which the variant template does not monomorphize
(allowed: [64, 256, 1024, 4096, 16384]); add it to LIB_TEMPLATE's bench_variant sizes or drop it from the decl
```
This fails on the very first family in the list. Every one of the nine families uses W or field-count values
outside the allow-list except the coincidental overlap at 64 and 256, and the rejection is whole-decl, not
per-value, so a single disallowed size blocks the entire family. **No variant crate exists for any boundary
family today; the pipeline cannot produce a number.** The synthesis document itself names the root cause as
one of its "four verified structural facts" ("batch width W and program size collide if both ride n/sizes;
they must be separated," `synthesis_and_build-plan.md:29-30`) but the delivered fix (reinterpreting the same
`n`/`sizes` slot's *meaning* from byte-size to batch-width) never touched the upstream allow-list that gates
which values that slot may hold. This is the single blocking item before any run.

**2. The fairness-audit document is stale against the actual source on two whole benches.**
`fairness-audit-and-built-machinery.md` states under "What is not yet built": "Bench 2 (marshalling
AoS/SoA-native/SoA-transposed). The only unbuilt timing bench. It needs a multi-field record model added to
the carrier... This is a real carrier extension" and lists "The composition matrix... built after the
individuals run." Both are built, wired into `matrix_decls()`, cross-validated, and green:
`mock/benches/carrier/src/bench/boundary/marshal.rs` (four cells, a byte-exact cross-validation test
`layouts_agree_and_match_in_process_byte_exact`) and `matrix.rs` (nine cells, `abi_boundary_w`). I confirmed
both pass under `cargo test`. A document whose stated purpose is "the pre-run record of what the arc has
BUILT... the step-6 artefact the expert panel reviews before any timing run" that misreports two whole
families' build state is not a reliable gate artefact as it stands; a panel reading it would wrongly believe
marshalling and the composition matrix still need building, and would not know to check either for the gaps
Findings 3-4 name (because the document never considers them, having assumed they do not exist).

**3. The audit's own trap checklist contradicts its own "not yet built" section on the identical trap.**
The trap checklist row for "reps-starvation at high W" reads: "N held fixed, so payload work per pass is
constant across W | **by construction**." The "not yet built" section, four lines later in the same document,
lists: "A minimum-reps floor across the W sweep (haoran trap 1)... enforce comparable reps, report
`batch_count`" as a **pending run-time item**. These are the same trap (both cite "haoran trap 1" / reps
starvation), and the document asserts both "handled by construction" and "not yet built" for it in two
different sections. Reading the actual calibration mechanism resolves which is right: `calibrate_reps`
(`mockspace/bench-core/src/lib.rs:476-482`) sizes repetitions off a single wall-clock-floor probe
(`CALIBRATION_FLOOR_TICKS = 2048`, `lib.rs:464`), and nothing in `scaffold::warm` or the boundary cells
enforces a *minimum rep count* independent of that floor. N held fixed keeps the *payload work per pass*
comparable (true, and correctly reasoned in the `fill_seeds`/`cross_column` doc comments), but it does nothing
to keep the *number of calibrated repetitions* comparable, because a W=1 pass (k=256 cheap crossings) and a
W=256 pass (k=1 expensive crossing) take wildly different wall-clock time per rep, so the floor-based
calibrator will assign very different rep counts to each. "By construction" is the wrong verdict for that row;
the "not yet built" section is the accurate one.

**4. Benches 6 (lifecycle) and 7 (residency) have no executed cross-validation; their test docstrings assert
a property their test bodies do not check.**
`lifecycle.rs`'s only test is `matrix_decls_declare_the_family_shape`, which checks tags and names, nothing
about output. Its doc comment reads: "A fresh-per-column handle computes the identical crossing fold as the
held handle (the instance is stateless across calls beyond its scratch). Guards the shape; crossing
correctness is the shared `cross.rs` cross-validation." The test body does not assert any fold equality
anywhere; the claim in the comment is not checked by the test it is attached to. `residency.rs` has the
identical pattern: "The residency cells differ only in buffer provenance, not computation... Proven against
the shared crossing cross-validation in `cross.rs`; here we guard the declared shape," again with only a
shape-tag test. The underlying reasoning (both cells cross into `cr_execute_scalar_runtime_w`, which bench 1
already validated, so correctness should transfer) is plausible, but it is an argument, not an executed check,
and it is the only pair of the seven built timing families with no executed check at all (compare: cross,
soa, marshal, sink, entry, zig all have a dedicated `*_byte_exact` test). The audit document's own
cross-validation status list (five benches enumerated: 1, 3, 4, 5, 8) correctly omits 6 and 7 from the "proven
byte-exact" claim, so the document is honest about this specific gap even while being wrong about bench 2; but
nowhere does it flag the gap as something to close, and the "every real crossing cell is proven... floors are
exempt" line in the synthesis document's fairness-gates section reads as a blanket claim that these two
families quietly do not meet.

**5. The `fresh_buffer` residency cell's "cold" framing is unlikely to measure what it claims.**
`residency.rs:47-53`: `cell fresh_buffer` allocates `vec![0u64; N_TOTAL]` (a 2048-byte, N_TOTAL=256-word
buffer) fresh inside the timed cell body, on every one of the scaffold's `ITERS=16` inner calls, for every
calibrated rep (potentially thousands). The doc comment claims: "the allocation and first-touch cost of a cold
input region per call are charged to the timed pass." A same-size-class allocation freed and re-requested in a
tight loop is close to the textbook case a free-list allocator serves from a warm cache without touching new
pages: the same physical block is very likely handed back on nearly every iteration, so what gets measured is
allocator bookkeeping (a `malloc`/`free` round trip for a small, hot-in-cache block), not the first-touch
page-fault or TLB-miss cost a genuinely cold registered-buffer question is asking about. This does not make
the number meaningless, but it makes the label ("cold, re-pinned") overclaim what the mechanism can actually
produce, and a reader taking the eventual delta between `reused_buffer` and `fresh_buffer` at face value as
"the cost of a cold region" would be trusting a number that most likely reflects a much cheaper effect.

**6. The ISA-shape gate's NEON evidence for `cr_execute_soa_runtime_w` is misattributed to the wrong symbol.**
The audit table's evidence column for `cr_execute_soa_runtime_w` reads "437 NEON-shaped lines (`v*.2d`/`ld1`/
`fmla`/`.16b`)." I disassembled that exact symbol myself: its body is 59 instructions total (lines 2014-2072
of my `otool -tV` extract), of which only the 4 register loads for the incoming chunk (`ldp q0,q1`/`ldp
q2,q3`) are vector instructions; the rest is scalar bookkeeping plus a `bl` into
`interpret_vertical_checksum_into::<8>`. The bulk of the NEON traffic (`cmeq.2d`, `cmhi.2d`, `and.16b`,
`sub.2d`, `neg.2d`, dozens of them) lives in that separate, non-inlined callee, whose own body runs 284 lines
(lines 29-313 of the extract). Whatever produced the "437" figure almost certainly measured the callee, or the
callee plus related vertical functions, not the named symbol `cr_execute_soa_runtime_w`. This does not
undermine the underlying claim (the SoA payload genuinely vectorises, confirmed above under "What is sound"),
but a reader who takes the audit's row literally and disassembles exactly the named symbol will not find
anything close to 437 NEON lines there, and would have reasonable cause to doubt the whole gate's rigor even
though the substantive conclusion is correct. Attribute evidence to the symbol that actually carries it, or
name the callee explicitly.

**7. The Zig cross-language comparator (bench 8) crosses the wrong dispatch shape for its own stated purpose.**
`carrier-zig/interp.zig`'s own module doc names two dispatch shapes: a switch (`interpSwitch`) and a
guaranteed-tail-call token-threaded loop (`interpTail`, `@call(.always_tail)`), and states plainly: "The tail
shape is the one Rust needed a nightly feature (`become`) to express; Zig has it stable, and the shipped vehje
runtime is Zig, so **this is the load-bearing cross-language cell**." But every batched entry the ABI-bench
family (`zig.rs`, and the composition matrix's `zig_runtime_w` cell) actually crosses into routes through
`scalarBatch`, which calls `interpSwitch` (line 281 of `interp.zig`), never `interpTail`. There is no `zr_*`
export anywhere in the file that reaches the tail-call interpreter. So bench 8, whose entire reason to exist
per the synthesis document is "the real target's callee-side codegen matters... the shipped vehje runtime is
Zig," measures the callee-side codegen of the dispatch shape the file's own comment calls secondary (switch),
not the one it calls load-bearing (tail-call). The byte-exact cross-validation against the Rust runtime passes
(confirmed, `zig_crossing_matches_rust_byte_exact`), so this is a case where a fully green, cross-validated
cell still answers a narrower and differently-shaped question than the one it is built to answer: it tells
you the ABI crossing cost into a Zig switch-dispatch interpreter, not into the tail-call interpreter that is
this project's own stated production shape.

**8. Single-probe calibration has no outlier protection, and it interacts with Finding 3.**
`timed_calibrated!` (`bench-core/src/lib.rs:556-595`) probes the run block **exactly once** to size the
repetition count, then discards that probe and times a fresh loop of the computed rep count. A single
anomalous probe (a context switch, a cache-cold first pass before the scaffold's own digest/first-touch passes
have run, a stray interrupt) sizes the entire calibrated measurement with no retry, no median-of-N, no
sanity re-probe. This compounds with Finding 3: at the high-W end of the sweep, where reps are already fewest
(large per-crossing payload, few crossings per pass, larger wall-clock time per rep), a bad single probe has
the most leverage over the final rep count and the least averaging to wash it out.

**9. `normalise_mode` is hardcoded to `"subtract"` in the shared generator; the design's stated composition-matrix
headline analysis calls for `"ratio"`.**
`generate.rs:80`: `to_spec` unconditionally sets `normalise_mode: Some("subtract".to_string())` for every
matrix, including the composition families. The synthesis document's composition-matrix section states the
reporting plan explicitly needs "floor decomposition against the ladder using the harness
`with_floor`/`normalise_mode = "ratio"` machinery," and the whole "headline is a ratio, not a yes/no" framing
(the `C_cross / I_payload` plot) depends on ratio-shaped analysis. I did not trace far enough into
`bench-harness/src/analysis.rs` to determine whether `MatrixSpec.normalise_mode` is the single knob the
ratio-based headline plot depends on, or whether that plot is computed downstream from the raw committed CSVs
independent of this per-run setting. I flag this as unresolved rather than asserting a defect (see open
questions).

**10. Minor: two unused-import warnings in freshly-authored fairness-critical files.**
`cargo test --features boundary,vertical` reports unused-import warnings in `matrix.rs` (the unused
`mockspace_bench_matrix::boundary::Runtime` import) and `zig.rs` (the unused `program_bytes` import). Neither
affects correctness. Worth a one-line cleanup before this code is presented as the finished, panel-reviewed
state; a warning-clean build is cheap and is itself a small signal of care in code that gates a go/no-go
decision.

## The honesty ledger

**Is the cross-validation actually exercising the timed path?** For benches 1, 2, 3, 4, 5, 8: yes, verified.
Each cross-validation test resolves and calls the identical exported symbol the timed cell resolves and calls
(same `cr_*`/`zr_*` name, same handle-construction path), and I ran every one of these tests myself against
freshly built dylibs. For benches 6 and 7: no executed cross-validation exists; the correctness argument is
"this reuses an entry point validated elsewhere," which is reasonable but unexecuted (Finding 4).

**Does the fixed-N shape actually keep reps comparable?** No, and the document's own trap checklist and "not
yet built" section disagree with each other about exactly this (Finding 3). Fixed N keeps *payload work per
pass* comparable across the W-sweep, which is what the `fill_seeds`/`cross_column` documentation actually
claims and is correctly reasoned. It says nothing about *repetition count* comparability under floor-based
calibration, and nothing in the built machinery enforces that separately.

**What is claimed handled-by-construction that is not?** The trap-checklist row "reps-starvation at high W...
by construction" (Finding 3). Separately, "auto-vectorisation of the scalar cell... confirmed" and "W=1 SoA
not a fair scalar anchor... by construction" both hold up under my own disassembly and source reading.

**What is measured that should not be trusted, once numbers exist?** The `fresh_buffer` residency delta
(Finding 5, likely allocator overhead mislabelled as first-touch/cold-page cost) and the bench-8 Zig entry-form
conclusion (Finding 7, measures the switch interpreter, not the tail-call interpreter the file itself calls
load-bearing). Both would produce clean, cross-validated, green numbers that answer a narrower or different
question than their labels claim.

**What is missing that a reader would assume is covered?** Two things the audit document itself is silent
about as gaps: executed cross-validation for benches 6 and 7 (present for every other family, silently absent
here, never named as owed), and any coverage of the tail-call Zig dispatch shape (never mentioned anywhere as
a gap, despite the Zig source file's own comment naming it as the load-bearing shape). And, as of right now,
every number the document's readers would assume exists once the panel clears it: none can be produced,
because generation itself fails on the first family (Finding 1).

## Open questions for the synthesiser

**A. The `TEMPLATE_SIZES`/`LIB_TEMPLATE` size collision.** Extend the shared upstream allow-list (and the
`#[bench_variant(sizes = [...])]` list in `LIB_TEMPLATE`) to include the W-sweep and field-count values
alongside the existing byte-buffer sizes, keeping one shared template and one code path for every bench
family in the tree; or build a boundary-specific generation path that does not route through the
byte-buffer-sized `TEMPLATE_SIZES` assumption at all, keeping the semantic axis (batch width) fully separate
from the axis the shared harness was built for (input byte-buffer size) at the cost of a second generation
path to maintain. Either resolves Finding 1; the tradeoff is shared-template uniformity against fidelity to
the fact that W is not the same kind of quantity the shared harness's size axis was designed for.

**B. Bench 2's scope.** Keep the current host/runtime-side XOR-fold-to-seed synthetic proxy (already built,
cross-validated, answers "what does reading N words AoS vs SoA cost at the boundary" before any downstream
vectorisation), or build the real multi-field carrier record model the original design called for (a genuine
extension to the interpreter's record shape, answering the broader "what does the carrier's own multi-field
marshalling cost" question, at the cost of a real semantic change to the carrier and a new design/review
surface). The current build answers a real and legitimate question; whether it is the question bench 2 was
meant to answer is the open call.

**C. The Zig comparator's dispatch shape.** Leave bench 8 crossing into the switch-dispatch Zig interpreter
(already built, byte-exact validated, minimal additional surface), or add a second Zig cell crossing into the
tail-call interpreter (`interpTail`), which the Zig source's own module doc names as the shape closest to the
real target runtime, at the cost of extending the `zr_*` export surface and its cross-validation to a second
dispatch family. The tradeoff is measuring the dispatch shape the vehicle actually intends to ship against the
added build and validation surface of a second Zig entry family.

**D. Lifecycle and residency cross-validation.** Add executed byte-exact equality tests for
fresh-handle-vs-held-handle (bench 6) and fresh-buffer-vs-reused-buffer (bench 7), bringing them in line with
every other built family, at the cost of two more test functions; or accept the by-reuse-of-validated-entry-
points argument as sufficient given how small and simple the surrounding harness code is, at the cost of these
two families being the only ones in the tree whose correctness rests on an argument rather than an executed
check.

**E. `fresh_buffer`'s realism.** Keep the current small, fixed-size `Vec` allocate/free churn (already built,
simple, cheap), accepting that the measured delta likely reflects allocator bookkeeping more than first-touch
or TLB-miss cost; or redesign the cell to force genuinely cold pages (a much larger buffer than the allocator's
free-list would service from cache, or explicit page-eviction/mmap-based cold forcing, or cycling many
distinct buffers so no single one stays resident), at the cost of added complexity and platform-specific
mechanism to earn the "cold, re-pinned" label the cell already carries.

**F. `normalise_mode`.** Confirm whether the hardcoded `"subtract"` in the shared `generate_all`/`to_spec` path
is orthogonal to a separate downstream ratio-based analysis (no change needed, the ratio plot is computed from
the raw CSVs independent of this setting), or whether the composition matrix's ratio-based headline plot
genuinely depends on this exact knob being `"ratio"` for the composition families (needs either a per-decl
override or a second normalisation mode surfaced through `MatrixDecl`). I did not trace far enough into the
harness's analysis code to resolve this myself.

**G. The fairness-audit document's maintenance.** Refresh `fairness-audit-and-built-machinery.md` now against
the actual source state (cheap, immediate, closes Finding 2 and the Finding 3 internal contradiction), or treat
it as superseded by a fresh pre-run document written after Findings 1-9 are addressed, accepting the risk that
in the interim a panel or reader consulting the existing document is working from material that is wrong about
two whole benches' build state and self-contradictory about a named trap.
