# Python Clause — survey for the Rust port

**Date:** 2026-04-21
**Corpus:** `/Users/orgrinrt/Dev/stellar-heritage/tools/clause/` —
~41k LOC Python compiler, 25k LOC tests, plus playset/deploy/artgen
siblings. ~264 `.py` files, 99 test files, 1511 test functions.

**Headline.** Python Clause is a Rust-syntax authoring language
(`.cse`) whose bodies transpile to **Clausewitz script** — the
plaintext scripted format Paradox Interactive games consume
(Stellaris, CK3, HOI4, EU4, Victoria 3). The Clause grammar mimics
Rust's surface ~1:1 (items/traits/impls/enums/macros/generics);
the target is not bytecode, not Python — it is game-engine text
files under `common/scripted_effects/auto/*.txt`,
`events/auto/*.txt`, etc., routed per-item-kind. That reframes
everything: Clause is a **source-to-source transpiler** from a
typed, modular, macro-capable Rust-ish surface onto a flat,
untyped, name-collision-prone script tree.

> The Rust port is not chained to the Paradox target. Clause as
> a general-purpose authoring language can emit any target; the
> Paradox emit path is one adapter. This survey uses
> "Clausewitz" / "Paradox" as concrete examples because that is
> what the Python implementation emits today.

## 1. Language surface

See `original-docs/CLAUSE_EBNF.md` and `GRAMMAR.md` for the
canonical spec. Headline:

- **Items**: `mod`, `use`, `pub` / `pub(crate)` / `pub(super)`,
  `struct`, `enum`, `trait`, `impl`, `fn`, `const`, `static`,
  `type` (alias), **`event`** (Clause-native),
  **`expect` / `actual`** (KMP-style multiplatform pairing,
  Clause-native), `extern` (engine-provided),
  **`sealed`** (only defining crate may impl), `macro`
  (user-authored proc macros).
- **Control flow**: `if` / `else`, `match`, `for ... in`,
  `while`, `loop`, `return`, `break`, `continue`,
  `?` (early-return propagation).
- **Types**: `PathType`, `RefType` (`&T` / `&mut T` — parsed,
  **erased at transpile**; Clausewitz has no borrow concept),
  `TupleType`, generic args (`<T>` turbofish `::<T>` in expr
  position). Scalars: `i32` / `int`, `f32` / `float`, `bool`,
  `String`, `()`, marker structs.
- **Patterns**: wildcard, literal, ident (with `mut`), path,
  tuple-struct (`Ok(x)`), tuple, range (`1..=5`), or
  (`a | b`), rest (`..`), ref, bind (`x @ pat`).
- **Expressions**: literal, path, macro invocation, unary,
  ref, tuple, struct literal (with `..base` update), binary,
  assign + compound assign, range, call, method call, field
  access, index, block, if, match, for, while, loop, closure
  (`|p| body`), return, break, continue, paren, question.
- **Attributes**: outer `#[...]` + inner `#![...]`.
  Load-bearing:
  - `#[name("...")]` — rename emitted symbol.
  - `#[as_scripted_effect]` / `#[as_scripted_trigger]` /
    `#[as_script_value]` / `#[as_inline_script]` — force
    emission kind.
  - `#[file("...")]` — override output path.
  - `#[repr(<backend>)]` / `#[prefer(<backend>)]` — storage
    backend.
  - `#[cfg(dlc="X")]` / `#[cfg(mod="X")]` /
    `#[cfg(feature="X")]` — conditional compilation.
  - `#[supersedes(...)]` / `#[deprecated(...)]` — rename /
    migration.
  - `#[patch(target="...")]` — patch existing item.
  - `#[test]` — test harness.
- **Generics**: named params, per-param bounds, `where`,
  **const generics** (`const N: Type`), supertraits
  (`trait Sub: Super {}`), associated types (`type X;` in
  traits, `type X = T;` in impls), associated constants.
- **Reserved but unused**: `unsafe`, `async`, `await`, `dyn`,
  `move`. Tokens produced; parser rejects them in expr form.
- **Visibility**: `private` / `pub` / `pub(crate)` /
  `pub(super)`.
