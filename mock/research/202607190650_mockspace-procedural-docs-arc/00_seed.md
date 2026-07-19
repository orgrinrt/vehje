# Procedural documents: the language, its IR, and where it lives

**Date:** 2026-07-19
**Status:** seed for iteration. Nothing here is committed. No code has been written against it.
**Purpose:** capture a long design conversation before it is lost, so it can be stress-tested by
independent readers and sharpened, and so that no work lands in `vehje` that turns out to be redundant.

## Why this exists

Mockspace generates documents from templates. Templates currently carry two mechanisms over the same
braces: placeholder substitution (`{{project_name}}`) and registry reference resolution
(`{{ law::keys }}`). Both were built incrementally, and a query subcommand was then added over the second.

The ambition that broke the incremental approach is **procedural documents**: prefer querying the
registry to restating it, so a fact is maintained in exactly one place. A crate document should not
re-explain what a law is; it should reference the row that states it, and stay current by construction.

Taken seriously, that ambition needs iteration, conditionals, bindings, and scope. Which is to say it
needs a language, and the incremental path was building one accidentally, one verb at a time.

## The claim this rests on

**A template is a program whose output is text.**

`vehje` is a language whose output is another language's source. Emitting markdown is another backend. The
one real difference is that documents must *evaluate* — a query runs against live registry data mid-render
and its result is interpolated — while vehje only ever *emits*.

That difference is smaller than it looks: any language with compile-time constants needs an evaluator, and
a language that emits source needs one more than most. Building it with a real consumer beats designing it
speculatively.

## The architecture

```
many syntaxes  →  one IR  →  many outputs
```

Stated by the maintainer and load-bearing throughout. vehje was written as one syntax to many outputs; the
generalisation expands the input side, which was always the overlooked half.

```
text (a fragment in a document, or a string at a prompt)
  → lex        tokens
  → parse      AST          per-language vocabulary, arena
  → lower      IR           the shared semantic core
  → resolve    bindings     scopes, names to slots
  → check      typed IR     diagnostics
  → evaluate   Value
  → render     text         per output format
```

The registry DSL is a **second front end**, not a second language: it lowers to the same IR that vehje's
own syntax lowers to. mockspace therefore adopts vehje's semantic core without adopting its surface
syntax, which keeps vehje's grammar churn away from a tool nine repositories depend on.

## Surface syntax

Two forms, split by a principle rather than by convenience:

**Inline carries expressions. Fences carry statements.**

A loop cannot render mid-sentence, so control flow is inherently block-level. A conditional *expression*
is still an expression, so it stays inline.

### Inline: a markdown code span

    the working set is bounded at `@constant::froxel_range::value`
    `@(tripwire::where(!enforced_by)::count() > 0 ? "some are ungated" : "all are gated")`

Native markdown. The raw template renders as code rather than as noise, editors highlight it, and a reader
who has never heard of this system sees something plausible rather than a broken incantation.

### Fenced: a code block

    ```query
    for row in tripwire::where(!enforced_by) {
        "- **{row::name}** owes a gate at {row::milestone}"
    }
    ```

Block-level output: tables, lists, generated sections. Full language, control flow included. Obsidian's
dataview arrived at the same two-form split, which is corroboration rather than imitation: the split falls
out of markdown being block-structured.

### What was rejected

- `{{ … }}` for both. Once blocks exist, `{{ for x in y { … } }}` closes four braces meaning two different
  things.
- Block tags, `{{#for}} … {{/for}}`. Familiar from the template-language landscape and disliked on
  ergonomic grounds; they read badly interleaved with prose and are worse at a glance.
- A custom sigil delimiter such as `@{ … }`. Superseded by code spans, which are already markdown.

## Settled

**Node vocabulary is a compile-time type parameter, not a runtime tag.** The earlier proposal packed
`(language, node)` into a `Uint<12>` so several languages could share one IR at runtime. That premise was
wrong for the actual case: a build loads one input language and selects one output context. Extensibility
belongs in the type system.

```rust
Ir<V: Vocabulary>
```

Different languages are different instantiations, monomorphised, no runtime dispatch. A coproduct
vocabulary stays available for a platform that genuinely wants two at once, without every consumer paying
for the possibility.

**Projection is one node with a type rule.** `law::keys::statement` lowers to `Project { base, name }`
throughout, and the type rule decides what projection means per value type: namespace to row, row to
field. Since it resolves at check time to a concrete access, the generality costs nothing at runtime. The
registry DSL has no reference implementation and has never been formalised, so it is free to be shaped by
what lowers cleanly.

**Storage is a trait with a const-generic default.** The arena is host-provided so the language fits
whatever flow, with a default implementation for consumers that want one. hilavitkutin already expresses
these contracts statically; reinventing them would be the reinvention this workspace exists to prevent.

**vehje is an IR with grammar plugins on both sides**, not a framework for building languages. The
framework-only option is a commodity: everyone has written a lexer framework and ours would be
indistinguishable. The IR-with-plugins option is where the stack's identity shows up.

## Open: the value domain

The one question deliberately left undecided, because the first framing was wrong and a replacement should
not be improvised.

**The error:** the value domain was sketched as scalars plus opaque `HostRef` handles, so that the language
core would never own unbounded data. That imports the **Lua frame**: an application lends data to an
embedded script across a boundary.

