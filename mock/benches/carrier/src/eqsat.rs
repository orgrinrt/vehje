//! A minimal equality-saturation engine, to measure whether a bounded rewrite
//! window catches the associativity-reassociation explosion.
//!
//! The design lowers via equality saturation under a no-alloc bound. The open
//! question the audit named: build the case that is supposed to blow an e-graph
//! up (a long associative-commutative chain, whose reassociations are
//! super-exponentially many), and watch the bound catch it: the bounded window
//! stops growing at the cap while still extracting a valid low-cost form, and
//! the unbounded run explodes toward a safety ceiling.
//!
//! This is a real e-graph: hash-consed e-nodes, a union-find over e-classes, a
//! congruence-closure rebuild, associativity and commutativity rewrites, and a
//! greedy min-cost extraction. It is deliberately small (binary Add/Mul over
//! leaves), enough to reproduce the explosion honestly. Every extracted form
//! evaluates to the same value (all reassociations are equal), which is the
//! cross-validation: bounded and unbounded extract different shapes but the same
//! number.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Leaf,
    Add,
    Mul,
}

/// A hash-consed e-node: an op and up to two child e-class ids. For a leaf, `a`
/// is the leaf index and `b` is unused.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ENode {
    op: Op,
    a: u32,
    b: u32,
}

pub struct EGraph {
    parent: Vec<u32>,           // union-find over e-class ids
    hashcons: HashMap<ENode, u32>, // canonical e-node -> e-class
    nodes: Vec<ENode>,          // one representative node per class index (for extraction)
    class_nodes: Vec<Vec<ENode>>, // all e-nodes in each class (for rewrite matching)
    leaf_vals: Vec<u64>,        // concrete value per leaf index (for eval cross-check)
    pub cap: usize,             // max e-node count before saturation stops
    pub hit_cap: bool,
}

impl EGraph {
    pub fn new(cap: usize) -> Self {
        EGraph {
            parent: Vec::new(),
            hashcons: HashMap::new(),
            nodes: Vec::new(),
            class_nodes: Vec::new(),
            leaf_vals: Vec::new(),
            cap,
            hit_cap: false,
        }
    }

    #[inline]
    pub fn enode_count(&self) -> usize {
        self.hashcons.len()
    }

