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

### `202607221530` (first-principles rearchitecture; designed program profiles)

Landed after this deliverable's first draft (commit f9bc225, 289-line committed snapshot audited here; its
working tree had grown to 469 lines and may still be in progress, so the additions past the commit are not
audited). It is the strongest of the three on one specific and load-bearing point: the program itself is
the dominant variable, and a single random `generate()` sample fixes, at one unknown coordinate, exactly
the properties (opcode-stream predictability, operand locality, arity mix, leaf fraction) that mechanically
drive every dispatch and record-form number. Its fix is designed program profiles, each a fixed `GenParams`
preset that stresses one mechanism: P_madd (native-anchored single motif), P_tight (predictable + local),
P_scatter (unpredictable + cache-hostile), P_wideselect (arity-heavy, stresses spill), P_leaf
(decode-bound), P_real (balanced). Plus a minimal generator upgrade (an `op_weights` vector replacing
uniform op selection, which makes arity/leaf/heavy profiles expressible orthogonally), native-normalized
per-profile reporting (every cell as x-native, turning relative interpreter trivia into the
interpret-versus-tier-up decision the runtime actually faces), a dispatch-fraction-dilution critique with an
optional heavy-op knob to make dispatch's share of the per-node budget legible, and two honest heuristic
selectors including a per-region one that dispatches different program regions with the locally-best shape.

What I take from it into this proposal:

- The designed program profiles, wholesale, as the concrete instantiation of this deliverable's
  program-shape basis. My shape-basis section named the dimensions (vocabulary, topology, operand-locality)
  but left the points generic; 1530's P_madd/P_tight/P_scatter/P_wideselect/P_leaf/P_real are the concrete,
  mechanism-targeted presets that basis should be. I replace my generic four-to-five-point basis with its
  named profile set.
- The `op_weights` generator upgrade. It is the clean mechanism that makes arity mix, leaf fraction, and a
  heavy-op profile all controllable by reweighting the vocabulary, and this proposal's operand-access,
  spill, and value-representation axes need exactly that control. Adopted.
- Native-normalized-per-profile reporting. It composes with my per-stage sub-timing: report each stage's
  cost and the whole path both in absolute time and as a multiple of the per-profile native ceiling.
  Adopted as the reporting standard.
- The per-region selector, as a champion strictly more ambitious than my per-stage adaptive selector, and
  orthogonal to it: per-region varies dispatch across spatial regions of one program; per-stage varies the
  strategy across pipeline stages. Both compose, and a selector that does both is the maximal champion.
  Adopted alongside mine.
- The dispatch-fraction-dilution critique and the heavy-op knob, as a complement to 7081's null-dispatch
  floor: the floor isolates dispatch by subtraction, the heavy-op profile isolates it by dilution ratio.
  Both, cross-checked, are stronger than either.

What I do not take, or where mine differs:

- Its decision to keep the axis set at form x dispatch (x profile x iters) and hold the IR fixed. That is
  right for its scope but narrower than this directive's maximal remit: it does not compose the optimize,
  intern, value-representation, operand-access, fusion, or output-building stages that the corpus benched in
  isolation. I keep my eight-stage pipeline as the axis set and fold its profiles in as the shape axis, so
  the matrix is profiles x the full pipeline, not profiles x form x dispatch.
- Its "keep the IR exactly as is." Value representation cannot compose over a u64-only IR, so I keep the
  per-node type-tag extension; but I adopt its minimal-change discipline (the tag is off on the static
  canonical path, and the extension is the only IR change).

Net across all three: 1530 decides what program (designed profiles + op-weights + native-normalized
reporting), 7081 decides how to measure a composition's cost (S plus k times I, reference floors, hot and
cold regimes), and this deliverable decides how many pipeline stages actually compose and how to time each
in place (the eight-stage matrix + per-stage sub-timing harness feature). The synthesis is one experiment:
the eight-stage pipeline matrix, run over 1530's designed profiles, measured as 7081's cost-model lines with
reference floors and per-stage sub-timings, across the hot-single and cold-many regimes, with per-stage and
per-region adaptive selectors judged against the oracle envelope. None of the three is redundant; each
supplies an axis the other two left pinned.

