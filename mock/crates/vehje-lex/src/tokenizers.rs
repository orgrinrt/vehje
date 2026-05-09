//! Per-kind tokeniser helpers.
//!
//! Each helper assumes the cursor is already positioned at the first
//! byte of the token it handles and returns the classified
//! `TokenKind` plus the `(start, end)` byte offsets of the consumed
//! range. The calling lexer is responsible for turning `(start,
//! end)` into a `Span` (attaching the session's `FileId`) and for
//! attaching any collected trivia.
//!
//! Every helper uses only `Cursor` for source access and ASCII-only
//! char classes. Non-ASCII identifier support is a deferred concern.

use vehje_ir::TokenKind;
use notko::Maybe;

use crate::cursor::Cursor;
use crate::keyword::lookup_keyword;
use crate::trivia::TriviaKind;

/// `true` if `b` can start an identifier (ASCII letter or `_`).
pub const fn is_ident_start(b: u8) -> bool {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_')
}

/// `true` if `b` may continue an identifier (start chars plus digits).
pub const fn is_ident_cont(b: u8) -> bool {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9')
}

/// `true` if `b` is an ASCII decimal digit.
pub const fn is_digit(b: u8) -> bool {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    matches!(b, b'0'..=b'9')
}

/// `true` if `b` is whitespace we classify as `TriviaKind::Whitespace`.
///
/// Space, tab, CR, LF, form-feed, vertical tab. Rust follows Unicode
/// Pattern_White_Space; we keep the ASCII subset here and let a
/// future unicode round widen it.
pub const fn is_whitespace(b: u8) -> bool {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | 0x0B | 0x0C)
}

/// Scan an identifier starting at the current cursor position. The
/// cursor must be pointing at an `is_ident_start` byte. Returns the
/// classified `TokenKind` (a keyword variant or `TokenKind::Ident`)
/// and the `(start, end)` byte-offset pair.
pub fn read_ident_or_keyword(c: &mut Cursor<'_>) -> (TokenKind, u32, u32) {
    let start = c.pos_u32();
    let _ = c.bump_byte();
    while let Maybe::Is(b) = c.peek_byte() {
        if is_ident_cont(b) {
            let _ = c.bump_byte();
        } else {
            break;
        }
    }
    let end = c.pos_u32();
    let slice = &c.src()[start as usize..end as usize];  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    let text = match core::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => "",
    };
    let kind = match lookup_keyword(text) {
        Maybe::Is(k) => k,
        Maybe::Isnt => TokenKind::Ident,
    };
    (kind, start, end)
}

/// Scan a decimal integer literal starting at the current cursor
/// position. The cursor must be pointing at an ASCII digit. This
/// round does not consume numeric suffixes, underscores, float
/// fractions, exponents, or alternate-base prefixes; those are
/// scheduled for a later round.
pub fn read_int_literal(c: &mut Cursor<'_>) -> (TokenKind, u32, u32) {
    let start = c.pos_u32();
    while let Maybe::Is(b) = c.peek_byte() {
        if is_digit(b) {
            let _ = c.bump_byte();
        } else {
            break;
        }
    }
    let end = c.pos_u32();
    (TokenKind::IntLit, start, end)
}

/// Scan a line comment. The cursor must be pointing at the leading
/// `/`; the caller has already confirmed the second byte is also
/// `/`. Consumes up to (but not including) the terminating `\n`.
/// Returns `(kind, start, end)`.
pub fn read_line_comment(c: &mut Cursor<'_>) -> (TriviaKind, u32, u32) {
    let start = c.pos_u32();
    let _ = c.bump_byte();
    let _ = c.bump_byte();
    while let Maybe::Is(b) = c.peek_byte() {
        if b == b'\n' {
            break;
        }
        let _ = c.bump_byte();
    }
    let end = c.pos_u32();
    (TriviaKind::LineComment, start, end)
}

/// Scan a non-nested block comment. The cursor must be pointing at
/// the leading `/`; the caller has already confirmed the second byte
/// is `*`. Consumes through the terminating `*/`; if EOF is reached
/// first, the scan stops at EOF and the caller may emit an
/// `unterminated block comment` diagnostic.
pub fn read_block_comment(c: &mut Cursor<'_>) -> (TriviaKind, u32, u32) {
    let start = c.pos_u32();
    let _ = c.bump_byte();
    let _ = c.bump_byte();
    while let Maybe::Is(b) = c.peek_byte() {
        if b == b'*' && c.peek_byte_at(1) == Maybe::Is(b'/') {
            let _ = c.bump_byte();
            let _ = c.bump_byte();
            break;
        }
        let _ = c.bump_byte();
    }
    let end = c.pos_u32();
    (TriviaKind::BlockComment, start, end)
}

/// Scan a whitespace run. The cursor must be pointing at an
/// `is_whitespace` byte. Returns `(kind, start, end)`.
pub fn read_whitespace(c: &mut Cursor<'_>) -> (TriviaKind, u32, u32) {
    let start = c.pos_u32();
    while let Maybe::Is(b) = c.peek_byte() {
        if is_whitespace(b) {
            let _ = c.bump_byte();
        } else {
            break;
        }
    }
    let end = c.pos_u32();
    (TriviaKind::Whitespace, start, end)
}

