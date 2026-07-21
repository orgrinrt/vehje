#!/usr/bin/env python3
"""Honest IR-driven strategy showdown. ONE mockup IR program (the scenario) executed by every
strategy x tier, so copy-and-patch, branch, predicate, jumptable, chain_rev, bintree, profiled are
compared on the SAME footing and SAME workload. Tiers:
  int : a REAL tree-walking IR interpreter dispatches every node (the honest interpreted plumbing).
  nat : the IR lowered to native (reference ceiling; what an optimizing backend produces).
  jit : copy-and-patch emits the IR to machine code (added in the jit generator).
The strategy only changes how the Match node is handled; arms are IR subtrees (walked in int, inlined
in nat). Scenario `skew2`: key ~70% arm 2. K=4. Cross-validated (all agree)."""
import os
SIZES = "[64, 256, 1024, 4096, 16384]"

IR_COMMON = '''#[derive(Clone, Copy)]
struct Node { op: u8, a: u32, b: u32, imm: u64 }
const INPUT: u8 = 0; const CONST: u8 = 1; const MUL: u8 = 3; const XOR: u8 = 4; const ROTL: u8 = 5; const KEYOP: u8 = 6; const MATCH: u8 = 7;
#[inline(always)]
fn key_fn(b: u8) -> u64 { if b < 179 { 2 } else { [0u64,1,3][(b as usize) % 3] } }
fn build() -> (Vec<Node>, [u32;4], u32) {
    let mut n: Vec<Node> = Vec::new();
    macro_rules! push { ($op:expr,$a:expr,$b:expr,$imm:expr) => {{ let id=n.len() as u32; n.push(Node{op:$op,a:$a,b:$b,imm:$imm}); id }}; }
    let input = push!(INPUT,0,0,0);
    let ps=[0x100000001b3u64,0x9e3779b97f4a7c15,0xc2b2ae3d27d4eb4f,0x165667b19e3779f9];
    let qs=[0x27d4eb2f165667c5u64,0x85ebca77c2b2ae63,0xff51afd7ed558ccd,0xc4ceb9fe1a85ec53];
    let mut arms=[0u32;4];
    for i in 0..4 { let cp=push!(CONST,0,0,ps[i]); let mul=push!(MUL,input,cp,0); let cq=push!(CONST,0,0,qs[i]); let xor=push!(XOR,mul,cq,0); arms[i]=push!(ROTL,xor,0,(i as u64)+1); }
    let key=push!(KEYOP,0,0,0); let mroot=push!(MATCH,key,0,4);
    (n, arms, mroot)
}
use std::sync::OnceLock;
static PROG: OnceLock<(Vec<Node>, [u32;4], u32)> = OnceLock::new();'''

# strategy -> the Match-node handler body (k in scope, returns u64). Uses eval(nodes,arms,arm,b).
MATCH = {
 "branch":    "let arm = if k==0 {a[0]} else if k==1 {a[1]} else if k==2 {a[2]} else {a[3]}; eval(nd,a,arm,b)",
 "chain_rev": "let arm = if k==3 {a[3]} else if k==2 {a[2]} else if k==1 {a[1]} else {a[0]}; eval(nd,a,arm,b)",
 "profiled":  "let arm = if k==2 {a[2]} else if k==0 {a[0]} else if k==1 {a[1]} else {a[3]}; eval(nd,a,arm,b)",
 "jumptable": "eval(nd,a,a[k as usize],b)",
 "bintree":   "let arm = if k<2 { if k<1 {a[0]} else {a[1]} } else { if k<3 {a[2]} else {a[3]} }; eval(nd,a,arm,b)",
 "predicate": "let mut v=[0u64;4]; let mut i=0usize; while i<4 { v[i]=eval(nd,a,a[i],b); i+=1; } v[k as usize]",
}

