# The first-party language: what it is, and what it needs from the framework

**Date:** 2026-07-25
**Status:** research. A consumer-shape study, not a framework design round.
**Why it lives here:** the framework's own design says front-end scaffolding is "offered opt-in, not shipped now, extracted once a live consumer proves the shared shape". This is that study: what the first real consumer looks like, and which of the framework's extension points it actually exercises. The language itself is a separate repo, and naming it is the maintainer's call.

The framework can now run a program. It evaluates literals, variables, bindings including recursive ones, conditionals, lambdas with lexical closures, and application; a family operation reaches a host handler, so real computation happens; and a produced value crosses back to the host. What nobody can do is write a program, because there is no surface syntax and no front end. That is the gap this study is about.

The language is not a new idea. It is the one this workspace is named after. Clause exists as roughly 41k lines of Python with 1511 tests: a Rust-syntax authoring language whose grammar mimics Rust almost one to one, extended with items that language needed for its domain. Its stated design intent has always been Rust-feeling generics, traits, and associated types, statically analysable, with the strictness dialled to what a scripting language wants rather than to what a systems language must have. What it never had is an interpreter. It compiles and it emits, and `clause test` is compile-only. So the goal now is the language that heritage was reaching for, on a framework that can actually execute it.

Three things follow from that, and they shape everything below. The surface should feel like Rust to anyone who writes Rust, because that is the whole ergonomic bet and the heritage already validated it against real users. The type system should be the parts of Rust that catch mistakes (generics, traits, associated types, exhaustive matching) without the parts that exist to manage memory (ownership, borrowing, lifetimes), because values here are immutable and the runtime has no aliasing problem to solve. And every static guarantee must survive to the residual, because the framework's whole thesis is that the type system is the verification layer and the runtime is a dumb evaluator of already-proven programs.

## What the language is

A general-purpose scripting language with a Rust-shaped surface. Items are `fn`, `struct`, `enum`, `trait`, `impl`, `type`, `const`, and `use`. Expressions are Rust's: literals, paths, calls, method calls, field access, indexing, blocks, `let`, `if`/`else`, `match`, `loop`/`while`/`for`, closures, operators with Rust's precedence. Patterns are Rust's: literals, bindings, wildcards, tuples, structs, enum variants, with `|` alternatives and `if` guards. Attributes exist and drive conditional compilation.

It is not Rust in the places where Rust's cost is memory management. There is no ownership, no borrowing, no lifetimes, and no `mut` on bindings. Values are immutable, which is what makes the lease and region proof discharge statically and lets the runtime inherit proven-valid references with no collector. A binding rebinds by shadowing, the way `let` already does in Rust. This is the single largest simplification and it is the one that makes the language a scripting language rather than a systems one.

## The type system, and what "statically analysable" buys

Generics are parametric with trait bounds, written as Rust writes them. Traits declare methods and associated types; `impl Trait for Type` provides them; bounds constrain type parameters. Associated types project through a bound, so `T::Item` means what it means in Rust. Inference is Hindley-Milner over the expression language with the bounds as constraints, which the heritage already planned and which is the standard answer for a language without subtyping.

Three deliberate relaxations from Rust, each with its reason. Coherence is checked but not orphan-restricted across the whole world, because a script's universe is its own program plus its declared dependencies, not an open ecosystem. Trait objects are absent for now; polymorphism is monomorphised, which suits a language whose programs are small and whose framework already prefers compile-time dispatch over any form of `dyn`. Higher-kinded types are out; associated types cover the cases that matter without the inference cost.

What the analysis buys is not merely catching type errors. Every construct resolves to a Core form whose family and effect membership is known, so the framework's inclusion check can refuse a program that uses a construct its target does not support, naming the construct. That is the property the whole design exists for, and the language surface has to preserve it: nothing in the surface may be dynamic enough to hide which family an operation belongs to.

## How the surface reaches the Core

The Core has twelve forms and the surface has far more syntax, so most of the front end's work is desugaring rather than translation. The mapping is direct enough to state completely.

