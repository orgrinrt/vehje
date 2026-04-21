# Rust Clause — parity + extension plan

**Date:** 2026-04-21
**Reads:** `python-clause-survey.md`, `lessons-learned.md`.
**Purpose:** the ordered task / design-round set that takes
the Rust Clause skeleton to feature parity with Python Clause
and beyond.

This plan is the set of **design-round topics** (not code
tasks). Each topic maps to one design round which produces
its own doc CL + src CL. Rough size estimates per round are
given for scheduling; they are not contracts.

## Current state (2026-04-21)

- `clause-lex`: real body (1032 LOC). Identifiers, keywords,
  decimal integer literals, operators, punctuation, line /
  block comments, trivia, diagnostic sink. **String / char /
  raw / byte / float literals deferred.**
- `clause-ir`: type definitions for `Diagnostic`, `Span`,
  `FileId`, `TokenKind`, `NodeId`, `ScopeId`, `AstNodeKind`.
- `clause-syntax`: parser skeleton. Handles empty slice and
  single-literal inputs; rejects everything else.
- `clause-resolve`: skeleton walk; returns empty `Resolved`.
- `clause-typecheck`: skeleton with 10 validator stubs that
  push zero diagnostics.
- `clause-codegen`: skeleton targets that return empty
  artifacts.
- `clause-schedule`: empty crate.
- `clause` binary: CLI with real `lex` subcommand + frozen
  stubs for `parse` / `check` / `build` / `run`.
- `clause-runtime-abi` / `clause-runtime-driver` /
  `clause-runtime-tests`: skeleton runtime boundary.

