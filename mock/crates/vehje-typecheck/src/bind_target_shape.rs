//! Bind-target-shape validator.
//!
//! Enforces target-registry entry format: every `CodegenTarget`
//! claimed by a bind directive must have a well-formed entry in
//! the target registry. Guards against typos and stale bind
//! tables.
//!
//! Skeleton round: `validate` returns `Vec::new()`. Real body is
//! BACKLOG (one follow-up micro-round).

use hilavitkutin_api::DiagnosticSink;
use vehje_ir::Diagnostic;

use crate::ctx::ValidatorCtx;
use crate::validator::Validator;

/// Bind-target-shape validator, target-registry entry-format
/// check.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty diagnostic vec; the real walk lands in a
/// dedicated follow-up round.
#[derive(Debug)]
pub struct BindTargetShape;

impl Validator for BindTargetShape {
    fn name(&self) -> &'static str {
        "bind-target-shape"
    }

    fn validate(&self, _ctx: &ValidatorCtx, _sink: &mut dyn DiagnosticSink<Diagnostic>) {}
}
