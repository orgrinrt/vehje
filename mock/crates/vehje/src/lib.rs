//! vehje, the framework's public API and composition.
//!
//! The two plug-in contracts a consumer builds against (the grammar
//! contract on the input side, the target contract on the output side),
//! the orchestration that runs a program from grammar through the Core
//! passes to a checked residual, and the distribution-composition entry
//! point that links a set of targets at compile time.
//!
//! `#![no_std]`, no alloc.

#![no_std]
// const_trait_impl: WATCH-allowed (unstable-features.md); required by
// hilavitkutin-str's `str_const!` in the reference test.
#![feature(const_trait_impl)]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use notko::Outcome;

/// The consumer-facing surface, re-exported so a grammar plus a family plus a
/// target is built against `vehje` alone. Grouped by role (see the DESIGN).
///
/// The byte sink a target emits through.
pub use hilavitkutin_api::sink::ByteEmitter;
/// The output surface: the target contract, the mint, the shared fold, and the
/// witness.
pub use vehje_codegen::{check_for, fold_core, Checked, CodegenError, Target};
/// IR building, the family machinery, the set constructors and inclusion
/// witnesses, and the grade vocabulary.
pub use vehje_ir::{
    AccessSet, Arena, Builder, Cons, Contains, ContainsAll, Core, Diagnostic, EffectMask, Empty,
    Family, FamilyId, Grade, GradeTable, Literal, Node, NodeList, NodeRef, Phase, Span, TargetSets,
};
/// The resolve pass, its side-table, and its family hook.
pub use vehje_resolve::{
    resolve, resolve_into, resolve_into_with, resolve_with, CoreFamilies, FamilyResolve, Resolution,
    ResolveError,
};
/// The graded check pass, its evidence token, and its family hook.
pub use vehje_typecheck::{check, check_with, CheckError, FamilyCheck, Graded};

/// The input-side plug-in: a consumer's front-end produces well-formed
/// Core-plus-family IR.
///
/// The framework mandates the IR contract and offers, but does not
/// mandate, front-end scaffolding. A grammar lowers its surface syntax
/// into the Core forms plus its family node kinds, building into a
/// caller-provided arena and returning the program root.
pub trait Grammar {
    /// The grammar's error type.
    type Error;

