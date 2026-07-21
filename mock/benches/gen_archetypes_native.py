#!/usr/bin/env python3
"""Native tier of the per-branch archetype showdown: the SAME shared program as the interp tier, but
lowered to compiled Rust with each branch's strategy expressed as a native construct (if-chain / match /
binary-search / eval-all / hot-first). Same bench structure (per-branch, per-type, whole) x strategies,
so tier is a true variant dimension on the same program. Cross-validated against interp (identical output).
Reveals where the strategy choice diverges under native (dispatch cheap => strategies converge, predicate
still loses) vs interp (node-walk dominates)."""
import os
SIZES = "[64, 256, 1024, 4096, 16384]"
STRATS = {"seq":0,"pred":1,"table":2,"tree":3,"prof":4}
DEFAULT="table"
# archetypes: (kind, cnt, shape) kind: 'm'atch4 'M'atch8 'i'fchain ; shape: 'lin','nm','ni','blk'
ARCH=[('m',4,'lin'),('m',4,'nm'),('M',8,'lin'),('m',4,'blk'),('i',4,'lin'),('i',4,'ni'),('i',4,'blk')]
MATCH_IDX=[i for i,a in enumerate(ARCH) if a[0] in ('m','M')]
IFCH_IDX =[i for i,a in enumerate(ARCH) if a[0]=='i']

HELPERS='''#[inline(always)] fn key4(b:u64)->u64 { if b<179 {2} else {[0u64,1,3][(b as usize)%3]} }
#[inline(always)] fn lin(b:u64,i:u64)->u64 { (b.wrapping_mul(0x100000001b3u64.wrapping_mul(i+1)) ^ 0x9e3779b97f4a7c15u64.wrapping_add(i)).rotate_left(((i%13)+1) as u32) }
#[inline(always)] fn nm(b:u64,i:u64)->u64 { match key4(b) { 0=>lin(b,i*4),1=>lin(b,i*4+1),2=>lin(b,i*4+2),_=>lin(b,i*4+3) } }
#[inline(always)] fn ni(b:u64,i:u64)->u64 { let bk=((b>=64)as u64)+((b>=128)as u64)+((b>=192)as u64); match bk {0=>lin(b,i*4),1=>lin(b,i*4+1),2=>lin(b,i*4+2),_=>lin(b,i*4+3)} }
#[inline(always)] fn blk(b:u64,i:u64)->u64 { lin(b,i) ^ lin(b,i*7+1) ^ lin(b,i*7+2) ^ lin(b,i*7+3) ^ lin(b,i*7+4) }'''

def arm_expr(shape,j):
    return {'lin':f'lin(b,{j})','nm':f'nm(b,{j})','ni':f'ni(b,{j})','blk':f'blk(b,{j})'}[shape]

def match_branch(kind,cnt,shape,strat):
    a=[arm_expr(shape,j) for j in range(cnt)]
    key = 'key4(b)' if kind=='m' else '(b & 7)'
    if strat=='table':
        arms=' '.join(f'{j}=>{a[j]},' for j in range(cnt-1))
        return f'{{ let k={key}; match k {{ {arms} _=>{a[cnt-1]} }} }}'
    if strat=='seq':
        chain=' else '.join(f'if k=={j} {{ {a[j]} }}' for j in range(cnt-1))
        return f'{{ let k={key}; {chain} else {{ {a[cnt-1]} }} }}'
    if strat=='prof':
        order=[2]+[j for j in range(cnt) if j!=2]
        chain=' else '.join(f'if k=={order[t]} {{ {a[order[t]]} }}' for t in range(cnt-1))
        return f'{{ let k={key}; {chain} else {{ {a[order[cnt-1]]} }} }}'
    if strat=='pred':
        arr=', '.join(a)
        return f'{{ let k={key}; let arr=[{arr}]; arr[k as usize] }}'
    if strat=='tree':
        def bt(lo,hi):
            if hi-lo==1: return a[lo]
            mid=(lo+hi)//2
            return f'if k<{mid} {{ {bt(lo,mid)} }} else {{ {bt(mid,hi)} }}'
        return f'{{ let k={key}; {bt(0,cnt)} }}'

