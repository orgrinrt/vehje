# The vehje bench evidence: senior synthesis and the way forward (Fabian Giesen)

Method note before anything else: I did not take the three specialist reads on trust. I re-read the topic, the
bench sources, the findings files, `bench.toml`, `PORT_NOTES.md`, `BRANCH_STRATEGY_MAP.md`, the harness's
`perf.rs`, and the committed CSVs, and where a claim was cheap to test I tested it. Two of the checks below are
new experiments run for this audit (the struct-size compile check and the opaque-program ceiling re-run), and one
of them overturns the topic's single most load-bearing number. Everything I assert about the sources is either
quoted from a specific file and line or reproduced by a run whose output is stated inline.

## The one-paragraph judgement (does it hold water, and the single most important correction)

The evidence base splits cleanly into two classes, and the topic wears the rigor of the better class as if it
covered both. The in-harness work (the branch/multiway/archetype suites, `branch_tier`, the `hx_*` ports) is
genuinely good: per-variant cdylib isolation, byte-exact cross-validation, committed data (291 result CSVs, none
empty), and its flagship result, the tier-dependent branch-strategy reversal, is the best artifact in the corpus
and can land in the design text as written. The standalone overnight Zig one-shots, which happen to back most of
the decision-changing headlines, are a different animal: empty or header-only committed CSVs, hand-rolled timing,
two results that are arithmetic models reported as measurements, one warm-load "cache" that is a no-op branch,
and one comparison whose baseline is a strawman. The single most important correction: the "native is only 1.0x
to 1.2x over a good interpreter" ceiling, which the topic elevates to a headline that resizes the entire native
tier, is a measurement artifact. The interpreted program in `ceiling.zig` is a two-instruction comptime-visible
constant, so LLVM partially evaluates the interpreter over it (the first Futamura projection, performed by the
optimizer), and the bench then compares native code against native code. I re-ran it with the program laundered
through an opaque pointer, which is the condition the real runtime always operates under, and the ratio moved
from 1.2x to 10.7x on the same workload with cross-validation still passing. The design conclusion the number
was used for (interpreter floor first, copy-and-patch opt-in) survives on other legs, but the number itself must
not enter the design text. The topic, to its credit, asked for exactly this audit before hardening; the audit's
answer is: about half of it hardens as-is, and the half that does not is concentrated precisely in the findings
marked "these change a decision."

## Audit of the three expert reads (one subsection each: holds / thin / missed / where I checked the source and what I found)

### Frank McSherry (benchmark honesty)

**Holds.** All five of his specific claims verify against the source, and none of them is an exaggeration.
The `deltaSemi` strawman is exactly as described: `reach.zig:24` scans every one of the 16M edges every round
and only skips the OR body when `dirty[e[1]]` is false; no out-edge index is ever built, so the "delta" variant
does strictly more work per round than `wholeColumn` by construction. Worse than he states, actually: the comment
on line 19 says "index edges by child for delta propagation" and then no index follows, so the source comments
the algorithm it does not implement, and line 2 calls it "true semi-naive," which it is not. The throughput-unit
mix is real (line 36 computes edges times rounds per ms for whole, line 39 edges per ms for delta). The
`stack.zig:31` warm loop is verbatim `const changed = (i == 1234)`, a comparison against a literal; no input
hashing, no cache lookup, no content addressing. The warm number is the cost of one `compileMod` (about 2 us for
400 nodes) plus a free loop, so "warm = 1 recompile" is true by construction of the loop, not by measurement of
a cache. The empty CSVs verify byte-for-byte: `reach.csv` 0 bytes, `stack.csv` 0 bytes, `incr.csv` 48 bytes
(header only), and, beyond his list, `dispatch.csv` 36 bytes and `recwidth.csv` 42 bytes, both header-only, which
extends the non-reproducibility to the two headline BN benches he did not flag. `depdag.zig:57` is confirmed
frictionless arithmetic (`finite8_ms += ceil(width/8) * COMPILE_MS_PER_MOD`, a constant 0.05 ms per mod, no
threads, no barriers, no timing of any compile). And the honest 4.2x net cold-load figure does sit in the
interner-merge findings while the topic quotes 5.6x and 7.7x.

**Thin.** His praise of `incr.zig` as the strongest artifact is right but under-argued, and there is a detail
that actually strengthens it: the committed edit sets a fresh bit (`1 << 63`) that no ancestor has, so the
differential early-out never fires in the measured run and what is measured is the full ancestor path, the worst
case for the incremental side. The 20-visit number is conservative, which is the correct direction to err. Worth
saying because it distinguishes this bench's honesty from its neighbors'. His open question 3 (hold standalone
benches to harness discipline) under-weights that the harness's 16384-byte cap makes the 8M-scale runs
structurally impossible in-harness; the answer has to be a discipline for standalone benches, not a port. (The
carrier proposal below is that answer.)

**Missed.** Two things. First, the harness "confirmation" of his strawman finding is itself hollow: PORT_NOTES
lists `hx_reach` as harness-confirming "whole beats delta +31%," but the `hx_reach__delta` variant source is a
third algorithm entirely, a 64-node toy with a delta bitmask, one edge per node, and a fixed three rounds. It
neither reproduces the standalone strawman nor implements real semi-naive, so the "+31% harness-confirmed" line
in PORT_NOTES double-counts a comparison that has never been run fairly at any scale. Second, he treats the
harness tier as uniformly rigorous without noting that the topic's two headline interpreter numbers (BN1
dispatch, BN2 record width) never went through it at all; that observation is Wingo's, and the two reads need
joining (see synthesis).

**Where I checked.** `reach.zig` in full, `stack.zig` in full, `incr.zig` in full, `depdag.zig` in full, the
merge findings, the CSV byte counts, `results/` (291 CSVs, zero empty), and the `hx_reach__delta` variant source.
Every claim of his I checked held. His is the most accurate of the three reads.

