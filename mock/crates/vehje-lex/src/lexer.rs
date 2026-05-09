//! Top-level lexer.
//!
//! `Lexer<'a>` walks a source slice, emitting `Token` values. Each
//! `next()` call:
//!
//! 1. Consumes leading trivia (whitespace + line/block comments).
//! 2. Tokenises one "real" token.
//! 3. Consumes trailing trivia up to and including the next newline;
//!    if the token is followed by another real token on the same
//!    line, the trivia up to that token attaches as trailing.
//! 4. Returns the token with trivia attached.
//!
//! After the last real token a final `TokenKind::Eof` token is
//! emitted so consumers that prefer EOF-as-a-token work naturally.
//! A subsequent `next()` call returns `Maybe::Isnt`.
//!
//! Errors (unterminated block comment, unclassifiable byte) emit a
//! diagnostic into a caller-supplied sink and produce
//! `TokenKind::Unknown` / `TokenKind::BlockComment` as appropriate.
//! The sink interface keeps the lexer `no_std` and alloc-free; the
//! most common case (no diagnostics) costs nothing.

use vehje_ir::{
    ByteOffset, DiagPhase, Diagnostic, FileId, Severity, Span, TokenKind,
};
use notko::Maybe;

use crate::cursor::Cursor;
use crate::token::Token;
use crate::tokenizers::{
    is_digit, is_ident_start, is_whitespace, read_block_comment,
    read_ident_or_keyword, read_int_literal, read_line_comment,
    read_operator_or_punct, read_whitespace,
};
use crate::trivia::{Trivia, TriviaKind, TriviaSet};

/// Diagnostic sink shape. A no-op sink simply discards; a real sink
/// buffers into a caller-owned fixed-size array.
///
/// Kept as a generic function pointer so no `dyn` is needed; the
/// common case is a stateless forwarder.
pub type DiagSink<'a> = &'a mut dyn FnMut(Diagnostic);

/// The lexer.
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    file: FileId,
    emitted_eof: bool,  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207 lint:allow(no-public-raw-field) tracked: #207
}

impl<'a> Lexer<'a> {
    /// Construct a lexer over `src` for source file `file`.
    pub fn new(src: &'a [u8], file: FileId) -> Self {
        Self { cursor: Cursor::new(src), file, emitted_eof: false }
    }

    /// Construct a lexer from a `&str`.
    pub fn from_str(src: &'a str, file: FileId) -> Self {  // lint:allow(no-bare-string) tracked: #207
        Self::new(src.as_bytes(), file)
    }

    /// The file id this lexer was built with.
    pub const fn file(&self) -> FileId {
        self.file
    }

    /// Produce the next token, or `Maybe::Isnt` once EOF has been
    /// emitted and the cursor is past the end of input.
    ///
    /// The non-diagnostic `next` wraps `next_with_diag` with a
    /// sink that swallows; call `next_with_diag` directly when the
    /// caller wants to see lex-level diagnostics.
    pub fn next(&mut self) -> Maybe<Token> {
        let mut sink = |_d: Diagnostic| {};
        self.next_with_diag(&mut sink)
    }

    /// Produce the next token, reporting lex diagnostics through
    /// `sink`.
    pub fn next_with_diag(&mut self, sink: DiagSink<'_>) -> Maybe<Token> {
        if self.emitted_eof {
            return Maybe::Isnt;
        }
        let mut trivia = TriviaSet::new();
        self.consume_leading_trivia(&mut trivia, sink);

        if self.cursor.is_eof() {
            let pos = ByteOffset(self.cursor.pos_u32());
            self.emitted_eof = true;
            return Maybe::Is(Token::new(
                TokenKind::Eof,
                Span::new(self.file, pos, pos),
                trivia,
            ));
        }

        let (kind, start, end) = self.scan_one(sink);
        self.consume_trailing_trivia(&mut trivia, sink);

        Maybe::Is(Token::new(
            kind,
            Span::new(self.file, ByteOffset(start), ByteOffset(end)),
            trivia,
        ))
    }

    /// Scan one real token starting at the current cursor position.
    /// The cursor is guaranteed non-EOF at entry.
    fn scan_one(&mut self, sink: DiagSink<'_>) -> (TokenKind, u32, u32) {
        let b0 = match self.cursor.peek_byte() {
            Maybe::Is(b) => b,
            Maybe::Isnt => {
                let p = self.cursor.pos_u32();
                return (TokenKind::Eof, p, p);
            },
        };

        if is_ident_start(b0) {
            return read_ident_or_keyword(&mut self.cursor);
        }
        if is_digit(b0) {
            return read_int_literal(&mut self.cursor);
        }
        if let Maybe::Is(t) = read_operator_or_punct(&mut self.cursor) {
            return t;
        }

        // Unclassifiable byte: advance one UTF-8 code point so the
        // cursor still makes progress, emit a diagnostic, and return
        // `TokenKind::Unknown`.
        let start = self.cursor.pos_u32();
        if self.cursor.bump().isnt() {
            // Malformed UTF-8 mid-stream; fall back to byte advance.
            let _ = self.cursor.bump_byte();
        }
        let end = self.cursor.pos_u32();
        (sink)(Diagnostic::new(
            DiagPhase::Lex,
            Severity::Error,
            Span::new(self.file, ByteOffset(start), ByteOffset(end)),
            "unrecognised byte in source",
        ));
        (TokenKind::Unknown, start, end)
    }

