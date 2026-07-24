//! The target's declared type-level sets, as a shared carrier trait.
//!
//! A target declares two type-level sets: the families it handles
//! (`Supports`) and the effects it permits (`Permits`). Both are `AccessSet`
//! cons-lists over vehje's family and effect markers. The declaration lives
//! on this carrier trait, in `vehje-ir` below both the check and the codegen
//! crates, so the witness mint in the check crate can derive a target's sets
//! from the target itself rather than take them as free type parameters. That
//! is what makes the mint unforgeable along the target axis: a caller cannot
//! substitute a support set the target does not declare.

use crate::AccessSet;

/// A target's declared family and effect sets.
///
/// `vehje-codegen`'s `Target` extends this; the mint derives its inclusion
/// bounds from `T::Supports` and `T::Permits` through it.
pub trait TargetSets {
    /// The family set the target handles.
    type Supports: AccessSet;
    /// The effect set the target permits.
    type Permits: AccessSet;
}
