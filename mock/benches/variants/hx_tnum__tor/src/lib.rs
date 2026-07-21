use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_tnum__tor", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut v=0u64; let mut m=0u64; for &b in input.iter(){ let bv=b as u64; v|=bv; m=m&!v; acc^=v^m; v=v.wrapping_add(bv); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
