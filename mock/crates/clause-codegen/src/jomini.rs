//! `JominiTarget` — Clausewitz engine script target.
//!
//! Placeholder for the future `clause-jomini` sibling repo
//! extension that emits Clausewitz engine script for Paradox
//! games (Stellaris, CK3, EU4, HoI4, etc.). Named after Jomini,
//! the shared backend Paradox uses across the Clausewitz-family
//! engines.
//!
//! Skeleton round: `emit` returns an empty `SourceFile` artifact
//! (Clausewitz script is text, not bytecode). The real backend
//! body moves out to `github.com/orgrinrt/clause-jomini` once it
//! has a meaningful implementation.

use notko::Outcome;

use crate::artifact::{ArtifactKind, CodegenArtifact};
use crate::ctx::CodegenCtx;
use crate::error::CodegenError;
use crate::target::CodegenTarget;

/// Jomini-target ZST — Clausewitz engine script backend.
///
/// ZST; carries no state. Stub returns an empty `SourceFile`
/// artifact; the real lowering walk lands once the clause-jomini
/// sibling repo is bootstrapped (BACKLOG).
#[derive(Debug)]
pub struct JominiTarget;

impl CodegenTarget for JominiTarget {
    fn name(&self) -> &'static str {
        "jomini"
    }

    fn emit(&self, _ctx: &CodegenCtx) -> Outcome<CodegenArtifact, CodegenError> {
        Outcome::Ok(CodegenArtifact::empty(ArtifactKind::SourceFile))
    }
}
