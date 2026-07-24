//! vehje-lower, the framework's lowering stage.
//!
//! Lowering (constant folding, common-subexpression elimination, macro
//! expansion) is the third of the compile pipeline's operations, neither
//! checking nor output generation. It ships the cheap runtime subset now (a
//! bounded single bottom-up pass, hash-consing over `vehje-ir`'s one structural
//! hash, a fixed rule set, no unbounded saturation, no alloc), with the
//! dev-time equality-saturation e-graph as a deferred second stratum behind the
//! same fold interface. Macro expansion is a compile-stage handler dispatch.
//!
//! The cheap rewrites are redirecting: they never create nodes, they record a
//! `NodeRef`-to-`NodeRef` redirect in a caller-lent `Rewrite`. Folding an `If`
//! with a literal condition redirects the `If` to its taken branch; CSE
//! redirects a duplicate subtree to its first occurrence. A rewrite that must
//! create nodes (A-normal-form hoisting) is the deferred mutable-arena path.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::{Bool, Maybe};
use vehje_ir::{hash_of, Arena, Literal, Node, NodeRef, StructuralHash};

/// A caller-lent remap side-table: a node's index maps to its replacement.
///
/// The redirecting rewrites (const-fold, CSE) record a redirect here rather
/// than mutating the arena; `resolve` follows the chain to a node's current
/// representative, and the lowered root is the root resolved. No allocation:
/// the region is caller-provided, like `Resolution` and `GradeTable`. A redirect
/// always points to a node with a smaller arena index (a taken branch or an
/// earlier occurrence), so `resolve`'s chain strictly decreases and terminates.
pub struct Rewrite<'a> {
    remap: &'a mut [Maybe<NodeRef>],
}

impl<'a> Rewrite<'a> {
    /// Wrap a caller-provided region sized like the node arena; every entry
    /// starts unredirected (the caller fills it with `Maybe::Isnt`).
    pub fn new(remap: &'a mut [Maybe<NodeRef>]) -> Self {
        Self { remap }
    }

    /// Record that `from` is replaced by `to`.
    pub fn redirect(&mut self, from: NodeRef, to: NodeRef) {
        if from.index().0 < self.remap.len() {
            self.remap[from.index().0] = Maybe::Is(to);
        }
    }

    /// Follow the remap chain to `at`'s current representative.
    pub fn resolve(&self, at: NodeRef) -> NodeRef {
        let mut cur = at;
        loop {
            if cur.index().0 >= self.remap.len() {
                return cur;
            }
            match self.remap[cur.index().0] {
                // only follow a redirect to a strictly smaller index, so the
                // chain cannot loop.
                Maybe::Is(next) if next.index().0 < cur.index().0 => cur = next,
                _ => return cur,
            }
        }
    }
}

/// Which lowering stratum to run.
///
/// `Cheap` ships now; `Saturate` is the deferred dev-time equality-saturation
/// e-graph, reserved so its arrival is additive (nothing downstream changes
/// when it lands).
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum LowerStrategy {
    /// The cheap bounded const-fold plus CSE subset (the load-time path).
    #[default]
    Cheap,
    /// The dev-time equality-saturation e-graph (deferred).
    // FIXME: the no-alloc bounded streaming e-graph is the flagged original
    // research; it lands behind this variant, driven through the same `Lower`
    // entry, with an iterative work-stack extractor (recursion is a hard wall).
    Saturate,
}

/// The fixed rule set the cheap subset applies, and the crate's validated-data
/// slice.
///
/// The same table `vehje-runtime-gen` packages for the runtime's load-time
/// lowering of arriving scripts, so the Rust crate is the definition and the
/// Zig stage is the specialised execution.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RuleTable {
    /// Whether constant folding is enabled in this rule set.
    pub const_fold: Bool,
    /// Whether common-subexpression elimination is enabled.
    pub cse: Bool,
}

impl Default for RuleTable {
    fn default() -> Self {
        Self { const_fold: Bool::FALSE, cse: Bool::FALSE }
    }
}

