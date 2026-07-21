use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const W: u32 = 24;

#[inline(never)]
fn arm_taken(b: u8, work: u32) -> u64 {
    let mut a: u64 = 0xcbf29ce484222325 ^ (b as u64);
    let mut k = 0; while k < work { a = a.wrapping_mul(0x100000001b3) ^ (a >> 29); k += 1; }
    a
}
#[inline(never)]
fn arm_nottaken(b: u8, work: u32) -> u64 {
    let mut a: u64 = 0x9e3779b97f4a7c15u64.wrapping_add(b as u64);
    let mut k = 0; while k < work { a = a.rotate_left(13).wrapping_add(0x100000001b3); k += 1; }
    a
}

// strategy `profiled_hot`, arm-cost `h` (W=24), situation `pred05`: ~5% taken, predictable (b<13)
#[bench_variant("br_profiled_hot_h_pred05", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0;
            for &b in input.iter() {
                let cond = b < 13;
                let mut v = arm_nottaken(b, W); if cond { v = arm_taken(b, W); }
                acc ^= v;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
