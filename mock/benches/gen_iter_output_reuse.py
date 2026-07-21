#!/usr/bin/env python3
"""Generate the iter-fusion / interp-output-building / record-update-reuse
harness benches: self-contained plain-std Rust cdylib VARIANT crates under
variants/, plus a bench.toml fragment written to
carrier_fragments/iter_output_reuse.toml (NOT appended to bench.toml).

These three are the audit-fixed replacements for the earlier standalone
Zig benches (iter-fusion/, interp-output-building/, record-update-reuse/)
and for the first-pass hx_iter_fusion / hx_output / hx_reuse harness ports
(which the PORT_NOTES flagged as too weak: no OnceLock-style once-built
state, output not folded from the FFI input so results could hoist, and a
single materialized-vs-fused pair rather than the full depth and push/pull
axes). Every defect the audit named is structurally fixed here:

- state that does not depend on the FFI input is built ONCE and reused; every
  per-call setup that DOES depend on the input (seed records, scratch buffers,
  sharing masks) is built OUTSIDE the timed block, so only the measured work
  is timed.
- the accumulator folds the FFI `input`, so nothing hoists to a constant.
- same-result variants produce byte-identical 8-byte `output`, which the
  harness cross-validates (MAY_DIFFER=false).
- the strongest version of each alternative is measured; the normalise
  baseline is the neutral strategy the design lands on.

Clusters and axes:
  iter-fusion  two depth sections (2-stage, 3-stage); within each,
               materialized vs fused-push vs fused-pull. baseline = fused-pull.
  interp-output  one section; format-to-temp+copy vs format-in-place vs
                 data-driven span-list. baseline = format-in-place.
  record-reuse  three sharing sections (0%, 20%, 60%); within each,
                always-copy vs in-place-when-unique vs always-mutable-ceiling.
                baseline = in-place-when-unique. copy-elision defeated with
                core::hint::black_box so the copy is real. copy and mutable
                are input-and-fraction-independent, so one binary of each is
                shared across the three sections.

Run: python3 gen_iter_output_reuse.py && for v in <printed list>; do
  (cd variants/$v && cargo build --release); done
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SIZES = [64, 256, 1024, 4096, 16384]
SIZES_RS = "[64, 256, 1024, 4096, 16384]"

CARGO = """\
[workspace]
[package]
name = "{name}"
version = "0.0.0"
edition = "2021"
publish = false
[lib]
name = "{name}"
path = "src/lib.rs"
crate-type = ["cdylib"]
[dependencies]
mockspace-bench-core = {{ git = "https://github.com/hiisi-digital/mockspace", branch = "dev", features = ["std"] }}
mockspace-bench-macro = {{ git = "https://github.com/hiisi-digital/mockspace", branch = "dev" }}
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
"""

# ── iter-fusion ────────────────────────────────────────────────────────────
# One generated program of N input bytes IS the pipeline source. The pipeline
# `source |> filter(even) |> map(*3+1) [|> map(^0x5a5a)] |> sum` runs ITERS
# passes, each pass perturbed by `k` so the whole array is re-walked with a
# different FFI-derived source, giving enough work at small N. The map stages
# are made opaque with black_box so the vectoriser cannot collapse either side
# (the interpreted-stage model): the only difference measured is the fused
# single-pass shape versus the materialized store-then-reload traffic through
# the temp arrays. Temps are allocated OUTSIDE the timed block (untimed), so
# the bench measures the intermediate memory traffic, not the allocation the
# no-alloc runtime forbids outright.
ITER_PRELUDE = """\
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const ITERS: usize = 16;

#[inline(always)]
fn xf(x: u64) -> u64 { core::hint::black_box(x.wrapping_mul(3).wrapping_add(1)) }
#[inline(always)]
fn xf2(x: u64) -> u64 { core::hint::black_box(x ^ 0x5a5a) }
"""

ITER_PUSH2 = ITER_PRELUDE + """
#[bench_variant("iterfuse_push2", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let mut i = 0usize;
            while i < N {
                let x = (input[i] ^ kb) as u64;
                if x & 1 == 0 { acc = acc.wrapping_add(xf(x)); }
                i += 1;
            }
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

