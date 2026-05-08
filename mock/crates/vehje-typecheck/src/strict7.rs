//! STRICT7, `clippy-batch`.
//!
//! A curated batch of style / bug-prone patterns (named after
//! Python's clippy inspiration). Ported from the Python-era
//! `strict7` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use vehje_ir::Diagnostic;
use hilavitkutin_api::DiagnosticSink;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT7 validator, curated style / bug-prone pattern batch.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict7;

impl Validator for Strict7 {
    fn name(&self) -> &'static str {
        "clippy-batch"
    }

    fn validate(
        &self,
        _ctx: &ValidatorCtx,
        _sink: &mut dyn DiagnosticSink<Diagnostic>,
    ) {
    }
}
