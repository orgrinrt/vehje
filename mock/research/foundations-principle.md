# The foundations principle. read before anything else

**This is the single most important doc in `mock/research/`.
Every other doc in this directory is subordinate to this one.
Every design round touching clause is subordinate to this one.**

## The rule

Clause is not a standalone compiler. Clause is an application of
the `clause-dev` foundations.

The foundations are:

- **notko**. fallibility primitives (`Maybe<T>`, `Outcome<T, E>`,
  `Just<T>`, `Boundable`, `NonZeroable`). Plus `notko-macros`
  (`#[optimize_for]`).
- **arvo**. numeric primitives, bit contracts, hashing, graph /
  sparse / bitmask / spectral / combinatorics primitives.
- **hilavitkutin**. the pipeline execution engine. WorkUnit
  declarations, AccessSets, Column / Resource / Virtual / Field
  storage descriptors, the scheduler, the thread pool, the
  persistence bridge, the context framework, the interning
  system, the build-time pass plugin.

Clause uses these for everything. Clause does not invent its own
infrastructure. Clause does not reach for `crates.io` for
infrastructure that the foundations cover, or should cover.

If clause needs infrastructure the foundations do not ship, the
correct response is:

1. Name the need.
2. Decide whether it is a clause concern, a hilavitkutin concern,
   or an arvo concern.
3. If hilavitkutin / arvo, extend the foundations. Clause then
   applies the extension.
4. Only if the need is demonstrably clause-specific (not
   generalisable) does it land inside a clause crate.

The burden of proof sits on "demonstrably clause-specific".
Default to the foundations.

## Why this rule exists

The clause-dev workspace is a coherent stack. Three properties
fall out of that coherence:

1. **One place to fix anything.** A caching bug, a scheduling
   bug, a persistence bug is fixed once in hilavitkutin. Every
   consumer benefits. If clause invents its own cache, a bug
   needs fixing in both places and drift begins.
2. **One surface to learn.** A contributor reading hilavitkutin
   can navigate the compile pipeline. A contributor who has
   written one WorkUnit can write a compile pass. The
   vocabulary is shared. Code review loads the same context.
3. **One optimiser.** hilavitkutin-build's LLVM pass plugin +
   PGO / BOLT pipeline optimises every WorkUnit in the stack.
   If clause runs passes outside the engine, those passes miss
   the whole-program optimiser.

Python Clause violated this principle. it bolted a pass
scheduler on top of `graphlib`, a cache on top of SQLite, a
serialiser on top of `json` with custom tags, a task registry on
top of `inspect` + `pkgutil`. Every one of those is the wrong
answer for the Rust rewrite. The foundations already solve each
problem better.

## The mapping

This table is exhaustive. If a need is on it, the mapping is the
answer.