ITER_PULL2 = ITER_PRELUDE + """
#[bench_variant("iterfuse_pull2", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let s: u64 = (0 .. N)
                .map(|i| (input[i] ^ kb) as u64)
                .filter(|x| x & 1 == 0)
                .map(xf)
                .fold(0u64, |a, x| a.wrapping_add(x));
            acc = acc.wrapping_add(s);
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

ITER_MAT2 = ITER_PRELUDE + """
#[bench_variant("iterfuse_mat2", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let mut tmp1 = vec![0u64; N];
    let mut tmp2 = vec![0u64; N];
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let mut n1 = 0usize;
            let mut i = 0usize;
            while i < N {
                let x = (input[i] ^ kb) as u64;
                if x & 1 == 0 { tmp1[n1] = x; n1 += 1; }
                i += 1;
            }
            let mut j = 0usize;
            while j < n1 { tmp2[j] = xf(tmp1[j]); j += 1; }
            let mut j = 0usize;
            while j < n1 { acc = acc.wrapping_add(tmp2[j]); j += 1; }
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

ITER_PUSH3 = ITER_PRELUDE + """
#[bench_variant("iterfuse_push3", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let mut i = 0usize;
            while i < N {
                let x = (input[i] ^ kb) as u64;
                if x & 1 == 0 { acc = acc.wrapping_add(xf2(xf(x))); }
                i += 1;
            }
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

ITER_PULL3 = ITER_PRELUDE + """
#[bench_variant("iterfuse_pull3", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let s: u64 = (0 .. N)
                .map(|i| (input[i] ^ kb) as u64)
                .filter(|x| x & 1 == 0)
                .map(xf)
                .map(xf2)
                .fold(0u64, |a, x| a.wrapping_add(x));
            acc = acc.wrapping_add(s);
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

ITER_MAT3 = ITER_PRELUDE + """
#[bench_variant("iterfuse_mat3", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let mut tmp1 = vec![0u64; N];
    let mut tmp2 = vec![0u64; N];
    let mut tmp3 = vec![0u64; N];
    timed! { run {
        let mut acc: u64 = 0;
        let mut k = 0usize;
        while k < ITERS {
            let kb = k as u8;
            let mut n1 = 0usize;
            let mut i = 0usize;
            while i < N {
                let x = (input[i] ^ kb) as u64;
                if x & 1 == 0 { tmp1[n1] = x; n1 += 1; }
                i += 1;
            }
            let mut j = 0usize;
            while j < n1 { tmp2[j] = xf(tmp1[j]); j += 1; }
            let mut j = 0usize;
            while j < n1 { tmp3[j] = xf2(tmp2[j]); j += 1; }
            let mut j = 0usize;
            while j < n1 { acc = acc.wrapping_add(tmp3[j]); j += 1; }
            k += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

# ── interp-output-building ─────────────────────────────────────────────────
# The Interp text-generation hot path: instantiate `"Hi {name}, {count} items
# worth {total}.\\n"` once per input byte, ITERS passes. The template data
# index `i` derives from the FFI input, so the produced text (and therefore
# the folded output hash) varies with input and cannot hoist. All three
# strategies produce byte-identical text for the same `i`, so the folded
# 8-byte output matches and the harness cross-validates on the produced text.
# The reusable 64-byte output buffer lives on the stack, reused per template.
OUT_PRELUDE = """\
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const ITERS: usize = 8;

const NAMES: [&[u8]; 8] = [
    b"alice", b"bob", b"charlie", b"dave", b"eve", b"frank", b"grace", b"heidi",
];
const LIT0: &[u8] = b"Hi ";
const LIT1: &[u8] = b", ";
const LIT2: &[u8] = b" items worth ";
const LIT3: &[u8] = b".\\n";

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
"""

# each strategy exposes `fn emit(out, i) -> usize`; the timed driver is shared.
OUT_DRIVER = """
#[bench_variant("{name}", sizes = SIZES)]
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
"""

OUT_TEMP = OUT_PRELUDE + """
// A: format each value into a scratch buffer, then memcpy it into the output.
#[inline(always)]
fn emit(out: &mut [u8], i: usize) -> usize {
    let (name, count, total) = data(i);
    let mut scratch = [0u8; 24];
    let mut c = 0usize;
    out[c .. c + LIT0.len()].copy_from_slice(LIT0); c += LIT0.len();
    out[c .. c + name.len()].copy_from_slice(name); c += name.len();
    out[c .. c + LIT1.len()].copy_from_slice(LIT1); c += LIT1.len();
    let n = write_int(&mut scratch, count); out[c .. c + n].copy_from_slice(&scratch[.. n]); c += n;
    out[c .. c + LIT2.len()].copy_from_slice(LIT2); c += LIT2.len();
    let n = write_int(&mut scratch, total); out[c .. c + n].copy_from_slice(&scratch[.. n]); c += n;
    out[c .. c + LIT3.len()].copy_from_slice(LIT3); c += LIT3.len();
    c
}
""" + OUT_DRIVER.replace("{name}", "interp_out_temp")

OUT_INPLACE = OUT_PRELUDE + """
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
""" + OUT_DRIVER.replace("{name}", "interp_out_inplace")

OUT_SPAN = OUT_PRELUDE + """
// C: the interpreter-tier lowering. The template is a data-driven step list
// walked at runtime; literals memcpy'd wholesale, values formatted in place.
#[derive(Clone, Copy)]
enum Op { Lit, Sval, Ival0, Ival1 }
const STEPS: [(Op, &[u8]); 7] = [
    (Op::Lit, LIT0), (Op::Sval, b""), (Op::Lit, LIT1), (Op::Ival0, b""),
    (Op::Lit, LIT2), (Op::Ival1, b""), (Op::Lit, LIT3),
];

