#!/usr/bin/env python3
"""Generate the effect-lattice / tnum / cheap-lowering bench cluster onto the
mockspace bench harness. These replace three standalone hand-timed Zig probes
that the audit flagged: they ran outside the harness (empty or header-only CSVs)
and their results were not reproducible or cross-validated.

The audit-fixed shape here, per bench:

  effect-lattice-inference
    Two measurements, each a bench with a thermometer variant and a naive
    branch-max variant that compute the identical effect lattice:
      effect_inference  the bottom-up OR-join over a node DAG
      effect_inclusion  the whole-program inclusion gate (script <= target)
    Thermometer encodes the per-family grade (none=00, read=01, write=11) so the
    lattice join is a bitwise OR, the graded bind is the same OR, and the
    inclusion check is (script & ~target)==0. The branch-max variant computes the
    same lattice with a per-family max/compare loop over a binary encoding. Both
    decode to the identical canonical grade totals, so outputs are byte-identical
    and the harness cross-validates them; the delta isolates the branch-free win.
    Baseline: thermometer.

  tnum-abstract-arith
    tnum_multiply  tnum-multiply-always-loop vs tnum-multiply-with-known-operand
                   fast path. Both produce identical tnum results (the fast path
                   is exact for both-known operands, and falls to the same loop
                   otherwise), so the harness cross-validates. Baseline: the fast
                   path. Shows the fast path collapsing the multiply-by-constant
                   majority to a concrete multiply.
    tnum_linear    the cheap linear/logical transfers (add/and/or/shl) each vs a
                   concrete-u64 baseline. These are independent algorithms with
                   legitimately different outputs, so may_differ=true. Baseline:
                   concrete. Sizes the pervasive-tracking cost (~1ns regime).

  cheap-lowering-subset
    AUDIT FIX: the benefit of const-fold + CSE is NODE-COUNT reduction, not
    per-op latency (LLVM already CSEs the recompute, so a time bench measures the
    hash-cons overhead as pure cost with no benefit shown). So the measured
    quantity is node count: the routine returns the post-fold+CSE node count in
    the output (high 32 bits; the low 32 carry an input checksum for anti-hoist
    and per-variant determinism). cl_foldcse does fold + hash-cons dedup;
    cl_foldonly does fold only (no dedup, count stays N). The reduction ratio is
    read from the outputs. may_differ=true. Baseline: cl_foldonly. The FINDINGS
    states explicitly this is a node-count metric, not a latency metric.

State is built ONCE per (variant, size) subprocess via OnceLock from a
compile-time seed; the measured work folds the FFI input so nothing hoists; the
variant crates are self-contained plain-std (no carrier IR dep).

Run: python3 gen_effect_tnum_cheaplower.py && for v in <generated>; do \
  (cd variants/$v && cargo build --release); done
The generator writes its bench sections to carrier_fragments/effect_tnum_cheaplower.toml
(it does NOT edit bench.toml).
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SIZES = [64, 256, 1024, 4096, 16384]
SIZES_LIT = "[64, 256, 1024, 4096, 16384]"

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

# ---------------------------------------------------------------------------
# effect lattice: shared DAG prelude
# ---------------------------------------------------------------------------
EFFECT_PRELUDE = """\
const FAMILIES: u64 = 24;
const DAG_SEED: u64 = 0x243f_6a88_85a3_08d3;

struct Node {{ op: u8, a: u32, b: u32, fam: u8, grade: u8 }}

#[inline(always)]
fn splitmix(s: &mut u64) -> u64 {{
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}}

fn gen_dag(n: usize) -> Vec<Node> {{
    let mut s = DAG_SEED ^ (n as u64).wrapping_mul(0x0100_0000_01b3);
    let leaves = (n / 8).max(4).min(n);
    let mut v: Vec<Node> = Vec::with_capacity(n);
    for i in 0..n {{
        if i < leaves {{
            let fam = (splitmix(&mut s) % FAMILIES) as u8;
            let grade = (1 + splitmix(&mut s) % 2) as u8; // 1 (read) or 2 (write)
            v.push(Node {{ op: 0, a: 0, b: 0, fam, grade }});
        }} else {{
            let a = (splitmix(&mut s) % i as u64) as u32;
            let b = (splitmix(&mut s) % i as u64) as u32;
            v.push(Node {{ op: 1, a, b, fam: 0, grade: 0 }});
        }}
    }}
    v
}}

