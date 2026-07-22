//! Vertical / SoA data-parallel interpretation: one program over W inputs at
//! once, one dispatch amortised over a SIMD vector of values.
//!
//! The scalar interpreter runs the program once per input, paying one dispatch
//! per node per input. The vertical interpreter lays the results out
//! structure-of-arrays: each node's slot is a `Simd<u64, W>` holding that node's
//! value for W distinct inputs, and one dispatch computes all W lanes with a SIMD
//! op. Dispatch (and decode) is amortised over W inputs, so its per-input share
//! is 1/W of the scalar cost. This maps directly onto vehje's real per-record
//! column evaluation (the same residual run over a column of W records), so it is
//! a first-class beyond-runtime shape, not a footnote, and it is allowed to win.
//!
//! It runs over the flat predecoded form (the honest baseline in-memory shape).
//! Cross-validation: lane l of the vertical results equals the scalar
//! interpreter's results for input seed l, for every node, so the vertical cell
//! computes the identical program on identical inputs and only the evaluation
//! shape differs.

use crate::ir::op;
use crate::predecode::Predecoded;
use core::simd::cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd};
use core::simd::{Select, Simd};

/// Unchecked SoA load of node `idx`'s W-lane vector, matching the scalar
/// interpreters' unchecked operand discipline (program validated once).
#[inline(always)]
unsafe fn vload<const W: usize>(base: *const Simd<u64, W>, idx: u32) -> Simd<u64, W> {
    *base.add(idx as usize)
}

/// Interpret the flat program over W inputs at once. `seeds` are the W input
/// values (one per lane); `results` is a caller-owned SoA scratch of one
/// `Simd<u64, W>` per node. Fills `results`; the caller reduces it to a checksum.
#[inline]
pub fn interpret_vertical<const W: usize>(
    p: &Predecoded,
    seeds: &[u64; W],
    results: &mut [Simd<u64, W>],
) {
    let seed_v = Simd::from_array(*seeds);
    let zero = Simd::splat(0u64);
    let one = Simd::splat(1u64);
    let shmask = Simd::splat(63u64);
    let wp = results.as_mut_ptr();
    let rp = wp as *const Simd<u64, W>;
    let np = p.nodes.as_ptr();
    for i in 0..p.nodes.len() {
        let nd = unsafe { *np.add(i) };
        let v = match nd.op {
            op::INPUT => seed_v,
            op::CONST => Simd::splat(p.consts[nd.a as usize]),
            op::ADD => unsafe { vload(rp, nd.a) + vload(rp, nd.b) },
            op::SUB => unsafe { vload(rp, nd.a) - vload(rp, nd.b) },
            op::MUL => unsafe { vload(rp, nd.a) * vload(rp, nd.b) },
            op::AND => unsafe { vload(rp, nd.a) & vload(rp, nd.b) },
            op::OR => unsafe { vload(rp, nd.a) | vload(rp, nd.b) },
            op::XOR => unsafe { vload(rp, nd.a) ^ vload(rp, nd.b) },
            // mask the shift amount to 0..63 to match scalar wrapping_shl/shr.
            op::SHL => unsafe { vload(rp, nd.a) << (vload(rp, nd.b) & shmask) },
            op::SHR => unsafe { vload(rp, nd.a) >> (vload(rp, nd.b) & shmask) },
            op::MIN => unsafe { vload(rp, nd.a).simd_min(vload(rp, nd.b)) },
            op::MAX => unsafe { vload(rp, nd.a).simd_max(vload(rp, nd.b)) },
            op::EQ => unsafe { vload(rp, nd.a).simd_eq(vload(rp, nd.b)).select(one, zero) },
            op::LT => unsafe { vload(rp, nd.a).simd_lt(vload(rp, nd.b)).select(one, zero) },
            op::SELECT => unsafe {
                let cond = vload(rp, nd.a).simd_ne(zero);
                cond.select(vload(rp, nd.b), vload(rp, nd.c))
            },
            op::NEG => unsafe { zero - vload(rp, nd.a) },
            op::NOT => unsafe { !vload(rp, nd.a) },
            _ => zero,
        };
        unsafe { *wp.add(i) = v };
    }
}

/// Run the vertical interpreter over W seeds and reduce the SoA results to one
/// scalar checksum (every node, every lane). Exists so the disasm probe can
/// isolate the vertical dispatch loop (which inlines here) without the probe
/// crate having to enable `portable_simd` and name `Simd` itself. The reduction
/// is a separate loop after the dispatch loop, so the packed-NEON dispatch code
/// stays identifiable in the disassembly.
#[inline]
pub fn interpret_vertical_checksum<const W: usize>(p: &Predecoded, seeds: &[u64; W]) -> u64 {
    let mut results = vec![Simd::<u64, W>::splat(0); p.nodes.len()];
    interpret_vertical::<W>(p, seeds, &mut results);
    let mut h = 0u64;
    for v in &results {
        let arr = v.as_array();
        for &lane in arr {
            h = h.rotate_left(7) ^ lane;
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::checksum;
    use crate::ir::{encode, Decoded, REC24};
    use crate::{generate, GenParams};

    fn check_lanes<const W: usize>(node_count: usize) {
        let prog = generate(&GenParams {
            node_count,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let p = crate::predecode::predecode(&d);
        let n = prog.nodes.len();

        // W distinct input seeds.
        let mut seeds = [0u64; W];
        for (l, s) in seeds.iter_mut().enumerate() {
            *s = 0x9e37_79b9_7f4a_7c15u64.wrapping_mul((l as u64) + 1) | 1;
        }

        let mut vres = vec![Simd::<u64, W>::splat(0); n];
        interpret_vertical::<W>(&p, &seeds, &mut vres);

        // each lane must equal the scalar interpreter for that lane's seed.
        let mut sres = vec![0u64; n];
        for (l, &seed) in seeds.iter().enumerate() {
            crate::interp::interpret(&d, seed, &mut sres);
            let sck = checksum(&sres);
            let lane_ck = {
                let mut h = 0u64;
                for v in &vres {
                    h = h.rotate_left(7) ^ v.as_array()[l];
                }
                h
            };
            assert_eq!(sck, lane_ck, "vertical W={W} lane {l} diverged from scalar");
        }
    }

    #[test]
    fn vertical_lanes_match_scalar() {
        check_lanes::<4>(500);
        check_lanes::<8>(500);
        check_lanes::<4>(37);
        check_lanes::<8>(1024);
    }
}
