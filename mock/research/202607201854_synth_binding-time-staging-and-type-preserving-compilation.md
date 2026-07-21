# Binding-time, staging, and type-preserving compilation: compacted, one section per thread

**Date:** 2026-07-20
**What this is:** a compaction of the binding-time, staging, and compilation theory the certified-generation
panel continuation banked, chiefly the settlement-and-architecture doc
`202607201854_certgen_panel_continuation/02_settlements-and-next-architecture.md` and the core-nodes-and-bridge
doc `.../03_core-nodes-typesystem-and-zig-bridge.md`. One section per thread, whole-picture prose with a bearing
clause, dropping the exact constructions. For specifics, read the named sources. This sits beside the five
`202607201618_synth_*` domain docs and the two adjacent-theory syntheses as banked reading. It carries only the
theory, not the continuation's design opinions. The common ground: how staged specialization is structured
across more than two levels, how effect ordering is made explicit, how a recursive analysis becomes an iterative
machine, and how a proof established at generation time is preserved into a low-level form checkable without the
prover present.

## Multi-level binding-time analysis and multi-cogen: specialization across more than two levels

Glück and Jørgensen's "Efficient Multi-Level Generating Extensions for Program Specialization" (PLILP 1995, LNCS
982) generalises the two-level (static versus dynamic) split of classical partial evaluation to an arbitrary
number of binding times ordered as a lattice, and constructs a multi-level generating extension (a multi-cogen)
that discharges each computation at the earliest level whose inputs are all available. The companion work on
multi-stage specialization with relative binding times handles arbitrary staging depth, where a stage's binding
time is expressed relative to its enclosing stage rather than as an absolute level. The load-bearing result is
that "discharge each property at the earliest level its inputs are known" is a well-defined lattice operation,
not an ad hoc case analysis, and that a generating extension can be specialised across the whole level lattice.

Bearing on vehje: the arc runs a three-level problem (language definition at dev time, bundled content at our
build, arriving script at the consumer's load) through what had been a two-level framing; multi-level BTA is the
exact tool for three or more levels, and relative binding times are the named escape if staged macros ever
demand a fourth. It is what makes the three-loci split a consequence of the analysis rather than an optimisation
invented after the fact.

## Two-level functional languages: the annotation style for a staged core

Nielson and Nielson's "Two-Level Functional Languages" (Cambridge University Press, 1992) is the systematic
treatment of a lambda calculus in which every subterm carries a binding-time annotation (compile-time or
run-time), with a type discipline ensuring the annotations are consistent, so that the compile-time fragment
can be evaluated away leaving a well-formed run-time residual. It is the calculus-level statement of staged
computation, the setting in which "specialising to the early input agrees with running the two-level term
directly" is a theorem about annotation soundness.

Bearing on vehje: it is the annotation style for the arc's core calculus, in which each of the ratified forms
carries a binding-time annotation, extended from the classical two levels to the three (or the lattice of
knowledge sources) the multi-cogen work supports. It is the frame in which the interpolation form and the branch
forms are binding-time boundaries, and in which the staging-soundness theorem is stated.

## A-normal form: effect ordering without a sequencing form

Flanagan, Sabry, Duba, and Felleisen's "The Essence of Compiling with Continuations" (PLDI 1993) introduces
A-normal form, an intermediate representation in which every non-trivial subexpression is named by a `let`
binding, so that evaluation order is exactly binding order and every intermediate result is a variable. It
achieves, with a direct-style `let`-normalisation, what continuation-passing style achieves with explicit
continuations: a canonical order for effects, with pure bindings free to move because they carry no ordering
obligation. It is the reason a language needs no dedicated statement-sequencing form to pin effect order.

Bearing on vehje: A-normal form is the IR discipline under which the ratified forms carry effect ordering for
free, every effect named-bound so order equals binding order, which is what lets the arc decline a block or
sequence form. It complements the LMS effect-edge reading (from the purity synth doc), where A-normal form gives
the baseline order and the effect edges give the reorderable-pure-versus-pinned-effect distinction.

## Defunctionalization and the functional correspondence: from recursive analysis to iterative machine

Reynolds's "Definitional Interpreters for Higher-Order Programming Languages" (1972) introduces
defunctionalization, the transformation that replaces higher-order functions (in particular the continuations of
a recursive walk) with a first-order data type of tags plus an `apply` function, turning a recursive traversal
into an explicit machine over a stack of defunctionalized frames. Danvy and Nielsen's "Defunctionalization at
Work" (PPDP 2001) is the modern treatment, and Ager, Biernacki, Danvy, and Midtgaard's "A Functional
Correspondence between Evaluators and Abstract Machines" (PPDP 2003, BRICS RS-03-13) proves that a recursive
evaluator and its iterative abstract machine agree, obtained mechanically by closure conversion, CPS transform,
and defunctionalization of the continuation.

Bearing on vehje: defunctionalization is the principled route from a naturally recursive traversal (the lease
inference, the validation walk) to the iterative, explicit-work-stack kernel the arc requires for
comptime-stack safety, and the functional correspondence is the proof that the recursive specification (easy to
reason about) and the iterative machine (required by the runtime) compute the same thing. It is the theory under
"one source, an explicit stack, provably the same result."

## Certifying versus certified compilation: a per-instance checkable certificate

Necula and Lee's "The Design and Implementation of a Certifying Compiler" (PLDI 1998), read alongside Necula's
proof-carrying code, draws the distinction between a certified compiler (proven correct once and for all, for
every input, as CompCert is) and a certifying compiler (which emits, with each compilation, a checkable
certificate that the particular output is correct, re-verified per instance). The certifying route trusts the
checker rather than the compiler, and the checker is small and re-run on every artifact.

Bearing on vehje: certifying compilation is the precise word for what the arc does on its decidable axes, where
each compilation emits masks re-derived and re-checked after decode rather than trusting the generator once. It
separates the arc's per-instance re-proving from the once-and-for-all certified compilation of CompCert, and it
is the honest label for the assurance the two decidable axes actually carry.

## Typed Assembly Language: types preserved into a low-level checkable form

Morrisett, Walker, Crary, and Glew's "From System F to Typed Assembly Language" (TOPLAS 21(3), 1999) carries
source-level type information through every stage of compilation down to a typed assembly language whose
well-formedness is checkable at the assembly level without the source-level type checker present, so a low-level
artifact certifies its own safety by structure. The types are erased for execution but their consequences are
baked into the shape of the emitted code, which a type-checker over the low-level language can re-verify.

Bearing on vehje: TAL is the closest structural precedent for the arc's core move, a proof established at
generation time and preserved into the structure of the generated low-level form (the specialised runtime) whose
well-formedness the target compiler re-checks, with neither the source prover nor its type system shipping. It
is the citation for "the proof is compiled into the structure, not re-run and not re-embedded," lifted from
assembly to a specialised runtime.

## Bidirectional transformations and lenses: one source, two consistent projections

Foster, Greenwald, Moore, Pierce, and Schmitt's "Combinators for Bidirectional Tree Transformations" (TOPLAS
29(3), 2007) formalises lenses: a source artifact and a view derived from it, with a well-behavedness law
(get-put and put-get) guaranteeing the two stay consistent under edits to either. It is the theory of "one
truth, two projections kept consistent by construction" rather than one artifact serialised into another and
trusted to match.

