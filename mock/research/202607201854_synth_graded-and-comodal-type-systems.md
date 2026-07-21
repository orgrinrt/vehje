# Graded and (co)modal type systems: compacted, one section per thread

**Date:** 2026-07-20
**What this is:** a compaction of the graded and modal type-theory surfaced by the certified-generation panel
continuation, chiefly the novelty audit
`202607201854_certgen_panel_continuation/04_novelty-audit-grading-and-the-conservative-recombinations.md`. One
section per thread, conveying the whole picture and why it bears on vehje, dropping the exact typing rules and
proofs. For the specifics, read the named sources. This sits beside the five `202607201618_synth_*` domain docs,
the two compactions, and `202607201854_synth_adjacent-theory-for-certified-generation.md` as banked required
reading. It carries only the theory, not the continuation's design opinions or the options it hangs off that
theory. The common ground of every thread here is one move: index a single typing judgment by algebraic grades
drawn from a semiring or a lattice, so that properties the arc currently tracks as three or four parallel
analyses (binding time, effects, lease/usage, assurance) become coordinates of one system.

## Graded monads and parametric effect monads: effects as a grade

Katsumata's "Parametric Effect Monads and Semantics of Effect Systems" (POPL 2014) gives the categorical
account of an effect system as a monad indexed by an ordered monoid of effect grades. Each computation type
carries a grade (the effects it may perform), sequencing multiplies grades in the monoid, and the subeffecting
order on grades is exactly the inclusion order an effect system checks. The construction is the semantic
statement of "an effect annotation is an element of an algebra, and the type system is arithmetic in that
algebra": bind composes grades, the unit is the pure grade, and a coercion up the order is always sound. It is
the categorical backbone under the type-and-effect discipline the arc already cites, generalised from a fixed
effect lattice to any ordered monoid of grades.

Bearing on vehje: the effect axis (a residual's effect set included in the target's permitted set) is a graded
monad in this sense, with `Permits` inclusion as the grade order. It is the piece that lets the effect axis and
the lease/usage axis be combined rather than proven separately (the effect-coeffect thread below), and it is the
categorical home for the gradual effect systems already banked in the adjacent-theory synth.

## Coeffects: per-variable, context-dependent grades

Petricek, Orchard, and Mycroft's "Coeffects: A Calculus of Context-Dependent Computation" (ICFP 2014, and
Petricek's thesis, 2017) is the dual of an effect system. Where an effect grades what a computation does to the
world (a property of the conclusion), a coeffect grades what a computation demands of its context (a property of
each variable in the environment). The judgment carries a grade per free variable saying how that variable is
used: how many times, under what capability, at what security level, with what liveness. Formally it is a graded
comonad, the categorical dual of the graded monad above, and the two are genuinely different structures because
usage is per-input and effect is per-output; a single per-computation flag cannot express a per-operand usage
distinction.

Bearing on vehje: the lease link/consume bit is a per-operand usage annotation (does this op retain this
specific input past the call), which is a coeffect, not an effect. Coeffects are the precise theory for the
lease axis's per-operand resolution, and the distinction (usage lives on the edge from op to operand, not on
the op) is why a per-op effect flag loses resolution across an op with several operands.

## Combining effects and coeffects via grading: the unification

Gaboardi, Katsumata, Orchard, Breuvart, and Uustalu's "Combining Effects and Coeffects via Grading" (ICFP 2016)
is the paper that puts a graded monad (effects) and a graded comonad (coeffects) in one type system and gives
the distributive law that lets them interact soundly. It is the constructive form of the folklore that "region
systems and effect systems are one system": rather than an analogy, it is a proven adjunction tying what a
computation demands of its inputs to what it does to the world, so a property stated on one side has a defined
image on the other.

Bearing on vehje: this is the theory that turns the arc's central move (derive the lease bit from the declared
effect) from a slogan grounded only in Talpin-Jouvelot into a construction, because it names the exact
mechanism by which a per-output effect grade and a per-input usage grade cohere. It is the citation the arc's
lease-from-effect proposal wants when it claims the two axioms collapse into one, and it composes with the
gradual effect and gradual verification machinery already banked.

## Graded modal types in a working language: Granule

Orchard, Liepelt, and Eades's "Quantitative Program Reasoning with Graded Modal Types" (Granule, ICFP 2019) is
the existence proof that the graded programme is implementable, not merely categorical. Granule is a functional
language whose types carry graded modalities, and the same grading discipline expresses linear and affine
resource usage, effects, information-flow security, and hardware-schedule constraints as coordinates of one
type, checked by one typechecker. It demonstrates that effects, usage, and other context properties do not need
separate analyses; they are grades over different semirings threaded through one judgment.