- **Module system**: `mod foo;` (filesystem sibling
  `foo.cse` or `foo/mod.cse`), `mod foo { ... }` inline,
  multi-segment self-id (`mod heritage::bloodline;`). `use
  a::{B, C}`, `use a::*`, `use a::X as Y`, `pub use`
  re-export.

### Clause-unique surface (vs. Rust)

1. `event Name for Country { body }` — first-class
   content-item declaration targeting Paradox event kinds.
2. `struct Bloodline : Country { ... }` — colon-syntax
   **bind target**; binds a struct to a Clausewitz scope /
   entity class (`Country`, `Pop`, `Planet`, …), making its
   fields live as scope-local storage.
3. `expect` / `actual` KMP-style cross-crate /
   cross-platform stubs.
4. `sealed trait` / `sealed struct` — coherence sealed.
5. Struct field modifiers `mut` and `const` — Clause-specific.
6. `#[cfg(dlc=..., mod=..., feature=...)]` with `all()` /
   `any()` / `not()` evaluator. DLC- and mod-aware
   conditional compilation.
7. `#[patch(target="mod::path::Item")]` — patch-chain
   declarations pointing at existing items to overlay.

## 2. Macro system

Hybrid: built-in macros (Python callables registered via
`register_macro("name", ...)` decorator) + user-authored
declarative macros (`macro NAME(PARAMS) -> TYPE { BODY }` with
Clause-IR body).

Two position registries: **expression-position**
(`MacroInvokeExpr → Expr`) and **item-position**
(`MacroInvokeItem → list[Item]` spliced into parent).

User macro execution: tree-walking Clause-IR interpreter
(`macro_interpreter.py`) against an environment binding params
to token streams. Returns a value coercible to
`MacroTokenStream`. `quote!` is special form, supports `$ident`
and `$(expr)` interpolation. Hygiene: no gensym; relies on
strict-no-shadow lint (STRICT1). Emitted bindings use
`__prefix`. Call-site span propagation: diagnostics point at
the invocation, not the body.

**Subset limitation:** user macro bodies cannot use `struct`,
`trait`, `impl`, `match`, closures, or true mutable aliasing.

Files: `macro_expansion.py` (1135L), `macro_interpreter.py`
(956L), `macro_runtime.py` (291L).

## 3. Compiler architecture

### Two parallel pipelines (the architectural tension)

**A. Clause front-end** — runs directly in `clause_cli.py
cmd_build`, NOT through the pass scheduler:

1. Lex — `grammar/clause/lexer.py` (424L), subclasses base
   `grammar/lexer.py`.
2. Parse — `grammar/clause/parser.py` (2367L). Recursive
   descent, **error-recovering** (accumulates diagnostics,
   inserts `ErrorNode` / `ErrorExpr` / `ErrorItem`, syncs to
   item-starters or `;` / `}`).
3. Resolve modules — `grammar/clause/resolver.py` (625L).
   Filesystem-walks `.cse` tree from `src/lib.cse`, builds
   `ResolvedCrate` / `ResolvedModule`, pairs `expect` /
   `actual`, records `UseDecl → ImportBinding`, flags dead
   files.
4. Expand macros — `macro_expansion.py` walk; runs user
   macros via `macro_interpreter.py`.
5. Cfg pruning — `cfg.py` drops items whose `#[cfg(...)]`
   fails against `CompileEnv`.
6. Cross-crate impls — `cross_crate.py` unions dep
   `impls_by_self_type`.
7. Typecheck — `typecheck.py` (2058L). Builds symbol table
   (fqpath keyed), method table, enforces orphan rule,
   walks bodies for pattern-exhaustiveness on enum matches
   (including `Option` / `Result`), collects patch chains,
   matches `expect` / `actual` shapes.
   **Deliberately omitted**: Hindley-Milner inference,
   generic-bound checking, monomorphization, scope-chain
   traversal validation.
8. Validate (framework) —
   `grammar/clause/validators/framework.py`. Single-walk
   visitor with per-node `enter` / `leave` +
   `finalize_scope(scope, ctx)`. Scope stack hardcoded
   (BlockExpr, FnDecl body, MatchArm). `ValidatorSink`
   dedupes on `(span.start, span.end, code)`. Separate
   checks: `shadow_check.py`, `unreachable_check.py`,
   `unused_check.py`.