// per-lane max over the 24 two-bit families (naive branch-max join)
#[inline(always)]
fn bmax_join(a: u64, b: u64) -> u64 {{
    let mut r = 0u64;
    let mut f = 0u64;
    while f < FAMILIES {{
        let sh = f * 2;
        let la = (a >> sh) & 3;
        let lb = (b >> sh) & 3;
        let m = if la > lb {{ la }} else {{ lb }};
        r |= m << sh;
        f += 1;
    }}
    r
}}

#[inline(always)]
fn bmax_grade_sum(e: u64) -> u64 {{
    let mut g = 0u64;
    let mut f = 0u64;
    while f < FAMILIES {{
        g += (e >> (f * 2)) & 3;
        f += 1;
    }}
    g
}}
"""

# effect inference variants ---------------------------------------------------
EFFECT_INFER = """\
// AUTO-GENERATED by gen_effect_tnum_cheaplower.py. Effect inference axis:
// bottom-up lattice join over a fixed node DAG, {enc} encoding. The DAG is
// generated once per (variant, size) subprocess from a compile-time seed; the
// timed loop recomputes the join folding an input-derived extra effect into the
// root so the pass depends on the FFI input and does not hoist. The per-node
// canonical grade total (identical across encodings) is folded into the output,
// so thermo and branchmax produce byte-identical output for cross-validation.
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

{prelude}
static DAG: OnceLock<Vec<Node>> = OnceLock::new();
const ITERS: usize = 8;

