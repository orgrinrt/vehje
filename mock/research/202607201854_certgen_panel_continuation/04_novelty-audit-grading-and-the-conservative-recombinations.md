# Novelty audit of docs 01-03: where the continuation took the standard move, and the recombinations that would supersede it

**Date:** 2026-07-20
**Phase:** panel continuation (worker-fork audit, reports once, settles nothing)
**Role taken:** a mathematically-aligned compiler-theory veteran with a stated distaste for taking the field's
default answer. The conviction driving this pass: excellence is not found by obeying tradition and not by
ignoring it, but by composing established pieces until a new shape falls out that supersedes the pieces it was
built from. Every hole below therefore arrives with concrete recombinations, several per item, none binding, as
options for op and the maintainer to choose among or reject.
**Reads:** `01_theory-citation-verification-and-adjacent-threads.md`, `02_settlements-and-next-architecture.md`,
`03_core-nodes-typesystem-and-zig-bridge.md`, and through them the round topics (1315, 1316, 1513, 1534, 1627,
1845), the panel (`../202607201641_certified-generation-panel/01`..`08b`), and the synth domain docs
(`../202607201618_synth_*`).

## The stance, and the one-paragraph verdict

Docs 01-03 are careful, well-cited, and correct on their own terms. That is also the complaint. Doc 02 states
its own posture plainly: the seven calls "settle as seven blesses, because the panel posed them correctly and
the field has standard answers." A design that congratulates itself on reaching the field's standard answer has
told you exactly where to look for the missed idea. The continuation banked, in doc 01, precisely the theory
that would let the design supersede its own frame (reachability types, gradual effects, the region-effect
unification) and then, in docs 02 and 03, used that theory conservatively: reachability got filed as a
"mutable-consumer fallback" rather than promoted to the primary lattice; the three axes got unified only by
analogy ("each is an instance of one frame") rather than by construction; and the single trust seam that
actually leaks was left unnamed while a differential gate was placed on the seam that does not. The verdict:
nothing in 01-03 is wrong, and three things in them are a generation behind the recombination that is sitting
in the banked citations. The centerpiece recombination (Part 1) collapses the design's three separate lattices
into one graded structure with one soundness argument. The eight sharper holes (Part 2) each carry two or three
concrete options. Part 3 banks the theory the recombinations need and that 01-03 did not reach for. Part 4 is
the fairness pass: the calls that are genuinely right and should not be touched.

## Part 1: the centerpiece. Three axes, three lattices, one graded (co)modal system

The arc now carries three monotone structures and runs a monotone analysis over each:

- a binding-time lattice (`Lang < Script < Runtime`, doc 02 talk 1), over which every input and effect is
  discharged at the earliest level its knowledge is available;
- an effect lattice (the `Permits` set with `Reads`/`Writes` over environments), over which inclusion is an
  order check and gradual effects give a derivation;
- a lease/region lattice (lexical depth, a finite chain of height equal to the depth cap, doc 02 call 4), over
  which the lease inference is monotone dataflow converging to a least fixpoint.

Doc 02's call 6 and doc 08 unify these by *analogy*: each axis is "one instance of staged gradual verification,
labelled by which fragment it occupies." That is a filing system, not a construction. A filing system gives you
one vocabulary; it does not give you one theorem. The mathematics on the table gives you one theorem, and
neither continuation doc reached for it.

**The recombination.** Binding time is a graded necessity modality (Davies-Pfenning's modal analysis of staged
computation, POPL 1996 / JACM 2001, is the modal reading of exactly the two-level split doc 08 modelled with
Nielson-Nielson two-level lambda; grade the `□` by the binding-time lattice and the multi-level structure of
talk 1 is a graded `□`). Effects are a graded monad (Katsumata, parametric effect monads, POPL 2014; the
`Permits` inclusion is grade ordering). Leases-as-usage are a graded comonad, that is, a coeffect
(Petricek-Orchard-Mycroft, coeffects, ICFP 2014). And the paper that unifies the last two already exists:
Gaboardi, Katsumata, Orchard, Breuvart, Uustalu, "Combining Effects and Coeffects via Grading," ICFP 2016. A
working language carries all of it at once: Granule (Orchard-Liepelt-Eades, ICFP 2019) has graded modal types
where the same grading discipline expresses effects, resource usage, and information flow as coordinates of one
type.

