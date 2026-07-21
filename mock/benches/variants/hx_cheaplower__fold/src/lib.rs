use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_cheaplower__fold", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut seen=[0u64;64]; for &b in input.iter(){ let key=(b as u64).wrapping_mul(0x100000001b3)^0x9e3779b9; let h=(key&63) as usize; let x=if seen[h]!=0 {seen[h]} else {seen[h]=key; key}; acc^=x.wrapping_add(x); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
