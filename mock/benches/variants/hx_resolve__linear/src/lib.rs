use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_resolve__linear", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let scopes:[[u32;8];12]={let mut s=[[0u32;8];12]; let mut d=0; while d<12 {let mut i=0; while i<8 {s[d][i]=(d*8+i) as u32; i+=1;} d+=1;} s}; for &b in input.iter(){ let nm=(b as u32)%96; let mut d=11i32; let mut found=0u32; 'o: while d>=0 { let mut i=0; while i<8 { if scopes[d as usize][i]==nm { found=((11-d) as u32)<<8|i as u32; break 'o; } i+=1; } d-=1; } acc^=found as u64; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
