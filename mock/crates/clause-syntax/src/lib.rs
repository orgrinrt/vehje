//! clause-syntax — skeleton parser for the Clause authoring
//! language.
//!
//! Consumes a token slice from clause-lex, produces an `Ast`: a
//! flat arena of `AstNode`s keyed by `NodeId`. Syntax-level
//! diagnostics are carried as `SyntaxError`, which converts to
//! `clause_ir::Diagnostic` for merging into the compiler driver's
//! diagnostic pipeline.
//!
//! Round-one scope is deliberately minimal: the skeleton harness
//! (types + driver + peek/bump helpers) plus a `parse` entry point
//! that handles the two trivial programs (empty input, single
//! literal). Every grammar production — primary path, binary,
//! unary, call, block, let, if, match, fn, type, struct, enum,
//! module, pattern — lands as its own follow-up micro-round on top
//! of the stable harness. Error recovery and incremental reparse
//! are also BACKLOG.
//!
//! `#![no_std]`; `alloc` is pulled in only so `parse()` can surface
//! multi-error results as a `Vec<SyntaxError>`. The AST arena is a
//! fixed-size array; larger programs arrive with a const-generic
//! arena once the real parser exercises the need.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

extern crate alloc;

pub mod ast;
pub mod error;
pub mod parser;

use alloc::vec;
use alloc::vec::Vec;

pub use ast::{Ast, AstNode, MAX_CHILDREN, MAX_NODES};
pub use error::{SyntaxError, SyntaxErrorKind};
pub use parser::{Parser, TokenCursor};

pub use clause_ir::{AstNodeKind, NodeId, Span};
pub use clause_lex::Token;
pub use clause_ir::TokenKind;

/// Parse a token slice into an `Ast`.
///
/// The skeleton handles:
///
/// - Empty slice (or a slice of just `Eof`) → empty `Ast`.
/// - A single `IntLit` followed by optional `Eof` → an `Ast`
///   containing one `AstNodeKind::Expr` node spanning the
///   literal.
/// - Anything else → `Err(vec![SyntaxErrorKind::UnexpectedToken])`.
///
/// The error arm is `Vec<SyntaxError>` rather than a single
/// `SyntaxError` so future multi-error recovery extends the vec
/// without another signature churn. Today the vec carries one
/// element in the error case.
///
/// Every deferred production flips from `UnexpectedToken` to a
/// real parse in its own follow-up round.
// lint:allow(bare_collection) — the diagnostic return surface across every compiler phase crate matches what clause-typecheck and clause-resolve already ship; storage-crate collection types target mockspace domain graphs not host-side compiler syntax-error batches here
pub fn parse(tokens: &[Token]) -> Result<Ast, Vec<SyntaxError>> {
    Parser::new(tokens).parse().map_err(|e| vec![e])
}