9. Storage routing — `storage_router.py` +
   `storage_catalog.py` (927L combined). Walks every
   `struct X: BindTarget`, picks a Clausewitz backend per
   field from {`flag`, `variable`, `scripted_variable`,
   `event_target`, `scripted_list`, `string_storage`,
   `paired`}. Honours `#[repr]` hard override + `#[prefer]`
   soft hint. Reconciles with manifest `[storage]` lock →
   `CL_STORAGE_DRIFT`.
10. Transpile — `transpile.py` (1878L). For each `fn` body
    and each `event`, produces an `EmitPlan(item_name,
    item_kind, ClauseBlock body, source_span)`. Output is a
    **tree** (`ClauseValue` | `ClauseScalar` | `ClauseBlock`
    | `ClauseBlockItem`), decoupled from pretty-printing.
    Auto-names `<crate>_<module::path>_<fn>`. Kind
    selection: fn with return-type → `script_value`; fn
    returning unit / `Result` → `scripted_effect`; `event
    Name for Country` → `country_event`. `#[as_*]`
    attributes override.
11. Codegen — `codegen.py` (321L). Routes each `EmitPlan` to
    `<crate>/<stellaris-subdir>/<module-hint>.txt` per a
    hardcoded `_KIND_DIRECTORY` table. Detects collisions,
    emits `source_map.json` for 1-indexed line ranges back
    to `.cse` origin.
12. Manifest — `grammar/clause/manifest.py`. Processes
    `#[supersedes]` / `#[deprecated]` into
    `symbol_aliases.manifest` YAML. Marked
    `FIXME(rename-naive)` — cross-crate, chained, non-event
    kinds all unfinished.

**B. Pass scheduler** — `compiler/passes/`. Built for the
pre-Clause `.txt`-patch pipeline (the `megapatch` legacy).
Runs INDEPENDENTLY of the Clause front-end today. Five pass
kinds: `Analyzer`, `Linter`, `Optimizer`, `Writer`,
`Generator` (LLM). `PassRegistry` auto-discovers
`compiler.passes.builtin.*` + optional user_dir.

Scheduler: topological sort (`graphlib.TopologicalSorter` +
alphabetical tiebreak for determinism), SHA-256 fingerprint
over (pass_id, version, read artifacts' producer
fingerprints, config_inputs), SQLite `pass_artifacts` cache
with type-tagged JSON codec (tuples, tuple-keyed dicts,
registered IR dataclasses, `PatchError`).

12 `Artifact` kinds: PATCHES, ITEMS, BUILT_ITEMS,
COMPILE_RESULTS, RULES_MAP, LOAD_ORDER, VOCABULARY,
CALL_GRAPH, LEAF_SET, DEPTH_MAP, USAGE_MAP, GLOBAL_VARS.

14 builtin passes: `ITEM_INDEX`, `CALL_GRAPH_ANALYZER`,
`DEPTH_ANALYZER`, `LEAF_ANALYZER`, `GLOBAL_VAR_LOADER`,
`COMPILE`, `LEAF_INLINER` (optimizer), 7 linters
(`CONTENT_SYNTAX_LINT` 868L with 12+ sub-rules,
`PATCH_SHAPE_LINT` 569L with 13 sub-rules,
`UNDEFINED_REFS_LINT` 471L, three scripted-call checks,
`PROBE_ECHO` Generator stub).

### Patch ops

`compile/ops/`: 8 operations with priority order:
`override` > `replace_in_item` > `inject_field` >
`prefer_mod` > `delete_item` > `insert_item` >
`merge_union` > `no_override`.

### Diagnostics

`grammar/diagnostic.py` + `grammar/diagnostic_codes.py`
(2062L). ~110 `CL_*` codes with `Explanation(brief, detail,
example, fix, related)`. `clause explain --code CL_FOO`
serves this.

### Spec ingest

`compiler/spec_ingest/` (1214L) + 3 adapters (901L):
`harvest` runs source adapters (`paradox_docs.py`,
`cwtools.py`, `vanilla_analysis.py`) → `merger.py` →
`overlay.py` → `writer.py` to produce per-game
extern-signature tables under `tools/clause/data/<game>/`.
Feeds `extern` declarations and scope-aware identifier
resolution.

### Units

