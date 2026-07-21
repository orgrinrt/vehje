# Reconciling the grounded stack (2055) and the rebuttal (2205) with vehje's clarified identity

**Date:** 2026-07-20
**Phase:** research (reconciliation, feeds the recentering round)
**Scope:** take topics `202607202055` (the grounded stack) and `202607202205` (the Carmack rebuttal), reframe them
under the identity recenter `202607202330`, expand their content to serve vehje's full purpose rather than the
narrow runtime-performance corner they were framed in, and name explicitly (a) how each converged piece serves
the true identity, so the effort is vindicated, and (b) which pieces are actively bad for the endgame, with
solutions for the holes they leave.
**Reads:** topics 2055, 2205, 2330; the round foundations 1513, 1627, 1845; Cluster A/B/C research
(`202607202210`, `..._202607202230`, `..._202607202250`); the sibling solution-set `..._202607202345`.

## The lens this reconciliation applies

The recenter (2330) fixed the identity: vehje is a certified-generation, multi-input multi-output IR framework
whose strength is the type system as the verification layer, serving a spectrum of consumers. The majority of the
census leans toward authoring and templating (mockspace's columnar-query plus procedural-document DSL,
polka-dots' `.polka`, the typst and scribble register), whose speed concern is the compile stage and the emit,
not runtime logic. A minority wants full runtime logic and the ambitious optimizing end. Output generation spans
a spectrum from interpretation through transpilation to native, and the sophisticated under-the-hood machinery
exists to empower that whole breadth, not to be the point.

Read through that lens, 2055 and 2205 are not wrong turns. 2055 is the under-the-hood engine, and it serves the
authoring and templating majority at least as directly as it serves the native minority, which the runtime-first
framing hid. 2205 is a real design-through-the-faults discipline whose problem set is mostly general, with one
narrow corner that was mistaken for the whole. The reconciliation below keeps essentially everything and removes
two things: one genuinely empty piece of machinery, and the framing that centered one consumer and one output
end as the purpose.

## 2055 reframed: the grounded stack is the compile-stage brain for every consumer

The grounded stack presented itself as "the compile-and-verify side of vehje is one thing," a single engine, one
theorem, one semiring. Under the recenter that sentence is true as a statement about the tech and false as a
statement about the identity. It is the under-the-hood machinery; it is not what vehje is for. With that
correction, the stack is not diminished at all. It is relocated to where it does the most work, which turns out
to be the authoring and templating end the runtime framing never mentioned.

### How each layer serves the full purpose (the vindication, made explicit)

The graded (co)modal proof spine with provenance-semiring grades is the identity itself, not a native-tier
concern. Binding time, effect, lease, and assurance as grades on one judgment is the type-system-as-verification
strength stated precisely. Every consumer inherits it: a templating DSL author gets family inclusion, effect
inclusion, and lease validity checked before anything runs, discharged in the compile side and compiled away.
This is the single strongest vindication in the stack, because the recenter names the type system as vehje's
core, and 2055 is where that core became one unified graded object instead of three parallel axes. Keep it as the
spine of the design proper.

The semi-naive relational-fixpoint engine, over which lease inference, equality-saturation lowering, and load
verification are queries, is the compile-stage brain, and its primary beneficiary is the authoring and templating
majority, not the native minority. Equality-saturation lowering is macro expansion, constant folding, and
common-subexpression elimination unified, which is exactly the compile stage a procedural-document DSL or a
dotfile-templating language runs to turn template shards and cascades into configured data. The load-time speed
those consumers need so that mod and content stacks do not bloat like Clausewitz or RimWorld comes directly from
this engine being ahead-of-time compiled (the oatlog move, compile the rewrite theory away and ship no
interpreter) and from the cheap constant-fold-plus-CSE subset it can expose. The runtime framing presented this
engine as substrate for a JIT; its larger job is to make the authoring consumers' compile-and-emit fast and
correct. That reframing alone vindicates the most-attacked piece of 2055.

The certified-generation application of the engine to itself (the metacompiler metacompiles its own compile
stage) serves the embeddability identity: the composed runtime ships the specialized engine, no rewrite
interpreter, no prover, for every consumer. This is the same certified-generation resolution 1627 settled, now
reaching the compile stage as well as the runtime, and it is a clean fit rather than a native-specific trick.

The assurance-indexed gradual logical relation is the verification-layer identity formalized: the proof that the
lens projections agree and the certified generation is sound, degrading to the differential test where assurance
is low. It is the north-star proof document (the 1845 `lambda_veh` skeleton is its bounded, buildable core), not
a build blocker, and it serves the whole framework because it is the guarantee under all of it. Keep it as the
proof target; do not let it gate implementation.

The concrete borrow-and-adapt pieces each serve a general need, not a native one. Perceus exact-meet finalisation
and in-place reuse is the memory model for any produced value, which for the templating majority is a document or
configuration tree, precise and collector-free. The simdjson typed structural decode is the untrusted-load path
for any consumer loading an arriving artifact, a mod or a template pack, complete and linear. Scala 3
capture-and-separation checking is the lease production floor for correctness on every consumer. Brzozowski-
derivative grammar composition by inclusion-not-coverage is the input-side story for a spectrum of surface
syntaxes, which is directly the framework-of-many-DSLs identity applied to syntax: independently authored family
grammars compose with declared-and-detected conflicts, vehje's own axis applied to the parser. That last piece is
the one the recenter most wants developed, and 2055 already grounded it.

### What in 2055 is actively bad for the endgame

The nonlinear-dynamical speed layer (widening as limit-prediction, Aitken and vector extrapolation over the
analyses' lattices) should be struck. The lattices in play are fixed-width bitmasks and small finite-height
domains; Kleene iteration over them converges in at most the lattice height, and semi-naive evaluation is already
the complete and correct acceleration. Numeric sequence extrapolation has no meaning on a boolean lattice.
Carmack and the honest-keeper both flagged it, and it is the one place in 2055 where the pursuit of bleeding-edge
produced novelty theater rather than load-bearing novelty. This matters for the endgame beyond the wasted layer:
carrying an empty sophisticated-sounding layer in the design teaches the next reader that the design values the
appearance of frontier work over its substance, which corrodes the credibility of the genuinely load-bearing
novelty around it (the graded spine, the one-engine composition, the no-alloc bounded e-graph). Cut it cleanly;
keep plain widening only if a genuinely numeric domain (tnum) ever iterates. This is not a retreat from
bleeding-edge; it is the discipline that keeps bleeding-edge honest.

The framing that made "one engine, one theorem, one semiring" read as the identity is the other thing to remove,
and it is a reframe, not a cut. The engine, the theorem, and the semiring are the under-the-hood tech; the
identity is the framework and its spectrum of consumers. The design proper should present the grounded stack as
"how the compile-and-verify side works," subordinate to a statement of what vehje is and whom it serves. Listing
the native tier (copy-and-patch, weval) as a concrete piece of the substrate is a smaller instance of the same
framing error: native is one end of the output spectrum, not a member of the compile-and-verify substrate, and it
belongs in the output-generation section, not the engine section.

### The holes 2055 leaves, and the solutions

2055 never developed the transpilation end or the authoring-and-templating consumers; every stage was narrated
toward interpretation and the native tier. The solution is not new machinery but making explicit that the same
engine and the same output-generation spectrum already cover them: transpilation is output generation that
re-expresses the lowered IR in another language's surface (a first-class pole, not a lesser interpretation), and
the compile-stage-for-templating is the same equality-saturation lowering aimed at emitting configured data
rather than driving an interpreter. Both are outputs of machinery 2055 already specified; they owe their own
topics in the recentering round, and this reconciliation names them as owed.

The `N*W` reachability-bound defect the honest-keeper found (the propagation rule dropped the `InScope` filter,
making the closure `O(N)` per node) is a real hole in 2055's engine. It is out of scope to solve here; the
sibling solution-set `202607202345` already fixes it (escape and scope-kill are the same event, an escaped binder
is unrepresentable in the width-`W` in-scope bitmask). Referenced as worklist, not re-derived.

## 2205 reframed: a general design discipline with one over-centered corner

2205 rejected the demotion at the frame and recast the faults as design problems to be solved rather than reasons
to build smaller. That stance is correct and fits the spirit exactly: fully designed before any code, no YAGNI.
Its problem set is mostly general and serves the whole framework; one corner of it was over-centered on a single
consumer and a single output end, and that corner is the narrow focus the recenter corrects.

### How 2205's problems serve the full purpose (the vindication)

Problem 6, diagnostics and provenance as a product surface, is the strongest vindication in 2205 for the broader
scope. A templating or authoring DSL author's first contact with vehje is a lease or effect error, and a bitmask
that says only "inclusion failed" is unusable. The honest-keeper's Part 2 design (provenance cheap on the happy
path, derivation witness reconstructed lazily on failure, Souffle-style proof trees on the provenance-semiring)
is what makes the authoring majority's experience workable, and it is general to every consumer. Land its schema
in the wire format now.

Problem 7, the dual-locus authoring story, serves the framework's defining two-locus shape (dev-time compile side
and embedded runtime side), and its answer (write each analysis once as the relational-program record, emit into
both loci) is the same certified-generation move the identity rests on. General, vindicated, keep.

