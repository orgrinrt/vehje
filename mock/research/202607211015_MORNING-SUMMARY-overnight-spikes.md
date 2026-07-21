# Morning summary: the overnight spike/sketch/bench run (2026-07-21)

The full de-risking inventory (`202607210200_spike-sketch-bench-inventory.md`) ran to completion. Every item
landed with an artifact + findings. Toolchain: Zig 0.16.0, Rust nightly-2026-05-28, aarch64 (Apple Silicon).
Nothing was rate-limited; the whole floor was covered and then some. Below: the outcome of each, the key finding,
and the design call it settles. Load-bearing NEW findings and the decisions to fold into the design proper are
called out at the top.

## The decisions the numbers force (fold these into the design)

1. **Interpreter dispatch = plain `switch`, NOT tail-threading (BN1).** Switch is the FASTEST (4.58 ns/op) across
   every case, contrary to the dynamic-language literature, because vehje's lean statically-checked flat IR makes
   the switch branch well-predicted and the fancy-dispatch machinery costs more than it saves. This **removes
   `preserve_none` from the critical path entirely** (SK3 confirmed 0.16 has no preserve_none; it does not matter).
2. **Node record = 24 bytes (3 inline operands), NOT 16 (BN2 + SP2).** 24B beats 16B at every call fraction
   (sequential stream hides the density penalty; inlining the 3rd operand avoids the pool spill). The real arity
   distribution (SP2: arity<=3 = 96.5%) confirms 3 inline operands cover almost everything. This FLIPS the
   provisional Cluster B/C 16B lean.
3. **Comptime kernel MUST be iterative (SK5).** Recursive comptime SIGSEGVs the compiler at ~2500 depth (not a
   graceful error). The certified-generation kernel is an explicit defunctionalised work-stack.
4. **`@Type` is gone in Zig 0.16 (SK1).** Reification is now `@Enum`/`@Union`/`@Struct`/`@Int`/... The design
   docs' `@Type` references are stale; update them. Cert-gen itself works with the new builtins.
5. **Ship the runtime in ReleaseSafe (BN5).** Safety checks are free (-0.1%) on the memory-latency-bound
   interpreter workload; keep the safety net.
6. **E-graph extraction must be an iterative cost fixpoint (SP6).** Recursive extraction hangs on the cycles
   e-graphs get after merges.
7. **Three-loci boundary is superlinear-or-large, not all-content (BN0).** O(N) content validation at comptime is
   cheap (~27us/record); only superlinear (O(N^2)) or very large content needs the native build.zig step.
8. **Reachability lease fix = the binder rule + a region-promotion bound (SK4).** The counter-audit's fix, not the
   solution-set's InScope-only version (which is unsound the other way). Confirmed executably.
9. **Bounded multi-shot Handle is feasible no-alloc (SK2, CR1).** Host-lent budget + a fail-closed comptime
   budget-fit check + deterministic enumeration. Budget-fit is a structural bound for deterministic patterns, so
   no full AARA is needed.
10. **Wire format: aligned + padded regions (SK20 + BN4).** Zero-copy read and the 33 GB/s SIMD structural decode
    both require it; one constraint.

## Feasibility sketches (all WORKS unless noted)

- **SK1 cert-gen core** WORKS. The crux: Rust emits validated data, Zig comptime specialises a hand-authored
  engine, illegal families unrepresentable, total dispatch, folded inclusion check; both certifications; both
  negatives fail-to-lower. Finding: @Type -> @Enum family.
- **SK5 iterative comptime kernel** WORKS. Recursive segfaults ~2500 depth; iterative folds 100k clean.
- **SK3 dispatch + preserve_none** WORKS. No preserve_none in 0.16; always_tail + explicit args keeps state in
  arg registers, dispatch is a jump, no stack growth. Floor sound on stock Zig.
- **SK2 bounded multi-shot Handle** WORKS. No-alloc bounded multi-shot via host-lent budget + comptime fit-check,
  deterministic; overflow fails closed at compile time. CR1 feasible.
- **SK20 / SK18 / SK21 value transport** WORK. Arena (children-first, no back-patch, zero-copy), typed structural
  decode (linear, acyclic-by-construction, rejects corruption, not a fixpoint), reserve/commit sink (wire ==
  in-process). Alignment required for zero-copy.
