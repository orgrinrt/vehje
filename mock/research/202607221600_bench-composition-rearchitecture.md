# Bench composition rearchitecture: measure the cost model, not a point

**Date:** 2026-07-22
**Status:** proposal. Rearchitecture of the interpreter-composition bench so it measures meaningful,
attributable quantities across the real axes, rather than one number at a hidden operating point. Free
to overhaul the premise where a different framing yields more illuminating data.
**Scope:** the carrier interpreter benches on the shared IR program (dispatch, record-width, predecode,
threaded, and their compositions). Does not touch the non-interpreter benches (reach, incr, eqsat,
interner, retract, cfg, thermo), which run different workloads and cannot share this IR program.
**Companions:** `202607220300_bench-remeasure-synthesis.md` (honest baselines), `202607221200_bench-beating-attempts.md`
(the A1-A3 attempt log), `202607211539_bench-evidence-audit-panel/` (the ten bench-honesty rules).

## The problem with a single operating point

The current composition plan measures each (form, dispatch) pair as one wall-clock number: parse or
predecode once outside the timed region, then run a fixed sixteen interpretation passes inside it. That
number is a single point sampled from a two-parameter cost model, and the sample hides both parameters.
A composition's real cost is a setup term paid once plus a per-evaluation term paid every run. Sixteen
is an arbitrary choice of how many runs amortise the setup, and it happens to sit on one side of every
amortisation crossover, so the bench silently answers "which is faster when you run a program exactly
sixteen times" and presents it as "which is faster."

This is the deepest soundness gap, and it is not the only one. The per-node checksum fold contaminates
the per-evaluation term with work that is identical across variants but dilutes the signal the bench
exists to isolate. The workload is a single program evaluated many times with a warm instruction cache
and a warm branch predictor, which is one real runtime regime but not the only one, and it is precisely
the regime that most favours the shapes the arc has been promoting. The program itself is one generator
shape, and the dispatch findings already moved between two op vocabularies, so one shape measures one
point on a surface known to be non-flat. And the proposed adaptive champion selects on a value known at
compile time, which is not a selection a runtime can make.

None of these are reasons the arc's findings are wrong. They are reasons the arc's findings are
under-determined: true at their operating point, silent about the rest of the space, and stated with
more generality than the measurement supports. The fix is not more careful wording around a point
measurement. It is to measure the cost model whose parameters the point measurement collapses, across
the axes that move those parameters, with the dispatch signal cleanly attributable. That is a more
elaborate bench, and it produces conclusions the current one structurally cannot.

## The reframe: every composition is a line, not a point

Model each composition's cost as `total(k) = S + k * I`, where `S` is the one-time setup cost to get
from wire bytes to a dispatch-ready form (parse for the wire forms; parse plus predecode for the flat
form), `I` is the cost of one interpretation pass over the program, and `k` is the number of times the
program is evaluated. Every composition is then a line in `(k, time)` space with intercept `S` and slope
`I`. The composition matrix becomes a family of lines, and the questions that matter are answered by
their slopes, intercepts, and intersections rather than by a single sampled height.

Measure `S` and `I` directly by sweeping `k` over a geometric ladder (`1, 2, 4, 8, 16, 32, 64, 128`) and
fitting `time = S + k * I` by least squares. The slope is the per-evaluation cost, the intercept is the
setup cost, and the coefficient of determination reports whether the linear model holds. A clean fit
(R^2 near one) means the composition really is "fixed setup plus constant marginal cost" and the two
numbers fully describe it. A poor fit means something non-linear intrudes (a cache tier crossed as the
per-run working set grows, or setup that is not actually amortised), and that deviation is itself a
finding rather than noise to average away.

This single change subsumes the entire predecode-honesty question and answers it analytically. Predecode
pays exactly when `k > S_predecode / (I_wire - I_flat)`: the setup cost of the predecode pass divided by
the per-evaluation saving it buys. Measure the predecode setup and both slopes and the break-even run
count falls out as a number with a confidence interval, instead of being hidden inside the choice of
sixteen. "Flat wins at sixteen iterations" becomes "flat pays for itself after N evaluations, so it wins
for per-frame and per-entity work and loses for run-once scripts," which is the honest claim the caveat
in A2 was gesturing at.

