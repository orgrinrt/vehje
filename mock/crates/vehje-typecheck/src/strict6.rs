//! STRICT6, `ambiguous-imports`.
//!
//! Detects use-tree ambiguity (the same name resolving to two
//! different paths at the same scope). Coordinates with
//! vehje-resolve, which surfaces the candidate set. Ported from
//! the Python-era `strict6` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use hilavitkutin_api::DiagnosticSink;
use vehje_ir::Diagnostic;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT6 validator, use-tree ambiguity detection.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict6;

impl Validator for Strict6 {
    fn name(&self) -> &'static str {
        "ambiguous-imports"
    }

    fn validate(&self, _ctx: &ValidatorCtx, _sink: &mut dyn DiagnosticSink<Diagnostic>) {}
}
