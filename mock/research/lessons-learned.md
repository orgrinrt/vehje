# Lessons learned from Python Clause

**Date:** 2026-04-21
**Audience:** Rust Clause port architects and implementers.

> **Substrate principle is load-bearing.** Read
> `substrate-principle.md` before this doc. Every "how to do X
> better" answer below goes through notko / arvo / hilavitkutin
> primitives, never through external crates.

Reading this doc assumes a pass through
`python-clause-survey.md` first. This doc is opinionated —
it says what the Rust port should inherit, what it should
reject, and where it should go further than the Python
reference, always via the substrate.

## 1. What the Python version got right

### 1.1 Error-recovering parser as default

The Python parser accumulates diagnostics, inserts
`ErrorNode` / `ErrorExpr` / `ErrorItem`, and syncs to
item-starters or `;` / `}` rather than failing at the first
mistake. This is load-bearing: a build that stops at the
first syntax error is hostile; a build that reports every
error in one pass is humane.

The Rust port should preserve this. Error recovery nodes
are first-class AST variants, not sentinel values. They carry
spans. Downstream phases skip them. Diagnostics stream
through the sink.

### 1.2 Emission as a tree, not as text

`ClauseValue` / `ClauseScalar` / `ClauseBlock` /
`ClauseBlockItem` decouple transpile (structural) from
pretty-printing (textual). Format bugs, indentation tweaks,
and target-specific whitespace concerns live in one place.
Transpile never thinks about tabs.

