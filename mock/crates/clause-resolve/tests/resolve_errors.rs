//! ResolveError variant construction + field readback.

use clause_ir::{ByteOffset, FileId, Span};
use clause_resolve::ResolveError;

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end))
}

#[test]
fn unresolved_identifier_constructs() {
    let err = ResolveError::UnresolvedIdentifier {
        name: "foo".to_string(),
        span: span(3, 6),
    };
    match err {
        ResolveError::UnresolvedIdentifier { ref name, span: s } => {
            assert_eq!(name, "foo");
            assert_eq!(s, span(3, 6));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn duplicate_definition_constructs() {
    let err = ResolveError::DuplicateDefinition {
        name: "bar".to_string(),
        span: span(10, 13),
        prev_span: span(1, 4),
    };
    match err {
        ResolveError::DuplicateDefinition { ref name, span: s, prev_span: p } => {
            assert_eq!(name, "bar");
            assert_eq!(s, span(10, 13));
            assert_eq!(p, span(1, 4));
        }
        _ => panic!("wrong variant"),
    }
}
