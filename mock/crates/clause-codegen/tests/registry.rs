//! Target-registry tests.
//!
//! Assert the const `TARGETS` list has exactly two entries,
//! `lookup` finds the built-ins and rejects unknown names, and
//! `emit_for` surfaces `TargetNotFound` on miss.

use clause_codegen::{CodegenCtx, CodegenError, TargetRegistry};
use clause_resolve::Resolved;

#[test]
fn registry_has_two() {
    assert_eq!(TargetRegistry::TARGETS.len(), 2);
}

#[test]
fn registry_lookup_native() {
    let target = TargetRegistry::lookup("native").expect("native target present");
    assert_eq!(target.name(), "native");
}

#[test]
fn registry_lookup_jomini() {
    let target = TargetRegistry::lookup("jomini").expect("jomini target present");
    assert_eq!(target.name(), "jomini");
}

#[test]
fn registry_lookup_missing() {
    assert!(TargetRegistry::lookup("bogus").is_none());
}

#[test]
fn registry_emit_for_missing() {
    let resolved = Resolved::empty();
    let ctx = CodegenCtx::new(&resolved);
    let err = TargetRegistry::emit_for("bogus", &ctx).expect_err("missing target errors");
    assert!(matches!(err, CodegenError::TargetNotFound { .. }));
}
