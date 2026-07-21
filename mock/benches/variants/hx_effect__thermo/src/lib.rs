use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_effect__thermo", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut e=0u64; for &b in input.iter(){ let fam=(b as u64)%24; let g:u64=1+((b as u64>>3)&1); let bits=if g==2 {0b11} else {0b01}; e|=bits<<(fam*2); acc=acc.wrapping_add(e&0xff); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
