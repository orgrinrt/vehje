//! STRICT2 — `unused`.
//!
//! Detects dead bindings — declared but never read / referenced.
//! Ported from the Python-era `strict2` lint.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use clause_ir::Diagnostic;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// STRICT2 validator — dead-binding detection.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct Strict2;

impl Validator for Strict2 {
    fn name(&self) -> &'static str {
        "unused"
    }

    fn validate(&self, _ctx: &ValidatorCtx) -> Vec<Diagnostic> {
        Vec::new()
    }
}
