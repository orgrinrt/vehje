//! The optimize build-half stage: constant folding, common-subexpression
//! elimination, and dead-code elimination on the IR program.
//!
//! A real runtime does not interpret the residual as authored; it folds
//! constants, deduplicates repeated subexpressions, and drops dead nodes while it
//! builds the in-memory form. This stage does the same as a `Program -> Program`
//! transform, so the downstream form/dispatch cells run the SMALLER optimized
//! program and the matrix can measure how a node-count-shrinking pre-pass moves
//! every downstream working-set crossover. Its cost is an honest part of the
//! setup term `S`.
//!
//! Correctness contract: the optimize axis changes the node count, so a
//! full-array checksum is not comparable across it. What is preserved is the
//! program's observable output: the values of its live-out (sink) nodes. The
//! transform returns the new ids of the original sinks so a consumer folds
//! exactly those, and that fold must equal the un-optimized program's sink fold
//! for every input (the exact property a sound rewrite has, and a strong test
//! that the pre-pass is correct).

use crate::ir::{op, Node, Program};
use std::collections::HashMap;

/// The optimized program plus the new ids of the original live-out (sink) nodes.
pub struct Optimized {
    pub prog: Program,
    pub out_ids: Vec<u32>,
}

/// Live-out nodes: those referenced by no later node. CONST operands are pool
/// indices, not node refs, so they never mark a node referenced.
pub fn sinks(prog: &Program) -> Vec<u32> {
    let n = prog.nodes.len();
    let mut referenced = vec![false; n];
    for node in &prog.nodes {
        if node.op != op::CONST {
            for &o in &node.operands {
                referenced[o as usize] = true;
            }
        }
    }
    (0..n as u32).filter(|&i| !referenced[i as usize]).collect()
}

/// Evaluate one op on constant operand values. MUST match `interp::interpret`'s
/// arms exactly, or const-folding would change the observable result.
fn eval_op(opcode: u8, a: &[u64]) -> u64 {
    match opcode {
        op::ADD => a[0].wrapping_add(a[1]),
        op::SUB => a[0].wrapping_sub(a[1]),
        op::MUL => a[0].wrapping_mul(a[1]),
        op::AND => a[0] & a[1],
        op::OR => a[0] | a[1],
        op::XOR => a[0] ^ a[1],
        op::SHL => a[0].wrapping_shl(a[1] as u32),
        op::SHR => a[0].wrapping_shr(a[1] as u32),
        op::MIN => a[0].min(a[1]),
        op::MAX => a[0].max(a[1]),
        op::EQ => (a[0] == a[1]) as u64,
        op::LT => (a[0] < a[1]) as u64,
        op::SELECT => {
            if a[0] != 0 {
                a[1]
            } else {
                a[2]
            }
        }
        op::NEG => a[0].wrapping_neg(),
        op::NOT => !a[0],
        _ => 0,
    }
}

