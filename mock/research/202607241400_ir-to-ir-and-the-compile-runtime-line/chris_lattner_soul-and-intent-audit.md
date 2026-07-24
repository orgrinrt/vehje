# The soul and intent of vehje, audited (Chris Lattner)

**Author lens:** compiler and language infrastructure; multi-level IR design; the engineering of adoption;
what makes an extension seam survive third-party authors. Fresh read, no continuation of prior panel framing.
**Read in full:** every file in this directory (the two `canonical_candidate_` documents, `00_context.md`,
the six expert deliverables, the four worker forks, the inventory, `op-ratification-answers.md`); the shipped
canon `mock/design_rounds/202607241615/202607241545_topic.the-vehje-canon.md` and its changelists; the
second-direction audit `mock/research/202607241700_second-direction-audit.md`; and the sources and benches
behind the load-bearing claims (`vehje-typecheck/src/lib.rs`, `vehje-ir/src/grade.rs`, `vehje-ir/src/hash.rs`,
`vehje-runtime-gen/src/lib.rs`, `vehje-lower/src/lib.rs` + `DESIGN.md.tmpl`, `vehje-schedule/src/lib.rs`,
`vehje-runtime-abi/src/wire/residual.rs`, `mock/benches/results/RUN_SUMMARY.md`,
`mock/benches/results/carrier_native_ceiling/FINDINGS.md`). Where I cite a source line below, I re-read that
line myself rather than trusting a panel citation.

## Verdict in one paragraph

The soul-and-intent document holds. The identity it enumerates (an embeddable multi-input multi-output IR
framework with a small closed Core, open consumer families, inclusion-not-coverage targets, a binding-time
lattice as the compile/runtime structure, certified generation as data not source, and a bench-decided
working discipline) is coherent, internally consistent after the panel's bounded correction set, and well
grounded in real prior art it names honestly by degree of commitment. Its one-sentence soul survived five
independent readers unchanged, and its structure (a positive enumeration and a negative inverse that audit
each other, with concessions applied inline) is the right shape for an identity document; the
commitment-tier device (married / half-married / heeded / killed / negative-precedent) is the best single
contribution in the directory and I would steal it for any infrastructure project. The essential thing is
not wrong. What the document got wrong is characteristic rather than structural: it pinned bench numbers
harder than the committed evidence supports (three of SPJ's four factual findings are citation-integrity
failures inside a document whose own method is "hard data over everything"), it presented its proudest
novelty (CR1) as more proven than the sketches support (Wingo's re-execution-versus-reinstatement catch is
the sharpest technical finding here), and its §8 "four commitments are shadows of one idea" committed
exactly the unify-by-analogy move its own §9.A celebrates the round for refusing. Every one of these was
caught by the panel, ratified by op, and is corrected in the shipped canon (`202607241545`), which is the
strongest possible evidence for the document's deepest claim: the discipline it canonizes (adversarial
audit, bench supremacy with epistemic humility, supersede-forward) actually works on its own outputs.

## What holds

