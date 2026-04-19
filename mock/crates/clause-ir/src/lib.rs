//! clause-ir — data contracts shared across compiler phases.
//!
//! This crate hosts the types that two or more phase crates consume:
//! `Span`, `FileId`, `ByteOffset`, `NodeId`, `ScopeId`, `Diagnostic`,
//! `TokenKind`, `AstNodeKind`. No algorithmic content lives here; the
//! crate is intentionally a leaf dependency on the Clause dependency
//! DAG so every phase can pull in its type vocabulary without pulling
//! in any other phase's implementation.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod diagnostic;
pub mod ids;
pub mod nodes;
pub mod span;
pub mod tokens;

pub use diagnostic::{Diagnostic, Severity};
pub use ids::{NodeId, ScopeId};
pub use nodes::AstNodeKind;
pub use span::{ByteOffset, FileId, Span};
pub use tokens::TokenKind;
