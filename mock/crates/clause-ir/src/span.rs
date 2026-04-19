//! Source spans and their identity types.
//!
//! `Span` carries a `(file, start, end)` byte range into the source
//! buffer addressed by `FileId`. Byte offsets are used rather than char
//! offsets so that diagnostics round-trip cleanly against LSP's
//! byte-based protocol; char-offset conversion is a downstream renderer
//! concern.
//!
//! Both `FileId` and `ByteOffset` are `#[repr(transparent)]` newtypes
//! around `u32`. Consumers treat them as opaque identifiers; the inner
//! field is public only to keep construction cheap in the few call
//! sites that actually synthesise them (workspace driver, lexer).

/// Dense interned identifier for a loaded source file.
///
/// Plan-time assigned by the workspace driver. A `FileId` is only
/// meaningful within the workspace session that minted it.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct FileId(pub u32);

/// A byte offset into a source buffer.
///
/// Stored as `u32`; sources larger than 4 GiB are out of scope.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ByteOffset(pub u32);

/// A half-open byte range `[start, end)` inside the source file `file`.
///
/// Spans are `Copy`; cloning is cheap. An empty span (`start == end`)
/// is valid and used for synthetic diagnostics attached to a single
/// point.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Span {
    pub file: FileId,
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl Span {
    /// Construct a new span from raw parts.
    pub const fn new(file: FileId, start: ByteOffset, end: ByteOffset) -> Self {
        Self { file, start, end }
    }

    /// Length of the span in bytes.
    pub const fn len(self) -> u32 {
        self.end.0.saturating_sub(self.start.0)
    }

    /// `true` if the span covers zero bytes.
    pub const fn is_empty(self) -> bool {
        self.start.0 == self.end.0
    }
}
