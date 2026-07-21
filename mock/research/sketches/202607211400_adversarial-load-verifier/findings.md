# Adversarial load-verifier robustness (untrusted-path security boundary)

**Date:** 2026-07-21
**Outcome:** WORKS. Every attack arena REJECTED, no crash, correct `false`. Zig 0.16.0, aarch64. Artifact: `adversarial.zig`.
**Settles:** the untrusted structural-decode (SK18) is the security boundary for ARRIVING mods (a mod is
data authored elsewhere, then loaded and run by the composed Zig runtime). Confirm it fails closed on hostile
input rather than trusting the on-disk arena.

## Why this matters

The two-artifact model means the runtime loads validated DATA emitted by the Rust compiler. But a shipped mod
is a data file that crossed a trust boundary: it may be corrupt, truncated, or hand-forged to attack the
loader. The runtime therefore cannot assume the arena satisfies the compiler's invariants; it must re-verify
structurally at load, cheaply, and reject anything malformed. This probes that re-verification against five
attack classes.

## Attacks and results

| attack class | forged violation | result |
|---|---|---|
| valid baseline | well-formed arena | `true` (accepted) |
| cycle / forward child | pool child-index `>= own index` (breaks the backward-acyclic invariant the zero-copy walk relies on) | `false` |
| pool span OOB | node's child-span base past the pool length | `false` |
| blob overrun + index overflow | blob ref `a=0xFFFFFFFF, b=0xFFFFFFFF` (attempts `a+b` wraparound to a small in-range value) | `false` (u64 index math cannot wrap a u32 pair) |
| unknown kind | node kind outside the reified tag set | `false` |
| over-deep | a valid backward chain 200 deep against a depth cap of 16 | `false` |

## The finding: fails closed, and the integer-overflow class is defeated by the u64-math rule

Every hostile arena is rejected without a crash, and each for the right structural reason. The load-time
verifier is a single forward pass that checks, per node: kind is in range, blob refs sit inside the blob,
pool child-spans sit inside the pool, every child index is strictly backward (`< own index`, which is
simultaneously the acyclicity guarantee and what makes the children-first zero-copy walk sound), and the
computed depth stays under the cap.

The load-bearing detail is the **integer-overflow class**. A naive verifier that checks `a + b > len` in u32
arithmetic can be fooled: `0xFFFFFFFF + 0xFFFFFFFF` wraps to `0xFFFFFFFE` in u32, and a further crafted pair
wraps to a small in-range value, passing a bounds check while the real span is enormous. The fix is already in
the decode contract: **all index/bounds arithmetic widens to u64 before the comparison** (`@as(u64, n.a) + n.b`),
so the sum cannot wrap within the u32 operand range and the OOB span is caught. This is a wire-format contract
requirement, not an implementation nicety: the verifier must do bounds math in a width strictly wider than the
index fields it validates. With 32-bit index fields, u64 math is mandatory on the untrusted path.

## Design impact

- The untrusted load path fails closed on all five structural attack classes; the arena format does not need a
  signature/checksum to be *memory-safe* on load (a checksum still helps detect accidental corruption early,
  but is a separate integrity concern from the structural safety proven here).
- Lock into the wire-format contract: **load-time structural verification is mandatory, single-pass, and does
  all bounds arithmetic in u64** (one width wider than the 32-bit index fields). The verifier rejects on: kind
  out of range, blob ref out of range, pool span out of range, any non-backward child index, depth over cap.
- The verifier cost is one extra forward pass over the arena (plus the depth fold), on top of the walk itself.
  It runs once per mod load, amortised across the whole run, so it does not touch steady-state throughput. The
  mod-stack cold-load bench (2000 mods, ~4ms) already includes decode-shaped work, so the verified load stays
  in that budget.

## Artifacts
- `adversarial.zig` (the verifier + five forged arenas + the valid baseline).