## Addendum: are the current variants and IR actually representative? (a loud, honest audit)

This is the audit that matters most and is easiest to skip: passing cross-validation proves every variant
computes the same result, and proves nothing about whether every variant measures the same thing. Two
variants can fold the identical checksum while one does incidental work the other does not, or elides work
the other pays. The shipped A1-A3 variants were read line by line for this. There is one real confound that
partially invalidates a published headline, plus several softer issues and one clean bill.

### The confound that matters: the threaded variants elide bounds checks the others pay

This is the loud one. The switch, function-table, and predecoded interpreters read operands and results
through checked slice indexing (`interp.rs:35` `results[d.operand(...) as usize]`, `interp.rs:64`
`results[i] = v`, `predecode.rs:67` `p.nodes[i]`, `predecode.rs:73` `results[a]`). The two threaded
interpreters read them through unchecked raw-pointer arithmetic (`interp_threaded.rs:67`
`*r.add(dec.operand(...))`, `interp_threaded.rs:50` `*r.add(i) = v`; `predecode.rs:203-204`
`*r.add(nd.a)` / `*r.add(nd.b)`, `predecode.rs:202` `*p.add(i)`). The flat-threaded handler elides two
bounds checks per node (the node-array load and each results load) that flat-switch pays.

So the threaded-versus-switch comparison is not a clean dispatch-shape measurement. It confounds the
preserve-none dispatch with bounds-check elision on every operand and result access, and operand/result
access is a large share of per-node cost. The A1 small-n threaded win and the A3 "flat-threaded is
best-or-tied everywhere" both inherit this: some unknown fraction of the threaded advantage is the elided
checks, not the dispatch. It happened because the preserve-none handlers thread a raw `*mut u64` and raw
pointer arithmetic was the path of least resistance, not a considered choice. It is exactly the
"hacked-together advantage" the audit was asked to find, and it means A1 and A3 are overstated by an
unquantified amount until it is fixed.

The fix is not to make threaded checked (that would understate it symmetrically); it is to make operand and
result access an explicit, shared primitive used identically by every dispatch shape, and to promote
checked-versus-unchecked to its own axis if it is interesting. Every interpreter should call one
`#[inline(always)]` operand-load and result-store, so the only thing the dispatch axis varies is dispatch.
Until that lands, the threaded cells are not isolated and their numbers carry an asterisk.

### The if-chain variant is a strawman as written

`interpret_ifchain` (`interp.rs:155+`) orders its cascade by ascending opcode (INPUT, CONST, ADD, SUB, ...),
so a hot binary op sits behind several cold comparisons. The match-lowering finding it is meant to test
(an if-chain beating the jump table) was about a frequency-ordered chain with the hot ops first. As written
the variant measures an arbitrarily-ordered chain and will under-perform the real technique, so an
"if-chain loses" result from this cell would be an artifact of the ordering, not a property of if-chains.
Either frequency-order the cascade (and state the ordering is part of the strategy) or carry both orders as
a sub-axis; do not ship the opcode-ordered chain as "the if-chain."

### Setup asymmetry: the flat forms get a larger free ride

Every variant excludes its setup from the timed region: the wire forms exclude `Decoded::parse`, the flat
forms exclude parse plus `predecode`. The flat forms therefore move strictly more work out of the
measurement, and at the fixed sixteen iterations that free setup is a real advantage a run-once program
would never grant. This is the same setup-versus-per-evaluation honesty gap the cost-model reframe fixes;
noting it here because it is a per-variant asymmetry (flat benefits more than wire), not a uniform one, so
the flat wins are partly a measurement framing that the `S + k * I` sweep must replace before they can be
stated as general.

