//! Cursor peek / bump round-trips.

use notko::Maybe;
use vehje_lex::Cursor;

#[test]
fn empty_source_is_eof() {
    let c = Cursor::new(b"");
    assert!(c.is_eof());
    assert_eq!(c.peek_byte(), Maybe::Isnt);
    assert_eq!(c.peek(), Maybe::Isnt);
}

#[test]
fn peek_does_not_advance() {
    let c = Cursor::new(b"abc");
    assert_eq!(c.peek_byte(), Maybe::Is(b'a'));
    assert_eq!(c.peek_byte(), Maybe::Is(b'a'));
    assert_eq!(c.peek(), Maybe::Is('a'));
    assert_eq!(c.peek(), Maybe::Is('a'));
}

#[test]
fn bump_byte_advances_one() {
    let mut c = Cursor::new(b"xyz");
    assert_eq!(c.bump_byte(), Maybe::Is(b'x'));
    assert_eq!(c.bump_byte(), Maybe::Is(b'y'));
    assert_eq!(c.bump_byte(), Maybe::Is(b'z'));
    assert_eq!(c.bump_byte(), Maybe::Isnt);
    assert!(c.is_eof());
}

#[test]
fn bump_returns_char_and_advances() {
    let mut c = Cursor::new("héllo".as_bytes());
    // 'h'
    assert_eq!(c.bump(), Maybe::Is('h'));
    // 'é' is 2 bytes in UTF-8.
    assert_eq!(c.bump(), Maybe::Is('é'));
    // l, l, o
    assert_eq!(c.bump(), Maybe::Is('l'));
    assert_eq!(c.bump(), Maybe::Is('l'));
    assert_eq!(c.bump(), Maybe::Is('o'));
    assert_eq!(c.bump(), Maybe::Isnt);
}

#[test]
fn peek_at_offset() {
    let c = Cursor::new(b"abc");
    assert_eq!(c.peek_byte_at(0), Maybe::Is(b'a'));
    assert_eq!(c.peek_byte_at(1), Maybe::Is(b'b'));
    assert_eq!(c.peek_byte_at(2), Maybe::Is(b'c'));
    assert_eq!(c.peek_byte_at(3), Maybe::Isnt);
}

#[test]
fn bump_n_advances_unchecked() {
    let mut c = Cursor::new(b"abcde");
    c.bump_n(2);
    assert_eq!(c.pos_u32(), 2);
    assert_eq!(c.peek_byte(), Maybe::Is(b'c'));
    c.bump_n(10); // overshoot clamps to end
    assert!(c.is_eof());
}

#[test]
fn pos_tracks_advance() {
    let mut c = Cursor::new(b"abc");
    assert_eq!(c.pos_u32(), 0);
    let _ = c.bump_byte();
    assert_eq!(c.pos_u32(), 1);
    let _ = c.bump_byte();
    assert_eq!(c.pos_u32(), 2);
}