/// Attempt to scan a single operator or punctuation token. Uses
/// longest-match semantics: three-byte candidates (`..=`, `<<=`,
/// `>>=`, `...`) are tried before two-byte candidates, two-byte
/// before one-byte. Returns `Maybe::Isnt` if the cursor is not on an
/// operator byte (caller should fall through to `Unknown`).
pub fn read_operator_or_punct(c: &mut Cursor<'_>) -> Maybe<(TokenKind, u32, u32)> {
    let start = c.pos_u32();
    let b0 = match c.peek_byte() {
        Maybe::Is(b) => b,
        Maybe::Isnt => return Maybe::Isnt,
    };
    let b1 = c.peek_byte_at(1);
    let b2 = c.peek_byte_at(2);

    if let Maybe::Is(k) = three_byte_op(b0, b1, b2) {
        c.bump_n(3);
        return Maybe::Is((k, start, c.pos_u32()));
    }
    if let Maybe::Is(k) = two_byte_op(b0, b1) {
        c.bump_n(2);
        return Maybe::Is((k, start, c.pos_u32()));
    }
    if let Maybe::Is(k) = one_byte_op(b0) {
        c.bump_n(1);
        return Maybe::Is((k, start, c.pos_u32()));
    }
    Maybe::Isnt
}

fn three_byte_op(b0: u8, b1: Maybe<u8>, b2: Maybe<u8>) -> Maybe<TokenKind> {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    let b1 = match b1 {
        Maybe::Is(b) => b,
        Maybe::Isnt => return Maybe::Isnt,
    };
    let b2 = match b2 {
        Maybe::Is(b) => b,
        Maybe::Isnt => return Maybe::Isnt,
    };
    Maybe::Is(match (b0, b1, b2) {
        (b'.', b'.', b'=') => TokenKind::DotDotEq,
        (b'.', b'.', b'.') => TokenKind::DotDotDot,
        (b'<', b'<', b'=') => TokenKind::ShlEq,
        (b'>', b'>', b'=') => TokenKind::ShrEq,
        _ => return Maybe::Isnt,
    })
}

fn two_byte_op(b0: u8, b1: Maybe<u8>) -> Maybe<TokenKind> {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    let b1 = match b1 {
        Maybe::Is(b) => b,
        Maybe::Isnt => return Maybe::Isnt,
    };
    Maybe::Is(match (b0, b1) {
        (b':', b':') => TokenKind::ColonColon,
        (b'-', b'>') => TokenKind::Arrow,
        (b'=', b'>') => TokenKind::FatArrow,
        (b'.', b'.') => TokenKind::DotDot,
        (b'=', b'=') => TokenKind::EqEq,
        (b'!', b'=') => TokenKind::NotEq,
        (b'<', b'=') => TokenKind::LtEq,
        (b'>', b'=') => TokenKind::GtEq,
        (b'&', b'&') => TokenKind::AndAnd,
        (b'|', b'|') => TokenKind::OrOr,
        (b'<', b'<') => TokenKind::Shl,
        (b'>', b'>') => TokenKind::Shr,
        (b'+', b'=') => TokenKind::PlusEq,
        (b'-', b'=') => TokenKind::MinusEq,
        (b'*', b'=') => TokenKind::StarEq,
        (b'/', b'=') => TokenKind::SlashEq,
        (b'%', b'=') => TokenKind::PercentEq,
        (b'&', b'=') => TokenKind::AndEq,
        (b'|', b'=') => TokenKind::OrEq,
        (b'^', b'=') => TokenKind::CaretEq,
        _ => return Maybe::Isnt,
    })
}

fn one_byte_op(b0: u8) -> Maybe<TokenKind> {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    Maybe::Is(match b0 {
        b'+' => TokenKind::Plus,
        b'-' => TokenKind::Minus,
        b'*' => TokenKind::Star,
        b'/' => TokenKind::Slash,
        b'%' => TokenKind::Percent,
        b'=' => TokenKind::Eq,
        b'<' => TokenKind::Lt,
        b'>' => TokenKind::Gt,
        b'!' => TokenKind::Bang,
        b'&' => TokenKind::And,
        b'|' => TokenKind::Or,
        b'^' => TokenKind::Caret,
        b'~' => TokenKind::Tilde,
        b',' => TokenKind::Comma,
        b';' => TokenKind::Semi,
        b':' => TokenKind::Colon,
        b'.' => TokenKind::Dot,
        b'?' => TokenKind::Question,
        b'@' => TokenKind::At,
        b'#' => TokenKind::Pound,
        b'$' => TokenKind::Dollar,
        b'{' => TokenKind::LBrace,
        b'}' => TokenKind::RBrace,
        b'(' => TokenKind::LParen,
        b')' => TokenKind::RParen,
        b'[' => TokenKind::LBracket,
        b']' => TokenKind::RBracket,
        _ => return Maybe::Isnt,
    })
}
