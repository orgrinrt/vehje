//! vehje-fixpoint, the framework's compile-stage relational engine.
//!
//! A generic, no-alloc, semi-naive relational-fixpoint evaluator with
//! congruence support, over caller-provided columnar storage. It reuses the
//! `hilavitkutin-api` columnar contract as its storage vocabulary and the
//! `arvo` bitmask and hash primitives as its algorithms, and adds the
//! semi-naive deltas, congruence closure, and content-addressed incremental
//! keying those do not carry.
//!
//! The engine is deliberately free of any IR dependency: it names no IR type
//! and stays a mechanism, not a client. Each client (the lease and
//! reachability inference, the equality-saturation lowering, the diagnostics
//! witness reconstruction) defines its own relation schema over it.
//!
//! `#![no_std]`, no alloc: relations, e-class parents, and artifact stores all
//! live in caller-provided storage.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod congruence;
pub mod engine;
pub mod incr;

pub use congruence::Congruence;
pub use engine::{Delta, Engine, Relation, RelationSet, Rule};
pub use incr::{ArtifactSet, Incremental, Store};
