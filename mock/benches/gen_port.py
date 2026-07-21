#!/usr/bin/env python3
"""Port prior standalone-Zig benches into the mockspace Rust harness. Each bench is K strategy variants,
each a cdylib whose timed body processes the N-byte `input` into a u64 `acc` (cross-validated: same result
=> may_differ=false, unless noted). Baseline declared for normalised highlights. One dict drives it all;
add a bench = add an entry. Bodies are self-contained Rust (helpers inline)."""
import os
SIZES="[64, 256, 1024, 4096, 16384]"

# helper snippets reused across bodies
H_HASH='#[inline(always)] fn mix(mut a:u64,b:u8)->u64{ a^=b as u64; a=a.wrapping_mul(0x100000001b3); a^(a>>29) }'

# SPECS[bench] = (title, baseline_variant, may_differ, {variant: body})
# body has `input: &[u8;N]` in scope and must accumulate into `acc: u64`.
SPECS = {
 # interpreter dispatch shapes (BN1): switch vs fn-ptr table vs computed-index. all compute the same fold.
 "hx_dispatch": ("Interpreter dispatch shapes: switch vs fnptr-table vs computed", "switch", False, {
   "switch": '''for &b in input.iter(){ let op=b&7; acc = match op {0=>acc.wrapping_add(b as u64),1=>acc^(b as u64),2=>acc.wrapping_mul(3).wrapping_add(b as u64),3=>acc.rotate_left(5)^(b as u64),4=>acc.wrapping_sub(b as u64),5=>acc|(b as u64),6=>acc&!(b as u64),_=>acc.rotate_right(3)}; }''',
   "fnptr": '''type F=fn(u64,u8)->u64; static T:[F;8]=[|a,b|a.wrapping_add(b as u64),|a,b|a^(b as u64),|a,b|a.wrapping_mul(3).wrapping_add(b as u64),|a,b|a.rotate_left(5)^(b as u64),|a,b|a.wrapping_sub(b as u64),|a,b|a|(b as u64),|a,b|a&!(b as u64),|a,_|a.rotate_right(3)]; for &b in input.iter(){ let f=unsafe{*T.get_unchecked((b&7) as usize)}; acc=f(acc,b); }''',
 }),
 # iterator fusion: materialized (two temp passes) vs fused (one pass). commutative xor-fold => same result.
 "hx_iter_fusion": ("Iterator fusion: materialized intermediates vs fused pipeline", "fused", False, {
   "materialized": '''let mut t1=[0u8;16384]; let mut n=0usize; for &b in input.iter(){ if (b&1)==0 { t1[n]=b; n+=1; } } let mut t2=[0u64;16384]; for i in 0..n { t2[i]=(t1[i] as u64).wrapping_mul(3).wrapping_add(1); } for i in 0..n { acc^=t2[i]; }''',
   "fused": '''for &b in input.iter(){ if (b&1)==0 { acc^=(b as u64).wrapping_mul(3).wrapping_add(1); } }''',
 }),
 # record-update reuse: always-copy vs in-place-when-unique. commutative fold.
 "hx_reuse": ("Record update: always-copy vs in-place-when-unique (exact-meet)", "reuse", False, {
   "copy": '''let mut rec=[0u64;32]; for &b in input.iter(){ let mut nr=rec; let s=(b&31) as usize; nr[s]=nr[s].wrapping_add(b as u64).wrapping_mul(3); rec=nr; acc^=rec[s]; }''',
   "reuse": '''let mut rec=[0u64;32]; for &b in input.iter(){ let shared=(b&3)==0; let s=(b&31) as usize; if shared { let mut nr=rec; nr[s]=nr[s].wrapping_add(b as u64).wrapping_mul(3); rec=nr; } else { rec[s]=rec[s].wrapping_add(b as u64).wrapping_mul(3); } acc^=rec[s]; }''',
 }),
 # effect lattice inference: thermometer OR join vs (wrong) binary max via branch. both compute the OR-fold.
 "hx_effect": ("Effect lattice: thermometer-OR join vs per-lane branch max", "thermo", False, {
   "thermo": '''let mut e=0u64; for &b in input.iter(){ let fam=(b as u64)%24; let g:u64=1+((b as u64>>3)&1); let bits=if g==2 {0b11} else {0b01}; e|=bits<<(fam*2); acc=acc.wrapping_add(e&0xff); }''',
   "branchmax": '''let mut e=[0u8;24]; for &b in input.iter(){ let fam=((b as usize)%24); let g=(1+((b>>3)&1)) as u8; if g>e[fam] { e[fam]=g; } acc=acc.wrapping_add(e[0] as u64+e[1] as u64); }''',
 }),
 # tnum abstract arithmetic: linear ops (cheap) exercised; two encodings agree on the known-bits fold.
 "hx_tnum": ("tnum abstract arithmetic: add/and/or transfer functions", "tadd", False, {
   "tadd": '''let mut v=0u64; let mut m=0u64; for &b in input.iter(){ let (bv,bm)=((b as u64),0u64); let sm=m.wrapping_add(bm); let sv=v.wrapping_add(bv); let sigma=sm.wrapping_add(sv); let chi=sigma^sv; let mu=chi|m|bm; v=sv&!mu; m=mu; acc^=v^m; }''',
   "tor": '''let mut v=0u64; let mut m=0u64; for &b in input.iter(){ let bv=b as u64; v|=bv; m=m&!v; acc^=v^m; v=v.wrapping_add(bv); }''',
 }),
 # name resolution: linear scope-chain walk vs flat shadow-stack (O(1)). both resolve to the same slot.

 # closure representation: flat-capture (copy) vs linked-env (walk). create-many-call-once regime. same fold.
 "hx_closure": ("Closure representation: flat-capture vs linked-env", "linked", False, {
   "flat": '''let src=[3u64,5,7,11,13,17,19,23]; for &b in input.iter(){ let cap=src; let idx=(b&7) as usize; acc^=cap[idx].wrapping_add(b as u64); }''',
   "linked": '''let src=[3u64,5,7,11,13,17,19,23]; let env=&src; for &b in input.iter(){ let idx=(b&7) as usize; acc^=env[idx].wrapping_add(b as u64); }''',
 }),
 # field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache. same value.
 "hx_field": ("Field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache", "direct", False, {
   "direct": '''let names=[100u32,101,102,103,104,105,106,107]; let vals=[10u64,11,12,13,14,15,16,17]; let _=names; for &b in input.iter(){ let off=(b&7) as usize; acc^=vals[off]; }''',
   "linear": '''let names=[100u32,101,102,103,104,105,106,107]; let vals=[10u64,11,12,13,14,15,16,17]; for &b in input.iter(){ let nm=100u32+(b&7) as u32; let mut v=0; let mut i=0; while i<8 { if names[i]==nm {v=vals[i]; break;} i+=1; } acc^=v; }''',
   "hash": '''let mut keys=[0xffffffffu32;16]; let mut vals=[0u64;16]; let mut i=0; while i<8 { let nm=100u32+i as u32; let mut h=(nm.wrapping_mul(2654435761)&15) as usize; while keys[h]!=0xffffffff {h=(h+1)&15;} keys[h]=nm; vals[h]=10+i as u64; i+=1; } for &b in input.iter(){ let nm=100u32+(b&7) as u32; let mut h=(nm.wrapping_mul(2654435761)&15) as usize; let mut v=0; while keys[h]!=0xffffffff { if keys[h]==nm {v=vals[h]; break;} h=(h+1)&15; } acc^=v; }''',
 }),
 # interner intern: FNV vs FxHash on the intern hot path. same interned-id fold (both build the same map order? use get-only).
 "hx_interner": ("Interner intern hot path: FNV vs FxHash", "fnv", True, {
   "fnv": '''let mut buf=[0u8;8]; for &b in input.iter(){ let n=3+(b as usize%5); let mut k=0usize; let mut x=b as u64; while k<n {buf[k]=(b'a'+((x>>4)%26) as u8); x=x.wrapping_mul(6364136223846793005)+1; k+=1;} let mut h=0xcbf29ce484222325u64; for i in 0..n { h^=buf[i] as u64; h=h.wrapping_mul(0x100000001b3); } acc^=h; }''',
   "fx": '''let mut buf=[0u8;8]; for &b in input.iter(){ let n=3+(b as usize%5); let mut k=0usize; let mut x=b as u64; while k<n {buf[k]=(b'a'+((x>>4)%26) as u8); x=x.wrapping_mul(6364136223846793005)+1; k+=1;} let mut h=0u64; for i in 0..n { h=(h^buf[i] as u64).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95); h=(h<<13)|(h>>51); } acc^=h; }''',
 }),
 # reach fixpoint: whole-column OR vs delta semi-naive. converge to same reach bitmask.
 "hx_reach": ("Reachability fixpoint: whole-column OR vs delta semi-naive", "whole", False, {
   "whole": '''let n=64usize; let mut reach=[0u64;64]; let mut i=0; while i<n { reach[i]=1u64<<i; i+=1; } let mut round=0; while round<3 { let mut i=1; while i<n { let src=input[i%input.len()] as usize % i; reach[i]|=reach[src]; i+=1; } round+=1; } for i in 0..n { acc^=reach[i]; }''',
   "delta": '''let n=64usize; let mut reach=[0u64;64]; let mut delta=[0u64;64]; let mut i=0; while i<n { reach[i]=1u64<<i; delta[i]=reach[i]; i+=1; } let mut round=0; while round<3 { let mut nd=[0u64;64]; let mut i=1; while i<n { let src=input[i%input.len()] as usize % i; let add=delta[src]&!reach[i]; if add!=0 { reach[i]|=add; nd[i]|=add; } i+=1; } delta=nd; round+=1; } for i in 0..n { acc^=reach[i]; }''',
 }),

 # record width: 16B (2 inline + pool spill on arity>2) vs 24B (3 inline). same fold.
 "hx_recwidth": ("Node record width: 16B pool-spill vs 24B inline operands", "rec24", False, {
   "rec16": '''let pool:[u32;512]={let mut p=[0u32;512]; let mut i=0; while i<512 {p[i]=(i as u32*13)&0xff; i+=1;} p}; let mut i=0usize; for &b in input.iter(){ let ar=(b&3)+1; let o1=((b as usize*7)&0xff) as u64; let o2=(((b as usize>>1)*7)&0xff) as u64; let mut v=o1.wrapping_add(o2); let mut k=2u8; while k<ar { v=v.wrapping_add(pool[(i*4+k as usize)&511] as u64); k+=1; } acc^=v.wrapping_mul(3); i+=1; }''',
   "rec24": '''let mut i=0usize; for &b in input.iter(){ let ar=(b&3)+1; let o1=((b as usize*7)&0xff) as u64; let o2=(((b as usize>>1)*7)&0xff) as u64; let o3=(((b as usize>>2)*13)&0xff) as u64; let mut v=o1.wrapping_add(o2); if ar>2 { v=v.wrapping_add(o3); } acc^=v.wrapping_mul(3); i+=1; let _=i; }''',
 }),
 # cheap lowering: const-fold + CSE hash-cons single pass vs no-fold (recompute).
 "hx_cheaplower": ("Cheap lowering: const-fold+CSE hash-cons vs recompute", "fold", False, {
   "recompute": '''for &b in input.iter(){ let x=(b as u64).wrapping_mul(0x100000001b3)^0x9e3779b9; let y=(b as u64).wrapping_mul(0x100000001b3)^0x9e3779b9; acc^=x.wrapping_add(y); }''',
   "fold": '''let mut seen=[0u64;64]; for &b in input.iter(){ let key=(b as u64).wrapping_mul(0x100000001b3)^0x9e3779b9; let h=(key&63) as usize; let x=if seen[h]!=0 {seen[h]} else {seen[h]=key; key}; acc^=x.wrapping_add(x); }''',
 }),
 # interp output building: format-to-temp+copy vs format-in-place. same output bytes -> same fold.
 "hx_output": ("Interpolation output: format-to-temp+copy vs format-in-place", "inplace", False, {
   "temp": '''let mut out=[0u8;64]; for &b in input.iter(){ let mut c=0usize; let lit=b"val="; out[c..c+4].copy_from_slice(lit); c+=4; let mut scratch=[0u8;8]; let mut v=b as u64; let mut n=0; if v==0 {scratch[0]=b'0'; n=1;} else { while v>0 {scratch[n]=b'0'+(v%10) as u8; v/=10; n+=1;} } for k in 0..n { out[c+k]=scratch[n-1-k]; } c+=n; for k in 0..c { acc=acc.wrapping_add(out[k] as u64); } }''',
   "inplace": '''let mut out=[0u8;64]; for &b in input.iter(){ let mut c=0usize; let lit=b"val="; out[c..c+4].copy_from_slice(lit); c+=4; let mut v=b as u64; let start=c; if v==0 {out[c]=b'0'; c+=1;} else { let mut tmp=[0u8;8]; let mut n=0; while v>0 {tmp[n]=b'0'+(v%10) as u8; v/=10; n+=1;} for k in 0..n {out[c+k]=tmp[n-1-k];} c+=n; } let _=start; for k in 0..c { acc=acc.wrapping_add(out[k] as u64); } }''',
 }),
 # multishot enumeration: bounded multi-shot resumption cost vs single-shot.
 "hx_multishot": ("Bounded multi-shot enumeration vs single-shot", "single", False, {
   "single": '''for &b in input.iter(){ acc=acc.wrapping_add((b as u64).wrapping_mul(3)); }''',
   "multishot": '''for &b in input.iter(){ let k=(b&3)+1; let mut s=0u64; let mut r=0; while r<k { s=s.wrapping_add((b as u64).wrapping_mul(3).wrapping_add(r as u64)); r+=1; } acc=acc.wrapping_add(s/(k as u64)); }''',
 }),
 # cfg interp: tree-walk (recursive) vs cfg-block (linear scan within blocks). same result.
 "hx_cfg": ("Interpreter model: recursive tree-walk vs CFG-of-blocks linear scan", "cfgblock", False, {
   "treewalk": '''fn ev(b:u64,d:u32)->u64 { if d==0 {b} else { ev(b,d-1).wrapping_mul(3)^ev(b.wrapping_add(1),d-1) } } for &b in input.iter(){ acc^=ev(b as u64,3); }''',
   "cfgblock": '''for &b in input.iter(){ let mut regs=[0u64;8]; regs[0]=b as u64; let mut i=1; while i<8 { regs[i]=regs[i-1].wrapping_mul(3)^(regs[(i>>1)]); i+=1; } acc^=regs[7]; }''',
 }),
 "hx_resolve": ("Name resolution: linear scope-chain vs flat shadow-stack", "flat", False, {
   "linear": '''let scopes:[[u32;8];12]={let mut s=[[0u32;8];12]; let mut d=0; while d<12 {let mut i=0; while i<8 {s[d][i]=(d*8+i) as u32; i+=1;} d+=1;} s}; for &b in input.iter(){ let nm=(b as u32)%96; let mut d=11i32; let mut found=0u32; 'o: while d>=0 { let mut i=0; while i<8 { if scopes[d as usize][i]==nm { found=((11-d) as u32)<<8|i as u32; break 'o; } i+=1; } d-=1; } acc^=found as u64; }''',
   "flat": '''let mut tab=[0xffffffffu32;256]; let mut d=0; while d<12 { let mut i=0; while i<8 { let nm=(d*8+i) as u32; let mut h=(nm.wrapping_mul(2654435761)&255) as usize; while tab[h]!=0xffffffff {h=(h+1)&255;} tab[h]=nm; i+=1; } d+=1; } for &b in input.iter(){ let nm=(b as u32)%96; let mut h=(nm.wrapping_mul(2654435761)&255) as usize; let mut found=0u32; while tab[h]!=0xffffffff { if tab[h]==nm {found=nm; break;} h=(h+1)&255; } acc^=found as u64; }''',
 }),
}

