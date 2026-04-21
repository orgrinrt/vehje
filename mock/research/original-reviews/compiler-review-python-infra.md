# Compiler Review — Python Infrastructure Lens

Reviewer: senior Python infrastructure engineer.
Scope: `tools/megapatch/megapatch_compiler/` as of 2026-04-16.
Method: read TENETS, compiler-passes design, COMPILER_EXTENDING, then
walked the package from entry-point inward.

## What's Good

- **Pass framework is well-conceived.** `passes/base.py` is a clean abstract
  hierarchy (Pass → Analyzer/Linter/Optimizer/Writer) with class-level metadata
  (`pass_id`, `requires`, `reads`, `writes`, `version`, `config_inputs`) that
  the scheduler can introspect. `Pass.validate()` runs at registration. Discovery
  via `pkgutil.walk_packages` for builtins + directory-scan for user passes is
  the right pattern and matches the linter framework.
- **Cache design is principled.** `pass_artifacts` table keyed on
  `(pass_id, compile_unit, artifact_kind)` with fingerprint-based invalidation
  (`passes/cache.py:20-28`). Fingerprint default in `base.py:187-208` folds
  pass_id + version + read-artifact fingerprints + config inputs — exactly the
  right composition. Per-put commits (`cache.py:76-83`) trade speed for
  crash-safety, which is the right call.
- **Subprocess use is bounded and disciplined.** Only three places
  (`validate/cwtools_runner.py`, `deploy/deploy.py`, `ingest/download.py`) and
  each wraps an external binary (cwtools, rsync, steamcmd) that genuinely must
  run out-of-process. All three use Protocol-based mockable invokers
  (`SteamcmdInvoker`, `CwtoolsInvocation`, `CommandRunner`). Tenet 9 is not
  being violated — the leaf-inliner in-memory port already happened.
- **Parallel worker story is correct.** `parallel.py:101` explicitly uses
  `mp.get_context("spawn")` — exactly right for SQLite safety across fork.
  Per-worker SQLite connections with WAL. `test_parallel_equivalence.py`
  pins the byte-identical contract.
- **SQLite use is competent.** WAL + `synchronous=NORMAL` in `cache.py:230-231`,
  batched `executemany` at 5000 rows (`cache.py:323`), schema migration via
  `_migrate_patches_columns` (`cache.py:213-223`), JOIN-based grouping query
  in `iter_items_grouped` (`cache.py:357-389`). No N+1 anti-patterns I found
  in the hot paths.
- **Test coverage is broad.** 37 unit + 12 integration test files, covering
  the scheduler, cache invalidation, per-pass behavior, parallel equivalence,
  and heritage/bundle unit builds. Unit-level passes have dedicated
  test files (`test_call_graph_analyzer.py`, `test_leaf_analyzer.py`, etc.).

## What's Bad

- **No `pyproject.toml` / `setup.py` anywhere.** `find -maxdepth 3` turned up
  zero packaging metadata. The CLI entry point is `python3 -m
  megapatch_compiler` which only works from the right working directory with
  `tools/megapatch/` on `sys.path`. There's no `pip install -e .`, no
  console_scripts entry point, no version pin. For a 4000-line compiler that's
  a real gap.
- **`tools/` subpackage naming collision.** The compiler has a `tools/`
  subpackage (`feature.py`, `trace.py`, `new_patch.py`) sitting inside
  `megapatch_compiler/` — while the outer project also has a `tools/` directory.
  Confusing to grep, confusing to import paths. The inner ones are
  user-facing CLI helpers; call them `cli_helpers/` or fold into
  `__main__.py`. `migrate_patches.py` is a similar one-shot — lives under
  `ingest/` but it's really a CLI command.
