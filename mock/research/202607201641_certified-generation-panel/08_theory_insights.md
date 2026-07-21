# A mathematical model for the panel's converging direction (theory insight, audit 8)

**Task:** read the certified-generation panel in order (01 through 07), find the mathematical and CS-theory
paradigms the panel is iteratively converging toward, and propose a unifying mathematical model (combining
theory rather than adopting one wholesale) that supports and could help prove or otherwise enforce that
direction, with citations, including where the synthesis is novel. This is a worker-fork deliverable: it
reports once and stops. It does not re-settle the shape; it names the frame the shape already lives in and
gives the proof document a formal target.

## What the panel converges to, stated as one object

Strip the engineering and the panel is describing a single mathematical object. There is a two-input engine
`E(language_definition, script)`. There are three binding times at which parts of its input become known:
language-definition time (dev), bundled-content time (our build), and script-arrival time (the consumer's
load). For each safety property the design cares about (family inclusion, effect inclusion, non-null, lease
safety, bounds), the design discharges that property *at the earliest binding time at which the relevant input
is known*: as a static compile-time proof where the program is known early (folded to a Zig `@compileError`),
and as a runtime check where it is known late (a total, type-checked traversal at load). The specialiser
(Zig `comptime` plus the native build step) is the machine that performs this staging, and the runtime residual
is a bounded, single-pass, no-alloc check. Every correction the four expert audits landed, and every one of the
compiler-theory veteran's seven answers, is a statement about this object. The claim of this document is that
the object already has a name in the literature, that naming it gives every axis one soundness-argument shape
instead of five ad hoc ones, and that the specific instance vehje is building is a novel point in that space.

## The unifying frame: staged gradual verification with binding-time-indexed obligation discharge

The frame is the composition of three established theories, and the composition is the contribution.

**1. Gradual verification / hybrid type checking supplies the static-where-known, dynamic-where-not
stratification.** Flanagan's hybrid type checking (POPL 2006; Knowles and Flanagan, TOPLAS 2009) enforces a
specification statically where it can be proven and inserts a dynamic cast where it cannot, over a calculus of
refinement types (`{x:Int | x > 0}`, which is exactly "make illegal states unrepresentable" written as a
subset type). Bader, Aldrich and Tanter's gradual verification (VMCAI 2018) lifts this from typing to program
verification: partial or missing specifications are soundly backed by run-time checking. This is precisely the
panel's "one check, comptime-folded to a compile error where the program is static, a runtime scan where it is
a script." The armed generational-reference harness is not an ad hoc test rig; it *is* the dynamic residual of
the lease property in the gradual-verification sense, run under CI to falsify the static axioms.

**2. Abstracting Gradual Typing supplies the systematic derivation and the soundness discipline.** Garcia,
Clark and Tanter (POPL 2016) show that a gradual system is derived from its static counterpart through a Galois
connection between precise and imprecise types, and that the static and dynamic semantics both fall out of that
connection systematically. This matters here for a concrete reason: the panel keeps hand-deriving each axis's
static-plus-dynamic pair, and AGT says do not hand-derive it, derive it from the static discipline through the
abstraction/concretization pair, and the soundness (the gradual guarantee: a fully-static program behaves
exactly as its statically-checked self, a fully-dynamic program is safe, and adding precision never changes a
safe behaviour) comes for free from the connection. Vehje has a static discipline per axis; AGT is the tool
that turns each into its comptime-folds-or-runtime-checks form with a proof rather than an argument.

