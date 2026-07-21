use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_output__inplace", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut out=[0u8;64]; for &b in input.iter(){ let mut c=0usize; let lit=b"val="; out[c..c+4].copy_from_slice(lit); c+=4; let mut v=b as u64; let start=c; if v==0 {out[c]=b'0'; c+=1;} else { let mut tmp=[0u8;8]; let mut n=0; while v>0 {tmp[n]=b'0'+(v%10) as u8; v/=10; n+=1;} for k in 0..n {out[c+k]=tmp[n-1-k];} c+=n; } let _=start; for k in 0..c { acc=acc.wrapping_add(out[k] as u64); } }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
