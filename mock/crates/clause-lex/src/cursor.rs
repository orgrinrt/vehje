//! Byte cursor with UTF-8-safe peek.
//!
//! `Cursor<'a>` walks a `&'a [u8]` source buffer left-to-right. The
//! two `_byte` accessors work at raw-byte granularity (cheap, what the
//! ASCII tokenisers use); `peek` / `bump` decode a UTF-8 code point at
//! the current position and return `Maybe<char>`.
//!
//! A malformed UTF-8 sequence makes `peek` / `bump` return `Maybe::Isnt`
//! without advancing; the byte-level accessors will still see the
//! offending bytes and the lexer's error path can classify them as
//! `Unknown`.

use vehje_ir::ByteOffset;
use notko::Maybe;

/// Forward-only byte cursor into the source slice.
///
/// The cursor tracks its position as a `u32`; the source is expected
/// to be under 4 GiB. Construction is infallible; `pos` starts at 0.
#[derive(Copy, Clone, Debug)]
pub struct Cursor<'a> {
    src: &'a [u8],
    pos: u32,
}

impl<'a> Cursor<'a> {
    /// Construct a cursor over `src` positioned at byte 0.
    pub const fn new(src: &'a [u8]) -> Self {
        Self { src, pos: 0 }
    }

    /// Current position as a `ByteOffset`.
    pub const fn pos(&self) -> ByteOffset {
        ByteOffset(self.pos)
    }

    /// Current position as a raw `u32`.
    pub const fn pos_u32(&self) -> u32 {
        self.pos
    }

    /// Full source slice. Lexers that need a sub-slice for keyword
    /// lookup reach through this accessor.
    pub const fn src(&self) -> &'a [u8] {
        self.src
    }

    /// `true` if the cursor is at the end of the source.
    pub const fn is_eof(&self) -> bool {
        self.pos as usize >= self.src.len()
    }

    /// Length of the source in bytes.
    pub const fn len(&self) -> u32 {
        self.src.len() as u32
    }

    /// Peek the byte at the cursor without advancing.
    pub fn peek_byte(&self) -> Maybe<u8> {
        match self.src.get(self.pos as usize).copied() {
            Some(b) => Maybe::Is(b),
            None => Maybe::Isnt,
        }
    }

    /// Peek the byte `n` steps ahead without advancing.
    pub fn peek_byte_at(&self, n: u32) -> Maybe<u8> {
        match self.src.get((self.pos + n) as usize).copied() {
            Some(b) => Maybe::Is(b),
            None => Maybe::Isnt,
        }
    }

    /// Advance one byte, returning it.
    pub fn bump_byte(&mut self) -> Maybe<u8> {
        let b = match self.peek_byte() {
            Maybe::Is(b) => b,
            Maybe::Isnt => return Maybe::Isnt,
        };
        self.pos += 1;
        Maybe::Is(b)
    }

    /// Peek the `char` at the cursor, decoding UTF-8. Returns
    /// `Maybe::Isnt` at EOF or on a malformed UTF-8 sequence. The
    /// cursor is not advanced in either case.
    pub fn peek(&self) -> Maybe<char> {
        match decode_utf8(&self.src[self.pos as usize..]) {
            Maybe::Is((ch, _)) => Maybe::Is(ch),
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// Advance one UTF-8-encoded code point, returning it.
    ///
    /// Returns `Maybe::Isnt` at EOF or on a malformed UTF-8 sequence.
    /// On malformed input the cursor does not advance; callers that
    /// want to make progress in that case should use `bump_byte`.
    pub fn bump(&mut self) -> Maybe<char> {
        let (ch, len) = match decode_utf8(&self.src[self.pos as usize..]) {
            Maybe::Is(v) => v,
            Maybe::Isnt => return Maybe::Isnt,
        };
        self.pos += len as u32;
        Maybe::Is(ch)
    }

    /// Advance exactly `n` bytes unconditionally.
    ///
    /// Used by tokenisers that have already peeked the shape they are
    /// consuming (e.g. a two-byte operator). The caller is responsible
    /// for ensuring the skipped bytes form a valid unit.
    pub fn bump_n(&mut self, n: u32) {
        let target = self.pos + n;
        self.pos = if target as usize > self.src.len() {
            self.src.len() as u32
        } else {
            target
        };
    }
}

/// Decode the first UTF-8 code point from `bytes`.
///
/// Returns the `char` and the number of bytes consumed on success.
/// Returns `Maybe::Isnt` for an empty slice or a malformed sequence.
fn decode_utf8(bytes: &[u8]) -> Maybe<(char, u8)> {
    let b0 = match bytes.first() {
        Some(b) => *b,
        None => return Maybe::Isnt,
    };
    // ASCII fast path.
    if b0 < 0x80 {
        return Maybe::Is((b0 as char, 1));
    }
    let (len, init) = if b0 & 0b1110_0000 == 0b1100_0000 {
        (2u8, (b0 & 0b0001_1111) as u32)
    } else if b0 & 0b1111_0000 == 0b1110_0000 {
        (3u8, (b0 & 0b0000_1111) as u32)
    } else if b0 & 0b1111_1000 == 0b1111_0000 {
        (4u8, (b0 & 0b0000_0111) as u32)
    } else {
        return Maybe::Isnt;
    };
    if bytes.len() < len as usize {
        return Maybe::Isnt;
    }
    let mut acc = init;
    let mut i = 1;
    while i < len as usize {
        let b = bytes[i];
        if b & 0b1100_0000 != 0b1000_0000 {
            return Maybe::Isnt;
        }
        acc = (acc << 6) | (b & 0b0011_1111) as u32;
        i += 1;
    }
    let ch = match char::from_u32(acc) {
        Some(c) => c,
        None => return Maybe::Isnt,
    };
    Maybe::Is((ch, len))
}
