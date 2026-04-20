//! `Validator` trait.
//!
//! A `Validator` inspects the resolver output (`Resolved`) and
//! emits `Diagnostic`s describing what it finds. Validators are
//! stored in the const `ValidatorRegistry::VALIDATORS` list as
//! `&'static dyn Validator` trait objects, so the trait must be
//! object-safe.
//!
//! Skeleton round: every shipped validator's `validate` returns
//! `Vec::new()`. The R4 design (2026-04-26) finalised a richer
//! associated-const shape (`const NAME`, `const CATEGORY`,
//! `fn accepts(kind: AstNodeKind) -> bool`, `&mut DiagnosticSink`
//! emission). That retrofit is BACKLOG; it lands once the first
//! real validator body surfaces the need.

use clause_ir::Diagnostic;

use crate::ctx::ValidatorCtx;

/// Validator interface.
///
/// Implementors should be ZSTs (zero-sized types) — validator
/// state belongs in `ValidatorCtx`, not the trait object. `Sync`
/// is required so the registry's `&'static dyn Validator`
/// entries are thread-safe for future rayon-backed execution
/// (BACKLOG).
pub trait Validator: Sync {
    /// Canonical name used in diagnostics + suppression filters.
    ///
    /// Kebab-case, stable across releases. Matches the Python-era
    /// names (`no-shadow`, `unused`, `assign-to-immut`, …).
    fn name(&self) -> &'static str;

    /// Walk `ctx.resolved()` and produce diagnostics.
    ///
    /// Skeleton implementations return `Vec::new()`; real bodies
    /// land in follow-up rounds (one per validator).
    fn validate(&self, ctx: &ValidatorCtx) -> Vec<Diagnostic>;
}