Type-surface discipline is in place: fallibility sweep
landed (`Option`/`Result` → `Maybe`/`Outcome`), collections
sweep landed (`Vec` returns → `DiagnosticSink` / `ByteEmitter`
bounds). Primitives sweep (#73.3) remains.

## Milestone structure

Six milestones. Each milestone is several design rounds.
The order is dependency-driven; reordering within a
milestone is OK where cross-cuts allow.

```
M0: type-surface + schedule spine   [polish, foundation]
M1: parse the language               [real front-end]
M2: resolve + typecheck              [real mid-end]
M3: storage + transpile + codegen    [real back-end]
M4: macros                           [first-class feature]
M5: passes + validators + spec       [ecosystem frame]
M6: cli + manifest + tooling         [productisation]
```

At end of M3 the compiler can emit a working Clausewitz
artifact from a Clause source file. That is the point where
"clause actually works" in the user's sense. M4-M6 turn the
compiler into the authoring tool the Python version aspired
to be.

## M0 — type-surface + schedule spine

Prerequisites for everything else. Also finishes the #73
sweep.

### M0.1 — primitives sweep (#73.3)

Replace bare `usize` / `u*` / `i*` / `f*` / `bool` / `String`
with `USize` / `UFixed` / `IFixed` / `FastFloat` /
`StrictFloat` / `Bool` / `hilavitkutin_str::Str`. Scope:
every clause crate except `clause-runtime-abi` (FFI carve-out
via SHAME.md). Resolves the deferred struct fields from #73.2
(`Scope::symbols` HashMap, `ScopeTree::scopes` Vec,
`Resolved::resolution` Vec, `Manifest::dependencies` Vec).

Size: medium. One PR. Dependency: none. Outcome: zero bare
primitives anywhere; type surface matches arvo / hilavitkutin
sibling repos.

### M0.2 — clause-schedule pass DAG skeleton

Populate the empty `clause-schedule` crate. Defines the
`Pass` trait, `Artifact` typed-enum, pass registry via
`linkme` / `inventory`-style compile-time inventory, topo
sort over declared reads / writes. No real passes yet — that
is M5. This round is the spine.

Size: small-medium. One PR. Dependency: M0.1 (clean type
surface first). Outcome: `clause-schedule` non-empty; the
binary's hard-coded phase chain is ready to be replaced by
graph walks in later rounds.

### M0.3 — unify the pipeline from day one

Rewire the `clause` binary to run through `clause-schedule`
for every subcommand, even with the current stubs. Passes are
named (`LexPass`, `ParsePass`, `ResolvePass`,
`TypecheckPass`, `CodegenPass`). Subcommands stop at
different points in the DAG.

Size: small. One PR. Dependency: M0.2. Outcome: no more
hard-coded phase chains in `clause/src/*.rs`. The "two
parallel pipelines" architectural mistake from Python is
avoided from the start.

## M1 — parse the language

The front-end. By end of M1, `clause parse` produces a real
AST for real programs.

### M1.1 — lexer feature-complete

Extend `clause-lex` to cover the remaining literal forms:
string literals (`"..."` with escapes), char literals
(`'x'`), float literals, byte literals (`b'x'`), byte-string
literals (`b"..."`), raw string literals (`r#"..."#`), shebang
line, BOM, unicode escapes, nested block comments. Diagnostics
for each failure mode.

Size: medium. One PR. Dependency: M0.3. Outcome: lexer covers
every token the `original-docs/GRAMMAR.md` grammar names.

### M1.2 — parser: items, types, visibility

Parser productions for `mod` / `use` / `pub` variants /
`struct` / `enum` / `trait` / `impl` / `fn` / `const` /
`static` / `type` / `extern` / `sealed`. Generics with
where-clauses and supertraits. Visibility modifiers. Associated
types / consts.

Size: large. One PR. Dependency: M1.1. Outcome: every item
form in `original-docs/CLAUSE_EBNF.md` parses into an AST
node.

### M1.3 — parser: patterns + expressions

All pattern forms (wild / literal / ident / path /
tuplestruct / tuple / range / or / rest / ref / bind). All
expression forms (literal / path / macro-invocation / unary /
ref / tuple / struct-literal / binary / assign / compound
assign / range / call / method call / field access / index /
block / if / match / for / while / loop / closure / return /
break / continue / paren / question). Error recovery nodes
(`ErrorExpr`, `ErrorPat`).

Size: large. One PR. Dependency: M1.2. Outcome:
`clause parse` produces a complete AST for the Python test
corpus's parseable inputs.

### M1.4 — parser: attributes + `event` / `expect` / `actual`

Outer + inner attributes. `#[name]` / `#[as_*]` / `#[file]` /
`#[cfg]` / `#[supersedes]` / `#[deprecated]` /
`#[patch]` / `#[test]` / `#[repr]` / `#[prefer]`. Clause-
unique items: `event Name for Target { body }`,
`expect fn X();`, `actual fn X() { body }`. `sealed` modifier
on traits / structs. Struct field modifiers `mut` / `const`.

Size: medium. One PR. Dependency: M1.3. Outcome: the
Clause-unique language surface is fully parsed.

### M1.5 — module resolver

Populate `clause-resolve` with the filesystem walker. From
`src/lib.cse`, resolve `mod foo;` declarations to sibling
files or `foo/mod.cse`. Inline `mod foo { ... }` recognised.
Multi-segment self-identification (`mod a::b;`). Dead-file
detection. `use` tree resolution with `{,}` groups, glob,
`as` rename, `pub use` re-exports.

Size: medium. One PR. Dependency: M1.4. Outcome: multi-file
Clause programs resolve into a `ResolvedCrate` with
`ResolvedModule`s and binding tables.

## M2 — resolve + typecheck

Mid-end. By end of M2, `clause check` catches real semantic
errors.

### M2.1 — cfg evaluator

Implement `#[cfg(dlc=X, mod=Y, feature=Z, all=..., any=...,
not=...)]` evaluator against a `CompileEnv { dlcs, mods,
features }`. Drop items whose cfg fails before typecheck.

Size: small. One PR. Dependency: M1.4. Outcome:
conditional-compilation works; DLC / mod / feature gates
remove items deterministically.

### M2.2 — typecheck: symbol + method tables, orphan + sealed

Build fq-path-keyed symbol table. Method table
(self-type → methods). Enforce orphan rule + sealed coherence.
Pattern exhaustiveness on enum matches (including
`Maybe` / `Outcome` built-ins).

Size: large. One PR. Dependency: M1.5 + M2.1. Outcome: type
errors, missing-arm errors, orphan / sealed violations
surface correctly.

### M2.3 — typecheck: generics, bounds, associated types

Generic parameter binding, where-clause satisfaction,
associated-type resolution, associated-const resolution,
supertrait traversal. Const-generic parameters typed.

Size: large. One PR. Dependency: M2.2. Outcome: generic code
type-checks — where Python Clause deliberately punts, Rust
Clause delivers.

### M2.4 — typecheck: `expect` / `actual` pairing

Match `expect` declarations in one crate against `actual`
implementations in another. Shape check: same signature, same
generics, compatible visibility.

Size: small-medium. One PR. Dependency: M2.3 + M1.5.
Outcome: cross-crate stub pairing works.

### M2.5 — typecheck: inference

Hindley-Milner-flavoured local inference. Python Clause
deliberately omits this; the Rust port delivers it. Scope:
let-bindings, method receivers, closure parameters, generic
specialisation at call sites. Full subtyping / trait-bound
propagation deferred to a later round if the first pass is
too big.

Size: large. One or two PRs. Dependency: M2.3. Outcome: the
author can omit most local type annotations.

### M2.6 — shadow / unreachable / unused checkers

Ports of `shadow_check.py` (372L), `unreachable_check.py`
(274L), `unused_check.py` (506L). Emit STRICT1 / STRICT2 /
STRICT3-equivalent diagnostics.

Size: medium. One PR. Dependency: M2.2. Outcome: the three
strict lints ship.

### M2.7 — validator framework

Single-walk visitor with per-node enter / leave +
finalize_scope. Dedup sink keyed on (span, code). The ten
validator stubs (`Strict1..7`, `Coherence`, `OrphanRule`,
`BindTargetShape`) each get their real body. Some will land
separately; this round ships the framework that lets them
coexist.

Size: medium. One PR. Dependency: M2.6 + the existing
validator skeleton from #30. Outcome: validators run in one
walk; diagnostics are deduped.

## M3 — storage + transpile + codegen

Back-end. By end of M3, `clause build` emits real Clausewitz
artifacts. **This is the "clause actually works" milestone.**

### M3.1 — storage catalog

New crate `clause-storage`. Seven-backend catalog
(`flag` / `variable` / `scripted_variable` / `event_target` /
`scripted_list` / `string_storage` / `paired` — initial
Paradox-target list; the architecture is target-agnostic).
`BindTarget` trait with associated backend registry. Each
backend is an associated type driven by const-generic
`#[repr]` / `#[prefer]` attributes.

Size: medium-large. One PR. Dependency: M2.2. Outcome:
`struct X : Country { field: T }` gets a typed backend
assignment.

### M3.2 — storage routing + manifest lock drift

`StorageRouter` walks every `struct X : BindTarget`, picks
backends per field, reconciles with manifest `[storage]`
lock, emits `CL_STORAGE_DRIFT` on divergence. `#[repr]` hard
override, `#[prefer]` soft hint.

Size: medium. One PR. Dependency: M3.1 + M6.1 (manifest
parser). If M6.1 is not yet ready, ship M3.2 without lock
reconciliation and add reconciliation when manifest lands.
Outcome: storage routing is stable and auditable.

### M3.3 — transpile — tree model + simple forms

New crate `clause-transpile`. Emission-tree types
(`ClauseValue` / `ClauseScalar` / `ClauseBlock` /
`ClauseBlockItem`), deliberately neutral to output format.
Transpile covers `fn` bodies with: literals, path
expressions, simple calls, struct literals, field access,
if / else, match (arms limited to literal / path / tuple
patterns), return. `EmitPlan { item_name, item_kind, body,
source_span }` as the transpile output.

Size: large. One PR. Dependency: M3.1 + M2.2. Outcome: a
Clause `fn` that does straight-line arithmetic or simple
branching transpiles to real Clausewitz script AST.

### M3.4 — transpile — control flow + closures

The part Python Clause punted on. `for` / `while` / `loop` /
closures / complex patterns all emit real code, not
placeholder comments. Requires a concrete lowering strategy
(since Clausewitz has limited flow-control primitives — the
lowering is non-trivial and may introduce local scripted
variables for loop bookkeeping).

Size: large. One or two PRs. Dependency: M3.3. Outcome: the
full Clause language lowers to the Paradox target.

### M3.5 — codegen — kind routing + collision detect

Rewrite `clause-codegen` around the `EmitPlan` + emission
tree. Routing table `item_kind → output subdir`. `#[file]`
override. Collision detection with clear diagnostic. Output
pretty-printer (tabs, braces, comments).

Size: medium. One PR. Dependency: M3.4. Outcome: `.txt`
files land at the right paths; duplicates are flagged.

### M3.6 — source maps + `#[cfg]`-stripped emission

`source_map.json` (or `.bin` — pick the format in the design
round) sidecar per output file. 1-indexed line ranges from
emitted text back to `.cse` source. `#[cfg]`-stripped items
never appear in output.

Size: small-medium. One PR. Dependency: M3.5 + M2.1. Outcome:
runtime errors in the Paradox engine can be traced to Clause
source.

### M3.7 — auto-name + attribute overrides + kind heuristics

`#[name("literal")]`, `#[as_scripted_effect]` /
`#[as_scripted_trigger]` / `#[as_script_value]` /
`#[as_inline_script]`. Kind heuristic: fn with return type →
`script_value`; fn returning `()` or `Outcome` →
`scripted_effect`; `event Name for Country` → `country_event`.

Size: small. One PR. Dependency: M3.5. Outcome: attribute
surface matches the Python spec; heuristics converge on the
same kind assignments as the Python test corpus.

### M3.8 — extern signatures + Clausewitz G0 parser

New crate `clause-clausewitz`. Parser for Clausewitz script
(the target language). Needed for: reading extern signatures
from spec-ingest data, reading existing mod content for patch
ops, cwtools integration. One parser, not two.

Size: medium. One PR. Dependency: M3.3 (uses emission tree).
Outcome: Clause can read the target-language source, not just
write it.

### M3.9 — patch ops

Eight operations with priority: `override` > `replace_in_item`
> `inject_field` > `prefer_mod` > `delete_item` >
`insert_item` > `merge_union` > `no_override`. Per-item driver
like Python's `compile/item.py`. `#[patch(target="...")]`
declarations get resolved.

Size: large. One PR. Dependency: M3.8 + M2.2. Outcome: mod
authors can patch vanilla / other-mod content from Clause
source.

## M4 — macros

After M3, the compiler works end-to-end on macro-free inputs.
M4 brings macros up to and beyond Python parity.

### M4.1 — macro expansion: built-in macros

New crate `clause-macros`. Built-in macro registry
(compile-time inventory, no runtime registration). Ship:
`stringify!`, `concat!`, `format!`, `include_str!`, `env!`,
`compile_error!`, `assert!`, `assert_eq!`, `dbg!`, `println!`
/ `eprintln!` (engine-log-aware), `quote!`. Expression-
position and item-position registries.

Size: medium. One PR. Dependency: M3.3. Outcome: built-in
macros work; most Python tests that use built-ins are
portable.

### M4.2 — user-authored macros: declarative subset

`macro NAME(PARAMS) -> T { BODY }` — declarative subset
(pattern → template). Matches a superset of `macro_rules!`
with cleaner syntax. Hygiene via scope stack (NOT via
no-shadow; that's a different feature). Call-site span
propagation. `$ident` / `$($rep)*` / `$(expr)`
interpolations.

Size: large. One PR. Dependency: M4.1 + M2.2. Outcome: users
can define pattern-based macros as in Rust `macro_rules`.

### M4.3 — user-authored macros: procedural via shared walker

Rust Clause macro bodies may contain the full language,
including `struct` / `trait` / `impl` / `match` / closures.
Key difference from Python: **the macro interpreter is the
same IR walker the typechecker uses.** No separate tree-walker,
no divergence. Macro body compiles to MIR, runs at compile
time against a bound environment of token-stream params.

Size: large. One PR. Dependency: M4.2 + M2.3. Outcome: macros
are first-class. This is a strict improvement over Python
Clause.

### M4.4 — macro hygiene + diagnostics

Scoped hygiene so macro-introduced bindings don't collide
with caller scope. Shadow-lint continues to catch author
mistakes, but hygiene is a language guarantee, not a lint
policy. Call-site vs body-site diagnostic spans cleanly
distinguished.

Size: medium. One PR. Dependency: M4.3. Outcome: macro users
don't need to worry about name collisions; diagnostics are
precise.

## M5 — passes + validators + spec

Ecosystem. After M4 the compiler is a complete language.
M5 builds the authoring ecosystem around it.

### M5.1 — validator implementations (STRICT1-7 + coherence + orphan + bind-target-shape)

The ten validators from the Python typecheck and strict
families get real bodies. Fits on top of the validator
framework from M2.7.

Size: medium-large. One PR, possibly split. Dependency:
M2.7 + M3.2. Outcome: the same STRICT lint family that
Python Clause ships.

### M5.2 — content lint pack

Port the big Python linters: `CONTENT_SYNTAX_LINT` (12+
sub-rules around scripted-effect / scripted-trigger /
script-value authoring), `PATCH_SHAPE_LINT` (13 sub-rules
for patch ops), `UNDEFINED_REFS_LINT` (entity / scripted-
trigger / scripted-effect / event / localisation-key checks),
three scripted-call-depth checks.

Size: large. Probably two PRs. Dependency: M5.1 + M3.9.
Outcome: the Python content-lint surface is fully ported.

### M5.3 — spec ingest — adapters + merger

New crate `clause-spec`. Spec adapters (paradox docs,
cwtools, vanilla analysis), merger (multi-source
reconciliation), overlay (user-defined extension), writer
(per-game extern-signature table). Ships the same
`compiler/spec_ingest/` surface as Python.

Size: large. Two PRs. Dependency: M2.4 (extern consumes
spec). Outcome: `clause spec harvest` + `clause spec search`
/ `show` / `scope` / `list` work.

### M5.4 — analyzer passes

`ITEM_INDEX`, `CALL_GRAPH_ANALYZER`, `DEPTH_ANALYZER`,
`LEAF_ANALYZER`, `GLOBAL_VAR_LOADER` — the five core analyzers
that downstream linters and optimizers consume.

Size: medium. One PR. Dependency: M0.2 + M3.5. Outcome:
artifact graph covers the Python analyzer surface.

### M5.5 — optimizers

`LEAF_INLINER` ports first. Further optimizations (constant
folding, dead-code elimination, scope merging) listed in a
BACKLOG that later rounds pick from.

Size: medium. One PR. Dependency: M5.4. Outcome: output
`.txt` files are smaller and faster to evaluate in the
engine, matching Python Clause's optimization passes.

## M6 — cli + manifest + tooling

Productisation. After M5 the compiler is complete. M6 turns
it into a daily-driver tool.

### M6.1 — manifest parser

`Clause.toml` reader. Workspace + package. Dep graph
(workspace / path / version / features). DLC / feature
declarations. Rename declarations. Storage lock section.
`AutoDiscover` rules. Provides the multi-file build graph
the front-end currently lacks.

Size: medium-large. One PR. Dependency: M1.5. Outcome:
multi-file multi-crate Clause projects build end-to-end.

### M6.2 — rename shims

`#[supersedes]` / `#[deprecated]` → `symbol_aliases.manifest`
sidecar. This time without the Python `FIXME(rename-naive)`
— supports cross-crate renames, chained renames,
non-event-kind renames, and save-embedded migration. Design
round goes first; the limitations must be lifted before
implementation.

Size: medium. One PR for design + one for implementation.
Dependency: M6.1. Outcome: evolving Clause code does not
break existing save games or consumer scripts.

### M6.3 — diagnostics registry (structured)

Replace the free-form Python codes with a structured
registry: (category, severity, code, brief, detail, example,
machine-readable fix patch, related-codes). `clause explain`
reads this. `clause fix` can apply the machine-readable
patches. ~110 codes enumerated and categorised.

Size: medium. One PR. Dependency: pieces accumulate; should
be retrofit every round that adds a code, but this round
formalises the structure. Outcome: authoring-experience
ergonomics match or exceed Python `clause explain`.

### M6.4 — CLI: `check` / `build` / `test` / `new` / `explain`

Populate the frozen stubs. `check` runs up to typecheck and
validator; `build` runs through codegen; `test` compiles
`#[test]`-flagged functions; `new` scaffolds a workspace;
`explain` serves the structured registry.

Size: medium. One PR. Dependency: M6.3 + all earlier
milestones. Outcome: five of the Python CLI subcommands work.

### M6.5 — CLI: `doc` / `fmt` / `fix`

`doc` generates markdown from Clause doc comments. `fmt` is
the language formatter. `fix` applies machine-readable
diagnostic fixes. None of these exist in Python Clause
meaningfully.

Size: medium-large. Two PRs (doc separately). Dependency:
M6.4. Outcome: Rust Clause exceeds Python in authoring
support.

### M6.6 — CLI: `compile-crate` / `build-playset` / `doctor` / `spec` / `deploy`

The remaining Python CLI subcommands. `compile-crate` / 
`build-playset` handle cross-crate orchestration. `doctor`
runs preflight probes. `spec` is the spec-ingest driver.
`deploy` copies artifacts to the engine's mod directory.

Size: medium-large. Two-three PRs. Dependency: M6.4 + M5.3.
Outcome: every Python CLI subcommand has a Rust equivalent.

### M6.7 — playset orchestration

New crate `clause-playset`. Multi-mod orchestration, workshop
scan, load-order compute (evidence-weighted graph), item
extraction. Python has a lot here; port the meaningful parts
and drop the legacy.

Size: large. Probably three PRs. Dependency: M6.1 + M5.4.
Outcome: users running dozens of coexisting mods get coherent
build output.

## M7 — optional beyond-parity extensions

After M6, Rust Clause matches Python Clause. M7 is where it
goes further. No ordering; pick as priorities shift.

### M7.1 — interpreter (`clause run`)

Python Clause has no `run` subcommand. Rust Clause can run
Clause bytecode in a dedicated interpreter for testing,
scripting, and agent-driven workflows. Bytecode format gets
its own design round; likely a simple stack VM with typed
values per the notko fallibility ladder.

Size: large. Two-three PRs. Outcome: `clause run foo.cse`
executes; `clause test` runs real tests, not just compiles.

### M7.2 — LSP server

Full-IDE integration. Hover, go-to-def, find-references,
rename (uses M6.2 rename infrastructure), diagnostics push,
code actions (apply M6.3 fixes). Built on the
error-recovering parser's incremental reparse surface.

Size: very large. Multiple PRs. Outcome: Rust Clause is the
first Paradox modding language with serious editor support.

### M7.3 — debug adapter

DAP server so authors can set breakpoints in Clause source
and step through execution (via interpreter or via
instrumented Paradox engine).

Size: very large. Outcome: modders debug instead of
printf-ing.

### M7.4 — additional emission targets

Python Clause emits Clausewitz only. Rust Clause is
target-agnostic by design. Additional targets:
- `clause-lua` — Lua target for modding environments that
  use Lua.
- `clause-llvm` — ahead-of-time compile to native for
  performance-sensitive bits.
- `clause-wasm` — Wasm target for sandboxed execution.

Size: very large each. Outcome: Clause is a polyglot
authoring language.

### M7.5 — formal verification integration

Structured type system + `#[pure]` / `#[invariant]`
attributes + SMT solver integration. Lets authors annotate
assumptions the engine can't check, get them verified at
compile time.

Size: very large. Research-stage. Outcome: Clause becomes a
formal-methods candidate.

## Estimated effort summary

| Milestone | Rough PR count | Calendar estimate (solo, focused) |
|-----------|----------------|-----------------------------------|
| M0        | 3              | 1-2 weeks                         |
| M1        | 5              | 4-6 weeks                         |
| M2        | 7              | 8-10 weeks                        |
| M3        | 9              | 10-12 weeks                       |
| M4        | 4              | 4-6 weeks                         |
| M5        | 8              | 8-10 weeks                        |
| M6        | 10             | 10-12 weeks                       |
| M7        | open           | beyond-parity, ongoing            |

Total to M6 (full parity + exceed): ~46 PRs, ~8-12 months
solo. Parallelizable across ~3 contributors cuts to ~4-6
months.

## How to consume this plan

- Each milestone entry is a design-round topic. Opening it
  means writing the topic, doc CL, src CL, executing, and
  merging — the standard mockspace flow.
- Dependencies are hard. Don't start a round whose
  dependency is not landed unless you accept rebase pain.
- Sizes are rough. Some rounds will be smaller than
  estimated; a few will be bigger and need splitting.
- The `clause-dev` workspace rules apply throughout. No new
  bare primitives. No runtime registration. No heap in
  phase-crate internals. Type-surface discipline stays on.
- `original-docs/CLAUSE_EBNF.md` and `GRAMMAR.md` are the
  language spec. Diverge only with a design round
  documenting the divergence.
- Python tests in `/Users/orgrinrt/Dev/stellar-heritage/
  tools/clause/compiler/tests/` are a feature oracle — a
  passing Python test should pass the Rust compiler on the
  same input (where the feature is implemented).
- Python source is a semantic reference, not a blueprint.
  When in doubt, read the Python to learn the *what*; decide
  the *how* from `lessons-learned.md`.

---

When M6 lands, Rust Clause is a complete reimplementation of
Python Clause, plus: type inference, hygienic+procedural
macros, structured diagnostics, zero-copy cross-crate
artifacts, unified pipeline, declarative pass registration,
formatter + doc-gen + fix-apply tooling. M7 extends it
beyond any modding-language existing today.
