# Clause Language Audit — 2026-04

Gap register between `DESIGN.md` / `GRAMMAR.md` (normative) and the
shipped compiler under `tools/clause/compiler/`. Each section below
classifies concrete design claims against the current implementation.

**State legend**

| State | Meaning |
|-------|---------|
| **LIVE** | Design matches shipped code; has tests. |
| **PARTIAL** | Exists but incomplete / placeholder; enforcement or lowering missing. |
| **STUB** | Commented-out or `NotImplementedError`-equivalent path. |
| **MISSING** | No code path at all. |
| **DRIFT** | Shipped code contradicts the design — a red flag. |

Tasks referenced here live in the task list. When a gap has no
existing task, a NEW entry is recorded and must be filed.

This document is living — each overnight review-rotation pass folds
new findings in. Date-stamp changes; don't rewrite history.

---

## §8 Structs

### §8.1 Field declarations  **LIVE**
- Design: `StructDecl { fields: tuple[StructField] }` with optional `mut` / `const` modifiers, optional default.
- Impl: `ast.py` StructField + `parser._parse_struct_field`.
- Tests: `test_grammar_clause_parser.StructTests`.

### §8.2 Bind targets (the colon) **LIVE**
- Design: `struct X : Target` — Target must impl `Bind`.
- Impl: parser handles `:`, typecheck validates bind target via cross-crate impl table.
- Tests: `test_grammar_clause_parser.StructTests.test_struct_with_scope_bind_target` etc. + `test_std_bind`.

### §8.3 Struct literals (`MyStruct { field: value }`) **LIVE** (as of #118)
- Design: constructable anonymous-value form with shorthand + update-base.
- Impl: `StructLiteralExpr`, parser disambiguation against block bodies, typecheck validation.
- Tests: `test_grammar_clause_parser.StructLiteralTests` + `test_grammar_clause_typecheck.StructLiteralTypecheckTests`.