/// Optimize a program with the chosen passes. `fold` const-folds pure constant
/// subtrees, `cse` deduplicates structurally identical nodes, `dce` drops nodes
/// not reachable from the live-outs.
pub fn optimize(prog: &Program, cse: bool, fold: bool, dce: bool) -> Optimized {
    let n = prog.nodes.len();

    // Phase 1: compute the compile-time constant value of each node where it has
    // one (CONST, or a pure op whose operands are all constant). INPUT and
    // anything reachable from it are not constant.
    let mut cval: Vec<Option<u64>> = vec![None; n];
    for i in 0..n {
        let node = &prog.nodes[i];
        match node.op {
            op::CONST => cval[i] = Some(prog.consts[node.operands[0] as usize]),
            op::INPUT => {}
            _ => {
                if fold {
                    let mut vals = Vec::with_capacity(node.operands.len());
                    let mut all_const = true;
                    for &o in &node.operands {
                        match cval[o as usize] {
                            Some(v) => vals.push(v),
                            None => {
                                all_const = false;
                                break;
                            }
                        }
                    }
                    if all_const {
                        cval[i] = Some(eval_op(node.op, &vals));
                    }
                }
            }
        }
    }

    // Phase 2: build the new program with CSE. Old nodes are processed in order
    // (children before parents), so remapped operands always point at already
    // emitted new nodes, keeping the new list children-before-parents.
    let mut new_nodes: Vec<Node> = Vec::with_capacity(n);
    let mut new_consts: Vec<u64> = Vec::new();
    let mut map: Vec<u32> = vec![0; n];
    let mut const_pool: HashMap<u64, u32> = HashMap::new();
    let mut cse_const: HashMap<u64, u32> = HashMap::new();
    let mut cse_op: HashMap<(u8, Vec<u32>), u32> = HashMap::new();
    let mut input_id: Option<u32> = None;

    // get-or-create a CONST node for value `v`.
    let mk_const = |v: u64,
                        new_nodes: &mut Vec<Node>,
                        new_consts: &mut Vec<u64>,
                        const_pool: &mut HashMap<u64, u32>,
                        cse_const: &mut HashMap<u64, u32>|
     -> u32 {
        if cse {
            if let Some(&id) = cse_const.get(&v) {
                return id;
            }
        }
        let pool_idx = *const_pool.entry(v).or_insert_with(|| {
            let idx = new_consts.len() as u32;
            new_consts.push(v);
            idx
        });
        let id = new_nodes.len() as u32;
        new_nodes.push(Node {
            op: op::CONST,
            operands: vec![pool_idx],
        });
        cse_const.insert(v, id);
        id
    };

    for i in 0..n {
        let node = &prog.nodes[i];
        let new_id = if let (true, Some(v)) = (fold, cval[i]) {
            // folded to a constant
            mk_const(v, &mut new_nodes, &mut new_consts, &mut const_pool, &mut cse_const)
        } else if node.op == op::CONST {
            let v = prog.consts[node.operands[0] as usize];
            mk_const(v, &mut new_nodes, &mut new_consts, &mut const_pool, &mut cse_const)
        } else if node.op == op::INPUT {
            if cse {
                if let Some(id) = input_id {
                    id
                } else {
                    let id = new_nodes.len() as u32;
                    new_nodes.push(Node { op: op::INPUT, operands: vec![] });
                    input_id = Some(id);
                    id
                }
            } else {
                let id = new_nodes.len() as u32;
                new_nodes.push(Node { op: op::INPUT, operands: vec![] });
                id
            }
        } else {
            let operands: Vec<u32> = node.operands.iter().map(|&o| map[o as usize]).collect();
            if cse {
                let key = (node.op, operands.clone());
                if let Some(&id) = cse_op.get(&key) {
                    id
                } else {
                    let id = new_nodes.len() as u32;
                    new_nodes.push(Node { op: node.op, operands });
                    cse_op.insert(key, id);
                    id
                }
            } else {
                let id = new_nodes.len() as u32;
                new_nodes.push(Node { op: node.op, operands });
                id
            }
        };
        map[i] = new_id;
    }

    let orig_sinks = sinks(prog);
    let mut out_ids: Vec<u32> = orig_sinks.iter().map(|&s| map[s as usize]).collect();

    let mut result = Program {
        consts: new_consts,
        nodes: new_nodes,
    };

    // Phase 3: DCE. Keep only nodes reachable from the live-outs, renumber.
    if dce {
        let m = result.nodes.len();
        let mut keep = vec![false; m];
        let mut stack: Vec<u32> = out_ids.clone();
        for &s in &out_ids {
            keep[s as usize] = true;
        }
        while let Some(x) = stack.pop() {
            let node = &result.nodes[x as usize];
            if node.op != op::CONST {
                for &o in &node.operands {
                    if !keep[o as usize] {
                        keep[o as usize] = true;
                        stack.push(o);
                    }
                }
            }
        }
        let mut remap = vec![0u32; m];
        let mut kept_nodes: Vec<Node> = Vec::new();
        for i in 0..m {
            if keep[i] {
                remap[i] = kept_nodes.len() as u32;
                let node = &result.nodes[i];
                let operands = if node.op == op::CONST || node.op == op::INPUT {
                    node.operands.clone()
                } else {
                    node.operands.iter().map(|&o| remap[o as usize]).collect()
                };
                kept_nodes.push(Node { op: node.op, operands });
            }
        }
        for id in out_ids.iter_mut() {
            *id = remap[*id as usize];
        }
        result.nodes = kept_nodes;
    }

    Optimized { prog: result, out_ids }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::checksum_at;
    use crate::ir::{encode, Decoded, REC24};
    use crate::{generate, GenParams};

    fn out_fold(prog: &Program, out_ids: &[u32], seed: u64) -> u64 {
        let bytes = encode(prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r = vec![0u64; prog.nodes.len().max(1)];
        crate::interp::interpret(&d, seed, &mut r);
        checksum_at(&r, out_ids)
    }

    #[test]
    fn optimize_preserves_outputs() {
        let profiles = ["real", "madd", "tight", "scatter", "wideselect", "leaf"];
        let flags = [
            (true, false, false),
            (false, true, false),
            (false, false, true),
            (true, true, true),
        ];
        for name in profiles {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 700;
            let prog = generate(&gp);
            let orig_sinks = sinks(&prog);
            for &(cse, fold, dce) in &flags {
                let opt = optimize(&prog, cse, fold, dce);
                assert!(opt.prog.is_well_formed(), "{name} ({cse},{fold},{dce}) ill-formed");
                for seed in [0u64, 1, 42, 12345, 999_999] {
                    let a = out_fold(&prog, &orig_sinks, seed);
                    let b = out_fold(&opt.prog, &opt.out_ids, seed);
                    assert_eq!(
                        a, b,
                        "{name} ({cse},{fold},{dce}) output diverged at seed {seed}"
                    );
                }
                // the optimized program is never larger than the original.
                assert!(opt.prog.nodes.len() <= prog.nodes.len());
            }
        }
    }
}
