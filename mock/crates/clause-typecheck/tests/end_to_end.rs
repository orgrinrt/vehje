//! End-to-end `typecheck` entry tests.
//!
//! Drive the top-level `typecheck(&Resolved) -> Vec<Diagnostic>`
//! over a default `Resolved`. Skeleton round: always returns
//! empty.

use clause_resolve::Resolved;
use clause_typecheck::typecheck;

#[test]
fn typecheck_empty_resolved_returns_empty() {
    let resolved = Resolved::empty();
    assert!(typecheck(&resolved).is_empty());
}

#[test]
fn typecheck_empty_then_runnable_twice() {
    let resolved = Resolved::empty();
    let first = typecheck(&resolved);
    let second = typecheck(&resolved);
    assert!(first.is_empty());
    assert!(second.is_empty());
}
