//! Seeded, deterministic program generation over the axes the earlier corpus
//! kept improvising per bench: node count, op-vocabulary size, op-stream
//! correlation, operand-locality window, and const-pool size. One seed gives
//! one program, byte-reproducibly, with no external RNG so a run is exactly
//! repeatable and free of any dependency surface.

use crate::ir::{op, Node, Program};

/// splitmix64: a tiny, well-distributed seeded stream. Deterministic and
/// dependency-free.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_u64() % n as u64) as usize
        }
    }

    /// A probability check at 1/1000 resolution.
    #[inline]
    pub fn chance(&mut self, per_mille: u32) -> bool {
        (self.next_u64() % 1000) < per_mille as u64
    }
}

/// Generation axes. Defaults match the shipped-runtime assumptions; a bench
/// moves exactly one of these off its default.
#[derive(Clone, Copy, Debug)]
pub struct GenParams {
    pub seed: u64,
    /// How many nodes the program holds.
    pub node_count: usize,
    /// How many distinct ops the stream draws from, `1..=op::COUNT`.
    pub op_vocab: u8,
    /// Per-mille chance a node reuses the previous op, modelling a correlated
    /// (non-i.i.d.) instruction stream. 0 is fully random, 1000 is one op
    /// repeated.
    pub op_correlation: u32,
    /// Operands are drawn from the `locality_window` most recent earlier nodes.
    /// A small window models the backward-local access the arena is built for.
    pub locality_window: usize,
    /// Size of the const pool the leaf CONST nodes draw from.
    pub const_count: usize,
}

impl GenParams {
    /// The shipped-runtime default operating point. Every axis bench is a delta
    /// from this.
    pub fn default_point() -> Self {
        GenParams {
            seed: 0x5eed_1234_abcd_0001,
            node_count: 4096,
            op_vocab: op::COUNT,
            op_correlation: 0,
            locality_window: 64,
            const_count: 32,
        }
    }
}

/// Generate a well-formed program from the params. The first `LEAF_SEED` nodes
/// are CONST so later nodes always have earlier operands to reference; from then
/// on ops are drawn under the correlation model and operands within the
/// locality window.
pub fn generate(p: &GenParams) -> Program {
    const LEAF_SEED: usize = 8;
    let mut rng = Rng::new(p.seed);
    let vocab = p.op_vocab.max(1).min(op::COUNT);
    let const_count = p.const_count.max(1);

    let consts: Vec<u64> = (0..const_count).map(|_| rng.next_u64()).collect();
    let mut nodes: Vec<Node> = Vec::with_capacity(p.node_count);
    let mut prev_op: u8 = op::CONST;

    for i in 0..p.node_count {
        // Node 0 is INPUT so the whole DAG depends on the per-call seed and
        // cannot be hoisted. The next leaves are CONST so later nodes always
        // have earlier operands. From then on ops are drawn under the
        // correlation model (INPUT, op COUNT-1, is excluded from the draw and
        // reached only through node 0).
        let op_code = if i == 0 {
            op::INPUT
        } else if i < LEAF_SEED {
            op::CONST
        } else if p.op_correlation > 0
            && prev_op != op::CONST
            && prev_op != op::INPUT
            && rng.chance(p.op_correlation)
        {
            prev_op
        } else {
            (rng.below(vocab as usize)) as u8
        };

        let node = if op_code == op::INPUT {
            Node { op: op::INPUT, operands: vec![] }
        } else if op_code == op::CONST {
            Node {
                op: op::CONST,
                operands: vec![rng.below(const_count) as u32],
            }
        } else {
            let arity = op::ARITY[op_code as usize] as usize;
            let lo = i.saturating_sub(p.locality_window.max(arity));
            let span = i - lo;
            let operands = (0..arity)
                .map(|_| (lo + rng.below(span)) as u32)
                .collect();
            Node { op: op_code, operands }
        };
        prev_op = node.op;
        nodes.push(node);
    }

    Program { consts, nodes }
}
