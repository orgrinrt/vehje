# SK2 findings: no-alloc bounded multi-shot Handle over a host-lent budget

**Date:** 2026-07-21
**Outcome:** WORKS (the core mechanism of CR1 is feasible and no-alloc). Scope + refinements noted honestly.
**Settles:** CR1 feasibility (op's "multi-shot without heap via a host-lent, statically-validated, deterministic
budget"), and informs the budget-fit validation sub-decision.
**Toolchain:** Zig 0.16.0.

## What was demonstrated

A bounded multi-shot algebraic effect (nondeterministic `amb`/`choose` with a collect-all handler) realised with
NO heap:

- **Multi-shot resumption:** the handler resumes the continuation once per choice combination (12 resumptions for
  domains 2x3x2), which is genuine multi-shot (the continuation runs many times), the thing one-shot cannot do.
- **No heap:** the continuation state (the choice vector) and the results live in fixed host-lent buffers (a
  caller-provided `*[CAP]i64` and a small `[doms.len]u32`). No allocation.
- **Fail-closed comptime budget-fit check:** the handler comptime-checks that the pattern's footprint
  (`prod(domains)`) fits the lent budget `CAP`; a pattern that outgrows it (`10x10x10 = 1000` combos into a
  16-slot budget) is a compile error ("bounded multi-shot pattern needs 1000 continuation slots, host lent only
  16"), confirmed by `demo_overflow.zig` failing to compile. This is the strict-by-design budget validation op
  asked for.
- **Deterministic:** a fixed mixed-radix enumeration order; two runs produce identical results (asserted). Fits
  the stack's determinism ethos.

The mechanism: the continuation is defunctionalised into a pure function of its captured choices, and the handler
enumerates the bounded choice space in the fixed buffer. This is exactly op's "host lends a bounded budget, we
validate the pattern fits it, and it is deterministic," made concrete and no-alloc.

## Honest scope and refinements (the design work beyond feasibility)

- **This is the defunctionalised bounded realisation**, which covers the common bounded multi-shot patterns
  (nondeterminism, collect-all, bounded backtracking/search). It does NOT capture an arbitrary live native stack
  for general multi-shot; CR1's claim is precisely bounded multi-shot via a host-lent validated budget, and the
  defunctionalised enumeration IS that. Generalising from the hand-written `program` continuation to one COMPILED
  from the IR is engineering (the generated engine emits the defunctionalised continuation), not a feasibility
  gap.
- **Prefix sharing is a refinement, not a blocker.** The flat enumeration re-runs the whole continuation per
  combo, so it does not share the work before the first choice point. A DFS with an explicit fixed-capacity stack
  (the stack holding the shared prefix state, budget = max-depth x frame-size, comptime-checked the same way)
  shares prefixes and is also no-alloc; it is the efficiency refinement for interspersed choice points. Both are
  bounded and budgeted; the flat version is the simplest proof of feasibility.
- **The budget-fit validation mechanism** here is the direct structural bound (`prod(domains) <= CAP`), which is
  the "just bound it, deterministic" option op floated, and it is enough for structurally-bounded patterns. A
  full potential/AARA analysis is only needed if a pattern's resumption count is data-dependent rather than
  structurally bounded; the structural bound suffices for the deterministic bounded case and avoids re-opening
  full AARA.

## Design impact

CR1 is feasible: bounded multi-shot handlers are no-alloc-realisable via a host-lent budget with a fail-closed
comptime budget-fit check, deterministic. The Handle form can support multi-shot (not just one-shot) without
heap, exactly as op directed. The budget-fit is a structural bound for deterministic patterns (no full AARA
needed), which answers the CR1 validation-mechanism sub-decision toward "structural bound + determinism." The
Core-re-derivation topic (202607210120) can proceed on this basis; the generated-continuation and prefix-sharing
refinements are its engineering, not open feasibility.

## Artifacts
- `handle.zig` (the bounded multi-shot handler + comptime budget-fit check)
- `demo_ok.zig` (12 resumptions, no heap, deterministic), `demo_overflow.zig` (budget-fit @compileError)
