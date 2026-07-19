# Syntax correction: three contexts, two grammars

**Date:** 2026-07-19
**Source:** the maintainer, during panel one.
**Supersedes:** the parts of `09_shape.md` that treat `{{ }}` as a live surface, in particular the "References
already nest inside registry data" paragraph in its disagreements section, which describes the current
implementation rather than the proposal. Written as a separate file rather than an edit so that panel one's
members reviewed a stable artefact.

## The rule

Three contexts, two grammars, no per-context dialect.

**A field declared as holding references only** (`crates`, `provenance`, `blocked_by`) carries bare
references with no delimiters. The field's declared type names the target, so `crates` holds
`crates::world`, not `{{ crates::world }}` and not an opaque string.

**A field holding prose that may contain references or expressions** (`what`, `note`, a fragment's `body`)
uses the markdown grammar: fenced blocks for statements, inline code spans for expressions. The maintainer's
reasoning is that such bodies exist only to be embedded procedurally into markdown documents, so they are the
same kind of thing as a template body and should not have a dialect of their own.

**A `.md.tmpl` file** uses the same markdown grammar.

So there is one prose grammar wherever prose appears, and it does not matter whether the bytes live in a
template file or inside a TOML string.

`{{ }}` is not part of the proposal at any layer. It is the current implementation's surface and is
legacy-to-remove.

## What this resolves

**Pesce's three-populations finding is dead.** It observed that `{{ }}` would carry placeholders, registry
references and query expressions disambiguated only by lookup order, with a fixed eight-field placeholder
struct silently shadowing a colliding namespace. There is no shared delimiter surface for that to happen in.

**Muratori's over-matching finding is fixed by the same change that fixes the syntax.** `~` degraded into
`cell.contains(&want)` (`mockspace/src/registry/resolve.rs:279`) because `crates` is declared
`type = "string[]"` and its values are text. With the field typed as a reference list, membership is on ids,
and `store` cannot collide with `store-policy`. The syntax simplification and the correctness fix are one
change, not two.

**Cycle detection gets cheaper.** With reference-bearing fields declared as such, the reference graph is known
from the schema before any evaluation runs, rather than discovered by parsing values. Cycles are a graph
property checked once, not a runtime trap guarded by a recursion bound.

## The implication worth taking further

If a prose field uses the same grammar as a template, then a prose field **is a document fragment, not a
string**. `what`, `note` and a fragment `body` are the same kind of thing as a template body.

Consequences, each of which the design should carry deliberately:

A prose field has a family set. It participates in whatever target-support check the design lands on. A
`what` containing a table can be refused by a plaintext target, which today nothing knows.

The current flattening of cells to rendered markdown in `resolve_data`
(`mockspace/src/registry/resolve.rs`, near line 662) is then the wrong shape rather than an implementation
detail. It is also the direct cause of the substring-matching bug above, since matching happens against
rendered text.

The fragment registry proposed in `05_correction_and_aim.md` stops being a new mechanism and becomes a
recognition: every prose field already is a fragment body. What a fragment namespace adds is declared
parameters and phrasing variants, not the idea of parameterised prose itself.

## What the field type vocabulary needs

Three shapes rather than one:

| shape | example fields | what it means |
|---|---|---|
| `ref<ns>[]` or `ref<root>[]` | `crates`, `blocked_by`, `provenance` | references only, typed by target |
| `content` | `what`, `note`, fragment `body` | prose in the markdown grammar, a document fragment |
| scalar | `id`, `kind`, `status` | opaque text, never re-parsed |

Two distinct reference target kinds exist and the type must name which: a namespace row (`crates::world`)
and a source line under a `[ref.roots]` root (`seed::ROADMAP::98`).

Today every one of these is `string` or `string[]`, which is why all three behave the same and all three are
wrong in the same way.