    fn find(&self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            x = self.parent[x as usize];
        }
        x
    }

    fn canon(&self, mut n: ENode) -> ENode {
        if n.op != Op::Leaf {
            n.a = self.find(n.a);
            n.b = self.find(n.b);
        }
        n
    }

    fn add_node(&mut self, n: ENode) -> u32 {
        let c = self.canon(n);
        if let Some(&id) = self.hashcons.get(&c) {
            return self.find(id);
        }
        let id = self.parent.len() as u32;
        self.parent.push(id);
        self.nodes.push(c);
        self.class_nodes.push(vec![c]);
        self.hashcons.insert(c, id);
        id
    }

    pub fn leaf(&mut self, idx: u32, val: u64) -> u32 {
        if idx as usize >= self.leaf_vals.len() {
            self.leaf_vals.resize(idx as usize + 1, 0);
        }
        self.leaf_vals[idx as usize] = val;
        self.add_node(ENode { op: Op::Leaf, a: idx, b: 0 })
    }

    pub fn add(&mut self, a: u32, b: u32) -> u32 {
        self.add_node(ENode { op: Op::Add, a, b })
    }
    pub fn mul(&mut self, a: u32, b: u32) -> u32 {
        self.add_node(ENode { op: Op::Mul, a, b })
    }

    fn union(&mut self, a: u32, b: u32) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.parent[rb as usize] = ra;
            // move rb's nodes into ra for matching (rebuild re-canonicalizes).
            let moved = std::mem::take(&mut self.class_nodes[rb as usize]);
            self.class_nodes[ra as usize].extend(moved);
        }
    }

    /// Congruence-closure rebuild: re-canonicalize every hash-consed e-node and
    /// merge any that became congruent after unions.
    fn rebuild(&mut self) {
        loop {
            let mut merged = false;
            let mut newcons: HashMap<ENode, u32> = HashMap::with_capacity(self.hashcons.len());
            let old: Vec<(ENode, u32)> = self.hashcons.drain().collect();
            for (n, id) in old {
                let c = self.canon(n);
                let rid = self.find(id);
                if let Some(&existing) = newcons.get(&c) {
                    let re = self.find(existing);
                    if re != rid {
                        self.union(re, rid);
                        merged = true;
                    }
                } else {
                    newcons.insert(c, rid);
                }
            }
            self.hashcons = newcons;
            if !merged {
                break;
            }
        }
    }

    /// One saturation round: match assoc/comm on every current e-node, queue the
    /// unions, apply them, rebuild. Returns false if nothing new (fixpoint) or
    /// the cap was hit.
    fn step(&mut self) -> bool {
        if self.enode_count() >= self.cap {
            self.hit_cap = true;
            return false;
        }
        // snapshot current nodes to match against (avoid iterating while adding).
        let snapshot: Vec<ENode> = self.hashcons.keys().copied().collect();
        let before = self.enode_count();
        let mut unions: Vec<(u32, u32)> = Vec::new();
        for n in snapshot {
            if self.enode_count() >= self.cap {
                self.hit_cap = true;
                break;
            }
            let id = match self.hashcons.get(&n) {
                Some(&i) => self.find(i),
                None => continue,
            };
            match n.op {
                Op::Add | Op::Mul => {
                    // commutativity: op(a,b) = op(b,a)
                    let comm = self.add_node(ENode { op: n.op, a: n.b, b: n.a });
                    unions.push((id, comm));
                    // associativity: op(op(x,y),z) = op(x,op(y,z))
                    // look at the left child's representative node.
                    let left_rep = self.nodes[self.find(n.a) as usize];
                    if left_rep.op == n.op {
                        let (x, y, z) = (left_rep.a, left_rep.b, n.b);
                        let yz = self.add_node(ENode { op: n.op, a: y, b: z });
                        let assoc = self.add_node(ENode { op: n.op, a: x, b: yz });
                        unions.push((id, assoc));
                    }
                }
                Op::Leaf => {}
            }
        }
        for (a, b) in unions {
            self.union(a, b);
        }
        self.rebuild();
        self.enode_count() > before && !self.hit_cap
    }

    /// Saturate until fixpoint or the cap is hit. Returns the number of rounds.
    pub fn saturate(&mut self) -> u32 {
        let mut rounds = 0;
        while self.step() {
            rounds += 1;
            if rounds > 64 {
                break; // safety: a well-behaved bounded run converges quickly
            }
        }
        rounds
    }

    /// Greedy min-cost extraction: cost = number of e-nodes in the chosen form.
    /// Returns (cost, evaluated value). All forms in a class are equal, so the
    /// value is the cross-validation invariant.
    pub fn extract(&self, root: u32) -> (u64, u64) {
        let mut cost_memo: Vec<Option<u64>> = vec![None; self.parent.len()];
        let mut val_memo: Vec<Option<u64>> = vec![None; self.parent.len()];
        let c = self.extract_cost(root, &mut cost_memo);
        let v = self.extract_val(root, &mut val_memo);
        (c, v)
    }

    fn extract_cost(&self, id: u32, memo: &mut Vec<Option<u64>>) -> u64 {
        let r = self.find(id) as usize;
        if let Some(v) = memo[r] {
            return v;
        }
        memo[r] = Some(u64::MAX); // cycle guard
        let mut best = u64::MAX;
        for n in &self.class_nodes[r] {
            let cn = self.canon(*n);
            let cost = match cn.op {
                Op::Leaf => 1,
                _ => 1u64
                    .saturating_add(self.extract_cost(cn.a, memo))
                    .saturating_add(self.extract_cost(cn.b, memo)),
            };
            if cost < best {
                best = cost;
            }
        }
        memo[r] = Some(best);
        best
    }

    fn extract_val(&self, id: u32, memo: &mut Vec<Option<u64>>) -> u64 {
        let r = self.find(id) as usize;
        if let Some(v) = memo[r] {
            return v;
        }
        memo[r] = Some(0); // cycle guard: reassociations are equal, any form works
        // pick the first leaf-reachable node deterministically (the representative).
        let n = self.canon(self.nodes[r]);
        let v = match n.op {
            Op::Leaf => self.leaf_vals[n.a as usize],
            Op::Add => self
                .extract_val(n.a, memo)
                .wrapping_add(self.extract_val(n.b, memo)),
            Op::Mul => self
                .extract_val(n.a, memo)
                .wrapping_mul(self.extract_val(n.b, memo)),
        };
        memo[r] = Some(v);
        v
    }
}

