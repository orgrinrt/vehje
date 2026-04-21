# Compiler Passes — Design

Date: 2026-04-15
Status: proposed
Supersedes: the subprocess-based `tools/inline-leaves.py` post-process.
Feeds: multi-task implementation batch to follow.

## Headline

The compiler becomes a DAG of passes, LLVM-style. Four pass kinds
(analyzers, linters, optimizers, writers) all subclass a common
`Pass` type, declare their inputs/outputs and dependencies, run in
topologically-sorted order, and cache their results keyed on input
fingerprints. Adding a new analysis or optimization is writing a new
module. The compiler core never grows.

Every compile unit — `megapatch`, `heritage`, any future first-party
mod, or a combined bundle — runs the same pipeline against the same
IR. Transformations never get embedded copies in per-mod shell scripts.

---

## Why this change

The current pipeline mixes two architectures:

1. **Linter framework:** the right shape. `PatchLinter` / `ItemLinter` /
   `TreeLinter` subclasses, auto-discovered via `linters/builtin/`, each
   producing `Finding` records. 26 lints registered, some cross-cutting
   concerns handled (vocabulary, ownership, baseline). Clean.

2. **Everything else:** hand-wired. Compilation is a hard-coded
   `pipeline.py:run()` function. Post-processing is a single
   `_run_inline_leaves()` subprocess invoking a 390-line standalone
   script against the written filesystem tree. The `inline_leaves`
   step can't use the compile cache, can't run on the in-memory IR,
   can't share analysis with a lint or another optimization, can't
   naturally run on heritage's output too. Heritage has its own
   embedded copy of the same algorithm in `heritage-assemble.sh`.

The lint framework proved the extensibility model works. This design
applies the same model to the rest of the pipeline.

### Concrete pain the current architecture causes

- **Phase 9 silently dropped during the bash→Python port** (2026-03).
  The pipeline docstring claimed `Phase 9 — post-process (inline-
  leaves, future: cwtools)`; `config.inline_leaves_script` existed.
  But nothing actually called the subprocess. Every megapatch build
  for weeks shipped without depth optimization (fixed 2026-04-15 via
  subprocess integration — a stopgap, not the real fix).

- **Heritage has an embedded copy** of the same 390-line algorithm
  inside `tools/heritage-assemble.sh`. Any change to inlining
  semantics has to be made in two places. When the Python pipeline
  lost the step, heritage kept working because of the duplication.
  Second-copy of a transformation is always a bug waiting to happen.

- **Optimizations run after write.** `inline-leaves.py` reads text
  files off disk, rewrites them, writes them back. Compile cache
  knows nothing about this — subsequent incremental builds can't
  tell whether the inline-leaves output is current. The invariant
  "DB state matches disk state" is violated for the output tree.

- **No way to share analysis.** A lint that wants to know "which
  scripted triggers are leaves?" today has to build that knowledge
  itself, duplicating whatever inline-leaves computes. Ditto
  "which scripted triggers are unused?" — separate TreeLinter walks
  the tree from scratch.

- **Compile units are second-class.** There's no clean way to run
  the full pipeline on heritage's input tree with heritage's patch
  tree — the Python compiler hard-codes `megapatch/patches/` and
  `megapatch/` output prefix. Heritage runs a separate bash
  pipeline. Any future first-party mod would need a third pipeline.

---

## Core concepts

### IR

The compiler's in-memory representation is already defined:

- **Item** — `(directory, item_id)` with its `ItemSource` list (from
  the workshop mod tree + vanilla) and `Patch` list (from YAML).
- **CompileResult** — the output of compiling one item:
  `content` string, `source_label`, errors, warnings, fingerprint.

The IR today is: `list[Item] → list[CompileResult]`. Passes read and
write against this list.

### Pass

Abstract base class. Every pass subclasses it.

