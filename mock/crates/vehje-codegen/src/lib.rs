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

use hilavitkutin_api::sink::ByteEmitter;
use notko::Outcome;
use vehje_ir::{Arena, ContainsAll, Node, NodeRef, Span, TargetSets};

/// The checked-program witness, minted by `vehje-check` (the proof witness
/// belongs with the prover) and re-exported here because `emit` consumes it.
pub use vehje_typecheck::Checked;

/// The evidence-of-check token, minted by `vehje-check`'s `check` and consumed
/// by `check_for` to mint a `Checked`. Re-exported here so a caller can name it
/// between the two calls.
pub use vehje_typecheck::Graded;

/// The output plug-in contract.
///
/// A target declares the family set it handles and the effect set it
/// permits, both type-level sets over the `AccessSet` machinery, and
/// emits a checked residual. A target is total over the families it
/// declares.
pub trait Target: TargetSets + core::fmt::Debug {
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
pub fn check_for<'a, T, Families, Effects>(graded: Graded<'a>) -> Checked<'a, T>
where
    T: Target,
    T::Supports: ContainsAll<Families>,
    T::Permits: ContainsAll<Effects>,
{
    // route through the sanctioned mint in vehje-typecheck (the witness crate);
    // `Checked` cannot be constructed here directly. The `Graded` evidence proves
    // a clean check ran, and the `where` bounds (derived from the target through
    // `TargetSets`) are the inclusion witness the mint requires.
    vehje_typecheck::mint_checked::<T, Families, Effects>(graded, core::marker::PhantomData)
}

/// The one generic fold over the Core substrate: a pre-order walk that
/// invokes `visit` on each node before recursing into its children.
///
/// This is the shared traversal every target folds over; a target
/// supplies the per-node emit logic through `visit` (matching on the node
/// kind) and never re-implements the walk. Family nodes reach `visit`
/// through `Raw`; a family's own sub-structure is walked by the family's
/// fold extension.
// A `Raw` node's payload (its family operation's Core child handles) is folded
// like any variadic form's children, so `visit` sees them. A family payload
// encoding beyond the child-handle traversal (a family-interpreted opaque blob)
// waits on the first consumer that defines one.
pub fn fold_core<F: FnMut(&Node)>(arena: &Arena<'_>, at: NodeRef, visit: &mut F) {
    let node = arena.get(at);
    visit(&node);
    match node {
        Node::Lit(_) | Node::Var(_) => {}
        Node::Raw { payload, .. } => {
            // descend into the family node's payload so a target's `visit` sees
            // the family operation's Core children (a `Var`, a nested form); the
            // `visit` closure is the family fold hook.
            for child in arena.list(payload) {
                fold_core(arena, *child, visit);
            }
        }
        Node::Let { value, body, .. } => {
            fold_core(arena, value, visit);
            fold_core(arena, body, visit);
        }
        Node::Lambda { body, .. } => fold_core(arena, body, visit),
        Node::Apply { callee, args } => {
            fold_core(arena, callee, visit);
            for child in arena.list(args) {
                fold_core(arena, *child, visit);
            }
        }
        Node::Project { base, .. } => fold_core(arena, base, visit),
        Node::If { cond, then_branch, else_branch } => {
            fold_core(arena, cond, visit);
            fold_core(arena, then_branch, visit);
            fold_core(arena, else_branch, visit);
        }
        Node::Match { scrutinee, arms } => {
            fold_core(arena, scrutinee, visit);
            for child in arena.list(arms) {
                fold_core(arena, *child, visit);
            }
        }
        Node::Iter { seq, body } => {
            fold_core(arena, seq, visit);
            fold_core(arena, body, visit);
        }
        Node::Interp { value } => fold_core(arena, value, visit),
        Node::Handle { body, clauses } => {
            fold_core(arena, body, visit);
            for child in arena.list(clauses) {
                fold_core(arena, *child, visit);
            }
        }
    }
}

/// A codegen diagnostic.
///
/// Carries the refused construct's span. The offending target is the
/// monomorphic `T` the error is returned from (`T: Target` is `Debug`),
/// so no bare target-name string is stored on the error.
// FIXME: name the missing family or effect on the variant once the
// two-stage runtime check builds these (the compile-time path discharges
// through the `ContainsAll` bounds, so M0 constructs neither variant yet).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CodegenError {
    /// A construct outside the target's declared family set.
    UnsupportedFamily { span: Span },
    /// An effect outside the target's permitted set.
    ForbiddenEffect { span: Span },
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo::{Identity, USize};
    use notko::Maybe;
    use vehje_ir::{Builder, FamilyId, Literal};

    #[test]
    fn fold_core_descends_into_raw_payload() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // Raw(family, [(), ()]): the two Lit children must be visited, so a
        // target's `visit` sees a family node's Core sub-structure.
        let a = match b.lit(Literal::Unit, Span::default()) {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        };
        let c = match b.lit(Literal::Unit, Span::default()) {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        };
        let payload = match b.alloc_list(&[a, c]) {
            Maybe::Is(l) => l,
            Maybe::Isnt => panic!("pool full"),
        };
        let raw = match b.raw(FamilyId::default(), payload, Span::default()) {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        };
        let arena = b.into_arena();

        let mut count = USize(0);
        fold_core(&arena, raw, &mut |_node| {
            count = USize(count.0 + 1);
        });
        // the Raw node plus its two payload children: three visits.
        assert_eq!(count, USize(3));
    }
}
