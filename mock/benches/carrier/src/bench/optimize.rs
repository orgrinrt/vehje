//! Optimize stage (none / CSE / fold / DCE / canon / all), one bench per profile.
//!
//! Each cell runs the optimize pass with its own flag set in setup (so the O(N)
//! optimize + encode is not timed), then the timed region is the DOWNSTREAM
//! interpret cost over the resulting (possibly smaller) program. `checksum_at`
//! over the live-outs keeps the changed node count comparable. `none` is the
//! baseline and reuses the shared setup; every other strategy carries its own.
//! eqsat is dropped from the shipped set (the review panel found its marginal
//! contribution zero-or-negative in the pipeline); `canon` is commutative-operand
//! canonicalization + CSE, and `all` is CSE + fold + DCE + canon.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Cell state: the optimized-and-decoded program, its live-out ids, and a scratch.
/// `Decoded` borrows its bytes, leaked to `'static` as elsewhere.
pub struct St {
    pub d: c::ir::Decoded<'static>,
    pub out_ids: Vec<u32>,
    pub r: Vec<u64>,
}

fn build(profile: &str, n: usize, cse: bool, fold: bool, dce: bool, canon: bool) -> St {
    let mut gp = c::GenParams::profile(profile).unwrap();
    gp.node_count = n;
    let prog = c::generate(&gp);
    let opt = c::optimize::optimize(&prog, cse, fold, dce, false, canon);
    let bytes: &'static [u8] = Vec::leak(c::ir::encode(&opt.prog, &c::ir::REC24));
    let d = c::ir::Decoded::parse(bytes, c::ir::REC24).unwrap();
    let r = vec![0u64; d.node_count.max(1)];
    St { d, out_ids: opt.out_ids, r }
}

bench_matrix! {
    name: "carrier_optimize",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "none",
    regime: warm,

    setup |profile: &str, n: usize| -> St { build(profile, n, false, false, false, false) }

    cell none  |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.out_ids) }
    cell cse   setup |profile: &str, n: usize| -> St { build(profile, n, true, false, false, false) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.out_ids) }
    cell fold  setup |profile: &str, n: usize| -> St { build(profile, n, false, true, false, false) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.out_ids) }
    cell dce   setup |profile: &str, n: usize| -> St { build(profile, n, false, false, true, false) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.out_ids) }
    cell canon setup |profile: &str, n: usize| -> St { build(profile, n, true, false, false, true) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.out_ids) }
    cell all   setup |profile: &str, n: usize| -> St { build(profile, n, true, true, true, true) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.out_ids) }
}