## Attributing the per-evaluation cost: reference floors

A per-evaluation slope on its own says a composition is faster, not why. To attribute the difference to
dispatch rather than to memory traffic or arithmetic, the matrix carries two reference cells that bracket
every real composition.

The lower reference is the shape-specialised native loop that already exists as the native-ceiling
bench: no interpretation, no dispatch, the compiler having seen the whole program. Its slope is the
irreducible compute-and-memory floor for this program's arithmetic.

The upper-structure reference is a null-dispatch interpreter: the same operand loads, the same result
store, the same final checksum, but every node executed as a single fixed operation with no opcode
dispatch at all. Its slope is the cost of the interpreter's memory and bookkeeping structure with the
dispatch removed. The gap between a real composition and the null-dispatch floor is the dispatch cost,
cleanly isolated; the gap between the null-dispatch floor and the native ceiling is the interpretation
structure's overhead beyond dispatch. Every measured slope decomposes into native compute, plus
structure, plus dispatch, and the matrix reports which axis each composition actually moves. Without
this decomposition a two-percent total difference is uninterpretable; with it, a two-percent total that
is a thirty-percent dispatch difference diluted by a large shared memory term is legible.

## Measurement-fidelity fixes

Three changes make each `I` measure what it claims.

Move the checksum out of the hot loop. The interpreter must store every node result, because later nodes
read earlier ones and that data dependency is the real work; but the rolling `rotate_left(7) ^ v` fold
per node is a cross-validation and keep-alive device, not interpretation, and it adds a fixed per-node
cost that dilutes the dispatch signal. Fold one checksum over the whole `results` array after the pass
instead. Cross-validation is preserved and in fact strengthened: a full checksum over every node's final
value is a stricter witness of semantic agreement than a rolling per-node hash that can alias. Keep-alive
is preserved: `results` is consumed by the post-pass checksum and the checksum escapes through the FFI
output, so nothing dead-code-eliminates. This does shift the absolute numbers from the A1-A3 runs, which
is the correct cost of measuring the intended quantity; the relative conclusions are expected to hold and
will be re-established on the cleaner signal.

Decouple the input stream from the program size. The current seed is `input[k % N] ^ k`, which ties the
input index to the node count and folds the loop counter into the value. Use a fixed pre-generated seed
array indexed by the evaluation counter alone, identical across every size and composition, so the only
thing the size axis changes is the program.

Floor the timed-region duration. At twenty-four megahertz the CNTVCT quantum is about forty-two
nanoseconds, and the smallest programs run in a couple of microseconds, a few dozen ticks, too coarse for
a per-evaluation slope. Size the internal repeat count so every timed region, at every `(n, k)` point,
runs at least ten microseconds, keeping the quantisation error under half a percent. Measure the setup
term `S` in its own timed region that rebuilds the form on each repeat rather than reading it from the
one-time cache, so setup is measured directly and cross-checked against the regression intercept; a
disagreement between the two flags a broken assumption.

## The workload regimes: the premise overhaul

The single-program-evaluated-many-times loop measures the steady state with a warm instruction cache and
a warm branch predictor. That is the per-frame and per-entity regime, and it is real, but it is exactly
the regime in which threaded dispatch's per-site branch-predictor pressure and the flat form's compact
footprint show their best case. A runtime also loads many distinct small programs and evaluates each a
few times or once, and there the instruction cache and the branch predictor are cold at every program
boundary, which is the regime that punishes the many-indirect-branch-site shapes and rewards the single
returning call site. Measuring only the warm regime over-generalises every dispatch conclusion.

The rearchitected bench measures two regimes as separate benches sharing the carrier. The hot-single
regime is the cost-model sweep above: one program, `k` evaluations, `k` swept, `S` and `I` extracted. The
cold-many regime evaluates a batch of distinct programs, each generated from a different seed at the same
shape, each evaluated a small fixed number of times, with the batch large enough that no program's
working set stays resident across the batch. The cold-many slope is dominated by the cost the warm regime
never pays: the branch predictor and instruction cache re-learning at each program switch. It is likely
that the dispatch ranking inverts again between the two regimes, and if it does, that inversion is the
most useful single result the matrix can produce, because it maps directly onto the runtime's two real
usage patterns.

