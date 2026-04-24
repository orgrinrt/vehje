//! End-to-end `typecheck` entry tests.
//!
//! Drive the top-level `typecheck(&Resolved, &mut sink)` over a
//! default `Resolved`. Skeleton round: always pushes nothing.

use arvo::USize;
use vehje_ir::Diagnostic;
use vehje_resolve::Resolved;
use vehje_typecheck::typecheck;
use hilavitkutin_api::{sink::CountingSink, Len};

#[test]
fn typecheck_empty_resolved_returns_empty() {
    let resolved = Resolved::empty();
    let mut sink = CountingSink::<Diagnostic>::new();
    typecheck(&resolved, &mut sink);
    assert_eq!(sink.len(), USize(0));
}

#[test]
fn typecheck_empty_then_runnable_twice() {
    let resolved = Resolved::empty();
    let mut sink = CountingSink::<Diagnostic>::new();
    typecheck(&resolved, &mut sink);
    typecheck(&resolved, &mut sink);
    assert_eq!(sink.len(), USize(0));
}