```python
class Pass(abc.ABC):
    pass_id: str                        # unique, UPPER_SNAKE_CASE
    kind: Literal["analyzer", "linter",
                  "optimizer", "writer"]
    category: str                       # "correctness", "performance", …

    # Declared dependencies.
    requires: tuple[str, ...] = ()      # other pass_ids
    reads: tuple[Artifact, ...] = ()    # IR artifacts consumed
    writes: tuple[Artifact, ...] = ()   # artifacts produced

    @abstractmethod
    def run(self, ctx: PassContext) -> PassResult: ...

    def fingerprint_inputs(self, ctx: PassContext) -> str:
        """Hash every input this pass reads; default implementation
        walks `reads` tuple. Override for passes with non-artifact
        inputs (e.g. config flags, external file content)."""
```

`Artifact` is an enum of named IR components:

```python
class Artifact(Enum):
    PATCHES           = "patches"              # the Patch list
    ITEMS             = "items"                # the Item list
    COMPILE_RESULTS   = "compile_results"      # list[CompileResult]
    VOCABULARY        = "vocabulary"           # symbol sets
    CALL_GRAPH        = "call_graph"           # scripted_trigger/effect edges
    LEAF_SET          = "leaf_set"             # inlineable leaves
    DEPTH_MAP         = "depth_map"            # per-item max depth
    UNUSED_SET        = "unused_set"           # never-called first-party symbols
    # … growable
```

### Pass kinds

**Analyzer** — reads artifacts, produces a new artifact. Pure
function of its inputs. Output cached in SQLite keyed on input
fingerprint.

```python
class CallGraphAnalyzer(Analyzer):
    pass_id = "CALL_GRAPH_ANALYZER"
    reads  = (Artifact.COMPILE_RESULTS,)
    writes = (Artifact.CALL_GRAPH,)

    def run(self, ctx):
        graph = build_call_graph(ctx.compile_results)
        return PassResult(artifacts={Artifact.CALL_GRAPH: graph})
```

**Linter** — reads artifacts, produces Findings. Doesn't mutate IR.

```python
class UnusedScriptedTriggerLint(Linter):
    pass_id = "UNUSED_FIRST_PARTY_SCRIPTED_TRIGGER"
    requires = ("CALL_GRAPH_ANALYZER",)
    reads    = (Artifact.CALL_GRAPH, Artifact.COMPILE_RESULTS)

    def run(self, ctx):
        findings = []
        for trig in ctx.call_graph.nodes:
            if trig.ownership == FIRST_PARTY and not trig.callers:
                findings.append(Finding(...))
        return PassResult(findings=findings)
```

Every existing `PatchLinter/ItemLinter/TreeLinter` becomes a Linter
subclass. Their semantics don't change — the framework changes around
them.

**Optimizer** — reads artifacts, rewrites IR artifacts. The result
replaces the input artifact for downstream passes.

```python
class LeafInliner(Optimizer):
    pass_id = "LEAF_INLINER"
    requires = ("LEAF_ANALYZER", "CALL_GRAPH_ANALYZER")
    reads    = (Artifact.COMPILE_RESULTS, Artifact.LEAF_SET,
                Artifact.CALL_GRAPH)
    writes   = (Artifact.COMPILE_RESULTS,)

    def run(self, ctx):
        new_results = apply_inlining(
            ctx.compile_results, ctx.leaf_set, ctx.call_graph
        )
        return PassResult(artifacts={Artifact.COMPILE_RESULTS: new_results})
```

When an optimizer writes to an artifact, any downstream pass that
reads it gets the rewritten version. Analyzers that depend on it
invalidate their cache entries.

**Writer** — reads IR, writes to filesystem. Always last. One writer
per output mode (patch overlay, flatten bundle, etc).

### Pass dependencies

Every pass declares `requires: tuple[str, ...]` naming other passes
that must run first. The scheduler builds a DAG, topologically sorts,
and runs in order. Passes that don't depend on each other can run in
parallel (if safe).

Reads/writes are the finer-grained artifact dependencies. A pass that
reads `CALL_GRAPH` implicitly depends on whoever produces it. The
scheduler can derive `requires` from `reads` when desired.

### Fact storage

Analyzer artifacts land in a DB table per kind:

```sql
CREATE TABLE pass_artifacts (
    pass_id       TEXT NOT NULL,
    compile_unit  TEXT NOT NULL,       -- megapatch | heritage | bundle | …
    fingerprint   TEXT NOT NULL,       -- hash of inputs this pass read
    artifact_kind TEXT NOT NULL,       -- Artifact enum value
    payload_json  TEXT NOT NULL,       -- serialized artifact
    created_at    TEXT NOT NULL,
    PRIMARY KEY (pass_id, compile_unit, artifact_kind)
);
```

One row per (pass, unit, artifact). Fingerprint lets the scheduler
detect staleness — if current input fingerprint matches, load from
cache; otherwise re-run.

Artifacts too large to JSON-serialize comfortably (big call graphs,
the whole compile_results list) use the existing compile-cache table
or a dedicated blob table. Design decision per artifact; default is
JSON.

### Pass context

Runtime state passed to each pass:

```python
@dataclass
class PassContext:
    config: Config
    unit: CompileUnit                  # megapatch | heritage | …
    conn: sqlite3.Connection
    artifacts: dict[Artifact, Any]     # populated by the scheduler
    findings: list[Finding]            # accumulated; linters append
    logger: Logger
```

Passes never open DB connections themselves; they ask `ctx.conn`.
Passes never look for sibling passes' outputs directly; they declare
a read and the scheduler deposits it in `ctx.artifacts`.

---

## Compile units

A compile unit describes WHAT to compile. Three shipped, one
extensible pattern:

```python
@dataclass(frozen=True)
class CompileUnit:
    name: str                          # "megapatch", "heritage", "bundle"
    patch_dirs: tuple[Path, ...]       # YAML patch trees to parse
    item_filter: Callable[[Item], bool] | None
                                       # None = all items from items table
    output_dir: Path                   # e.g. .dist/megapatch/
    inline_script_prefix: str          # for the leaf-inliner
    descriptor: Descriptor             # .mod / descriptor.mod content
    dependencies: tuple[str, ...] = () # other units this loads after
```

Shipped units:

| name | patch_dirs | prefix | output_dir | depends_on |
|---|---|---|---|---|
| `megapatch` | `megapatch/patches/` | `megapatch` | `.dist/megapatch/` | — |
| `heritage` | `megapatch/heritage/patches/` | `heritage` | `.dist/heritage/` | megapatch |
| `heritage_late` | `megapatch/heritage_late/patches/` | `heritage_late` | `.dist/heritage_late/` | heritage |
| `bundle` | all of the above combined | `bundle` | `.dist/bundle/` | — |

CLI becomes:

```bash
megapatch build                    # builds every enabled unit in order
megapatch build --unit megapatch   # megapatch only
megapatch build --unit heritage    # heritage only (fails if megapatch
                                   #   isn't built, per dependency)
megapatch build --unit bundle      # all-in-one self-contained mod
```

User-defined units could live in a `megapatch/units/*.toml` directory —
same auto-discovery pattern as macros/linters. Deferred; no use case
today.

### Unit-aware passes

Passes that vary by unit (e.g., leaf-inliner's prefix) read the unit
from `ctx.unit`. Unit-invariant passes (a lint that runs on any
compiled output) ignore it.

---

## Pipeline scheduling

### Load order

1. **Bootstrap**: scheduler reads config, discovers all passes from:
   - `megapatch_compiler/passes/builtin/` (shipped)
   - `megapatch/passes/` (user-defined, like macros + linters)
2. **Validate**: every pass's `requires` and `reads` must resolve to
   other registered passes / artifacts. Circular deps are an error.
3. **Schedule per unit**: for each requested compile unit, build the
   pass DAG, topologically sort, produce an execution plan.

### Execution

For each pass in the plan:

1. Compute the pass's input fingerprint (hash of every `reads`
   artifact + any declared external inputs like config flags).
2. Look up `(pass_id, unit, artifact)` in `pass_artifacts` table.
3. **Cache hit** (fingerprint matches): deserialize cached payload
   into `ctx.artifacts`. Skip `run()`.
