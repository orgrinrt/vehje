//! Trait-impl coherence validator.
//!
//! Enforces trait coherence: no two impls of the same trait for
//! the same type (pair-wise overlap detection across the impl
//! graph).
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use hilavitkutin_api::DiagnosticSink;
use vehje_ir::Diagnostic;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// Coherence validator, trait-impl overlap detection.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Coherence;

impl Validator for Coherence {
    fn name(&self) -> &'static str {
        "coherence"
    }

    fn validate(&self, _ctx: &ValidatorCtx, _sink: &mut dyn DiagnosticSink<Diagnostic>) {}
}
