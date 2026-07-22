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
    /// Per-op sampling weights, indexed by opcode. A mid-stream op is drawn from
    /// `0..op_vocab` proportional to its weight, so arity mix, leaf fraction, and
    /// heavy-op fraction are all controllable by reweighting the vocabulary. All
    /// weights equal reproduces uniform-over-vocab; a weight of zero excludes an
    /// op. This subsumes `op_vocab` (a vocab of size k is weights zero past k),
    /// which stays as a convenience clamp.
    pub op_weights: [u16; op::COUNT as usize],
}

/// Build a weight array from `(opcode, weight)` pairs, zero elsewhere.
pub fn weights(pairs: &[(u8, u16)]) -> [u16; op::COUNT as usize] {
    let mut w = [0u16; op::COUNT as usize];
    for &(o, x) in pairs {
        w[o as usize] = x;
    }
    w
}

/// Uniform weights over every op (the balanced default).
pub const fn uniform_weights() -> [u16; op::COUNT as usize] {
    [1u16; op::COUNT as usize]
}

impl GenParams {
    /// The shipped-runtime default operating point. Every axis bench is a delta
    /// from this. Identical to `p_real`.
    pub fn default_point() -> Self {
        GenParams {
            seed: 0x5eed_1234_abcd_0001,
            node_count: 4096,
            op_vocab: op::COUNT,
            op_correlation: 0,
            locality_window: 64,
            const_count: 32,
            op_weights: uniform_weights(),
        }
    }

    /// P_real: the balanced "typical program" and the matrix centre.
    pub fn p_real() -> Self {
        Self::default_point()
    }

    /// P_madd: a single repeated multiply-add motif, tight locality. The clean
    /// native-anchor and the maximally-predictable dispatch case.
    pub fn p_madd() -> Self {
        GenParams {
            op_correlation: 900,
            locality_window: 4,
            op_weights: weights(&[(op::CONST, 1), (op::ADD, 4), (op::MUL, 4)]),
            ..Self::default_point()
        }
    }

    /// P_tight: predictable and local. Correlated stream, small window, the
    /// hot-loop-body case (threaded/if-chain should shine on dispatch).
    pub fn p_tight() -> Self {
        GenParams {
            op_correlation: 900,
            locality_window: 8,
            op_weights: weights(&[
                (op::CONST, 1),
                (op::ADD, 1),
                (op::SUB, 1),
                (op::MUL, 1),
                (op::AND, 1),
                (op::OR, 1),
                (op::XOR, 1),
            ]),
            ..Self::default_point()
        }
    }

    /// P_scatter: unpredictable and cache-hostile. i.i.d. stream, full vocab,
    /// operands drawn from the whole prefix (the adversarial case).
    pub fn p_scatter() -> Self {
        GenParams {
            op_correlation: 0,
            locality_window: usize::MAX,
            op_weights: weights(&[
                (op::CONST, 1),
                (op::ADD, 1),
                (op::SUB, 1),
                (op::MUL, 1),
                (op::AND, 1),
                (op::OR, 1),
                (op::XOR, 1),
                (op::SHL, 1),
                (op::SHR, 1),
                (op::MIN, 1),
                (op::MAX, 1),
                (op::EQ, 1),
                (op::LT, 1),
                (op::SELECT, 1),
                (op::NEG, 1),
                (op::NOT, 1),
            ]),
            ..Self::default_point()
        }
    }

    /// P_wideselect: arity-heavy, weighted toward ternary SELECT plus a
    /// condition producer, to stress record width and the spill path.
    pub fn p_wideselect() -> Self {
        GenParams {
            op_weights: weights(&[
                (op::CONST, 2),
                (op::ADD, 2),
                (op::MUL, 2),
                (op::LT, 2),
                (op::SELECT, 6),
            ]),
            ..Self::default_point()
        }
    }

    /// P_leaf: decode-bound, weighted heavily toward CONST leaves, so dispatch
    /// is light and form (decode cost) dominates.
    pub fn p_leaf() -> Self {
        GenParams {
            op_weights: weights(&[(op::CONST, 8), (op::ADD, 1), (op::MUL, 1)]),
            ..Self::default_point()
        }
    }

    /// The six designed profiles, by stable name, for the matrix generator.
    pub fn profile(name: &str) -> Option<Self> {
        Some(match name {
            "real" => Self::p_real(),
            "madd" => Self::p_madd(),
            "tight" => Self::p_tight(),
            "scatter" => Self::p_scatter(),
            "wideselect" => Self::p_wideselect(),
            "leaf" => Self::p_leaf(),
            _ => return None,
        })
    }
}

/// Draw an op from `0..vocab` proportional to `weights`. Falls back to uniform
/// if the visible weights sum to zero. One `next_u64` draw, so generation stays
/// deterministic and cheap.
#[inline]
fn weighted_op(rng: &mut Rng, weights: &[u16; op::COUNT as usize], vocab: usize) -> u8 {
    let vis = &weights[..vocab.min(op::COUNT as usize)];
    let total: u32 = vis.iter().map(|&w| w as u32).sum();
    if total == 0 {
        return rng.below(vocab) as u8;
    }
    let mut r = (rng.next_u64() % total as u64) as u32;
    for (i, &w) in vis.iter().enumerate() {
        let w = w as u32;
        if r < w {
            return i as u8;
        }
        r -= w;
    }
    (vocab - 1) as u8
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
            weighted_op(&mut rng, &p.op_weights, vocab as usize)
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
