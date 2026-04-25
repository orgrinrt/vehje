//! Line + block comments attach as trivia, not as real tokens.

use vehje_ir::{FileId, TokenKind};
use vehje_lex::{Lexer, TriviaKind};

#[test]
fn line_comment_before_token_is_leading() {
    let mut lx = Lexer::from_str("// header\nfoo", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    let leading = t.trivia.leading();
    assert!(!leading.is_empty(), "expected leading trivia");
    // First piece is the line comment.
    assert!(
        leading.iter().any(|tr| tr.kind == TriviaKind::LineComment),
        "expected a LineComment in leading trivia"
    );
}

#[test]
fn block_comment_before_token_is_leading() {
    let mut lx = Lexer::from_str("/* a */ foo", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    assert!(
        t.trivia
            .leading()
            .iter()
            .any(|tr| tr.kind == TriviaKind::BlockComment),
        "expected a BlockComment in leading trivia"
    );
}

#[test]
fn line_comment_after_token_is_trailing() {
    let mut lx = Lexer::from_str("foo // trailing\nbar", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    assert!(
        t.trivia
            .trailing()
            .iter()
            .any(|tr| tr.kind == TriviaKind::LineComment),
        "expected a LineComment in trailing trivia"
    );
    // Next real token is `bar`.
    let t2 = lx.next().unwrap();
    assert_eq!(t2.kind, TokenKind::Ident);
}

#[test]
fn block_comment_between_two_tokens_on_same_line() {
    let mut lx = Lexer::from_str("foo /* c */ bar", FileId(0));
    let a = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::Ident);
    // Block comment on the same line becomes trailing of `foo`.
    assert!(
        a.trivia
            .trailing()
            .iter()
            .any(|tr| tr.kind == TriviaKind::BlockComment),
        "expected BlockComment in trailing"
    );
    let b = lx.next().unwrap();
    assert_eq!(b.kind, TokenKind::Ident);
}

#[test]
fn block_comment_is_not_nested() {
    // `/* /* */ */`, the non-nesting lexer closes on the first `*/`,
    // leaving ` */` as unrecognised trailing punctuation.
    let mut lx = Lexer::from_str("/* /* */ x", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    assert_eq!(t.span.start.0, 9);
}

#[test]
fn line_comment_runs_to_newline() {
    let mut lx = Lexer::from_str("// abc\nx", FileId(0));
    let t = lx.next().unwrap();
    assert_eq!(t.kind, TokenKind::Ident);
    // Leading trivia should cover `// abc` plus a whitespace (the
    // trailing newline).
    let leading = t.trivia.leading();
    assert!(leading.iter().any(|tr| tr.kind == TriviaKind::LineComment));
    assert!(leading.iter().any(|tr| tr.kind == TriviaKind::Whitespace));
}
