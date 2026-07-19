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

use hilavitkutin_api::sink::ByteEmitter;
use notko::Outcome;
use vehje_ir::{Arena, Cons, Core, Diagnostic, Empty, Node, NodeRef};

/// The `Target` output contract, re-exported from `vehje-codegen`.
pub use vehje_codegen::{Checked, CodegenError, Target};

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

/// Run a program from its IR through the Core passes to a checked
/// residual for a target.
///
/// M0 defines the orchestration entry; wiring resolve, check, the family
/// and effect inclusion checks, and the residual hand-off to the runtime
/// driver is the next behavior gate.
// FIXME: run vehje-resolve then vehje-typecheck (check) over the arena,
// run the inclusion checks to produce a Checked, and hand it to the
// runtime driver. M0 ships the surface.
pub fn run<T: Target>(_target: &T, _arena: &Arena<'_>, _root: NodeRef) -> Outcome<(), Diagnostic> {
    Outcome::Ok(())
}

/// A reference target that emits a pre-order tag dump of the Core forms.
///
/// The framework's own reference distribution target: it supports the
/// `Core` family, permits no effects, and folds the checked residual to a
/// simple textual rendering. It exists to exercise the whole pipeline
/// (IR to resolve to inclusion check to emit) end to end, not as a
/// production backend.
pub struct DebugTarget;

impl Target for DebugTarget {
    type Supports = Cons<Core, Empty>;
    type Permits = Empty;
    const NAME: &'static str = "debug";

    fn emit<S: ByteEmitter>(
        &self,
        checked: &Checked<'_, Self>,
        sink: &mut S,
    ) -> Outcome<(), CodegenError> {
        fold(checked.arena(), checked.root(), sink);
        Outcome::Ok(())
    }
}

/// Pre-order fold: push each form's tag, then recurse into its children.
fn fold<S: ByteEmitter>(arena: &Arena<'_>, at: NodeRef, sink: &mut S) {
    match arena.get(at) {
        Node::Lit(_) => sink.push_bulk(b"lit "),
        Node::Var(_) => sink.push_bulk(b"var "),
        Node::Let { value, body, .. } => {
            sink.push_bulk(b"let ");
            fold(arena, value, sink);
            fold(arena, body, sink);
        }
        Node::Lambda { body, .. } => {
            sink.push_bulk(b"lambda ");
            fold(arena, body, sink);
        }
        Node::Apply { callee, args } => {
            sink.push_bulk(b"apply ");
            fold(arena, callee, sink);
            for child in arena.list(args) {
                fold(arena, *child, sink);
            }
        }
        Node::Project { base, .. } => {
            sink.push_bulk(b"project ");
            fold(arena, base, sink);
        }
        Node::If { cond, then_branch, else_branch } => {
            sink.push_bulk(b"if ");
            fold(arena, cond, sink);
            fold(arena, then_branch, sink);
            fold(arena, else_branch, sink);
        }
        Node::Match { scrutinee, arms } => {
            sink.push_bulk(b"match ");
            fold(arena, scrutinee, sink);
            for child in arena.list(arms) {
                fold(arena, *child, sink);
            }
        }
        Node::Iter { seq, body } => {
            sink.push_bulk(b"iter ");
            fold(arena, seq, sink);
            fold(arena, body, sink);
        }
        Node::Interp { value } => {
            sink.push_bulk(b"interp ");
            fold(arena, value, sink);
        }
        Node::Raw { .. } => sink.push_bulk(b"raw "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use arvo::{Bool, Identity, USize};
    use hilavitkutin_api::capability::{BulkPush, Push};
    use hilavitkutin_str::str_const;
    use vehje_codegen::check_for;
    use vehje_ir::{Builder, Literal};

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
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut pool));

        // let x = () in x
        let x = str_const!("x");
        let unit = expect(b.lit(Literal::Unit));
        let var = expect(b.var(x));
        let root = expect(b.let_(Bool::FALSE, x, unit, var));
        let arena = b.into_arena();

        // the program uses only the Core family and no effects, so it is
        // included in DebugTarget's (Core) support and () permit sets.
        let checked = check_for::<DebugTarget, Cons<Core, Empty>, Empty>(&arena, root);

        // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream buffer init; byte contract; tracked: #207
        let mut sink = BufSink { buf: [0; 128], len: USize::ZERO };
        let target = DebugTarget;
        assert!(matches!(target.emit(&checked, &mut sink), Outcome::Ok(())));

        // pre-order: let, then the value (lit), then the body (var).
        assert_eq!(&sink.buf[..sink.len.0], b"let lit var ");
    }
}
