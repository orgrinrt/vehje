//! Dispatch shape over the predecoded `Predecoded` form, one bench per profile.
//!
//! Same axis as `dispatch`, but the program is predecoded once in setup (owned,
//! no leak) and every cell dispatches over the flat node array. `direct` needs
//! the handlers resolved ahead of the timed region, so it carries its own setup.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state: the predecoded program and a results scratch.
pub struct St {
    pub pd: c::predecode::Predecoded,
    pub r: Vec<u64>,
}

/// The `direct` cell also carries the resolved handler pointers (one per node),
/// built in its per-cell setup so handler resolution is not charged to the
/// timed region. `threaded_direct::H` lives behind the `threaded` feature.
#[cfg(feature = "threaded")]
pub struct StDirect {
    pub pd: c::predecode::Predecoded,
    pub r: Vec<u64>,
    pub hs: Vec<c::predecode::threaded_direct::H>,
}

bench_matrix! {
    name: "carrier_predecode",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "switch",
    floor: "null",
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

    cell switch   |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell fntable  |s, seed| { c::predecode::interpret_predecoded_fntable(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell regcache |s, seed| { c::predecode::interpret_predecoded_regcache(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell null     |s, seed| { c::predecode::interpret_predecoded_nulldispatch(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell threaded #[feature = "threaded"]
                  |s, seed| { c::predecode::interpret_predecoded_threaded(&s.pd, seed, &mut s.r); c::checksum(&s.r) }
    cell direct
        #[feature = "threaded"]
        setup |profile: &str, n: usize| -> StDirect {
            let mut gp = c::GenParams::profile(profile).unwrap();
            gp.node_count = n;
            let bytes = c::ir::encode(&c::generate(&gp), &c::ir::REC24);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let r = vec![0u64; pd.nodes.len()];
            let mut hs: Vec<c::predecode::threaded_direct::H> = Vec::new();
            c::predecode::resolve_handlers(&pd, &mut hs);
            StDirect { pd, r, hs }
        }
        |s, seed| { c::predecode::interpret_predecoded_direct(&s.pd, &s.hs, seed, &mut s.r); c::checksum(&s.r) }
}
