# Lessons learned from Python Clause

**Date:** 2026-04-21
**Audience:** Rust Clause port architects and implementers.

Reading this doc assumes a pass through
`python-clause-survey.md` first. This doc is opinionated —
it says what the Rust port should inherit, what it should
reject, and where it should go further than the Python
reference.

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

Rust port inherits this wholesale. `bincode` + `blake3`
replace JSON + SHA-256 for speed; otherwise the shape
stays.

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
Binary format (same `bincode` + `blake3` pattern) for speed.

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
#108 landed). Passes are registered declaratively via
something like `linkme` / `inventory` or a generated static
table at build time. Known at compile time = analysable at
compile time.

### 2.5 Hand-rolled JSON codec for artifacts

`passes/artifacts.py` invents tags like `\x00dc`,
`\x00tuple`, `\x00tupdict`, `\x00patcherr` to round-trip
tuples, tuple-keyed dicts, and frozen dataclasses through
JSON. Hundreds of lines fighting `json.dumps`.

**Rust port: `serde` + `bincode` / `rkyv`**, and the whole
problem disappears.

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

**Rust port: typed on-disk layout** — one file per pass
artifact, `blake3`-named, `rkyv`-serialised, no global
write lock. Or a single-file format with per-table
atomicity (sled, redb). Don't inherit the bottleneck.

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

## 4. Five features strictly better in Rust

### 4.1 Parser error recovery via `logos` + recursive descent

`logos` for token classification (pure fn, codegen'd DFA),
recursive descent hand-rolled on top. Multi-error reports,
typed spans, tree-sitter-compatible grammar as a
side-effect if we want an LSP later.

### 4.2 Arena-allocated immutable AST with `NodeId` indexing

Fixed by construction. No mutate-after-parse. Traversal
iterators are trivially safe. Interning by index is the
natural collision-avoidance strategy. Cross-crate AST
serialisation is trivially `rkyv`-zerocopy.

### 4.3 Pass fingerprint with `blake3` + `bincode`

`blake3` is ~10× faster than SHA-256 on common inputs and
is still cryptographically sound. `bincode` on
`#[derive(Serialize)]` structs is faster than any JSON
codec. Fingerprinting is on the hot path — speeding it
speeds every build.

### 4.4 Shared walker for typecheck + macro interpretation

If the macro interpreter is the same IR walker the
typechecker uses (with a different evaluation mode), the
language's semantics are defined once. No drift. No
"works in typecheck, fails at expansion" bug class.

### 4.5 Storage catalog as const-generic trait

Instead of string-matching on backend names in a Python
dict, the storage backend is an associated type on a
`#[repr(...)]`-derived const parameter. Compile-time
dispatch, const-foldable, statically checkable.

### 4.6 Diagnostics via `ariadne` or `annotate-snippets`

Multi-span highlights, colour, unicode underlines, label
chains out of the box. Free quality.

### 4.7 Cross-crate artifacts via `rkyv` zero-copy

Tens-of-ms → sub-ms artifact loads. Important when the
workspace has dozens of crates.

## 5. Meta-lesson: what to design next

The Python implementation teaches one meta-lesson: **keep
the pass scheduler, the authoring front-end, the content
back-end, and the per-target adapters as one architecture,
not three bolted together.** The Rust port has the chance
to do this from day one because the skeleton is fresh.

Concretely:

1. `clause-schedule` must ship a real pass DAG before any
   phase grows a body. The front-end should be a chain of
   passes from the very first `clause build` that does more
   than lex.
2. The macro interpreter must share the walker with the
   typechecker. If that means building the typechecker
   harness first and the macro body evaluator as a mode on
   top, do it.
3. Asset decorators and content-target attributes ship with
   their codegen, not before. If the codegen isn't ready,
   the attribute isn't in the grammar.
4. Rename / deprecation / migration is a design-round
   topic before it's an implementation topic. No
   `FIXME(rename-naive)` twins in the Rust code.

Everything else in the Python implementation is salvage
material or cautionary tale. The language itself — its
Rust-ish surface, its `event` / `expect` / `actual` /
`sealed` / `#[cfg]` additions, its storage routing and
transpile model — is a genuinely good language design.
Port it; improve it; remove the rot; ship it.
