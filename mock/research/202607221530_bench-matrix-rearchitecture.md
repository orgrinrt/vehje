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
