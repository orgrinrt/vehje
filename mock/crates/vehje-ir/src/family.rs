//! The family axis: the extension unit.
//!
//! A family is a type. Each consumer contributes its own families; `Core`
//! is the reference family (its node kinds are the eleven Core forms). A
//! program carries a type-level set of the families it uses, built from
//! the `hilavitkutin-api` `AccessSet` machinery (`Empty` / `Cons<H, T>`)
//! with family markers as members. `Cons` requires only `H: 'static`, so
//! the markers drop in with no upstream change; inclusion against a
//! target's declared set is the `ContainsAll` check.

/// A family: an extension unit contributing node kinds and semantics.
///
/// The type IS the identity used in the family-set typestate. Its `Debug`
/// rendering carries the human-readable name for diagnostics, so no bare
/// name const is stored. A numeric id (for the runtime family bitmask and
/// the `Raw` payload) is assigned separately from a family's declaration.
// FIXME: the numeric-id assignment from a Family (for the runtime bitmask
// and Raw) lands with the two-stage runtime check; M0 carries the
// type-level identity only.
pub trait Family: 'static + core::fmt::Debug {}

/// The Core family: the shared evaluation substrate every grammar lowers
/// into. Its node kinds are the eleven Core forms in `node`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Core;

impl Family for Core {}
