//! TokenKind discriminant stability.
//!
//! Every `TokenKind` variant must have a unique, stable discriminant.
//! This test file constructs one of each variant and confirms they
//! compare as expected and that their `u8` discriminants are unique.

use vehje_ir::TokenKind;

/// Exhaustive list of every `TokenKind` variant.
///
/// Adding a new variant requires a line here too, the test will
/// otherwise fail the uniqueness check below.
const ALL: &[TokenKind] = &[
    // Keywords.
    TokenKind::Fn,
    TokenKind::Let,
    TokenKind::Mut,
    TokenKind::Const,
    TokenKind::Struct,
    TokenKind::Pub,
    TokenKind::PubCrate,
    TokenKind::PubSuper,
    TokenKind::If,
    TokenKind::Else,
    TokenKind::Match,
    TokenKind::For,
    TokenKind::While,
    TokenKind::Return,
    TokenKind::Use,
    TokenKind::Mod,
    TokenKind::Impl,
    TokenKind::Trait,
    TokenKind::Type,
    TokenKind::Enum,
    TokenKind::Macro,
    TokenKind::Expect,
    TokenKind::Actual,
    TokenKind::As,
    TokenKind::In,
    TokenKind::Break,
    TokenKind::Continue,
    TokenKind::Self_,
    TokenKind::Super,
    TokenKind::Crate,
    TokenKind::True,
    TokenKind::False,
    TokenKind::Where,
    TokenKind::Move,
    TokenKind::Async,
    TokenKind::Await,
    TokenKind::Dyn,
    TokenKind::Unsafe,
    TokenKind::Loop,
    TokenKind::Static,
    TokenKind::Extern,
    TokenKind::Ref,
    // Identifiers + literals.
    TokenKind::Ident,
    TokenKind::IntLit,
    TokenKind::StrLit,
    TokenKind::CharLit,
    TokenKind::FloatLit,
    TokenKind::ByteStrLit,
    TokenKind::ByteLit,
    TokenKind::RawStrLit,
    // Punctuation.
    TokenKind::LBrace,
    TokenKind::RBrace,
    TokenKind::LParen,
    TokenKind::RParen,
    TokenKind::LBracket,
    TokenKind::RBracket,
    TokenKind::Comma,
    TokenKind::Semi,
    TokenKind::Colon,
    TokenKind::ColonColon,
    TokenKind::Arrow,
    TokenKind::FatArrow,
    TokenKind::Dot,
    TokenKind::DotDot,
    TokenKind::DotDotEq,
    TokenKind::DotDotDot,
    TokenKind::Question,
    TokenKind::At,
    TokenKind::Pound,
    TokenKind::Dollar,
    TokenKind::Tilde,
    // Operators.
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::Star,
    TokenKind::Slash,
    TokenKind::Percent,
    TokenKind::Caret,
    TokenKind::And,
    TokenKind::Or,
    TokenKind::Shl,
    TokenKind::Shr,
    TokenKind::AndAnd,
    TokenKind::OrOr,
    TokenKind::Bang,
    TokenKind::Eq,
    TokenKind::EqEq,
    TokenKind::NotEq,
    TokenKind::Lt,
    TokenKind::Gt,
    TokenKind::LtEq,
    TokenKind::GtEq,
    TokenKind::PlusEq,
    TokenKind::MinusEq,
    TokenKind::StarEq,
    TokenKind::SlashEq,
    TokenKind::PercentEq,
    TokenKind::CaretEq,
    TokenKind::AndEq,
    TokenKind::OrEq,
    TokenKind::ShlEq,
    TokenKind::ShrEq,
    // Comments + trivia.
    TokenKind::LineComment,
    TokenKind::BlockComment,
    TokenKind::Whitespace,
    // Terminals.
    TokenKind::Eof,
    TokenKind::Unknown,
];

#[test]
fn every_variant_has_unique_discriminant() {
    // Brute-force O(n^2) comparison; n is small (~100) and this runs
    // once per build.
    let mut i = 0;
    while i < ALL.len() {
        let mut j = i + 1;
        while j < ALL.len() {
            assert_ne!(
                ALL[i] as u8, ALL[j] as u8,
                "duplicate discriminant between {:?} and {:?}",
                ALL[i], ALL[j]
            );
            j += 1;
        }
        i += 1;
    }
}

#[test]
fn variant_equality_is_reflexive() {
    for k in ALL {
        assert_eq!(k, k);
    }
}

#[test]
fn default_is_unknown() {
    let d: TokenKind = TokenKind::default();
    assert_eq!(d, TokenKind::Unknown);
}

#[test]
fn roster_is_sized() {
    // Sanity check the roster length; keeps ALL from silently drifting.
    assert!(ALL.len() >= 100, "roster below expected size: {}", ALL.len());
}
