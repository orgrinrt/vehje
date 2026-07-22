//! vehje-bench-carrier: the shared IR, generators, and reference interpreter
//! that every vehje bench varies exactly one axis of.
//!
//! The design answers the audit that found the earlier corpus full of bespoke
//! toy interpreters with divergent bugs (a partially-evaluated ceiling, a
//! mislabeled record stride, empty CSVs). Four things live here once instead of
//! per-bench: the wire-format IR with compile-time-asserted layouts (`ir`), the
//! seeded deterministic generators over the axes benches kept improvising
//! (`gen`), the reference interpreter whose contested axes are variant points
//! (`interp`), and the checksum contract that cross-validates variants
//! (`checksum`).
//!
//! Two properties are structural, not disciplinary. The program crosses into a
//! variant as wire bytes, so a variant's optimizer cannot see the program and
//! cannot partially evaluate the interpreter over it (the failure that made the
//! native-ceiling bench measure native-versus-native). And every bench moves one
//! axis off one shared operating point (`GenParams::default_point` plus the
//! shipped interpreter defaults), so the measurements compose by construction
//! rather than being asserted additive across incompatible loops.
//!
//! The program format and checksum are language-neutral, so a Zig cdylib
//! variant can consume identical program bytes for the few questions where the
//! shipped language is load-bearing (dispatch shape, the real runtime, and
//! wire-format differential); every other bench is Rust-default.

// The threaded interpreter uses two incomplete nightly features (the
// preserve-none calling convention and guaranteed tail calls) that only the
// preserve_none dispatch variant opts into, so they are gated behind a cargo
// feature and never touch the default carrier build the other variants use.
#![cfg_attr(feature = "threaded", feature(explicit_tail_calls, rust_preserve_none_cc))]
#![cfg_attr(feature = "threaded", allow(incomplete_features))]
// Vertical/SoA data-parallel interpretation uses portable SIMD, gated so only
// the vertical cells pull the nightly feature.
#![cfg_attr(feature = "vertical", feature(portable_simd))]

pub mod access;
pub mod cfg;
pub mod checksum;
pub mod eqsat;
pub mod gen;
pub mod incr;
pub mod interp;
#[cfg(feature = "threaded")]
pub mod interp_threaded;
pub mod ir;
pub mod native;
pub mod optimize;
pub mod predecode;
pub mod reach;
#[cfg(feature = "vertical")]
pub mod vertical;
pub mod retract;
pub mod sharded_intern;
pub mod thermo;
pub mod valrepr;

pub use checksum::Checksum;
pub use gen::{generate, GenParams, Rng};
pub use access::checksum;
pub use interp::{
    interpret, interpret_bittree, interpret_fntable, interpret_ifchain, interpret_ifchain_ascending,
    interpret_nulldispatch, run_over_input,
};
pub use predecode::{
    interpret_predecoded, interpret_predecoded_fntable, interpret_predecoded_nulldispatch,
    interpret_predecoded_regcache, predecode, Predecoded,
};
#[cfg(feature = "threaded")]
pub use interp_threaded::interpret_threaded;
pub use native::{madd_bytes, madd_program, native_madd};
pub use ir::{
    encode, Decoded, Layout, Node, Program, ALL_LAYOUTS, REC12, REC16, REC20, REC24, REC32,
};

/// Build the wire bytes for a program of `node_count` nodes at `layout`, from
/// the default operating point. A variant calls this once per process (the
/// orchestrator gives each variant process a single size) to get the program it
/// interprets. Every record-width variant generates the same program and only
/// changes the layout, so their checksums must agree; the cross-validation in
/// the harness enforces exactly that.
pub fn program_at(node_count: usize, layout: Layout) -> Vec<u8> {
    let mut p = GenParams::default_point();
    p.node_count = node_count;
    encode(&generate(&p), &layout)
}