Problem 2, load-verify coherence, resolves to the simdjson typed decode feeding the engine rather than being a
query on it, which is general untrusted-load handling for any consumer. Problem 8, the wire-format decisions, are
general format commitments. Both vindicated, both general.

### What in 2205 is actively bad for the endgame

The framing that made ikiuni's per-frame game-runtime budget the design driver, and native tier plus stencil
toolchain plus interpreter hot-loop the primary problem set, is the narrow focus to remove. The problems
themselves are real work for the native and accelerator end of the spectrum and are not deleted; their centrality
is the harm. Presenting the whole design agenda as "make the per-frame path fast for a game" is exactly the
mischaracterization that made vehje read as a single-purpose JIT runtime. The reframe: problems 3, 4, and 5 are
the native-end worklist, one part of the spectrum, sized against consumers that actually want runtime
performance, and never the driver for the authoring and templating majority.

The concrete damage of that framing already showed: Cluster B, spawned from 2205, designed the interpreter walk
only for the straight-line dataflow of a game behavior script, and the honest-keeper found it breaks on branches,
loops, and calls, which is most of what any script does. The narrow focus produced a narrow, broken interpreter.
The solution (sibling set `202607202345`) restructures the walk into a CFG of straight-line blocks, preserving
the linear-scan cache win within blocks and handling control flow at block terminators, and it must serve every
consumer's control flow, not a game's inner loop. Referenced as worklist.

