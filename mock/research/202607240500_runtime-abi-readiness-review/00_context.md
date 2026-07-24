# Context: vehje runtime-ABI boundary readiness review

## The artefact

The vehje framework's runtime-facing ABI boundary: `vehje-runtime-abi` (the tier-tagged C ABI and residual
serialization the eventual Zig runtime consumes) and `vehje-runtime-driver` (the compiler-side dispatch over
that ABI). These are the seam a consumer's runtime builds against, so their soundness gates whether the
machinery is buildable-upon.

Files (`mock/crates/`):
- `vehje-runtime-abi/src/lib.rs` (the crate contract; `#![no_std]`, no alloc; bare primitives only at the
  `#[repr(C)]` wire boundary).
- `vehje-runtime-abi/src/entry.rs` (`ABI_VERSION`, the `#[repr(C)] BatchColumn` descriptor the host fills and
  passes by pointer, and `BatchedColumnEntry = unsafe extern "C" fn(call: *const BatchColumn)`; the actual
  `#[no_mangle] extern "C"` export is FIXME'd).
- `vehje-runtime-abi/src/value.rs` (`repr(transparent)` value nodes, the value arena).
- `vehje-runtime-abi/src/sink.rs` (the `#[repr(C)]` reserve/commit sink, an iteratee whose backpressure is
  inherent).
- `vehje-runtime-abi/src/encode.rs` (the residual serialization; block/function tables).
- `vehje-runtime-driver/src/lib.rs` (`dispatch`, the safe `Reader` over the value arena, `DriverError`).

The batched-column entry shape (one runtime-W batched column entry, W >= 2) is the decision an ABI
batched-execute benchmark validated on the performance and fairness axes; this review is the correctness and
soundness axis, not the performance one.

## The design oracle

- `mock/crates/vehje-runtime-abi/DESIGN.md.tmpl` and its `DEEPDIVE_*` (the value-transport contract).
- `mock/crates/vehje-runtime-driver/DESIGN.md.tmpl`.
- The bench-arc design memo `mock/research/202607231735_runtime-abi-batched-execute-bench-design/` (the
  batched-column-entry decision and its rationale).
- Workspace rules: no-bare-primitives (bare primitives permitted only at documented `#[repr(C)]` FFI
  boundaries with a tracked `lint:allow`), no_std / no_alloc, the C-ABI-crosses-no-build-env-effect rule.

## The question

Evaluate whether this runtime-ABI boundary is sound and ready for a consumer's runtime to build against, and
what if anything still blocks that. Give particular attention to: the `#[repr(C)]` layout stability and field
ordering of `BatchColumn` and the sink; the `unsafe extern "C"` entry contract and whether the pointer,
lifetime, and aliasing obligations are stated and honoured; the value arena and its safe `Reader` (are the
bounds and the typed decode actually sound, or is there a path to an out-of-bounds or type-confused read); the
residual serialization round-trip; the ABI versioning; and the no-alloc / bare-primitive-only-at-the-boundary
discipline. Reach your own conclusion from the source and the oracle. The first-light state (the `#[no_mangle]`
export and table population FIXME'd) is intended; distinguish a genuine soundness or contract defect in what
has landed from a mechanism deliberately deferred.