### Fair-but-diluting, and faithful-disadvantage (not hacks)

Two things look like fairness problems and are not. The per-node checksum fold is paid identically by every
variant, so it does not advantage any cell; it does dilute the dispatch signal uniformly, which is a
reason to move it out of the loop (for signal clarity) but not a representativeness fault. And the threaded
shape's per-node re-derivation of interpreter state (`view(d)` and the node-count reload each handler,
`interp_threaded.rs:54,74`) is intrinsic to context threading under preserve-none, so it is a faithful
disadvantage of the real shape, correctly represented, and must not be "optimised away" to flatter the
cell, or the cell stops representing threading.

### The IR is representative for what it currently composes, and not yet for value representation

The 17-op u64 node DAG is a faithful small value-graph IR, and the wire layouts and flat predecode are
honest representations of it (PNode is genuinely 16 bytes: `repr(C)`, u8 plus three u32 with padding,
`predecode.rs:25`). But value representation does not compose over it: the `valrepr` bench interprets a
separate mixed int/float mini-IR, not this program. Folding a value-representation axis into the shared-IR
matrix without the per-node type-tag extension would put a cell in the matrix that runs a different program,
silently breaking the shared-program contract the whole matrix rests on. Either extend the IR (this
proposal's type tag) so all three representations run the one program and cross-validate, or keep value
representation out of the shared-IR matrix entirely. There is no honest middle.

### cdylib isolation and program opacity: a clean bill

The isolation is real and correct, and this is worth stating affirmatively because it is the foundation the
rest stands on. Each variant is its own crate with `crate-type = ["cdylib"]`, `lto = "fat"`,
`codegen-units = 1`, and a static path dependency on the carrier, so each dylib gets its own monomorphised,
fully-LTO'd copy of the interpreter with no cross-variant inlining, and the harness loads each in a separate
subprocess by dlopen, so no variant can perturb another. The program crosses into each variant as bytes
generated at runtime in a `OnceLock` (`program_vocab` in the get-or-init), so the optimiser never sees the
program and cannot partial-evaluate the interpreter over it, which is the exact failure that made the old
native-ceiling bench measure native-against-native. And the shared-program property is real: every variant
generates from the same fixed `GenParams::default_point()` seed, so all interpret the byte-identical
program, and only the harness input stream varies. Isolation, opacity, and shared-program are sound as
built; the representativeness faults above are in the variant bodies and the framing, not in the harness
scaffolding.

### The bottom line for op

One shipped result is confounded (threaded's advantage is dispatch plus bounds-check elision, currently
inseparable, so A1 and A3 are overstated by an unknown amount), one planned variant is a strawman as
written (opcode-ordered if-chain), one framing advantage favours the flat forms (larger free setup), and
the IR cannot honestly carry a value-representation axis without the type-tag extension. The harness
isolation itself is clean. Before the matrix is written, unify operand access into one shared primitive
across all dispatch shapes, frequency-order (or dual-order) the if-chain, adopt the `S + k * I` setup
accounting, and gate the value-representation axis on the IR extension. With those four, the cells measure
their strategy and nothing else; without them, the matrix inherits the same confounds at scale and produces
a larger body of precisely-wrong numbers.

## Addendum: make the carrier final-like, or the insight is only about a toy

The representativeness audit above is about fairness between variants. This one is about a deeper thing: even
a perfectly fair comparison between naive variants of a toy interpreter yields conclusions about the toy, not
about the runtime vehje will ship. If the carrier interpreter is a naive tree-walk over an arithmetic DAG
with a hash keep-alive, then "flat-threaded beats wire-switch by 1.26x" is a true fact about that naive
interpreter and an unreliable guide to what the real Zig residual runtime should do. To get usable insight,
each variant must be the best realistic form of its strategy, and the shared carrier must be shaped like the
thing being built. Below is what that means concretely, grounded in vehje's actual runtime contract (a
tier-tagged residual: a flat serialized IR arena at the baseline, an optimized bytecode above it, native
code at the top).