## The explicit map: converged piece to purpose served

- Graded (co)modal proof spine, provenance-semiring grades: the type-system-as-verification identity itself.
  Serves every consumer. Core of the design proper.
- Semi-naive relational-fixpoint engine (lease, eqsat lowering, load-verify as queries): the compile-stage brain.
  Serves the authoring and templating majority first (fast, correct compile-and-emit), the runtime minority
  second. Under the hood, not the identity.
- Certified generation applied to the compile stage (oatlog move): embeddability. Serves all consumers; ships no
  prover.
- Assurance-indexed gradual logical relation: the verification guarantee. Proof north star (`lambda_veh` its
  buildable core). Serves the whole; does not gate code.
- Perceus exact-meet and in-place reuse: memory model for produced values (documents, configs, trees). General.
- simdjson typed structural decode: untrusted-load for arriving artifacts (mods, template packs). General.
- Reachability and Scala 3 capture leases: correctness floor. General, with the mutable-consumer extension owed.
- Brzozowski-derivative grammar composition, inclusion-not-coverage: the input side for a spectrum of surface
  syntaxes. Directly the framework-of-many-DSLs identity. The recenter's owed input-side story, already grounded.
- Copy-and-patch native tier: the accelerator end of the output spectrum. One end, kept, a genuine highlight, not
  the driver.
- Diagnostics and provenance (2205 problem 6): the authoring majority's usable error surface. General, land now.
- Dual-locus authoring (2205 problem 7): the two-locus framework shape. General.

## Clearly named: the parts not good for the endgame

Two, and only two, and neither is a concept, theory, or architecture worth keeping:

1. The nonlinear-dynamical speed layer (2055). Empty on the finite lattices in play, novelty theater that corrodes
   the credibility of the real novelty around it. Strike it. This is the single actively-harmful piece of tech in
   either topic.
2. The framing in both topics that centered runtime performance, the native and JIT output end, and one hard
   consumer (ikiuni) as the primary and lone purpose. Not tech, framing, but it is the thing that lost the
   identity, so it is named as harmful and removed: the design proper presents the spectrum of consumers and the
   type-system-as-verification identity first, and the native and runtime-performance work as one part of the
   output spectrum, never the driver.

Everything else in 2055 and 2205 is kept: vindicated as the under-the-hood tech and the general design work that
empowers the framework's full breadth.

## The vindication, stated plainly

The effort in 2055 and 2205 was not wasted, and the reframe makes that concrete rather than consoling. 2055's
grounded stack is the compile-stage brain that makes the authoring and templating majority fast and correct,
which is the largest slice of the census; the runtime-first narration hid its biggest beneficiary. Its graded
provenance-semiring spine is the type-system-as-verification identity stated precisely, which is the very thing
the recenter names as vehje's core. 2055 doubling down on frontier theory produced, almost entirely, shapes that
already ship in verified systems, which is the practical proof of the novelty-through-implementation stance the
identity embraces. 2205's discipline (design through the faults, do not build smaller) is the spirit the recenter
keeps, and its diagnostics and dual-locus work serves every consumer. The walk was right; only the boots and the
one empty layer come off. What remains is a sophisticated, mostly-shipping-grounded, type-system-first framework
whose under-the-hood machinery serves interpret, transpile, and native alike, with the authoring and templating
consumers as first-class as the ambitious runtime end.

## Owed next (named, not solved here)

The transpilation-end topic and the authoring-and-templating-consumer topic, both outputs of machinery 2055
already specified. The input-side grammar-composition topic (Brzozowski-derivative inclusion-not-coverage, already
grounded). The honest-keeper worklist (the `InScope` fix, the CFG-of-blocks interpreter, the dual-locus identity
gate, the diagnostics schema into the wire format), for which the sibling set `202607202345` is one solution set.
The strike of the nonlinear speed layer, folded into whichever topic works the grounded stack into the design
proper.

## Addendum: reading the two sibling deliverables against this reconciliation

