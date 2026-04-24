#![no_std]

//! vehje-schedule — pass DAG + scheduler harness for the Vehje
//! compiler.
//!
//! Consumes `arvo-graph` for DAG topology and the
//! `hilavitkutin-api` WorkUnit contracts for per-pass access-set
//! declaration. Hosts the `CompilerSchedule` type that the driver
//! assembles from lex + parse + resolve + typecheck + codegen
//! passes, and exposes the entry point consumers call to run a
//! compilation.
//!
//! Skeleton round: ships the marker `CompilerSchedule` type and
//! the `ScheduleError` carrier. Real pass wiring (M0.2, tracked
//! under #131) follows in a dedicated round that re-frames each
//! compiler phase as a hilavitkutin WorkUnit.

/// Placeholder type for the full pass schedule the driver
/// assembles across every compiler phase.
///
/// Skeleton shape: unit struct. The M0.2 round (see #131) grows
/// this into a concrete DAG over WorkUnit handles, fed by the
/// `arvo-graph` topology primitive and dispatched via the
/// hilavitkutin scheduler.
pub struct CompilerSchedule;

/// Error carrier returned by schedule construction and execution.
///
/// Skeleton: one variant flagging the not-yet-wired state.
/// Expands in M0.2 (#131) to cover DAG cycle detection,
/// access-set conflict reporting, and missing-pass diagnostics.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScheduleError {
    /// The pass DAG has not been populated yet. Returned by the
    /// skeleton `CompilerSchedule::run` until M0.2 (#131) wires
    /// the real pass graph.
    Unimplemented,
}

impl CompilerSchedule {
    /// Construct an empty schedule harness.
    pub const fn new() -> Self {
        Self
    }
}

impl Default for CompilerSchedule {
    fn default() -> Self {
        Self::new()
    }
}
