# The completeness census: the whole authoring grammar against the Core

**Date:** 2026-07-25
**Status:** research. A completeness audit, not a design round.
**Why it exists:** the bar for the first-party language is the complete intended language, not a runnable subset. A slice that evaluates arithmetic and recursion proves the seams; it does not discharge the goal. This census walks the normative grammar production by production and records, for each, whether the framework can express it today, what it erases into, or what is owed. The output is the exact owed list, which is the useful thing.

The normative grammar is `original-docs/CLAUSE_EBNF.md`, the shipped parser's spec from before the framework's redesign. It stands as the benchmark: Rust-shaped items and expressions, generics with trait bounds, associated types, procedural macros, patterns, attributes, plus the items the original target needed. The redesign changed what vehje is, not what the language has to be able to say.

The interesting result is that most of the grammar never reaches the Core at all, and the part that does reaches it through four owed mechanisms rather than a long tail of missing forms. Three of the four already have a home in the design; one of them, non-local control flow, has a home nobody has yet noticed it fits.

## The four erasure tiers

A production reaches the Core only if it survives every earlier pass. Sorting the grammar by which pass consumes it collapses most of it before the twelve forms are in play.

**Resolve erases naming.** `mod`, `use`, `UseTree`, multi-segment `Path`, `Visibility` and its `pub(crate)` forms, and path-relative `self`/`super`/`crate` are all name-resolution vocabulary. Resolution turns a path into a binder identity, and binder identity is a `Sym`. Nothing about module structure survives into an evaluation form.

**Macro expansion erases macros.** `macro` declarations, item-position and expression-position `MacroInvoke`, the `$` interpolation sigil, and the `quote!` body are consumed by the compile-stage discharge of the handler discipline. A macro produces surface syntax that is re-lowered; no macro construct exists downstream of expansion. This is worth stating precisely because it is where the census meets an owed axis, below.

**Monomorphisation erases types.** `Generics`, `TypeParam`, `ConstParam`, `BoundList`, `WhereClause`, `TraitDecl`, `ImplBlock`, associated `type` and `const`, supertrait lists, turbofish and `PathGenerics`, and the whole of `TypeExpr` disappear. A generic function becomes one concrete lambda per instantiation; a trait-method call becomes a direct application of the resolved impl. The Core is untyped by construction, which is prove-then-erase working exactly as designed: the type system is the verification layer and the residual carries no proof. `dyn Trait` being reserved-but-unproductive in the original grammar is fortunate rather than accidental, since compile-time dispatch is the framework's posture too, so the census owes no trait-object mechanism.

**Target lowering erases attributes.** `#[cfg]` prunes before anything else; the target-specific attributes are read by the target that cares and never become evaluation.

What remains after those four is expressions, and expressions are where the census earns its keep.

## What reaches the Core, and how

The mapping below is complete for the expression and pattern grammar. Entries marked expressible have a lowering today; entries marked owed name what is missing.

| Surface | Core form | State |
|---|---|---|
| integer, boolean, string literal | `Lit` | expressible |
| float literal, char literal | none | owed (see below) |
| identifier reference | `Var` | expressible |
| `let p = e;` then rest of block | nested `Let` | expressible, patterns owed |
| block with statements and a tail | nested `Let`, tail is the value | expressible |
| `if` / `if let` / `else` | `If` | expressible |
| `match` | `Match` | owed (pattern representation) |
| `for p in e { b }` | `Iter` | owed (evaluation, patterns) |
| `while` / `loop` | `let rec` plus `Apply` | owed (via the exit mechanism) |
| `return`, `break`, `break v`, `continue`, `?` | none | owed (see below) |
| method call | `Apply` after trait resolution | expressible |
| field access `a.b` | `Project` | owed (evaluation) |
| index `a[i]` with a computed index | none | owed (see below) |
| binary and unary operators | `Raw` family operation | expressible |
| `&e`, `&mut e`, `&T`, `&mut T` | erased to the pointee | expressible |
| range `a..b`, `a..=b` | `Raw` producing a sequence | expressible |
| closure, `move` closure | `Lambda` | expressible |
| struct literal, `..base` tail | `Raw` producing a record | owed (compound return) |
| enum variant construction | `Raw` producing a tagged value | owed (compound return) |
| tuple literal, unit | `Raw` producing a record, `Lit(Unit)` | owed, expressible |
| string interpolation | `Interp` | owed (evaluation) |
| keyword call arguments | reordered to positional in resolve | expressible |
| assignment and compound assignment | rebinding, or a place effect | conflict (see below) |

## The four things owed

**One. Compound operands and returns across the host boundary.** A family operation cannot yet receive or return anything but a scalar or a string, so no program can build a record, a sequence, a tuple, or an enum variant. Since content-as-values makes construction a family concern and leaves the Core as the eliminator algebra, this single escape gates every compound value in the language. It is the widest unblock in the census and it is already the standing next increment. Everything in the table marked "compound return" resolves with it, and `Project` becomes implementable the moment records exist to project from.

