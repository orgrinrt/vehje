//! Coherence / OrphanRule / BindTargetShape validator tests.
//!
//! Each asserts the validator reports its documented name and pushes
//! nothing into a `CountingSink` when run over an empty `Resolved`.

use arvo::USize;
use hilavitkutin_api::Len;
use hilavitkutin_api::sink::CountingSink;
use vehje_ir::Diagnostic;
use vehje_resolve::Resolved;
use vehje_typecheck::{BindTargetShape, Coherence, OrphanRule, Validator, ValidatorCtx};

#[test]
fn coherence_name_and_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    let mut sink = CountingSink::<Diagnostic>::new();
    assert_eq!(Coherence.name(), "coherence");
    Coherence.validate(&ctx, &mut sink);
    assert_eq!(sink.len(), USize(0));
}

#[test]
fn orphan_rule_name_and_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    let mut sink = CountingSink::<Diagnostic>::new();
    assert_eq!(OrphanRule.name(), "orphan-rule");
    OrphanRule.validate(&ctx, &mut sink);
    assert_eq!(sink.len(), USize(0));
}

#[test]
fn bind_target_shape_name_and_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    let mut sink = CountingSink::<Diagnostic>::new();
    assert_eq!(BindTargetShape.name(), "bind-target-shape");
    BindTargetShape.validate(&ctx, &mut sink);
    assert_eq!(sink.len(), USize(0));
}
