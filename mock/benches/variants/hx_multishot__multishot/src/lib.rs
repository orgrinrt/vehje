use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_multishot__multishot", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter(){ let k=(b&3)+1; let mut s=0u64; let mut r=0; while r<k { s=s.wrapping_add((b as u64).wrapping_mul(3).wrapping_add(r as u64)); r+=1; } acc=acc.wrapping_add(s/(k as u64)); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
