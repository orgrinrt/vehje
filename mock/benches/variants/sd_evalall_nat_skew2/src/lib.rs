use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
const W: u32 = 1;
#[inline(never)]
fn arm(idx: u32, b: u8, w: u32) -> u64 {
    let mut a: u64 = (0x100000001b3u64).wrapping_mul(idx as u64 + 1) ^ (b as u64);
    let mut c = 0; while c < w { a = a.rotate_left(7).wrapping_mul(0x9e3779b97f4a7c15) ^ (a >> 31); c += 1; }
    a
}
// SHOWDOWN native tier: strategy `evalall`, scenario `skew2`
#[bench_variant("sd_evalall_nat_skew2", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() {
            let k: u32 = if b < 179 { 2 } else { [0u32,1,3][(b as usize) % 3] };
            let idx: u32 = k;
            let mut arr = [0u64; 4]; let mut j = 0u32; while j < 4 { arr[j as usize] = arm(j, b, W); j += 1; } let v = arr[k as usize];
            acc ^= v;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
