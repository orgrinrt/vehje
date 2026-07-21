use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_output__temp", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut out=[0u8;64]; for &b in input.iter(){ let mut c=0usize; let lit=b"val="; out[c..c+4].copy_from_slice(lit); c+=4; let mut scratch=[0u8;8]; let mut v=b as u64; let mut n=0; if v==0 {scratch[0]=b'0'; n=1;} else { while v>0 {scratch[n]=b'0'+(v%10) as u8; v/=10; n+=1;} } for k in 0..n { out[c+k]=scratch[n-1-k]; } c+=n; for k in 0..c { acc=acc.wrapping_add(out[k] as u64); } }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
