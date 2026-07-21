# The output spectrum: one term, many targets, and interpret-versus-transpile as one axis

**Date:** 2026-07-20
**Domain:** producing output from a shared representation across many targets. Tagless-final and polymorphic
embedding, the interpret-to-transpile spectrum, the writer/renderer contract, coverage versus inclusion, trait
and interface dispatch over an open operation set, and what a new target actually costs.
**Synthesised from:** `prior_art/07_rust_final_encoding_arenas.md` (§1-3),
`prior_art/04_racket_scribble.md`, `prior_art/03_pandoc_djot.md`, `prior_art/02_jsonnet_dhall.md`
(manifestation), `prior_art/06_mlir_ods_nanopass.md` (trait/interface dispatch), `prior_art/05_typst.md`
(targets), `prior_art/01_lms_delite.md` (backends).

## The one idea underneath all of it

Interpretation and transpilation are not different operations; they are the same operation aimed at different
targets. A term written once against an interface can be handed to an instance that evaluates it, an instance
that emits code, or an instance that pretty-prints it, without the term changing; the choice of what a target
does is a property of the instance, not of the source. This is the tagless-final / polymorphic-embedding
result, and the corpus demonstrates it four independent times: LMS's per-backend codegen traits over one IR,
Racket/Scribble's pluggable render backends over one document tree, Pandoc's writers over one AST, and
Jsonnet's manifestation functions over one value tree. "Full interpretation" reduces everything and yields a
value; "pure transpilation" reduces nothing and re-expresses the term in another language; every real target
sits somewhere between, and where it sits is a staging choice, not a category.

The Rust write-ups in `prior_art/07` state the type-level payoff directly: two interpreters for one `Lang`
trait, `type Repr<T> = T` (a direct evaluator) and `type Repr<T> = String` (a pretty-printer/codegen backend),
both implementing the same trait, and passing `L::int(1)` to `L::add` is a compile-time error in both. Adding
a backend is a new impl; the term never changes.

## Two encodings, and why real Rust compilers picked the other one

