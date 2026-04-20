//! Span construction + equality.

use clause_ir::{ByteOffset, FileId, Span};

#[test]
fn span_new_preserves_components() {
    let span = Span::new(FileId(7), ByteOffset(3), ByteOffset(9));
    assert_eq!(span.file, FileId(7));
    assert_eq!(span.start, ByteOffset(3));
    assert_eq!(span.end, ByteOffset(9));
}

#[test]
fn span_len_and_is_empty() {
    let empty = Span::new(FileId(0), ByteOffset(4), ByteOffset(4));
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);

    let one = Span::new(FileId(0), ByteOffset(4), ByteOffset(10));
    assert!(!one.is_empty());
    assert_eq!(one.len(), 6);
}

#[test]
fn span_equality_is_structural() {
    let a = Span::new(FileId(1), ByteOffset(0), ByteOffset(5));
    let b = Span::new(FileId(1), ByteOffset(0), ByteOffset(5));
    let c = Span::new(FileId(2), ByteOffset(0), ByteOffset(5));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn default_span_is_zeroed() {
    let d: Span = Span::default();
    assert_eq!(d.file, FileId(0));
    assert_eq!(d.start, ByteOffset(0));
    assert_eq!(d.end, ByteOffset(0));
    assert!(d.is_empty());
}

#[test]
fn newtype_reprs_are_copy() {
    // Compile-time: FileId / ByteOffset must be `Copy`.
    let f = FileId(1);
    let _f2 = f;
    let _f3 = f;

    let b = ByteOffset(2);
    let _b2 = b;
    let _b3 = b;
}
