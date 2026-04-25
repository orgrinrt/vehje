//! Validator execution context.
//!
//! `ValidatorCtx` is the read-only carrier every validator sees.
//! Skeleton round: it holds a reference to the `Resolved` bundle
//! and nothing else. Future rounds grow this into a richer
//! context (diagnostic sink for incremental emission, workspace
//! info for cross-crate lookups, per-validator config read from
//! `Vehje.toml`, …); all of those are BACKLOG.

use vehje_resolve::Resolved;

/// Read-only context passed to every `Validator::validate` call.
///
/// Holds the resolver output. Validators should not mutate
/// anything through the context, diagnostics come back via the
/// `Vec<Diagnostic>` return value. A mutable `DiagnosticSink` is
/// BACKLOG for once emission volume justifies it.
#[derive(Debug)]
pub struct ValidatorCtx<'a> {
    resolved: &'a Resolved,
}

impl<'a> ValidatorCtx<'a> {
    /// Construct a context around `resolved`.
    pub fn new(resolved: &'a Resolved) -> Self {
        Self { resolved }
    }

    /// Borrow the resolver output.
    pub fn resolved(&self) -> &Resolved {
        self.resolved
    }
}
