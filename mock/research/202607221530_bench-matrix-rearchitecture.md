# Bench composition matrix: a first-principles rearchitecture before we write it

**Date:** 2026-07-22
**Status:** design deliverable, pre-implementation. Critiques the in-flight composition-matrix plan,
answers the four questions raised to op, and proposes the purpose-built shared IR program, the matrix
dimensions, the reporting normalization, and the heuristic-selection variants. Nothing here is built
yet; this is the architecture to agree before the matrix is written, so we build the illuminating
experiment once instead of a plausible-but-shallow one twice.

The short version: the current plan measures the right cells but over the wrong program. A random
`generate()` sample fixes, at one unknown point, exactly the properties (opcode predictability,
operand locality, arity mix) that DRIVE every number the matrix reports. Those properties are not the
backdrop; they are the axes. Make the program purpose-built and parameterized along them, normalize
every cell against a per-program native ceiling so dispatch overhead is legible, and add honest
runtime heuristic selectors as first-class cells. Then the matrix reveals the interaction surface
instead of one slice of it.

## What the current plan gets right

- The shared-program correctness contract is sound: one `generate()` output, encoded at different
  layouts or predecoded to flat, every cell folding the identical rolling checksum, cross-validated
  byte-exact. That is a real "same program, different composition" guarantee, not an asserted one.
  Keep it, absolutely, and extend it (below).
- Form x dispatch as the implementation axes is correct, and expressing the branch-lowering strategies
  as dispatch shapes over the shared opcode (switch=jump-table, ifchain=cascade, plus bittree and
  perfect-hash) is the right way to fold the branch-strategy question into the interpreter rather than
  bolting on the unrelated synthetic `ab_*` workload.
- The instinct to sweep iteration count is correct and important (it is the amortization/tier axis;
  see Q1).

## Where it is too shallow, and why it matters

### 1. The program is a random sample, not a probe. This is the load-bearing flaw.

Every dispatch-shape difference the matrix measures is, mechanistically, a difference in how a shape
handles the *opcode stream's predictability* and the *operand reference locality*. Threaded dispatch
wins when the opcode stream is correlated (its per-handler indirect branch learns the local
successor distribution); fntable's single indirect-call site wins when the stream is high-entropy;
the switch jump-table sits between; ifchain wins when a few ops dominate in frequency order. The
record-form differences are, mechanistically, differences in *working-set size* and *operand-load
locality*.

`generate(GenParams::default_point())` fixes `op_correlation`, `locality_window`, `op_vocab`, and the
arity mix at one point and hands the matrix a single sample of program-space. The matrix then reports
where the crossovers fall *for that one sample*. The A1 vocab-4-vs-vocab-17 result already showed the
crossovers MOVE with a program property. So a one-program matrix is measuring one unknown coordinate
and calling it the answer. That is the difference between "flat-threaded wins here" and "flat-threaded
wins in the predictable-local regime, loses in the scatter regime, and here is the boundary."

The fix is not to add machinery to the IR. The generator already exposes the three knobs that matter:
`op_correlation` (0 = i.i.d. random opcode stream, high = long runs of one op), `locality_window`
(operand draws from the k most recent nodes), and `op_vocab`. The flaw is that the plan treats these
as fixed defaults instead of as **designed profile dimensions of the matrix**.

### 2. The dispatch signal is diluted by a fixed per-node workload, and we never quantify by how much.

Every cell does the same per-node work beyond dispatch: the operand loads, the op itself, the
`results[i] = v` store, and the `hash = rotate_left(7) ^ v` keep-alive. That fixed work is fair (it is
identical across cells) but it means a 2x dispatch-shape difference surfaces as maybe a 1.2-1.5x bench
difference, because dispatch is only a fraction of per-node cost. The crossovers we have been reading
(1.2-1.5x) are therefore LARGE underlying dispatch deltas, diluted by a constant. The plan never makes
the dilution legible, so a reader cannot tell whether "1.26x" means "dispatch barely matters" or
"dispatch matters enormously and is 60% of a small per-node budget." Without the dispatch fraction, the
matrix numbers are uninterpretable in absolute terms.

### 3. No native ceiling per program, so "how close to the metal" is invisible.

`native_madd` exists but is used only in the isolated native-ceiling bench. In the matrix, every cell
should be reported as a multiple of a per-program native ceiling (the same program executed with no
opcode dispatch). "flat-threaded is 1.26x wire-switch" is a relative fact between two interpreters;
"flat-threaded is 1.4x native and wire-switch is 1.8x native" is the fact that tells you flat-threaded
closes two-thirds of the interpreter's gap to compiled code. The second framing is the one that
informs the runtime's tiering decision (is the interpreter good enough, or is the JIT worth it), which
is the actual design question underneath all of this.

### 4. The adaptive champion, as specified, is not honest and not ambitious enough.

Selecting (form, dispatch) by a const generic `N` resolves at compile time per monomorphized size,
which no real runtime can do. And selecting only by program size ignores the properties that actually
determine the winner (predictability, locality, iteration budget). A champion that is both honest
(runtime branch on measured properties, overhead included in the timing) and property-aware is the
single most illuminating cell in the whole matrix, because it answers "can a runtime that picks cheaply
at load time beat every fixed strategy across the whole program-space?" That is worth designing well,
not bolting on.

### 5. Iteration count is real but under-framed.

Fixing `ITERS = 16` bakes the amortization regime into a constant and hides the cold-vs-hot story
exactly where predecode's honesty lives. This is right in the plan's Q1 instinct; it just needs to be
elevated to a first-class axis and named for what it is: tier pressure.

## First principles: what drives each measured mechanism, and what the program must control

| Measured thing | Mechanism | Program property that drives it | Generator knob |
|---|---|---|---|
| dispatch shape (switch/fntable/threaded/ifchain) | branch prediction of the opcode stream | opcode-stream predictability + vocabulary | `op_correlation`, `op_vocab` |
| record form (wire stride, flat) | working-set size + operand-load locality | program size + operand reference distance | `node_count`, `locality_window` |
| record width / spill | how many nodes exceed inline operand slots | arity mix (fraction of ternary/high-arity) | (needs op-weight control, see below) |
| decode vs dispatch balance | fraction of nodes that are leaves vs ops | leaf fraction | (needs op-weight control) |
| dispatch as a fraction of per-node cost | per-op execution weight | presence of a heavy op | (needs a heavy op, optional) |
| amortization (predecode/tiering) | how many times the program runs before discard | iteration count | bench regime axis |

Three of these are already controllable (`op_correlation`, `locality_window`, `op_vocab`,
`node_count`). Two need a small, clean generator addition: **per-op sampling weights**. One is optional:
a heavy op.

## The purpose-built shared program

Keep the IR itself as is (17-op DAG, const pool, children-before-parents, the wire layouts, the flat
predecode). It is comprehensible and sufficient. The purpose-built-ness goes into the GENERATOR and a
set of DESIGNED profiles, plus one minimal generator upgrade.

### Generator upgrade: replace uniform op selection with an op-weight vector

Today op selection is `rng.below(op_vocab)` (uniform over the first `op_vocab` opcodes). Replace with
an `op_weights: [u16; op::COUNT]` field on `GenParams`: opcodes are sampled proportional to their
weight (with `op_correlation` still governing run-length correlation on top). This one change makes
arity mix, leaf fraction, and heavy-op fraction all directly controllable, because each of those is
just a reweighting of the vocabulary. It subsumes `op_vocab` (a vocab of size k is weights that are
zero past k) so `op_vocab` can stay as a convenience constructor. Minimal, orthogonal, no IR change.

### Optional: one heavy op for dispatch-fraction control

Add a single multi-cycle opcode (for example a 64x64->128 widening multiply reduced back, or a fixed
short dependent chain, ~8-12 cycles) so that a profile can dial the per-node execution weight up and
thereby dial the dispatch FRACTION down. Running the matrix at both a light-op profile (dispatch is a
large fraction, shape differences are stark) and a heavy-op profile (dispatch is a small fraction,
shape differences wash out) directly quantifies critique #2: it shows how much of the per-node budget
dispatch actually is. This is optional for a first cut but is the clean way to make the dilution
legible without hardware counters.

