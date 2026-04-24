//! `ValidatorRegistry` — const-sized list of all shipped
//! validators plus a `run_all` collector.
//!
//! The registry is a ZST with an associated `VALIDATORS` slice
//! and an associated `run_all` function. The slice holds
//! `&'static dyn Validator` entries — this is the single
//! sanctioned `dyn` exception in vehje-typecheck, mirroring the
//! `TargetRegistry` pattern in vehje-codegen (see R3 DESIGN).
//! It's confined to the registry iteration surface and uses
//! `'static` lifetimes only.
//!
//! Dynamic registration (a `register` method for plugin
//! validators) is BACKLOG — the skeleton round ships only the
//! ten core validators.

use vehje_ir::Diagnostic;
use vehje_resolve::Resolved;
use hilavitkutin_api::DiagnosticSink;

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
/// validator shipped in vehje-typecheck.
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

    /// Run every validator in `VALIDATORS` against `ctx`, forwarding
    /// `sink` to each for diagnostic emission.
    ///
    /// Execution is sequential this round. Parallel (rayon)
    /// fan-out is BACKLOG.
    pub fn run_all(
        ctx: &ValidatorCtx,
        sink: &mut dyn DiagnosticSink<Diagnostic>,
    ) {
        for validator in Self::VALIDATORS {
            validator.validate(ctx, sink);
        }
    }
}

/// Type-check `resolved` by running every core validator against
/// it, pushing diagnostics into `sink`.
///
/// Skeleton round: each validator pushes nothing, so `sink` stays
/// empty. Each deferred validator body flips its stub into a real
/// check in its own follow-up round.
pub fn typecheck(
    resolved: &Resolved,
    sink: &mut dyn DiagnosticSink<Diagnostic>,
) {
    let ctx = ValidatorCtx::new(resolved);
    ValidatorRegistry::run_all(&ctx, sink);
}
