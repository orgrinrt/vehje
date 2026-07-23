# The carrier matrix: senior synthesis + the way forward (Fabian Giesen)

**Date:** 2026-07-23. **Role:** senior synthesis over the three specialist reads
(`agner_fog_methodology.md`, `haoran_xu_composition.md`, `chris_fallin_codegen.md`).
**Method:** every load-bearing empirical claim below was re-verified against the primary
source, not taken from the experts' say-so. I read `gen_matrix.rs` in full, the carrier's
`access.rs` / `ir.rs` / `interp.rs` / `predecode.rs` / `vertical.rs` / `ops.rs` /
`stackbc.rs` / `gen.rs` and the copypatch/stencil module docs, the pinned bench-core
calibration path, the driver `main.rs`, the design topic's Part 1, and the raw CSVs for the
contested cells. I also re-ran the optimizer node-count probe myself (a throwaway example
against the live `optimize()` API, run and deleted): the `cseeqsat == eqsat` identity, the
eqsat marginal contribution inside `all`, and the sink fractions are all replicated
bit-for-bit below, plus one extension the probe surfaced that no expert reported.

## The one-paragraph judgement

The matrix is decision-grade for five axis choices and not yet decision-grade for the one
question it was commissioned to answer. Decision-grade now: register residual over stack
bytecode, record width is a density decision not a speed decision, fold+CSE+DCE always on,
eqsat cut as shipped, vertical eight-lane column evaluation as the highest-payoff execution
shape (honest multiplier 4.8x, not 6x). Not decision-grade: the tier-selection question
(when does predecode, the optimizer, or native codegen pay for itself), because the design's
own stated backbone, the `total(k) = S + k*I` line fit, was never built, and every setup
cost `S` in the matrix is hidden in untimed prep. On top of that, the single most important
microarchitectural fact in the data was misread by everyone including the run summary: the
N-sweep cliff between 1024 and 4096 is not a data-cache capacity effect (the generator's own
parameters rule that out; the real profile's operand window is 64 nodes, 512 bytes, and the
const pool is 32 entries), it is the branch predictor's capacity to memorize the per-node
dispatch sequence under the calibration re-warm. That reread flips which columns generalize:
the small-N "dispatch barely matters" ties are the memorization artifact, and the N=16384
saturated-predictor column is the closest thing the matrix has to an honest cold-dispatch
measurement. The essential missing things are cheap: measure the S terms directly, add one
cold-regime mode, run the PMU pass, and re-derive the dispatch tables against the null-floor
the harness already carries.

## Audit of the expert reads

### Agner Fog (microarchitecture + benchmark validity)