`compiler/units/`: `CompileUnit(name, patch_dirs, output_dir,
inline_script_prefix, descriptor)`. `Descriptor(name,
supported_version="4.*", tags, remote_file_id, picture)` —
emits Stellaris `descriptor.mod`.

### Clausewitz G0 sub-grammar

`compiler/grammar/clausewitz/` (1079L): lexer / parser / AST
/ tokens for raw Clausewitz script. Needed for patch-ops
reading existing vanilla / mod text, for cwtools validation,
and for depth / balance checks. `compiler/clausewitz/` (545L)
adds `balance.py`, `localisation.py`, `vars.py`, thin parser
wrapper. **Two Clausewitz parsers coexist** — a consolidation
debt.

## 4. Emission targets

**Primary**: plaintext Clausewitz script files under
`<out>/<crate>/<subdir>/<filename>.txt`. Subdir routing table
in `codegen.py`:

- `scripted_effects/auto/`
- `scripted_triggers/auto/`
- `script_values/auto/`
- `inline_scripts/auto/`
- `events/auto/`
- `on_actions/auto/`
- `scripted_variables/auto/`

Plus a `.mod` descriptor per unit. Plus `symbol_aliases.
manifest`, `renames.json`, `source_map.json` sidecars.

**Secondary**: cwtools validator output
(`validate/cwtools_runner.py`), doc markdown
(`docgen.py`), feature reports (JSON), storage lock back into
`Clause.toml`.

**No interpreter / `run` subcommand.** `clause test` is
compile-only (test passes = body compiles without errors).
Runtime semantics live in the Paradox engine.

### Non-compiler siblings

- `artgen/` — 11 shell scripts + 1 Python helper for
  invoking external AI image generation. Dispatched via
  `clause art <sub>`. Orthogonal to the compiler pipeline.
- `playset/` — multi-mod orchestration: modlist discovery,
  workshop scan, load-order compute (evidence-weighted
  graph), item extraction. Has its own pass surface.
- `deploy/` — local copy or SSH rsync of `.dist/` to
  Stellaris mod directory or remote host.

## 5. CLI surface

`clause_cli.py` (2126L) — single `clause` command with
subcommands: `check`, `build`, `compile-crate`,
`build-playset`, `new`, `explain` (with `--code CL_FOO`,
`--list-codes`, or positional symbol explanation), `test`,
`doc`, `art`, `gen <family> --llm {offline,gemini}`, `doctor`
(8 probes), `playset` passthrough, `deploy` passthrough,
`spec {harvest, search, show, scope, list}`, `features
[--json]`.

Plus `compiler/__main__.py` extras: `build`, `stats`, `cache`.

Common build flags: `--out .dist`, `--dlc X` (repeatable),
`--mod X`, `--feature X`, `--verbose`, `--no-storage-lock`.

### Config

`Clause.toml` workspace + package manifest: `edition="2026"`,
`crate_type ∈ {foundation, shared, content, toolchain}`,
deps with `from_workspace=true` / `path=` / `version=`,
`[dlc]`, `[features]`, `[[rename]]`, `[storage]` lock
section, `AutoDiscover` rules. `.env.local` for env
overrides.

## 6. Status, gaps, known-deferred

58+ TODO / FIXME / deferred markers across 33 files. Salient:

- Typecheck: no Hindley-Milner, no bound checking, no
  monomorphization, no scope-chain traversal validation.
- Transpile: `for` / `while` / `loop` / closures / complex
  patterns emit placeholder comment + diagnostic, not real
  code. Type-aware dispatch (`self.field` → `set_variable`,
  scope-chain traversals) deferred.
- Codegen: asset decorators (`#[gfx_sprite_type]`,
  `#[localisation]`, `#[icon]`) not wired.
- Macros: item-position and statement-position invocations
  partial; user macro bodies can't use struct / trait /
  impl / match / closures.
- Manifest: no git-diff auto-rename-detect; sham handler
  body gen deferred; only event-kind shams.
- Parser: tuple exprs, struct literals as expressions,
  lifetimes, async / await / unsafe blocks — all
  deliberately omitted.
- CLI: `clause fmt` absent; `clause test` runtime stub;
  `clause doc` stub-level; `clause spec` read-only.
- Clause front-end does NOT use the pass scheduler — runs
  phases directly in `cmd_build`.
