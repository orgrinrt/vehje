# Purity, totality, effects, and the compile-runtime split

**Date:** 2026-07-20
**Domain:** what reduces when, and what guarantees a language can carry as a property of its type system rather
than a runtime check. Totality and termination, content-addressed determinism, laziness and thunks, effect
classification as read/write edges, binding-time and the compile-versus-runtime split, and lifetime/lease
contracts.
**Synthesised from:** `prior_art/02_jsonnet_dhall.md`, `prior_art/01_lms_delite.md` (the effect system),
`prior_art/08_terra_procmacro_comptime.md` (eager specialization, comptime), `prior_art/04_racket_scribble.md`
(phase separation), and the compile-versus-runtime and lease material in
`runtime-value-transfer-study/dotnet_clr.md`.

## Why this domain matters

A framework that splits work between a dev-time compiler and a runtime, and that wants to prove properties
ahead of execution, is making three separable decisions: what evaluates when (binding-time), what is
guaranteed by the type system versus checked dynamically (totality, purity, effects), and how a value's
lifetime is contracted across the split. The corpus has clean, load-bearing prior art for each, and the
strongest single lesson is that totality and effect-safety, when made type-system properties, become
guarantees with *no separate analysis pass* rather than runtime checks, at a stated ergonomic cost.

## Totality as a type property: Dhall

Dhall is the reference for "termination is a type-system guarantee, not a runtime check." It types with a
restricted System Fω under which every well-typed expression provably terminates. There is no general
recursion: a self-referential `let` or function is a type error, and the only built-in recursive structure is
`List`, consumed only through structurally-terminating primitives like `List/fold`. Recursive data (trees,
JSON-like ASTs) uses Church/Boehm-Berarducci encoding, where a recursive value *is* its own eliminator (one
case-handler per constructor), finite by construction because every consumption is a fold, never a
Y-combinator fixpoint. The evaluator strong-normalizes any well-typed expression to a canonical
beta/alpha-normal term.

The cost is stated plainly and is worth transplanting as a warning: recursive-looking code must be
hand-transformed into fold form (Dhall ships a "How to translate recursive code to Dhall" howto), deep
structures cost more than a native recursive type, and nothing whose termination is not structurally evident
is expressible. In exchange, type-checking and evaluation both provably complete in finite time, which is the
property everything downstream (the semantic hash, the import cache) leans on. This is the exact shape of the
tradeoff a totality-carrying language makes: a real expressiveness tax for a language-wide guarantee with no
termination-analysis pass.

## Content-addressed determinism: Dhall's semantic hash

Dhall's semantic integrity check is the reference for reproducible, mirror-independent, content-addressed
values, and it depends on totality. The procedure: resolve imports, beta-normalize (`e₁ ⇥ e₂`), alpha-normalize
(rename bound variables to De Bruijn indices, `e₂ ↦ e₃`), CBOR-encode (every expression variant is a CBOR
array tagged by an integer label), SHA-256, base16. The hash is invariant under comments and formatting
(stripped at parse), under bound-variable naming (killed by alpha-normalization), and under which
expression/import set produced the same normal form; it is sensitive only to actual semantics. An import can be
pinned `sha256:<hash>` and the resolver refuses a mismatch. The load-bearing dependency: the hash requires
normalization to be tractable before hashing, which is *unusable under non-termination*, so content-addressing
this way is a payoff of totality, not an independent feature. CBOR and SHA-256 are themselves no_std-compatible
(depending on neither GC nor laziness), the most portable part of Dhall's design, modulo a fixed-capacity
buffer instead of a growable one. For any design wanting deterministic, cacheable, diff-stable output, this is
the mechanism, and its precondition (a normal form that always exists) is the thing to secure first.

## Laziness and thunks, and why they resist no-alloc

Jsonnet is the reference for lazy evaluation and its cost. Values are a tagged union; array elements, function
arguments, and object field values are all thunks. A `HeapThunk` holds `{ body, upValues, filled, content,
self, offset }`; forcing evaluates once and caches into `content`. The VM runs an explicit frame stack with a
forward/unwind goto state machine rather than native recursion, avoiding host stack limits on deep recursion,
a mechanism worth noting on its own (deep evaluation without blowing the host stack, at the cost of goto-based
control flow harder to read than a naive recursive `eval`). Objects are a binary tree of simple/extended nodes
with an `offset` counter tracking `super` depth, so `self`/`super` resolve per-call-frame rather than via a
fixed vtable.

