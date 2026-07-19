//! The effect axis: a parametrized, type-level effect vocabulary.
//!
//! Every construct carries an effect set alongside its family. The
//! vocabulary is parametrized, not enumerated: `Pure`, `Reads<E>`,
//! `Writes<E>`, where `E` ranges over an open environment axis
//! (`BuildEnv`, `RuntimeEnv`, and later kinds). `Pure` is the empty set.
//!
//! An effect set is a second instantiation of the `hilavitkutin-api`
//! `AccessSet` machinery, a perfect mirror of the family axis: each
//! `Reads<E>` / `Writes<E>` at a concrete `E` is a distinct `'static`
//! marker, so it drops into `Cons<H, T>` as a member. `Writes<BuildEnv>`
//! exists for free; no fifth member is reserved.

use core::marker::PhantomData;

/// The build environment: the registry, `self`-context, build resources.
/// Any effect over this environment is reduced away by the compiler
/// before emission; none survives into a residual.
pub struct BuildEnv;

/// The runtime environment: live inputs and outputs the Zig runtime
/// touches. Effects over this environment are the ones a residual may
/// carry.
pub struct RuntimeEnv;

/// A read of environment `E`.
pub struct Reads<E>(PhantomData<E>);

/// A write of environment `E`.
pub struct Writes<E>(PhantomData<E>);

/// The empty effect set (a construct that touches no environment).
///
/// A marker for ergonomics; semantically it is the absence of any
/// `Reads` / `Writes` member from an effect set.
pub struct Pure;