## The program-shape basis

Dispatch cost depends on the entropy of the opcode stream and the predictability of its transitions;
memory cost depends on how far a node's operands sit from it in the results array and how deep the
dependency chains run. One generator shape fixes all of these. Sweep a small basis that brackets the
space rather than a full grid: a low-entropy small-vocabulary stream against the diverse full vocabulary
(the v4-against-v17 axis that already showed movement), a deep-narrow topology of long dependency chains
with poor instruction-level parallelism against a wide-shallow topology of many independent nodes, and a
near-operand-locality against a far-operand-locality variant that stresses the results-array cache
behaviour. Four to five shape points, chosen to sit at the corners of that space, are enough to show
whether a composition's win is a property of the composition or an artefact of one program.

## The matrix, formally

A composition cell is a point in `(form, dispatch, shape, regime)`, and for each cell the bench reports
the pair `(S, I)` plus the derived `total(k)` line, in the cold-many regime a single cold slope.

- form: `wire12, wire16, wire20, wire24, wire32, flat16`. The record representation. The narrow wire
  layouts spill high-arity operands to the pool; the flat form is a uniform sixteen bytes. This axis
  interacts with the working-set crossover, because a narrower record stays resident to a larger program
  size.
- dispatch: `switch, fntable, threaded, ifchain, bittree`. Opcode to operation. The switch lowers to a
  jump table, the fntable is an indirect call through a returning loop, the threaded shape is a
  preserve-none guaranteed-tail-call chain, the ifchain is a linear branch cascade, and the bittree is a
  balanced binary search over the opcode. This axis carries the branch-strategy question directly: the
  match-lowering finding that an if-chain beats a jump table on this machine is tested inside the real
  interpreter here, and the tree adds the log-depth branch strategy the isolated bench did not compose.
- shape: the four-to-five point basis above.
- regime: `hot-single` (k-swept) and `cold-many`.

The full cross is too large to build and, more importantly, too large to read. Use a fractional-factorial
design centred on a reference cell of `wire24, switch, canonical-shape, hot-single`, which is the shipped
default. Build the full form-by-dispatch grid at the canonical shape in the hot regime, which is the
primary interaction surface of about thirty cells. Then take one-axis-at-a-time slices out from the
centre: the whole form-by-dispatch grid repeated once in the cold-many regime, and the grid repeated at
each non-canonical shape. Every axis is measured against the common centre without the combinatorial
blow-up, and every reported interaction is relative to the shipped default, which is the comparison a
reader actually wants. The two reference floors, native and null-dispatch, are measured at the centre and
at each shape so the attribution decomposition is available everywhere it is needed.

The correctness contract binds the whole matrix: every cell runs the identical `generate()` program for a
given shape and seed, differing only in how that program sits in memory and how dispatch maps opcode to
operation, and every cell folds the identical post-pass checksum. The harness asserts that all cells
agree byte-exact at each `(shape, seed)` before any timing is trusted. A divergence fails the bench
rather than reporting a faster wrong answer. This is what makes "the same program, slotted into each
composition" a proven property rather than a claim.

## The champions: oracle envelope and validated cost-model selector

Two champions, and the relationship between them is the headline result.

The oracle champion is the lower envelope of the matrix: at each `(n, k, shape, regime)` point, the
fastest cell that was actually measured. It is not a runtime strategy, it is the theoretical best the
composition space contains, and it is the target every real selector is judged against.

The cost-model champion is a real, measured variant that a runtime could ship. At load time a runtime
knows the program's node count and can be told or can estimate the expected evaluation count; it does not
know `N` at compile time, so the selection must be a runtime branch on those values. The selector holds a
small calibration table of `(S, I)` per form and dispatch, computes the predicted `total(k)` for the
candidate compositions at the actual `(node_count, expected_k)`, picks the predicted cheapest, and pays
the branch and the table lookup for doing so. Measure it as a cell. The finding is how closely the
model-driven pick tracks the oracle envelope: if it stays within the noise floor of the best fixed cell
at every point, the cost model is validated as predictive and the selector is a shippable adaptive
interpreter; if it diverges, the divergence localises exactly where the simple `S + k * I` model fails
and what second-order effect a real runtime selector would need to account for. Either outcome is a
genuine result, and both are stronger than a hardcoded size threshold that only restates a crossover the
matrix already drew.

