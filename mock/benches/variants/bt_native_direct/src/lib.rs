//! Branch tier: native_direct. The kernel as compiled Rust = the steady-state
//! output copy-and-patch produces (native scalar, no interpreter dispatch).
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[bench_variant("bt_native_direct", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() {
            if (b & 1) == 0 { acc = acc.wrapping_add(b as u64).wrapping_mul(3); }
            else { acc ^= (b as u64) << 1; }
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
