# The four calls, settled

**Date:** 2026-07-19
**Decided by:** the maintainer, against the options in `07_toolbox.md`.
**Supersedes:** the closed-core recommendation in `07_toolbox.md`, which was wrong. The reasoning for the
reversal is worked out below rather than asserted, because the reversal turns on a mechanism the survey did
not surface.

## 1. Staging: the target decides, uniformly

An expression's fate is a property of the output target, not of the source and not of a per-node annotation.
A markdown target reduces everything, so a loop runs and only its accumulated content appears. A Lua target
preserves control flow, so the same loop is emitted as a Lua loop. One source, two fates, and the source
never names which.

This keeps a fragment reusable across targets, which is the whole point of fragments, and it avoids the
MetaOCaml and LMS shape where the source must know which targets exist.

### The hybrid the maintainer flagged, and where it is safe

The maintainer noted that the target-decides and context-decides options might combine, possibly with
heuristics, confidence weighting, or predicates. Two of those three are safe and one is not.

**Safe: target policy as the default, context as a local override.** This is not a heuristic. It is a
two-level rule with a clear precedence. A Lua target preserves control flow by default, but an interpolation
hole inside emitted Lua is generation-time, because that is what an interpolation hole means. Typst does
exactly this with `target()` queryable inside show rules; ordinary templating does it whenever `{{ }}`
appears inside otherwise-literal output. The rule is decidable by reading the source and the target, with no
inference.

**Safe: declared predicates, checked rather than inferred.** A fragment may assert that a subexpression must
be generation-time (a registry query cannot survive into Lua, because the registry does not exist at Lua
runtime) or must be residual. The assertion is a constraint the checker verifies against the target's
policy, and a conflict is a diagnostic naming both sides. This is a proof obligation, not a guess.

**Not safe: confidence weighting and inferred staging.** Automatic binding-time analysis has a documented
history of not working well: the literature's own assessment is that automatic analyses have limited
applicability and that annotations still have to be created and maintained by hand. Worse for this use case,
a document generator's output lands in git diffs. Staging decided by a weighted heuristic means the same
input can produce different output as the heuristic's inputs drift, and the diff shows prose changes that no
author made. Predictability is a hard requirement here in a way it is not for a compiler optimising for
speed.

So: target sets the policy, context can invert it locally by an explicit and readable rule, predicates
constrain and are checked, and nothing is inferred probabilistically.

## 2. Core shape: open families, with typestate carrying the exhaustiveness

The maintainer's question was whether typestate can supply what exhaustiveness supplies, so that open
families do not cost the check-before-emit guarantee. The answer is yes, and the reasoning below is why the
`07_toolbox.md` recommendation was wrong.

### The false tension

The tension was posed as: a closed enum lets a backend's match be checked for totality, an open family set
does not, therefore openness costs the guarantee. That framing assumes the guarantee has to be *coverage*,
meaning the backend proves it handles every case in a fixed universe.

It does not. The guarantee can be *inclusion* instead: prove that what a document contains is within what
the target declares it supports. Same implication, different direction, and inclusion does not need the
universe to be closed.

### The shape

A family is a type. A document carries a type-level set of the families it uses. A target declares a
type-level set of families it supports. Emission is bounded on the target's set containing all of the
document's.

This is the `AccessSet` machinery, already shipped in `hilavitkutin-api/src/access.rs`: a sealed
`AccessSet` trait, `Contains<S>` for single membership, `ContainsAll<L>` for set inclusion. hilavitkutin uses
it to prove a WorkUnit only touches stores it declared. The document-and-target problem is the same problem
with different nouns.

### Why this is better than exhaustiveness, not merely equal to it

Under a closed enum, every backend must handle every constructor, including constructors it has no sensible
rendering for. That is why Pandoc's writers contract is "drop what isn't mine" with no diagnostic: totality
forced them to accept everything, so the only way to handle the unhandleable was to discard it silently.

Under set inclusion, a backend legitimately declares a smaller set. A plain-text target that genuinely
cannot express a definition list says so, and a document using one is refused with the construct named
rather than degraded into something wrong. The guarantee gets stronger as the world opens, which is the
opposite of what the earlier framing predicted.

### The honest caveat: where the proof is compile-time and where it is not