- **SK17 CFG-of-blocks interpreter** WORKS. Loop + branch + call (bounded frame stack) run correctly; the Cluster
  C control-flow fix.
- **SK16 Perceus exact-meet** WORKS. Peak memory = frontier (3 of 1M), collector-free, no runtime refcounts.
- **SK4 reachability binder-rule** WORKS. Binder rule + region-promotion bound; N*W naming bound, escape bound.
- **SK6 / SK7 / SK23** WORK. Byte-shift decode (unaligned), panic-free error unions, comptime==runtime gate.
- **SK25 C-host panic-free** WORKS. C host embeds the Zig runtime over the C ABI, error codes, no panic crosses.
- **SK12 depth-lease floor** WORKS. Depth-min propagation, LIFO-sound (freed at its lease-region).
- **SK19 tnum + SK13 generational-ref** WORK. Numeric bound proof without eval; use-after-free caught by one
  generation compare.
- **SK22 provenance witness** WORKS. Lease error reconstructs the escape path from resident relations, happy path
  free.
- **SK14 capture** (folded into SP1). Scala-3 capture = the reach coeffect, no separate machinery.
- **SK15 PE-as-extraction** (folded into SP7). Static predicate -> folded strategy.

## Benches (numbers)

- **BN0 comptime cost cliff:** O(N) content cheap (~27us/rec), the cliff is superlinearity; refines 1845/05.
- **BN1 dispatch shapes:** switch 4.58, direct-threaded 6.24, tail 6.38 ns/op (local). Switch wins.
- **BN2 record width:** 24B (3.82-4.28) beats 16B (4.43-5.65) at all call fractions.
- **BN3 incremental scaling:** edit-one = O(depth) (10-20 visits) vs O(N) full; validates D6 (additive case).
- **BN4 SIMD structural decode:** 33.07 GB/s vs 2.31 scalar (14.3x).
- **BN5 ReleaseFast vs ReleaseSafe:** -0.1% (safety free for the memory-bound interpreter).

## Spikes (the big novelties, all feasible)

- **SP5 + SK11 relational engine + reach bitmask:** no-alloc semi-naive fixpoint, reach as a u64 lattice column,
  converges in bounded rounds. The compile-stage brain substrate works.
- **SP6 no-alloc bounded e-graph:** saturate-then-extract works ((x+0)*1 -> x); extraction must be iterative.
- **SP4 + SK8 + SK9 graded spine:** the four-axis grade composes (bt join, eff union, lease combine, assurance a
  bt projection); inclusion-not-coverage holds. The identity's core coheres.
- **SP3 three-loci build.zig content step:** native content validation gates the build.
- **SP7 + SK15 strategy selection:** grade-directed const-time evaluator portfolio, native = one strategy chosen
  where it wins, degrades to the floor.
- **SP1 + SK14 primitive vocab:** 25 primitives bounded; a 2nd family adds 0. Deegen semantic-def tractable.
- **SP2 corpus distributions (Python stdlib proxy):** arity<=3 = 96.5% (validates 24B), depth 3+ = 0.6% (tiny
  depth cap), live-binder <64 = 99.8% (u64 bitmask well-sized), closure rate 17% (avoidance matters for a
  minority).

## What still needs building (named, not feasibility gaps)

- The full multi-relation-join engine (leapfrog-triejoin, AGM) and eqsat query over SP5's substrate (SP5 scoped
  to the reach query; SP6 to bounded saturate-then-extract).
