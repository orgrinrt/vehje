//! An end-to-end consumer built against `vehje` alone.
//!
//! This is the experiment behind the readiness claim: a minimal consumer
//! defines a family and a target, builds a program that puts a Core
//! sub-expression inside a family node, and runs the whole compile pipeline
//! (resolve, graded check, mint the `Checked`, emit) to a residual. A passing
//! run is proof the framework hosts a consumer, and the test doubles as the
//! reference a real consumer (the mockspace DSL) follows.

use arvo::{Identity, USize};
use hilavitkutin_api::capability::{BulkPush, Push};
use notko::{Maybe, Outcome};
use vehje::{
    check, check_for, fold_core, resolve_into, Arena, Builder, ByteEmitter, Checked, CodegenError,
    Cons, Core, Empty, Family, FamilyId, Grade, GradeTable, Literal, Node, NodeRef, Resolution,
    Span, Target, TargetSets,
};

/// The consumer's family: a single marker, plugged in through `Raw`.
#[derive(Debug)]
struct Demo;

impl Family for Demo {}

/// The consumer's target: it handles `Core` and `Demo`, permits no effects, and
/// emits a pre-order tag per node through the shared fold.
#[derive(Debug)]
struct DemoTarget;

impl TargetSets for DemoTarget {
    type Supports = Cons<Core, Cons<Demo, Empty>>;
    type Permits = Empty;
}

impl Target for DemoTarget {
    fn emit<S: ByteEmitter>(
        &self,
        checked: &Checked<'_, Self>,
        sink: &mut S,
    ) -> Outcome<(), CodegenError> {
        fold_core(checked.arena(), checked.root(), &mut |node| {
            let tag: &[u8] = match node { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream tag fed to the ByteEmitter contract; bytes are the 8-bit I/O unit; tracked: #207
                Node::Lit(_) => b"lit ",
                Node::Raw { .. } => b"raw ",
                _ => b"other ",
            };
            sink.push_bulk(tag);
        });
        Outcome::Ok(())
    }
}

/// A fixed-buffer byte sink, standing in for the host-provided emit target.
struct BufSink {
    buf: [u8; 64], // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream buffer; bytes are the 8-bit I/O unit of the ByteEmitter contract; tracked: #207
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

fn at(m: Maybe<NodeRef>) -> NodeRef {
    match m {
        Maybe::Is(r) => r,
        Maybe::Isnt => panic!("arena full"),
    }
}

#[test]
fn a_consumer_family_and_target_attach_end_to_end() {
    let mut nodes = [Node::Lit(Literal::Unit); 8];
    let mut spans = [Span::default(); 8];
    let mut pool = [NodeRef::new(USize::ZERO); 8];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));

    // Raw(Demo, [()]): a Core unit literal inside the consumer's family node.
    let unit = at(b.lit(Literal::Unit, Span::default()));
    let payload = match b.alloc_list(&[unit]) {
        Maybe::Is(l) => l,
        Maybe::Isnt => panic!("pool full"),
    };
    let root = at(b.raw(FamilyId::default(), payload, Span::default()));
    let arena = b.into_arena();

    // resolve names, run the graded check to obtain the evidence, mint the
    // Checked for DemoTarget (the program's family set is included in the
    // target's declared support), and emit.
    let mut binders = [Maybe::Isnt; 8];
    let mut res = Resolution::new(&mut binders);
    assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

    let mut grade_region = [Grade::default(); 8];
    let mut grades = GradeTable::new(&mut grade_region);
    let graded = match check(&arena, root, &res, &mut grades) {
        Outcome::Ok(g) => g,
        Outcome::Err(_) => panic!("check failed"),
    };
    let checked = check_for::<DemoTarget, Cons<Core, Cons<Demo, Empty>>, Empty>(graded);

    let mut sink = BufSink { buf: [0; 64], len: USize::ZERO }; // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: byte-stream buffer init; byte contract; tracked: #207
    let target = DemoTarget;
    assert!(matches!(target.emit(&checked, &mut sink), Outcome::Ok(())));

    // the emit descended into the family node's payload: the Raw node, then its
    // Core child. The whole grammar-free consumer path ran end to end.
    assert_eq!(&sink.buf[..sink.len.0], b"raw lit ");
}