4. **Cache miss**: call `pass.run(ctx)`, take its `PassResult`, write
   each produced artifact to the table and to `ctx.artifacts`.
5. If the pass is a linter, accumulate findings into `ctx.findings`.
6. If the pass is an optimizer, any downstream pass that reads the
   rewritten artifact now sees the new version; their cache entries
   get invalidated via fingerprint mismatch.

### Parallelism

Passes at the same topological level with disjoint reads/writes can
run in parallel. The scheduler has a job pool; default `jobs=1`
(matches current behavior). `--jobs N` fans out. Within-pass
parallelism (e.g. the existing parallel compile loop) remains the
pass's internal concern.

### Error handling

A pass's `run()` can return `PassResult(errors=[...])`. Scheduler
collects errors per pass. Behavior by kind:

- Analyzer error → downstream passes that require it are **skipped**
  (not run). Build result reports skipped passes in summary.
- Linter error → findings that were accumulated before the failure
  are kept; downstream is unaffected.
- Optimizer error → IR is not rewritten. Downstream passes see
  previous version of the artifact. Warning reported.
- Writer error → build fails.

Release mode: any pass error is a build failure. Debug mode: errors
accumulate but the build completes.

### Debug output

For every run, emit a pass report (`build_manifest.json` existing
file grows a `passes` section):

```json
{
  "passes": [
    {"id": "PATCHES_LOADER",         "unit": "megapatch", "duration_ms": 120, "cache": "miss"},
    {"id": "ITEM_INDEX_BUILDER",      "unit": "megapatch", "duration_ms": 850, "cache": "miss"},
    {"id": "COMPILE",                 "unit": "megapatch", "duration_ms": 4200, "cache": "miss",
     "items_processed": 13538},
    {"id": "CALL_GRAPH_ANALYZER",     "unit": "megapatch", "duration_ms": 180, "cache": "miss"},
    {"id": "LEAF_ANALYZER",           "unit": "megapatch", "duration_ms": 45,  "cache": "miss",
     "leaves_found": 677},
    {"id": "UNUSED_TRIGGER_LINT",     "unit": "megapatch", "duration_ms": 12,  "cache": "miss",
     "findings": 0},
    {"id": "LEAF_INLINER_OPT",        "unit": "megapatch", "duration_ms": 95,  "cache": "miss",
     "inlined": 677, "callsites_rewritten": 40},
    {"id": "DEPTH_ANALYZER",          "unit": "megapatch", "duration_ms": 75,  "cache": "miss"},
    {"id": "DEPTH_LINT",              "unit": "megapatch", "duration_ms": 8,   "cache": "miss",
     "findings": 0},
    {"id": "WRITE_OVERLAY",           "unit": "megapatch", "duration_ms": 1100, "cache": "miss",
     "files_written": 4530}
  ]
}
```

---

## Caching

### Fingerprinting

Input fingerprint = hash of:
- Every artifact this pass `reads` (their own stored fingerprints
  from the cache, or the computed fingerprint of the current
  in-memory value).
- Pass version tag (class-level string; bump when semantics change).
- Config flags that affect this pass (declared via
  `config_inputs: tuple[str, ...]`).

Example — `LeafInliner.fingerprint_inputs`:

```
hash(
    compile_results_fingerprint,
    leaf_set_fingerprint,
    call_graph_fingerprint,
    "LeafInliner:v3",            # class version
    config.leaf_inline_prefix,   # unit.inline_script_prefix
)
```

### Invalidation cascade

Optimizer O writes artifact A. If O's input fingerprint changed → O
re-runs → A has a new fingerprint. Every pass that reads A sees the
new fingerprint → their own fingerprint changes → they re-run. Chain
propagates through the DAG.

### Cache eviction

SQLite table is append-only per pass + unit + artifact. The PRIMARY
KEY `(pass_id, compile_unit, artifact_kind)` means a new fingerprint
simply overwrites the previous entry. No eviction needed — the table
stays small (one row per active pass).

When a pass is removed from the registry (someone deletes the
module), its rows linger. A startup housekeeping pass can GC entries
for unknown pass_ids. Low priority.