### The designed profiles (program-space points)

Each profile is a named `GenParams` preset chosen to stress one mechanism and to make different cells
win, so the matrix has contrast instead of one coordinate. Proposed set (more is strictly better for
insight; this is the minimum that spans the mechanisms):

- **P_madd** (single-motif, native-anchored): the existing madd chain. One repeated motif, tight
  locality. Purpose: the clean native-ceiling calibration point (native_madd specializes it exactly),
  and the maximally-predictable dispatch case (threaded and ifchain should shine).
- **P_tight** (predictable + local): `op_correlation` high (~900), `locality_window` small (~8),
  moderate vocab. The hot-loop-body case. Predicts threaded/ifchain win on dispatch, flat wins on
  locality.
- **P_scatter** (unpredictable + cache-hostile): `op_correlation` 0, `locality_window` = node_count
  (uniform), full vocab. The adversarial case. Predicts fntable/switch single-site prediction win,
  threaded punished, wide records punished.
- **P_wideselect** (arity-heavy): op-weights concentrated on ternary SELECT plus binary ops. Purpose:
  stress record width and the spill path (SELECT is inline in REC16+, spills in REC12). Predicts the
  width axis matters most here.
- **P_leaf** (decode-bound): op-weights heavy on CONST/INPUT. Purpose: dispatch-light, decode-heavy;
  shows where form (decode cost) dominates and dispatch shape barely matters.
- **P_real** (balanced): the current `default_point`, as the "typical program" reference.

Six profiles. Each is one deterministic preset. The cross-validation contract extends: within a
profile all forms/dispatch agree on the checksum; across profiles the checksums differ (asserted, so we
know the profiles genuinely differ and are not accidentally the same program).

## Reporting: native-normalized, per profile

Every profile that is hand-specializable to native (P_madd cleanly; any single-motif profile) carries a
native baseline cell, and every interpreter cell in that profile is reported as `x native`. For the
multi-op profiles where a faithful hand-native is not practical, report cells relative to the fastest
interpreter cell in that profile AND cite the nearest native-anchored profile, so the reader can place
the whole profile on the absolute scale. The deliverable of the matrix is then a set of tables reading
"in regime X, the composition spread runs from A x native (best) to B x native (worst), and the
crossover between shapes S1 and S2 falls at size N / iteration count I."

This is the single highest-value change beyond the questions asked: it turns relative interpreter
trivia into a decision-grade statement about how close each composition gets to compiled code, which is
what the runtime's interpret-vs-tier-up decision actually needs.

## The matrix: dimensions and controlled-experiment discipline

Dimensions, grouped by how they are expressed:

- **Program profile** (P_madd, P_tight, P_scatter, P_wideselect, P_leaf, P_real): one bench per profile.
  Holding the profile fixed per bench is the experimental control: within a bench only the
  implementation varies, so any difference is attributable to form x dispatch, not to the program.
- **Form** (wire12, wire16, wire24, wire32, flat16): variant axis.
- **Dispatch** (switch, fntable, threaded, ifchain, bittree, perfecthash): variant axis. Cells =
  form x dispatch.
- **Native** (per profile where anchorable): the ceiling cell.
- **Size / working set** (`node_count` 64..16384): the harness's within-bench sweep.
- **Iteration count / tier pressure** (1, 4, 16, 64, 256): the amortization axis (see Q1). Expressed as
  a small number of iteration-count variant families (the timed region runs the program `iters` times;
  `iters` is a compile-time const per family), so a profile bench is emitted at a few iteration points.
- **Heuristic selectors** (per-program and per-region): variant cells present in every profile bench.

That is a large space. Do NOT brute-force the full cross-product; that is the "meaningless explosion"
failure. Design it as a controlled experiment with a staged build:

- **Stage 1, the interaction surface.** Profiles {P_madd, P_tight, P_scatter, P_real} x form {wire24,
  flat} x dispatch {switch, fntable, threaded, ifchain} + native (where anchorable) + the per-program
  selector, at iters=16, size swept. This is the core result: how form x dispatch interacts with
  program character, native-normalized. ~4 profiles x (2x4 + 1 native + 1 selector) = ~40 variants.
- **Stage 2, the tier axis.** Re-emit Stage 1's most informative profiles at iters {1, 16, 64, 256} to
  draw the cold-vs-hot crossover and the predecode amortization boundary.
- **Stage 3, the width and branch axes.** Add forms {wire12, wire32} and dispatch {bittree,
  perfecthash} on the profiles where they matter (P_wideselect for width, P_scatter for branch).
- **Stage 4, the payoff.** The per-region heuristic selector across all profiles and regimes.

Each stage is a coherent commit with its own findings. The architecture supports the full elaborate
matrix; the staging keeps every step tractable and attributable.

### Variant naming

`carrier_mx_{profile}_{form}_{dispatch}[_i{iters}]`, e.g. `carrier_mx_tight_flat_threaded`,
`carrier_mx_scatter_wire24_fntable_i64`, `carrier_mx_madd_native`, `carrier_mx_real_sel` (per-program
selector), `carrier_mx_real_rsel` (per-region selector). Generated by one `gen_carrier_matrix.py` from
a declarative profile x form x dispatch table, so the whole matrix is data, not hand-written crates.

## Answers to the four questions

1. **Iteration-count axis: yes, and reframe it as tier pressure.** Sweep {1, 4, 16, 64, 256}. It is the
   cold-vs-hot / amortization axis and the honest home of the predecode caveat. It is a first-class
   dimension, not a constant.

2. **One program or several: several, and designed, not random.** Program properties (predictability,
   locality, arity mix, leaf fraction) are themselves the axes that drive the results. Use the six
   designed profiles above (each a fixed `GenParams` preset), one bench per profile, so every cell's
   win is shown to be regime-specific rather than a one-sample artifact. This is the biggest upgrade.

3. **Adaptive champion: runtime selection on measured properties, overhead included, and add a
   per-region selector.** Key it on cheaply-measurable program properties at load time (node_count as a
   working-set proxy, the known iteration budget, and a sampled opcode-run-length and arity estimate),
   branch at runtime, and count the measurement cost in the timing. Selection variable is the tuple
   (size, iteration budget, sampled predictability/arity), not size alone. See the next section.

4. **Branch strategies as dispatch shapes: confirmed, and widen the dispatch axis.** Fold them in as
   dispatch shapes over the shared opcode: switch (jump-table), ifchain (cascade), bittree (balanced
   comparison tree), perfecthash (hash-to-table). The synthetic `ab_*` branch workload stays a separate
   bench (it does not run the IR program); optionally cross-reference its finding in the P_scatter
   analysis, since P_scatter is the interpreter-side instance of the same unpredictable-branch question.

## Should we write heuristic-selection variants? Yes, and they are the point.

Op's framing is exactly right: not hand-assigned per-branch or per-chunk variants, but variants that
CHOOSE a strategy from a cheap heuristic. Two, both honest (runtime measurement + runtime branch, cost
included):

- **Per-program selector.** At load, compute a cheap feature vector: node_count, the iteration budget,
  a sampled estimate of opcode run-length (predictability) and arity mix from a small prefix of the
  program. Map the feature vector to a (form, dispatch) choice via a decision rule fitted to the Stage 1
  and Stage 2 tables (for example: few iterations -> wire-switch cold; many iterations + predictable ->
  flat-threaded; many iterations + scatter -> flat-fntable; leaf-heavy -> flat-switch). The claim to
  test: this selector is best-or-tied against every fixed cell across the WHOLE profile x size x
  iteration space, which no fixed cell can be. If it is, that is the headline result: adaptive selection
  strictly dominates any fixed interpreter shape, and the cost of the heuristic is negligible.

- **Per-region selector (the ambitious, most illuminating one).** Segment the program into runs by a
  cheap single forward pass (for example maximal runs of one opcode, or fixed-size windows), estimate
  each run's local predictability, and dispatch each run with the locally-best shape (a predictable run
  gets threaded, a scattered run gets fntable), switching shape at run boundaries. This tests whether
  INTRA-program adaptivity beats whole-program choice, which is the question a real tiering runtime
  faces when different regions of one program have different character. It is still a heuristic (the
  segmentation and per-run choice are computed from a measure, not hand-assigned), which is exactly what
  op asked for. If per-region beats per-program on the mixed profiles, that is a genuinely novel result
  about interpreter design, and it is the "far beyond what these can" that op is chasing.

