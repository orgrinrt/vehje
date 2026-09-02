//! Parser diagnostic carrier.
//!
//! `SyntaxError` is `Copy` and alloc-free: it carries a static
//! message, a source span, and a kind discriminator. A `From`
//! conversion produces a `vehje_ir::Diagnostic` so the compiler
//! driver can merge syntax errors into its diagnostic pipeline
//! without re-keying the payload.
//!
//! The set of error kinds tracks what the parser can actually
//! produce today; follow-up rounds extend the enum as new grammar
//! productions learn to fail in new ways.

use vehje_ir::{DiagPhase, Diagnostic, Severity, Span};

/// Classification of a syntax error.
///
/// The skeleton parser only produces the three kinds listed here;
/// additional kinds (missing delimiter, invalid pattern, ambiguous
/// operator) arrive alongside the production that first emits them.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)] // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
pub enum SyntaxErrorKind {
    /// A token appeared where none of the active productions expect
    /// one.
    UnexpectedToken,
    /// The parser hit end of input while still expecting more
    /// tokens.
    UnexpectedEof,
    /// A literal token was present but its contents were malformed
    /// (e.g. integer out of range once literal parsing ships).
    InvalidLiteral,
}

impl Default for SyntaxErrorKind {
    fn default() -> Self {
        Self::UnexpectedToken
    }
}

/// A single syntax-level diagnostic.
///
/// `Copy`; cloning is free. `message` is `&'static str` so the
/// whole error costs nothing to pass around.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SyntaxError {
    pub kind:    SyntaxErrorKind,
    pub span:    Span,
    pub message: &'static str,
}

impl SyntaxError {
    /// Construct an `UnexpectedToken` error.
    pub const fn unexpected_token(span: Span, message: &'static str) -> Self {
        Self {
            kind: SyntaxErrorKind::UnexpectedToken,
            span,
            message,
        }
    }

    /// Construct an `UnexpectedEof` error.
    pub const fn unexpected_eof(span: Span, message: &'static str) -> Self {
        Self {
            kind: SyntaxErrorKind::UnexpectedEof,
            span,
            message,
        }
    }

    /// Construct an `InvalidLiteral` error.
    pub const fn invalid_literal(span: Span, message: &'static str) -> Self {
        Self {
            kind: SyntaxErrorKind::InvalidLiteral,
            span,
            message,
        }
    }
}

impl From<SyntaxError> for Diagnostic {
    fn from(err: SyntaxError) -> Self {
        Diagnostic::new(DiagPhase::Syntax, Severity::Error, err.span, err.message)
    }
}
