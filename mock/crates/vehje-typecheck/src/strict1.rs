//! STRICT1, `no-shadow`.
//!
//! Detects variable shadowing across scopes. Ported from the
//! Python-era `strict1` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use hilavitkutin_api::DiagnosticSink;
use vehje_ir::Diagnostic;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT1 validator, variable-shadow detection.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict1;

impl Validator for Strict1 {
    fn name(&self) -> &'static str {
        "no-shadow"
    }

    fn validate(&self, _ctx: &ValidatorCtx, _sink: &mut dyn DiagnosticSink<Diagnostic>) {}
}