**Two. Non-local control flow, which `Handle` already covers.** `return`, `break`, `break` with a value, `continue`, and `?` all exit a construct from inside it, and the Core has no form that does that. The mechanism it wants is the twelfth form. An early exit is an operation, the enclosing function or loop body is the handled computation, and the handler discharges by yielding the value without resuming. That is the canonical one-shot effect, and it is the cheapest discharge `Handle` supports: it never resumes, so it needs none of the bounded multi-shot budget machinery that the hard case does.

Two consequences follow. `loop` and `while` need no new Core form either, because an unbounded loop is a recursive binding applied to itself with the exit expressed as the same effect, and recursive bindings evaluate today. And `Handle`'s clause representation is deferred in the Core with an explicit note that it lands with its first consumer; this is that consumer, and it arrives asking for the easy half rather than the hard one. That ordering is favourable and was not visible before walking the grammar.

**Three. The pattern representation.** `Match` stores its arms as a bare node list and defers what a pattern is. The grammar needs the full Rust pattern language: wildcards, literals with ranges, bindings, paths, tuple-struct and tuple forms, references, rest, alternatives, and guards. Patterns also appear outside `match`, in `let`, in `for`, and in closure and function parameters, so the representation is load-bearing well beyond one form. This is a Core design round whose trigger has fired.

**Four. The binding-time axis, which is what makes macros real.** The original grammar has `macro` as a first-class item with a signature and a mandatory body returning a type. That is a function evaluated at a compile stage, which is precisely a binding-time coordinate: the operation is discharged by whichever stage provides its handler. The axis exists in the IR and the check pass hard-codes it to a constant rather than computing it. So macros are not blocked on a missing Core form; they are blocked on an axis the graded spine already declares and does not yet populate. That connects the language bar directly to an item already on the framework's own owed agenda, which is the kind of coincidence worth trusting.

## Two smaller forks and one genuine conflict

**Dynamic projection.** `Project` carries a static string key, which serves field access and named member access. Indexing by a computed value has no form. Either the Core grows a projection whose key is a node, or dynamic indexing is a family operation. The first reading says the Core owns elimination and `Iter` already eliminates sequences, so a dynamic key belongs beside the static one. The second says the family introduced the sequence and may therefore eliminate it. Both are coherent; the fork is real and belongs in the Core round that settles patterns, since both touch what elimination means.

**Float and char literals.** The literal set is unit, boolean, integer, and interned string. The grammar has floats with suffixes and character literals. A character is its code point once types erase, so integer carries it, at the cost of the family no longer being able to tell a character from a number without the type it just erased. A float has no such reading. Either the literal set grows, or floats are a family-introduced value like records are. The determinism posture argues for deciding this deliberately rather than by default, since a float literal's representation is exactly where reproducibility is won or lost.

**Mutation is the one real conflict.** The grammar has `let mut`, mutable struct fields, the full compound-assignment operator set, `static mut`, and `&mut`. The current design holds values immutable, and that is not incidental: immutability is what lets the lease proof discharge statically and what removes the need for a collector. So the benchmark language and the settled architecture genuinely disagree here, and the disagreement has to be resolved rather than papered over.

The resolution that keeps both is that most `mut` is not mutation. A local binding reassigned inside a block is rebinding, and rebinding is what `Let` already does: `x += 1` becomes a fresh binding of `x` to the result, in scope for the remainder. That is faithful to the source's meaning and costs nothing. What it does not cover is a genuine place: a field of a value someone else can observe, or a `static mut`. Those are a runtime effect, which is to say a family operation the host handles, which is the same discipline everything else routes through. The census's position is therefore that `mut` splits: local rebinding erases into `Let`, and genuine places become effects. That preserves the grammar without touching the immutability the architecture rests on.

## What this reorders

The compound escape stays first, because it unblocks the most. But it must be designed together with the value representation rather than before it, since the decision about how a produced value is represented is the same decision as how it is interned, deduplicated, and reclaimed. Building the value arena and then revisiting its representation would be building it twice.

`Handle`'s one-shot discharge moves up, because it turns out to gate a large and unglamorous part of any real program, and because it arrives needing only the tractable half of a mechanism whose hard half is still open. The pattern round follows, then the binding-time axis, which is where the language stops being an expression evaluator and starts being the metaprogrammable thing the grammar describes.

The front-end half remains parallel and unblocked. Generics, traits, associated types, inference, and monomorphisation all erase before emission, so none of them wait on any of the above.

## Where the domain-flavoured items land

The grammar carries items the original target needed: `event X for T`, `expect` and `actual`, `sealed`, the bind-target colon on a struct declaration, and the `extern` family. They are worth a paragraph because they look like they need bespoke machinery and do not.

`event X for T { ... }` registers a named handler for a named operation, which is the handler discipline with surface syntax on it. `expect` and `actual` declare an obligation and its provider, chosen at whichever stage supplies it, which is a binding-time coordinate. `sealed` is a coherence restriction the checker enforces and erases. The bind-target colon is a bound, erased with every other bound. `extern` declares host-provided scopes, structs, traits, impls, and functions, and a host-provided operation is a family operation whose handler is the host.

So the domain items are the framework's own two mechanisms wearing a target's vocabulary, which is a good sign for the claim that the framework owns the substrate and the consumer owns the surface.
