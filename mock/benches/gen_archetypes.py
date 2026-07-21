#!/usr/bin/env python3
"""Per-branch strategy showdown over ONE shared IR program of branch ARCHETYPES (match + if-else-chain,
varied arm shapes). The interpreter dispatches each branch node's `strat` field, so variants differ only
in which branch(es) use which strategy. Three bench levels, all interp tier (real tree-walk):
  per-branch  (arch_bN): vary one archetype branch's strat, others fixed  -> best strategy PER archetype.
  per-type    (arch_match / arch_ifchain): vary all match / all ifchain branches -> best per node-type.
  whole       (arch_whole): vary every branch together -> best single overall strategy.
Every strategy is semantically identical per branch (returns the same arm), so cross-validated. Extensible."""
import os
SIZES = "[64, 256, 1024, 4096, 16384]"
STRATS = {"seq":0, "pred":1, "table":2, "tree":3, "prof":4}
DEFAULT = 2  # TABLE for the fixed branches
# archetypes: (name, kind) kind: 'm'=match 'i'=ifchain
ARCH = [("match4_linear","m"),("match4_nested","m"),("match8_linear","m"),("match4_blocks","m"),
        ("ifchain4_linear","i"),("ifchain4_nested","i"),("ifchain4_blocks","i")]
NARCH=len(ARCH)
MATCH_IDX=[i for i,(_,k) in enumerate(ARCH) if k=="m"]
IFCH_IDX =[i for i,(_,k) in enumerate(ARCH) if k=="i"]

