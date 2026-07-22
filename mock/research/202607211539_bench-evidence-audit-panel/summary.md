# Bench-evidence audit panel: summary

Round: an independent audit of `mock/design_rounds/202607211347_topic.bench-and-sketch-evidence-synthesis.md`
and the benches and sketches it consolidates. Four reads, three parallel specialists plus a synthesiser:
McSherry (benchmark honesty), Wingo (runtime reality), Jhala (verification), Giesen (synthesis, plus two new
experiments run for the audit). Shared brief: `00_context.md`. This file indexes the four; the detail and the
line anchors live in them.

## Verdict

Not dependable as written. Roughly half the corpus is bankable, and the unbankable half is concentrated in
exactly the findings the topic marks "these change a decision." Every architecture call survives; what fails is
five specific numbers or claims that are, respectively, an optimizer artifact, a no-op loop, an arithmetic
model, a strawman comparison, and a false methodology claim. `fabian_giesen_synthesis.md:576-602`.

## Two findings Giesen proved by experiment (neither specialist caught these)

- The native 1.0x-to-1.2x ceiling is an LLVM partial-evaluation artifact. `ceiling.zig`'s two-instruction
  program is a comptime-visible constant, so the optimizer specialises the interpreter into native code (the
  first Futamura projection, done by the compiler) and the bench compares native to native. Laundered through an
  opaque pointer (the condition a real runtime always operates under): 1.2x becomes 10.7x on the same workload,
  cross-validation still passing; corrected interp 3.5 ns/op agrees with the standalone dispatch bench, the
  cross-check that says which number is real. Overturns the topic's third headline.
  `fabian_giesen_synthesis.md:21-27, 99-116, 243-245`.
- The record-width benches never measured the widths they name. Compiler-checked: the "16-byte" and "24-byte"
  `extern struct`s are 12 and 20 bytes; "24 is precisely optimal" cites two strides that were never measured,
  and a true-16-byte three-operand layout was never benched and could win outright.
  `fabian_giesen_synthesis.md:228-232, 507-520`.

## McSherry, benchmark honesty (`frank_mcsherry_benchmark-honesty.md`)

- reach-fixpoint "whole-column OR beats delta semi-naive": the delta variant is a strawman (`reach.zig:24` scans
  all 16M edges every round, does strictly more work than whole-column, builds no out-edge index, and the source
  comments an index it never implements while calling itself "true semi-naive"). Never run fairly at any scale;
  the `hx_reach` "confirmation" is a third toy algorithm at 64 nodes. 8M/50ms is single-shot with an empty CSV.
