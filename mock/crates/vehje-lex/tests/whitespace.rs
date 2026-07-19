//! Whitespace (spaces, tabs, newlines) coalesces into trivia.

use vehje_ir::{FileId, TokenKind};
use vehje_lex::{Lexer, TriviaKind};

#[test]
fn leading_whitespace_is_trivia_on_first_token() {
    let mut lx = Lexer::from_str("   foo", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    assert_eq!(t.span.start.0, 3);
    assert!(
        t.trivia
            .leading()
            .iter()
            .any(|tr| tr.kind == TriviaKind::Whitespace),
        "expected whitespace trivia"
    );
}

#[test]
fn tabs_and_spaces_mix_into_single_run() {
    let mut lx = Lexer::from_str("\t \t foo", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    assert_eq!(t.span.start.0, 4);
    let leading = t.trivia.leading();
    assert_eq!(leading.len(), 1, "expected one run, got {:?}", leading);
}

#[test]
fn trailing_whitespace_on_same_line_is_trailing() {
    let mut lx = Lexer::from_str("foo   \nbar", FileId(0));
    let a = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::Ident);
    // Trailing picks up the three spaces plus the newline.
    assert!(
        a.trivia
            .trailing()
            .iter()
            .any(|tr| tr.kind == TriviaKind::Whitespace)
    );
}

#[test]
fn empty_input_emits_only_eof() {
    let mut lx = Lexer::from_str("", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Eof);
    assert!(lx.next().isnt());
}

#[test]
fn whitespace_only_input_emits_eof_with_leading_trivia() {
    let mut lx = Lexer::from_str("   \n\t", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Eof);
    assert!(!t.trivia.leading().is_empty());
}

#[test]
fn newline_between_tokens_ends_trailing_of_first() {
    // The first token gets the newline as trailing; the second
    // token's leading is empty.
    let mut lx = Lexer::from_str("foo\nbar", FileId(0));
    let a = lx.next().unwrap();
    let b = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::Ident);
    assert_eq!(b.kind, TokenKind::Ident);
    assert!(
        a.trivia
            .trailing()
            .iter()
            .any(|tr| tr.kind == TriviaKind::Whitespace)
    );
    assert!(b.trivia.leading().is_empty());
}
