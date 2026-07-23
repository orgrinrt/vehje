//! The relational core: relations, rules, the evaluator, and per-round deltas.
//!
//! A relation is a set of tuples the engine grows to a fixpoint. A rule
//! derives head tuples from body tuples. `Engine::evaluate` runs a rule over
//! caller-provided columnar storage until no new tuple is derived, using
//! whole-column evaluation (the reach fixpoint's measured winner). The
//! relations are finite-height lattices and the rules are monotone, so the
//! loop terminates.

use arvo::strategy::Hot;
use arvo::{Bits, Bool, USize};
use arvo_bitmask::Mask;
use hilavitkutin_api::ColumnValue;
use notko::Maybe;

/// The bit word backing the per-round delta mask: 64 relation slots.
type DeltaWord = Bits<64, Hot>;

/// One relation a client defines over the engine.
///
/// The tuple type is the record the relation stores (small and `Copy`, a
/// `hilavitkutin-api` column value). The slot indexes the relation in the
/// engine's delta mask. The caller lends the relation's columnar storage to
/// `Engine::evaluate` as a `RelationSet`; the schema here names the tuple and
/// the slot, not the storage.
pub trait Relation {
    /// The tuple type stored in this relation.
    type Tuple: ColumnValue;
    /// The relation's bit slot in the engine's 64-slot delta mask.
    const SLOT: USize;
}

/// A semi-naive rule: a body relation, a head relation, and the derivation.
///
/// The shipping shape is single-body: one body tuple derives at most one head
/// tuple. `Maybe::Isnt` means the body tuple fires nothing this round.
// FIXME: the full multi-relation worst-case-optimal join (leapfrog-triejoin
// with AGM bounds) is deferred; a single-body rule cannot express a
// multi-way join. Lands when a client's rule set needs it (see BACKLOG).
pub trait Rule {
    /// The relation this rule reads and writes.
    ///
    /// The shipping single-body shape reads and writes one relation (the
    /// reach / transitive-closure shape). A body and head split into distinct
    /// relations arrives with the multi-relation join.
    type Rel: Relation;

    /// Derive a head tuple from one body tuple, or `Maybe::Isnt`.
    fn derive(
        &self,
        tuple: <Self::Rel as Relation>::Tuple,
    ) -> Maybe<<Self::Rel as Relation>::Tuple>;
}

/// The caller-provided columnar storage of one relation, with set semantics.
///
/// The engine reads tuples by position and inserts derived tuples; `insert`
/// returns `Bool::TRUE` when the tuple was newly added (absent before), which
/// is how the fixpoint detects that a round changed the relation. Storage is
/// the caller's: a fixed-capacity column, never a heap-grown container.
pub trait RelationSet<T: ColumnValue> {
    /// The number of tuples currently in the relation.
    fn len(&self) -> USize;
    /// The tuple at position `at` (`at < len`).
    fn get(&self, at: USize) -> T;
    /// Insert `tuple` if absent. `Bool::TRUE` when newly added.
    fn insert(&mut self, tuple: T) -> Bool;
}

/// The per-round change set a semi-naive step produces.
///
/// `changed` marks which relation slots gained a tuple; `derived` counts the
/// tuples added. The bookkeeping is what makes re-evaluation touch only what
/// changed once semi-naive deltas ship.
pub struct Delta {
    changed: Mask<DeltaWord>,
    derived: USize,
}

impl Delta {
    /// An empty delta: no slot changed, nothing derived.
    pub fn empty() -> Self {
        Self { changed: Mask::empty(), derived: USize(0) }
    }

    /// Whether no relation slot changed (the fixpoint signal).
    pub fn is_empty(&self) -> Bool {
        self.changed.is_empty()
    }

    /// Whether relation slot `slot` gained a tuple.
    pub fn touched(&self, slot: USize) -> Bool {
        self.changed.contains(slot)
    }

    /// The count of tuples derived across the evaluation.
    pub fn derived(&self) -> USize {
        self.derived
    }

    fn mark(&mut self, slot: USize) {
        self.changed.insert(slot);
    }

    fn add_one(&mut self) {
        self.derived = self.derived + USize(1);
    }
}

