use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }
#[bench_variant("hx_dispatch__fnptr", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        type F=fn(u64,u8)->u64; static T:[F;8]=[|a,b|a.wrapping_add(b as u64),|a,b|a^(b as u64),|a,b|a.wrapping_mul(3).wrapping_add(b as u64),|a,b|a.rotate_left(5)^(b as u64),|a,b|a.wrapping_sub(b as u64),|a,b|a|(b as u64),|a,b|a&!(b as u64),|a,_|a.rotate_right(3)]; for &b in input.iter(){ let f=unsafe{*T.get_unchecked((b&7) as usize)}; acc=f(acc,b); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