INTERP = r'''use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[derive(Clone, Copy)]
struct Node { op: u8, a: u32, imm: u64, aux: u32, strat: u8 }
const INPUT:u8=0; const CONST:u8=1; const MUL:u8=2; const XOR:u8=3; const ROTL:u8=4;
const KEY4:u8=6; const KEY8:u8=7; const MATCH:u8=8; const IFCHAIN:u8=11;
struct Prog { nodes: Vec<Node>, arms: Vec<u32>, th: Vec<u64>, roots: Vec<u32> }
#[inline(always)] fn key4(b:u8)->u64 { if b<179 {2} else {[0u64,1,3][(b as usize)%3]} }
#[inline(always)] fn key8(b:u8)->u64 { (b as u64)&7 }
struct Bld { p: Prog }
impl Bld {
    fn n(&mut self,op:u8,a:u32,imm:u64,aux:u32,strat:u8)->u32{ let id=self.p.nodes.len() as u32; self.p.nodes.push(Node{op,a,imm,aux,strat}); id }
    fn arm_linear(&mut self, input:u32, i:u64)->u32{
        let cp=self.n(CONST,0,0x100000001b3u64.wrapping_mul(i+1),0,0);
        let m=self.n(MUL,input,0,cp,0);
        let cq=self.n(CONST,0,0x9e3779b97f4a7c15u64.wrapping_add(i),0,0);
        let x=self.n(XOR,m,0,cq,0);
        self.n(ROTL,x,(i%13)+1,0,0)
    }
    fn arm_nested_m(&mut self, input:u32, i:u64)->u32{
        let key=self.n(KEY4,input,0,0,0); let base=self.p.arms.len() as u32;
        for j in 0..4 { let a=self.arm_linear(input, i*4+j); self.p.arms.push(a); }
        self.n(MATCH,key,0, base | (4u32<<24), DEFAULT_STRAT)
    }
    fn arm_nested_i(&mut self, input:u32, i:u64)->u32{
        let tb=self.p.th.len() as u64; self.p.th.push(64); self.p.th.push(128); self.p.th.push(192);
        let base=self.p.arms.len() as u32;
        for j in 0..4 { let a=self.arm_linear(input, i*4+j); self.p.arms.push(a); }
        self.n(IFCHAIN,input,tb, base | (4u32<<24), DEFAULT_STRAT)
    }
    fn arm_blocks(&mut self, input:u32, i:u64)->u32{
        let mut acc=self.arm_linear(input, i);
        for j in 1..5u64 { let blk=self.arm_linear(input, i*7+j); let x=self.n(XOR,acc,0,blk,0); acc=x; }
        acc
    }
    fn arm_of(&mut self, input:u32, i:u64, shape:u8, kind:u8)->u32{
        match shape { 1=> if kind==0 {self.arm_nested_m(input,i)} else {self.arm_nested_i(input,i)},
                      2=>self.arm_blocks(input,i), _=>self.arm_linear(input,i) }
    }
    fn match_arch(&mut self, input:u32, keyop:u8, k:usize, strat:u8, shape:u8)->u32{
        let key=self.n(keyop,input,0,0,0); let base=self.p.arms.len() as u32;
        for i in 0..k { let a=self.arm_of(input,i as u64,shape,0); self.p.arms.push(a); }
        self.n(MATCH, key, 0, base | ((k as u32)<<24), strat)
    }
    fn ifchain_arch(&mut self, input:u32, k:usize, strat:u8, shape:u8)->u32{
        let tb=self.p.th.len() as u64;
        let ths=[50u64,120,200]; for t in 0..k-1 { self.p.th.push(ths[t]); }
        let base=self.p.arms.len() as u32;
        for i in 0..k { let a=self.arm_of(input,i as u64,shape,1); self.p.arms.push(a); }
        self.n(IFCHAIN, input, tb, base | ((k as u32)<<24), strat)
    }
}
fn eval(p:&Prog, node:u32, b:u8)->u64{
    let n=p.nodes[node as usize];
    match n.op{
        INPUT=>b as u64, CONST=>n.imm,
        MUL=>eval(p,n.a,b).wrapping_mul(eval(p,n.aux,b)),
        XOR=>eval(p,n.a,b)^eval(p,n.aux,b),
        ROTL=>eval(p,n.a,b).rotate_left(n.imm as u32),
        KEY4=>key4(b), KEY8=>key8(b),
        MATCH=>{
            let k=eval(p,n.a,b); let base=(n.aux&0xffffff) as usize; let cnt=(n.aux>>24) as usize;
            let arm=|i:usize| p.arms[base+i];
            match n.strat{
                1=>{ let mut acc=0u64; for i in 0..cnt { let v=eval(p,arm(i),b); if i==k as usize {acc=v;} } acc }
                2=>eval(p,arm(k as usize),b),
                3=>{ let mut lo=0usize; let mut hi=cnt; while hi-lo>1 { let mid=(lo+hi)/2; if (k as usize)<mid {hi=mid} else {lo=mid} } eval(p,arm(lo),b) }
                4=>{ let chosen=if k as usize==2 && cnt>2 {arm(2)} else { let mut c=arm(cnt-1); for i in 0..cnt { if i==k as usize {c=arm(i);break;} } c }; eval(p,chosen,b) }
                _=>{ let mut c=arm(cnt-1); for i in 0..cnt { if i==k as usize {c=arm(i);break;} } eval(p,c,b) }
            }
        }
        IFCHAIN=>{
            let base=(n.aux&0xffffff) as usize; let cnt=(n.aux>>24) as usize; let tb=n.imm as usize;
            let arm=|i:usize| p.arms[base+i]; let th=|i:usize| p.th[tb+i]; let bb=b as u64;
            // bucket = |{ i<cnt-1 : b >= th(i) }| ; every strat returns arm[bucket], different method
            match n.strat{
                1=>{ let mut vals=[0u64;8]; for i in 0..cnt { vals[i]=eval(p,arm(i),b); } let mut bk=0usize; for i in 0..cnt-1 { if bb>=th(i){bk+=1;} } vals[bk] }
                2=>{ let mut bk=0usize; for i in 0..cnt-1 { bk += (bb>=th(i)) as usize; } eval(p,arm(bk),b) }
                3=>{ let mut lo=0usize; let mut hi=cnt-1; while lo<hi { let mid=(lo+hi)/2; if bb>=th(mid){lo=mid+1}else{hi=mid} } eval(p,arm(lo),b) }
                4=>{ let h=cnt/2; let bk=if h>0 && h<cnt-1 && bb>=th(h-1) && bb<th(h) {h} else { let mut x=0; while x<cnt-1 { if bb<th(x){break;} x+=1; } x }; eval(p,arm(bk),b) }
                _=>{ let mut bk=0usize; while bk<cnt-1 { if bb<th(bk){break;} bk+=1; } eval(p,arm(bk),b) }
            }
        }
        _=>0
    }
}
fn build(strats:&[u8;NARCH_CONST])->Prog{
    let mut bld=Bld{p:Prog{nodes:vec![],arms:vec![],th:vec![],roots:vec![]}};
    let input=bld.n(INPUT,0,0,0,0);
    let mut roots=vec![];
    roots.push(bld.match_arch(input,KEY4,4,strats[0],0));
    roots.push(bld.match_arch(input,KEY4,4,strats[1],1));
    roots.push(bld.match_arch(input,KEY8,8,strats[2],0));
    roots.push(bld.match_arch(input,KEY4,4,strats[3],2));
    roots.push(bld.ifchain_arch(input,4,strats[4],0));
    roots.push(bld.ifchain_arch(input,4,strats[5],1));
    roots.push(bld.ifchain_arch(input,4,strats[6],2));
    bld.p.roots=roots; bld.p
}
#[inline(always)] fn program(p:&Prog,b:u8)->u64{ let mut acc=0u64; for &r in &p.roots { acc^=eval(p,r,b); } acc }
use std::sync::OnceLock;
static PROG: OnceLock<Prog> = OnceLock::new();
'''

