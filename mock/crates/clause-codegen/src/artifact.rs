//! Codegen artifact descriptor.
//!
//! `CodegenArtifact` is a post-emission descriptor: a kind
//! discriminator identifying what the emitted output represents.
//! Bytes and diagnostics flow through caller-provided sinks
//! (`ByteEmitter` and `DiagnosticSink<Diagnostic>`) during the
//! `CodegenTarget::emit` call rather than bundled into the
//! artifact itself.
//!
//! The R3 design finalised a richer `CodegenOutput { bytes,
//! manifest, references }` shape with a `ManifestFragment` for
//! distribution hints and `&[SymbolRef]` for cross-target symbol
//! references. That retrofit is BACKLOG; lands when the first
//! real target backend surfaces the need.

/// Artifact kind — what the emitted payload represents.
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

/// Codegen artifact descriptor — kind discriminator only.
///
/// Bytes and diagnostics flow through caller-provided sinks
/// during `CodegenTarget::emit`; the artifact reports only what
/// kind of output was emitted.
#[derive(Debug)]
pub struct CodegenArtifact {
    /// What the emitted payload represents.
    pub kind: ArtifactKind,
}

impl CodegenArtifact {
    /// Build an empty artifact descriptor of the given `kind`.
    ///
    /// Used by skeleton target stubs and by real target backends
    /// as the canonical return value after pushing bytes /
    /// diagnostics into the sinks their caller provided.
    pub fn empty(kind: ArtifactKind) -> Self {
        Self { kind }
    }
}
