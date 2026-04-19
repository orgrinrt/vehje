//! Diagnostic construction + severity ordering.

use clause_ir::{ByteOffset, Diagnostic, FileId, Severity, Span};

const DUMMY_SPAN: Span =
    Span::new(FileId(0), ByteOffset(0), ByteOffset(0));

#[test]
fn diagnostic_new_preserves_fields() {
    let d = Diagnostic::new(Severity::Warning, DUMMY_SPAN, "hello");
    assert_eq!(d.severity, Severity::Warning);
    assert_eq!(d.span, DUMMY_SPAN);
    assert_eq!(d.message, "hello");
    assert!(d.related.is_empty());
}

#[test]
fn diagnostic_error_helper_sets_severity() {
    let d = Diagnostic::error(DUMMY_SPAN, "boom");
    assert_eq!(d.severity, Severity::Error);
}

#[test]
fn severity_discriminants_follow_order() {
    // Error is the most severe. Discriminants ascend in declaration
    // order so a `u8::from` style cast is meaningful for sorting.
    assert_eq!(Severity::Error as u8, 0);
    assert_eq!(Severity::Warning as u8, 1);
    assert_eq!(Severity::Info as u8, 2);
    assert_eq!(Severity::Help as u8, 3);
}

#[test]
fn default_severity_is_error() {
    let s: Severity = Severity::default();
    assert_eq!(s, Severity::Error);
}

#[test]
fn diagnostic_with_related_labels() {
    static RELATED: &[Span] = &[
        Span::new(FileId(1), ByteOffset(0), ByteOffset(1)),
        Span::new(FileId(1), ByteOffset(2), ByteOffset(3)),
    ];
    let d = Diagnostic {
        severity: Severity::Info,
        span: DUMMY_SPAN,
        message: "with labels",
        related: RELATED,
    };
    assert_eq!(d.related.len(), 2);
}
