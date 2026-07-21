# Certified generation: the theory (audit 3, PL-theory and verified-compilation lens)

**Date:** 2026-07-20
**Subject:** `202607201627_topic.compiling-the-proof-into-generated-code.md`, read after the four prior topics
in chronological order, then `01_carmack.md`, `02_giesen.md`, and `03_op-and-agent-response.md` as the panel
input this audit builds on.
**Method:** read the five synthesis docs and the five topics in order, then the three panel documents;
re-derived the partial-evaluation and region-inference claims from the primary literature (Futamura;
Jones, Gomard, Sestoft; Taha and Sheard; Tofte and Talpin; Calcagno, Helsen, Thiemann; Leroy; Pnueli et al.);
verified the Zig comptime semantics claims that the comptime-equals-runtime question turns on (type-punning
rules, host-leakage prohibition, target-awareness) against the Zig issue tracker and current write-ups. Where
I reconstruct the depth-ladder metatheorem's proof shape, that is first-principles work over this round's own
material, and I say so.

## Verdict in one paragraph

The certified-generation core is sound, and the panel's trajectory through it has been correct: audit 1's
certification-accounting correction is right, audit 2's two overturns (one inference at two binding times; the
MLKit recalibration) are right, and I confirm both from the theory side rather than merely deferring to them.
What this audit adds is precision about three things the panel has so far handled by gesture. First, the
design's actual assurance shape is not typed staged metaprogramming and it is not verified compilation; it is
partial evaluation by a hand-annotated specialiser whose every output instance is re-type-checked, plus
translation validation over the data bridge. That combination is respectable and adequate for the structural
properties claimed, but the word "certified" must be scoped to structural well-formedness, because
type-checking certifies shape, never semantics, and the design doc must not drift toward reading as a
verified-compiler claim. Second, the comptime-equals-runtime equivalence that "one codec, two binding times"
rests on is an engineering intention of Zig, not a theorem of Zig; it holds exactly on a statable
binding-time-invariant subset, and I give the concrete condition list (typed fixed-width integers, explicit
overflow handling, no floats, byte-shift or well-defined-layout-only decoding, no function-pointer dispatch,
budget-bounded loops) plus the mechanical closure: differential testing of the kernel at both binding times as
a standing build gate. Third, the depth-ladder lease metatheorem is genuinely provable as stated, and it is
strictly easier than what the literature has already proven for richer calculi, but it is a conditional
theorem whose per-family link-bit obligations are axioms discharged by testing, whose binder forms need
structural rules beyond the one-bit table, and whose paper proof requires a formal core semantics that does
not yet exist as an artifact. Nothing here reopens the resolution. All of it belongs in the design before the
experiment is trusted.

## The certified-generation core, judged as theory

**The Futamura identification is correct, one level up.** The general engine is a two-input program
E(language-definition, script). Specialising E to a fixed language definition and shipping the residual is
partial evaluation of E with respect to its static input, which is the first Futamura projection with the
language definition playing the role the program plays classically (Futamura, "Partial Evaluation of
Computation Process: An Approach to a Compiler-Compiler," 1971; Jones, Gomard, Sestoft, *Partial Evaluation
and Automatic Program Generation*, 1993). The parent audit's phrasing ("the first projection lifted from the
program level to the language level") is the standard mix-equation reading and I endorse it. The
binding-time division is done by hand (comptime annotations), which places the design in the staged-programming
lineage (the two-level languages of Nielson and Nielson; MetaML, Taha and Sheard 1997) rather than in
automatic partial evaluation, and that distinction matters for what is guaranteed, next.

