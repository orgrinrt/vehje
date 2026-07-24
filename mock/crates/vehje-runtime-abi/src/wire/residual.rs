//! The residual crossing descriptor: a control-flow-graph-of-blocks program.
//!
//! The residual is not a flat value. The runtime parses, lowers, and executes
//! it for arriving scripts, so the wire form models a program: a serialized
//! node arena, the child-index pool, the string blob, a block table, a function
//! table, and the root. Control flow lives only at block terminators. A loop is
//! a back-edge carrying loop-carried values as block arguments; a call saves a
//! return record on a depth-capped frame stack.
//!
//! This module names the descriptor types. The serialization that populates the
//! block, function, and diagnostics sections rides on the encoder in
//! [`super::serialize`]; the node/pool/blob sections are written today, the
//! higher-level tables are schema-now, population-later.

use vehje_ir::{NodeRef, Span, Str};

use arvo::{Bool, USize};

/// The representation a residual crosses the ABI in.
///
/// Narrowed to name only the internal execution form: interpret the serialized
/// arena, or run the lowered bytecode. It is not a native-versus-managed axis.
/// Native is a method-agnostic point on the output spectrum, chosen per region
/// where a grade proves it wins, not a tier tag here.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tier {
    /// The flat serialized IR arena: the reference semantics, and the form the
    /// first working version ships.
    Arena,
    /// The optimized linear bytecode (encoder deferred, see BACKLOG).
    Bytecode,
}

/// A block identifier: an index into the block table.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockId(pub USize);

/// A contiguous range of nodes in the serialized node arena.
///
/// A block is a maximal straight-line region, so its nodes are a single
/// contiguous span: `len` records starting at `start`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NodeRange {
    /// The first node of the range.
    pub start: NodeRef,
    /// The number of nodes in the range.
    pub len: USize,
}

/// The kind of control transfer a block ends with.
///
/// Control flow lives only at terminators. A conditional is a `Branch` between
/// two successors; an unconditional edge (including a loop back-edge) is a
/// `Jump`; a `Call` transfers into a function and saves a return record; a
/// `Return` unwinds one frame.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TerminatorKind {
    /// Return from the current function frame.
    Return,
    /// Unconditional transfer to a single successor (a forward edge or a loop
    /// back-edge).
    Jump,
    /// Conditional transfer between successors on a scrutinee.
    Branch,
    /// Transfer into a callee, saving a return record on the frame stack.
    Call,
}

/// A range of successor block ids in the successor-id pool.
///
/// A block's successors are a contiguous slice of the flat successor pool, the
/// same indirection shape the node arena uses for child lists.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SuccRange {
    /// The first successor id.
    pub start: USize,
    /// The number of successor ids.
    pub len: USize,
}

/// A value signature: the arity of a block-argument, function-argument, or
/// function-result list.
///
/// Block arguments carry loop-carried values across a back-edge; a function's
/// argument and result signatures name its calling convention.
// FIXME: the arity alone is a placeholder; per-position value-kind tags land
// with the value-kind vocabulary the check pass fixes. tracked: #207
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Signature {
    /// The number of values in the list.
    pub arity: USize,
}

/// One block table record: its node range, terminator, successors, and the
/// block-argument signature.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Block {
    /// The contiguous node range that forms the block's straight-line body.
    pub nodes: NodeRange,
    /// How the block ends.
    pub terminator: TerminatorKind,
    /// The block's successor block ids.
    pub successors: SuccRange,
    /// The loop-carried block-argument signature.
    pub args: Signature,
}

/// One function table record: its entry block and calling convention.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Function {
    /// The block a call to this function enters.
    pub entry: BlockId,
    /// The argument signature.
    pub args: Signature,
    /// The result signature.
    pub results: Signature,
}

/// The block table: the blocks of the residual's control-flow graph.
// FIXME: no producer constructs `Block` / `Function` / `BlockTable` yet. The
// lowering from `vehje_ir::Node` into blocks is owed, downstream of
// `vehje-lower`'s `Anf` (which names the join points that give a nested branch
// its successor blocks). These are the designed residual shape the producer
// builds toward, not dead code to remove.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockTable<'img> {
    /// The block records, indexed by [`BlockId`].
    pub blocks: &'img [Block],
}

impl<'img> BlockTable<'img> {
    /// Wrap a block-record slice.
    pub const fn new(blocks: &'img [Block]) -> Self {
        Self { blocks }
    }

    /// The number of blocks.
    pub fn len(&self) -> USize {
        USize(self.blocks.len())
    }

