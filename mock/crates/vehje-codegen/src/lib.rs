//! vehje-codegen, the framework's output machinery.
//!
//! A target declares what it supports, and everything else is a proof
//! obligation against that declaration. A target names a type-level set
//! of families it handles (`Supports`) and a type-level set of effects it
//! permits (`Permits`); emission is bounded on the program's families
//! being included in `Supports` and its effects in `Permits`. The
//! guarantee is inclusion, not coverage.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use core::marker::PhantomData;

use hilavitkutin_api::sink::ByteEmitter;
use notko::Outcome;
use vehje_ir::{AccessSet, Arena, ContainsAll, NodeRef, Span};

/// The output plug-in contract.
///
/// A target declares the family set it handles and the effect set it
/// permits, both type-level sets over the `AccessSet` machinery, and
/// emits a checked residual. A target is total over the families it
/// declares.
pub trait Target {
    /// The families this target handles.
    type Supports: AccessSet;
    /// The effects this target permits.
    type Permits: AccessSet;
    /// The target name, resolved through the compile-time composition.
    const NAME: &'static str;

    /// Emit a residual this target has been proven to accept, writing
    /// through a caller-provided byte sink.
    ///
    /// A target folds over the Core substrate plus the families it
    /// declares, total over that declared set. A shared generic fold
    /// helper over Core is a later refinement; the contract is that a
    /// target consumes a proven `Checked` and emits.
    fn emit<S: ByteEmitter>(
        &self,
        checked: &Checked<'_, Self>,
        sink: &mut S,
    ) -> Outcome<(), CodegenError>
    where
        Self: Sized;
}

/// A witness that a program has been inclusion- and effect-checked
/// against a specific target.
///
/// `emit` accepts only a `Checked`; a `Checked` is producible only by
/// running the check for that target (see [`check`]). The obligation to
/// check is enforced at compile time; for a statically known program the
/// inclusion discharges at compile time.
pub struct Checked<'a, T> {
    arena: &'a Arena<'a>,
    root: NodeRef,
    _target: PhantomData<T>,
}

impl<'a, T> Checked<'a, T> {
    /// The checked program's arena.
    pub fn arena(&self) -> &'a Arena<'a> {
        self.arena
    }

    /// The checked program's root node.
    pub fn root(&self) -> NodeRef {
        self.root
    }
}

/// Check a statically known program against target `T` at compile time.
///
/// The `where` bounds are the inclusion proof: the target's `Supports`
/// contains every family in the program's `Families` set, and its
/// `Permits` contains every effect in the program's `Effects` set. A
/// mismatch is a compile error naming the missing family or effect
/// (through the `Contains` diagnostic). This is the static half of the
/// two-stage proof; the runtime half (a bitmask over a parsed program's
/// family ids) produces the same `Checked` witness through a checked
/// path.
pub fn check_for<'a, T, Families, Effects>(
    arena: &'a Arena<'a>,
    root: NodeRef,
) -> Checked<'a, T>
where
    T: Target,
    T::Supports: ContainsAll<Families>,
    T::Permits: ContainsAll<Effects>,
{
    Checked { arena, root, _target: PhantomData }
}

/// A codegen diagnostic.
///
/// Carries the refused construct and the target, not a bare string.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CodegenError {
    /// A construct outside the target's declared family set.
    UnsupportedFamily { span: Span, target: &'static str },
    /// An effect outside the target's permitted set.
    ForbiddenEffect { span: Span, target: &'static str },
}
