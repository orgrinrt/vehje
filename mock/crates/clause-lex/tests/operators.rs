//! Longest-match operator and punctuation recognition.

use vehje_ir::{FileId, TokenKind};
use vehje_lex::Lexer;

fn kinds(src: &str) -> Vec<TokenKind> {
    let mut lx = Lexer::from_str(src, FileId(0));
    let mut out = Vec::new();
    while let Some(t) = lx.next() {
        out.push(t.kind);
    }
    out
}

#[test]
fn single_char_operators() {
    assert_eq!(kinds("+"), vec![TokenKind::Plus, TokenKind::Eof]);
    assert_eq!(kinds("-"), vec![TokenKind::Minus, TokenKind::Eof]);
    assert_eq!(kinds("*"), vec![TokenKind::Star, TokenKind::Eof]);
    assert_eq!(kinds("/"), vec![TokenKind::Slash, TokenKind::Eof]);
    assert_eq!(kinds("%"), vec![TokenKind::Percent, TokenKind::Eof]);
    assert_eq!(kinds("="), vec![TokenKind::Eq, TokenKind::Eof]);
    assert_eq!(kinds("<"), vec![TokenKind::Lt, TokenKind::Eof]);
    assert_eq!(kinds(">"), vec![TokenKind::Gt, TokenKind::Eof]);
    assert_eq!(kinds("!"), vec![TokenKind::Bang, TokenKind::Eof]);
    assert_eq!(kinds("&"), vec![TokenKind::And, TokenKind::Eof]);
    assert_eq!(kinds("|"), vec![TokenKind::Or, TokenKind::Eof]);
    assert_eq!(kinds("^"), vec![TokenKind::Caret, TokenKind::Eof]);
    assert_eq!(kinds("~"), vec![TokenKind::Tilde, TokenKind::Eof]);
    assert_eq!(kinds("?"), vec![TokenKind::Question, TokenKind::Eof]);
    assert_eq!(kinds("@"), vec![TokenKind::At, TokenKind::Eof]);
    assert_eq!(kinds("#"), vec![TokenKind::Pound, TokenKind::Eof]);
    assert_eq!(kinds("$"), vec![TokenKind::Dollar, TokenKind::Eof]);
}

#[test]
fn punctuation() {
    assert_eq!(kinds(","), vec![TokenKind::Comma, TokenKind::Eof]);
    assert_eq!(kinds(";"), vec![TokenKind::Semi, TokenKind::Eof]);
    assert_eq!(kinds(":"), vec![TokenKind::Colon, TokenKind::Eof]);
    assert_eq!(kinds("."), vec![TokenKind::Dot, TokenKind::Eof]);
    assert_eq!(kinds("{"), vec![TokenKind::LBrace, TokenKind::Eof]);
    assert_eq!(kinds("}"), vec![TokenKind::RBrace, TokenKind::Eof]);
    assert_eq!(kinds("("), vec![TokenKind::LParen, TokenKind::Eof]);
    assert_eq!(kinds(")"), vec![TokenKind::RParen, TokenKind::Eof]);
    assert_eq!(kinds("["), vec![TokenKind::LBracket, TokenKind::Eof]);
    assert_eq!(kinds("]"), vec![TokenKind::RBracket, TokenKind::Eof]);
}

#[test]
fn two_char_operators() {
    assert_eq!(kinds("::"), vec![TokenKind::ColonColon, TokenKind::Eof]);
    assert_eq!(kinds("->"), vec![TokenKind::Arrow, TokenKind::Eof]);
    assert_eq!(kinds("=>"), vec![TokenKind::FatArrow, TokenKind::Eof]);
    assert_eq!(kinds(".."), vec![TokenKind::DotDot, TokenKind::Eof]);
    assert_eq!(kinds("=="), vec![TokenKind::EqEq, TokenKind::Eof]);
    assert_eq!(kinds("!="), vec![TokenKind::NotEq, TokenKind::Eof]);
    assert_eq!(kinds("<="), vec![TokenKind::LtEq, TokenKind::Eof]);
    assert_eq!(kinds(">="), vec![TokenKind::GtEq, TokenKind::Eof]);
    assert_eq!(kinds("&&"), vec![TokenKind::AndAnd, TokenKind::Eof]);
    assert_eq!(kinds("||"), vec![TokenKind::OrOr, TokenKind::Eof]);
    assert_eq!(kinds("<<"), vec![TokenKind::Shl, TokenKind::Eof]);
    assert_eq!(kinds(">>"), vec![TokenKind::Shr, TokenKind::Eof]);
    assert_eq!(kinds("+="), vec![TokenKind::PlusEq, TokenKind::Eof]);
    assert_eq!(kinds("-="), vec![TokenKind::MinusEq, TokenKind::Eof]);
    assert_eq!(kinds("*="), vec![TokenKind::StarEq, TokenKind::Eof]);
    assert_eq!(kinds("/="), vec![TokenKind::SlashEq, TokenKind::Eof]);
    assert_eq!(kinds("%="), vec![TokenKind::PercentEq, TokenKind::Eof]);
    assert_eq!(kinds("&="), vec![TokenKind::AndEq, TokenKind::Eof]);
    assert_eq!(kinds("|="), vec![TokenKind::OrEq, TokenKind::Eof]);
    assert_eq!(kinds("^="), vec![TokenKind::CaretEq, TokenKind::Eof]);
}

#[test]
fn three_char_operators() {
    assert_eq!(kinds("..="), vec![TokenKind::DotDotEq, TokenKind::Eof]);
    assert_eq!(kinds("..."), vec![TokenKind::DotDotDot, TokenKind::Eof]);
    assert_eq!(kinds("<<="), vec![TokenKind::ShlEq, TokenKind::Eof]);
    assert_eq!(kinds(">>="), vec![TokenKind::ShrEq, TokenKind::Eof]);
}

#[test]
fn longest_match_wins() {
    // `==` must not tokenise as `= =`.
    assert_eq!(kinds("=="), vec![TokenKind::EqEq, TokenKind::Eof]);
    // `..=` must not tokenise as `.. =`.
    assert_eq!(kinds("..="), vec![TokenKind::DotDotEq, TokenKind::Eof]);
    // `..` must not be greedy and eat a third `.` that is not there.
    assert_eq!(kinds(".."), vec![TokenKind::DotDot, TokenKind::Eof]);
    // Adjacent `>>` is a single `Shr`.
    assert_eq!(kinds(">>"), vec![TokenKind::Shr, TokenKind::Eof]);
    // A space breaks longest-match, two `Gt`s.
    assert_eq!(
        kinds("> >"),
        vec![TokenKind::Gt, TokenKind::Gt, TokenKind::Eof]
    );
}
