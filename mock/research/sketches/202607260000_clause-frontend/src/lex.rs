//! The lexer: source bytes to a lent token buffer.
//!
//! Writes into a caller-provided `&mut [Token]` and reports how many it wrote,
//! refusing rather than growing when the buffer is full. That is the same
//! lending shape the IR arena uses, so a host sizes one budget for the whole
//! pipeline rather than one per stage.
//!
//! Trivia is skipped rather than recorded, with one exception: a doc comment is
//! a token, because the doc pass consumes it and a lexer that folds it into
//! whitespace makes that pass impossible to write later.

use arvo::USize;
use notko::Outcome;

use crate::token::{Keyword, Span, Token, TokenKind};

/// What can go wrong reading source.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LexError {
    /// The lent token buffer is full.
    TokenBufferFull { at: USize },
    /// A string, char, or raw string with no closing delimiter.
    Unterminated { at: USize },
    /// A block comment with no closing delimiter.
    UnterminatedComment { at: USize },
    /// A byte that begins no token.
    UnexpectedByte { at: USize },
}

/// Read `src` into `out`, returning how many tokens were written.
///
/// The last token is always `Eof`, so a parser reads a token at every position
/// rather than branching on whether one exists.
pub fn lex(src: &str, out: &mut [Token]) -> Outcome<USize, LexError> {
    let b = src.as_bytes();
    let mut i: usize = 0;
    let mut n: usize = 0;

    macro_rules! push {
        ($kind:expr, $start:expr, $end:expr) => {{
            if n >= out.len() {
                return Outcome::Err(LexError::TokenBufferFull { at: USize($start) });
            }
            out[n] = Token::new($kind, Span::new(USize($start), USize($end)));
            n += 1;
        }};
    }

    while i < b.len() {
        // trivia
        if b[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
            let start = i;
            let doc = i + 2 < b.len() && b[i + 2] == b'/';
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            if doc {
                push!(TokenKind::DocComment, start, i);
            }
            continue;
        }
        if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
            let start = i;
            // block comments nest, so a depth counter rather than a scan for the
            // first close
            let mut depth = 1usize;
            i += 2;
            while i < b.len() && depth > 0 {
                if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if b[i] == b'*' && i + 1 < b.len() && b[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if depth > 0 {
                return Outcome::Err(LexError::UnterminatedComment { at: USize(start) });
            }
            continue;
        }

        let start = i;

        // raw string: r"..." or r#"..."#, matching hash counts on both sides
        if b[i] == b'r' && i + 1 < b.len() && (b[i + 1] == b'"' || b[i + 1] == b'#') {
            let mut h = 0usize;
            let mut j = i + 1;
            while j < b.len() && b[j] == b'#' {
                h += 1;
                j += 1;
            }
            if j < b.len() && b[j] == b'"' {
                j += 1;
                loop {
                    if j >= b.len() {
                        return Outcome::Err(LexError::Unterminated { at: USize(start) });
                    }
                    if b[j] == b'"' {
                        let mut k = 0usize;
                        while k < h && j + 1 + k < b.len() && b[j + 1 + k] == b'#' {
                            k += 1;
                        }
                        if k == h {
                            j += 1 + h;
                            break;
                        }
                    }
                    j += 1;
                }
                i = j;
                push!(TokenKind::RawStr, start, i);
                continue;
            }
        }

        // identifier or keyword
        if b[i] == b'_' || b[i].is_ascii_alphabetic() {
            while i < b.len() && (b[i] == b'_' || b[i].is_ascii_alphanumeric()) {
                i += 1;
            }
            let text = &src[start..i];
            let kind = match Keyword::of(text) {
                Some(k) => TokenKind::Keyword(k),
                None => TokenKind::Ident,
            };
            push!(kind, start, i);
            continue;
        }

        // number: an integer unless a fraction or an exponent makes it a float
        if b[i].is_ascii_digit() {
            let mut float = false;
            if b[i] == b'0' && i + 1 < b.len() && matches!(b[i + 1], b'x' | b'b' | b'o') {
                i += 2;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
            } else {
                while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'_') {
                    i += 1;
                }
                // a dot is a fraction only if a digit follows, so `1..2` stays a
                // range and `1.foo()` stays a method call
                if i + 1 < b.len() && b[i] == b'.' && b[i + 1].is_ascii_digit() {
                    float = true;
                    i += 1;
                    while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'_') {
                        i += 1;
                    }
                }
                if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
                    float = true;
                    i += 1;
                    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
                        i += 1;
                    }
                    while i < b.len() && b[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                // a free-form suffix rides along for a later pass to read
                while i < b.len() && (b[i] == b'_' || b[i].is_ascii_alphanumeric()) {
                    i += 1;
                }
            }
            push!(if float { TokenKind::Float } else { TokenKind::Int }, start, i);
            continue;
        }

        // string and char, sharing the escape rule
        if b[i] == b'"' || b[i] == b'\'' {
            let quote = b[i];
            i += 1;
            loop {
                if i >= b.len() {
                    return Outcome::Err(LexError::Unterminated { at: USize(start) });
                }
                if b[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if b[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push!(if quote == b'"' { TokenKind::Str } else { TokenKind::Char }, start, i);
            continue;
        }

        // punctuation, longest match first
        let (kind, len) = match punct(&b[i..]) {
            Some(p) => p,
            None => return Outcome::Err(LexError::UnexpectedByte { at: USize(i) }),
        };
        i += len;
        push!(kind, start, i);
    }

    push!(TokenKind::Eof, b.len(), b.len());
    Outcome::Ok(USize(n))
}

/// The punctuation token at the head of `b`, longest match first.
///
/// Order matters and is the whole correctness of this function: `..=` must be
/// tried before `..`, `<<=` before `<<` before `<`, and so on, or a longer
/// token lexes as its own prefix.
fn punct(b: &[u8]) -> Option<(TokenKind, usize)> {
    use TokenKind::*;
    let three = |s: &[u8]| b.len() >= 3 && &b[..3] == s;
    let two = |s: &[u8]| b.len() >= 2 && &b[..2] == s;

    if three(b"..=") {
        return Some((DotDotEq, 3));
    }
    if three(b"<<=") {
        return Some((ShlEq, 3));
    }
    if three(b">>=") {
        return Some((ShrEq, 3));
    }

    if two(b"->") {
        return Some((Arrow, 2));
    }
    if two(b"=>") {
        return Some((FatArrow, 2));
    }
    if two(b"::") {
        return Some((ColonColon, 2));
    }
    if two(b"..") {
        return Some((DotDot, 2));
    }
    if two(b"<<") {
        return Some((Shl, 2));
    }
    if two(b">>") {
        return Some((Shr, 2));
    }
    if two(b"==") {
        return Some((EqEq, 2));
    }
    if two(b"!=") {
        return Some((Ne, 2));
    }
    if two(b">=") {
        return Some((Ge, 2));
    }
    if two(b"<=") {
        return Some((Le, 2));
    }
    if two(b"&&") {
        return Some((AndAnd, 2));
    }
    if two(b"||") {
        return Some((OrOr, 2));
    }
    if two(b"+=") {
        return Some((PlusEq, 2));
    }
    if two(b"-=") {
        return Some((MinusEq, 2));
    }
    if two(b"*=") {
        return Some((StarEq, 2));
    }
    if two(b"/=") {
        return Some((SlashEq, 2));
    }
    if two(b"%=") {
        return Some((PercentEq, 2));
    }
    if two(b"&=") {
        return Some((AmpEq, 2));
    }
    if two(b"|=") {
        return Some((PipeEq, 2));
    }
    if two(b"^=") {
        return Some((CaretEq, 2));
    }
    if two(b"#!") {
        return Some((PoundBang, 2));
    }

    Some(match b[0] {
        b'+' => (Plus, 1),
        b'-' => (Minus, 1),
        b'*' => (Star, 1),
        b'/' => (Slash, 1),
        b'%' => (Percent, 1),
        b'=' => (Eq, 1),
        b'<' => (Lt, 1),
        b'>' => (Gt, 1),
        b'!' => (Bang, 1),
        b'&' => (Amp, 1),
        b'|' => (Pipe, 1),
        b'^' => (Caret, 1),
        b'~' => (Tilde, 1),
        b'.' => (Dot, 1),
        b',' => (Comma, 1),
        b';' => (Semi, 1),
        b':' => (Colon, 1),
        b'?' => (Question, 1),
        b'$' => (Dollar, 1),
        b'#' => (Pound, 1),
        b'(' => (LParen, 1),
        b')' => (RParen, 1),
        b'{' => (LBrace, 1),
        b'}' => (RBrace, 1),
        b'[' => (LBracket, 1),
        b']' => (RBracket, 1),
        _ => return None,
    })
}
