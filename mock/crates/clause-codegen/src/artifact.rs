//! Codegen artifact payload.
//!
//! `CodegenArtifact` is the per-target output bundle: a
//! discriminator (`ArtifactKind`) identifying what the bytes
//! represent, the raw byte payload, and any diagnostics
//! surfaced during emission.
//!
//! Skeleton round: the payload is flat. The R3 design finalised
//! a richer `CodegenOutput { bytes, manifest, references }`
//! shape with a `ManifestFragment` for distribution hints and
//! `&[SymbolRef]` for cross-target symbol references. That
//! retrofit is BACKLOG; lands when the first real target
//! backend surfaces the need.

use clause_ir::Diagnostic;

/// Artifact kind — what the `CodegenArtifact.bytes` payload
/// represents.
///
/// Covers the common codegen output shapes: raw binaries,
/// relocatable object files, assembly text, generic source
/// output (for script-target backends like `jomini`), and
/// configuration files (for targets that emit side-band
/// settings alongside the primary artifact).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum ArtifactKind {
    /// Raw executable / linked binary output.
    Binary = 0,
    /// Relocatable object file (ELF / Mach-O / COFF).
    Object = 1,
    /// Target-specific assembly text.
    Assembly = 2,
    /// Source-language output (e.g. Clausewitz script from the
    /// future `jomini` backend).
    SourceFile = 3,
    /// Configuration file output (e.g. distribution manifest).
    Config = 4,
}

/// Codegen artifact — bytes + diagnostics + kind discriminator.
///
/// The skeleton round's `empty` constructor gives zero-byte
/// payload + zero diagnostics; real target backends populate
/// `bytes` with the emitted code and `diagnostics` with any
/// warnings / errors surfaced during emission.
#[derive(Debug)]
pub struct CodegenArtifact {
    /// What the `bytes` payload represents.
    pub kind: ArtifactKind,
    /// Emitted artifact payload bytes.
    // lint:allow(bare_collection) — the artifact payload byte surface across every codegen target matches what the R3 CodegenOutput shape uses directly; storage-crate collection types target mockspace domain graphs not host-side compiler artifact payloads here
    pub bytes: Vec<u8>,
    /// Diagnostics produced during emission. Empty on the
    /// happy path; populated when a target wants to surface
    /// warnings or non-fatal info alongside a successful
    /// emission.
    // lint:allow(bare_collection) — the diagnostic return surface across every compiler phase crate matches what clause-resolve and clause-typecheck already ship; storage-crate collection types target mockspace domain graphs not host-side compiler diagnostics here
    pub diagnostics: Vec<Diagnostic>,
}

impl CodegenArtifact {
    /// Build an empty artifact of the given `kind`.
    ///
    /// Zero-byte payload, no diagnostics. Used by skeleton
    /// target stubs and by real target backends as an initial
    /// value they then populate.
    pub fn empty(kind: ArtifactKind) -> Self {
        Self { kind, bytes: Vec::new(), diagnostics: Vec::new() }
    }
}
