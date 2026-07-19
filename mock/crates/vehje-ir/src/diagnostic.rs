//! Diagnostic type shared across the framework's phases.
//!
//! `Diagnostic` is `Copy` and references only `'static` strings, so the
//! passes stay `no_std` and alloc-free. A host renderer pairs each
//! diagnostic with the span's source text and produces the human-facing
//! output.

use crate::span::Span;

/// Severity of a diagnostic.
///
/// `Error` is the most severe. Consumers match on the variant rather than
/// comparing discriminants.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Severity {
    /// A hard error: emission cannot proceed.
    #[default]
    Error,
    /// A warning: emission proceeds, the author should look.
    Warning,
    /// Informational.
    Info,
    /// A hint toward a fix.
    Help,
}

/// The framework phase that produced a diagnostic.
///
/// Lets a renderer or filter route by phase without parsing the static
/// message. The phases are the framework's own, not any one consumer's
/// compiler stages.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Phase {
    /// Name resolution over the Core binders.
    #[default]
    Resolve,
    /// Checking the resolved Core IR.
    Check,
    /// Emission through a target.
    Emit,
    /// The runtime side, for diagnostics surfacing across the ABI.
    Runtime,
}

/// Structured diagnostic carrying phase, severity, primary span, static
/// message, and related spans (secondary labels).
///
/// Both `message` and `related` are `'static`; diagnostics own no heap.
/// Producers emit into a caller-supplied sink; this type does not
/// prescribe the sink shape.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Diagnostic {
    pub phase: Phase,
    pub severity: Severity,
    pub span: Span,
    pub message: &'static str,
    pub related: &'static [Span],
}

impl Diagnostic {
    /// Construct a diagnostic with no related labels.
    pub const fn new(phase: Phase, severity: Severity, span: Span, message: &'static str) -> Self {
        Self { phase, severity, span, message, related: &[] }
    }

    /// Construct an error-severity diagnostic with no related labels.
    pub const fn error(phase: Phase, span: Span, message: &'static str) -> Self {
        Self::new(phase, Severity::Error, span, message)
    }
}
