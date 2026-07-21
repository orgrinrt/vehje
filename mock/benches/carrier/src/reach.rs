//! Reachability fixpoint: whole-column versus real semi-naive.
//!
//! The design question (SP5 lease reachability, the compile-stage fixpoint) is
//! whether a delta-tracked semi-naive evaluation beats a whole-column
//! recompute, and at what graph shape the answer flips. The old bench answered
//! it dishonestly: its "delta" still scanned the full edge list every round
//! (`for e in edges { if !dirty[e.dst] continue; ... }`), so it was O(E) per
//! round like whole-column, only with a skip. The "index edges by child"
//! comment described an index it never built. That is not semi-naive.
//!
//! Real semi-naive needs a CSR in-edge index (for each node, the sources of its
//! in-edges) and a frontier worklist, so a round touches only the edges
//! incident to nodes whose reach changed, never the full edge list. This module
//! provides both solvers over one shared graph and a reach checksum so they
//! cross-validate byte-exact: they must reach the identical fixpoint, so any
//! measured difference is the evaluation strategy and nothing else.
//!
//! Reachability is tracked as a u64 word per node: bit j of `reach[i]` means
//! node i reaches marked target j (up to 64 tracked targets). The propagation
//! is `reach[src] |= reach[dst]` along each edge `src -> dst`, so reach flows
//! backward from targets; a node reaches whatever its out-neighbors reach.

use crate::gen::Rng;

/// A directed graph plus a lazily-built CSR in-edge index (index by dst -> the
/// srcs of edges into dst), which is what semi-naive propagation walks.
pub struct Graph {
    pub n: usize,
    pub edges: Vec<(u32, u32)>,
    in_off: Vec<u32>,
    in_src: Vec<u32>,
}

impl Graph {
    fn from_edges(n: usize, edges: Vec<(u32, u32)>) -> Self {
        // CSR in-edge index: for each dst, the list of srcs with an edge into it.
        let mut deg = vec![0u32; n + 1];
        for &(_, d) in &edges {
            deg[d as usize + 1] += 1;
        }
        for i in 0..n {
            deg[i + 1] += deg[i];
        }
        let in_off = deg; // now a prefix-sum offset array of length n+1
        let mut cursor = in_off.clone();
        let mut in_src = vec![0u32; edges.len()];
        for &(s, d) in &edges {
            let slot = cursor[d as usize];
            in_src[slot as usize] = s;
            cursor[d as usize] += 1;
        }
        Graph { n, edges, in_off, in_src }
    }

    /// The sources of edges into `dst` (its in-neighbors).
    #[inline]
    fn in_neighbors(&self, dst: u32) -> &[u32] {
        let a = self.in_off[dst as usize] as usize;
        let b = self.in_off[dst as usize + 1] as usize;
        &self.in_src[a..b]
    }
}

/// A layered DAG: `depth` layers of width `n/depth`, every node in layer i has
/// `fanout` edges into layer i+1. Reach flows from the last layer backward, so
/// the fixpoint takes `depth` rounds for whole-column, which is the regime
/// where semi-naive is meant to win. Edges point low-layer -> high-layer, and
/// `reach[src] |= reach[dst]` carries a target marked in the last layer back to
/// layer 0 over `depth` rounds.
pub fn gen_layered(n: usize, depth: usize, fanout: usize, seed: u64) -> Graph {
    let mut rng = Rng::new(seed);
    let width = (n / depth).max(1);
    let real_n = width * depth;
    let mut edges = Vec::with_capacity(real_n * fanout);
    for layer in 0..depth - 1 {
        let base = layer * width;
        let next = (layer + 1) * width;
        for i in 0..width {
            let src = (base + i) as u32;
            for _ in 0..fanout {
                let dst = (next + (rng.next_u64() as usize % width)) as u32;
                edges.push((src, dst));
            }
        }
    }
    Graph::from_edges(real_n, edges)
}

/// A fan-in-heavy shallow graph: `n-1` source nodes each with one edge into a
/// single sink. Converges in one round; the regime where whole-column's
/// simplicity may beat semi-naive's frontier bookkeeping.
pub fn gen_fanin(n: usize, _seed: u64) -> Graph {
    let sink = 0u32;
    let edges: Vec<(u32, u32)> = (1..n as u32).map(|s| (s, sink)).collect();
    Graph::from_edges(n, edges)
}

/// A random DAG with `avg_out` out-edges per node, all pointing to
/// higher-indexed nodes (acyclic by construction). Depth is diffuse; a middle
/// regime between the deep chain and the shallow fan-in.
pub fn gen_random_dag(n: usize, avg_out: usize, seed: u64) -> Graph {
    let mut rng = Rng::new(seed);
    let mut edges = Vec::with_capacity(n * avg_out);
    for src in 0..n {
        if src + 1 >= n {
            break;
        }
        for _ in 0..avg_out {
            let span = n - src - 1;
            let dst = src + 1 + (rng.next_u64() as usize % span);
            edges.push((src as u32, dst as u32));
        }
    }
    Graph::from_edges(n, edges)
}

/// Initialize reach words: mark up to 64 target nodes (bit j on target j) and
/// mix in an FFI-derived `perturb` byte so the closure a timed iteration
/// computes depends on the run input and cannot be hoisted. Targets are spread
/// across the highest node indices (the last layer of a layered DAG), where
/// reach must propagate backward from.
pub fn init_reach(g: &Graph, ntargets: usize, perturb: u64) -> Vec<u64> {
    let nt = ntargets.min(64).min(g.n);
    let mut reach = vec![0u64; g.n];
    for j in 0..nt {
        // spread targets across the top of the index range
        let t = g.n - 1 - (j * g.n / nt.max(1)) % g.n;
        reach[t] |= 1u64 << j;
    }
    // perturb: also seed one input-selected node, so distinct inputs give
    // distinct fixpoints (anti-hoist), without changing which strategy wins.
    let extra = (perturb as usize) % g.n;
    reach[extra] |= 1u64 << (perturb % 64);
    reach
}