Both selectors' measurement passes run inside the timed region (or are amortized like predecode and
that amortization is itself swept), so their overhead is paid honestly and shows up in the numbers.

## Cross-validation and honesty invariants (carried and strengthened)

- Every cell in a profile folds the identical checksum; the harness cross-validates. Non-negotiable.
- Native-anchor cells fold the same checksum as the interpreters (native_madd already does).
- Across profiles, checksums differ (asserted), proving the profiles are genuinely distinct programs.
- Selector overhead is inside the timed region or amortized-and-swept; never hidden.
- Every table reports x native where anchored, and states the regime (profile, size, iters) so a number
  is never quoted context-free. This is the "caveats survive summarization" rule applied to the matrix.

## Recommended immediate decisions (for op)

1. Adopt the six designed profiles (or a subset of at least P_madd + P_tight + P_scatter + P_real) as
   separate benches; drop the single-random-program plan.
2. Add the `op_weights` generator field (small, orthogonal) to enable arity-mix and leaf profiles;
   treat the heavy op as optional stage-3 work.
3. Adopt native-normalized reporting on the anchorable profiles.
4. Elevate iteration count to a swept tier axis {1, 16, 64, 256}.
5. Build both heuristic selectors; make the per-program selector a Stage 1 cell and the per-region
   selector the Stage 4 payoff.
6. Widen dispatch to {switch, fntable, threaded, ifchain, bittree, perfecthash}; keep the `ab_*`
   synthetic branch bench separate and cross-referenced from P_scatter.

The net: same correctness contract, but the program becomes a designed probe over the properties that
drive the results, every number becomes legible against the native ceiling, the amortization/tier axis
is explicit, and the heuristic selectors turn the matrix from "which fixed shape wins" into "does
adaptive selection beat every fixed shape, and by how much, and where." That is the elaborate,
meaningful experiment worth building once.

## Amendment: audit of the sibling deliverable, a better framework, and the synthesis

A sibling fork produced `202607221600_bench-composition-rearchitecture.md` ("measure the cost model, not
a point") independently. It is excellent, and on the central reframe it is stronger than my original
above. This section audits it honestly, states what I take, leave, share, and replace, then pushes past
both into a better bench FRAMEWORK and a single synthesized recommendation.

### What the sibling gets more right than my original

1. **The line model `total(k) = S + k*I`, fitted, not sampled.** My original elevated iteration count to
   a swept axis {1,16,64,256}. The sibling reframes each composition as a LINE in (k, time): a one-time
   setup intercept `S` plus a per-evaluation slope `I`, measured by sweeping `k` over a geometric ladder
   and fitting by least squares with a reported R^2. This is strictly better: it turns the predecode
   break-even into an analytic number (`k > S_predecode / (I_wire - I_flat)`) with a confidence bound
   instead of a value read off between two sampled points, and the R^2 turns any non-linearity into a
   finding rather than noise. **I replace my iteration-count axis wholesale with the sibling's
   line-fit.**

2. **Two reference floors, native AND null-dispatch, for slope decomposition.** My original proposed
   native-normalization (good) and a heavy-op axis to reveal the dispatch dilution (indirect). The
   sibling adds a *null-dispatch* interpreter (same operand loads, same result store, same checksum, but
   every node executed as one fixed op with no dispatch) as an upper-structure floor. Then every slope
   decomposes cleanly into native-compute + interpretation-structure + dispatch, with dispatch isolated
   exactly. This is a direct, cleaner solution to my critique #2 than my heavy-op idea. **I take the
   null-dispatch floor and retire the heavy-op-for-fraction argument** (a heavy op may still be a realism
   knob, but it is no longer how we measure the dispatch fraction).

3. **Measurement-fidelity fixes I did not have.** Move the checksum out of the hot loop (post-pass fold
   over the whole results array): removes the per-node hash from the per-evaluation term AND is a
   stricter cross-validation witness. Decouple the input seed from program size (a fixed seed array
   indexed by the eval counter, not `input[k % N] ^ k`, which currently ties input to node count and
   folds the loop counter into the value). Floor every timed region to >=10us so the 42ns CNTVCT quantum
   does not swamp small programs, and measure `S` in its own rebuild-each-repeat region so it is direct
   and cross-checks the regression intercept. **I take all three.** These are real soundness gaps my
   original missed. Honest cost: they shift the A1-A3 absolute numbers, which is correct (those measured
   dispatch-plus-hash); the relative conclusions must be re-established on the clean signal.

4. **Two cache-state regimes: hot-single and cold-many.** My original varied program *properties*
   (predictability, locality, arity) but all in the warm-cache steady state. The sibling adds an
   orthogonal and arguably more important axis: warm (one program run k times, hot icache and predictor,
   the per-frame regime) versus cold (a batch of distinct programs each run a few times, cold at every
   boundary, the load-many-small-scripts regime). The dispatch ranking may invert between them, and that
   inversion maps directly onto the runtime's two real usage patterns. **I take the cold-many regime; it
   is a dimension I missed entirely.**

5. **Oracle envelope + cost-model selector as the champion pair.** The sibling's oracle envelope (the
   lower envelope of all measured cells = the theoretical best the space contains) and its cost-model
   selector (a runtime that holds a small `(S, I)` calibration table, computes predicted `total(k)` at
   the actual node_count and expected_k, and picks the cheapest, paying the branch and lookup) are more
   principled than my "decision rule fitted to the tables." The headline becomes how closely the
   model-driven pick tracks the oracle. **I replace my hand-fitted per-program selector with the
   sibling's model-driven selector-vs-oracle-envelope framing.**

6. **Fractional-factorial design from the shipped-default centre.** Build the full form-by-dispatch grid
   only at the canonical (wire24, switch, canonical-shape, hot) centre, then take one-axis slices out.
   Cleaner experimental design than my "staged build by value," and every interaction reads relative to
   the shipped default, which is the comparison a reader wants. **I take it, and fold my staging into
   it.**

### What of mine survives and should be shared into the merged design

