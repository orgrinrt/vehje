# CFG interp throughput: control-flow-heavy per-instruction cost

**Strength: measurement** (CNTVCT_EL0, 5 runs median; the interpreted result cross-validated against a direct
Rust computation of the same kernel, the oracle). The audit's fifth standalone scaling bench, ported onto the
carrier as a register-CFG interpreter.

## The question

Every other interp bench here is straight-line. The vehje runtime also has a CFG-of-blocks interpreter tier,
whose profile differs: a large fraction of executed steps are block terminators (branches, loop back-edges).
The workload is a nested-loop numeric kernel (`acc += seed * inner_counter` over outer x inner iterations) run
through a small register VM (8 registers, blocks ending in jump / conditional-branch / return).

## Result

| outer | inner | instrs | terminators | steps | time | ns/step | cyc/step | ns/instr | control-flow |
|---|---|---|---|---|---|---|---|---|---|
| 2000 | 16000 | 96.0M | 32.0M | 128.0M | 383.7 ms | 3.00 | 9.6 | 4.00 | 25% |

## The finding

The register-CFG interpreter runs at ~3.0 ns/step (~9.6 cycles), with 25% of executed steps being control-flow
terminators. Two things this says:

- **The per-step cost is interpreter dispatch overhead, not the register op.** A register op (read two
  registers, compute, write one, all in registers or L1) is a cycle or two of real work; the ~9.6 cycles/step
  is the interpreter's own control flow: the `match` on the opcode, the block-instruction loop, the terminator
  `match`. Interpretation overhead dominates a tight in-cache kernel, where there is no large working set to
  stream (unlike the straight-line interp streaming a 96 MB wire at ~9.8 ns/op). Against a native compiled loop
  (~1-2 cycles/iteration), this is roughly 5x to 9x, which is the interpreter tax on a hot loop and the
  motivation for a native tier where such loops are hot.
- **The 25% control-flow is nearly free here because the branches are predictable.** The loop-head branches are
  taken N times then fall through, a pattern the branch predictor learns, so the terminators are as cheap as
  the arithmetic steps. This is the predictable case; a data-dependent branch (an unpredictable conditional in
  the loop body) would mispredict and cost far more per terminator. The 25% figure is the control-flow density,
  not a misprediction cost; the honest headline is "control-flow-heavy but predictable is cheap."

Design implication (op's call): the CFG interpreter tier is fine for cold or control-light code, but a tight
hot loop pays a ~5-9x interpretation tax that a native tier removes; the predictability of the loop's own
branches means the control-flow density alone does not make the interpreter slow, only unpredictable branches
would.

## Cost-model / boundary

3.0 ns/step at 3.2 GHz is 9.6 cycles/step, consistent with an interpreter dispatching per instruction (opcode
match + loop bookkeeping + terminator match) over an all-in-cache working set (5 blocks, 8 registers). Boundary:
a single kernel shape (nested loop, predictable branches). The unpredictable-branch case is the natural
follow-up variant; here the named bench is the loop kernel, whose branches predict, so this measures
control-flow density cost, not misprediction cost.
