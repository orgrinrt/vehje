//! Structural content-addressed hashing of IR subtrees.
//!
//! One within-stage consumer: hash-consing in `vehje-lower`. Cross-artifact
//! identity is a separate concern: the manifest in `vehje-runtime-gen` and
//! `vehje-fixpoint`'s incremental dedup both key on a byte-image hash
//! (`xxhash3_64` over the serialized form), not on this word-fold, precisely
//! because the byte-image is interner-independent.
//!
//! Structurally distinct subtrees hash distinct and structurally equal subtrees
//! hash equal WITHIN ONE INTERNER: the leaf fold mixes a name's interner
//! handle, which is stable within a single interner but not across interners.
//! So this hash is sound for within-compilation hash-consing (one interner);
//! it is NOT interner-stable, and cross-interner or cross-artifact identity is
//! the byte-image hash's job, not this one.
//!
//! The fold mixes each node's discriminant with its children's hashes as
//! 64-bit hash words. The dev-side compiler may recurse (the Zig-comptime
//! recursion wall is a runtime concern, not a compile-side one); the arena
//! depth is bounded by the depth cap.

use arvo::strategy::Hot;
use arvo_bits::Bits;
use arvo_hash::ContentHash;

use crate::arena::Arena;
use crate::intern::Str;
use crate::node::{Literal, Node, NodeRef};

/// A content address of an IR subtree.
///
/// A newtype over the `arvo` content hash. `vehje-fixpoint` and
/// `vehje-runtime-gen` consume the underlying content-hash value; this type
/// names the identity for the `vehje-ir` consumers.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct StructuralHash(ContentHash);

impl StructuralHash {
    /// The underlying content-hash value, for consumers that key on the
    /// content address without naming this type.
    pub fn value(self) -> ContentHash {
        self.0
    }
}

// This is a Merkle-style fold: a node's hash mixes its discriminant with its
// children's content-hash WORDS. The input is hash words (a tree of already
// content-addressed subtrees), not a flat byte image, so it is a different
// operation from `arvo_hash::xxhash3_64` (which `vehje-fixpoint`'s incremental
// layer and `vehje-runtime-gen` use to hash byte images at the wire boundary).
// The word-fold avoids serialising the tree to bytes just to hash it. The
// hashing arithmetic works on raw 64-bit hash words: integer hashing is the
// hash domain, a documented boundary confined to this module and the
// `Bits<64>` construction below.

/// The FNV-1a offset basis and prime, the mixing constants.
const SEED: u64 = 0xcbf2_9ce4_8422_2325; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain mixing constant; tracked: #207
const PRIME: u64 = 0x0000_0100_0000_01b3; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain mixing constant; tracked: #207

/// Mix one hash word into the accumulator (FNV-1a step).
fn mix(h: u64, v: u64) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain words at the hashing boundary; tracked: #207
    (h ^ v).wrapping_mul(PRIME)
}

/// Fold the subtree rooted at `at` to its structural hash.
///
/// Post-order: a node's hash mixes its discriminant with its children's
/// hashes, so structurally equal subtrees hash equal.
pub fn hash_of(arena: &Arena<'_>, at: NodeRef) -> StructuralHash {
    let node = arena.get(at);
    let mut h = mix(SEED, discriminant(&node));
    let fold_child = |h: u64, c: NodeRef| mix(h, hash_of(arena, c).0.to_raw()); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain word from the child's content hash; tracked: #207
    match node {
        Node::Lit(lit) => {
            h = mix(h, lit_tag(lit));
            h = mix(h, lit_value(lit));
        }
        Node::Var(name) => h = mix(h, str_word(name)),
        Node::Let { value, body, .. } => {
            h = fold_child(h, value);
            h = fold_child(h, body);
        }
        Node::Lambda { body, .. } => h = fold_child(h, body),
        Node::Apply { callee, args } => {
            h = fold_child(h, callee);
            for c in arena.list(args) {
                h = fold_child(h, *c);
            }
        }
        Node::Project { base, key } => {
            h = fold_child(h, base);
            h = mix(h, str_word(key));
        }
        Node::If { cond, then_branch, else_branch } => {
            h = fold_child(h, cond);
            h = fold_child(h, then_branch);
            h = fold_child(h, else_branch);
        }
        Node::Match { scrutinee, arms } => {
            h = fold_child(h, scrutinee);
            for c in arena.list(arms) {
                h = fold_child(h, *c);
            }
        }
        Node::Iter { seq, body } => {
            h = fold_child(h, seq);
            h = fold_child(h, body);
        }
        Node::Interp { value } => h = fold_child(h, value),
        Node::Raw { family, payload } => {
            h = mix(h, family_word(family));
            for c in arena.list(payload) {
                h = fold_child(h, *c);
            }
        }
        Node::Handle { body, clauses } => {
            h = fold_child(h, body);
            for c in arena.list(clauses) {
                h = fold_child(h, *c);
            }
        }
    }
    StructuralHash(Bits::<64, Hot>::from_raw(h))
}

/// The node's discriminant, a distinct hash word per Core form.
fn discriminant(node: &Node) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain discriminant; tracked: #207
    match node {
        Node::Lit(_) => 1,
        Node::Var(_) => 2,
        Node::Let { .. } => 3,
        Node::Lambda { .. } => 4,
        Node::Apply { .. } => 5,
        Node::Project { .. } => 6,
        Node::If { .. } => 7,
        Node::Match { .. } => 8,
        Node::Iter { .. } => 9,
        Node::Interp { .. } => 10,
        Node::Raw { .. } => 11,
        Node::Handle { .. } => 12,
    }
}