/// Like [`program_at`] but with a chosen op-vocabulary size, for the dispatch
/// bench that sweeps how the switch's prediction advantage depends on how many
/// distinct ops the stream draws from.
pub fn program_vocab(node_count: usize, op_vocab: u8, layout: Layout) -> Vec<u8> {
    let mut p = GenParams::default_point();
    p.node_count = node_count;
    p.op_vocab = op_vocab;
    encode(&generate(&p), &layout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_program_is_well_formed() {
        let prog = generate(&GenParams::default_point());
        assert!(prog.is_well_formed());
        assert_eq!(prog.nodes.len(), GenParams::default_point().node_count);
    }

    #[test]
    fn profiles_well_formed_distinct_and_agree() {
        // Each designed profile generates a well-formed program; the profiles are
        // genuinely distinct (distinct checksums, so they are not accidentally the
        // same program); and within a profile every wire dispatch shape agrees on
        // the checksum (the cross-validation the matrix rests on).
        let names = ["real", "madd", "tight", "scatter", "wideselect", "leaf"];
        let mut seen: Vec<u64> = Vec::new();
        for name in names {
            let mut gp = GenParams::profile(name).expect("profile exists");
            gp.node_count = 600;
            let prog = generate(&gp);
            assert!(prog.is_well_formed(), "profile {name} ill-formed");
            let bytes = encode(&prog, &REC24);
            let d = Decoded::parse(&bytes, REC24).unwrap();
            let mut r = vec![0u64; prog.nodes.len()];
            interpret(&d, 12345, &mut r);
            let sw = checksum(&r);
            let shapes: [fn(&Decoded, u64, &mut [u64]); 4] = [
                interpret_fntable,
                interpret_ifchain,
                interpret_ifchain_ascending,
                interpret_bittree,
            ];
            for f in shapes {
                f(&d, 12345, &mut r);
                assert_eq!(sw, checksum(&r), "profile {name}: a dispatch shape diverged");
            }
            assert!(!seen.contains(&sw), "profile {name} is not distinct from an earlier profile");
            seen.push(sw);
        }
    }

    #[test]
    fn cross_layout_identical_checksum() {
        // The cross-validation contract: one program run through every record
        // layout must produce a byte-identical checksum. Only the stride and the
        // inline-versus-pool operand path differ; the semantics never do. This
        // is what lets the record-width bench trust that it is measuring layout
        // cost and not an accidental behaviour change.
        let mut p = GenParams::default_point();
        p.node_count = 512;
        let prog = generate(&p);
        let input: Vec<u8> = (0..=255u8).cycle().take(1024).collect();
        let mut results = vec![0u64; prog.nodes.len()];

        let mut reference: Option<u64> = None;
        for layout in ALL_LAYOUTS {
            let bytes = encode(&prog, &layout);
            let d = Decoded::parse(&bytes, layout).expect("wire bytes parse");
            assert_eq!(d.node_count, prog.nodes.len());
            let cs = run_over_input(&d, &input, &mut results);
            match reference {
                None => reference = Some(cs),
                Some(r) => assert_eq!(cs, r, "layout {} diverged from reference", layout.name),
            }
        }
    }

    #[test]
    fn checksum_depends_on_input() {
        // If the result did not depend on the input bytes, the interpreter would
        // be a hoistable constant and the bench would measure nothing. Distinct
        // inputs must give distinct checksums.
        let prog = generate(&GenParams {
            node_count: 256,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut results = vec![0u64; prog.nodes.len()];
        let a = run_over_input(&d, &[1, 2, 3, 4], &mut results);
        let b = run_over_input(&d, &[9, 8, 7, 6], &mut results);
        assert_ne!(a, b);
    }

    #[test]
    fn native_madd_matches_interp() {
        // The shape-specialized native madd loop must fold the identical hash as
        // the switch interpreter over the same madd program, so the ceiling
        // bench's interp-vs-native ratio is dispatch overhead and not a
        // difference in what was computed.
        let prog = native::madd_program(64);
        assert!(prog.is_well_formed());
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r = vec![0u64; prog.nodes.len()];
        for seed in [0u64, 1, 42, 12345, 999_999] {
            interpret(&d, seed, &mut r);
            assert_eq!(
                checksum(&r),
                native::native_madd(&d, seed),
                "native diverged from interp at seed {seed}"
            );
        }
    }

    #[test]
    fn dispatch_shapes_agree() {
        // The switch interpreter and the function-pointer-table interpreter must
        // compute identical results, so the dispatch bench measures dispatch
        // cost and nothing else.
        let prog = generate(&GenParams {
            node_count: 400,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r = vec![0u64; prog.nodes.len()];
        for seed in [0u64, 1, 42, 255, 1000] {
            interpret(&d, seed, &mut r);
            let sw = checksum(&r);
            let shapes: [fn(&Decoded, u64, &mut [u64]); 4] = [
                interpret_fntable,
                interpret_ifchain,
                interpret_ifchain_ascending,
                interpret_bittree,
            ];
            for f in shapes {
                f(&d, seed, &mut r);
                assert_eq!(sw, checksum(&r), "a wire dispatch shape diverged at seed {seed}");
            }
        }
    }

    #[test]
    fn spill_path_matches_inline_path() {
        // A high-arity op (SELECT, arity 3) is inline in REC16 but spills to the
        // pool in REC12 (two inline slots). Both must compute the same result,
        // which is the specific correctness the pool-spill decode path needs.
        let mut prog = Program::default();
        prog.consts = vec![7, 11, 13];
        prog.nodes.push(Node { op: ir::op::INPUT, operands: vec![] });
        prog.nodes.push(Node { op: ir::op::CONST, operands: vec![0] });
        prog.nodes.push(Node { op: ir::op::CONST, operands: vec![1] });
        prog.nodes.push(Node { op: ir::op::CONST, operands: vec![2] });
        prog.nodes.push(Node { op: ir::op::SELECT, operands: vec![0, 2, 3] });
        assert!(prog.is_well_formed());
        let mut results = vec![0u64; prog.nodes.len()];
        let input = [0u8, 1, 2, 3, 4];

        let b12 = encode(&prog, &REC12);
        let b16 = encode(&prog, &REC16);
        let c12 = run_over_input(&Decoded::parse(&b12, REC12).unwrap(), &input, &mut results);
        let c16 = run_over_input(&Decoded::parse(&b16, REC16).unwrap(), &input, &mut results);
        assert_eq!(c12, c16);
    }
}