- The slotted/colored/graded e-graph extensions + the streaming window over SP6's base.
- DBSP-style counted differential dataflow for retraction (BN3's additive case works; general edit needs it).
- The generated-continuation and prefix-sharing DFS for the Handle (SK2 proved the bounded-enumeration core).
- The lambda_veh proof document (T1/T2/T3) -- a writing artifact, not run tonight.
- A census-specific corpus measurement when the real consumers exist (SP2 used a proxy).
- The native copy-and-patch stencil toolchain + its vs-interpreter bench (needs weeks of object-format work;
  deferred, and BN1/SP7 suggest its win is regime-specific anyway).

## Capstone: the pieces compose end-to-end (the strongest single validation)

A 50,000-node script run through the full pipeline (lease inference -> cheap lowering -> interpretation ->
value-arena emission -> typed validate) composes and runs end to end in **3.4 ms** (67 us / 1k-nodes). The
compile stage (lease + lower = 3.24 ms) dominates and the runtime (interp + emit + validate = 127 us) is cheap,
matching the authoring/templating majority's compile-stage-heavy / trivial-execute profile exactly. Each stage's
per-node cost is consistent with its standalone bench, so the pieces compose with predictable additive costs. A
realistic few-thousand-node script is sub-millisecond; with the D6 incremental caching, a mod stack loads in
reasonable time. This is the strongest evidence the design holds as a whole, not just piece by piece.
(`sketches/202607211100_e2e-integration/`)

## Expansion benches (post-inventory, for perspective)

- **BN1-heavy (dispatch across op-body weight):** switch faster than tail at every body weight (W=1..64; 4.48 vs
  6.69 up to 47.6 vs 51.8). Tail-threading never wins for vehje's bounded op vocabulary. The one untested regime
  is many op-kinds (100+), which SP1 shows vehje is not in (~25 primitives).
- **Arena locality sweep:** interp ns/op is FLAT (~4.4-4.5) for backward windows 2..1M, only +33% at full-4M
  random. The cache cliff is soft and far out (temporal locality keeps recent results warm); the backward-local
  discipline is a modest, non-fragile benefit.
- **Reach-fixpoint scale:** lease inference runs 8M nodes / 16M edges in ~50 ms (630 M-edge-OR/s), within a
  compile budget. Whole-column OR BEATS delta semi-naive for realistic shallow graphs (2 rounds; delta
  bookkeeping not repaid) -> use whole-column for the lease fixpoint.
- **Value-arena throughput:** encode 2.4 GB/s (RNG-bound lower bound), typed validate 9.7 GB/s scalar (33 SIMD),
  zero-copy walk 1.1 GB/s (pool-indirected). Transport (encode+validate) is cheap; the walk is compute and
  corroborates BN2 (pool indirection expensive -> 24B inline operands turn 10.7 -> 4.4 ns/node).
- **CFG interp throughput:** register CFG interp = 1.70 ns/instr, 587 M-instr/s even control-flow-heavy (44%
  branches), ~2.6x faster/instr than the arena walk (register file L1-hot). Clarifies the layers: the register-CFG
  interp is the PROGRAM executor (fast); the value-arena is the OUTPUT. Per-frame script cost is negligible
  (few-hundred-instr script = sub-microsecond vs a 16.6 ms frame), so the native tier is a ceiling, not a floor.
- **Cheap lowering subset:** const-fold + CSE in one no-alloc pass gives 76% node reduction at 34 M-nodes/s, the
  load-time compile stage that directly serves the templating/config majority (redundant IR -> big reduction).
- **Multi-shot enumeration:** ~2.5 ns/resumption (400-500 M-resume/s); bounded multi-shot (SK2/CR1) is fast as
  well as no-alloc.
- **Transpilation:** a transpile target is a thin IR->text fold; the same IR emits to C (compiles, runs to 14)
  and Lua. The transpile pole op flagged as undeveloped is buildable and trivial; interpret<->transpile is one
  operation aimed at different targets.
- **Mod-stack load (op's RimWorld/Clausewitz concern, DECISIVE):** a 2000-mod stack cold-loads in 4 ms; editing
  one mod warm-loads in 0.003 ms (1 recompile + 1999 cache hits, warm/cold = 0.00076). Load cost scales with the
  CHANGE, not the stack size. D6's content-addressed caching solves the load-bloat concern that motivated it.
- **Streaming value-transfer:** a 5.86 MB value (1430x the budget) streams through a 4 KB residency window (peak
  4092 B, 1432 flushes); huge/nested outputs stream in whole-subtree chunks with backpressure, never whole.
- **Runtime binary size:** the specialized engine is ~1 KB + ~1 byte/family; certified generation adds negligible
  size (the FamilyTag is a comptime type), so the runtime stays small and embeddable. The size is the fixed
  hand-authored engine + real family semantics, not the specialization.
- **SIMD decode width:** 16-byte vectors (native aarch64 NEON) are fastest at 38.2 GB/s; 32/64B are slower. Use
  16-byte vectors.
- **Record width, pinned:** adding a 32-byte (4 inline) variant, rec32 == rec24 (the 4th operand is unused,
  arity>=4 = 1.3%). 24 bytes is precisely optimal (16 < 24 = 32).
- **Untrusted load-verifier robustness (security boundary for arriving mods):** the structural decode rejects
  all five attack classes (forward-child/cycle, pool-span OOB, blob-overrun + integer-overflow, unknown-kind,
  over-deep) fast and without crashing. The load-bearing detail: the integer-overflow class is defeated only by
  doing bounds arithmetic one width above the index fields (`@as(u64, a) + b` for 32-bit fields); naive u32
  `a + b > len` wraps and passes a forged span. Wire-format contract: load-time structural verification is
  mandatory, single-pass, u64 bounds math; the untrusted path fails closed without needing a signature for
  memory safety. (`sketches/202607211400_adversarial-load-verifier/`)
- **Compile-stage parallelism (cold-load scaling):** the compile stage the e2e showed dominates cost is
  embarrassingly parallel across independent mods. A 2000-mod stack compiles at 1t=101.7ms, 2t=2.00x, 4t=3.87x,
  8t=5.64x (the last four threads are this part's efficiency cores plus memory-bandwidth bound). Cold load scales
  near-linearly with cores; combined with the warm mod-stack-load result (recompile only what changed), op's
  RimWorld/Clausewitz load-bloat concern is answered on both axes: cold parallelises with cores, warm
  parallelises with changes. The contract to keep is per-mod-local compile state (no cross-mod interner mutation
  at the parallel boundary). (`sketches/202607211430_compile-stage-parallelism/`)
- **Interner-merge cost (the serial tail of that parallelism):** per-mod-local interners are what make the
  compile parallel, but one composed runtime needs one string table, so the parallel compile has a merge tail.
  Merging 512K local strings (2000 mods, ~78% shared vocabulary) into 71.7K distinct globals takes 6.2ms
  single-threaded (7.1x dedup), which is ~34% of the parallel compile. The honest cold-load number is therefore
  parallel-compile (18ms) + merge (6.2ms) = 24.2ms vs 102ms serial, a 4.2x net win (Amdahl caps it below the raw
  5.6x). The per-mod-local contract is validated as a net win; the merge is the next parallelisation lever if load
  must go lower (it is a pure shardable dedupe), not needed now. (`sketches/202607211500_interner-merge-cost/`)
- **Cross-mod dependency DAG (the realistic mod case):** op's biggest consumers (Clausewitz, RimWorld) have mods
  that override and extend each other, forming a load-order dependency DAG, not independent mods. That DAG is wide
  and shallow (2000 mods, up to 3 deps each = 20 levels, ~100 mods per level), so 8-core level-sync cold compile
  hits 7.7x (better than independent mods, because wide levels keep cores busy; the critical-path floor is 100x,
  so cold load is core-bound not path-bound). Warm incremental recompile propagates to transitive dependents:
  ~19 mods (1%) for a typical edit, but up to 803 (40%) for a foundational base-mod edit, so warm cost is bimodal
  and statically knowable from the dependent count. This closes the full load-time story: independent or DAG, cold
  parallelises to the core count; warm recompiles the transitive-dependent set; the interner-merge tail applies to
  cold in all cases. (`sketches/202607211530_cross-mod-dep-dag/`)
- **Interpreter value representation (bench-driven fork):** the cfg-interp bench used untyped u64 registers, but a
  dynamic script value is tagged at runtime (int/float/bool/strref/nil). NaN-boxing (8B) beats tagged-8B
  (0.96 vs 1.17 ns/op, ~18%) and stores floats natively, where tagged-8B cannot fit an f64 in its 61 payload bits
  and must box. The 16-byte tagged union is rejected outright (2x memory on the hottest data structure). Since
  vehje's consumers (doc DSLs, config, game scripts) all carry floats, NaN-boxing is the recommendation, with the
  usual documented constraints (51-bit pointer payload, proven by LuaJIT/SpiderMonkey/JSC). Value-op cost (~1ns)
  sits just under dispatch (~1.7ns), so representation is real but not dominant, and the same NaN-box form is used
  in the 24-byte record's inline operands for one representation across arena and register file.
  (`sketches/202607211600_value-representation/`)
- **Register-VM vs stack-VM (bench-driven fork):** the biggest interpreter-design lever, unresolved by BN1
  (dispatch) and cfg-interp (CFG model). On `((k0*k1)+k2)` x4M, a register VM with RK operands (an operand is a
  register or a constant) runs each expression in 3 instructions and 3.03 ns; a stack VM runs 6 instructions and
  5.65 ns. Register is 1.86x faster because it executes half the instructions at near-equal per-instruction cost
  (Lua's 5.0-to-5.1 result reproduced). Recommend the register VM for the interpreter tier; it converges with the
  switch-dispatch, NaN-box, and CFG-of-blocks findings on a fast interpreter floor. Code compactness is the
  classic counter (a tight 1-byte-opcode stack VM has smaller bytecode), but that gap closes under compression and
  the streaming/mmap path already bounds resident size, so the 1.86x speed wins.
  (`sketches/202607211700_stack-vs-register-vm/`)
- **Interpreter vs native ceiling, copy-and-patch sized (op's "we want JIT" question):** how much faster is
  native than the register interpreter on the same program? Measuring it fairly is the whole problem: a naive loop
  lets LLVM auto-vectorise the native side to ~0 (unfair, copy-and-patch does not vectorise); a big gather makes
  it memory-bound (native 1.0x). The fair baseline (L1-resident, non-vectorisable data-dependent index, native
  arithmetic as an opaque scalar `madd`) gives, across three regimes: DRAM-gather 1.0x, serial-dependency 1.2x,
  independent-compute 1.2x. So **copy-and-patch native buys only ~1.0 to 1.2x over a good register interpreter**:
  the interpreter's dispatch is already so small (switch + NaN-box + CFG) that an out-of-order core hides it, and
  the only regime where native "wins big" needs LLVM auto-vectorisation that copy-and-patch cannot do. This sizes
  op's "native = ceiling" with numbers and reframes copy-and-patch as a smaller ceiling than its earlier
  feasibility probes implied: the interpreter floor is the priority (near-native for scalar code); copy-and-patch
  is a modest opt-in accelerator (build it, do not prioritise it, do not depend on it); the transformative
  ~10x-class native win needs a vectorising JIT (LLVM/Cranelift, the "environment-plumbing around LLVM" endgame),
  reserved for the rare genuinely compute-bound consumer. (`sketches/202607211730_interp-vs-native-ceiling/`)
- **E-graph saturation scaling (upgrades the SP6 feasibility sketch, which scoped scaling out):** a proper
  e-graph (hash-map hashcons + union-find + egg-style rebuild) saturated to fixpoint with commutativity and
  distributivity (the classic explosion driver) grows a consistent ~5.2x the initial program size, linearly not
  exponentially, in 4 rounds regardless of depth, because hashcons congruence deduplicates the equivalent forms.
  The feared eqsat explosion is tamed for well-behaved rule sets. The 4096-node cap bites at depth 5 (4102 vs
  10572 nodes) as a hard tractability bound. This validates two design decisions: eqsat is dev-time-only (the ~5x
  linear multi-round growth is compile-appropriate, not per-frame; the runtime uses the cheap-lowering const-fold
  + CSE subset), and the bounded streaming window is a hard safety mechanism (it guarantees termination and a
  memory bound for any consumer rule set, including pathological ones no static analysis can vet), not just tuning.
  (`benches/egraph-saturation-scaling/`)
- **Record-update reuse throughput (sizes the exact-meet payoff SK16 left open):** SK16 sized the peak-memory
  side of in-place-when-unique reuse (frontier-bounded, emit-time counting, no runtime refcounts); this sizes the
  throughput side on the record-update hot path that templating/config/script consumers hammer. Always-copy runs
  ~5 ns/update; in-place-when-unique runs 0.53 ns/update at 0% sharing (the mutable ceiling exactly, zero copies,
  9.72x), 1.52 ns at 20% (3.32x), 3.37 ns at 60% (1.50x), and never loses to copy. So exact-meet reuse is a pure
  win on both axes: frontier-bounded peak memory (SK16) and 1.5x to 9.7x faster mutation (this), with the entire
  cost paid at emit time (the runtime reads a uniqueness bit and branches, no refcount traffic). This is the
  "functional semantics, imperative speed" enabler for the authoring/templating consumers: locally-built records
  are mostly unique, landing in the 3x-to-10x regime. Build the reuse analysis. (`benches/record-update-reuse/`)
- **Interpolation output-building (the templating text-generation hot path):** the `Interp` form lowers to
  interleaved literal-span and formatted-value appends into the output sink, which is what op's biggest consumers
  (doc DSLs, config, typst/scribble templating) do all day. Across three strategies on a representative template:
  format-to-temp-then-copy 1538 MB/s, format-in-place 2337 MB/s, data-driven span-list (the interpreter-tier
  lowering) 1039 MB/s. Two decisions: the reserve/commit sink API must expose the cursor for in-place value
  formatting (1.5x over format-to-temp, purely from the avoided scratch and second copy), and the span-list
  lowering, though 2.2x slower than specialized straight-line, still runs at ~1 GB/s, so output-building is never
  a bottleneck (a 10 MB document is under 10 ms; real templated documents are kilobytes, produced in microseconds).
  Ship the data-driven span-list as the default (literals are residual constants, holes are format steps),
  interpreter-walked at ~1 GB/s; compile-side specialization to straight-line is an optional ~2x for the rare
  extreme-volume consumer. (`benches/interp-output-building/`)
- **Match lowering strategy (the Match core form op folded into the core set):** since Match does not translate to
  `If`s and lowers on its own, the compiler owns arm selection. Across arm counts with arbitrary sparse keys:
  if-chain is O(K) (0.79 to 16.3 ns as K goes 2 to 64), jump-table is flat O(1) (0.51 ns), decision-tree is
  O(log K) (0.96 to 5.35 ns); and a hot arm (90% hitting the first-tested arm) drops the if-chain to 0.60 to
  2.14 ns. The per-match lowering rule: dense keys (enum/tag, the common match-on-kind case) lower to a jump table
  (flat 0.51 ns, wins at K>=4); a profile-hot arm lowers to a hot-first if-chain (competitive, handles guards
  inline); sparse few arms (K<=3) to an if-chain; sparse many arms to a binary decision tree. The full lowering is
  Maranget pattern-matrix compilation, this per-node choice at each decision node, guards as a local in-bucket
  if-chain, nested patterns as a tree over scrutinees. The jump-table's flat 0.51 ns confirms tag-dispatch is
  essentially free, validating the cheap branching on the kind-tagged NaN-box value model.
  (`benches/match-lowering/`)
- **Iterator fusion (the Iter core form's pipeline lowering):** iteration pipelines (filter, map, emit-per-item)
  are pervasive in the consumers, and the lowering fork is materialized-intermediates vs fused, plus push vs pull.
  On opaque interpreted-model stages, materialized runs 3.43/3.61 ns/elem (2/3-stage) and fused runs ~3.20/3.23.
  Four conclusions: fusion is mandatory (materialized intermediates need allocation, forbidden by no-alloc, so the
  Iter form must fuse); the fusion edge grows with pipeline depth (7% at 2 stages, 11% at 3, because each
  materialized stage adds a full pass plus a temp while fusion stays one pass); push and pull are
  throughput-equal, so choose pull (strictly more composable, supports the lazy/early-terminating operators like
  take and first that consumers need); and fusion additionally unlocks vectorisation in the native tier (a fused
  reduction the vectoriser sees whole, which materialization's temp traffic blocks). The Iter form lowers to a
  pull-based fused pipeline with no per-stage allocation. (`benches/iter-fusion/`)
- **Project field-access strategy (the record hot path):** field access `record.field` splits by whether the
  record shape is statically known. Direct offset (compile-resolved) is 0.54 ns; a monomorphic inline cache is
  0.61 ns; hash lookup 1.27 ns; linear scan 1.57 ns. The load-bearing hazard: at a polymorphic site the inline
  cache is the WORST strategy (1.89 ns, slower than hash and linear, because the shape guard misses and pays a
  refill), so an inline cache must detect megamorphic sites and self-disable to a hash lookup. The rule: resolve to
  a direct offset wherever the shape is static (the majority given ahead-of-time compilation, so most projections
  are a free load); dynamic-shape access uses a monomorphic inline cache that self-disables to a hash lookup once
  the site goes polymorphic. The record's shape_id is the inline-cache guard key, the same discriminant the NaN-box
  tag and the Match jump-table use, so one shape id serves value branching, match dispatch, and field caching; and
  field names are interned u32 ids, so lookups compare integers not strings. (`benches/project-field-access/`)
- **Closure representation (Lambda + Apply), selected by escape:** the flat-capture vs linked-environment fork
  inverts by usage. Create-once-call-many favours flat (O(1) access at 0.53 ns vs linked's O(depth) 0.45 to
  2.84 ns); create-many-call-once favours linked (1.42 ns pointer creation vs flat's 2.5 to 4.1 ns capture copy).
  So there is no single best representation; escape decides it, and the design already computes escape via the
  reachability-binder rule plus region-promotion (the N*W fix). A non-escaping closure (the Iter-pipeline callback
  the pipeline consumes and drops) lowers to a linked environment (cheap, no copy, because the parent frame
  outlives it); an escaping closure (returned or stored) lowers to flat captures (mandatory, since the parent is
  reclaimed, and it falls out of the same region-promotion the design already does). Closure representation is
  therefore a consequence of the escape verdict, not a separate decision, and it keeps the common Iter-callback
  case cheap (1.42 ns, no capture copy). (`benches/closure-representation/`)
- **tnum abstract arithmetic (the compile-side numeric residual):** the design tracks numeric values with tnums
  (the eBPF-verifier tristate `{value, mask}` domain) to prove bounds and fold constants at compile time. The
  question is whether it can run on every numeric op. The linear and logical transfers are essentially free
  (tnum_add 0.95 ns, and 0.87, or/shl ~0.68, within 1.4x of concrete), so the compiler can track the linear
  domain pervasively (a 500K-node program is ~0.5 ms). The one expensive op is tnum multiply (149 ns, a 64-bit
  long-multiplication loop), which needs a known-operand fast path: both-known lowers to a concrete multiply
  (0.68 ns), covering multiply-by-constant (the majority), and only genuine unknown-times-unknown falls to the
  loop, paid demand-driven where the result actually feeds a proof. So the numeric residual is affordable as a
  pervasive compile-time analysis, with the standard abstract-interpretation discipline on multiply (cheap
  transfers, a fast path for the expensive op, demand-driven for the rest). (`benches/tnum-abstract-arith/`)
- **Partial-eval specialization (the graded binding-time PE):** SP7/SK15 proved PE selection works; this sizes how
  much it shrinks the residual and confirms termination. Three results: PE terminates by construction (the
  binding-time grade strictly decreases on every unfold, well-founded, verified, so it cannot diverge on a static
  recursion); it is cheap (~100 to 230 ns/node, ~50 to 100 ms for a 500K-node program, compile-time-fine); and its
  reduction tracks static structure, not just the static fraction. A random static/dynamic mix folds only 1.2x to
  2.9x (a node folds only if its whole subtree is static), but a block-structured program (a template: static text
  blocks plus dynamic holes) folds 3.8x to 7.0x, because each static block collapses to one constant. For the
  templating consumers this is the decisive result: PE eliminates the static document structure at compile time so
  the runtime executes only the dynamic holes, the "compile the template, run only the holes" property made
  quantitative, composing with the interp-output span-list lowering. (`benches/partial-eval-specialization/`)
- **Effect lattice inference + inclusion (the correctness core):** certified generation rejects any construct
  whose effect exceeds the target's permits, so the effect proof is the load-bearing correctness mechanism. The
  ordered lattice (none < Reads < Writes per family) is made free by thermometer-encoding the grade (none=00,
  read=01, write=11): then the lattice join is a bitwise OR, the graded-monad bind is the same OR, and the
  inclusion check is a subset test `(script & ~target) == 0`, all verified correct. Over an 8M-node DAG, effect
  inference (the OR-join) runs at 6.14 ns/node (memory-bound on the child gather, the op is one OR, cache-hot it
  matches lease-fixpoint's ~630 M-edge-OR/s), and the inclusion gate runs at 0.39 ns/node (2.5 G-node/s, so an 8M
  program's effect proof discharges in ~3 ms). So the correctness core is negligible on the compile side with
  thermometer encoding, and effect inference shares the exact bitmask-propagation machinery as the lease-fixpoint
  reachability analysis (one bottom-up DAG join, differing only in payload). (`benches/effect-lattice-inference/`)
- **Resolve pass name resolution (the vehje-resolve compile-side pass):** converting each variable reference to its
  (depth, slot) binder, the last named compile-side pass sized. Of three scope-lookup strategies, the flat
  shadow-stack symbol table (one hash `name -> binder stack`, push/pop on scope enter/exit) wins decisively at
  0.91 ns/ref (1.1 G-ref/s, O(1) per reference), versus hashed-per-scope 5.43 ns and linear scope-chain 13.05 ns
  (both O(depth)). It is 6x to 14x faster because it resolves in one hash lookup independent of nesting depth,
  where the scope-chain walks pay O(depth) that compounds for deeply-nested templating blocks and comprehensions.
  Resolution converts names to slots once at compile time (500K refs in ~0.45 ms), so runtime Var access is a
  direct slot read (O(1), already benched); the names live at compile time, the residual carries slot indices.
  This is the static half of name-to-slot mapping (lexical variables), distinct from the dynamic half (Project
  field access on a runtime-shaped record, which needs the inline-cache/hash). (`benches/resolve-name-scope/`)
- **Interner intern hot path (the lex/parse per-token cost):** interning a string to its u32 id runs on every
  token and underlies resolve, field access, and effects. It costs ~19 ns/token (51 to 55 M/s), string-hash-bound
  (hash-in plus compare-on-hit), and its knobs are second-order: load factor is irrelevant across 25% to 75%
  (common tokens hit on the first probe, so a high memory-cheap load factor is free), and hash choice barely
  matters (FNV about equals FxHash for short tokens). So the interner is not a bottleneck (500K tokens in ~10 ms)
  and does not warrant tuning; pick a simple hash and a high load factor. The real lever is avoiding redundant
  interns (id reused per distinct token; incremental re-parse uses a source-position-to-id cache), not table
  cleverness. (`benches/interner-intern-hotpath/`)

A Zig 0.16 API-notes reference (`research/202607211300_zig-0.16-api-notes.md`) consolidates the std/builtin
changes the sketches hit (`@Type` -> `@Enum`/`@Union`/`@Struct`, `std.time.Timer`/`posix.clock_gettime`/`io`/`fs`
reorganised, `ArrayList.empty`, catch-spacing, no `preserve_none`) so the implementation work does not
re-discover them.

Net across the expansions: the interpreter floor is fast (1.7 ns/instr for programs, 4.4 ns/node for value
walks), lease inference is fast (50 ms at 8M), transport is cheap, the switch-dispatch and 24-byte-record and
whole-column-fixpoint choices are all reconfirmed under wider conditions, and the native tier is repeatedly shown
to be a ceiling rather than a necessity.

Two load-bearing resolutions were also validated concretely:
- **Dual-locus dissolves to one engine (counter-audit's fix).** One Zig engine compiled once to a C-ABI object,
  linked by both a Rust dev-side host and a C runtime-side host, gives bit-identical results, so identity holds by
  construction, not by differential test. Cluster C Finding 2 is closed by returning to canon.
  (`sketches/202607211130_dual-locus-shared-object/`)
- **The native-tier core mechanism is feasible (Carmack break 3).** On aarch64 macOS, MAP_JIT allocate -> write an
  aarch64 stencil -> patch its immediate -> W^X toggle -> icache invalidate -> execute works (patched f(100)=142).
  The two-gate capability model is confirmed: the run-time probe succeeds where permitted (native available) and
  fails-and-tiers-down where not (iOS/consoles -> the fast interpreter floor). Only the stencil-EXTRACTION
  toolchain remains deferred. (`sketches/202607211200_copy-and-patch-mapjit-probe/`)

## Where the artifacts are

Sketches: `vehje/mock/research/sketches/2026072102*..1000_*` (each with a `findings.md`). Benches:
`vehje/mock/benches/{comptime-cost-cliff, interp-dispatch, record-width, incremental-scaling,
simd-structural-decode, releasefast-vs-safe}` (each with CSV + findings). Progress + per-item outcomes:
`.shared/state/vehje-overnight-2026-07-21.md`. Nothing was committed (mockspace TOPIC-phase gate; the artifacts +
findings on disk are the deliverable, per the intern's discipline).