## What this lets us conclude that the point bench cannot

The rearchitected bench answers, for any composition and any pair of composptions, questions the single
point cannot express. The break-even evaluation count above which predecode pays, as a number rather than
a baked constant. The program size at which the working-set crossover flips each dispatch ranking, per
form, because the form axis moves the crossover. Whether every dispatch ranking inverts between the warm
per-frame regime and the cold many-program regime, which is the difference between the two real runtime
usage patterns. How much of each measured difference is dispatch, how much is interpretation structure,
and how much is irreducible compute, via the two reference floors. Whether a shippable cost-model-driven
selector can actually track the theoretical best. And whether any of these conclusions is a property of
the composition or an artefact of one program shape, because the shape basis brackets the space. These
are decision-grade answers about the runtime's interpreter tier; the point bench produces a ranking at
one hidden operating point and leaves the generalisation to prose.

## Implementation plan

The carrier changes come first because the fidelity fixes touch every interpreter. Refactor every
`interpret_*` and `interpret_predecoded_*` to write results and return nothing self-checksummed, with a
single shared `checksum(results)` fold applied by the wrapper after the pass; this is mechanical and the
existing cross-validation tests move to comparing the post-pass checksum. Add the null-dispatch reference
interpreter for both the wire and the flat form. Add the `bittree` dispatch for both forms. Extend the
generator to expose the shape basis (topology and operand-locality parameters on top of the existing node
count and op vocabulary). Add a directly-timeable setup path that rebuilds the form without the one-time
cache. Add the cost-model selector and its calibration-table type.

The harness side needs a generator that emits the fractional-factorial cell set with the `k` sweep
encoded as a family of benches (one bench per `k`, the existing size axis carrying `n`), plus the
cold-many bench, and that records the cross-validation checksum per `(shape, seed)`. The run protocol is
the existing discipline held strictly: serial process runs, at least three per cell, report the minimum
for throughput cells and the median with spread for the regression points, do not rank differences inside
the noise floor, fit `S` and `I` with reported R^2, and carry the ns-to-cycles-to-IPC sanity line per
slope with an out-of-range IPC flagged as an artefact rather than a result. The committed CSVs and the
tracked `.bench_history` trail travel with every bench commit.

## Risks and honest caveats

Moving the checksum out of the hot loop shifts every absolute number the arc has recorded so far; this is
intended, the prior numbers were measuring dispatch-plus-hash, and the relative conclusions must be
re-established on the cleaner signal rather than assumed to carry. The full matrix is large even
fractioned; the mitigation is programmatic variant generation and batched parallel builds, but the runs
stay strictly serial because concurrent runs perturb the timing. The preserve-none and guaranteed-tail-call
features are incomplete on the pinned nightly and a threaded cell may fail to compile on some shape; if it
does, that cell is honestly recorded as blocked on the toolchain rather than worked around. The
CNTVCT quantum bounds small-program fidelity and is the reason for the timed-region duration floor;
below it the per-evaluation slope at the smallest sizes should be reported with a widened error bar rather
than a false precision. And the cost-model champion validates a first-order model, so a clean track
against the oracle is evidence the model is sufficient at this granularity, not proof no second-order
effect exists; the divergence points, if any, are where that claim is bounded.

## Answers to the four open questions, mapped

The iteration-count axis is not merely added, it is promoted to the central measurement: the `S + k * I`
decomposition is the whole reframe, and the break-even run count is computed analytically rather than
sampled at one `k`. The program-shape question is answered by a four-to-five point basis bracketing the
vocabulary, topology, and operand-locality dimensions, run as one-axis slices out from the canonical
centre so every shape is measured against a common reference without a full grid. The adaptive champion is
made honest by selecting at runtime on the node count and expected evaluation count from a calibration
table, measured as a real cell, and judged against the oracle envelope so the result is a validated
predictive model rather than a restated threshold. The branch strategies enter as the dispatch axis,
`switch` and `fntable` and `threaded` and `ifchain` and `bittree`, testing the match-lowering finding
inside the real interpreter; the separate synthetic branch benches remain their own workload because they
do not run this IR program, and their strategy question is what the dispatch axis carries here.
