//! `TargetRegistry` — const-sized list of all shipped codegen
//! targets plus `lookup` + `emit_for` helpers.
//!
//! The registry is a ZST with an associated `TARGETS` slice and
//! associated `lookup` / `emit_for` functions. The slice holds
//! `&'static dyn CodegenTarget` entries — this is the single
//! sanctioned `dyn` exception in vehje-codegen per R3 DESIGN,
//! mirroring the `ValidatorRegistry` pattern in vehje-typecheck.
//! It's confined to the registry iteration surface and uses
//! `'static` lifetimes only.
//!
//! Dynamic registration (a `register` method for plugin targets,
//! e.g. for `vehje-jomini` loaded via `dlopen`) is BACKLOG —
//! the skeleton round ships only the two built-in targets.

use vehje_ir::Diagnostic;
use hilavitkutin_api::{ByteEmitter, DiagnosticSink};
use notko::{Maybe, Outcome};

use crate::artifact::CodegenArtifact;
use crate::ctx::CodegenCtx;
use crate::error::CodegenError;
use crate::jomini::JominiTarget;
use crate::native::NativeTarget;
use crate::target::CodegenTarget;
use vehje_resolve::Resolved;

/// Target registry — const iteration surface over every
/// codegen target shipped in vehje-codegen.
///
/// Carries no state; all functionality is associated. Plugin
/// targets will gain a dynamic registration path in a future
/// round (BACKLOG).
#[derive(Debug)]
pub struct TargetRegistry;

impl TargetRegistry {
    /// The two built-in targets, in the documented order:
    /// `NativeTarget`, then `JominiTarget`.
    ///
    /// Consumers should not depend on ordering, but the test
    /// suite asserts membership for this round to catch
    /// accidental drops.
    pub const TARGETS: &'static [&'static dyn CodegenTarget] = &[&NativeTarget, &JominiTarget];

    /// Look up a target by name. Linear scan over `TARGETS`.
    ///
    /// Returns `Maybe::Isnt` if no target matches. The scan is
    /// linear but the list is tiny (two entries this round; at
    /// most a handful even after plugin loading); a hashmap is not
    /// justified.
    pub fn lookup(name: &str) -> Maybe<&'static dyn CodegenTarget> {
        for target in Self::TARGETS {
            if target.name() == name {
                return Maybe::Is(*target);
            }
        }
        Maybe::Isnt
    }

    /// Emit an artifact via the target registered under `name`,
    /// pushing bytes into `bytes` and diagnostics into
    /// `diagnostics`.
    ///
    /// Returns `Outcome::Err(CodegenError::TargetNotFound { name:
    /// "" })` if no target matches. The empty-name sentinel lets
    /// `CodegenError` remain `Copy`; the caller retains the
    /// input-side string context and re-reports if needed. A
    /// follow-up round retrofits a richer error carrier if the
    /// sentinel surfaces as painful in practice.
    pub fn emit_for(
        name: &str,
        ctx: &CodegenCtx,
        bytes: &mut dyn ByteEmitter,
        diagnostics: &mut dyn DiagnosticSink<Diagnostic>,
    ) -> Outcome<CodegenArtifact, CodegenError> {
        match Self::lookup(name) {
            Maybe::Is(target) => target.emit(ctx, bytes, diagnostics),
            Maybe::Isnt => Outcome::Err(CodegenError::TargetNotFound { name: "" }),
        }
    }
}

/// Emit a codegen artifact for `resolved` via the target
/// registered under `target_name`, pushing emitted bytes into
/// `bytes` and diagnostics into `diagnostics`.
///
/// Returns `Outcome::Err(CodegenError::TargetNotFound { name: ""
/// })` if no target matches (the caller retains the input-side
/// name context).
///
/// Skeleton round: each built-in target returns an artifact
/// descriptor without pushing anything. Each deferred backend
/// flips its stub into a real lowering path in its own follow-up
/// round.
///
/// # Caller obligation
///
/// `TargetNotFound`'s `name` field is a sentinel (`""`) for
/// efficiency — `CodegenError` is `Copy` and can't own a runtime
/// string. Callers who need to render `"target not found: {name}"`
/// should hold the `target_name` they passed in and interpolate
/// at render time.
pub fn emit(
    resolved: &Resolved,
    target_name: &str,
    bytes: &mut dyn ByteEmitter,
    diagnostics: &mut dyn DiagnosticSink<Diagnostic>,
) -> Outcome<CodegenArtifact, CodegenError> {
    let ctx = CodegenCtx::new(resolved);
    TargetRegistry::emit_for(target_name, &ctx, bytes, diagnostics)
}