**3. Binding-time analysis and the Futamura mix equation supply the staging.** Consel and Danvy's line
(Tutorial Notes on Partial Evaluation, POPL 1993; Palsberg, "Binding-Time Analysis: Abstract Interpretation
versus Type Inference," ICCL 1994) establishes that the early/late division is itself an abstract
interpretation, and that specialising the engine to its early input is the first Futamura projection
(Futamura 1971; Jones, Gomard, Sestoft, 1993). The three-loci split Expert 4 forced is not an optimisation the
panel invented; it is binding-time analysis with three binding times instead of two, and the correctness of
"one inference, two binding times" is the mix equation: `[[kernel]](static, dynamic) = [[mix(kernel,
static)]](dynamic)`. The comptime fold of a check where the program is bundled content is exactly
specialisation of gradually-verified code, which is a named technique (rule-based program specialisation to
optimise gradually typed code, KBS 2019): use the static knowledge to fold away the checks it discharges,
leave the runtime residual for what it cannot.

The three compose into one discipline: **for each safety property P (a monotone predicate over a lattice
`L_P`), derive its gradual counterpart via AGT's Galois connection, then discharge it at the earliest binding
time where the program is known (BTA / the Futamura mix), folding the static residual to a compile error and
leaving the dynamic residual as a bounded single-pass abstract-interpretation pass for the late-bound script
case; the specialiser is the cast-inserter; soundness is the gradual guarantee composed with the mix
equation.** That is staged gradual verification indexed by binding time, and it is the frame the panel is
converging on without having named it.

## The five mechanisms as five instances of the one frame

- **Family and effect inclusion** are `residual_set ⊆ declared_set` on a finite powerset lattice; the static
  side is a decidable set check, the gradual side is the runtime bitmask, and the "gradual effect system" is
  literally a published construction (Bañados Schwerter, Garcia, Tanter, "A Theory of Gradual Effect Systems,"
  ICFP 2014, which derives gradual effects by abstract interpretation over an effect lattice). Expert 3's
  "re-prove, not re-check, the decidable axes after decode" is the observation that for these two axes the
  predicate is decidable and can be re-established on the far side of the bridge, which is proof-carrying-code
  witness re-checking (Necula, PCC, POPL 1997) rather than mere translation validation.
- **Non-null** is a refinement/subset type discharged by Zig's optional type; it is the trivial refinement
  `{x | x ≠ ⊥}`, static by construction, no dynamic residual needed, which is the fully-static end of the
  gradual spectrum.
- **Dispatch totality** is a total function out of a finite coproduct; the universal property of the coproduct
  forces exhaustiveness, which is why Zig's exhaustive `switch` certifies it structurally. The initial-algebra
  reading (Meijer, Fokkinga, Paterson, "Functional Programming with Bananas, Lenses, Envelopes and Barbed
  Wire," FPCA 1991) is the same fact: a fold over an initial algebra is total exactly when it handles every
  constructor.
- **Lease safety** is the region-and-effect axis, and its static discipline is the degenerate, strictly-LIFO,
  totally-depth-ordered fragment of the Tofte-Talpin region calculus (Information and Computation 132(2),
  1997), whose soundness in the general case is hard but whose syntactic soundness was given in ordinary
  Wright-Felleisen progress-and-preservation style by Calcagno, Helsen and Thiemann (Information and
  Computation 173(2), 2002). The inference is a monotone dataflow analysis (Kildall, POPL 1973) over the
  complete lattice of lexical depths, which is a finite chain, so it has finite height (equal to the depth cap)
  and converges in one pass; promotion is widening to the top element, sound by the abstract-interpretation
  over-approximation theorem (Cousot and Cousot, POPL 1977); monotone annotations are order-preserving on the
  chain, sound by the join being an upper bound.
- **Bounds validation on the untrusted path** is the dynamic residual that has no static counterpart, because
  the script is maximally late-bound; it is the pure-dynamic end of the gradual spectrum, and its termination
  is the backward-link acyclicity argument below.

One frame, five instances, and the "certified vocabulary" op is asked to bless (call 6) becomes mechanical:
each axis is labelled by which fragment of the gradual system it occupies. Non-null and totality are fully
static. Family and effect are gradual with a decidable, re-provable dynamic residual. Lease is gradual with a
conditionally-sound dynamic residual (conditional on the per-family link-bit axioms). Bounds are pure dynamic.
Family semantics are outside the verification system entirely (tested). Termination is not in the system at
all (no totality axis).

## The formal skeleton the proof document should target

The veteran (audit 6, call 7) is right that the semantics skeleton is experiment-independent and can be drafted
in parallel. Here is the target, in the frame above:

- **A two-level (staged) core `λ_veh`** with the eleven ratified forms, a small-step operational semantics
  with an explicit region store indexed by a live-depth stack, and a binding-time annotation on each subterm
  (early / late) in the Nielson-Nielson two-level style.
- **Per axis, a static judgment plus its AGT-derived gradual judgment.** For leases: the store typing
  `Σ ⊢ e : τ @ d` with the invariant *reachability respects the lease order* (every pointee's lease depth is
  no greater than any pointer that reaches it), and its gradual counterpart with an unknown-lease element `¿`
  whose concretization is "any depth," from which the runtime cast (the load-time depth check, and its armed
  generational oracle) is derived.
- **Theorem T1 (staging soundness / the mix equation).** Specialising `λ_veh` to its early-bound input agrees
  with running the two-level term directly: `[[E]](L, s) = [[mix(E, L)]](s)`. Technique: Consel-Danvy /
  Jones-Gomard-Sestoft correctness of offline partial evaluation; MetaML annotation soundness (Taha and Sheard,
  PEPM 1997) for the erasure property that makes "one kernel, two binding times" agree.
- **Theorem T2 (the gradual guarantee, per axis).** The static and dynamic checks of each axis agree where both
  apply, adding precision never turns a safe run unsafe, and the pure-dynamic residual is safe. Technique: the
  AGT Galois-connection derivation (Garcia-Clark-Tanter) and the gradual-guarantee formulation (Siek, Vitousek,
  Cimini, Boyland, "Refined Criteria for Gradual Typing," SNAPL 2015).
- **Theorem T3 (region soundness for the lease axis).** Progress and preservation for `λ_veh`'s region
  fragment, giving *no dereference into a reset region*, conditional on the per-family link/consume bits being
  conservative. Technique: Calcagno-Helsen-Thiemann 2002 verbatim, restricted to the LIFO chain. The per-family
  bits are the axiom set (Tofte-Talpin's assumed-correct primitive effect signatures), discharged empirically
  by the armed harness.
- **The crux immutability lemma** (`a value created inside body A escapes A only through A's result value`) is
  the lemma that lets T3 range over the immutable core without alias analysis; **the cross-chunk lemma** is the
  same lemma at chunk granularity and, per Expert 4, is on the critical path, so it is a proof obligation the
  chunker discharges *by construction* (constrain chunk boundaries to whole-subtree, result-only escape) rather
  than a hoped fact; **backward-link acyclicity** (backward-only links imply a DAG imply single-pass
  termination) is the totality argument for the load-time checker and the reason the dynamic residuals are
  bounded single-pass fixpoints, hence no-alloc.
- **The bridge** is translation validation (Pnueli, Siegel, Singerman, TACAS 1998; Necula, PLDI 2000) with the
  PCC re-checking upgrade for the two decidable axes, and it is *validated, not verified*, with the
  irreducible trusted rim being the encoder (for the lease bit-table only, after the upgrade), the pinned
  toolchains, and the comptime-interpreter's agreement with compiled code (closed by the differential gate).

## What is genuinely novel, positioned honestly

The pieces are all published. Staged gradual typing exists (Staged Gradual Typing, GPCE 2025) and even combines
gradual typing with multi-stage run-time code generation. Gradual effect systems exist (ICFP 2014). Gradual
verification exists (VMCAI 2018). The region calculus and its syntactic soundness exist (1997, 2002).
Specialising gradually-typed code to remove runtime checks exists (KBS 2019). I did not, in this search, find a
unified account that does the following four things together, which is where vehje's novelty honestly sits:

1. **Stages gradual verification across a language-versus-script binding-time split**, so the "static" end is
   not a programmer's type annotation but the *language definition*, and the specialiser that inserts the
   dynamic residuals is a compiler-generator (the first Futamura projection), not a cast-insertion pass over
   one program. The static/dynamic frontier is a binding-time frontier, which folds AGT and BTA into one
   derivation; that composition is not, as far as this search found, a named result.
2. **Unifies three different verification disciplines (set inclusion, an effect system, a region system) under
   one staged-gradual-verification account discharged at the earliest binding time**, so all three inherit one
   soundness-argument shape (Galois connection plus mix equation) instead of three separate metatheories.
3. **Realises the dynamic residual as a bounded-frontier, single-pass, no-alloc abstract interpretation over a
   DAG**, tying the runtime-check memory bound to the region calculus's LIFO depth (the depth cap). Gradual
   verification's dynamic residuals are not usually analysed for no-alloc, single-pass, bounded-memory
   realizability; doing so, and proving the bound equals the lattice height, is novel engineering-theory that
   the streaming spine needs and that the field has not packaged.
4. **Names the composed central theorem** `mix ∘ gradual-guarantee`: specialising a gradually-verified program
   at an early binding time folds its static residual to a compile error and preserves its dynamic residual for
   the late-bound case, and the whole is sound because AGT gives the per-axis gradual guarantee and BTA gives
   the mix equation. Stating and proving that single composition is the design's central correctness claim, and
   it is the theorem the proof document should be built around.

None of this reopens the resolution; it is the mathematics *under* the direction the panel already chose. Its
value is threefold: it gives the seven blessed calls one shared justification (each is a fact about staged
gradual verification, not a separate gamble), it hands the proof document a formal target with the exact
prior-art proof technique named per theorem (so "proven once" becomes a bounded, known task rather than a
promise), and it locates the design's honest guarantee inventory as a labelling of each axis by its fragment of
the gradual system, which is exactly the scoped meaning of "certified" call 6 asks op to bless.

## The one place the model suggests an answer nobody in the panel reached

The panel treats the lease axis's per-family link/consume bits as an axiom set discharged empirically by the
armed harness, and treats family/effect inclusion as re-provable decidable predicates. The staged-gradual frame
suggests a stronger move available for the effect rim that the veteran identified as the real unsoundness
vector (a `consume` where a `link` was needed on a host-retaining operand). Because effects are already a
gradual effect system (ICFP 2014) derived by abstract interpretation over an effect lattice, and because
retention-through-a-host-boundary *is an effect* (`Writes<HostEnv>` in vehje's own parametrised effect
vocabulary), the link/consume obligation on an operand is not an independent bit to be gotten right by hand: it
is *derivable from the operand's declared effect*. An operand an op writes into host-retained state is, by the
effect classification the design already computes, an operand whose lease must be `link`. So the two axioms the
veteran flags as the trusted surface (the effect classification and the lease link bit) can be made *one*
axiom, with the lease bit a monotone function of the effect set rather than a second independent declaration.
That collapses the most realistic unsoundness vector into a single already-tested surface (the effect table),
and it is exactly the kind of "combine two theories rather than adopt one" move the task asked for: the region
system and the effect system are, per Talpin-Jouvelot's original type-and-effect region work, one system, and
vehje should declare the lease bit as an effect consequence, not a parallel input. This is a proposal, not a
settled result; it is the single place the mathematics offers an answer the panel had not reached, and it
should be tested against the census consumers' host-boundary ops before it is trusted.

## References

- Cormac Flanagan, "Hybrid Type Checking," POPL 2006; Kenneth Knowles and Cormac Flanagan, "Hybrid Type
  Checking," TOPLAS 2009. (Static-where-provable, dynamic cast otherwise, over refinement types.)
- Johannes Bader, Jonathan Aldrich, Éric Tanter, "Gradual Program Verification," VMCAI 2018. (Gradual
  verification; sound static/dynamic combination of specifications.)
- Ronald Garcia, Alison Clark, Éric Tanter, "Abstracting Gradual Typing," POPL 2016. (Deriving a gradual
  system from a static one via a Galois connection; the gradual guarantee.)
- Jeremy Siek, Michael Vitousek, Matteo Cimini, John Boyland, "Refined Criteria for Gradual Typing," SNAPL
  2015. (The gradual guarantee, stated as criteria.)
- Felipe Bañados Schwerter, Ronald Garcia, Éric Tanter, "A Theory of Gradual Effect Systems," ICFP 2014.
  (Gradual effects by abstract interpretation over an effect lattice.)
- "Staged Gradual Typing," GPCE 2025. (Gradual typing combined with multi-stage run-time code generation, the
  nearest prior art to vehje's staged-plus-gradual shape.)
- "Rule-based program specialization to optimize gradually typed code," Knowledge-Based Systems 2019.
  (Specialising gradually-typed code to remove runtime checks where static info exists: the comptime fold.)
- Yoshihiko Futamura, "Partial Evaluation of Computation Process," 1971; Neil Jones, Carsten Gomard, Peter
  Sestoft, *Partial Evaluation and Automatic Program Generation*, 1993. (The projections and the mix equation.)
- Charles Consel, Olivier Danvy, "Tutorial Notes on Partial Evaluation," POPL 1993; Jens Palsberg,
  "Binding-Time Analysis: Abstract Interpretation versus Type Inference," ICCL 1994. (BTA as abstract
  interpretation; the three-loci split as three binding times.)
- Walid Taha, Tim Sheard, "Multi-Stage Programming with Explicit Annotations," PEPM 1997. (MetaML; annotation
  soundness / the erasure property behind "one kernel, two binding times.")
- Mads Tofte, Jean-Pierre Talpin, "Region-Based Memory Management," Information and Computation 132(2), 1997;
  Cristiano Calcagno, Simon Helsen, Peter Thiemann, "Syntactic Type Soundness Results for the Region Calculus,"
  Information and Computation 173(2), 2002; Andrew Wright, Matthias Felleisen, "A Syntactic Approach to Type
  Soundness," Information and Computation 115(1), 1994. (The lease metatheorem's calculus and proof technique.)
- Jean-Pierre Talpin, Pierre Jouvelot, "The Type and Effect Discipline," LICS 1992 / Information and
  Computation 1994. (Type-and-effect and region as one system: the basis for deriving the lease bit from the
  effect.)
- Patrick Cousot, Radhia Cousot, "Abstract Interpretation," POPL 1977; Gary Kildall, "A Unified Approach to
  Global Program Optimization," POPL 1973. (Monotone dataflow, widening as safe over-approximation: the lease
  inference and promotion.)
- Amir Pnueli, Michael Siegel, Eli Singerman, "Translation Validation," TACAS 1998; George Necula, "Translation
  Validation for an Optimizing Compiler," PLDI 2000; George Necula, "Proof-Carrying Code," POPL 1997. (The
  bridge: validated-not-verified, with PCC-style re-checking for the decidable axes.)
- Erik Meijer, Maarten Fokkinga, Ross Paterson, "Functional Programming with Bananas, Lenses, Envelopes and
  Barbed Wire," FPCA 1991. (Initial-algebra folds and totality over a coproduct: dispatch totality.)
- Xavier Leroy, "Formal Verification of a Realistic Compiler," CACM 2009; Kumar, Myreen, Norrish, Owens,
  "CakeML," POPL 2014. (What semantic verification actually costs, and why vehje's "certified" must scope to
  structural properties.)