/// A literal's tag as a hash word.
fn lit_tag(lit: Literal) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain literal tag; tracked: #207
    match lit {
        Literal::Unit => 100,
        Literal::Bool(_) => 101,
        Literal::Int(_) => 102,
        Literal::Str(_) => 103,
    }
}

/// A literal's value as a hash word, so `Int(1)` and `Int(2)` (and distinct
/// bools and interned strings) hash distinct rather than colliding on the tag.
fn lit_value(lit: Literal) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit hash-domain literal value; tracked: #207
    match lit {
        Literal::Unit => 0,
        Literal::Bool(b) => b.0 as u64, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bool widened to a hash-domain word; tracked: #207
        Literal::Int(i) => i.to_raw() as u64, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-bit int raw widened to a hash-domain word; tracked: #207
        Literal::Str(s) => str_word(s),
    }
}

/// An interned name as a hash word, for the leaf content of `Var`, `Project`,
/// and string literals.
// FIXME: this folds the interner HANDLE, which is stable only within one
// interner. That is sufficient for within-compilation hash-consing (the only
// current consumer), but a cross-interner or cross-artifact consumer of
// `hash_of` would need the interned CONTENT folded instead, which means
// threading the interner through `hash_of`. Cross-artifact identity today is
// served by `vehje-fixpoint`'s byte-image `xxhash3_64`, not this fold. Tracked
// #29. The DESIGN's hash section still says "stable across arenas"; that
// clause is owed a narrowing in a follow-up doc round (the templates are locked
// this round).
fn str_word(s: Str) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: interned string handle widened to a hash-domain word; tracked: #207
    s.to_bits().to_raw() as u64 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 32-bit interned handle widened to a hash-domain word; tracked: #207
}

/// A family id as a hash word, for the `Raw` discriminant contribution.
fn family_word(family: crate::node::FamilyId) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: family id widened to a 64-bit hash word; tracked: #207
    family.get().to_raw() as u64 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: family id as a hash-domain word; tracked: #207
}

#[cfg(test)]
mod tests {
    use super::*;

    use arvo::strategy::Hot;
    use arvo::{Bool, Identity, Int, USize};
    use hilavitkutin_str::str_const;

    use crate::builder::Builder;
    use crate::node::Literal;
    use crate::span::Span;

    fn at(m: arvo::Maybe<NodeRef>) -> NodeRef {
        match m {
            arvo::Maybe::Is(r) => r,
            arvo::Maybe::Isnt => panic!("arena full"),
        }
    }

    fn arena4() -> ([Node; 4], [Span; 4], [NodeRef; 4]) {
        (
            [Node::Lit(Literal::Unit); 4],
            [Span::default(); 4],
            [NodeRef::new(USize::ZERO); 4],
        )
    }

    // the regression the leaf-content fold fixes: same discriminant, same
    // literal tag, different value must hash distinct, not collide.
    #[test]
    fn distinct_int_literals_hash_distinct() {
        let (mut n, mut s, mut p) = arena4();
        let mut b = Builder::new(Arena::new(&mut n, &mut s, &mut p));
        let one = at(b.lit(Literal::Int(Int::<64, Hot>::from_raw(1)), Span::default()));
        let two = at(b.lit(Literal::Int(Int::<64, Hot>::from_raw(2)), Span::default()));
        let arena = b.into_arena();
        assert_ne!(hash_of(&arena, one), hash_of(&arena, two));
    }

    #[test]
    fn distinct_bool_literals_hash_distinct() {
        let (mut n, mut s, mut p) = arena4();
        let mut b = Builder::new(Arena::new(&mut n, &mut s, &mut p));
        let t = at(b.lit(Literal::Bool(Bool::TRUE), Span::default()));
        let f = at(b.lit(Literal::Bool(Bool::FALSE), Span::default()));
        let arena = b.into_arena();
        assert_ne!(hash_of(&arena, t), hash_of(&arena, f));
    }

    // the reviewer's flagged case: `Var(x)` and `Var(y)` must not collide.
    #[test]
    fn distinct_vars_hash_distinct() {
        let (mut n, mut s, mut p) = arena4();
        let mut b = Builder::new(Arena::new(&mut n, &mut s, &mut p));
        let x = at(b.var(str_const!("x"), Span::default()));
        let y = at(b.var(str_const!("y"), Span::default()));
        let arena = b.into_arena();
        assert_ne!(hash_of(&arena, x), hash_of(&arena, y));
    }

    // equal subtrees still hash equal (the invariant the incremental layer needs).
    #[test]
    fn equal_int_literals_hash_equal() {
        let (mut n, mut s, mut p) = arena4();
        let mut b = Builder::new(Arena::new(&mut n, &mut s, &mut p));
        let a = at(b.lit(Literal::Int(Int::<64, Hot>::from_raw(7)), Span::default()));
        let c = at(b.lit(Literal::Int(Int::<64, Hot>::from_raw(7)), Span::default()));
        let arena = b.into_arena();
        assert_eq!(hash_of(&arena, a), hash_of(&arena, c));
    }
}
