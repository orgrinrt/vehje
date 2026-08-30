# clause-runtime-zig backlog

## Deferred runtime body

- **Arena allocator**. Per-session state owning compile-time
  buffers. No global allocator; arenas are cleared on
  `vehje_runtime_shutdown` (BACKLOG entry point).
- **IR interpreter**. Evaluates macro bodies + generative
  subgraph blueprints against the incoming `AbiNode` buffers.
- **Scratch-var evaluator**. Folds compile-time expressions
  to byte blobs the compiler re-inserts into the AST.
- **TokenStream emitter**. Macros return `AbiToken` arrays.
- **Diagnostic emission**. Maps Zig-side failures to
  `AbiDiagnostic` records. Matches the Rust `VehjeDiagnostic`
  shape (flat `AbiSpan`, pointer-plus-length message, kind).

## Deferred entry points

- `vehje_runtime_init(arena_bytes: u32)`. Matches the Rust
  `vehje_runtime_init` BACKLOG entry.
- `vehje_runtime_shutdown(handle)`. Teardown.
- `vehje_runtime_invoke_macro(handle, nodes, len)`. Macro
  dispatch.
- `vehje_runtime_eval_scratch(handle, expr_nodes, len)`.
  Scratch evaluation.
- `vehje_runtime_expand_generative(handle, blueprint, len)`.
  Generative subgraph expansion.
- `vehje_runtime_last_error(handle) -> *const AbiDiagnostic`.
  Error retrieval.

## Deferred framework features

- **Zig test suite**. `zig build test` runs unit tests
  alongside build.
- **Cross-compilation support**. `zig build -Dtarget=...` for
  Linux / macOS / Windows triple permutations.
- **`abi_version` check on load**. Runtime publishes its
  ABI version; driver refuses on mismatch.
- **Little-endian guard**. Refuse big-endian loads.
- **Panic-to-abort convention**. Zig-side aborts trigger
  a controlled abort rather than unwinding into Rust.
- **Build integration with cargo mock**. Optional automated
  `zig build` step during `cargo mock check`; currently out
  of scope (mockspace ignores `.zig` files).

## Non-blocking follow-ups

- JIT backend (once the interpreter stabilises).
- Native benchmark harness.
- `zig build -Dtest-filter=` for selective test runs.
- Debug-symbol emission and crash-report breadcrumbs.
