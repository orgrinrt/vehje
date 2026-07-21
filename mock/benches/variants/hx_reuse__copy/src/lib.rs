use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_reuse__copy", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut rec=[0u64;32]; for &b in input.iter(){ let mut nr=core::hint::black_box(rec); let s=(b&31) as usize; nr[s]=nr[s].wrapping_add(b as u64).wrapping_mul(3); rec=nr; acc^=rec[s]; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
