//! STRICT4, `int-overflow`.
//!
//! Detects compile-time-detectable integer overflow in const-eval
//! + literal contexts. Ported from the Python-era `strict4` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use hilavitkutin_api::DiagnosticSink;
use vehje_ir::Diagnostic;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT4 validator, compile-time integer-overflow audit.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict4;

impl Validator for Strict4 {
    fn name(&self) -> &'static str {
        "int-overflow"
    }

    fn validate(&self, _ctx: &ValidatorCtx, _sink: &mut dyn DiagnosticSink<Diagnostic>) {}
}
