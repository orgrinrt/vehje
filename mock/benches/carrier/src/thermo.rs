//! The thermometer-grade encoding and its lattice identity, the algebraic claim
//! the effect benches (`effect_inference`, `effect_inclusion`) rest on and that
//! the audit's C6 wanted gated by an exhaustive check rather than asserted in a
//! topic table.
//!
//! A per-family effect grade `g` in `0..=63` is encoded in thermometer form as
//! `(1 << g) - 1`: grade 0 is `0b0`, grade 1 is `0b1`, grade 2 is `0b11`, and so
//! on, a prefix of set bits. The identity is that this encoding turns the grade
//! lattice into bitwise operations: the join (per-family max) is bitwise OR, and
//! inclusion (`a <= b`) is the bitwise subset test `enc(a) & !enc(b) == 0`. That
//! is why a thermometer-encoded inclusion check is one AND per effect (the
//! `eg_thermo` variant) while a branch-max check is a per-family loop.
//!
//! The `eg_thermo` / `ei_thermo` variants re-implement this inline; they should
//! use these functions (a DRY follow-up). The point here is the executable gate.

/// Thermometer-encode a single-family grade `g` (0..=63) as a prefix of set bits.
#[inline]
pub const fn enc(g: u32) -> u64 {
    if g == 0 {
        0
    } else if g >= 64 {
        u64::MAX
    } else {
        (1u64 << g) - 1
    }
}

/// The grade a thermometer word decodes back to (its population count, since the
/// set bits form a prefix). Inverse of [`enc`] for a single family.
#[inline]
pub const fn dec(w: u64) -> u32 {
    w.count_ones()
}

/// Bitwise join (OR) of two thermometer words. The identity claims this equals
/// the thermometer encoding of the per-family grade max.
#[inline]
pub const fn join(a: u64, b: u64) -> u64 {
    a | b
}

/// Bitwise inclusion test: is `a`'s grade `<=` `b`'s? For thermometer prefixes
/// this is the subset test.
#[inline]
pub const fn included(a: u64, b: u64) -> bool {
    a & !b == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three_lattice_identity() {
        // The 3x3 exhaustive check the audit asked for: over grades {0,1,2}, the
        // thermometer join equals the encoded max and inclusion equals grade <=.
        for a in 0u32..3 {
            for b in 0u32..3 {
                assert_eq!(
                    join(enc(a), enc(b)),
                    enc(a.max(b)),
                    "join(enc({a}),enc({b})) != enc(max)"
                );
                assert_eq!(
                    included(enc(a), enc(b)),
                    a <= b,
                    "inclusion enc({a})<=enc({b}) != ({a}<={b})"
                );
            }
        }
    }

    #[test]
    fn lattice_identity_full_single_family_range() {
        // Generalize past 3x3: the identity holds for every single-family grade
        // pair in 0..=63, so the bench's encoding is sound at any grade count.
        for a in 0u32..=63 {
            for b in 0u32..=63 {
                assert_eq!(join(enc(a), enc(b)), enc(a.max(b)));
                assert_eq!(included(enc(a), enc(b)), a <= b);
                assert_eq!(dec(enc(a)), a); // enc/dec round-trip
            }
        }
    }

    #[test]
    fn multi_family_packing_is_independent() {
        // Grades packed into disjoint 2-bit family lanes (as the effect benches
        // do, 24 families in a u64 with 2 bits each here shown for 3 families)
        // join and include lane-independently: the whole-word OR/subset equals
        // the per-lane result.
        let fams = [(0u32, 2u32), (1, 0), (2, 1)]; // (family, grade)
        let pack = |gs: &[(u32, u32)]| -> u64 {
            let mut w = 0u64;
            for &(f, g) in gs {
                // encode grade g (0..=2, thermometer 2-bit) at lane f*2
                w |= ((1u64 << g) - 1) << (f * 2);
            }
            w
        };
        let a = pack(&fams);
        let b = pack(&[(0, 2), (1, 1), (2, 1)]); // b dominates a in every lane
        assert!(included(a, b), "a should be included in the lane-wise-larger b");
        assert!(!included(b, a), "b exceeds a in some lane");
        assert_eq!(join(a, b), b, "join with a dominator is the dominator");
    }
}
