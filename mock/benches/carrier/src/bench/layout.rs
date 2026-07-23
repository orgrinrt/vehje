//! Record layout (REC12..REC32) with fixed switch dispatch, one bench per profile.
//!
//! Only the record stride and the inline-versus-pool operand path differ across
//! cells; the dispatch (switch) and the semantics are held fixed, so the bench
//! isolates record-layout cost. Each cell decodes with its own layout, so each
//! carries its own setup; `rec24` is the baseline and reuses the shared setup.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state: a decoded program (at REC24 for the baseline) and a scratch.
/// `Decoded` borrows its bytes, leaked to `'static` as in the dispatch family.
pub struct St {
    pub d: c::ir::Decoded<'static>,
    pub r: Vec<u64>,
}

fn build(profile: &str, n: usize, layout: c::ir::Layout) -> St {
    let mut gp = c::GenParams::profile(profile).unwrap();
    gp.node_count = n;
    let bytes: &'static [u8] = Vec::leak(c::ir::encode(&c::generate(&gp), &layout));
    let d = c::ir::Decoded::parse(bytes, layout).unwrap();
    let r = vec![0u64; d.node_count];
    St { d, r }
}

bench_matrix! {
    name: "carrier_layout",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "rec24",
    regime: warm,

    setup |profile: &str, n: usize| -> St { build(profile, n, c::ir::REC24) }

    cell rec12 setup |profile: &str, n: usize| -> St { build(profile, n, c::ir::REC12) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell rec16 setup |profile: &str, n: usize| -> St { build(profile, n, c::ir::REC16) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell rec20 setup |profile: &str, n: usize| -> St { build(profile, n, c::ir::REC20) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell rec24 |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell rec32 setup |profile: &str, n: usize| -> St { build(profile, n, c::ir::REC32) }
               |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
}
