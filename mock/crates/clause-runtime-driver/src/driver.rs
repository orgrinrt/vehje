//! `RuntimeDriver` — compiler-side orchestrator wrapping a
//! runtime handle.
//!
//! Skeleton round: placeholder struct with no state. The
//! real state body (`Arc<dyn RuntimeHandle>` + per-session
//! arena + cached diagnostic translations) is BACKLOG.
//!
//! This round exists to establish the public type surface;
//! concrete state fields land with `libloading` integration.

/// Compiler-side runtime orchestrator.
///
/// Skeleton: carries no state. Construct via
/// `RuntimeDriver::new()`. The follow-up round (dlopen
/// integration) grows the private state with a handle field
/// and per-session caches.
#[derive(Debug)]
pub struct RuntimeDriver {
    _private: (),
}

impl RuntimeDriver {
    /// Construct a fresh driver.
    ///
    /// Skeleton: returns an empty placeholder. The
    /// follow-up round wires actual initialisation (loading
    /// a dylib, resolving symbols, preparing the per-session
    /// arena).
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for RuntimeDriver {
    fn default() -> Self {
        Self::new()
    }
}