Under this recombination the design has one judgment, `Γ ⊢ e : A` where every variable in `Γ` carries a coeffect
grade (its lease/usage) and the conclusion carries an effect grade (its `Permits` obligation) and a modal grade
(its binding time), and the single soundness statement is the graded adjunction that ties the comonad to the
monad. The three "separate proofs" doc 02 talk 8 sketches (T1 staging, T2 gradual guarantee, T3 region
soundness) become three projections of one graded metatheorem. The gradual guarantee itself is known to compose
with grading (the AGT machinery doc 01 verified is a Galois connection on a lattice; grades are lattice
elements; the gradual-graded combination is the natural pushout). The staged-gradual-verification frame is not
wrong; it is the shadow this graded system casts when you forget the grades and keep only the labels.

Why this supersedes rather than decorates. The current frame forces the maintainer to prove three metatheories
and *argue* they cohere. The graded frame proves one and *derives* the three. It also answers, for free, the
question docs 02 and 03 kept circling without naming: how do the axes interact when they meet on one form?
`Apply` consumes leases (coeffect), carries effects (effect grade), and sits at a binding time (modal grade);
under three separate lattices their interaction is a case analysis nobody has written; under one graded judgment
their interaction is the typing rule for `Apply`, which the graded calculus gives you once.

Options, none binding:

- Option 1A, adopt grading as the proof-document spine (supersede doc 02 talk 10). Keep the surface prose plain
  per the workspace vocabulary rules, but make `λ_veh` a graded calculus and prove one graded soundness theorem
  with three corollaries. Cost: the maintainer must read Gaboardi-2016 and Granule. Payoff: one metatheorem, and
  the axis-interaction rules fall out instead of being argued.
- Option 1B, adopt grading only for the two axes that already want it (effect and lease, via Gaboardi-2016's
  effect-coeffect combination) and keep binding time as a separate modal layer. Cost: two proofs instead of one,
  but the hardest interaction (effect-coeffect, which is call 1's whole subject) is unified. Payoff: less to
  swallow; still supersedes the effect-lease analogy with a construction.
- Option 1C, decline grading, keep the staged-gradual analogy, but *state* that the three-lattice coherence is
  assumed and not proven, so a later reader knows the frame is a filing system. Cost: honest weakness recorded.
  Payoff: zero new theory to learn; this is the do-nothing option named so the choice is explicit.

The rest of Part 2 is written to stand whether or not grading is adopted, but several holes dissolve entirely
under Option 1A, and I mark those.

## Part 2: the sharper holes, each with options

### H1. The lease bit is mis-filed as an effect; it is a coeffect (dissolves under 1A/1B)

Call 1 and talk 2 derive the per-operand `link`/`consume` bit from the operand's op's declared *effect*
(`Writes<Host>`), and celebrate "collapsing two axioms into one." The collapse is real but the filing is
mathematically imprecise, and the imprecision costs resolution. An effect is what a computation does to the
world; the lease bit is a statement about how an op *uses one of its operands* (does it retain that specific
input past the call). Two ops can both carry `Writes<Host>` yet retain different operands: one stashes its first
argument, another its second. A single per-op effect flag cannot distinguish them; the lease bit is
per-operand. To make the effect carry per-operand resolution you must index it by operand, and a per-operand,
per-variable usage annotation *is* a coeffect (Petricek-Orchard-Mycroft), not an effect. So talk 2's
"derive the bit from the effect" is really "derive the bit from a coeffect the design has not named," and
filing it under the effect axis either loses resolution (one flag, many operands) or silently reinvents
coeffects inside the effect table.

This is not pedantry. The devil's advocate in talk 2 already noticed the symptom (the `Raw` op that stashes an
operand while declaring no effect) and called it "relocating the axiom." The relocation is exactly the tell: the
property does not want to live on the op (effect), it wants to live on the edge from op to operand (coeffect).