#[inline(always)]
fn emit(out: &mut [u8], i: usize) -> usize {
    let (name, count, total) = data(i);
    let mut c = 0usize;
    let mut s = 0usize;
    while s < STEPS.len() {
        let (op, span) = STEPS[s];
        match op {
            Op::Lit => { out[c .. c + span.len()].copy_from_slice(span); c += span.len(); }
            Op::Sval => { out[c .. c + name.len()].copy_from_slice(name); c += name.len(); }
            Op::Ival0 => { c += write_int(&mut out[c ..], count); }
            Op::Ival1 => { c += write_int(&mut out[c ..], total); }
        }
        s += 1;
    }
    c
}
""" + OUT_DRIVER.replace("{name}", "interp_out_spanlist")

# ── record-update-reuse ────────────────────────────────────────────────────
# The record-update hot path. Seed records and (for reuse) the sharing mask
# derive from the FFI input and are built OUTSIDE the timed block. Every update
# writes an input-derived value into a fixed field, so the accumulated final
# record is input-dependent and identical across always-copy / reuse /
# always-mutable (the sharing verdict changes only WHEN reuse copies, never the
# values), which is exactly what the harness cross-validates. The full copy is
# kept real with core::hint::black_box on the destination slot, defeating the
# copy-elision LLVM would otherwise apply to the dead intermediate fields.
REUSE_PRELUDE = """\
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
"""

# always-copy: every field-set copies the whole record into a fresh slot first.
REUSE_COPY = REUSE_PRELUDE + """
#[bench_variant("rec_copy", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let seed = build_seed::<N>(input);
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
            let mut u = 0usize;
            while u < UPDATES {
                let nn = top;
                let src = arena[cur];
                arena[nn] = src;
                core::hint::black_box(&arena[nn]);
                arena[nn][u & (FIELDS - 1)] = (u as i64).wrapping_add(base);
                top += 1;
                cur = nn;
                u += 1;
            }
            let mut f = 0usize;
            while f < FIELDS { acc = acc.wrapping_add(arena[cur][f] as u64); f += 1; }
            i += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

# always-mutable ceiling: always in place, ignores sharing (unsafe in general).
REUSE_MUT = REUSE_PRELUDE + """
#[bench_variant("rec_mut", sizes = SIZES)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let seed = build_seed::<N>(input);
    let mut arena = vec![[0i64; FIELDS]; CAP];
    timed! { run {
        let mut acc: u64 = 0;
        let mut top = 0usize;
        let mut i = 0usize;
        while i < N {
            if top + 2 >= CAP { top = 0; }
            let cur = top;
            arena[cur] = seed[i & 1023];
            top += 1;
            let base = seed[i & 1023][0];
            let mut u = 0usize;
            while u < UPDATES {
                arena[cur][u & (FIELDS - 1)] = (u as i64).wrapping_add(base);
                u += 1;
            }
            let mut f = 0usize;
            while f < FIELDS { acc = acc.wrapping_add(arena[cur][f] as u64); f += 1; }
            i += 1;
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
"""

# in-place-when-unique: mutate in place while the emit-time uniqueness bit
# holds; copy once when forced-shared, then the copy is unique again. FRAC is
# the compile-time sharing fraction; only this variant reads the sharing mask.
REUSE_TMPL = REUSE_PRELUDE + """
const FRAC: u64 = {frac};

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

#[bench_variant("{name}", sizes = SIZES)]
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
"""


def reuse_variant(name, frac):
    return REUSE_TMPL.replace("{frac}", str(frac)).replace("{name}", name)