The transfer verdict is unambiguous and important: **thunks are heap-shaped by necessity.** A `HeapThunk` is
aliased from multiple bindings, forced exactly once, needing a boxed, GC-tracked object with identity; no
no-alloc mapping exists without either eager evaluation (losing laziness) or an arena standing in for the heap.
The `HeapExtendedObject`/`HeapSimpleObject` tree and `offset`-based super tracking assume arbitrary cyclic,
pointer-linked graphs (`super` chains, closures over arbitrary environments) with no stack-only substitute for
runtime self-reference. The design implication: a no-alloc runtime either evaluates eagerly (giving up
laziness) or accepts an arena as its heap; there is no free lazy evaluation without a collector or an arena.

## Effect classification as read/write edges: LMS

LMS is the reference for tracking effects precisely enough to keep pure code freely movable while pinning
observable side effects. Every side-effecting `Def` is wrapped in a `Reflect` node carrying an explicit
dependency list; every reified sub-block is a `Reify`. A `Summary` classifies an operation along ten
booleans/lists (`maySimple`/`mstSimple`/`mayGlobal`/`mstGlobal`/`resAlloc`/`control` plus `mayRead`/`mstRead`/
`mayWrite`/`mstWrite` symbol lists), with named constructors (`Pure`/`Simple`/`Global`/`Alloc`/`Control`).
`reflectEffect`/`reflectWrite` build a `Reflect` whose `deps` list is computed from the ambient ordered
context filtered by read/write overlap with prior writers/readers.

The mechanism, stated for transfer: **ordering is not implicit in emission order, it is an explicit edge list
per effectful node**, computed once at staging time from the read/write sets, so a scheduler that respects
those edges reproduces the same total order an eager top-to-bottom interpreter would produce, while pure code
is free to move (sink to its uses, or be reordered/duplicated) because it carries no ordering edges.
`reifyEffectsHere` resets the ambient context so a branch's effects are captured locally into its own `Reify`,
which is how effects sink into the correct block, one `Block` per conditional/loop body. The cost is named
directly: a nontrivial `Summary` lattice every new effectful primitive must classify correctly, and a
miscategorization is a silent correctness bug (a write miscategorized as pure can be reordered past a dependent
read). This is the precise reference for an effect axis that wants to be both a correctness guarantee and a
scheduling input, and for the discipline it demands (every primitive's effect classification must be right, and
getting it wrong fails silently).

## Binding-time and the compile-versus-runtime split

Three sources triangulate what evaluates when.

Terra (`prior_art/08`) draws the sharpest line: specialization is *eager* ("we perform specialization
eagerly... while we perform typechecking and linking lazily, only when a function is called"). A worked
example shows Lua-side mutation between definition and use is captured *by value at specialization time*, not
by live reference. So an escape's value is fixed when the term is specialized, and an unrepresentable escape
fails then, as a compile-time error, never at runtime. Zig comptime is the same binding-time discipline
without a separate stage: a value is comptime-known or it is runtime data, and a comptime-known value "simply
becomes runtime data at any point downstream where nothing forces it to stay compile-time-known" (there is no
splice operator; the transition is where a value flows into a runtime position). The branch quota (default
1000, ratcheting upward) is the guard that comptime evaluation terminates.

Racket (`prior_art/04`) contributes phase separation: `begin-for-syntax` and `define-syntaxes` are
compile-time (transformer) phases distinct from run-time, and hygiene is a per-expansion scope-set operation
baked into the expander. The `#lang` mechanism itself is a binding-time statement: a reader phase (bytes to
syntax objects) and an expander phase, and a language customizes semantics by exporting a `#%module-begin`
that runs at expansion time to wrap the body.

The .NET boundary (`runtime-value-transfer-study/dotnet_clr.md`) adds the pragmatic split lesson from the other
direction: the shift from `[DllImport]` (a marshalling IL stub synthesized *at runtime*) to `[LibraryImport]`
(a Roslyn source generator emitting the marshalling code *at compile time*) was made because runtime codegen
of the boundary is AOT-hostile, opaque, and un-inlinable. The lesson, stated in the source: "resolve the
boundary shape ahead of time into inspectable, static code; runtime codegen of the boundary was a mistake paid
for in AOT-hostility, opacity, and cost." This is direct support for discharging boundary work at generation
time rather than at run time.

