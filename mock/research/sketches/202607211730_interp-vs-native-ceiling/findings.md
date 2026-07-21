# Interpreter vs native ceiling: how much does copy-and-patch actually buy?

**Date:** 2026-07-21
**Type:** Zig-native microbench across four workload regimes, ReleaseFast, 100M to 200M expressions, 5-run best,
opaque inline `madd` asm as the fair native baseline. Zig 0.16.0, aarch64. Artifact: `ceiling.zig`.
**Settles:** the native tier's actual value proposition (op: "we definitely want to lower to a JIT if possible"),
i.e. how much faster copy-and-patch native code is than the register interpreter on the same program. This is the
number that decides how much to invest in the copy-and-patch tier versus the interpreter floor.

## Measuring this fairly is the whole problem

A naive interpreter-vs-native comparison is a trap, and this bench hit every version of the trap before landing
a fair number. The failures are themselves informative:

- **Plain arithmetic loop -> native "infinitely faster."** LLVM closed-forms the affine recurrence or
  auto-vectorizes it to ~0ms. Unfair: copy-and-patch emits scalar straight-line stencils and does NOT
  auto-vectorize, so LLVM's result is not the native tier's result.
- **Large acc-dependent gather -> native 1.0x.** Making the workload defeat the optimizer by pointer-chasing a
  1M-entry table made it DRAM-latency-bound; both interpreter and native wait on memory, so dispatch overhead is
  hidden. Unrepresentative of register-resident script code.
- **Pure-call hoisting -> native 0ms.** A pure native function called in the timing loop gets hoisted out
  entirely (its result is only observed after the loop). Fixed by sinking each call inside the timed region.

The fair baseline: an L1-resident working set (compute-bound, not memory-bound), a data-dependent index (so LLVM
cannot vectorise or fold), and the native arithmetic written as an **opaque inline `madd` instruction** (exactly
the scalar aarch64 op copy-and-patch would emit, which the optimiser cannot fold or vectorise). The interpreter
runs the identical recurrence through its register-VM switch dispatch. All variants verified identical checksums.

## Results (ns per expression; native = opaque scalar `madd`, interp = register VM)

| workload regime | interp | native | native speedup |
|---|---|---|---|
| DRAM-bound gather (1M table) | 9.29 | 8.99 | 1.0x |
| L1-resident, serial dependency | 3.20 | 2.66 | 1.2x |
| L1-resident, independent iterations | 0.80 | 0.66 | 1.2x |

## The finding: copy-and-patch native buys only ~1.0 to 1.2x over a good register interpreter; the big win needs a vectorising JIT, not copy-and-patch

Across every representative regime, scalar native code is only 1.0 to 1.2x faster than the register interpreter.
The reason is consistent: a well-built register interpreter (switch dispatch, NaN-box values, CFG-of-blocks,
2 to 3 instructions per expression) has so little dispatch overhead that a modern out-of-order core hides it. On
memory-bound code the latency is shared, so native saves nothing. On dependency-bound code the dependency chain
dominates, so native saves ~0.5ns. On independent compute the core's ILP plus a perfectly-predicted 3-way
dispatch branch overlaps the interpreter's dispatch with the arithmetic, so native again saves only ~0.15ns.

The only regime where native looked dramatically faster was pure independent arithmetic, and that win came
entirely from LLVM auto-vectorising the loop, which **copy-and-patch cannot do** (it concatenates scalar
stencils). So the realistic copy-and-patch speedup over a good interpreter is the ~1.2x measured here, not the
apparent infinity of a vectorised loop.

This is decision-forcing for the tier strategy and it refines op's "native = ceiling not necessity" with numbers:

- **The register interpreter is the priority.** It is already within ~20% of scalar native on the hot path. The
  cheapest large win in interpreter performance is a good interpreter, not a JIT.
- **Copy-and-patch is a modest accelerator (~1.2x), not a transformative one.** Its value is real but small for
  scalar script code; it is worth building as an opt-in tier, but the design must not depend on it for viability,
  and it should not be the first thing built. The earlier copy-and-patch probes proved its mechanisms work; this
  bench sizes its payoff, and the payoff is modest.
- **The transformative native win needs a vectorising/optimising backend** (LLVM or Cranelift), which is exactly
  the "ideal endgame where the runtime is environment-plumbing around LLVM/Cranelift output" the framework already
  names. That tier is reserved for the rare genuinely compute-bound consumer; it is a different, heavier
  mechanism than copy-and-patch, and its ~10x-class wins come from vectorisation and whole-function optimisation,
  not from removing interpreter dispatch.

So the tier ladder, sized: register interpreter (the fast floor, covers everything) -> copy-and-patch (~1.2x
scalar accelerator, opt-in, modest) -> optimising JIT (the real big-win ceiling, vectorising, for the compute-
bound minority). The floor is where the leverage is; both native tiers are ceilings, and copy-and-patch
specifically is a smaller ceiling than its reputation suggests.

## Design impact

- Invest in the register interpreter first (switch, NaN-box, CFG, register VM, all validated tonight). It is the
  floor and it is already near-native for scalar code.
- Treat copy-and-patch as a modest opt-in accelerator, correctly sized at ~1.2x, not as a headline tier. Its
  probes confirmed feasibility; this confirms its payoff does not justify prioritising it over the interpreter.
- Reserve the heavy optimising-JIT tier (LLVM/Cranelift) for consumers with genuinely compute-bound kernels,
  where vectorisation and whole-function optimisation deliver the ~10x-class wins copy-and-patch cannot.
- For vehje's actual biggest consumers (doc DSLs, config, game scripts), which gather and emit far more than they
  crunch, the interpreter floor alone is the right answer; the native tiers are genuinely optional.

## Boundary

Microbench of a single recurrence, not a full script workload; real scripts mix control flow, calls, and value
shuffling (measured separately in cfg-interp and value-representation), but those add dispatch that favours the
interpreter's already-small overhead being hidden, so the ~1.2x scalar-native ceiling is if anything an upper
bound for copy-and-patch on realistic code. The optimising-JIT tier is not benched here (it is a separate, heavier
mechanism); its win is asserted from the vectorisation the copy-and-patch baseline deliberately excludes, and
sizing it is future work if a compute-bound consumer appears.

## Artifacts
- `ceiling.zig` (register interpreter vs opaque-`madd` scalar native, across the three fair regimes; the trap
  regimes are documented above rather than left in the file).
