# Sketch: does the typestate survive erasure into data?

**Hypothesis.** A Rust-side typestate invariant, erased into untyped bytes and
crossed to Zig, survives specialisation implicitly: any shape that violates it
becomes unrepresentable rather than merely being checked. Standing call 12 holds
this as conviction; standing call 18 makes its failure existential for the
project.

**Question, framed per call 20.** Not "can the runtime verify this operation is
well formed" (per-instance verification, needs the instance, puts the prover on
the program side). Whether a malformed operation can be **built** at all.

**Outcome: WORKS**, under one condition. See `findings.md`.

Run: `zig test proof.zig` (Zig 0.16.0). Seven tests.

Nothing here sees a program, a script, or an IR. Only an operation definition.
