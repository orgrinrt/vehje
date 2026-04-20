//! Resolver and manifest error carriers.
//!
//! `ResolveError` is the diagnostic surface for name-resolution
//! failures. Skeleton round carries two variants
//! (`UnresolvedIdentifier`, `DuplicateDefinition`); additional
//! variants arrive with the resolution rules that first emit
//! them. `ManifestError` carries the two shapes the stub parser
//! can produce (`Parse`, `NotImplemented`); real TOML parsing
//! will extend it.
//!
//! `From<ResolveError> for Diagnostic` emits an
//! `Severity::Error` diagnostic with a static placeholder message
//! per variant. Per-variant rendering lands alongside the rule
//! that produces the error.

use clause_ir::{DiagPhase, Diagnostic, Severity, Span};

/// Name-resolution failure.
///
/// The skeleton produces no `ResolveError` directly; the variants
/// exist so follow-up rounds can construct and propagate them as
/// soon as their rule lands.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ResolveError {
    /// A reference could not be resolved to any in-scope symbol.
    UnresolvedIdentifier {
        /// The referenced name.
        name: String,
        /// Span of the reference.
        span: Span,
    },
    /// The same name was declared twice in the same scope.
    DuplicateDefinition {
        /// The declared name.
        name: String,
        /// Span of the duplicate declaration.
        span: Span,
        /// Span of the previous declaration.
        prev_span: Span,
    },
}

impl ResolveError {
    /// Primary span of this error.
    pub fn span(&self) -> Span {
        match self {
            Self::UnresolvedIdentifier { span, .. } => *span,
            Self::DuplicateDefinition { span, .. } => *span,
        }
    }
}

impl From<ResolveError> for Diagnostic {
    fn from(err: ResolveError) -> Self {
        let span = err.span();
        let message: &'static str = match err {
            ResolveError::UnresolvedIdentifier { .. } => "unresolved identifier",
            ResolveError::DuplicateDefinition { .. } => "duplicate definition",
        };
        Diagnostic::new(DiagPhase::Resolve, Severity::Error, span, message)
    }
}

/// Manifest parsing failure.
///
/// `Parse` is reserved for real parser errors once TOML parsing
/// lands. `NotImplemented` is the sentinel the skeleton stub
/// returns for any non-empty input.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ManifestError {
    /// Structural parse failure (reserved for the real parser).
    Parse {
        /// Static error message.
        message: &'static str,
    },
    /// The stub parser does not handle non-empty input yet.
    NotImplemented,
}
