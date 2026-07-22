//! Liveness slot allocation: the value arena a real runtime uses.
//!
//! The other interpreters store every node's result into a `results[node_count]`
//! array kept live for the whole pass, so the working set is the entire program
//! and every form/cache crossover is measured over that whole array. A real
//! runtime does register/slot allocation: a backward last-use pass assigns each
//! value a slot in a small arena and frees the slot when the value dies, so the
//! resident working set is the program's liveness WIDTH (a small constant for
//! typical DAGs), not the node count. This attacks the exact term that dominated
//! the large-program regime.
//!
//! The program is unchanged; only where values live changes. Cross-validation is
//! on the live-out (sink) values, read from their slots, which are preserved.

use crate::ir::{op, Program};
use crate::optimize::sinks;

/// A node in slot form: operands and destination are slot indices (except a
/// CONST's `a`, which stays a const-pool index).
#[derive(Clone, Copy)]
pub struct SlotNode {
    pub op: u8,
    pub dst: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

/// A slot-allocated program: nodes over a slot arena of `num_slots`, plus the
/// slots holding the live-out values.
pub struct SlotProgram {
    pub nodes: Vec<SlotNode>,
    pub consts: Vec<u64>,
    pub num_slots: usize,
    pub out_slots: Vec<u32>,
}

/// Allocate slots by liveness. A value's slot is freed at its last use and reused
/// by a later value, so the arena is sized to the live width; live-outs are never
/// freed so they survive to the final fold.
pub fn slot_allocate(prog: &Program) -> SlotProgram {
    let n = prog.nodes.len();

    // last use of each node: the largest index that references it as a node
    // operand. Live-outs are pinned (never die) with usize::MAX.
    let mut last_use = vec![0usize; n];
    for (j, node) in prog.nodes.iter().enumerate() {
        if node.op != op::CONST {
            for &o in &node.operands {
                last_use[o as usize] = j;
            }
        }
    }
    let mut pinned = vec![false; n];
    for &s in &sinks(prog) {
        pinned[s as usize] = true;
        last_use[s as usize] = usize::MAX;
    }

    let mut slot_of = vec![0u32; n];
    let mut free: Vec<u32> = Vec::new();
    let mut next_slot: u32 = 0;
    let mut slot_nodes: Vec<SlotNode> = Vec::with_capacity(n);

    for i in 0..n {
        let node = &prog.nodes[i];
        // record operand slots (assigned for earlier nodes) BEFORE freeing/alloc.
        let (a, b, c) = match node.op {
            op::INPUT => (0, 0, 0),
            op::CONST => (node.operands[0], 0, 0), // pool index, not a slot
            op::SELECT => (
                slot_of[node.operands[0] as usize],
                slot_of[node.operands[1] as usize],
                slot_of[node.operands[2] as usize],
            ),
            op::NEG | op::NOT => (slot_of[node.operands[0] as usize], 0, 0),
            _ => (
                slot_of[node.operands[0] as usize],
                slot_of[node.operands[1] as usize],
                0,
            ),
        };

        // free operands that die at i (not live-outs) so the dst can reuse them.
        // Safe: a node reads all operands then writes dst, and a dead operand has
        // no later reader. Dedupe repeated operands (e.g. ADD(o, o)) so a slot is
        // never pushed to the free list twice, which would let two later nodes
        // claim the same slot.
        if node.op != op::CONST {
            let mut freed: [u32; 3] = [u32::MAX; 3];
            let mut nf = 0usize;
            for &o in &node.operands {
                let o = o as usize;
                if !pinned[o] && last_use[o] == i {
                    let s = slot_of[o];
                    if !freed[..nf].contains(&s) {
                        free.push(s);
                        freed[nf] = s;
                        nf += 1;
                    }
                }
            }
        }

        // allocate the destination slot.
        let dst = free.pop().unwrap_or_else(|| {
            let s = next_slot;
            next_slot += 1;
            s
        });
        slot_of[i] = dst;
        slot_nodes.push(SlotNode { op: node.op, dst, a, b, c });
    }

    let out_slots: Vec<u32> = sinks(prog).iter().map(|&s| slot_of[s as usize]).collect();

    SlotProgram {
        nodes: slot_nodes,
        consts: prog.consts.clone(),
        num_slots: next_slot.max(1) as usize,
        out_slots,
    }
}

/// Interpret the slot-allocated program. `slots` is a caller-owned arena of
/// length at least `num_slots`. Fills the arena; the caller folds `out_slots`.
#[inline]
pub fn interpret_slotted(sp: &SlotProgram, input_seed: u64, slots: &mut [u64]) {
    let wp = slots.as_mut_ptr();
    let cp = sp.consts.as_ptr();
    let np = sp.nodes.as_ptr();
    // unchecked slot load/store (arena sized to num_slots, all slots in range).
    #[inline(always)]
    unsafe fn ld(base: *const u64, s: u32) -> u64 {
        *base.add(s as usize)
    }
    for i in 0..sp.nodes.len() {
        let nd = unsafe { *np.add(i) };
        let rp = wp as *const u64;
        let v = match nd.op {
            op::INPUT => input_seed,
            op::CONST => unsafe { *cp.add(nd.a as usize) },
            op::ADD => unsafe { ld(rp, nd.a).wrapping_add(ld(rp, nd.b)) },
            op::SUB => unsafe { ld(rp, nd.a).wrapping_sub(ld(rp, nd.b)) },
            op::MUL => unsafe { ld(rp, nd.a).wrapping_mul(ld(rp, nd.b)) },
            op::AND => unsafe { ld(rp, nd.a) & ld(rp, nd.b) },
            op::OR => unsafe { ld(rp, nd.a) | ld(rp, nd.b) },
            op::XOR => unsafe { ld(rp, nd.a) ^ ld(rp, nd.b) },
            op::SHL => unsafe { ld(rp, nd.a).wrapping_shl(ld(rp, nd.b) as u32) },
            op::SHR => unsafe { ld(rp, nd.a).wrapping_shr(ld(rp, nd.b) as u32) },
            op::MIN => unsafe { ld(rp, nd.a).min(ld(rp, nd.b)) },
            op::MAX => unsafe { ld(rp, nd.a).max(ld(rp, nd.b)) },
            op::EQ => unsafe { (ld(rp, nd.a) == ld(rp, nd.b)) as u64 },
            op::LT => unsafe { (ld(rp, nd.a) < ld(rp, nd.b)) as u64 },
            op::SELECT => unsafe {
                if ld(rp, nd.a) != 0 {
                    ld(rp, nd.b)
                } else {
                    ld(rp, nd.c)
                }
            },
            op::NEG => unsafe { ld(rp, nd.a).wrapping_neg() },
            op::NOT => unsafe { !ld(rp, nd.a) },
            _ => 0,
        };
        unsafe { *wp.add(nd.dst as usize) = v };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::{checksum_at, checksum};
    use crate::ir::{encode, Decoded, REC24};
    use crate::{generate, GenParams};

    #[test]
    fn slotted_preserves_outputs_and_shrinks_arena() {
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 800;
            let prog = generate(&gp);
            let orig_sinks = sinks(&prog);
            let sp = slot_allocate(&prog);
            // the arena is no larger than the node count, and typically much smaller.
            assert!(sp.num_slots <= prog.nodes.len(), "{name} arena not smaller");

            let bytes = encode(&prog, &REC24);
            let d = Decoded::parse(&bytes, REC24).unwrap();
            let mut refr = vec![0u64; prog.nodes.len()];
            let mut arena = vec![0u64; sp.num_slots];
            for seed in [0u64, 1, 42, 12345, 999_999] {
                crate::interp::interpret(&d, seed, &mut refr);
                let want = checksum_at(&refr, &orig_sinks);
                // arena must be re-zeroed per run is not required (all live slots
                // are written before read), but reuse it to catch stale reads.
                interpret_slotted(&sp, seed, &mut arena);
                let got = checksum_at(&arena, &sp.out_slots);
                assert_eq!(want, got, "{name} slotted output diverged at seed {seed}");
            }
            // silence unused import warning on some profiles.
            let _ = checksum(&refr);
        }
    }
}
