use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
const W: u32 = 1;
#[inline(never)]
fn arm(idx: u32, b: u8, work: u32) -> u64 {
    let mut a: u64 = (0x100000001b3u64).wrapping_mul(idx as u64 + 1) ^ (b as u64);
    let mut c = 0; while c < work { a = a.rotate_left(7).wrapping_mul(0x9e3779b97f4a7c15) ^ (a >> 31); c += 1; }
    a
}
// multiway `jumptable`, arm-cost `c` (W=1), situation `mw3_uni`: 3-way, uniform key
#[bench_variant("mw_jumptable_c_mw3_uni", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() {
            let k = (b as u32) % 3;
            let v;
            v = match k { 0 => arm(0, b, W), 1 => arm(1, b, W), _ => arm(2, b, W) };
            acc ^= v;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
