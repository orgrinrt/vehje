# CFG interp throughput: predictable vs unpredictable control flow

**Strength: measurement** (CNTVCT_EL0, 5 runs median; every kernel's interpreted result cross-validated
against a direct Rust oracle). The audit's fifth standalone scaling bench, ported onto the carrier as a
register-CFG interpreter, plus a predictability variant that isolates the branch-misprediction cost.

## The question

The vehje runtime has a CFG-of-blocks interpreter tier whose profile differs from the straight-line interp: a
large fraction of executed steps are block terminators. Two sub-questions: what does control-flow-heavy
interpretation cost, and how much of that cost is branch MISPREDICTION versus control-flow density? The
branchy kernels answer the second: both do the identical work per iteration (one MUL, one ADD, one AND, then a
branch on a parity bit); only the branch SOURCE differs. Predictable branches on the loop counter's parity (a
learnable ABAB alternation); unpredictable branches on a per-iteration LCG value's parity (~50/50, unlearnable).

## Result (register-CFG interp)

| kernel | steps | ns/step | cyc/step | control-flow |
|---|---|---|---|---|
| nested-loop            | 128.0M | 2.83 | 9.0  | 25% |
| branchy-predictable    | 108.0M | 3.03 | 9.7  | 44% |
| branchy-unpredictable  | 108.0M | 3.24 | 10.4 | 44% |

Unpredictable minus predictable (same kernel): **+0.21 ns/step (+0.7 cycles), the misprediction tax.**

## The finding

Two things, both design-relevant:

- **Per-step interpretation cost is dispatch overhead, not the register op.** ~9-10 cycles/step for
  register-only arithmetic in cache is the interpreter's own control flow (opcode match, block-instruction
  loop, terminator match), consistent with the ~5-9x interpretation tax over a native loop.
- **Branch misprediction is real but diluted.** A 50%-mispredicting branch adds only +0.7 cycles/step on
  average. Per iteration that is ~+6 cycles for the one data-dependent branch (nine steps per iteration times
  0.7), which is the full M1 mispredict penalty (~13 cycles) at a 50% miss rate. So the misprediction costs the
  whole penalty per mispredicting branch, but because it is one branch among ~nine interpreted steps, its
  per-step average impact is modest (+7%). The interpreter's already-high per-step latency and the
  out-of-order engine absorb most of the branch cost that would dominate on tight native code.

Design implication (op's call): the CFG interpreter tier pays the branch-mispredict penalty in full per
data-dependent branch, so branch-dense code with unpredictable conditions is the case where the interpreter
hurts most and a native tier helps most; but for arithmetic-heavy code with a modest branch density, even
fully-unpredictable branches raise the per-step cost only ~7%, because interpretation overhead dominates.
Control-flow DENSITY alone (the 25% vs 44% here) is not what makes the interpreter slow; unpredictable
branches are, and only in proportion to how many steps are branches.

## Cost-model / boundary

The +6 cycles/iteration misprediction cost matches a ~13-cycle M1 branch-mispredict penalty at a ~50% miss
rate; the predictable and unpredictable kernels are byte-identical in work (one MUL, one ADD, one AND per
iteration) so the difference is the branch source alone, physically consistent. Boundary: one data-dependent
branch per iteration; a kernel with several unpredictable branches per iteration would scale the tax
proportionally (the per-step average would rise toward the full per-branch penalty).
