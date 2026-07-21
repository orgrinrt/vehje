use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
#[derive(Clone, Copy)]
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
static PROG: OnceLock<(Vec<Node>, [u32;4], u32)> = OnceLock::new();
// REAL tree-walking interpreter: dispatch every node. Match handled by strategy `chain_rev`.
fn eval(nd: &[Node], a: &[u32;4], node: u32, b: u8) -> u64 {
    let n = nd[node as usize];
    match n.op {
        INPUT => b as u64,
        CONST => n.imm,
        MUL => eval(nd,a,n.a,b).wrapping_mul(eval(nd,a,n.b,b)),
        XOR => eval(nd,a,n.a,b) ^ eval(nd,a,n.b,b),
        ROTL => eval(nd,a,n.a,b).rotate_left(n.imm as u32),
        KEYOP => key_fn(b),
        MATCH => { let k = eval(nd,a,n.a,b); let arm = if k==3 {a[3]} else if k==2 {a[2]} else if k==1 {a[1]} else {a[0]}; eval(nd,a,arm,b) }
        _ => 0,
    }
}
// SHOWDOWN-IR interp tier: strategy `chain_rev`
#[bench_variant("ir_chain_rev_int", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let (nd, a, mroot) = PROG.get_or_init(build);
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() { acc ^= eval(nd, a, *mroot, b); }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
