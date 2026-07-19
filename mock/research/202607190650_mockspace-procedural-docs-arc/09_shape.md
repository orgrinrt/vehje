# The shape, and the mockspace DSL held against it

**Date:** 2026-07-19
**Rests on:** `08_decisions.md` (the four settled calls), `07_toolbox.md` (the mechanisms and their sources).
**Status:** a proposal concrete enough to attack. The two core lists in particular are this document's own
work and carry no external authority.

## The shape in one page

Text in one of several surface grammars is read into a concrete node tree held in an arena. One generic fold
walks that tree into any target implementation. A target is three things: a staging policy, a declared family
set, and a total renderer over the families it declared. Nothing else varies.

```
surface grammar  →  arena node tree  →  fold  →  target
   (per language)      (core + families)          (policy + family set + renderer)
```

The pieces, by role rather than by crate name, since naming is not settled:

**The arena and node tree.** A concrete node enum indexed by a 4-byte id, following what rustc's HIR,
rust-analyzer, oxc and swc all do. Variable-arity children use the `EntityList` plus `ListPool` shape, a
4-byte handle rather than a 24-byte `Vec`. Node construction goes through an interner, so structural equality
deduplicates on the way in and common subexpression elimination is a property of the constructor rather than
a pass.

**The families.** Each is a Rust trait. Its required interfaces are supertraits. A family set is a type-level
cons-list carrying `Contains` and `ContainsAll`, which is `hilavitkutin-api`'s `AccessSet` machinery applied
to a different noun. Families come in two flavours, and the distinction matters more than it first looks:

- **Node-contributing families** add constructors to the tree. The document algebra is one.
- **Value-contributing families** add types and functions but no constructors. Registry access is one: it
  contributes a `Rows` value and operations over it, all reached through ordinary `Apply`. It does not grow
  the core at all.

Most families should be the second kind. A family that wants new node kinds should have to justify why its
concept cannot be a value plus functions, because the second kind costs nothing and the first costs an arm in
every fold.

**The targets.** A staging policy (reduce everything, or preserve control flow), a declared family set, and a
renderer total over that set. Adding a target is an implementation, never a change to the core.

**The interner.** `hilavitkutin-str` supplies the handle discipline: `Str` is 4 bytes, `repr(transparent)`
over `Bits<32, Hot>`, one origin bit and three reserved and a 28-bit id, with compile-time literals costing
nothing through the linker-section path. The backing arena is ours to write, which the crate intends. Three
separate mechanisms rest on it: string storage, node hash-consing, and the semantic hash used for variation
seeding.

## The core forms

Nine. The survey's comparable numbers are Racket at 15 and Jsonnet at about 15, both of which carry
module-and-object machinery this does not need.

| form | shape | why it cannot fold into another |
|---|---|---|
| `Lit` | scalar constant | the leaves |
| `Ref` | name resolved to a binding slot | variable reference |
| `Let` | bind a name over a body | needed by fragments and by `for` |
| `Apply` | callee plus arguments | operators are applications; all family functions arrive here |
| `Project` | base plus name | namespace to row, row to field, decided by a type rule at check time |
| `Select` | condition, then, else | the conditional expression, usable inline |
| `Iter` | binder, source, body | per-row output in a chosen shape, the anchoring use case |
| `Seq` | ordered nodes | the monoid join; how `Iter` accumulates |
| `Lambda` | parameters plus body | fragments with declared parameters |

Everything the surface offers desugars into these and nothing survives as sugar, which is Jsonnet's
discipline. The `::` chaining, `where`, and the method-call form are all `Apply` and `Project` by the time
the fold sees them.

`Iter` earns its place precisely and narrowly. A filtered projection already renders a table with no loop:
`task::where(crates ~ self)::select(id, what, milestone)` needs no iteration. `Iter` exists for when the
per-row output is prose or a chosen shape rather than a table cell.

## The document algebra

A node-contributing family. Starting proposal, deliberately smaller than Pandoc's 14 block and 21 inline,
because Pandoc serves thirty input formats and this serves documentation.

Block: `Para`, `Plain`, `Heading`, `CodeBlock`, `Quote`, `BulletList`, `OrderedList`, `DefList`, `Table`,
`Rule`, `Div`, `Raw`.

Inline: `Text`, `Space`, `Break`, `Emph`, `Strong`, `Code`, `Link`, `Span`, `Raw`.