- **The concrete, mechanism-named program profiles.** The sibling's shape basis is abstract ("corners of
  the space"). My named presets are more directly buildable and each targets one mechanism: `P_madd`
  (single-motif, the clean native-anchor), `P_tight` (predictable+local), `P_scatter`
  (unpredictable+cache-hostile), `P_wideselect` (arity-heavy, the record-width/spill stress the
  sibling's basis does not call out), `P_leaf` (decode-bound), `P_real` (balanced). **Contribute these as
  the concrete instantiation of the sibling's shape-basis corners.**
- **The `op_weights` generator mechanism.** The sibling says "extend the generator to expose topology and
  operand-locality" but gives no mechanism. My per-op sampling-weight vector is the minimal, orthogonal
  way to control arity mix, leaf fraction, and (with a heavy op) op-cost in one field, subsuming
  `op_vocab`. **Contribute the mechanism.**
- **The per-region heuristic selector.** Neither the sibling's cost-model selector nor my per-program
  selector is intra-program adaptive. My per-region selector (segment the program by a cheap forward
  pass, dispatch each run with the locally-best shape) is a genuinely additional, more ambitious cell.
  **Keep it, and upgrade it (below) by fusing it with the sibling's cost model.**

### Take / leave / share / replace, at a glance

- **Take from sibling:** the `S + k*I` line-fit reframe; the null-dispatch floor; the three
  measurement-fidelity fixes; the cold-many regime; the oracle-envelope + cost-model-selector champions;
  the fractional-factorial-from-centre design.
- **Replace in mine with sibling's:** my iteration-count axis -> the line fit; my native-only
  normalization -> the two-floor decomposition; my hand-fitted per-program selector -> the model-driven
  selector; my staged-by-value build -> the fractional-factorial design.
- **Share into the merge (mine that the sibling lacks):** the six mechanism-named profiles; the
  `op_weights` generator field; the per-region selector.
- **Leave (mine, now redundant):** the heavy-op-as-dispatch-fraction-probe argument (the null-dispatch
  floor does it better); the discrete iteration points (the line fit subsumes them).

### Beyond both: a better framework for this kind of bench

Both deliverables still describe a matrix of hand-authored cdylib cells with post-hoc analysis. The
deeper move op is asking for is to make the FRAMEWORK fit the shape of this problem. This class of bench
("a composable implementation with a setup-plus-per-op cost model, measured across program-property and
cache-state axes, decomposed against floors, with an adaptive selector validated against an oracle") is
general enough to deserve first-class harness support, not per-bench scaffolding. A better framework
provides five primitives:

1. **Regression cells as a first-class bench type.** The bench declares "I am a cost-model cell" and the
   harness owns the geometric `k`-ladder, the >=10us duration-floor auto-calibration, the least-squares
   fit, and the reported `(S, I, R^2)`. Authors never hand-roll a `k` sweep or a fit again; the harness
   emits the line per cell. This generalises the existing throughput-cell type.

2. **Floor decomposition as a harness service.** The bench registers its native and null-dispatch floor
   cells once; the harness subtracts them and reports every cell's slope as native-compute + structure +
   dispatch automatically, per form (a null-dispatch floor per form, since wire-decode structure differs
   from flat structure, a refinement on the sibling's single floor). No bench wires the decomposition.

3. **Regime as a harness execution mode over pure cells.** Make the cell a pure function (program bytes
   -> checksum) and let the harness drive it warm (one program, `k` evals) or cold (a batch of distinct
   programs). hot-single and cold-many stop being different cell code and become how the harness runs the
   same cell. This is what lets the two regimes share one implementation and be provably the same code.

4. **Isolated cross-product codegen from a declarative spec.** The authoring cost of N cdylibs is the
   reason the matrix feels heavy. A framework primitive takes the orthogonal axis-implementations (the
   form decoders, the dispatch shapes) declared ONCE as pure pieces and emits the isolated per-cell
   cdylibs for the requested fractional-factorial cross-product, preserving the per-cell codegen
   isolation that prevents cross-composition optimizer contamination (the whole reason cells are separate
   binaries) while removing the hand authoring. The generators already gesture at this; the framework
   makes it a primitive: declare the axes, get the isolated matrix.

5. **Oracle envelope and selector validation as harness outputs.** Having every cell's `(S, I)` across
   `(shape, regime, n)`, the harness computes the lower envelope, auto-derives the cost-model selector's
   calibration table, and reports a candidate selector's REGRET against the envelope as a curve plus its
   integral (a single scalar: how much adaptivity leaves on the table). Selector validation becomes a
   framework service, not a bespoke analysis.

Two original refinements that neither deliverable has, enabled by this framework:

- **Unify the two regimes into one warmup-curve fit.** The hot/cold split is really the endpoints of a
  warmup transient: the first evaluation runs cold (predictor/icache untrained), later evaluations run
  warm. Instead of two separate benches, fit `total(k) = S + sum_{j<k} I(j)` where `I(j)` decays from a
  cold slope to a warm asymptote, and extract BOTH the cold-first-eval slope and the warm asymptotic
  slope from ONE richer `k`-sweep, plus the warmup shape itself. This measures the sibling's two regimes
  as one continuous curve and additionally reveals how many evaluations it takes each shape to warm up,
  which is the exact quantity a tiering runtime needs to decide when a program is "hot."

- **The per-region cost-model selector: fuse my per-region idea with the sibling's model.** Segment the
  program by a cheap forward pass, and for each segment use the fitted `(S, I)` model to pick the
  per-segment-best dispatch, switching shape at segment boundaries. This is honest (runtime measurement +
  runtime branch), model-driven (not hand-fitted), AND intra-program adaptive. It is the strongest
  champion in the space, and if it beats the whole-program cost-model selector on the mixed profiles, it
  is the genuinely novel result: intra-program strategy selection strictly dominates whole-program
  selection, which is the "far beyond what these can" op is chasing.

### The synthesized recommendation (the single best proposal)

Build the interpreter-composition bench as a **cost-model matrix on a purpose-built program basis, with
floor decomposition, two regimes unified as a warmup curve, and a validated adaptive selector**, on a
framework that provides regression cells, floor decomposition, regime execution modes, isolated
cross-product codegen, and oracle/regret reporting as primitives. Concretely:

- **Correctness contract (both agree, non-negotiable):** every cell runs the identical `generate()`
  program per (shape, seed), differs only in memory form and dispatch, and folds the identical post-pass
  checksum; the harness cross-validates byte-exact before trusting any timing, and asserts distinct
  shapes yield distinct checksums.
- **Cells:** form {wire12, wire16, wire24, wire32, flat16} x dispatch {switch, fntable, threaded,
  ifchain, bittree, perfecthash}, plus the native and per-form null-dispatch floors.
- **Program basis:** my six mechanism-named profiles, generated via the `op_weights` field plus
  `op_correlation` and `locality_window`, arranged as the sibling's fractional-factorial corners from the
  (wire24, switch, P_real) centre.
- **Regime / amortization:** the `S + k*I` line fit over a geometric `k`-ladder as the core measurement,
  extended to the warmup-curve fit that yields cold slope, warm slope, and warmup length in one sweep;
  the cold-many batch regime as the cross-check where the warmup curve's cold end should agree.
- **Reporting:** every slope decomposed into native + structure + dispatch against the two floors; the
  predecode break-even computed analytically; every number quoted with its (profile, n, k, regime)
  regime and its ns-to-cycles-to-IPC sanity line.
- **Champions:** the oracle envelope; the whole-program cost-model selector measured against it with a
  regret curve; and the per-region cost-model selector as the ambitious payoff.
- **Discipline (both agree):** serial runs, >=3 per cell, minimum for throughput and median+spread for
  regression points, no ranking inside the noise floor, committed CSVs and the tracked `.bench_history`
  trail travelling with every bench commit, threaded cells honestly recorded as toolchain-blocked if a
  shape fails to compile rather than worked around.

The net over both originals: the sibling's cost-model rigor and floor decomposition and regime insight,
plus my concrete profiles and generator mechanism and per-region adaptivity, plus the two refinements
(warmup-curve unification and the per-region cost-model selector) and the framework primitives that make
the whole elaborate experiment cheap to author and reusable beyond interpreters. That is the best
proposal the two deliverables can produce together.

## Second amendment: audit of the third deliverable and the final unified proposal

A third fork landed `202607221615_bench-maximal-composition-matrix.md` ("the maximal composition matrix").
It is the broadest of the three and it changes the ceiling of the whole exercise. It already adopts the
cost-model spine from `202607221600`, so the three deliverables now agree on the measurement backbone;
the third's contribution is breadth and attribution. Audited here, then folded into a single final
proposal.

### What the third gets that neither my synthesis nor the second had

1. **A composition is a path through an eight-stage pipeline, not a form-by-dispatch pair.** The stages:
   optimize (none / CSE / bounded eqsat), intern (none / hashed / sharded), record form (five wire strides
   / flat), value representation (static / tagged / nanbox), operand access (inline / pool-spill /
   interned-ref), dispatch (switch / fntable / threaded / ifchain / bittree / perfecthash), fusion (none /
   pairwise), output building (overwrite / cow / reuse). Nearly every isolated carrier bench maps onto one
   stage. This directly answers op's original expansion ("all of the things we've been benching, in
   compositions"): it inventories the whole corpus and turns each isolated finding into a composable axis,
   where my synthesis only composed form x dispatch plus program profiles. **I adopt the pipeline framing
   as the correct maximal structure. My form-by-dispatch grid becomes its Tier 0 spine.**

2. **Per-stage sub-timing: measure where the time goes within one composition run.** End-to-end time plus
   a named sub-timing per stage that executes in that path, on one warm state. Build-half stages (decode,
   optimize, intern) get direct sub-timing via a proposed upstream `timed_stage!` macro and a
   per-(variant, size, stage) CSV schema; run-half stages (dispatch, operand access, output), which are
   fused in the hot loop and would be destroyed by an in-loop barrier, get differential attribution
   against reference floors. **This is the highest-upside timing idea in any of the three, and it
   generalizes the two-floor decomposition: the run-half differential attribution IS the null-dispatch /
   inline-only floor differencing, applied per stage.** So the second deliverable's floor decomposition
   and my native-normalization are the same mechanism the third uses at per-stage granularity. I adopt
   per-stage sub-timing, with the important note that only the build-half direct sub-timing needs the
   upstream `timed_stage!` change; the run-half floor-differential attribution works today, so we can
   start there and add the harness feature for build-half detail.

3. **Extend the IR with a per-node type tag so value representation composes.** valrepr currently runs on a
   separate mixed-type mini-IR; a type tag on the shared node makes static / tagged / nanbox a real axis
   over the shared program. **I adopt it as a staged, off-by-default extension** (tag off for the
   static-canonical spine so the A1-A3 comparison survives up to the checksum move), because it is the only
   way value representation becomes composable rather than a side bench.

4. **The maximal matrix as definition, tiers as committed evidence.** The full coherent cross is ~20,000
   cells; the third keeps it maximal-in-definition and runs it in tiers (Tier 0 spine = form x dispatch 36
   cells; Tier 1 single-axis slices from the shipped-default centre; Tier 2 targeted pairwise for
   predicted-strong interactions like optimize-by-form and fusion-by-dispatch; Tier 3 full cross on
   demand). This is a cleaner and more honest resolution of "maximal but readable" than my staged-by-value
   build or the second's fractional-factorial-as-ceiling. **I adopt the tier structure; my
   fractional-factorial-from-centre is exactly its Tier 0 plus Tier 1.**

5. **A per-stage adaptive selector.** Beyond the second's whole-program cost-model selector, the third
   proposes a selector that picks each STAGE independently from the calibration table, composing a path the
   fixed matrix never enumerated, and tests whether per-stage-composed beats whole-program (i.e. whether
   the stages are independent). **I adopt it, and observe it is orthogonal to my per-region selector:**
   per-stage varies the strategy across the pipeline for the whole program; per-region varies one stage
   (dispatch) across program segments. The strongest possible champion fuses both: per-stage strategy
   selection with the dispatch stage additionally per-region. That fusion is mine to contribute on top of
   the third's per-stage idea.

### What of my synthesis survives against the third (share up)

- **The concrete mechanism-named profiles and the `op_weights` generator mechanism.** The third says
  "extend GenParams with a topology/depth knob" but does not name the arity-heavy or leaf-heavy profiles
  or give the sampling mechanism. My `P_wideselect` (arity/spill stress), `P_leaf` (decode-bound),
  `P_madd` (native anchor), and the `op_weights` vector are the concrete instantiation of its shape basis.
  Share up.
- **The warmup-curve regime unification.** Neither the second nor the third has it: both keep hot-single
  and cold-many as two separate regimes. Fitting `total(k) = S + sum_{j<k} I(j)` with `I(j)` decaying from
  a cold to a warm slope extracts both regimes' slopes AND the warmup length from one k-sweep, and the
  warmup length is exactly the "when is a program hot" quantity a tiering runtime needs. Keep as a
  refinement on the third's regime axis.
- **The better-framework primitives.** My "better framework" section (regression cells, floor
  decomposition, regime execution modes, isolated cross-product codegen, oracle/regret reporting as
  first-class harness services) is the natural home for the third's per-stage sub-timing (a sixth
  primitive) and its tiered on-demand generation. The third proposes the `timed_stage!` harness feature in
  isolation; my framing makes it one of a coherent set of primitives that turn this whole class of bench
  into declared-axes-in, decision-grade-surface-out. Keep, and fold `timed_stage!` in as a primitive.

### Take / leave / share / replace against the third

- **Take from the third:** the eight-stage pipeline framing; per-stage sub-timing (build-direct /
  run-differential); the IR type-tag extension for value-rep (staged, off by default); the maximal-matrix
  tier structure; the per-stage adaptive selector.
- **Replace in my synthesis with the third's:** my form-by-dispatch-plus-profiles matrix -> its eight-stage
  pipeline with form x dispatch as Tier 0; my fractional-factorial-from-centre -> its Tier 0 through Tier 3
  (same idea, better articulated and open-ended).
- **Share up (mine/second's that the third should absorb):** the cost-model line fit and two floors (the
  third already took these from the second); my concrete profiles + `op_weights`; my warmup-curve
  unification; my framework-primitives framing as the home for its `timed_stage!`; the per-region selector
  as the fusion partner for its per-stage selector.
- **Leave / caution:** the full ~20,000-cell cross is a real over-reach risk (a strong interaction can hide
  in an unrun Tier-3 cell); accept maximal-in-definition but commit evidence outward from Tier 0 on signal,
  not by trying to fill the space. The `timed_stage!` upstream harness change is the one cross-repo
  dependency; sequence it after the floor-based run-half attribution (which needs no upstream change) so the
  matrix is not blocked on it. The IR type tag shifts every absolute number and complicates the layout;
  stage value-rep composition after the form x dispatch x optimize spine is solid, not up front.

### The final unified proposal (best across all three)

Build the interpreter-composition bench as an **eight-stage pipeline matrix** (the third's breadth,
answering "compose everything we've benched") measured with the **`S + k*I` cost-model line fit, two
reference floors, and the post-pass checksum** (the second's rigor), instantiated on **six mechanism-named
program profiles via an `op_weights` generator field** (mine), with **per-stage attribution** (build-half
direct via `timed_stage!`, run-half differential via the floors), the **warmup-curve regime fit** that
yields cold slope, warm slope, and warmup length in one sweep (mine) cross-checked by the **cold-many batch
regime** (the second's), and a **three-tier champion ladder**: the oracle envelope, the whole-program
cost-model selector, and a **per-stage-plus-per-region cost-model selector** as the payoff (the third's
per-stage fused with my per-region). All of it sits on a **cost-model bench framework** that provides
regression cells, floor decomposition, regime execution modes, isolated cross-product codegen, per-stage
sub-timing, and oracle/regret reporting as reusable primitives (my framework section, with the third's
`timed_stage!` as one primitive), so the maximal matrix is a declarative axis catalogue emitting a tiered,
on-demand, decision-grade surface rather than hand-authored cells.

Rollout, sequenced so nothing blocks on the one upstream change:

1. **Carrier fidelity (all three agree, do first):** move the checksum out of the hot loop, decouple the
   input seed from program size, floor every timed region to >=10us. These re-baseline A1-A3 on a clean
   signal.
2. **Tier 0 spine:** the form x dispatch grid with native and per-form null-dispatch floors, measured as
   `S + k*I` lines on the `P_real` centre. Subsumes and replaces the original two-axis plan.
3. **Program basis:** the six profiles via `op_weights` + `op_correlation` + `locality_window`.
4. **Tier 1 slices, highest-leverage stage first:** optimize (CSE lifted from `cheap_lowering`, then
   bounded eqsat) is the highest-leverage axis because a pre-pass that shrinks node count changes the
   working set every downstream stage pays; then intern, fusion, output building.
5. **Regimes:** the warmup-curve fit as the unified hot/cold measurement, cross-checked by the cold-many
   batch.
6. **Per-stage attribution:** run-half floor-differential first (no upstream change), then the
   `timed_stage!` harness feature for build-half direct sub-timing.
7. **Value-rep axis:** the IR type-tag extension, off by default, staged after the spine is solid.
8. **Champions:** oracle envelope, then whole-program cost-model selector, then the per-stage-plus-per-region
   selector, reporting regret against the envelope.

This is the strongest suggestion the three deliverables produce together: the third's pipeline breadth and
per-stage attribution, the second's cost-model rigor and floor decomposition and cold regime, my concrete
profiles and generator mechanism and warmup-curve unification and framework primitives, and a fused
per-stage-plus-per-region adaptive champion that no single deliverable proposed. It composes the entire
corpus over one shared IR program, times each stage in place, decomposes every number into
compute-plus-structure-plus-dispatch, draws the amortization and warmup and working-set crossovers as
curves rather than points, and asks the one question worth the whole apparatus: can an adaptive runtime,
choosing each stage (and dispatch per region) from a cheap measured model, beat every fixed composition
across the entire program-and-regime space, and by how much.

## Third amendment (LOUD): are the variants actually representative, or hacked to look good?

Op asked the question that matters most and that the other deliverables skated past: forget the elegant
matrix design, are the variants I have ALREADY WRITTEN honest? Does any variant win because of how it was
hacked together to mimic its shape rather than because the shape is genuinely faster? Does any lose
unfairly? Are they truly isolated? I audited the real carrier source and the built cdylibs. The answer is
that there is one serious fairness bug, two unconfirmed label claims, and clean isolation. The landed A1
and A3 threaded findings are confounded and must not be trusted until the bug is fixed.

### The serious one: the threaded variants cheat on bounds checks.

Source-verified, not inferred. Grepping the interpreter modules for raw-pointer (`.add()`, unchecked)
versus indexed (`results[..]`, bounds-checked) operand access:

- `interp.rs` (switch, fntable, ifchain): **0** raw-pointer sites. Every operand read is bounds-checked
  `results[idx]` indexing.
- `interp_threaded.rs` (wire threaded): **7** raw-pointer sites. Every operand read is unchecked
  `*r.add(idx)`.
- `predecode.rs`: the switch and fntable flat interpreters use bounds-checked `results[a]`; the
  flat-threaded submodule uses unchecked `*r.add(idx)` / `*p.add(i)` (15 raw-pointer sites in the file,
  all in the threaded path).

So the threaded and flat-threaded variants elide the two-per-node array bounds checks (a compare plus a
predictable conditional branch each) that switch, fntable, ifchain, and the flat switch all pay. This is
not a property of threaded dispatch. It is an artefact of how I wrote it: the preserve-none handlers
thread a raw `*mut u64` because carrying a `&mut [u64]` slice with its lifetime through the guaranteed-
tail-call chain is awkward, and the raw pointer silently dropped the bounds check along with the lifetime.
The variant is measuring "threaded dispatch AND unchecked access" against "switch dispatch WITH checked
access," and reporting the difference as if it were dispatch alone.

The magnitude is not negligible and it aligns suspiciously with the findings. Two elided checks per node
matter most exactly where the threaded variants looked best: in the L1-resident small-n regime, where per-
node cost is a few nanoseconds and a predictable branch is a real fraction of it, A1 had threaded winning
by ~2% and A3 had flat-threaded winning by 1.24-1.49x. At large n the checks hide under memory latency,
and there A3 had flat-threaded merely TYING plain flat. That pattern (threaded wins where checks are cheap
to skip, ties where they are hidden) is exactly what an elided-bounds-check advantage would produce. So a
meaningful part, possibly most, of the A1 and A3 threaded advantage may be the missing checks, not the
dispatch shape. I flag this loudly: **the A1 and A3 threaded results are confounded and must be re-run
after the access discipline is normalized before any "threaded wins" claim is trusted.** A2 (predecode
flat beats wire) is clean, because both its variants are switch and both are bounds-checked, so that 10-26%
result stands.

The fix, and it is a fidelity requirement not a nicety: normalize the access-checking discipline across
every interpreter so the only thing that varies is the axis under test. The honest choice is that ALL
interpreters use `get_unchecked`: the program is validated once (children-before-parents, every operand an
earlier index, asserted by `is_well_formed` and the untrusted-load verifier), so every operand access is
provably in range and the bounds check is pure overhead a real runtime would also elide after validation.
Making them all unchecked removes the confound and measures dispatch cleanly. The alternative (all checked)
is not reachable for the threaded ABI-threaded raw pointer without re-adding an explicit compare, which
would itself be a different hand-shape. Unchecked-everywhere is the fair and realistic normalization.

### Two label claims I have NOT actually confirmed at the instruction level.

- **"switch" assumes the `match` lowers to a jump table.** The 17 opcodes are dense and contiguous (0..16),
  which is the case LLVM usually lowers to a jump table, so it is probably faithful, but I did not confirm
  it in the disassembly (inlining plus libstd noise defeated a quick objdump). If LLVM instead lowered the
  match to a balanced-comparison tree or an if-chain, the "switch" cell is mislabeled and the
  switch-versus-ifchain and switch-versus-bittree comparisons are partly degenerate. This must be confirmed
  with `cargo-show-asm` or a focused disassembly of the interpret loop before the dispatch axis is trusted.
- **"threaded" assumes preserve-none actually elides the callee-saved spills.** I proved the guaranteed
  tail call is real (a 20,000-node program does not overflow the stack, which only holds if `become` is a
  jump, not a call), so the tail-call half of the mechanism is confirmed. But the preserve-none half (no
  callee-saved register preserved across the dispatch, which is the actual source of the advantage) is not
  confirmed at the instruction level. `rust_preserve_none_cc` is an incomplete nightly feature; if it
  silently fell back to the standard ABI on some handler, the threaded cell would be paying spills and thus
  measuring something that is not preserve-none. The disassembly of a handler (look for the absence of
  `stp`/`ldp` of x19-x28 around the dispatch) must confirm it.

fntable is in better shape: the indirect call (`blr`) survives in the built dylib, so LTO did not
devirtualize the function-pointer table back into a switch, which was the real risk for that cell. It
appears faithful. ifchain is written as an explicit `if/else if` cascade and is structurally what it
claims, though it has not been benched in the matrix yet and its ordering (ascending opcode, not
frequency) is a documented choice that a profile-ordered variant should be compared against.

### A subtler asymmetry between forms, not just dispatch.

The flat variants may get a second, quieter bounds-check break: in the flat loop the compiler can often
prove the node index `i` is in range for the whole `results`/`nodes` slice and hoist or elide the check,
whereas the wire loop's operand indices come through `from_le_bytes` and are harder to prove, so its checks
survive. If so, part of the flat-over-wire win is elided checks on the flat side, not just the smaller
footprint. The `get_unchecked`-everywhere normalization fixes this too, and it should be verified that
after normalization the flat-over-wire margin (A2) holds, which I expect it will because the footprint and
wire-arithmetic differences are real and independent of checking.

### Isolation: this part is clean.

Every variant is its own cdylib, built with `lto = "fat"` and `codegen-units = 1`, dlopened by the harness
and run in its own subprocess, and the program crosses the FFI boundary as opaque bytes so a variant's
optimizer cannot see the program and cannot partially evaluate the interpreter over it (the exact failure
that made the original native-ceiling bench measure native-versus-native). Each variant statically links
its own copy of the carrier through the path dependency, so there is no shared-carrier object that could
let one variant's codegen affect another. Cross-variant isolation is airtight. The only within-variant risk
is the LTO devirtualization/lowering question above (does fntable stay indirect, does switch stay a jump
table), which is about whether a cell is labelled correctly, not about contamination between cells; fntable
already checks out, switch needs confirming.

### Per-variant verdict

- **switch (wire, flat):** representative IF the match is a jump table (confirm at ISA level); no unfair
  advantage; bounds-checked (the fair baseline).
- **fntable (wire, flat):** representative, indirect call confirmed to survive LTO; bounds-checked; fair.
- **ifchain (wire, flat):** structurally faithful; bounds-checked; fair; not yet benched; compare against a
  frequency-ordered variant.
- **threaded (wire, flat):** UNFAIR ADVANTAGE from unchecked access (source-confirmed), AND the
  preserve-none mechanism is unconfirmed at the ISA level. Do not trust A1/A3 threaded numbers until both
  are fixed: normalize to unchecked-everywhere, and confirm the no-spill codegen.
- **predecoded switch/fntable:** representative; the predecode is an honest one-time setup cost (`S`), not a
  hidden per-evaluation advantage, and the cost-model reframe measures it explicitly.

### What this adds to the build plan

Before the matrix is built, a fidelity pass is now step zero, ahead of even the checksum move: (1)
normalize every interpreter to `get_unchecked` operand access so dispatch is the only axis that varies; (2)
confirm at the ISA level, with `cargo-show-asm` or focused disassembly, that switch is a jump table,
fntable stays an indirect call, and threaded actually elides callee-saved spills, recording any cell that
fails to match its label as mislabelled rather than shipping it; (3) re-run A1 and A3 on the normalized,
confirmed variants and correct the beating-attempts log, because the current threaded findings are stated
with more confidence than the code earns. Only then does the composition matrix mean what it says, because
a matrix built on variants that each cheat differently measures the cheats, not the compositions. This is
the ten-honesty-rules "assert every layout" and "strongest opponent" rules applied to the variants
themselves: a variant must be the honest strongest form of the shape it names, and right now the threaded
variants are not, they are the shape plus a bounds-check break, and that has to be corrected before the
elaborate matrix is worth building on top of them.

## Fourth amendment: make the carrier and variants final-like, so the insight is usable

The audits so far fix fairness (make each variant the honest form of its shape). Op's last point pushes
further and it is the one that makes the whole exercise pay: fairness gives naive-but-honest comparisons,
and a naive-but-honest comparison of shapes that no real runtime would ship is still only trivia. The
shared IR, the way it evaluates, and each variant should express the shape a FINAL runtime would actually
use, so the matrix measures the design space of a real interpreter tier and the oracle envelope becomes a
buildable runtime rather than the winner of a toy race. Concretely, the current carrier has three
strawman properties that every variant inherits and that a real runtime would never have.

### Strawman 1: it stores every node's result forever. A real runtime allocates slots by liveness.

`results[i] = v` for every node, kept for the whole pass, is the single biggest gap between the carrier
and a real runtime. It means the working set is the ENTIRE program (node_count u64 slots), and every
form/cache crossover we have measured is over that whole array. A real runtime does register allocation:
it computes each value's last use and reuses freed slots, so the live working set is the program's
liveness WIDTH (the maximum number of simultaneously-live values), which for typical DAGs is a small
constant, not the node count. This changes the form and cache story fundamentally, because the resident
hot set stops growing with program size once it fits the live width. The naive carrier makes large
programs look memory-bound when a real slot-allocated runtime would keep them L1-resident.

Proposal: add a liveness/slot-allocation stage (a cheap backward last-use pass at predecode, emitting a
per-node slot index into a small slot arena sized to the liveness width). Operand access then reads from
`slots[slot_of[operand]]` and the working set is the live width. Make slot allocation an operand-access /
output-building axis value ("slot-allocated" versus the naive "store-all"), so the matrix measures exactly
how much the store-everything strawman inflated the working-set crossovers, and the final-like cells run
slot-allocated. This is the highest-value realism fix; without it the form axis is measuring a fiction.

### Strawman 2: the flat/predecoded form is a 1:1 transcription. A real runtime predecodes to an OPTIMIZED form.

`predecode` today copies each wire node to a flat record unchanged. A real runtime's load/warm path folds
constants, eliminates dead nodes, does CSE, and fuses superinstructions while it predecodes, so the flat
form it runs is SMALLER and cheaper than the source. The optimize pipeline stage (CSE from `cheap_lowering`,
bounded eqsat, plus constant folding and DCE) belongs IN the predecode path for the final-like cells, not
as a separate untaken axis. The insight this unlocks is the real one: how much does a warm runtime's
optimize-on-load buy over naive transcription, and does the shrunken node count move every downstream
form/dispatch crossover (it will, because it changes the working set). The predecode cost of the optimize
work is an honest part of `S` and the cost-model line captures its amortization.

### Strawman 3: the keep-alive is a rolling hash. A real runtime produces outputs and reuses buffers.

The per-node `rotate_left(7) ^ v` is a cross-validation and anti-DCE device, not what a runtime does. The
second-deliverable fix (fold one checksum post-pass) already removes it from the hot loop; the final-like
step is that the "output" is the live-out values written through the output-building stage (overwrite /
cow / reuse-arena), which is a real runtime concern, with the post-pass checksum computed over the live-out
set purely for cross-validation. So the output stage measures a real materialization strategy, and the
keep-alive stops being a fixed per-node tax that dilutes every signal.

### Per-variant: the strongest realistic form of each shape

Beyond making them fair (unchecked-everywhere post-validation), each variant should be the strongest
version of its shape a real runtime would ship, so the matrix compares best-against-best, not
naive-against-naive:

- **threaded:** thread a raw instruction pointer advanced by stride (or a predecoded-record pointer) and
  keep the hot interpreter state (ip, slot base, dispatch table base) minimal and register-resident across
  the `become` chain, rather than re-deriving `view(d)` and recomputing offsets each handler as the current
  code does. That re-derivation is naive overhead a real context-threaded interpreter does not pay; removing
  it is what makes threaded actually express the preserve-none advantage.
- **fntable:** the returning-loop indirect call is already close to its strongest form; ensure the handler
  does only the op and the store, with the loop owning dispatch, and confirm LTO keeps the call indirect
  (it does).
- **switch:** confirm the jump table; if LLVM ever fails to emit one for the dense arms, force it (a
  computed-goto-style table of label addresses is the honest strongest switch, though Rust cannot express
  labels-as-values, so the fallback is an explicit function/blockaddress table, which is really the
  threaded/fntable shape and should be labelled as such rather than pretending to be a switch).
- **predecoded + slot-allocated + fused + threaded/native:** add this as an explicit cell, the honest
  strongest interpreter the carrier can express (optimized flat form, slot-allocated working set,
  superinstruction-fused hot pairs, preserve-none dispatch). It is the materialized oracle-envelope path,
  and its gap to the native ceiling is THE usable number: how close can the best interpreter tier get to
  compiled code, which is exactly the interpret-versus-JIT tiering decision the runtime has to make.

### Why this is the usable-insight step, not gold-plating

With these, the matrix spans from the shipped-default naive interpreter (wire, switch, store-all, checked,
no optimize) at one end to the final-like optimal interpreter (optimized-flat, slot-allocated, fused,
threaded-or-native) at the other, with every pipeline stage's contribution attributed along the way. The
deliverable stops being "switch beats fntable by X on a toy" and becomes: "the shipped naive interpreter is
N times slower than the final-like interpreter, the final-like interpreter is M times native, here is which
pipeline stage buys each part of the gap, here is the program size and iteration count where each stage's
choice flips, and here is whether an adaptive selector that picks each stage from a cheap measure captures
the oracle envelope." That is a design document for the runtime's interpreter tier, derived from
measurement, which is the usable insight op is asking for and which naive variant comparisons structurally
cannot produce.

The discipline caveat stays: keep the naive baselines in the matrix as the reference points (the shipped
default is genuinely naive wire-switch-store-all, and the delta from it to the final-like shape is the
number that justifies the work), and hold every final-like addition to the same cross-validated checksum
and the same honest cost-model measurement, so "final-like" never means "optimized until it is no longer
the same program." The correctness contract is what keeps the elaborate, realistic carrier honest: every
cell, naive or final-like, computes the identical result on the identical program, or the bench fails.

## Fifth amendment: audit of the third deliverable's own addenda, and the corrected final synthesis

The third deliverable (`202607221615`) grew four addenda after I last synthesized it: a representativeness
audit, an IR-representativeness note, a final-like proposal, and a fifth addendum correcting the fourth. I
audited all four. The headline is corroboration on the thing that matters, several points where its
addenda are sharper than mine, and one genuine correction to my own fourth amendment that I adopt.

### Strong corroboration: two independent audits found the same confound

Its representativeness addendum reaches the identical core finding as my third amendment, independently and
with precise line citations: the threaded and flat-threaded interpreters use unchecked raw-pointer access
(`interp_threaded.rs:67,50`, `predecode.rs:203-204,202`) while switch, fntable, and the flat switch use
checked indexing (`interp.rs:35,64`, `predecode.rs:67,73`), so the threaded advantage is dispatch plus
bounds-check elision, and A1/A3 are overstated by an unquantified amount. Two independent reads landing on
the same bug is the strongest signal the arc has that it is real. Both prescribe the same fix (one shared
operand-load/result-store primitive used identically by every dispatch cell, checked-vs-unchecked promoted
to its own axis if interesting). This is now high-confidence, not a hypothesis: fix it before the matrix.

### Sharper points in its addenda that I absorb

- **The if-chain is a strawman as written.** `interpret_ifchain` orders its cascade by ascending opcode, so
  hot binary ops sit behind cold comparisons, whereas the match-lowering finding it exists to test was
  about a FREQUENCY-ordered chain. As written it will under-perform and an "if-chain loses" result would be
  an artifact of the ordering. I flagged the ordering as a documented choice; the third correctly flags it
  as a result-invalidating strawman. Fix: frequency-order the cascade (and state the ordering is part of
  the strategy) or carry both orders as a sub-axis. Absorbed.
- **The setup asymmetry is per-variant, not uniform.** Every cell excludes its setup, but the flat forms
  exclude parse PLUS predecode while the wire forms exclude only parse, so at a fixed iteration count the
  flat forms get a strictly larger free ride. I framed predecode as an honest `S` term; the third correctly
  notes it is an ASYMMETRIC free ride that flatters flat until the `S + k*I` sweep replaces the fixed count.
  This sharpens my A2 caveat: part of the flat win at fixed iters is the larger excluded setup, which the
  line fit dissolves. Absorbed.
- **Straight-line IR structurally understates the dispatch axis.** This is the biggest point I missed. The
  shared IR is a straight-line DAG evaluated in index order, with no branches or loops. Threaded dispatch's
  whole advantage is that the indirect branch at each handler tail learns the local opcode-successor
  distribution, and that is largest in hot LOOP BODIES where a handler sequence repeats. A linear stream
  never exercises it, so the threaded numbers are understated in exactly the direction that matters for the
  runtime, and my `op_correlation` profile knob (correlation on a linear stream) is a weaker proxy than real
  control flow. The IR should carry basic blocks and terminators (branch, loop back-edge, call) so dispatch
  is measured where it actually earns its keep. Absorbed as a first-class IR extension, higher priority than
  I had it; it also subsumes the `cfg` module into the shared IR rather than keeping a separate CFG bench.
- **Ground the form axis in vehje's real tiers.** The third maps the form axis onto vehje's actual
  tier-tagged residual (cold serialized residual as it crosses the ABI, the predecoded baseline arena the
  runtime builds on load, native code), so the matrix answers the runtime's real question (how much does
  building the baseline arena buy over interpreting the residual, and how far short of native does the best
  baseline interpreter fall, so is the bytecode or native tier worth building) rather than "which record
  width is fastest." I gestured at native-normalization; this nails it to the shipped ABI contract.
  Absorbed as the primary reporting lens for the form axis.

### The one correction to my own fourth amendment I adopt

The third's fifth addendum corrects a real error in my fourth amendment (and in its own fourth addendum):
I framed "final-like" as making the carrier RESEMBLE the shipped tier-tagged runtime. That is the wrong
posture for a bench. A bench does not conform to the already-chosen shape to validate it; it measures every
shape on its merits, as equals, including shapes the runtime does not use and might be better off adopting.
The correction changes three calls:

- **No shape is the privileged baseline.** wire24+switch stays only as a fixed normalization anchor that
  makes tables readable, explicitly NOT "the default" or "the thing to beat." My fourth amendment said
  predecode is the baseline and wire-decode is exotic "because that is what a real runtime does"; that
  pre-decides an answer the measurement should produce. Corrected: every cell is judged against native and
  against every other cell on merit, and the oracle envelope is the best MEASURED shape regardless of
  whether the runtime would have picked it.
- **The fidelity fixes stay, but for the honest reason.** The value arena, control flow, real output sink,
  typed values, and the shared operand primitive belong not because they make the carrier look like the
  runtime, but because each removes an ARTIFACT that buries some approach's true merit (store-every-node
  buries storage-reuse shapes; straight-line buries the loop regime; the mixed operand access measures
  something other than dispatch). Keep store-every-node and wire-decode-per-node as real measured cells too,
  not just worst-case references.
- **The tier mapping is a reporting lens, offered, not imposed.** Useful for reading the numbers for the
  tiering decision, applied after the fact; it must not constrain what is measured.

I adopt this correction and amend my fourth amendment accordingly: the goal is an artifact-free design-space
explorer, not a runtime look-alike.

### The beyond-runtime approaches it adds, which I take

The strongest consequence of the design-space-explorer framing is that the most valuable results are the
ones telling the runtime to do something it is not doing. The third names approaches outside the current
tier plan, each a first-class cell:

- **Vertical, data-parallel (SoA) interpretation: interpret one program over many inputs at once, SIMD
  across inputs, one dispatch amortized over a vector of values.** This is the standout idea in any of the
  three deliverables, because it maps directly onto vehje's real per-record evaluation (the same residual
  run over a COLUMN of records), so it could dominate exactly the workload the runtime cares about most
  while being nowhere in the current tier plan. It deserves its own axis (scalar vs vertical), not a
  footnote, and it is potentially the single largest result the matrix can produce. Fully taken.
- **Copy-and-patch stencil execution** (the weval / Cranelift lineage): the cheap near-native tier between
  interpreter and full native, answering whether jumping straight from baseline interpreter to full native
  skips a tier that is most of the win for a fraction of the cost. Taken.
- **Computed-goto / token-threading in the Zig cdylib**: Rust cannot express label-as-value computed goto,
  the Zig side can, and the cross-language cdylib isolation already in place drops a Zig dispatch cell into
  the same matrix on the same program bytes. Taken; it also closes the gap the "not expressible in Rust"
  saga (A1) left, from the other direction.
- **Trace / superblock dispatch** (dispatch per hot straight-line run, not per op) and **alternative
  residual encodings** (register vs stack bytecode vs the SSA value-graph the carrier uses now): both taken
  as axes, the residual-encoding one because the runtime's baseline-form choice rests on it and it is
  currently just assumed.

### What of my synthesis still stands, now reframed

The third's addenda do not supersede my contributions; they reframe them:

- The `S + k*I` line fit synthesized from the second, the warmup-curve regime unification, the `op_weights`
  generator field and the concrete mechanism-named profiles, and the fused per-stage-plus-per-region
  cost-model champion all still stand, and the design-space-explorer posture strengthens them: the champion
  is judged against the true oracle envelope (best measured shape, runtime-agnostic), and the profiles and
  the control-flow extension together give the program basis the entropy AND the loop structure the
  dispatch axis needs.
- The framework-primitives vision (regression cells, floor decomposition, regime modes, isolated
  cross-product codegen, per-stage sub-timing, oracle/regret reporting) is exactly the right home for a
  design-space explorer: it measures ALL shapes as equals, including the beyond-runtime ones, and the
  per-stage sub-timing plus the vertical/SoA and copy-and-patch and Zig-computed-goto cells all slot in as
  additional declared axes rather than bespoke benches.

### The final word across all three deliverables

The complete design, fusing all three: a **design-space-exploration bench**, not a shipped-shape validator,
that measures every interpreter shape on its merits against a native ceiling and a null-dispatch floor,
over one shared program expressed as a **block-structured, typed, control-flow-carrying IR** with a
**liveness-allocated value arena**, run through a **`S + k*I` cost-model warmup-curve fit** across designed
program profiles and cache regimes, with **per-stage attribution** decomposing every number into
compute-plus-structure-plus-dispatch, spanning the full pipeline of composable stages AND the
**beyond-runtime shapes** (vertical/SoA, copy-and-patch, Zig computed-goto, trace dispatch, alternative
residual encodings), with every cell fair (one shared operand primitive, no bounds-check confound, no
strawman ordering), every setup honestly accounted by the line fit, and the whole thing judged by an oracle
envelope and a per-stage-plus-per-region adaptive selector measured against it, on a framework that makes
the maximal matrix a declarative axis catalogue emitting a tiered, decision-grade surface. The immediate
gate before any of it: fix the bounds-check confound both audits found, re-run A1/A3, and stop trusting the
threaded numbers until they measure dispatch alone. That is the strongest, most honest, most illuminating
version of the experiment the three deliverables describe together, and it answers the question worth the
whole apparatus: across the entire space of interpreter shapes, including ones vehje does not build today,
which composition an adaptive runtime should choose at each point, how close it gets to native, and where
the shape it currently ships leaves the most on the table.
