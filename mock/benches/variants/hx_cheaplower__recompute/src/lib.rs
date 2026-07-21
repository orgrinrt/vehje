use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_cheaplower__recompute", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter(){ let x=(b as u64).wrapping_mul(0x100000001b3)^0x9e3779b9; let y=(b as u64).wrapping_mul(0x100000001b3)^0x9e3779b9; acc^=x.wrapping_add(y); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
