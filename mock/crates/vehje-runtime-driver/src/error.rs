//! Error enums for the loader + driver surfaces.
//!
//! Two separate enums because the failure modes are
//! disjoint: `LoaderError` covers library location /
//! resolution; `DriverError` covers post-load execution.
//! Both are `Clone + Eq + PartialEq + Debug`. `DriverError`
//! is NOT `Copy` because `ExecuteFailed` carries a data
//! payload the caller consumes via pattern-match.

/// Failures raised while locating and loading a runtime
/// library.
///
/// Skeleton round only ever returns `NotImplemented`;
/// `SymbolNotFound` and `LoadFailed` are declared for the
/// follow-up round that ships real `libloading` integration.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LoaderError {
    /// The loader does not yet know how to perform the real
    /// load operation. Skeleton sentinel, removed once
    /// `libloading` integration lands.
    NotImplemented,
    /// Library was found and opened, but a required
    /// `extern "C"` symbol was not resolvable.
    SymbolNotFound,
    /// The operating system refused the load (missing file,
    /// permission denied, architecture mismatch, …).
    LoadFailed,
}

/// Failures raised after a runtime has been loaded.
///
/// Skeleton round never constructs `HandleNull` or
/// `ExecuteFailed`; they exist so the follow-up round can
/// surface concrete failures without introducing a fresh
/// enum.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DriverError {
    /// The driver path did not reach a real execute body.
    /// Skeleton sentinel, removed once the driver wraps a
    /// concrete runtime handle.
    NotImplemented,
    /// Caller invoked the driver with a null / closed
    /// runtime handle.
    HandleNull,
    /// The runtime returned a non-`Ok` result code. The
    /// `code` matches the `VehjeResult` discriminant
    /// received over the FFI.
    ExecuteFailed {
        /// The raw integer result code from the FFI.
        code: i32,  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    },
}