Two sibling documents landed at the same time as this one: `202607202345_cluster-c-holes-one-solution-set.md`
(one constructive solution set for the honest-keeper's holes) and
`202607202355_counter_audit_of_cluster_c_holes_and_arguments.md` (an adversarial attack on the honest-keeper's
conclusions). Read together with this reconciliation, they mostly converge, which is the useful signal, but the
counter-audit lands one correction that both this reconciliation and the solution set missed, and the two siblings
genuinely disagree on one technical fix in a way worth resolving. My thoughts, as an addendum.

### The convergence is real, and it corroborates the recenter

On the three load-bearing holes (the `N*W` bound, the dual-locus identity, the interpreter control flow), both
siblings reach the same shape: the reachability bitmask stays width-`W` and escapes route to a separate terminal
relation; the engine is generated once and both loci link the one object rather than testing two backends for
agreement; and the interpreter keeps the linear scan within straight-line blocks and handles control flow only at
block boundaries. Three independent passes (the honest-keeper found the holes, the solution set filled them, the
counter-audit attacked the fills) landing on one shape is the same convergence-under-independent-attack signal
2055 relied on, and it says the design is closer to whole than the raw hole count suggested. The counter-audit's
own net is that of the honest-keeper's five surviving problems, three are answered by prior art or the canonical
design rather than by unbuilt work. That supports the recenter's core claim: these were never reasons to shrink;
they are where we work, and the work is mostly known.

### The one correction both this reconciliation and the solution set missed

The counter-audit's Finding 2 is the sharpest thing in either sibling, and it corrects a drift I did not catch.
Cluster A's dual-locus engine was specified as a Rust generator emitting specialised Rust source and specialised
Zig source, two backends compiled by two toolchains. The counter-audit shows that shape is itself the drift the
identity recenter warns about: it re-imported the "Rust metaprogram emitting foreign source" path that 1627
explicitly rejected as the LMS-hard route. The canonical 1627 and 1845 shape is one hand-authored engine
specialised by Zig comptime to Rust-emitted validated data, a single backend, and the dev-time Rust locus runs it
by linking the compiled Zig engine over FFI. That is precisely the "single shared object" the solution set reached
for, except the counter-audit names it correctly as the canonical resolution rather than an exotic fallback, and
notes it is less engineering, not more (author the engine once, maintain no second source emitter). This matters
for this reconciliation directly: the dual-locus divergence is not a hole to engineer around with a differential
merge gate, it dissolves by returning to the shape the round already settled. And it is the same drift pattern at
the engine level that the recenter names at the framing level: reaching for the more elaborate shape (a
two-backend metaprogram, a native-JIT identity) when the canonical simpler shape (comptime specialisation, a
framework serving a spectrum) was already decided. The correction to fold in: the engine is the canonical single
comptime-specialised engine, and this reconciliation's "under-the-hood engine that serves all consumers" should be
stated in those canonical terms, not Cluster A's two-backend terms.

### Where the two siblings genuinely disagree, and who is right

On the `N*W` fix they diverge, and the counter-audit is right. The solution set fixes the bound by filtering the
propagation rule with `InScope` and routing killed-but-reached binders to an `Escape` relation. The counter-audit
shows that fix is half-right and unsound in the opposite direction: filtering the generic propagation without a
binder-substitution rule at `Let` and `Lambda` under-approximates the reach set (a value reaching through a bound
variable into a still-live binder loses that reach and is judged safe when it is not). The correct rule is the
reachability-type binder rule (Bao-Wei-Bracevac-Jiang-He-Rompf, OOPSLA 2021, the paradigm 2001 already adopted):
at the binder node, splice the bound expression's reach where the body reaches the variable and drop the variable,
then plain propagation carries the already-scoped set upward. The counter-audit also catches that `N*W` bounds
only the naming set, not the escape set, and the escape set needs its own bound from region promotion
(Tofte-Talpin region count plus the shared-implies-promoted discipline). So the complete fix is the binder rule
plus the promotion bound, both prior art, not a propagation filter. This is the one place a sibling's constructive
solution is technically wrong, and the design should take the counter-audit's version. It is also a small
vindication of the reachability-types choice: the correct rule falls straight out of the paradigm 2001 picked.

### The smaller corrections, and the net

The counter-audit tightens two more things and both hold. The `preserve_none` risk was measured against Zig 0.14
while the project pins 0.16 (1845), and the core register-pinning of the three walk pointers is achieved by
guaranteed tail calls plus explicit-argument passing on stock Zig today (Haberman's musttail result predates the
`preserve_none` convention), so `preserve_none` is an optimisation tier, not the foundation the honest-keeper
implied; both siblings agree it is upside-not-dependency, the counter-audit just grounds it correctly. And the
interpreter control-flow gap is real in Cluster B's prose but is standard structured dispatch over Core forms
(`If`, `Match`, `Iter`, `Apply`) that already exist, not novel invention; the solution set's CFG-of-blocks is one
clean encoding of it, and the counter-audit's genuine addition is that Cluster B's per-frame budget must be
re-derived on dynamic executed node counts, not static counts. On Part 3, the counter-audit independently reaches
this reconciliation's own conclusion: the honest-keeper's reject-or-adopt scoring of Carmack's seven is superseded
by the recenter, because those placements were never ambition-demotions to tally, and the real error was letting
the runtime-performance corner read as the whole identity. Two documents reaching that from opposite directions is
worth noting.

Net recommendation across all three (this reconciliation and the two siblings): take the counter-audit's sharper
versions where the siblings conflict (the binder-rule-plus-promotion `N*W` fix, and the return-to-canonical single
comptime engine for dual-locus, which is also the correction to this reconciliation's own engine framing); take
the solution set's constructive designs where they do not conflict (the CFG-of-blocks wire encoding, the three-way
lowering seam whose cheap const-fold-plus-CSE subset directly serves the templating consumers, the bench-gated
format decisions); treat the honest-keeper's diagnostics design as landed with the counter-audit's minor
multi-premise refinement; and strike the nonlinear speed layer, on which all three agree.