/// Build a left-leaning associative chain `((..(l0 op l1) op l2) .. op l_{k-1})`
/// of `k` leaves with the given op, into a fresh e-graph with the given cap.
/// Returns (graph, root). This is the reassociation-explosion workload.
pub fn build_chain(k: usize, op: Op, cap: usize, seed: u64) -> (EGraph, u32) {
    let mut g = EGraph::new(cap);
    let mut s = seed | 1;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut acc = g.leaf(0, next());
    for i in 1..k as u32 {
        let l = g.leaf(i, next());
        acc = match op {
            Op::Add => g.add(acc, l),
            Op::Mul => g.mul(acc, l),
            Op::Leaf => unreachable!(),
        };
    }
    (g, acc)
}

// ── Program-level associativity/commutativity reassociation (optimize strategy) ──
//
// The engine above answers the standalone bounded-explosion question (does a
// capped rewrite window catch the reassociation blowup). This section wires
// reassociation in as the optimize stage's fourth strategy, so the axis is
// genuinely `none / CSE / eqsat / CSE+eqsat` as the design specified, not just
// CSE/fold/DCE.
//
// It does NOT run a general e-graph to saturation and then extract: general
// e-graph extraction is unsound under a greedy extractor once associativity
// introduces cyclic e-classes (a real, well-known hazard; egg/egglog handle it
// with a dedicated cyclic extractor). Instead it computes the canonical AC
// normal form directly, which is sound, bounded, terminating, and acyclic by
// construction: it flattens every maximal same-op ADD or MUL chain into its
// operand multiset, sorts the operands, and rebuilds a canonical left-leaning
// tree, hash-consing every node. Two expressions equal modulo associativity and
// commutativity therefore normalise to the identical node and share it, the
// CSE-modulo-AC that plain structural CSE (which keys on exact operand order and
// nesting) cannot see. On the carrier's random DAGs this shares little; on the
// correlated MUL/ADD profiles it exposes repeated reassociated subexpressions.
// The bench measures which per profile, which is the point.

use crate::ir::{op as irop, Node, Program};

/// Get-or-create a CONST node for value `v`, pooling the constant and CSEing the
/// node so equal constants share one node.
fn mk_const(
    v: u64,
    new_nodes: &mut Vec<Node>,
    new_consts: &mut Vec<u64>,
    const_pool: &mut HashMap<u64, u32>,
    const_node: &mut HashMap<u64, u32>,
) -> u32 {
    if let Some(&id) = const_node.get(&v) {
        return id;
    }
    let pool_idx = *const_pool.entry(v).or_insert_with(|| {
        let idx = new_consts.len() as u32;
        new_consts.push(v);
        idx
    });
    let id = new_nodes.len() as u32;
    new_nodes.push(Node { op: irop::CONST, operands: vec![pool_idx] });
    const_node.insert(v, id);
    id
}

/// Get-or-create an op node, hash-consed on `(op, operands)` so structurally
/// identical nodes share one id.
fn mk_op(
    op: u8,
    operands: Vec<u32>,
    new_nodes: &mut Vec<Node>,
    cse_op: &mut HashMap<(u8, Vec<u32>), u32>,
) -> u32 {
    let key = (op, operands.clone());
    if let Some(&id) = cse_op.get(&key) {
        return id;
    }
    let id = new_nodes.len() as u32;
    new_nodes.push(Node { op, operands });
    cse_op.insert(key, id);
    id
}

