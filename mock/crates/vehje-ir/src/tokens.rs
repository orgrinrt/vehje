//! Lexical token discriminators.
//!
//! `TokenKind` is the shared vocabulary between the lexer (the
//! producer) and the parser (the consumer). The enum is exhaustive
//! over every token the Vehje grammar defines, even for tokenisers
//! that are not yet implemented; deferred literal variants (string,
//! char, float, raw string, byte literal) are listed here as
//! placeholders so downstream consumers never need to be re-keyed
//! when those tokenisers land.
//!
//! The enum is `#[repr(u8)]` for stable, compact discriminants;
//! tests elsewhere pin individual variant values so adding a new
//! variant in the middle of the list is caught mechanically.

/// Discriminator for every token shape the Vehje grammar emits.
///
/// See `docs/tokens.md` (forthcoming) for the canonical mapping of
/// source characters to variants. Keyword variants are spelled in
/// UpperCamelCase without the `Kw` prefix; operator variants use a
/// short glyph-based name (`Plus`, `EqEq`, `FatArrow`).
///
/// The variant `Self_` carries a trailing underscore because `self`
/// is a Rust keyword and cannot be used as a variant name unraw.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)] // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
pub enum TokenKind {
    // ── Keywords ──────────────────────────────────────────────
    Fn,
    Let,
    Mut,
    Const,
    Struct,
    Pub,
    PubCrate,
    PubSuper,
    If,
    Else,
    Match,
    For,
    While,
    Return,
    Use,
    Mod,
    Impl,
    Trait,
    Type,
    Enum,
    Macro,
    Expect,
    Actual,
    As,
    In,
    Break,
    Continue,
    Self_,
    Super,
    Crate,
    True,
    False,
    Where,
    Move,
    Async,
    Await,
    Dyn,
    Unsafe,
    Loop,
    Static,
    Extern,
    Ref,

    // ── Identifiers and literals ──────────────────────────────
    Ident,
    IntLit,
    StrLit,
    CharLit,
    FloatLit,
    ByteStrLit,
    ByteLit,
    RawStrLit,

    // ── Punctuation ───────────────────────────────────────────
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semi,
    Colon,
    ColonColon,
    Arrow,
    FatArrow,
    Dot,
    DotDot,
    DotDotEq,
    DotDotDot,
    Question,
    At,
    Pound,
    Dollar,
    Tilde,

    // ── Operators ─────────────────────────────────────────────
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,
    Shl,
    Shr,
    AndAnd,
    OrOr,
    Bang,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,

    // ── Comments and trivia ───────────────────────────────────
    LineComment,
    BlockComment,
    Whitespace,

    // ── Terminals ─────────────────────────────────────────────
    Eof,
    Unknown,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Unknown
    }
}
