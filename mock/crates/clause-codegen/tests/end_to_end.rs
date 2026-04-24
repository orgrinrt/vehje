//! End-to-end `emit` entry tests.
//!
//! Drive the top-level `emit(&Resolved, &str) ->
//! Result<CodegenArtifact, CodegenError>` over a default
//! `Resolved`. Skeleton round: `native` always returns an empty
//! `Binary` artifact, missing targets surface `TargetNotFound`.

use vehje_codegen::{ArtifactKind, CodegenError, emit};
use vehje_resolve::Resolved;

#[test]
fn emit_native_empty_resolved() {
    let resolved = Resolved::empty();
    let artifact = emit(&resolved, "native").expect("native emit ok");
    assert_eq!(artifact.kind, ArtifactKind::Binary);
    assert!(artifact.bytes.is_empty());
    assert!(artifact.diagnostics.is_empty());
}

#[test]
fn emit_missing_target() {
    let resolved = Resolved::empty();
    let err = emit(&resolved, "bogus").expect_err("missing target errors");
    assert!(matches!(err, CodegenError::TargetNotFound { .. }));
}