Options:

- Option H1a (dissolves under 1A/1B), model retention as a graded coeffect: each operand position of each family
  op carries a usage grade in a small lattice whose relevant point is `retained` versus `consumed`, and the lease
  bit *is* that grade, not a derivation from a coarser flag. Gaboardi-2016 then ties the coeffect grade to the
  effect grade so the "region and effect are one system" intuition (Talpin-Jouvelot) becomes a proven
  effect-coeffect adjunction rather than a slogan. Granule is the existence proof that this typechecks.
- Option H1b, keep the effect-derivation but make the effect *operand-indexed* explicitly (`Writes<Host>` becomes
  `Retains(i)` naming the operand position), which is coeffects in effect clothing but keeps the design's
  vocabulary. Cheaper to adopt, less clean, but honest about per-operand resolution.
- Option H1c, keep call 1 exactly as written and accept the resolution loss: a `Writes<Host>` op conservatively
  marks *all* its operands `link`. Sound (over-approximation, promotion is always safe per Cousot-Cousot), but
  imprecise, and the imprecision is exactly the "precision loss" Lua was promised. This is the do-nothing option;
  it is sound, it just leaves precision on the table that a coeffect grade would recover.

### H2. Translation validation is on the wrong seam; the Rust-to-Zig emitter is the unnamed TCB item

This is the sharpest fair catch in the audit, and docs 01-03 cite the exact precedent that identifies their own
gap without connecting it. Doc 02 call 6 notes that CompCert "trusts its unverified pretty-printer and
assembler rim." Vehje has the identical rim and does not name it. The certified-generation story is: rustc types
the Rust data and generator (certification 1), the Zig compiler types the specialised engine (certification 2).
Between them sits a step nobody certified: the Rust code that *serialises* the family table, effect masks, and
lease schema into Zig `comptime` const source text. rustc proves that serialiser returns a well-typed `String`;
it does not prove the bytes it returns *decode, on the Zig side, to the same table rustc validated*. That
emitter is vehje's unverified pretty-printer rim, and it is the one seam where a bug corrupts the guarantee
silently, because both certifications pass: rustc is happy (the Rust table is consistent), the Zig compiler is
happy (the emitted consts are well-formed Zig), and the emitted consts are nonetheless a faithless image of the
Rust table.

Meanwhile doc 02 talk 4 places a differential gate on the *kernel* (run it at two binding times, assert equal
output). That gate validates the mix equation operationally, which is valuable, but the mix equation was never
the leaky part: the kernel is one source compiled twice, and the two binding times agreeing is close to a
tautology under the staged frame. The trust actually leaks at the handoff, and there is no gate there.

Options:

- Option H2a, round-trip translation validation on the handoff (the gate that is actually needed). At our build
  time, have the Zig side re-serialise the consts *as it parsed them* back into the canonical wire form, and a
  build gate diffs that against the Rust emission byte-for-byte (or by content hash, Dhall-style). This is
  Pnueli-Siegel-Singerman translation validation applied where the translation happens, not on the kernel. It
  makes the emitter untrusted-but-checked instead of trusted-and-unchecked.
- Option H2b, one declarative source, two projections (the lens move). Neither Rust nor Zig owns the language
  definition; both project it. The language definition is a single declarative artifact (a schema), the Rust
  typestate is one projection of it and the Zig consts are another, and a checker proves the two projections
  agree by construction (bidirectional transformations / lenses, Foster et al., TOPLAS 2007). Then there is no
  emitter to trust because neither side serialises the other; the synth docs already found "one truth, many
  projections kept consistent by a generator" is the recurring winning pattern (index doc, cross-cutting thread
  4), and this applies it to the handoff itself.
