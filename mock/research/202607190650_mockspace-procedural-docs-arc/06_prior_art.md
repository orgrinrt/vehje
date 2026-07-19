# Prior art, and what it says about the shared IR

**Date:** 2026-07-19
**Method:** three independent research passes, primary sources only, dispatched neutrally. Findings below are
cited; the synthesis is this document's own.
**Bearing on the design:** it contradicts one stated premise, confirms two instincts, and identifies one place
where this design would be genuinely ahead of everything surveyed.

## The premise under test

The design was framed with this floor: a stable, comprehensive representation set, expressive and granular
enough that any output target can take it and produce the same intent in Lua, C#, Rust, markdown or HTML.
Covering logic and markup both.

That is two claims. The first is that any input can be reduced to a shared representation. The second is that
the shared representation can be *comprehensive*, meaning fixed and complete across all targets. The research
supports the first and contradicts the second, consistently, across every mature system found.

## What the evidence says about comprehensiveness

**MLIR refused it explicitly.** Its first design principle is titled "Little builtin, everything
customizable", and its builtin dialect contains **two operations** (`module` and
`unrealized_conversion_cast`). The stated reasoning names the failure mode directly: while it is possible to
put all operations, types and attributes in a single dialect, "it would quickly become unmanageable due to
the large number of simultaneously present concepts and name conflicts." The paper also concedes that
fragmentation across dialects is "unlikely" to have a purely technical solution, which is the honest cost of
the choice.

