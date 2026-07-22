//! Value representation: static/raw vs runtime-tagged vs NaN-boxed.
//!
//! The fourth carrier variant axis the audit named (after dispatch shape,
//! record layout, and block structure). It maps to a real vehje question: the
//! graded type system can prove a value's type at compile time, so the runtime
//! could carry values RAW (no tag), the static-typing win. If instead the
//! runtime must support dynamically-typed values, each value needs a runtime
//! type tag and each op pays to read and dispatch on it. This measures that
//! price directly: the same mixed integer/float program interpreted three ways.
//!
//! - static: values are raw `u64` bits; the node's type is known statically
//!   (from the program), so ops pick int-vs-float behaviour without a per-value
//!   tag. This is the statically-typed runtime.
//! - tagged: values are `(tag, bits)` pairs; every op reads the operand tags at
//!   run time to dispatch. This is the classic dynamically-typed tagged runtime.
//! - nanbox: values are NaN-boxed `u64` (an i32 lives in a quiet-NaN payload, a
//!   double is its own bits); every op checks the box and unboxes. The packed
//!   dynamic representation JavaScript engines use.
//!
//! Scope, stated honestly. The program mixes int and float nodes, but each op
//! site is MONOMORPHIC: an IADD always takes int operands, an FADD always float
//! (the generator types them so). So the tag/box branches are predictable per
//! site, and this measures the interpreter-tier cost of CARRYING and reading a
//! representation (tag storage, box/unbox), NOT the megamorphic misprediction
//! cost that hurts dynamic typing at a polymorphic site. That misprediction cost
//! is a compiled/JIT-tier concern (specializing a site to its observed types); a
//! pure interpreter branches per node either way, so it cannot exhibit the
//! static-specialization win, which is why this bench is correctly a
//! representation-cost measurement, not a static-vs-dynamic-dispatch one.
//!
//! All three interpreters compute the identical logical values and fold the
//! identical checksum, so any measured gap is the representation and nothing
//! else.

use crate::gen::Rng;

pub mod op {
    pub const INPUT: u8 = 0; // int, from the FFI seed
    pub const CONST_INT: u8 = 1;
    pub const CONST_FLT: u8 = 2;
    pub const IADD: u8 = 3;
    pub const IMUL: u8 = 4;
    pub const FADD: u8 = 5;
    pub const FMUL: u8 = 6;
    pub const I2F: u8 = 7;
    pub const F2I: u8 = 8;
}

pub const TY_INT: u8 = 0;
pub const TY_FLT: u8 = 1;

#[derive(Clone, Copy)]
pub struct VNode {
    pub op: u8,
    pub ty: u8,
    pub a: u32,
    pub b: u32,
    pub imm: u64, // const payload: i32 bits for CONST_INT, f64 bits for CONST_FLT
}

/// Generate a valid typed DAG of `n` nodes mixing int and float work. Children
/// precede parents; binary ops take two same-typed operands; conversions bridge.
pub fn gen_valprog(n: usize, seed: u64) -> Vec<VNode> {
    let mut rng = Rng::new(seed);
    let mut nodes: Vec<VNode> = Vec::with_capacity(n);
    let mut ints: Vec<u32> = Vec::new();
    let mut flts: Vec<u32> = Vec::new();
    // node 0: INPUT (int)
    nodes.push(VNode { op: op::INPUT, ty: TY_INT, a: 0, b: 0, imm: 0 });
    ints.push(0);
    for i in 1..n as u32 {
        let r = rng.next_u64();
        let choice = r % 8;
        let node = match choice {
            0 => {
                let v = (rng.next_u64() as i32) | 1;
                ints.push(i);
                VNode { op: op::CONST_INT, ty: TY_INT, a: 0, b: 0, imm: v as u32 as u64 }
            }
            1 => {
                let v = ((rng.next_u64() as i32 as f64) * 0.5 + 1.0).abs() + 1.0;
                flts.push(i);
                VNode { op: op::CONST_FLT, ty: TY_FLT, a: 0, b: 0, imm: v.to_bits() }
            }
            2 | 3 if ints.len() >= 2 => {
                let a = ints[(rng.next_u64() as usize) % ints.len()];
                let b = ints[(rng.next_u64() as usize) % ints.len()];
                let o = if choice == 2 { op::IADD } else { op::IMUL };
                ints.push(i);
                VNode { op: o, ty: TY_INT, a, b, imm: 0 }
            }
            4 | 5 if flts.len() >= 2 => {
                let a = flts[(rng.next_u64() as usize) % flts.len()];
                let b = flts[(rng.next_u64() as usize) % flts.len()];
                let o = if choice == 4 { op::FADD } else { op::FMUL };
                flts.push(i);
                VNode { op: o, ty: TY_FLT, a, b, imm: 0 }
            }
            6 if !ints.is_empty() => {
                let a = ints[(rng.next_u64() as usize) % ints.len()];
                flts.push(i);
                VNode { op: op::I2F, ty: TY_FLT, a, b: 0, imm: 0 }
            }
            _ if !flts.is_empty() => {
                let a = flts[(rng.next_u64() as usize) % flts.len()];
                ints.push(i);
                VNode { op: op::F2I, ty: TY_INT, a, b: 0, imm: 0 }
            }
            _ => {
                // fallback: an int const, always valid
                let v = (rng.next_u64() as i32) | 1;
                ints.push(i);
                VNode { op: op::CONST_INT, ty: TY_INT, a: 0, b: 0, imm: v as u32 as u64 }
            }
        };
        nodes.push(node);
    }
    nodes
}

