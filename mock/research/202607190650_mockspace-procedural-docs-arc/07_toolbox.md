# The toolbox: mechanisms taken from prior art, and the forks they price

**Date:** 2026-07-19
**Sources:** the eight deep passes in `prior_art/`, which carry the citations and verbatim quotes. This
document is the synthesis and does not re-cite; go to the numbered file for evidence.
**Supersedes:** the architecture recommendation at the end of `06_prior_art.md`, on one point, noted below.

## What the numbers say

Every system that fixed a core kept it small, and two arrived at the same figure independently.

| system | core size | what the number counts |
|---|---|---|
| Racket | 15 expression forms, plus module-only forms | evaluation model |
| Jsonnet | ~15 constructs after total desugaring | evaluation model |
| Pandoc | 14 block, 19 to 21 inline constructors | document structure |
| Djot | comparable, with attributes uniform on every node | document structure |
| Typst | 28 `Value` variants, `Content` and `Styles` first-class | runtime value domain |
| MLIR builtin | 2 operations | deliberately not a core at all |

The evaluation core and the document algebra are separate counts on separate axes, and a system needing both
needs both numbers. Nothing in the survey suggests either exceeds the low tens.

Costs, where anyone published one:

| thing | cost |
|---|---|
| a complete Pandoc writer | ~380 lines (Man.hs, 379, nearly all per-constructor logic) |
| four related writer variants | 1936 lines shared |
| Scribble's `@`-reader | 672 lines |
| nanopass's generation framework | ~4600 lines |
| MLIR ODS saving per operation | 10 declarative lines against ~35 hand-written |
| a minimal MLIR dialect | exactly four files |
| Pandoc's hand-written traversal | ~90 `Walkable` instances, deliberately not derived |

## Mechanisms worth taking

Each of these is a specific, sourced mechanism rather than a principle, with what it solves for us.

**Content as an arena handle, not a refcounted pointer.** Typst's `Content` is a hand-rolled manually
refcounted type-erased `Arc` with clone-on-write, and its cheap-clone property is inseparable from the heap.
The property actually needed is that content is cheap to copy and behaves like a value. An arena index gives
that for 4 bytes and a copy, which is the pattern `hilavitkutin-str`'s `Str` already uses. The semantics
transfer; the implementation inverts, and in our direction it gets simpler rather than harder, because a
generation-time arena has one lifetime and everything in it dies together.

**A borrowed, non-allocating chain for scopes.** Typst's `StyleChain` folds properties outward through a
borrowed linked list instead of merging eagerly into a map, and its own doc comment states the reasoning.
This is exactly the shape the DSL needs for bindings, `self`, and fragment parameters, and it is heap-free
inside an otherwise heap-dependent system, so the mechanism is already isolated from its host's assumptions.

**Hash-consing on construction, which makes common subexpression elimination free.** LMS nodes are case
classes looked up by structural equality at construction time, so CSE is a consequence of the constructor
rather than a pass. In our shape the arena's node constructor is the interner and there is no optimisation
pass to write. It does require an interner, which is the load-bearing dependency called out below.

**Effect dependencies as a fixed-width summary.** LMS carries read and write sets so a scheduler reproduces
an interpreter's ordering without re-deriving it, over ten fields across simple, global, alloc and control
against may, must, read and write. That is a bitflag set, not a heap structure. The persistent-list state
threading around it does not transfer; the encoding does.

**An explicit frame stack instead of native recursion.** Jsonnet's C++ evaluator runs on one, and Djot's
filter walker is a hand-written mutable-stack walk rather than a typeclass traversal. Two systems chose this
for reasons unrelated to allocation, which means the shape we need anyway is also the mainstream choice.

**A branch quota instead of a totality proof.** Zig bounds comptime with a backwards-branch quota, default
1000, raisable per site. Typst bounds document convergence at five relayout passes. Dhall instead proves
termination and pays for it by Church-encoding every recursive type and hand-transforming recursive
algorithms into fold form. For a generator inside a build, an error at a bound is worth as much as a proof
and costs a counter.

**Reuse the memoizer as the fixpoint detector.** Typst detects convergence by reusing `comemo`'s
memoization-invalidation constraints rather than building a dependency graph for the purpose. If we memoize
anything, it already knows what changed.

**Normalize, canonically encode, then hash.** Dhall's semantic hash is beta-normalize, alpha-normalize, CBOR,
SHA-256, base16, invariant under comments, formatting and variable naming. It is also the only documented,
versioned normal form in the survey, and CBOR plus SHA-256 is the one piece of any of these designs that is
already `no_std`-portable. This is the candidate answer to the deterministic-variation seed, and it doubles
as a cache key.

