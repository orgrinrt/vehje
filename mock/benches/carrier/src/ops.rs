//! The one definition of every op's semantics.
//!
//! The dispatch cells deliberately differ in SKELETON (a switch, a
//! function-pointer table, a frequency-ordered cascade, a bit-tree, a
//! preserve-none handler chain, a SIMD lane loop, a machine-code stencil): that
//! difference is the experiment. What must NOT differ is what each op MEANS. The
//! reviewer found the twelve arithmetic/logical binops hand-transcribed across at
//! least seven sites, so a semantics change (say, making SHR arithmetic instead
//! of logical) had seven places to land correctly and six chances to silently
//! not. A bench built to show "derive every tier from one definition" should not
//! itself be the counterexample.
//!
//! These macros are that single definition for the SCALAR cells. Each cell keeps
//! its own dispatch skeleton and calls `binop_body!(OP, a, b)` / `unop_body!(OP,
//! a)` for the op's semantics, so the meaning of each op lives in exactly one
//! place. `optimize::eval_op` (the const-folder, which MUST agree with the
//! interpreters or folding would change results) uses the same macro, which turns
//! that "must agree" from a discipline into a guarantee. The SIMD cell has its own
//! `binop_simd!` (a genuinely different representation, `Simd<u64, W>` ops, not
//! `u64` ops); the machine-code cells keep their aarch64 encoders / stencils, the
//! irreducibly-different third representation. Every representation, though, is
//! keyed by the same `op` opcodes, so a new op is a compile error everywhere until
//! handled, which is the drift guard the single list buys.

/// Scalar semantics of a binary op, as a `u64` expression over `$a`, `$b`. The
/// single source of truth every scalar cell's binary arms defer to.
macro_rules! binop_body {
    (ADD, $a:expr, $b:expr) => { $a.wrapping_add($b) };
    (SUB, $a:expr, $b:expr) => { $a.wrapping_sub($b) };
    (MUL, $a:expr, $b:expr) => { $a.wrapping_mul($b) };
    (AND, $a:expr, $b:expr) => { $a & $b };
    (OR, $a:expr, $b:expr) => { $a | $b };
    (XOR, $a:expr, $b:expr) => { $a ^ $b };
    (SHL, $a:expr, $b:expr) => { $a.wrapping_shl($b as u32) };
    (SHR, $a:expr, $b:expr) => { $a.wrapping_shr($b as u32) };
    (MIN, $a:expr, $b:expr) => { $a.min($b) };
    (MAX, $a:expr, $b:expr) => { $a.max($b) };
    (EQ, $a:expr, $b:expr) => { ($a == $b) as u64 };
    (LT, $a:expr, $b:expr) => { ($a < $b) as u64 };
}

/// Scalar semantics of a unary op.
macro_rules! unop_body {
    (NEG, $a:expr) => { $a.wrapping_neg() };
    (NOT, $a:expr) => { !$a };
}

/// SIMD semantics of a binary op, as a `Simd<u64, W>` expression over `$a`, `$b`.
/// The vector representation of the same op the scalar `binop_body!` defines, kept
/// beside it so a semantics change is visible in one file. The needed splat
/// constants are constructed inline via `Simd::splat` (a path, so it resolves to
/// the call site's `Simd` import; macro hygiene would hide caller locals). The
/// `SimdOrd` / `SimdPartialEq` / `SimdPartialOrd` / `Select` traits must be
/// imported at the call site (the vertical cell provides them).
#[cfg(feature = "vertical")]
macro_rules! binop_simd {
    (ADD, $a:expr, $b:expr) => { $a + $b };
    (SUB, $a:expr, $b:expr) => { $a - $b };
    (MUL, $a:expr, $b:expr) => { $a * $b };
    (AND, $a:expr, $b:expr) => { $a & $b };
    (OR, $a:expr, $b:expr) => { $a | $b };
    (XOR, $a:expr, $b:expr) => { $a ^ $b };
    // mask the shift amount to 0..63 to match scalar wrapping_shl/shr on u64.
    (SHL, $a:expr, $b:expr) => { $a << ($b & Simd::splat(63u64)) };
    (SHR, $a:expr, $b:expr) => { $a >> ($b & Simd::splat(63u64)) };
    (MIN, $a:expr, $b:expr) => { $a.simd_min($b) };
    (MAX, $a:expr, $b:expr) => { $a.simd_max($b) };
    (EQ, $a:expr, $b:expr) => { $a.simd_eq($b).select(Simd::splat(1u64), Simd::splat(0u64)) };
    (LT, $a:expr, $b:expr) => { $a.simd_lt($b).select(Simd::splat(1u64), Simd::splat(0u64)) };
}

pub(crate) use binop_body;
pub(crate) use unop_body;
#[cfg(feature = "vertical")]
pub(crate) use binop_simd;