**Holds, verified at the source.** Finding 1 (the calibration re-warm makes the surrounding
workload irrelevant to `algo_ns`): confirmed. The six-stage workload wraps `algo_call()` in
`mock/benches/src/main.rs:17-30`, and the calibrated reps loop inside `entry`
(`bench-core/src/lib.rs:543-553`) re-runs the 16-iteration block until the 2048-tick floor
clears, so the workload's cache/predictor perturbation amortizes to nothing in the reported
number. Finding 2 (checksum co-timed): confirmed; the template's `{body}` includes
`acc ^= c::checksum(&r)` inside `timed_calibrated!` (`gen_matrix.rs:54-63,171`), so the
`00_context.md` claim that the checksum is "out of the timed region" is wrong as written.
The truth is narrower: it is out of the per-node dispatch loop (which is what the design
topic's Part 1 actually specified) but inside the timed region, and both the design topic
and `access.rs:20` itself say the intended correction is differencing against the
null-dispatch floor. The harness even wires `normalise_mode: "subtract"`
(`gen_matrix.rs:118`), just against the wrong baseline (`switch`, not `nullfloor`). So the
defect is real but it lives in the reporting layer, and the fix is post-processing of data
already collected, not a re-run. Finding 4 (vertical scalar baseline is the wire
interpreter, vert cells are predecoded): confirmed in source (`gen_matrix.rs:320` uses
`wire_prep` + `c::interpret(&d, ...)`; the vert cells run over cached `Predecoded`,
`gen_matrix.rs:311-336`) and in the numbers (vertical scalar real N=16384 is 2.603 ms per
input, matching the wire switch at 2.612 ms, not the predecoded switch at 2.102 ms).
Corrected vert8 multiplier 4.8x. Finding 5 (per-16-execution absolute values; integer
truncation of `(__end - __start) / __reps` at `bench-core/src/lib.rs:553`): confirmed.
Finding 6 (reps-variable output makes byte-level cross-validation unreliable): confirmed
from the template alone; each rep seeds `acc` from `output[0]` and writes the final `acc`
back, so the terminal output bytes are a function of the reps count, which is itself sized
from a timing probe. Finding 7 (dirty tree): confirmed in the meta.json.

**Thin.** The magnitude bookkeeping in Finding 2 treats the whole 0.22x null-floor as the
shared tax. The floor is memory traffic plus loop structure plus checksum together; the
checksum alone is a serial rotate-xor dependency chain, roughly two cycles per element,
which at N=16384 is on the order of 5 to 8 percent of the switch cell, not 22. The
ratio-compression direction is right; the "understated by ~1.3x" figure is an upper bound
on the compression, not the checksum's share.

**Confidently wrong: the mechanism of Finding 3.** The L1D-capacity story does not survive
the generator's own parameters, which the methodology read never opened. `gen.rs:88-96`
sets `const_count: 32` (a 256-byte pool) and `locality_window: 64` for the real profile
(operands are drawn from the 64 most recent nodes, a 512-byte back-window into the results
array). Every data structure except the sequential program stream and the sequential
results/checksum writes is L1-resident at every N, and sequential streams prefetch. Three
observations in the shipped data kill the cache story directly. First, madd and tight have
byte-identical footprints to real at every N (same REC24 stream, same results array) and
show no cliff at all: madd switch is 3.07 / 3.05 / 2.99 ns per node at N=1024 / 4096 /
16384, ruler-flat. Second, `nullfloor`, which performs the same record decode and the same
operand loads with no opcode branch (`interp.rs:413-432`), is flat across the cliff
(1.76 / 1.78 ns per node at 1024 / 4096). Third, the cliff magnitude across profiles
(real 14.7x step, scatter 13.6x, wideselect 11x, leaf 4.9x, tight 4.1x, madd 4.0x, i.e.
no cliff for the last two) is ordered exactly by op-stream entropy (uniform-17 /
iid-16 / 5-op select-heavy / 3-op leaf-heavy / correlated-7 / correlated-3), not by any
locality or footprint parameter. The mechanism consistent with all three is branch
predictor capacity: under the calibration re-warm, an N<=1024 program's entire per-node
dispatch target sequence fits the predictor's history capacity and gets memorized, so all
dispatch shapes look nearly free; between 1024 and 4096 the sequence stops fitting; by
16384 prediction has degraded to what the stream's intrinsic entropy supports. The ~10 ns
per node that real/scatter converge to at 16384 is consistent with roughly one indirect
mispredict per node on Firestorm plus the base work. The rank reshuffle at N=4096 that
Agner flagged as placement noise is then not noise at all: `bittree` (a tree of two-way
conditional branches, large direction-predictor capacity) systematically beats `switch`
(one indirect jump-table branch, small target-predictor capacity) by 2.4x exactly in the
transition regime, 12 of 12 raw rows, and even `ifchainlin` (all conditional compares)
edges the switch there. The 4096 column is not an artifact to discard; it is the one
column showing the dispatch-shape-by-predictor-capacity interaction, mid-transition.
Where this reread bites hardest is Agner's own open question 1: he asks whether vehje's
regime is the L1 columns or the L2 column. The real answer is that the small-N columns are
the memorized-predictor regime, which a deployment running many distinct residuals per
frame will never see (the predictor is shared and aliased across programs), so the
saturated N=16384 numbers, not the small-N ties, are the better oracle even for small
residuals. This is inference from timing shape, as his claims were; the PMU re-run
(BRANCH_MISPRED vs L1D miss counters) adjudicates between us mechanically, and I want it
run for that reason alone.

**Also missed.** A reproducible 12-of-12 anomaly in `vert4` at N=1024 on the two
far-operand profiles: 266 us mean on real (min 226, max 310) against 35.5 us at N=256 and
a clean 4x-per-4x-N scaling everywhere else; vert8 and scalar scale normally through the
same point. It does not change any decision (vert8 dominates vert4 everywhere regardless)
but it is exactly the kind of unexplained systematic the PMU pass should name before the
vertical numbers are quoted as clean. And the second asymmetry in the residual family,
below, which is his kind of finding and sat one file over.

### Haoran Xu (interpreter / JIT composition)

**Holds, verified at the source.** Finding 1's factual spine is fully confirmed: the design
topic's Part 1 states `total(k) = S + k*I` with a geometric k-ladder, least-squares fit,
reported R^2, and the two-floor decomposition, verbatim at
`202607221700_topic...md:117-146`, calls it "not an added axis, it is the central
measurement", and predicts the warm/cold ranking inversion at `:164-176`; none of that
machinery exists in the shipped tree (I grepped as he did; zero matches), and the JIT
constructors sit in untimed `{prep}` (`gen_matrix.rs:289-290`; one correction of detail:
prep is not once-per-process for the JIT cells, `JitCode::new` runs per harness call since
it is outside the `OnceLock`, but every run is untimed, so his substance stands). Finding 2
(residual checksum asymmetry, full-array fold on the register side vs live-out-only on the
stack side): confirmed at `gen_matrix.rs:238,242`. Finding 3 (sink fractions): replicated
exactly by my own probe at N=16384: real 18.4%, madd 10.4%, tight 12.1%, scatter 35.6%,
wideselect 11.7%, leaf 66.8%. His operation-count check on the vertical checksum (both
cells fold exactly `8 * node_count` rotate-xor steps) is also confirmed in source
(`gen_matrix.rs:321`, `vertical.rs:107-120`).

**Thin.** Two places. First, the remedy for Finding 1 is presented as "build the k-sweep as
designed", but the microarchitecture lens exposes a trap in the design as written: a
k-ladder inside one process is not cold at any rung. Predictor and cache state persist
across rungs, and `timed_calibrated!` re-warms within each rung anyway, so a least-squares
fit over an in-process ladder measures S plus a warm I and never sees the cold slope the
design's own two-regime section wants. The honest cold instrument needs process-per-point
or program-set cycling (see the bench-changes section); the design's Part 1 needs that
amendment before anyone builds it verbatim. Second, his open question 1 leans on "vehje's
dominant regime may be warm steady-state, so the warm number may be the one that matters".
With the predictor-capacity reread, warm-in-this-harness means memorized-predictor warm,
which even a hot per-frame residual only enjoys if that one residual dominates the frame.
A frame executing hundreds of distinct residuals aliases the predictor across them; the
shipped small-N warm numbers are optimistic even for the steady-state consumer.

**Missed, and it is his own finding class.** The residual family has a second asymmetry
sitting next to the checksum one: the register cell interprets the wire `Decoded` view
(zero-copy over REC24 bytes, per-operand wire arithmetic; `ir.rs:246-311`), while the stack
cell interprets a fully predecoded `Vec<Bc>` enum stream (`stackbc.rs:22-50`), the same
decoded-vs-wire imbalance he and Agner both caught in the vertical family, pointing the
other way. Both asymmetries burden the register side, so the direction of the headline
survives a fortiori, but the honest comparison is `interpret_predecoded` vs the stack
bytecode with `checksum_at(sinks)` on both sides, and the shipped predecode tables already
suggest what it shows: predecoded register at madd N=16384 is 766 us against the stack
cell's 1.497 ms, roughly 2x, larger than the reported 1.91x. The reported "1.91x" is a
composite of three axes (encoding, decode form, checksum scope), not a measurement of one.

**Adjudications assigned to me.** On checksum-in-timed-region: Agner and the context brief
conflict; the source adjudicates. The checksum is inside the timed region (Agner right, the
brief wrong) but out of the per-node loop by deliberate design, with the design-intended
correction being null-floor differencing that the summary then failed to apply. Verdict:
harness correct per design, reporting wrong per design; fix in post-processing. On
JIT-cost-hidden-in-prep, defect or scope choice: both, at different levels, and the split
matters. At cell level it is a documented scope choice (`optimize.rs:9-10` and the
copypatch module doc both say S is an honest setup term; Fallin verified the caching is
sound). At deliverable level it is a defect, because RUN_SUMMARY finding 7 converts an
execution-parity measurement into a recommendation ("copy-and-patch is the sound default")
whose actual justification, near-zero codegen S, is the one quantity this run never
measured, and because the design document that commissioned the matrix explicitly made S
half of the central measurement. Verdict: keep the cells, indict the summary sentence, and
measure S, which is cheap (the constructors are pure functions; timing them is a trivial
cell family).

### Chris Fallin (codegen + optimizer soundness)

**Holds, and I replicated the empirical core myself.** My independent probe (all six
profiles, N=4096 and N=16384) reproduces every number he reported and the pattern
everywhere he did not: `cseeqsat` and `eqsat` produce byte-identical node counts in all
twelve combinations (e.g. madd 22785 vs 22785 at 4096, 91659 vs 91659 at 16384); `fold`
alone and `dce` alone equal `none`'s node count everywhere (structural no-ops on this
generator, exactly for the reason he gives); `cse+fold+dce` equals `all` on madd, tight,
and leaf (eqsat's marginal contribution is zero) and beats `all` on real (14169 vs 14235
at 16384), scatter (15082 vs 15168), and wideselect (13960 vs 14521). His terminology
finding 5 is confirmed at `gen_matrix.rs:289-290` against the two module docs, which are
scrupulously correct while the bench-visible tags and the summary prose invert them. His
venue correction is right: copy-and-patch is Xu and Kjolstad, OOPSLA 2021 (Proc. ACM
Program. Lang. 5, OOPSLA), not PLDI; `copypatch.rs:12` and `stencil.rs:3` cite the wrong
venue. His identification of the eqsat sharing loss as the tree-vs-DAG extraction problem
(the egg paper, Willsey et al., POPL 2021, treats DAG-cost extraction as the separate,
harder problem) is the correct prior-art frame.

