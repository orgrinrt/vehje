//! Per-target smoke tests.
//!
//! Asserts each built-in target exposes the documented name and
//! returns an empty artifact from its skeleton `emit`.

use arvo::USize;
use hilavitkutin_api::{BulkPush, Len, Push};
use vehje_codegen::{CodegenCtx, CodegenTarget, Diagnostic, JominiTarget, NativeTarget};
use vehje_resolve::Resolved;

/// Bounded counting byte sink for `ByteEmitter` (= `Push<u8> + BulkPush<u8>`).
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

/// Bounded counting diagnostic sink for `DiagnosticSink<Diagnostic>`.
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
    let mut bytes = ByteCounter::default();
    let mut diags = DiagCounter::default();
    let _artifact = NativeTarget
        .emit(&ctx, &mut bytes, &mut diags)
        .expect("native emit ok");
    assert_eq!(bytes.count, 0); // lint:allow(no-bare-numeric) reason: skeleton emits nothing; tracked: #413
    assert_eq!(diags.count, 0); // lint:allow(no-bare-numeric) reason: skeleton emits nothing; tracked: #413
}

#[test]
fn jomini_emit_empty_artifact() {
    let resolved = Resolved::empty();
    let ctx = CodegenCtx::new(&resolved);
    let mut bytes = ByteCounter::default();
    let mut diags = DiagCounter::default();
    let _artifact = JominiTarget
        .emit(&ctx, &mut bytes, &mut diags)
        .expect("jomini emit ok");
    assert_eq!(bytes.count, 0); // lint:allow(no-bare-numeric) reason: skeleton emits nothing; tracked: #413
    assert_eq!(diags.count, 0); // lint:allow(no-bare-numeric) reason: skeleton emits nothing; tracked: #413
}
