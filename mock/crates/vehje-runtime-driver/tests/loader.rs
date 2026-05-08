//! Loader tests.
//!
//! Skeleton round: `RuntimeLoader::load` always returns
//! `Err(LoaderError::NotImplemented)`, assert the error
//! variant for two different input paths.

use std::path::Path;

use vehje_runtime_driver::{LoaderError, RuntimeLoader};

#[test]
fn load_nonexistent_returns_not_implemented() {
    let result = RuntimeLoader::load(Path::new("/nonexistent/libvehje_runtime.dylib"));
    assert!(matches!(result, Err(LoaderError::NotImplemented)));
}

#[test]
fn load_bogus_returns_not_implemented() {
    let result = RuntimeLoader::load(Path::new("bogus-name"));
    assert!(matches!(result, Err(LoaderError::NotImplemented)));
}
