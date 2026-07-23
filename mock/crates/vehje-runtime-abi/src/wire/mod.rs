//! The residual wire form: the descriptor types and their serialization.
//!
//! The residual is a control-flow-graph-of-blocks program. [`residual`] names
//! the crossing descriptor and its tables ([`Residual`], [`BlockTable`],
//! [`FunctionTable`], the diagnostics schema); [`serialize`] walks a checked
//! arena and writes the tier-0 byte image.

pub mod residual;
pub mod serialize;

pub use residual::{
    Block, BlockId, BlockTable, DiagnosticsSchema, Function, FunctionTable, NodeRange, Residual,
    Signature, SuccRange, TerminatorKind, Tier,
};
pub use residual::{BinderSite, ProvenanceEntry, ViolationSeed};

pub use serialize::serialize;