def variant(name, strats):
    body = INTERP.replace("DEFAULT_STRAT", str(DEFAULT)).replace("NARCH_CONST", str(NARCH))
    st = "[" + ",".join(str(s) for s in strats) + "]"
    lib = body + f'''
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let p = PROG.get_or_init(|| build(&{st}));
    timed! {{ run {{
        let mut acc: u64 = 0;
        for &b in input.iter() {{ acc ^= program(p, b); }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''
    cargo=f'''[workspace]
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

def sizes_block(bn, vs):
    o=[f'[bench.{bn}]']
    return o

def bench_sections():
    out=[]
    def addbench(bn, title, variants):
        out.append(f'[bench.{bn}]'); out.append(f'title = "{title}"')
        out.append('workload = "realistic"'); out.append('master_seed = 0x7654_3210_fedc_ba98')
        for n in [64,256,1024,4096,16384]:
            out.append(f'[[bench.{bn}.sizes]]'); out.append(f'n = {n}'); out.append(f'variants = [{", ".join(variants)}]')
        out.append('')
    # per-branch
    for j,(an,_) in enumerate(ARCH):
        vs=[]
        for sn,sv in STRATS.items():
            strats=[DEFAULT]*NARCH; strats[j]=sv; name=f"ab_b{j}_{sn}"; variant(name,strats)
            vs.append(f'"variants/{name}/target/release/{name}"')
        addbench(f"arch_b{j}", f"Per-branch strategy: archetype {j} ({an}), interp tier", vs)
    # per-type: all match, all ifchain
    for tname, idxs in [("match", MATCH_IDX), ("ifchain", IFCH_IDX)]:
        vs=[]
        for sn,sv in STRATS.items():
            strats=[DEFAULT]*NARCH
            for i in idxs: strats[i]=sv
            name=f"ab_{tname}_{sn}"; variant(name,strats); vs.append(f'"variants/{name}/target/release/{name}"')
        addbench(f"arch_{tname}", f"Per-type strategy: all {tname} branches one strategy, interp tier", vs)
    # whole
    vs=[]
    for sn,sv in STRATS.items():
        name=f"ab_whole_{sn}"; variant(name,[sv]*NARCH); vs.append(f'"variants/{name}/target/release/{name}"')
    addbench("arch_whole", "Whole-program single strategy (all branches), interp tier", vs)
    return "\n".join(out)

if __name__=="__main__":
    with open("bench.toml","a") as f: f.write("\n"+bench_sections())
    total=(NARCH+3)*len(STRATS)
    print(f"generated {total} archetype variants: {NARCH} per-branch + 2 per-type + 1 whole, x {len(STRATS)} strategies")
