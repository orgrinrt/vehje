//! Retraction: recompute vs counted differential on edge deletion.
//!
//! Semi-naive evaluation (the `reach` module) handles additive edits: adding an
//! edge only grows the fixpoint. Deletion is the hard case, and the design names
//! a DBSP-style counted differential as owed for it. This is the catalogue's
//! "small bench of recompute-versus-counted on a delete."
//!
//! The relation is single-target reachability: does a node reach a fixed sink?
//! `reach[u]` holds when some live out-edge `u -> v` has `reach[v]` (or `u` is
//! the sink). The counted representation additionally keeps `count[u]`, the
//! number of live out-neighbours of `u` that reach the sink, the derivation
//! multiplicity: `u` reaches the sink iff `count[u] > 0` (or `u` is the sink).
//!
//! On deleting a batch of edges, two strategies reach the identical answer (the
//! cross-validation):
//! - recompute: rebuild `reach` from scratch on the surviving graph (a backward
//!   BFS from the sink). Correct, but O(surviving graph) every time, oblivious to
//!   how localised the change was.
//! - counted: propagate only the retraction. For each deleted `u -> v` with `u`
//!   and `v` reaching the sink, decrement `count[u]`; if it hits zero, `u` stops
//!   reaching the sink, so decrement its in-neighbours' counts, cascading.
//!   Touches only the nodes whose support actually changed.
//!
//! Graphs are DAGs (edges point to lower indices, toward the sink 0), so the
//! count-based support is acyclic and the retraction cascade terminates.

/// A directed graph with in-edge and out-edge CSR indices that carry the edge id
/// per entry, so per-edge liveness is an O(1) check during traversal (no edge
/// scanning). Out-edges drive the initial reach/count; in-edges drive the
/// retraction cascade.
pub struct RGraph {
    pub n: usize,
    pub sink: u32,
    edges: Vec<(u32, u32)>,
    live: Vec<bool>,
    out_off: Vec<u32>,
    out_dst: Vec<u32>,
    out_eid: Vec<u32>,
    in_off: Vec<u32>,
    in_src: Vec<u32>,
    in_eid: Vec<u32>,
}

// Build a CSR keyed on `key(edge)`, storing the other endpoint and the edge id.
fn build_csr(n: usize, edges: &[(u32, u32)], by_src: bool) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let key = |e: &(u32, u32)| if by_src { e.0 } else { e.1 } as usize;
    let other = |e: &(u32, u32)| if by_src { e.1 } else { e.0 };
    let mut off = vec![0u32; n + 1];
    for e in edges {
        off[key(e) + 1] += 1;
    }
    for i in 0..n {
        off[i + 1] += off[i];
    }
    let mut cursor = off.clone();
    let mut val = vec![0u32; edges.len()];
    let mut eid = vec![0u32; edges.len()];
    for (i, e) in edges.iter().enumerate() {
        let slot = cursor[key(e)] as usize;
        val[slot] = other(e);
        eid[slot] = i as u32;
        cursor[key(e)] += 1;
    }
    (off, val, eid)
}