Twenty-one against Pandoc's thirty-five. Dropped: `LineBlock`, `Figure`, `Note`, `Cite`, `SmallCaps`,
`Superscript`, `Subscript`, `Quoted`, `Strikeout`, `Underline`, `Math`. Each of those is a candidate to argue
back in, and `Note` and `Math` are the two most likely to be genuinely needed.

Following Djot rather than Pandoc, attributes are uniform on every node rather than available only on `Div`
and `Span`. Pandoc grew `Div` and `Span` as generic containers *after the fact* precisely because attributes
were not uniform, and its whole filter ecosystem is built on that retrofit.

Content is a value, per Typst, and it is a monoid. That is the enabling detail rather than an incidental one:
`Iter` accumulating markup only works if there is an associative join with an identity. Where Typst refcounts
a heap-allocated `Content` with clone-on-write, this uses an arena index, which is 4 bytes and `Copy` and
gets value semantics for free because a generation-time arena has one lifetime.

## The target model

```
Target:
    Supports: FamilySet        (type-level, checked by ContainsAll)
    Policy:   Reduce | Preserve
    render:   total over Supports
```

Emission requires proof that the document's family set is within `Supports`. For statically known terms that
proof is a compile-time bound. For parsed documents the family set is a runtime bitmask computed during
parsing and checked against the target's const bitmask, with typestate making the check unskippable:
`emit` accepts only `Checked<T>`, and `Checked<T>` has one constructor.

Targets cluster, and the count of paths is smaller than the count of formats. Markdown, terminal and plain
are one implementation with switches, which is what Pandoc concluded when `Plain` turned out to be Markdown
with a mode flag. HTML is its own, which is what Typst concluded when HTML export had to bypass the `Frame`
representation entirely. Program source is a third. Three paths.

## Bounded evaluation without a totality proof

An explicit frame stack rather than native recursion, which is what Jsonnet's evaluator and Djot's filter
walker both use for reasons unrelated to allocation, and which a fixed-capacity arena needs anyway. A
backwards-branch quota bounds iteration, as Zig's comptime does at a default of 1000, raisable per site.
Depth is bounded by the frame arena's capacity.

Dhall's alternative, proving termination outright, costs Church-encoding every recursive type and
hand-transforming recursive algorithms into fold form. For a generator inside a build, an error at a bound is
worth as much as a proof and costs a counter.

## Cross-document references

Scribble's four passes: traverse to a fixed point, collect, resolve, render, where traverse is a persistent
fixed-point loop with no mutation and deferred values produce their content in a later pass while reading
accumulated tables. This is needed the moment one document refers to another's content, which mockspace
already does through the crate index.

Typst's convergence trick applies on top: reuse the memoizer's invalidation constraints as the fixpoint
detector rather than building a dependency graph for the purpose, bounded at a small pass count.

---

# Held against the mockspace DSL

The shape above is general. This section checks it against what mockspace actually needs, using the real
registries in `ikiuni_renderer/mock/registry/`: fifteen namespaces including `task`, `law`, `tripwire`,
`bench`, `constant`, `ruling`, `data_shape`, `technique`, `spike`, `equation`, `facet`, `field`,
`abstraction`, `reference`.

## The anchoring use case, concretely

A real `task` row carries `id`, `kind`, `milestone`, `what`, `blocked_by`, `crates`, `provenance`, `note`,
and sometimes `options`. The maintainer's stated case was "for each thing in tasks that mentions crate self,
write a table row in format xyz columns."

````markdown
```query
for t in task::where(crates ~ self) {
    "| {t::id} | {t::what} | {t::milestone} |"
}
```
````

Lowering, to check the core covers it: `Iter` with binder `t`, source `Apply(Project(task, where), [crates ~ self])`,
body `Seq[Lit, Project(t,id), Lit, Project(t,what), Lit, Project(t,milestone), Lit]`. Nine forms, all present,
nothing missing. The `~` is `Apply` on a family function, and `self` is a `Ref` resolved from the scope chain.

Inline, the code-span form:

```markdown
the working set is bounded at `@constant::froxel_range::value`
```

Lowering: `Project(Project(constant, froxel_range), value)`. One form.

## The scope chain

`self` is what makes a fragment reusable: the same block in any crate document resolves to that crate.
Without it the filter is hand-written per document, which is the duplication the whole exercise exists to
remove.

