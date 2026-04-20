//! clause-lex — skeleton lexer for the Clause authoring language.
//!
//! Consumes a source buffer, produces a token stream with trivia
//! attached. Round-one scope covers:
//!
//! - ASCII identifiers and the full keyword list from the grammar
//!   seed.
//! - Decimal integer literals (no suffixes, no underscores, no float
//!   fractions, no alternate bases — those are deferred).
//! - ASCII operators and punctuation with longest-match resolution.
//! - Line comments (`// …`) and block comments (`/* … */`,
//!   non-nested — nested block comments are in BACKLOG).
//! - Whitespace as trivia, attached to the next real token.
//! - Explicit `Eof` token at end-of-input.
//! - `Unknown` token plus diagnostic for any unclassifiable byte.
//!
//! String, char, raw-string, byte-string, and float literals, plus
//! numeric suffixes, unicode escapes, doc comments, nested block
//! comments, shebangs, and BOMs are deferred to a later round.
//!
//! `#![no_std]`, no alloc. The diagnostic sink is a caller-supplied
//! `FnMut` so no channel / buffer type is prescribed here.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod cursor;
pub mod keyword;
pub mod lexer;
pub mod stream;
pub mod token;
pub mod tokenizers;
pub mod trivia;

pub use cursor::Cursor;
pub use keyword::{KEYWORDS, lookup_keyword};
pub use lexer::{DiagSink, Lexer};
pub use stream::TokenStream;
pub use token::Token;
pub use trivia::{Trivia, TriviaKind, TriviaSet};