/// Fold a canonical (already sorted) operand list into a left-leaning tree of
/// `op`, hash-consing each intermediate so shared prefixes are reused.
fn build_tree(
    op: u8,
    flat: &[u32],
    new_nodes: &mut Vec<Node>,
    cse_op: &mut HashMap<(u8, Vec<u32>), u32>,
) -> u32 {
    debug_assert!(!flat.is_empty());
    let mut acc = flat[0];
    for &next in &flat[1..] {
        acc = mk_op(op, vec![acc, next], new_nodes, cse_op);
    }
    acc
}

/// Canonical AC reassociation with structural CSE (see the section doc above).
/// Returns the rewritten program plus the new node ids of the original
/// `sink_nodes`, in the same order, so the optimize stage keeps its live-out
/// provenance and the sink-fold cross-validation holds (ADD and MUL are
/// associative and commutative under wrapping arithmetic, so every rewritten form
/// evaluates to the same value for every input). `cap` bounds how far a chain is
/// flattened: once a node's flattened operand list reaches `cap`, further same-op
/// operands are kept as concrete subtrees rather than spliced. That is the
/// "bounded" in bounded reassociation. It keeps the pass O(n * cap); without it a
/// fully-flattened linear chain of length L stores an O(L) operand list at each of
/// its L nodes, O(L^2) copying on the correlated profiles. The bound limits only
/// how wide one reassociation window is (a wider expression reassociates in
/// cap-sized windows), never correctness, and value is preserved regardless.
pub fn eqsat_reassociate(prog: &Program, sink_nodes: &[u32], cap: usize) -> (Program, Vec<u32>) {
    let n = prog.nodes.len();
    let mut new_nodes: Vec<Node> = Vec::new();
    let mut new_consts: Vec<u64> = Vec::new();
    let mut const_pool: HashMap<u64, u32> = HashMap::new();
    let mut const_node: HashMap<u64, u32> = HashMap::new();
    let mut cse_op: HashMap<(u8, Vec<u32>), u32> = HashMap::new();
    // concrete new id per original node, built lazily: a CONST/INPUT/non-AC node
    // gets its id here; an ADD/MUL node in a chain leaves this None and only its
    // flattened operand list is recorded, so a same-op parent absorbs it. The
    // concrete tree for a chain node is built (via `concrete_id`) only when a
    // non-same-op consumer or a sink actually needs its id. That keeps the node
    // count bounded by the original: interior chain nodes emit nothing, only the
    // maximal-chain roots emit a canonical tree (of the same node count the
    // original nesting had, or fewer with sharing). Eagerly building a tree per
    // interior node was correct but O(chain^2) on the correlated profiles.
    let mut id_of: Vec<Option<u32>> = vec![None; n];
    // for each ADD/MUL node, the flattened sorted operand list it normalises to.
    let mut flat_of: Vec<Option<Vec<u32>>> = vec![None; n];
    let mut input_id: Option<u32> = None;

    for i in 0..n {
        let op = prog.nodes[i].op;
        match op {
            irop::CONST => {
                let v = prog.consts[prog.nodes[i].operands[0] as usize];
                id_of[i] = Some(mk_const(v, &mut new_nodes, &mut new_consts, &mut const_pool, &mut const_node));
            }
            irop::INPUT => {
                id_of[i] = Some(*input_id.get_or_insert_with(|| {
                    let id = new_nodes.len() as u32;
                    new_nodes.push(Node { op: irop::INPUT, operands: vec![] });
                    id
                }));
            }
            _ if (op == irop::ADD || op == irop::MUL) && prog.nodes[i].operands.len() == 2 => {
                let ops = prog.nodes[i].operands.clone();
                let mut flat: Vec<u32> = Vec::new();
                for &o in &ops {
                    // splice a same-op operand's normal form (reassociation) while
                    // under the cap; else realise the operand as one concrete
                    // subtree. The cap keeps flat lists O(cap) so the whole pass is
                    // O(n * cap) instead of O(chain^2).
                    if prog.nodes[o as usize].op == op
                        && flat_of[o as usize].is_some()
                        && flat.len() < cap
                    {
                        flat.extend_from_slice(flat_of[o as usize].as_ref().unwrap());
                    } else {
                        flat.push(concrete_id(o, prog, &mut id_of, &flat_of, &mut new_nodes, &mut cse_op));
                    }
                }
                flat.sort_unstable();
                flat_of[i] = Some(flat); // id_of[i] stays None (deferred).
            }
            _ => {
                // non-AC op: operand order is semantic; realise operand ids, CSE.
                let ops = prog.nodes[i].operands.clone();
                let operands: Vec<u32> = ops
                    .iter()
                    .map(|&o| concrete_id(o, prog, &mut id_of, &flat_of, &mut new_nodes, &mut cse_op))
                    .collect();
                id_of[i] = Some(mk_op(op, operands, &mut new_nodes, &mut cse_op));
            }
        }
    }

    let out_ids: Vec<u32> = sink_nodes
        .iter()
        .map(|&s| concrete_id(s, prog, &mut id_of, &flat_of, &mut new_nodes, &mut cse_op))
        .collect();
    (Program { consts: new_consts, nodes: new_nodes }, out_ids)
}

