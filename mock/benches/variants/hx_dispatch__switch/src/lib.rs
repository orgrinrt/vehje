use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_dispatch__switch", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter(){ let op=b&7; acc = match op {0=>acc.wrapping_add(b as u64),1=>acc^(b as u64),2=>acc.wrapping_mul(3).wrapping_add(b as u64),3=>acc.rotate_left(5)^(b as u64),4=>acc.wrapping_sub(b as u64),5=>acc|(b as u64),6=>acc&!(b as u64),_=>acc.rotate_right(3)}; }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