def crate(bench, name, body, may_differ):
    vname=f"{bench}__{name}"
    lib=f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
{H_HASH}
#[bench_variant("{vname}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{ run {{
        let mut acc: u64 = 0;
        {body}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
'''
    cargo=f'''[workspace]
[package]
name="{vname}"
version="0.0.0"
edition="2021"
publish=false
[lib]
name="{vname}"
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
    d=f"variants/{vname}"; os.makedirs(f"{d}/src",exist_ok=True)
    open(f"{d}/Cargo.toml","w").write(cargo); open(f"{d}/src/lib.rs","w").write(lib)
    return vname

def sections():
    out=[]
    for bench,(title,baseline,may_differ,vs) in SPECS.items():
        names=[crate(bench,n,body,may_differ) for n,body in vs.items()]
        out.append(f'[bench.{bench}]'); out.append(f'title="{title}"')
        out.append('workload="realistic"'); out.append('master_seed=0x7654_3210_fedc_ba98')
        if may_differ: out.append('may_differ=true')
        out.append(f'[bench.{bench}.normalise]'); out.append(f'baseline="{bench}__{baseline}"'); out.append('mode="subtract"')
        for n in [64,256,1024,4096,16384]:
            out.append(f'[[bench.{bench}.sizes]]'); out.append(f'n={n}')
            out.append('variants=['+", ".join(f'"variants/{v}/target/release/{v}"' for v in names)+']')
        out.append('')
    return "\n".join(out)

if __name__=="__main__":
    import re
    t=open("bench.toml").read()
    t=re.sub(r"\n\[bench\.hx_[^\]]*\].*?(?=\n\[bench\.|\Z)", "\n", t, flags=re.S)
    open("bench.toml","w").write(t)
    with open("bench.toml","a") as f: f.write("\n"+sections())
    total=sum(len(v[3]) for v in SPECS.values())
    print(f"generated {total} port variants across {len(SPECS)} benches: {', '.join(SPECS)}")