impl RuleTable {
    /// The default cheap-subset rule table: fold and share both on.
    pub fn cheap() -> Self {
        Self { const_fold: Bool::TRUE, cse: Bool::TRUE }
    }
}

/// Constant folding: fold an operation whose operands are all statically known.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct ConstFold;

impl ConstFold {
    /// Fold the subtree rooted at `root`, recording redirects in `rw`.
    ///
    /// The Core-level fold that needs no family operation is an `If` whose
    /// condition resolves to a literal boolean: the `If` is redirected to its
    /// taken branch.
    // FIXME: fold a family primitive over literal operands once the family
    // payload encoding lands; the Core `If`/condition fold is the fold available
    // without a family operation.
    pub fn apply(&self, arena: &Arena<'_>, root: NodeRef, rw: &mut Rewrite<'_>) {
        fold_consts(arena, root, rw);
    }
}

/// Bottom-up const-fold walk: fold children, then fold this node.
fn fold_consts(arena: &Arena<'_>, at: NodeRef, rw: &mut Rewrite<'_>) {
    match arena.get(at) {
        Node::Lit(_) | Node::Var(_) | Node::Raw { .. } => {}
        Node::Let { value, body, .. } => {
            fold_consts(arena, value, rw);
            fold_consts(arena, body, rw);
        }
        Node::Lambda { body, .. } => fold_consts(arena, body, rw),
        Node::Apply { callee, args } => {
            fold_consts(arena, callee, rw);
            for c in arena.list(args) {
                fold_consts(arena, *c, rw);
            }
        }
        Node::Project { base, .. } => fold_consts(arena, base, rw),
        Node::If { cond, then_branch, else_branch } => {
            fold_consts(arena, cond, rw);
            fold_consts(arena, then_branch, rw);
            fold_consts(arena, else_branch, rw);
            // if the condition resolves to a literal boolean, this If folds to
            // the taken branch.
            if let Node::Lit(Literal::Bool(b)) = arena.get(rw.resolve(cond)) {
                let taken = if b.0 { then_branch } else { else_branch };
                rw.redirect(at, taken);
            }
        }
        Node::Match { scrutinee, arms } => {
            fold_consts(arena, scrutinee, rw);
            for c in arena.list(arms) {
                fold_consts(arena, *c, rw);
            }
        }
        Node::Iter { seq, body } => {
            fold_consts(arena, seq, rw);
            fold_consts(arena, body, rw);
        }
        Node::Interp { value } => fold_consts(arena, value, rw),
        Node::Handle { body, clauses } => {
            fold_consts(arena, body, rw);
            for c in arena.list(clauses) {
                fold_consts(arena, *c, rw);
            }
        }
    }
}

/// Common-subexpression elimination by hash-consing over the one structural
/// hash: a subtree that hashes equal to an earlier one is shared.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Cse;

impl Cse {
    /// The structural hash of the subtree rooted at `root`, the identity a
    /// hash-cons table keys on. Its benefit is node-count reduction, not per-op
    /// time.
    pub fn hash(&self, arena: &Arena<'_>, root: NodeRef) -> StructuralHash {
        hash_of(arena, root)
    }

    /// Share equal subtrees under `root`, recording each duplicate's redirect to
    /// its first occurrence in `rw`.
    ///
    /// `table` is a caller-lent hash-cons table (a linear-scan region of
    /// hash-to-first-node entries the caller fills with `Maybe::Isnt`). Each
    /// subtree that hashes equal to an earlier one is redirected to that earlier
    /// node. The structural hash folds leaf content, so a hash match is a
    /// structural match at the 64-bit hash's confidence (the standard hash-cons
    /// discipline; a structural re-verify on match is a belt-and-suspenders
    /// refinement).
    pub fn apply(
        &self,
        arena: &Arena<'_>,
        root: NodeRef,
        rw: &mut Rewrite<'_>,
        table: &mut [Maybe<(StructuralHash, NodeRef)>],
    ) {
        cse_share(arena, root, rw, table);
    }
}