    /// Consume all trivia at the current cursor: whitespace, line
    /// comments, block comments in any mix. Stops at the first real
    /// token or at EOF.
    fn consume_leading_trivia(&mut self, set: &mut TriviaSet, sink: DiagSink<'_>) {
        loop {
            let b = match self.cursor.peek_byte() {
                Maybe::Is(b) => b,
                Maybe::Isnt => return,
            };
            if is_whitespace(b) {
                let (k, s, e) = read_whitespace(&mut self.cursor);
                set.push_leading(Trivia::new(k, self.mk_span(s, e)));
                continue;
            }
            if b == b'/' && self.cursor.peek_byte_at(1) == Maybe::Is(b'/') {
                let (k, s, e) = read_line_comment(&mut self.cursor);
                set.push_leading(Trivia::new(k, self.mk_span(s, e)));
                continue;
            }
            if b == b'/' && self.cursor.peek_byte_at(1) == Maybe::Is(b'*') {
                let start = self.cursor.pos_u32();
                let (k, s, e) = read_block_comment(&mut self.cursor);
                // Detect unterminated: if the last two consumed bytes
                // are not `*/`, emit a diagnostic.
                if !terminated_block(self.cursor.src(), s, e) {
                    (sink)(Diagnostic::new(
                        DiagPhase::Lex,
                        Severity::Error,
                        Span::new(
                            self.file,
                            ByteOffset(start),
                            ByteOffset(e),
                        ),
                        "unterminated block comment",
                    ));
                }
                set.push_leading(Trivia::new(k, self.mk_span(s, e)));
                continue;
            }
            return;
        }
    }

    /// Consume trivia that follows a just-scanned token, up to and
    /// including the next newline. If EOF or another real token is
    /// reached first, stops there. On reaching a newline, the
    /// newline is included in the trailing trivia run and the
    /// function returns; a subsequent `next()` call will pick up any
    /// further trivia as leading trivia of the next token.
    fn consume_trailing_trivia(&mut self, set: &mut TriviaSet, sink: DiagSink<'_>) {
        loop {
            let b = match self.cursor.peek_byte() {
                Maybe::Is(b) => b,
                Maybe::Isnt => return,
            };
            if is_whitespace(b) {
                let start = self.cursor.pos_u32();
                // Consume a run of non-newline whitespace, then
                // optionally a single newline.
                while let Maybe::Is(bb) = self.cursor.peek_byte() {
                    if bb == b'\n' {
                        let _ = self.cursor.bump_byte();
                        break;
                    }
                    if is_whitespace(bb) {
                        let _ = self.cursor.bump_byte();
                    } else {
                        break;
                    }
                }
                let end = self.cursor.pos_u32();
                set.push_trailing(Trivia::new(
                    TriviaKind::Whitespace,
                    self.mk_span(start, end),
                ));
                // A newline terminates the trailing run.
                if end > start
                    && self.cursor.src()[(end - 1) as usize] == b'\n'  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
                {
                    return;
                }
                // No newline yet: keep looking for comments on the
                // same line.
                continue;
            }
            if b == b'/' && self.cursor.peek_byte_at(1) == Maybe::Is(b'/') {
                let (k, s, e) = read_line_comment(&mut self.cursor);
                set.push_trailing(Trivia::new(k, self.mk_span(s, e)));
                // Line comment runs to the end of the line; the
                // newline terminator (if any) becomes the next run's
                // leading whitespace.
                return;
            }
            if b == b'/' && self.cursor.peek_byte_at(1) == Maybe::Is(b'*') {
                let start = self.cursor.pos_u32();
                let (k, s, e) = read_block_comment(&mut self.cursor);
                if !terminated_block(self.cursor.src(), s, e) {
                    (sink)(Diagnostic::new(
                        DiagPhase::Lex,
                        Severity::Error,
                        Span::new(
                            self.file,
                            ByteOffset(start),
                            ByteOffset(e),
                        ),
                        "unterminated block comment",
                    ));
                }
                set.push_trailing(Trivia::new(k, self.mk_span(s, e)));
                continue;
            }
            return;
        }
    }

    fn mk_span(&self, start: u32, end: u32) -> Span {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
        Span::new(self.file, ByteOffset(start), ByteOffset(end))
    }
}

/// `true` if the block-comment span ends with the `*/` terminator.
fn terminated_block(src: &[u8], start: u32, end: u32) -> bool {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    let e = end as usize;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    if e < (start as usize) + 4 {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
        return false;
    }
    src.get(e - 2) == Some(&b'*') && src.get(e - 1) == Some(&b'/') // lint:allow(bare_option) tracked: #115
}