**Why it is wrong here:** there is no host. The registry is not foreign data on loan; it is the program's
subject. Templates are not scripts embedded in mockspace; they are the documents. The boundary does not
exist in the domain, and drawing it makes the relational half a bolt-on to a scalar language when relations
are the point.

**The direction, not yet a decision:** the value domain is **columnar natively**. Records and columns are
first-class values that the type system knows. Three things collapse into one if that holds:

- a "host data provider" becomes a data source, in the same sense that source text is a source
- "host functions" become the language's own operators over its own data model, so `where` and `select` are
  not foreign calls bolted on
- "opaque values" disappear; a table is a value with a type

It also merges with the storage question: if the value domain is columnar and hilavitkutin already
expresses columns and storage contracts, then the arena and the data model are one thing rather than two.

**What must be established before this is decided:** whether hilavitkutin's `Column` and storage contracts
are separable from its scheduler. The runtime is not ready for use; the contracts may still be usable. If
they are not separable, the columnar direction needs a different foundation and this section is reopened.

## The parts

| # | part | where | state |
|---|---|---|---|
| P1 | generic lexer over token kind | vehje-lex | exists (25 pub fns, 8 test files), needs the type parameter |
| P2 | fragment sources, host span mapping | vehje-ir | new |
| P3 | precedence driver | vehje-syntax | new |
| P4 | IR node vocabulary | vehje-ir | 9 coarse variants today; the design work |
| P5 | value domain | vehje-ir | open, see above |
| P6 | resolve against real nodes | vehje-resolve | scopes exist (32 pub fns), binding does not |
| P7 | typecheck | vehje-typecheck | near-empty (4 pub fns) |
| P8 | interpreter | new | does not exist; vehje wants one for const evaluation |
| P9 | data sources and operators | new | reshaped by the value-domain question |
| P10 | text backend | new | beside the Clausewitz backend |
| P11 | DSL front end | mockspace | new |
| P12 | mockspace tail: registry as tables, `query` | mockspace | partly exists |

P1 to P3 are independent and unblock everything. P4 gates P6 to P8. P11 and P12 are the only parts
specific to mockspace.

## What must be stress-tested before any vehje work

The point of this document. Each of these could invalidate a chunk of the plan, and each is cheaper to
answer now than after the code exists.

1. **Is hilavitkutin's storage contract separable from its scheduler?** Gates P5, and P5 gates most of
   the rest.
2. **Does the two-form syntax survive contact with real documents?** Convert one crate document by hand,
   fully, and read it back. If it becomes a rendering of the tables it queries, the ambition needs a
   stated boundary, not more verbs.
3. **Is "one IR, two syntaxes" real or aspirational?** The registry DSL is expression-and-query shaped;
   vehje's language is statement-and-item shaped. If lowering both to one vocabulary requires nodes that
   one side never emits, the sharing is nominal.
4. **What does the type system have to know?** Tables, rows, columns, scalars, and the projection rule.
   If that is a different type system from the one vehje needs for its own language, P7 is two pieces of
   work wearing one name.
5. **Does no-alloc survive iteration?** A loop over 2,687 rows producing text has to put the text
   somewhere. Whether that is a caller-provided sink, a fixed arena, or streaming output changes the
   interpreter's shape.
6. **Is the interpreter the same thing vehje needs for const evaluation?** Claimed above, unverified. If
   const evaluation wants a different execution model, P8 is not shared work.

## Prior art, and why it is not adopted

**MLIR** is the closest conceptual match: one infrastructure, extensible dialects. It is C++ inside LLVM,
assumes a heap and a machine-code-shaped world, and its Rust story is thin bindings over a C API. Adopting
it means a docs tool depends on an LLVM build. Worth reading before designing the IR; not adoptable.

**Racket's `#lang`** is grammar plugins onto a shared core, and is the other serious precedent. It assumes
a runtime with a heap and a garbage collector.

**Rust ecosystem**: `rowan` is rust-analyzer's concrete syntax tree and allocates; `salsa` is incremental
computation rather than an IR; `cranelift` is machine-level; `egg` solves a different problem.

Nothing occupies the niche this would: extensible IR, `no_std`, no-alloc, source-to-source. That gap is
real rather than merely unexplored, which is the argument for building rather than adopting, and the
argument for keeping the core small enough that the claim stays true.

## Context: why vehje stalled, and what is true now

Worth recording because it is the risk that would repeat.

vehje's last seven design rounds advanced the language zero times: three documentation and audit rounds,
three test-rehab rounds chasing `arvo` Round D and `Maybe` surface drift, and one build-config round after
a failure that was transitive through arvo. `arvo` landed `refactor!: migrate arvo containers and
algorithms off Cap to Capacity` on 2026-05-30, the day after vehje's last round. vehje did not stop at a
blocker; its forward motion was consumed by substrate churn.

As of 2026-07-19 that has stopped. `notko` has not moved in seven weeks. `arvo` has been additive only
since 2026-06-07, with both breaking changes six weeks behind. The full vehje workspace builds clean and
passes 129 tests against current `arvo`, `notko` and `hilavitkutin`. hilavitkutin is highly active, but in
the engine rather than in anything a front end touches.

The lesson to carry: the front end depends on `notko` and `arvo`, and those are the two that have gone
quiet. That is what makes resuming reasonable now and did not a month ago.
