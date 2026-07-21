use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_reach__whole", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let n=64usize; let mut reach=[0u64;64]; let mut i=0; while i<n { reach[i]=1u64<<i; i+=1; } let mut round=0; while round<3 { let mut i=1; while i<n { let src=input[i%input.len()] as usize % i; reach[i]|=reach[src]; i+=1; } round+=1; } for i in 0..n { acc^=reach[i]; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
