use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_interner__fnv", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut buf=[0u8;8]; for &b in input.iter(){ let n=3+(b as usize%5); let mut k=0usize; let mut x=b as u64; while k<n {buf[k]=(b'a'+((x>>4)%26) as u8); x=x.wrapping_mul(6364136223846793005)+1; k+=1;} let mut h=0xcbf29ce484222325u64; for i in 0..n { h^=buf[i] as u64; h=h.wrapping_mul(0x100000001b3); } acc^=h; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
