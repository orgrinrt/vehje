#!/usr/bin/env python3
"""Unified strategy SHOWDOWN: every dispatch strategy on the SAME K-way scenario,
each in three TIERS (native / interp / jit) so copy-and-patch, plain branching,
luts, predicate, profiled, backwards-linear are all comparable on the same footing.
The interp tier runs the strategy's dispatch as BYTECODE through a switch-interpreter
(the interpreted plumbing) so the native/jit comparison is fair; native lowers it to
compiled code; jit copy-and-patches it to machine code. Arms are a shared #[inline(never)]
call in every tier, so the tier difference is purely the DISPATCH cost. K=4.

Scenario `skew2`: key ~70% arm 2, else spread => distinguishes profiled (hot-first),
chain (tests 0 first), chain_rev, jumptable. Extensible: add strategies/tiers/scenarios."""
import os
SIZES = "[64, 256, 1024, 4096, 16384]"
K = 4

# dispatch bytecode ISA: (op, arg). op: 0=EQ 1=LT 2=BRT 3=RETI 4=RETK
def chain_prog(order):
    # test order[0..K-1] in sequence; default = order[K-1]
    prog = []
    # first the compare/branch pairs, then the RETI targets
    for i in range(K-1):
        prog.append((0, order[i]))    # EQ order[i]
        prog.append((2, None))        # BRT -> target (patched)
    prog.append((3, order[K-1]))      # RETI default
    ret_base = len(prog)
    for i in range(K-1):
        prog.append((3, order[i]))    # RETI order[i]
    # patch BRT targets: the j-th BRT (at index 2*j+1) -> ret_base + j
    for j in range(K-1):
        prog[2*j+1] = (2, ret_base + j)
    return prog
def bintree_prog():
    # K=4 balanced
    return [(1,2),(2,6),(1,3),(2,5),(3,3),(3,2),(1,1),(2,9),(3,1),(3,0)]
def jumptable_prog(): return [(4,0)]      # RETK
def evalall_prog(): return [(4,0)]        # RETK; wrapper evals all

STRATS = {
    # name: (prog, order_for_native, all_arms?)
    "chain":     (chain_prog([0,1,2,3]), [0,1,2,3], False),
    "chain_rev": (chain_prog([3,2,1,0]), [3,2,1,0], False),
    "profiled":  (chain_prog([2,0,1,3]), [2,0,1,3], False),  # hot arm 2 first
    "jumptable": (jumptable_prog(),      None,      False),
    "bintree":   (bintree_prog(),        None,      False),
    "evalall":   (evalall_prog(),        None,      True),
}
SCEN = {
    "skew2": ("key ~70% arm 2, else spread", "let k: u32 = if b < 179 { 2 } else { [0u32,1,3][(b as usize) % 3] };"),
}
COSTS = {"c": 1}

ARM = '''#[inline(never)]
fn arm(idx: u32, b: u8, w: u32) -> u64 {
    let mut a: u64 = (0x100000001b3u64).wrapping_mul(idx as u64 + 1) ^ (b as u64);
    let mut c = 0; while c < w { a = a.rotate_left(7).wrapping_mul(0x9e3779b97f4a7c15) ^ (a >> 31); c += 1; }
    a
}'''

def native_dispatch(order):
    s = ""
    for i in range(K-1):
        s += f"if k == {order[i]} {{ {order[i]} }} else "
    s += f"{{ {order[K-1]} }}"
    return "let idx: u32 = " + s + ";"

def wrapper(all_arms):
    if all_arms:
        return f'let mut arr = [0u64; {K}]; let mut j = 0u32; while j < {K} {{ arr[j as usize] = arm(j, b, W); j += 1; }} let v = arr[k as usize];'
    else:
        return 'let v = arm(idx, b, W);'

def native_lib(name, strat, scen):
    prog, order, all_arms = STRATS[strat]
    _, keyexpr = SCEN[scen]
    if strat in ("jumptable", "evalall"):
        disp = "let idx: u32 = k;"
    elif strat == "bintree":
        disp = "let idx: u32 = if k < 2 { if k < 1 { 0 } else { 1 } } else { if k < 3 { 2 } else { 3 } };"
    else:
        disp = native_dispatch(order)
    return f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
const W: u32 = 1;
{ARM}
// SHOWDOWN native tier: strategy `{strat}`, scenario `{scen}`
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{ run {{
        let mut acc: u64 = 0;
        for &b in input.iter() {{
            {keyexpr}
            {disp}
            {wrapper(all_arms)}
            acc ^= v;
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''

def interp_lib(name, strat, scen):
    prog, order, all_arms = STRATS[strat]
    _, keyexpr = SCEN[scen]
    progrs = "&[" + ", ".join(f'({op}u8, {a}i16)' for (op,a) in prog) + "]"
    return f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
const W: u32 = 1;
{ARM}
// dispatch bytecode + switch interpreter (the interpreted plumbing)
static PROG: &[(u8, i16)] = {progrs};
#[inline(never)]
fn dispatch(prog: &[(u8, i16)], k: u32) -> u32 {{
    let mut pc = 0usize; let mut flag = false;
    loop {{
        let (op, a) = prog[pc];
        match op {{
            0 => {{ flag = k == a as u32; pc += 1; }}
            1 => {{ flag = k < a as u32; pc += 1; }}
            2 => {{ if flag {{ pc = a as usize; }} else {{ pc += 1; }} }}
            3 => return a as u32,
            _ => return k,
        }}
    }}
}}
// SHOWDOWN interp tier: strategy `{strat}`, scenario `{scen}`
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{ run {{
        let mut acc: u64 = 0;
        for &b in input.iter() {{
            {keyexpr}
            let idx = dispatch(PROG, k);
            {wrapper(all_arms)}
            acc ^= v;
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''

def crate(name, lib):
    cargo = f'''[workspace]
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
'''
    d = f"variants/{name}"; os.makedirs(f"{d}/src", exist_ok=True)
    open(f"{d}/Cargo.toml","w").write(cargo); open(f"{d}/src/lib.rs","w").write(lib)

def bench_sections():
    out = []
    for scen in SCEN:
        variants = []
        for strat in STRATS:
            for tier in ("nat","int"):
                variants.append(f'"variants/sd_{strat}_{tier}_{scen}/target/release/sd_{strat}_{tier}_{scen}"')
        bn = f"showdown_{scen}"
        out.append(f'[bench.{bn}]')
        out.append(f'title = "Strategy showdown ({scen}): all strategies x tiers (native/interp) same footing"')
        out.append('workload = "realistic"'); out.append('master_seed = 0x7654_3210_fedc_ba98')
        for n in [64,256,1024,4096,16384]:
            out.append(f'[[bench.{bn}.sizes]]'); out.append(f'n = {n}')
            out.append(f'variants = [{", ".join(variants)}]')
        out.append('')
    return "\n".join(out)

if __name__ == "__main__":
    cnt = 0
    for scen in SCEN:
        for strat in STRATS:
            crate(f"sd_{strat}_nat_{scen}", native_lib(f"sd_{strat}_nat_{scen}", strat, scen)); cnt += 1
            crate(f"sd_{strat}_int_{scen}", interp_lib(f"sd_{strat}_int_{scen}", strat, scen)); cnt += 1
    with open("bench.toml","a") as f: f.write("\n"+bench_sections())
    print(f"generated {cnt} showdown variants (native+interp)")