### Andy Wingo (runtime reality)

**Holds.** The `v_tail.zig` handlers are `callconv(.c)` with no `preserve_none`, confirmed, and SK3's findings
confirm the deeper fact: Zig 0.16.0's `CallingConvention` union has no `preserve_none` at all, so the Deegen
configuration is not testable on the pinned toolchain, which makes BN1 a toolchain verdict and not a dispatch-shape
law, exactly as he frames it. The random op stream is confirmed (`common.zig:12`, `intRangeAtMost(u8, 0, 2)`),
three ops plus halt. The method overstatement is confirmed and is worse than a nuance: `bench.toml` carries 93
bench entries and none of them is interp-dispatch, record-width, cfg-interp, or value-arena; those are standalone
binaries with hand-rolled 9-pass-median wall-clock timing. And `perf.rs` is feature-gated (`perf-counters`),
default-off, with no such feature enabled in the vehje bench workspace's Cargo.toml, so every number in the whole
corpus is wall-clock and the topic's "hardware-counter timing" phrase is false for every number it covers. His
composite-interpreter point (the "good interpreter" is five mechanisms benched separately and asserted additive)
is correct and important, and the ceiling findings' own prose describes the benched interpreter as "switch
dispatch, NaN-box values, CFG-of-blocks" when `ceiling.zig` contains none of the latter two and uses 4-byte
instructions, not 24-byte nodes.

**Thin.** Two places. First, his record-width critique ("rec16 spills all three operands and loops") is accurate
but he stops one step short of the actual bug in that bench (below). Second, the op-correlation argument is the
weakest of his findings as stated: on a modern TAGE-class predictor with long global history, a single switch
branch predicts correlated streams far better than the 1990s Ertl-Gregg model assumes, so a correlated stream
helps the switch too, not only the threaded shape; the more load-bearing confounds are the missing
`preserve_none` and the three-op vocabulary. The bench's own 11-op variant already shows the gap narrowing
(switch 7.27 vs tail 8.01, 10%, versus 39% at three ops), which is the real signal that the verdict is
regime-bounded; he could have cited it.

**Missed, and where he is confidently wrong.** The big one. He banks "JIT value tracks interpreter quality"
(1.2x against a good register VM, 10x against a naive loop) as "the most honest result in the corpus." It is
not; it is a reconciliation story constructed to harmonize a real number with an artifact. He correctly smelled
that the 1.2x was measured against an idealized interpreter, but the truth is stronger than "idealized": the
interpreter in `ceiling.zig` is not being interpreted at all in the timed region. The two-instruction program is
a comptime-visible constant, `runInterp` is fully visible to LLVM, and nothing launders the code array, so the
optimizer constant-folds the dispatch, resolves the switch, resolves the `rk` constant-pool branches, and emits
straight-line native code. The bench then reports that native is 1.2x faster than native. I verified this
experimentally rather than by argument: rebuilt as committed (interp 0.81 ns/expr, native 0.66, ratio 1.2x,
reproducing the findings), then laundered only the code slice through an empty `asm volatile` with a memory
clobber and re-ran: interp 7.05 ns/expr, native unchanged at 0.66, ratio 10.7x, checksums still matching within
each run. The corrected interp cost (3.5 ns per op) is consistent with the corpus's own standalone dispatch bench
(4.58 ns/op), which is the cross-check that says the corrected number is the real one. The physics smell test
would have caught this without the experiment: 0.81 ns/expr is about 2.5 cycles for roughly fifteen instructions
of claimed dispatch work, an IPC no core delivers. Consequence for his read: his findings 4 and 5 were
directionally right but under-called; the honest statement is not "the ceiling is somewhat above 1.2x," it is
"the 1.2x row is not a measurement of an interpreter, and the compute-bound ceiling for scalar native over this
register VM is in the 5x to 10x band," with only the DRAM-bound 1.0x row surviving as real physics.

**Where I checked.** `v_tail.zig`, `common.zig`, `ceiling.zig` plus its findings, SK3's findings, `bench.toml` in
full, `perf.rs`, PORT_NOTES, and the two-variant ceiling experiment in the scratchpad (as-committed versus
opaque-program, both ReleaseFast, output quoted above).

### Ranjit Jhala (verification)

**Holds.** The load-verifier check verifies exactly: zero `assert` in `adversarial.zig`, six `std.debug.print`
lines with "(expect false)" comments, one human eyeball as the gate. For an artifact guarding the untrusted-input
security boundary, that is the right thing to flag and the fix is trivial. His strength ladder (proof, exhaustive
finite check, property test, single demonstration) is the correct taxonomy, and his placements check out where I
verified them: SK2's budget-fit rejection is compile-time-exhaustive over a decidable predicate and genuinely
earns "proof"; the dual-locus identity is by construction (one object, two linkers) with the hash as
defense-in-depth, which the sketch itself frames honestly; the u64 bounds-math argument is a width-counting proof
that the sketch under-sells as "confirmed by test." His finding that the N*W bound's positive half is one shape
at one size is accurate per the sketch's own description.

**Thin.** He under-differentiates by question type. For mechanism-feasibility questions (MAP_JIT, dual-locus
linking, the comptime SIGSEGV wall), a single demonstration is the appropriate and sufficient standard; a
mechanism that works once on the pinned toolchain works. The ladder matters for universally-quantified
correctness claims, which in this corpus are exactly four: the N*W bound, the thermometer-lattice identity, the
verifier's rejection completeness, and the tnum transfer functions (where soundness properly rests on the eBPF
literature, as he says). Grading everything on one ladder makes the table look weaker than it is. He also does
not cost his own fixes: the 3x3 exhaustive lattice check and the `test`-block conversion are each under an hour,
which changes the "is it worth doing" question into a non-question.