The mechanism is Typst's `StyleChain`: a borrowed, non-allocating linked list that folds outward rather than
merging eagerly into a map. Bindings from `Let`, the `Iter` binder, fragment parameters and the ambient
document context all live on one chain, and lookup walks outward. It is heap-free inside a system that is
otherwise heap-dependent, which is exactly the property that makes it portable to here.

## Fragments as a registry namespace

The `self` mechanism generalises into a namespace whose rows are parameterised prose. A fragment row declares
parameters and a body; a document calls it with arguments; the phrasing is maintained in one place.

```toml
[[fragment]]
id = "crate_pending_work"
params = ["crate", "band"]
body = """
{crate::name} has {task::where(crates ~ crate, milestone ~ band)::count()} open items in {band}.
"""
variants = [
  "{crate::name} carries {n} open items in {band}.",
  "{n} items remain open against {crate::name} in {band}.",
]
```

This is `Lambda` plus `Seq`, with the variant list selected by the semantic hash of the call per decision 4.
It needs no new core form, which is the test a family should pass.

## The registry family contributes values, not nodes

`where`, `select`, `count`, `sort` and the match operators are functions over a `Rows` value, reached through
`Apply`. The family adds a type and a function table and grows the core by zero.

That is the pattern to hold every future family to. The document algebra genuinely needs constructors because
its values *are* structure. Registry access does not, and neither will most things.

## Where the shape and mockspace disagree

Three real frictions, stated rather than smoothed.

**Templates are data, so the strongest proof degrades.** Decision 2's compile-time family proof applies to
terms written in Rust. `.md.tmpl` files are read at run time, so their family set is a runtime bitmask and the
guarantee is an unskippable runtime check rather than a compile error. This is still ahead of every surveyed
system, none of which check at all. It is worth an hour on whether compiling templates ahead of time into a
checked form closes the gap, because if it does, the design gets meaningfully stronger and the answer is
probably cheap.

**mockspace is `std` and the language is not.** mockspace today depends on `mockspace-lint-rules`,
`libloading`, `tree-sitter`, `toml_edit` and `serde`, with no stack dependency at all. The `no_std` and
no-alloc discipline applies to the extractable language crates, and mockspace consumes them across a
boundary. That is consistent with the stated aim of extracting this for reuse, but it means the boundary is
real and needs naming rather than assuming.

**References already nest inside registry data.** This one was found by reading the real rows rather than
reasoning about them, and it is the most consequential of the three. A `task` row's `crates` field literally
contains `["{{ crates::world }}", "{{ crates::store }}"]`, and `provenance` contains
`["seed::ROADMAP::98"]`. So references do not only appear in templates. They appear inside the data the
evaluator reads, which means resolution is recursive: evaluating `t::crates` yields values that are
themselves unevaluated references.

The consequences are not small. Evaluation order matters, so the effect ordering from LMS's summary encoding
is needed sooner than expected. A reference cycle across registry rows is now possible and must be detected
rather than hung on, which the frame-stack depth bound partly covers and a proper cycle check covers
properly. And the semantic hash for variation seeding must be taken over the *normalized* form, after nested
references resolve, or two calls that differ only in unresolved spelling will seed differently.

## What the DSL needs that the general shape does not provide

**The `@` escaping rules**, taken from Scribble's reader: `|...|` escapes plus delimiter switching for
literal braces, in a deterministic character grammar that ran to 672 lines there. This is what lets a document
about this system contain examples of this system, which is a hard requirement given the documentation being
generated is partly about the generator.

**Two surface forms with a principled split.** Inline code spans carry expressions; fenced blocks carry
statements. The split is not convenience: a loop cannot render mid-sentence, so control flow is inherently
block-level, while a conditional expression stays an expression and stays inline.

**A `query` subcommand sharing the exact template grammar**, so the same string can be tested at a prompt and
pasted into a document. This falls out of the front end being one thing rather than two.

## The open questions this shape does not close

The document algebra's exact membership, particularly whether `Note` and `Math` come back in.

What a registry query *means* inside code destined for a runtime target. Decision 1 makes it expressible and
checkable through the target's policy and declared predicates; it does not say which answer is right, and
the recursive-reference finding above makes the question sharper rather than softer.

Whether ahead-of-time template compilation lifts the parsed-document check from runtime to compile time.

Whether an explicit variation seed can be expressed with zero syntax when unused, which is the condition the
maintainer attached to having it at all.
