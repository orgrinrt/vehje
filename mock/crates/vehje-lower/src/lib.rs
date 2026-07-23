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
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::Bool;
use vehje_ir::{hash_of, Arena, NodeRef, StructuralHash};

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
    /// Fold the subtree rooted at `root`, returning the (possibly new) root.
    // FIXME: fold a node whose operands are all `Lit` and whose binding-time
    // grade proves them statically known; M-level walks and returns the root
    // unchanged until the arena-rewrite (building the folded node) lands.
    pub fn apply(&self, _arena: &Arena<'_>, root: NodeRef) -> NodeRef {
        root
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

    /// Share equal subtrees under `root`, returning the (possibly new) root.
    // FIXME: build the hash-cons table (StructuralHash to NodeRef) and rewrite
    // a duplicate subtree to a reference to its first occurrence; M-level
    // exposes the keying hash and returns the root unchanged.
    pub fn apply(&self, _arena: &Arena<'_>, root: NodeRef) -> NodeRef {
        root
    }
}

/// A-normal form: name every intermediate so effect order is explicit in the
/// residual and no separate sequencing form is needed.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Anf;

impl Anf {
    /// Normalise the subtree rooted at `root` to A-normal form.
    // FIXME: hoist each non-trivial subexpression into a `Let` binding so
    // evaluation order equals binding order; M-level returns the root unchanged
    // until the arena-rewrite lands.
    pub fn apply(&self, _arena: &Arena<'_>, root: NodeRef) -> NodeRef {
        root
    }
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
    // splice the expansion; M-level returns the root unchanged (no consumer
    // family defines a macro operation yet).
    pub fn apply(&self, _arena: &Arena<'_>, root: NodeRef) -> NodeRef {
        root
    }
}

/// The lowering stage entry.
///
/// Takes a `LowerStrategy` and rewrites the IR through the arena, returning the
/// new root. The strategy is where the two strata plug in, so a caller chooses
/// the cheap subset or (later) saturation without the stage changing shape.
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

    /// Lower the program rooted at `root` under `strategy`, returning the new
    /// root. Under `Cheap`, expands macros, then folds constants, then shares
    /// common subexpressions, then normalises to A-normal form.
    pub fn lower(&self, strategy: LowerStrategy, arena: &Arena<'_>, root: NodeRef) -> NodeRef {
        match strategy {
            LowerStrategy::Cheap => {
                let mut r = MacroExpand.apply(arena, root);
                if self.rules.const_fold.0 {
                    r = ConstFold.apply(arena, r);
                }
                if self.rules.cse.0 {
                    r = Cse.apply(arena, r);
                }
                Anf.apply(arena, r)
            }
            // FIXME: the Saturate stratum (the deferred dev-time e-graph) runs
            // here behind the same interface; M-level falls through to the
            // cheap subset until it lands.
            LowerStrategy::Saturate => self.lower(LowerStrategy::Cheap, arena, root),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo::{Identity, USize};
    use vehje_ir::{Builder, Literal, Node, Span};

    fn at(m: arvo::Maybe<NodeRef>) -> NodeRef {
        match m {
            arvo::Maybe::Is(r) => r,
            arvo::Maybe::Isnt => panic!("arena full"),
        }
    }

    #[test]
    fn cheap_lowering_preserves_a_trivial_program() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
        let unit = at(b.lit(Literal::Unit, Span::default()));
        let arena = b.into_arena();

        assert!(matches!(arena.get(unit), Node::Lit(_)));
        let out = Lower::cheap().lower(LowerStrategy::Cheap, &arena, unit);
        assert_eq!(out, unit);
    }
}