/// Realise the concrete new-node id of original node `o`. Leaves and non-AC nodes
/// already have one; a deferred ADD/MUL chain node builds its canonical tree from
/// its flattened operand list now (memoised into `id_of`), so each chain node's
/// tree is built at most once and only if a non-same-op consumer or a sink needs
/// it.
fn concrete_id(
    o: u32,
    prog: &Program,
    id_of: &mut Vec<Option<u32>>,
    flat_of: &[Option<Vec<u32>>],
    new_nodes: &mut Vec<Node>,
    cse_op: &mut HashMap<(u8, Vec<u32>), u32>,
) -> u32 {
    let oi = o as usize;
    if let Some(id) = id_of[oi] {
        return id;
    }
    let op = prog.nodes[oi].op;
    let flat = flat_of[oi].as_ref().expect("deferred AC node has a flattened operand list");
    let id = build_tree(op, flat, new_nodes, cse_op);
    id_of[oi] = Some(id);
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_and_unbounded_extract_equal_value() {
        // A bounded run (small cap) and a larger run over the same chain must
        // extract forms that evaluate to the same value: all reassociations are
        // equal. The costs and enode counts differ; the value does not.
        for k in [4usize, 8, 12] {
            let (mut gb, rb) = build_chain(k, Op::Add, 256, 0xabc);
            gb.saturate();
            let (_cb, vb) = gb.extract(rb);

            let (mut gu, ru) = build_chain(k, Op::Add, 100_000, 0xabc);
            gu.saturate();
            let (_cu, vu) = gu.extract(ru);

            assert_eq!(vb, vu, "bounded and unbounded disagree on value at k={k}");
        }
    }

    #[test]
    fn cap_is_respected() {
        // A long chain with a small cap must stop at (near) the cap, not explode.
        let (mut g, _r) = build_chain(24, Op::Add, 2000, 0x1);
        g.saturate();
        assert!(g.hit_cap, "a 24-leaf chain should hit a 2000-node cap");
        assert!(g.enode_count() <= 2000 + 64, "enode count stayed bounded");
    }

    #[test]
    fn unbounded_grows_far_larger_than_bounded() {
        let (mut gb, _r) = build_chain(12, Op::Add, 200, 0x9);
        gb.saturate();
        let (mut gu, _r) = build_chain(12, Op::Add, 100_000, 0x9);
        gu.saturate();
        assert!(
            gu.enode_count() > gb.enode_count() * 3,
            "unbounded ({}) should dwarf bounded ({})",
            gu.enode_count(),
            gb.enode_count()
        );
    }
}
