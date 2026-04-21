# Extending the Compiler

The compiler is a pipeline of auto-discovered **passes**. Adding a new
analysis, lint, or optimization does not require editing any compiler
code. You drop a file into `megapatch/passes/`, define a `Pass`
subclass, and the next build picks it up and schedules it.

This is Tenet 9 in practice: *the compiler itself is small. The
intelligence lives in the passes, and the set of passes grows over the
project's lifetime.*

## When to add a pass

Add a pass when:

- A class of bug should be caught at compile-time instead of in
  playtest.
- A mechanical optimization can rewrite IR to produce faster or
  depth-smaller output.
- A fact about the compiled tree is needed by more than one consumer
  (e.g. a call graph shared by the leaf inliner, depth linter, and
  unused-symbol linter).

Don't add a pass when:

- A one-off check would do (write a unit test on `CompileResult` in a
  test file).
- The work belongs to the patch author (lint YAML shape inside the
  existing linter framework).

## Pick the right kind

```
Analyzer    reads artifacts → writes artifacts          (facts)
Linter      reads artifacts → emits Findings             (diagnostics)
Optimizer   reads artifacts → rewrites IR artifacts      (transforms)
Writer      reads artifacts → writes files to disk       (emission)
```

If your pass produces data several other passes will consume, it's an
Analyzer. If it answers *is this right?*, it's a Linter. If it rewrites
COMPILE_RESULTS to produce a smaller/faster/cleaner output, it's an
Optimizer. If it turns artifacts into files, it's a Writer.

## File layout

Shipped passes live in `tools/megapatch/megapatch_compiler/passes/builtin/`.

User passes live in `megapatch/passes/` at the repo root. Discovery
imports every non-underscored `.py` file and registers any concrete
`Pass` subclass it finds. Private helpers go in `_helpers.py` style
filenames so discovery skips them.

## The contract

```python
from megapatch_compiler.passes.artifacts import Artifact
from megapatch_compiler.passes.base import Linter, PassResult


class MyLint(Linter):
    pass_id = "MY_LINT"                 # UPPER_SNAKE_CASE, unique
    category = "correctness"            # free-form grouping
    requires = ("CALL_GRAPH_ANALYZER",) # other passes that must run first
    reads = (Artifact.CALL_GRAPH,)      # artifacts this pass consumes
    writes = ()                         # artifacts this pass produces
    version = "1"                       # bump when algorithm changes
    config_inputs = ()                  # config attrs affecting output

    def run(self, ctx):
        graph = ctx.artifacts[Artifact.CALL_GRAPH]
        findings = []
        # ... derive findings from graph ...
        return PassResult(findings=findings)
```

`requires` is the explicit-sequencing list — it creates DAG edges
regardless of artifact flow. `reads` and `writes` document artifact
dependencies for human readers and the cache. The scheduler uses
`requires` for ordering; use it whenever you need a pass to run after
another one.

## Fingerprinting (when your pass has external inputs)

By default, a pass's fingerprint is `(pass_id, version, reads →
artifact fingerprints, config_inputs → values)`. Any change to those
invalidates the cache and re-runs the pass. If your pass reads
filesystem state, an environment variable, or anything not already
covered, override `fingerprint_inputs(ctx)` and fold those inputs into
the hash — otherwise you'll get stale cache hits.

## Testing

Passes are pure Python, so direct unit testing is easy. See
`tests/unit/test_user_passes.py` for the pattern:

```python
from megapatch_compiler.passes import PassContext, Artifact

ctx = PassContext(config=..., unit="megapatch", conn=...)
ctx.artifacts[Artifact.CALL_GRAPH] = {...synthetic graph...}
result = MyLint().run(ctx)
assert len(result.findings) == 1
```

Full integration tests (where the real pipeline runs your pass
end-to-end against a synthetic project tree) follow the pattern in
`tests/integration/test_pipeline_scheduler.py`.

## Anti-patterns

- **Don't re-scan content if an analyzer exists.** If you want to know
  what calls what, read `CALL_GRAPH`. If you need a list of leaves,
  read `LEAF_SET`. Duplicating the scan costs time AND invites drift
  between analyzer and consumer.
- **Don't shell out.** If you find yourself writing
  `subprocess.run(...)` from inside a pass, the thing you're shelling
  out to should probably be a pass.  That's the load-bearing example
  in Tenet 9 — the old `inline-leaves.py` subprocess is what the whole
  framework was designed to eliminate.
- **Don't depend on pipeline state beyond `ctx`.** Anything your pass
  needs flows through `ctx.artifacts`, `ctx.config`, `ctx.conn`,
  `ctx.unit`. Reading module globals or importing pipeline-internal
  state makes the pass non-reproducible and breaks caching.

## Worked example

See `megapatch/passes/example_find_todos.py` for a shipped user pass
that flags TODO comments in compiled output we authored. It reads
`COMPILE_RESULTS`, filters to first-party items, scans for the TODO
pattern, and emits INFO-level findings. No compiler edits; no
subprocess; no filesystem side effects.

## Related

- `docs/megapatch/design/compiler-passes.md` — architecture + design
  decisions behind the pass pipeline
- Tenet 9 in `docs/megapatch/TENETS.md` — the durable statement of why
  this is the way
- `docs/megapatch/LINTING.md` — legacy (PatchLinter / ItemLinter /
  TreeLinter) framework, still fully supported alongside pass-based
  lints
