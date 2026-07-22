use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const ITERS: usize = 16;

#[inline(always)]
fn xf(x: u64) -> u64 { core::hint::black_box(x.wrapping_mul(3).wrapping_add(1)) }
#[inline(always)]
fn xf2(x: u64) -> u64 { core::hint::black_box(x ^ 0x5a5a) }

#[bench_variant("iterfuse_mat2", sizes = [64, 256, 1024, 2048, 3072, 4096, 6144, 8192, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let mut tmp1 = vec![0u64; N];
    let mut tmp2 = vec![0u64; N];
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let mut n1 = 0usize;
            let mut i = 0usize;
            while i < N {
                let x = (input[i] ^ kb) as u64;
                if x & 1 == 0 { tmp1[n1] = x; n1 += 1; }
                i += 1;
            }
            let mut j = 0usize;
            while j < n1 { tmp2[j] = xf(tmp1[j]); j += 1; }
            let mut j = 0usize;
            while j < n1 { acc = acc.wrapping_add(tmp2[j]); j += 1; }
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
