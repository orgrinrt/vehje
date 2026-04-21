# Megapatch Compiler — LLVM-engineer review

Date: 2026-04-16
Reviewer: battle-scarred LLVM/GCC compiler engineer
Scope: `tools/megapatch/megapatch_compiler/`, commits #273–#292.

## What's Good

The pass framework contract is the right shape. `Pass`/`Analyzer`/`Linter`/`Optimizer`/`Writer` with `pass_id`/`requires`/`reads`/`writes`/`version`/`config_inputs` class attributes is exactly what LLVM's `PassInfo`/`AnalysisUsage` gives you, minus the legacy warts. `Pass.validate()` in `passes/base.py:143` catches misconfiguration at discovery time instead of mid-run — that's the right place to fail.

The auto-discovery story in `passes/registry.py:83` is clean: `pkgutil.walk_packages` for builtins, `spec_from_file_location` for user passes, skipping underscore-prefixed files, filtering imported-not-defined-here symbols via `obj.__module__ != module.__name__`. Duplicate `pass_id` is a hard error (`passes/registry.py:35`). This is a sane LLVM-style plug-in loader.

Tuple-key dict encoding in `passes/artifacts.py:79–86` (with the `\x1f` unit-separator marker) is a pragmatic SQLite round-trip. Dataclasses raise `_NotCacheable` rather than silently round-tripping into bare dicts — that's *exactly* the right call; silent shape drift in the IR would corrupt downstream passes invisibly. Bravo.

Per-pass commit (`passes/cache.py:75`) instead of batch-at-end means a mid-build crash doesn't lose pass_artifacts rows. Incremental builds benefit.

CompileUnit (`units/types.py:32`) as a frozen dataclass with explicit `patch_dirs`/`output_dir`/`inline_script_prefix`/`descriptor`/`dependencies` is textbook. `UnitRegistry.resolve_order` (`units/registry.py:42`) does proper cycle detection. `default_registry(cfg)` factory means tests can swap config paths without module-level side effects.

The depth analyzer (`passes/builtin/depth_analyzer.py`) knows leaves don't add a call frame (line 74) — that is genuine cross-pass awareness and exactly what a proper DAG enables.

## What's Bad

The scheduler does **not** derive DAG edges from `writes → reads` flow (`passes/scheduler.py:85–101`). The docstring is explicit about it and rationalizes it via the LEAF_INLINER read-and-write-COMPILE_RESULTS "optimizer between analyzer and writer" pattern. That rationalization is wrong. LLVM solved this 20 years ago: an optimizer that rewrites an artifact issues a *new version* of the artifact, and the scheduler orders on version flow, not on artifact identity. The current design pushes the entire ordering burden onto pass authors via explicit `requires`, and the result is already visible at `passes/builtin/overlay_writer.py:49` — `WriteOverlayPass.requires = ("COMPILE", "ITEM_INDEX", "LEAF_INLINER")` where `LEAF_INLINER` is hard-coded. Any future optimizer that also rewrites `COMPILE_RESULTS` (dead-code elim, constant folding, on_action aggregator) will need every downstream pass patched to add it to `requires`. That's a *scheduler core* problem, not an author problem, and it will rot as more optimizers land.

The pass "fingerprint" for `COMPILE` is a lie (`passes/builtin/compile_pass.py:11`, docstring: "the pass-level fingerprint is intentionally coarse"). `CompilePass` declares `reads = (Artifact.BUILT_ITEMS,)` — but `BUILT_ITEMS` is a list of `Item` dataclasses, and per `passes/artifacts.py:75–78` dataclasses raise `_NotCacheable`. So `BUILT_ITEMS` never persists an `artifact_fingerprint`, which means its contribution to downstream fingerprints is the empty string `""` (`passes/base.py:199`). Every dependent pass's fingerprint collapses to `(pass_id, version, "", config)`. This is a cache-correctness bomb. The sub-level "compile" SQLite table saves the day *today* because it's content-hashed per-item, but the pass-level cache cannot distinguish two runs with completely different item universes. Any future analyzer built on `BUILT_ITEMS` will false-hit.

