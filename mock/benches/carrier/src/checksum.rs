//! The checksum contract. Every variant folds its executed results into one
//! `u64` with this function, and the harness cross-validates that variants which
//! should agree produce byte-identical checksums. Semantic validation is the
//! one axis the earlier corpus had; it catches a variant that computes the wrong
//! answer, though not a variant that measures the wrong condition (that is what
//! the opacity boundary and the cost-model sanity line are for).

/// FNV-1a over the little-endian bytes of each folded value. Order-sensitive on
/// purpose: two variants that visit nodes in a different order but compute the
/// same multiset of results are legitimately different executions and should
/// not be forced to match.
#[derive(Clone, Copy, Debug)]
pub struct Checksum(pub u64);

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

impl Checksum {
    #[inline]
    pub fn new() -> Self {
        Checksum(FNV_OFFSET)
    }

    #[inline]
    pub fn fold(&mut self, value: u64) {
        for b in value.to_le_bytes() {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }
}

impl Default for Checksum {
    fn default() -> Self {
        Self::new()
    }
}
