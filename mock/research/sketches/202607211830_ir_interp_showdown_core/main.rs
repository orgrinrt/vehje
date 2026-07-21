// Core for the per-branch strategy showdown: ONE shared IR program with several branch ARCHETYPES.
// Each branch node carries a `strat` field; the interpreter dispatches it. Variants differ ONLY in the IR
// data (which branch uses which strategy), so we run the SAME program and, by varying one branch's strat
// while holding the rest fixed, isolate that branch's contribution -> best strategy per archetype.
#[derive(Clone, Copy)]
struct Node { op: u8, a: u32, imm: u64, aux: u32, strat: u8 }
// ops
const INPUT:u8=0; const CONST:u8=1; const MUL:u8=2; const XOR:u8=3; const ROTL:u8=4; const ADD:u8=5;
const KEY4:u8=6; const KEY8:u8=7; const MATCH:u8=8; const CMP:u8=9; const IF:u8=10;
// universal strategies
const SEQ:u8=0; const PRED:u8=1; const TABLE:u8=2; const TREE:u8=3; const PROF:u8=4;

struct Prog { nodes: Vec<Node>, arms: Vec<u32>, roots: Vec<u32> } // roots = the archetype branch roots

fn key4(b:u8)->u64 { if b<179 {2} else {[0u64,1,3][(b as usize)%3]} }
fn key8(b:u8)->u64 { (b as u64)&7 }

struct B<'a>{ p:&'a mut Prog }
impl<'a> B<'a>{
    fn n(&mut self, op:u8,a:u32,imm:u64,aux:u32,strat:u8)->u32{ let id=self.p.nodes.len() as u32; self.p.nodes.push(Node{op,a,imm,aux,strat}); id }
    // a linear arm: rotl(xor(mul(input,P),Q), s)
    fn arm_linear(&mut self, input:u32, i:u64)->u32{
        let p=self.n(CONST,0,0x100000001b3u64.wrapping_mul(i+1),0,0);
        let m=self.n(MUL,input,0,0,0); // a=input, imm unused; store second operand in aux
        // encode binary op operands: a=left, aux=right
        self.p.nodes[m as usize].aux=p;
        let q=self.n(CONST,0,0x9e3779b97f4a7c15u64.wrapping_add(i),0,0);
        let x=self.n(XOR,m,0,q,0);
        self.n(ROTL,x,(i%13)+1,0,0)
    }
    // a match branch over K arms with given key op and strat
    fn match_branch(&mut self, input:u32, keyop:u8, k:usize, strat:u8, arm_builder: &dyn Fn(&mut B, u32, u64)->u32)->u32{
        let key=self.n(keyop,input,0,0,0);
        let base=self.p.arms.len() as u32;
        for i in 0..k { let a=arm_builder(self, input, i as u64); self.p.arms.push(a); }
        self.n(MATCH, key, 0, base | ((k as u32)<<24), strat)
    }
}

fn eval(p:&Prog, node:u32, b:u8)->u64{
    let n=p.nodes[node as usize];
    match n.op{
        INPUT=>b as u64, CONST=>n.imm,
        MUL=>eval(p,n.a,b).wrapping_mul(eval(p,n.aux,b)),
        XOR=>eval(p,n.a,b)^eval(p,n.aux,b),
        ADD=>eval(p,n.a,b).wrapping_add(eval(p,n.aux,b)),
        ROTL=>eval(p,n.a,b).rotate_left(n.imm as u32),
        KEY4=>key4(b), KEY8=>key8(b),
        MATCH=>{
            let k=eval(p,n.a,b);
            let base=(n.aux&0xffffff) as usize; let cnt=(n.aux>>24) as usize;
            let arm=|i:usize| p.arms[base+i];
            match n.strat{
                PRED=>{ let mut acc=0u64; for i in 0..cnt { let v=eval(p,arm(i),b); if i==k as usize {acc=v;} } acc }
                TABLE=>eval(p,arm(k as usize),b),
                TREE=>{ // binary search over cnt
                    let mut lo=0usize; let mut hi=cnt;
                    while hi-lo>1 { let mid=(lo+hi)/2; if (k as usize)<mid {hi=mid} else {lo=mid} }
                    eval(p,arm(lo),b)
                }
                PROF=>{ // hot-first: test hot arm (2) first, else linear scan. always resolves to arm(k).
                    let chosen = if k as usize==2 && cnt>2 { arm(2) }
                        else { let mut c=arm(cnt-1); for i in 0..cnt { if i==k as usize { c=arm(i); break; } } c };
                    eval(p,chosen,b)
                }
                _=>{ // SEQ: linear if-chain
                    let mut chosen=arm(cnt-1);
                    for i in 0..cnt { if i==k as usize { chosen=arm(i); break; } }
                    eval(p,chosen,b)
                }
            }
        }
        _=>0
    }
}

fn build(strats:&[u8])->Prog{
    let mut p=Prog{nodes:vec![],arms:vec![],roots:vec![]};
    let mut b=B{p:&mut p};
    let input=b.n(INPUT,0,0,0,0);
    let lin=|bb:&mut B,inp:u32,i:u64| bb.arm_linear(inp,i);
    // archetype branches (strats[i] picks each branch's strategy)
    let r0=b.match_branch(input,KEY4,4,strats[0],&lin); // match4 linear
    let r1=b.match_branch(input,KEY8,8,strats[1],&lin); // match8 linear
    let r2=b.match_branch(input,KEY4,4,strats[2],&lin); // another match4 (stand-in for a different archetype)
    b.p.roots=vec![r0,r1,r2];
    p
}
fn program(p:&Prog,b:u8)->u64{ let mut acc=0u64; for &r in &p.roots { acc^=eval(p,r,b); } acc }

fn main(){
    // validate: any strat assignment gives the same result (strategies are semantically identical per branch)
    let assigns:[[u8;3];5]=[[SEQ,SEQ,SEQ],[PRED,PRED,PRED],[TABLE,TABLE,TABLE],[TREE,TREE,TREE],[PROF,PROF,PROF]];
    let progs:Vec<Prog>=assigns.iter().map(|a|build(a)).collect();
    let mut s=0x1234u64; let mut ok=true;
    for _ in 0..3000{ s=s.wrapping_mul(6364136223846793005).wrapping_add(1); let b=(s>>40) as u8;
        let r0=program(&progs[0],b);
        for p in &progs[1..]{ if program(p,b)!=r0 { ok=false; break; } }
        if !ok {break;}
    }
    println!("{}", if ok {"all strategies agree across all branches => per-branch strat swapping is semantically safe"} else {"MISMATCH"});
    println!("shared program: {} nodes, {} arms, {} archetype branches", progs[0].nodes.len(), progs[0].arms.len(), progs[0].roots.len());
}