/// Bottom-up CSE walk: share children, then key this node into the table.
fn cse_share(
    arena: &Arena<'_>,
    at: NodeRef,
    rw: &mut Rewrite<'_>,
    table: &mut [Maybe<(StructuralHash, NodeRef)>],
) {
    match arena.get(at) {
        Node::Lit(_) | Node::Var(_) | Node::Raw { .. } => {}
        Node::Let { value, body, .. } => {
            cse_share(arena, value, rw, table);
            cse_share(arena, body, rw, table);
        }
        Node::Lambda { body, .. } => cse_share(arena, body, rw, table),
        Node::Apply { callee, args } => {
            cse_share(arena, callee, rw, table);
            for c in arena.list(args) {
                cse_share(arena, *c, rw, table);
            }
        }
        Node::Project { base, .. } => cse_share(arena, base, rw, table),
        Node::If { cond, then_branch, else_branch } => {
            cse_share(arena, cond, rw, table);
            cse_share(arena, then_branch, rw, table);
            cse_share(arena, else_branch, rw, table);
        }
        Node::Match { scrutinee, arms } => {
            cse_share(arena, scrutinee, rw, table);
            for c in arena.list(arms) {
                cse_share(arena, *c, rw, table);
            }
        }
        Node::Iter { seq, body } => {
            cse_share(arena, seq, rw, table);
            cse_share(arena, body, rw, table);
        }
        Node::Interp { value } => cse_share(arena, value, rw, table),
        Node::Handle { body, clauses } => {
            cse_share(arena, body, rw, table);
            for c in arena.list(clauses) {
                cse_share(arena, *c, rw, table);
            }
        }
    }
    // key this node: an earlier node with the same hash is its canonical form.
    let h = hash_of(arena, at);
    let mut i = 0;
    while i < table.len() {
        match table[i] {
            Maybe::Is((sh, first)) => {
                if sh == h && first.index().0 != at.index().0 {
                    rw.redirect(at, first);
                    return;
                }
            }
            Maybe::Isnt => {
                table[i] = Maybe::Is((h, at));
                return;
            }
        }
        i += 1;
    }
    // table full: no sharing for this node (still correct, just larger).
}

/// A-normal form: name every intermediate so effect order is explicit in the
/// residual and no separate sequencing form is needed.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Anf;

impl Anf {
    /// Normalise the subtree rooted at `root` to A-normal form.
    // FIXME: hoisting each non-trivial subexpression into a fresh `Let` binding
    // creates new nodes, which the redirecting `Rewrite` cannot do; `Anf` lands
    // with the mutable-arena rewrite path. It is a no-op today.
    pub fn apply(&self, _arena: &Arena<'_>, _root: NodeRef, _rw: &mut Rewrite<'_>) {}
}

/// Macro expansion as a compile-stage handler dispatch.
///
/// A macro is a rewrite handled at the compile-stage handler; its effect is
/// discharged here, so the residual carries no unexpanded macro.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct MacroExpand;

impl MacroExpand {
    /// Expand the compile-stage-effect operations (macros) in the subtree.
    // FIXME: dispatch each macro operation to its compile-stage handler and
    // splice the expansion; a no-op today (no consumer family defines a macro
    // operation yet, and splicing an expansion needs the mutable-arena path).
    pub fn apply(&self, _arena: &Arena<'_>, _root: NodeRef, _rw: &mut Rewrite<'_>) {}
}

/// The lowering stage entry.
///
/// Takes a `LowerStrategy` and the caller-lent rewrite regions, records the
/// cheap subset's redirects, and returns the lowered root. The strategy is
/// where the two strata plug in, so a caller chooses the cheap subset or (later)
/// saturation without the stage changing shape.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Lower {
    /// The rule set to apply under the cheap strategy.
    pub rules: RuleTable,
}

impl Lower {
    /// A lowering stage with the default cheap rule table.
    pub fn cheap() -> Self {
        Self { rules: RuleTable::cheap() }
    }

