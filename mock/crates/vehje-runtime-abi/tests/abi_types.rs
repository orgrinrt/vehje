//! ABI type surface tests.
//!
//! Assert the discriminants of `VehjeResult`, construct a
//! `VehjeDiagnostic`, and verify variants compare unequal.

use vehje_runtime_abi::{AbiSpan, VehjeDiagnostic, VehjeDiagnosticKind, VehjeResult};

#[test]
fn vehje_result_ok_is_zero() {
    assert_eq!(VehjeResult::Ok as i32, 0);
}

#[test]
fn vehje_result_err_is_negative() {
    assert_eq!(VehjeResult::Err as i32, -1);
}

#[test]
fn vehje_result_variants_distinct() {
    assert!(VehjeResult::Ok != VehjeResult::Err);
    assert!(VehjeResult::Ok != VehjeResult::NullHandle);
    assert!(VehjeResult::Ok != VehjeResult::InvalidInput);
    assert!(VehjeResult::Err != VehjeResult::NullHandle);
    assert!(VehjeResult::Err != VehjeResult::InvalidInput);
    assert!(VehjeResult::NullHandle != VehjeResult::InvalidInput);
}

#[test]
fn vehje_diagnostic_construction() {
    let message: &[u8] = b"stub message";
    let diag = VehjeDiagnostic {
        span:        AbiSpan {
            file:  1,
            start: 2,
            end:   7,
        },
        message:     message.as_ptr(),
        message_len: arvo::USize(message.len()),
        kind:        VehjeDiagnosticKind::Warning,
    };
    assert_eq!(diag.kind, VehjeDiagnosticKind::Warning);
    assert_eq!(diag.span.file, 1);
    assert_eq!(diag.span.start, 2);
    assert_eq!(diag.span.end, 7);
    assert_eq!(diag.message_len, arvo::USize(message.len()));
}