Related and worse: `PATCHES`/`ITEMS` are stored as `dict[tuple[str, str], list[Patch]]` — the dict key handling works via the tuple-sep encoding, but the VALUES are `list[Patch]` where `Patch` is a dataclass. `_encode` on that list recurses and raises `_NotCacheable` on each `Patch`. So `PatchesLoaderPass` and `ItemsLoaderPass` also never cache, meaning every incremental build re-runs *every downstream analyzer* because `artifact_fingerprints[PATCHES]` is `""`. The custom `fingerprint_inputs` overrides in those two passes (`patches_loader.py:43`, `items_loader.py:40`) fold a source-hash from meta and *that's what makes their own fingerprints stable* — but the scheduler still records an empty fingerprint under `artifact_fingerprints[PATCHES]` because serialization fails. The design comment at `scheduler.py:355` says "passes carrying rich Python objects skip persistence until a serializer is added" — correct, but then the dependent-pass fingerprint computation needs to also use the *producing pass's* `fingerprint` (which *is* recorded by `put_artifact`) as fallback. It doesn't. See `scheduler.py:351–365`: the scheduler sets `artifact_fingerprints[art_kind] = fingerprint` *only inside the cache-write success branch*. When serialization fails we continue (line 361) and the `artifact_fingerprints` is never set for that artifact. Silent incremental-build incorrectness.

`pipeline.run()` (`pipeline.py:107`) has two pre-scheduler phases — data_import and loadorder (`pipeline.py:143–158`) — that are neither passes nor declared dependencies. They write mutable state (`patches`, `mods`, `load_order` tables) that the scheduler's passes *read* via `ctx.conn`. If a pass reads DB state that the scheduler doesn't know about, cache fingerprints can go stale invisibly. This is the exact "hidden pipeline-state dependency" the extending-docs rule on `ctx` (`COMPILER_EXTENDING.md:121`) warns against, and it's committed in the core pipeline itself.

The pre-scheduler ingest calls at `pipeline.py:178–185` — `ingest_parse_patches.ingest_patches(...)` and `_ingest_extract_items.ingest_items(...)` — are doing the loaders' actual work. The `PatchesLoaderPass.run` (line 26-41) docstring admits this: "pipeline.run ran ingest_patches with this unit's patch_dirs before the scheduler started — meta is populated, patches table is materialized." The "pass" loads an already-materialized table. That's not a pass, that's a thin wrapper reading a side-channel. The pass framework has not actually eaten the loader; it's coexisting with it.

## What Needs Fixing

### CRITICAL — Incremental-build correctness

**C1.** `scheduler._run_one` fails to populate `ctx.artifact_fingerprints` when serialization is skipped (`passes/scheduler.py:353–365`). Fix: always set `ctx.artifact_fingerprints[art_kind] = fingerprint` *before* the `try/except` around `serialize`. Otherwise downstream fingerprints hash an empty string and incremental builds cache-hit with unchanged fingerprints even when upstream inputs changed.

**C2.** `CompilePass` (`passes/builtin/compile_pass.py`) declares `reads=(BUILT_ITEMS,)` and `writes=(COMPILE_RESULTS,)` but has no custom `fingerprint_inputs`, and BUILT_ITEMS carries dataclasses. Its fingerprint collapses. The sub-cache saves correctness today but the scheduler will happily emit "COMPILE cache=hit" on a build where BUILT_ITEMS genuinely changed but its producer (ITEM_INDEX) also couldn't serialize. Either serialize CompileResult/Item (add dataclass-aware codec in `artifacts.py`) or have every pass in the loader→compile chain override `fingerprint_inputs` to fold the meta content-hash. Do both; codec is the real fix.

### HIGH — Architectural

**H1.** Scheduler must derive a soft `writes→reads` edge (`passes/scheduler.py:_topological_sort`). The optimizer-rewrite-shared-artifact case is solvable with LLVM's pattern: the topological edge is `A-writes-X before B-reads-X`, except when `B.writes ∩ {X} != ∅` and `A` is an analyzer-for-X's-writer-chain — i.e. treat an optimizer's `writes` as superseding the analyzer's. A simple rule works for today's pipeline: derive edges only from analyzer-writes and writer-reads; optimizers that read-and-write the same artifact sit on the explicit `requires` chain among themselves. That eliminates the `("LEAF_INLINER",)` hard-code in `overlay_writer.py:49`.

**H2.** The pre-scheduler ingest in `pipeline.run()` (`pipeline.py:178–185`) must move into the passes themselves. If `PatchesLoaderPass` doesn't own `ingest_patches`, it's not a pass. Today's "pass reads the DB that was populated by the procedural prelude" model defeats per-pass caching — a user who deletes a YAML and re-runs: `ingest_patches` updates the SQLite `patches` table, but `PatchesLoaderPass.fingerprint_inputs` only hashes a meta key. If a bug in `ingest_patches`'s source-hash update silently retains a stale hash, every downstream pass reads stale data and reports cache-hit. The pass must call `ingest_patches` itself and fold the *actual* YAML tree mtimes/content-hashes into its fingerprint.