Bearing on vehje: lenses are the mechanism under which a single declarative language definition could be
projected two ways (a compile-side representation and a generated-runtime representation) with their agreement a
property by construction rather than an unchecked serialisation, which is the pattern the earlier synthesis
already identified as the recurring winner ("one truth, many projections kept consistent by a generator"). It is
the theory for the handoff between the two ends of the pipeline and for deriving a value domain as a projection
of the node algebra rather than as a separately maintained parallel set.

## Sources

Multi-level BTA and multi-cogen: Glück, Jørgensen, Efficient Multi-Level Generating Extensions for Program
Specialization, PLILP 1995, LNCS 982 (https://link.springer.com/chapter/10.1007/BFb0026825); Glück, Jørgensen,
multi-stage specialization with relative binding times.

Two-level functional languages: Nielson, Nielson, Two-Level Functional Languages, Cambridge Tracts in
Theoretical Computer Science 34, 1992.

A-normal form: Flanagan, Sabry, Duba, Felleisen, The Essence of Compiling with Continuations, PLDI 1993
(https://dl.acm.org/doi/10.1145/155090.155113).

Defunctionalization and the functional correspondence: Reynolds, Definitional Interpreters for Higher-Order
Programming Languages, 1972 (reprinted HOSC 11(4), 1998); Danvy, Nielsen, Defunctionalization at Work, PPDP 2001
(https://www.brics.dk/RS/01/23/); Ager, Biernacki, Danvy, Midtgaard, A Functional Correspondence between
Evaluators and Abstract Machines, PPDP 2003 (https://www.brics.dk/RS/03/13/).

Certifying versus certified compilation: Necula, Lee, The Design and Implementation of a Certifying Compiler,
PLDI 1998 (https://dl.acm.org/doi/10.1145/277650.277752); Necula, Proof-Carrying Code, POPL 1997.

Typed Assembly Language: Morrisett, Walker, Crary, Glew, From System F to Typed Assembly Language, TOPLAS 21(3),
1999 (https://dl.acm.org/doi/10.1145/319301.319345).

Bidirectional transformations: Foster, Greenwald, Moore, Pierce, Schmitt, Combinators for Bidirectional Tree
Transformations, TOPLAS 29(3), 2007 (https://www.cis.upenn.edu/~bcpierce/papers/lenses-toplas-final.pdf).
