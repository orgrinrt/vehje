//! `extern "C"` entry-point tests.
//!
//! Verify the skeleton stubs: `clause_runtime_new` returns
//! null, `clause_runtime_free(null)` is safe, and
//! `clause_runtime_execute(null, _, 0)` returns `Err`.

use clause_runtime_abi::ClauseResult;
use clause_runtime_abi::exports::{clause_runtime_execute, clause_runtime_free, clause_runtime_new};

#[test]
fn new_returns_null_in_skeleton() {
    let rt = clause_runtime_new();
    assert!(rt.is_null());
}

#[test]
fn free_null_does_not_crash() {
    clause_runtime_free(core::ptr::null_mut());
}

#[test]
fn execute_null_returns_err() {
    let result = clause_runtime_execute(core::ptr::null_mut(), core::ptr::null(), 0);
    assert_eq!(result, ClauseResult::Err);
}
