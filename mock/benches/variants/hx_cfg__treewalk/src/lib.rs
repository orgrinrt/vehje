use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_cfg__treewalk", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        fn ev(b:u64,d:u32)->u64 { if d==0 {b} else { ev(b,d-1).wrapping_mul(3)^ev(b.wrapping_add(1),d-1) } } for &b in input.iter(){ acc^=ev(b as u64,3); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