- mod-stack warm 0.003ms / 1300x: tautological. `stack.zig:31` is `const changed = (i == 1234)`, a no-op branch,
  no cache, no content hash. Cold 4ms is a condensed model (findings' own full-pipeline estimate ~54ms).
- cross-mod DAG 7.7x: frictionless arithmetic (`depdag.zig:57`, `ceil(width/8)*const`, no threads or barriers)
  reported as beating the real 5.64x measurement. Honest net cold-load is 4.2x (Amdahl, with the 6.2ms interner
  tail), a figure that already exists in the artifacts and the topic drops.
- Genuinely honest and real: incremental-scaling node-visits (conservative, worst-case path), the real-threaded
  5.64x parallel compile, egraph 5.2x-linear and interner 19ns (both with committed CSVs).

## Wingo, runtime reality (`andy_wingo_runtime-reality.md`)

- The 1.2x native ceiling is measured against an interpreter vehje will not ship (2-instruction, untyped, no
  CFG, L1-only); the real edge is above 1.2x, the corpus's own `branch_tier` says 1.2x-to-10x, and the "good
  interpreter" is five mechanisms benched separately and asserted additive, never run as one artifact.
- "Switch beats tail-threading" is a toolchain verdict, not a refutation of Deegen: the tail variant is
  `callconv(.c)` with no `preserve_none` (Zig 0.16 lacks it entirely) over a random 3-op stream that deletes the
  correlation threading exists to exploit; the 11-op variant already narrows the gap from 39% to 10%.
- Method overstated: interp-dispatch, record-width, cfg-interp, value-arena are standalone Zig binaries with
  hand-rolled wall-clock timing and empty CSVs, not the harness the topic credits; and the harness's
  hardware-counter timing is feature-gated, default-off, not enabled (`perf.rs` returns zeros), so every number
  is wall-clock.
- Bankable: the branch tier-reversal (real harness), register-VM over stack, NaN-box, the compile-heavy profile.

## Jhala, verification (`ranjit_jhala_verification.md`)

- The uniform "WORKS / proven" language flattens a real strength gradient. Only SK2's compile-time budget-fit is
  exhaustive over its decidable predicate (a genuine proof); only the u64-overflow defense and the dual-locus
  bit-identity carry a structural argument independent of one run; the N*W bound, the effect-lattice check, the
  tnum soundness, and the load-verifier's five rejections are single demonstrations.
- The load verifier guards the untrusted-input security boundary with zero asserts, six `std.debug.print` lines,
  and one human eyeball. Needs an executable test gate.
- Caveat-erasure: per-row qualifiers in the sketches (alignment for zero-copy, u64-width bounds math,
  single-domain Handle check) are compressed into the same green stamp as the unconditional rows.

## The correction path (Giesen C1-C7 plus the carrier: `fabian_giesen_synthesis.md:253-595`)

- Minimal set before the topic hardens into design text: C1 retract the 1.2x native ceiling (re-measure a
  composite shipped-interpreter variant on the existing `branch_tier` harness, about half a day, or re-frame to
  the honest bracket and rest tier-priority on the capstone profile); C2 fix the record-width labels and run the
  widened rematch including the never-benched true-16-byte three-operand layout, before the wire format locks
  (the one day-one irreversible consumer); C3 soften the tail-threading verdict to toolchain-bound (text only);
  C4 swap the incremental-scaling headlines for the honest figures already in the artifacts and make the
  warm-load bench measure a real cache (about an hour each); C7 correct the methodology paragraph (wall-clock,
  counters off, which benches were standalone) and re-run the five empty-CSV benches. C5 (a real semi-naive) and
  C6 (gate the load verifier as `test` blocks, add strength labels) are one to two hours each.
- The carrier architecture endorses op's harness-in-harness instinct, adjusted: one `vehje-bench-carrier`
  library (the real wire-format IR with stride asserts, seeded deterministic generators over the axes the corpus
  kept improvising, one reference interpreter whose contested axes are variant points, a checksum contract) plus
  two drivers (the mockspace harness under the 16KB cap; one shared scale-runner above it). Two structural
  wins: FFI-borne programs make the partial-evaluation artifact impossible by construction, and one-axis-off-one-
  operating-point makes the numbers compose instead of being asserted additive. About two to three days; the
  C1/C2/C5 rematches land as its first benches, so repair and infrastructure are the same work. Upstream ask:
  mockspace documents its variant symbol contract as a plain C ABI so a Zig cdylib is a first-class variant.
- Ten bench-writing rules distilled from the failures (`fabian_giesen_synthesis.md:473-505`): workload data
  crosses an opacity boundary; assert every named layout; a cost-model sanity line in every findings file; the
  committed CSV is the bench; label models vs measurements vs proxies at every citation; a comparative claim
  needs the strongest good-faith opponent; the mechanism under test must exist in the code; single-shot numbers
  do not conclude; ranking stops at the noise floor; every caveat survives summarization.

## Bottom line

No architecture call moves: the interpreter floor, the wide-record lean, switch dispatch, content-addressed
caching, and the tier-dependent branch portfolio all remain right, most now for better-stated reasons. What
changes is that in five named places the topic would be citing measurements instead of an optimizer artifact, a
no-op loop, an arithmetic model, a strawman, and a false method claim. The evidence layer is worth having once
it is exactly as strong as it says it is. `fabian_giesen_synthesis.md:597-602`.
