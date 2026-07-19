//! End-to-end `emit` entry tests.
//!
//! Drive the top-level `emit(&Resolved, &str, &mut bytes, &mut diags)
//! -> Outcome<CodegenArtifact, CodegenError>` over a default
//! `Resolved`. Skeleton round: `native` always returns an empty
//! `Binary` artifact, missing targets surface `TargetNotFound`.

use arvo::USize;
use hilavitkutin_api::{BulkPush, Len, Push};
use vehje_codegen::{ArtifactKind, CodegenError, Diagnostic, emit};
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
fn emit_native_empty_resolved() {
    let resolved = Resolved::empty();
    let mut bytes = ByteCounter::default();
    let mut diags = DiagCounter::default();
    let artifact = emit(&resolved, "native", &mut bytes, &mut diags).expect("native emit ok");
    assert_eq!(artifact.kind, ArtifactKind::Binary);
    assert_eq!(bytes.count, 0); // lint:allow(no-bare-numeric) reason: skeleton emits nothing; tracked: #413
    assert_eq!(diags.count, 0); // lint:allow(no-bare-numeric) reason: skeleton emits nothing; tracked: #413
}

#[test]
fn emit_missing_target() {
    let resolved = Resolved::empty();
    let mut bytes = ByteCounter::default();
    let mut diags = DiagCounter::default();
    let err = emit(&resolved, "bogus", &mut bytes, &mut diags).expect_err("missing target errors");
    assert!(matches!(err, CodegenError::TargetNotFound { .. }));
}