// Canonical fold: int nodes contribute their i32 (sign-extended), float nodes
// their f64 bits. Every interpreter produces the same canonical value per node.
#[inline]
fn fold(acc: u64, canon: u64) -> u64 {
    acc.rotate_left(7) ^ canon
}

/// static/raw: values are bare u64 bits, types known statically per node.
pub fn interp_static(prog: &[VNode], input_seed: i32, scratch: &mut [u64]) -> u64 {
    let mut acc = 0u64;
    for (i, nd) in prog.iter().enumerate() {
        let v: u64 = match nd.op {
            op::INPUT => input_seed as u32 as u64,
            op::CONST_INT => nd.imm,
            op::CONST_FLT => nd.imm,
            op::IADD => {
                let (x, y) = (scratch[nd.a as usize] as u32 as i32, scratch[nd.b as usize] as u32 as i32);
                x.wrapping_add(y) as u32 as u64
            }
            op::IMUL => {
                let (x, y) = (scratch[nd.a as usize] as u32 as i32, scratch[nd.b as usize] as u32 as i32);
                x.wrapping_mul(y) as u32 as u64
            }
            op::FADD => (f64::from_bits(scratch[nd.a as usize]) + f64::from_bits(scratch[nd.b as usize])).to_bits(),
            op::FMUL => (f64::from_bits(scratch[nd.a as usize]) * f64::from_bits(scratch[nd.b as usize])).to_bits(),
            op::I2F => ((scratch[nd.a as usize] as u32 as i32) as f64).to_bits(),
            _ => (f64::from_bits(scratch[nd.a as usize]) as i32) as u32 as u64,
        };
        scratch[i] = v;
        let canon = if nd.ty == TY_INT { (v as u32 as i32) as i64 as u64 } else { v };
        acc = fold(acc, canon);
    }
    acc
}

/// tagged: values are (tag, bits); ops read operand tags at run time.
pub fn interp_tagged(prog: &[VNode], input_seed: i32, tags: &mut [u8], bits: &mut [u64]) -> u64 {
    let mut acc = 0u64;
    for (i, nd) in prog.iter().enumerate() {
        let (t, v): (u8, u64) = match nd.op {
            op::INPUT => (TY_INT, input_seed as u32 as u64),
            op::CONST_INT => (TY_INT, nd.imm),
            op::CONST_FLT => (TY_FLT, nd.imm),
            op::IADD | op::IMUL => {
                // dynamic dispatch on the operand tags (both must be int here).
                let (ta, tb) = (tags[nd.a as usize], tags[nd.b as usize]);
                let x = if ta == TY_INT { bits[nd.a as usize] as u32 as i32 } else { f64::from_bits(bits[nd.a as usize]) as i32 };
                let y = if tb == TY_INT { bits[nd.b as usize] as u32 as i32 } else { f64::from_bits(bits[nd.b as usize]) as i32 };
                let r = if nd.op == op::IADD { x.wrapping_add(y) } else { x.wrapping_mul(y) };
                (TY_INT, r as u32 as u64)
            }
            op::FADD | op::FMUL => {
                let (ta, tb) = (tags[nd.a as usize], tags[nd.b as usize]);
                let x = if ta == TY_FLT { f64::from_bits(bits[nd.a as usize]) } else { (bits[nd.a as usize] as u32 as i32) as f64 };
                let y = if tb == TY_FLT { f64::from_bits(bits[nd.b as usize]) } else { (bits[nd.b as usize] as u32 as i32) as f64 };
                let r = if nd.op == op::FADD { x + y } else { x * y };
                (TY_FLT, r.to_bits())
            }
            op::I2F => {
                let x = if tags[nd.a as usize] == TY_INT { bits[nd.a as usize] as u32 as i32 } else { f64::from_bits(bits[nd.a as usize]) as i32 };
                (TY_FLT, (x as f64).to_bits())
            }
            _ => {
                let x = if tags[nd.a as usize] == TY_FLT { f64::from_bits(bits[nd.a as usize]) } else { (bits[nd.a as usize] as u32 as i32) as f64 };
                (TY_INT, (x as i32) as u32 as u64)
            }
        };
        tags[i] = t;
        bits[i] = v;
        let canon = if t == TY_INT { (v as u32 as i32) as i64 as u64 } else { v };
        acc = fold(acc, canon);
    }
    acc
}

