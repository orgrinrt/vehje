//! Native ceiling: interpreter vs shape-specialized native madd loop, one bench.
//!
//! Uses the macro's `stream` regime: unlike every other family (one program
//! execution per iteration, O(N) work), each cell sweeps the whole harness input
//! byte stream against a size-N madd program, so its work is O(N) executions x
//! O(N) program = O(N^2), a THROUGHPUT-over-a-byte-stream measurement, not
//! per-execution latency. Its raw ns are NOT on the sibling families' scale and
//! must never be normalized against them. Sizes stop at 1024 (matching the JIT
//! tier): N=4096 is 16x the node-step work of N=1024 and, with calibrated
//! repeats, exceeds even the 600s driver window for no additional insight.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state: the decoded madd program and a scratch (bytes leaked to
/// `'static`).
pub struct St {
    pub d: c::ir::Decoded<'static>,
    pub r: Vec<u64>,
}

bench_matrix! {
    name: "carrier_native_ceiling",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep kind in [],
    sizes: [64, 256, 1024],
    baseline: "interp",
    regime: stream,

    setup |_profile: &str, n: usize| -> St {
        let bytes: &'static [u8] = Vec::leak(c::madd_bytes(n / 4, c::ir::REC24));
        let d = c::ir::Decoded::parse(bytes, c::ir::REC24).unwrap();
        let r = vec![0u64; d.node_count];
        St { d, r }
    }

    cell interp |s, input| {
        let mut acc = 0u64;
        for &b in input.iter() {
            c::interpret(&s.d, b as u64, &mut s.r);
            acc ^= c::checksum(&s.r);
        }
        acc
    }
    cell native |s, input| { c::native_madd_over_input(&s.d, input, &mut s.r) }
}