### Force re-run

```bash
megapatch build --rerun-pass LEAF_INLINER_OPT
megapatch build --rerun-all
```

Invalidates named pass(es) and any downstream passes. `--rerun-all`
drops the entire `pass_artifacts` table.

---

## First concrete passes (what ships in the rewrite)

### Loader passes (unit-scoped)

- `PATCHES_LOADER` — parse YAML → patches table. Reads config unit.patch_dirs.
- `ITEMS_LOADER` — extract items from workshop mods. Reads config.steam_content.
- `ITEM_INDEX_BUILDER` — groups items + patches per `(directory, item_id)` key.

### Compile pass

- `COMPILE` — the existing `compile_item()` loop, refactored. Reads
  ITEMS. Writes COMPILE_RESULTS.

### Analyzers

- `VOCABULARY_ANALYZER` — the existing `Vocabulary.from_db`. Reads
  ITEMS + COMPILE_RESULTS. Writes VOCABULARY.
- `CALL_GRAPH_ANALYZER` — walks compiled content, extracts edges
  between scripted_triggers/effects. Reads COMPILE_RESULTS. Writes
  CALL_GRAPH.
- `LEAF_ANALYZER` — iteratively identifies leaves in the call graph.
  Reads CALL_GRAPH. Writes LEAF_SET.
- `DEPTH_ANALYZER` — per-item maximum brace + call depth, following
  scripted calls via CALL_GRAPH. Reads COMPILE_RESULTS + CALL_GRAPH.
  Writes DEPTH_MAP.
- `USAGE_ANALYZER` — per-symbol caller count. Reads CALL_GRAPH.
  Writes USAGE_MAP.

### Linters (rewritten as Linter subclasses)

Existing 26 lints refactored to declare `reads` and `requires`. No
semantic change. Immediate wins:

- `UNUSED_FIRST_PARTY_SCRIPTED_TRIGGER` moves from re-scanning
  content in `cross_references.py` to reading USAGE_MAP.
- `DEEP_SCOPE_CHAIN` moves from re-parsing content to reading
  DEPTH_MAP.
- `TRIGGER_DEPTH_EXCEEDED` reads DEPTH_MAP; optionally becomes
  inlining-aware by reading LEAF_SET too.

### Optimizers

- `LEAF_INLINER_OPT` — replaces `tools/inline-leaves.py`. Reads
  LEAF_SET + CALL_GRAPH + COMPILE_RESULTS. Writes new
  COMPILE_RESULTS with leaves extracted to `common/inline_scripts/`
  entries (represented in-memory as additional CompileResults, so
  Write phase emits them like any other item) and callsites rewritten.

The rewrite drops the subprocess and the filesystem round-trip. The
writer phase sees the post-optimization IR and writes it straight out.

### Writers

- `WRITE_OVERLAY` — the current FIOS/LIOS `00_`/`zz_` prefix emission.
- `WRITE_FLATTEN` — the existing flatten mode.

Both read COMPILE_RESULTS. Exactly one writer runs per build,
selected by `--bundle` flag.

---

## Heritage + units

Today: `tools/heritage-assemble.sh` runs its own bash pipeline with
an embedded Python copy of inline-leaves.

After the rewrite: heritage becomes `CompileUnit("heritage", …)`.
`megapatch build --unit heritage` runs the exact same pass DAG
against heritage's inputs. The embedded copy in
`heritage-assemble.sh` gets deleted (or the shell script becomes a
thin wrapper calling `megapatch build --unit heritage`).

heritage_late gets its own unit. Any future mod does too. Three
lines of config per mod.

### The bundle unit

`megapatch build --unit bundle` compiles megapatch + heritage +
heritage_late into one output tree with one descriptor. A single mod
users can enable. Replaces the current client-side
`merge-and-optimize.sh`. Runs the same passes — the leaf inliner
operates on the combined symbol graph, so cross-mod leaves become
inline scripts too (heritage's `heritage_add_prestige = yes` called
from a megapatch event gets inlined if it qualifies).

---

## Discovery

