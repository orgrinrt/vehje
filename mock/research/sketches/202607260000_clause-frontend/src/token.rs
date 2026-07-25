//! The token vocabulary, from the normative grammar's lexical section.
//!
//! A token carries its kind and its span; it does not carry text. Literal and
//! identifier text is read back from the source by span when a later pass needs
//! it, which is what lets the token buffer be a flat lent array of fixed-size
//! records with no allocation and no borrows into a string table that does not
//! exist yet.

use arvo::USize;

/// A half-open byte range in the source.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Span {
    /// First byte.
    pub start: USize,
    /// One past the last byte.
    pub end: USize,
}

impl Default for Span {
    fn default() -> Self {
        Self { start: USize(0), end: USize(0) }
    }
}

impl Span {
    /// The span covering `start..end`.
    pub const fn new(start: USize, end: USize) -> Self {
        Self { start, end }
    }

    /// The source text this span names.
    pub fn of<'s>(&self, src: &'s str) -> &'s str {
        &src[self.start.0..self.end.0]
    }
}

/// A reserved word.
///
/// `dyn`, `unsafe`, `async`, and `await` are reserved and lex, but no
/// production accepts them; the parser refuses them by name rather than
/// treating them as identifiers, which is what makes the reservation mean
/// something to a reader who tries one.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Keyword {
    Mod,
    Use,
    Pub,
    Struct,
    Trait,
    Impl,
    Fn,
    Let,
    Mut,
    Const,
    Static,
    Extern,
    Expect,
    Actual,
    Event,
    Enum,
    Type,
    Macro,
    Sealed,
    If,
    Else,
    Match,
    For,
    While,
    Loop,
    Return,
    Break,
    Continue,
    In,
    LowerSelf,
    UpperSelf,
    Super,
    Crate,
    As,
    Where,
    Dyn,
    Move,
    Unsafe,
    Async,
    Await,
}

impl Keyword {
    /// The keyword `s` names, if it is one.
    pub fn of(s: &str) -> Option<Self> {
        Some(match s {
            "mod" => Self::Mod,
            "use" => Self::Use,
            "pub" => Self::Pub,
            "struct" => Self::Struct,
            "trait" => Self::Trait,
            "impl" => Self::Impl,
            "fn" => Self::Fn,
            "let" => Self::Let,
            "mut" => Self::Mut,
            "const" => Self::Const,
            "static" => Self::Static,
            "extern" => Self::Extern,
            "expect" => Self::Expect,
            "actual" => Self::Actual,
            "event" => Self::Event,
            "enum" => Self::Enum,
            "type" => Self::Type,
            "macro" => Self::Macro,
            "sealed" => Self::Sealed,
            "if" => Self::If,
            "else" => Self::Else,
            "match" => Self::Match,
            "for" => Self::For,
            "while" => Self::While,
            "loop" => Self::Loop,
            "return" => Self::Return,
            "break" => Self::Break,
            "continue" => Self::Continue,
            "in" => Self::In,
            "self" => Self::LowerSelf,
            "Self" => Self::UpperSelf,
            "super" => Self::Super,
            "crate" => Self::Crate,
            "as" => Self::As,
            "where" => Self::Where,
            "dyn" => Self::Dyn,
            "move" => Self::Move,
            "unsafe" => Self::Unsafe,
            "async" => Self::Async,
            "await" => Self::Await,
            _ => return None,
        })
    }

    /// Whether the grammar reserves this word without accepting it anywhere.
    pub const fn is_reserved_unused(self) -> bool {
        matches!(self, Self::Dyn | Self::Unsafe | Self::Async | Self::Await)
    }
}

/// What a token is.
///
/// Literals carry no decoded value: an integer's digits, a string's escapes,
/// and a raw string's hash count are all recoverable from the span, and
/// decoding at lex time would either allocate or fix a representation before
/// the type checker has said which one it wants.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TokenKind {
    Ident,
    Keyword(Keyword),
    Int,
    Float,
    Str,
    RawStr,
    Char,
    /// A doc comment, kept rather than discarded as trivia, because `clause doc`
    /// consumes it and a lexer that drops it makes that pass impossible.
    DocComment,

    // one-character punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Lt,
    Gt,
    Bang,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Dot,
    Comma,
    Semi,
    Colon,
    Question,
    Dollar,

    // multi-character punctuation
    Arrow,
    FatArrow,
    ColonColon,
    DotDot,
    DotDotEq,
    Shl,
    Shr,
    EqEq,
    Ne,
    Ge,
    Le,
    AndAnd,
    OrOr,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpEq,
    PipeEq,
    CaretEq,
    ShlEq,
    ShrEq,

    // brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    /// `#`, the outer-attribute opener.
    Pound,
    /// `#!`, the inner-attribute opener.
    PoundBang,

    /// End of input. A real token rather than an option, so every parser site
    /// reads a token rather than branching on whether one exists.
    Eof,
}

/// One token: what it is and where it came from.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Token {
    /// What it is.
    pub kind: TokenKind,
    /// Where it came from.
    pub span: Span,
}

impl Token {
    /// A token of `kind` covering `span`.
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