impl RGraph {
    pub fn new(n: usize, sink: u32, edges: Vec<(u32, u32)>) -> Self {
        let (out_off, out_dst, out_eid) = build_csr(n, &edges, true);
        let (in_off, in_src, in_eid) = build_csr(n, &edges, false);
        let live = vec![true; edges.len()];
        RGraph { n, sink, live, edges, out_off, out_dst, out_eid, in_off, in_src, in_eid }
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Restore all edges to live, so a fresh delete can be timed on the same
    /// graph without reallocating the CSR indices.
    pub fn reset_live(&mut self) {
        for l in self.live.iter_mut() {
            *l = true;
        }
    }
}

/// Recompute reach-to-sink from scratch over the live edges: backward BFS from
/// the sink. When `v` is known to reach the sink, any `u` with a live edge
/// `u -> v` reaches the sink too.
pub fn recompute(g: &RGraph, reach: &mut [bool], work: &mut Vec<u32>) {
    for r in reach.iter_mut() {
        *r = false;
    }
    reach[g.sink as usize] = true;
    work.clear();
    work.push(g.sink);
    while let Some(v) = work.pop() {
        let a = g.in_off[v as usize] as usize;
        let b = g.in_off[v as usize + 1] as usize;
        for i in a..b {
            let u = g.in_src[i];
            if g.live[g.in_eid[i] as usize] && !reach[u as usize] {
                reach[u as usize] = true;
                work.push(u);
            }
        }
    }
}

/// Compute the counted representation's initial state: reach (via recompute) and
/// count[u] = number of live out-neighbours of u that reach the sink.
pub fn init_counted(g: &RGraph, reach: &mut [bool], count: &mut [u32], work: &mut Vec<u32>) {
    recompute(g, reach, work);
    for u in 0..g.n {
        let a = g.out_off[u] as usize;
        let b = g.out_off[u + 1] as usize;
        let mut c = 0u32;
        for i in a..b {
            if g.live[g.out_eid[i] as usize] && reach[g.out_dst[i] as usize] {
                c += 1;
            }
        }
        count[u] = c;
    }
}

/// Delete a batch of edges and update reach by RECOMPUTE (rebuild from scratch).
pub fn delete_recompute(g: &mut RGraph, dels: &[usize], reach: &mut [bool], work: &mut Vec<u32>) {
    for &e in dels {
        g.live[e] = false;
    }
    recompute(g, reach, work);
}

/// Delete a batch of edges and update reach + count by COUNTED differential:
/// propagate only the retraction cascade over the nodes whose support changed.
pub fn delete_counted(
    g: &mut RGraph,
    dels: &[usize],
    reach: &mut [bool],
    count: &mut [u32],
    frontier: &mut Vec<u32>,
) {
    // Phase 1: apply every deletion using the PRE-BATCH reach (do not mutate
    // reach here). Deleting an edge u -> v to a currently-reaching v removes one
    // of u's derivations, so decrement count[u]. Using the pre-batch reach for
    // all decrements keeps them independent of the order deletions are listed and
    // of drops that happen later in this same batch.
    for &e in dels {
        if !g.live[e] {
            continue;
        }
        g.live[e] = false;
        let (u, v) = g.edges[e];
        if reach[v as usize] {
            count[u as usize] = count[u as usize].saturating_sub(1);
        }
    }
    // Collect the initial droppers: a source whose count reached zero and that
    // still reaches (reach guards against re-processing a source listed twice).
    frontier.clear();
    for &e in dels {
        let u = g.edges[e].0 as usize;
        if reach[u] && count[u] == 0 && u as u32 != g.sink {
            reach[u] = false;
            frontier.push(u as u32);
        }
    }
    while let Some(x) = frontier.pop() {
        let a = g.in_off[x as usize] as usize;
        let b = g.in_off[x as usize + 1] as usize;
        for i in a..b {
            let w = g.in_src[i];
            if g.live[g.in_eid[i] as usize] && reach[w as usize] {
                count[w as usize] = count[w as usize].saturating_sub(1);
                if count[w as usize] == 0 && w != g.sink {
                    reach[w as usize] = false;
                    frontier.push(w);
                }
            }
        }
    }
}

/// Fold a reach vector to a checksum for cross-validation.
pub fn reach_checksum(reach: &[bool]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for (i, &r) in reach.iter().enumerate() {
        if r {
            h = (h ^ i as u64).rotate_left(7).wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

/// A random DAG reaching toward node 0 (the sink): each node u>0 has `avg_out`
/// edges to lower-indexed nodes, so it is acyclic and many nodes reach 0.
pub fn gen_reach_graph(n: usize, avg_out: usize, seed: u64) -> RGraph {
    let mut s = seed | 1;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut edges = Vec::with_capacity(n * avg_out);
    for u in 1..n as u32 {
        for _ in 0..avg_out {
            let v = (next() as usize % (u as usize)) as u32;
            edges.push((u, v));
        }
    }
    RGraph::new(n, 0, edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agree_after_delete(n: usize, avg_out: usize, ndel: usize, seed: u64) {
        let mut g1 = gen_reach_graph(n, avg_out, seed);
        let mut g2 = gen_reach_graph(n, avg_out, seed);
        let (mut r1, mut r2) = (vec![false; n], vec![false; n]);
        let (mut c2, mut work) = (vec![0u32; n], Vec::new());
        init_counted(&g2, &mut r2, &mut c2, &mut work);
        recompute(&g1, &mut r1, &mut work);
        assert_eq!(reach_checksum(&r1), reach_checksum(&r2), "init disagreement");

        let mut s = seed ^ 0xdead;
        let mut nx = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        let m = g1.edge_count();
        let dels: Vec<usize> = (0..ndel).map(|_| (nx() as usize) % m).collect();

        delete_recompute(&mut g1, &dels, &mut r1, &mut work);
        let mut fr = Vec::new();
        delete_counted(&mut g2, &dels, &mut r2, &mut c2, &mut fr);
        assert_eq!(
            reach_checksum(&r1),
            reach_checksum(&r2),
            "recompute vs counted disagree (n={n} avg_out={avg_out} ndel={ndel} seed={seed})"
        );
    }

    #[test]
    fn recompute_eq_counted_over_shapes() {
        for &(n, ao) in &[(64usize, 2usize), (256, 3), (1024, 4)] {
            for &nd in &[1usize, 5, 20, 100] {
                for seed in [0u64, 1, 42, 7] {
                    agree_after_delete(n, ao, nd, seed ^ n as u64);
                }
            }
        }
    }
}
