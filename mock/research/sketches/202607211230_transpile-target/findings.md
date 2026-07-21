# Transpile target (expansion): the transpile pole of the output spectrum

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 + zig cc | artifact `transpile.zig`
**Settles:** transpilation as a point on the output-generation spectrum (1513), the "other side" op flagged as
never developed. A transpile target is an IR->text fold.

## Result
The same IR (`let v0 = (3+4) in v0*2`) is transpiled to two target languages by a simple fold over the nodes:
- **C:** `long prog(void){ long v0 = (3 + 4); return (v0 * 2); }` -- compiles with zig cc and runs to **14**.
- **Lua:** `local v0 = (3 + 4)` / `return v0 * 2`.

Transpilation reduces nothing and re-expresses the IR in the target language's source, the opposite pole from
full interpretation (which reduces everything to a value). A transpile target is just an IR->text fold, exactly
the output-spectrum synth's claim that interpretation and transpilation are the same operation aimed at different
targets, and that a target is a thin per-target surface (a fold) over the shared IR.

## Design impact
- The transpile end of the output spectrum is buildable and trivial: a target = an IR->text fold. The C output
  compiles and computes correctly; multiple targets (C, Lua) come from the same IR with per-target folds.
- This is the "other side" the runtime-performance focus never developed: consumers whose output is another
  language's source (vehje-jomini -> Clausewitz, vehje-lua -> Lua, a doc DSL -> markdown) are transpile targets,
  each a fold, on the same spectrum as the interpreting consumers. Combined with the inclusion-not-coverage
  target contract (D2), the output side spans interpret -> transpile uniformly.
- Staging note: a real transpiler would first reduce build-environment effects (const-fold, macro expansion via
  the cheap lowering) and then emit the residual as target source, so the transpile fold runs over the lowered
  IR. Here the IR is already simple; the cheap-lowering (const-fold+CSE) bench is the pre-pass.

## Artifacts
- `transpile.zig` (IR -> C and Lua folds), `gen_prog.c` (the emitted C, runs to 14).
