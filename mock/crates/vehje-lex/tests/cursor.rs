//! Cursor peek / bump round-trips.

use vehje_lex::Cursor;

#[test]
fn empty_source_is_eof() {
    let c = Cursor::new(b"");
    assert!(c.is_eof());
    assert_eq!(c.peek_byte(), None);
    assert_eq!(c.peek(), None);
}

#[test]
fn peek_does_not_advance() {
    let c = Cursor::new(b"abc");
    assert_eq!(c.peek_byte(), Some(b'a'));
    assert_eq!(c.peek_byte(), Some(b'a'));
    assert_eq!(c.peek(), Some('a'));
    assert_eq!(c.peek(), Some('a'));
}

#[test]
fn bump_byte_advances_one() {
    let mut c = Cursor::new(b"xyz");
    assert_eq!(c.bump_byte(), Some(b'x'));
    assert_eq!(c.bump_byte(), Some(b'y'));
    assert_eq!(c.bump_byte(), Some(b'z'));
    assert_eq!(c.bump_byte(), None);
    assert!(c.is_eof());
}

#[test]
fn bump_returns_char_and_advances() {
    let mut c = Cursor::new("héllo".as_bytes());
    // 'h'
    assert_eq!(c.bump(), Some('h'));
    // 'é' is 2 bytes in UTF-8.
    assert_eq!(c.bump(), Some('é'));
    // l, l, o
    assert_eq!(c.bump(), Some('l'));
    assert_eq!(c.bump(), Some('l'));
    assert_eq!(c.bump(), Some('o'));
    assert_eq!(c.bump(), None);
}

#[test]
fn peek_at_offset() {
    let c = Cursor::new(b"abc");
    assert_eq!(c.peek_byte_at(0), Some(b'a'));
    assert_eq!(c.peek_byte_at(1), Some(b'b'));
    assert_eq!(c.peek_byte_at(2), Some(b'c'));
    assert_eq!(c.peek_byte_at(3), None);
}

#[test]
fn bump_n_advances_unchecked() {
    let mut c = Cursor::new(b"abcde");
    c.bump_n(2);
    assert_eq!(c.pos_u32(), 2);
    assert_eq!(c.peek_byte(), Some(b'c'));
    c.bump_n(10); // overshoot clamps to end
    assert!(c.is_eof());
}

#[test]
fn pos_tracks_advance() {
    let mut c = Cursor::new(b"abc");
    assert_eq!(c.pos_u32(), 0);
    c.bump_byte();
    assert_eq!(c.pos_u32(), 1);
    c.bump_byte();
    assert_eq!(c.pos_u32(), 2);
}
