//! vehje-schedule, the framework's compile-side pass scheduler.
//!
//! A DAG of Core passes over `arvo-graph`, ordered by declared
//! dependencies, walked once (single-shot topological walks). This is the
//! compile side; the runtime side uses hilavitkutin's morsel-driven
//! execution, not this crate.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::Maybe;

/// A registered compile pass: a name, and the passes it depends on.
///
/// A pass reads and writes the IR; resolve and check are the framework
/// passes, a consumer's family passes register alongside them. The read
/// and write sets are a later addition (they drive the `arvo-bitmask`
/// adjacency); M0 carries the dependency edges by name.
// FIXME: add the per-pass read/write sets over the IR (arvo-bitmask
// AccessMask) that let the scheduler prove pass ordering, not just name
// dependencies. Lands with the real DAG wiring.
pub trait Pass {
    /// The pass name, for diagnostics and dependency reference.
    const NAME: &'static str;
}

/// A schedule diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ScheduleError {
    /// The pass DAG contains a cycle.
    Cycle,
    /// A declared dependency names no registered pass.
    MissingDependency,
}

/// A topological order of passes.
///
/// Produced by the pass DAG's `arvo-graph` topological walk; a cycle is
/// the `valid_count < N` signal that sort returns.
// FIXME: build the arvo-bitmask BitMatrix adjacency from the registered
// passes' dependency edges and run arvo-graph::topo_sort over it, mapping
// valid_count < N to ScheduleError::Cycle. M0 defines the surface; the
// DAG wiring is the next behavior gate.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Schedule;

impl Schedule {
    /// Build a schedule over a set of registered passes.
    pub fn build() -> Maybe<Self> {
        Maybe::Is(Self)
    }
}
