use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_closure__linked", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let src=[3u64,5,7,11,13,17,19,23]; let env=&src; for &b in input.iter(){ let idx=(b&7) as usize; acc^=env[idx].wrapping_add(b as u64); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
