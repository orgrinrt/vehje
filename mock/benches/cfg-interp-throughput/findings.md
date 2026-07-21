# CFG interp throughput (expansion): control-flow-heavy per-instruction cost

**Date:** 2026-07-21 | Zig 0.16.0 | data `cfg.csv` | expands BN1 (straight-line only) + SK17.

## Result (nested-loop numeric kernel, register CFG interp)
128M executed instructions (incl. block terminators) in 218 ms = **1.70 ns/instr, 587 M-instr/s**, with ~44% of
executed steps being control-flow terminators (branches/loop back-edges). Result 41,153,141 (correct).

## The clarification this gives
The register-based CFG interp (SK17) runs at 1.70 ns/instr even when control-flow-heavy, ~2.6x FASTER
per-instruction than the arena-walk interp (BN1: 4.4 ns/node). The reason is the memory profile: the register CFG
interp works on a small fixed register file that stays L1-hot and a small hot code array, so it is compute-bound
on the register ops; the arena walk loads each node from a large (48-96 MB) node array and reads operands from a
large result array, so it is memory-bound.

These are not competing designs, they are different LAYERS:
- The register-CFG interp is the PROGRAM executor (control flow + register ops), fast at 1.70 ns/instr.
- The value-arena (BN1/SK20) is the OUTPUT the program produces (a data tree), walked/emitted separately.

So the per-frame cost of running a script is (instructions executed) x 1.70 ns. A behavior script of a few
hundred instructions costs sub-microsecond, negligible against a 16.6 ms frame, even before any native tier. This
reinforces BN1/SP7: the interpreter floor is fast enough that the native tier is a ceiling, not a necessity.

## Design impact
The program interpreter is the register-based CFG-of-blocks (SK17) with switch dispatch (BN1), running at
~1.7 ns/instr including control flow. The value-arena is the output layer. Per-frame script cost is negligible;
the native/stencil tier is an accelerator for genuinely hot loops, not a floor requirement.