Documents parsed at runtime cannot carry a compile-time family set, because the set is not known until the
parse finishes. The resolution is two-stage, and the important part is that the discipline stays
compile-time even where the evidence does not.

For **statically known terms** (fragments written in Rust, renderers, anything in source) the full
compile-time proof applies: the family set is a type, `ContainsAll` discharges at compile time, and a
mismatch is a compile error.

For **parsed documents** the family set is computed during parsing as a runtime value, a bitmask over family
ids. The target's declared set is a type-level set that also lowers to a const bitmask. The check is one
bitwise operation. Typestate then makes it impossible to skip: `emit` accepts only a `Checked<T>`, and a
`Checked<T>` can only be produced by running the check for that target. The evidence is computed at runtime;
the obligation to compute it is enforced at compile time.

That is weaker than a pure static proof and stronger than anything in the survey, where no system checks at
all before emitting.

### The framework cost is a C++ and Scheme number, not a Rust one

The cost argument in `07_toolbox.md` leaned on nanopass's roughly 4600 lines of framework and MLIR's
TableGen. Both numbers come from languages that lack the mechanism Rust has.

MLIR needs ODS and TableGen because C++ cannot declare an operation's interface list declaratively, so the
declarations live in a separate language and a generator emits the C++. Nanopass needs macros because Scheme
records carry no type-level structure, so the pass-through clauses must be generated.

**In Rust, the trait system is the ODS.** A family is a trait. Its required interfaces are supertraits. The
"generic pass dispatches on traits rather than a closed enum" mechanism is trait bounds, written directly. The
generation framework that made openness affordable elsewhere is largely already in the language, so the
number that argued against openness does not transfer.

What remains genuinely hand-written is traversal, which Pandoc and Djot both concluded should be hand-written
anyway for performance reasons unrelated to this choice.

## 3. Renderers: total functions over the algebra

A renderer must handle every constructor in the families it declares, and that totality is checked.

Note how this composes with decision 2 rather than conflicting with it. Totality is over the **declared**
family set, not over a closed universe. A renderer declares `Supports = (Core, Doc)` and must be total over
those; it is not obliged to have an opinion about a family it never claimed. This is what makes the two
decisions consistent, and it is the same move as inclusion-instead-of-coverage one level down.

Jsonnet's manifestation-as-library shape (rendering functions written in the language itself, 20 to 160 lines
each) stays available on top: the DSL may compose and wrap total renderers, but cannot replace them. The
guarantee survives; the flexibility sits above it.

## 4. Variation seed: semantic hash of the call, with an optional explicit override

The default is Dhall's mechanism applied to the fragment call: normalize, alpha-normalize, canonically
encode, hash. Invariant under comments, formatting and variable naming, which is what makes it stable under
the renumbering that broke the document-path candidate this session.

An explicit seed field is available as an optional override where the author needs to pin a phrasing, subject
to finding an ergonomic way to express it. The requirement on that ergonomics: it must be optional at the
call site with zero syntax when unused, because a seed argument that every call has to carry would be worse
than the churn it prevents.

The known cost, accepted: changing any argument to a fragment call re-rolls its phrasing, because the
argument is part of the hashed call. Whether that reads as acceptable variation or as diff noise is an
empirical question that real documents will answer, and the explicit override is the escape hatch when it
turns out to be noise.

## What these four decide together

The staging call fixes what a target is: a policy plus a family set. The core call fixes how families
compose: open, with inclusion proofs instead of closed coverage. The renderer call fixes what a target owes:
totality over what it declared. The variation call is independent of the other three and can move without
disturbing them.

Three of the four are load-bearing on each other through one idea, which is worth naming because it is the
design's spine: **a target declares what it supports, and everything else is a proof obligation against that
declaration.** Exhaustiveness, feature checking, and staging predicates are three faces of the same
mechanism.

## What is still open after these

The two core lists, which are design work rather than a decision, and which the survey deliberately does not
answer.

The effect intent, narrowed but not closed by decision 1: the target's policy now decides whether a query
reduces, and predicates can constrain it, but nothing yet says what a query inside code destined for a
runtime target *means*. Decision 1 makes it expressible and checkable. It does not say which answer is right.

Whether the parsed-document runtime check can be lifted to compile time by compiling templates ahead of
time, which would close the one gap in decision 2's proof. Worth an hour of thought before accepting the
runtime path as permanent.