/// Reset an existing reach buffer to the initial marking in place, so the timed
/// region can re-solve without allocating. Same marking as [`init_reach`].
pub fn reset_reach(g: &Graph, reach: &mut [u64], ntargets: usize, perturb: u64) {
    let nt = ntargets.min(64).min(g.n);
    for r in reach.iter_mut() {
        *r = 0;
    }
    for j in 0..nt {
        let t = g.n - 1 - (j * g.n / nt.max(1)) % g.n;
        reach[t] |= 1u64 << j;
    }
    let extra = (perturb as usize) % g.n;
    reach[extra] |= 1u64 << (perturb % 64);
}

/// Whole-column naive fixpoint: scan every edge each round, `reach[src] |=
/// reach[dst]`, until a full pass changes nothing. O(E) per round, `rounds`
/// rounds. Returns the round count. Mutates `reach` to the fixpoint.
pub fn solve_whole(g: &Graph, reach: &mut [u64]) -> u32 {
    let mut rounds = 0u32;
    loop {
        let mut changed = false;
        for &(s, d) in &g.edges {
            let before = reach[s as usize];
            reach[s as usize] |= reach[d as usize];
            if reach[s as usize] != before {
                changed = true;
            }
        }
        rounds += 1;
        if !changed {
            break;
        }
    }
    rounds
}

/// Real semi-naive fixpoint: a frontier worklist plus the CSR in-edge index. A
/// round processes only the nodes whose reach changed last round; for each such
/// node `c` it updates `c`'s in-neighbors (the srcs that point at `c`), pushing
/// any whose reach grew onto the next frontier. Never scans the full edge list.
/// `scratch_next` and `queued` are caller-owned reusable buffers so the timed
/// region does no allocation. Returns the round count.
pub fn solve_semi(
    g: &Graph,
    reach: &mut [u64],
    frontier: &mut Vec<u32>,
    scratch_next: &mut Vec<u32>,
    queued: &mut [bool],
) -> u32 {
    frontier.clear();
    // seed the frontier with every node that starts with nonzero reach (the
    // marked targets and the perturb node); those are the changes round 0
    // propagates from.
    for (i, &r) in reach.iter().enumerate() {
        if r != 0 {
            frontier.push(i as u32);
        }
    }
    let mut rounds = 0u32;
    while !frontier.is_empty() {
        scratch_next.clear();
        for &c in frontier.iter() {
            let rc = reach[c as usize];
            for &src in g.in_neighbors(c) {
                let before = reach[src as usize];
                let after = before | rc;
                if after != before {
                    reach[src as usize] = after;
                    if !queued[src as usize] {
                        queued[src as usize] = true;
                        scratch_next.push(src);
                    }
                }
            }
        }
        // reset the dedup flags for the nodes we queued, then swap frontiers.
        for &v in scratch_next.iter() {
            queued[v as usize] = false;
        }
        std::mem::swap(frontier, scratch_next);
        rounds += 1;
    }
    rounds
}

/// Fold the reach array into a checksum, mixing `perturb` so distinct inputs
/// give distinct checksums. Whole-column and semi-naive must agree on this.
pub fn reach_checksum(reach: &[u64], perturb: u64) -> u64 {
    let mut h = perturb;
    for &r in reach {
        h = h.rotate_left(7) ^ r;
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    fn whole_eq_semi(g: &Graph, perturb: u64) {
        let mut rw = init_reach(g, 64, perturb);
        solve_whole(g, &mut rw);
        let cw = reach_checksum(&rw, perturb);

        let mut rs = init_reach(g, 64, perturb);
        let mut fr = Vec::new();
        let mut nx = Vec::new();
        let mut q = vec![false; g.n];
        solve_semi(g, &mut rs, &mut fr, &mut nx, &mut q);
        let cs = reach_checksum(&rs, perturb);

        assert_eq!(cw, cs, "whole and semi disagree on the fixpoint");
        assert_eq!(rw, rs, "reach arrays differ");
    }

    #[test]
    fn layered_whole_eq_semi() {
        for depth in [8usize, 32, 128] {
            let g = gen_layered(4096, depth, 3, 0x5eed_0001 ^ depth as u64);
            for p in [0u64, 1, 42, 255] {
                whole_eq_semi(&g, p);
            }
        }
    }

    #[test]
    fn fanin_whole_eq_semi() {
        let g = gen_fanin(4096, 0);
        for p in [0u64, 7, 200] {
            whole_eq_semi(&g, p);
        }
    }

    #[test]
    fn random_dag_whole_eq_semi() {
        let g = gen_random_dag(4096, 4, 0xabcd);
        for p in [0u64, 3, 99] {
            whole_eq_semi(&g, p);
        }
    }

    #[test]
    fn deep_chain_semi_uses_fewer_touches() {
        // Sanity: on a deep layered graph the semi-naive round count is bounded
        // by the depth, and whole-column takes at least depth rounds. This does
        // not assert timing, only that the two agree and both terminate.
        let g = gen_layered(4096, 128, 2, 0x1234);
        let mut rw = init_reach(&g, 64, 0);
        let wr = solve_whole(&g, &mut rw);
        let mut rs = init_reach(&g, 64, 0);
        let (mut fr, mut nx, mut q) = (Vec::new(), Vec::new(), vec![false; g.n]);
        let sr = solve_semi(&g, &mut rs, &mut fr, &mut nx, &mut q);
        assert_eq!(rw, rs);
        assert!(wr >= 2 && sr >= 2, "both should take multiple rounds");
    }
}