**Extended by my probe.** Two strengthenings he can now cite. First, eqsat standalone does
not merely fail to shrink; it inflates the residual 5.6x on madd at N=16384 (91659 nodes
from 16384), which is the whole mechanism of the 142x timing blowup: an unfolded,
rebuilt-and-duplicated program. Second, wideselect is the profile where eqsat-inside-`all`
does the most damage (+561 nodes, 4%), and the shipped timing table independently
corroborates it: `cse` beats `all` at N=16384 (1.574 ms vs 1.621 ms,
`RUN_SUMMARY.md:507-508`). The matrix's own data already contains a measured
eqsat-in-pipeline pessimization; nobody had connected the two tables.

**Thin.** His fix menu for the eqsat defect (correct the comment, or make CSE re-entrant /
DAG-aware) skips the cheap third shape that gets most of what AC-reassociation was for at
none of its cost: canonicalize commutative operand order in place (sort the two operands of
each commutative binop by node id, no flatten, no rebuild, no node creation) and let plain
structural CSE run over that. It is linear, allocation-free, cannot inflate the program,
and converts the class of trivially-commuted duplicates into CSE hits. The full AC story
(flatten-sort-rebuild or a saturating e-graph with DAG-aware extraction) stays a labelled
refinement gated on a consumer whose residuals actually carry long literal AC chains, which
none of the six profiles do (his finding 7).

