//! `extern "C"` entry-point tests.
//!
//! Verify the skeleton stubs: `vehje_runtime_new` returns
//! null, `vehje_runtime_free(null)` is safe, and
//! `vehje_runtime_execute(null, _, 0)` returns `Err`.

use vehje_runtime_abi::VehjeResult;
use vehje_runtime_abi::exports::{vehje_runtime_execute, vehje_runtime_free, vehje_runtime_new};

#[test]
fn new_returns_null_in_skeleton() {
    let rt = vehje_runtime_new();
    assert!(rt.is_null());
}

#[test]
fn free_null_does_not_crash() {
    vehje_runtime_free(core::ptr::null_mut());
}

#[test]
fn execute_null_returns_err() {
    let result = vehje_runtime_execute(core::ptr::null_mut(), core::ptr::null(), arvo::USize(0)); // lint:allow(no-bare-numeric) reason: zero-length input sentinel for null-input test; tracked: #412
    assert_eq!(result, VehjeResult::Err);
}
