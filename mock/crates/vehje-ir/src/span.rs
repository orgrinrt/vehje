//! Source spans and their identity types.
//!
//! `Span` carries a `(file, start, end)` byte range into the source
//! buffer addressed by `FileId`. Byte offsets are used rather than char
//! offsets so diagnostics round-trip against byte-based protocols; char
//! conversion is a downstream renderer concern.
//!
//! `FileId` and `ByteOffset` are 32-bit source-domain values (sources
//! larger than 4 GiB are out of scope), carried as `arvo::Uint<32, Hot>`.

use arvo::strategy::Hot;
use arvo::{Bool, Uint};

/// A 32-bit source-domain scalar (file ids, byte offsets).
type U32 = Uint<32, Hot>;

/// Dense interned identifier for a loaded source file.
///
/// Assigned by the host at load time. Only meaningful within the
/// session that minted it.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct FileId(U32);

impl FileId {
    /// Construct from a raw 32-bit id.
    pub const fn new(id: U32) -> Self {
        Self(id)
    }

    /// The underlying id.
    pub const fn get(self) -> U32 {
        self.0
    }
}

/// A byte offset into a source buffer.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct ByteOffset(U32);

impl ByteOffset {
    /// Construct from a raw 32-bit offset.
    pub const fn new(offset: U32) -> Self {
        Self(offset)
    }

    /// The underlying offset.
    pub const fn get(self) -> U32 {
        self.0
    }
}

/// A half-open byte range `[start, end)` inside the source file `file`.
///
/// `Copy`. An empty span (`start == end`) is valid and used for
/// synthetic diagnostics attached to a single point.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Span {
    pub file: FileId,
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl Span {
    /// Construct a span from its parts.
    pub const fn new(file: FileId, start: ByteOffset, end: ByteOffset) -> Self {
        Self { file, start, end }
    }

    /// `true` if the span covers zero bytes.
    pub fn is_empty(self) -> Bool {
        Bool(self.start == self.end)
    }
}
