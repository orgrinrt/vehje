//! ABI type surface tests.
//!
//! Assert the discriminants of `ClauseResult`, construct a
//! `ClauseDiagnostic`, and verify variants compare unequal.

use clause_runtime_abi::{AbiSpan, ClauseDiagnostic, ClauseDiagnosticKind, ClauseResult};

#[test]
fn clause_result_ok_is_zero() {
    assert_eq!(ClauseResult::Ok as i32, 0);
}

#[test]
fn clause_result_err_is_negative() {
    assert_eq!(ClauseResult::Err as i32, -1);
}

#[test]
fn clause_result_variants_distinct() {
    assert!(ClauseResult::Ok != ClauseResult::Err);
    assert!(ClauseResult::Ok != ClauseResult::NullHandle);
    assert!(ClauseResult::Ok != ClauseResult::InvalidInput);
    assert!(ClauseResult::Err != ClauseResult::NullHandle);
    assert!(ClauseResult::Err != ClauseResult::InvalidInput);
    assert!(ClauseResult::NullHandle != ClauseResult::InvalidInput);
}

#[test]
fn clause_diagnostic_construction() {
    let message: &[u8] = b"stub message";
    let diag = ClauseDiagnostic {
        span: AbiSpan { file: 1, start: 2, end: 7 },
        message: message.as_ptr(),
        message_len: message.len(),
        kind: ClauseDiagnosticKind::Warning,
    };
    assert_eq!(diag.kind, ClauseDiagnosticKind::Warning);
    assert_eq!(diag.span.file, 1);
    assert_eq!(diag.span.start, 2);
    assert_eq!(diag.span.end, 7);
    assert_eq!(diag.message_len, message.len());
}
