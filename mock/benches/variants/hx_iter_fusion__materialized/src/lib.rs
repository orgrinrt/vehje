use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_iter_fusion__materialized", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut t1=[0u8;16384]; let mut n=0usize; for &b in input.iter(){ if (b&1)==0 { t1[n]=b; n+=1; } } let mut t2=[0u64;16384]; for i in 0..n { t2[i]=(t1[i] as u64).wrapping_mul(3).wrapping_add(1); } for i in 0..n { acc^=t2[i]; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