**Missed.** The WORKS table's deeper problem is not only strength-flattening, it is caveat-erasure: several rows
carry a load-bearing qualifier in the sketch ("alignment required for zero-copy," "u64 bounds math one width
above the index fields," the Handle semantics checked on one domain product) that the topic's table compresses
into the same green stamp as the unconditional rows. A reader planning from the table alone inherits obligations
without knowing it. Also, nothing in his read connects the certification-language audit to the measurement side,
where the same flattening occurs (model versus measurement versus proxy all cited in one register); that
connection is the panel's actual finding and none of the three could see it alone.

**Where I checked.** `adversarial.zig` (grep for asserts, the print lines), the SK2/SK3/dual-locus findings, the
sketch tree listing, and spot-reads of the graded-spine and effect-lattice artifacts.

## The synthesis: the cross-cutting picture (the patterns visible only across all three)

**One: the evidence quality is inversely correlated with decision weight.** Join McSherry's finding (the scaling
headliners are standalone one-shots with empty CSVs) with Wingo's (BN1/BN2 never went through the harness) and
the pattern completes: of the topic's five findings-that-change-a-decision, four rest on the weaker evidence
class. The 24-byte record: standalone, header-only CSV, and mislabeled struct sizes (below). Switch-over-tail:
standalone, header-only CSV, and the harness port that "confirms" it contains no tail variant at all, because
Rust cannot express guaranteed tail calls, so the load-bearing comparison exists in exactly one unreproducible
binary. The native ceiling: a standalone sketch, now shown to be an artifact, whose committed file does not even
contain two of the three regimes in its findings table. The comptime-recursion wall: fine (a feasibility fact, a
crash reproduces or it does not). Only the branch tier-reversal, the fifth decision-changer, sits in the strong
class, with isolation, cross-validation, and 291 committed CSVs. Meanwhile the confirmations that were never in
doubt (iter fusion, resolve, field access) enjoy the full harness treatment. The de-risking run spent its rigor
where the risk was not.

**Two: cross-validation validates semantics, not measurement conditions.** The corpus's proudest discipline,
byte-exact cross-validation between variants, caught nothing wrong in the two worst artifacts, and could not
have. LLVM's partial evaluation of the ceiling interpreter preserves semantics perfectly, so the checksums match
while the timed condition is fake. The warm-load no-op computes the correct cache state, so any output check
would pass while the claimed mechanism (hash, look up, hit) does not exist in the code. Semantic validation and
condition validation are different axes, and the corpus has only the first. The missing discipline is a
cost-model plausibility pass: before believing a number, compute what the machine would have to be doing for the
number to be true. 0.81 ns/expr of claimed interpreter work fails that arithmetic instantly (2.5 cycles for
about fifteen instructions), and 0.003 ms for a 2000-mod warm load equals one `compileMod` almost to the digit,
which is precisely the signature of a loop that does nothing. Both artifacts were detectable from the published
numbers alone, before reading a line of source.

**Three: the optimizer defeats were aimed only at the expected failure direction.** The authors knew LLVM was
the adversary; the topic's methodology section lists the defeats with pride (`#[inline(never)]` arms, black_box
on the memcpy, the opaque `madd`). Every one of those guards the native side against looking too good. Nobody
guarded the interpreter side against the same adversary, because "LLVM makes the interpreter look too good" was
not in anyone's threat model, and that is exactly where it struck. Selective paranoia: the checks you write
guard the failure modes you imagined. The same blind spot produced the record-width mislabels: an `extern struct`
was assumed to have the intended size, and nobody asked `@sizeOf`, because struct size was not where anyone
expected an error. The general lesson for the bench discipline is that defeats and assertions need to cover the
setup invariants (program opacity, actual stride, counter state), not just the output values.

**Four: the topic is a lossy compressor with a systematic bias toward the flattering end.** In several cases the
artifact layer is honest and the topic layer drops the qualifier. The merge sketch computes the Amdahl-correct
4.2x net cold win; the topic headlines 5.6x and 7.7x. BN1's findings carry a real regime caveat (heavy op
bodies could shift the crossover); the topic compresses to "demote tail-threading to a measured-and-rejected
alternative." The mod-stack findings admit the full-pipeline cold estimate is about 54 ms; the topic prints 4 ms.
The ceiling findings, in fairness, are themselves wrong rather than wrongly compressed, but the pattern over the
rest is consistent: where a findings file contains both a shiny number and its discount, the topic cites the
shiny one. Jhala's certification-language finding is this same phenomenon on the sketch side (WORKS as a uniform
stamp). Losing precision in summarization is normal; losing it in one direction every time is a bias, and a
reader six months out will read the topic, not the findings files.

**Five: what the corpus does right should be named, because it is the template.** The branch-strategy study is
how all of this should look: a real harness, per-variant cdylibs, cross-validated outputs, arms forced to stay
branches, a normalise baseline with a stated reason, committed CSVs with meta and findings, conclusions that
state their regime, and a decision-relevant output (the tier reversal) that no amount of literature-quoting
would have produced. The e2e capstone's compile-heavy profile (3.24 ms compile, 127 us run) is the single most
design-relevant honest number in the corpus, because it grounds the tier-priority argument independently of the
broken ceiling number. And the run produced genuine negative results recorded as such (cheap-lowering is a
node-count win, not a time win; the output port is within noise at harness sizes), which is exactly the honesty
the rest should be held to.

## The tiered verdict (solid and bankable / directionally right but inflated / artifact or unsupported)