const BOX_TAG: u64 = 0xFFFC_0000_0000_0000;
#[inline]
fn box_int(i: i32) -> u64 {
    BOX_TAG | (i as u32 as u64)
}
#[inline]
fn is_boxed(v: u64) -> bool {
    (v >> 48) == 0xFFFC
}
#[inline]
fn unbox_int(v: u64) -> i32 {
    v as u32 as i32
}

/// nanbox: an i32 lives in a quiet-NaN payload; a double is its own bits. Every
/// op checks the box and unboxes. Finite doubles never collide with the box tag.
pub fn interp_nanbox(prog: &[VNode], input_seed: i32, vals: &mut [u64]) -> u64 {
    let mut acc = 0u64;
    for (i, nd) in prog.iter().enumerate() {
        let v: u64 = match nd.op {
            op::INPUT => box_int(input_seed),
            op::CONST_INT => box_int(nd.imm as u32 as i32),
            op::CONST_FLT => nd.imm, // a double, stored raw
            op::IADD | op::IMUL => {
                let va = vals[nd.a as usize];
                let vb = vals[nd.b as usize];
                let x = if is_boxed(va) { unbox_int(va) } else { f64::from_bits(va) as i32 };
                let y = if is_boxed(vb) { unbox_int(vb) } else { f64::from_bits(vb) as i32 };
                let r = if nd.op == op::IADD { x.wrapping_add(y) } else { x.wrapping_mul(y) };
                box_int(r)
            }
            op::FADD | op::FMUL => {
                let va = vals[nd.a as usize];
                let vb = vals[nd.b as usize];
                let x = if is_boxed(va) { unbox_int(va) as f64 } else { f64::from_bits(va) };
                let y = if is_boxed(vb) { unbox_int(vb) as f64 } else { f64::from_bits(vb) };
                let r = if nd.op == op::FADD { x + y } else { x * y };
                r.to_bits()
            }
            op::I2F => {
                let va = vals[nd.a as usize];
                let x = if is_boxed(va) { unbox_int(va) } else { f64::from_bits(va) as i32 };
                (x as f64).to_bits()
            }
            _ => {
                let va = vals[nd.a as usize];
                let x = if is_boxed(va) { unbox_int(va) as f64 } else { f64::from_bits(va) };
                box_int(x as i32)
            }
        };
        vals[i] = v;
        let canon = if nd.ty == TY_INT {
            let iv = if is_boxed(v) { unbox_int(v) } else { f64::from_bits(v) as i32 };
            (iv as i64) as u64
        } else {
            v
        };
        acc = fold(acc, canon);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_representations_agree() {
        for seed in [0u64, 1, 42, 12345, 999] {
            let prog = gen_valprog(500, 0x7a1 ^ seed);
            for input in [0i32, 7, -3, 100000] {
                let mut sc = vec![0u64; prog.len()];
                let s = interp_static(&prog, input, &mut sc);
                let mut tg = vec![0u8; prog.len()];
                let mut bt = vec![0u64; prog.len()];
                let t = interp_tagged(&prog, input, &mut tg, &mut bt);
                let mut nb = vec![0u64; prog.len()];
                let n = interp_nanbox(&prog, input, &mut nb);
                assert_eq!(s, t, "static vs tagged differ (seed {seed}, input {input})");
                assert_eq!(s, n, "static vs nanbox differ (seed {seed}, input {input})");
            }
        }
    }

    #[test]
    fn program_mixes_types() {
        // The workload must actually mix int and float nodes, else the tag is a
        // predictable constant and the cost is understated.
        let prog = gen_valprog(1000, 0x5);
        let ints = prog.iter().filter(|n| n.ty == TY_INT).count();
        let flts = prog.iter().filter(|n| n.ty == TY_FLT).count();
        assert!(ints > 100 && flts > 100, "workload not mixed: {ints} int, {flts} flt");
    }
}
