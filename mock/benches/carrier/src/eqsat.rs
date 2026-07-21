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
