//! Tests for the embedded runtime and its transfer sink.

use super::*;

/// A stand-in runtime that writes a fixed payload through the sink, so the
/// transfer contract is testable without any built artifact.
mod fake {
    use super::*;

    /// The bytes the stand-in commits.
    pub const PAYLOAD: &[u8] = &[1, 2, 3, 4]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test payload bytes at the FFI boundary; tracked: #207

    pub extern "C" fn new(_scratch: *mut u8, _len: usize) -> *mut c_void { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI handle; tracked: #207
        core::ptr::null_mut()
    }

    pub extern "C" fn free(_rt: *mut c_void) {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI handle; tracked: #207

    /// Reserve, write, commit: the whole-value single-reserve case.
    pub extern "C" fn execute( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI entry; tracked: #207
        _rt: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _input: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        sink: *const Sink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _host: *const Host, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        // SAFETY: the driver passes a live sink for the duration of the call.
        let s = unsafe { &*sink };
        let dst = (s.reserve)(s.userdata, PAYLOAD.len());
        if dst.is_null() {
            return -1; // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
        }
        // SAFETY: reserve returned at least PAYLOAD.len() writable bytes.
        unsafe { core::ptr::copy_nonoverlapping(PAYLOAD.as_ptr(), dst, PAYLOAD.len()) };
        (s.commit)(s.userdata, PAYLOAD.len());
        0 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }

    /// A runtime that always reports failure, without touching the sink.
    pub extern "C" fn execute_fails( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI entry; tracked: #207
        _rt: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _input: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _sink: *const Sink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _host: *const Host, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        -1 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }

    pub fn entries() -> RuntimeEntries {
        RuntimeEntries { new, free, execute }
    }

    pub fn failing_entries() -> RuntimeEntries {
        RuntimeEntries { new, free, execute: execute_fails }
    }
}

#[test]
fn a_committed_value_lands_in_the_lent_buffer() {
    let rt = Runtime::new(fake::entries());
    let mut out = [0u8; 16]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test output buffer bytes; tracked: #207
    let written = match rt.execute(&[], &mut out) {
        Outcome::Ok(n) => n,
        Outcome::Err(_) => panic!("the stand-in commits successfully"),
    };
    assert_eq!(written, USize(fake::PAYLOAD.len()));
    assert_eq!(&out[..written.0], fake::PAYLOAD);
}

#[test]
fn a_buffer_too_small_is_refused_rather_than_overrun() {
    // The sink returns null when the chunk does not fit, which the runtime
    // reads as backpressure. The stand-in reports failure rather than writing
    // past the end, and the lent buffer is untouched.
    let rt = Runtime::new(fake::entries());
    let mut out = [0u8; 2]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: deliberately undersized test buffer; tracked: #207
    assert!(matches!(rt.execute(&[], &mut out), Outcome::Err(DriverError::ExecuteFailed)));
    assert_eq!(out, [0, 0]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the buffer must be untouched; tracked: #207
}

#[test]
fn a_runtime_failure_is_named_not_swallowed() {
    let rt = Runtime::new(fake::failing_entries());
    let mut out = [0u8; 16]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test output buffer bytes; tracked: #207
    assert!(matches!(rt.execute(&[], &mut out), Outcome::Err(DriverError::ExecuteFailed)));
}

#[test]
fn nothing_committed_reads_as_zero_bytes() {
    // A program the host kept nothing from: the call succeeds and commits
    // nothing, which is a length of zero rather than an error.
    extern "C" fn quiet( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI entry; tracked: #207
        _rt: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _input: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _sink: *const Sink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _host: *const Host, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        0 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }
    let rt = Runtime::new(RuntimeEntries { new: fake::new, free: fake::free, execute: quiet });
    let mut out = [0u8; 16]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test output buffer bytes; tracked: #207
    assert!(matches!(rt.execute(&[], &mut out), Outcome::Ok(n) if n == USize(0)));
}

#[cfg(feature = "dynamic")]
mod dynamic_tests {
    use super::*;

    #[test]
    fn loading_something_that_is_not_a_library_is_named() {
        let bad = b"/nonexistent/not-a-library.dylib\0"; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: platform path bytes at the loader boundary; tracked: #207
        assert!(matches!(super::dynamic::load(bad), Outcome::Err(DriverError::LoadFailed)));
    }
}
