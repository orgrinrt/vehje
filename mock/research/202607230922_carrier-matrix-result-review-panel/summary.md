# Panel summary: carrier matrix result + methodology review

Round `202607230922`. Three parallel specialists (agner-fog microarchitecture/validity,
haoran-xu interpreter-composition, chris-fallin codegen/optimizer), one synthesis
(fabian-giesen). Every load-bearing claim re-verified against primary source by the
synthesiser; two optimizer claims replicated by an independent node-count probe. Terse index;
detail at the cited `file:linestart..lineend`.

## Verdict

Decision-grade for five axis choices; not decision-grade for the tier-selection question it was
commissioned to answer. `fabian_giesen_synthesis.md:15..34`. Machinery is sound; every defect
lives in the reporting layer or in one missing measurement, not in the harness core.
`fabian_giesen_synthesis.md:253..260`.

## The dominant finding (all three lenses converge)

The design oracle's own "central measurement", the `total(k) = S + k·I` cost-model k-sweep, was
never built; everything shipped is a single warm point at fixed k=16. `haoran_xu_composition.md:98..187`,
`202607221700_topic...md:117..176`. Consequence: every setup cost `S` (JIT codegen, optimizer,
predecode) is hidden in untimed prep, so the tier breakeven (when native/optimize/predecode pays
for itself) is unknowable. The near-native "copypatch ≈ stencil within 1%" measures only warm
execution parity; copy-and-patch's actual selling point is compile-time `S`, which is the one
dimension never measured. `haoran_xu_composition.md:98..139`, `chris_fallin_codegen.md:25`.

## The re-read that flips the interpretation (giesen overrules agner-fog)

The N=1024→4096 cost step is branch-predictor capacity, not an L1D cache cliff. Evidence: madd/tight
share real's byte footprint yet show no step; nullfloor (same loads, no opcode branch) is flat;
step magnitude orders by op-stream entropy. `fabian_giesen_synthesis.md:73..108`. So the small-N
"dispatch barely matters" ties are the memorized-predictor artifact and N=16384 is the honest
near-cold column; at saturation fntable wins ~10% and the shape spread is ~35%. Prior art: Rohou/
Swamy/Seznec CGO 2015 (big predictors absorb dispatch) is the memorized half; the capacity cliff is
that result's validity boundary. `fabian_giesen_synthesis.md:224..243,289..299`. PMU
(BRANCH_MISPRED vs L1D_MISS) adjudicates mechanically.

## Magnitude corrections (direction holds)

- Vertical SIMD 6x → 4.8x: scalar baseline runs the WIRE interpreter (2.603ms/input = wire switch),
  vert cells use cached predecode; ~20% of the "win" is decode asymmetry. Fix: baseline against the
  predecoded scalar. `agner_fog_methodology.md:111..125`, `fabian_giesen_synthesis.md:54..58`.
- Dispatch "2-15%" understates pure dispatch: the O(N) fidelity checksum is co-timed INSIDE the
  loop (the brief's "out of the timed region" is wrong); compression is an upper bound ~1.3x, the
  checksum's own share is ~5-8% not 22%. Fix: difference against nullfloor (subtract mode is wired,
  pointed at the wrong baseline). `agner_fog_methodology.md:70..88`, `fabian_giesen_synthesis.md:66..71`.
- Register-vs-stack 1.91x is a three-axis composite (encoding + wire-vs-predecoded decode form +
  full-vs-liveout checksum), all burdening the register side; true predecoded-register-vs-stack is
  ~2x. `haoran_xu_composition.md:189..229,150..160`.

## Optimizer (chris-fallin, replicated twice)

- `cse+eqsat` == `eqsat` bit-identical node counts on all 12 profile/size combos; CSE recovers
  nothing. `chris_fallin_codegen.md:27..29`, `fabian_giesen_synthesis.md:180..194`.
- eqsat's marginal contribution to `all` is zero-or-negative everywhere; standalone it inflates madd
  5.6x (mechanism of the 142x); wideselect shows a measured in-pipeline pessimization (`cse` 1.574ms
  beats `all` 1.621ms in the shipped table). `chris_fallin_codegen.md:29,37`, `fabian_giesen_synthesis.md:196..203`.
  RUN_SUMMARY finding 6's "CSE+eqsat recovers / run the whole pipeline" is contradicted by the run's
  own data. fold-alone and dce-alone are structural no-ops on this DAG. `chris_fallin_codegen.md:31`.
- Terminology inverted: cell tagged `copypatch` is direct instruction-selection; cell tagged
  `stencil` is the real Xu-Kjolstad copy-and-patch. RUN_SUMMARY finding 7 inherits it. Venue is
  OOPSLA 2021, not PLDI. `chris_fallin_codegen.md:33..35`.

## Other confirmed methodology facts

Calibration re-warm makes the surrounding workload irrelevant to `algo_ns` (warm-throughput, not
cold latency) `agner_fog_methodology.md:52..68`; reps-variable output makes MAY_DIFFER=false
cross-validation inert `agner_fog_methodology.md:140..153`; reported values are per-16-execution
with integer-truncation bias `agner_fog_methodology.md:127..138`; run taken from a dirty tree
(`d772801-dirty`) `agner_fog_methodology.md:155..159`. Fresh: reproducible vert4 N=1024 anomaly on
far-operand profiles `fabian_giesen_synthesis.md:110..114`.

## What survives as decision-grade (giesen table)

`fabian_giesen_synthesis.md:305..316`. Register over stack (High); record width null = density
choice, regime-bound (High); fold+CSE+DCE always on (High, replicated); eqsat out as shipped (High);
switch-or-fntable dispatch, threading only for real control flow (High); vert8 column eval the
largest lever at 4.8x (High); native ~2.6-4.3x warm floor (High for the floor, measured where the
interpreter is at its best); nanbox/tagged one-shape (Low-medium).

## Costed open questions (each >=3 options, ranked)

Q1 cold regime: program-set cycling + first-rep column (B+C) over the design's verbatim k-ladder (A,
in-process rungs aren't cold). `:378..411`. Q2 eqsat: delete + commutative-operand canonicalization
(A+B), park DAG-aware extraction behind a named trigger. `:413..439`. Q3 native: S-cost family +
imm12 lift to measure native where the interpreter is honest. `:441..465`. Q4 tier ladder: redefine
the middle tier as predecoded-register + fold/CSE/DCE (stack bytecode closed on evidence), and
reserve a batched `execute(residual, inputs[W], outputs[W])` entry in the runtime C ABI now (the one
non-deferrable, ABI-shaped decision). `:467..500`. Q5 record width: REC16 for density. `:503..527`.

## Bottom line

`fabian_giesen_synthesis.md:529..560`. Report fixes over existing data (nullfloor differencing,
predecoded vertical baseline, residual composite flag, un-invert JIT tags, OOPSLA venue, clean-commit
re-run). Cell fixes (mechanical): checksum_at(sinks) both residual cells, register cell predecoded,
reps-invariant output, shared seed table. Add: S-cost family, program-set-cycling + first-rep cold
mode, PMU re-run (now load-bearing, adjudicates predictor-vs-cache), imm12 lift, entropy×locality
grid. Remove: eqsat from default, stack-bytecode middle-tier framing. The single immediate ABI
decision: reserve the batched entry point.
