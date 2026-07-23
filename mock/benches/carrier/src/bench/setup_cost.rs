//! Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit /
//! copypatch-emit), one bench per profile.
//!
//! The S term of `total(k) = S + k*I`: the ONE-TIME cost to get from raw material
//! to a dispatch-ready form, which every execution family hides in untimed setup.
//! Here the CONSTRUCTION is the timed region and the raw input is built in setup.
//! `parse` (baseline) times a wire decode from bytes; `predecode` times the
//! predecode from an already-parsed program; the rest time a build from an owned
//! `Program`. The two emit cells are the load-bearing pair: `emitdirect` is
//! per-node instruction selection, `emitcopypatch` is the copy-and-patch stencil
//! codegen; both are the pure codegen (a `Vec<u32>` of machine words), the axis
//! Xu and Kjolstad's technique was invented for and the execution matrix cannot
//! see. Both are `jit`-gated.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Baseline (`parse`) state: the raw encoded wire bytes.
pub struct StBytes {
    pub bytes: Vec<u8>,
}

/// `predecode` cell state: an already-parsed program (parse is not timed here;
/// only predecode is). `Decoded` borrows, so bytes are leaked to `'static`.
pub struct StDec {
    pub d: c::ir::Decoded<'static>,
}

/// The build-from-Program cells' state: an owned program.
pub struct StProg {
    pub prog: c::ir::Program,
}

fn program(profile: &str, n: usize) -> c::ir::Program {
    let mut gp = c::GenParams::profile(profile).unwrap();
    gp.node_count = n;
    c::generate(&gp)
}

bench_matrix! {
    name: "carrier_setup",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "parse",
    regime: warm,

    setup |profile: &str, n: usize| -> StBytes {
        StBytes { bytes: c::ir::encode(&program(profile, n), &c::ir::REC24) }
    }

    cell parse |s, _seed| {
        let d = c::ir::Decoded::parse(&s.bytes, c::ir::REC24).unwrap();
        d.node_count as u64
    }
    cell predecode
        setup |profile: &str, n: usize| -> StDec {
            let bytes: &'static [u8] = Vec::leak(c::ir::encode(&program(profile, n), &c::ir::REC24));
            StDec { d: c::ir::Decoded::parse(bytes, c::ir::REC24).unwrap() }
        }
        |s, _seed| { let pd = c::predecode::predecode(&s.d); pd.nodes.len() as u64 }
    cell stackcompile
        setup |profile: &str, n: usize| -> StProg { StProg { prog: program(profile, n) } }
        |s, _seed| { let sp = c::stackbc::compile(&s.prog); sp.num_locals as u64 }
    cell optall
        setup |profile: &str, n: usize| -> StProg { StProg { prog: program(profile, n) } }
        |s, _seed| { let opt = c::optimize::optimize(&s.prog, true, true, true, false, true); opt.prog.nodes.len() as u64 }
    cell emitdirect
        #[feature = "jit"]
        setup |profile: &str, n: usize| -> StProg { StProg { prog: program(profile, n) } }
        |s, _seed| { let code = c::copypatch::emit(&s.prog).expect("emit"); code.len() as u64 }
    cell emitcopypatch
        #[feature = "jit"]
        setup |profile: &str, n: usize| -> StProg { StProg { prog: program(profile, n) } }
        |s, _seed| { let code = c::stencil::emit_stencil(&s.prog).expect("emit"); code.len() as u64 }
}
