//! Validator-registry tests.
//!
//! Assert the const `VALIDATORS` list has exactly ten entries,
//! `run_all` pushes nothing on an empty `Resolved`, the documented
//! ordering is preserved, and no names collide.

use std::collections::HashSet;

use arvo::USize;
use hilavitkutin_api::Len;
use hilavitkutin_api::sink::CountingSink;
use vehje_ir::Diagnostic;
use vehje_resolve::Resolved;
use vehje_typecheck::{ValidatorCtx, ValidatorRegistry};

#[test]
fn registry_has_ten() {
    assert_eq!(ValidatorRegistry::VALIDATORS.len(), 10);
}

#[test]
fn registry_run_all_empty_resolved() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    let mut sink = CountingSink::<Diagnostic>::new();
    ValidatorRegistry::run_all(&ctx, &mut sink);
    assert_eq!(sink.len(), USize(0));
}

#[test]
fn registry_contains_expected_names() {
    let names: Vec<&'static str> = ValidatorRegistry::VALIDATORS
        .iter()
        .map(|v| v.name())
        .collect();
    assert_eq!(names, vec![
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
    ]);
}

#[test]
fn registry_names_unique() {
    let mut seen = HashSet::new();
    for v in ValidatorRegistry::VALIDATORS {
        assert!(
            seen.insert(v.name()),
            "duplicate validator name: {}",
            v.name()
        );
    }
    assert_eq!(seen.len(), 10);
}
