use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_cfg__cfgblock", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter(){ let mut regs=[0u64;8]; regs[0]=b as u64; let mut i=1; while i<8 { regs[i]=regs[i-1].wrapping_mul(3)^(regs[(i>>1)]); i+=1; } acc^=regs[7]; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
