# The maximal composition matrix: the shared IR as a staged pipeline, every stage a measured axis

**Date:** 2026-07-22
**Status:** proposal. Envisions the largest composition matrix that is still meaningful and insightful:
the shared IR program treated as a multi-stage pipeline where every stage that has ever been benched in
isolation becomes an orthogonal axis, and one composition is one strategy chosen per stage. Free to
overhaul the premise and to extend the IR so that more isolated findings become composable.
**Scope:** the shared-IR interpreter pipeline and the build pipeline that feeds it. Maps every isolated
carrier bench onto an axis, or explains why it stays a separate workload.
**Companions:** `202607220300_bench-remeasure-synthesis.md` (honest baselines), `202607221200_bench-beating-attempts.md`
(A1-A3), `202607221600_bench-composition-rearchitecture.md` (the cost-model reframe, audited in the addendum),
`202607211539_bench-evidence-audit-panel/` (the ten bench-honesty rules).

## The framing: a composition is a path through a staged pipeline

The prior composition plan crossed two axes, form and dispatch. That is a small corner of what the corpus
has actually measured. The shared IR program is not interpreted by a single choice; it flows through a
pipeline of stages, and nearly every isolated bench in the corpus was measuring one stage of that pipeline
in isolation, with all the other stages pinned. The largest meaningful matrix is the one that unpins every
stage at once and lets each be chosen independently, so a composition is a full path: one strategy per
stage, end to end, over the identical program.

Stated as a pipeline, the shared IR program passes through a build half that produces an in-memory form and
a run half that evaluates it:

1. optimize (an optional pre-pass that rewrites the program: none, common-subexpression elimination, a
   bounded equality-saturation rewrite),
2. intern (how the const pool and repeated operands are deduplicated: none, hashed, sharded),
3. record form (how a node sits in memory: the five wire strides, or the predecoded flat form),
4. value representation (how a runtime value is carried: raw static, runtime-tagged, NaN-boxed),
5. operand access (how a node reaches its inputs: inline slot, pool spill, interned reference),
6. dispatch (opcode to operation: switch, function-pointer table, preserve-none threaded, if-chain,
   bit-test tree, perfect hash),
7. fusion (superinstructions: none, pairwise-fused hot pairs),
8. output building (how results are materialised and emitted: overwrite in place, copy-on-write, reuse
   arena).

Stages 1 to 3 are the build half (they determine setup cost `S`); stages 4 to 8 are the run half (they
determine per-evaluation cost `I`). Each stage is exactly a thing the corpus already benched alone. A
composition is a choice at every stage; the matrix is the set of all such choices; and because every
stage runs against the one shared `generate()` program and folds one cross-validated checksum, every cell
is genuinely the same program observed under a different full path.

## Inventory: what was benched in isolation, and where each lands

The corpus splits cleanly into pipeline stages of the shared-IR interpreter, a separate build-pipeline
family that also composes, and genuinely separate workloads that do not.

Interpreter-pipeline stages (each becomes a run-half axis):

- Dispatch shape. `carrier_dispatch` (switch, fntable, threaded) and the match-lowering suite
  (`arch_*`/`archn_*`/`ir_match`: if-chain, jump table, tree, whole) measured opcode dispatch in
  isolation. Becomes the dispatch axis, and folds the branch-strategy question inside the real
  interpreter (the isolated finding was if-chain beats jump table on this machine; here it is retested
  under real operand loads and real memory pressure).
- Record form. `carrier_record_width` (REC12/16/20/24/32) and `carrier_predecode` (wire versus flat)
  measured how a node sits in memory. Becomes the record-form axis; it interacts with the working-set
  crossover because a narrower record stays L1-resident to a larger program.
- Value representation. `carrier_valrepr` (static, tagged, NaN-boxed) measured value carrying on a
  separate mixed-type mini-IR. It does not currently compose because the shared IR is u64-only; the
  premise overhaul below extends the IR with a per-node type tag so value representation becomes a real
  axis over the shared program rather than a bench on a private program.
