use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_resolve__flat", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut tab=[0xffffffffu32;256]; let mut d=0; while d<12 { let mut i=0; while i<8 { let nm=(d*8+i) as u32; let mut h=(nm.wrapping_mul(2654435761)&255) as usize; while tab[h]!=0xffffffff {h=(h+1)&255;} tab[h]=nm; i+=1; } d+=1; } for &b in input.iter(){ let nm=(b as u32)%96; let mut h=(nm.wrapping_mul(2654435761)&255) as usize; let mut found=0u32; while tab[h]!=0xffffffff { if tab[h]==nm {found=nm; break;} h=(h+1)&255; } acc^=found as u64; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