**The framework identity is the correct shape, and it is the MLIR thesis with a differentiated frontier.**
"Framework, not a language; a grammar plugs into the input side, a target into the output side; the
framework owns the shared Core, the passes, and the ABI" (`canonical_candidate_the-soul-and-intent-of-
vehje.md:25-31`) is the same structural bet MLIR made (Lattner et al., CGO 2021): stop hand-writing one
compiler per language, own the shared representation and plumbing, let consumers define their level of
abstraction. Two things distinguish vehje's version rather than merely repeat it. First,
inclusion-not-coverage (§2.4, `:77-80`): a target declares `Supports`/`Permits` and refuses anything
outside, named. That is the SPIR-V `OpCapability` / MLIR target-legality discipline lifted into the type
system, and refusing-with-the-construct-named beats Pandoc-style silent degradation, which the doc
correctly identifies as the failure mode. Second, the graded proof spine and certified generation
(§2.2-2.3, `:67-76`) are things MLIR deliberately does not carry: MLIR has no effect or binding-time proof
in its type system and is heap-heavy by design. Whether the spine lands is open (see below), but as
identity, "the type system is the verification layer" on a `no_std`/no-alloc stack is a real,
differentiated position, not a re-skin of existing infrastructure.

**The binding-time lattice as the compile/runtime line is right, and its corroboration is unusually
strong.** The four-point lattice {LanguageAuthor, Bundler, HostLoader, Runtime} at `vehje-ir/src/
grade.rs:105-115` (verified) is a multi-level generalisation of classic two-level binding-time analysis
(Nielson-Nielson; Jones-Gomard-Sestoft 1993), and the canon correctly cites Glück-Jørgensen multi-level
generating extensions for the discharge rule. Rompf's pinning (AOT at LanguageAuthor and Bundler; load-time
compilation, not profile-guided JIT, at HostLoader; interpretation as a Runtime execution form;
const-inlining mandates a specialisation stage at the binding time the constant becomes known, never a
native step; `tiark_rompf_staging-and-the-line.md:54-70`) is the correct resolution of the panel's (B)
question, and the fact that an independent staging read reconstructed the round's own axis from code alone
is the kind of corroboration you almost never get. The Futamura framing is used correctly: specialising the
generic Zig engine to the language definition at build time is the first projection as a build step, which
is precisely what weval does for wasm interpreters in production, so the technique has an existence proof
under exactly these no-JIT-permission constraints. The two-artifact split (Rust never in the runtime) is
independently forced, not fiat: every no-alloc ahead-of-time-shipping staged system reaches it because
there is no hosted compiler to load code back into (the Terra `saveobj` shape, not the LMS `compile`
shape), a point Fallin's addendum lands well (`chris_fallin_weval-and-the-load-stage.md:323-333`).

**The prior-art posture, held by degree of commitment, is the document's best invention.** §6's married /
half-married / heeded tiers, plus the inverse's killed and negative-precedent tiers, encode "a bench trumps
a prior-art cite" as a per-citation state (`canonical_candidate...:259-341`). Every named work is real and
correctly attributed: Davies-Pfenning (JACM 2001) for staged modality, Katsumata (POPL 2014) graded monads,
Petricek-Orchard-Mycroft coeffects, Gaboardi et al. (ICFP 2016) for the combination, Granule as
implementability evidence, Bao et al. (OOPSLA 2021) reachability types, Plotkin-Pretnar (ESOP 2009)
handlers, Xu-Kjølstad copy-and-patch (OOPSLA 2021, and the tag inversion that mislabelled it PLDI was
itself caught), Filliâtre-Conchon hash-consing, Flanagan et al. ANF (PLDI 1993), Reynolds 1972
defunctionalization, Kohlbecker et al. 1986 and Flatt (POPL 2016) hygiene, Tate et al. (POPL 2009) and egg
(POPL 2021) equality saturation, Necula-Lee certifying compilation, CompCert's trusted rim. The
half-married ledger records genuine bench overrides: Deegen-lineage tail-threading imported on the
dynamic-language literature and re-scoped to CFG terminator transfers only, which is consistent with the
published record that wide out-of-order cores plus modern predictors erode threading's advantage on
predictable streams (Rohou et al., CGO 2015, "don't trust folklore") while threading still wins where
dispatch is genuinely unpredictable. A project that writes down how tightly it holds each borrowed idea,
and demotes on measurement with the demotion dated and cited, is doing something most infrastructure
projects never do.

**The bench-honesty section is load-bearing and its sharpest insight is correct.** §5's record
(`canonical_candidate...:197-257`) of the over-claim-then-adversarial-correction cycle, and especially §9.F's
observation that the framework's own central mechanism hid inside its own benchmark (the compiler
partially evaluated the interpreter into native, so the native-ceiling bench compared native to native
until the workload was forced across an opacity boundary, `:434-453`), is a subtle, real, and durable
lesson. I verified the corrected numbers against the committed artifacts: `carrier_native_ceiling/
FINDINGS.md:15-21` reports the switch interpreter at ~2.0x native as the floor-at-best-case, and
`RUN_SUMMARY.md` finding 10 reports ~1.5x over the best (predecoded) interpreter; findings 2, 6, 8, and 9
carry the threading-wins-on-CFG (trace 0.44x of switch), eqsat-out (marginal zero-or-negative), direct-isel
~2x cheaper compile, and vert8 ~4.8x results exactly as the soul pair cites them. The through-line the
document draws (the value is in the cheap binding-time-directed reducer and the vertical-SIMD batched ABI,
not the e-graph or the stencil) is what the data says.

**"Native is never the driver" survives every number on record, and the product framing seals it.** The
scalar-native ceiling dispute (1.2x vs 1.5x vs 2.0x) never threatened the qualitative claim, because even
the largest live number is nowhere near the 10x that would make native the point, and the 10x class needs
vectorisation copy-and-patch cannot do. Wingo's deflator is the best strategic sentence in the directory:
the authoring-and-templating majority is compile-heavy and trivial-execute (3.24 ms compile, 127 us run on
the 50k-node capstone), so the interpreter's dispatch cost governs the product and the native multiple is
nearly moot for the identity (`andy_wingo_soul-double-take.md:44-52`).

**The negative fork's terminator reconciliation is the single most valuable design catch.** Rompf, fresh to
the round, proposed a fuel cap for the expansion fixpoint (`tiark_rompf...:26`); the round had already
killed cap-as-termination for this class of unfold because a no-alloc cap makes lowering non-deterministic
(`202607202055`, PE-as-extraction, cited at `canonical_candidate_soul-and-its-inverse...:232-235`). The
negative fork supplied the provenance, both forks converged, SPJ layered it into the three-layer statement
(F6), Wingo folded it into generative freshness, and op then corrected the panel in the other direction
(recursive macros are wanted; the cap returns as an always-on hang-guard, categorically distinct from
cap-as-semantics, `op-ratification-answers.md:11-29`). That full arc, panellist instinct vs recorded kill vs
op's actual intent, resolved with all three distinctions preserved (well-foundedness as semantics, cap as
diagnostic backstop, exhaustion distinct from passthrough), is design governance working exactly as it
should, and the canon's 2G carries it correctly, including Leroy's always-on fidelity fix.

## What is thin or wrong

1. **§8's "the four commitments are its shadows" is unify-by-analogy, in a document that canonises
unify-by-construction.** The claim (`canonical_candidate...:360-374`): one signature projected, the proof
compiled away, two artifacts, and native-never-the-driver are four projections of the one handler
discipline. Two of the four are not derivable from it: the census identity and native-never-the-driver are
op's identity call plus bench evidence, and a vehje with the identical handler discipline but a
game-runtime-first identity is perfectly coherent, which proves independence (SPJ F3,
`simon_peyton_jones_soul-audit.md:102-114`). This matters operationally because the doc nominates §8 as
"the fastest test" for drift, and a mechanism-only test passes proposals that betray the identity.
Infrastructure history says identity and mechanism are separate axes: LLVM's identity survived multiple IR
mechanism changes; deriving one from the other gets the identity falsified the day the mechanism moves.
Traceability note the panel did not draw: the shadows framing originates in the idealistic synthesis
("Not four commitments: one. The four are its shadows,"
`worker-fork_idealistic-synthesis...:43-49`), written 34 minutes before the positive fork, whose §8 reads
"read through the idealistic synthesis." The slip was copied, not independently derived, which is exactly
the intermediate-round drift mechanism this workspace's own rules name. **Fixed** in canon (drift test =
identity and mechanism jointly, Section 10; "carried by" not "projections of").

2. **CR1 was presented as settled novelty when only its trivial half is demonstrated.** The soul doc lists
no-alloc bounded multi-shot continuations flat among the host-lent-budget uses (§2.10, `:102-106`) and as
"genuine vehje novelty" (§4.4, `:171-173`); the inverse calls it "the sharpest original contribution." Wingo
read the sketches nobody else opened and showed sk2 re-executes a pure caller-supplied function per choice
(the McCarthy way to do bounded `amb` since forever) and never captures or reinstates anything; prefix
sharing, waved off in the sketch findings as "a refinement," is the actual continuation-capture problem
where all the difficulty lives; and multi-shot under other effects in the handled body is a soundness
obligation (re-execution is only sound over a pure prefix, which is why OCaml 5 made resumption one-shot by
default, Sivaramakrishnan et al. PLDI 2021), currently invisible to the check because effect subtraction is
a FIXME (`vehje-typecheck/src/lib.rs:301-303`, verified). The novelty claim itself needs its scope: Koka
and Effekt compile multi-shot handlers but heap-allocate; bounded no-alloc multi-shot *reinstatement* would
be genuinely novel, and it is exactly the unproven half. **Fixed** in canon 2C (scope carried,
representation a bench-decided fork per op's XVII4), but the failure mode deserves its name: the project's
proudest claim was the least verified one in the document, which is the direction over-claiming always
takes.

3. **A "hard data over everything" document mis-cited its own hard data in at least four places.** SPJ F1
(the consolidation locked 1.0-1.5x, never the 2.0x the doc attributed to it), F2 ("~1.5x is the honest
ceiling" pinned as a scalar when the committed record holds two live baseline-indexed numbers, ~1.5x over
predecoded and ~2.0x over switch, spread unreconciled), F4a (register VM cited at the superseded 1.9x
composite instead of the corrected 1.41-3.06x per profile), F4b/F4c (NaN-boxing "married on the evidence"
overstating a low-to-medium-confidence cell; the tnum entry silently re-widening a killed scope). I
verified F2 against `carrier_native_ceiling/FINDINGS.md` and `RUN_SUMMARY.md` finding 10 myself: SPJ is
right, and the doc's "doubly-superseded" dismissal of the 2.0x was unfair to a number the committed cell
reproduces. The general lesson is worth stating harder than anyone did: dense line-cited provenance
*creates the appearance of verification without being verification*. Three independent readers (the
negative fork, SPJ, Fallin in his self-correction) each caught numeric drift the citations dressed as
settled. The canon's response (state ranges with their baseline index, owe the reconciling cell, and op's
new benches-are-provisional principle, `op-ratification-answers.md:228-235`) is the right correction, and
the proven/demonstrated/measured/intended verb ladder (Leroy, canon preamble `:17`) is the standing
counterweight. **Fixed**, but this is the document's characteristic weakness and future soul-grade prose
should treat every numeric pin as suspect until re-read from the committed CSV.

4. **The certification claims invoked CompCert's lineage without doing the one thing that lineage demands.**
"Doubly certified, no prover shipped" ran through both candidates with no statement of what is certified by
which checker and no enumeration of what remains trusted. Leroy's finding 3 and his trusted-base finding
(`xavier_leroy_canon-audit.md:95-103`, `:191-199`) are the deepest additions in the panel: rustc certifies
well-formedness/inclusion/grading of the emitted data, Zig comptime certifies the specialisation
type-checks against it, *neither certifies semantic preservation of any lowering*, and the trusted base
(Zig compiler, rustc, the C ABI marshalling, the byte-image hash, the open emitter seam, the
caller-supplied sets, the verifier's decode) was scattered across five sections until the canon's 2E wrote
the perimeter down. A certification claim without a stated perimeter is marketing; with one, it is
engineering. **Fixed** in canon 2E, verbatim the trusted-rim discipline.

5. **The soul doc carried one present-tense claim of its own that the shipped state contradicts, and the
class is systemic.** Canon Section 4 (inherited from the candidates) states the relational-fixpoint engine
"serves lease inference and load verification"; `grep -rln vehje_fixpoint::` across consumer crates returns
nothing, and no load verifier exists (second-direction audit item 9,
`202607241700_second-direction-audit.md:27`). The same audit found the pattern at scale where the soul pair
never looked: `vehje-schedule` is three near-empty stubs behind a fully-specified DESIGN (verified:
`Pass` is `pub trait Pass: core::fmt::Debug {}`, `PassDag` a unit struct, `Schedule::build()` returns
`Maybe::Is(Self)` unconditionally, `vehje-schedule/src/lib.rs:27-73`); the distribution-composition entry
point is claimed in `vehje/src/lib.rs:1-8` with zero implementation and zero FIXME; `vehje-fixpoint` calls
itself semi-naive while its own `engine.rs` FIXME says only whole-column ships. None of this indicts the
*identity*, but it shows the enumeration's scope boundary: the soul pair audited the design record against
itself and the panel audited the headline mechanisms against source, and the long tail of present-tense
crate-doc over-claiming was only caught by the second-direction walk op ordered. The two live wounds the
panel did catch (CFG dead surface, inert signature) and the tail Dolan caught are one weakness: surface
existence standing in for mandate satisfaction, named precisely by the changelist-drift fork
(`worker-fork_changelist-drift-and-claim-verification.md:35-62`).

6. **The maximal-shape ethos, as the candidates state it, lacks its own counterweight.** I am sympathetic
to the ethos (design the unification completely; reject re-tiering as the defer instinct; §3): it is the
correct posture for infrastructure, where the under-built foundation calcifies and the workarounds become
load-bearing. But this round's own record shows the failure mode on the ethos's blind side: a fully
designed, locked, "settled and not reopened" mechanism (the CFG-of-blocks interpreter) that read as built
for multiple rounds because nothing labelled it intended. Design-first only stays honest when every claim
carries its assurance verb, which the candidates did not systematise and the canon now does. The ethos and
the verb ladder are a matched pair; either alone fails (YAGNI on one side, aspiration-read-as-fact on the
other). The canon's 5 ("the honesty valve... prove the exact blocking constraint and design the maximal
thing that does hold") plus the preamble ladder is the complete form.

7. **Minor, uncorrected anywhere: "thermometer-encoded grade lattices" sits in the married prior-art tier
as if it were a citation.** It is a folklore encoding (monotone-set-as-prefix-mask), not a work one can be
married to. Harmless, but the tier device's value is its auditability, and a non-citable entry dilutes it.
Also minor: the soul doc's §5 preamble asserts "the harness is trusted" while §5's own record shows the
design's central measurement (the cost-model k-sweep) was never built until the PMU re-run's finding 8
closed it; the two statements coexist honestly but the preamble's framing runs a shade ahead of its own
evidence, the same lean as finding 3.

## The soul vs the shipped state

The identity is largely intent, and post-correction the record is honest about it. Verified state at audit
time: the binding grade is hard-coded empty (`vehje-typecheck/src/lib.rs:311`, with its FIXME); the check
pass reads no `vehje-signature` schema (zero code uses; the crate's one mention is a hedged doc comment);
`vehje-runtime-gen::generate` ignores `_signature` and whole-package byte-hashes caller-supplied slices
(`vehje-runtime-gen/src/lib.rs:108-124`); `Anf` and `MacroExpand` are FIXME'd no-ops
(`vehje-lower/src/lib.rs:385-408`); the CFG residual types are defined and never constructed, now carrying
the owed FIXME naming the `Anf` dependency (`vehje-runtime-abi/src/wire/residual.rs:98-125`); the `hash.rs`
header self-contradiction Leroy flagged is fixed in source (`vehje-ir/src/hash.rs:1-14` now states the
within-stage/cross-artifact boundary correctly); and `vehje-lower/DESIGN.md.tmpl:72` has been re-tensed to
"Recursive macros are designed to be supported. State: ...", closing the second-direction audit's headline
item 8. So the correction machinery is demonstrably running: between the panel, the canon, and the
second-direction audit, every load-bearing gap I checked is now either wired, FIXME'd, or re-tensed.

Where the two diverge, which should move: in almost every case the code should move toward the design,
because the design is coherent and the gaps are wiring, not direction (the drift forks' unanimous finding:
no rogue architecture, no lost identity, under-wired organs). Two places the *design* should keep carrying
the honesty rather than the code merely catching up: CR1's novelty claim stays scoped until reinstatement
is demonstrated (the bench fork decides the representation, and the claim's strength rides the result), and
the native ceiling stays a baseline-indexed range until the reconciling cell runs (canon Section 11 owes
it, explicitly including the `202607220300` 2.0-2.2x cell in scope so the reconciliation cannot quietly
clamp). One divergence is a genuine open design question rather than lag: `vehje-schedule`'s own DESIGN
concedes the crate may dissolve if family passes turn out to be handler instances of the compile-stage
discipline, and the canon never mentions the crate at all; that is a fork to decide, not a stub to fill
(see open questions).

The bench-provenance boundary Dolan surfaced deserves flagging as a standing scope rule: the 8M/16M/50ms
fixpoint figure measured a standalone Zig prototype, not the shipped Rust `Engine`, and neither the canon
nor the crate DESIGN says so. The number is real and drove a real decision, but "measured" in the verb
ladder should carry *what artifact* was measured; an algorithm-level bench certifying a shipped-crate claim
is a category slip one notch below the Futamura-artifact lesson the doc itself teaches.

## Notes on the panel

**Wingo, sharpest technical catch.** He was the only reader who opened sk2 and sk17, and the finding that
the runtime has no continuation model at all (the CFG interpreter is a plain call-stack register machine
with no prompt or resumption; the multi-shot proof re-runs a pure comptime function; the two do not
compose) converts the soul's proudest line from settled to scoped. His "one typing story over three
operationally distinct discharges" is the correct sharpening of the handler discipline, and his product
framing (the interpreter is the product for the templating majority) should stay prominent forever.

**SPJ, model audit.** Verified citations against primary source instead of trusting the forks'
cross-audit, which is how F1/F2/F4 were caught at all. F3 (the shadows slip) is the most conceptually
important finding in the directory. F2's insistence on the range-with-index over the pinned scalar is
exactly the right statistical instinct. I find nothing wrong in his file.

**Leroy, deepest addition.** The trusted-base enumeration and the proven/demonstrated/measured/intended
verb ladder are the two highest-value process artifacts the panel produced; both are now in the canon. His
finding 1 (the canon garbled op's always-on depth-cap into an else-branch, weakening the ratified guard)
is a real fidelity bug caught between ratification and canon, and the shipped canon 2G carries the fix.
His finding 9 (the `hash.rs` header contradicting its own body) is a fresh instance of the
surface-vs-mandate species, and it is fixed in source.

**Fallin, best operational eye, and the best self-correction.** Findings A (NodeRef carries no arena
brand; the exact bug class Cranelift's typed entity handles and ISLE exist to make unrepresentable), B
(`fold_core` is a visitor, the IR-to-IR rebuild is a catamorphism; reusing the name will misdirect the
implementer), C (the CFG dead surface), and D (`binding_time_ceiling` as signature data) are all correct
and all load-bearing; his canon-recheck's compounding-drift chain (sketch proves execution, topic cites it
as the mechanism, CL verifies surface, two notes re-certify past the gap) is the best process finding in
the round and generalises beyond this project. He was wrong once, Finding F as stated ("the canon cites
retracted evidence"), and his own recheck finding 4 walks it back publicly with the honest three-number
status; the retake fork's baseline-indexing then closed what he correctly left open. That sequence, being
wrong in public and correcting it with more rigor than the original claim, is the discipline working.

**Rompf, the right frame, thin at the edges.** The binding-time pinning is the panel's single most
valuable frame and it independently reconstructs the canon's own axis, the strongest corroboration on
record. The `Rep[T]`-vs-`T` reading of `Knowledge`, the thermometer-encoding finding, and the manifest-hash
unsoundness catch (addendum finding 0, a real would-be bug) are all first-rate. Thin: he cited the retracted
1.0-1.5x as settled, proposed a fuel cap without checking the round's recorded kill of cap-termination
(the negative fork had to supply the provenance), and missed that `vehje-fixpoint` already existed as the
designed home for the stratum he framed as an open unification question.

**The worker forks.** The negative fork's terminator reconciliation and its half-married-vs-already-
unseated distinction are genuine epistemic sharpenings; its four catches against the positive fork were all
verified and conceded. The changelist-drift fork's "surface existence standing in for mandate satisfaction"
is the correct root-cause of both live wounds and belongs in the CL-verifier design. The retake fork's
bench-supremacy re-adjudication (threading scoped, eqsat out, direct isel first) is what the data says,
though its "Fallin cited a doubly-stale intermediate" was itself over-confident per SPJ F2. The idealistic
synthesis's one-reducer thesis is attractive and I mostly believe it (it is the LMS collapse, and it is how
a well-factored staged IR should work: one catamorphism, discharge polymorphic in output kind), but it was
correctly kept out of canon as a forward proposal; it moved faster than the evidence twice (the shadows
framing the positive fork then copied, and asserting single-arena "dissolves" branding while the arena fork
was still open, caught by SPJ). The second-direction audit (Dolan) found the entire class the FIXME walk
structurally cannot, and its item 17 (the canon's Section 11 stale against a commit landed 23 minutes
before the canon's own timestamp, in a document claiming source-verified-at-authoring) is the neatest small
irony in the round: even the canon needs the discipline it prescribes.

## Open questions for the maintainer

1. **Status of the two candidate documents now that the canon exists.** They are still named
`canonical_candidate_` and remain the most detailed statement of several sections the canon compresses.
Options: mark them explicitly superseded-by-`202607241545` (one banner line each), or leave them as-is as
panel artifacts. Tradeoff: a future reader who greps into the candidates first inherits the four
pre-correction defects (the shadows framing, the pinned 1.5x, the flat CR1, the eqsat filing) unless the
supersession is marked; against that, editing panel artifacts after the fact cuts against the
audit-trail-immutability discipline. A banner is not a rewrite, but the call on where the line sits is
yours.

2. **`vehje-schedule`: build it to its DESIGN, or mark it contingent.** The crate is a stub; its DESIGN
describes a working DAG scheduler; its own text concedes it may dissolve if family passes become handler
instances of the compile-stage discipline (the one-reducer direction); and the canon never mentions it.
Building it now is cheap (`arvo-graph::topo_sort` exists and the FIXMEs name the wiring) but bets against
the unification; marking its DESIGN contingent keeps the option open but leaves a finished-looking doc on a
crate whose reason to exist is unproven either way. The second-direction audit ranked building it first by
unblock-value; the idealistic synthesis implies it dissolves. Both cannot be right.

3. **Bench provenance labelling.** Several Section 7 numbers measure standalone prototypes (Zig
`reach.zig`) rather than shipped crates, unstated. Options: extend the verb ladder with the measured
artifact ("measured, prototype-level" vs "measured, shipped-crate"), or re-run the load-bearing cells
against the shipped types when consumers land. The first is a one-line doc convention; the second is real
work whose value depends on how far the Rust implementations can drift from the prototyped algorithm.

4. **When the CR1 representation bench lands, does the novelty claim ride the result?** If segmented
capture-and-reinstate proves out no-alloc, the "no shipping effect system does this" claim is earned; if
the winner is compile-time CPS/defunctionalisation, the mechanism is closer to known ground (a data
structure the interpreter walks) and the claim should narrow to the budget-fit-proof framing. Deciding in
advance which wording each outcome gets would prevent the claim quietly keeping its strong form under the
weaker mechanism.

5. **The drift test's cost of joint checking.** Canon Section 10 requires identity and mechanism checked
jointly, which is right; but the identity clauses include judgment calls ("lets a consumer's specific need
drive a design choice") that two honest readers can score differently, where the mechanism clauses are
mostly mechanical. Whether to leave the identity half as judgment or to add worked examples per clause (the
way the vocabulary rules carry DO/DON'T tables) trades canon length against inter-reader consistency; the
first genuine dispute over a clause will tell you which was needed.

6. **The soul pair's method as a standing template.** The positive/negative fork pair with mutual audit,
the commitment tiers, and the ratification inventory worked well enough here that the question is whether
to canonise the *shape* (for the next arc, in vehje or a sibling repo) or treat it as a one-off scaled to
this consolidation's stakes. The cost was real (a dozen artifacts, multiple expert passes); the yield
(every headline defect caught before canon) was also real. Worth a deliberate call rather than defaulting
either way next time.
