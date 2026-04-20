//! Codegen execution context.
//!
//! `CodegenCtx` is the read-only carrier every target sees.
//! Skeleton round: it holds a reference to the `Resolved` bundle
//! and nothing else. Future rounds grow this into a richer
//! context (diagnostic sink for incremental emission, artefact
//! cache for incremental codegen, workspace info for cross-crate
//! lookups, per-target config read from `Clause.toml`, …); all
//! of those are BACKLOG.

use clause_resolve::Resolved;

/// Read-only context passed to every `CodegenTarget::emit` call.
///
/// Holds the resolver output. Targets should not mutate anything
/// through the context — diagnostics come back via the
/// `CodegenArtifact.diagnostics` field on the return value. A
/// mutable `DiagnosticSink` + `ArtefactCache` pair is BACKLOG
/// for once emission volume justifies it.
#[derive(Debug)]
pub struct CodegenCtx<'a> {
    resolved: &'a Resolved,
}

impl<'a> CodegenCtx<'a> {
    /// Construct a context around `resolved`.
    pub fn new(resolved: &'a Resolved) -> Self {
        Self { resolved }
    }

    /// Borrow the resolver output.
    pub fn resolved(&self) -> &Resolved {
        self.resolved
    }
}
