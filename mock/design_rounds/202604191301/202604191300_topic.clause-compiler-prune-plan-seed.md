# Compiler Prune Plan

**Task:** #81 CAUDIT
**Status:** Draft — awaiting user ACK before execution
**Audience:** future-self + any subagent executing the prune

This plan enumerates every file under `tools/clause/compiler/` and its
fate in the post-pivot Clause-first repo. Anything playset-,
modlist-, workshop-, or deploy-flavoured leaves the compiler and
moves to a sibling `clause playset` subcommand package (task #82).
Anything megapatch-era YAML-patch plumbing is removed. Anything
genuinely generic and game-agnostic stays.

## Classification policy

Four fates, strict order of preference:

1. **KEEP** — Clause-native compilation, generic Clausewitz script
   analysis (lexer/parser/AST/linters used by vanilla+mod API
   introspection), or game-agnostic framework (pass scheduler,
   artifact registry, cache, fingerprint, manifest parser, errors,
   utils). These stay at `tools/clause/compiler/`.
2. **ABSTRACT** — the capability is valuable but the implementation
   currently hardcodes megapatch / Stellaris conventions. The module
   stays; the hardcoded defaults come out, replaced by config or
   per-game rules under `tools/clause/data/`.
3. **MOVE → playset** — the module is orchestrating a whole playset
   (modlist parsing, steamcmd download, workshop scanning, load-order
   computation across sources, descriptor generation, deploy to the
   Stellaris mod dir). Moves to a new package
   `tools/clause/playset/` exposed via `clause playset`.
4. **DROP** — megapatch-era YAML-patch compilation, flatten-writer
   corpus emission, `new_patch` scaffolding, or passes that only
   operate on megapatch artifact shapes. Delete outright; if a
   capability is wanted later under Clause semantics, rewrite it
   cleanly rather than port megapatch code.

**Stricter than "technically generic":** a module may look generic in
isolation but if its only caller graph is megapatch-era orchestration,
keeping the code alive means carrying dead weight. In that case:
DROP, and rewrite against Clause IR when the need arises.

## Classification table

### Top-level

| path | loc | fate | reason |
|------|-----|------|--------|
| `__init__.py` | 3 | KEEP | package marker |
| `__main__.py` | 1130 | ABSTRACT | `megapatch` CLI — split: keep build/cache/validate here, move modlist/loadorder/workshop/deploy to `clause playset` |
| `clause_cli.py` | 469 | KEEP | Clause language CLI (check/build/compile-crate/new) — multi-game neutral |
| `build.py` | 455 | ABSTRACT | release/debug gate + linter phase scheduler. Release gate + policy stay; megapatch-specific unit-registration (heritage/megapatch_fios) comes out |
| `cache.py` | 554 | KEEP | SQLite cache + artifact codec — generic incremental build |
| `config.py` | 264 | ABSTRACT | drop megapatch names (CUSTOM_MOD_HEAD/TAIL, heritage_* dirs, `rules-fios-lios.txt`); keep paths + loader; replace with per-game rules lookup |
| `data_import.py` | 271 | MOVE → playset | imports mods.jsonl + collections.jsonl + modlists.jsonl — playset data, not compiler data |
| `errors.py` | 55 | KEEP | generic diagnostic types |
| `extract.py` | 251 | KEEP | item content extraction + @variable inlining — generic IR source |
| `fingerprint.py` | 52 | KEEP | generic incremental-build fingerprints |
| `index_items.py` | 72 | KEEP | patch-item union builder — generic IR assembly |
| `manifest.py` | 782 | KEEP | Clause.toml workspace + package parser (W1) — multi-game neutral |
| `model.py` | 172 | KEEP | IR dataclasses — generic |
| `parallel.py` | 131 | ABSTRACT | parallel worker pool infra. Keep; decouple from pipeline-specific task shapes |
| `pipeline.py` | 261 | MOVE → playset | full run orchestration (modlist → extract → ingest patches → ingest items → compile). `clause build` invokes scheduler directly; playset orchestration handles the broader flow |
| `rules.py` | 75 | ABSTRACT | FIOS/LIOS/MERGE dispatch. Keep dispatch; replace hardcoded filename with `Config.game_rules_path` resolved from `tools/clause/data/<game>.rules` |
| `writer.py` | 284 | ABSTRACT | CompileResult file emitter — generic; trim megapatch-specific descriptor boilerplate |

### `grammar/` — KEEP entirely (16 files, ~5300 loc)

Every file under `grammar/` stays as-is:

- `grammar/{__init__,ast,diagnostic,lexer,parser,span,stream,token,trivia}.py` — shared G0 framework
- `grammar/clause/*` (10 files) — Clause language compiler: lexer, tokens, parser, AST, manifest, resolver, typecheck, cfg, transpile, codegen
- `grammar/clausewitz/*` (5 files) — Clausewitz script lexer + parser + AST used for vanilla/mod API introspection

Rationale: this is the Clause-native core + the generic Clausewitz
analysis infrastructure the user explicitly flagged as "immensely
needed still". No megapatch coupling.

### `clausewitz/` — KEEP entirely (5 files, ~550 loc)

- `balance.py` — brace-balance validator
- `localisation.py` — .yml localisation parser
- `parser.py` — structural Clausewitz parser atop G2
- `vars.py` — file-scoped `@variable` scan
- `__init__.py`

Rationale: generic Clausewitz introspection, no game-specific knowledge.

### `compile/` — KEEP entirely (12 files, ~1000 loc)

- `compile/item.py` — per-item compile orchestration
- `compile/ops/{override,no_override,prefer_mod,replace_in_item,inject_field,insert_item,delete_item,merge_union}.py` — patch-op mutators

Rationale: patch-op semantics are generic IR mutations. Whether the
patches come from YAML (deprecated) or Clause attributes (future) is
the loader's concern, not the op's. These ops apply to IR items
regardless of source syntax.

### `linters/` — KEEP entirely (14 files, ~3400 loc)

- `linters/{__init__,base,baseline,ownership,policy,registry,vocabulary}.py` — framework
- `linters/builtin/{content_syntax,cross_references,patch_shape,undefined_refs}.py` — generic Clausewitz lints

Rationale: the lint framework is explicitly generic, and the builtin
lints check generic script properties (brace depth, quote balance,
undefined references). Ownership resolution reads a `first_party_mods`
frozenset off Config — which becomes generic once `config.py` is
abstracted. No changes needed here once Config ships.

### `loadorder/` — MOVE → playset (9 files, ~960 loc)

All of `loadorder/` moves:

- `loadorder/{__init__,compute}.py`
- `loadorder/evidence/{__init__,ingest}.py`
- `loadorder/evidence/parser_{community_list,descriptor,modlist_order,mods_jsonl,rules_constraints,rules_dependencies,workshop_pages}.py`

Rationale: load-order is **live right now**, not dormant. Even with
only our own crates in the playset, we ship multiple mod dirs
(one per crate) and those need deterministic ordering — FIOS crates
first, LIOS crates last, everything else topologically sorted from
`Clause.toml` dependencies. Third-party mods (e.g. shipset mods)
return as the project matures and feed additional weighted evidence
into the same engine. The whole `loadorder/` package moves intact
to `tools/clause/playset/` so `clause playset build` can compute
and apply the order as a regular build step.

### `ingest/` — split

| path | loc | fate | reason |
|------|-----|------|--------|
| `ingest/__init__.py` | 7 | MOVE → playset | re-exports move with contents |
| `ingest/discover.py` | 279 | MOVE → playset | workshop-dir scanning + file/id conflict inventory |
| `ingest/download.py` | 346 | MOVE → playset | steamcmd driver |
| `ingest/extract_items.py` | 213 | MOVE → playset | walks workshop tree indexing items — playset concern |
| `ingest/migrate_patches.py` | 571 | DROP | one-shot legacy YAML→item migration; migration is long done |
| `ingest/modlist.py` | 203 | MOVE → playset | modlist materialization |
| `ingest/parse_patches.py` | 439 | DROP | YAML patches → SQLite; Clause crates do not emit YAML patches |

### `patches/` — DROP

| path | loc | fate | reason |
|------|-----|------|--------|
| `patches/__init__.py` | 1 | DROP | empty; delete |
| `patches/items.py` | 86 | DROP | patch-item union builder reading the megapatch patches table |
| `patches/loader.py` | 88 | DROP | SQLite patches-table reader; zero live use outside YAML era |

Note: the *patch-op* semantics live under `compile/ops/` and stay.
What dies here is the ingestion of a "patches" table — a concept
specific to megapatch YAML and unused by Clause.

### `macros/` — DROP (8 files, ~830 loc)

- `macros/{__init__,registry,template}.py`
- `macros/builtin/{__init__,constant,for_slots,tier_levels,registry}.py`

Rationale: YAML-era `{{ macro }}` templates. Current impl is tied to
megapatch YAML patch shape and no live caller remains.

**Future macros are a real design space, not a closed door.** We
*will* want pre-compile macro expansion (e.g. for generating N
shipset entries from one declaration, tier-level family expansion,
or procedural civic scaffolds). Those return as a Clause language
feature — macro_rules!-style syntax over the Clause AST, executed in
a dedicated `MacroExpansionPass` before parsing/type-checking.
Task #39 G4a already tracks this. When that feature lands, its
package lives under `grammar/clause/macros/` (Clause-native),
not `compiler/macros/` (YAML-era).

### `flatten/` — DROP (2 files, ~246 loc)

- `flatten/__init__.py`
- `flatten/writer.py`

Rationale: bundle-mode overlay writer for the megapatch corpus; the
Clause-first pipeline emits mod directories per crate, not bundle
overlays.

### `deploy/` — MOVE → **own top-level subcommand** (3 files, ~278 loc)

- `deploy/{__init__,deploy,playset}.py` → `tools/clause/deploy/`

Rationale: deployment is its own concern, not compiler and not
playset. It writes built artifacts to a target (local Stellaris
mod dir, Steam Deck over SSH, generic export dir). The surface
shape is `clause deploy [--target local|deck|...]`, invoked after
`clause build` or `clause playset build`. Package moves verbatim
to `tools/clause/deploy/`; the internal module that generates
Paradox launcher playset entries stays in that package since it
only runs at deploy time.

### `passes/` — framework KEEP, builtin mixed

Framework (all KEEP):
- `passes/{__init__,artifacts,base,cache,registry,scheduler}.py`

Builtin passes:

| path | loc | fate | reason |
|------|-----|------|--------|
| `passes/builtin/__init__.py` | 12 | KEEP | discovery |
| `passes/builtin/call_graph_analyzer.py` | 156 | KEEP | generic Clausewitz analysis (scripted trigger/effect graph) |
| `passes/builtin/compile_pass.py` | 111 | KEEP | per-item compile executor — generic |
| `passes/builtin/depth_analyzer.py` | 199 | KEEP | call-chain recursion + brace depth — generic |
| `passes/builtin/flatten_writer.py` | 55 | DROP | WriteFlattenPass — megapatch bundle mode |
| `passes/builtin/global_var_loader.py` | 128 | KEEP | scripted-variables global resolver — generic |
| `passes/builtin/item_index_builder.py` | 44 | KEEP | patches+items union — generic IR stage |
| `passes/builtin/items_loader.py` | 45 | MOVE → playset | walks workshop tree via modlist; playset stage |
| `passes/builtin/leaf_analyzer.py` | 80 | KEEP | leaf-symbol identification — generic |
| `passes/builtin/linters/{__init__,depth_lint,unused_symbols}.py` | 178 | KEEP | generic Clausewitz lint passes |
| `passes/builtin/optimizers/{__init__,leaf_inliner}.py` | 281 | KEEP | leaf-inlining optimization — generic |
| `passes/builtin/overlay_writer.py` | 73 | DROP | WriteOverlayPass — megapatch patch-mode output |
| `passes/builtin/patches_loader.py` | 60 | DROP | YAML patches ingestion |

After prune, builtin passes split cleanly:
- Generic Clausewitz analysis: call_graph, depth, global_var,
  leaf_analyzer, linters, optimizers → stay
- Generic compile stages: compile_pass, item_index_builder → stay
- Playset stages: items_loader → moves with ingest
- Megapatch-specific writers: flatten_writer, overlay_writer,
  patches_loader → drop

A new generic writer pass `crate_output_writer.py` comes in under #82
to replace overlay_writer for Clause crates.

### `tools/` — split

| path | loc | fate | reason |
|------|-----|------|--------|
| `tools/__init__.py` | 7 | KEEP | package marker |
| `tools/feature.py` | 153 | KEEP | Clause feature scaffolding (struct + impl) — Clause-native |
| `tools/new_patch.py` | 233 | DROP | YAML patch scaffold — megapatch-era |
| `tools/trace.py` | 288 | KEEP | symbol call-trace debug tool — generic analysis utility |

### `units/` — KEEP, relabel

| path | loc | fate | reason |
|------|-----|------|--------|
| `units/{__init__,types,registry}.py` | 188 | KEEP | generic CompileUnit protocol |
| `units/builtin.py` | 81 | ABSTRACT | removes hardcoded `megapatch`/`heritage` unit entries; units now come from workspace manifest |

### `utils/` + `validate/` — KEEP entirely

- `utils/{__init__,fs}.py` — generic FS helpers
- `validate/{__init__,cwtools_runner}.py` — CWTools.exe invocation wrapper

Rationale: wholly generic.

### `assets/`

Dir contains baked-in data (schemas, templates). Audit separately
during execution — most likely all MOVE → playset data or DROP
depending on contents. Flagged as TBD.

## Summary

| fate | file count | approx. loc |
|------|-----------:|------------:|
| KEEP | 83 | ~13 200 |
| ABSTRACT | 8 | ~3 200 |
| MOVE → playset | 14 | ~2 700 |
| DROP | 13 | ~2 500 |

Total prune impact: ~5 200 loc leave `compiler/`; ~3 200 loc get
genericized; the remaining ~13 200 loc is untouched Clause-native
core + generic Clausewitz analysis.

## Execution phases

Ordered for incremental green-test commits. Each phase builds on the
previous; tests must stay green at every step.

### Phase 0 — plan ACK (this document)

User reviews + approves the classification above. Adjust any contested
calls before touching code.

### Phase 1 — DROP megapatch YAML plumbing

Single commit deleting:
- `ingest/parse_patches.py`, `ingest/migrate_patches.py`
- `patches/` (whole dir)
- `macros/` (whole dir)
- `flatten/` (whole dir)
- `tools/new_patch.py`
- `passes/builtin/{flatten_writer,overlay_writer,patches_loader}.py`
- any test files exercising these modules

Before this commit runs, grep for imports of each target module and
update/delete each call site. No module deleted without its callers
resolved.

### Phase 2 — ABSTRACT megapatch defaults out of Config

Single commit:
- Strip `CUSTOM_MOD_HEAD`, `CUSTOM_MOD_TAIL`, `DEFAULT_EVIDENCE_WEIGHTS`
  from `config.py`
- Drop `patch_dir`, `heritage_patch_dir`, `heritage_late_patch_dir`,
  `macro_dir`, `passes_dir`, `rules_file` fields
- Replace with `workspace_rules_path` → resolved from
  `Clause.toml.game` → `tools/clause/data/<game>.rules`
- Genericize `units/builtin.py` — unit list driven by workspace
  manifest, not hardcoded names
- Update `rules.py` to load from the new path
- Update `data_import.py` filename lookup

### Phase 3 — MOVE playset-flavoured modules

New package `tools/clause/playset/` containing:
- `loadorder/` (whole dir)
- `pipeline.py` (renamed → `playset/orchestrate.py`)
- `data_import.py` (renamed → `playset/data.py`)
- `ingest/` (whole dir minus dropped modules)
- `passes/builtin/items_loader.py`

Separately, `tools/clause/deploy/` (top-level sibling to `compiler/`
and `playset/`):
- `deploy/` (whole dir, verbatim move)

Two file-moving commits: one for playset, one for deploy. Each
fixes its imports in the same commit. The `__main__.py` `megapatch`
CLI subcommands dealing with these become stubs that call into
`clause.playset` / `clause.deploy` (pre-staging CLI1 + CLI4).

### Phase 4 — split `__main__.py`

- Leave pure-compile subcommands (`build`, `cache`, `validate`) under
  `compiler/__main__.py`
- Move `modlist`, `loadorder`, `deploy-*` subcommands out
- `clause playset` CLI dispatch lives at `playset/__main__.py`
- `pyproject.toml` adds no new entry point — everything still hangs
  off `clause` top-level CLI

### Phase 5 — drop `megapatch` entry point

Once #82 lands with `clause playset` covering every surviving
megapatch subcommand, remove the `megapatch = ...` script line from
`pyproject.toml`. Announce deprecation in README.

## Done criteria

- `grep -r "megapatch" tools/clause/compiler/` returns only
  historical references (docstrings, deprecation notices)
- `grep -r "heritage" tools/clause/compiler/` returns zero matches
- `grep -r "stellaris" tools/clause/compiler/` returns zero matches
- No import of `compile.patches`, `compile.macros`, `compile.flatten`
  anywhere in the repo
- `tools/clause/compiler/` contains only Clause-native compiler +
  generic Clausewitz analysis infrastructure
- All playset/modlist/workshop/deploy logic lives under
  `tools/clause/playset/` and is reachable via `clause playset`
- Tests green; `clause build` on a Clause crate still works
  end-to-end

## Cross-references

- #81 — this task
- #82 CLI1 — `clause playset` subcommand (destination for MOVE items)
- #83 GAME1 — crate rename (`vanilla` → `stellaris`, `dlc_*` →
  `stellaris_dlc_*`) — parallel, non-blocking
- #84 CLI2 — per-game rules config `tools/clause/data/<game>.rules`
  (consumed by Phase 2 Config abstraction)
- #85 ART1 — `clause art` (independent)
- #86 CLI3 — `clause doctor` (downstream of playset surface)
- #87 CLI4 — `clause install/deploy` (downstream of Phase 3 MOVE)