- Option H2c, a self-checking typed manifest (PCC on the data blob). Rust emits not raw consts but a manifest
  carrying its own structural checksum and shape descriptor, and Zig `comptime` verifies the manifest against the
  descriptor before specialising, refusing to build on a mismatch. This is proof-carrying code (Necula, which
  doc 02 already cites for the decidable axes) applied to the data bridge rather than to the script. Lighter than
  H2b, stronger than H2a's after-the-fact diff because the check is structural and at specialise time.

Any of the three closes a real hole. Doing none leaves the design with a certified generator whose one
uncertified step is the step that turns a proof into bytes.

### H3. Binding time is committed as a chain but the bundling case proves it is a lattice

Doc 02 talk 1 fixes three binding times as a total order `Lang < Script < Runtime` and names relative binding
times as the escape if a fourth is ever needed. Doc 03 piece 8 then quietly contradicts this: the "Script" level
bifurcates by *where* the script becomes known, our build (bundled) versus consumer load (arriving), and those
are not later-and-earlier on one chain, they are two different sources of knowledge that can each be present or
absent independently. A bundled script is known at our build; an arriving script is known at the host; a script
can in principle be partly bundled (a template) and partly arriving (its data). That is a partial order over
*sources of knowledge*, not a chain of times.

The math-clean move is to stop indexing by "time" (a chain) and index by "which sources of knowledge a value
depends on" (a lattice: the free distributive lattice, or just the powerset, over the sources
{language-author, bundler, host-loader, runtime}). Each input and each effect is indexed by the join of the
sources it needs, and "discharge at the earliest point all your sources are available" is a well-defined lattice
operation rather than a case analysis, and it handles bundling, partial bundling, and any future source without
a fourth-level patch.

Options:

- Option H3a (composes with 1A), index binding time by the join-semilattice of knowledge sources rather than a
  chain. Multi-level BTA already lives on a lattice (Glück-Jørgensen, which doc 02 cited but then flattened to a
  chain); this uses the tool as intended. Under Option 1A the modal grade is exactly this lattice.
- Option H3b, keep the three-level chain for the runtime enum (cheap, closed, fast) but let the *proof* range
  over the source lattice, the same closed-enum-in-types / rich-structure-in-proofs split doc 02 talk 1 already
  proposed for a different reason. This gets the lattice's correctness without the runtime cost, and bundling
  stops being a special case in the proof.
- Option H3c, keep the chain and treat bundling as a pre-pass that erases the bundler source (a bundled script is
  "compiled to" an arriving script known early), so the chain survives by construction. Cheapest; it works only
  as long as no value is *both* bundler-dependent and host-dependent, which a template-plus-data script violates,
  so this option carries a documented restriction.

### H4. The cross-chunk lemma and the oversized-subtree proof are one linear-continuation lemma

