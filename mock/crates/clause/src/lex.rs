//! `clause lex` — tokenise a file and print its token stream.

use vehje_ir::{Diagnostic, FileId, TokenKind};
use vehje_lex::{Lexer, Token};
use notko::Maybe;

pub fn run(args: &[String]) -> i32 { // lint:allow(bare_string) lint:allow(bare_numeric) reason: CLI entry plumbing; argv and exit code are the std-boundary shapes; tracked: #73
    let path = match crate::args::single_file_arg(args) {
        Maybe::Is(p) => p,
        Maybe::Isnt => {
            eprintln!("clause lex: missing <file>");
            return 2;
        }
    };
    let src = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("clause lex: {path}: {e}");
            return 2;
        }
    };
    let file = FileId(1); // lint:allow(bare_numeric) reason: single-file mode carries a literal FileId placeholder until multi-file manifest round; tracked: #73
    let mut lexer = Lexer::from_str(&src, file);
    let mut had_diag = false;
    let mut sink = |d: Diagnostic| {
        crate::diag::print(path, &src, &d);
        had_diag = true;
    };
    loop {
        match lexer.next_with_diag(&mut sink) {
            Maybe::Is(tok) => print_token(&src, &tok),
            Maybe::Isnt => break,
        }
    }
    if had_diag {
        1
    } else {
        0
    }
}

fn print_token(src: &str, tok: &Token) { // lint:allow(bare_string) reason: host-side CLI prints to stdout via &str; tracked: #73
    let start = tok.span.start.0 as usize;
    let end = tok.span.end.0 as usize;
    let slice = &src.as_bytes()[start..end.min(src.len())];
    let lit = core::str::from_utf8(slice).unwrap_or("<non-utf8>");
    if carries_literal(tok.kind) && !lit.is_empty() {
        println!("{start}..{end}  {:?}  {lit}", tok.kind);
    } else {
        println!("{start}..{end}  {:?}", tok.kind);
    }
}

fn carries_literal(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Ident
            | TokenKind::IntLit
            | TokenKind::StrLit
            | TokenKind::CharLit
            | TokenKind::FloatLit
            | TokenKind::ByteStrLit
            | TokenKind::ByteLit
            | TokenKind::RawStrLit
            | TokenKind::LineComment
            | TokenKind::BlockComment
            | TokenKind::Unknown
    ) || is_keyword(k)
        || is_operator_or_punct(k)
}

fn is_keyword(k: TokenKind) -> bool {
    use TokenKind::*;
    matches!(
        k,
        Fn | Let
            | Mut
            | Const
            | Struct
            | Pub
            | PubCrate
            | PubSuper
            | If
            | Else
            | Match
            | For
            | While
            | Return
            | Use
            | Mod
            | Impl
            | Trait
            | Type
            | Enum
            | Macro
            | Expect
            | Actual
            | As
            | In
            | Break
            | Continue
            | Self_
            | Super
            | Crate
            | True
            | False
            | Where
            | Move
            | Async
            | Await
            | Dyn
            | Unsafe
            | Loop
            | Static
            | Extern
            | Ref
    )
}

fn is_operator_or_punct(k: TokenKind) -> bool {
    use TokenKind::*;
    matches!(
        k,
        LBrace
            | RBrace
            | LParen
            | RParen
            | LBracket
            | RBracket
            | Comma
            | Semi
            | Colon
            | ColonColon
            | Arrow
            | FatArrow
            | Dot
            | DotDot
            | DotDotEq
            | DotDotDot
            | Question
            | At
            | Pound
            | Dollar
            | Tilde
            | Plus
            | Minus
            | Star
            | Slash
            | Percent
            | Caret
            | And
            | Or
            | Shl
            | Shr
            | AndAnd
            | OrOr
            | Bang
            | Eq
            | EqEq
            | NotEq
            | Lt
            | Gt
            | LtEq
            | GtEq
            | PlusEq
            | MinusEq
            | StarEq
            | SlashEq
            | PercentEq
            | CaretEq
            | AndEq
            | OrEq
            | ShlEq
            | ShrEq
    )
}
