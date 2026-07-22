# The vehje bench evidence through the real-runtime lens (Andy Wingo)

## Verdict in one line

The design calls are the ones I would ship, but the evidence is stronger on strategy rankings than on the absolute-magnitude claims the topic hangs its headlines on, and the single load-bearing number ("native is only 1.2x") is measured against an idealized interpreter that vehje will not actually ship.

## What holds (a runtime engineer can bank these)

- **Branch-strategy tier reversal.** `BRANCH_STRATEGY_MAP.md` is the best-built evidence here: real harness, cdylib subprocess isolation, cross-validated byte-exact xor-reduction, `#[inline(never)]` arms so a branch stays a branch, `normalise` baseline. Interp never eval-all (whole-program `pred` = 4.0x the field); native eval-all wins cheap-wide-unpredictable (match8 +26% vs table). This is real and it generalizes.
- **JIT value tracks interpreter quality.** `branch_tier` (interp 9-12x slower than native for a *naive* bytecode loop) reconciled with the 1.2x ceiling is the most honest result in the corpus. It is exactly right: the JIT is worth 10x against a tree-walker and 1.2x against a good register VM.
- **Register VM over stack VM** (`stack-vs-register-vm`, 3.03 vs 5.65 ns/expr): the mechanism is instruction count, matches Lua 5.0-to-5.1 and the Shi register-vs-stack literature. Bankable.
- **NaN-box over tagged-8B** for float-carrying consumers (`value-representation`, 0.96 vs 1.17 ns): modest, well-trodden, correct.
- **The compile-heavy / trivial-execute profile** (`e2e-integration`: 3.24 ms compile, 127 us runtime) matches vehje's actual target (authoring/templating that gathers-and-emits). This, not the JIT sizing, is where the architecture's value sits.

## Findings

1. **"Switch beats tail-threading" is a toolchain-and-workload verdict, not a refutation of Deegen.** `interp-dispatch/v_tail.zig` uses `callconv(.c)` with no `preserve_none`, so every handler pays a callee-save prologue/epilogue. Deegen's win *is* `preserve_none`. The findings admit this ("16-byte frame prologue/epilogue per op"); the topic does not, framing it as "refutes a dispatch assumption from Deegen." It refutes tail-threading-without-preserve_none on Zig 0.16, which is a fine reason to default to switch *now*, but not a dispatch-shape law.
2. **The dispatch program is a near-worst-case for threading.** `common.zig` generates a *random* stream of 3 ops. Threaded dispatch exists to exploit op-sequence *correlation* a shared switch branch cannot learn; a random stream deletes exactly that signal and hands switch its best case. Real bytecode is highly correlated. The ranking may not survive a realistic op stream.
3. **The 24-byte record win is real in direction, inflated in magnitude.** `record-width/rec16.zig` spills *all three* operands to a pool and loops `while (k<arity)`; a careful 16B (inline a,b, spill only c, single load, no loop) was never benched. And the win exists only because res[] reads are L1-hot backward-local, so node bytes are free (latency-bound, not bandwidth-bound). Fine day-one call; "precisely optimal, 16<24=32" is softer than stated.
4. **"Native is 1.0-1.2x over a good interpreter" is measured against an interpreter vehje will not ship.** `ceiling.zig`'s interpreter is a 2-instruction, untyped-i64, no-CFG, L1-only register loop. The real vehje interpreter adds NaN-box unboxing (+~1 ns/op, `value-representation`), CFG block transitions (1.70 ns/instr, `cfg-interp`), and 24B node loads. All slower, so native's real advantage on scalar-compute code is >1.2x. The corpus's own `branch_tier` point 1 says the honest range is 1.2x-to-10x; the topic collapses it to the optimistic end.
5. **The "good interpreter" is a composite never benched as one artifact.** switch + register + NaN-box + CFG + 24B are each measured in isolation and asserted additive. Additivity is not free: NaN-box unbox can serialize with dispatch; CFG back-edges perturb the switch predictor. The e2e capstone's interp stage (5.4 ns/node) is the plain arena walk, not the fast composite.

## The design-mapping check

- **Record (24B, 3 inline):** supported directionally; safe as day-one. Magnitude soft (finding 3).
- **Dispatch (switch, tail rejected):** correct default *for the current toolchain*; the "rejected" stamp over-hardens (findings 1-2).
- **Native tier (non-primary, 1.2x ceiling):** the *conclusion* (build interpreter first, native opt-in) is right; the *number* is optimistic (findings 4-5). Copy-and-patch losing to LLVM at scale (`branch_tier`) is genuine and the "stencil must apply the branch map" fix is sound.
- **Branch strategy (tier-dependent portfolio):** fully supported. Strongest mapping in the set.
- **CFG-of-blocks, NaN-box, register VM:** all supported.

## What over-claims or does not transfer

- **Method overstated:** the topic says the interpreter benches "ran through the mockspace harness (cdylib isolation, cross-validated, hardware-counter timing)." They did not: `interp-dispatch`, `record-width`, `cfg-interp`, `value-arena` are standalone Zig ReleaseFast binaries with hand-rolled 9-pass-median monotonic-clock timing, no isolation, no harness cross-validation (`dispatch.csv` is empty). Only the branch suite and `hx_*` ports use the harness. Even there, `perf.rs` shows counters are feature-gated and default-off (return zeros); timing is wall-clock `Instant`. "Hardware-counter timing" is inaccurate.
- **Mockup-over-synthetic:** absolute ns come from a mockup interpreter over random/nested-loop programs. Dispatch and branch rankings are precisely the results most sensitive to real op-sequence entropy, which synthetic streams do not carry.

## Open questions for the synthesiser

- Rebench tail-threading *with* `preserve_none` on a *correlated* op stream before stamping it rejected. Tradeoff: switch is simpler and portable now; preserve_none may reclaim a real win on realistic streams and larger op sets, at the cost of a calling-convention dependency.
- Bench the *actual composite* interpreter (switch+register+NaN-box+CFG+24B) as one artifact against native, before hardening the 1.2x ceiling and the copy-and-patch priority. Tradeoff: if the composite is meaningfully slower, native's value and copy-and-patch's rank both rise.
- Fair 16B rematch (inline-2 spill-1, no loop) vs 24B, at both L1-hot and larger-than-L1 working sets. Tradeoff: 24B inline-3 simplicity vs 16B density that re-enters once res[] spills L1 in real programs.
