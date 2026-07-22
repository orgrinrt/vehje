//! Cross-language cross-validation: the Zig carrier interpreter against the Rust
//! one, byte-exact, on identical REC24 wire bytes.
//!
//! The Zig cell is the one dispatch shape a Rust-only carrier cannot express (a
//! guaranteed-tail-call token-threaded loop) and the shipped vehje runtime is
//! Zig, so its correctness against the Rust reference is load-bearing. This test
//! generates a carrier program, pipes `[seed][program bytes]` to the `zigcheck`
//! CLI (which runs both Zig dispatch shapes, asserts they agree, and emits the
//! checksum), and compares that checksum to the Rust interpreter's. So one run
//! validates three things: Rust-vs-Zig, and Zig-switch-vs-Zig-tail.
//!
//! If `zigcheck` is not built the test skips (it is an optional cross-language
//! artifact; `mock/benches/carrier-zig/build.sh` builds it). It never fails for
//! a missing toolchain, only for an actual checksum divergence.

use std::io::Write;
use std::process::{Command, Stdio};

use vehje_bench_carrier::access::checksum;
use vehje_bench_carrier::ir::{encode, Decoded, REC24};
use vehje_bench_carrier::{generate, interpret, GenParams};

#[test]
fn zig_matches_rust_byte_exact() {
    let zigcheck = concat!(env!("CARGO_MANIFEST_DIR"), "/../carrier-zig/zigcheck");
    if !std::path::Path::new(zigcheck).exists() {
        eprintln!("skip zig_matches_rust_byte_exact: zigcheck not built (run mock/benches/carrier-zig/build.sh)");
        return;
    }

    for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
        let mut gp = GenParams::profile(name).unwrap();
        gp.node_count = 800;
        let prog = generate(&gp);
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r = vec![0u64; prog.nodes.len()];

        for seed in [0u64, 1, 42, 12345, 999_999] {
            interpret(&d, seed, &mut r);
            let rust_cs = checksum(&r);

            let mut child = Command::new(zigcheck)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("spawn zigcheck");
            {
                let mut si = child.stdin.take().unwrap();
                si.write_all(&seed.to_le_bytes()).unwrap();
                si.write_all(&bytes).unwrap();
            }
            let out = child.wait_with_output().unwrap();
            assert!(
                out.status.success(),
                "{name} seed {seed}: zigcheck failed (Zig switch/tail mismatch or error): {}",
                String::from_utf8_lossy(&out.stderr)
            );
            assert_eq!(out.stdout.len(), 8, "{name} seed {seed}: zigcheck did not emit an 8-byte checksum");
            let zig_cs = u64::from_le_bytes(out.stdout[..8].try_into().unwrap());
            assert_eq!(
                rust_cs, zig_cs,
                "{name} seed {seed}: Zig checksum {zig_cs:#x} diverged from Rust {rust_cs:#x}"
            );
        }
    }
}
