use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
const W: u32 = 1;
#[inline(never)]
fn arm(idx: u32, b: u8, work: u32) -> u64 {
    let mut a: u64 = (0x100000001b3u64).wrapping_mul(idx as u64 + 1) ^ (b as u64);
    let mut c = 0; while c < work { a = a.rotate_left(7).wrapping_mul(0x9e3779b97f4a7c15) ^ (a >> 31); c += 1; }
    a
}
// multiway `chain_rev`, arm-cost `c` (W=1), situation `mw8_skew`: 8-way, skewed to arm 0 (~70%)
#[bench_variant("mw_chain_rev_c_mw8_skew", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() {
            let k = if b < 179 { 0 } else { 1 + ((b as u32) & 7) % 7 };
            let v;
            if k == 7 { v = arm(7, b, W);
                } else if k == 6 { v = arm(6, b, W);
                } else if k == 5 { v = arm(5, b, W);
                } else if k == 4 { v = arm(4, b, W);
                } else if k == 3 { v = arm(3, b, W);
                } else if k == 2 { v = arm(2, b, W);
                } else if k == 1 { v = arm(1, b, W);
                } else { v = arm(0, b, W); }
            acc ^= v;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