Carry this forward. A neutral emission tree has zero runtime
cost in Rust (it's a `Node` enum), and it makes the
transpile → codegen boundary trivially testable.

### 1.3 Kind-keyed routing table

`codegen.py` routes by `item_kind`, not by path or attribute
magic. Every item kind knows its target subdirectory.
Collisions detected. `#[file]` overrides when needed.

Keep this. Route by kind first, let attributes be the
escape hatch, not the default.

### 1.4 Typed `Artifact` enum + SHA fingerprint + cache

The pass scheduler's `Artifact` enum lets passes declare
what they read and write in types, not names. The scheduler
derives write→read edges, topsorts, and caches on
`SHA-256(pass_id, version, read-artifact fingerprints,
config_inputs)`. Deterministic and sound.

Rust port inherits the *shape* — but the engine is
hilavitkutin, not a clause-internal scheduler. Passes are
hilavitkutin WorkUnits; artifacts are Columns declared by the
WorkUnit's AccessSet. The scheduler is hilavitkutin's own.
Fingerprinting uses `arvo_hash::ContentHash` over the
substrate's in-memory `Encoder` stream (no JSON, no
`bincode`; the engine's serialisation is the canonical one).
Cache keys live in `hilavitkutin_persistence`'s cold store.
If the substrate is missing any piece, extend the substrate —
do not duplicate the concept inside clause.

### 1.5 Structured diagnostic registry

`Explanation(brief, detail, example, fix, related)` with
~110 `CL_*` codes, served via `clause explain --code`. More
than a message string: a curriculum. Makes user errors
self-explanatory.

Keep the shape. Tighten the prose (register categories +
severity tiers + machine-readable fix patches).

### 1.6 Seven-backend storage catalog

Domain-specific but the pattern generalises: a struct field
gets *routed* to an implementation, with per-field hard
override (`#[repr]`) and soft hint (`#[prefer]`), reconciled
against a manifest lock file with drift detection.

The pattern is reusable for any scenario where the language
surface has multiple lowering options. Rust port: preserve
the pattern; keep the backends; add a registration mechanism
so downstream targets can contribute more.

### 1.7 `expect` / `actual` KMP-style cross-crate stubs

Declaring a spec in one crate and the body in another (or
in a target-specific crate) is a clean module-boundary
story. Clause uses it for multi-target splits; the Rust
port can reuse it for Rust-compiler / Zig-runtime split and
for game-specific target impls.

### 1.8 Attribute-driven kind selection

`#[as_scripted_effect]`, `#[as_script_value]`,
`#[as_inline_script]` override heuristic kind choice.
Heuristics are 95% right, attributes cover the 5%.

Pattern generalises: heuristic first, attribute as
precise override.

### 1.9 `CompileEnv` as first-class conditional-compilation

`#[cfg(dlc=X, mod=Y, feature=Z)]` evaluated against a
`CompileEnv` struct. Three orthogonal conditional axes, all
named. Rust port keeps this and adds `target=` (which
compile target) and `edition=` (which language edition).

### 1.10 Validator framework single-walk + scope stack + dedup sink

One visitor per program, not one per lint. Each validator
registers `enter` / `leave` / `finalize_scope` handlers.
Dedup sink keyed on `(span.start, span.end, code)` prevents
duplicate diagnostics.

Rust port: same shape. Visitor is a trait; scope stack is a
fixed-size array with const-generic depth cap. Sink is
the `DiagnosticSink` from `hilavitkutin-api`.

### 1.11 Source-map sidecars

`source_map.json` per emitted file, 1-indexed line ranges
back to `.cse` origin. Makes runtime errors traceable to
the author's source.

Rust port ships this from day one, not as a later polish.
Binary format lives in `hilavitkutin_persistence` cold store,
keyed by the output file's `arvo_hash::ContentHash`. The
source-map rows are a Column declared by clause-codegen; the
on-disk format is hilavitkutin's, not clause's.

### 1.12 Call-site span propagation for macros

Diagnostics from macro expansion point at the invocation
site, not the macro body. Essential for UX.

Rust port gets this free if the macro interpreter shares
the same `Span` type as the rest of the compiler.

### 1.13 Pass-kind distinction

`Analyzer` / `Linter` / `Optimizer` / `Writer` / `Generator`
— five kinds with distinct error-handling semantics. A
linter's failure is different from a writer's failure.

Rust port inherits the distinction, unhooks `Generator`
from LLM-retry semantics (that's one policy; there should
be others).

## 2. What the Python version got wrong

### 2.1 Two parallel pipelines

The pass scheduler was built for pre-Clause `.txt`-patch
orchestration. The Clause front-end runs outside it, which
means:

- Every build re-does lex / parse / resolve / typecheck
  from scratch.
- Caching is marked `(n/a)` in the build summary.
- The authoring pipeline and the content pipeline
  duplicate effort.

**Rust port: one scheduler from day one.** Every phase is
a pass with typed inputs and outputs. Caching is universal.
`cargo mock --check` and `clause build` use the same
pipeline, same cache.

### 2.2 `Option<T>` / `Result<T, E>` / `Vec<T>` in the language

`resolved_types.py` hard-matches names like `"Option"`,
`"Result"`, `"Vec"` to enable exhaustiveness + language
specialisations. These are Rust-isms imported whole-cloth
without the Rust semantics behind them.

**Rust port: the Clause language itself carries `Maybe<T>`
/ `Outcome<T, E>` / sink-shaped collection surfaces, not
`Option` / `Result` / `Vec`.** The compiler internals
already use them (via notko + hilavitkutin-api); the
language surface should too. This is a spec change, not
just an implementation detail.

### 2.3 Reference types parsed then erased

`RefType` / `RefExpr` / `RefPat` exist in the AST, consume
parse budget, survive typecheck, then the transpiler drops
them. The author thinks they're writing borrow-aware code;
the compiler treats `&foo` as `foo`.

**Rust port picks one.** Either (a) Clause has borrow-ish
semantics, enforced, useful; or (b) the reference syntax is
removed from the grammar. Middle ground is a lie.

### 2.4 Runtime pass discovery

`PassRegistry` uses `inspect` + `pkgutil` to walk
`compiler.passes.builtin.*`. Plugin-style runtime
registration.

**Rust port forbids this** (`no-runtime-registration` lint,
#108 landed). Passes are WorkUnits, registered through the
hilavitkutin engine's static composition surface. No
`linkme`, no `inventory`, no crate-level plugin trick —
hilavitkutin's own registration is the mechanism. If the
engine does not yet expose a registration surface that fits
clause's needs, that is a hilavitkutin round, not a clause
crate.

### 2.5 Hand-rolled JSON codec for artifacts

`passes/artifacts.py` invents tags like `\x00dc`,
`\x00tuple`, `\x00tupdict`, `\x00patcherr` to round-trip
tuples, tuple-keyed dicts, and frozen dataclasses through
JSON. Hundreds of lines fighting `json.dumps`.

**Rust port: `hilavitkutin_api::Encoder` / `Decoder`** with
persistence through `hilavitkutin_persistence` — the
substrate's own on-disk format. No `bincode`. No `rkyv`. No
`serde`. The engine's serialisation is the canonical one; if
it does not yet cover what clause needs, extend
hilavitkutin-api / hilavitkutin-persistence. Adding an
external codec dep is the wrong answer every time.

### 2.6 AST dataclass-defaults-of-None abuse

Dozens of AST nodes in `grammar/clause/ast.py` have fields
like `body: BlockExpr = field(default_factory=lambda:
None)  # type: ignore[assignment]`. Python's dataclass
can't express "required but forward-referenced", so the
code pretends `None` is a `BlockExpr`.

**Rust port: immutable AST, arena-allocated, indexed by
`NodeId`.** A reference is always valid; the borrow checker
prevents the class of mutate-after-parse bugs entirely.

### 2.7 Macro interpreter as a separate tree walker

`macro_interpreter.py` (956L) is a separate walker from
`typecheck.py`. Two interpretations of AST drift.

**Rust port: share the walker.** Lower macros onto the
same MIR the typechecker walks. One interpretation, one
source of truth.

### 2.8 Two Clausewitz parsers

`grammar/clausewitz/` (clean G0 subclass, 1079L) and
`compiler/clausewitz/` (legacy hand-rolled, 545L) coexist.
The legacy one was absorbed from pre-Clause tooling.

**Rust port: one parser.** Use the clean G0-style
grammar as the model; the legacy entry points get adapters
if they need them.

### 2.9 Free-form prose diagnostic codes

~110 `CL_*` codes with free-form `brief` / `detail` /
`example` / `fix` strings. No category. No severity
gradient. No machine-readable fix.

**Rust port: structured codes.** Each has a category (e.g.
`parse`, `typecheck`, `storage`), a severity (`error`,
`warning`, `note`), optional machine-readable fix (so
`clause fix` can patch source), and optional related code
links.

### 2.10 SQLite as the primary cache

Single-file, WAL-mode, "writes serialised through the
parent process". Works; slow under parallel builds.

**Rust port: `hilavitkutin_persistence` cold store** — one
entry per pass artifact, keyed by
`arvo_hash::ContentHash`, written through the substrate's
own format. No `sled`, no `redb`, no `bincode`, no `rkyv`.
The cold store already carries its own atomicity contract;
if clause finds it insufficient for the workload, extend
hilavitkutin-persistence. Duplicating the concept inside
clause is the wrong answer.

### 2.11 Asset decorators designed but not wired

`#[gfx_sprite_type]`, `#[localisation]`, `#[icon]`, et al.
exist in the grammar, exist in docs, not wired anywhere.
~15 months of design debt.

**Rust port: decide upfront.** Either wire them in the
codegen round that introduces them, or drop them from the
grammar. No dead syntax.

### 2.12 Reserved-but-unused keywords

`unsafe`, `async`, `await`, `dyn`, `move` — tokenised,
parser rejects them. Looks like a feature; is not.

**Rust port: pick one.** Either implement, or drop from
the lexer. The worst outcome is "tokenised, rejected, user
thinks it should work".

### 2.13 Generator pass coupled to LLM retry semantics

The `Generator` pass kind bakes in LLM-specific retry
policies. Conflates concerns.

**Rust port: generator is orthogonal to provider retry.**
The retry policy is a separate trait; `Generator` is just
"a pass that writes artifacts derived from external
input".

### 2.14 Rename system known-broken

`FIXME(rename-naive)` in `manifest.py`. Cross-crate
renames, chained renames, non-event-kind renames, and
save-embedded migration are all unfinished.

**Rust port: rename as a first-class problem.** Design
rounds explicitly cover cross-crate reach, chain
resolution, non-event-kind shams, and save-embedded
migration BEFORE implementation starts.

## 3. Five big downfalls

1. **Pipeline split left the Clause compile path without
   incremental caching.** The user sees every build as a
   cold build. Architecturally the biggest loss.
2. **Macro interpreter v1 covers only a subset.** Users hit
   "not supported inside macro body" walls for `struct`,
   `trait`, `impl`, `match`, closures. A macro system that
   won't let you define `struct` inside isn't much of a
   macro system.
3. **Asset-routing decorators designed but never wired.**
   Dead weight in the grammar that taught users to expect
   something the compiler doesn't do.
4. **Rename system acknowledged broken.** A single
   `FIXME(rename-naive)` line covers cross-crate / chained
   / non-event / save-embedded — four unsolved problems
   stacked.
5. **Transpile emits placeholder comments for `for` /
   `while` / `loop` / closures.** The grammar promises
   full control flow; the backend ships a subset. Clause
   programs that work in typecheck fail silently at
   transpile.

The common thread: **promise first, wire later** failed.
The grammar / CLI / docs got ahead of the backend. The
Rust port should keep grammar changes gated on backend
support.

## 4. What the substrate makes strictly better

Every item below is "Rust Clause does better than Python
Clause *because the substrate already solves this for the
whole stack*." None of these call out an external crate.
Every one of them is a consume-from-substrate or
extend-the-substrate answer.

### 4.1 Parser error recovery and arena-allocated AST

Hand-rolled recursive-descent parser (already shipped in
`clause-syntax` skeleton). AST lives in arena-allocated
Columns declared by the crate. Each `NodeId` is a
`arvo::USize` / `arvo::UFixed<N, 0, S>` indexing into the
Column — not a pointer. No `Arc` / `Rc` / `Box`. No
mutate-after-parse because the Column is write-once (a
hilavitkutin-engine invariant).

Benefit over Python: the borrow checker plus the engine's
AccessSet model prevents the whole class of
mutated-after-parse bugs Python Clause fights with
`field(default_factory=lambda: None)`.

### 4.2 Cross-crate artifacts through hilavitkutin-persistence

Cross-crate artifact handoff goes through the engine's
cold store, keyed by `arvo_hash::ContentHash`. The store
already ships zero-copy load semantics where applicable;
if clause needs a more specific cold-store read path,
extend hilavitkutin-persistence rather than reach for an
external serialiser.

Benefit over Python: one codec, one locking story, one
caching story, across the compile passes, the validators,
the spec-ingest harvester, and any future interpreter
state.

### 4.3 Pass fingerprinting via `arvo_hash::ContentHash`

`arvo_hash::ContentHash` is a typed content-address over
the input bytes of a pass (WorkUnit id, version, resolved
AccessSet, Column hashes for every read input,
config-input hashes). Faster than SHA-256 where the
underlying algorithm is tuned for that; the point is
*which layer owns the primitive*, not which algorithm it
picks today. If clause surfaces a need the current arvo-
hash doesn't cover, extend arvo-hash.

Benefit over Python: deterministic, typed, stack-wide.
Every WorkUnit in the workspace uses the same hash story;
clause gets it for free.

### 4.4 Shared walker for typecheck + macro interpretation

Python's typechecker and macro interpreter are two tree
walkers that drift. The Rust port collapses them: the
macro interpreter is a mode on the same IR walker the
typechecker uses. The IR walker itself is a WorkUnit —
its reads (AST Column, symbol-table Column) and writes
(typed-IR Column, diagnostic sink) are declared once.

Benefit over Python: no "typechecks but fails at
expansion" class of bugs. The engine's incremental
recompute automatically recomputes macro expansions when
their deps change.

### 4.5 Storage catalog as trait-first substrate consumers

The storage router is a WorkUnit family. Each backend
(`flag`, `variable`, `scripted_variable`, `event_target`,
`scripted_list`, `string_storage`, `paired`) is an
impl of a substrate trait — written against
`hilavitkutin_api` primitives for any persistent state it
needs. `#[repr]` / `#[prefer]` are const-generic drivers;
the router WorkUnit dispatches at monomorphisation time.

Benefit over Python: compile-time backend dispatch,
zero-cost, statically verifiable against the manifest
lock file. No dict-of-strings.

### 4.6 Diagnostics rendered through substrate sinks

Diagnostics emit through `DiagnosticSink<D>` from
hilavitkutin-api. Rendering (ANSI / plain / JSON / LSP
wire-format) is a WorkUnit downstream of the producer; it
reads the sink's Column and writes to a `ByteEmitter`
(also hilavitkutin-api).

Benefit over Python: no external renderer dep (`ariadne`,
`annotate-snippets`). Clause-owned renderer layered on
the same sink contract every other substrate consumer
uses. If the renderer shape generalises (color selection,
unicode underlines, label chains), it migrates *up* to
hilavitkutin-api — but the *dependency* never leaves the
stack.

### 4.7 Parallelism and thread pool from hilavitkutin

The engine owns the thread pool. Clause does not spawn
threads, does not call `rayon`, does not touch
`std::thread`. WorkUnits declare their AccessSets; the
scheduler runs independent WorkUnits in parallel. Morsel
size is the engine's responsibility.

Benefit over Python: full parallelism for every
independent compile phase, automatic, with zero
clause-side code touching thread primitives. Matches the
`no-runtime-spawn` lint (#107).

### 4.8 Runtime execution on hilavitkutin (future `clause run`)

When clause grows an interpreter, it does not grow a
second VM. The interpreter is a family of WorkUnits that
read a bytecode Column and write result Columns. Test
assertions are WorkUnits too. The engine's scheduler
runs them; the engine's persistence stores any
between-test state.

Benefit over Python (which has no `run`): a single
runtime, the same one the compile passes use, covers
both compile and interpret modes. No drift. No
duplicated scheduling code. No custom VM.

## 5. Meta-lesson: what to design next

The Python implementation teaches one meta-lesson:
**clause is not its own stack.** Python Clause tried to be
one — it invented a pass scheduler on top of `graphlib`, a
cache on top of SQLite, a serialiser on top of `json` with
custom tags, a task registry on top of `inspect` +
`pkgutil`. Every invention is a maintenance burden and a
drift vector.

Rust clause applies a substrate — notko, arvo,
hilavitkutin — that already solves these problems for the
whole stack. Clause adds only language-specific semantics
on top. Read `substrate-principle.md` for the full rule.

Concretely:

1. **The pass engine is hilavitkutin.** `clause-schedule`
   is where clause-specific WorkUnits live; it is not a
   pass framework. Every clause compile phase is a
   WorkUnit. Every artifact is a Column declared by its
   WorkUnit's AccessSet. The scheduler is hilavitkutin's.
2. **The macro interpreter is a WorkUnit that shares the
   IR walker with typecheck.** Same Columns. Same
   AccessSet shapes. No separate walker. No duplicated
   evaluation logic.
3. **Persistence is hilavitkutin-persistence.** Every cache
   key, every cross-crate artifact, every source-map
   sidecar. Clause does not spell binary formats. If the
   substrate is missing a cold-store entry shape, extend
   hilavitkutin-persistence — do not invent in clause.
4. **Numerics and hashing are arvo.** Every `NodeId` is an
   arvo primitive. Every fingerprint is `arvo_hash::
   ContentHash`. Every bit contract (TokenKind variants,
   storage-backend flags, visibility bitfields) is
   `arvo_bits::Bits<N>`.
5. **Fallibility is notko.** Every return shape uses
   `Maybe` / `Outcome` / `Just`. Every lint the workspace
   ships enforces this. `Option` / `Result` appear only at
   `core::` trait-method boundaries with tracked
   exemptions.
6. **Asset decorators and content-target attributes ship
   with their codegen, not before.** If the codegen isn't
   ready, the attribute isn't in the grammar.
7. **Rename / deprecation / migration is a design-round
   topic before it's an implementation topic.** No
   `FIXME(rename-naive)` twins in the Rust code.

Everything else in the Python implementation is salvage
material or cautionary tale. The language itself — its
Rust-ish surface, its `event` / `expect` / `actual` /
`sealed` / `#[cfg]` additions, its storage routing and
transpile model — is a genuinely good language design.
Port it; improve it; remove the rot; ship it on the
substrate.
