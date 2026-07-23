# Fix campaign: addressing the panel critique before the PMU re-run

Op directive (2026-07-23): fix all code holes/failures, write the missing measurements,
rewrite the wrongly-implemented cells, then re-run in full with the PMU (sudo) pass. Confirm
readiness via AskUserQuestion before the run.

Status legend: [ ] todo, [x] done, [~] in progress, [>] dispatched.

## Tier 1 — mechanical correctness (make the re-run honest)

- [ ] 1. Un-invert JIT variant tags: `copypatch`(JitCode=direct isel)->`direct`,
      `stencil`(StencilCode=real copy-and-patch)->`copypatch`. `gen_matrix.rs:287..291`.
      (chris-fallin F5)
- [ ] 2. Citation venue: copy-and-patch is Xu & Kjolstad OOPSLA 2021, not PLDI.
      `copypatch.rs:12`, `stencil.rs:3`. (chris-fallin F6)
- [ ] 3. Vertical scalar baseline runs the WIRE interpreter while vert cells use cached
      predecode: switch the scalar cell to `interpret_predecoded` so only lanes differ
      (honest 4.8x). `gen_matrix.rs:318..323`. (agner F4, haoran, giesen)
- [ ] 4. Residual family double asymmetry: register cell over wire `Decoded` + full-array
      checksum vs stack over predecoded + live-out checksum. Make register cell run
      `interpret_predecoded` and both cells `checksum_at(sinks)`. `gen_matrix.rs:236..247`.
      (haoran F2, giesen)
- [ ] 5. eqsat out of the default `all`; add commutative-operand canonicalization (sort each
      commutative binop's operands by node id, in place, then plain CSE). Park AC-reassociation
      behind a named trigger. `optimize.rs`, `eqsat.rs`, `gen_matrix.rs:254..262`. (chris-fallin
      F2/F3, giesen Q2)

## Tier 2 — fidelity plumbing

- [ ] 6. Reps-invariant FFI output: keep the anti-hoist acc chain for timing, but write a
      reps-invariant fidelity digest to `output` so MAY_DIFFER=false cross-validation means
      something. `gen_matrix.rs` template. (agner F6)
- [ ] 7. Shared seed table across sizes: today `input[k % N]` with k<16 reads the first 16
      bytes, which differ per size, so cross-size comparisons depend on per-size input bytes.
      Pin one shared 16-byte seed table. template + harness input. (giesen bench-changes 5)

## Tier 3 — the missing measurements (the "central measurement")

- [ ] 8. S-cost family `carrier_setup_cost`: time `Decoded::parse`, `predecode`,
      `stackbc::compile`, `optimize()` per strategy, `JitCode::new`, `StencilCode::new` per
      profile/size. Produces the tier breakeven table (the missing half of total = S + k*I).
      new family in gen_matrix.rs. (all three, giesen Q3-A)
- [ ] 9. Cold regime: (a) program-set cycling: M distinct same-profile programs round-robin in
      the timed region so no single program's dispatch sequence is memorized (the many-residuals
      deployment shape); (b) first-rep column `algo_ns_first` for true first-touch. template +
      harness + new family. (agner F1, giesen Q1-B+C)
- [x] 10. imm12 lift so the JIT cells run at N=4096/16384 (register-materialized base or adrp/add),
      measuring native where the interpreter is saturated not memorized. `copypatch.rs`,
      `stencil.rs`, native_family sizes. (giesen Q3-C)
- [x] 11. entropy x locality generator grid: op_correlation x locality_window as swept axes so
      "entropy dominates locality" becomes a measured surface. `gen.rs`, new family. (giesen
      synthesis 4, bench-changes 6)

## Tier 4 — design (separate from the bench re-run; flag for op)

- [ ] 12. Redefine the canonical middle tier as predecoded-register + fold/CSE/DCE (stack bytecode
      closed on evidence); reserve the batched `execute(residual, inputs[W], outputs[W])` entry in
      the runtime C ABI now (the one non-deferrable ABI decision). Design-doc/runtime work, not
      bench code. (giesen Q4)

## Re-run

- [ ] 13. Rebuild all variants; AskUserQuestion to confirm op ready + prompt for sudo; full run
      with `MOCKSPACE_BENCH_PERF=1` (PMU instructions/cycles). Re-aggregate with nullfloor
      differencing + corrected baselines. Re-run from a CLEAN commit (the prior run was
      d772801-dirty).
