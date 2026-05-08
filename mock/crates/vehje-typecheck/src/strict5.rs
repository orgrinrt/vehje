//! STRICT5, `use-after-move`.
//!
//! Enforces move-semantics invariants: references to bindings
//! that have been moved must error. Ported from the Python-era
//! `strict5` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use vehje_ir::Diagnostic;
use hilavitkutin_api::DiagnosticSink;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT5 validator, move-semantics invariant check.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict5;

impl Validator for Strict5 {
    fn name(&self) -> &'static str {
        "use-after-move"
    }

    fn validate(
        &self,
        _ctx: &ValidatorCtx,
        _sink: &mut dyn DiagnosticSink<Diagnostic>,
    ) {
    }
}
