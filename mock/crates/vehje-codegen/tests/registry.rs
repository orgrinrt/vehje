//! Target-registry tests.
//!
//! Assert the const `TARGETS` list has exactly two entries,
//! `lookup` finds the built-ins and rejects unknown names, and
//! `emit_for` surfaces `TargetNotFound` on miss.

use arvo::USize;
use hilavitkutin_api::{BulkPush, Len, Push};
use vehje_codegen::{CodegenCtx, CodegenError, Diagnostic, TargetRegistry};
use vehje_resolve::Resolved;

#[derive(Default)]
struct ByteCounter {
    count: usize, // lint:allow(no-bare-numeric) reason: test-internal byte counter; tracked: #413
}

impl Push<u8> for ByteCounter {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matches ByteEmitter bound; tracked: #72
    fn push(&mut self, _b: u8) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matches Push<u8>; tracked: #72
        self.count += 1; // lint:allow(no-bare-numeric) reason: test-internal counter; tracked: #413
    }
}

impl BulkPush<u8> for ByteCounter {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matches ByteEmitter bound; tracked: #72
    fn push_bulk(&mut self, items: &[u8]) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matches BulkPush<u8>; tracked: #72
        self.count += items.len(); // lint:allow(no-bare-numeric) reason: test-internal counter; tracked: #413
    }
}

#[derive(Default)]
struct DiagCounter {
    count: usize, // lint:allow(no-bare-numeric) reason: test-internal diagnostic counter; tracked: #413
}

impl Push<Diagnostic> for DiagCounter {
    fn push(&mut self, _d: Diagnostic) {
        self.count += 1; // lint:allow(no-bare-numeric) reason: test-internal counter; tracked: #413
    }
}

impl Len for DiagCounter {
    fn len(&self) -> USize {
        USize(self.count)
    }
}

#[test]
fn registry_has_two() {
    assert_eq!(TargetRegistry::TARGETS.len(), 2); // lint:allow(no-bare-numeric) reason: count check; tracked: #413
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
    // `lookup` returns `notko::Maybe`, not `Option`.
    assert!(TargetRegistry::lookup("bogus").isnt());
}

#[test]
fn registry_emit_for_missing() {
    let resolved = Resolved::empty();
    let ctx = CodegenCtx::new(&resolved);
    let mut bytes = ByteCounter::default();
    let mut diags = DiagCounter::default();
    let err = TargetRegistry::emit_for("bogus", &ctx, &mut bytes, &mut diags)
        .expect_err("missing target errors");
    assert!(matches!(err, CodegenError::TargetNotFound { .. }));
}