Directory layout:

```
tools/megapatch/megapatch_compiler/
├── passes/
│   ├── __init__.py
│   ├── base.py              # Pass / Analyzer / Linter / Optimizer / Writer
│   ├── scheduler.py         # DAG build, topological sort, run loop
│   ├── cache.py             # pass_artifacts table ops
│   ├── artifacts.py         # Artifact enum, serialization hooks
│   └── builtin/
│       ├── patches_loader.py
│       ├── items_loader.py
│       ├── item_index_builder.py
│       ├── compile_pass.py
│       ├── call_graph_analyzer.py
│       ├── leaf_analyzer.py
│       ├── depth_analyzer.py
│       ├── vocabulary_analyzer.py
│       ├── usage_analyzer.py
│       ├── optimizers/
│       │   └── leaf_inliner.py
│       ├── linters/           # existing 26 lints, refactored
│       │   └── … (existing patch_shape, content_syntax, etc.)
│       └── writers/
│           ├── overlay_writer.py
│           └── flatten_writer.py
└── units/
    ├── __init__.py
    ├── builtin.py              # ships megapatch, heritage, bundle
    └── registry.py             # loads user units from megapatch/units/
```

Discovery via `pkgutil.walk_packages` on `passes.builtin` — the same
pattern `linters.registry.discover()` uses today. User passes: scan
`megapatch/passes/*.py`. A shipped registry dict tracks everything.

Adding a pass:

1. Write one module in `passes/builtin/` or `megapatch/passes/`.
2. Subclass the right kind (`Analyzer` / `Linter` / `Optimizer` /
   `Writer`).
3. Set `pass_id`, `requires`, `reads`, `writes`.
4. Implement `run`.
5. Write a unit test using `LintHarness` (extended to cover passes in
   general) or a pass-specific harness.

No edits to the scheduler. No edits to the core.

---

## Migration path

Staged rollout across multiple commits. Each commit leaves the tree
buildable with green tests.

### Stage 1: Pass framework (no behavior change)

- `passes/base.py` — abstract base + Artifact enum
- `passes/cache.py` — pass_artifacts table
- `passes/scheduler.py` — DAG + executor (unit tested against
  synthetic passes)
- Unit tests for scheduler, cache, fingerprint invalidation

No existing code touches it yet. Ships as dead code that tests prove
works.

### Stage 2: Wrap the existing pipeline as passes

The current `pipeline.run()` becomes a hand-wired sequence of pass
instances. Each phase becomes a Pass subclass:

- `PatchesLoaderPass` wraps `ingest_parse_patches.ingest_patches`
- `CompilePass` wraps the existing compile loop
- `WriteOverlayPass` wraps `write_results`

Still called from `pipeline.run()` procedurally; but each block is
now an honest Pass. Tests still pass.

### Stage 3: Flip pipeline.run() to use the scheduler

Replace the hand-wired sequence with `scheduler.execute(plan)`. The
plan is built from the registered passes' declared dependencies.
Output is identical — same passes, same order. Integration tests
verify byte-identical output.

### Stage 4: Port LEAF_INLINER_OPT into the framework

Reimplement `tools/inline-leaves.py` as in-memory transformations on
`CompileResult` objects. Remove the subprocess. Delete the old
script. heritage-assemble.sh loses its embedded copy.

Regression tests: output should match the current subprocess-based
output byte-for-byte on the current corpus.

### Stage 5: Refactor linters to reads/requires

One commit per linter (or per small group). Each moves from
"scan content in-place" to "read the relevant analyzer's artifact".
E.g.:

- `UNUSED_FIRST_PARTY_SCRIPTED_TRIGGER` declares
  `requires = ("USAGE_ANALYZER",)` and reads from USAGE_MAP.
- `DEEP_SCOPE_CHAIN` declares `requires = ("DEPTH_ANALYZER",)`.

Tests unchanged. Performance improves (no re-scanning content N
times when N linters share the same analysis).

### Stage 6: Compile units

