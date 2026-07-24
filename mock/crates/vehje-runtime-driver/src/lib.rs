//! vehje-runtime-driver, the framework's compile-side runtime dispatch.
//!
//! The compiler-side embedding and dispatch over the wire ABI, and the
//! in-process consumer of what crosses back: it serializes a checked residual
//! through `vehje-runtime-abi`, hands it to the embedded runtime, maps the
//! runtime's result and diagnostics back to `vehje-ir` diagnostics, and holds
//! the safe reader over the value-arena a produced value crosses back in. The
//! one framework crate with a `std` role, for the dlopen of the runtime
//! artifact.
//!
//! `#![no_std]` on the core paths, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::USize;
use notko::Outcome;
use vehje_runtime_abi::{Residual, ValueArena};

/// A driver diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DriverError {
    /// The runtime rejected the residual.
    RuntimeRejected,
    /// The residual could not be serialized at the requested tier.
    SerializeFailed,
    /// A produced value failed the structural decode (a corrupt value-arena).
    CorruptValue,
}

/// Hand a checked residual to the runtime and read back its outcome.
///
/// Defines the dispatch entry; serialization through the tier tag, the runtime
/// linkage, and the result and diagnostic mapping are the next behavior gate.
// FIXME: serialize the residual at its tier through vehje-runtime-abi, call
// into the embedded runtime (statically linked, or dlopen'd behind the std
// feature), and map the wire diagnostics back to vehje-ir::Diagnostic. The
// surface ships; the embedding is owed.
pub fn dispatch(_residual: &Residual<'_>) -> Outcome<(), DriverError> {
    Outcome::Ok(())
}

/// The safe, bounds-checked reader over a produced value-arena.
///
/// The runtime output is a `vehje-runtime-abi` value-arena; the driver is the
/// in-process consumer, so it holds the safe reader and the validation pass a
/// produced value passes before the host reads it (the typed structural decode,
/// which the in-process path may skip and the untrusted path must run).
pub struct Reader<'a> {
    arena: ValueArena<'a>,
}

impl<'a> Reader<'a> {
    /// Wrap a produced value-arena for reading.
    pub fn new(arena: ValueArena<'a>) -> Self {
        Self { arena }
    }

    /// The wrapped value-arena, for the host to read its produced value.
    pub fn value(&self) -> &ValueArena<'a> {
        &self.arena
    }

    /// Validate the value-arena before the host reads it: a complete, linear,
    /// bounds-checked typed structural decode.
    ///
    /// One forward pass over the node records. For each node it checks that the
    /// region id is in the region table, the blob span is within the blob, the
    /// child span is within the child pool, and every child index is strictly
    /// below the node's own index. The children-before-parents emission order
    /// makes that last check the acyclicity proof: a value arena that passes has
    /// no forward reference and therefore no cycle, so a later read cannot loop
    /// or run out of bounds. The untrusted path must run this; the trusted
    /// in-process path may skip it.
    pub fn validate(&self) -> Outcome<(), DriverError> {
        let a = &self.arena;
        let n = a.len().0;
        let blob_len = a.blob.len();
        let pool_len = a.pool.len();
        let region_count = a.regions.len();

        if a.root.index().0 >= n {
            return Outcome::Err(DriverError::CorruptValue);
        }

        // each region-table entry bounds a contiguous node segment the runtime
        // frees on stack discipline, so its range must lie within the node
        // count (saturating so a malformed entry cannot overflow the check).
        let mut r = USize(0);
        while r.0 < region_count {
            let region = a.regions[r.0];
            if region.start.0.saturating_add(region.len.0) > n {
                return Outcome::Err(DriverError::CorruptValue);
            }
            r = USize(r.0 + 1);
        }

        let mut i = USize(0);
        while i.0 < n {
            let node = a.nodes[i.0];

            // the region id names a real region-table slot.
            if node.region.0 .0 >= region_count {
                return Outcome::Err(DriverError::CorruptValue);
            }

            // the blob span lies within the blob (saturating so a malformed
            // span cannot overflow the bound check).
            let blob_end = node.blob.offset.0.saturating_add(node.blob.len.0);
            if blob_end > blob_len {
                return Outcome::Err(DriverError::CorruptValue);
            }

            // the child span lies within the pool.
            let child_start = node.children.start.0;
            let child_end = child_start.saturating_add(node.children.len.0);
            if child_end > pool_len {
                return Outcome::Err(DriverError::CorruptValue);
            }

            // every child index is strictly below this node's index: the
            // monotone check the children-first order makes an acyclicity proof.
            let mut k = child_start;
            while k < child_end {
                if a.pool[k].index().0 >= i.0 {
                    return Outcome::Err(DriverError::CorruptValue);
                }
                k += 1;
            }

            i = USize(i.0 + 1);
        }
        Outcome::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo::Identity;
    use vehje_runtime_abi::{
        BlobSpan, Region, RegionId, ValueList, ValueNode, ValueRef, ValueTag,
    };

    fn node(tag: ValueTag, children: ValueList) -> ValueNode {
        ValueNode { tag, region: RegionId(USize::ZERO), children, blob: BlobSpan::EMPTY }
    }

    #[test]
    fn validates_a_well_formed_arena() {
        // outcome(seq[unit]) children-first: unit(0), seq(1)->child 0, outcome(2)->child 1.
        let nodes = [
            node(ValueTag::Unit, ValueList::EMPTY),
            node(ValueTag::Seq, ValueList { start: USize::ZERO, len: USize::ONE }),
            node(ValueTag::Outcome, ValueList { start: USize::ONE, len: USize::ONE }),
        ];
        let pool = [ValueRef::new(USize::ZERO), ValueRef::new(USize::ONE)];
        let blob: [u8; 0] = []; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: empty test blob; bytes are the value-arena unit; tracked: #207
        let regions = [Region { start: USize::ZERO, len: USize(3) }];
        let arena = ValueArena::new(&nodes, &pool, &blob, &regions, ValueRef::new(USize(2)));
        let reader = Reader::new(arena);
        assert!(matches!(reader.validate(), Outcome::Ok(())));
    }

    #[test]
    fn rejects_an_out_of_range_region() {
        // one node, but the region table claims a 5-node segment: an out-of-
        // bounds region the decode must reject.
        let nodes = [node(ValueTag::Unit, ValueList::EMPTY)];
        let pool: [ValueRef; 0] = [];
        let blob: [u8; 0] = []; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: empty test blob; bytes are the value-arena unit; tracked: #207
        let regions = [Region { start: USize::ZERO, len: USize(5) }];
        let arena = ValueArena::new(&nodes, &pool, &blob, &regions, ValueRef::new(USize::ZERO));
        let reader = Reader::new(arena);
        assert!(matches!(reader.validate(), Outcome::Err(DriverError::CorruptValue)));
    }

    #[test]
    fn rejects_a_forward_reference() {
        // node 0 points forward at node 1: a cycle-capable forward reference the
        // monotone check must reject.
        let nodes = [
            node(ValueTag::Seq, ValueList { start: USize::ZERO, len: USize::ONE }),
            node(ValueTag::Unit, ValueList::EMPTY),
        ];
        let pool = [ValueRef::new(USize::ONE)];
        let blob: [u8; 0] = []; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: empty test blob; bytes are the value-arena unit; tracked: #207
        let regions = [Region { start: USize::ZERO, len: USize(2) }];
        let arena = ValueArena::new(&nodes, &pool, &blob, &regions, ValueRef::new(USize::ZERO));
        let reader = Reader::new(arena);
        assert!(matches!(reader.validate(), Outcome::Err(DriverError::CorruptValue)));
    }
}