**Missed.** Nothing he said is wrong. What his lens stops short of is connecting his own
S-cost finding to the tier decision: he asks for a pass-cost family; the decision-grade
artifact is the breakeven table that family enables (below).

## The synthesis: where the lenses intersect

Fusing the three reads plus the primary source produces five findings none of the lenses
could see alone.

**1. The N axis is secretly the warm/cold axis.** Haoran's biggest finding is that the
design's cold/warm regime split was never built. Agner's biggest finding is that the
reported numbers are warm-steady-state. The predictor-capacity reread joins them: because
the dispatch predictor memorizes small programs under re-warm and saturates on large ones,
the existing N sweep already walks from the fully-warm regime (N<=1024: predictor has the
whole program) to a proxy for the cold-dispatch regime (N=16384: prediction is down to
stream entropy, which is what a cold or aliased predictor gives you). The matrix
accidentally contains much of the cold-regime information the design asked for, encoded
along the wrong axis. Consequences per headline: "dispatch shape barely matters" (finding
1) is true only in the memorized regime; at N=16384 on high-entropy profiles the spread is
0.90x (fntable) to 1.25x (linear scan), a 35 percent swing, and the function table wins.
"Threading loses on straight-line" (finding 2) holds in both regimes, so it is stronger
than reported. The native tier's 2.6 to 4.3x (finding 7) is measured at N<=1024 against an
interpreter at its memorized best; in the saturated regime the interpreter's per-node cost
quadruples while native code (no dispatch branch at all) does not, so the honest cold /
large-program native margin is plausibly 8 to 12x, and the imm12 cap currently confines
the JIT cells to exactly the regime that flatters the interpreter. The vert8 win grows
with entropy for the same reason (real/scatter 0.17x, madd 0.37x): one dispatch mispredict
is amortized over eight lanes, so vectorization is, among other things, mispredict
amortization, which is the component of interpreter cost that survives every regime.

**2. Dispatch cost is multiplicative with prediction structure, not additive.** The
summary's framing (dispatch is a small additive slice over a large shared floor) is the
memorized-regime picture. The transition column shows shape-dependent factors of 2.4x
between dispatch strategies that tie elsewhere. For vehje this means the dispatch-shape
decision cannot be made from a single regime's column, and the right default (switch or
fntable, nothing exotic) happens to be right in both regimes, which is why the conclusion
survives even though the reasoning under it in the summary does not.

**3. Every defect found by any lens lives in the reporting layer or in one missing
measurement, not in the machinery.** The three lenses independently verified the harness
core (per-variant cdylib isolation, single-source op semantics, unchecked shared access,
byte-exact cross-validation where reps allow, calibration mathematics). The four
distortions (checksum ratio compression, vertical baseline asymmetry, residual double
asymmetry, JIT S exclusion) are all either post-processing fixes over data already on disk
or one cheap added cell family. That is an unusually good place to be after a first full
run, and it argues for a surgical follow-up, not a rebuild.

**4. The generator parameters, not the profiles' names, are the real experimental axes,
and op-stream entropy dominates operand locality for interpreters.** madd/tight vs
real/scatter differ in both entropy and locality, but the matrix plus `gen.rs` separates
them: locality is nearly free in this design (window 64 is 512 bytes; even scatter's
unbounded window only grazes L1 at the top size) while entropy moves per-node cost 4x.
For vehje: the tier a residual wants is predicted by its opcode-stream entropy (how mixed
its op sequence is), not by its size, and the generator should grow an explicit
entropy-times-locality grid so this stops being an inference.

