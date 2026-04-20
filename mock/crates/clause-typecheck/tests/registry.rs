//! Validator-registry tests.
//!
//! Assert the const `VALIDATORS` list has exactly ten entries,
//! `run_all` returns empty on an empty `Resolved`, the documented
//! ordering is preserved, and no names collide.

use std::collections::HashSet;

use clause_resolve::Resolved;
use clause_typecheck::{ValidatorCtx, ValidatorRegistry};

#[test]
fn registry_has_ten() {
    assert_eq!(ValidatorRegistry::VALIDATORS.len(), 10);
}

#[test]
fn registry_run_all_empty_resolved() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    assert!(ValidatorRegistry::run_all(&ctx).is_empty());
}

#[test]
fn registry_contains_expected_names() {
    let names: Vec<&'static str> =
        ValidatorRegistry::VALIDATORS.iter().map(|v| v.name()).collect();
    assert_eq!(
        names,
        vec![
            "no-shadow",
            "unused",
            "assign-to-immut",
            "int-overflow",
            "use-after-move",
            "ambiguous-imports",
            "clippy-batch",
            "coherence",
            "orphan-rule",
            "bind-target-shape",
        ]
    );
}

#[test]
fn registry_names_unique() {
    let mut seen = HashSet::new();
    for v in ValidatorRegistry::VALIDATORS {
        assert!(seen.insert(v.name()), "duplicate validator name: {}", v.name());
    }
    assert_eq!(seen.len(), 10);
}
