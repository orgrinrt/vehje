# Context: audit of the vehje bench-and-sketch evidence synthesis

## What is under evaluation

The evidence-synthesis topic `mock/design_rounds/202607211347_topic.bench-and-sketch-evidence-synthesis.md`,
together with the benchmarks and feasibility sketches it consolidates. The topic gathers a body of overnight
benchmark and sketch results and maps them onto design decisions for the vehje framework (an embeddable
multi-input/multi-output compiler IR framework: a Rust dev-time compiler that emits validated data, a Zig
comptime-specialised no_std/no_alloc runtime, a graded co-modal type-system proof spine, certified generation,
reachability-type leases, a relational-fixpoint compile substrate, an equality-saturation lowering stage, a
CFG-of-blocks interpreter, and copy-and-patch as a non-primary native tier).

## The question

Evaluate whether this evidence is dependable and whether the topic's conclusions follow from it. Three parts:

1. Do the benchmarks and sketches actually measure what their conclusions claim? Are they methodologically
   sound (variant isolation, fair baselines, cross-validation, defeating compiler artifacts such as
   if-conversion, auto-vectorisation, copy-elision, dead-code elimination), or are any of them tautological,
   mismeasured, swamped, or artifacts of the harness rather than signals about real behaviour?
2. Do the findings genuinely support the design conclusions the topic draws from them (the 24-byte record, the
   switch dispatch, the native-tier sizing, the tier-dependent branch strategy, the incremental-scaling story,
   the "proven feasible" sketches, and the rest)?
3. What, if anything, is over-claimed, weak, mismeasured, or still unproven, and what would it take to make it
   dependable?

Reach your own conclusion from the source and the artefacts. The evidence is neither assumed sound nor assumed
flawed; that is the thing to determine.

## Source material (read the artefacts, not only the summary)

- The topic under evaluation: `mock/design_rounds/202607211347_topic.bench-and-sketch-evidence-synthesis.md`.
- The branch-strategy maps: `mock/benches/BRANCH_STRATEGY_MAP.md`, `mock/benches/PORT_NOTES.md`.
- The consolidated overnight write-up: `mock/research/202607211015_MORNING-SUMMARY-overnight-spikes.md`.
- The benches themselves (each a directory with variant sources, a CSV, a meta, and a findings file):
  `mock/benches/` contains `interp-dispatch`, `record-width`, `match-lowering`, `iter-fusion`,
  `project-field-access`, `resolve-name-scope`, `effect-lattice-inference`, `tnum-abstract-arith`,
  `partial-eval-specialization`, `record-update-reuse`, `egraph-saturation-scaling`, `closure-representation`,
  `interner-intern-hotpath`, `interp-output-building`, `reach-fixpoint-scale`, `arena-locality`,
  `value-arena-throughput`, `cfg-interp-throughput`, `cheap-lowering-subset`, `incremental-scaling`,
  `mod-stack-load`, and more. The per-variant cdylibs are under `mock/benches/variants/`, the promoted results
  under `mock/benches/results/`, and the harness config is `mock/benches/bench.toml`. The branch-strategy
  generators (`gen_branch_variants.py`, `gen_multiway_variants.py`, `gen_showdown_ir.py`, `gen_archetypes.py`,
  `gen_archetypes_native.py`, `gen_port.py`) show exactly how each variant was constructed.
- The sketches (each a directory with a `findings.md` stating WORKS / FAILS / INCONCLUSIVE and why):
  `mock/research/sketches/2026072102*` through `mock/research/sketches/202607211830_*`.
- The bench harness itself, for judging the methodology: `../mockspace/bench-harness/` (per-variant cdylib
  subprocess isolation, hardware-counter timing, byte-exact cross-validation between variants, the declarative
  `[bench.*.normalise]` baseline subtraction, the CSV plus meta plus findings artifact trail, and the report
  and summary generators).
- The design the evidence is meant to justify: `../.shared/state/vehje-certgen-arc.md` (the current design head),
  and the design-round topics `mock/design_rounds/202607210045_*`, `202607210120_*`, and `202607202001_*` (the
  converged shape, parts of which are explicitly superseded by the later topics). `mock/CLAUDE.md` states the
  framework identity and constraints.

## Constraints and known caveats to weigh

- Toolchain: Zig 0.16.0, Rust nightly-2026-05-28, aarch64 Apple Silicon.
- The interpreter-tier benches run a mockup IR interpreter, not the eventual vehje runtime; their absolute
  numbers are representative rather than final, and the transferable part is claimed to be the strategy
  rankings.
- The corpus distributions that back several wire-format choices use a Python-standard-library proxy, not the
  real vehje consumers, which do not exist yet.
- The harness caps working sets at 16384 bytes, so anything cache-bound past L1 is out of the harness's range.
- WebSearch budget may be exhausted this session; ground the audit in the artefacts and your own knowledge
  rather than fresh search where needed.
