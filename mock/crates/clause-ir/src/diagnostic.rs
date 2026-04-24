//! Diagnostic type shared across every compiler phase.
//!
//! `Diagnostic` is `Copy` and references only `'static` strings. This
//! keeps the lexer / parser / type-checker `no_std` and alloc-free; a
//! downstream renderer (in the `vehje` binary) is responsible for
//! pairing each diagnostic with the span's source text and producing
//! the human-facing output.
//!
//! Every `Diagnostic` carries a `DiagPhase` discriminant naming the
//! compiler phase that produced it. The renderer can filter / route
//! by phase without having to parse the static `message` string.

use crate::span::Span;

/// Severity of a diagnostic.
///
/// Ordering is meaningful: `Error < Warning < Info < Help` in discriminant
/// value, but semantically `Error` is the most severe. Consumers that
/// need to compare severities should match on the variant.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
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

/// Compiler phase that produced a diagnostic.
///
/// Lets a diagnostic consumer (renderer, sink, filter) route by phase
/// without parsing the static `message` string. The variants map 1:1
/// onto the shipped phase crates: `vehje-lex`, `vehje-syntax`,
/// `vehje-resolve`, `vehje-typecheck`, `vehje-codegen`, and the
/// runtime / ABI shim for diagnostics that surface from the Zig
/// side through `vehje-runtime-driver`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
pub enum DiagPhase {
    Lex = 0,
    Syntax = 1,
    Resolve = 2,
    Typecheck = 3,
    Codegen = 4,
    Runtime = 5,
}

/// Structured diagnostic carrying phase, severity, primary span, and
/// static message text plus a set of related spans (secondary labels).
///
/// Both `message` and `related` are `'static`; diagnostics never own
/// heap memory. Producers emit diagnostics into a caller-supplied
/// sink; this type intentionally does not prescribe the sink shape.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Diagnostic {
    pub phase: DiagPhase,
    pub severity: Severity,
    pub span: Span,
    pub message: &'static str,
    pub related: &'static [Span],
}

impl Diagnostic {
    /// Construct a diagnostic with no related labels.
    pub const fn new(
        phase: DiagPhase,
        severity: Severity,
        span: Span,
        message: &'static str,
    ) -> Self {
        Self { phase, severity, span, message, related: &[] }
    }

    /// Construct an error-severity diagnostic with no related labels.
    pub const fn error(
        phase: DiagPhase,
        span: Span,
        message: &'static str,
    ) -> Self {
        Self::new(phase, Severity::Error, span, message)
    }
}