- **`pipeline.py` still has procedural prelude outside the scheduler.**
  Lines 132-189 run Phase 0/2/3 (preflight, data_import, loadorder ingest,
  macro registry, pre-ingest `ingest_patches`/`ingest_items`) in hand-wired
  code BEFORE the scheduler gets invoked at line 198. The design doc
  explicitly calls this out as a Stage 3 target ("Flip pipeline.run() to use
  the scheduler") but it hasn't landed. Any loader-pass fingerprint that
  reads from meta depends on this prelude having run first — a fragile
  implicit ordering.
- **`__main__.py` is 1003 lines.** One file with ~14 `cmd_*` functions, a
  500-line `build_parser()`, and ad-hoc argument-threading. This should be
  one module per subcommand under a `cli/` package, or a click/typer-based
  split. The length alone is a testability/maintainability hazard.
- **`build.py:486-522 _reconstruct_results_from_tree` is a code smell.**
  It's explicitly documented as "provisional": tree-linters need
  CompileResults but pipeline.run() doesn't return them, so the build
  round-trips via disk reads. The comment "a later refactor can have
  pipeline.run() return the CompileResults directly" has been there long
  enough to be real tech debt. The final COMPILE_RESULTS artifact is right
  there in `ExecutionResult.artifacts`; plumb it through.

## What Needs Fixing

### CRITICAL

1. **Scheduler skip logic is effectively dead** (`scheduler.py:273-295`).
   `_should_skip_for_upstream` has an explicit `pass` with a comment saying
   "For now just trust the plan." This means an analyzer failure upstream
   of a writer won't actually skip downstream — the writer will happily run
   against whatever was (or wasn't) populated in `ctx.artifacts`. This
   defeats the error-handling contract documented in the scheduler docstring
   (lines 13-22). Fix: track which passes produced which artifacts, and
   skip if the pass writing any `reads` input was marked skipped or errored.

2. **`conn.commit()` on a closed connection path in pipeline.run()**
   (`pipeline.py:211-212`). The scheduler gets a connection, passes emit
   via per-put commits (`passes/cache.py:83`), and then pipeline.run closes
   it on line 212. But `build.py:388` re-opens the DB *twice* for linter
   contexts (`_build_lint_context`) via `cache.open_db` without commit
   coordination. Not a bug today because each reopen is short-lived and
   read-only, but the pattern is fragile — one write from the lint phase
   in the future and you have a "DB is locked" landmine.

### HIGH

3. **`bare except Exception` in scheduler `_run_one`** (`scheduler.py:311`).
   The `# noqa: BLE001` acknowledges it, but catching every exception and
   returning it as a string in a `PassReport.error` loses type information
   for callers. Consider a structured `PassFailure` dataclass with
   `exc_type`, `exc_message`, `traceback_text` so downstream tooling
   (manifest writer, CI) can differentiate transient from hard failures.

4. **`passes/artifacts.py` serializer silently skips dataclass values**
   (`artifacts.py:75-78`). The scheduler swallows the `TypeError` as
   "not cacheable" at `scheduler.py:353-361` and logs at DEBUG. A pass
   that returns a dataclass-bearing artifact silently loses caching without
   any visible warning, producing confusing "why didn't my cache hit?"
   behavior. Upgrade the log to WARNING, or make it a startup-time
   validation error when a pass's declared `writes` includes an Artifact
   that carries dataclass data.

5. **No DAG library; custom Kahn's implementation with O(n) insort**
   (`scheduler.py:80-151`). Current `_insort` is O(n) per insert; fine at
   tens of passes, but there's no reason not to use `graphlib.TopologicalSorter`
   from stdlib (Python 3.9+). Less code, battle-tested, cycle detection
   built in. Would delete ~40 lines and remove a subtle maintenance surface.

6. **`CompilePass` returns `list[CompileResult]` — dataclasses — as an
   artifact** (`passes/builtin/compile_pass.py:90-91`). This hits the
   "not cacheable" path in `artifacts._encode` (dataclass detection).
   The compile cache (the lower-level `compile` SQLite table, `cache.py:91-98`)
   still catches unchanged items, but the whole COMPILE_RESULTS artifact
   always re-materializes in memory and never cache-hits at pass level.
   That's OK today because compile is the dominant cost, but it means the
   CALL_GRAPH / LEAF / DEPTH analyzers built on top also always recompute —
   they read COMPILE_RESULTS whose fingerprint changes every run. Either
   add a CompileResult serializer, or have CompilePass expose a
   content-fingerprint-list artifact that downstream analyzers key on.

### MEDIUM

7. **`pipeline.py:174-185` imports inside the function body** and does the
   actual ingest — mixing scheduler-prelude and scheduler invocation. Pull
   the ingest into a dedicated `IngestBootstrap` callable; the pipeline.run
   body should be ~30 lines (preflight, bootstrap, scheduler.execute, return
   stats).

8. **`Config` is partially frozen + runtime `replace()`** (`config.py:112`,
   `pipeline.py:189`). `frozen=True` on Config gives the illusion of
   immutability, but `replace(cfg, filter_dir=..., jobs=...)` at runtime
   creates a new one with runtime-only knobs. Consider splitting into
   `Config` (immutable, path/DB configuration) and `BuildOptions` (runtime,
   mutable, passed alongside). Current design works but mixes concerns.

9. **No structured logging.** `logging.getLogger("megapatch.scheduler")` etc.
   are used, but output is `print(...)` in `__main__.py` and ad-hoc
   `logger.warning`/`logger.debug` elsewhere. A proper logging config (e.g.
   `structlog` or stdlib `logging.config.dictConfig`) would unify debug
   traces and make the pass report reproducible for CI.

10. **`Any`-soup in `PassContext`** (`base.py:76-77`). `config: Any`,
    `unit: Any` with a "CompileUnit | str — for back-compat" comment. Back-compat
    with what? Tests. That's a test-code smell seeping into production types.
    Update the tests, type `unit: CompileUnit` and `config: Config`, delete
    the `getattr(ctx.unit, 'inline_script_prefix', <fallback>)` everywhere
    (e.g. `leaf_inliner.py:48-55`).

### LOW

11. **`_robust_rmtree` retry loop** in `build.py:236-258` — catches OSError,
    sleeps 50ms, retries up to 3x. Reasonable macOS workaround but undocumented
    in a shared helper. Move to a utility module with a docstring explaining
    the APFS race.

12. **`writer.py:111` `_LOC_LANG_RE = __import__("re").compile(...)`** — the
    `__import__` stunt is to avoid a top-level `import re`, but there's no
    reason not to; just import normally.

13. **`CwtoolsTreeLinter` has `require_cwtools` flag but it's plumbed
    manually** — not connected to the pass framework, not auto-discovered
    as a Pass. Once linters migrate to Pass-kind (per the design doc Stage 5),
    this becomes one unified knob.

## What Could Be Improved

- **Ship a `pyproject.toml`** declaring: package name, `tools/megapatch`
  layout hint, `[project.scripts] megapatch = "megapatch_compiler.__main__:main"`,
  and pinned deps (pyyaml shows up in ingest, plus stdlib-only otherwise).
  Then `pipx install -e tools/megapatch` gives everyone a `megapatch` command.
- **Replace hand-rolled DAG with `graphlib.TopologicalSorter`**. Drop ~40
  lines in `scheduler.py`.
- **Unify the two linter frameworks.** The "classic" linters
  (`linters/builtin/*.py` — PatchLinter/ItemLinter/TreeLinter) and the new
  Pass-kind linters (`passes/builtin/linters/*.py`) both emit `Finding`s but
  go through different invocation paths (`build.py:172-209` vs scheduler).
  The design doc's Stage 5 calls for this. Doing it closes the "pipeline
  doesn't return CompileResults" hack.
- **First-class structured logging + tracing** — every Pass run already has
  a `PassReport` with duration; pipe those into a JSON lines trace file for
  perf regression detection. Ties into a DEPTH_ANALYZER/DEPTH_LINT story
  where the compiler itself gets profiled over time.
- **Pass-level dry-run / introspection CLI** — `megapatch passes list`,
  `megapatch passes plan --unit bundle`, `megapatch passes graph --dot` to
  visualize the DAG. Free once the registry is in place. Currently pass
  state is only visible in `build_manifest.json` after a full build.

## Specific Code Locations

- `tools/megapatch/megapatch_compiler/pipeline.py:132-189` — procedural
  prelude that should be scheduler-native.
- `tools/megapatch/megapatch_compiler/passes/scheduler.py:273-295` —
  dead skip-logic, documented bug.
- `tools/megapatch/megapatch_compiler/passes/scheduler.py:80-151` —
  hand-rolled Kahn's, replace with `graphlib`.
- `tools/megapatch/megapatch_compiler/passes/scheduler.py:311` — blanket
  Exception catch.
- `tools/megapatch/megapatch_compiler/passes/base.py:76-77` — `Any`-typed
  `config` and `unit`.
- `tools/megapatch/megapatch_compiler/passes/artifacts.py:75-78` — silent
  dataclass skip, cache miss.
- `tools/megapatch/megapatch_compiler/passes/builtin/compile_pass.py:90-91` —
  dataclass-bearing artifact that can't cache.
- `tools/megapatch/megapatch_compiler/__main__.py` (1003 lines) — split
  into `cli/` package.
- `tools/megapatch/megapatch_compiler/tools/` — rename to `cli_helpers/`
  to disambiguate from outer `tools/`.
- `tools/megapatch/megapatch_compiler/build.py:486-522` —
  `_reconstruct_results_from_tree`, the provisional hack.
- `tools/megapatch/megapatch_compiler/build.py:236-258` — `_robust_rmtree`
  should move to a `utils/fs.py`.
- `tools/megapatch/megapatch_compiler/writer.py:111` — replace `__import__`
  with normal import.
- `tools/megapatch/megapatch_compiler/config.py:140-147` — runtime-knob
  fields on frozen Config; split into BuildOptions.
- (no file) — missing `pyproject.toml`.

## Verdict

The pass framework is the strongest part of the codebase and the part that
matters most for the project's long-term trajectory. The design doc and the
code agree on intent. The biggest outstanding debt is the procedural prelude
in `pipeline.py` (Stage 3 of the design that never landed) and the
dataclass-artifact cacheability gap — both known, both fixable without
architectural change. Second-tier debt is packaging (no `pyproject.toml`),
CLI bloat (`__main__.py` at 1003 lines), and the two parallel linter
frameworks. None of this blocks current builds; all of it will bite as more
passes land.