**GCC's language-independent tree is not language-independent.** GENERIC's own definition is circular ("if
you can express it with the codes in `tree.def`, it's GENERIC"), and front ends are permitted to carry
language-dependent tree codes provided they supply a hook to lower them. The genuinely shared level is
GIMPLE, which is lower and smaller. The canonical many-front-ends-one-tree system quietly allows per-front-end
node kinds.

**The strongest quantitative evidence argues for many small representations, not one.** Keep and Dybvig
replaced five back-end passes in Chez Scheme with over fifty nanopasses, each with its own formally defined
intermediate language. Generated code came out 15 to 26.6 percent faster, with compile time within a factor
of two. Their own framing is that supporting the complexity in a small number of passes is no longer
desirable. The cost they identify is boilerplate, which is why the approach only became affordable once a
DSL generated the unchanged-form traversals.

**Every many-input pivot that succeeded did so by modelling less.** WebAssembly's core has four types and no
strings, records or lists, precisely because a string in C is not a string in Rust; the high-level semantics
then returned as a mandatory second specification (the component model and WIT). Racket's fully expanded core
is roughly sixteen expression forms. Both are small, fixed and closed. Racket declines the many-output half
entirely.

**Truffle avoids a shared representation altogether.** Its polyglot interop is a message protocol over
per-language ASTs and objects. The one design spanning genuinely dissimilar languages shares a calling
convention, not a representation.

**Nothing found covers logic and markup in one representation.** Searched for directly. Pandoc is many-in and
many-out over markup only. MLIR is logic only.

The pattern across all of it: comprehensiveness and sharing trade against each other. Systems that shipped
picked a small core plus extensibility, or a small core plus accepted loss. None achieved both.

## The two fixed cores that do work, and what they measure

The contradiction is narrower than it first reads, because two fixed small cores in this survey are genuine
successes.

**Racket's core is about sixteen expression forms** and has stayed closed. It works because every input
language expanding into it agrees on an evaluation model.

**Pandoc's core is fourteen block constructors and twenty-one inline ones.** It works, in production, for
twenty years, many-in and many-out. It works because it is *deliberately under-expressive*: the manual states
plainly that the representation is less expressive than many formats it converts between and that conversion
should be expected to be lossy. That is not an apology. It is the design choice, made so that loss is uniform
and predictable rather than eliminated.

These two do not compete. They measure different things: an evaluation model, and a document structure. The
system under design needs exactly those two and, on the evidence, nothing else at the core.

That is the numeric support for the intuition that this collapses to several primitives with variants. Both
halves have a shipped precedent in the right order of magnitude.

## How the two cores join: content is a value

The unification is Typst's, and Scribble's before it. `content` is a first-class value type. All markup
produces content values; element functions construct content by taking fields as arguments, so markup and
constructor call are the same thing seen twice.

So the document core is not a second representation sitting beside the logic core. It is the **value domain**
of the logic core. A fragment returns content, a loop accumulates content, and markup emission is rendering a
value.

The research names the enabling detail, which is not incidental: **content must be a monoid.** Typst's `#for`
joins the results of each iteration into one value; Scribble accumulates into lists. Without an associative
join with an identity, a loop producing markup degrades into string concatenation and every structural
guarantee is lost at the first iteration.

The systems that instead modelled markup as data-only, with logic strictly outside it (Pandoc's filters,
Org's transcoders, mdast's handlers), all grew an ad-hoc extension layer anyway: Lua filters, `data.hName`,
derived backends. That is the value model reinvented at the metadata level, one tier down and less checkable.

## The escape hatch is structural, not a leak

Worth stating separately because it recurs without exception.

MLIR needed `unrealized_conversion_cast`, an operation whose whole job is absorbing type mismatch during
conversion, and it is one of only two operations in the builtin dialect. Pandoc needed `RawBlock Format` and
`RawInline Format`. Typst needed `html.elem`. GENERIC needed language-dependent tree codes. Every system
surveyed has one.

The consequence is documented and consistent: raw content is unanalysable, keyed to one target, and silently
dropped by every other. Pandoc's writers contract is literally "drop what isn't mine", with no diagnostic.

The conclusion to carry is not that escape hatches are avoidable. It is that they are a structural
requirement of any shared representation, and therefore should be designed deliberately, typed, and visible
to the checker, rather than discovered later and bolted on.

## Where target mismatch is handled, and the four strategies

Observed across the survey, in increasing order of honesty:

**Silent drop.** Pandoc raw content for a foreign format, Org's missing transcoder, mdast `html` nodes by
default. The dominant complaint in every case: the loss is invisible at conversion time and surfaces only
when someone reads the output.

**Raw escape hatch keyed by target.** Preserves the content for one target at the cost of portability, and
defeats every filter and analysis.

**Format negotiation at the value.** Scribble's `convertible` values offer several representations and the
backend picks.

**Transcode to a lower but universal medium.** Typst's `html.frame` lays content out as inline SVG when HTML
cannot express it. When the target cannot say it, embed a picture of it.

Only the last two preserve something without lying.

## The gap: nobody checks before emitting

Both research passes reached this independently, and it is the one place this design would be ahead of
everything surveyed.

**No system found statically checks, before emission, that a document only uses constructs its chosen output
target supports.** Typst's `target()` is a runtime query inside a show rule. Its HTML incompleteness is
gated by a feature flag on the whole compiler, not per construct. Pandoc has no notion of a document being
HTML-clean. Org resolves a missing transcoder by omission. Every one of them discovers the mismatch at write
time, and most respond by dropping content without a diagnostic.

This workspace already has the discipline that closes that gap. Targets declare the feature set they support,
and a document using a construct outside it is refused at check time with the specific construct named,
rather than emitted lossily.

Under a final encoding this is close to free. If a target is a trait implementation and feature families are
trait extensions, then a document requiring the document family cannot be handed to a target that does not
implement it, and the failure is a compile error at the exact mismatch. For documents parsed at runtime the
type system cannot carry it directly, so the same check moves into the checker against a declared feature
set. That is a pass to write rather than a guarantee for free, but it is still static, still ahead of the
prior art, and the declared-set shape is one the workspace already uses in `AccessSet`.

## The mechanism for evaluate-versus-emit is final encoding, not staging

Recorded because the first framing in this round was wrong.

Multi-stage programming (MetaOCaml, LMS) puts the decision in the **source**: brackets, or a `Rep[T]` type
distinction. The program says what defers. That is unusable here, because it would require the source to know
which targets exist.

**Tagless-final** (Carette, Kiselyov and Shan) has the property instead. A term is written once against an
interface, then instantiated at an interpreter that evaluates it, or one that emits code, or one that pretty
prints, without the term being touched. The choice is entirely which instance is picked, which is to say
entirely a property of the target.

It is also the known solution to the expression problem, which matters here because both axes must stay open:
new outputs are new implementations, new syntax families are new traits extending the old.

In Rust the interface is a trait with an associated `Repr` and each target is an implementation.
Monomorphised, no dynamic dispatch, `no_std` clean. It lands on rung one of `harness-the-type-system.md`,
which is the same answer arriving from compiler theory rather than from the rule.

The practical shape is a hybrid, because front ends parse text and must produce something concrete: parse into
an arena representation of the core forms, then one generic fold from that into any interpreter
implementation. One core, one fold, N targets as N implementations. The cost is that adding a core node kind
means a trait method, a fold arm, and an implementation in every target, which is exactly why the core has to
stay small.

Note what the Futamura projections do and do not give. Partial evaluation genuinely is a per-construct
decision to reduce or residualise, but it residualises into the same language it reduces in. Emitting a
*different* target is a separate concern the projections do not cover.

## Effects are the thing that bites

Confirmed rather than speculated. An evaluating interpreter performs effects; an emitting one must
residualise them; ordering survives only if the representation carries effect dependencies explicitly. LMS
does exactly that, for exactly this reason.

Concretely: a registry query inside a loop is fine to reduce for markdown output. Reducing it for a Lua target
means the query ran at generation time, which is either precisely what was wanted or silently wrong, and
nothing in the design so far expresses which.

The neighbouring hazards, all with names and known-hard status: scope extrusion, which MetaOCaml can only
catch dynamically; cross-stage persistence, where a value can be evaluated but not written back as target
syntax (a closure, a handle, a pointer); and non-termination, which Dhall avoids by giving up Turing
completeness outright and gaining, in exchange, normalisation that cannot fail once typechecking passes.

For a generator that runs inside a build, provable termination is worth more than recursion. That is a real
option with precedent, not a compromise.

## Two findings that bear on open questions

**Rendering written in the language itself has shipped.** Jsonnet's output formats
(`std.manifestYamlDoc`, `manifestIni`, `manifestPython`) are Jsonnet functions, a library rather than engine
machinery. The ambition to describe the markdown, html and plaintext outputs using the same mechanism that
emits Lua is not a stretch of this design; it is someone else's shipped decision.

**Dhall is the only system surveyed with a documented, versioned normal form**, with a standard binary
encoding and a semantic hash. That is a candidate answer to the open variation-seed question: seed the
phrasing choice from the semantic hash of the fragment call. Stable under renumbering and reformatting,
changing only when the call's meaning changes. Whether re-rolling on an argument change is acceptable is a
judgement call, but it is sharper than the document path, which this session already proved fragile.

## What this changes

The intent stands unaltered: define an input, define an output, and have them compose without per-pair work.
Every system surveyed that attempted this achieved the composition. That half is well-trodden and is not the
risk.

The vehicle changes. A stable comprehensive representation set is contradicted by every data point, including
by the two most serious attempts at exactly it. What replaces it:

A **small fixed evaluation core**, on the order of Racket's sixteen forms and probably fewer, since neither
continuations nor runtime macros are needed.

A **content value type with a document algebra**, on the order of Pandoc's set and probably fewer, forming a
monoid, first-class in the evaluation core rather than beside it.

**Target constructs outside the core living in families**, per MLIR, with a deliberately designed and typed
escape hatch rather than a discovered one.

**Targets as final-encoding implementations**, so evaluate-versus-emit is a property of the instance, and so
that adding a target is an implementation rather than a change to the core.

**A declared feature set per target, checked before emission**, which is the one thing nothing surveyed does.

One caution carried from nanopass: many small representations beat one only when defining a representation is
nearly free. If the family mechanism is expensive to use, the design collapses back toward a monolith by
default. The tooling around family definition is therefore a design constraint, not a convenience.

And one boundary, so the nanopass result is not misapplied: it concerns many representations in *sequence*
along a lowering pipeline. Racket and Pandoc concern one representation across *breadth*, many inputs or many
outputs. Different axes, both true, neither refuting the other.

## Where this design would be alone

Every system in the survey heap-allocates: MetaOCaml, LMS on the JVM, Jsonnet, Nix, Dhall, CUE, Starlark,
Racket, Pandoc, Typst.

Two exceptions share one shape. Terra's generator is Lua with a garbage collector, while the generated code
has no runtime at all. Rust procedural macros are a compiled dynamic library with full `std` and a heap,
emitting code that can be `no_std`. In both, the generator allocates freely and the output has no runtime.

So the shipping-scale precedent is generator-with-heap, output-without-runtime. Building the generator side
without a heap has no precedent found, and the apparent reason is that nobody needed to rather than that it
cannot be done. That is where the genuine novelty sits, and therefore where the risk sits.

## Sources

MLIR: [arXiv 2002.11054](https://arxiv.org/abs/2002.11054),
[Rationale](https://mlir.llvm.org/docs/Rationale/Rationale/),
[Builtin Dialect](https://mlir.llvm.org/docs/Dialects/Builtin/).
GCC: [GENERIC](https://gcc.gnu.org/onlinedocs/gccint/GENERIC.html),
[Language-dependent trees](https://gcc.gnu.org/onlinedocs/gccint/Language-dependent-trees.html).
Nanopass: [Keep and Dybvig, ICFP 2013](https://www.cs.tufts.edu/comp/150FP/archive/icfp13.pdf).
Racket: [syntax model](https://docs.racket-lang.org/reference/syntax-model.html),
[Scribble core](https://docs.racket-lang.org/scribble/core.html).
WebAssembly: [FAQ](https://webassembly.org/docs/faq/),
[Why the component model](https://component-model.bytecodealliance.org/design/why-component-model.html).
Truffle: [implementation framework](https://www.graalvm.org/latest/graalvm-as-a-platform/language-implementation-framework/).
Language workbenches: [Erdweg et al.](https://homepages.cwi.nl/~storm/publications/lwc13paper.pdf),
[Hutchinson et al. on industrial adoption](https://www.sciencedirect.com/science/article/pii/S0167642313000786).
Pandoc: [pandoc-types](https://hackage-content.haskell.org/package/pandoc-types-1.23.1.2/docs/Text-Pandoc-Definition.html),
[MANUAL](https://pandoc.org/MANUAL.html),
[Beyond Markdown](https://johnmacfarlane.net/beyond-markdown.html), [Djot](https://djot.net/).
Typst: [content](https://typst.app/docs/reference/foundations/content/),
[scripting](https://typst.app/docs/reference/scripting/), [html](https://typst.app/docs/reference/html/).
Org: [export reference](https://orgmode.org/worg/dev/org-export-reference.html),
[babel](https://orgmode.org/manual/Working-with-Source-Code.html).
Staging: [MetaOCaml](https://okmij.org/ftp/ML/MetaOCaml.html),
[LMS](https://www.cs.purdue.edu/homes/rompf/papers/rompf-wf16.pdf),
[Terra PLDI'13](https://cs.stanford.edu/~zdevito/pldi071-devito.pdf),
[tagless-final](https://okmij.org/ftp/tagless-final/course/lecture.pdf).
Config languages: [Dhall beta-normalization](https://github.com/dhall-lang/dhall-lang/blob/master/standard/beta-normalization.md),
[Jsonnet output formats](https://jsonnet.org/articles/output-formats.html).
