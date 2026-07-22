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

/// The reassociation window for the eqsat pre-pass: the maximum number of
/// operands a single ADD/MUL expression is flattened across before the bound
/// keeps the rest as concrete subtrees. This is the no-alloc-style bound that
/// keeps the pass O(n * window); a wider expression is reassociated in
/// window-sized pieces. 64 is comfortably wider than any real reassociatable
/// motif in the profiles while keeping flat lists small.
pub const EQSAT_WINDOW: usize = 64;

/// Optimize a program with the chosen passes. `eqsat` runs a bounded
/// associativity/commutativity equality-saturation reassociation pre-pass (the
/// design's fourth optimize strategy, so the axis is none / CSE / eqsat /
/// CSE+eqsat), `fold` const-folds pure constant subtrees, `cse` deduplicates
/// structurally identical nodes, `dce` drops nodes not reachable from the
/// live-outs. eqsat runs first so fold/CSE/DCE see the reassociated form.
pub fn optimize(prog: &Program, cse: bool, fold: bool, dce: bool, eqsat: bool) -> Optimized {
    // eqsat reassociation pre-pass. It rewrites ADD/MUL modulo assoc+comm and
    // extracts a min-cost DAG that preserves the original live-out values, so the
    // downstream passes operate on the reassociated form. sinks(original) is
    // computed before the rewrite so the provenance of the live-outs survives it.
    let eq_holder;
    let (prog, base_sinks): (&Program, Vec<u32>) = if eqsat {
        let orig = sinks(prog);
        let (p, outs) = crate::eqsat::eqsat_reassociate(prog, &orig, EQSAT_WINDOW);
        eq_holder = p;
        (&eq_holder, outs)
    } else {
        (prog, sinks(prog))
    };
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

    // base_sinks are the live-outs of the (possibly eqsat-rewritten) program, in
    // the original sink order, so out_ids keep the original program's live-out
    // provenance through both eqsat and the CSE/fold/DCE renumbering.
    let mut out_ids: Vec<u32> = base_sinks.iter().map(|&s| map[s as usize]).collect();

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
        // (cse, fold, dce, eqsat): the design's optimize axis is none / CSE /
        // eqsat / CSE+eqsat, plus fold and dce and their compositions.
        let flags = [
            (true, false, false, false),  // CSE only
            (false, true, false, false),  // fold only
            (false, false, true, false),  // DCE only
            (false, false, false, true),  // eqsat only
            (true, false, false, true),   // CSE + eqsat
            (true, true, true, false),    // CSE + fold + DCE
            (true, true, true, true),     // everything
        ];
        for name in profiles {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 700;
            let prog = generate(&gp);
            let orig_sinks = sinks(&prog);
            for &(cse, fold, dce, eqsat) in &flags {
                let opt = optimize(&prog, cse, fold, dce, eqsat);
                assert!(
                    opt.prog.is_well_formed(),
                    "{name} ({cse},{fold},{dce},{eqsat}) ill-formed"
                );
                for seed in [0u64, 1, 42, 12345, 999_999] {
                    let a = out_fold(&prog, &orig_sinks, seed);
                    let b = out_fold(&opt.prog, &opt.out_ids, seed);
                    assert_eq!(
                        a, b,
                        "{name} ({cse},{fold},{dce},{eqsat}) output diverged at seed {seed}"
                    );
                }
                // CSE/fold/DCE are size-non-increasing. eqsat is a normalization,
                // not a shrink: on a DAG it can lose cross-consumer sharing and
                // transiently enlarge (the CSE+eqsat combo recovers it), so the
                // size guarantee only holds for the non-eqsat strategies. The
                // load-bearing invariant for eqsat is value preservation, asserted
                // in the seed loop above.
                if !eqsat {
                    assert!(opt.prog.nodes.len() <= prog.nodes.len());
                }
            }
        }
    }

    #[test]
    fn sink_count_varies_across_profiles() {
        // Fusion, liveness, and output-building all cost-scale with sink count,
        // and sink count is an emergent property of each profile's
        // locality_window, not a stated parameter: a small window (p_madd: 4,
        // p_tight: 8) leaves a band of recent nodes unreferenced, while
        // p_scatter's usize::MAX window lets any later node reference any earlier
        // one, structurally shrinking the sink set. Comparing a stage delta across
        // profiles without reporting each profile's sink count risks attributing a
        // locality-driven cardinality difference to the stage itself. This records
        // the per-profile counts (the number the bench report must carry) and
        // confirms the confound is real, not hypothetical: they genuinely differ.
        let profiles = ["real", "madd", "tight", "scatter", "wideselect", "leaf"];
        let counts: Vec<(&str, usize)> = profiles
            .iter()
            .map(|&name| {
                let mut gp = GenParams::profile(name).unwrap();
                gp.node_count = 4096;
                (name, sinks(&generate(&gp)).len())
            })
            .collect();
        for &(name, c) in &counts {
            assert!(c > 0, "{name} has no sinks");
        }
        let first = counts[0].1;
        assert!(
            counts.iter().any(|&(_, c)| c != first),
            "sink counts should vary across profiles (locality-driven confound): {counts:?}"
        );
    }

    #[test]
    fn interned_operands_gains_little_on_p_madd() {
        // The interned-operand exclusion (named in the fairness audit) claims
        // operand tuples do not repeat on this value-DAG, so interning (a, b)
        // pairs cannot win. p_madd is the profile built to stress that claim: a
        // correlated MUL/ADD stream over a 4-wide locality window, the case most
        // likely to reproduce operand tuples. Measure the actual distinct-(a, b)
        // ratio among binary-op nodes. Because the window slides with the node
        // index (lo = i - window), the ABSOLUTE (a, b) tuples an interner would
        // key on shift as the program advances even when relative positions
        // repeat, so distinct tuples stay near the binary-node count and interning
        // finds almost nothing to dedup. If this ratio ever drops, the exclusion
        // does not hold on p_madd and this test surfaces it.
        use std::collections::HashSet;
        let mut gp = GenParams::p_madd();
        gp.node_count = 4096;
        let prog = generate(&gp);
        let mut binops = 0usize;
        let mut distinct: HashSet<(u32, u32)> = HashSet::new();
        for node in &prog.nodes {
            if op::ARITY[node.op as usize] == 2 {
                binops += 1;
                distinct.insert((node.operands[0], node.operands[1]));
            }
        }
        assert!(binops > 100, "p_madd should have many binary ops, got {binops}");
        assert!(
            distinct.len() * 100 >= binops * 90,
            "operand tuples repeat more than expected on p_madd: {} distinct of {} binops ({}%); \
             the interned-operand exclusion may not hold on this profile",
            distinct.len(),
            binops,
            distinct.len() * 100 / binops
        );
    }
}
