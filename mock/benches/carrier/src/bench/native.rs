//! Near-native tier: interp vs direct instruction-selection vs copy-and-patch
//! stencil, one bench per profile.
//!
//! `interp` is the switch interpreter baseline. `direct` is per-node instruction
//! selection (copypatch.rs `JitCode`); `copypatch` is the copy-and-patch stencil
//! mechanism (stencil.rs `StencilCode`, memcpy + imm12 hole-patch, no per-node
//! isel), the naming per Xu and Kjolstad (OOPSLA 2021). Both JIT cells build
//! their code in setup (not timed) and run it in the timed region; both are
//! `jit`-gated (aarch64 + macos). Full size sweep: the imm12 window cap is lifted
//! (register-offset addressing for indices at or beyond 4096).

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Interp cell state (baseline): decoded program + scratch (bytes leaked to
/// `'static`).
pub struct St {
    pub d: c::ir::Decoded<'static>,
    pub r: Vec<u64>,
}

/// Direct instruction-selection cell state: the compiled `JitCode` (owns its
/// mmap'd machine code) and a scratch.
#[cfg(feature = "jit")]
pub struct StDirect {
    pub jit: c::copypatch::JitCode,
    pub r: Vec<u64>,
}

/// Copy-and-patch stencil cell state: the compiled `StencilCode` and a scratch.
#[cfg(feature = "jit")]
pub struct StStencil {
    pub jit: c::stencil::StencilCode,
    pub r: Vec<u64>,
}

bench_matrix! {
    name: "carrier_native",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "interp",
    regime: warm,

    setup |profile: &str, n: usize| -> St {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let bytes: &'static [u8] = Vec::leak(c::ir::encode(&c::generate(&gp), &c::ir::REC24));
        let d = c::ir::Decoded::parse(bytes, c::ir::REC24).unwrap();
        let r = vec![0u64; d.node_count];
        St { d, r }
    }

    cell interp |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    cell direct
        #[feature = "jit"]
        setup |profile: &str, n: usize| -> StDirect {
            let mut gp = c::GenParams::profile(profile).unwrap();
            gp.node_count = n;
            let prog = c::generate(&gp);
            let jit = c::copypatch::JitCode::new(&prog).expect("jit");
            let r = vec![0u64; prog.nodes.len()];
            StDirect { jit, r }
        }
        |s, seed| { s.jit.run(seed, &mut s.r); c::checksum(&s.r) }
    cell copypatch
        #[feature = "jit"]
        setup |profile: &str, n: usize| -> StStencil {
            let mut gp = c::GenParams::profile(profile).unwrap();
            gp.node_count = n;
            let prog = c::generate(&gp);
            let jit = c::stencil::StencilCode::new(&prog).expect("jit");
            let r = vec![0u64; prog.nodes.len()];
            StStencil { jit, r }
        }
        |s, seed| { s.jit.run(seed, &mut s.r); c::checksum(&s.r) }
}
