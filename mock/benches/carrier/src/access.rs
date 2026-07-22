//! Shared operand access, checksum, and the null-dispatch floor.
//!
//! Every interpreter, of every dispatch shape, reaches operands and stores
//! results through the SAME primitive here, so the dispatch axis varies dispatch
//! and nothing else. Before this module the threaded interpreters used unchecked
//! raw-pointer access while the switch/fntable/ifchain interpreters used checked
//! slice indexing, which confounded the dispatch measurement with bounds-check
//! elision (the fault two independent audits found in the A1/A3 threaded
//! results). The normalization is unchecked everywhere: the program is validated
//! once at load (`Program::is_well_formed`: children-before-parents, every
//! operand an earlier index), so every operand access is provably in range and
//! the bounds check is pure overhead a real runtime elides post-validation.
//!
//! The checksum moves OUT of the per-node dispatch loop: interpreters fill the
//! results array and nothing else, and the caller folds one checksum over the
//! whole array after the pass. Because `results[i]` is written in node order and
//! the fold runs in index order, the post-pass checksum is byte-identical to the
//! old per-node rolling hash, so cross-validation values are unchanged while the
//! dispatch inner loop is clean. The fold is identical across every variant, so
//! it dilutes uniformly and is removed by the null-dispatch floor differencing.

/// Unchecked load of `results[idx]`. Sound post-validation (see module docs).
#[inline(always)]
pub unsafe fn rload(base: *const u64, idx: u32) -> u64 {
    *base.add(idx as usize)
}

/// Unchecked store `results[idx] = v`. Sound post-validation (see module docs).
#[inline(always)]
pub unsafe fn rstore(base: *mut u64, idx: usize, v: u64) {
    *base.add(idx) = v;
}

/// Post-pass checksum over the results array: an order-sensitive fold computed
/// once per interpretation pass, outside the dispatch loop. Byte-identical to the
/// old per-node rolling hash (same values, same order), so it is both the
/// cross-validation witness and the anti-DCE keep-alive.
#[inline]
pub fn checksum(results: &[u64]) -> u64 {
    let mut h = 0u64;
    for &v in results {
        h = h.rotate_left(7) ^ v;
    }
    h
}
