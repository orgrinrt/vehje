//! clause-typecheck — skeleton type checker + validator framework
//! for the Clause authoring language.
//!
//! Consumes the `Resolved` bundle produced by clause-resolve;
//! runs a fixed list of core validators (STRICT1-7 plus
//! `coherence` / `orphan-rule` / `bind-target-shape`) and returns
//! a flat `Vec<Diagnostic>`.
//!
//! Round-one scope is deliberately minimal: the harness
//! (`Validator` trait + `ValidatorRegistry` + `ValidatorCtx` +
//! ten ZST validator stubs) plus a top-level `typecheck` entry
//! that walks the registry over an empty `Resolved` and returns
//! an empty vec. Every validator body (the actual shadow check,
//! overlap detection, orphan-rule walk, …) lands as its own
//! follow-up micro-round on top of this stable harness.
//!
//! This crate uses `std`; it is host-side (compiler phase), not
//! runtime. The `no_std` / fixed-arena discipline on
//! `clause-ir`, `clause-lex`, `clause-syntax` does not propagate
//! here.
//!
//! R4 (2026-04-26) finalised a richer trait shape with
//! associated `NAME` / `CATEGORY` consts, `accepts(kind)` AST
//! routing, and a `DiagnosticSink` emission API. That retrofit
//! is BACKLOG; this skeleton ships a narrower instance-method
//! form (`name(&self)`, `validate(&self, ctx) -> Vec<Diagnostic>`)
//! that is directly object-safe and suits the const `&'static dyn`
//! registry iteration surface without `const_in_trait` gymnastics.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod bind_target_shape;
pub mod coherence;
pub mod ctx;
pub mod orphan_rule;
pub mod registry;
pub mod strict1;
pub mod strict2;
pub mod strict3;
pub mod strict4;
pub mod strict5;
pub mod strict6;
pub mod strict7;
pub mod validator;

pub use bind_target_shape::BindTargetShape;
pub use coherence::Coherence;
pub use ctx::ValidatorCtx;
pub use orphan_rule::OrphanRule;
pub use registry::ValidatorRegistry;
pub use strict1::Strict1;
pub use strict2::Strict2;
pub use strict3::Strict3;
pub use strict4::Strict4;
pub use strict5::Strict5;
pub use strict6::Strict6;
pub use strict7::Strict7;
pub use validator::Validator;

pub use clause_ir::{Diagnostic, NodeId, Span};
pub use clause_resolve::Resolved;

/// Type-check `resolved` by running every core validator against
/// it. Returns the flattened diagnostic vec.
///
/// Skeleton round: each validator returns an empty diagnostic
/// vec, so the result is always empty. Each deferred validator
/// body flips its stub into a real check in its own follow-up
/// round.
// lint:allow(bare_collection) — the diagnostic return surface across every compiler phase crate matches what clause-resolve already ships; storage-crate collection types target mockspace domain graphs not host-side compiler diagnostics here
pub fn typecheck(resolved: &Resolved) -> Vec<Diagnostic> {
    let ctx = ValidatorCtx::new(resolved);
    ValidatorRegistry::run_all(&ctx)
}
