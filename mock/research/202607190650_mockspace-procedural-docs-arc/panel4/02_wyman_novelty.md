# Testing the novelty claims (Chris Wyman)

## The judgement in one line

The design's headline contribution is real but is stated one field too wide: refusing a document against a
target's declared feature set is standard practice in compiler and asset toolchains (SPIR-V, `javac
--release`, browserslist) and genuinely absent from document converters, while two other claims the round
rests on ("nothing occupies the many-in/many-out niche spanning logic and markup" and "the registry as
primary representation is a frontier move") have prior art that shipped decades ago and that the round
cited without applying.

## The novelty ledger

**Static pre-emission target-feature checking. ESTABLISHED as a mechanism, NOVEL only within document
conversion, and the round's phrasing does not say which it means.**

Searched: static validation of a document against an output format's supported constructs; DITA
specialisation and constraint modules; target-environment validation in shader toolchains; compiler
target-feature checking; browser-target linting. The mechanism exists and ships in four independent
lineages.

- **SPIR-V.** A module declares its capabilities with `OpCapability`; "a validator can validate that the
  module uses only its declared capabilities" and "a client API is allowed to reject modules declaring
  capabilities it does not support" ([Khronos SPIR-V
  spec](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html)). `spirv-val --target-env vulkan1.0`
  refuses a module whose declared capability the environment forbids, naming the capability
  ([example](https://github.com/KhronosGroup/SPIRV-LLVM/issues/236)). This is decision 2's inclusion check,
  shipped since 2015, including the "name the offending construct" diagnostic.
- **`javac --release N`.** Compiles against a per-release API signature database (`ct.sym`) and, unlike the
  older `-source`/`-target`, "will detect and generate an error when using APIs that don't exist in previous
  releases" ([Morling on ct.sym](https://www.morling.dev/blog/the-anatomy-of-ct-sym-how-javac-ensures-backwards-compatibility/),
  [Baeldung](https://www.baeldung.com/java-compiler-release-option)). Refusal, not degradation, decided by
  the chosen target.
- **Rust `#[target_feature]`.** Safe callers must enable every feature the callee enables, enforced at
  compile time ([RFC 2045](https://rust-lang.github.io/rfcs/2045-target-feature.html)).
- **browserslist plus `doiuse` / `stylelint-no-unsupported-browser-features` / `eslint-plugin-compat`.**
  The closest hit, because CSS is a declarative document language: a project declares a target set, and a
  linter statically refuses constructs those targets do not support, sourced from caniuse or
  [MDN browser-compat-data](https://github.com/3846masa/stylelint-browser-compat)
  ([plugin](https://www.npmjs.com/package/stylelint-no-unsupported-browser-features)).

Where it stops, and where the round is right: **no document-conversion system was found doing it.** DITA
constraint modules restrict content models per document type, not per output capability, and "do not and
cannot change element semantics"
([OASIS](https://docs.oasis-open.org/dita/v1.2/os/spec/archSpec/configuration-specialization-and-constraints.html));
they are grammar conformance, upstream of any target. Sphinx degrades at write time through
`unknown_visit`, a warning after conversion has begun
([sphinx#8936](https://github.com/sphinx-doc/sphinx/issues/8936)). Pandoc drops silently, as phase one
established. So the survivable claim is: *statically checking a document against its output target's
declared feature set is established practice outside documents and absent inside them.* "The one thing
nothing surveyed does" is true of the survey and false of the field, and a design doc that ships the wider
phrasing will be corrected by the first reader who has written a shader.

**Many input grammars, one representation, many targets, spanning logic and markup. NOT NOVEL. The round's
negative rests on a false sentence about Racket.**

`06_prior_art.md:53` asserts nothing found covers logic and markup in one representation, and `:65` that
"Racket declines the many-output half entirely." The second is wrong. Scribble's renderer is refined by a
mixin from `scribble/text-render`, `scribble/markdown-render`, `scribble/html-render` or
`scribble/latex-render` ([Racket docs](https://docs.racket-lang.org/scribble/renderer.html)); with
`#lang` supplying many input grammars over one fully-expanded core, and content as a first-class value,
Racket plus Scribble occupies the claimed niche on all three axes. Two more: **noweb** is language
independent with TeX, LaTeX, HTML and troff back ends, producing code and documentation from one source
([Ramsey](https://dl.acm.org/doi/fullHtml/10.5555/326984.326985)); **DSSSL** put Scheme logic over SGML
markup into RTF, TeX and HTML. The round also cites Erdweg et al. on language workbenches and then does not
apply it, which is the whole MDE tradition of many grammars into one model out to code and documents.

What is left after this correction is not nothing, but it is not the niche. It is the *conjunction* of that
niche with pre-emission target checking and with no heap on the generator side. The latter remains the
round's own best-identified novelty (`06_prior_art.md:246-257`), and it is untouched.

**Target-decided staging. ESTABLISHED, with a better citation than the one the round uses.**

Tagless-final already carries it and the round says so. The sharper source is Hofer, Ostermann, Rendel and
Moors, "Polymorphic Embedding of DSLs," GPCE 2008 (Most Influential Paper, GPCE 2018): pure embedding
"forces the DSL designer to commit to a single semantics," and the fix is that "optimizations and analyses
become just special interpretations of the DSL program"
([paper](https://www.informatik.uni-marburg.de/~rendel/hofer08polymorphic.pdf),
[ACM](https://dl.acm.org/doi/10.1145/1449913.1449935)). One term, interpretation chosen by the instance, is
exactly decision 1. Also: decision 1 frames source annotation as the only alternative, which is wrong.
Offline partial evaluation's binding-time analysis computes a division without source brackets; the round
names Jones, Gomard and Sestoft without using them. The residue that may be novel is narrow and worth
claiming precisely: *reduce-or-residualise decided by target policy, locally invertible by an explicit
context rule, with author predicates checked against that policy rather than inferred.* The three-part
composition is unlocated; "target-decided staging" alone is eighteen years old.

**Deterministic phrasing variation seeded by a content hash. NOVEL, on a second independent search.**

I did not inherit Wronski's result. Searched: deterministic natural language generation; paraphrase
selection with stable output under regeneration; deterministic text generation with seeds; template
variation in surface realisation; hash-seeded selection to avoid diff churn. Closest neighbours:
[SteadyText](https://github.com/julep-ai/steadytext), which fixes a seed to make an LLM's *generation*
reproducible, and Stable Diffusion seed practice, which is the same shape in images. Both make a
*generator* deterministic; neither selects among *pre-authored* variants, and neither derives the seed from
a semantic hash of the term being rendered. Dhall's versioned normal form and semantic hash supply the
hashing half and the round cites it correctly. Two independent negative searches now agree. Note the
awkward pairing: this is the round's only cleanly novel mechanism and also the one its own synthesis lists
as blocked on whether anyone wants it (`panel3/04:390-391`).

**The relational inversion. ESTABLISHED, and egglog is the wrong citation for it.**

Storing a program relationally and treating the tree as a view over that store is CodeQL's entire design:
de Moor et al., ".QL for Source Code Analysis," SCAM 2007
([PDF](https://codeql.github.com/publications/ql-for-source-code-analysis.pdf)), with "many tasks in source
code analysis can be viewed as evaluating queries over a relational representation of the code"; the
mature treatment is Avgustinov et al., "QL: Object-oriented Queries on Relational Data," ECOOP 2016
([DROPS](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2016.2)). Earlier still,
bddbddb (Whaley and Lam, PLDI 2004) and Doop. Egglog unifies Datalog with equality saturation, which is a
*rewriting* result; it does not establish the store-the-tree-relationally move Stachowiak used it for.
That mis-aim is independent of the venue error Wronski caught.

## What the round claimed as novel and is not

The many-in/many-out niche spanning logic and markup (Racket plus Scribble, noweb, DSSSL, MDE).
Target-decided staging (polymorphic embedding, tagless-final). The relational inversion (.QL, 2007).
Pre-emission target-feature checking as a *mechanism* (SPIR-V, `javac --release`, browserslist).

## What is genuinely novel, and narrower than stated

Pre-emission target-feature checking **applied to document conversion**, where every surveyed system
degrades. The no-heap generator (`06_prior_art.md:246-257`), still zero precedent and still the largest
unbacked claim. The variation seed derived from a semantic hash of the call. The three-part staging
composition named above. Note that all four are narrow and none is the niche framing the round leads with.

## What the prior art learned that this round has not

**A boolean feature bit is not enough, and every shipped system in this space discovered that.** MDN
browser-compat-data carries partial-implementation flags and version-ranged support; caniuse carries
"supported with known bugs." The round's design is one bit per family in a bitmask (`08_decisions.md:96`).
The day a target supports a construct badly rather than not at all, the bitmask has no way to say so, and
the answer will be either a lie or a schema migration. Adopt three states before the mask locks.

**SPIR-V's capabilities have dependencies:** declaring one implicitly declares those it rests on, which is
why the declaration stays short as the vocabulary grows. The round's family set is flat, so every document
must name every family transitively. Cheap to fix now, painful later.

**`javac --release` needed `ct.sym`, a shipped historical database, because the target's feature set is
versioned and the compiler is not the target.** Today every target here is in-repo. The first external
target (Lua 5.1 against 5.4) turns the target declaration into a versioned artefact needing exactly ct.sym's
shape.

**Scribble is the honest picture of the intended endpoint.** Many renderers over one content-value core,
working for twenty years, with fidelity diverging silently between backends. That divergence is the gap
this design closes, and it is a better motivating example than Pandoc because it already has the value
model.

## Corrections to Wronski and to earlier phases

**Wronski's egg correction is right and I confirm it** (POPL 2021, Distinguished Paper, reprinted in
PLDI 2022 Research Highlights). I add that fixing the venue does not fix the citation: egglog was aimed at
the relational-inversion claim and does not support it.

**The data-side reference figure is wrong again, for the third time.** The brief carries 6623. A complete
prefix histogram over `mock/registry` gives **6628**: 3475 `seed::`, 2286 `crates::`, 861 row-to-row
(650 `reference::`, 93 `spike::`, 66 `bench::`, 31 `tripwire::`, 7 `constant::`, 5 `ruling::`, 3 `law::`,
3 `equation::`, 2 `technique::`, 1 `facet::`), plus 3 `vocab::` and 3 `reg::` that no phase has counted.
Giesen's row-to-row figure is 861, not 862. The population has now been reported as 5761, 6623 and 6628,
and each error was the same class: an incomplete enumeration of prefixes rather than a histogram over all
of them.

**Verified and correct:** 2676 rows (per-namespace sum, exact), 14 namespace directories plus `vocab.toml`
at 10 rows, 207 files. I did not verify the 1007 template-side references and am not asserting them.

**`06_prior_art.md:65` is false** and should be struck: Racket does not decline many outputs. It is
load-bearing under a novelty claim, which is why it matters.

## Open provocations for the panellists after me

1. Restate the headline claim as scoped to document conversion, and cite SPIR-V and `javac --release` for
   the mechanism. The narrow claim is defensible and interesting; the wide one fails on first contact.
2. Adopt a three-state support model (supported, partial, absent) and capability dependencies before the
   family bitmask locks. Both are free now.
3. Re-aim the relational citation from egglog to .QL/ECOOP 2016, and read how QL preserves a tree-shaped
   API over a relational store. That is the exact shape Stachowiak proposed, already solved and shipped.
4. Publish the prefix-histogram command alongside the reference figure so the fourth count is the last one.
5. Decide whether the variation seed is worth its design budget. It is the round's cleanest novelty and its
   own synthesis cannot say whether anyone wants it.