**Tier A, bankable as stated.** The branch-strategy maps and the tier reversal (interp: never eval-all, one-arm
strategies tie; native: eval-all wins cheap-wide-unpredictable by 26%, loses elsewhere), including the
stencil-must-apply-the-map consequence. The copy-and-patch-loses-to-LLVM-at-scale result (1.47x at 4096, 2.58x
at 16384) and its diagnosis (no if-conversion, no vectorization). The naive-interp 9x to 12x row of
`branch_tier`. The comptime-recursion SIGSEGV wall and the iterative-kernel mandate (toolchain-pinned fact, and
the design was already iterative by intent). SK2's compile-time budget-fit rejection (a real exhaustive check).
Dual-locus bit-identity (by construction). Register VM over stack VM, NaN-box over tagged-8B (both modest,
well-trodden, correctly measured). CFG-of-blocks feasibility, Perceus exact-meet, iter-fusion mandatory,
flat shadow-stack resolve, interner hot path, MAP_JIT mechanism feasibility, the e2e compile-heavy profile, the
egraph conclusion in its honest form ("the cap is mandatory"; the 5.2x linearity is rule-set-specific and the
findings say so), the parallel-compile 5.64x as the real threaded number, the 4.2x net cold win, and the
blast-radius traversal statistics (real traversal, synthetic DAG, stated as such).

