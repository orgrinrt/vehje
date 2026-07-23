//! The grade vocabulary: the four proof axes as concrete runtime types.
//!
//! The graded proof spine lives in the proof and in the design's coherence
//! argument; at the machine level the axes are concrete closed enums and
//! fixed-capacity bitmasks. All four reuse one bitmask machinery: the
//! `arvo_bitmask::Mask` over the `arvo_bits::QWord` word for the runtime
//! masks, so a join is a bitwise union and inclusion is a subset test. A
//! `vehje-check` pass computes the grades and writes them into `GradeTable`,
//! a side-table parallel to the node arena.

use arvo::strategy::Hot;
use arvo::{Bool, USize};
use arvo_bits::Bits;
use arvo_bitmask::Mask;

/// The word backing every grade mask: 64 slots.
type GradeWord = Bits<64, Hot>;

/// The runtime bitmask of a construct's inferred effect operations.
///
/// Thermometer-shaped so the lattice join is a bitwise union and the
/// inclusion check against a target's permitted set is a subset test.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct EffectMask(Mask<GradeWord>);

impl core::fmt::Debug for EffectMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EffectMask({:?})", self.0.to_word())
    }
}

impl EffectMask {
    /// The empty effect (a pure construct).
    pub fn empty() -> Self {
        Self(Mask::empty())
    }

    /// Record that operation slot `op` is invoked.
    pub fn insert(&mut self, op: USize) {
        self.0.insert(op);
    }

    /// Whether operation slot `op` is in the set.
    pub fn contains(self, op: USize) -> Bool {
        self.0.contains(op)
    }

    /// The lattice join (union) of two effect sets.
    pub fn join(self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }

    /// Whether `self` is included in `permitted` (the inclusion check): every
    /// operation of `self` is in `permitted`.
    pub fn included_in(self, permitted: Self) -> Bool {
        Bool(self.0.union(permitted.0).to_word() == permitted.0.to_word())
    }
}

/// The reachability qualifier: the set of in-scope binder slots a value can
/// reach. The lease coeffect's grade.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct ReachMask(Mask<GradeWord>);

impl core::fmt::Debug for ReachMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ReachMask({:?})", self.0.to_word())
    }
}

impl ReachMask {
    /// Reaches no binder (a closed value).
    pub fn empty() -> Self {
        Self(Mask::empty())
    }

    /// Record that binder slot `b` is reached.
    pub fn insert(&mut self, b: USize) {
        self.0.insert(b);
    }

    /// Whether binder slot `b` is reached.
    pub fn contains(self, b: USize) -> Bool {
        self.0.contains(b)
    }

    /// The union of two reach sets (child propagation before the binder rule
    /// drops the bound variable).
    pub fn join(self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }
}

/// One knowledge source in the binding-time lattice.
///
/// A closed set: the sources a value's inputs can be known at. The
/// build-versus-runtime split lives here, not on the effect axis.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BindingTime {
    /// Known when the language author writes the language definition.
    LanguageAuthor,
    /// Known when a program is bundled at our build.
    Bundler,
    /// Known when the host loads a program.
    HostLoader,
    /// Known only at runtime.
    Runtime,
}

/// A binding-time grade: a join over the knowledge sources, so partial
/// bundling is a join rather than a special case.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Knowledge(Mask<GradeWord>);

impl core::fmt::Debug for Knowledge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Knowledge({:?})", self.0.to_word())
    }
}

impl Knowledge {
    /// Nothing known yet.
    pub fn empty() -> Self {
        Self(Mask::empty())
    }

    /// Record that a source is available.
    pub fn insert(&mut self, source: BindingTime) {
        self.0.insert(source.slot());
    }

    /// Whether a source is available.
    pub fn has(self, source: BindingTime) -> Bool {
        self.0.contains(source.slot())
    }

    /// The join of two binding-time grades.
    pub fn join(self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }
}

impl BindingTime {
    /// The bit slot this source occupies in a `Knowledge` grade.
    // lint:allow(no-bare-numeric) reason: the four knowledge-source bit slots are a fixed index contract into the Knowledge mask, analogous to an enum discriminant; tracked: #207
    fn slot(self) -> USize {
        match self {
            BindingTime::LanguageAuthor => USize(0),
            BindingTime::Bundler => USize(1),
            BindingTime::HostLoader => USize(2),
            BindingTime::Runtime => USize(3),
        }
    }
}

/// The lease grade of a value: its reachability qualifier.
///
/// The degenerate immutable strictly-nested depth-lease is a `ReachMask`
/// collapsed to lexical nesting, the floor that ships first.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Lease(pub ReachMask);

/// The assurance lattice: how a property is discharged.
///
/// A closed set, computed as a projection of the binding-time grade crossed
/// with the property.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Assurance {
    /// Proven statically and folded to a compile error on violation.
    StaticCertified,
    /// Made unrepresentable in the generated structure.
    StructurallyCertified,
    /// Re-proven per instance at our build.
    ReprovenPerInstance,
    /// Checked by a bounded dynamic residual at runtime.
    DynamicallyChecked,
    /// Checked by the differential harness, not proven.
    Tested,
    /// No guarantee claimed.
    #[default]
    Unclaimed,
}

/// The per-node grade record the check pass computes.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Grade {
    /// The construct's inferred effect operations.
    pub effect: EffectMask,
    /// The construct's lease (reachability qualifier).
    pub lease: Lease,
    /// The construct's binding-time grade.
    pub binding: Knowledge,
    /// The construct's assurance.
    pub assurance: Assurance,
}

/// The grade side-table: one `Grade` per node, keyed by the node's index,
/// over a caller-provided region parallel to the node arena.
///
/// Written by the `vehje-check` graded fold, read by `vehje-lower` and the
/// emit stage. No allocation: the region is caller-lent, like the arena.
pub struct GradeTable<'a> {
    grades: &'a mut [Grade],
}

impl<'a> GradeTable<'a> {
    /// Wrap a caller-provided region sized like the node arena.
    ///
    /// The caller must size the region to at least the node arena's capacity;
    /// `get` and `set` index by the node's arena index and assume it is in
    /// range (the arena's `push` bounds-checks node creation, so a table sized
    /// to the arena is always large enough).
    // FIXME: return a `Maybe`/`Outcome` from `get`/`set` (or take the arena
    // length) so an undersized region is a diagnostic rather than a panic; the
    // check pass sizes the region to the arena today.
    pub fn new(grades: &'a mut [Grade]) -> Self {
        Self { grades }
    }

    /// Read a node's grade by its arena index.
    pub fn get(&self, at: USize) -> Grade {
        self.grades[at.0]
    }

    /// Write a node's grade by its arena index.
    pub fn set(&mut self, at: USize, grade: Grade) {
        self.grades[at.0] = grade;
    }
}
