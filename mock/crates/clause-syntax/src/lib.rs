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
//! `#![no_std]`, no `alloc`. Multi-error surfacing flows through a
//! caller-provided `DiagnosticSink<SyntaxError>`; the parser never
//! allocates. The AST arena is a fixed-size array; larger programs
//! arrive with a const-generic arena once the real parser
//! exercises the need.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::{Ast, AstNode, MAX_CHILDREN, MAX_NODES};
pub use error::{SyntaxError, SyntaxErrorKind};
pub use parser::{parse, Parser, TokenCursor};

pub use clause_ir::TokenKind;
pub use clause_ir::{AstNodeKind, NodeId, Span};
pub use clause_lex::Token;