**5. Vectorized interpretation vs JIT compilation is a solved-shape question in the
database world, and the matrix reproduces the known answer.** vert8 is precisely
vectorized expression evaluation in the MonetDB/X100 sense (Boncz, Zukowski, Nes, CIDR
2005): amortize interpretation overhead over a vector of values. Copy-and-patch is the
compile-per-query side. The direct comparison literature (Kersten, Leis, Kemper, Neumann,
Pavlo, Boncz, "Everything You Always Wanted to Know About Compiled and Vectorized Queries
But Were Afraid to Ask", PVLDB 11(13), 2018) found the two land within a small factor of
each other with complementary strengths: vectorization wins on data-parallel,
cache-resident evaluation; compilation wins on branchy, data-dependent per-item logic.
This matrix says the same thing in vehje's vocabulary: vert8 4.8x on straight-line DAG
evaluation over many records; native 2.6 to 4.3x per execution; trace-threading 2.2x on
real control flow. The prior art also settles the "which one is the tier" instinct: they
are not competitors, they are two tiers with different applicability predicates
(batchable-per-record vs hot-and-branchy), and the canonical design should encode both
predicates rather than rank the two mechanisms. Related prior art for the other findings,
so nothing here is claimed novel that is not: register-vs-stack dispatch cost was measured
by Shi, Casey, Ertl, Gregg ("Virtual Machine Showdown: Stack Versus Registers", VEE 2005),
same direction, same reason (fewer, simpler VM instructions beat implicit-operand density);
the dispatch/prediction story descends from Ertl and Gregg ("The Structure and Performance
of Efficient Interpreters", JILP 2003) as corrected on modern predictors by Rohou, Swamy,
Seznec ("Branch Prediction and the Performance of Interpreters: Don't Trust Folklore",
CGO 2015), whose headline (big ITTAGE-class predictors absorb interpreter dispatch, so
threaded dispatch stops mattering) is exactly the memorized-regime half of this matrix,
while the capacity cliff is the boundary of that result's validity that their small
benchmarks did not cross; and the warmup-curve measurement discipline the design's Part 1
wants is the subject of Barrett, Bolz-Tereick, Killick, Mount, Tratt ("Virtual Machine
Warmup Blows Hot and Cold", OOPSLA 2017), which should be cited by the harness when the
curve fit is built, since its core finding (warmup is often non-monotone and sometimes
never arrives) is a warning against assuming the two-slope model fits with a high R^2.

## What the matrix trustworthy establishes for the canonical design

These survive all three lenses plus my re-verification, with the corrected magnitudes.

| Result | Corrected reading | Confidence |
|---|---|---|
| Register residual beats stack bytecode | Direction certain; true gap is larger than the reported 1.91x because both shipped asymmetries burden the register side; predecoded-register vs stack is roughly 2x on madd/tight | High |
| Record width REC12..REC32 is a null result | Width is free while cache-resident; choose by density. Unproven only for DRAM-streaming cold loads | High, regime-bound |
| fold+CSE+DCE as the always-on pipeline | The entire `all` win is fold+DCE (with CSE modest); node counts prove it | High (replicated) |
| eqsat as shipped must not ship | Zero-or-negative marginal contribution in `all` on every profile; 5.6x standalone inflation; a measured timing pessimization on wideselect | High (replicated twice) |
| Dispatch: switch or fntable, nothing exotic | Ties in the memorized regime; fntable wins ~10% in the saturated regime; threading loses 1.12 to 1.17x on straight-line in both | High |
| Threading/trace pays only on real control flow | CFG trace 0.45x, threaded 0.79x; single kernel shape (5-block nested loop), so directional | Medium-high |
| vert8 column evaluation is the largest win | 4.8x against the fair predecoded scalar baseline (not 6x); mechanism (mispredict + dispatch amortization over lanes) is the most regime-stable in the matrix | High |
| Native tier ~2.6 to 4.3x warm floor | Real, byte-validated execution; measured against the interpreter's best case, so a floor on the true margin; copypatch/stencil execution parity is real and expected | High for the floor |
| nanbox/tagged beat static u64 at scale | One program shape, correctly hedged by the summary | Low-medium |

Two reporting corrections ride along: the two JIT variant tags must swap to match the
technique names (the cell tagged `copypatch` is direct instruction selection; the cell
tagged `stencil` is copy-and-patch), and the run must be re-taken from a clean commit
before any figure is cited durably (the meta records `d772801-dirty`).

## What it does NOT establish yet, and the bench changes needed to

**1. The S terms, and with them the tier breakeven table.** Nothing in the matrix times
`optimize()` (per strategy), `stackbc::compile`, `predecode`, `Decoded::parse`,
`JitCode::new`, or `StencilCode::new`. All are pure functions already isolated behind
clean call boundaries; a `carrier_setup_cost` family that runs each under
`timed_calibrated!` per profile and size is a few dozen generator lines and produces the
one artifact the tier decision actually needs: `k*(tier_a -> tier_b) = S_b - S_a over
I_a - I_b`, per profile. This is the design's Part 1 delivered as a table instead of a
regression, and it is the cheapest of all the changes. It also settles copypatch-vs-stencil
on the axis the paper actually claims (codegen cost), where the two should differ by an
order of magnitude.

**2. Cold execution, measured honestly.** The calibrated harness structurally cannot see
a cold pass, and an in-process k-ladder does not fix that (state persists across rungs).
Two instruments work. The surgical one: program-set cycling. Generate M distinct programs
(same profile, different seeds), cycle through them round-robin inside the timed region,
report per-execution cost as M grows through the predictor's capacity. The aggregate still
clears the 2048-tick floor (calibration stays valid because the floor applies to the
aggregate), but no single program's dispatch sequence can be memorized, which is precisely
the many-residuals-per-frame deployment shape. The blunt one: a first-rep-only column
(`algo_ns_first` next to the calibrated median; the probe pass is already timed
separately, so this is nearly free in the harness) for true first-touch latency, accepting
quantization noise at small N. Do both; they answer different halves (aliased-warm vs
first-touch).

**3. The PMU pass, which is now load-bearing rather than nice-to-have.** Two named
mechanism disputes ride on it: L1D-capacity vs predictor-capacity for the 4096 cliff
(BRANCH_MISPRED_RETIRED vs L1D_CACHE_MISS across the N sweep decides it in one run), and
the unexplained vert4 N=1024 anomaly. Until it runs, every "at the metal" sentence in all
four review documents, mine included, is inference from timing shape.

**4. Reporting fixes over existing data, no re-run required.** Re-derive the dispatch and
predecode tables as `(variant - nullfloor)` per the design's own two-floor plan (the data
is on disk; the subtract mode is even wired, pointed at the wrong baseline). Re-baseline
the vertical family against the predecoded scalar (the predecode family's switch column is
the right denominator and already exists; quote 4.8x). Flag the residual family's number
as a three-axis composite pending the mechanical fix (both cells `checksum_at(sinks)`,
register cell over `interpret_predecoded`), which is a one-line-per-cell change and one
family re-run. Swap the JIT tags, fix the OOPSLA venue, rewrite summary findings 6 and 7
(finding 6's "run the whole pipeline" becomes "run fold+CSE+DCE; eqsat is evidence-against
as shipped"; finding 7's conclusion becomes a maintenance argument explicitly, plus the
S-cost family when it lands).

**5. Fidelity plumbing.** Make the FFI output reps-invariant (write the first rep's `acc`,
or fold reps idempotently) so `MAY_DIFFER = false` byte comparison means something again;
pin one 16-byte seed table shared across sizes so cross-size comparisons stop depending on
per-size input bytes (today `input[k % N]` with k < 16 always reads the first 16 bytes,
but those bytes differ per size).

**6. One axis the generator should grow.** An explicit entropy-by-locality grid
(op_correlation x locality_window as first-class swept axes, a 3x3 grid would do) so the
"entropy dominates locality" inference in synthesis finding 4 becomes a measured surface,
and so a consumer can locate its residuals on that surface and read the tier prediction
off it.

## Open questions, each with >=3 costed options

### Q1: How should the cold regime be measured?

Option A, the design's Part 1 verbatim: geometric k-ladder with least-squares fit and
R^2, one process per family. Effort: high (new harness mode, fit machinery, per-rung
plumbing). Risk: medium-high; in-process rungs are not cold (state persists), so built
verbatim it produces a beautiful fit of the wrong curve; needs the process-per-rung
amendment, multiplying subprocess count by the ladder length. Payoff: the full instrument,
S and I and warmup length per composition. Forecloses: nothing, but its cost invites
deferral of the whole cold question.

Option B, program-set cycling: M distinct same-profile programs round-robin in the timed
region, sweep M. Effort: low-medium (one new prep shape plus a loop change; calibration
machinery untouched). Risk: low; the aggregate-floor trick keeps timer validity, and the
measurement matches the actual deployment shape (many residuals, aliased predictor).
Payoff: the aliased-warm regime per dispatch shape, which is the number the runtime
schedule actually experiences. Forecloses: does not measure first-touch S amortization
(that is Q1-orthogonal; the S family covers it).

Option C, first-rep column: report the probe pass (`algo_ns_first`) alongside the
calibrated median. Effort: trivial (the probe is already timed; plumb it to a CSV column).
Risk: quantization noise at small N (the exact problem calibration solved), so
small-program first-touch numbers stay soft. Payoff: true cold first-touch for N >= 1024
essentially free. Forecloses: nothing.

Option D, ship warm-only and relabel every claim as warm-throughput. Effort: zero. Risk:
the tier decision gets made on the regime that most flatters interpreters; the design
document itself predicted the ranking inversion. Payoff: none beyond honesty. Forecloses:
the central measurement, again.

Ranking: B, then C (do both; together they cost a fraction of A), then A (build later if
tier-selection thresholds become a product feature the runtime must compute per consumer),
D last. Recommendation: B + C in the next generator pass. The choice is op's.

### Q2: What happens to eqsat in the canonical optimizer?

Option A, delete it: ship fold+CSE+DCE, remove the AC pass from `all`. Effort: trivial.
Risk: low on the evidence (zero-or-negative marginal contribution everywhere measured);
residual risk is that real consumer programs carry long literal AC chains the six profiles
do not. Payoff: a strictly smaller, faster, evidence-backed pipeline; S shrinks too (the
eqsat pass is plausibly the most expensive stage, per Fallin's ledger). Forecloses:
nothing; the pass stays in the tree as a non-default.

Option B, replace with commutative operand canonicalization: sort each commutative binop's
operand pair by node id in place, then plain CSE. Effort: low (tens of lines plus tests).
Risk: low; it cannot inflate the program and is trivially value-preserving for the same
wrapping-integer reason. Payoff: captures trivially-commuted duplicates (the cheap majority
of what AC-reassociation catches on DAG-shaped code) with none of the rebuild explosion.
Forecloses: nothing; it is a strict subset of the AC ambition.

Option C, fix the real problem: make extraction DAG-aware (or iterate flatten/hash-cons to
a fixpoint) and re-bench. Effort: high; DAG-cost extraction is the genuinely hard half of
the e-graph literature (the egg authors treat it as a separate problem). Risk: medium; even
done right, the six profiles offer it nothing to win on, so the re-bench likely still shows
zero until a consumer workload with real AC chains exists. Payoff: the only path on which
"eqsat pays" can ever become true in-pipeline. Forecloses: a lot of effort if the consumer
workloads never materialize the chains.

Ranking: A now, B in the same round (they compose), C parked behind a named trigger ("a
consumer residual corpus shows AC chains longer than the fusion window"). Recommendation:
A + B. The choice is op's.

### Q3: What must the native tier measure before copy-and-patch is locked as canonical?

Option A, the S-cost family: time `JitCode::new`, `StencilCode::new`, `optimize()`,
`stackbc::compile`, `predecode` directly and publish the breakeven table. Effort: low.
Risk: none worth naming; pure functions, existing harness. Payoff: converts finding 7 from
a maintenance argument dressed as a performance result into the actual tiering curve, and
measures copy-and-patch on the axis the technique was invented for. Forecloses: nothing.

Option B, lock copy-and-patch on the maintenance argument alone and defer S indefinitely.
Effort: zero. Risk: medium; the maintenance argument is true and probably sufficient, but
the tier-selection logic (when to JIT at all) still cannot be written without S, so the
deferral just moves the same work later while the summary's inverted terminology hardens
in the record. Payoff: nothing measurable. Forecloses: the compile-time crossover number
until someone circles back.

Option C, lift the imm12 cap (register-materialized base addressing, or adrp/add pairs)
so the JIT cells run at N=4096 and 16384. Effort: medium (encoder work in two files plus
revalidation). Risk: low-medium (more hand-written encodings to get right, but the
byte-exact cross-checks catch errors). Payoff: measures the native margin in the
saturated-predictor regime, where the interpreter is 4x worse per node and the true native
advantage (plausibly 8 to 12x) lives; this is the single number most likely to change how
aggressive the native tier's rollout should be. Forecloses: nothing.

Ranking: A immediately, C next (it is the interesting one), B rejected as the status quo
wearing a decision's clothes. Recommendation: A + C, in that order. The choice is op's.

### Q4: What shape does vehje's canonical tier ladder take, given all of the above?

Option A, the three tiers as currently framed (flat IR arena interpreter, optimized
bytecode, native code), unchanged. Effort: zero now. Risk: medium; "optimized bytecode" as
a distinct middle encoding is exactly what the residual family refuted (the stack bytecode
lost to the register form on every profile), so the frame invites building the wrong
middle tier. Payoff: no churn in the ABI vocabulary. Forecloses: nothing technically, but
it leaves the middle tier's definition misleading.

Option B, redefine the middle tier as "predecoded register form + fold/CSE/DCE", no new
encoding. The wire arena stays the interchange and cold-start form; tier 1 is the same
program predecoded flat (16-byte PNode records) with the always-on optimizer applied;
tier 2 is native. Effort: low (it is a specification change; the measured cells for it
already exist and won). Risk: low; every measured axis backs it (predecode 0.8x, register
over stack, fold+CSE+DCE). Payoff: the middle tier becomes the thing the data says wins,
and the stack-bytecode branch of the design space is closed with evidence attached.
Forecloses: a stack-encoded middle tier (correctly, on this evidence).

Option C, option B plus a batched entry point reserved in the C ABI from day one: the
runtime contract carries `execute(residual, inputs[W], outputs[W])` alongside the scalar
entry, so the vert8-shaped evaluation (the largest single win in the matrix, and the one
that maps directly onto per-record column evaluation) is reachable without an ABI break
when the vectorized evaluator lands. The tier tag stays orthogonal to the entry arity.
Effort: low at the contract layer now (one function shape and a capability bit), the
evaluator itself is later work. Risk: low; the contract cost of carrying an unimplemented
batched entry is one honest FIXME plus a red conformance test, which is the correct
resting state for a designed-but-unbuilt mechanism. Payoff: the 4.8x is a schema decision
today instead of a breaking change later; batch-of-one degrades to the scalar path.
Forecloses: nothing; scalar-only consumers ignore it.

Ranking: C, then B (C contains B), then A. Recommendation: C. This is the one decision I
would not defer, because it is the only one that is expensive to retrofit: everything else
in this list is a bench change or a pass toggle, but the entry-point arity is ABI. The
choice is op's.

### Q5: Which record width does the canonical wire layout pick?

Option A, REC16 (4-byte header, three inline operands, no padding, four records per cache
line, no spill for ternary SELECT). Effort: none (measured, within noise of all others).
Risk: header carries no shape_id; if a future record needs per-node metadata the header
grows and the choice revisits. Payoff: densest no-spill layout; 33% smaller than REC24,
which is pure win for cold-load footprint and DRAM streaming, the one regime the null
result does not cover (smaller can only help there). Forecloses: per-node u32 metadata
without a layout bump.

Option B, REC12 (densest, ternary ops spill to the pool). Effort: none. Risk: the spill
path puts a second dependent load on SELECT-heavy programs precisely in the profiles
(wideselect) that already stress prediction; the matrix shows it costs nothing warm, but
the cold/streaming regime is unmeasured and the spill indirection is the kind of thing
that only bites there. Payoff: 25% denser than REC16 on binop-dominated residuals.
Forecloses: cheap ternary ops.

Option C, REC24 (status quo default in the cells). Effort: none. Risk: none. Payoff: 8
bytes of header room for future metadata. Cost: 50% larger than REC16 for padding and a
shape_id nothing reads; the "24-byte-optimal" hypothesis this width descended from is now
refuted by the matrix. Forecloses: nothing.

Ranking: A, then C (if the shape_id/metadata slot has a named future consumer), then B.
Recommendation: REC16, with the choice recorded as density-motivated (the matrix
establishes speed-indifference in cache, and density is strictly favorable in the
unmeasured streaming regime). The choice is op's.

## The bottom line for op

Take home. The machinery is sound and the run is a success wearing a flawed report: of the
nine headline findings, seven survive with corrected magnitudes and two need rewriting
(finding 6's eqsat framing is contradicted by the run's own node counts and timing;
finding 7's copy-and-patch recommendation is a maintenance argument presented as a
measured one, under inverted terminology). The decision-grade results to bank now:
register residual, fold+CSE+DCE always on, eqsat out (as shipped), switch-or-fntable
dispatch with threading reserved for real control flow, REC16-grade density freedom, and
the batched/vectorized column evaluation as the single largest lever at an honest 4.8x.

Fix. In the report, from existing data: dispatch tables re-derived against `nullfloor`;
vertical quoted against the predecoded scalar; residual flagged as a three-axis composite;
JIT tags and prose un-inverted; OOPSLA venue; clean-commit re-run for citability. In the
cells, mechanical: `checksum_at(sinks)` both residual cells, register cell predecoded;
reps-invariant output; shared seed table.

Add. The S-cost family (the missing half of the central measurement, and the cheapest
item on this page); program-set cycling plus a first-rep column for the cold regime; the
PMU re-run, which is now load-bearing because it adjudicates the named mechanism dispute
(predictor capacity vs L1D capacity) that determines which columns generalize; imm12 lift
so the native tier is measured where the interpreter is honest; the entropy-by-locality
generator grid.

Remove. eqsat from the default pipeline (keep the code, park it behind the named
trigger); the stack-bytecode middle-tier framing from the canonical design's vocabulary;
the "checksum out of the timed region" sentence from the context brief, which is how
reporting drift starts.

The one contract decision worth making immediately, because it is ABI and everything else
is not: reserve the batched entry point in the runtime C ABI now. The rest of this
document is bench work; that line is the design.