    /// Whether the table is empty.
    pub fn is_empty(&self) -> Bool {
        Bool(self.blocks.is_empty())
    }
}

/// The function table: the entry points and calling conventions of the
/// residual's functions.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FunctionTable<'img> {
    /// The function records.
    pub functions: &'img [Function],
}

impl<'img> FunctionTable<'img> {
    /// Wrap a function-record slice.
    pub const fn new(functions: &'img [Function]) -> Self {
        Self { functions }
    }

    /// The number of functions.
    pub fn len(&self) -> USize {
        USize(self.functions.len())
    }

    /// Whether the table is empty.
    pub fn is_empty(&self) -> Bool {
        Bool(self.functions.is_empty())
    }
}

/// One binder-site record: where a name is bound, for witness reconstruction.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BinderSite {
    /// The binding node (a `Let`, `Lambda`, or handler clause).
    pub node: NodeRef,
    /// The bound name.
    pub name: Str,
}

/// One provenance record: a relation instance and where it originated.
///
/// The per-relation provenance arrays let the engine reconstruct which
/// lease or effect relation a failing record participated in, and the node
/// that introduced it.
// FIXME: `relation` is a bare origin node until the relation-kind vocabulary
// from the grade tables is mirrored here; the per-relation split into distinct
// arrays lands with the reconstruction query. tracked: #207
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProvenanceEntry {
    /// The node the relation constrains.
    pub node: NodeRef,
    /// The node that introduced the relation.
    pub origin: NodeRef,
}

/// One violation seed: the minimal datum to lazily reconstruct a lease or
/// effect error's witness on the failure path.
// FIXME: the seed carries only the failing site today; the seed's relation
// selector and the witness-replay cursor land with the reconstruction query.
// tracked: #207
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ViolationSeed {
    /// The node at which the violation surfaces.
    pub site: NodeRef,
}

/// The diagnostics schema carried in the wire format.
///
/// The six schema additions that let the single engine reconstruct a lease or
/// effect error's witness lazily on the failure path: the span table, the
/// binder-site table, the per-relation provenance arrays, and the violation
/// seeds (the optional residual debug section and the provenance-complete
/// engine boundary are properties of the surrounding image, not slices here).
/// Landed in the wire format now so the reconstruction query has them.
// FIXME: all four arrays are populated by the check pass emitting provenance as
// it discharges relations; the encoder walk does not yet write them, so a
// serialized residual carries empty diagnostics until that emit lands.
// tracked: #207
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DiagnosticsSchema<'img> {
    /// One source span per node, parallel to the node arena.
    pub spans: &'img [Span],
    /// The binder sites, for name resolution on the failure path.
    pub binder_sites: &'img [BinderSite],
    /// The per-relation provenance records.
    pub provenance: &'img [ProvenanceEntry],
    /// The violation seeds.
    pub violation_seeds: &'img [ViolationSeed],
}

impl<'img> DiagnosticsSchema<'img> {
    /// An empty diagnostics schema: no spans, binders, provenance, or seeds.
    ///
    /// The shape a residual carries until the check pass emits provenance.
    pub const EMPTY: Self = Self {
        spans: &[],
        binder_sites: &[],
        provenance: &[],
        violation_seeds: &[],
    };
}

/// A produced residual's crossing descriptor: typed views over the serialized
/// image sections the runtime reads.
///
/// Carries the serialized node arena, the child-index pool, the string blob,
/// the block table, the function table, and the root, plus the tier it was
/// serialized at and the diagnostics schema. The three raw sections are byte
/// views into the caller-provided image ([`super::serialize`] wrote them); the
/// block, function, and diagnostics tables are the higher-level schema over the
/// same image.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Residual<'img> {
    /// The representation the serialized residual crosses in.
    pub tier: Tier,
    /// The program's root node.
    pub root: NodeRef,
    /// The serialized node arena bytes.
    pub nodes: &'img [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: serialized FFI wire node-arena bytes; tracked: #207
    /// The serialized child-index pool bytes.
    pub pool: &'img [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: serialized FFI wire child-pool bytes; tracked: #207
    /// The serialized string blob bytes.
    pub blob: &'img [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: serialized FFI wire string-blob bytes; tracked: #207
    /// The control-flow-graph block table.
    pub blocks: BlockTable<'img>,
    /// The function table.
    pub functions: FunctionTable<'img>,
    /// The diagnostics schema for lazy witness reconstruction.
    pub diagnostics: DiagnosticsSchema<'img>,
}