    /// Lower a source into IR, building into `arena`, returning the
    /// program root.
    // FIXME: the source input type is a byte source the grammar reads;
    // M0 fixes the IR-producing shape, the source-side type lands with
    // the first consumer grammar.
    fn lower(&self, arena: &mut Arena<'_>) -> Outcome<NodeRef, Self::Error>;
}

/// Run a program from its IR through the per-program compile path.
///
/// The dev-time compiler's per-program path: resolve the Core binders, then
/// (the next wiring gate) the graded check, the cheap lowering, the inclusion
/// check to a `Checked`, the emit through the target, and the hand-off to the
/// runtime driver. M-level validates resolution and reports an unresolved name
/// as a diagnostic.
// FIXME: thread caller-provided grade and resolution regions to run
// vehje-check (the graded judgment) and vehje-lower (the cheap subset), then
// build the Checked and emit through the target into a sink and hand the
// residual to vehje-runtime-driver. The resolve stage ships; the rest is the
// next behavior gate.
pub fn run<T: Target>(_target: &T, arena: &Arena<'_>, root: NodeRef) -> Outcome<(), Diagnostic> {
    match resolve(arena, root) {
        Outcome::Ok(()) => Outcome::Ok(()),
        Outcome::Err(ResolveError::Unresolved { span, .. }) => {
            Outcome::Err(Diagnostic::error(Phase::Resolve, span, "unresolved name"))
        }
    }
}

// FIXME: the per-language path (`compile_language`) drives a `vehje-signature`
// `Signature` through the check and lower stages to collect their
// validated-data slices, then hands them to `vehje-runtime-gen::generate` to
// produce the language package and manifest. M-level exposes
// `vehje-runtime-gen::generate` directly (a consumer composes the slices); the
// orchestration that produces the slices from the signature is the next gate,
// and adding it here as a thin forward would only duplicate `generate`, so it
// lands when it drives the stages.

/// A reference target that emits a pre-order tag dump of the Core forms.
///
/// The framework's own reference distribution target: it supports the
/// `Core` family, permits no effects, and folds the checked residual to a
/// simple textual rendering. It exists to exercise the whole pipeline
/// (IR to resolve to inclusion check to emit) end to end, not as a
/// production backend.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DebugTarget;

impl TargetSets for DebugTarget {
    type Supports = Cons<Core, Empty>;
    type Permits = Empty;
}

impl Target for DebugTarget {
    fn emit<S: ByteEmitter>(
        &self,
        checked: &Checked<'_, Self>,
        sink: &mut S,
    ) -> Outcome<(), CodegenError> {
        // The traversal is codegen's shared `fold_core`; the target
        // supplies only the per-form tag.
        fold_core(checked.arena(), checked.root(), &mut |node| {
            let tag: &[u8] = match node { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream tag fed to the ByteEmitter contract; bytes are the 8-bit I/O unit; tracked: #207
                Node::Lit(_) => b"lit ",
                Node::Var(_) => b"var ",
                Node::Let { .. } => b"let ",
                Node::Lambda { .. } => b"lambda ",
                Node::Apply { .. } => b"apply ",
                Node::Project { .. } => b"project ",
                Node::If { .. } => b"if ",
                Node::Match { .. } => b"match ",
                Node::Iter { .. } => b"iter ",
                Node::Interp { .. } => b"interp ",
                Node::Raw { .. } => b"raw ",
                Node::Handle { .. } => b"handle ",
            };
            sink.push_bulk(tag);
        });
        Outcome::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use arvo::{Bool, Identity, USize};
    use hilavitkutin_api::capability::{BulkPush, Push};
    use hilavitkutin_str::str_const;
    use vehje_codegen::check_for;
    use vehje_ir::{Builder, Grade, GradeTable, Literal, Span};
    use vehje_resolve::{resolve_into, Resolution};
    use vehje_typecheck::check;

    /// A byte sink that collects into a fixed buffer, for the test.
    struct BufSink {
        buf: [u8; 128], // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream buffer; bytes are the 8-bit I/O unit of the ByteEmitter contract; tracked: #207
        len: USize,
    }

    impl Push<u8> for BufSink { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: foreign Push<u8> signature; byte contract; tracked: #207
        fn push(&mut self, item: u8) { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: foreign Push<u8> signature; byte contract; tracked: #207
            self.buf[self.len.0] = item;
            self.len = USize(self.len.0 + 1);
        }
    }

    impl BulkPush<u8> for BufSink { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: foreign BulkPush<u8> signature; byte contract; tracked: #207
        fn push_bulk(&mut self, items: &[u8]) { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: foreign BulkPush<u8> signature; byte contract; tracked: #207
            for b in items {
                self.push(*b);
            }
        }
    }

    fn expect(m: notko::Maybe<NodeRef>) -> NodeRef {
        match m {
            notko::Maybe::Is(r) => r,
            notko::Maybe::Isnt => panic!("arena full"),
        }
    }

    #[test]
    fn first_light_emits_the_program() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // let x = () in x
        let x = str_const!("x").as_sym();
        let unit = expect(b.lit(Literal::Unit, Span::default()));
        let var = expect(b.var(x, Span::default()));
        let root = expect(b.let_(Bool::FALSE, x, unit, var, Span::default()));
        let arena = b.into_arena();

        // resolve names, then run the graded check to obtain the evidence the
        // mint requires, then mint the witness. the program uses only the Core
        // family and no effects, so it is included in DebugTarget's (Core)
        // support and () permit sets.
        let mut binders = [notko::Maybe::Isnt; 8];
        let mut res = Resolution::new(&mut binders);
        assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));
        let mut grade_region = [Grade::default(); 8];
        let mut grades = GradeTable::new(&mut grade_region);
        let graded = match check(&arena, root, &res, &mut grades) {
            Outcome::Ok(g) => g,
            Outcome::Err(_) => panic!("check failed"),
        };
        let checked = check_for::<DebugTarget, Cons<Core, Empty>, Empty>(graded);

        // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream buffer init; byte contract; tracked: #207
        let mut sink = BufSink { buf: [0; 128], len: USize::ZERO };
        let target = DebugTarget;
        assert!(matches!(target.emit(&checked, &mut sink), Outcome::Ok(())));

        // pre-order: let, then the value (lit), then the body (var).
        assert_eq!(&sink.buf[..sink.len.0], b"let lit var ");
    }
}
