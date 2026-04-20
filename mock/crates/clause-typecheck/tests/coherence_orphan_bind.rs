//! Coherence / OrphanRule / BindTargetShape validator tests.
//!
//! Each asserts the validator reports its documented name and
//! returns an empty diagnostic vec on an empty `Resolved`.

use clause_resolve::Resolved;
use clause_typecheck::{BindTargetShape, Coherence, OrphanRule, Validator, ValidatorCtx};

#[test]
fn coherence_name_and_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    assert_eq!(Coherence.name(), "coherence");
    assert!(Coherence.validate(&ctx).is_empty());
}

#[test]
fn orphan_rule_name_and_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    assert_eq!(OrphanRule.name(), "orphan-rule");
    assert!(OrphanRule.validate(&ctx).is_empty());
}

#[test]
fn bind_target_shape_name_and_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    assert_eq!(BindTargetShape.name(), "bind-target-shape");
    assert!(BindTargetShape.validate(&ctx).is_empty());
}
