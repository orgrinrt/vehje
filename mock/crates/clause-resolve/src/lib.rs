//! clause-resolve — skeleton name resolver for the Clause
//! authoring language.
//!
//! Consumes the `Ast` produced by clause-syntax; produces a
//! `Resolved` bundle carrying the AST plus a `ScopeTree` and a
//! per-`NodeId` resolution map. Also hosts the `Clause.toml`
//! manifest parser (stubbed this round; real TOML handling is
//! BACKLOG).
//!
//! Round-one scope is deliberately minimal: the harness (types
//! + driver + manifest stub + error carriers) plus a top-level
//! `resolve` entry point that walks nothing and returns a
//! `None`-filled resolution map sized to `ast.len()`. Every
//! resolution rule (top-level items, local bindings, paths, use
//! statements, globs, generics, self-type, trait methods, macro
//! hygiene) lands as its own follow-up micro-round on top of
//! this stable harness.
//!
//! This crate uses `std`; it is host-side (compiler phase), not
//! runtime. The `no_std` / fixed-arena discipline on `clause-ir`,
//! `clause-lex`, `clause-syntax` does not propagate here.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod error;
pub mod manifest;
pub mod resolved;
pub mod resolver;
pub mod scope;
pub mod symbol;

pub use error::{ManifestError, ResolveError};
pub use manifest::{Manifest, parse_manifest};
pub use resolved::Resolved;
pub use resolver::Resolver;
pub use scope::{Scope, ScopeTree};
pub use symbol::{Symbol, SymbolKind};

pub use clause_ir::{Diagnostic, NodeId, ScopeId, Span};
pub use clause_syntax::{Ast, AstNode};

/// Resolve an AST into a `Resolved` bundle.
///
/// The skeleton does not walk the AST — it returns a `Resolved`
/// whose `resolution` vec is `None`-filled to `ast.len()`. Each
/// deferred resolution rule flips a subset of those slots as it
/// lands.
pub fn resolve(ast: &Ast) -> Result<Resolved, ResolveError> {
    Resolver::new().resolve(ast)
}
