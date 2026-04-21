//! ResolveError variant construction + field readback.

use clause_ir::{ByteOffset, FileId, Span};
use clause_resolve::ResolveError;
use hilavitkutin_str::str_const;

fn span(start: u32, end: u32) -> Span { // lint:allow(no-bare-numeric) reason: test-helper boundary — Span constructors take bare u32s until the substrate offers a typed-width span builder; tracked: #81
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end)) // lint:allow(no-bare-numeric) reason: FileId(0) sentinel for single-file tests; tracked: #81
}

#[test]
fn unresolved_identifier_constructs() {
    let err = ResolveError::UnresolvedIdentifier {
        name: str_const!("foo"),
        span: span(3, 6),
    };
    match err {
        ResolveError::UnresolvedIdentifier { name, span: s } => {
            assert_eq!(name, str_const!("foo"));
            assert_eq!(s, span(3, 6));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn duplicate_definition_constructs() {
    let err = ResolveError::DuplicateDefinition {
        name: str_const!("bar"),
        span: span(10, 13),
        prev_span: span(1, 4),
    };
    match err {
        ResolveError::DuplicateDefinition { name, span: s, prev_span: p } => {
            assert_eq!(name, str_const!("bar"));
            assert_eq!(s, span(10, 13));
            assert_eq!(p, span(1, 4));
        }
        _ => panic!("wrong variant"),
    }
}
