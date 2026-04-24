//! `ClauseDiagnostic` — `#[repr(C)]` carrier for a single
//! runtime-emitted diagnostic.
//!
//! The diagnostic carries a flat `AbiSpan` (file / start / end
//! as bare `u32`s — the richer `FileId` / `ByteOffset` wrapper
//! from `vehje-ir` is deferred to the R2-retrofit round), a
//! pointer-plus-length message, and a kind discriminant. The
//! pointer is borrowed: the caller owns the backing string,
//! and the pointer stays valid only as long as the caller's
//! buffer does.
//!
//! Skeleton round: no diagnostic is ever emitted (the runtime
//! returns `Err` without populating a diagnostic). The type is
//! declared so downstream code can compile against the ABI
//! surface that future rounds will populate.

/// Flat ABI span — `(file, start, end)` as bare `u32`s.
///
/// The richer `FileId` / `ByteOffset` wrappers live in
/// `vehje_ir::Span`. This round carries the triple directly
/// to keep the ABI surface minimal; the driver-side
/// `AbiSpan` → `vehje_ir::Span` conversion is BACKLOG.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbiSpan {
    /// File identifier (matches `vehje_ir::FileId::0`).
    pub file: u32, // lint:allow(arvo-types-only) tracked: #207
    /// Start byte offset (matches
    /// `vehje_ir::ByteOffset::0`).
    pub start: u32, // lint:allow(arvo-types-only) tracked: #207
    /// End byte offset (exclusive).
    pub end: u32, // lint:allow(arvo-types-only) tracked: #207
}

/// Diagnostic kind — mirrors `vehje_ir::Severity` but carries
/// only the three kinds the runtime emits (Help / Warning-vs-
/// info-splits are a compiler-side concern).
#[repr(i32)] // lint:allow(arvo-types-only) tracked: #207
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ClauseDiagnosticKind {
    /// Error — execution fails, result code is non-`Ok`.
    Error = 0,
    /// Warning — execution succeeds but emits a notice.
    Warning = 1,
    /// Info — diagnostic has no severity consequence.
    Info = 2,
}

/// Single diagnostic record, `#[repr(C)]` for FFI.
///
/// `message` is a borrowed pointer to a UTF-8 byte sequence of
/// length `message_len`. The callee does not own the buffer;
/// callers must keep it alive for the duration of the FFI
/// call that consumes the diagnostic.
///
/// `Eq` / `PartialEq` / `Hash` cannot be derived because of
/// the raw pointer field; a manual comparator (compare the
/// pointed-to byte slice rather than the pointer identity)
/// is BACKLOG.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClauseDiagnostic {
    /// Span the diagnostic refers to.
    pub span: AbiSpan,
    /// Pointer to the message bytes (UTF-8, borrowed).
    pub message: *const u8, // lint:allow(arvo-types-only) tracked: #207
    /// Length of the message in bytes.
    pub message_len: arvo::USize,
    /// Severity-ish classification.
    pub kind: ClauseDiagnosticKind,
}
