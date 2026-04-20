//! `TargetRegistry` — const-sized list of all shipped codegen
//! targets plus `lookup` + `emit_for` helpers.
//!
//! The registry is a ZST with an associated `TARGETS` slice and
//! associated `lookup` / `emit_for` functions. The slice holds
//! `&'static dyn CodegenTarget` entries — this is the single
//! sanctioned `dyn` exception in clause-codegen per R3 DESIGN,
//! mirroring the `ValidatorRegistry` pattern in clause-typecheck.
//! It's confined to the registry iteration surface and uses
//! `'static` lifetimes only.
//!
//! Dynamic registration (a `register` method for plugin targets,
//! e.g. for `clause-jomini` loaded via `dlopen`) is BACKLOG —
//! the skeleton round ships only the two built-in targets.

use crate::artifact::CodegenArtifact;
use crate::ctx::CodegenCtx;
use crate::error::CodegenError;
use crate::jomini::JominiTarget;
use crate::native::NativeTarget;
use crate::target::CodegenTarget;

/// Target registry — const iteration surface over every
/// codegen target shipped in clause-codegen.
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
    /// Returns `None` if no target matches. The scan is linear
    /// but the list is tiny (two entries this round; at most a
    /// handful even after plugin loading); a hashmap is not
    /// justified.
    pub fn lookup(name: &str) -> Option<&'static dyn CodegenTarget> {
        for target in Self::TARGETS {
            if target.name() == name {
                return Some(*target);
            }
        }
        None
    }

    /// Emit an artifact via the target registered under `name`.
    ///
    /// Returns `Err(CodegenError::TargetNotFound { name: "" })`
    /// if no target matches. The empty-name sentinel lets
    /// `CodegenError` remain `Copy`; the caller retains the
    /// input-side string context and re-reports if needed. A
    /// follow-up round retrofits a richer error carrier if the
    /// sentinel surfaces as painful in practice.
    pub fn emit_for(name: &str, ctx: &CodegenCtx) -> Result<CodegenArtifact, CodegenError> {
        match Self::lookup(name) {
            Some(target) => target.emit(ctx),
            None => Err(CodegenError::TargetNotFound { name: "" }),
        }
    }
}