**H3.** `CompilePass.writes=(COMPILE_RESULTS,)` and `LeafInliner.writes=(COMPILE_RESULTS,)`. Both populate the same artifact slot. When both run, `ctx.artifact_fingerprints[COMPILE_RESULTS]` ends up with the inliner's fingerprint (correct for downstream) but `WriteOverlayPass` reads `ctx.artifacts[COMPILE_RESULTS]` which is whatever was last written. If the inliner erroneously early-returns the unmodified list (`leaf_inliner.py:154-157`), its fingerprint still replaces COMPILE's in the fingerprint map, and a downstream cache can hit on the wrong stored version. Design needs an artifact-versioning notion (LLVM's "analysis invalidation") not a single slot.

**H4.** `_should_skip_for_upstream` (`passes/scheduler.py:273`) has a dead branch. The comment at line 285–294 admits: "We can't tell without knowing who writes what, so err toward running … trust the plan." So upstream-analyzer errors don't cascade through `reads` edges, only through explicit `requires`. That means a pass that implicitly depends on a failed analyzer *will* run with missing artifacts, behave weirdly, and likely produce a spurious error. The scheduler knows every pass's `writes` — build a reverse index at plan time (`artifact → producer_pass_id`) and skip any reader whose producer is in `skipped`. This is ~10 lines.

### MEDIUM

**M1.** `_topological_sort` uses O(n²) `_insort` (`scheduler.py:144`). Fine for 20 passes, would be embarrassing at 500. Use `heapq`.

**M2.** The scheduler doesn't parallelize passes on the same topological level with disjoint reads/writes (design doc `compiler-passes.md:330–335` promises it, scheduler doesn't implement it). Low priority but named in the contract — remove the promise or deliver it.

**M3.** `drop_all`/`drop_pass`/`drop_unit` in `passes/cache.py` don't commit. `put_artifact` commits per-row (line 83). The `--rerun-all` / `--rerun-pass` flows rely on the caller committing. The scheduler doesn't commit after `drop_pass` in `scheduler.plan` (`scheduler.py:194-198`); if the build crashes before a later `put_artifact` commits, the evicted rows resurrect because the DELETE is in an uncommitted txn and the BEGIN implicit by subsequent INSERT rolls it back on crash. Fix: commit after the rerun drops.

**M4.** `Artifact` is an enum (`passes/artifacts.py:33`) but user passes wanting a new artifact kind cannot add one without editing the enum. That contradicts Tenet 9's "nothing about adding a new analysis should require editing the compiler core." LLVM has a registry-of-string-keyed-analyses model for this. For Stellaris scale this is a paper cut, but for the extensibility tenet it's a real wart.

**M5.** `WriteOverlayPass` has `writes=()` so the scheduler always runs it (`overlay_writer.py:51`), but the docstring at `overlay_writer.py:13` claims this is by design for "file I/O side effects aren't cache-lookup-able." That's true for the bytes on disk, but the *decision* to write is cacheable — a fingerprint over (COMPILE_RESULTS fp, output_dir, rules_map fp) that matches the previous write means the files are already there and identical. `build.py` blows away the staging dir every build (`build.py:362`) so this is moot at the build level, but if staging becomes incremental the writer needs a cache too.

**M6.** `PatchesLoaderPass.fingerprint_inputs` (`patches_loader.py:43`) folds `parse_patches_source_hash` from meta — but the meta key is only populated *if* `pipeline.run` calls `ingest_patches` first. A test that bypasses `pipeline.run` and invokes the scheduler directly gets an empty hash and a wrong fingerprint. This is tight coupling to the outer shell. Fix: the pass calls `ingest_patches` itself (see H2).

### LOW

**L1.** `_canonical` in `base.py:211` uses `str(value)` as last resort — stable within a session but not across Python versions if anyone ever `repr`'s a Path with a different separator. Prefer explicit `os.fspath` for paths.

**L2.** `Scheduler.execute` creates a fresh `PassContext` per run (line 230). CompileUnit-scoped context sharing across a `--unit bundle` build where multiple units share the same DB isn't defined. Today `pipeline.run` only runs one unit per call; if `run_build` is extended to multi-unit you need a richer context.

**L3.** The `gc_unknown_passes` helper (`passes/cache.py:114`) exists but nothing calls it. Wire it into `Scheduler.__init__` or a CLI `cache gc` subcommand, otherwise deleted passes leave rows forever.

## What Could Be Improved