/// The evaluator.
///
/// Holds the accumulated delta and a round counter across `evaluate` calls.
/// One engine drives every relation a client defines; the relations live in
/// caller storage, not here.
pub struct Engine {
    delta: Delta,
    rounds: USize,
}

impl Engine {
    /// A fresh engine with an empty delta and zero rounds.
    pub fn new() -> Self {
        Self { delta: Delta::empty(), rounds: USize(0) }
    }

    /// The accumulated per-round delta.
    pub fn delta(&self) -> &Delta {
        &self.delta
    }

    /// The total rounds evaluated across all `evaluate` calls.
    pub fn rounds(&self) -> USize {
        self.rounds
    }

    /// Run `rule` over `set` to a fixpoint, returning the rounds taken.
    ///
    /// Whole-column evaluation: each round scans the tuples present at the
    /// round's start, derives from each, and inserts the results. A round
    /// that adds nothing is the fixpoint. Monotone rules over a finite set
    /// terminate. Newly inserted tuples are seen on the next round, so the
    /// derivation reaches transitive consequences.
    // FIXME: this ships the whole-column strategy (the reach fixpoint's
    // measured winner on shallow graphs) only. Semi-naive delta evaluation
    // (firing a rule only on the previous round's new tuples) and the
    // provenance-semiring grades are deferred; the `Delta` type is reserved
    // for them. The DESIGN's "semi-naive deltas where the bookkeeping repays
    // itself" is not yet built.
    pub fn evaluate<R, S>(&mut self, rule: &R, set: &mut S) -> USize
    where
        R: Rule,
        S: RelationSet<<R::Rel as Relation>::Tuple>,
    {
        let mut rounds = USize(0);
        loop {
            let mut changed = Bool(false);
            let len = set.len();
            let mut i = USize(0);
            while i < len {
                let tuple = set.get(i);
                if let Maybe::Is(derived) = rule.derive(tuple) {
                    if set.insert(derived).0 {
                        changed = Bool(true);
                        self.delta.add_one();
                    }
                }
                i = i + USize(1);
            }
            rounds = rounds + USize(1);
            if !changed.0 {
                break;
            }
            self.delta.mark(<R::Rel as Relation>::SLOT);
        }
        self.rounds = self.rounds + rounds;
        rounds
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Reach;
    impl Relation for Reach {
        type Tuple = USize;
        const SLOT: USize = USize(0);
    }

    struct IncrToCap {
        cap: USize,
    }
    impl Rule for IncrToCap {
        type Rel = Reach;
        fn derive(&self, t: USize) -> Maybe<USize> {
            let next = t + USize(1);
            if next < self.cap { Maybe::Is(next) } else { Maybe::Isnt }
        }
    }

    struct ArraySet {
        data: [USize; 8],
        len: USize,
    }
    impl ArraySet {
        fn new() -> Self {
            Self { data: [USize(0); 8], len: USize(0) }
        }
    }
    impl RelationSet<USize> for ArraySet {
        fn len(&self) -> USize {
            self.len
        }
        fn get(&self, at: USize) -> USize {
            self.data[at.0]
        }
        fn insert(&mut self, tuple: USize) -> Bool {
            let mut i = USize(0);
            while i < self.len {
                if self.data[i.0] == tuple {
                    return Bool(false);
                }
                i = i + USize(1);
            }
            self.data[self.len.0] = tuple;
            self.len = self.len + USize(1);
            Bool(true)
        }
    }

    #[test]
    fn reach_fixpoint_grows_to_cap() {
        let mut set = ArraySet::new();
        set.insert(USize(0));
        let rule = IncrToCap { cap: USize(5) };
        let mut engine = Engine::new();
        let rounds = engine.evaluate(&rule, &mut set);

        // starting {0}, deriving t+1 while < 5, closes to {0,1,2,3,4}.
        assert_eq!(set.len(), USize(5));
        assert!(engine.delta().touched(USize(0)).0);
        assert!(!engine.delta().is_empty().0);
        assert_eq!(engine.delta().derived(), USize(4));
        assert!(rounds > USize(1));
    }
}
