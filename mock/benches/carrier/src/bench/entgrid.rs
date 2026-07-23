//! Entropy x locality surface, one bench.
//!
//! A 3x3 swept surface holding the op distribution uniform and the dispatch fixed
//! (predecoded switch) so the ONLY variables are op-stream entropy (op_correlation:
//! 0 iid / 500 mid / 900 highly correlated, per-mille chance to repeat the prior
//! op, so LOW correlation is HIGH entropy) and operand locality (locality_window:
//! 4 tight / 64 default / ~unbounded). Measures the surface directly so a consumer
//! can locate its residuals on it, instead of "entropy predicts per-node cost"
//! staying an inference. `c0_w64` is the baseline and reuses the shared setup;
//! the other eight cells carry their own gen params.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Cell state: the predecoded program (at that cell's entropy/locality point) and
/// a scratch.
pub struct St {
    pub pd: c::predecode::Predecoded,
    pub r: Vec<u64>,
}

fn build(n: usize, corr: u32, win: usize) -> St {
    let mut gp = c::GenParams::default_point();
    gp.node_count = n;
    gp.op_correlation = corr;
    gp.locality_window = win;
    let bytes = c::ir::encode(&c::generate(&gp), &c::ir::REC24);
    let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
    let pd = c::predecode::predecode(&d);
    let r = vec![0u64; pd.nodes.len()];
    St { pd, r }
}

bench_matrix! {
    name: "carrier_entgrid",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep kind in [],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "c0_w64",
    regime: warm,

    setup |_profile: &str, n: usize| -> St { build(n, 0, 64) }

    cell c0_w4     setup |_profile: &str, n: usize| -> St { build(n, 0, 4) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c0_w64    |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c0_wmax   setup |_profile: &str, n: usize| -> St { build(n, 0, 1usize << 20) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c500_w4   setup |_profile: &str, n: usize| -> St { build(n, 500, 4) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c500_w64  setup |_profile: &str, n: usize| -> St { build(n, 500, 64) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c500_wmax setup |_profile: &str, n: usize| -> St { build(n, 500, 1usize << 20) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c900_w4   setup |_profile: &str, n: usize| -> St { build(n, 900, 4) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c900_w64  setup |_profile: &str, n: usize| -> St { build(n, 900, 64) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell c900_wmax setup |_profile: &str, n: usize| -> St { build(n, 900, 1usize << 20) }
                   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
}
