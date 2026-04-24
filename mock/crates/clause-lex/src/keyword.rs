//! Keyword classification table.
//!
//! `KEYWORDS` is a flat `&[(&str, TokenKind)]` scanned linearly; the
//! round-one list is small enough that a linear scan beats a hashmap
//! (no alloc, better branch predictability). Keyword matching is
//! case-sensitive per the Vehje language spec — `fn` is a keyword,
//! `Fn` is an ident.

use vehje_ir::TokenKind;
use notko::Maybe;

/// The authoritative keyword list.
///
/// Order is not semantically significant; the scan is linear. Keep
/// entries grouped by category for readability.
pub const KEYWORDS: &[(&str, TokenKind)] = &[  // lint:allow(no-bare-string) tracked: #207
    // Declaration.
    ("fn", TokenKind::Fn),
    ("let", TokenKind::Let),
    ("mut", TokenKind::Mut),
    ("const", TokenKind::Const),
    ("struct", TokenKind::Struct),
    ("pub", TokenKind::Pub),
    ("use", TokenKind::Use),
    ("mod", TokenKind::Mod),
    ("impl", TokenKind::Impl),
    ("trait", TokenKind::Trait),
    ("type", TokenKind::Type),
    ("enum", TokenKind::Enum),
    ("macro", TokenKind::Macro),
    ("expect", TokenKind::Expect),
    ("actual", TokenKind::Actual),
    ("static", TokenKind::Static),
    ("extern", TokenKind::Extern),
    // Control flow.
    ("if", TokenKind::If),
    ("else", TokenKind::Else),
    ("match", TokenKind::Match),
    ("for", TokenKind::For),
    ("while", TokenKind::While),
    ("loop", TokenKind::Loop),
    ("return", TokenKind::Return),
    ("break", TokenKind::Break),
    ("continue", TokenKind::Continue),
    ("in", TokenKind::In),
    // Reference / scope.
    ("self", TokenKind::Self_),
    ("super", TokenKind::Super),
    ("crate", TokenKind::Crate),
    // Literals.
    ("true", TokenKind::True),
    ("false", TokenKind::False),
    // Modifiers / reserved.
    ("as", TokenKind::As),
    ("where", TokenKind::Where),
    ("move", TokenKind::Move),
    ("async", TokenKind::Async),
    ("await", TokenKind::Await),
    ("dyn", TokenKind::Dyn),
    ("unsafe", TokenKind::Unsafe),
    ("ref", TokenKind::Ref),
];

/// Look up `ident` in the keyword table. Case-sensitive.
///
/// Returns `Maybe::Isnt` if the identifier is not a keyword; the
/// caller should then emit `TokenKind::Ident`.
pub fn lookup_keyword(ident: &str) -> Maybe<TokenKind> {  // lint:allow(no-bare-string) tracked: #207
    let mut i = 0;
    while i < KEYWORDS.len() {
        let (s, k) = KEYWORDS[i];
        if str_eq(s.as_bytes(), ident.as_bytes()) {
            return Maybe::Is(k);
        }
        i += 1;
    }
    Maybe::Isnt
}

/// Byte-wise string equality. Avoids pulling in `str::eq`'s machinery
/// in a `no_std` const-friendly shape.
fn str_eq(a: &[u8], b: &[u8]) -> bool {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}
