#!/usr/bin/env python3
"""Multiway (if/elseif chain) branch strategies, appended to the branch bench.
K-way selection `if k==0 {A0} else if k==1 {A1} ... else {A_{K-1}}`. Strategies:
  chain      : sequential if/elseif compares to the ONE selected arm.
  jumptable  : match -> LLVM jump table, O(1) dispatch to the ONE selected arm.
  bintree    : manual binary search, log2(K) compares to the ONE selected arm.
  predicate_all: evaluate ALL K arms, select the k-th (branchless, K x work; cheap arms only).
Situations differ by the KEY distribution: uniform (each arm 1/K) vs skewed (arm 0 dominant).
Cross-validated (all compute the same xor-reduction). Extensible.
Writes crates into variants/ and APPENDS bench sections to bench.toml."""
import os
SIZES = "[64, 256, 1024, 4096, 16384]"
ARM = '''#[inline(never)]
fn arm(idx: u32, b: u8, work: u32) -> u64 {
    let mut a: u64 = (0x100000001b3u64).wrapping_mul(idx as u64 + 1) ^ (b as u64);
    let mut c = 0; while c < work { a = a.rotate_left(7).wrapping_mul(0x9e3779b97f4a7c15) ^ (a >> 31); c += 1; }
    a
}'''
COSTS = {"c": 1, "h": 24}
# situations: (K, desc, key_expr) key in [0,K) from byte b
MWSITS = {
    "mw3_uni":  (3, "3-way, uniform key",       "let k = (b as u32) % 3;"),
    "mw3_skew": (3, "3-way, skewed to arm 0 (~80%)", "let k = if b < 205 { 0 } else if b < 230 { 1 } else { 2 };"),
    "mw8_uni":  (8, "8-way, uniform key",       "let k = (b as u32) & 7;"),
    "mw8_skew": (8, "8-way, skewed to arm 0 (~70%)", "let k = if b < 179 { 0 } else { 1 + ((b as u32) & 7) % 7 };"),
}
def chain_body(K):
    parts = []
    for i in range(K-1):
        parts.append(f'{"if" if i==0 else "} else if"} k == {i} {{ v = arm({i}, b, W);')
    parts.append(f'}} else {{ v = arm({K-1}, b, W); }}')
    return "\n                ".join(parts)
def chain_rev_body(K):
    # backwards linear: test k==K-1 first, down to 0 (wins when the hot arm is LAST)
    parts = []
    for j, i in enumerate(range(K-1, 0, -1)):
        parts.append(f'{"if" if j==0 else "} else if"} k == {i} {{ v = arm({i}, b, W);')
    parts.append('} else { v = arm(0, b, W); }')
    return "\n                ".join(parts)
def jumptable_body(K):
    arms = " ".join(f'{i} => arm({i}, b, W),' for i in range(K-1))
    return f'v = match k {{ {arms} _ => arm({K-1}, b, W) }};'
def bintree_body(K, lo=0, hi=None, ind=0):
    if hi is None: hi = K
    if hi - lo == 1:
        return f'v = arm({lo}, b, W);'
    mid = (lo + hi) // 2
    return (f'if k < {mid} {{ {bintree_body(K, lo, mid, ind+1)} }} '
            f'else {{ {bintree_body(K, mid, hi, ind+1)} }}')
def predicate_all_body(K):
    # evaluate all arms into a fixed array, select k-th (branchless)
    arms = ", ".join(f'arm({i}, b, W)' for i in range(K))
    return f'let all = [{arms}]; v = all[k as usize];'
def strat_body(strat, K):
    if strat == "chain": return chain_body(K)
    if strat == "chain_rev": return chain_rev_body(K)
    if strat == "jumptable": return jumptable_body(K)
    if strat == "bintree": return bintree_body(K)
    if strat == "predicate_all": return predicate_all_body(K)
    raise ValueError(strat)
def applicable(strat, cost, K):
    if strat == "predicate_all" and cost == "h": return False  # K heavy arms = absurd
    return True
STRATS = ["chain", "chain_rev", "jumptable", "bintree", "predicate_all"]
def crate(strat, cost, sit):
    name = f"mw_{strat}_{cost}_{sit}"
    K, desc, keyexpr = MWSITS[sit]; W = COSTS[cost]
    body = strat_body(strat, K)
    lib = f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
const W: u32 = {W};
{ARM}
// multiway `{strat}`, arm-cost `{cost}` (W={W}), situation `{sit}`: {desc}
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{ run {{
        let mut acc: u64 = 0;
        for &b in input.iter() {{
            {keyexpr}
            let v;
            {body}
            acc ^= v;
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''
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
    return name
def bench_sections():
    out = []
    for cost in COSTS:
        for sit in MWSITS:
            variants = [f'"variants/mw_{s}_{cost}_{sit}/target/release/mw_{s}_{cost}_{sit}"'
                        for s in STRATS if applicable(s, cost, MWSITS[sit][0])]
            bn = f"multiway_{sit}_{cost}"; desc = MWSITS[sit][1]
            costname = "cheap-arm" if cost=="c" else "heavy-arm"
            out.append(f'[bench.{bn}]')
            out.append(f'title = "Multiway branch strategies, {costname}, {sit}: {desc}"')
            out.append('workload = "realistic"')
            out.append('master_seed = 0x7654_3210_fedc_ba98')
            for n in [64,256,1024,4096,16384]:
                out.append(f'[[bench.{bn}.sizes]]'); out.append(f'n = {n}')
                out.append(f'variants = [{", ".join(variants)}]')
            out.append('')
    return "\n".join(out)
if __name__ == "__main__":
    names=[]
    for cost in COSTS:
        for sit in MWSITS:
            K = MWSITS[sit][0]
            for s in STRATS:
                if applicable(s, cost, K): names.append(crate(s, cost, sit))
    with open("bench.toml","a") as f: f.write("\n"+bench_sections())
    print(f"generated {len(names)} multiway variants")