Call 5 and talk 3 carry two proof obligations for the streaming spine: the cross-chunk lemma (a cross-chunk
reference is consumed-or-promoted by construction) and, separately, the oversized-subtree continuation-is-consumed
proof that talk 3 admits is "genuinely hard" and might fail, forcing a hard size limit. Treating them as two
proofs is the tell that a unifying structure was missed. Both are the same statement about a producer that emits
a large structure incrementally while keeping a bounded frontier: that is a *coroutine*, and the frontier-bound
property is *linearity of its continuation*. Model the streaming producer as a defunctionalised one-shot
coroutine whose suspended frames are exactly the chunks (defunctionalization is already banked in doc 03 for the
kernel), and "a continuation is consumed once and never re-referenced" is one-shot use of a linear continuation
(one-shot continuations, Bruggeman-Waddell-Dybvig, PLDI 1996; linear continuation-passing,
Berdine-O'Hearn-Reddy-Thielecke, 2002). Under that framing the cross-chunk lemma and the oversized-subtree
proof are the same lemma (the continuation is linear, so no earlier frame is retained), proven once, over both
the whole-subtree boundary and the intra-subtree continuation chain.

Options:

- Option H4a, model the spine as a defunctionalised one-shot coroutine and prove frontier-boundedness once via
  continuation linearity. This subsumes talk 3's two proofs and, better, tells you *when* the oversized-subtree
  case is safe (exactly when the continuation stays one-shot) rather than leaving it as a hoped proof or a size
  limit.
- Option H4b, keep the two separate proofs but share their induction (talk 3 already notes both live in "the same
  store-typing induction over the post-order interval structure"). This is the conservative version: one
  induction, two corollaries, no continuation theory. Cheaper, still better than two independent proofs.
- Option H4c, accept talk 3's fallback (a hard subtree-size limit, oversized is a load error) and drop the
  intra-subtree continuation chain entirely. Honest and simple; it restricts the value shapes a consumer can
  produce, which for a streaming renderer like ikiuni is probably fine and for a batch code generator is probably
  not. Name the restriction and let the census decide.

### H5. The value-node-kind schema wants to be a projection, not a parallel closed set

Doc 03 piece 1 leans toward option (a): a closed value-kind set plus a `Raw`-value escape, "symmetric to the
node side." Symmetry is an aesthetic argument, and it hides an algebraic fact: a produced value is a *normal
form*, that is, it is built only from introduction (constructor) forms, which are a subset of the evaluation
algebra. You do not need to declare value-kinds as a second closed set and keep it in sync with the node set by
hand; you can *project* them as the introduction-form fragment of the combined core-plus-family algebra. One
declaration (the algebra), two projections: the eliminator side is the IR you fold, the introduction side is the
value domain you produce. This is the same "one truth, two projections" pattern the synth index named as the
recurring winner, and doc 03 reached for symmetry instead of projection, which means two things to keep in sync
instead of one thing viewed twice.

Options:

- Option H5a, derive value-kinds as the introduction-form projection of the node algebra (one declaration, two
  views). A family that adds a node constructor gets its value-kind for free; the certified generator makes the
  value union total by the same construction that makes the node union total, because they are the same
  declaration. This is doc 03's option (c) (reuse the format) done right: not "a value is a normal-form IR term"
  (which conflates elimination and introduction and strains the region tags, correctly rejected), but "a value
  is the introduction fragment," which is a clean subset.
- Option H5b, doc 03's leaned option (a): a separately declared closed value-kind set plus `Raw`-value escape.
  Simple, explicit, and it costs a hand-maintained sync between two closed sets that a later family extension can
  break silently. Fine if families rarely add value-carrying constructors.
- Option H5c, make the value domain fully family-parametric (doc 03's option (b)): each family contributes its
  value-kinds. Most expressive, most surface, and it pushes the totality burden onto per-family generation. Right
  only if the census shows families genuinely need bespoke value shapes (jomini's route-tagged blocks are the
  test case doc 03 named); the L2-vocabulary experiment doc 03 flagged is the thing that decides this.

### H6. Sharing, lease-meet, and streaming are one problem the docs treat as three

The value-arena is index-referenced, so a child can be shared (two parents point at one index): the arena is a
DAG, not a tree. Three separate discussions in the docs are actually one consequence of that:

- doc 02 call 1's lease inference computes a per-reference depth by "depth-min propagation" (a *meet* in the
  lattice);
- doc 03 banks Knuth attribute grammars and calls the lease a "synthesised attribute" computed "one pass, no
  extra walk";
- doc 02 call 5's streaming spine wants a bounded frontier over a post-order walk.

These collide. A synthesised attribute over a *DAG* (a shared node) is not single-pass unless you memoise the
shared node's attribute, and the shared node's lease is the *meet* of its referrers' leases, so you cannot
finalise it until you have seen all referrers, which over a *streamed, chunked* arena is exactly the cross-chunk
retention problem again. So sharing, the lease-meet, and streaming are three faces of one structure, and the
"one pass, no extra walk" promise (op's constraint, topic 1316) holds cleanly only over a tree, not over the
shared DAG the index-referenced arena permits. The docs never state whether the value-arena shares or is a pure
tree, and the answer changes the lease inference from single-pass to fixpoint-with-memoisation.

Options:

- Option H6a, forbid sharing in the produced value (the arena is a tree; a shared subvalue is copied). Then the
  synthesised-attribute single-pass promise is exactly true, the cross-chunk lemma is clean, and the cost is
  duplicated subvalues. For a streaming renderer this is often the right trade (locality beats sharing); for a
  value with large shared substructure it is a real cost.
- Option H6b, permit sharing but require every shared node to be a *chunk root* (promoted), so its lease is
  finalised at emission and a later referrer takes the already-fixed lease rather than lowering it. This makes the
  meet degenerate (a promoted node's lease is the outermost region, an over-approximation) and keeps single-pass
  streaming, at the cost of promoting every shared node one level higher than strictly needed. This composes
  perfectly with the chunker contract (talk 3) because "shared implies promoted" is exactly "cross-chunk implies
  root."
- Option H6c, permit general sharing and pay for it: the lease inference becomes a proper least-fixpoint over the
  DAG with memoisation (still monotone, still finite-height, so it terminates, but no longer one-pass), and
  streaming must retain a shared node's chunk until its last referrer is seen (a reference-count on shared chunk
  roots, dropped at zero). This is the most expressive and reintroduces exactly the retention the streaming spine
  was avoiding, bounded by the number of live shared roots rather than by depth. Name it so op can weigh sharing
  against the single-pass promise; do not let the arena be a DAG by accident while the proof assumes a tree.

### H7. The dismissed twelfth form may be the unification, not a niche

Doc 03 rules the eleven forms complete and names one plausible twelfth, a `Handle` for resumable algebraic
effects (Plotkin-Pretnar), then dismisses it because "none of the nine census consumers wants this today."
Census-driven form cuts are exactly the reasoning a novelty-first reviewer distrusts: the census is today's
consumers, and the effect axis already exists as first-class machinery. Here is the recombination the dismissal
skips. The design already has (a) an effect axis, (b) a host-call surface (runtime-env effects the host
services), and (c) macro expansion (a compile-stage rewrite). Algebraic effects and handlers are the single
theory that unifies all three: an effect is an operation, a handler is what services it, a host-call is an effect
handled by the host, and macro expansion is a *compile-stage effect handled by the runtime's compile stage*. The
twelfth form is not a niche future want; it is a candidate for the missing construction that collapses three
mechanisms the design currently treats separately (the effect set, the host boundary, the macro expander) into
one handler discipline. The `build-env` versus `runtime-env` effect split doc 02 talk 1 draws is precisely a
two-handler story (the compile stage handles build-env operations, the host handles runtime-env operations), and
naming it as such would let the same theory carry both.

Options:

- Option H7a, adopt algebraic effects and handlers as the *underlying* model of the effect axis (not a new
  surface form; a semantic foundation), so host-calls and macro expansion are handler instances. This is a big
  theoretical commitment and would want its own topic, but it is the recombination that supersedes "effects are a
  static set plus reduction" with "effects are operations, discharge is handling," which composes directly with
  the graded-monad reading in Part 1 (graded algebraic effects are a studied combination). Right if the census
  ever grows a consumer wanting user-defined control (generators, async, backtracking); ikiuni's future
  event/coroutine needs are the obvious pressure.
- Option H7b, keep the effect axis as a static set (the current design) but *record* that host-calls and macro
  expansion are the two handler instances a later algebraic-effects generalisation would unify, so the door is
  marked. Zero cost now, and it stops a future maintainer re-deriving the twelfth form from scratch.
- Option H7c, hold the line exactly as doc 03 has it (eleven forms, no handler, census-driven), and accept that a
  future consumer wanting resumable control reopens the core cut. Defensible; just do not present the census as
  proof the form is unnecessary, only as proof it is unnecessary *now*.

### H8. "Certified" is scoped by a binary; a grade would let it be a dial

Doc 02 call 6 scopes "certified" with a hard partition: structural properties are certified, semantics are
tested, termination is not claimed. That partition is honest and correct under the current frame. Under the
graded frame (Part 1) it stops being a binary and becomes a dial: each property carries a grade saying *how much*
of it is discharged statically versus by a dynamic residual, and "certified" is the top grade, "tested" a lower
grade, "dynamic-checked" a lower one still, all on one scale rather than three named buckets. This is what
quantitative type theory (Atkey, LICS 2018) and graded refinement give you: the assurance level is a resource
you can account for per property, and the guarantee inventory becomes a computed grade vector rather than a
hand-curated list (which is what doc 02 call 6 asks for anyway when it says "make it mechanical by labelling each
axis"). The grade *is* the mechanical label, with an algebra behind it.

Options:

- Option H8a (dissolves under 1A), express the guarantee inventory as a grade per property on one assurance
  lattice (static-certified, structurally-certified, re-proven-per-instance, dynamically-checked, tested,
  unclaimed), so "certified" is a point on a scale with an algebra, and composing two properties composes their
  grades by the lattice meet. This makes call 6's "mechanical labelling" literally mechanical.
- Option H8b, keep the three named buckets (certified/tested/unclaimed) but define them as the top, middle, and
  bottom of an implicit order, so at least the ordering is stated and a fourth assurance level (the re-proven
  decidable axes, which doc 02 already treats as *stronger* than merely certified via PCC re-checking) has a home
  instead of being an aside. Cheap; it just names the order the buckets already have.
- Option H8c, leave call 6 as written. It is correct and honest; the only loss is that "certifying compilation
  stronger than certified for the decidable axes" (doc 02's own observation) sits awkwardly outside the
  three-bucket partition it should be the top of.

## Part 3: theory to bank that 01-03 did not reach for

Doc 01 banked reachability types, gradual effects, mechanised staging, and the region-effect unification, which
are the right foundations and are correctly used for what they were used for. The recombinations above need the
following, none of which appears in 01, 02, or 03. Each is real, reachable, and attributed; where a
recombination above leans on it, the hole is named.

- Graded monads and parametric effect monads. Katsumata, "Parametric Effect Monads and Semantics of Effect
  Systems," POPL 2014. The effect axis as a graded monad; the `Permits` inclusion is grade ordering. Lands on
  Part 1 and H8.
- Coeffects. Petricek, Orchard, Mycroft, "Coeffects: A Calculus of Context-Dependent Computation," ICFP 2014,
  and Petricek's thesis. The correct home for the lease/usage bit (H1): a per-variable graded context
  annotation, which is what link/consume actually is.
- Combining effects and coeffects via grading. Gaboardi, Katsumata, Orchard, Breuvart, Uustalu, ICFP 2016. The
  paper that unifies effect and coeffect grades in one system; it is the construction that turns Talpin-Jouvelot's
  "region and effect are one system" slogan (which doc 01 verified) into a proof. Lands on Part 1 and H1.
- Graded modal types in a working language. Orchard, Liepelt, Eades, "Quantitative Program Reasoning with Graded
  Modal Types" (Granule), ICFP 2019. The existence proof that effects, resource usage, and information flow
  coexist as grades in one typechecker; the concrete evidence that Part 1 is implementable, not just provable.
- Modal analysis of staged computation. Davies, Pfenning, "A Modal Analysis of Staged Computation," POPL 1996 /
  JACM 48(3), 2001. The modal (`□`) reading of binding time that doc 08 approached with Nielson-Nielson
  two-level lambda; grading the `□` by the binding-time lattice is the modal-grade of Part 1 and the clean form
  of the three-level chain (H3).
- Quantitative type theory. Atkey, "Syntax and Semantics of Quantitative Type Theory," LICS 2018; McBride, "I
  Got Plenty o' Nuttin'," 2016. Resource-quantity-annotated types; the frame for reading the depth cap and the
  lease as quantities and for grading assurance (H8).
- One-shot and linear continuations. Bruggeman, Waddell, Dybvig, "Representing Control in the Presence of
  One-Shot Continuations," PLDI 1996; Berdine, O'Hearn, Reddy, Thielecke, "Linear Continuation-Passing," HOSC
  2002. The theory that collapses the cross-chunk lemma and the oversized-subtree proof into one linearity
  statement (H4).
- Bidirectional transformations / lenses. Foster, Greenwald, Moore, Pierce, Schmitt, "Combinators for
  Bidirectional Tree Transformations," TOPLAS 29(3), 2007. The "one declarative source, two consistent
  projections" mechanism for the Rust-to-Zig handoff (H2b) and the node/value projection (H5a).
- Algebraic effects and handlers. Plotkin, Pretnar, "Handlers of Algebraic Effects," ESOP 2009; Bauer, Pretnar,
  "Programming with Algebraic Effects and Handlers," 2015. The theory under H7, the candidate unification of the
  effect set, the host boundary, and macro expansion; graded algebraic effects are the composition with Part 1.

The through-line: doc 01 banked the horizontal theory (each axis's own modern lineage) and the recombinations
here are the vertical theory (the one framework that indexes all axes at once, grading), which is the shape the
persona's mandate points at, compose the pieces until a new shape supersedes them.

## Part 4: what I would not touch (the fairness pass)

Novelty for its own sake is the failure mode this persona is one step from, so the calls that are genuinely
right get said plainly:

- The certified-generation resolution itself (1627) is sound and should not be reopened. The recombinations above
  sharpen how it is proved and where its trust seam is (H2), not whether it holds.
- Call 3 (spill as a host-lent capability) and call 4 (the depth cap as finite lattice height) are correct as
  written; the capability model and the finite-height convergence argument are the right theory used the right
  way. H6 interacts with call 4's frontier bound but does not contradict it.
- The reachability-types bank (doc 01 B1) is the single most valuable thing in the three docs, and the only
  complaint is that docs 02 and 03 used it as a fallback rather than promoting it; that is a usage note, not an
  error.
- The eleven Core forms hold (doc 03). H7 is not "a form is missing"; it is "the dismissal reasoning is weak and
  a deeper unification is on the table." The forms are right; the argument for their completeness should lean on
  algebra, not on the census.
- Experiment-first (call 7) is correct, and the L2-primitive-vocabulary enumeration doc 03 named as the pivotal
  next experiment is the right pivotal experiment; nothing above changes that it should run before the bridge
  commits.

## Closing

The three continuation docs did the hard, honest work of verifying the trail and settling the calls, and they
banked, in doc 01, exactly the theory that would let the design supersede its own frame. Then they spent that
theory conservatively: the three axes were unified by analogy where grading would unify them by construction; the
lease bit was filed as an effect where it is a coeffect; the translation-validation gate was placed on the kernel
where the trust actually leaks at the emitter; the streaming spine was given two proofs where one linear
continuation carries both; the value domain was given a parallel closed set where a projection would keep one
truth; sharing was left implicit where it changes the lease inference from single-pass to fixpoint; and the
twelfth form was dismissed by census where algebraic effects might be the missing unification. None of these is a
correction; each is a recombination of theory the maintainer can adopt, partially adopt, or reject. The single
thesis worth carrying out of this pass: the design's five axes are not five instances of one filing system, they
are five grades of one graded (co)modal type system, and proving that one system is the move that supersedes the
staged-gradual frame instead of restating it. Everything else here is an option hanging off that observation or
standing beside it.