#[bench_variant("{name}", sizes = {sizes})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let dag = DAG.get_or_init(|| gen_dag(N));
    let mut eff = vec![0u64; N];
    timed! {{ run {{
        let mut acc: u64 = 0;
        for k in 0..ITERS {{
            let ib = input[k] as u64;
            let xf = ib % FAMILIES;
            let xg = 1 + ((ib >> 3) & 1); // 1 or 2
            for i in 0..N {{
                let nd = &dag[i];
                eff[i] = if nd.op == 0 {{
                    let mut e = {leaf};
                    if i == 0 {{ e = {perturb}; }}
                    e
                }} else {{
                    {join}
                }};
            }}
            let mut c = ib;
            for i in 0..N {{ c = c.wrapping_mul(0x0100_0000_01b3) ^ {scalar}; }}
            acc ^= c;
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
"""

EFFECT_INFER_ENC = {
    "thermo": {
        "leaf": "(if nd.grade == 2 { 0b11u64 } else { 0b01u64 }) << (nd.fam as u64 * 2)",
        "perturb": "e | ((if xg == 2 { 0b11u64 } else { 0b01u64 }) << (xf * 2))",
        "join": "eff[nd.a as usize] | eff[nd.b as usize]",
        "scalar": "(eff[i].count_ones() as u64)",
    },
    "branchmax": {
        "leaf": "(nd.grade as u64) << (nd.fam as u64 * 2)",
        "perturb": "bmax_join(e, xg << (xf * 2))",
        "join": "bmax_join(eff[nd.a as usize], eff[nd.b as usize])",
        "scalar": "bmax_grade_sum(eff[i])",
    },
}

# effect inclusion variants ---------------------------------------------------
EFFECT_INCL = """\
// AUTO-GENERATED by gen_effect_tnum_cheaplower.py. Effect inclusion gate:
// scan every node's effect against a target permit-set, counting violations,
// {enc} encoding. Effects are computed once per subprocess (from the fixed DAG)
// into the variant's encoding; the timed loop runs only the inclusion scan, with
// the target's one family perturbed per iteration by the FFI input so the scan
// is not loop-invariant. The violation count (identical across encodings) is
// folded into the output for cross-validation.
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

{prelude}
fn compute_eff(dag: &[Node]) -> Vec<u64> {{
    let mut eff = vec![0u64; dag.len()];
    for i in 0..dag.len() {{
        let nd = &dag[i];
        eff[i] = if nd.op == 0 {{ {leaf} }} else {{ {join} }};
    }}
    eff
}}

static EFF: OnceLock<Vec<u64>> = OnceLock::new();
const ITERS: usize = 16;
const PERMIT: u64 = 20; // base target permits grade 2 on families 0..20, none on 20..24

#[bench_variant("{name}", sizes = {sizes})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let eff = EFF.get_or_init(|| compute_eff(&gen_dag(N)));
    let mut base: u64 = 0;
    {{
        let mut f = 0u64;
        while f < PERMIT {{ base |= {permit2} << (f * 2); f += 1; }}
    }}
    timed! {{ run {{
        let mut acc: u64 = 0;
        for k in 0..ITERS {{
            let ib = input[k] as u64;
            let tf = ib % FAMILIES;
            let tg = (ib >> 4) % 3; // permit grade 0, 1, or 2 for family tf
            let mask = 3u64 << (tf * 2);
            let target = (base & !mask) | ({permitg} << (tf * 2));
            let mut viol: u64 = 0;
            for &e in eff.iter() {{
                if {violates} {{ viol += 1; }}
            }}
            acc = acc.wrapping_mul(0x0100_0000_01b3) ^ viol ^ ib;
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
"""

EFFECT_INCL_ENC = {
    "thermo": {
        "leaf": "(if nd.grade == 2 { 0b11u64 } else { 0b01u64 }) << (nd.fam as u64 * 2)",
        "join": "eff[nd.a as usize] | eff[nd.b as usize]",
        "permit2": "0b11u64",
        # thermo permit bits for grade tg: 0->00, 1->01, 2->11
        "permitg": "((1u64 << tg) - 1)",
        "violates": "(e & !target) != 0",
    },
    "branchmax": {
        "leaf": "(nd.grade as u64) << (nd.fam as u64 * 2)",
        "join": "bmax_join(eff[nd.a as usize], eff[nd.b as usize])",
        "permit2": "0b10u64",
        "permitg": "tg",
        # per-family: violation if any node family grade exceeds target grade
        "violates": "bincl_violates(e, target)",
    },
}

BINCL_HELPER = """\
#[inline(always)]
fn bincl_violates(script: u64, target: u64) -> bool {
    let mut f = 0u64;
    while f < FAMILIES {
        let sh = f * 2;
        if ((script >> sh) & 3) > ((target >> sh) & 3) { return true; }
        f += 1;
    }
    false
}
"""

# ---------------------------------------------------------------------------
# tnum: shared prelude
# ---------------------------------------------------------------------------
TNUM_PRELUDE = """\
const TNUM_SEED: u64 = 0x2717_a5b3_c9d1_e2f4;

#[inline(always)]
fn splitmix(s: &mut u64) -> u64 {{
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}}

// tnum = (value: known bits, mask: unknown bits). Generate a mix where ~80% of
// operands are fully known (mask == 0): the multiply-by-constant / folded case.
fn gen_ops(n: usize) -> Vec<(u64, u64)> {{
    let mut s = TNUM_SEED ^ (n as u64).wrapping_mul(0x0100_0000_01b3);
    let mut v: Vec<(u64, u64)> = Vec::with_capacity(n);
    for _ in 0..n {{
        let r = splitmix(&mut s);
        let mask = if r % 5 == 0 {{ r >> 20 }} else {{ 0 }};
        let value = (splitmix(&mut s) ^ 0x1234_5678_9abc_def0) & !mask;
        v.push((value, mask));
    }}
    v
}}

#[inline(always)]
fn tadd(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {{
    let sm = a.1.wrapping_add(b.1);
    let sv = a.0.wrapping_add(b.0);
    let sigma = sm.wrapping_add(sv);
    let chi = sigma ^ sv;
    let mu = chi | a.1 | b.1;
    (sv & !mu, mu)
}}

#[inline(always)]
fn tshl(a: (u64, u64), sft: u32) -> (u64, u64) {{
    ((a.0 << sft) & !((a.1 << sft)), a.1 << sft)
}}

// general tnum multiply: eBPF long-multiplication over the bits of one operand
#[inline(always)]
fn tmul_loop(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {{
    let mut acc = (0u64, 0u64);
    let mut aa = a;
    let mut bb = b;
    let mut i = 0;
    while i < 64 {{
        if (aa.0 & 1) != 0 {{
            acc = tadd(acc, bb);
        }} else if (aa.1 & 1) != 0 {{
            acc = tadd(acc, (0u64, bb.0 | bb.1));
        }}
        aa = (aa.0 >> 1, aa.1 >> 1);
        bb = tshl(bb, 1);
        i += 1;
    }}
    acc
}}
"""

TNUM_MUL = """\
// AUTO-GENERATED by gen_effect_tnum_cheaplower.py. tnum multiply axis:
// {desc}. Operands (a mix ~80% both-known) are built once per subprocess from a
// compile-time seed; the timed loop multiplies pairs folding the FFI input so
// nothing hoists. The fast path is exact for both-known operands and falls to
// the same loop otherwise, so both variants produce byte-identical output.
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

{prelude}
#[inline(always)]
fn tmul(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {{
    {body}
}}

static OPS: OnceLock<Vec<(u64, u64)>> = OnceLock::new();
const ITERS: usize = 2;

#[bench_variant("{name}", sizes = {sizes})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let ops = OPS.get_or_init(|| gen_ops(N));
    let len = ops.len();
    timed! {{ run {{
        let mut av: u64 = 0;
        let mut am: u64 = 0;
        for k in 0..ITERS {{
            let seed = input[k] as usize;
            let mut acc_v: u64 = 0;
            let mut acc_m: u64 = 0;
            for i in 0..len {{
                let a = ops[(i + seed) % len];
                let b = ops[(i * 2 + seed + 1) % len];
                let r = tmul(a, b);
                acc_v ^= r.0;
                acc_m = acc_m.wrapping_add(r.1);
            }}
            av = av.wrapping_mul(0x0100_0000_01b3) ^ acc_v ^ (input[k] as u64);
            am ^= acc_m;
        }}
        let out = av ^ am;
        output.copy_from_slice(&out.to_le_bytes());
    }} }}
}}
"""

TNUM_MUL_BODY = {
    "loop": "tmul_loop(a, b)",
    "fastpath": "if a.1 == 0 && b.1 == 0 { (a.0.wrapping_mul(b.0), 0) } else { tmul_loop(a, b) }",
}

TNUM_LIN = """\
// AUTO-GENERATED by gen_effect_tnum_cheaplower.py. tnum linear-transfer axis:
// {desc}. Operands are built once per subprocess from a compile-time seed; the
// timed loop applies the transfer over the operand array folding the FFI input.
// These are independent transfer functions with legitimately different results
// (this bench sets may_differ=true); the point is the per-op cost floor.
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

{prelude}
static OPS: OnceLock<Vec<(u64, u64)>> = OnceLock::new();
const ITERS: usize = 8;

#[bench_variant("{name}", sizes = {sizes})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let ops = OPS.get_or_init(|| gen_ops(N));
    let len = ops.len();
    timed! {{ run {{
        let mut acc: u64 = 0;
        for k in 0..ITERS {{
            let seed = input[k] as usize;
            let mut sv: u64 = 0;
            let mut sm: u64 = 0;
            for i in 0..len {{
                let a = ops[(i + seed) % len];
                let b = ops[(i * 3 + seed + 1) % len];
                let r = {body};
                sv ^= r.0;
                sm = sm.wrapping_add(r.1);
            }}
            acc = acc.wrapping_mul(0x0100_0000_01b3) ^ sv ^ sm ^ (input[k] as u64);
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
"""

TNUM_LIN_BODY = {
    # concrete baseline: plain u64 add, no mask tracking
    "concrete": "(a.0.wrapping_add(b.0), 0u64)",
    "add": "tadd(a, b)",
    "and": "{ let alpha = a.0 | a.1; let beta = b.0 | b.1; let v = a.0 & b.0; (v, alpha & beta & !v) }",
    "or": "{ let v = a.0 | b.0; let mu = a.1 | b.1; (v, mu & !v) }",
    "shl": "tshl(a, (i as u32) & 63)",
}

# ---------------------------------------------------------------------------
# cheap lowering: node-count-reduction metric
# ---------------------------------------------------------------------------
CHEAP_PRELUDE = """\
const PROG_SEED: u64 = 0x3c5e_f01d_c5e0_a1b2;

struct PNode {{ op: u8, a: u32, b: u32, val: i64 }}

#[inline(always)]
fn splitmix(s: &mut u64) -> u64 {{
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}}

// redundancy-heavy IR: a small const pool plus repeated add/mul subexpressions
// over a sliding window, so const-fold and CSE have plenty to collapse.
fn gen_prog(n: usize) -> Vec<PNode> {{
    let mut s = PROG_SEED ^ (n as u64).wrapping_mul(0x0100_0000_01b3);
    let mut v: Vec<PNode> = Vec::with_capacity(n);
    for i in 0..n {{
        if i < 8 {{
            v.push(PNode {{ op: 0, a: 0, b: 0, val: i as i64 }});
            continue;
        }}
        let k = splitmix(&mut s) % 3; // 0 const, 1 add, 2 mul
        if k == 0 {{
            let c = (splitmix(&mut s) % 6) as i64; // few distinct consts, CSE-able
            v.push(PNode {{ op: 0, a: 0, b: 0, val: c }});
        }} else {{
            let lo = if i > 32 {{ (i - 32) as u64 }} else {{ 0 }};
            let span = i as u64 - lo;
            let a = (lo + splitmix(&mut s) % span) as u32;
            let b = (lo + splitmix(&mut s) % span) as u32;
            v.push(PNode {{ op: if k == 1 {{ 1 }} else {{ 2 }}, a, b, val: 0 }});
        }}
    }}
    v
}}

#[inline(always)]
fn input_checksum<const N: usize>(input: &[u8; N]) -> u64 {{
    let mut cs = 0u64;
    for i in 0..N {{ cs = cs.wrapping_mul(0x0100_0000_01b3) ^ (input[i] as u64); }}
    cs & 0xffff_ffff
}}
"""

CHEAP_FOLDCSE = """\
// AUTO-GENERATED by gen_effect_tnum_cheaplower.py. Cheap lowering, fold + CSE:
// a single bottom-up no-alloc-shaped pass doing const-fold and common-
// subexpression elimination via an open-addressing hash-cons. The MEASURED
// QUANTITY is the reduced NODE COUNT (returned in output high 32 bits), not
// wall-clock time: the audit found a time bench misleading here because LLVM
// already CSEs the recompute, so timing measures hash-cons overhead as pure
// cost. The output low 32 bits carry an input checksum for anti-hoist and
// per-variant determinism.
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

{prelude}
static PROG: OnceLock<Vec<PNode>> = OnceLock::new();

#[bench_variant("{name}", sizes = {sizes})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let prog = PROG.get_or_init(|| gen_prog(N));
    let n = prog.len();
    let hcap = (4 * n).next_power_of_two().max(16);
    let mut hkey = vec![u64::MAX; hcap];
    let mut hval = vec![0u32; hcap];
    let mut remap = vec![0u32; n];
    let mut o_op = vec![0u8; n];
    let mut o_a = vec![0u32; n];
    let mut o_b = vec![0u32; n];
    let mut o_val = vec![0i64; n];
    let cs = input_checksum(input);
    timed! {{ run {{
        let mut on: u32 = 0;
        for h in hkey.iter_mut() {{ *h = u64::MAX; }}
        for i in 0..n {{
            let nd = &prog[i];
            let (op, a, b, val) = if nd.op != 0 {{
                let ca = remap[nd.a as usize];
                let cb = remap[nd.b as usize];
                if o_op[ca as usize] == 0 && o_op[cb as usize] == 0 {{
                    let va = o_val[ca as usize];
                    let vb = o_val[cb as usize];
                    let v = if nd.op == 1 {{ va.wrapping_add(vb) }} else {{ va.wrapping_mul(vb) }};
                    (0u8, 0u32, 0u32, v)
                }} else {{
                    (nd.op, ca, cb, 0i64)
                }}
            }} else {{
                (0u8, 0u32, 0u32, nd.val)
            }};
            let key = ((op as u64) << 56)
                ^ (val as u64).wrapping_mul(1_099_511_628_211)
                ^ ((a as u64) << 20)
                ^ (b as u64);
            let mut h = (key.wrapping_mul(0x9e37_79b9_7f4a_7c15) as usize) & (hcap - 1);
            loop {{
                if hkey[h] == u64::MAX {{
                    hkey[h] = key;
                    hval[h] = on;
                    o_op[on as usize] = op;
                    o_a[on as usize] = a;
                    o_b[on as usize] = b;
                    o_val[on as usize] = val;
                    remap[i] = on;
                    on += 1;
                    break;
                }} else if hkey[h] == key {{
                    remap[i] = hval[h];
                    break;
                }}
                h = (h + 1) & (hcap - 1);
            }}
        }}
        let out = ((on as u64) << 32) | cs;
        output.copy_from_slice(&out.to_le_bytes());
    }} }}
}}
"""

CHEAP_FOLDONLY = """\
// AUTO-GENERATED by gen_effect_tnum_cheaplower.py. Cheap lowering, fold only:
// the same const-fold pass WITHOUT the hash-cons dedup, so every node is
// emitted and the reduced node count equals the input node count N (0% node
// reduction). This is the neutral baseline for the node-count-reduction metric:
// the delta between this count and the fold+CSE count is the CSE benefit. The
// MEASURED QUANTITY is the node count in output high 32 bits, not time.
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

{prelude}
static PROG: OnceLock<Vec<PNode>> = OnceLock::new();

#[bench_variant("{name}", sizes = {sizes})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let prog = PROG.get_or_init(|| gen_prog(N));
    let n = prog.len();
    let mut o_op = vec![0u8; n];
    let mut o_val = vec![0i64; n];
    let cs = input_checksum(input);
    timed! {{ run {{
        let mut on: u32 = 0;
        for i in 0..n {{
            let nd = &prog[i];
            let (op, val) = if nd.op != 0 {{
                let ca = nd.a as usize;
                let cb = nd.b as usize;
                if o_op[ca] == 0 && o_op[cb] == 0 {{
                    let v = if nd.op == 1 {{
                        o_val[ca].wrapping_add(o_val[cb])
                    }} else {{
                        o_val[ca].wrapping_mul(o_val[cb])
                    }};
                    (0u8, v)
                }} else {{
                    (nd.op, 0i64)
                }}
            }} else {{
                (0u8, nd.val)
            }};
            o_op[on as usize] = op;
            o_val[on as usize] = val;
            on += 1; // no dedup: every node emitted
        }}
        let out = ((on as u64) << 32) | cs;
        output.copy_from_slice(&out.to_le_bytes());
    }} }}
}}
"""


def write_variant(name, cargo_name, lib_src):
    d = os.path.join(HERE, "variants", name, "src")
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(HERE, "variants", name, "Cargo.toml"), "w") as f:
        f.write(CARGO.format(name=cargo_name))
    with open(os.path.join(d, "lib.rs"), "w") as f:
        f.write(lib_src)


def bench_section(bench, title, seed, baseline, variants, may_differ=False, mode="subtract"):
    paths = ", ".join(f'"variants/{v}/target/release/{v}"' for v in variants)
    out = [
        f"[bench.{bench}]",
        f'title = "{title}"',
        'workload = "realistic"',
        f"master_seed = {seed}",
    ]
    if may_differ:
        out.append("may_differ = true")
    out += [
        f"[bench.{bench}.normalise]",
        f'baseline = "{baseline}"',
        f'mode = "{mode}"',
    ]
    for n in SIZES:
        out.append(f"[[bench.{bench}.sizes]]")
        out.append(f"n = {n}")
        out.append(f"variants = [{paths}]")
    return "\n".join(out) + "\n"


def main():
    made = []
    sections = []

    # --- effect inference ---
    for enc in ("thermo", "branchmax"):
        name = f"ei_{enc}"
        e = EFFECT_INFER_ENC[enc]
        src = EFFECT_INFER.format(
            enc=enc, name=name, sizes=SIZES_LIT,
            prelude=EFFECT_PRELUDE.format(),
            leaf=e["leaf"], perturb=e["perturb"], join=e["join"], scalar=e["scalar"],
        )
        write_variant(name, name, src)
        made.append(name)
    sections.append(bench_section(
        "effect_inference",
        "Effect inference: thermometer OR-join vs naive branch-max lattice join",
        "0x1eff_1a11_ce00_0001", "ei_thermo", ["ei_thermo", "ei_branchmax"],
    ))

    # --- effect inclusion ---
    for enc in ("thermo", "branchmax"):
        name = f"eg_{enc}"
        e = EFFECT_INCL_ENC[enc]
        prelude = EFFECT_PRELUDE.format()
        if enc == "branchmax":
            prelude = prelude + "\n" + BINCL_HELPER
        src = EFFECT_INCL.format(
            enc=enc, name=name, sizes=SIZES_LIT, prelude=prelude,
            leaf=e["leaf"], join=e["join"], permit2=e["permit2"],
            permitg=e["permitg"], violates=e["violates"],
        )
        write_variant(name, name, src)
        made.append(name)
    sections.append(bench_section(
        "effect_inclusion",
        "Effect inclusion gate: thermometer subset test vs naive per-family compare",
        "0x1eff_9a7e_ce00_0002", "eg_thermo", ["eg_thermo", "eg_branchmax"],
    ))

    # --- tnum multiply ---
    for tag, body, desc in (
        ("loop", TNUM_MUL_BODY["loop"], "always the 64-bit long-multiplication loop"),
        ("fastpath", TNUM_MUL_BODY["fastpath"], "both-known fast path lowers to a concrete multiply, else the loop"),
    ):
        name = f"tm_{tag}"
        src = TNUM_MUL.format(name=name, sizes=SIZES_LIT, prelude=TNUM_PRELUDE.format(), body=body, desc=desc)
        write_variant(name, name, src)
        made.append(name)
    sections.append(bench_section(
        "tnum_multiply",
        "tnum multiply: always-loop vs known-operand fast path (abstract arith)",
        "0x2717_0000_100b_0003", "tm_fastpath", ["tm_fastpath", "tm_loop"],
    ))

    # --- tnum linear ---
    lin = [("concrete", "concrete"), ("add", "tnum_add"), ("and", "tnum_and"), ("or", "tnum_or"), ("shl", "tnum_shl")]
    lin_names = []
    for tag, desc in lin:
        name = f"tl_{tag}"
        src = TNUM_LIN.format(name=name, sizes=SIZES_LIT, prelude=TNUM_PRELUDE.format(),
                              body=TNUM_LIN_BODY[tag], desc=desc)
        write_variant(name, name, src)
        made.append(name)
        lin_names.append(name)
    sections.append(bench_section(
        "tnum_linear",
        "tnum linear transfers: add/and/or/shl vs concrete u64 (abstract arith)",
        "0x2717_0000_11ec_0004", "tl_concrete", lin_names, may_differ=True,
    ))

    # --- cheap lowering (node-count metric) ---
    write_variant("cl_foldcse", "cl_foldcse", CHEAP_FOLDCSE.format(name="cl_foldcse", sizes=SIZES_LIT, prelude=CHEAP_PRELUDE.format()))
    write_variant("cl_foldonly", "cl_foldonly", CHEAP_FOLDONLY.format(name="cl_foldonly", sizes=SIZES_LIT, prelude=CHEAP_PRELUDE.format()))
    made += ["cl_foldcse", "cl_foldonly"]
    sections.append(bench_section(
        "cheap_lowering_nodecount",
        "Cheap lowering node-count reduction: fold+CSE vs fold-only (metric = node count, not time)",
        "0x3c5e_f01d_c5e0_0005", "cl_foldonly", ["cl_foldonly", "cl_foldcse"], may_differ=True,
    ))

    frag_dir = os.path.join(HERE, "carrier_fragments")
    os.makedirs(frag_dir, exist_ok=True)
    header = (
        "# effect-lattice / tnum / cheap-lowering bench cluster.\n"
        "# Generated by gen_effect_tnum_cheaplower.py. Concatenate into bench.toml.\n"
        "# NOTE: cheap_lowering_nodecount output encodes the reduced NODE COUNT in\n"
        "# the high 32 bits (metric), not latency; see results/*/FINDINGS_SCAFFOLD.md.\n\n"
    )
    with open(os.path.join(frag_dir, "effect_tnum_cheaplower.toml"), "w") as f:
        f.write(header + "\n".join(sections))

    print("generated variants:", ", ".join(made))
    print("wrote carrier_fragments/effect_tnum_cheaplower.toml")


if __name__ == "__main__":
    main()
