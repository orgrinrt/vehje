//! The transfer sink: one reserve-then-commit protocol for both boundaries.
//!
//! A produced value streams out through a reserve-then-commit contract, an
//! iteratee whose backpressure is inherent. In-process it is a `#[repr(C)]`
//! struct of two function pointers plus opaque userdata; out-of-process it is
//! the executable's stdout pipe, where the OS pipe buffer is the backpressure.
//! The backing memory is host-lent up front, so the runtime holds no output
//! memory after a call, which is what makes the wire form equal the in-process
//! form.
//!
//! The value serialises depth-first, children-before-parents, so streaming
//! happens at whole-subtree chunk boundaries under a residency budget: reserve
//! a chunk's worth of backing, write the chunk, commit the bytes written.

use core::ffi::c_void;

/// Reserve backing for the next chunk.
///
/// Given the opaque userdata and a byte `hint` for the chunk about to be
/// written, returns a pointer to at least `hint` writable bytes, or null if the
/// sink cannot satisfy the reservation (the backpressure signal). The runtime
/// writes the chunk into the returned region, then calls the [`CommitFn`].
pub type ReserveFn =
    extern "C" fn(userdata: *mut c_void, hint: usize) -> *mut u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI reserve callback; the C ABI is the contract; tracked: #207

/// Commit the bytes written into the last reservation.
///
/// Given the opaque userdata and the number of bytes actually written into the
/// region the [`ReserveFn`] returned, advances the sink. A commit smaller than
/// the hint is legal: the reserve is an upper bound.
pub type CommitFn =
    extern "C" fn(userdata: *mut c_void, written: usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI commit callback; the C ABI is the contract; tracked: #207

/// The reserve-then-commit transfer sink: two function pointers plus opaque
/// userdata.
///
/// The in-process form of the one transfer protocol. Whole-value mode is the
/// degenerate single-reserve case. The struct is `#[repr(C)]` so the host
/// passes it across the C ABI by value or by pointer; the runtime never retains
/// it past the call.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VehjeSink {
    /// Reserve backing for the next chunk.
    pub reserve: ReserveFn,
    /// Commit the bytes written into the last reservation.
    pub commit: CommitFn,
    /// Host-owned opaque context, passed back to both callbacks.
    pub userdata: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI opaque userdata pointer; the C ABI is the contract; tracked: #207
}

impl VehjeSink {
    /// Assemble a sink from its two callbacks and the host context.
    pub const fn new(reserve: ReserveFn, commit: CommitFn, userdata: *mut c_void) -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI opaque userdata pointer; the C ABI is the contract; tracked: #207
        Self { reserve, commit, userdata }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn reserve_stub(_userdata: *mut c_void, _hint: usize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI callback signature under test; tracked: #207
        core::ptr::null_mut()
    }

    extern "C" fn commit_stub(_userdata: *mut c_void, _written: usize) {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI callback signature under test; tracked: #207

    #[test]
    fn sink_is_two_pointers_plus_userdata() {
        // the struct shape is the contract: three pointer-sized fields, no
        // padding surprises on the platforms the C host targets.
        assert_eq!(
            core::mem::size_of::<VehjeSink>(),
            3 * core::mem::size_of::<*mut c_void>() // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pointer-count arithmetic in a layout assertion; tracked: #207
        );

        // it assembles from two callbacks and a null context, and the callbacks
        // round-trip back out as the same function pointers.
        let sink = VehjeSink::new(reserve_stub, commit_stub, core::ptr::null_mut());
        assert!(sink.userdata.is_null());
        assert!(core::ptr::eq(sink.reserve as *const (), reserve_stub as *const ()));
    }
}
