//! `ValidatorRegistry` — const-sized list of all shipped
//! validators plus a `run_all` collector.
//!
//! The registry is a ZST with an associated `VALIDATORS` slice
//! and an associated `run_all` function. The slice holds
//! `&'static dyn Validator` entries — this is the single
//! sanctioned `dyn` exception in clause-typecheck, mirroring the
//! `TargetRegistry` pattern in clause-codegen (see R3 DESIGN).
//! It's confined to the registry iteration surface and uses
//! `'static` lifetimes only.
//!
//! Dynamic registration (a `register` method for plugin
//! validators) is BACKLOG — the skeleton round ships only the
//! ten core validators.

use clause_ir::Diagnostic;

use crate::bind_target_shape::BindTargetShape;
use crate::coherence::Coherence;
use crate::ctx::ValidatorCtx;
use crate::orphan_rule::OrphanRule;
use crate::strict1::Strict1;
use crate::strict2::Strict2;
use crate::strict3::Strict3;
use crate::strict4::Strict4;
use crate::strict5::Strict5;
use crate::strict6::Strict6;
use crate::strict7::Strict7;
use crate::validator::Validator;

/// Validator registry — const iteration surface over every core
/// validator shipped in clause-typecheck.
///
/// Carries no state; all functionality is associated. Plugin
/// validators will gain a dynamic registration path in a future
/// round (BACKLOG).
#[derive(Debug)]
pub struct ValidatorRegistry;

impl ValidatorRegistry {
    /// The ten core validators, in the documented order:
    /// `Strict1..=Strict7`, then `Coherence`, `OrphanRule`,
    /// `BindTargetShape`.
    ///
    /// Consumers should not depend on ordering, but the test
    /// suite asserts it for this round to catch accidental
    /// reorders.
    pub const VALIDATORS: &'static [&'static dyn Validator] = &[
        &Strict1,
        &Strict2,
        &Strict3,
        &Strict4,
        &Strict5,
        &Strict6,
        &Strict7,
        &Coherence,
        &OrphanRule,
        &BindTargetShape,
    ];

    /// Run every validator in `VALIDATORS` against `ctx` and
    /// return the flattened diagnostic vec.
    ///
    /// Execution is sequential this round. Parallel (rayon)
    /// fan-out is BACKLOG.
    // lint:allow(bare_collection) — the diagnostic return surface across every compiler phase crate matches what clause-resolve already ships; storage-crate collection types target mockspace domain graphs not host-side compiler diagnostics here
    pub fn run_all(ctx: &ValidatorCtx) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for validator in Self::VALIDATORS {
            out.extend(validator.validate(ctx));
        }
        out
    }
}