**The `@`-reader as a deterministic character grammar, with real escaping.** Scribble's reader is 672 lines
mapping `@cmd[datum]{text}` onto a function call. It answers the escaping question left open in
`05_correction_and_aim.md`: `|...|` escapes plus delimiter switching for literal braces. A document about
this system can therefore contain examples of this system, which is a requirement we have and had no answer
for.

**Four passes, with a persistent fixed-point traverse.** Scribble runs traverse, collect, resolve, render,
where traverse is a literal persistent-hash fixed-point loop with no mutation, and deferred values
(`traverse-element`, `delayed-element`, `part-relative-element`) produce their content in a later pass while
reading accumulated tables. That is how cross-references resolve without mutating the document, and we will
need it the moment one document refers to another's content.

**Variable-arity children without a `Vec`.** `cranelift-entity`'s `EntityList` plus `ListPool` is a 4-byte
handle against a 24-byte `Vec`. This is the concrete answer to "a node has N children" under no-alloc.

**Ship the shared representation as its own package.** `pandoc-types` is versioned separately from pandoc,
so the AST is independent of every reader and writer. Ours should be its own crate for the same reason,
independent of mockspace and of vehje.

**Total desugaring, with nothing surviving as sugar.** Jsonnet's object comprehensions collapse into one core
comprehension, and `self` and `super` are rewritten into generated bindings. The surface conveniences
(`::` chaining, `where`, the method-call form) become `Apply` and `Project`, and the core never learns they
existed.

## Targets cluster: the count of paths is not the count of formats

Two independent findings, from opposite directions. Pandoc's `Plain` is not a module at all, just Markdown
with a mode flag, and four writer variants share 1936 lines. Typst's HTML export bypasses the `Frame` layout
representation entirely, because the visual and structural intents diverge, and its own docs say perfect HTML
is therefore unreachable.

