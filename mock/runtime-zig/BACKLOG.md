# clause-runtime-zig — backlog

## Deferred runtime body

- **Arena allocator** — per-session state owning compile-time
  buffers. No global allocator; arenas are cleared on
  `clause_runtime_shutdown` (BACKLOG entry point).
- **IR interpreter** — evaluates macro bodies + generative
  subgraph blueprints against the incoming `AbiNode` buffers.
- **Scratch-var evaluator** — folds compile-time expressions
  to byte blobs the compiler re-inserts into the AST.
- **TokenStream emitter** — macros return `AbiToken` arrays.
- **Diagnostic emission** — maps Zig-side failures to
  `AbiDiagnostic` records. Matches the Rust `ClauseDiagnostic`
  shape (flat `AbiSpan`, pointer-plus-length message, kind).

## Deferred entry points

- `clause_runtime_init(arena_bytes: u32)` — matches the Rust
  `clause_runtime_init` BACKLOG entry.
- `clause_runtime_shutdown(handle)` — teardown.
- `clause_runtime_invoke_macro(handle, nodes, len)` — macro
  dispatch.
- `clause_runtime_eval_scratch(handle, expr_nodes, len)` —
  scratch evaluation.
- `clause_runtime_expand_generative(handle, blueprint, len)` —
  generative subgraph expansion.
- `clause_runtime_last_error(handle) -> *const AbiDiagnostic`
  — error retrieval.

## Deferred framework features

- **Zig test suite** — `zig build test` runs unit tests
  alongside build.
- **Cross-compilation support** — `zig build -Dtarget=...` for
  Linux / macOS / Windows triple permutations.
- **`abi_version` check on load** — runtime publishes its
  ABI version; driver refuses on mismatch.
- **Little-endian guard** — refuse big-endian loads.
- **Panic-to-abort convention** — Zig-side aborts trigger
  a controlled abort rather than unwinding into Rust.
- **Build integration with cargo mock** — optional automated
  `zig build` step during `cargo mock check`; currently out
  of scope (mockspace ignores `.zig` files).

## Non-blocking follow-ups

- JIT backend (once the interpreter stabilises).
- Native benchmark harness.
- `zig build -Dtest-filter=` for selective test runs.
- Debug-symbol emission and crash-report breadcrumbs.
