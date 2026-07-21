# Adjacent theory for certified generation: compacted, one section per thread

**Date:** 2026-07-20
**What this is:** a compaction of the adjacent theory surfaced while re-verifying the certified-generation
arc's citations (`202607201854_certgen_panel_continuation/01_theory-citation-verification-and-adjacent-threads.md`).
One section per thread, conveying the whole picture of each and why it bears on vehje, dropping the exact
theorem statements and proofs. For the specifics, read the named sources. This sits beside the five
`202607201618_synth_*` domain docs and the two compactions as banked required reading; it is the theory the
paper trail did not yet carry but that the certified-generation direction leans on or is strengthened by.
Nothing here reopens the resolution; it is the mathematics under and beside the direction already chosen.

## Reachability types: the modern successor to region-and-effect, and the theory for the lease axis

Reachability types are the most directly applicable thread and the one that changes how the lease axis
should be grounded. The founding paper (Bao, Wei, Bračevac, Jiang, He, Rompf, "Reachability Types: Tracking
Aliasing and Separation in Higher-Order Functional Programs," OOPSLA 2021) attaches to each value the set of
variables it can reach, which is exactly the question the lease axis asks: does this reference escape the
scope that produced it, and if so, through what. Where Tofte-Talpin was built for a skeletal ML and reasons
about a stack of regions, reachability types are built for higher-order functional programs with capturing
and escaping closures, which is the shape a real consumer language actually produces, and they track aliasing
and separation rather than assuming a value is unshared. The lineage is active, not historical: polymorphic
reachability types add generics and freshness (OOPSLA 2024), "Free to Move" adds flow-sensitive effects
(2025), and "Escape with Your Self" adds sound bidirectional typing with avoidance (2025).

The avoidance result is the load-bearing one for vehje. The avoidance problem is the precise technical name
for the moment the lease design already identified: a value escapes a scope and the analysis must express its
type without the escaped binder, or fail. The lease topic decided that failure is a hard compile error that
forces an explicit lease; the avoidance work is the theory of exactly when that failure is unavoidable versus
when a wider lease can be inferred, which is the inference-ergonomics question the paradigm audit named as
the whole game. Because reachability types track aliasing and separation directly, they also cover the
mutable-and-aliased case (a Lua table shared across bindings) that the immutable-value narrowing does not,
so they are the principled fallback for the mutable consumer rather than a collector. The bearing on vehje:
ground the lease axis on this lineage, keep Tofte-Talpin and the Calcagno-Helsen-Thiemann syntactic
soundness as the metatheorem for the immutable LIFO core, and use the avoidance results as the theory of the
inference-failure boundary and the mutable-consumer fallback.

## Semantics-preserving multi-stage programming with let-insertion: the template for staging soundness

Two 2025 results give the staging-soundness theorem (the agreement between specialising the engine to its
early input and running the two-level term directly) a machine-checked, effect-aware template that is newer
and sharper than the MetaML citation the panel used. "When Do Staging Annotations Preserve Semantics?"
(2025) takes Amin and Rompf's untyped multi-stage lambda-up-down calculus, which is the calculus that models
LMS-style automatic let-insertion, restricts it to a typed two-stage setting, adds side effects, and proves
semantics preservation, characterising exactly when staging annotations preserve meaning and when they do
not. "Mechanised Semantics of Multi-stage Programming" (2025) gives a Rocq mechanisation of a core calculus
for compile-time and run-time staging with effects, establishing type soundness, elaboration soundness, and
phase distinction.

Two things make this more than a citation upgrade. First, lambda-up-down is the semantic core of LMS, which
the staged-metaprogramming domain doc already carries as prior art, so this closes the loop between the LMS
material already banked and the formal target the proof document needs. Second, let-insertion is not
incidental to vehje: it is the mechanism that keeps staged code sound in the presence of effects, and it is
exactly what the streaming spine and the macro expander need when a build-stage reduction has to hoist a
bound computation out of a subterm without changing when its effect happens. The bearing on vehje: anchor
the staging-soundness theorem on this mechanised, effectful, two-stage result rather than on MetaML 1997, and
note that a Rocq template exists if that theorem is ever to be machine-checked rather than paper-proved.

## Generic type-and-effect and its gradual form: the exact frame for the effect axis

Marino and Millstein's "A Generic Type-and-Effect System" (TLDI 2009) interprets an effect system as
privilege checking: every effectful operation requires a privilege to perform it, a type carries the
privilege set it needs, and the whole framework is parameterised over the privilege vocabulary and over how
privileges are adjusted and checked per context. That is vehje's effect axis stated in the literature's own
terms. A residual's effect set must be included in the target's permitted set, and the vocabulary of effects
is consumer-declared rather than fixed, which is the parameterisation Marino-Millstein makes central. The
gradual counterpart, needed for the runtime-arriving-script case where the effect set is only known late, is
"Gradual Type-and-Effect Systems" (Bañados Schwerter, Garcia, Tanter, Journal of Functional Programming
2016), which extends the Marino-Millstein framework with an unknown-effect element and derives the
static-plus-dynamic checking systematically, a subtler construction than unknown types in ordinary gradual
typing. The bearing on vehje: the effect axis has a single named home, and the citation chain for it is
Marino-Millstein for the privilege framing, the JFP 2016 paper for its gradual form, and, together with the
region-and-effect unification below, they are what let the design collapse the lease link and consume bit
into a consequence of the declared effect rather than an independent declaration.

## Region and effect as one system: the anchor for deriving the lease bit from the effect

The single most load-bearing move in the arc, that the lease link/consume obligation on an operand is
derivable from that operand's declared effect because retention through a host boundary is itself an effect,
rests on the fact that region systems and effect systems are one system. The precise anchor is Talpin and
Jouvelot's "Polymorphic Type, Region and Effect Inference" (Journal of Functional Programming 1992), which
reconstructs type, region, and effect in one inference algorithm over an implicitly-typed functional
language with imperative operations, treating regions as sets of possibly-aliased references and effects as
approximations of the imperative behaviour on those regions. The companion "The Type and Effect Discipline"
(LICS 1992) is the origin of the discipline; the JFP paper is where the three-way unification is explicit.
The bearing on vehje: this is the citation that turns the lease-from-effect proposal from a slogan into a
grounded move, and combined with the generic and gradual effect frames above it makes the lease bit a
monotone function of the effect set, collapsing the most realistic unsoundness vector into the already-tested
effect table.

## Space-efficient and evidence-based gradual typing: prior art for the bounded no-alloc dynamic residual

The claim that the design's bounded, single-pass, no-alloc dynamic residual is unpackaged by the gradual
field is too strong, and the correction is a gift rather than a loss. Space-efficient gradual typing is a
studied line: the space-efficient-coercion work that keeps the accumulated casts on a value bounded rather
than growing without limit, "Abstracting Gradual Typing Moving Forward: Precise and Space-Efficient"
(PACMPL 2021), and the recent evidence-based "Compiling Gradual Types with Evidence." The field has bounded
the space that gradual casts occupy. What it has not packaged is tying that memory bound to a region
calculus's LIFO depth, so that the no-alloc frontier of the runtime check is provably bounded by the lattice
height, which equals the depth cap. The bearing on vehje: narrow the novelty claim honestly to the
region-depth-bound tie, and treat the space-efficient-coercion techniques as borrowable machinery for the
streaming spine's dynamic residual rather than something to invent from scratch.

## Region inference leaks, and the sound hybrids that fix it: the lease fallback, named

The cautionary tale the paradigm audit told against the pure-static lease stance is real and worth carrying
as banked history. Pure region inference leaks: the ML Kit exhibited space leaks in its global region, with
measured memory a large multiple of the same programs run with regions plus a collector (a range from a few
times to over a hundred times, per Hallenberg and Elsman), and the sound fix was to add a garbage collector
back, later refined into region inference combined with generational garbage collection. Cyclone reached the
same conclusion from the safe-systems-language side and shipped a hybrid: lexical regions as the backbone,
plus unique pointers, plus reference counting, plus a collected heap, because static regions alone could not
carry every real program with overlapping, non-nested lifetimes. The modern statement of the same fallback
is the reference-capability line ("Reference Capabilities for Flexible Memory Management," 2023). The bearing
on vehje: the lease axis's compile-error-only stance is a real commitment with a known failure history, and
these papers name the escape valves should the pure-static lease prove too strict for a mutable consumer.
Combined with reachability types above, the coherent position is: immutable core stays pure-static; the
mutable or escaping fallback is a reachability-type or reference-capability discipline, not a collector.

## Lighter grounding

Three further pointers, banked without a full section. "Type, Ability, and Effect Systems: Perspectives on
Purity, Semantics, and Expressiveness" (2025) is a recent survey that situates effect systems, capabilities,
and region and reachability tracking in one frame, convenient when the effect and lease axes are introduced
together. Glück's "Is There a Fourth Futamura Projection?" and Williams and colleagues' "Revisiting the
Futamura Projections: A Diagrammatic Approach" are clean modern restatements of the projections and the mix
equation, useful companions to the classic Jones-Gomard-Sestoft reference. Palsberg's "Correctness of
Binding-Time Analysis" is the tightest anchor for the claim that the early/late binding-time split is sound,
tighter than the abstract-interpretation-versus-type-inference comparison paper that names the technique.

## Sources

Reachability types: Bao, Wei, Bračevac, Jiang, He, Rompf, OOPSLA 2021
(https://dl.acm.org/doi/10.1145/3485516); Polymorphic Reachability Types, OOPSLA 2024; Free to Move:
Reachability Types with Flow-Sensitive Effects, 2025 (https://arxiv.org/pdf/2510.08939); Escape with Your
Self: Bidirectional Typing with Avoidance for Reachability Types, PACMPL 2025
(https://doi.org/10.1145/3808335).

Staging: When Do Staging Annotations Preserve Semantics? Typed Semantics-Preserving MSP with Let-Insertion,
2025 (https://arxiv.org/pdf/2606.30854); Mechanised Semantics of Multi-stage Programming, PACMPL 2025
(https://doi.org/10.1145/3798260); Amin and Rompf's lambda-up-down / LMS staging semantics.

Effect and region: Marino, Millstein, A Generic Type-and-Effect System, TLDI 2009
(https://web.cs.ucla.edu/~todd/research/tldi09.pdf); Bañados Schwerter, Garcia, Tanter, Gradual
Type-and-Effect Systems, JFP 26, 2016 (https://www.cs.ubc.ca/~rxg/gtes.pdf); Talpin, Jouvelot, Polymorphic
Type, Region and Effect Inference, JFP 1992, and The Type and Effect Discipline, LICS 1992.

Space-efficient gradual typing: Abstracting Gradual Typing Moving Forward: Precise and Space-Efficient,
PACMPL 2021 (https://dl.acm.org/doi/10.1145/3434342); Compiling Gradual Types with Evidence
(https://arxiv.org/pdf/2512.22684).

Region-leak history and hybrids: Grossman, Morrisett et al, Region-Based Memory Management in Cyclone
(https://www.cs.umd.edu/projects/cyclone/papers/cyclone-regions.pdf); Elsman, Combining Region Inference and
Generational Garbage Collection; Reference Capabilities for Flexible Memory Management, 2023
(https://arxiv.org/pdf/2309.02983).

Lighter grounding: Type, Ability, and Effect Systems: Perspectives on Purity, Semantics, and Expressiveness,
2025 (https://arxiv.org/pdf/2510.07582); Glück, Is There a Fourth Futamura Projection?; Williams et al,
Revisiting the Futamura Projections: A Diagrammatic Approach (https://arxiv.org/pdf/1611.09906); Palsberg,
Correctness of Binding-Time Analysis, JFP.
