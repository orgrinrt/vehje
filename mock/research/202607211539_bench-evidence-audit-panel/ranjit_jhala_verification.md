# The vehje bench evidence through the verification lens (Ranjit Jhala)

## Verdict in one line

Every correctness sketch is a small set of hand-picked test cases with a WORKS stamp, not a proof and not an exhaustive check; the topic's uniform "proven"/"WORKS" vocabulary erases a real strength gradient that a reader needs to see.

## What is genuinely established, and at what strength

- **Comptime-recursion SIGSEGV wall (SK5):** empirical demonstration, one shape (left-spine tree), one toolchain pin (Zig 0.16.0). Real and reproducible as reported, but a toolchain-specific implementation fact, not a theorem; future Zig may move the wall.
- **Dual-locus bit-identical (sk_dual-locus):** proof-by-construction for the identity claim (one compiled object, two linkers, so every input is trivially identical), corroborated, not established, by a single-input hash match.
- **Budget-fit `@compileError` (SK2):** exhaustive over the decidable predicate `prod(domains) <= CAP`, checked by the compiler at every instantiation. This is the one artifact that earns "proof."
- **Reachability N*W bound (SK4/reach.rs):** the soundness counterexample (broken rule loses `o`) is a genuine disproof (one counterexample refutes a universal claim). The positive claims ("correct rule sound", "bound holds independent of N") are single demonstrations at N=2000, W=4: consistent with the bound, not a proof of it for all N.
- **tnum numeric bounds:** single demonstration per operator; the underlying transfer functions are the literature's (eBPF verifier), so soundness rests on that prior art, not on this sketch's testing.
- **Effect-inclusion thermometer check:** correctness "verified" is three booleans from one hand-picked pair, not exhaustive over the lattice.
- **Adversarial load-verifier:** `std.debug.print` output with "(expect false)" comments, no `assert`, no test harness, six runs total, human-eyeballed once. Weaker than "property-tested"; it is an un-gated single demonstration.

## Findings

1. **N*W naming bound.** Claim: the bound holds for all N given the binder rule. Invariant needed: `|reach(e)| <= W` for every well-formed nested-let term of arbitrary shape and depth. `reach.rs` checks one shape at N=2000. No induction, no property test varying shape/W. Demonstrated, not proven.
2. **tnum multiply fast path.** Claim: "80% both-known" halves the average cost. This is a workload assumption fed into the bench, not a measured property of any real corpus; it sizes a hypothetical, it does not validate the 80% figure.
3. **Effect-inclusion "thermometer makes the lattice free."** The algebraic argument (max = OR under unary encoding) is a real, checkable identity; the sketch checks it at one point, not for all thermometer-code pairs in the 24-family x 3-state space (a 3x3 exhaustive table would have cost nothing and would have been an actual proof over the finite domain).
4. **u64-bounds-math overflow defense.** The mechanism argument (u32+u32 fits in u64 without wraparound) is a real, general proof by bit-width counting, independent of the single crafted test case. The test corroborates; the argument, not the test, is what carries the guarantee. Correctly reasoned, under-labeled as "confirmed by test" rather than "proven by the width argument, tested once."
5. **Bounded multi-shot Handle.** The budget-fit rejection is exhaustively checked (compile-time arithmetic over a decidable predicate). The multi-shot *semantics* (12 resumptions match the enumerated domain) is checked once, on one small domain product; not proven for arbitrary domain shapes.
6. **Dual-locus identity.** Correctly argued as identity-by-construction; the hash match is one input, appropriately treated as defense-in-depth rather than the primary guarantee, per the sketch's own honest framing.

## The certification-language audit

The topic's table stamps SK1, SK4, SK2, SK17, SK16, the MAP_JIT probe, SK20/18/21, the load-verifier, SK12/19/13, SK25 all as "WORKS" with no strength qualifier. "Unrepresentable" (SK1, illegal states unrepresentable via `@Type`/enum) is earned at the type level for the cases exercised; whether it is unrepresentable for the full family space is not shown. "Holds" (bounds, inclusion) is earned only where thermometer-encoding turns the claim into a checked algebraic identity (finding 3); elsewhere "holds" is asserted from one instance. "Proven feasible" is fair for mechanism questions (MAP_JIT, dual-locus linking) and overreaching for correctness questions (N*W, tnum, effect inclusion), which need either exhaustive finite-domain checks or actual induction, neither present.

## What is over-claimed as proven when it is only demonstrated

The N*W bound's positive half, the tnum soundness (beyond the literature's own proof), the effect-lattice correctness (checked on one pair, not the finite lattice), and the load-verifier's "confirmed" framing (no automated assertion gate, print-and-eyeball).

## Open questions for the synthesiser

- Should the topic distinguish, per finding, proof / exhaustive-finite-check / property-test / single-demo, rather than one WORKS bucket?
- Is a cheap exhaustive check over the 3-state x N-family lattice (finding 3) or an induction over let-nesting shape (finding 1) worth doing before these harden into the design text?
- Does the load-verifier sketch need to become an actual `test` block with `assert`, given it guards an untrusted-input security boundary?
