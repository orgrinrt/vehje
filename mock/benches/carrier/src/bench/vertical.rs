//! Vertical/SoA SIMD, one bench per profile.
//!
//! Every cell processes EXACTLY 8 inputs per timed call (scalar: 8 scalar passes;
//! vert4: two W=4 passes; vert8: one W=8 pass), so the raw per-call times are
//! directly comparable with no per-input divisor. The SoA scratch is pre-allocated
//! in setup and reused, so no allocation is charged to the timed region; the
//! scalar baseline reuses its buffer identically, and both run over a predecoded
//! form, so the only measured difference is scalar-lanes vs SIMD-lanes. The vert
//! cells need the `vertical` feature (portable_simd).

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Scalar baseline state: the predecoded program and a scratch.
pub struct St {
    pub pd: c::predecode::Predecoded,
    pub r: Vec<u64>,
}

/// W=4 vertical state: the predecoded program and its pre-allocated SoA scratch.
#[cfg(feature = "vertical")]
pub struct StV4 {
    pub pd: c::predecode::Predecoded,
    pub soa: Vec<core::simd::Simd<u64, 4>>,
}

/// W=8 vertical state.
#[cfg(feature = "vertical")]
pub struct StV8 {
    pub pd: c::predecode::Predecoded,
    pub soa: Vec<core::simd::Simd<u64, 8>>,
}

#[cfg(feature = "vertical")]
fn predecode_for(profile: &str, n: usize) -> c::predecode::Predecoded {
    let mut gp = c::GenParams::profile(profile).unwrap();
    gp.node_count = n;
    let bytes = c::ir::encode(&c::generate(&gp), &c::ir::REC24);
    let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
    c::predecode::predecode(&d)
}

bench_matrix! {
    name: "carrier_vertical",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "scalar",
    regime: warm,

    setup |profile: &str, n: usize| -> St {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let bytes = c::ir::encode(&c::generate(&gp), &c::ir::REC24);
        let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
        let pd = c::predecode::predecode(&d);
        let r = vec![0u64; pd.nodes.len()];
        St { pd, r }
    }

    // 8 predecoded scalar interprets per call, reusing `r`.
    cell scalar |s, seed| {
        let mut acc = 0u64;
        for j in 0..8u64 {
            let sd = seed ^ j.wrapping_mul(0x9e37_79b9);
            c::predecode::interpret_predecoded(&s.pd, sd, &mut s.r);
            acc ^= c::checksum(&s.r);
        }
        acc
    }
    cell vert4
        #[feature = "vertical"]
        setup |profile: &str, n: usize| -> StV4 {
            let pd = predecode_for(profile, n);
            let soa = c::vertical::make_scratch::<4>(pd.nodes.len());
            StV4 { pd, soa }
        }
        |s, seed| {
            let mut acc = 0u64;
            for pass in 0..2u64 {
                let mut inputs = [0u64; 4];
                for (l, x) in inputs.iter_mut().enumerate() {
                    *x = seed ^ (pass * 4 + l as u64).wrapping_mul(0x9e37_79b9);
                }
                acc ^= c::vertical::interpret_vertical_checksum_into::<4>(&s.pd, &inputs, &mut s.soa);
            }
            acc
        }
    cell vert8
        #[feature = "vertical"]
        setup |profile: &str, n: usize| -> StV8 {
            let pd = predecode_for(profile, n);
            let soa = c::vertical::make_scratch::<8>(pd.nodes.len());
            StV8 { pd, soa }
        }
        |s, seed| {
            let mut inputs = [0u64; 8];
            for (l, x) in inputs.iter_mut().enumerate() {
                *x = seed ^ (l as u64).wrapping_mul(0x9e37_79b9);
            }
            c::vertical::interpret_vertical_checksum_into::<8>(&s.pd, &inputs, &mut s.soa)
        }
}
