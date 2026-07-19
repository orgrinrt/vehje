//! `NativeTarget`, Rust/LLVM codegen target.
//!
//! Placeholder for the future Rust / LLVM-IR / cranelift
//! backend. Ported-in-spirit from the Python-era native
//! compilation path.
//!
//! Skeleton round: `emit` returns an empty `Binary` artifact.
//! Real body is BACKLOG (one follow-up micro-round once the
//! target-agnostic IR lowering layer lands).

use hilavitkutin_api::{ByteEmitter, DiagnosticSink};
use notko::Outcome;
use vehje_ir::Diagnostic;

use crate::artifact::{ArtifactKind, CodegenArtifact};
use crate::ctx::CodegenCtx;
use crate::error::CodegenError;
use crate::target::CodegenTarget;

/// Native-target ZST, Rust / LLVM / cranelift backend.
///
/// ZST; carries no state. The implementation is a stub that
/// returns an empty `Binary` artifact; the real lowering walk
/// lands in a dedicated follow-up round on top of the IR
/// lowering layer.
#[derive(Debug)]
pub struct NativeTarget;

impl CodegenTarget for NativeTarget {
    fn name(&self) -> &'static str {
        "native"
    }

    fn emit(
        &self,
        _ctx: &CodegenCtx,
        _bytes: &mut dyn ByteEmitter,
        _diagnostics: &mut dyn DiagnosticSink<Diagnostic>,
    ) -> Outcome<CodegenArtifact, CodegenError> {
        Outcome::Ok(CodegenArtifact::empty(ArtifactKind::Binary))
    }
}
