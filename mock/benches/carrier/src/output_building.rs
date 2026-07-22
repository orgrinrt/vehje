//! Output-building stage: how the program's outputs are materialized.
//!
//! Interpretation computes a value per node, but a consumer only observes the
//! program's live-outs (its sink nodes, referenced by no later node). How those
//! outputs are delivered is a real runtime choice with a measurable cost, and it
//! is orthogonal to dispatch, so it is a composition stage. Three strategies over
//! the same interpreted program, cross-validated on the sink values:
//!
//! - full: the interpreter fills the whole `results[node_count]` array and the
//!   consumer reads the sinks out of it. The natural shape every dispatch cell
//!   already produces; the output footprint is the whole array.
//! - compact: the interpreter fills a scratch array, then a post-pass gathers
//!   only the sink values into a small `outputs[num_sinks]` buffer. The consumer
//!   holds a compact output; the cost is one extra gather pass over the sinks.
//! - inline: the interpreter writes intermediates to scratch and, for a sink
//!   node, also appends its value to a compact output cursor as it is computed.
//!   No separate gather pass; the output is built during interpretation. Needs a
//!   per-node "is this a sink" test in the loop.
//!
//! The three deliver the identical sink values in sink order, which is the
//! cross-validation (a checksum over the compact output equals `checksum_at` over
//! the scratch sinks). The stage measures output materialization, not dispatch:
//! all three use the same switch interpreter underneath.

use crate::access::{checksum, checksum_at, rload, rstore};
use crate::ir::{op, Decoded};

/// A sink-membership bitmap, built once from the program's sink list (setup, an
/// honest `S` term hoisted out of the timed region). `is_sink[i]` is true if node
/// `i` is a live-out.
pub struct SinkMask {
    pub is_sink: Vec<bool>,
    pub sinks: Vec<u32>,
}

impl SinkMask {
    pub fn new(node_count: usize, sinks: &[u32]) -> SinkMask {
        let mut is_sink = vec![false; node_count];
        for &s in sinks {
            is_sink[s as usize] = true;
        }
        SinkMask {
            is_sink,
            sinks: sinks.to_vec(),
        }
    }
}

/// full: interpret into the whole results array; the consumer reads sinks from
/// it. Returns the checksum over the sinks (what the consumer observes).
pub fn build_full(d: &Decoded, seed: u64, results: &mut [u64], mask: &SinkMask) -> u64 {
    crate::interp::interpret(d, seed, results);
    checksum_at(results, &mask.sinks)
}

/// compact: interpret into scratch, then gather the sink values into `out`
/// (`out.len() == mask.sinks.len()`). Returns the checksum over the compact
/// output, which must equal the full strategy's.
pub fn build_compact(d: &Decoded, seed: u64, scratch: &mut [u64], mask: &SinkMask, out: &mut [u64]) -> u64 {
    crate::interp::interpret(d, seed, scratch);
    for (o, &s) in out.iter_mut().zip(mask.sinks.iter()) {
        *o = scratch[s as usize];
    }
    checksum(out)
}

/// inline: interpret, and as each sink node's value is computed, append it to the
/// compact output cursor. One fused pass, no separate gather, at the cost of a
/// per-node sink test. `out` must hold at least `mask.sinks.len()` values; the
/// sinks are appended in node (== sink) order. Returns the checksum over the
/// written output.
///
/// This re-implements the switch interpreter's evaluation inline so it can emit
/// at each sink; it uses the same shared `access` primitive, so the only
/// difference from `build_full` plus a gather is the fused inline emit.
pub fn build_inline(d: &Decoded, seed: u64, scratch: &mut [u64], mask: &SinkMask, out: &mut [u64]) -> u64 {
    let rp = scratch.as_mut_ptr();
    let mut oc = 0usize; // output cursor
    for i in 0..d.node_count {
        let opc = d.op_at(i);
        let v = unsafe {
            match opc {
                op::CONST => d.const_at(d.operand(i, 0, 1) as usize),
                op::INPUT => seed,
                op::NEG => rload(rp, d.operand(i, 0, 1)).wrapping_neg(),
                op::NOT => !rload(rp, d.operand(i, 0, 1)),
                op::SELECT => {
                    if rload(rp, d.operand(i, 0, 3)) != 0 {
                        rload(rp, d.operand(i, 1, 3))
                    } else {
                        rload(rp, d.operand(i, 2, 3))
                    }
                }
                other => {
                    let a = rload(rp, d.operand(i, 0, 2));
                    let b = rload(rp, d.operand(i, 1, 2));
                    match other {
                        op::ADD => a.wrapping_add(b),
                        op::SUB => a.wrapping_sub(b),
                        op::MUL => a.wrapping_mul(b),
                        op::AND => a & b,
                        op::OR => a | b,
                        op::XOR => a ^ b,
                        op::SHL => a.wrapping_shl(b as u32),
                        op::SHR => a.wrapping_shr(b as u32),
                        op::MIN => a.min(b),
                        op::MAX => a.max(b),
                        op::EQ => (a == b) as u64,
                        op::LT => (a < b) as u64,
                        _ => 0,
                    }
                }
            }
        };
        unsafe { rstore(rp, i, v) };
        if mask.is_sink[i] {
            out[oc] = v; // emit at the sink, in node order
            oc += 1;
        }
    }
    checksum(&out[..oc])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{encode, REC24};
    use crate::optimize::sinks;
    use crate::{generate, GenParams};

    #[test]
    fn output_strategies_agree() {
        // The three output-building strategies deliver the identical sink values
        // (same checksum) across every profile and seed, so the stage measures
        // output materialization and nothing else. inline emits in node order,
        // which equals sink order (sinks() returns ascending node indices), so it
        // matches the compact gather and the full checksum_at.
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 700;
            let prog = generate(&gp);
            let sk = sinks(&prog);
            let mask = SinkMask::new(prog.nodes.len(), &sk);
            let bytes = encode(&prog, &REC24);
            let d = Decoded::parse(&bytes, REC24).unwrap();
            let mut scratch = vec![0u64; prog.nodes.len()];
            let mut out = vec![0u64; sk.len()];
            for seed in [0u64, 1, 42, 12345, 999_999] {
                let cs_full = build_full(&d, seed, &mut scratch, &mask);
                let cs_compact = build_compact(&d, seed, &mut scratch, &mask, &mut out);
                let cs_inline = build_inline(&d, seed, &mut scratch, &mask, &mut out);
                assert_eq!(cs_full, cs_compact, "{name} seed {seed}: compact diverged from full");
                assert_eq!(cs_full, cs_inline, "{name} seed {seed}: inline diverged from full");
            }
        }
    }

    #[test]
    fn sinks_are_ascending() {
        // inline emits in node order and relies on sinks() being ascending so the
        // emit order matches the compact gather order.
        let prog = generate(&GenParams::profile("real").unwrap());
        let sk = sinks(&prog);
        assert!(sk.windows(2).all(|w| w[0] < w[1]), "sinks must be ascending node indices");
    }
}
