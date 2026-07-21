use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const FIELDS: usize = 16;
const UPDATES: usize = 8;
const CAP: usize = 4096;
type Rec = [i64; FIELDS];

fn build_seed<const N: usize>(input: &[u8; N]) -> Vec<Rec> {
    let mut s = vec![[0i64; FIELDS]; 1024];
    let mut x = 0x1234_5678u64;
    for (r, rec) in s.iter_mut().enumerate() {
        for f in 0 .. FIELDS {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            let inb = input[(r * FIELDS + f) % N] as u64;
            rec[f] = ((x >> 40) ^ inb) as i64;
        }
    }
    s
}

const FRAC: u64 = 20;

fn build_shared<const N: usize>(input: &[u8; N]) -> Vec<bool> {
    let mut b = vec![false; 1024];
    let mut x = 0x7u64;
    for (j, bj) in b.iter_mut().enumerate() {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        let inb = input[j % N] as u64;
        *bj = ((x >> 40) ^ inb) % 100 < FRAC;
    }
    b
}

#[bench_variant("rec_reuse_s20", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let seed = build_seed::<N>(input);
    let shared = build_shared::<N>(input);
    let mut arena = vec![[0i64; FIELDS]; CAP];
    timed! { run {
        let mut acc: u64 = 0;
        let mut top = 0usize;
        let mut i = 0usize;
        while i < N {
            if top + UPDATES + 2 >= CAP { top = 0; }
            let mut cur = top;
            arena[cur] = seed[i & 1023];
            top += 1;
            let base = seed[i & 1023][0];
            let mut is_unique = true;
            let mut u = 0usize;
            while u < UPDATES {
                let forced_shared = shared[(i * UPDATES + u) & 1023];
                if is_unique && !forced_shared {
                    arena[cur][u & (FIELDS - 1)] = (u as i64).wrapping_add(base);
                } else {
                    let nn = top;
                    let src = arena[cur];
                    arena[nn] = src;
                    core::hint::black_box(&arena[nn]);
                    arena[nn][u & (FIELDS - 1)] = (u as i64).wrapping_add(base);
                    top += 1;
                    cur = nn;
                    is_unique = true;
                }
                u += 1;
            }
            let mut f = 0usize;
            while f < FIELDS { acc = acc.wrapping_add(arena[cur][f] as u64); f += 1; }
            i += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
