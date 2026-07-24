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
// const_trait_impl: WATCH-allowed (unstable-features.md); required by
// hilavitkutin-str's `str_const!` for the capture-safety test's variable names.
#![feature(const_trait_impl)]
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
    /// its canonical (lowest-index) occurrence in `rw`.
    ///
    /// `table` is a caller-lent hash-cons table (a linear-scan region of
    /// hash-to-node entries the caller fills with `Maybe::Isnt`). Only
    /// variable-free subtrees are shared: a subtree containing a `Var` could,
    /// in another occurrence with the same name, resolve to a different binder,
    /// so sharing it would capture. On a hash match the two subtrees are
    /// structurally re-verified (the hash is a fast filter, the compare is the
    /// soundness guarantee), and the higher-index node is redirected to the
    /// lower so the redirect chain strictly decreases.
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

/// Bottom-up CSE walk. Shares children, then keys this node if it is
/// variable-free. Returns whether the subtree rooted at `at` is variable-free
/// (contains no `Var` and no opaque `Raw`), the condition for it to be safely
/// shareable without capture.
fn cse_share(
    arena: &Arena<'_>,
    at: NodeRef,
    rw: &mut Rewrite<'_>,
    table: &mut [Maybe<(StructuralHash, NodeRef)>],
) -> Bool {
    // `var_free` stays an inferred boolean (no bare-primitive annotation); the
    // function returns the stack `Bool`.
    let var_free = match arena.get(at) {
        Node::Lit(_) => true,
        // a Var is a free reference here; a Raw's family payload is opaque, so
        // neither is treated as shareable (still recurse a Raw's payload for its
        // own children's sharing).
        Node::Var(_) => false,
        Node::Raw { payload, .. } => {
            for c in arena.list(payload) {
                cse_share(arena, *c, rw, table);
            }
            false
        }
        Node::Let { value, body, .. } => {
            let a = cse_share(arena, value, rw, table).0;
            let b = cse_share(arena, body, rw, table).0;
            a && b
        }
        Node::Lambda { body, .. } => cse_share(arena, body, rw, table).0,
        Node::Apply { callee, args } => {
            let mut vf = cse_share(arena, callee, rw, table).0;
            for c in arena.list(args) {
                vf = cse_share(arena, *c, rw, table).0 && vf;
            }
            vf
        }
        Node::Project { base, .. } => cse_share(arena, base, rw, table).0,
        Node::If { cond, then_branch, else_branch } => {
            let a = cse_share(arena, cond, rw, table).0;
            let b = cse_share(arena, then_branch, rw, table).0;
            let c = cse_share(arena, else_branch, rw, table).0;
            a && b && c
        }
        Node::Match { scrutinee, arms } => {
            let mut vf = cse_share(arena, scrutinee, rw, table).0;
            for c in arena.list(arms) {
                vf = cse_share(arena, *c, rw, table).0 && vf;
            }
            vf
        }
        Node::Iter { seq, body } => {
            let a = cse_share(arena, seq, rw, table).0;
            let b = cse_share(arena, body, rw, table).0;
            a && b
        }
        Node::Interp { value } => cse_share(arena, value, rw, table).0,
        Node::Handle { body, clauses } => {
            let mut vf = cse_share(arena, body, rw, table).0;
            for c in arena.list(clauses) {
                vf = cse_share(arena, *c, rw, table).0 && vf;
            }
            vf
        }
    };
    if !var_free {
        return Bool(false);
    }
    // key this variable-free node: a structurally-equal earlier node is its
    // canonical form; the lowest index stays canonical so the redirect chain
    // strictly decreases.
    let h = hash_of(arena, at);
    let mut i = 0;
    while i < table.len() {
        match table[i] {
            Maybe::Is((sh, first)) => {
                if sh == h && structurally_equal(arena, at, first).0 {
                    if at.index().0 < first.index().0 {
                        rw.redirect(first, at);
                        table[i] = Maybe::Is((sh, at));
                    } else if at.index().0 > first.index().0 {
                        rw.redirect(at, first);
                    }
                    return Bool(true);
                }
            }
            Maybe::Isnt => {
                table[i] = Maybe::Is((h, at));
                return Bool(true);
            }
        }
        i += 1;
    }
    // table full: no sharing for this node (still correct, just larger).
    Bool(true)
}

/// Whether the subtrees rooted at `a` and `b` are structurally identical: the
/// soundness confirm on a hash match, so a 64-bit hash collision cannot merge
/// two distinct subtrees. Called only on variable-free subtrees, so a `Var`
/// never appears; the `Var` arm returns `false` defensively.
fn structurally_equal(arena: &Arena<'_>, a: NodeRef, b: NodeRef) -> Bool {
    let eq = match (arena.get(a), arena.get(b)) {
        (Node::Lit(x), Node::Lit(y)) => x == y,
        (
            Node::Let { rec: r1, name: n1, value: v1, body: b1 },
            Node::Let { rec: r2, name: n2, value: v2, body: b2 },
        ) => {
            r1 == r2
                && n1 == n2
                && structurally_equal(arena, v1, v2).0
                && structurally_equal(arena, b1, b2).0
        }
        (Node::Lambda { param: p1, body: b1 }, Node::Lambda { param: p2, body: b2 }) => {
            p1 == p2 && structurally_equal(arena, b1, b2).0
        }
        (Node::Apply { callee: c1, args: a1 }, Node::Apply { callee: c2, args: a2 }) => {
            structurally_equal(arena, c1, c2).0 && lists_equal(arena, a1, a2).0
        }
        (Node::Project { base: b1, key: k1 }, Node::Project { base: b2, key: k2 }) => {
            k1 == k2 && structurally_equal(arena, b1, b2).0
        }
        (
            Node::If { cond: c1, then_branch: t1, else_branch: e1 },
            Node::If { cond: c2, then_branch: t2, else_branch: e2 },
        ) => {
            structurally_equal(arena, c1, c2).0
                && structurally_equal(arena, t1, t2).0
                && structurally_equal(arena, e1, e2).0
        }
        (Node::Match { scrutinee: s1, arms: a1 }, Node::Match { scrutinee: s2, arms: a2 }) => {
            structurally_equal(arena, s1, s2).0 && lists_equal(arena, a1, a2).0
        }
        (Node::Iter { seq: s1, body: b1 }, Node::Iter { seq: s2, body: b2 }) => {
            structurally_equal(arena, s1, s2).0 && structurally_equal(arena, b1, b2).0
        }
        (Node::Interp { value: v1 }, Node::Interp { value: v2 }) => {
            structurally_equal(arena, v1, v2).0
        }
        (Node::Raw { family: f1, payload: p1 }, Node::Raw { family: f2, payload: p2 }) => {
            f1 == f2 && lists_equal(arena, p1, p2).0
        }
        (Node::Handle { body: b1, clauses: c1 }, Node::Handle { body: b2, clauses: c2 }) => {
            structurally_equal(arena, b1, b2).0 && lists_equal(arena, c1, c2).0
        }
        _ => false,
    };
    Bool(eq)
}

/// Structural equality over two child lists: same length, pairwise equal.
fn lists_equal(arena: &Arena<'_>, a: vehje_ir::NodeList, b: vehje_ir::NodeList) -> Bool {
    if a.len.0 != b.len.0 {
        return Bool(false);
    }
    let la = arena.list(a);
    let lb = arena.list(b);
    let mut i = 0;
    while i < la.len() {
        if !structurally_equal(arena, la[i], lb[i]).0 {
            return Bool(false);
        }
        i += 1;
    }
    Bool(true)
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
mod tests;