A `fn` becomes nested `Lambda`, one per parameter, since the Core's application is one argument at a time. A call becomes `Apply`. A block with statements becomes nested `Let`, one per statement, with the tail expression as the innermost body; a statement whose value is discarded binds a fresh minted binder, which is exactly what the ANF pass already mints. `if`/`else` becomes `If`. `match` becomes `Match` over the scrutinee with its arms, and exhaustiveness is checked in the front end rather than trusted at runtime. Field access and indexing become `Project`. A `for` loop becomes `Iter`. String interpolation becomes `Interp`. A `struct` literal becomes a record value, and an `enum` variant becomes a tagged one.

Everything else is a family operation. Arithmetic, comparison, logical operators, string operations, and every host service reach the runtime as `Raw` carrying a family id and operands, dispatched to the handler the host provides. That is not a workaround; it is the design. The Core has no `+` because addition is not an evaluation form, and the framework owns no family because a family is a consumer's vocabulary. The language declares its own arithmetic family and ships the handler, which is precisely the extension point the framework just grew.

Recursion needs the recursive binding form, which the runtime has. Mutual recursion between top-level functions needs a group of them in scope for each other, which is the same mechanism applied to a set rather than to one binder.

## What the framework still owes this consumer

Working through the mapping surfaces exactly what is missing, which is the useful output of a study like this.

The evaluated set stops short of what the surface needs. `Match` has no evaluation, so no pattern matching runs; `Project` has none, so no field access runs; `Iter` has none, so no loop runs; `Interp` has none, so no interpolation runs. Each is a Core form the language leans on directly, and each is named in the runtime's intended list. They are the next runtime increments, and the order above is roughly their order of importance to a language.

Values stop at unit, boolean, and integer. A language needs strings, sequences, and records as first-class values, both to evaluate and to hand back. The value image already has tags for all of them, so the contract is written and the runtime side is not.

Operands to a family handler are scalars only. A string operation cannot receive its string, which blocks the entire string half of any standard library. This is the extension point the family-dispatch round explicitly named and deferred, and it is now the binding constraint on usefulness rather than a theoretical gap.

The graded spine computes effect and lease but hard-codes binding time and assurance. A language with compile-time evaluation and macros needs binding time to be real, because the whole compile-versus-runtime split is that coordinate. This is the frontier item most directly blocking the language's more interesting half.

## The shape of the work

The front end is a lexer, a parser producing an arena AST, a resolver, a type checker, and a lowering pass into the Core builder. The heritage's lessons are explicit about what to inherit and what to refuse: inherit the error-recovering parser and the structured diagnostic registry; refuse the two parallel pipelines that were its largest architectural debt, refuse language-special-cased `Option`, `Result`, and `Vec`, and refuse runtime pass discovery. The framework's own passes already have a single pipeline, so the debt has no way in this time.

A staged first light is the honest path. The subset that runs today is literals, bindings, conditionals, functions, calls, and arithmetic through a family handler, which is enough for a real program with recursion and enough to prove every seam end to end from source text to a returned value. Pattern matching, records, and strings follow as the runtime grows the forms they need. Traits and generics are a front-end concern and can land before the runtime grows at all, since they erase by monomorphisation before anything is emitted.

## Open questions for the maintainer

The name. The framework's own documentation calls the language "named later", and the heritage's name is Clause, which is also this workspace's name. Whether the first-party language keeps that name, and whether it is one repo or a family of them, is the maintainer's call and blocks nothing until the repo is created.

Whether the language's arithmetic is a first-party family the language repo ships, or a small standard family the framework ships as a reference. The framework owning no family argues for the former; every consumer redeclaring integer addition argues that a reference family shipped as a separate crate, not as part of the framework, is worth considering.

How much of the heritage's domain-specific surface survives into the general-purpose language. `event`, `expect`/`actual`, `sealed`, and the patch attributes were built for a specific target, and the census now routes that target through its own consumer. The general-purpose language may want a subset of them or none.
