# The carrier matrix through the interpreter/JIT-composition lens (Haoran Xu)

**Date:** 2026-07-23. **Scope:** the full run under `mock/benches/results/carrier_*/` and
`RUN_SUMMARY.md`, cross-checked against current `mock/benches/carrier/src/` and
`mock/benches/src/bin/gen_matrix.rs`, against my own two prior rounds
(`202607221930_carrier-machinery-review-haoran-xu.md`,
`202607230030_full-matrix-review/haoran_xu_review1.md`), and against the design oracle
(`mock/design_rounds/202607221700_topic.bench-composition-matrix-synthesis.md`). I re-derived
every claim below from source or from a script I ran and reverted, not from the prior reviews'
say-so, and I ran a probe against live `sinks()` output to get real numbers rather than reasoning
from the shape of the code alone.

Ten of my prior review's ten findings, and four of my second review's five, are confirmed fixed at
the source level, not just claimed fixed: I grepped for the actual macro invocations and read the
actual bodies rather than trusting the module-doc claims. That is the strong part of this round.
The weak part is new, and it is the largest single finding across all three of my rounds: the
design document that scoped this entire matrix states, as its own explicit "central measurement,"
a cost model of `total(k) = S + k*I` fit across a geometric k-sweep with a reported R^2, specifically
to settle the iteration-count question and to decompose every measured slope against two reference
floors. None of that machinery was built. What shipped is a single fixed-`k` warm-steady-state point
per cell, with the JIT tier's entire setup cost hidden in untimed prep. This is not a subtle ISA
artifact like my prior rounds' if-chain or vertical findings; it is the load-bearing methodology
the design called "not an added axis, the central measurement" (`202607221700...md:124`), absent
from the shipped harness in its entirety.

## Verdict in one line

Trustworthy for what it actually measures (warm-steady-state, fixed small-k execution cost, per
cell, cross-validated byte-exact); not decision-grade for the question vehje actually has to answer
about its native/JIT tier, because the design's own stated backbone (the S+k*I cost model, the
two-regime warmup-curve fit, the two-reference-floor slope decomposition) was never built, and the
near-native tier's entire codegen cost is silently excluded from every timed measurement it reports.

## What is composition-sound (specific, cite file:line)

The op-semantics single-source discipline now reaches every scalar dispatch family without
exception. `ops.rs:27-40`'s `binop_body!` is the one definition; my second review's Finding 4 (CFG
had three independent hand-transcriptions) is closed: `cfg.rs:113,116,119,122` and
`cfg_threaded.rs:96-99` both now call `crate::ops::binop_body!` directly, confirmed by grep, not by
the module doc's claim (`cfg.rs:71-76` for the switch cell's inline match arms too). Six other files
(`interp.rs`, `predecode.rs`, `stackbc.rs:93-110`, `vertical.rs`, `trace.rs`, `optimize.rs`) already
deferred to it per my prior round. The vector twin `binop_simd!` (`ops.rs:56-70`) is the SIMD
representation's single source, used exclusively by `vertical.rs:56-67`. Nine of the matrix's ten
independent representations (every scalar cell plus the vertical cell) now derive from exactly two
definitions instead of the seven-plus hand-transcriptions my first review found; the machine-code
cells (`copypatch.rs`, `stencil.rs`) are the honestly-irreducible third representation, each
independently cross-validated byte-exact against the interpreter.