Promote hard-coded megapatch-only configs to first-class
`CompileUnit` values. `megapatch build --unit heritage` ports the
heritage pipeline. Delete `heritage-assemble.sh` or thin it down.

### Stage 7: New passes start landing

Once the framework is solid, adding analyses/optimizations becomes
weekly work. Candidates:

- `DEAD_CODE_ELIMINATOR_OPT` — drop first-party scripted symbols
  with no callers.
- `CONSTANT_FOLDING_OPT` — expand compile-time `@variable` uses
  where the value is a literal.
- `MULTI_SOURCE_CALL_ANALYZER` — detect scripted calls that resolve
  differently per mod load order.
- `ON_ACTION_AGGREGATOR_ANALYZER` — the MERGE semantics for
  on_actions as a proper pass.
- `LOCALISATION_COVERAGE_ANALYZER` — every text key has a string in
  every enabled language.

Each lands as its own commit, no scheduler touched.

---

## Open questions / deferred

**Q1: Artifact serialization format.** JSON works for everything
today. Big artifacts (call graph at 14k items might be several MB)
could want msgpack or a binary format later. Defer until profile
shows cost.

**Q2: Pass parallelization safety.** The framework allows it but we
ship with `jobs=1`. Real parallelism needs passes to declare
"read-only" vs "mutating" and the scheduler to fan out non-conflicting
passes. Follow-up after stage 4.

**Q3: Cross-unit analysis.** When `--unit bundle` runs, the leaf
inliner sees the combined symbol graph. Good. But some analyzers
might only make sense per-unit (ownership attribution within
heritage vs megapatch). Decide per-analyzer; the `unit` field in
PassContext makes it explicit.

**Q4: Incremental within a pass.** The cache invalidates at
pass-level granularity. A pass that touches only 3 of 14k items on
an incremental build still re-runs in full. Stellaris's compile
time is dominated by extract (I/O bound) + compile (CPU bound); the
pass-level cache is enough for now. If leaf-inlining ever takes
>500ms, we revisit intra-pass incrementality.

**Q5: User units via config file.** Not needed today; defer. If we
ever want it, `megapatch/units/<name>.toml` + auto-discovery.

**Q6: Passes that read external files** (workshop mod source,
cwtools config). Those need to include file content or mtime in
their fingerprint. The Pass base class gets a `fingerprint_external`
hook that returns a dict. Default no-op.

**Q7: Writing the pass report.** Stats per pass land in
`build_manifest.json`'s new `passes` section. Format is documented
above. Consumers: CI, the build summary printer, potentially a
`megapatch passes` CLI to introspect pass state.

---

## What this enables

Once the framework is in place, these become one-file changes:

- **AST-level patching**: an analyzer builds a Clausewitz AST per
  item, optimizers operate on the AST instead of byte-level
  find/replace. Solves the `replace_in_item` brittleness from the
  original item-centric design doc.
- **Regression detection**: a pass that diffs COMPILE_RESULTS
  against the previous build's cached version, flags unexpected
  changes at release-gate time.
- **Playtest-driven lints**: a bug surfaces in-game → write an
  analyzer that detects the pattern + a lint that flags it. Commit
  lands without touching anything else.
- **Cross-mod integration checks**: an analyzer that reads the call
  graph for the bundle unit can flag "this scripted_trigger is only
  defined in heritage but megapatch calls it" — impossible to
  detect in per-unit builds, natural in a bundle-scope pass.
- **Performance optimization**: profile-guided pass that identifies
  frequently-fired on_actions and rewrites them for less depth.
- **Localization pipeline**: a pass that reads ITEMS, extracts every
  string in a "loc-key position", and generates stub `.yml` entries
  for missing translations.

---

## Summary

The compiler becomes a small scheduler around a growing registry of
passes. Each pass is a module. Adding intelligence to the compiler
is adding a module, not modifying the compiler. Every compile unit
gets every pass. Every incremental build re-uses every unchanged
analysis. The current architecture — hand-wired phases + one
subprocess-based post-process — becomes the pass framework's first
migrated example, proving out the model before the real growth
phase of adding dozens more analyses over the project's lifetime.