- Operand access. `project_field_access` (monomorphic versus polymorphic field projection) measured how
  a value's field is reached. In the IR this is how a node reaches an operand: a direct inline slot, a
  pool-spilled indirection, or an interned reference. Becomes the operand-access axis, and it is coupled
  to record form (narrow records spill sooner) but not identical to it.
- Fusion. `iter_fusion` (depth-2 and depth-3 fusion) measured superinstruction fusing of consecutive
  operations. Becomes the fusion axis: none versus pairwise-fused hot pairs, checksum-preserving by
  storing every intermediate and folding it.
- Output building. `interp_output_building` and `iter_output_reuse` (copy-on-write versus full copy,
  reuse) measured how results are materialised. Becomes the output-building axis, a post-pass stage.

Build-pipeline stages (each becomes a build-half axis; these produce the program the run half evaluates):

- Optimize pre-pass. `cheap_lowering` (CSE, 59 to 75 percent node-count reduction) and
  `egraph-saturation` (bounded equality saturation) measured program-shrinking rewrites. Become the
  optimize axis: none, CSE, bounded eqsat, or CSE-then-eqsat. This is the highest-leverage axis nobody
  composed: a pre-pass that shrinks the node count changes the working set every downstream stage pays,
  so its interaction with record form and dispatch is exactly the kind of cross-stage effect a matrix
  exists to reveal.
- Intern. `interner_intern` (single versus sharded) measured const/token interning. Becomes the intern
  axis for the const pool and repeated-operand deduplication: it changes the const-pool size and thus the
  const-load locality the run half pays.

Separate workloads (not the shared IR program; they stay their own benches, and the addendum notes where
a second matrix could form):

- `carrier_reach` (semi-naive fixpoint), `retract` (counted differential), `incr_reload` (content-hash
  cache), `sharded_intern` as a standalone, `thermo` (thermometer lattice), `arena-locality`,
  `value-arena`, `cfg` (register-CFG interp, a different IR). These evaluate different algorithms on
  different data. They cannot be cells in the shared-IR matrix. Two of them (eqsat, interner) reappear
  above as build-half stages because in that role they operate on the shared IR; as standalone
  algorithm benches they remain separate.

The `cfg` register-CFG interpreter is a special case: it is a second IR (a control-flow-block machine,
not a data-flow node DAG). It is worth a parallel, smaller matrix of its own (dispatch by the same axis
set over the CFG form), but it does not share the node-DAG program, so it is not folded into the main
matrix. Noted for a sibling matrix, out of scope here.

## The maximal matrix, enumerated

A composition is one strategy per stage. The stage cardinalities:

- optimize: 4 (none, CSE, eqsat, CSE+eqsat)
- intern: 3 (none, hashed, sharded)
- record form: 6 (wire12, wire16, wire20, wire24, wire32, flat16)
- value representation: 3 (static, tagged, nanbox)
- operand access: 3 (inline, pool-spill, interned-ref)
- dispatch: 6 (switch, fntable, threaded, ifchain, bittree, perfecthash)
- fusion: 2 (none, fused)
- output building: 3 (overwrite, cow, reuse)

The full logical cross is 4 * 3 * 6 * 3 * 3 * 6 * 2 * 3 = 69,984 compositions per (shape, size, k,
regime) point. That is the largest matrix; it is also unbuildable and, more importantly, unreadable in
full. Some cells are also incoherent (flat form pins operand access to inline; interned-ref requires the
intern stage on). After removing incoherent cells the coherent cross is roughly 20,000, still far past
what is worth building blindly.

The resolution is not to shrink the axis set (the directive is maximal, and each axis earns its place by
having been benched alone). It is to make the full matrix generatable on demand and to run it in
prioritised tiers, so the complete space is expressible and any cell is one command away, while the
committed evidence grows outward from the shipped default along the axes that move the result most.

- Tier 0, the spine. The full record-form by dispatch grid, at the canonical shape, canonical everything
  else, hot regime: 6 * 6 = 36 cells. This is the primary interaction surface and it subsumes the
  original two-axis plan. Baseline cell: wire24, switch, all-else-canonical (the shipped default).
