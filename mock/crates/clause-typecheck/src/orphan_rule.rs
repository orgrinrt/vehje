//! Orphan-rule validator.
//!
//! Enforces the orphan rule: an `impl Trait for Type` is only
//! legal if either the trait or the type is local to the current
//! crate. Keeps downstream coherence tractable.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use clause_ir::Diagnostic;
use hilavitkutin_api::DiagnosticSink;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// Orphan-rule validator — foreign-type / foreign-trait impl
/// guard.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct OrphanRule;

impl Validator for OrphanRule {
    fn name(&self) -> &'static str {
        "orphan-rule"
    }

    fn validate(
        &self,
        _ctx: &ValidatorCtx,
        _sink: &mut dyn DiagnosticSink<Diagnostic>,
    ) {
    }
}
