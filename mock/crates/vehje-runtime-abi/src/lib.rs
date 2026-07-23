//! vehje-runtime-abi, the framework's per-script wire family.
//!
//! What crosses the compiler-to-runtime boundary and in what representation.
//! The residual is a control-flow-graph-of-blocks program (a serialized node
//! arena, the child-index pool, the string blob, a block table, and a function
//! table); a produced value crosses back as a value-arena of the same
//! structural family; the reserve-then-commit sink is the one transfer protocol
//! for both boundaries; and the one runtime-W batched-column `extern "C"` entry
//! takes a column of records per call. Pure type surface plus the entry
//! signature, no runtime logic.
//!
//! One crate, not two, even though it carries a residual arena and a
//! value-arena: they are one structural family with one record-layout
//! definition. It splits into modules, [`wire`], [`value`], [`sink`], and
//! [`entry`], over the shared [`encode`] walk.
//!
//! `#![no_std]`, no alloc. Bare primitives appear only at the `#[repr(C)]` wire
//! boundary (the documented FFI exception); the crate's own logic uses arvo,
//! notko, and hilavitkutin-str types.

#![no_std]
// const_trait_impl (test-only): WATCH-allowed (unstable-features.md); the
// blob round-trip test builds interned strings with hilavitkutin-str's
// `str_const!`, which needs it.
#![cfg_attr(test, feature(const_trait_impl))]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod encode;
pub mod entry;
pub mod sink;
pub mod value;
pub mod wire;

/// The format-agnostic residual encoder contract and node walk.
pub use encode::{encode, LitTag, NodeTag, ResidualEncoder};

/// The residual wire form: the crossing descriptor, its tables, and the tier-0
/// serialization.
pub use wire::{
    serialize, BinderSite, Block, BlockId, BlockTable, DiagnosticsSchema, Function, FunctionTable,
    NodeRange, ProvenanceEntry, Residual, Signature, SuccRange, TerminatorKind, Tier, ViolationSeed,
};

/// The value-arena a produced value crosses back in.
pub use value::{BlobSpan, Region, RegionId, ValueArena, ValueList, ValueNode, ValueRef, ValueTag};

/// The reserve-then-commit transfer sink.
pub use sink::{CommitFn, ReserveFn, VehjeSink};

/// The batched-column `extern "C"` entry signature and the ABI version.
pub use entry::{BatchColumn, BatchedColumnEntry, ABI_VERSION};