**What actually secures the residual, stated exactly.** Partial-evaluation theory gives semantic preservation
of the residual only if the specialiser itself is correct; the mix equation is a definition of correctness,
not a free theorem. Typed staging (MetaML, typed Template Haskell, LMS) gives a once-proven guarantee that
every residual is well-typed, established when the staged program is type-checked. Zig comptime gives
neither: the comptime interpreter is an unverified specialiser, and there is no staged type system proving
residual well-typedness once and for all. What Zig gives instead is per-instance re-checking: every
specialisation the build forces is type-checked as ordinary Zig at instantiation. That is
translation-validation-shaped assurance (Pnueli, Siegel, Singerman, "Translation Validation," TACAS 1998;
Necula, "Translation Validation for an Optimizing Compiler," PLDI 2000): instead of proving the generator
correct once, each generated instance is checked. Combined with audit 1's lazy-analysis finding, the honest
accounting is: rustc type-checks the data and the typestate proves its consistency; the Zig compiler
re-checks each forced instantiation of the specialised engine; and the systematic comptime re-checks (or
audit 2's wire-bridge decode) validate the translation between them. Three mechanisms, all real, none of them
the LMS-lineage "well-typed generator run can only produce correct enforcement" story. The topic's paragraph
citing LMS, typed Template Haskell, and Terra attaches to the rejected Rust-emits-Zig-source path, exactly as
audit 1 found; I confirm the finding from the theory side and add that the correct replacement citations are
partial evaluation (Jones-Gomard-Sestoft) for the architecture and translation validation (Pnueli et al.,
Necula) for the assurance.

**What "certified" can and cannot mean here.** Type-checking certifies well-formedness: illegal states
unrepresentable, dispatch total, error paths handled, layouts guaranteed. It does not certify functional
correctness. A total, name-keyed, exhaustively-switched dispatch can still route to a handler whose body
computes the wrong semantics; audit 2's name-keyed dispatch narrows the transposition risk and closes none of
the semantic risk. Semantic correctness of generated or specialised code is the verified-compiler problem,
and the price of actually having it is known: CompCert proved semantic preservation through every pass in Coq
(Leroy, "Formal Verification of a Realistic Compiler," CACM 2009) and still trusts its unverified
pretty-printer and assembler rim; CakeML closed that rim only by verifying down to machine code. Vehje is not
buying either, and should not, but then the design's guarantee inventory must read: family inclusion, effect
inclusion, non-null, dispatch totality, and (conditionally, below) lease safety are certified structural
properties; family semantics are authored and tested, never proven; script termination is not claimed at all
(vehje has no totality axis; Dhall shows what buying one costs, per the purity synthesis, and vehje has
correctly declined). The primary topic already contains this admission ("whether the certified-generation
technique reaches only the structural discipline... or can be pushed further into the semantics"); the answer
from the theory is: it reaches the structural discipline, semantics need testing or a semantics-preservation
proof nobody is proposing to pay for, and the doc phase should freeze that answer.

**The tagless-final citation should be demoted to the contract level.** The interpret-transpile spectrum as
one algebra with many instances is Carette-Kiselyov-Shan and Hofer et al., correctly cited for the *contract*.
The *mechanism* vehje actually ships is initial encoding plus fold over an arena-held tree, which the
output-spectrum synthesis itself documents as the shape every real Rust compiler chose. Nothing is wrong
architecturally; the citation should just stop implying the final encoding is load-bearing anywhere in the
implementation.

## The comptime-equals-runtime equivalence (soundness and conditions)

The property "one codec, two binding times" needs has a name in the staging literature: it is the erasure or
annotation-soundness property of a staged language, that evaluating the staged program agrees with evaluating
the unstaged one (MetaML's semantics were built to make this a theorem). Zig provides it as a design
intention, not a theorem: comptime evaluation is a second implementation of Zig's semantics, an interpreter
inside the compiler, and "the same source computes the same result at both times" is exactly as true as that
interpreter agrees with the compiled code. There is no formal Zig semantics to prove agreement against. So
the equivalence is a trust-plus-discipline property, and the discipline is statable precisely.

**Points in Zig's favour, verified.** Comptime is hermetic (no I/O of any kind) and deterministic, which
op's note 1 answer already records. More importantly for this design, comptime is target-aware, not
host-aware: under cross-compilation, comptime code observes the target's byte order and integer sizes, never
the host's. That removes the classic cross-compilation seam entirely; there is no endianness divergence
between the comptime decode at our build and the runtime decode on the consumer's machine, by language
design. This is a genuine structural advantage over, say, running the kernel as a build.rs-style host program.

**The divergence set the kernel must exclude, concretely:**

1. **Arbitrary-precision comptime integers.** `comptime_int` is unbounded; the kernel must use explicitly
   typed fixed-width integers everywhere so both binding times compute in the same domain.
2. **Overflow behaviour.** On typed integers, comptime overflow is a compile error; runtime overflow is a
   panic in safe builds and undefined in ReleaseFast. The same input can therefore fail the build at one
   binding time and trap (or worse) at the other, which is a fidelity seam in the *failure* semantics, not the
   success semantics. Condition: all kernel arithmetic uses explicit checked, wrapping, or saturating
   operations with error-union results, so value-or-error agrees at both times. This composes with the
   panic-free invariant the panel already adopted; the same style discharges both.
3. **Floats.** `comptime_float` is 128-bit and runtime float behaviour is mode-dependent. Keep floats out of
   the kernel entirely. The design is integer- and fixed-point-shaped throughout (masks, depths, indices), so
   this costs nothing, but it should be stated as a rule rather than an accident.
4. **Pointer reinterpretation.** Comptime dereference requires the pointee to have well-defined layout, and a
   load whose bits are partially undefined is wholly undefined at comptime (the type-punning rules of Zig
   issue #9646). The wire records are `extern`/`packed` structs, so in-place overlay reads are comptime-legal
   in principle, but two practical hazards remain: `@embedFile` yields bytes of alignment 1, so overlay decode
   at comptime needs explicit alignment handling, and any undefined padding poisons comptime loads. The
   robust form of the dual-binding kernel is byte-shift decoding (explicit shifts and ors per field) rather
   than pointer overlay; if overlay is wanted for the hot runtime path, the experiment must confirm the same
   overlay code is accepted and agrees at comptime, else the kernel keeps the byte-shift form and the runtime
   may add a separate validated fast path. This is a checkable condition, and it is the one place audit 2's
   move 1 ("the same decoder the shipped runtime uses") may quietly not survive contact with comptime.
5. **No function-pointer dispatch.** Comptime cannot call through runtime function pointers, so the
   allocator-parametrisation answer in doc 03 cannot mean `std.mem.Allocator` (a vtable). It must mean either
   a comptime type parameter specialising a concrete provider per binding time, or, better, an
   allocation-free kernel over caller-supplied fixed buffers. The design wants the latter anyway; state it.
6. **Budgets.** Comptime termination is enforced by the branch quota; runtime termination of the load passes
   is enforced by the traversal and depth limits. Both are budget-bounded, and neither is a totality proof.
   The kernel's own loops are bounded by input size, which the post-order, backward-link-only wire form
   guarantees structurally (below), so the only budget question is sizing, not termination.

**The mechanical closure: differential testing as a build gate.** The conditions above make divergence
implausible; a test makes it checked. Run the kernel at comptime over the language blob (and any bundled
corpus) into consts; run the identical kernel at runtime in a test over the same bytes; assert bitwise
equality of results, including the error cases from the negative corpus. This is cheap, it rides the
experiment audit 2 already widened, and it converts the erasure property from an intention into a
per-build-verified fact. With the condition list committed and the differential gate standing, I judge the
equivalence sound, and I judge it *not* sound to rely on without them: doc 03's "a discipline, not a wall" is
correct, but the discipline is the six items above, not a vibe.

## The depth-ladder lease metatheorem (is it actually provable as stated)

Audit 2's schema, reconstructed: leases are lexical body depths on a strictly nested stack (de Bruijn levels
over the binder stack); each family operand position carries one link-or-consume bit; inference propagates
minimum depth over linking uses transitively; the metatheorem says conservative bits imply every value's
lease is no shallower than any use, hence no dereference into a reset region.

**Where it sits in the literature, and why that makes it tractable.** This is the degenerate case of the
region calculus: regions totally ordered by lexical depth, allocation and deallocation in LIFO discipline, no
region variables, no region polymorphism, no letregion under polymorphic recursion. Tofte and Talpin proved
soundness for the full calculus and the proof was famously hard (rule-based co-induction; Tofte and Talpin,
"Region-Based Memory Management," Information and Computation 132(2), 1997); the field then produced simple
syntactic proofs in Wright-Felleisen style (Helsen and Thiemann; then Calcagno, Helsen, Thiemann, "Syntactic
Type Soundness Results for the Region Calculus," Information and Computation 173(2), 2002). The depth ladder
is strictly weaker than what those papers prove: a syntactic progress-and-preservation proof over the eleven
core forms, with a store typing indexed by the live-depth stack and the invariant that reachability respects
the lease order (the lease of every pointee is no deeper than the lease of every pointer that reaches it), is
routine by post-2002 standards. So yes: provable as stated, tractably, once. Three qualifications keep the
claim honest.

**Qualification 1: it is a conditional theorem, and the condition is per-family axioms.** The induction over
the eleven core forms is ours to prove. The family obligations ("this op's bits conservatively approximate
its semantics") are axioms per family, undischargeable in general because family semantics are hand-authored
Zig; they are exactly the shape of Tofte-Talpin's assumed-correct primitive effect signatures and of the LMS
`Summary` lattice the purity synthesis warns about ("a miscategorization is a silent correctness bug"). The
theorem's honest form is: *for all programs, if every family's declared bits are conservative, then no
dereference into a reset region occurs.* Audit 2's armed generational-check harness is the correct empirical
discharge of the axioms (and Vale's region work, which elides generation checks over immutable regions, is
the right precedent that the shipped cost stays zero). The design doc should print the theorem in its
conditional form and name the axioms as the trusted surface.

**Qualification 2: the one-bit table under-describes the binder forms, and the crux lemma should be named.**
For ordinary operators, one bit per operand position is right. For binders it is not a bit: a Lambda's
closure links every captured free variable (a per-program set the inference computes from the IR, not a
declared constant), and a Let links its bound value into its body's evaluation. These are fixed structural
rules for the core forms, not table entries; the table is for family extensions. And the argument that
defeats the sibling-body hazard (audit 2's own finding 1: types cannot see same-kind instance boundaries)
deserves to be stated as the crux lemma of the whole proof: *in the immutable core, a value created inside
body A is reachable outside A only through A's result value.* Given that lemma, min-depth propagation through
result linking is exactly what makes a cross-sibling flow lengthen the lease to the common ancestor, and the
inference never needs to see instances at all. The lemma is where immutability does its load-bearing work; it
is also precisely what mutable stores break, which is why the Lua analysis below routes every store through
the conservative rule.

**Qualification 3: there is no formal semantics artifact yet.** "Proven once on paper" requires a small-step
(or big-step with explicit region store) semantics of the eleven forms to induct over. None exists in the
round. Writing one is small (the calculus is first-order over a flat arena; Calcagno-Helsen-Thiemann is the
template) but it is a real named deliverable, and until it lands the metatheorem is a promise, not an
artifact. It should be a deliverable of the lease round, sequenced with (I would say after) the one-family
experiment, and before the doc CL that relies on the word "proven."

**Promotion as the sound fallback: yes, by the standard lattice argument, with two riders.** Lengthening a
lease moves the value toward the outermost region, which over-approximates the required lifetime; safety of
over-approximation in the safe direction is the ordinary abstract-interpretation argument (the analysis
computes a lower bound on required lifetime; anything at or above the bound is safe). Promotion to the output
region is the top element, hence always safe for memory validity. Rider one: promotion converts inference
imprecision into peak-memory pressure inside the host budget, so the budget-exhaustion path (a `reserve` that
cannot be satisfied) must itself be a graceful, panic-free error, or the lease axis's fallback quietly
violates the panel's own untrusted-path invariant. Rider two: a promoted value must be expressible in the
chunked streaming form (it will be emitted late and referenced across chunks), so the fallback's soundness at
the format level depends on the cross-chunk lemma audit 2 flagged and doc 03 left open. That lemma ("a
cross-chunk reference is by construction consumed-or-promoted, because chunk boundaries are whole-subtree
boundaries") is plausible and unproven; it belongs in the same proof document as the metatheorem, since both
induct over the same interval structure.

**Monotone annotations: genuinely incapable of introducing unsoundness, with one scoping condition.** An
explicit lease that may only lengthen keeps the effective lease at or above the inferred lower bound, so the
invariant is preserved for any annotation the consumer writes; this is sound by the same lattice argument,
and it is the right design for the consumer surface. The scoping condition: annotations must only influence
region placement. If lease values ever feed anything else (chunk scheduling, wire nesting emitted before
inference completes, a family's own semantics reading its operand's region tag), the monotonicity argument
covers only the placement use, and each additional use needs its own monotonicity check. Cheap to state now.

**The Lua claim (precision loss, never unsoundness): defensible, and here is the exact argument and its
boundary.** A store of v into mutable table T makes v reachable from T, so soundness needs lease(v) at or
above lease(T). If every store is declared link, the static fixpoint imposes exactly that; and because the
inference is a static minimum over all potential flows, mutation *ordering* cannot break it (a later store
into a longer-lived alias is just another flow in the fixpoint). Without alias analysis the conservative
collapse is that anything stored into any mutable cell promotes toward the depth of the shallowest table it
could reach, degenerating to promotion, which is the advertised precision loss. So the claim holds, on two
conditions that are the claim's real content: (i) the store primitives of the mutable family are declared
link on the stored operand, with no exceptions, and (ii) *every* effectful or host-boundary op of every
family that can retain a reference (a callback registration, a host-visible cache, an interner) declares link
on the retained operand. Condition (ii) is where I sharpen the panel: the trusted surface is not "eleven rows
of link bits"; it is the link bits of every family op including the effectful rim, and a wrong consume on a
host-callback operand is the single most realistic unsoundness vector in the whole axis. The armed harness
tests it (fuzz under armed generation checks; a trap indicts the declaration), and the doc should name it as
the place declarations are most likely to be wrong, because the retention there is invisible in the IR.

**A structural bonus the design should claim on purpose.** The transfer topic's children-before-parents,
backward-link-only wire order makes the arriving value graph a DAG by construction once bounds validation has
confirmed all links point backward. Every load pass (validation, mask accumulation, lease inference) then
terminates structurally with no cycle detection, no occurs check, and no visitor stack, and audit 2's
linear-scan observation follows. This is a small theorem ("backward-only links imply acyclicity, imply
single-pass termination") worth one paragraph in the same proof document, because it is the totality argument
for the load-time checker itself, which nobody has otherwise supplied.

## One proven inference at two binding times, and the wire-codec bridge

**The overturn stands.** Audit 2's correction of audit 1 is architecturally forced: under the three-codegens
topic the Rust side never sees content, so there is no Rust-side act that could compute a particular
program's regions, and any "dev-time content is certified generation" story would in fact be a second
implementation with a fidelity seam. One implementation, executed by the comptime interpreter for bundled
content and by the shipped binary at load, is the only shape consistent with the round's own layering. What
the panel had not priced is that the soundness of this move *is* the comptime-equals-runtime equivalence, so
its precise conditions are the six-item list above, and the differential gate is the mechanism that keeps the
two binding times provably in agreement build over build. With those committed, I find no residual fidelity
seam beyond the ordinary "trust the pinned toolchain" seam every compiled system carries.

**The bridge: trust is relocated and then partially discharged, not removed, and the relocation is the right
trade.** Accounting before: an ad-hoc printer from rustc-typed structures into Zig const syntax, unverified,
exercised once per language, whose only checker is the Zig parser. Accounting after: `encode.rs`, a codec
that is exercised on every residual the system ever produces, that has a validator, and that supports the one
verification a codec admits cheaply, the round-trip test (decode of encode is identity, checkable
property-based over generated tables); plus comptime re-checks over the decoded structures. That is
translation validation of the bridge on every build. The CompCert analogy audit 1 drew is exactly right and
cuts both ways: CompCert's theorem famously stops at the pretty-printer, and closing such rims completely is
a CakeML-sized project. Vehje's remaining rim is the encoder, the pinned Zig toolchain, and the comptime
interpreter's agreement with compiled code. The design should say "validated, not verified" about the bridge
and stop there; that is the honest maximum short of verified extraction, and it is enough.

**One upgrade the panel has not named: re-prove, not just re-check, the decidable axes after transport.** The
family and effect proofs are decidable set inclusions over small data. After the comptime decode, the build
can re-run them wholesale (not merely check internal consistency): recompute the masks from the family table,
re-derive dispatch totality against the tag enum, re-verify the lease table's well-formedness. For those two
axes this upgrades the bridge from "trust the round trip" to "the property is re-established on the far side,
so the encoder is no longer trusted for them at all." The lease *metatheorem* cannot be re-run (it is a paper
proof about the algorithm, not about data), so for the lease axis the bridge trust reduces to the bit table's
integrity, which the re-checks cover. This asymmetry (re-provable set axes, re-checkable-only lease inputs)
should be stated in the design, because it is the exact boundary of what the "certified twice" phrase can
honestly cover per axis.

## Where the theory overreaches or has gaps

1. **"Certified twice" conflates type-checking with verification.** Scope the word: structural properties are
   certified; semantics are tested; nothing is semantically verified in the CompCert sense, by choice. The
   guarantee inventory per axis belongs in the doc CL verbatim.
2. **The typed-staging citations attach to the rejected path** (audit 1's finding, confirmed here from the
   theory). Replace with partial evaluation for the architecture and translation validation for the
   assurance; keep tagless-final at the contract level only.
3. **"Proven once" is currently a promise.** No formal semantics of the eleven core forms exists to induct
   over. The metatheorem, the crux immutability lemma, the cross-chunk lemma, and the backward-link totality
   note are one small proof document; name it as a deliverable of the lease round.
4. **The trusted surface is understated.** It is: the link bits of every family op *including the effectful
   and host-boundary rim* (the realistic unsoundness vector), the encoder (for the lease inputs only, after
   the re-prove upgrade), the pinned toolchains, and the comptime-runtime agreement (closed by the
   differential gate). Say all four; the "eleven rows of bits" phrasing is rhetorically satisfying and
   materially incomplete.
5. **Doc 03's allocator-parametrisation answer is glib as written.** A vtable allocator cannot run at
   comptime; the kernel is either specialised over a comptime provider type or, better, allocation-free over
   caller buffers. Restate before anyone builds to the current wording.
6. **The streaming spine (op note 2) re-introduces untrusted-input structure mid-pipeline.** Spilled IR read
   back from disk is bytes again: either the read-back re-runs the bounds validation (cheap, linear, my
   recommendation) or the spill file is declared trusted-local with an integrity check, but one of the two
   must be chosen, and the choice affects whether the post-order interval structure the lease scan needs is
   re-established or merely assumed after a spill round trip. Doc 03 lists determinism and interval
   preservation as open; add the trust posture to the same open item.
7. **No totality axis, so "proven-safe" must never be read as "terminates."** Scripts may diverge; the
   budgets (traversal limits, quotas, host-lent memory) are the containment, and they are policy, not proof.
   One sentence in the design prevents the strongest misreading of the whole certified-generation story.

## Open questions for op and the next expert

1. **Bless the scoped meaning of "certified"** (structural properties by construction; lease safety
   conditional on declared bits, algorithm proven once; bridge validated, decidable axes re-proven post
   transport; semantics tested; termination not claimed). This is wording, but it is the wording the entire
   design will be read through.
2. **Commit the comptime-kernel conditions and the differential gate** as build-gate requirements alongside
   the forcing root, the negative corpus, and the Zig pin. The equivalence is sound exactly on that subset
   and unproven off it.
3. **Sequence the proof document.** My recommendation: run the widened one-family experiment first (it will
   shake the schema), then write the formal semantics plus metatheorem plus the two lemmas against the
   surviving schema, before the doc CL locks any "proven" language.
4. **Decide the spill trust posture** (re-validate on read-back versus trusted-local), and fold it into doc
   03's open item 2.
5. **For the next expert:** pressure-test the effectful-rim link-bit discipline against the real census
   consumers (Lua table and registry primitives, Jomini's event-and-scope effects, host callback families):
   enumerate the ops that can retain a reference and check the conservative default actually gets declared,
   because that rim is where I judge the axis will first be wrong in practice. And probe whether the
   cross-chunk lemma survives the streaming spine's spill boundaries, which is the one interaction of op note
   2 with the lease proof that nobody has yet walked.

## Bottom line

Adopt, with the accounting finished rather than merely corrected. The core move is partial evaluation done at
the right binding time with per-instance re-checking, which is a sound and well-precedented assurance shape
so long as nobody calls it verification. The comptime-equals-runtime equivalence holds on a subset the design
can commit to and test differentially, and nowhere else. The depth-ladder metatheorem is real mathematics of
a deliberately easy kind, easier than what Calcagno, Helsen, and Thiemann already proved for a richer
calculus, and the design should pay the small cost of actually writing it down, in conditional form, with its
crux lemma and its per-family axioms named as the trusted surface. The bridge is validated, not verified, and
for the two decidable axes it can be better than validated by re-proving them after decode. Every correction
in this audit is a precision fix, not a redesign; the resolution has now survived three lenses with its
architecture intact, and what stands between it and a doc CL is one experiment, one condition list, and one
short proof document.