def interp_lib(name, strat):
    handler = MATCH[strat]
    return f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
{IR_COMMON}
// REAL tree-walking interpreter: dispatch every node. Match handled by strategy `{strat}`.
fn eval(nd: &[Node], a: &[u32;4], node: u32, b: u8) -> u64 {{
    let n = nd[node as usize];
    match n.op {{
        INPUT => b as u64,
        CONST => n.imm,
        MUL => eval(nd,a,n.a,b).wrapping_mul(eval(nd,a,n.b,b)),
        XOR => eval(nd,a,n.a,b) ^ eval(nd,a,n.b,b),
        ROTL => eval(nd,a,n.a,b).rotate_left(n.imm as u32),
        KEYOP => key_fn(b),
        MATCH => {{ let k = eval(nd,a,n.a,b); {handler} }}
        _ => 0,
    }}
}}
// SHOWDOWN-IR interp tier: strategy `{strat}`
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let (nd, a, mroot) = PROG.get_or_init(build);
    timed! {{ run {{
        let mut acc: u64 = 0;
        for &b in input.iter() {{ acc ^= eval(nd, a, *mroot, b); }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''

# native tier: the IR lowered to native (arms inlined, strategy applied to the match). Reference ceiling.
def native_lib(name, strat):
    # inline arm computation: arm_i(b) = rotl(xor(mul(b, Pi), Qi), i+1)
    arm_fns = '''#[inline(always)]
fn arm(i: u32, b: u8) -> u64 {
    const PS: [u64;4]=[0x100000001b3,0x9e3779b97f4a7c15,0xc2b2ae3d27d4eb4f,0x165667b19e3779f9];
    const QS: [u64;4]=[0x27d4eb2f165667c5,0x85ebca77c2b2ae63,0xff51afd7ed558ccd,0xc4ceb9fe1a85ec53];
    ((b as u64).wrapping_mul(PS[i as usize]) ^ QS[i as usize]).rotate_left(i+1)
}'''
    nat = {
     "branch":    "let idx=if k==0 {0} else if k==1 {1} else if k==2 {2} else {3}; arm(idx,b)",
     "chain_rev": "let idx=if k==3 {3} else if k==2 {2} else if k==1 {1} else {0}; arm(idx,b)",
     "profiled":  "let idx=if k==2 {2} else if k==0 {0} else if k==1 {1} else {3}; arm(idx,b)",
     "jumptable": "arm(k,b)",
     "bintree":   "let idx=if k<2 { if k<1 {0} else {1} } else { if k<3 {2} else {3} }; arm(idx,b)",
     "predicate": "let mut v=[0u64;4]; let mut i=0u32; while i<4 { v[i as usize]=arm(i,b); i+=1; } v[k as usize]",
    }[strat]
    return f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
#[inline(always)]
fn key_fn(b: u8) -> u32 {{ if b < 179 {{ 2 }} else {{ [0u32,1,3][(b as usize) % 3] }} }}
{arm_fns}
// SHOWDOWN-IR native tier (IR lowered; reference ceiling): strategy `{strat}`
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{ run {{
        let mut acc: u64 = 0;
        for &b in input.iter() {{ let k = key_fn(b); acc ^= {{ {nat} }}; }}
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
    d=f"variants/{name}"; os.makedirs(f"{d}/src",exist_ok=True)
    open(f"{d}/Cargo.toml","w").write(cargo); open(f"{d}/src/lib.rs","w").write(lib)

STRATS = ["branch","chain_rev","profiled","jumptable","bintree","predicate"]
def bench_section():
    variants=[]
    for s in STRATS:
        for t in ("int","nat"):
            variants.append(f'"variants/ir_{s}_{t}/target/release/ir_{s}_{t}"')
    out=['[bench.ir_match]','title = "IR-driven strategy showdown: interp vs native, all strategies, same mockup IR"',
         'workload = "realistic"','master_seed = 0x7654_3210_fedc_ba98']
    for n in [64,256,1024,4096,16384]:
        out.append('[[bench.ir_match.sizes]]'); out.append(f'n = {n}')
        out.append(f'variants = [{", ".join(variants)}]')
    return "\n".join(out)+"\n"

if __name__ == "__main__":
    c=0
    for s in STRATS:
        crate(f"ir_{s}_int", interp_lib(f"ir_{s}_int", s)); c+=1
        crate(f"ir_{s}_nat", native_lib(f"ir_{s}_nat", s)); c+=1
    with open("bench.toml","a") as f: f.write("\n"+bench_section())
    print(f"generated {c} IR-showdown variants (interp+native)")
