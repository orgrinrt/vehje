//! The effect axis: effects are operations, discharge is handling.
//!
//! After the handler unification, an effect is an operation a construct
//! invokes, and a handler services it. The type-level effect set is an
//! `AccessSet` instantiation over `Effect` operation markers, the mirror of
//! the family axis, used for a target's `Permits` declaration. The runtime
//! bitmask shadow of the inferred effect set is `EffectMask` (see `grade`).
//!
//! This supersedes the old `Pure` / `Reads<E>` / `Writes<E>` over
//! `BuildEnv` / `RuntimeEnv` lattice, which conflated effect (what
//! world-change) with binding time (when known). Binding time now lives on
//! its own graded modality (see `grade::BindingTime`); the effect axis
//! carries only the operation set.

/// An effect operation: a `'static` marker for one operation a construct may
/// invoke (a host-call, a compile-stage macro operation, and so on).
///
/// A family or a consumer declares its own effect operations as ZST markers
/// implementing this trait. An effect set is a type-level `AccessSet` over
/// these markers; a program's effect is the operations it invokes that are
/// not locally handled, and a target `Permits` a set of runtime operations
/// the residual's unhandled operations must be included in. The `Debug`
/// rendering carries the human-readable operation name for diagnostics.
pub trait Effect: 'static + core::fmt::Debug {}
