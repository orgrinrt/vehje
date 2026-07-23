//! Residual encoding: predecoded register/SSA vs stack bytecode, one bench per
//! profile.
//!
//! Isolate ONE axis, register/SSA vs stack-bytecode encoding, on the identical
//! program. Both cells run over a predecoded form and BOTH fold `checksum_at`
//! over the program's live-out sinks (equal element count), so the reported ratio
//! is encoding cost and not a composite of decode form or checksum scope. The
//! register cell is the baseline and uses the shared setup; the stack cell builds
//! a different form (a `StackProgram`), so it carries its own setup.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Register/SSA cell state (baseline): the predecoded program, its live-out sink
/// ids (folded so the compared scope matches the stack cell), and a scratch.
pub struct StReg {
    pub pd: c::predecode::Predecoded,
    pub sink_ids: Vec<u32>,
    pub r: Vec<u64>,
}

/// Stack-bytecode cell state: the compiled stack program plus its operand stack
/// and local slots. The live-outs are `sp.out_locals`.
pub struct StStack {
    pub sp: c::stackbc::StackProgram,
    pub st: Vec<u64>,
    pub lo: Vec<u64>,
}

bench_matrix! {
    name: "carrier_residual",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "register",
    regime: warm,

    setup |profile: &str, n: usize| -> StReg {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let prog = c::generate(&gp);
        let sink_ids = c::optimize::sinks(&prog);
        let bytes = c::ir::encode(&prog, &c::ir::REC24);
        let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
        let pd = c::predecode::predecode(&d);
        let r = vec![0u64; pd.nodes.len()];
        StReg { pd, sink_ids, r }
    }

    cell register |s, seed| {
        c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r);
        c::access::checksum_at(&s.r, &s.sink_ids)
    }
    cell stack
        setup |profile: &str, n: usize| -> StStack {
            let mut gp = c::GenParams::profile(profile).unwrap();
            gp.node_count = n;
            let sp = c::stackbc::compile(&c::generate(&gp));
            let st = vec![0u64; 64];
            let lo = vec![0u64; sp.num_locals];
            StStack { sp, st, lo }
        }
        |s, seed| {
            c::stackbc::interpret_stack(&s.sp, seed, &mut s.st, &mut s.lo);
            c::access::checksum_at(&s.lo, &s.sp.out_locals)
        }
}