    /// Lower the program rooted at `root` under `strategy`, returning the
    /// lowered root. Under `Cheap`, expands macros, folds constants, then shares
    /// common subexpressions, recording redirects in `rw` (and CSE keys in
    /// `cse_table`); the lowered root is `root` resolved through `rw`.
    pub fn lower(
        &self,
        strategy: LowerStrategy,
        arena: &Arena<'_>,
        root: NodeRef,
        rw: &mut Rewrite<'_>,
        cse_table: &mut [Maybe<(StructuralHash, NodeRef)>],
    ) -> NodeRef {
        match strategy {
            LowerStrategy::Cheap => {
                MacroExpand.apply(arena, root, rw);
                if self.rules.const_fold.0 {
                    ConstFold.apply(arena, root, rw);
                }
                if self.rules.cse.0 {
                    Cse.apply(arena, root, rw, cse_table);
                }
                Anf.apply(arena, root, rw);
                rw.resolve(root)
            }
            // FIXME: the Saturate stratum (the deferred dev-time e-graph) runs
            // here behind the same interface; falls through to the cheap subset
            // until it lands.
            LowerStrategy::Saturate => {
                self.lower(LowerStrategy::Cheap, arena, root, rw, cse_table)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo::strategy::Hot;
    use arvo::{Bool, Identity, Int, USize};
    use vehje_ir::{Builder, Span};

    fn at(m: Maybe<NodeRef>) -> NodeRef {
        match m {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        }
    }

    fn int(b: &mut Builder<'_>, v: Int<64, Hot>) -> NodeRef {
        at(b.lit(Literal::Int(v), Span::default()))
    }

    #[test]
    fn const_fold_folds_if_true_to_the_taken_branch() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // if true then 1 else 2: distinct branches make the fold observable.
        let cond = at(b.lit(Literal::Bool(Bool::TRUE), Span::default()));
        let a = int(&mut b, Int::<64, Hot>::from_raw(1));
        let bb = int(&mut b, Int::<64, Hot>::from_raw(2));
        let iff = at(b.if_(cond, a, bb, Span::default()));
        let arena = b.into_arena();

        let mut remap = [Maybe::Isnt; 8];
        let mut rw = Rewrite::new(&mut remap);
        ConstFold.apply(&arena, iff, &mut rw);
        assert_eq!(rw.resolve(iff), a);
    }

    #[test]
    fn const_fold_folds_if_false_to_the_else_branch() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        let cond = at(b.lit(Literal::Bool(Bool::FALSE), Span::default()));
        let a = int(&mut b, Int::<64, Hot>::from_raw(1));
        let bb = int(&mut b, Int::<64, Hot>::from_raw(2));
        let iff = at(b.if_(cond, a, bb, Span::default()));
        let arena = b.into_arena();

        let mut remap = [Maybe::Isnt; 8];
        let mut rw = Rewrite::new(&mut remap);
        ConstFold.apply(&arena, iff, &mut rw);
        assert_eq!(rw.resolve(iff), bb);
    }

    #[test]
    fn cse_shares_two_equal_subtrees() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // two structurally equal Int(7)s under one parent: CSE redirects the
        // later to the earlier.
        let first = int(&mut b, Int::<64, Hot>::from_raw(7));
        let second = int(&mut b, Int::<64, Hot>::from_raw(7));
        let parent = at(b.if_(first, second, first, Span::default()));
        let arena = b.into_arena();

        let mut remap = [Maybe::Isnt; 8];
        let mut rw = Rewrite::new(&mut remap);
        let mut table = [Maybe::Isnt; 8];
        Cse.apply(&arena, parent, &mut rw, &mut table);
        assert_eq!(rw.resolve(second), first);
    }

    #[test]
    fn cheap_lowering_preserves_a_trivial_program() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
        let unit = at(b.lit(Literal::Unit, Span::default()));
        let arena = b.into_arena();

        let mut remap = [Maybe::Isnt; 8];
        let mut rw = Rewrite::new(&mut remap);
        let mut table = [Maybe::Isnt; 8];
        let out = Lower::cheap().lower(LowerStrategy::Cheap, &arena, unit, &mut rw, &mut table);
        assert_eq!(out, unit);
    }
}
