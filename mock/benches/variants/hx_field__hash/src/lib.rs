use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_field__hash", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut keys=[0xffffffffu32;16]; let mut vals=[0u64;16]; let mut i=0; while i<8 { let nm=100u32+i as u32; let mut h=(nm.wrapping_mul(2654435761)&15) as usize; while keys[h]!=0xffffffff {h=(h+1)&15;} keys[h]=nm; vals[h]=10+i as u64; i+=1; } for &b in input.iter(){ let nm=100u32+(b&7) as u32; let mut h=(nm.wrapping_mul(2654435761)&15) as usize; let mut v=0; while keys[h]!=0xffffffff { if keys[h]==nm {v=vals[h]; break;} h=(h+1)&15; } acc^=v; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