- `clause gen` LLM integration is stubbed.
- Rename system marked `FIXME(rename-naive)`.

### Test coverage

1511 test functions across 99 files. Unit tests pair with
most modules 1:1 (`test_grammar_clause_parser.py`,
`test_storage_router.py`, …). Integration tests cover every
CLI subcommand. Golden fixtures in `tests/golden/outputs/`
are thin (only `override_plus_replace.txt` visible).

Heavy coverage: grammar, typecheck, storage, passes.
Light coverage: macros (two files), spec_ingest adapters.

## 7. Downfalls and architectural tensions

1. **Two superimposed pipelines.** Pass scheduler was built
   for pre-Clause `.txt`-patch orchestration (megapatch
   legacy). The new Clause front-end runs *outside* the
   scheduler, bypassing caching entirely. Every build
   re-does lex / parse / resolve / typecheck from scratch.
   `BuildSummary` in `clause_cli.py` admits cache
   hit / miss surfaces as "(n/a)".
2. **AST abuses `field(default_factory=lambda: None)` +
   `type: ignore[assignment]`** — dozens of nodes use this
   because Python dataclasses can't express "required but
   forward-referenced". Fighting Python, and the
   `Optional`-in-disguise hurts readability.
3. **Artifact JSON codec** (`passes/artifacts.py`) — custom
   tuple / dataclass type tags (`\x00dc`, `\x00tuple`,
   `\x00tupdict`, `\x00patcherr`) because stdlib json can't
   round-trip tuple-keyed dicts or frozen dataclasses.
   `serde` derive collapses this entirely.
4. **Macro interpreter is a separate tree-walker**, not the
   same walker the typechecker uses. Two interpretations of
   the AST diverge over time.
5. **Clausewitz sub-grammar duplicates** —
   `grammar/clausewitz/` (1079L, clean G0 subclass) and
   `compiler/clausewitz/` (545L, legacy hand-rolled) both
   parse Clausewitz. Consolidation debt.
6. **Pass discovery via `inspect` + `pkgutil`** —
   runtime-registration. Per clause-dev lints this is
   forbidden (`no-runtime-registration`). Rust port must
   use declarative inventory.
7. **SQLite single-file cache**, WAL-mode covered,
   "writes serialized through the parent process" — soft
   bottleneck for large workspaces.
8. **106 `CL_*` codes across a 2062-line registry** with
   free-form prose — needs structure (category, severity
   tier, fix-suggestion machine-readability).
9. **Reference types parsed then erased** (`RefType`,
   `RefExpr`, `RefPat`) — author-facing sugar with zero
   enforcement. Port must decide: implement borrow-ish
   semantics, or reject the syntax.
10. **`Option<T>` / `Result<T, E>` / `Vec<T>` are recognised
    as special generics** in `resolved_types.py`
    (hardcoded name-matches). Per clause-dev doctrine these
    should be `Maybe` / `Outcome` / no-Vec — the *source
    language* itself carries the baggage here, not just the
    compiler.

## 8. Rust-port guidance summary

Deep guidance lives in `lessons-learned.md` and
`parity-plan.md`. Summary:

- **Must-replicate features** (≥45): full grammar + parser,
  error recovery, module resolver, `expect` / `actual`,
  orphan + sealed, cross-crate impl union, typecheck
  (symbol / method tables + pattern exhaustiveness + patch
  chains + `Option` / `Result`), `#[cfg]` evaluator,
  macro system (expression + item positions + built-ins +
  user macros + hygiene), storage router (7 backends +
  override + hint + lock drift), transpile tree + auto-name
  + kind heuristics + attr override, codegen routing +
  collision detect + source maps, diagnostics registry
  (~110 codes), manifest format, CLI (14 subcommands),
  pass scheduler (typed artifacts + sha fingerprint +
  cache + 5 pass kinds), shadow / unused / unreachable,
  validator framework, spec-ingest adapters + merger,
  rename shims, build modes (release staging + atomic
  replace + debug direct), playset orchestration +
  evidence-weighted load order, doctor probes, deploy,
  Clausewitz G0 parser, patch ops.