- **Artifact versioning.** Teach `Artifact` to carry a version in the slot, not just a value. Optimizers bump the version; analyzers read the version they were built against. LLVM-style "preserved analyses" would drop the need for H3's explicit handling.
- **Pass-level profiling surfaces.** `PassReport.duration_ms` is there, but there's no per-pass memory tracking, no warnings for passes that take >30% of total build. A 50-pass future build will want it.
- **Deterministic pass ordering under parallel dispatch.** Even with level-parallelism, output must be bit-identical. The tiebreaker on `pass_id` alphabet (`scheduler.py:125`) is good — document it as load-bearing.
- **Pass-framework test harness.** `tests/unit/test_user_passes.py` exists but a `PassHarness` akin to `LintHarness` would make new-pass authoring frictionless (spawn a throwaway `Scheduler` + `PassRegistry` + synthetic artifacts, run one pass, assert results).
- **AST-level IR.** Design doc mentions this as future (`compiler-passes.md:703`). The current IR is `str` content on `CompileResult`, and `leaf_inliner._extract_body` (`leaf_inliner.py:61`) re-parses brace structure with a hand-written tokenizer. Every optimizer that touches content re-invents this. Land an AST artifact soon — it'll amortize across LEAF_INLINER, future CONSTANT_FOLDING, DEAD_CODE_ELIM, and the `replace_in_item` brittleness already mentioned in the design doc.
- **Unify the two lint frameworks.** `build.py:383–413` runs the classic PatchLinter/ItemLinter/TreeLinter alongside pass-kind linters. Design doc's Stage 5 says "refactor linters to reads/requires." Finish the migration; carrying two frameworks is how you get 900-line build.py files.

## Specific Code Locations

- `tools/megapatch/megapatch_compiler/passes/scheduler.py:85–101` — docstring explicitly declines to derive writes→reads edges; see H1.
- `tools/megapatch/megapatch_compiler/passes/scheduler.py:353–365` — fingerprint-on-cache-fail bug (C1).
- `tools/megapatch/megapatch_compiler/passes/scheduler.py:273–295` — dead-branch upstream-skip logic (H4).
- `tools/megapatch/megapatch_compiler/passes/scheduler.py:194–199` — `plan()` drops rows without committing (M3).
- `tools/megapatch/megapatch_compiler/passes/base.py:187–208` — default `fingerprint_inputs`; reads `artifact_fingerprints.get(a, "")` which is the root of C1.
- `tools/megapatch/megapatch_compiler/passes/artifacts.py:75–78` — dataclass → `_NotCacheable` is correct policy; missing dataclass codec for `CompileResult`/`Item`/`Patch` is what makes C2 / H2 real.
- `tools/megapatch/megapatch_compiler/passes/builtin/compile_pass.py` — needs `fingerprint_inputs` override (C2).
- `tools/megapatch/megapatch_compiler/passes/builtin/patches_loader.py:26–41` — "pass" is a thin wrapper around a side-effect; move `ingest_patches` into the pass (H2).
- `tools/megapatch/megapatch_compiler/passes/builtin/items_loader.py:19–45` — same shape as patches_loader.
- `tools/megapatch/megapatch_compiler/passes/builtin/overlay_writer.py:49` — hard-coded `LEAF_INLINER` in `requires` (H1 will eliminate).
- `tools/megapatch/megapatch_compiler/passes/builtin/optimizers/leaf_inliner.py:32–33` — TODO admitting the prefix hard-code; `_prefix_from(ctx)` already reads `ctx.unit.inline_script_prefix` correctly at line 55, but `config_inputs` is missing so changing the prefix via config wouldn't invalidate cache.
- `tools/megapatch/megapatch_compiler/pipeline.py:143–189` — procedural pre-scheduler phases + pre-ingest that make the "passes" framework partially ceremonial (H2).
- `tools/megapatch/megapatch_compiler/build.py:383–413` — two parallel lint frameworks; Stage-5 migration unfinished.

## Bottom line

The skeleton is right. The framework would be recognizable to anyone who's touched LLVM's legacy PassManager. The bones of extensibility — auto-discovery, explicit artifact contracts, per-pass cache with fingerprinting, DAG scheduler — are all in place and the framework tests exist.

The flesh isn't there yet. Three unfinished pieces bite: (1) the dataclass-artifact serialization gap silently zeroes out fingerprints for every downstream pass, making the cache correctness claim false in practice; (2) the pre-scheduler procedural phases mean the passes aren't actually owning their inputs; (3) the scheduler's refusal to derive write→read edges forces author-side explicit `requires` that won't scale past a dozen optimizers.

Fix C1 this week (ten-line fix in scheduler). Plan a dataclass codec (`artifacts.py`) and retire pipeline.py's pre-scheduler shim over the next month. The rest of the list is healthy technical debt to manage as the pass count grows from today's ~11 builtin passes toward the dozens the design doc envisions.