- Tier 1, single-axis slices from the spine. For each remaining stage, sweep that stage alone through the
  Tier-0 spine while pinning the others canonical: optimize (4), intern (3), value-rep (3), operand
  access (3), fusion (2), output (3). This measures each stage's main effect against the common baseline
  without the cross blow-up. Roughly 36 * (sum of the other cardinalities) but pinned, so on the order of
  a few hundred cells.
- Tier 2, targeted pairwise cross for the stages whose interaction is predicted to be strong: optimize by
  record form (a CSE pre-pass changes the working set the form pays), fusion by dispatch (fusion removes
  dispatches, so its value depends on dispatch cost), value-rep by dispatch (tag checks are branches that
  interact with the dispatch predictor). Each a small full sub-grid.
- Tier 3, the full coherent cross, generated and run only where Tier 2 reveals a non-additive interaction
  worth the full grid, or on demand for a specific question.

Every tier is emitted by one generator from one axis catalogue, so the "largest matrix" exists as a
complete definition and a runnable set; the tiers govern what is committed as evidence, not what is
expressible. This keeps the matrix maximal in definition and meaningful in what it actually spends machine
time measuring.

Each axis pins the others at a declared canonical value, and the canonical point is the shipped default so
every slice reads as a delta from what actually ships: optimize=none, intern=none, record=wire24,
value=static, operand=inline, dispatch=switch, fusion=none, output=overwrite.

## Timing: individual per-stage sub-timings AND the end-to-end run

The directive's timing question is the one with the most upside, and the answer is both, enabled by an
upstream harness feature.

A single wall-clock per composition answers "how fast is this whole path" and nothing about where the time
goes. But every composition is a pipeline of stages, and the most insightful data the matrix can produce is
per-stage attribution within a single run: how long decode took, how long the dispatch loop took, how long
output building took, for this exact path, on the same warm state. That decomposition is what turns a
two-percent end-to-end difference into a legible "the dispatch stage is thirty percent cheaper but it is
one fifth of the run." No amount of cross-cell differencing recovers per-stage cost as cleanly as measuring
each stage directly in place.

So the proposal is to measure, per composition run:

