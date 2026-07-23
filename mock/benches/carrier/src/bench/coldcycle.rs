//! Cold / aliased-predictor dispatch: 16 distinct programs cycled per pass, one
//! bench per profile.
//!
//! Every other family runs ONE program many times under the calibration re-warm,
//! so a small program's whole per-node dispatch-target sequence is memorized by
//! the branch predictor and every dispatch shape looks nearly free (the review
//! panel's central microarchitecture finding). This family uses the `cold_cycle`
//! regime: it cycles 16 DISTINCT programs (same profile+size, different generator
//! seed) round-robin, one per inner iteration, so no single program's dispatch
//! sequence fits the predictor. Compare cell-for-cell against `carrier_predecode`
//! (memorized): the delta is the memorization the warm numbers hide. Sizes stay in
//! the memorized regime [64,256,1024] where the effect lives.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state: 16 distinct predecoded programs and a scratch sized to the
/// largest (all 16 share the size N, so the max is N).
pub struct St {
    pub pds: Vec<c::predecode::Predecoded>,
    pub r: Vec<u64>,
}

bench_matrix! {
    name: "carrier_coldcycle",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024],
    baseline: "switch",
    regime: cold_cycle(16),

    setup |profile: &str, n: usize| -> St {
        let pds: Vec<c::predecode::Predecoded> = (0..16u64)
            .map(|i| {
                let mut gp = c::GenParams::profile(profile).unwrap();
                gp.node_count = n;
                gp.seed ^= i.wrapping_mul(0x9e37_79b9_7f4a_7c15);
                let bytes = c::ir::encode(&c::generate(&gp), &c::ir::REC24);
                let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
                c::predecode::predecode(&d)
            })
            .collect();
        let width = pds.iter().map(|q| q.nodes.len()).max().unwrap_or(1);
        St { pds, r: vec![0u64; width] }
    }

    cell switch  |s, k, seed| { let pd = &s.pds[k % s.pds.len().max(1)]; c::predecode::interpret_predecoded(pd, seed, &mut s.r); c::checksum(&s.r) }
    cell fntable |s, k, seed| { let pd = &s.pds[k % s.pds.len().max(1)]; c::predecode::interpret_predecoded_fntable(pd, seed, &mut s.r); c::checksum(&s.r) }
    cell null    |s, k, seed| { let pd = &s.pds[k % s.pds.len().max(1)]; c::predecode::interpret_predecoded_nulldispatch(pd, seed, &mut s.r); c::checksum(&s.r) }
    cell threaded #[feature = "threaded"]
                 |s, k, seed| { let pd = &s.pds[k % s.pds.len().max(1)]; c::predecode::interpret_predecoded_threaded(pd, seed, &mut s.r); c::checksum(&s.r) }
}
