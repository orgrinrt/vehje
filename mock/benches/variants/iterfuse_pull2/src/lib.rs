use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const ITERS: usize = 16;

#[inline(always)]
fn xf(x: u64) -> u64 { core::hint::black_box(x.wrapping_mul(3).wrapping_add(1)) }
#[inline(always)]
fn xf2(x: u64) -> u64 { core::hint::black_box(x ^ 0x5a5a) }

#[bench_variant("iterfuse_pull2", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let s: u64 = (0 .. N)
                .map(|i| (input[i] ^ kb) as u64)
                .filter(|x| x & 1 == 0)
                .map(xf)
                .fold(0u64, |a, x| a.wrapping_add(x));
            acc = acc.wrapping_add(s);
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
