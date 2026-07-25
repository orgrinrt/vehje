//! Lexer tests against the normative grammar's lexical section.

use arvo::USize;
use clause_frontend::{lex, Keyword, LexError, Token, TokenKind};
use notko::Outcome;

fn kinds(src: &str) -> ([TokenKind; 64], usize) {
    let mut buf = [Token::new(TokenKind::Eof, Default::default()); 64];
    let n = match lex(src, &mut buf) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("lex failed: {e:?}"),
    };
    let mut out = [TokenKind::Eof; 64];
    for i in 0..n {
        out[i] = buf[i].kind;
    }
    (out, n)
}

#[test]
fn a_generic_function_lexes_to_its_parts() {
    let (k, n) = kinds("pub fn id<T: Clone>(x: T) -> T { x }");
    assert_eq!(k[0], TokenKind::Keyword(Keyword::Pub));
    assert_eq!(k[1], TokenKind::Keyword(Keyword::Fn));
    assert_eq!(k[2], TokenKind::Ident);
    assert_eq!(k[3], TokenKind::Lt);
    assert_eq!(k[5], TokenKind::Colon);
    assert_eq!(k[7], TokenKind::Gt);
    assert_eq!(k[13], TokenKind::Arrow);
    assert_eq!(k[n - 1], TokenKind::Eof);
}

#[test]
fn the_multi_character_operators_take_their_longest_match() {
    // the whole correctness of the punctuation table: a longer token must not
    // lex as its own prefix
    let (k, _) = kinds("..= .. << <<= >>= == != <= >= && || ::");
    assert_eq!(k[0], TokenKind::DotDotEq);
    assert_eq!(k[1], TokenKind::DotDot);
    assert_eq!(k[2], TokenKind::Shl);
    assert_eq!(k[3], TokenKind::ShlEq);
    assert_eq!(k[4], TokenKind::ShrEq);
    assert_eq!(k[5], TokenKind::EqEq);
    assert_eq!(k[6], TokenKind::Ne);
    assert_eq!(k[7], TokenKind::Le);
    assert_eq!(k[8], TokenKind::Ge);
    assert_eq!(k[9], TokenKind::AndAnd);
    assert_eq!(k[10], TokenKind::OrOr);
    assert_eq!(k[11], TokenKind::ColonColon);
}

#[test]
fn a_dot_is_a_fraction_only_when_a_digit_follows() {
    // `1..2` is a range and `1.foo()` is a method call; only `1.5` is a float
    let (k, _) = kinds("1..2");
    assert_eq!(k[0], TokenKind::Int);
    assert_eq!(k[1], TokenKind::DotDot);
    assert_eq!(k[2], TokenKind::Int);

    let (k, _) = kinds("1.foo()");
    assert_eq!(k[0], TokenKind::Int);
    assert_eq!(k[1], TokenKind::Dot);

    let (k, _) = kinds("1.5e-3f64");
    assert_eq!(k[0], TokenKind::Float);
}

#[test]
fn a_numeric_suffix_rides_along_with_its_literal() {
    let (k, n) = kinds("42u32");
    assert_eq!(k[0], TokenKind::Int);
    assert_eq!(n, 2); // the literal and Eof: the suffix is not its own token
}

#[test]
fn raw_strings_match_their_hash_count_on_both_sides() {
    let (k, n) = kinds(r####"r#"a "quoted" thing"#"####);
    assert_eq!(k[0], TokenKind::RawStr);
    assert_eq!(n, 2);

    // an inner `"#` that does not close the outer count must not end it
    let (k, n) = kinds(r####"r##"has "# inside"##"####);
    assert_eq!(k[0], TokenKind::RawStr);
    assert_eq!(n, 2);
}

#[test]
fn block_comments_nest() {
    let (k, n) = kinds("1 /* outer /* inner */ still */ 2");
    assert_eq!(k[0], TokenKind::Int);
    assert_eq!(k[1], TokenKind::Int);
    assert_eq!(n, 3);
}

#[test]
fn a_doc_comment_is_a_token_but_a_line_comment_is_trivia() {
    // the doc pass consumes doc comments, so a lexer that folds them into
    // whitespace makes that pass impossible
    let (k, n) = kinds("/// doc\n// plain\nfn f() {}");
    assert_eq!(k[0], TokenKind::DocComment);
    assert_eq!(k[1], TokenKind::Keyword(Keyword::Fn));
    // doc comment, `fn f ( ) { }`, Eof
    assert_eq!(n, 8);
}

#[test]
fn an_escaped_quote_does_not_end_a_string() {
    let (k, n) = kinds(r#""a \" b""#);
    assert_eq!(k[0], TokenKind::Str);
    assert_eq!(n, 2);
}

#[test]
fn reserved_but_unproductive_words_still_lex_as_keywords() {
    // they lex so the parser can refuse them BY NAME rather than accepting them
    // as identifiers, which is what makes the reservation mean anything
    let (k, _) = kinds("dyn unsafe async await");
    assert_eq!(k[0], TokenKind::Keyword(Keyword::Dyn));
    assert!(matches!(k[0], TokenKind::Keyword(w) if w.is_reserved_unused()));
}

#[test]
fn an_unterminated_string_is_named_not_silently_accepted() {
    let mut buf = [Token::new(TokenKind::Eof, Default::default()); 8];
    assert!(matches!(
        lex(r#""no end"#, &mut buf),
        Outcome::Err(LexError::Unterminated { .. })
    ));
}

#[test]
fn a_full_token_buffer_is_a_refusal_not_an_overrun() {
    let mut buf = [Token::new(TokenKind::Eof, Default::default()); 2];
    assert!(matches!(
        lex("a b c d", &mut buf),
        Outcome::Err(LexError::TokenBufferFull { .. })
    ));
}

#[test]
fn spans_name_the_source_text_they_came_from() {
    let src = "let answer = 42;";
    let mut buf = [Token::new(TokenKind::Eof, Default::default()); 16];
    let n = match lex(src, &mut buf) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("{e:?}"),
    };
    assert_eq!(buf[1].span.of(src), "answer");
    assert_eq!(buf[3].span.of(src), "42");
    let _ = (n, USize(0));
}
