//! Error-type tests.
//!
//! Construct every variant of both `LoaderError` and
//! `DriverError`; verify variants compare unequal and
//! `ExecuteFailed` round-trips its code payload.

use vehje_runtime_driver::{DriverError, LoaderError};

#[test]
fn loader_error_variants_distinct() {
    assert!(LoaderError::NotImplemented != LoaderError::SymbolNotFound);
    assert!(LoaderError::NotImplemented != LoaderError::LoadFailed);
    assert!(LoaderError::SymbolNotFound != LoaderError::LoadFailed);
}

#[test]
fn driver_error_variants_distinct() {
    assert!(DriverError::NotImplemented != DriverError::HandleNull);
    assert!(DriverError::NotImplemented != DriverError::ExecuteFailed { code: -1 });
    assert!(DriverError::HandleNull != DriverError::ExecuteFailed { code: -1 });
    assert!(DriverError::ExecuteFailed { code: -1 } != DriverError::ExecuteFailed { code: -2 });
}

#[test]
fn driver_error_execute_failed_carries_code() {
    let err = DriverError::ExecuteFailed { code: -7 };
    match err {
        DriverError::ExecuteFailed { code } => assert_eq!(code, -7),
        _ => panic!("expected ExecuteFailed variant"),
    }
}

#[test]
fn loader_error_construction() {
    let e1 = LoaderError::NotImplemented;
    let e2 = LoaderError::SymbolNotFound;
    let e3 = LoaderError::LoadFailed;
    for err in [e1, e2, e3] {
        match err {
            LoaderError::NotImplemented
            | LoaderError::SymbolNotFound
            | LoaderError::LoadFailed => {}
        }
    }
}