**Tier B, directionally right, magnitude or framing inflated.** The record-width decision: inlining the third
operand genuinely beats a pool spill (mechanism corroborated independently by the value-arena bench), but the
benched strides are 12, 20, and 32 bytes, not 16, 24, and 32 (compiler-verified: the `extern struct`s in
`rec16.zig` and `rec24.zig` are 12 and 20 bytes; the "// 16 bytes" and "// 24 bytes" comments are wrong), so
"24 is precisely optimal" cites two strides that were never measured; add the L1-hot regime dependence, the
unbenched fair 16-byte contender, and the Python-proxy corpus behind the 96.5% arity cut. Right call to make
today; wrong to call it precise. Switch dispatch: the right ship-now default on a toolchain that cannot express
the alternative fairly, with a gap that already narrows from 39% to 10% between three and eleven ops;
"measured-and-rejected" and "refutes Deegen" over-harden a verdict whose other arm was never runnable. The
incremental-scaling story: the mechanism argument is standard and sound, the blast-radius numbers are real, but
the 4 ms cold is a condensed model (the findings' own full-pipeline estimate is about 54 ms) and the honest
parallel figure is 5.64x measured with a 4.2x net, not 7.7x. Whole-column versus delta: whole-column at 50 ms
for 8M nodes is plausible and budget-relevant even at 2x error, but the comparative claim holds only against a
crippled delta, and the harness "confirmation" is a third toy algorithm; real semi-naive is unanswered. The
reach and egraph absolute throughputs: single-shot, hand-transcribed, empty CSVs; directional only.

**Tier C, artifact or unsupported; must not enter the design text as stated.** The 1.0x to 1.2x native ceiling
(LLVM partial-evaluation artifact; corrected compute-bound figure on the same bench is 10.7x, with only the
DRAM-bound 1.0x regime surviving; the committed file also cannot reproduce two of the findings' three rows).
The 0.003 ms / 1300x warm load (a no-op branch, not a cache; the number is one recompile by construction). The
7.7x "DAG parallelises better than independent" (frictionless arithmetic compared against a real run; as a
model ceiling it is fine, as a property it is unsupported). The "hardware-counter timing" claim (counters are
feature-gated, off, and not enabled; everything is wall-clock). The "interpreter benches ran through the
harness" claim (BN1/BN2/cfg/value-arena did not). And the uniform WORKS stamp insofar as it covers the
un-gated load verifier and the single-demonstration halves of the correctness sketches.

## What must be corrected, each with >=3 costed options (re-measure / re-frame / drop), ranked, with a recommendation

### C1. The native-tier ceiling (the 1.2x artifact)

1. **Re-measure properly.** Extend the existing `branch_tier` harness bench (it already has interp, native, and
   stencil variants and cross-validation) with a composite register-VM variant: opaque program, NaN-box values,
   CFG blocks, 24-byte records, i.e. the interpreter vehje will actually ship, against the same opaque-madd
   native kernel across the three regimes. Effort: about half a day, the harness scaffolding exists. Risk: low;
   the likely outcome is a compute-bound ceiling in the 3x to 8x band and a memory-bound floor near 1x.
   Forecloses nothing; produces the real number the tier-investment decision wants.
2. **Re-frame without re-measuring.** Strike the 1.2x everywhere; state the honest bracket from the artifacts
   that survive (DRAM-bound about 1.0x, naive-interp 9x to 12x from `branch_tier`, good-VM figure unmeasured),
   and rest the interpreter-first priority on the workload-profile argument (the capstone's 3.24 ms compile
   versus 127 us run), which carries it alone for the declared consumers. Effort: an hour of text. Risk: the
   design carries a wide bracket and someone re-litigates the tier order later. Forecloses precision in deciding
   how much copy-and-patch is worth.
3. **Drop the quantitative sizing.** Return the native-tier question to op's recenter (taste call: keep the
   endgame, do not privilege it) with no multiplier attached. Effort: minutes. Risk: the genuinely useful
   memory-bound insight is lost, and "1.2x" survives in institutional memory unless explicitly retracted.
4. **Cheap interim re-run.** Re-run only my opaque-program variant of `ceiling.zig` across its three regimes as
   a corrected standalone, commit the CSV, and schedule option 1 later. Effort: two hours. Risk: still a
   standalone one-shot, still not the composite VM.

Ranking: 1 > 4 > 2 > 3. Recommend 1. Whatever is chosen, the retraction of 1.2x is not optional; the number is
false and it currently headlines a design correction.

### C2. The record width (mislabeled strides, missing fair contender)

1. **Fix and rematch.** Correct the structs to true 16/24/32 (check with `@sizeOf`, which is a one-line
   assertion the bench should carry permanently), add the fair contenders (see the variant catalogue below,
   including the never-considered true-16-byte three-operand layout), and run at both L1-hot and larger-than-L1
   `res[]` working sets, standalone plus the `hx_recwidth` port. Effort: two to three hours. Risk: low; most
   likely a wide record still wins and the claim becomes true instead of approximately true. Forecloses nothing;
   and since wire format is day-one and expensive to break, this is the one place where re-measuring before
   locking is clearly worth it.
2. **Re-frame.** Keep the 24-byte decision, reword the evidence to what was measured ("inline-3 at 20-byte
   stride beats inline-2-plus-pool at 12-byte stride and ties inline-4 at 32; direction robust, exact strides
   mislabeled in the bench"), fix the source comments, drop "precisely optimal." Effort: under an hour. Risk:
   the wire format locks on a bench that never measured its stride; if the real consumer corpus (not the Python
   proxy) shifts the arity mix, the revisit is a format break.
3. **Drop the bench backing.** Choose 24 bytes on the arity census alone (96.5% at most three operands) and mark
   the performance claim as owed. Effort: minutes. Risk: discards a real, mechanistically corroborated
   measurement (pool indirection at 10.7 versus 4.4 ns/node in the value-arena bench) for no gain.

Ranking: 1 > 2 > 3. Recommend 1, and carry the SP2 proxy-corpus caveat into the design text in the same edit
regardless of option.

### C3. Dispatch (switch versus tail-threading)

1. **Re-frame.** State it as: switch is the shipped dispatch on Zig 0.16, where `preserve_none` does not exist,
   so the Deegen configuration is untestable on the pinned toolchain; the measured gap narrows from 39% to 10%
   as the op vocabulary grows from three to eleven; revisit triggers are a materially larger or heavier op
   vocabulary, or Zig gaining a register-preserving convention. Delete "measured-and-rejected" and "refutes
   Deegen." Effort: text only. Risk: none; the ship decision is unchanged and honest optionality is preserved.
2. **Re-measure the toolchain-independent question in C.** Clang has both `musttail` and `preserve_none`; build
   the same 25-op interpreter both ways in C as an oracle for what Zig would get if the convention lands.
   Effort: about a day. Risk: cross-toolchain transfer is arguable; it answers the "law" question, not the
   "ship" question, and the ship question is already answered.
3. **Re-measure within Zig at realistic vocabulary and correlation.** Sweep op count (3, 11, 25, 50) with
   correlated streams and heavier bodies, still without `preserve_none` because it does not exist. Effort: half
   a day. Risk: cannot answer the actual open question; likely re-confirms switch at small vocabularies and
   locates the crossover, which has some value for the revisit trigger.
4. **Drop tail-threading from the design text entirely.** Risk: erases the audit trail of why switch was chosen,
   which invites re-doing the work later.

Ranking: 1 > 3 > 2 > 4. Recommend 1 now; 3 is a fine follow-up if the op vocabulary grows past SP1's count.

### C4. The incremental-scaling cluster (warm no-op, cold model, 7.7x model)

1. **Re-frame immediately.** In the topic and anywhere D6 is justified: warm-load number replaced by "warm =
   one recompile plus an O(stack) change-detection cost this bench does not measure"; cold 4 ms labeled a
   condensed cost model with the findings' own 54 ms full-pipeline estimate stated; 7.7x relabeled a
   frictionless level-sync ceiling; 5.64x named the measured parallel figure and 4.2x the net with the interner
   tail; the 1300x deleted. Effort: an hour of text. Risk: none; every replacement number already exists in the
   artifacts.
2. **Re-measure the warm path honestly.** Implement the actual mechanism: content-hash all N mod inputs, look
   up a real cache, miss on the edited one, recompile it. Effort: about an hour. Risk: none; warm load will
   still be excellent (hashing 2000 small mods is sub-millisecond to a few ms) and the claim becomes a
   measurement of a cache instead of a branch. Forecloses nothing.
3. **Re-measure the DAG parallel claim with real threads.** A level-sync loader with barriers over the synthetic
   DAG, real `compileMod` calls, 8 threads. Effort: half a day. Risk: low; replaces both the model and the
   independent-mod extrapolation with one real number, likely between 4x and 6x net.
4. **Drop the quantitative D6 validation** and rest on "content-addressed caching is a standard mechanism with
   standard results." Risk: op adopted D6 specifically on the load-bloat concern; dropping the numbers re-opens
   a settled worry for no reason when options 1 to 3 are this cheap.

Ranking: 1 first and immediately, then 2 and 3 as follow-ups; 4 not recommended. Recommend 1 + 2 (both are
about an hour each).

### C5. Whole-column versus delta semi-naive

1. **Re-measure with a real semi-naive.** Build the CSR out-edge index and frontier worklist, run at the three
   scales, commit the CSV this time. Effort: one to two hours. Risk: none worth naming; on two-round shallow
   graphs whole-column may well still win, and then the design claim becomes true; on deeper graphs the answer
   changes and the design wants to know that too. This is the cheapest genuinely-open re-measure in the set.
2. **Re-frame.** "Whole-column chosen for the shallow-graph regime on simplicity; the delta comparison is
   inconclusive because the benched delta was not semi-naive (full edge scan per round)." Keep the 50 ms
   absolute as directional. Effort: minutes. Risk: leaves a core compile-stage mechanism choice resting on an
   unanswered comparison.
3. **Drop the comparative claim,** keep only the absolute budget number after a re-run that commits data.
   Effort: an hour. Risk: same as 2 plus loses the honest framing.

Ranking: 1 > 2 > 3. Recommend 1, and fix the throughput-unit mix and the lying "index edges by child" comment
in the same pass.

### C6. Verification strength labeling and the load-verifier gate

1. **Gate and grade.** Convert `adversarial.zig` to `test` blocks with asserts (the security boundary gets an
   executable gate, per the catalogue-edge-cases discipline); add the 3x3 exhaustive thermometer-lattice check;
   add a property test varying shape and W for the N*W bound's positive half; add a strength column
   (proof / exhaustive / property / demo) to the topic's table. Effort: two to four hours total. Risk: none.
   Forecloses nothing.
2. **Grade only.** Add the strength column and per-row caveats (alignment, u64-width, single-domain Handle
   check) without new tests. Effort: an hour. Risk: the untrusted-load verifier remains print-and-eyeball, which
   for that specific artifact is the one unacceptable residue.
3. **Full property-testing campaign** across all sketches. Effort: days. Risk: over-engineering
   pre-implementation sketches; the value is concentrated in the three or four universally-quantified claims,
   which option 1 already covers.

Ranking: 1 > 2 > 3. Recommend 1.

### C7. The methodology paragraph itself

1. **Correct and re-emit.** Fix the topic's method text (wall-clock timing, counters off; name per finding
   whether it is harness-isolated or standalone), and re-run the five standalone scaling benches so their CSVs
   actually contain the data the findings transcribe. Effort: a few hours. Risk: none. This also creates the
   class label (harness / standalone / model / proxy) that every future citation of these numbers should carry.
2. **Text-only correction.** Effort: minutes. Risk: the committed artifacts stay unreproducible, which the
   workspace's own bench discipline exists to prevent.
3. **Port everything to the harness.** Effort: days, and structurally impossible for the 8M-scale runs under
   the 16384-byte cap. Not the right shape; the right shape is the carrier-plus-scale-runner architecture below.

Ranking: 1 > 2 > 3. Recommend 1.

## The repair architecture: the carrier, the discipline, and the benches that were never run

The corrections above fix the existing artifacts. This section is the constructive half: how to rebuild so the
failure classes cannot recur, whether op's "harness-in-the-harness" instinct is the right shape, and the full
catalogue of variants and benches that should exist and do not.

### The carrier: one IR-and-interpreter loop as the shared workload (op's unification, evaluated)

Op's intent: extract a reusable thing from the archetype-showdown pattern so that everything is benched against
the same method, with a close-enough IR and interpreter in the loop. The intent is correct, and the evidence for
it is sitting in this audit: the corpus currently contains at least five independent hand-rolled toy
interpreters (`interp-dispatch/common.zig`, `record-width/common_rw.zig`, `ceiling.zig`'s RInstr loop, the
cfg-interp walker, and the showdown IR the `arch_*` suite generates), each with its own program generator, its
own timing loop, and its own bugs. That is un-extracted shared structure, and the two worst artifacts in the
corpus (the PE'd ceiling interpreter, the mislabeled record strides) are both bugs in bespoke copies of
machinery that should exist once. The archetype-showdown pattern is the proven copy: one shared IR program,
per-bench variation of a single axis, everything else pinned, normalised against a fixed baseline,
cross-validated. Generalise exactly that.

The vehicle I would build (op's sketch adjusted in two places): not literally a harness nested inside the
harness, but a **carrier library with two drivers**.

The carrier is one crate (working name `vehje-bench-carrier`) that owns four things. First, the IR definition,
which should be the real proposed wire format (the 24-byte record as actually specced, with `@sizeOf`/const
asserts pinning every stride), so the carrier is the first consumer of the wire format and nothing in it is
throwaway. Second, the program generators, seeded and deterministic, parameterised over the axes the corpus
kept improvising: op vocabulary size, op-mix and correlation model (i.i.d. random, Markov op-pair chains with
tunable entropy, and trace-shaped streams once real consumers exist), arity distribution (from SP2, swappable
when the census lands), operand-locality window, control-flow shape (straight-line, loops, call depth), and
working-set size. Third, the reference interpreter, written so that each contested axis is a variant point:
dispatch shape, value representation, record layout, block structure. Each bench variant is the same interpreter
with exactly one axis moved off the shipped default (switch + NaN-box + 24B + CFG). Fourth, the checksum
contract, so every variant cross-validates byte-exact as the harness already demands.

The two drivers. Driver one is the mockspace harness, exactly as the `arch_*` suite uses it today: each axis
setting compiled as its own cdylib variant, workloads within the 16384-byte cap, `[bench.*.normalise]` against
the default-configuration baseline, CSV plus meta plus findings promoted to `results/`. Driver two is a single
shared scale-runner binary (one `main`, written once, never copied) that links the same carrier and runs the
beyond-L1 and multi-million-node questions the cap excludes: multi-run with min/median/spread reported, CSV
emission mandatory (the bench refuses to conclude without writing its data), and the same checksum validation.
This resolves the McSherry-versus-cap tension honestly: the 8M-scale runs can never be harness benches, but
they can share every line of workload generation, validation, and reporting discipline with the ones that are.

Two properties make this architecture structurally immune to the failure classes this audit found, and they are
the real argument for it beyond deduplication. The first: **in the harness driver, the program crosses the FFI
boundary as input bytes**, generated by the driver and handed to the variant cdylib at run time. The variant's
LLVM cannot see the program, so the partial-evaluation artifact that broke the ceiling bench is impossible by
construction, not by discipline. (The `hx_*` ports already have this shape; `ceiling.zig` broke precisely
because it abandoned it.) The scale-runner preserves the same property by loading or generating programs behind
an opaque boundary. The second: because every bench varies one axis off one shared operating point, the numbers
compose by construction. Wingo's composite-additivity objection (five mechanisms measured in five incompatible
idealized loops, additivity asserted) dissolves: the composite is the carrier default, and every measurement is
a delta from it.

Two cautions on the same proposal, so it is adopted with eyes open. Normalised deltas are differences of two
noisy measurements; when a strategy delta is small against the carrier's per-node cost, the subtraction
amplifies relative noise. The `arch_*` suite already handles this correctly by declaring statistical dead heats
rather than fake rankings; the carrier must keep that, report the raw pair alongside the delta, and refuse to
rank below the noise floor. Where more signal is needed, add a magnified mode (raise the frequency of the
construct under test in the generated program) and label it as such: operating-point mode answers "does it
matter," magnified mode answers "which way does it point," and neither substitutes for the other. Second
caution: the language boundary. Dispatch-shape questions are Zig-specific (`@call(.always_tail)` has no Rust
equivalent, which is why the `hx_dispatch` port silently lost the tail variant), so a Rust-only carrier cannot
carry them. The clean resolution is to make the carrier's program format and checksum contract
language-neutral (it is the wire format; bytes in, checksum out) and allow both Rust and Zig interpreter
implementations to consume identical program bytes. That yields cross-language differential validation of the
wire format itself for free, which is design-relevant far beyond benching (it exercises the real untrusted-load
path). The upstream ask this implies: mockspace's bench harness documents its variant symbol contract as a
plain C ABI so a Zig-built cdylib is a first-class variant. That is a small, honest upstream fix (the
fix-the-stack-upstream rule applies) and it removes the last reason for standalone Zig timing loops to exist.

One more thing the carrier must be designed for: **the mockup interpreter is interim by declaration.** When the
real Zig runtime lands, the carrier's generators, program format, checksums, drivers, and CSV discipline all
stay; the interpreter-under-test becomes the real one. Nothing in the carrier should encode "toy" assumptions
that would have to be unwound; that is why the IR must be the real wire format from day one.

Concrete build steps, in order: (1) extract the showdown IR generator plus the archetype pattern from
`gen_showdown_ir.py`/`gen_archetypes.py` into the carrier crate with the record layout as specced and stride
asserts; (2) write the reference interpreter with the four variant axes and the checksum contract; (3) stand up
the harness driver by porting `hx_dispatch` and `hx_recwidth` onto it (these become the first two carrier
benches, replacing their bespoke copies); (4) write the scale-runner main and move the reach/egraph/arena-scale
questions onto it; (5) propose the C-ABI variant contract upstream so the Zig dispatch variants join; (6) delete
the five bespoke toy interpreters as each question migrates, per the no-legacy-shims discipline. Steps 1 to 4
are roughly two to three days of work total, and every subsequent bench gets cheaper than the bespoke path it
replaces.

### Rules for writing any vehje bench (the discipline the failures teach)

These generalise the audit's findings into checkable rules. They belong in the benches' README or as harness
lint checks where mechanisable, not in anyone's memory.

1. **Workload data crosses an opacity boundary.** Programs, tables, and inputs are runtime data to the code
   under test: through FFI in the harness, through a laundered pointer or file load in a standalone. A bench
   whose interpreted program is a comptime-visible constant is measuring LLVM's specializer, not an interpreter.
2. **Assert every layout the conclusion names.** If the finding says "24 bytes," the source carries
   `comptime assert(@sizeOf(Node) == 24)` (or the Rust equivalent). A stride claim without a stride assert is
   unverified, and this round proved it stays wrong silently.
3. **A cost-model sanity line is part of every findings file.** Translate the headline ns/op into cycles and an
   implied instructions-per-cycle at the known clock; state it; if the implied IPC exceeds what the core can
   retire, the number is wrong somewhere and the bench does not conclude. Both Tier C artifacts fail this
   arithmetic on their published numbers alone.
4. **The committed CSV is the bench.** No findings file may transcribe numbers that are not in a committed CSV
   produced by the run. An empty or header-only CSV means the bench has not happened yet. (Five of them shipped
   this round.)
5. **Models, measurements, and proxies are labeled at the source and at every citation.** An arithmetic model
   (depdag Q1) may be reported as a ceiling, never as a run. A condensed cost model (stack.zig cold) carries its
   scale factor to the realistic estimate. A proxy corpus (SP2) rides with every number derived from it.
6. **A comparative claim requires a good-faith strongest opponent.** Before publishing "A beats B," the findings
   must state what the strongest reasonable B is and confirm the benched B is it. The crippled delta and the
   spill-everything rec16 both fail this test and both produced published conclusions.
7. **The mechanism under test must exist in the code.** A cache bench contains a hash and a lookup; a
   parallelism bench contains threads and barriers. If the mechanism is elided, the bench is a model (rule 5).
8. **Single-shot numbers do not conclude.** Minimum three runs, report median and spread; the harness does this,
   the scale-runner must, and any remaining standalone must.
9. **Ranking stops at the noise floor.** Dead heats are reported as dead heats (the arch suite already does
   this; keep it).
10. **Every caveat survives summarization.** If a findings file states a regime bound or a discount (the 4.2x
    net, the heavy-op crossover), any document citing the headline must carry the caveat in the same sentence.
    This is the anti-DC-offset rule for the topic layer.

### The catalogue: variants never considered that should have been, and benches not done that should be done

**Record width (the never-considered contenders).** The bench compared inline-2-spill-all against inline-3 and
inline-4, and its own field inventory shows the space is richer. A true 16-byte record with THREE inline
operands exists if the header is 4 bytes (op u8, arity u8, flags u16, then three u32 operands): four nodes per
cache line and no pool at all, which would beat both benched layouts if the design's header needs fit in 4
bytes. If the header needs 8 bytes (op, arity, flags, plus the u32 shape_id the design names as the universal
discriminant), the natural layout is exactly 20 bytes, which is, ironically, what rec24 accidentally measured.
So the honest sweep is 12/16/20/24/32 with an explicit field inventory per stride tied to the design's actual
header requirement, not "16 versus 24" in the abstract. Add the two fair spill designs at narrow widths:
inline-2 with a single pooled pair (one extra load, no loop), and the inline extension-slot scheme (an arity-3
node occupies two consecutive stream slots, keeping sequential locality and no pool gather). Run all of it at
L1-hot and at res[] working sets of 256KB and 8MB, because the density axis only starts paying once res[]
leaves L1. The current conclusion conflated record width with operand count; these variants separate the axes.

**Dispatch.** Op-count sweep (3, 11, 25, 50) crossed with stream models (i.i.d., Markov-correlated at two
entropy levels) and two body weights, all on the carrier. Add the missing realism variant: dispatch measured
with CFG back-edges present, since block transitions perturb the predictor state the pure-stream benches keep
pristine. The toolchain-independent oracle (C with clang `musttail` plus `preserve_none`, same 25-op
interpreter, switch versus threaded) is the only way to answer the Deegen question before Zig grows the
convention; a day of work, clearly labeled as an oracle for the revisit trigger rather than a ship input.

**Native tier.** The composite-VM-versus-native bench (C1 option 1) on the carrier: shipped-default interpreter
against opaque-madd scalar native against the existing stencil variant, across the three regimes (L1
compute-bound, serial dependency, DRAM gather), program always FFI-borne. Then the stencil variants the branch
map already calls for and lists as next: a branchless (csel) stencil for unpredictable cheap branches, a
jump-table stencil for multiway, verified to close the large-n gap against `native_direct`. Add the adaptive
strategy (per-branch counter switching branch/branchless as observed predictability drifts) and the simd_batch
partition strategy from the map's own next-variants list; both are cheap on the archetype scaffolding and both
feed the proof-directed strategy portfolio directly.

**Fixpoint and relational substrate.** Real semi-naive (CSR out-edge index plus frontier worklist) at the three
scales, plus a graph-shape sweep the current synthetic generator cannot express: chain-depth 8, 32, 128
(deep-propagation regimes where whole-column's full rescans lose), and a fan-in-heavy shape. The current
bench's backward-window generator guarantees two-round convergence, which bakes the conclusion in; the sweep
finds the actual crossover so the design can state "whole-column below depth D, worklist above" instead of a
universal claim. Also the retraction case: the topic already names DBSP-counted differential as owed for
non-additive edits; a small bench of recompute-versus-counted on a delete lands with that work, not before.

**Incremental and load.** The honest warm-load bench (content-hash all inputs, real cache lookup, one miss); a
real threaded level-sync DAG loader with barriers (replaces the ceil(width/8) model); the sharded interner
merge (the named next lever: shard the dedupe, measure the tail shrink); and a full-pipeline cold load at one
realistic mod size to anchor the condensed model's scale factor (the 54 ms estimate becomes a measurement).

**E-graph.** The missing explosion driver: associativity/reassociation rule sets, which are the case the
"5.2x linear" claim is known not to cover, plus upward-merge congruence repair in the rebuild. The point is not
to prove linearity (it will not hold); it is to demonstrate the bounded streaming window actually terminating
and bounding memory on the pathological set, which is the design's safety mechanism and is currently asserted,
never exercised. This is the catalogue-edge-cases discipline applied to a bench: build the case that is
supposed to break, and watch the cap catch it.

**Counters and the harness itself.** Two upstream mockspace items. First, the C ABI variant contract (above).
Second, wire the `perf-counters` feature's macOS path (kperf) or explicitly document it as unavailable, then
either enable it for the branch suites (mispredict counts would turn the branch map's inferred mispredict
stories into observed ones) or strike "hardware-counter timing" from every description permanently. Either
resolution is honest; the current state (claimed on, actually off) is the only unacceptable one.

**Corpus.** The census harness for real consumers is already named as owed. Cheap interim hedge worth an
afternoon: run the same distribution extraction over one more unrelated proxy corpus (a Lua codebase, or a
Clausewitz-script dump if one is at hand, since that is the actual target domain). If arity/depth/binder-width
distributions are stable across two unrelated proxies, the 96.5% cut gains real confidence; if they diverge,
that is a finding the wire format wants before it locks, not after.

**Sketch-side (from C6, restated as build items).** The load verifier as `test` blocks with asserts plus a
fuzz-shaped generator over the five attack classes; the 3x3 exhaustive thermometer-lattice table; a property
test over let-nesting shapes for the N*W bound. Each is under an hour and upgrades a single demonstration to
the strength tier its design role requires.

## The bottom line for op (is the topic dependable as design justification as written, or the minimal set that must land first)

Not as written. The topic is roughly half bankable, and the unbankable half is concentrated in exactly the
findings it presents as decision-forcing. The branch-strategy study, the feasibility sketches (with strength
labels), the capstone profile, and the confirmations can be cited today. The minimal set that must land before
the topic hardens into design text: retract the 1.2x native ceiling and replace it with the corrected bracket or
the re-measured composite number (C1; the tier ordering survives, the multiplier does not); fix the record-width
labeling and run the widened rematch, including the never-benched true-16B-three-operand layout, before the wire
format locks, since that is the one day-one irreversible consumer of these numbers (C2); soften the
tail-threading epitaph to a toolchain verdict (C3); swap the incremental-scaling headlines for the honest
figures that already exist in the artifacts and make the warm-load bench measure a cache (C4); and correct the
methodology paragraph so no future reader believes counters were on or that BN1/BN2 were isolation-tested (C7).
C5 and C6 are each an hour or two and close the remaining honest gaps.

On the constructive side: op's carrier instinct is right and should be built as the carrier-plus-two-drivers
shape above (a shared IR-and-interpreter library, the real wire format from day one, driven by the mockspace
harness under the cap and by one shared scale-runner above it). It is not just deduplication; the FFI-borne
program makes the worst artifact class in this round structurally impossible, and the one-axis-off-one-operating-
point convention makes the numbers compose instead of being asserted additive. Steps 1 to 4 of the build plan
are two to three days and the rematches in C1/C2/C5 land naturally as its first benches, so the repair work and
the infrastructure work are the same work.

None of this moves the architecture: the interpreter floor, the wide-record lean, switch dispatch, the caching
mechanism, and the tier-dependent branch portfolio all remain the right calls, most of them now for better-stated
reasons. What changes is that the design would be citing measurements instead of, in the five places named, an
optimizer artifact, a no-op loop, an arithmetic model, a strawman, and a method claim that is false. The
evidence layer is worth having; it earns trust by being exactly as strong as it says it is, and after these
corrections it would be.