The shared `access::rload`/`rstore` (`access.rs:22-32`) and the checksum-out-of-the-hot-loop
discipline (`access.rs:38-45`) are real and reach every dispatch shape I checked, including the
native ceiling: `native.rs:79-115`'s `native_madd` was the one place this had NOT landed in my
first review (it inline-folded a rolling hash twice per chain step); it is fixed, confirmed by
reading the loop body, which now writes `results[chain0 + 2*s] = mul` / `... = add` with no fold,
matching `interp::interpret` exactly, and `native_madd_over_input` (`native.rs:117-130`) folds one
`access::checksum` per pass exactly like `interp::run_over_input`. The module doc at
`native.rs:20-29` states the fix and the reason it mattered (this ceiling is the denominator every
profile's slope was meant to normalize against), and the fix is real, not just claimed.

The JIT-window panic (my second review's Finding 1, blocking) is resolved cleanly: `native_family()`
(`gen_matrix.rs:282-298`) now calls `spec_sized(..., vec![64, 256, 1024])` instead of the default
five-size sweep, capping the near-native tier below the imm12 window with margin, and the inline
comment (`gen_matrix.rs:292-296`) states the reason plainly rather than letting the run discover it
by crash. `native_ceiling_family()` (`gen_matrix.rs:343-362`) does the identical cap and carries an
explicit, correct caveat that its O(N^2) throughput shape is not comparable to any sibling family's
per-execution numbers (`gen_matrix.rs:351-360`), which RUN_SUMMARY's own caveats section repeats
verbatim (`RUN_SUMMARY.md:96-98`). This is the right resolution of my second review's Finding 3: not
a fix to make the cost basis match its siblings (which would have destroyed the throughput idiom
this cell is supposed to measure), but a clean, correctly-labeled disclosure.

The vertical/SIMD family's per-input divisor problem (my second review's Finding 2, the family
explicitly flagged as the likely headline result) has the strongest resolution of any finding across
both rounds: not a manual recomputation and not a harness-level weight field, but a redesign of the
scalar baseline itself so every cell processes exactly 8 inputs per call
(`gen_matrix.rs:301-339`'s doc comment states this explicitly). I verified this is not merely
input-count-matched but checksum-cost-matched too, which neither prior review checked: scalar's body
(`gen_matrix.rs:321`) calls `c::checksum(&r)` eight separate times, each an O(node_count) fold, for
a total of `8 * node_count` rotate-xor operations; `interpret_vertical_checksum_into::<8>`
(`vertical.rs:107-120`) folds `results.iter()` (node_count SoA slots) times 8 lanes each, also
exactly `8 * node_count` rotate-xor operations, just organized as an outer-node/inner-lane loop
instead of eight outer-input passes. Same total keep-alive work, same total checksum work, the only
real difference is the amortization of dispatch and decode across lanes, which is precisely the
mechanism under test. This is a genuinely fair comparison, confirmed at the operation-count level,
not just the input-count level the design doc's own framing stopped at.

`stencil.rs` is a real, second build of the actual copy-and-patch mechanism (my first review's
Finding 4), not a rename of the hand-encoder: `stencil.rs:1-36`'s module doc states the distinction
plainly and correctly (stencils assembled once via `global_asm!`, `stencil.rs:44-90` and beyond,
codegen is memcpy-plus-imm12-patch with no per-node instruction selection), and `copypatch.rs:1-28`
was relabeled honestly as the direct-codegen comparator, not the paper's mechanism. Both share one
calling convention (`stencil.rs:26-29`) specifically so their runtime numbers are comparable, and
`RUN_SUMMARY.md`'s finding 7 (both hit 2.6-4.3x over the interpreter, within ~1% of each other) is
exactly what I would predict once dispatch is fully eliminated from both: at that point the two
techniques converge on the compute floor, and the real cost-shape difference between them (codegen
time, not execution time) is not what this number measures. See Finding 1 below.

## Findings (numbered)

### 1. The near-native tier's entire codegen cost, the one thing that distinguishes copy-and-patch from hand instruction selection in the literature, is excluded from every timed measurement (BLOCKING for the design question, not for correctness)

**Problem.** `native_family()`'s JIT cells build `jit_prep` as `program_prep(p)` plus a results
allocation (`gen_matrix.rs:286`), then wire `JitCode::new(prog).expect("jit")` /
`StencilCode::new(prog).expect("jit")` INTO THAT SAME PREP STRING (`gen_matrix.rs:289-290`), not
into the `{body}` half of the template. The generator template (`gen_matrix.rs:44-64`) substitutes
`{prep}` textually before `timed_calibrated! { run { ... } }` and `{body}` inside the timed loop; a
`OnceLock`-style once-per-process pattern is standard for every family's prep, but here it means
codegen (whether the hand-instruction-selecting `emit()` pass or the stencil-copy-and-patch pass)
runs exactly once per process, entirely outside the timed region, and is never charged to any
reported number. Only `jit.run(seed, &mut r); acc ^= c::checksum(&r);` (`gen_matrix.rs:289-290`)
is timed. I confirmed this by tracing the template substitution directly, not by inference from the
doc comments.

**Why it distorts.** Xu and Kjolstad's copy-and-patch paper (PLDI 2021, cited by name at
`stencil.rs:3`) is titled "A Fast Compilation Technique," not a fast execution technique: its central
empirical claim is that copy-and-patch reaches near-native EXECUTION speed at a small FRACTION of a
real backend's COMPILATION cost, which is exactly why it is attractive for JIT tiers that cannot
amortize a full optimizing-compiler pass. This matrix's own source explicitly frames the cost-shape
difference between `copypatch.rs` and `stencil.rs` in codegen terms: `copypatch.rs:14-19` names
"this cell's per-node codegen does instruction selection, the stencil cell's does a copy plus a few
immediate patches. Naming them apart keeps each number honest about the mechanism it measures." That
sentence, read next to `gen_matrix.rs:289-290`, is not true of the number the matrix actually
reports: the reported ~1% delta between copypatch and stencil (RUN_SUMMARY finding 7) is an
execution-time-only comparison with codegen amortized to exactly zero in the measurement, which
means the ONE place these two techniques are expected to differ (compile time: instruction selection
is a real per-node decision procedure, stencil copy-and-patch is a fixed-size memcpy plus a handful
of imm12 field writes) is precisely the dimension this run is silent on. A reader taking "both hit
2.6 to 4.3x, within one percent of each other" at face value (RUN_SUMMARY.md:57-62: "The PoC proves
both paths are viable and neither dominates, so the copy-and-patch route (far cheaper to maintain
than per-op hand asm) is the sound default") is being told a maintenance-cost argument, which is
true and reasonable, dressed as though it were also a performance argument for copy-and-patch over
instruction selection, which this run does not establish, because it never measured the dimension
where copy-and-patch's real advantage lives.

This is not a corner case for vehje: the runtime's residual tiers include "native code... where the
runtime is env-plumbing around LLVM/cranelift output" (`vehje/.claude/CLAUDE.md`, "Two sides"), and
the decision between a template-copy JIT (copy-and-patch) and a real backend is fundamentally a
compile-time-versus-execution-time tradeoff for exactly the one-shot and warm-up-sensitive workloads
the design doc itself flags in the very next section it wrote (see below). Measuring only the warm
asymptote answers "which is faster once already compiled," which is not vehje's actual question for
a tier whose whole selling point is compiling fast.

**It is not a local oversight; it contradicts the design's own explicit backbone.** The synthesis
topic that scoped this matrix states, verbatim, as "Part 1: the measurement backbone (measure a
line, not a point)" (`202607221700...md:117`): "Model every composition's cost as `total(k) = S +
k * I`: a one-time setup cost `S` to get from wire bytes to a dispatch-ready form, a per-evaluation
cost `I` for one interpretation pass, and `k` evaluations... Measure `S` and `I` by sweeping `k` over
a geometric ladder (1, 2, 4, ... 128) and fitting by least squares with a reported R^2"
(`202607221700...md:119-124`), explicitly framed as "the answer to op's Q1 (iteration count): it is
not an added axis, it is the central measurement" (`202607221700...md:124`). The same document's
"Two regimes, unified as a warmup curve" section (`202607221700...md:164-176`) goes further: it
predicts the very failure this run exhibits ("The single-program-many-times loop measures the warm
steady state... which is exactly the regime that most favours the shapes the arc has promoted... The
dispatch ranking plausibly inverts between [warm and cold]"), and calls for "one warmup-curve fit
`total(k) = S + sum_{j<k} I(j)`... extracting the cold slope, the warm slope, AND the warmup length."
I grepped the entire `mock/benches/` tree (`carrier/`, `src/bin/gen_matrix.rs`) for any trace of a
k-sweep, a least-squares fit, an R^2, a cold/warm slope, or a warmup length: zero matches. What
shipped is a single fixed `ITERS = 16` (`gen_matrix.rs:46`) with `timed_calibrated!` auto-repeating
that WHOLE 16-iteration block by an integer multiplier until the CNTVCT floor clears
(confirmed by reading `timed_calibrated!`'s expansion in the pinned harness,
`~/.cargo/git/checkouts/mockspace-*/bench-core/src/lib.rs:524-529`, which is a single-repeat-count
calibration primitive, not a k-sweep regression). This is the single point the design's own Part 1
opened by warning against measuring.

The "two reference floors" attribution (`202607221700...md:132-146`, "Every measured slope then
decomposes into native-compute + interpretation-structure + dispatch, with dispatch isolated
exactly. This is what makes a 2% end-to-end difference legible... Without it the matrix numbers are
uninterpretable in absolute terms") is half-realized: the null-dispatch floor DOES appear as a
same-table reference row in every wire/predecode family (e.g. `RUN_SUMMARY.md:106`'s `nullfloor`
row), which is genuinely useful. The native ceiling does not: it is its own isolated single bench
(`carrier_native_ceiling`, `RUN_SUMMARY.md:665-672`), never used as a regressor to decompose any of
the other nine families' slopes into native-compute-plus-structure-plus-dispatch, the way the design
explicitly called for. Both halves of "measure a line, decompose against two floors" are, in
practice, "measure a point, compare against one floor."

**Concrete fix.** Two independent things, matching the two design sections that specified them.
First, for the near-native tier specifically: move `JitCode::new`/`StencilCode::new` (and, for
comparability, `optimize::optimize`'s eqsat pass and `stackbc::compile`, which my second review's
Finding 5 already flagged as uncached-but-untimed-adjacent) into a k-sweep that includes codegen at
k=1 and amortizes it across k=1..128, per the design's own `total(k) = S + k*I` model
(`202607221700...md:119-124`); report S (the codegen-only cost, isolable by subtracting the k=2
point's per-call cost from k=1's, or by timing `JitCode::new` directly) alongside I. Second, more
broadly: implement the geometric-ladder k-sweep plus least-squares fit for at least the families the
design names as candidates for the warmup-curve unification (the near-native tier and the optimize
stage are the two whose S term is genuinely large relative to I; the dispatch/predecode/layout
families' S term, one `Decoded::parse`, is cheap enough that a point measurement is probably fine
for those, but that judgment itself is exactly what the R^2-fitted line was supposed to make
legible rather than assumed). File: `gen_matrix.rs:282-298` (native_family, where S is hidden),
`202607221700...md:117-176` (Part 1 in full, unimplemented).

### 2. The residual-encoding family folds a full-array checksum for the register cell and a live-out-only checksum for the stack cell, an asymmetric secondary cost inside the timed region that the reported ratio never accounts for

**Problem.** `residual_family()`'s two cells (`gen_matrix.rs:236-247`) are meant to isolate one
axis, register/SSA versus stack-bytecode encoding, on the identical program. The register cell's
body is `c::interpret(&d, seed, &mut r); acc ^= c::checksum(&r);` (`gen_matrix.rs:238`), an
O(node_count) fold over EVERY node's result. The stack cell's body is
`c::stackbc::interpret_stack(sp, seed, &mut st, &mut lo); acc ^= c::access::checksum_at(&lo,
&sp.out_locals);` (`gen_matrix.rs:242`), a fold over only `sp.out_locals`, the program's live-out
sinks (`stackbc.rs:83`, computed via `optimize::sinks`), a strict, usually small, subset of
`node_count`. Both interpreters do write every node's value into their scratch array
(`stackbc.rs:122-161`'s `Bc::Store(slot)` fires once per compiled node, so `locals` is fully
populated, matching `results`), so the INTERPRETATION cost basis is fair. The CHECKSUM cost basis
is not: I probed live `sinks()` output at N=16384 across all six profiles (reverting the change
before finishing this review) and found sink fractions of 10.4% (madd), 11.7% (wideselect), 12.1%
(tight), 18.4% (real), 35.6% (scatter), 66.8% (leaf). For the madd profile specifically, that is a
~10x difference in checksum-fold operation count between the two cells, executed inside the timed
region for every one of the 16 outer iterations.

**Why it distorts.** `checksum`/`checksum_at` (`access.rs:38-58`) are one `rotate_left(7) ^ v` per
element, cheap but not free relative to the interpreters' own per-node cost (a handful of loads,
one compute, one store). At N=16384 the register cell pays a full 16384-element fold every pass
while the stack cell pays roughly 1705 (madd) to 5827 (scatter) elements; this is real, uncounted-for
work loaded onto exactly the cell the headline finding says wins. RUN_SUMMARY finding 4 ("Register
VM decisively beats stack VM... up to 1.91x") is directionally almost certainly still correct
(register genuinely has fewer, simpler per-node operations than stack's load-load-compute-store
bytecode sequence, per `stackbc.rs:92-161` versus a direct register read/write), but the reported
ratio is not a clean measurement of register-versus-stack dispatch cost alone: it is that cost, PLUS
an asymmetric checksum tax that happens to fall on the winning side. The direction of the
contamination is conservative here (it very likely understates, not manufactures, register's real
advantage), which is the only reason this is a methodology finding and not a "the headline is wrong"
finding, but it means the specific number ("up to 1.91x") should not be quoted as a clean measurement
of the residual-encoding axis in isolation, and a future family with the opposite asymmetry (a
cheaper checksum on the LOSING side) would not get the same benefit of the doubt.

**Concrete fix.** Give the register cell the same live-out-only fold: `checksum_at(&r,
&sinks(prog))` computed once in prep (the sink ids do not change across the timed passes for a
fixed program), so both cells fold over the identical index set and the only remaining difference
is genuinely the encoding's dispatch and memory-access cost. This is a small, mechanical change,
symmetric with how `optimize_family()` already does exactly this (`gen_matrix.rs:276`, `checksum_at`
over `out_ids` for every strategy). File: `gen_matrix.rs:236-247` (residual_family), `stackbc.rs:83`
(`out_locals` derivation, the pattern to mirror on the register side).

### 3. Sink-cardinality is now measured (my second review's Finding 8, resolved) but the concrete magnitude I found (a 6.4x spread, 10.4% to 66.8%, across profiles at N=16384) is larger than the test that "resolved" it demonstrates, and it directly explains why the checksum contamination in Finding 2 is profile-dependent

**Problem/status.** This is a confirmation-plus-quantification, not a new defect. `optimize.rs`'s
`sink_count_varies_across_profiles` test (present in current source; I did not need to re-add it)
asserts the counts differ at a fixed N=4096 without asserting how MUCH they differ. My probe at
N=16384 (script written, run, and reverted; not committed) found real: sinks vs profile were 3015
(real, 18.40%), 1705 (madd, 10.41%), 1976 (tight, 12.06%), 5827 (scatter, 35.57%), 1922 (wideselect,
11.73%), 10949 (leaf, 66.83%). The magnitude matters for two reasons the "counts genuinely differ"
test cannot surface: it is the exact mechanism behind Finding 2's checksum asymmetry (a family whose
checksum shape depends on sink count will show its worst comparability distortion on the profiles
with the smallest sink fraction, i.e. madd and wideselect, and its best on leaf, where 66.8% of
nodes are live-out and the two checksum shapes nearly converge), and it means any future reader
comparing `carrier_optimize_leaf`'s numbers against `carrier_optimize_madd`'s at face value (both
already correctly use `checksum_at`, so this specific pair is fair internally, per Finding 2's fix
recommendation) is comparing a stage whose output is two-thirds live nodes against one whose output
is one-tenth live nodes, a real structural difference in what "the downstream program" even is
across profiles that no summary table currently surfaces as a number.

**Concrete fix.** Report the six sink fractions (or at least the two extremes, madd's 10.4% and
leaf's 66.8%) directly in `RUN_SUMMARY.md`'s caveats section, not just as a passing test assertion,
since it is load-bearing for reading every cross-profile table in the matrix, not only the optimize
family. File: `optimize.rs` (`sinks`, `sink_count_varies_across_profiles`), `RUN_SUMMARY.md:90-98`
(the caveats section, the natural home for it).

## The fidelity + comparability ledger (per interpreter shape)

**Switch, fntable, natural if-chain, frequency-ordered if-chain, bit-tree (straight-line wire).**
Sound. Identical op semantics (`ops::binop_body!`), identical operand access
(`access::rload`/`rstore`), identical checksum shape. The natural if-chain and the frequency-ordered
if-chain both compile to the same jump-table lowering as switch (my first review's Finding 3,
disclosed correctly by the crate and by RUN_SUMMARY finding 1's framing); `ifchainlin`
(`interp.rs:273-296`, `black_box`-barriered) is the one cell that is a genuinely different mechanism
(a real linear compare cascade), correctly reported as the only real penalty. Every cell in this
group does identical total work per iteration.

**Preserve-none threaded (straight-line and CFG).** Sound and independently ISA-confirmed by me in
my first round (zero spills, real tail branches). Straight-line threading loses to switch
(RUN_SUMMARY finding 2, 1.12-1.17x), CFG threading wins decisively (0.45-0.79x), exactly where the
mechanism predicts (real control flow, indirect transfers). No comparability defect; this is the
strongest-audited cell in the matrix across all three review rounds.

**CFG switch/fntable/threaded/trace.** Sound. The eager-load asymmetry on SET (my first review's
Finding 2) is fixed and confirmed at the source level (`cfg.rs:106-121`'s new `CFn` signature); the
op-semantics single-source gap (my second review's Finding 4) is fixed (`cfg.rs:113-122`,
`cfg_threaded.rs:96-99`). All four cells run the identical `Block` structure to the identical
termination condition. No open concern.

**Register versus stack residual.** Interpretation cost basis is fair (both fill an equal-sized
scratch array, one entry per node). Checksum cost basis is NOT fair (Finding 2): register folds
the full array, stack folds only its live-out subset, a real, uncounted, direction-known
(conservative) contamination of the reported ratio.

**Record layout (REC12..REC32).** Sound; every layout runs the identical switch dispatch over the
identical program, the only variable is decode arithmetic. RUN_SUMMARY finding 3 (under 1% spread,
refuting a 24-byte-optimal hypothesis) is a clean null result precisely because this family has no
comparability defect to hide behind.

**Value representation (static/tagged/nanbox).** Sound within its own isolated mini-IR; each of the
three representations runs the identical `valrepr` program. Correctly scoped as its own island (not
integrated with the main IR, as my first review noted and this review re-confirms is still the case,
`gen_matrix.rs:225-233`).

**Optimize stage (none/cse/fold/dce/eqsat/cseeqsat/all).** Internally fair (uniform `checksum_at`
over each strategy's own live-outs, `gen_matrix.rs:276`), but cross-profile comparison needs the
sink-fraction context from Finding 3 to read correctly, and that context is not currently surfaced
where a reader would see it.

**Vertical/SoA SIMD.** Sound, and more rigorously so than either prior review established: input
count AND total checksum-fold operation count are matched between scalar and vertical cells
(verified above, both `8 * node_count` fold operations). This is the one family I would call
genuinely decision-grade as measured, modulo Finding 1's broader point that it, like every other
family, reports only the warm asymptote.

**Near-native tier (interp/copypatch/stencil).** Semantically sound (byte-exact cross-validation,
confirmed independently in both prior rounds). NOT sound as a claim about the copy-and-patch
technique's actual value proposition (Finding 1): the reported numbers answer "which executes
faster once compiled," never "which compiles faster for comparable execution speed," which is the
question the cited paper and vehje's own tiering design actually turn on.

**Native ceiling.** Sound and correctly disclosed as an O(N^2) throughput idiom, not comparable to
sibling per-execution numbers (my second review's Finding 3, resolved by disclosure rather than
reshaping, the right call). Its role as one of the design's stated "two reference floors" for
decomposing every other family's slope was never realized (Finding 1); it exists today as an
isolated single bench, not a regressor.

## Novel angles the prior reviewers missed

**The design's own stated central measurement (the S+k*I cost model and the warmup-curve unification
of cold/warm regimes) was never built, and this is a bigger gap than any single-cell fairness bug
either prior review found.** Both my earlier rounds, correctly, audited whether individual cells
measure their claimed technique fairly against their siblings. Neither round stepped back to check
whether the MATRIX AS A WHOLE measures the axis the design document that commissioned it says is
"the central measurement": the iteration-count axis, resolved via a k-sweep line fit rather than a
single point. I found zero trace of that machinery anywhere in the shipped harness. This reframes
every headline finding in RUN_SUMMARY, not just the near-native tier: "dispatch shape barely
matters" (finding 1), "threading wins only where control flow is real" (finding 2), "optimization
payoff is entirely program-dependent" (finding 6) are all warm-steady-state, fixed-small-k claims,
and the design document itself predicted the dispatch ranking "plausibly inverts" between the warm
and cold regimes (`202607221700...md:169`). None of the nine warm-regime findings have been
cross-checked against a cold-regime measurement, despite the design calling that comparison "the
central measurement," not an optional extra.

**A cost-model gap is a different kind of finding than an ISA-artifact gap, and it changes what
"decision-grade" means for this matrix.** My first two rounds found individual mechanisms that
compiled to something other than their label claimed (if-chain collapsing to a jump table, a
copy-and-patch cell that was really a hand-encoder). Those are falsifiable, fixable, one-cell-at-a-
time defects. The S+k*I gap is different in kind: it is not that any cell measures the wrong thing,
it is that the MATRIX measures only one point on a curve the design explicitly said mattered along
its whole length, for a question (vehje's tier-selection decision between an interpreter residual, a
bytecode residual, and native/JIT code) whose real answer likely depends on where a given consumer
sits on that curve (a long-running service versus a short compile-once-run-once script). No amount
of per-cell fairness auditing catches this, because every cell, individually, IS measuring itself
fairly against its siblings at the one k it was run at; the gap is in what k range was chosen to
report at all.

## Open questions for the synthesiser

**Is the S+k*I / warmup-curve gap (Finding 1) worth a second bench-generation pass before this run
is treated as decision-grade for vehje's tier choice, or is the warm-steady-state number good enough
for the near-term decision with the cold/compile-time question tracked as a named follow-up?**
Building the k-sweep is real, non-trivial work (a geometric ladder per family, a least-squares fit,
reported R^2, and for the JIT cells specifically moving codegen out of prep and into the sweep). The
counter-consideration: vehje's runtime is described as running a residual "per-frame, per-entity"
in steady state for at least some consumers (per the engine framing in the workspace's hilavitkutin
material), which is exactly the regime this run already measures well; if that is vehje's dominant
workload shape, the warm number may be the one that matters most in practice, and the cold/compile
number matters most for a different consumer shape (a one-shot compile-run, e.g. a build-time
macro-expansion pass) that may or may not be vehje's near-term priority. Both readings are
defensible; I do not have visibility into which consumer shape vehje is optimizing for first.

**Should the residual-family checksum asymmetry (Finding 2) be fixed before or after any re-run,** given
it is a small, mechanical, one-line-per-cell change (`checksum_at` on both sides) with a
conservative-direction bias that very likely does not overturn the headline finding, versus the
cost of invalidating and re-running one already-collected family's numbers. I lean toward "fix before
quoting the specific ratio anywhere durable" given how cheap the fix is, but the tradeoff between
"re-run one family now" and "note the caveat and fix on the next natural re-run" is not mine to
settle alone.

**Does the copy-and-patch-versus-instruction-selection compile-time question (the real crux Finding
1 surfaces) matter enough to vehje's actual near-term tier decision to justify building it now, or is
"both execute at parity, stencil-copy-and-patch is cheaper to maintain" (RUN_SUMMARY finding 7,
which is TRUE as a maintenance claim even without a compile-time measurement) sufficient grounds to
pick copy-and-patch and defer the compile-time characterization indefinitely?** The maintenance
argument alone may be enough to make the engineering call regardless of what a compile-time
measurement would show, in which case building the k=1 codegen-time sweep is effort spent confirming
a decision already made on other, still-valid grounds. Whether that is the right call, or whether
vehje's tier-selection logic needs an actual measured crossover point (per the design's own "flat
pays for itself after N evaluations" framing at `202607221700...md:114`, restated generally rather
than only for the predecode-versus-flat question it originally illustrated) before the copy-and-patch
tier is locked in as canonical, is a design-priority call, not a methodology call.
