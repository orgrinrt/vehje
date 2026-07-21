use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
const W: u32 = 24;
#[inline(never)]
fn arm(idx: u32, b: u8, work: u32) -> u64 {
    let mut a: u64 = (0x100000001b3u64).wrapping_mul(idx as u64 + 1) ^ (b as u64);
    let mut c = 0; while c < work { a = a.rotate_left(7).wrapping_mul(0x9e3779b97f4a7c15) ^ (a >> 31); c += 1; }
    a
}
// multiway `jumptable`, arm-cost `h` (W=24), situation `mw8_skew`: 8-way, skewed to arm 0 (~70%)
#[bench_variant("mw_jumptable_h_mw8_skew", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() {
            let k = if b < 179 { 0 } else { 1 + ((b as u32) & 7) % 7 };
            let v;
            v = match k { 0 => arm(0, b, W), 1 => arm(1, b, W), 2 => arm(2, b, W), 3 => arm(3, b, W), 4 => arm(4, b, W), 5 => arm(5, b, W), 6 => arm(6, b, W), _ => arm(7, b, W) };
            acc ^= v;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