Bearing on vehje: Granule is the concrete evidence that vehje's separate axes (binding time, effect, lease)
could be coordinates of one graded judgment with one soundness argument rather than three metatheories argued to
cohere. It is the shipped counterexample to the assumption that each axis needs its own proof.

## Modal analysis of staged computation: binding time as a necessity modality

Davies and Pfenning's "A Modal Analysis of Staged Computation" (POPL 1996, and JACM 48(3), 2001) reads
staged, multi-level computation through the necessity modality of modal logic: a value known at an earlier stage
inhabits a `box`-typed proposition, and running it later is the modality's elimination. It is the modal reading
of exactly the two-level split the arc modelled with Nielson-Nielson two-level lambda, and it makes the
binding-time distinction a logical connective rather than an annotation convention.

Bearing on vehje: grading the necessity modality by the arc's binding-time lattice (`Lang < Script < Runtime`,
and the bundling refinement that makes it a lattice of knowledge sources rather than a chain) makes the
multi-level binding-time structure a graded `box`, which is what lets binding time join the same graded system
as effects and usage. It is the third coordinate's categorical home, alongside the graded monad (effects) and
graded comonad (usage).

## Quantitative type theory: assurance and resource as a quantity

Atkey's "Syntax and Semantics of Quantitative Type Theory" (LICS 2018) and McBride's "I Got Plenty o' Nuttin'"
(2016) annotate each variable with a quantity drawn from a semiring (zero, one, many, or a richer resource
algebra) and thread those quantities through dependent typing so that resource usage is a first-class,
checkable property of a term. It is the dependent-type expression of the same grading idea, with the semiring
supplying the arithmetic of "how much" rather than merely "which."

Bearing on vehje: quantitative typing is the frame in which the depth cap and the lease can be read as
quantities, and in which the assurance level of a property (statically certified, structurally certified,
re-proven per instance, dynamically checked, tested, unclaimed) becomes a graded quantity on one scale rather
than a hand-partitioned set of buckets. It is the theory that would make the guarantee inventory a computed
grade vector.

## Algebraic effects and handlers: operations and the discharge that services them

Plotkin and Pretnar's "Handlers of Algebraic Effects" (ESOP 2009) and Bauer and Pretnar's "Programming with
Algebraic Effects and Handlers" (2015) model an effect as a signature of operations and a handler as the
piece of program that services those operations, giving a uniform semantics for exceptions, state, nondeterminism,
generators, async, and any user-defined control effect. An operation is performed; a surrounding handler
interprets it, optionally resuming the computation. Graded algebraic effects are a studied composition with the
graded-monad reading above, so operations-and-handlers and effect-grades are not rival accounts but layers of
one.

Bearing on vehje: algebraic effects are the semantic foundation under which the effect set, the host-call
surface (an operation serviced by the host), and macro expansion (a compile-stage operation serviced by the
runtime's compile stage) are three instances of one handler discipline rather than three separate mechanisms.
It is the theory named as the candidate unification behind the one plausible twelfth Core form (a resumable
`Handle`), and it composes with grading because graded algebraic effects exist.

## Sources

Graded monads: Katsumata, Parametric Effect Monads and Semantics of Effect Systems, POPL 2014
(https://dl.acm.org/doi/10.1145/2535838.2535846).

Coeffects: Petricek, Orchard, Mycroft, Coeffects: A Calculus of Context-Dependent Computation, ICFP 2014
(https://dl.acm.org/doi/10.1145/2628136.2628160); Petricek, Context-Dependent Computation (thesis), 2017.

Effect-coeffect grading: Gaboardi, Katsumata, Orchard, Breuvart, Uustalu, Combining Effects and Coeffects via
Grading, ICFP 2016 (https://dl.acm.org/doi/10.1145/2951913.2951939).

Graded modal types: Orchard, Liepelt, Eades, Quantitative Program Reasoning with Graded Modal Types (Granule),
ICFP 2019 (https://dl.acm.org/doi/10.1145/3341714).

Modal staging: Davies, Pfenning, A Modal Analysis of Staged Computation, POPL 1996 and JACM 48(3), 2001
(https://www.cs.cmu.edu/~fp/papers/jacm00.pdf).

Quantitative type theory: Atkey, Syntax and Semantics of Quantitative Type Theory, LICS 2018
(https://bentnib.org/quantitative-type-theory.pdf); McBride, I Got Plenty o' Nuttin', 2016
(https://personal.cis.strath.ac.uk/conor.mcbride/PlentyO-CR.pdf).

Algebraic effects and handlers: Plotkin, Pretnar, Handlers of Algebraic Effects, ESOP 2009
(https://homepages.inf.ed.ac.uk/gdp/publications/Effect_Handlers.pdf); Bauer, Pretnar, Programming with
Algebraic Effects and Handlers, JLAMP 2015 (https://arxiv.org/abs/1203.1539).
