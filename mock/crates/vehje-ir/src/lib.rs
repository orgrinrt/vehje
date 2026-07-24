//! vehje-ir, the framework's shared representation.
//!
//! The Core substrate forms, the `Family` extension trait plus effect
//! classification, the family-set and effect-set typestate (reusing
//! `hilavitkutin-api`'s `AccessSet`), the node builder, and interner
//! integration. No algorithms; the data contract the passes and targets
//! read and write. The IR lives in caller-provided memory.
//!
//! `#![no_std]`, no alloc.

#![no_std]
// const_trait_impl: WATCH-allowed (unstable-features.md); required by
// hilavitkutin-str's `str_const!` for the interned-name construction in tests.
#![feature(const_trait_impl)]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod arena;
pub mod binder;
pub mod builder;
pub mod diagnostic;
pub mod effect;
pub mod family;
pub mod grade;
pub mod hash;
pub mod intern;
pub mod node;
pub mod span;
pub mod target;

pub use arena::Arena;
pub use binder::BinderDomain;
pub use builder::Builder;
pub use diagnostic::{Diagnostic, Phase, Severity};
pub use effect::Effect;
pub use family::{Core, Family};
pub use grade::{
    Assurance, BindingTime, EffectMask, Grade, GradeTable, Knowledge, Lease, ReachMask,
};
pub use hash::{hash_of, StructuralHash};
pub use intern::{ArenaInterner, Str, StringInterner};
pub use node::{FamilyId, Literal, Node, NodeList, NodeRef};
pub use hilavitkutin_sym::Sym;
pub use span::{ByteOffset, FileId, Span};
pub use target::TargetSets;

/// The type-level set machinery, reused for the family and effect axes.
///
/// A family set and an effect set are both cons-lists built from `Empty`
/// and `Cons<H, T>`; `Contains` and `ContainsAll` are the membership and
/// inclusion witnesses; `Concat` appends. Reused verbatim from
/// `hilavitkutin-api` with vehje's own markers as members.
pub use hilavitkutin_api::access::{AccessSet, Concat, Cons, Contains, ContainsAll, Empty};
