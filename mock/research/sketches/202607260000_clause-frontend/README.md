# The Clause front end (sketch)

A sketch, in the sense the workspace uses the word: exploratory implementation that lives as an audit trail and
extracts to its own repo when that repo exists. The framework's own rule is that a consumer's surface syntax
never lands in framework code, so this is deliberately outside `mock/crates/` and is structured as a library so
extraction is a move rather than a rewrite.

## What it is

The front end for the first-party language: the surface the pre-redesign grammar specifies
(`mock/research/original-docs/CLAUSE_EBNF.md`, which is normative), lowered to the vehje Core.

The bar is the COMPLETE grammar, not a runnable subset. Generics, traits, associated types, and macros are the
point rather than the stretch goal, and the completeness census
(`mock/research/202607251530_language-completeness-census.md`) is the map of how each production reaches the
Core.

## Discipline

`no_std`, no allocator, caller-lent arenas, matching the framework rather than diverging from it. A parser into
a lent AST arena is the same shape as the IR arena it lowers into, so the two halves read alike.

Most of the type system erases before emission: generics, traits, and associated types monomorphise away, so
that half proceeds independently of anything the runtime is still growing.
