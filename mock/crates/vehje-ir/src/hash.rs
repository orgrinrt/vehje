//! Structural content-addressed hashing of IR subtrees.
//!
//! One definition, three consumers: hash-consing in `vehje-lower`, dedup
//! keying in `vehje-fixpoint`'s incremental layer (over the content-hash
//! value, not this type), and the manifest in `vehje-runtime-gen`. Defining
//! it per consumer would fork the identity primitive the incremental
//! mechanism's proof-preservation argument depends on. Equal subtrees hash
//! equal, stably across arenas, so a cache hit across artifacts is sound.
//!
//! The fold mixes each node's discriminant with its children's hashes as
//! 64-bit hash words. The dev-side compiler may recurse (the Zig-comptime
//! recursion wall is a runtime concern, not a compile-side one); the arena
//! depth is bounded by the depth cap.

use arvo::strategy::Hot;
use arvo_bits::Bits;
use arvo_hash::ContentHash;

use crate::arena::Arena;
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
        Node::Lit(lit) => h = mix(h, lit_tag(lit)),
        Node::Var(_) => {}
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
        Node::Project { base, .. } => h = fold_child(h, base),
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
        Node::Raw { family, .. } => h = mix(h, family_word(family)),
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

/// A family id as a hash word, for the `Raw` discriminant contribution.
fn family_word(family: crate::node::FamilyId) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: family id widened to a 64-bit hash word; tracked: #207
    family.get().to_raw() as u64 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: family id as a hash-domain word; tracked: #207
}
