//! Congruence closure: an e-class union-find over caller-provided storage.
//!
//! When two terms are proven equal their classes merge, which is what an
//! equality-saturation client needs and a plain Datalog engine does not carry.
//! The closure is iterative, driven by a work-stack, never recursive, because
//! recursion is a hard compiler wall on the runtime side. The parent array is
//! caller-lent, like every other relation store here.

use arvo::{Bool, USize};

/// A union-find over an e-class parent array the caller provides.
///
/// The type carries no state; every operation takes the caller's `parents`
/// slice, where `parents[x]` is `x`'s parent and a root is its own parent. The
/// caller sizes and owns the slice, so there is no allocation here.
pub struct Congruence;

impl Congruence {
    /// A congruence handle. Stateless; operations take the parent slice.
    pub fn new() -> Self {
        Self
    }

    /// The representative of `x`, with iterative path halving.
    ///
    /// Path halving points each visited node at its grandparent as it walks,
    /// flattening the tree over time without a second pass and without
    /// recursion.
    pub fn find(parents: &mut [USize], x: USize) -> USize {
        let mut cur = x;
        while parents[cur.0] != cur {
            let grand = parents[parents[cur.0].0];
            parents[cur.0] = grand;
            cur = grand;
        }
        cur
    }

    /// Union the classes of `a` and `b`. `Bool::TRUE` if they were distinct.
    ///
    /// A `TRUE` result is the signal a new equality was recorded, which a
    /// caller propagating congruence uses to enqueue the parents that may now
    /// have become equal.
    pub fn union(parents: &mut [USize], a: USize, b: USize) -> Bool {
        let ra = Self::find(parents, a);
        let rb = Self::find(parents, b);
        if ra == rb {
            return Bool(false);
        }
        parents[rb.0] = ra;
        Bool(true)
    }

    /// Close a batch of pending equalities iteratively over the work-stack.
    ///
    /// The caller-lent `pending` slice is the work-stack: each pair unions two
    /// classes. Returns the number of unions that merged distinct classes.
    // FIXME: this closes the given equalities but does not yet re-derive the
    // congruence rule (equal terms in equal positions are equal): merging two
    // classes should enqueue their parents' equalities. The full slotted /
    // colored congruence extension, where the work-stack grows as parents
    // become equal, is deferred until the equality-saturation client needs it.
    pub fn close(parents: &mut [USize], pending: &[(USize, USize)]) -> USize {
        let mut merges = USize(0);
        let mut i = USize(0);
        while i < USize(pending.len()) {
            let (a, b) = pending[i.0];
            if Self::union(parents, a, b).0 {
                merges = merges + USize(1);
            }
            i = i + USize(1);
        }
        merges
    }
}

impl Default for Congruence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_merges_classes() {
        // four singleton classes: each node is its own root.
        let mut parents = [USize(0), USize(1), USize(2), USize(3)];

        assert!(Congruence::union(&mut parents, USize(0), USize(1)).0);
        assert!(Congruence::union(&mut parents, USize(2), USize(3)).0);
        // 0 and 1 already share a class.
        assert!(!Congruence::union(&mut parents, USize(0), USize(1)).0);

        assert_eq!(
            Congruence::find(&mut parents, USize(0)),
            Congruence::find(&mut parents, USize(1))
        );

        // closing (1, 3) merges the {0,1} and {2,3} classes.
        let merges = Congruence::close(&mut parents, &[(USize(1), USize(3))]);
        assert_eq!(merges, USize(1));
        assert_eq!(
            Congruence::find(&mut parents, USize(0)),
            Congruence::find(&mut parents, USize(2))
        );
    }
}
