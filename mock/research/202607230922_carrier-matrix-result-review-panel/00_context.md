# Context brief: carrier interpreter-composition matrix, result + methodology review

Shared ground for the panel. Factual. Read this, then the artefact, then form your own
judgement through your assigned lens.

## The artefact under evaluation

A benchmark matrix measuring how the vehje "carrier IR" (17 opcodes: CONST=0 .. INPUT=16)
executes under different dispatch shapes, value representations, residual layouts, optimizer
levels, and beyond-runtime approaches (SIMD, native codegen, cross-language). 45 benches, ~213
variants, run once as a coherent wall-clock batch on 2026-07-23 on Apple Silicon (M-series).

Two things are under review, jointly:

1. **The methodology.** Is each measurement measuring what it claims? Are the cross-variant
   comparisons fair? Are the reported ratios real effects or artifacts of the harness, the
   timer, the calibration, anti-hoist guards, or the program generator?
2. **The results.** Given sound methodology, what do the numbers actually establish, and how
   confidently?

## Where everything lives (read the primary source, not only the summary)

Repo root for all paths: `~/Dev/clause-dev/vehje/`.

- **Results summary + full matrix:** `mock/benches/results/RUN_SUMMARY.md` (9 findings + tables).
- **Per-bench raw data:** `mock/benches/results/carrier_*/` (each has per-size `*.csv` with
  columns `run,pass,cooldown_ms,mode,variant,batch_idx,e2e_ns,algo_ns,bridge_ns,batch_count,
  score,input_tag,instructions,cycles`, plus `*_findings.md` and `*.meta.json`).
- **The matrix generator (the harness the numbers come from):** `mock/benches/src/bin/gen_matrix.rs`.
  This emits one release cdylib per variant from a shared template; read the template body, the
  `timed_calibrated!` usage, the anti-hoist `acc` chaining, the per-family cell definitions, and
  the size/profile sets.
- **The carrier crate (the measured code):** `mock/benches/carrier/src/`. Load-bearing files:
  `ops.rs` (the single-source `binop_body!` / `binop_simd!` op semantics every cell instantiates),
  `access.rs` (the shared unchecked `rload`/`rstore` + post-pass `checksum` fidelity path),
  `interp.rs` + `interp_threaded.rs` (dispatch shapes), `predecode.rs`, `cfg.rs` + `cfg_threaded.rs`
  (register-VM + threading), `valrepr.rs`, `stackbc.rs`, `optimize.rs` + `eqsat.rs` + `fusion.rs`
  (optimizer axis), `native.rs` + `copypatch.rs` + `stencil.rs` (native codegen), `vertical.rs`
  (portable_simd), `gen.rs` (program generator + the 6 profiles: real/madd/tight/scatter/
  wideselect/leaf), `ir.rs` (REC12/16/20/24/32 wire layouts).
- **The bench harness itself:** `mockspace` bench-harness (the cdylib-per-variant subprocess
  driver, the `timed_calibrated!` macro, the CNTVCT timer, the `perf.rs` kperf PMU path). Source in
  the pinned mockspace checkout under `~/.cargo/git/checkouts/mockspace-*/` or the local clone at
  `~/Dev/clause-dev/mockspace/`.
- **The design oracle (what this matrix exists to inform):**
  `mock/design_rounds/202607221700_topic.bench-composition-matrix-synthesis.md` (the synthesis topic
  that scoped the matrix; the two final sections + amendments are authoritative). vehje's runtime is
  a tier-tagged residual (flat IR arena, optimized bytecode, or native code) executed by a small Zig
  runtime over a C ABI; the residual tier is orthogonal to correctness. See `.claude/CLAUDE.md`
  "Two sides" for that framing.
- **The prior review round (two experts, a subset/earlier state):**
  `mock/research/202607221930_carrier-machinery-review-haoran-xu.md` (machinery review),
  `mock/research/202607230030_full-matrix-review/` (`haoran_xu_review1.md`, `agner_fog_review2.md`).
  These reviewed the built machinery before the full run and drove a fix pass (commit d772801). You
  may read them for what was already raised, or form your judgement independently first.

## Methodology facts (established, for grounding)

- Each `(variant, size, mode)` is compiled as its own release cdylib (fat LTO, one codegen unit) and
  run in a separate subprocess, so no cross-variant inlining or shared-codegen contamination.
- Every arithmetic cell derives its op semantics from one source (`ops::binop_body!` scalar,
  `binop_simd!` SIMD); only the dispatch/decode SKELETON differs per cell. Register reads/writes go
  through one shared unchecked `access::rload`/`rstore`; a `checksum` after the hot loop validates
  fidelity out of the timed region.
- `timed_calibrated!` auto-repeats a cell until it clears the ~2048-tick CNTVCT (24MHz) quantization
  floor; the template chains a running `acc` across iterations as an anti-hoist / anti-DCE guard.
- Reported `algo_ns` is the median across replicated passes (12 rows per variant per size seen in
  the CSVs: several runs x passes). Ratios in the summary are vs a per-family baseline at the largest
  size.
- `instructions` and `cycles` columns are 0 in this run: the kperf PMU path needs a `sudo`-gated
  session not used here. It is host-side bracketing (no ABI change); a re-run would populate those
  two columns and nothing else.
- The 6 program profiles come from `gen.rs` `GenParams::profile`. `native_ceiling` is a distinct
  O(N^2) throughput-over-a-byte-stream idiom (not per-execution latency), capped at N<=1024.

## The question (neutral)

Evaluate, through your assigned lens, whether this matrix's methodology is sound and its results
are trustworthy and decision-grade, and what (if anything) they establish or fail to establish about
the shape vehje should build canonically. Reach your own conclusion from the primary source and the
raw CSVs. Where you find a measurement that does not measure what it claims, a comparison that is not
fair, a ratio that is a harness/timer/generator artifact, or a result that is over- or under-claimed,
name it precisely with `file:line` and the concrete failure. Where a result is sound and load-bearing
for the canonical design, say so and say why. Bring at least one angle the prior reviewers did not.
