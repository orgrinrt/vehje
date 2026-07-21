use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[inline(always)] fn key4(b:u64)->u64 { if b<179 {2} else {[0u64,1,3][(b as usize)%3]} }
#[inline(always)] fn lin(b:u64,i:u64)->u64 { (b.wrapping_mul(0x100000001b3u64.wrapping_mul(i+1)) ^ 0x9e3779b97f4a7c15u64.wrapping_add(i)).rotate_left(((i%13)+1) as u32) }
#[inline(always)] fn nm(b:u64,i:u64)->u64 { match key4(b) { 0=>lin(b,i*4),1=>lin(b,i*4+1),2=>lin(b,i*4+2),_=>lin(b,i*4+3) } }
#[inline(always)] fn ni(b:u64,i:u64)->u64 { let bk=((b>=64)as u64)+((b>=128)as u64)+((b>=192)as u64); match bk {0=>lin(b,i*4),1=>lin(b,i*4+1),2=>lin(b,i*4+2),_=>lin(b,i*4+3)} }
#[inline(always)] fn blk(b:u64,i:u64)->u64 { lin(b,i) ^ lin(b,i*7+1) ^ lin(b,i*7+2) ^ lin(b,i*7+3) ^ lin(b,i*7+4) }
#[inline(always)] fn program(b:u64)->u64 { 0u64 ^ { let k=key4(b); match k { 0=>lin(b,0), 1=>lin(b,1), 2=>lin(b,2), _=>lin(b,3) } } ^ { let k=key4(b); match k { 0=>nm(b,0), 1=>nm(b,1), 2=>nm(b,2), _=>nm(b,3) } } ^ { let k=(b & 7); match k { 0=>lin(b,0), 1=>lin(b,1), 2=>lin(b,2), 3=>lin(b,3), 4=>lin(b,4), 5=>lin(b,5), 6=>lin(b,6), _=>lin(b,7) } } ^ { let k=key4(b); match k { 0=>blk(b,0), 1=>blk(b,1), 2=>blk(b,2), _=>blk(b,3) } } ^ { if b<50 { lin(b,0) } else if b<120 { lin(b,1) } else if b<200 { lin(b,2) } else { lin(b,3) } } ^ { if b<50 { ni(b,0) } else if b<120 { ni(b,1) } else if b<200 { ni(b,2) } else { ni(b,3) } } ^ { if b<50 { blk(b,0) } else if b<120 { blk(b,1) } else if b<200 { blk(b,2) } else { blk(b,3) } } }
#[bench_variant("an_ifchain_seq", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc:u64=0;
        for &b in input.iter() { acc ^= program(b as u64); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