def ifchain_branch(cnt,shape,strat):
    a=[arm_expr(shape,j) for j in range(cnt)]
    T=[50,120,200]
    if strat=='table':
        bk='('+'+'.join(f'(b>={T[t]}) as u64' for t in range(cnt-1))+')'
        arms=' '.join(f'{j}=>{a[j]},' for j in range(cnt-1))
        return f'{{ let bk={bk}; match bk {{ {arms} _=>{a[cnt-1]} }} }}'
    if strat in ('seq','prof'):
        chain=' else '.join(f'if b<{T[t]} {{ {a[t]} }}' for t in range(cnt-1))
        return f'{{ {chain} else {{ {a[cnt-1]} }} }}'
    if strat=='pred':
        bk='('+'+'.join(f'(b>={T[t]}) as u64' for t in range(cnt-1))+')'
        arr=', '.join(a)
        return f'{{ let arr=[{arr}]; let bk={bk}; arr[bk as usize] }}'
    if strat=='tree':
        bk='('+'+'.join(f'(b>={T[t]}) as u64' for t in range(cnt-1))+')'
        arms=' '.join(f'{j}=>{a[j]},' for j in range(cnt-1))
        return f'{{ let bk={bk}; match bk {{ {arms} _=>{a[cnt-1]} }} }}'

def branch_expr(idx, strat):
    kind,cnt,shape=ARCH[idx]
    if kind in ('m','M'): return match_branch(kind,cnt,shape,strat)
    return ifchain_branch(cnt,shape,strat)

def variant(name, strat_names):
    # strat_names: list of 7 strategy names (per branch)
    branches='0u64 ^ '+' ^ '.join(branch_expr(j, strat_names[j]) for j in range(len(ARCH)))
    lib=f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
{HELPERS}
#[inline(always)] fn program(b:u64)->u64 {{ {branches} }}
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{ run {{
        let mut acc:u64=0;
        for &b in input.iter() {{ acc ^= program(b as u64); }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''
    cargo=f'''[workspace]
[package]
name="{name}"
version="0.0.0"
edition="2021"
publish=false
[lib]
name="{name}"
path="src/lib.rs"
crate-type=["cdylib"]
[dependencies]
mockspace-bench-core={{ git="https://github.com/hiisi-digital/mockspace", branch="dev", features=["std"] }}
mockspace-bench-macro={{ git="https://github.com/hiisi-digital/mockspace", branch="dev" }}
[profile.release]
opt-level=3
lto="fat"
codegen-units=1
'''
    d=f"variants/{name}"; os.makedirs(f"{d}/src",exist_ok=True)
    open(f"{d}/Cargo.toml","w").write(cargo); open(f"{d}/src/lib.rs","w").write(lib)

def bench_sections():
    out=[]
    def addbench(bn,title,variants,baseline):
        out.append(f'[bench.{bn}]'); out.append(f'title="{title}"')
        out.append('workload="realistic"'); out.append('master_seed=0x7654_3210_fedc_ba98')
        out.append(f'[bench.{bn}.normalise]'); out.append(f'baseline="{baseline}"'); out.append('mode="subtract"')
        for n in [64,256,1024,4096,16384]:
            out.append(f'[[bench.{bn}.sizes]]'); out.append(f'n={n}'); out.append(f'variants=[{", ".join(variants)}]')
        out.append('')
    for j in range(len(ARCH)):
        vs=[]
        for sn in STRATS:
            names=[DEFAULT]*len(ARCH); names[j]=sn; nm_=f"an_b{j}_{sn}"; variant(nm_,names)
            vs.append(f'"variants/{nm_}/target/release/{nm_}"')
        addbench(f"archn_b{j}", f"Per-branch strategy (NATIVE tier): archetype {j}", vs, f"an_b{j}_table")
    for tname,idxs in [("match",MATCH_IDX),("ifchain",IFCH_IDX)]:
        vs=[]
        for sn in STRATS:
            names=[DEFAULT]*len(ARCH)
            for i in idxs: names[i]=sn
            nm_=f"an_{tname}_{sn}"; variant(nm_,names); vs.append(f'"variants/{nm_}/target/release/{nm_}"')
        addbench(f"archn_{tname}", f"Per-type strategy (NATIVE tier): all {tname}", vs, f"an_{tname}_table")
    vs=[]
    for sn in STRATS:
        nm_=f"an_whole_{sn}"; variant(nm_,[sn]*len(ARCH)); vs.append(f'"variants/{nm_}/target/release/{nm_}"')
    addbench("archn_whole", "Whole-program single strategy (NATIVE tier)", vs, "an_whole_table")
    return "\n".join(out)

if __name__=="__main__":
    with open("bench.toml","a") as f: f.write("\n"+bench_sections())
    print(f"generated {(len(ARCH)+3)*len(STRATS)} native archetype variants")
