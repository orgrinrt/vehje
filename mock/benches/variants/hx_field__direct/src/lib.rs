use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_field__direct", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let names=[100u32,101,102,103,104,105,106,107]; let vals=[10u64,11,12,13,14,15,16,17]; let _=names; for &b in input.iter(){ let off=(b&7) as usize; acc^=vals[off]; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
