use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_recwidth__rec24", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut i=0usize; for &b in input.iter(){ let ar=(b&3)+1; let o1=((b as usize*7)&0xff) as u64; let o2=(((b as usize>>1)*7)&0xff) as u64; let o3=(((b as usize>>2)*13)&0xff) as u64; let mut v=o1.wrapping_add(o2); if ar>2 { v=v.wrapping_add(o3); } acc^=v.wrapping_mul(3); i+=1; let _=i; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