The classic tradeoff (Wadler's expression problem, cited by name in `prior_art/07`):

- **Final encoding (trait + associated type/GAT per backend).** Adding a backend is free (new impl); adding a
  node is a breaking trait change (every impl must add the method). In Rust specifically it carries a stack of
  costs the OCaml/Haskell original does not: no higher-kinded types, so no dyn-erased term; forced
  monomorphization per interpreter (`fn term<E: ExprSym>() -> E::Repr`, called once per interpreter,
  monomorphized N times); boxed closures for laziness (`if_` taking `Box<dyn Fn() -> Self>`); and, per the
  authors' own words, it never reaches the ergonomics of the original because Rust cannot defer or erase a
  term's interpreter choice to run time the way a typeclass dictionary does. A trait with `type Repr<T>`
  cannot be boxed as `dyn` past the simplest first-order case (rust-lang/rust#81823), so there is no
  type-erased "list of interpreters."
- **Initial encoding (concrete enum + `match` per backend).** Adding a backend is free (new function); adding
  a node edits every function. The value is first-class: constructible once, stored, pattern-matched, cloned,
  walked by any number of backends without recompiling, trivially heterogeneous.

The load-bearing empirical finding in `prior_art/07` §3: **no real Rust compiler ships tagless-final as its
term representation.** rustc, rust-analyzer, oxc, swc, cranelift, and Cloudflare's wirefilter all use initial
encoding (a concrete enum/struct tree, arena- or index-held). The tagless-final material that exists is
blog-scale. The mainstream resolution of the same underlying problem is the hybrid: parse once into a
concrete arena/index-based tree, then write ordinary generic functions or a visitor/fold trait that walks that
fixed representation. A "backend" becomes a function or trait impl closing over the shared arena types, with
no interpreter-generic type parameter threading through the IR. This sidesteps every object-safety/GAT/boxing
problem and is exactly the flat-array-plus-index shape the next domain doc covers. (Wirefilter reports the
same resolution from another angle: it moved away from `trait Expr { fn execute() }`-per-node dispatch toward
compiling the AST once into a `Box<dyn Fn(&Ctx) -> bool>` closure tree, decoupling "the shape built" from "the
shape executed," and measured the extra boxing as "negligible.")

## The writer/renderer contract: how a target plugs in

Three systems show the concrete shape of a target, at three sizes.

**Pandoc** is the minimal contract: one function type, dispatched by an association list, not a typeclass.
`data Writer m = TextWriter (WriterOptions -> Pandoc -> m Text) | ByteStringWriter (...)`, and `writers ::
[(Text, Writer m)]`. A new writer is one function plus one tuple appended. The cost model is measured: the
minimal `Writers.Man` is 379 lines exhaustively matching all 14 `Block` and 19 `Inline` constructors once
each; the complex `Writers.Markdown` is 1936 lines implementing four variants (markdown, commonmark, markua,
plain) as one shared `pandocToMarkdown` parameterized by `envVariant`, and `Writers.Plain` does not exist as a
standalone module, it reuses ~1900 lines under a mode flag. A third shape, `Writers/Djot.hs` (300 lines), is
an adapter into the standalone `djoths` package, so most syntax knowledge lives outside Pandoc. The takeaway
for target clustering: standalone writers run ~380 lines, clustered writers sharing a mode flag amortize to
far less per format, and adapters push the work into a separately-versioned library.

**Racket/Scribble** shows the refined-by-mixin contract. A backend is `(render-mixin %) -> (class % ...)` over
a shared `render%` (1251 lines, written once) that implements the four-pass `traverse/collect/resolve/render`
pipeline; a backend overrides ~a dozen leaf methods (`render-part`, `render-flow`, `render-table`, ...).
`text-render.rkt` is standalone and complete at 322 lines; `markdown-render.rkt` is 381 lines as a mixin over
the text renderer, nearly all of it Markdown escaping and table logic, not pipeline plumbing. So "a new output
form costs override-a-dozen-leaf-methods, not reimplement-the-pipeline."

**Typst** shows the crate-per-backend shape downstream of one documented IR: `typst-render` (raster),
`typst-svg`, `typst-pdf`, `typst-html`, all downstream of `typst-layout`'s `Frame` tree (position+size+paint),
"the one documented IR between realized content and a concrete output." `typst-html` is the instructive
divergence: it converts the *realized element tree* straight into an `HtmlNode` DOM, bypassing `Frame`, and
the module doc states the incompleteness directly ("Typst cannot simply produce perfect HTML... It cannot
always know what the best semantic HTML representation of your content is"): no CSS emission, no fragment
export, gated experimental. The lesson: a single frame-tree IR serves paged/raster/vector targets cleanly, and
a structurally-different target (a DOM) has to fork upstream of the shared IR and pays an honest fidelity gap.

## Coverage versus inclusion: the design axis that decides the failure mode

This is the sharpest cross-source finding, and it comes from Pandoc's failure mode contrasted with the
totality discipline of exhaustive writers.

Under **coverage**, every backend must handle every constructor in a closed universe, including ones it cannot
render, which forces the Pandoc contract for raw content: `RawBlock`/`RawInline` carry a `newtype Format Text`
(case-insensitive), and each writer decides per-node whether raw content is *theirs* by literal string match,
falling through to a warning, not an error: `blockToMan _ b@(RawBlock f str) | f == Format "man" = literal str
| otherwise = report (BlockNotRendered b) >> return empty`. No central registry; convention plus a per-writer
guard, dropping silently-ish (a warning) what is not the writer's. This is the "drop what isn't mine" contract,
and it is what a closed-universe coverage model forces.

Under **inclusion**, a target declares the set it supports, and a program using something outside it is refused
with the construct named, not degraded. The exhaustive-writer discipline is the small version of this: Pandoc's
own `Walkable` traversal (`walkBlockM`) is an exhaustive pattern match, so a new `Block` constructor forces a
compile error until handled, and Racket's `render%` leaf methods are similarly total over the model. The
inclusion model is stronger than coverage, not merely equal, because a backend can honestly declare a smaller
set and refuse the rest by name. (The parent arc's own `06_prior_art.md` grounds inclusion in four independent
lineages: SPIR-V `OpCapability` checked by `spirv-val --target-env`, `javac --release` against `ct.sym`, Rust
`#[target_feature]` per RFC 2045, and browserslist with `doiuse`; the mechanism is established, the application
to a multi-target IR framework is where it sits ahead of the surveyed document converters.)

MLIR generalizes inclusion into *trait/interface dispatch over an open operation set*, and this is the
mechanism for "the op set grows forever but generic passes still work." A generic pass never switches on
concrete op identity; it queries a trait (`op->hasTrait<MyTrait>()`) or casts to an interface (`if
(ExampleOpInterface e = dyn_cast<ExampleOpInterface>(op)) e.method()`). Canonicalization, CSE, DCE,
bufferization all operate over an ever-growing op set by dispatching on trait/interface, never on a closed
enum. The stated cost: an op that forgets to declare `Pure` is invisible to DCE, so the openness is only as
good as each op's opt-in. Dependencies compose as supertraits (declaring a capability pulls what it rests on),
so declarations stay short as the vocabulary grows, with no separate dependency-DAG model.

## The escape hatch for "a value the model does not know about"

Two systems ship the same seam for letting the representation hold a type it was not designed to know.
Racket's `content?` is a closed recursive union that admits, as one of its cases, "anything `convertible?`":
`prop:convertible` holds one procedure `(v request default) -> result`, and `convert` asks for `'text`,
`'png-bytes`, `'svg-bytes`, etc., a producer answering what it can and returning `default` otherwise. This is
the seam letting `content?` hold a pict/image/plot without the model knowing those types exist, at the cost of
structural undecidability and a `convert`+fallback everywhere. Typst's `Value::Dyn(Dynamic)` is the same
escape hatch as one enum variant among 28. The lesson for a closed-core-plus-open-families design: a single
typed negotiation form (offer the representations you can be; the target takes what it understands) is the
established way to keep a closed model open at exactly one controlled point.

## What a new target costs, consolidated

| System | Target contract | Measured floor | Shared engine written once |
|---|---|---|---|
| Pandoc | one `WriterOptions -> Pandoc -> m Text` + association-list tuple | ~380 lines standalone; ~0 marginal in a clustered mode-flag writer | the `Walkable` traversal + Builder |
| Scribble | mixin over `render%`, override ~12 leaf methods | 322 lines (text), 381 (markdown-as-mixin) | `render%` four-pass pipeline, 1251 lines |
| Typst | crate downstream of the `Frame` tree | crate-per-backend | `typst-layout`'s `Frame` IR |
| LMS | one `emitNode` `case` arm per IR node type, per backend | a few lines per node per backend | `GenericCodegen`, ~320 lines |
| MLIR | trait/interface opt-in per op | declarative, per op | generic passes over trait/interface |

The unifying shape: one shared engine or traversal written once, plus a small per-target surface that is
linear in the node/op vocabulary, plus a dispatch mechanism (association list, mixin, trait/interface) that
lets targets and node kinds grow on independent axes. The inclusion-not-coverage choice is what makes a
target's declared-smaller-set honest rather than a silent-drop.

## What would not transfer to no_std, no-alloc Rust

Every system here is heap-and-GC-shaped at the traversal layer. Pandoc's `[Block]`/`Meta` map, Djot's finger
trees, Racket's syntax objects and persistent `#hasheq`, Typst's refcounted `Content`, LMS's persistent lists
for effect context: all assume a collector or an allocator. `Walkable`'s ~90 hand-instantiated instances are
ad-hoc polymorphism with no monomorphization-only analogue; a no-alloc port needs dynamic dispatch through a
closed vtable of node kinds or generated per-type-pair recursion. The transferable core is not the traversal
machinery but the *contracts*: association-list-or-generated-match dispatch, exhaustive-match totality
(compile error on an unhandled case), inclusion-not-coverage, the single typed escape hatch, and one shared
engine plus a thin per-target surface.

## Sources

Primary synthesised docs:
`202607190650_mockspace-procedural-docs-arc/prior_art/07_rust_final_encoding_arenas.md` (§1-3, tagless-final
and the initial-encoding resolution), `.../04_racket_scribble.md`, `.../03_pandoc_djot.md`,
`.../02_jsonnet_dhall.md` (manifestation-as-library), `.../06_mlir_ods_nanopass.md` (trait/interface
dispatch, DRR), `.../05_typst.md` (targets and the HTML divergence), `.../01_lms_delite.md` (per-backend
codegen). Inclusion-not-coverage lineages are in the arc root `06_prior_art.md`.

Key underlying citations: Carette, Kiselyov, Shan, "Finally Tagless, Partially Evaluated," JFP 2009; Hofer,
Ostermann, Rendel, Moors, "Polymorphic Embedding of DSLs," GPCE 2008; the Pandoc/pandoc-types and djot.js/
djoths source; Racket `scribble/core` and `base-render`; Typst `typst-layout`/`typst-html`; MLIR Traits and
Interfaces docs; the tagless-final-in-Rust write-ups (getcode.substack.com, the two Louy2 dev.to posts) and
rust-lang/rust#81823.
