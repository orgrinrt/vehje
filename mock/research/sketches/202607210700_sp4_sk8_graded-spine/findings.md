# SP4 + SK8 (+ SK9, SK10) findings: the four-axis graded (co)modal spine composes

**Date:** 2026-07-21 | **Outcome:** WORKS (coherence/feasibility of the grade algebra) | Rust nightly | `graded.rs`
**Settles:** SP4 (the graded judgment composes and the axis interactions fall out), SK8 (assurance derived from
binding-time), SK9 (binding-time as a joined knowledge-source lattice); touches SK10 (effect-as-operations).

## Result
A bottom-up grade computation over an expression IR with the four axes:
- **binding-time (SK9):** a knowledge-source mask over {lang_author, bundler, host_loader, runtime}, combined by
  JOIN (not a chain), so a program mixing a lang-author literal, a bundler var, and a host var has bt = 0b0111
  (lang|bundler|host). Partial bundling is a join, exactly SK9's point.
- **effect (graded monad):** an op mask combined by UNION; the composed program's effect is {host_call, macro}.
- **lease (coeffect):** a per-binder reachability mask combined by OR; the program's lease is {b0, b1}.
- **assurance (SK8):** derived from binding-time (the latest source present decides): the host-bound program is
  `LoadChecked`; an all-lang-author program is `Static` (highest); `assurance_of(LANG) > assurance_of(RUNTIME)`.
  Assurance is a projection of the binding-time grade, not an independent axis, confirming SK8.

The axis interactions fall out of the composition rules (on `Op`: bt joins, eff unions, lease combines), not as
separate case analyses. **Inclusion-not-coverage** holds: the well-graded program (`eff = {host_call, macro}`) is
included in a target permitting `{host_call, macro}`; an over-effecting program (does `alloc`, target forbids it)
is rejected by `eff & !permits != 0`.

## SK10 note (effect-as-operations / handlers)
The effect axis here treats `host_call` and `macro` as effect OPERATIONS on the mask, which is the D3
handler-instance framing (a host-call is a host-handled effect op, macro is a compile-stage-handled effect op).
This spike models them as operations composing by union; the full handler-discharge (an op leaving the residual
effect once handled) is the Core-re-derivation's engineering, but the operation-composition shape is confirmed.

## Scope (honest)
This is a coherence/feasibility check that the four-axis grade ALGEBRA composes and the axis interactions and
inclusion fall out. It is NOT the `lambda_veh` soundness metatheorem (T1/T2/T3), which is the proof-doc work. The
literature (Granule) is the evidence the graded (co)modal type system typechecks; this confirms the vehje-specific
four-axis composition is coherent and computable.

## Design impact
The graded (co)modal spine (the identity's core) composes: one grade with four axes, combined by join/union/combine,
assurance a projection of binding-time, inclusion-not-coverage as the effect check. The Core-re-derivation and the
proof doc build on this confirmed algebra.