### Map the form axis onto vehje's real tiers, so the matrix answers the tiering question

The single most useful reframing: the form axis is not five arbitrary record widths plus a flat form, it is
the runtime's actual execution tiers. The serialized wire record is the residual as it crosses the ABI (the
cold, just-loaded form). The predecoded flat form is the baseline arena the runtime builds on load. The
native-ceiling is the native tier. Framed this way, the matrix stops being "which record width is fastest"
trivia and becomes the decision the runtime actually faces: how much does building the baseline arena from
the residual buy over interpreting the residual directly, and how far short of native does the best baseline
interpreter fall, so is the bytecode or native tier worth building at all. That is decision-grade. Keep the
individual wire widths as a sub-axis of the cold tier (they answer the residual-encoding question), but
report the form axis primarily as cold-residual versus baseline-arena versus native.

### The IR: typed values, a real value arena, and control flow

Three changes move the IR from toy to final-like, in priority order.

Typed values. The u64-only value model is why value representation cannot compose. A final IR carries a
typed value (at minimum int and float; ideally the residual's real value union). Add a per-node result type
so the value representation is intrinsic: the static, tagged, and NaN-boxed cells then interpret the one
shared program and cross-validate, and the value-representation cost is measured on the real IR rather than a
private mini-program. This is the type-tag extension the audit named, stated here as a positive design move
rather than a caveat.

A real value arena with lifetimes. The current interpreter stores every node's result into a
`results[node_count]` array kept live for the whole pass, and at scale that array is the dominant memory
traffic (A2 and A3 were memory-bound at large n precisely because of it). No production interpreter
materialises every intermediate forever; it allocates values into a reused arena sized to the live set, freed
as values die. Model a real value arena with a liveness-driven slot allocator, so the memory the interpreter
touches reflects the live set, not the node count. This is the highest-leverage realism change, because it
attacks the exact term that dominates the large-program regime, and it turns the output-building axis from a
toy post-pass into the real question of how values are allocated and reused. The naive "store every node" is
kept only as the worst-case reference cell.

Control flow. The IR is currently straight-line: a DAG evaluated in index order with no branches or loops in
the program. This is not just unrealistic, it structurally understates the thing the dispatch axis exists to
measure. Threaded dispatch's whole advantage is that the indirect branch at each handler tail learns the
local opcode-successor distribution, and that advantage is largest in hot loop bodies where the same handler
sequence repeats. A linear stream never exercises that. The IR should carry basic blocks and terminators
(branch, loop back-edge, and eventually call), so dispatch is measured under real control flow where the
branch predictor and the threaded shape actually earn or lose their keep. Without this the matrix measures
dispatch on the one workload that least resembles where dispatch matters, and the threaded numbers are
understated in the direction that matters for the runtime. The existing `cfg` module already has a
block-and-terminator machine; the move is to make the shared IR itself block-structured rather than keeping a
separate straight-line DAG and a separate CFG bench.

### Eval: a real output sink, and predecode as the default not the exotic option

Replace the per-node hash keep-alive with a real output sink. The `rotate_left(7) ^ v` per node exists only
to keep results live and to cross-validate; both are achievable with a post-pass checksum over the arena (as
7081 proposes), and the keep-alive should instead be the interpreter writing its actual outputs (the live
results at the end, or a designated output node's value) into the FFI output region, which is real work a
real interpreter does. That makes output building a measured stage rather than an artifact, and removes the
fixed per-node dilution from every dispatch number.

Treat predecode as the baseline, not a contender. A real runtime does not re-parse wire bytes on every
evaluation; it builds a dispatch-ready form once on load and runs that. So the predecoded flat form is the
realistic baseline, and the wire-decode-every-node interpreter is the exotic case (a cold, run-once, or
memory-constrained tier), not the default. Framing the default as the wire switch is itself a toy artifact of
where the arc started. The matrix should center on the baseline-arena forms and treat re-decoding wire as the
cold-tier variant.

### Dispatch: each variant its best realistic form, plus the ones the runtime would actually build

The dispatch axis is only usable if each cell is the optimal realisation of its strategy, so the comparison
is best-against-best, not best-against-strawman.

- Direct threading, not indirect. The current threaded interpreter looks up `TABLE[opcode]` on every step.
  The final threaded form stores the resolved handler pointer directly in the predecoded node, so dispatch is
  a jump through the instruction's own handler field with no table load. Resolve opcodes to handler pointers
  at predecode time and the threaded cell measures true direct threading, which is what a production
  threaded interpreter ships; the table-indirect version stays as a labelled less-optimal point.
- Superinstructions. Fuse the most frequent consecutive producer-consumer node pairs into single handlers
  that compute both and keep the intermediate in a register, skipping its arena round-trip. This attacks the
  memory-traffic term directly and is a standard production technique; it is the cell most likely to move the
  large-program regime, and it composes with the value arena (fusion removes a value from the live set).
- Register or accumulator caching. Keep the most-recently-produced value in a register so an immediately
  consuming node avoids the arena load, the interpreter analogue of top-of-stack caching. Cheap, real, and
  measurable.
- Frequency-ordered if-chain and the perfect-hash and bit-tree strategies, as already noted, each in their
  strongest form.
- One shared operand-load and result-store primitive used identically by every dispatch cell, so the axis
  varies dispatch alone (the audit's confound fix), with checked-versus-unchecked promoted to its own axis if
  it proves interesting.

The insight this yields is the usable kind: not "threaded beats switch on a toy," but "on the block-structured
IR with a real value arena, direct-threaded dispatch with superinstructions reaches N times native and M
percent over the flat-switch baseline in the hot-loop regime, and the gap to native is small enough (or not)
that the bytecode or native tier is (or is not) worth building." That is a statement the runtime design can
act on.

### What stays deliberately simple

Not everything should chase realism. The determinism and the cross-validation checksum contract stay exactly
as they are (they are what make the matrix trustworthy, not what make it a toy). The generator stays a
seeded, dependency-free deterministic function. The point is to make the interpreter and IR shaped like the
real thing, not to import the whole runtime; the carrier remains a measurement instrument, just one whose
measured object now resembles what ships.

### Priority

If only some of this lands before the matrix, the order by insight-per-effort is: the shared operand
primitive (unblocks honest dispatch comparison, small), the real output sink plus post-pass checksum
(removes dilution, small), the value arena with lifetimes (attacks the dominant large-program term, medium),
control flow in the IR (unlocks the regime where dispatch actually matters, medium-large), direct threading
and superinstructions (the cells most likely to change the tiering conclusion, medium), typed values (unlocks
the value-representation axis, medium). Cold-tier framing of the form axis is free and should be adopted
immediately. Each is independently landable and independently improves the fidelity of every cell that
follows.

## Addendum: the bench explores the design space, it does not validate the shipped shape (refines the previous addendum)

The previous addendum is right that a toy carrier yields toy insight, but it over-corrected in one direction:
it kept talking about making the carrier resemble the actual tier-tagged runtime, treating the runtime's
shape as the target the bench should conform to. That framing is wrong for a bench, and this addendum
corrects it. A bench is not there to confirm that the shape already chosen is good. It is there to measure
every shape and approach on its own merits, as equals, including shapes the runtime does not currently use
and might be better off adopting. The runtime's current design is one point in the space, not the frame the
space is measured against.

The distinction is concrete and it changes several calls from the previous addendum.

No shape is the privileged baseline. The previous addendum said predecode is the baseline and wire-decode is
the exotic case because that is what a real runtime does. For the bench that is exactly the wrong move: it
pre-decides an answer the measurement should produce. Wire24-plus-switch stays as the normalization anchor,
but only as a fixed reference point that makes tables readable, explicitly not a claim that it is the default,
the thing to beat, or the shape that ships. Every cell is judged against the native ceiling and against every
other cell on merit, and the oracle envelope is the best measured shape at each point regardless of whether
the runtime would have picked it. If the data says a shape the runtime does not build wins, that is the
result, and it is a recommendation to the runtime, not an anomaly to explain away.

Fidelity changes stay, but for a different reason. The value arena, control flow, real output sink, typed
values, and the shared operand primitive all still belong, but not because they make the carrier look like
the runtime. They belong because each one lets some approach show its true merits instead of an artifact: the
value arena because materialise-every-node is a measurement artifact that buries the shapes which reuse
storage, control flow because a straight-line stream cannot exercise the regime where several approaches earn
their keep, the shared operand primitive because without it the axis measures something other than what it
claims. The motivation is "let every approach compete at its best and remove artifacts," not "conform to the
shipped design." Keep materialise-every-node and wire-decode-per-node as real measured cells too; they are
approaches on their own merits, not just worst-case references.

The tier mapping is one reporting lens, offered, not imposed. Reporting the form axis as cold-residual versus
baseline-arena versus native is a useful way to read the numbers for the tiering decision, so keep it as a
lens available in the analysis. But it must not constrain what is measured or privilege those three points;
the individual widths, the exotic forms, and the beyond-runtime shapes are all first-class cells, and the
tier lens is applied after the fact to the subset it illuminates.

### Approaches beyond the current runtime, worth measuring because the bench might vindicate them

The strongest reason to unforce the shape is that the most valuable results are the ones that tell the
runtime to do something it is not doing. A design-space bench should therefore carry approaches that are not
in the current tier plan, each measured on its own merits:

- Vertical, data-parallel interpretation. Interpret one program over many inputs at once, SoA across inputs,
  so one dispatch is amortised over a SIMD vector of values. This is a genuinely different shape from the
  scalar per-input loop, and it maps directly onto vehje's real per-record evaluation pattern (the same
  residual run over a column of records), so it could dominate exactly the workload the runtime cares about
  most while being nowhere in the current tier plan. This deserves its own axis, not a footnote: scalar
  versus vertical is potentially the largest single result the matrix can produce.
- Copy-and-patch stencil execution. Paste per-op precompiled machine-code stencils into a buffer and run it,
  the tier between interpreter and full native (the Cranelift weval and the copy-and-patch lineage). It is a
  cheap-to-build near-native tier the current plan does not include, and measuring it answers whether the
  jump from baseline interpreter straight to full native skips a tier that is most of the win for a fraction
  of the cost.
- Computed-goto and token-threading in the Zig cdylib. Rust cannot express label-as-value computed goto, but
  the Zig side can, and it is a real dispatch shape the Rust variants structurally cannot represent. The
  cross-language cdylib isolation already in place means a Zig dispatch cell drops into the same matrix on the
  same program bytes; measuring it closes the gap the Rust-only dispatch axis leaves.
- Trace or superblock dispatch. Record hot straight-line runs once and dispatch per-trace instead of per-op,
  amortising dispatch over a whole trace. A different granularity of the same question, and the one that most
  directly attacks dispatch cost in hot loops.
- Alternative residual shapes. Register bytecode versus stack bytecode versus the SSA value-graph the carrier
  uses now are three different residual encodings, and which one interprets fastest is an open question the
  runtime's baseline-form choice rests on; measure all three rather than assuming the value-graph.
- Allocation strategies for the value arena as an axis in themselves: none (materialise all), simple slot
  reuse, linear-scan register allocation. The winner informs how much the runtime should invest in liveness
  at load time.

None of these is required for a first matrix, but the design must leave room for them as first-class cells
rather than treating the runtime's current tiers as the boundary of the space. The point of the whole
exercise, per op, is that we do not force the actual shape of things; we measure all the shapes on their
merits, and the ones that win beyond what the runtime does today are precisely the insight worth having.