# variant_name -> full lib.rs source (SIZES placeholder substituted below).
VARIANTS = {
    "iterfuse_mat2": ITER_MAT2,
    "iterfuse_push2": ITER_PUSH2,
    "iterfuse_pull2": ITER_PULL2,
    "iterfuse_mat3": ITER_MAT3,
    "iterfuse_push3": ITER_PUSH3,
    "iterfuse_pull3": ITER_PULL3,
    "interp_out_temp": OUT_TEMP,
    "interp_out_inplace": OUT_INPLACE,
    "interp_out_spanlist": OUT_SPAN,
    "rec_copy": REUSE_COPY,
    "rec_mut": REUSE_MUT,
    "rec_reuse_s00": reuse_variant("rec_reuse_s00", 0),
    "rec_reuse_s20": reuse_variant("rec_reuse_s20", 20),
    "rec_reuse_s60": reuse_variant("rec_reuse_s60", 60),
}


def write_variant(name, src):
    d = os.path.join(HERE, "variants", name, "src")
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(HERE, "variants", name, "Cargo.toml"), "w") as f:
        f.write(CARGO.format(name=name))
    with open(os.path.join(d, "lib.rs"), "w") as f:
        f.write(src.replace("sizes = SIZES", "sizes = " + SIZES_RS))


def path(n):
    return f'"variants/{n}/target/release/{n}"'


def section(bench, title, seed, baseline, names):
    paths = ", ".join(path(n) for n in names)
    out = [
        f"[bench.{bench}]",
        f'title = "{title}"',
        'workload = "realistic"',
        f"master_seed = {seed}",
        f"[bench.{bench}.normalise]",
        f'baseline = "{baseline}"',
        'mode = "subtract"',
    ]
    for n in SIZES:
        out.append(f"[[bench.{bench}.sizes]]")
        out.append(f"n = {n}")
        out.append(f"variants = [{paths}]")
    return "\n".join(out) + "\n"


def main():
    for name, src in VARIANTS.items():
        write_variant(name, src)

    sections = [
        section(
            "iter_fusion_d2",
            "Iterator fusion (depth 2): materialized vs fused push vs fused pull",
            "0x5eed_17e5_0f20_0003",
            "iterfuse_pull2",
            ["iterfuse_mat2", "iterfuse_push2", "iterfuse_pull2"],
        ),
        section(
            "iter_fusion_d3",
            "Iterator fusion (depth 3): materialized vs fused push vs fused pull",
            "0x5eed_17e5_0f30_0003",
            "iterfuse_pull3",
            ["iterfuse_mat3", "iterfuse_push3", "iterfuse_pull3"],
        ),
        section(
            "interp_output",
            "Interp output-building: format-to-temp+copy vs in-place vs span-list",
            "0x5eed_00b7_bd10_0004",
            "interp_out_inplace",
            ["interp_out_temp", "interp_out_inplace", "interp_out_spanlist"],
        ),
        section(
            "record_reuse_s00",
            "Record update (0% shared): always-copy vs in-place-when-unique vs mutable ceiling",
            "0x5eed_2ee5_e000_0005",
            "rec_reuse_s00",
            ["rec_copy", "rec_reuse_s00", "rec_mut"],
        ),
        section(
            "record_reuse_s20",
            "Record update (20% shared): always-copy vs in-place-when-unique vs mutable ceiling",
            "0x5eed_2ee5_e020_0005",
            "rec_reuse_s20",
            ["rec_copy", "rec_reuse_s20", "rec_mut"],
        ),
        section(
            "record_reuse_s60",
            "Record update (60% shared): always-copy vs in-place-when-unique vs mutable ceiling",
            "0x5eed_2ee5_e060_0005",
            "rec_reuse_s60",
            ["rec_copy", "rec_reuse_s60", "rec_mut"],
        ),
    ]

    frag_dir = os.path.join(HERE, "carrier_fragments")
    os.makedirs(frag_dir, exist_ok=True)
    header = (
        "# iter-fusion / interp-output-building / record-update-reuse bench\n"
        "# sections. Generated by gen_iter_output_reuse.py. Fragment only: the\n"
        "# main agent composes this into bench.toml; do not edit bench.toml here.\n\n"
    )
    with open(os.path.join(frag_dir, "iter_output_reuse.toml"), "w") as f:
        f.write(header + "\n".join(sections))

    print("generated variants:", ", ".join(VARIANTS.keys()))
    print("wrote carrier_fragments/iter_output_reuse.toml")


if __name__ == "__main__":
    main()
