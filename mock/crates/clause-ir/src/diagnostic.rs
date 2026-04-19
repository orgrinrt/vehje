//! Diagnostic type shared across every compiler phase.
//!
//! `Diagnostic` is `Copy` and references only `'static` strings. This
//! keeps the lexer / parser / type-checker `no_std` and alloc-free; a
//! downstream renderer (in the `clause` binary) is responsible for
//! pairing each diagnostic with the span's source text and producing
//! the human-facing output.

use crate::span::Span;

/// Severity of a diagnostic.
///
/// Ordering is meaningful: `Error < Warning < Info < Help` in discriminant
/// value, but semantically `Error` is the most severe. Consumers that
/// need to compare severities should match on the variant.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum Severity {
    Error = 0,
    Warning = 1,
    Info = 2,
    Help = 3,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Error
    }
}

/// Structured diagnostic carrying severity, primary span, and static
/// message text plus a set of related spans (secondary labels).
///
/// Both `message` and `related` are `'static`; diagnostics never own
/// heap memory. Producers emit diagnostics into a caller-supplied
/// sink; this type intentionally does not prescribe the sink shape.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Span,
    pub message: &'static str,
    pub related: &'static [Span],
}

impl Diagnostic {
    /// Construct a diagnostic with no related labels.
    pub const fn new(severity: Severity, span: Span, message: &'static str) -> Self {
        Self { severity, span, message, related: &[] }
    }

    /// Construct an error-severity diagnostic with no related labels.
    pub const fn error(span: Span, message: &'static str) -> Self {
        Self::new(Severity::Error, span, message)
    }
}
