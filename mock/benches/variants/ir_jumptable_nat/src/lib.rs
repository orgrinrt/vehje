use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)]
fn key_fn(b: u8) -> u32 { if b < 179 { 2 } else { [0u32,1,3][(b as usize) % 3] } }
#[inline(always)]
fn arm(i: u32, b: u8) -> u64 {
    const PS: [u64;4]=[0x100000001b3,0x9e3779b97f4a7c15,0xc2b2ae3d27d4eb4f,0x165667b19e3779f9];
    const QS: [u64;4]=[0x27d4eb2f165667c5,0x85ebca77c2b2ae63,0xff51afd7ed558ccd,0xc4ceb9fe1a85ec53];
    ((b as u64).wrapping_mul(PS[i as usize]) ^ QS[i as usize]).rotate_left(i+1)
}
// SHOWDOWN-IR native tier (IR lowered; reference ceiling): strategy `jumptable`
#[bench_variant("ir_jumptable_nat", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() { let k = key_fn(b); acc ^= { arm(k,b) }; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