| Need                     | Wrong answer           | Canonical answer                                      |
|--------------------------|------------------------|-------------------------------------------------------|
| Fallibility              | `Option` / `Result`    | `notko::{Maybe, Outcome, Just}`                       |
| Integer widths           | `usize` / `u*` / `i*`  | `arvo::{USize, UFixed<N, 0, S>, IFixed<I, F, S>}`     |
| Float                    | `f32` / `f64`          | `arvo::{FastFloat, StrictFloat}`                      |
| Bool                     | `bool`                 | `arvo::Bool`                                          |
| Bounded ints             | `NonZeroU32` etc.      | `arvo::Cap`, `notko::Boundable`, `notko::NonZeroable` |
| Strings                  | `String` / `&str`      | `hilavitkutin_str::Str` (interned, 32-bit handle)     |
| Collections, return     | `Vec<T>`               | `&mut impl Collector<T>` / sink from hilavitkutin-api |
| Collections, parameter  | `&[T]` is fine         | `&[T]` or `impl IntoIterator`                         |
| Collections, field      | `Vec<T>`               | `hilavitkutin_api::Seq<T, N: Cap>`                    |
| Map field                | `HashMap<K, V>`        | `hilavitkutin_api::Map<K, V, N: Cap>`                 |
| Diagnostic sink          | `Vec<Diagnostic>`      | `&mut impl DiagnosticSink<D>` (hilavitkutin-api)      |
| Byte emitter             | `Vec<u8>`              | `&mut impl ByteEmitter` (hilavitkutin-api)            |
| Bit contracts            | `bitflags`             | `arvo_bits::{Bits<N>, Bitfield}`                      |
| Bitmask                  | `bitvec`               | `arvo_bitmask::{Mask64, Sieve}`                       |
| Sparse storage           | ad-hoc                 | `arvo_sparse`                                         |
| Graph / DAG              | `petgraph`             | `arvo_graph`                                          |
| Spectral / linear alg.   | `nalgebra`             | `arvo_spectral`                                       |
| Combinatorics            | ad-hoc                 | `arvo_comb`                                           |
| Content hash             | `blake3` / `sha2`      | `arvo_hash`                                           |
| Serialization, on disk  | `bincode` / `rkyv`     | `hilavitkutin_persistence` cold store                 |
| Serialization, in mem   | `serde` roundtrip      | direct `hilavitkutin_api::Encoder` / `Decoder`        |
| Persistence / cache      | `sled` / `redb`        | `hilavitkutin_persistence` hot+cold bridge            |
| Pass / task scheduler    | `graphlib` / handroll  | `hilavitkutin` engine, each pass is a WorkUnit       |
| Runtime registration     | `inventory` / `linkme` | Forbidden (lint #108). Static composition only        |
| Task parallelism         | `rayon` / `tokio`      | `hilavitkutin` pre-allocated thread pool              |
| Context / env            | ad-hoc                 | `hilavitkutin_ctx`                                    |
| Build-time optimisation  | `build.rs` handwritten | `hilavitkutin_build` (LLVM passes, PGO, BOLT)         |
| Fn-callback bound        | `Fn(&A) -> bool`       | `arvo::{Pred, Pred2, Pred3}`                          |
| Proc-macro optimisation  | handwritten            | `notko_macros::#[optimize_for]`                       |
| Parser library           | `logos` / `chumsky`    | Hand-rolled in clause-lex / clause-syntax             |
| Diagnostic renderer      | `ariadne`              | clause-owned, built on hilavitkutin-api sinks         |

## The runtime is hilavitkutin. Full stop.

When clause runs, whether to compile source or to interpret
Clause bytecode, it runs on hilavitkutin. There is no "clause
runtime". There is no separate "interpreter VM". There is
hilavitkutin executing WorkUnits.

Compile mode:

- Each compile phase (lex, parse, resolve, typecheck, transpile,
  codegen, validators, analyzers, optimisers) is a WorkUnit.
- Source bytes live in a Column. Tokens live in a Column.
  AST nodes live in a Column. Resolved symbols live in a
  Column. Each phase is a WorkUnit that reads its input
  Columns and writes its output Columns.
- The hilavitkutin scheduler topologically orders the WorkUnits
  by AccessSet. Parallelism is automatic where independent.
  Caching is automatic via hilavitkutin-persistence.
- Incremental rebuild is automatic, the engine's own
  change-detection machinery decides which WorkUnits rerun.

Interpret mode (future):

- Clause bytecode is a Column of instruction records.
- The interpreter is a WorkUnit that reads instruction Columns
  + bound scope Columns, writes result Columns.
- The engine's thread pool runs interpretation. The engine's
  persistence stores interpretation state if needed.
- Clause `#[test]` blocks become WorkUnit invocations with
  assertion WorkUnits downstream.

Corollary: `clause-schedule` is not a pass framework. It is the
crate where clause-specific WorkUnits live. The pass framework
IS hilavitkutin. `clause-schedule` exposes the WorkUnit types,
AccessSet declarations, and Column descriptors clause needs.
Other clause crates register their WorkUnits through the
standard hilavitkutin registration (static composition, no
runtime registry).

## What "expand the foundations" means in practice

When clause needs something hilavitkutin does not ship:

1. **Write the design down.** What is the need? What would the
   hilavitkutin API look like? Which existing hilavitkutin
   crates does it fit into?
2. **Open a hilavitkutin design round.** Topic → doc CL → src
   CL → lock → close. The standard mockspace flow applied to
   the hilavitkutin repo, not the clause repo.
3. **Land the hilavitkutin round first.** Before the clause
   round that consumes the new capability opens.
4. **Clause consumes via the standard `.workspace = true`
   dep.** No local shims, no re-exports from clause crates
   that duplicate hilavitkutin surface.

Same rule for arvo. A clause need for a new numeric width, a
new bit contract, a new graph algorithm, goes to arvo first,
then clause consumes.

### Examples the port will hit

**Need: cross-crate artifact storage with content-addressed
naming.**

- Wrong: `rkyv` + a local file-layout.
- Right: hilavitkutin-persistence cold store, keyed by
  `arvo_hash::ContentHash`. If the cold store does not yet
  expose content-addressed keys, extend hilavitkutin-persistence
  (small round) to provide `ColdStore::put_by_content_hash(...)
  -> Str` and `get_by_content_hash(Str) -> Maybe<&[u8]>`, then
  consume.

**Need: pass dependency DAG with topological sort.**

- Wrong: `graphlib`-equivalent or `petgraph` in clause.
- Right: hilavitkutin scheduler already topologically orders
  WorkUnits by AccessSet. If clause needs an introspection API
  over the resolved schedule (for `clause explain --schedule`),
  extend hilavitkutin to expose a read-only query over the
  resolved plan.

**Need: source-map sidecar binary format.**

- Wrong: a new bincode format in clause-codegen.
- Right: hilavitkutin-persistence cold store entry, keyed by
  output file's content hash. The source-map rows are a Column
  declared by clause-codegen; the persistence format is
  hilavitkutin's.

**Need: diagnostic rendering to ANSI / plain / JSON.**

- Wrong: `ariadne` dependency.
- Right: clause-owned renderer that takes a `DiagnosticSink`
  and walks its entries. Output is produced through a
  `ByteEmitter` (also hilavitkutin-api). If the renderer has
  reusable shape, span tagging, line-wrapping, color
  selection, it can migrate upward to hilavitkutin-api or a
  new `hilavitkutin-render` crate, but the *dependency* never
  leaves the stack.

**Need: fast content hash for pass fingerprint.**

- Wrong: `blake3` dep.
- Right: `arvo_hash::ContentHash`. If arvo-hash does not yet
  ship a specific hash variant, extend arvo-hash (the crate
  already exists, #116 landed). Clause consumes.

**Need: parser tokenizer (DFA, codegen'd).**

- Not a foundations concern. Tokenization is clause-lex's
  domain, hand-rolled. No `logos` dependency. The *kind* of
  code that writes tokenizers does not belong in the
  foundations. it is application logic. However, any bit
  contracts the tokenizer uses (e.g. a class-mask of byte
  categories) must use `arvo_bits` / `arvo_bitmask`.

**Need: multi-span diagnostic label chains (Python Clause had
`related: &'static [Span]`).**

- Extend hilavitkutin-api's `DiagnosticSink` contract to
  carry related spans. Clause consumes.

## Red flags that mean you are about to violate this rule

- You are about to add a crate to `Cargo.toml` that is not in
  the foundations or the clause workspace.
- You are about to write a `mod cache` or `mod scheduler` or
  `mod pool` or `mod arena` inside a clause crate.
- You are about to define a struct whose field is `Vec<T>` in
  a shipping interface.
- You are about to write `std::sync::Arc` or `std::sync::
  Mutex` in clause.
- You are about to implement a graph traversal in clause
  instead of calling arvo-graph.
- You are about to spell a binary format, SHA256, bincode,
  msgpack, in clause code.
- You are sketching a "clause-runtime" crate that has its own
  thread pool.
- You catch yourself writing `fn spawn(` or `fn run_on_thread(`.

When you spot one of these, stop. Check the mapping table
above. Open a foundations round if the table does not answer.

## Enforcement

This principle enforces itself through:

- **Workspace lints**: `no-bare-vec`, `no-bare-string`,
  `no-bare-numeric`, `no-bare-option`, `no-bare-result`,
  `no-dyn-dispatch`, `no-runtime-registration`,
  `no-runtime-spawn`, `no-alloc` (no-heap first principle),
  `arvo-types-only`, `trait-first-signatures`,
  `forbidden-imports` (std::* / alloc::* / runtime crates).
  All shipped, all strict, all apply to clause.
- **CLAUDE.md** in the workspace and each repo. Pulls in the
  type-surface + cookbook + no-heap + exact-widths + primitive-
  vocabulary rules.
- **Pre-commit hooks** that run the above lints.

Lints are necessary but insufficient. They catch the spellings.
They do not catch "you wrote a cache layer". The architectural
rule lives here, in this doc, and must be in the head of
everyone writing clause design rounds.

## Summary

- Substrate owns: scheduling, parallelism, persistence,
  caching, numeric types, bit contracts, string interning,
  hashing, graph traversal, sparse storage, spectral algebra,
  combinatorics, bool, fallibility, predicates, build-time
  optimisation.
- Clause owns: language surface, parser, type system,
  semantic analysis, language-specific lints, target
  lowering, CLI surface.
- When in doubt, default to the foundations.
- The engine that runs clause IS hilavitkutin. Compile
  passes are WorkUnits. The interpreter is WorkUnits. There
  is no second runtime.
- If clause needs something the foundations do not ship,
  extend the foundations first, then consume from clause.
