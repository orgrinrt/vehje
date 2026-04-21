//! `CodegenTarget` trait.
//!
//! A `CodegenTarget` consumes the resolver output (via
//! `CodegenCtx`) and emits a `CodegenArtifact` carrying the
//! target's raw bytes plus any diagnostics surfaced during
//! emission. Targets are stored in the const
//! `TargetRegistry::TARGETS` list as `&'static dyn CodegenTarget`
//! trait objects, so the trait must be object-safe.
//!
//! Skeleton round: every shipped target's `emit` returns an
//! `Ok(CodegenArtifact::empty(kind))`. The R3 design
//! (2026-04-26) finalised a richer associated-const shape
//! (`const NAME`, `const VERSION`, `fn accepts(kind:
//! AstNodeKind) -> bool`, `fn stages() -> &'static [Stage]`,
//! `fn compile(unit, ctx) -> Result<CodegenOutput, Diagnostic>`,
//! `fn render_diagnostic(err: &TargetError) -> Diagnostic`).
//! That retrofit is BACKLOG; it lands once the first real
//! target backend surfaces the need.

use clause_ir::Diagnostic;
use hilavitkutin_api::{ByteEmitter, DiagnosticSink};
use notko::Outcome;

use crate::artifact::CodegenArtifact;
use crate::ctx::CodegenCtx;
use crate::error::CodegenError;

/// Codegen-target interface.
///
/// Implementors should be ZSTs (zero-sized types) — target state
/// belongs in `CodegenCtx`, not the trait object. `Sync` is
/// required so the registry's `&'static dyn CodegenTarget`
/// entries are thread-safe for future multi-target parallel
/// codegen (BACKLOG).
pub trait CodegenTarget: Sync {
    /// Canonical name used in manifest declarations + CLI
    /// `--target <name>` selection.
    ///
    /// Kebab-case or lowercase single word; stable across
    /// releases. Examples: `"native"`, `"jomini"`.
    fn name(&self) -> &'static str;

    /// Walk `ctx.resolved()` and emit a codegen artifact.
    ///
    /// Emitted bytes push into `bytes`; any diagnostics surfaced
    /// during emission push into `diagnostics`. The return value
    /// carries only the artifact kind discriminator.
    ///
    /// Skeleton implementations return
    /// `Outcome::Ok(CodegenArtifact::empty(kind))` without pushing
    /// anything; real bodies land in follow-up rounds (one per
    /// target).
    ///
    /// `bytes` and `diagnostics` are `&mut dyn` rather than `&mut
    /// impl` because the trait is stored in the const registry as
    /// `&'static dyn CodegenTarget`; dyn methods are object-safe
    /// and must not carry `impl Trait` parameters.
    ///
    /// # Implementor note
    ///
    /// `Diagnostic.message` is `&'static str`. Targets that surface
    /// diagnostics must use string literals or `const` slices — no
    /// `format!`-produced strings. Span-enriched rendering is the
    /// responsibility of the diagnostic renderer, not the target
    /// body.
    fn emit(
        &self,
        ctx: &CodegenCtx,
        bytes: &mut dyn ByteEmitter,
        diagnostics: &mut dyn DiagnosticSink<Diagnostic>,
    ) -> Outcome<CodegenArtifact, CodegenError>;
}
