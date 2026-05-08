#![no_std]

//! vehje-codegen, skeleton codegen framework + extension-point
//! contract for the Vehje authoring language.
//!
//! Consumes the `Resolved` bundle produced by vehje-resolve;
//! routes emission through a fixed list of built-in
//! `CodegenTarget` implementations (`native`, `jomini`) via the
//! const `TargetRegistry`, and returns a `CodegenArtifact`.
//!
//! Round-one scope is deliberately minimal: the harness
//! (`CodegenTarget` trait + `TargetRegistry` + `CodegenCtx` +
//! two ZST target stubs + `CodegenArtifact` / `ArtifactKind` /
//! `CodegenError` payload types) plus a top-level `emit` entry
//! that walks the registry over an empty `Resolved` and returns
//! an empty artifact. Every target backend (real Rust/LLVM for
//! `NativeTarget`, Clausewitz emission for `JominiTarget` via
//! the future `vehje-jomini` sibling repo, …) lands as its own
//! follow-up round on top of this stable harness.
//!
//! This crate uses `std`; it is host-side (compiler phase), not
//! runtime. The `no_std` / fixed-arena discipline on
//! `vehje-ir`, `vehje-lex`, `vehje-syntax` does not propagate
//! here.
//!
//! R3 (2026-04-26) finalised a richer trait shape with
//! associated `NAME` / `VERSION` consts, `accepts(kind)` AST
//! routing, a `stages()` pipeline entry, and a `compile(unit,
//! ctx) -> Result<CodegenOutput, Diagnostic>` emission API with
//! a `CodegenOutput { bytes, manifest, references }` payload.
//! That retrofit is BACKLOG; this skeleton ships a narrower
//! instance-method form (`name(&self)`, `emit(&self, ctx) ->
//! Result<CodegenArtifact, CodegenError>`) that is directly
//! object-safe and suits the const `&'static dyn` registry
//! iteration surface without `const_in_trait` gymnastics. This
//! mirrors exactly the R4 → vehje-typecheck skeleton pattern.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod artifact;
pub mod ctx;
pub mod error;
pub mod jomini;
pub mod native;
pub mod registry;
pub mod target;

pub use artifact::{ArtifactKind, CodegenArtifact};
pub use ctx::CodegenCtx;
pub use error::CodegenError;
pub use jomini::JominiTarget;
pub use native::NativeTarget;
pub use registry::{emit, TargetRegistry};
pub use target::CodegenTarget;

pub use vehje_ir::Diagnostic;
pub use vehje_resolve::Resolved;