### §8.4 Strings via StringStorage **PARTIAL**
- Design: field with `String`-like type gets implicit StringStorage backend.
- Impl: storage router has STRING_STORAGE branch that emits a `CL_STORAGE_STRING_UNIMPLEMENTED` warning (#108). Actual runtime-mutable string hack is deferred.
- Task: **#111** STD-STR1 stdlib strings.

---

## §9 Traits

### §9.1 Method receiver **LIVE** (as of #113)
- Design: `self`, `mut self`, `&self`, `&mut self`.
- Impl: `_parse_fn_param` accepts all four; `&self`/`&mut self` erase to `self`/`mut self` at AST level.
- Tests: `test_grammar_clause_refs.RefParamTests`.

### §9.2 Associated types and constants **LIVE** (as of #106 + #162)
- **Types**: LIVE as of #106 — `type Name;` / `type Name = T;` both parse; typecheck validates placement + impl completeness + unknown/duplicate slots.
- **Constants**: LIVE as of #162 — `const N: T;` and `const N: T = v;` both parse; typecheck validates placement + impl completeness + unknown/duplicate slots; cross-crate impls of dep traits validated through the impl table.

### §9.3 Supertraits **LIVE** (as of #163)
- Design: `trait Sub : Super { … }`.
- Impl: `_parse_trait_decl` parses the `: Super + Super2` supertrait list after the generic params. Typecheck validates that every impl of Sub has matching impls of each supertrait for the same self-type; cross-crate supertrait map flows through `CrossCrateImplTable`.

### §9.4 Purity + function decomposition **MISSING**
- Design: replacement for kind tracking; fn bodies classified as pure/effect/trigger/value per rule §9.4.
- Impl: typecheck module docstring mentions "Kind inference — trigger / effect / value / inline_script per DESIGN §9.4" as deferred.
- Task: **#120** T2 kind inference.

### §9.5 Sealed traits **LIVE** (as of #105)
- Design: private-supertrait pattern; only declaring crate may impl.
- Impl: `sealed` keyword + `_check_sealed_coherence` + cross-crate sealed-trait tracking.
- Tests: `test_sealed_enforcement`.

### §9.6 No `dyn Trait` **LIVE**
- Design: `dyn` reserved but never accepted.
- Impl: `KW_DYN` exists as reserved keyword; no parser path.

---

## §10 Impls

### §10.1 Coherence / orphan rule **LIVE** (as of #123)
- Design: trait OR self-type must live in the current crate.
- Impl: `_check_orphan_rule` walks ImplBlock AST directly so empty-body impls are covered. The previously xfailed test `test_clause_check.CheckOrphanRuleTests` is now a real passing assertion.
- Tests: `test_clause_check.CheckOrphanRuleTests.*`.

### §10.2 Method body conventions **LIVE**
- Design: method body expressions / statements as in §18.
- Impl: fn bodies walk through typecheck + transpile.

### §10.3 References — erased ergonomic sugar **LIVE** (design reconciled 2026-04-20)
- Design: references are accepted in every position (`&T`, `&mut T`, `&expr`, `&mut expr`, `&self`, `&mut self`) and erase to their pointee at every downstream layer.
- Impl: #113 ships parsing + erasure in typecheck + transpile.
- Tests: `test_grammar_clause_refs.*`.
- DESIGN.md §10.3 rewritten to reflect shipped reality (drift fixed 2026-04-20).

### §10.4 `Self` (capital-S) **LIVE**
- Design: refers to the impl's self-type.
- Impl: `KW_SELF_TYPE` handled by path parser.

---

## §11 Generics

### §11.1 Generic functions and impls **LIVE at explicit sites** (as of 4 × #160 slices)
- Design: `fn f<T: Bind>(x: T)` with bounds + `where` clauses.
- Impl: parse + AST retention LIVE as of #115. Four enforcement slices shipped:
  1. Turbofish call sites (`foo::<T>(x)`) — b13a666e.
  2. Struct-literal turbofish (`Registry::<T> { ... }`) — ea3670d7.
  3. Detached `where T: Bound` alongside inline bounds — 341882fa.
  4. Impl-block trait generic args (`impl Iter<T> for …`) — 6a805efb.
- All route through `_check_generic_args_bounds(path, span)` + `_effective_bounds_for_param(item, i)` in `typecheck.py`.
- Tests: `test_grammar_clause_typecheck.GenericBoundEnforcementTests` (12 cases).
- **Still open**: inferred-generic call sites (blocked on #119 T1 inference), projection-subject where predicates (`where T::Entity: Bound`), monomorphization hookup.

### §11.2 Const generics **PARTIAL** (as of #164; enforcement deferred)
- Design: `struct Array<T, const N: int> {}`.
- Impl: `_parse_generic_params_full` accepts `const N: Type` alongside type parameters; AST records the const type. Enforcement at instantiation sites (verifying the argument is a compile-time constant of the declared type) waits for T3 monomorphization (#121).
- Task: **#121** T3 monomorphization.
- Tests: `test_grammar_clause_parser.ConstGenericParamTests`.

### §11.3 No higher-kinded types **LIVE**
- Design: not supported, reserved.
- Impl: no HKT path.

---

## §12 Option / Result / no nulls

### §12 Option + Result **PARTIAL**
- Design: full type + methods.
- Impl: `crates/std/src/option.cse` and `result.cse` exist but are thin stubs; no `map/and_then/unwrap_or` methods beyond the stubs.
- Tasks: **#130** STD-OPT, **#131** STD-RES.

### §12.1 Codegen for Option / Result **PARTIAL**
- Design: per-type manifest ABI.
- Impl: storage router handles `Option<T>` via paired-encoding (#108 paired backend). `Result<T, E>` codegen is MISSING.
- Task: NEW — `[X-OPT] Option<T> / Result<T,E> codegen arms alongside storage router`.

### §12.2 Pattern matching **PARTIAL**
- Design: `match` on Option/Result with exhaustive arms.
- Impl: `_check_match_exhaustiveness` handles enum variants including Option/Result. Extended patterns (#114) now parse *and* validate shape: `_validate_pattern` in `typecheck.py` emits `CL_REST_PAT_POSITION`, `CL_RANGE_PAT_EMPTY`, `CL_RANGE_PAT_TYPE`, `CL_OR_PAT_ARITY` (fbe8cfca). Exhaustiveness extension for tuple / range / or-pattern coverage still open.
- Task: **#161** G6b exhaustiveness extension; transpile lowering split to **#166** G6c (blocked on #125).

### §12.3 The `?` operator **PARTIAL** (typecheck LIVE as of cef2f4ee)
- Design: `expr?` — early-return propagation.
- Impl: `QuestionExpr` parses; `_check_question_expr` validates the enclosing fn's return type is `Result<…>` or `Option<…>` (peeling `RefType` wrappers); emits `CL_QUESTION_IN_NON_RESULT` otherwise. Desugaring to a match over Err/None is MISSING — blocked on #125 match lowering.
- Task: **#156** G11 `?` operator desugaring (desugar slice remains).

---

## §14 Stdlib

### §14.0 Bind targets (`Bind` sealed trait) **LIVE**
- Design: `pub sealed trait Bind {}` + scope target impls (Country, Pop, Planet, …) + Registry<T> + Singleton.
- Impl: `crates/std/src/bind.cse` ships the sealed trait (post #105) + 10 target impls; cross-crate validation via `_check_bind_targets`.
- Tests: `test_std_bind` + `test_sealed_enforcement.StdBindSealedTests`.

### §14.0.1 StorageRouter **LIVE**
- Design: per-field optimal backend picking (FLAG, VARIABLE, SCRIPTED_VARIABLE, EVENT_TARGET, SCRIPTED_LIST, STRING_STORAGE, PAIRED).
- Impl: `storage_router.py` + transpile arms (#57 + #108).
- Tests: `test_storage_router`, `test_storage_emit`.

### §14.X Other stdlib modules **STUB**
- `std::iter`, `std::collections`, `std::temporal`, `std::events`, `std::scratch`, `std::numeric`, `std::prelude` all exist as near-empty crates with placeholder comments.
- Tasks: **#132**–**#138** per module.

---

## §15 Events

### §15 Event bodies **PARTIAL**
- Design: `event Name for Scope { body }` — body transpiles to `country_event`/`planet_event`/etc.
- Impl: parser + basic transpile wiring in place; event-specific primitive lowering (diplomatic actions, mean_time_to_happen, etc.) is DEFERRED.
- Task: **#129** X6 event body lowering.

---

## §18 Body code conventions

### §18 Expressions + statements **PARTIAL**
- Design: Rust-ish expression grammar; body transpiles to Clausewitz effect blocks.
- Impl: parser covers most shapes (literals, paths, calls, field access, method calls, unary/binary, ranges, closures-parse, match, if/for/while/loop, let/return/break/continue, struct literals, tuples, refs, `?`). Transpile lowers most of this to placeholder comments.
- Tasks: **#124**–**#128** X1–X5 transpile work.

### §18 Closures **PARTIAL**
- Design: `|x| x + 1` as inline script_value; capturing closures extract to named items.
- Impl: `ClosureExpr` parses; transpile emits `"|…| …"` placeholder.
- Task: **#116** G8 closure lowering.

---

## §16 Modules + resolver

### §16.1 Module declarations **LIVE**
- Design: `mod foo;` file-routed, `mod foo { }` inline.
- Impl: `ModuleResolver` handles both.
- Tests: `test_grammar_clause_resolver`.

### §16.2 `use` declarations **LIVE at resolved sites** (as of 2 × #151 slices)
- Design: `use path::Item`, `use path::{A, B}`, `use path::*`, `use path::Item as Alias`.
- Impl:
  - Grouped / aliased / relative (`super`, `crate`, `self`) all LIVE in resolver.
  - **Visibility enforcement** (7787ed65): `_check_import_visibility` validates `use` targets against declared visibility, emits `CL_VISIBILITY_VIOLATION`. Handles `pub` / `pub(crate)` / `pub(super)` / private.
  - **Glob expansion** (2ce82307): `_resolve_path_ref` walks `glob=True` bindings and matches `foo::segs[0]` for each `use foo::*`; respects visibility.
- Tests: `ImportVisibilityTests` + `GlobImportTests`.
- **Still open**: re-export chain following (`pub use A::X` flowing through multi-crate), cross-crate visibility (needs dep symbol info), circular-import detection.
- Task: **#151** R1 resolver edges (in progress).

---

## §27 Tooling / CLI

### §27.x `clause explain` + diagnostic registry **LIVE** (as of #165)
- Design: `rustc --explain`-style prose lookups for every compiler-emitted code.
- Impl: `diagnostic_codes.py` registry holds 181 entries — every code emitted by `grammar/**/*.py` is registered. Two acceptance tests (`test_every_emitted_code_is_registered`, `test_no_stale_registry_entries`) enforce bidirectional sync. `clause explain --code CODE` prints brief + detail + example + fix + related. `clause explain --list-codes` dumps the full set.
- Tests: `test_diagnostic_codes` (13 cases).

---

## Other gaps surfaced while auditing

### BindPat (`x @ pat`) **MISSING**
- Design: implied by §12.2 but not explicit.
- Impl: AST has `BindPat` stub (from #114) but no parser path because `@` isn't a tokenised sigil.
- Decision needed: ship the `AT` token + parser, or leave BindPat dormant.

### Validation-layer gaps (2026-04-19 pass)

User-led audit of clippy-grade correctness checks — tracked state + newly filed tasks:

| Check | State | Task |
|---|---|---|
| Shadowing is an error | LIVE | `CL_SHADOW` (#100) |
| Visibility violation on use / path | LIVE | `CL_VISIBILITY_VIOLATION` (#151) |
| Type mismatch in expression (str→int, arg/ret) | MISSING | #119 T1 HM inference |
| Ambiguous reassignment w/o clear type | MISSING | #119 |
| Scope-chain field access | MISSING | #122 T4 |
| Kind inference | MISSING | #120 T2 |
| **Assigning to non-mut** | MISSING | **#168 STRICT2** (new) |
| **Integer literal overflow** | MISSING | **#169 STRICT3** (new) |
| **Use-after-move (lightweight)** | MISSING | **#170 STRICT4** (new) |
| **Ambiguous use-import collision** | MISSING | **#171 STRICT5** (new) |
| **Unresolved path silent UnknownType** | MISSING | **#172 STRICT6** (new) |
| **Clippy-grade batch (match arms, unused mut, unused generic, redundant bound, attr-wrong-item, div-by-zero, missing return, refutable let, non-const in const)** | MISSING | **#173 STRICT7** (new) |
| Unused bindings / imports / unreachable code | LIVE | #100 |
| Orphan / sealed / supertrait / bind-target / generic-bound / struct-literal / match-exhaustiveness / `?` / assoc-type+const / method-arity / duplicate-symbols / pattern-validity | LIVE | various |

---

## Summary (2026-04-20 snapshot, refreshed 2026-04-19 overnight)

| State | Count |
|-------|-------|
| LIVE | 19 |
| PARTIAL | 6 |
| STUB | 1 (stdlib module family) |
| MISSING | 2 (BindPat sigil; kind inference) |
| DRIFT | 0 |

**Delta since last refresh** (2026-04-19 overnight session):
- §11.1 generic bounds: PARTIAL → **LIVE at explicit sites** (four enforcement slices via #160).
- §12.2 pattern matching: shape-validity checks land (still PARTIAL overall — exhaustiveness extension open).
- §12.3 `?` operator: typecheck validation now LIVE (desugar remains blocked on #125).
- §16.2 `use` declarations: **LIVE** for visibility + glob expansion (two #151 slices).
- §27.x `clause explain`: **LIVE** registry covering every emitted code (#165).

**Still open MISSING-row entries** either have filed tasks (#119 T1, #120 T2, #122 T4, #168–#173 STRICT2–7) or "decision needed" markers (BindPat sigil).

The audit is **not complete** — sections 5, 6, 13, 17–20, 22–26, 28–34 still need dedicated passes.

**Next targets for audit deepening** (in priority order):
1. §22 File routing — storage router and codegen file placement rules.
2. §21 Decorators — every `#[attr]` should be listed with its runtime semantics or flagged MISSING.
3. §25 Bootstrap (stellaris-vanilla-spec crate ingest) — frozen at 3/7 sources per #58.
4. §30 Build cache / incremental rebuild — line-level pointers for #157 BLD1 fuzz.
