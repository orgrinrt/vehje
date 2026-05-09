//! Codegen execution context.
//!
//! `CodegenCtx` is the read-only carrier every target sees.
//! Skeleton round: it holds a reference to the `Resolved` bundle
//! and nothing else. Future rounds grow this into a richer
//! context (diagnostic sink for incremental emission, artefact
//! cache for incremental codegen, workspace info for cross-crate
//! lookups, per-target config read from `Vehje.toml`, …); all
//! of those are BACKLOG.

use vehje_resolve::Resolved;

/// Read-only context passed to every `CodegenTarget::emit` call.
///
/// Holds the resolver output. Targets receive the context plus two
/// caller-owned sinks (`&mut dyn ByteEmitter` and
/// `&mut dyn DiagnosticSink<Diagnostic>`); diagnostics push into the
/// sink, bytes push into the byte emitter, and the returned
/// `CodegenArtifact` carries only an `ArtifactKind`. A future
/// `ArtefactCache` for incremental codegen is BACKLOG.
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
