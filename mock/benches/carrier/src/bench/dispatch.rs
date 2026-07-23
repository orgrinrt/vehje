//! Dispatch shape over the wire `Decoded` form, one bench per profile.
//!
//! The switch interpreter is the baseline; every other shape (function-pointer
//! table, frequency- and ascending- and linear- if-chains, bit-tree) folds the
//! identical checksum, so the bench isolates dispatch cost. `nullfloor` is the
//! per-form null-dispatch floor; `threaded` is the token-threaded interpreter
//! (nightly `explicit_tail_calls` + preserve-none, behind the `threaded` feature).

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state: the decoded wire program and a results scratch. `Decoded`
/// borrows its bytes, so setup leaks them to `'static` (a bounded per-subprocess
/// leak reclaimed at exit, the standard bench idiom for a borrowing decoded
/// form; the old string matrix did the same via `OnceLock`'s static storage).
pub struct St {
    pub d: c::ir::Decoded<'static>,
    pub r: Vec<u64>,
}

bench_matrix! {
    name: "carrier_dispatch",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "switch",
    floor: "nullfloor",
    regime: warm,

    setup |profile: &str, n: usize| -> St {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let bytes: &'static [u8] = Vec::leak(c::ir::encode(&c::generate(&gp), &c::ir::REC24));
        let d = c::ir::Decoded::parse(bytes, c::ir::REC24).unwrap();
        let r = vec![0u64; d.node_count];
        St { d, r }
    }

    // every cell folds `checksum(&r)` as its single keep-alive; the scaffold
    // folds one per iteration, so the fidelity fold is symmetric by construction.
    cell switch     |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell fntable    |s, seed| { c::interpret_fntable(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell ifchain    |s, seed| { c::interpret_ifchain(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell ifchainasc |s, seed| { c::interpret_ifchain_ascending(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell ifchainlin |s, seed| { c::interpret_ifchain_linear(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell bittree    |s, seed| { c::interpret_bittree(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell threaded   #[feature = "threaded"]
                    |s, seed| { c::interpret_threaded(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell nullfloor  |s, seed| { c::interpret_nulldispatch(&s.d, seed, &mut s.r); c::checksum(&s.r) }
}
