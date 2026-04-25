//! `RuntimeLoader`, entry point for locating + loading a
//! runtime dylib.
//!
//! Skeleton round: `load` always returns
//! `Outcome::Err(LoaderError::NotImplemented)`. The follow-up
//! round wires the real path:
//!
//! 1. Check `CLAUSE_RUNTIME_PATH` env var.
//! 2. Fall back to a next-to-compiler-binary convention.
//! 3. Fall back to an embedded statically-linked backend.
//!
//! All of those are BACKLOG; this round establishes the
//! surface signature only.

use std::path::Path;

use notko::Outcome;

use crate::error::LoaderError;
use crate::handle::RuntimeHandle;

/// Loader entry. Zero-sized; associated functions only.
#[derive(Debug)]
pub struct RuntimeLoader;

impl RuntimeLoader {
    /// Locate + load the runtime dylib at `path`.
    ///
    /// Skeleton: returns `Outcome::Err(LoaderError::NotImplemented)`
    /// regardless of the path contents (including the empty
    /// string, a nonexistent file, or a real file). Follow-up
    /// round wires `libloading::Library::new(path)` + `dlsym`
    /// symbol resolution.
    pub fn load(path: &Path) -> Outcome<Box<dyn RuntimeHandle>, LoaderError> {
        let _ = path;
        Outcome::Err(LoaderError::NotImplemented)
    }
}
