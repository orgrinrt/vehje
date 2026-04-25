//! vehje-resolve, skeleton name resolver for the Vehje
//! authoring language.
//!
//! Consumes the `Ast` produced by vehje-syntax; produces a
//! `Resolved` bundle carrying the AST plus a `ScopeTree` and a
//! per-`NodeId` resolution map. Also hosts the `Vehje.toml`
//! manifest parser (stubbed this round; real TOML handling is
//! BACKLOG).
//!
//! Round-one scope is deliberately minimal: the harness (types
//! + driver + manifest stub + error carriers) plus a top-level
//! `resolve` entry point that walks nothing and returns a
//! `Maybe::Isnt`-filled resolution map sized to `ast.len()`.
//! Every resolution rule (top-level items, local bindings,
//! paths, use statements, globs, generics, self-type, trait
//! methods, macro hygiene) lands as its own follow-up
//! micro-round on top of this stable harness.
//!
//! vehje-resolve is `no_std` + `no_alloc` first, like every crate
//! in the stack. Host-side compiler phases do not inherit a
//! dispensation from the runtime's discipline. Current skeleton
//! uses `std::collections::HashMap` + `Vec` inside the scope
//! implementation as tracked escapes (see `SHAME.md` `## Scope`);
//! these flip to scheduler-managed `Column<Symbol>` +
//! `Map<Str, SymbolSlot>` when #131 (M0.2: vehje-schedule as
//! hilavitkutin WorkUnit home) lands. After that, `#![no_std]`
//! goes at the crate root and any remaining `std`-requiring
//! surface lands behind `#[cfg(feature = "std")]` as a feature-
//! gated opt-in for third-party ecosystem fit.

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
pub use resolver::{resolve, Resolver};
pub use scope::{Scope, ScopeTree};
pub use symbol::{Symbol, SymbolKind};

pub use vehje_ir::{Diagnostic, NodeId, ScopeId, Span};
pub use hilavitkutin_str::Str;
pub use vehje_syntax::{Ast, AstNode};
