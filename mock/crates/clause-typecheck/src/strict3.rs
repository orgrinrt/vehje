//! STRICT3 — `assign-to-immut`.
//!
//! Detects assignment to bindings declared immutable. Ported from
//! the Python-era `strict3` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use vehje_ir::Diagnostic;
use hilavitkutin_api::DiagnosticSink;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT3 validator — immutable-binding reassignment.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict3;

impl Validator for Strict3 {
    fn name(&self) -> &'static str {
        "assign-to-immut"
    }

    fn validate(
        &self,
        _ctx: &ValidatorCtx,
        _sink: &mut dyn DiagnosticSink<Diagnostic>,
    ) {
    }
}
