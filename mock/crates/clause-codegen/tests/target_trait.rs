//! Per-target smoke tests.
//!
//! Asserts each built-in target exposes the documented name and
//! returns an empty artifact from its skeleton `emit`.

use vehje_codegen::{CodegenCtx, CodegenTarget, JominiTarget, NativeTarget};
use vehje_resolve::Resolved;

#[test]
fn native_name_matches() {
    assert_eq!(NativeTarget.name(), "native");
}

#[test]
fn jomini_name_matches() {
    assert_eq!(JominiTarget.name(), "jomini");
}

#[test]
fn native_emit_empty_artifact() {
    let resolved = Resolved::empty();
    let ctx = CodegenCtx::new(&resolved);
    let artifact = NativeTarget.emit(&ctx).expect("native emit ok");
    assert!(artifact.bytes.is_empty());
    assert!(artifact.diagnostics.is_empty());
}

#[test]
fn jomini_emit_empty_artifact() {
    let resolved = Resolved::empty();
    let ctx = CodegenCtx::new(&resolved);
    let artifact = JominiTarget.emit(&ctx).expect("jomini emit ok");
    assert!(artifact.bytes.is_empty());
    assert!(artifact.diagnostics.is_empty());
}
