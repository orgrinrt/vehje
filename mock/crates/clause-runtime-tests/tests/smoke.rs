//! Smoke tests.
//!
//! Verify the harness fn propagates the loader's
//! `NotImplemented` error and that every re-exported type
//! from `clause-runtime-abi` + `clause-runtime-driver` can
//! be named + constructed from this crate.

use vehje_runtime_tests::{
    AbiSpan, ClauseDiagnostic, ClauseDiagnosticKind, ClauseResult, DriverError, LoaderError,
    RuntimeDriver, RuntimeLoader, test_runtime_roundtrip,
};

#[test]
fn test_runtime_roundtrip_returns_not_implemented() {
    let result = test_runtime_roundtrip();
    assert!(matches!(result, Err(LoaderError::NotImplemented)));
}

#[test]
fn types_are_accessible() {
    // Construct + name every re-exported type.
    let _driver = RuntimeDriver::new();
    let _driver_default: RuntimeDriver = RuntimeDriver::default();
    let _loader = RuntimeLoader;
    let _result = ClauseResult::Ok;
    let _kind = ClauseDiagnosticKind::Error;
    let _span = AbiSpan { file: 0, start: 0, end: 0 };
    let message: &[u8] = b"smoke";
    let _diag = ClauseDiagnostic {
        span: AbiSpan { file: 0, start: 0, end: 0 },
        message: message.as_ptr(),
        message_len: message.len(),
        kind: ClauseDiagnosticKind::Info,
    };
    let _loader_err = LoaderError::NotImplemented;
    let _driver_err = DriverError::NotImplemented;
}
