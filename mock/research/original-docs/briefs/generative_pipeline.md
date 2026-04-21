# Brief — #99 PIPE1: generative pipeline abstraction

**Status**: SHIPPED 2026-04-18.
**Blocks**: #98 ART2 (artgen rewrite).
**Supersedes**: earlier draft that proposed a sibling node framework
(rejected — we have a scheduler already; reuse it, don't reinvent it).

## What landed

- `Generator` pass kind + `RetryPolicy` in `tools/clause/compiler/
  passes/base.py`. Identity folded into fingerprint via
  `ctx.provider.identity()`.
- `Subgraph` helper pass in `passes/subgraph.py`. Owns a child
  `PassRegistry`, runs a child `Scheduler`, exports selected artifacts
  back to the outer graph. Fingerprint folds in every child's
  `(pass_id, version)` so cache invalidates transitively.
- `LLMProvider` protocol + `ProviderResponse` + `GeminiProvider` stub
  in `passes/builtin/generative/providers/`. Runtime-checkable; fake
  providers satisfy the contract by shape.
- Scheduler accepts `provider=...` in its constructor and
  `initial_artifacts=` / `initial_artifact_fingerprints=` in
  `execute()`. Retry loop wraps Generator runs only.
- `PassContext.provider` carries the provider through to passes.
- `clause gen <family>` CLI: discovers
  `compiler.passes.builtin.generative.<family>` and runs it against
  the shared cache. `--llm offline` (default) uses a deterministic
  stub; `--llm gemini` wires the v1 shim.
- Probe family at `passes/builtin/generative/probe/` with `EchoGenerator`
  for end-to-end wiring smoke tests.
- 27 new tests across `test_generator_pass.py`, `test_subgraph.py`,
  `test_llm_provider.py`, `test_clause_gen_cli.py`. Full suite green.

Prompt templating via slice-3 macros remains deferred to when a family
needs it; the infrastructure is in place (#95 already shipped the macro
interpreter). `#98 ART2` unblocks now.

## Locked decisions

- **Reuse existing scheduler + pass registry wholesale**. No parallel
  scheduler. Different subcommands discover different pass sets.
- **Add one new pass kind**: `Generator`. Distinct retry semantics for
  LLM / content-producing work. Everything else (reads/writes,
  fingerprint, cache) is unchanged.
- **Add `Subgraph` helper pass** that runs a child scheduler as one
  opaque node in the outer graph. Makes serializable pipeline
  authoring trivial.
- **Generative pass layout**: `passes/builtin/generative/{art,event,
  trait,...}/`. One subdir per pipeline family. `clause gen <family>`
  loads core passes + `generative/<family>/*`.
- **Pipeline files are Clause-authored**, not TOML. Dogfood the
  language. A pipeline file is `static PIPELINE: Subgraph = { ... };`
  in Clause.
- **Provider identity is part of every Generator's fingerprint**.
  Switching Gemini → OpenAI invalidates cached outputs. Providers
  give different responses; we shouldn't pretend otherwise. Cheap
  meta to record — future-flexible regardless of whether we end up
  using it (for example, letting agents from different providers
  continue each other's work when explicitly configured would be a
  follow-on; recording the fingerprint now costs nothing and keeps
  the option open).
- **Prompt templating via Clause itself**. Templates are `.cse` files
  that run through the slice-3 macro interpreter — same runtime, no
  divergent template language.
- **Sync primitives with ThreadPoolExecutor** for fan-out. Async /
  streaming deferred.

## Why

Current state:
- `tools/clause/artgen/` = ad-hoc bash + Python.
- `clause art` wraps the bash pipeline.
- `clause author event` is a separate one-off flow.
- No shared abstraction.

User direction (2026-04-18): "abstract the generative pipeline steps
so we can later easily use the same machinery and plumbing to write
automatic event creation via llms, perhaps automatic trait creation
and all that."

## Core insight: we already have the scheduler

`tools/clause/compiler/passes/` has:
- Four pass kinds: `Analyzer`, `Linter`, `Optimizer`, `Writer`.
- `PassRegistry` with `discover()` walking the builtin dir + optional
  user dir.
- `Scheduler` that resolves `reads`/`writes` into a DAG, topologically
  sorts, runs each pass against a shared `PassContext`, and caches
  every pass's output by fingerprint (inputs + config + version).
- Artifact type with hash-addressed persistence in the SQLite cache.

That's ~all the machinery the original brief proposed we build from
scratch. The right move is to **reuse it** and load different pass
sets depending on subcommand.

## Design

### One scheduler, multiple pass sets

`clause build` already loads the compiler passes. `clause art` and
future generative subcommands (`clause gen event`, `clause gen trait`,
etc.) load a different set of passes into the SAME scheduler. The
scheduler doesn't know or care what kind of work its passes do — it
just schedules them by reads/writes.

Concretely:
- Existing compiler passes are "compiler" passes. Live under
  `passes/builtin/`.
- Art-generation passes live under `passes/builtin/generative/art/`.
- Event-generation passes live under `passes/builtin/generative/event/`.
- Each subcommand picks which packages to discover and loads them.
  `clause build` loads core compiler passes. `clause gen art` loads
  core + `generative/art/*`. `clause gen event` loads core +
  `generative/event/*`.

No new scheduler. No new pass base class. No new registry. The
existing infrastructure handles DAG ordering, caching, and execution
for generative pipelines the same way it does for the compiler.

### New pass kind: `Generator`

One genuinely new thing: a `Generator` pass kind alongside
`Analyzer`/`Linter`/`Optimizer`/`Writer`. Generators call out to LLMs
(or any other content-producing source) and write an `Artifact` that
downstream passes consume.

The only reason `Generator` is its own kind: error semantics are
different. An LLM call that fails should retry with backoff; a
Generator's error shouldn't mark the pass as passed-but-empty. Making
it a distinct kind lets the scheduler apply the right error policy
without Generators pretending to be Analyzers with extra steps.

Everything else (reads/writes, fingerprint, cache) is unchanged —
caching a Generator's output by fingerprint is exactly what we want:
same prompt + same params → reuse the cached response instead of
calling Gemini again.

### LLM provider shim

`LLMProvider` protocol: `generate(prompt, params) -> Response`.

- `GeminiProvider` wraps existing Gemini calls. Only provider in v1.
- Providers are injected into the `PassContext` — Generators pull the
  active provider from the context, same way existing passes pull
  `Config`.
- Retry policy lives at the `Pass.run()` wrapper (scheduler-level), not
  inside each Generator.

### Sub-graphs as nodes

Second new primitive: a `Subgraph` pass — an Optimizer-style or
Generator-style pass whose internal work is itself a small DAG
resolved and run by the same scheduler in a nested context. To the
outer graph, it looks like one opaque pass with declared
reads/writes.

Why:
- **Serializable pipeline authoring.** A subgraph is just a list of
  pass IDs + a reads/writes boundary. That's a tiny JSON/TOML/Clause
  file. You can ship an "event-generation pipeline v1" as a
  saved-subgraph file; `clause gen event --pipeline my.subgraph`
  loads and runs it.
- **Recombinable.** A pipeline author composes a large flow out of
  small reusable subgraphs (one for prompt assembly, one for LLM
  call, one for validation, one for format conversion) without
  touching the outer scheduler.

Implementation is small: `Subgraph.run()` instantiates a child
scheduler with a filtered registry, runs it, unwraps the child's
final artifact, writes it out. Caching composes naturally — the
subgraph's fingerprint is its member passes' fingerprints.

### Prompt templating

We already have templating patterns elsewhere in the codebase (the
artgen prompt fragments, the event-author layered system). The
right move per user direction is:

**Dogfood Clause itself for templating.**

A prompt template is a `.cse` file in the content crate. Its output is
a `TokenStream` (or `String`) — exactly what macros already produce.
The runtime is the slice-3 macro interpreter. A "template pass" is a
Generator that resolves a `.cse` template + input bindings into a
final prompt string, then hands that off to the LLM provider pass.

This falls out of slice 3: once Clause macros can run at compile
time, they can run at generation time too — it's the same interpreter.
The whole thing stays inside the language.

## Scope of #99 v1

In:
- Add `Generator` pass kind to `passes/base.py`.
- Add `Subgraph` pass helper (convenience wrapper that runs a child
  scheduler).
- `LLMProvider` protocol + `GeminiProvider` shim in
  `passes/builtin/generative/providers/`.
- `PassContext` extension to carry the active provider.
- Scheduler-level retry policy for Generator errors.
- Integration test: a trivial generative pipeline that loads fake
  passes into the real scheduler and produces a cached artifact.

Out (deferred):
- Clause-templated prompts — blocked on slice 3 macros landing. Once
  slice 3 is in, prompt templating slots in naturally.
- Actual artgen pipeline port (#98 ART2 — next task after this).
- Additional providers (OpenAI, Anthropic, local). Add when needed;
  the protocol supports them trivially.
- Streaming responses. v1 blocks until complete.

## Open questions — all resolved 2026-04-18 (see Locked decisions above)

## Verification

- Unit: `Generator` kind registers, runs, errors retry, final error
  propagates correctly.
- Unit: `Subgraph` runs a child scheduler, caches at both levels.
- Unit: `LLMProvider` protocol — fake provider satisfies it,
  `GeminiProvider` passes its own contract test.
- Integration: `clause gen <probe>` where `<probe>` loads a fixture
  pass set, produces a deterministic artifact, second run hits cache.