- **Decisions worth inheriting** (≥10): error-recovering
  parser; emission-as-tree; routing table keyed on
  `item_kind`; typed `Artifact` enum; SHA-over-inputs
  fingerprinting; structured diagnostic codes; 7-backend
  storage catalog; `expect` / `actual`; attribute-driven
  kind selection; `CompileEnv` as first-class
  conditional-compilation driver; source-map sidecars;
  call-site span propagation; validator framework
  single-walk + scope stack + dedup sink; pass-kind
  distinction (Analyzer / Linter / Optimizer / Writer /
  Generator).

- **Decisions worth rejecting** (≥10): `Option` / `Result`
  / `Vec` as language primitives; reference types parsed
  then erased; `&str` / `String` / `u8..u128` as language
  primitives; two-parallel-pipeline split; runtime pass
  discovery; hand-rolled JSON codec for
  tuples / dataclasses; dataclass defaults abuse; two
  Clausewitz parsers; macro interpreter divergent from
  typecheck; free-form prose-only diagnostic registry;
  reserved-but-unused keywords; Generator coupled to LLM
  retry; SQLite-as-primary-cache for the pass scheduler;
  asset decorators designed-but-unwired.

- **Strictly better in Rust** (≥5): parser error recovery
  via `logos` + hand-rolled recursive descent with typed
  `Span`; pass fingerprinting with `bincode` + `blake3`;
  arena-allocated immutable AST; shared
  MIR / macro-interpreter; typed storage-catalog traits;
  `rkyv` zero-copy for cross-crate artifacts; diagnostics
  via `ariadne` or `annotate-snippets`.

## 9. Natural port ordering

Matches Python dependency order; cross-cuts flagged:

L0 (done) — `clause-lex`, `clause-syntax` skeleton.
L1 (partial) — `clause-ir`, `clause-resolve`,
`clause-typecheck`, `clause-codegen` shapes.
L2 — `clause-cfg` + `clause-macros` (cross-cuts
resolve + typecheck; design interpreter first, share IR
with typecheck).
L3 — `clause-storage` (catalog + router; depends on
typecheck's resolved-types).
L4 — `clause-transpile` + `clause-codegen-emit` (depends
on storage + typecheck).
L5 — `clause-manifest` + `clause-rename`.
L6 — `clause-passes` framework (probably lives in
hilavitkutin; typed artifacts + cache layer).
L7 — `clause-validators`.
L8 — `clause-spec-ingest`.
L9 — `clause-cli`.
L10 — Clausewitz G0 grammar (needed earlier for patch-ops
but can be stub-first).

### Crate-scope expansions likely needed

- `clause-syntax` must cover the full token + AST surface
  from `grammar/clause/`.
- `clause-macros` is its own crate (proc-macro +
  interpreter); Python merges them, Rust should separate.
- `clause-storage` + `clause-transpile` are not trivial
  bolt-ons; each ~1800–2500 equivalent Rust LOC.
- `clause-passes` needs typed artifact machinery + cache;
  probably belongs in `hilavitkutin` (pipeline engine)
  rather than `clause/`.
- `clause-playset`, `clause-deploy`, `clause-doctor`,
  `clause-spec` are discrete side-crates; scope each
  independently, not as subcommands of a monolith.

## 10. Key paths for follow-up

Grammar:
- `compiler/grammar/clause/{tokens.py, ast.py, parser.py,
  lexer.py, resolver.py, typecheck.py, transpile.py,
  codegen.py, cfg.py, storage_router.py, storage_catalog.py,
  macro_expansion.py, macro_interpreter.py, manifest.py}`

Pass framework:
- `compiler/passes/{base.py, scheduler.py, artifacts.py,
  registry.py, cache.py, subgraph.py}`

Builtin passes:
- `compiler/passes/builtin/{compile_pass.py, linters/*,
  optimizers/leaf_inliner.py}`

CLI:
- `compiler/clause_cli.py` + `compiler/__main__.py`

Manifest:
- `compiler/manifest.py`

Diagnostics:
- `compiler/grammar/diagnostic_codes.py`

Validator framework:
- `compiler/grammar/clause/validators/framework.py`

Playset:
- `playset/{orchestrate.py, data.py, loadorder/compute.py,
  ingest/*}`

Tests (feature oracle):
- `compiler/tests/{unit, integration}/` — 1511 test
  functions across 99 files.

---

All paths are rooted at
`/Users/orgrinrt/Dev/stellar-heritage/tools/clause/`.