- the end-to-end time (the composition's real cost, the number a consumer feels), and
- a named sub-timing per pipeline stage that actually executes in that path (build stages once, run
  stages per evaluation), all within one process on one warm state.

This requires an upstream feature on the mockspace bench harness, and it is worth building because it is
sound and it unlocks the whole per-stage-attribution dimension: extend the timing macro from one region to
several named sub-regions per bench call, each emitting its own CSV column keyed by stage name, with the
end-to-end region being the sum-enclosing outer region. Concretely, alongside the existing `timed! { run { .. } }`,
a `timed_stage!("decode") { .. }` / `timed_stage!("dispatch") { .. }` form that records each sub-region's
CNTVCT delta into a per-stage slot of the same `FfiBenchCall`, and a harness-side schema that writes one row
per (variant, size, stage) instead of one row per (variant, size). The cross-validation and the outer
timing are unchanged; the sub-timings are additive detail.

Soundness conditions for per-stage sub-timing, all satisfiable:

- Each sub-region must clear the CNTVCT quantum (about 42 ns at 24 MHz). Small stages over small programs
  will not, so each stage sub-region carries its own internal repeat count sized to exceed roughly 10 us,
  exactly as the end-to-end region does. A stage too cheap to measure even repeated is reported as
  below-resolution rather than as zero.
- Sub-timing must not perturb the path it measures. Reading CNTVCT twice around a stage adds two barriers;
  those are constant across compositions and cancel in the deltas, but the raw per-stage number carries
  that fixed overhead and the schema records the barrier cost once so it can be subtracted.
- The stages must be real boundaries in the code, not instrumentation seams that inhibit optimisation. A
  stage boundary that forces a spill changes the thing measured. The build stages (decode, optimize,
  intern) are naturally separable functions; the run-half stages (dispatch, operand access, output) are
  fused in the hot loop by design, so their sub-timing is measured by the reference-floor differencing
  from the companion proposal (null-dispatch isolates dispatch; an inline-only operand path isolates
  operand access) rather than by an in-loop barrier that would destroy the loop. This is the honest split:
  build-half stages get direct sub-timing; run-half stages get differential attribution against reference
  floors. Both land in the same per-stage schema.

The end-to-end number and the S-plus-k-times-I decomposition from the companion proposal both fall out:
the build-half sub-timings sum to `S` measured directly, the run-half attribution gives `I` decomposed by
stage, and the k-sweep still fits the line. Per-stage sub-timing and the cost-model reframe are
complementary, not alternatives: the reframe says measure `S` and `I` as a line; per-stage sub-timing says
measure what `S` and `I` are made of.

## Correctness contract

Every cell runs the identical `generate()` program for a given (shape, seed), differing only in the path
through the stages, and folds one post-pass checksum over the final `results` array (moved out of the hot
loop per the companion proposal, which both cleans the dispatch signal and strengthens cross-validation).
The harness asserts all coherent cells agree byte-exact at each (shape, seed) before any timing is trusted;
a divergence fails the bench. For the optimize axis, CSE and eqsat change the node count but must preserve
the program's observable result, so their cells cross-validate against the un-optimised cell on the final
checksum, which is the exact property a correct rewrite must have and a strong test that the pre-pass is
sound. For the value-representation axis on the type-extended IR, all three representations must agree on
the checksum, catching a tag or box bug as a divergence.

## Champions

Three, in increasing sophistication, and the relationships are the results.

- Oracle envelope: at each (n, k, shape, regime) point, the fastest coherent cell actually measured. The
  theoretical best the space contains; the target every real selector is judged against.
- Whole-program cost-model selector (adopted from the companion proposal): a runtime selector that, from a
  calibration table of per-stage `(S, I)`, predicts the cheapest full path for the actual (node_count,
  expected_k, measured_shape_features) and pays a branch and a table lookup to pick it. Measured as a real
  cell, judged against the oracle envelope. It selects one path for the whole program.
- Per-stage adaptive selector (the maximal-matrix contribution): a selector that chooses each stage
  independently from the calibration table rather than picking a single pre-canned path, because the
  matrix's per-stage attribution makes the stages separable. If the per-stage data shows record form is
  chosen by size and dispatch is chosen by op-entropy and output building is chosen by reuse rate, a
  selector can compose the per-stage-best into a path the fixed matrix never enumerated. Whether the
  per-stage-composed selector beats the whole-program selector is a direct test of whether the stages are
  actually independent or whether their interactions dominate; either answer is decision-grade for how a
  real adaptive interpreter should be built.

## Rollout

Carrier first. Add the missing dispatch strategies (bittree, perfecthash) for wire and flat; add the
null-dispatch and inline-only reference paths for run-half attribution; extend the IR node with a type tag
so value representation composes; add the optimize pre-passes (CSE exists as `cheap_lowering`, lift it onto
the shared program; add a bounded eqsat pass reusing the eqsat module); add the fusion pass and the
output-building strategies; move the checksum out of the hot loop across every interpreter. Extend
`GenParams` with the topology/depth knob to complete the shape basis (op_vocab, op_correlation,
locality_window, const_count already exist).

Harness next. Build the multi-sub-timing feature (`timed_stage!` plus the per-(variant, size, stage) CSV
schema) upstream in mockspace-bench-core; this is the one upstream change and it is worth it because it
unlocks the entire per-stage-attribution dimension for this matrix and every future staged bench. Then the
generator that emits the axis catalogue and the tiered cell set, the k-sweep as a bench family, the
cold-many regime, and the shape basis.

Run discipline unchanged and strict: serial process runs, at least three per cell, minimum for throughput
and median-with-spread for regression points, no ranking inside the noise floor, `S`/`I` fits with reported
R^2, the ns-to-cycles-to-IPC sanity line per slope, committed CSVs and the tracked `.bench_history` trail
with every commit.

## Honest caveats

The maximal matrix is enormous; the tiering is what makes it tractable, and the risk is that a strong
interaction hides in a Tier-3 cell never run. The mitigation is that Tier 2 explicitly targets the
predicted-strong interactions, and the full cross stays generatable so any suspected interaction is one
command from measurement. Extending the IR with a type tag changes the record layout and therefore shifts
every absolute number, the same intended cost the checksum move already incurs; the type tag is off for the
static-representation canonical path so the spine stays comparable to the A1-A3 baselines up to the
checksum move. The optimize axis introduces cells with different node counts for the same logical program,
so the per-node normalisations must switch to per-program or per-logical-result to stay comparable across
the optimize axis. The per-stage sub-timing of run-half stages is differential (against reference floors),
not direct, so its attribution inherits the reference floors' assumptions; this is stated at each such
number rather than presented as a direct measurement. And the perfect-hash dispatch is only a win if the
opcode set is static, which it is here, so that cell does not generalise to an extensible opcode set and is
labelled accordingly.

## Addendum: audit of the sibling deliverables

Two forks were assigned adjacent write-ups. At the time of writing only one had landed on disk,
`202607221600_bench-composition-rearchitecture.md` (commit 3820fa5); the second (the "critique and answer
the open questions" fork) had not yet written its file, so it could not be audited here. The audit below
covers the one available.

### `202607221600` (cost-model rearchitecture)

It is a strong deliverable and its core reframe is correct: model every composition as `total(k) = S + k * I`,
sweep `k`, fit the line, and read the answers off slopes and intercepts rather than a single sampled height.
That reframe is more fundamental than anything in this deliverable's own timing section, and it should be
the backbone.

What I take from it into this proposal, wholesale:

- The `S + k * I` cost-model reframe with the geometric k-sweep and least-squares fit. It is the correct
  model of a composition's cost and it answers the predecode break-even analytically. My per-stage
  sub-timing sits inside it (build sub-timings sum to `S`, run sub-timings decompose `I`), so the two are
  complementary and I adopt its model as the frame my stages populate.
- The two reference floors (native ceiling and null-dispatch interpreter). This is the cleanest dispatch
  attribution available, and I reuse it directly as the mechanism for sub-timing the run-half stages that
  cannot carry an in-loop barrier.
- Moving the checksum out of the hot loop. Correct on both counts (cleaner dispatch signal, stricter
  cross-validation), adopted as a carrier-wide change.
- The cold-many regime beside the hot-single regime. This is the single most important premise overhaul in
  either deliverable, because the dispatch ranking plausibly inverts between them and that inversion maps
  onto the runtime's two real usage patterns. Adopted as a first-class regime axis.
- The program-shape basis and the runtime (not compile-time) cost-model champion. Adopted; my per-stage
  adaptive selector is a strict extension of its whole-program selector.

What I do not take, or replace:

- Its scope decision to measure only the two axes form and dispatch and to explicitly exclude every other
  stage as "different workloads." That is the one place it is too conservative for this directive. Value
  representation, operand access, fusion, output building, CSE, and interning were all benched in isolation
  and all are stages of the same shared-IR pipeline (some after a small IR extension); excluding them
  leaves most of the corpus uncomposed. I replace its two-axis matrix with the eight-stage pipeline matrix,
  keeping its form-by-dispatch grid as Tier 0.
- Its fractional-factorial as the ceiling. It uses fractional-factorial to keep the matrix small; I keep
  fractional-factorial as the default committed tier but not as the definition of the matrix. The maximal
  matrix is the full coherent cross, generatable on demand, run in tiers. This is a difference of ambition
  the directive asks for, not a correctness disagreement.
- Its single timed region. It measures `S` and `I` by regression over one end-to-end region. I add the
  per-stage sub-timing harness feature so `S` decomposes into build-stage costs directly and the run half
  decomposes by reference-floor differencing, giving attribution the single-region regression cannot. This
  is additive to its model, not a replacement of it.

Net: adopt its cost-model spine, its reference floors, its checksum move, its cold-many regime, and its
runtime selector unchanged; extend its two-axis matrix to the full eight-stage pipeline; add the per-stage
sub-timing harness feature on top of its single-region timing. The two deliverables compose into one
design: its rigour on how to measure a composition, this one's breadth on how many composable stages the
shared IR actually has and how to time each in place.