## Lifetime and lease as a contract across the split

The .NET `Span<T>`/`Memory<T>` design is the reference for lifetime discipline, and it belongs in this domain
because it is a compile-versus-runtime lifetime story. `Span<T>` is a `ref struct` (stack-only, cannot be
boxed, cannot cross an `await`), which makes the compiler's lifetime rules *statically* guarantee the view
never outlives its buffer, the zero-copy safety contract enforced at compile time. `Memory<T>` exists because
`Span<T>` is too restrictive to store on the heap or cross an async call, so .NET added a heap-storable type
and projects a `Span<T>` out of it only at the synchronous moment of access. The two-type split (storable
capability versus access capability) exists because only one is expressible as a compiler-checked stack
lifetime, and the `Memory<T>` lease rules (a `void`-returning callee must not touch the buffer after
returning; the async analogue ends at the returned `Task`) codify the rest as a stated contract because the
compiler catches only the `Span<T>` subset.

The general shape this points at: a lifetime that *can* be proven statically (the stack-scoped borrow) should
be, and the residue that cannot (a value outliving the call, a cross-process handle) is carried by an explicit
named lease contract. This is the same split the value-transfer domain reached from the runtime side, and it
composes with the effect axis: build-environment work reduces away at compile time (the effect/binding-time
axis), run-time effects and their lifetimes are contracted at the boundary.

## Manifestation-in-the-guest versus renderers-in-the-host

A smaller but recurring purity-adjacent contrast worth carrying: Jsonnet writes its output formatters
(`std.manifestYamlDoc`, `std.manifestIni`, `std.manifestPython`) *in Jsonnet itself*, as recursive functions
doing runtime type-tag dispatch and string concatenation over the already-forced value tree; adding a format
is a library function, not an interpreter change, but every function re-derives type introspection from
scratch with no typed backend contract and only text output. Dhall does the structural opposite: its renderers
(`dhall-to-json`, `dhall-to-yaml`) are host-language (Haskell) code over a decoded native value. The axis is
whether output generation is guest-language code over the guest value tree (extensible without touching the
core, but untyped and text-only) or host code over a decoded value (typed, but a core change per format). This
is the purity-side view of the output-spectrum domain: manifestation runs *after* laziness is fully discharged
and totality is spent, over a settled normal form.

## Consolidated

The domain's guarantees are cheapest when they are type-system properties: totality removes the
termination-analysis pass (Dhall), and effect classification as read/write edges makes pure code free and
pins effects with no separate scheduling analysis (LMS). Both charge a real ergonomic tax (fold-encoded
recursion; a correct-or-silently-wrong effect lattice). Determinism and content-addressing are payoffs of
totality (a normal form that always exists), portable to no_std except for the growable buffer. Laziness is
the one feature that resists no-alloc outright (thunks are heap-shaped), forcing eager evaluation or an arena.
Binding-time discipline (eager specialization, comptime, phase separation, source-generated boundaries) says
discharge boundary and build-environment work at generation time and let only run-time-environment work reach
the runtime, and lifetime discipline says prove the stack-scoped borrow statically and contract the rest as a
named lease.

## Sources

Primary synthesised docs:
`202607190650_mockspace-procedural-docs-arc/prior_art/02_jsonnet_dhall.md` (totality, semantic hash, thunks,
manifestation), `.../01_lms_delite.md` (the `Summary` effect system and effect-edge scheduling),
`.../08_terra_procmacro_comptime.md` (eager specialization, comptime binding-time), `.../04_racket_scribble.md`
(phase separation, hygiene); `202607201227_runtime-value-transfer-study/dotnet_clr.md`
(`LibraryImport`-source-generation, `Span`/`Memory` lifetime discipline).

Key underlying citations: the Dhall standard (beta-/alpha-normalization, binary, imports, type-inference); the
Jsonnet spec and `core/vm.cpp`; LMS `src/internal/Effects.scala`; the Terra PLDI 2013 paper and Zig comptime
reference; the Racket syntax-model reference; the .NET P/Invoke source-generation and `Memory<T>` usage
guidelines.