So: within a cluster, one implementation with switches. Across clusters, separate paths, and do not force a
shared one. For us that is roughly three paths, not six targets. Markdown, terminal and plain are one. HTML
is its own. Program source (Lua, C#, Rust) is a third.

This corrects a claim made earlier in this round. Writing `to_markdown`, `to_terminal` and `to_html` by hand
was described as duplication the shared representation would remove. For the first two it is not
duplication, it is one renderer with a flag, and Pandoc reached that conclusion over twenty years. The
argument for a shared representation holds where targets genuinely diverge and is weaker between neighbours.

## The forks, now priced

### Closed core against open families

This is the one that matters, and it reverses the recommendation at the end of `06_prior_art.md`.

MLIR's generic passes dispatch through traits and interfaces (`hasTrait<T>()`, `dyn_cast<Interface>(op)`),
never on a closed operation-kind enum. That is precisely what lets a dialect be added without touching
existing passes, and it is precisely what makes exhaustiveness uncheckable. A backend either implements an
interface or does not, and nothing proves it covers what it will be handed.

The two coherent designs, with the survey's numbers attached:

**Closed small core, no generation machinery.** Pandoc's shape. About 35 constructors, ninety hand-written
traversal instances, roughly 380 lines per backend, zero framework, twenty years in production.
Exhaustiveness is checkable, so refusing a document whose constructs a target cannot express works.
Extending the core means touching every backend.

**Open families with generation machinery.** MLIR and nanopass's shape. Thousands of lines of macro
framework first (nanopass: ~4600), then cheap additions indefinitely, and passes that survive new node kinds
untouched. No exhaustiveness, so the checking novelty goes away.

The earlier recommendation took MLIR's authority and chose the second. With the costs visible that looks
wrong at our size. The saving is 3.5x per operation, which pays for itself across hundreds of operations and
dozens of dialects, not across an evaluation core of fifteen forms and a document algebra of thirty that
every comparable system froze and left frozen. The framework would cost more than the hand-writing it saves,
and it would cost the one thing nothing in the survey does.

**Recommendation: closed core, hand-written backends, no generation framework.**

### How the closed core still gets an escape hatch

Scribble resolves this better than either pole, and the resolution was already implied by a finding in
`06_prior_art.md`: every shared representation in the survey needed a typed escape hatch, without exception
(MLIR's `unrealized_conversion_cast`, Pandoc's `RawBlock Format`, Typst's `html.elem`, GENERIC's
language-dependent tree codes).

Scribble's document structures are plain immutable data, closed, and `content?` additionally admits the
open-world `convertible?` protocol, where a value offers several representations and the renderer picks.
Closed everywhere, open at exactly one declared point.

That is the shape to take. The escape hatch is the open-world case, it is one case, it is typed, and
confining openness to it preserves exhaustiveness for everything else. It also upgrades the escape hatch
from Pandoc's silent-drop behaviour to Scribble's negotiation: the value offers what it can be, and the
target takes what it understands, with the mismatch visible rather than dropped.

### Final encoding against initial encoding

Settled by evidence rather than argument. No production Rust compiler uses the final encoding; rustc's HIR,
rust-analyzer, oxc and swc all use a concrete node enum in an arena indexed by `u32`. GATs do not erase
behind `dyn`, recursion forces boxing, and a term generic over its interpreter is a monomorphised function
rather than a storable value.

**Initial encoding in an arena, with one generic fold into any backend.** Backends stay trait impls, so the
open axis for outputs is preserved without the final encoding's costs.

Note what this costs, since it was overstated earlier in this round: feature checking is not free from the
type system. It is free only for terms written in Rust. Our terms are parsed at runtime, so the check is a
pass we write against a declared feature set per target. Still static, still ahead of every system surveyed,
but a pass rather than a guarantee.

### Traversal: generated or hand-written

Pandoc hand-instantiates about ninety `Walkable` instances and deliberately does not derive them, because
the generic path is four to five times slower. MacFarlane then wrote Djot with a hand-written mutable-stack
walker rather than a typeclass at all. Two designs by the same author, both rejecting reflective traversal.

With the closed-core recommendation above, generated traversal has little left to buy. Hand-written, with
the explicit stack the no-alloc constraint wants anyway.

## The interner is the load-bearing dependency

Three separate mechanisms rest on it: string storage, node hash-consing for free CSE, and the semantic hash's
canonical encoding.

`hilavitkutin-str` ships more of this than the ecosystem survey found, and the survey's conclusion that no
heap-free interner exists was wrong because it looked only at crates.io. `Str` is 4 bytes,
`#[repr(transparent)]` over `Bits<32, Hot>`, one origin bit and three reserved flag bits and a 28-bit id,
`Copy` and `Eq` and `Hash` with comparison as integer equality. Compile-time strings cost nothing: `str_const!`
emits into a linker section with a truncated FNV-1a id, with real Linux and Mach-O walkers.

What it does not ship is the arena, deliberately. `ArenaInterner` is a two-method trait
(`arena_intern`, `arena_resolve`) and `StringInterner<A>` wraps it with the const-table short-circuit. So the
handle discipline, bit layout, const path, hashing and contract are solved and heap-free; the backing arena
is ours, by design.

Two observations from reading it. Every runtime intern first walks the const section linearly, comparing hash
then content, so the cost is multiplicative in the const table's size and wants measuring rather than
assuming at our volume. And our vocabulary splits well against that design: DSL keywords, core form names and
document constructor names are literals we control and get zero-cost handles, while registry data and
namespace names from `mockspace.toml` are the genuinely runtime side.

## What is still open

**The effect intent.** A registry query inside a loop is fine to reduce for markdown. Reducing it for a Lua
target means the query ran at generation time, which is either exactly right or silently wrong, and nothing
in the design expresses which. LMS's effect summary gives the machinery to order effects; it does not decide
this.

**The variation seed's exact input.** Dhall's semantic hash is the mechanism. Whether the hashed thing is the
fragment call including arguments, the fragment identity alone, or something with an explicit seed field
decides whether changing an argument re-rolls the phrasing. The requirement is stability under exactly what
should not change prose, and the document path already failed that test this session when ordering prefixes
shifted.

**Whether renderers are library code or total functions.** Jsonnet's manifestation functions are ordinary
Jsonnet, 20 to 160 lines each, with no typed backend contract, and that is the shipped precedent for
describing outputs in the language itself. Its cost is that nothing can verify a renderer handles what it is
given. We can have both only if renderers are total functions over the closed content algebra rather than
free-form code, which is a constraint on the fragment language and should be decided rather than discovered.

**The two core lists themselves.** The evaluation forms and the document constructors. The survey says both
stay in the low tens and both stay closed. It does not say which ones.

## Where we would still be alone

Every system surveyed heap-allocates. The two exceptions share one shape: Terra's generator is
garbage-collected Lua producing code with no runtime, and Rust proc macros are a heap-using dynamic library
emitting code that can be `no_std`. Generator-with-heap and output-without-runtime is the entire shipping
precedent.

A generator without a heap has none. The apparent reason is that nobody needed one, not that it cannot be
done, and the workspace ships more of the necessary machinery than the open ecosystem does. But it remains
the part with no map, and it should not be treated as the easy part merely because the constraint is
familiar.
