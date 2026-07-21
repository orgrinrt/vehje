use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const ITERS: usize = 8;

const NAMES: [&[u8]; 8] = [
    b"alice", b"bob", b"charlie", b"dave", b"eve", b"frank", b"grace", b"heidi",
];
const LIT0: &[u8] = b"Hi ";
const LIT1: &[u8] = b", ";
const LIT2: &[u8] = b" items worth ";
const LIT3: &[u8] = b".\n";

// minimal unsigned-decimal writer at the cursor; returns bytes written.
// identical across the three strategies, so it is not what is being compared.
#[inline(always)]
fn write_int(buf: &mut [u8], v: u64) -> usize {
    if v == 0 { buf[0] = b'0'; return 1; }
    let mut tmp = [0u8; 20];
    let mut n = 0usize;
    let mut x = v;
    while x > 0 { tmp[n] = b'0' + (x % 10) as u8; x /= 10; n += 1; }
    let mut k = 0usize;
    while k < n { buf[k] = tmp[n - 1 - k]; k += 1; }
    n
}

#[inline(always)]
fn data(i: usize) -> (&'static [u8], u64, u64) {
    (NAMES[i & 7], (i & 4095) as u64, (i.wrapping_mul(7) & 65535) as u64)
}

// B: format each value directly at the output cursor, no scratch, no copy.
#[inline(always)]
fn emit(out: &mut [u8], i: usize) -> usize {
    let (name, count, total) = data(i);
    let mut c = 0usize;
    out[c .. c + LIT0.len()].copy_from_slice(LIT0); c += LIT0.len();
    out[c .. c + name.len()].copy_from_slice(name); c += name.len();
    out[c .. c + LIT1.len()].copy_from_slice(LIT1); c += LIT1.len();
    c += write_int(&mut out[c ..], count);
    out[c .. c + LIT2.len()].copy_from_slice(LIT2); c += LIT2.len();
    c += write_int(&mut out[c ..], total);
    out[c .. c + LIT3.len()].copy_from_slice(LIT3); c += LIT3.len();
    c
}

#[bench_variant("interp_out_inplace", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut outbuf = [0u8; 64];
        let mut k = 0usize;
        while k < ITERS {
            let mut idx = 0usize;
            while idx < N {
                let i = (input[idx] as usize) ^ (k << 5) ^ (idx << 3);
                let wrote = emit(&mut outbuf, i);
                let mut b = 0usize;
                while b < wrote {
                    acc = acc.rotate_left(5) ^ (outbuf[b] as u64);
                    b += 1;
                }
                idx += 1;
            }
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
