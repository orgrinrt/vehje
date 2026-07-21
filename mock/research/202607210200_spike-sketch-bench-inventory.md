# Inventory: spikes, sketches, and benches to run before locking the design

**Date:** 2026-07-21
**Phase:** research (inventory)
**Purpose:** the de-risking worklist to run autonomously overnight so the morning's design continues with proof
and numbers. This is a comprehensive comb of the whole arc, not just the recent Cluster C and Core-re-derivation
items, because the biggest novelties (the no-alloc relational engine, the bounded streaming e-graph, the graded
spine, the comptime cost cliff, the owed 1845 experiment arms) were accrued earlier and are the load-bearing
risks.
**Repo state:** Zig 0.16.0 on PATH, Rust nightly-2026-05-28 pinned, `runtime-zig/` scaffolded, `vehje-runtime-abi`
has `wire.rs`/`encode.rs`, the arvo `mock/benches/` harness pattern to clone. No `mock/benches/` in vehje yet.
**Discipline:** sketches in `mock/research/sketches/<ts>_<topic>/` (WORKS/FAILS/INCONCLUSIVE, committed as audit
trail). Benches in `mock/benches/` on the harness (cdylib variants, shared workload, CSV + meta + findings, never
a bare `Instant::now()` loop). Spikes are larger scoped experiments.

## The overnight-critical set (P1, run first, all buildable tonight)

- **SK1 certified-generation core** [sketch]. Rust emits a validated family-table as data; Zig comptime
  specialises it to an IR tagged-union with the illegal family unrepresentable plus an exhaustive-`switch` total
  dispatch; both certifications confirmed; the comptime-folded inclusion check (`@compileError` for a known
  program, one runtime bitop for an arriving one). The architecture's crux (1627's own settling experiment).
- **SK3 Zig dispatch + `preserve_none` probe** [sketch]. Does 0.16 expose a `preserve_none`/`ghccc`-equivalent;
  does `@call(.always_tail)` with the walk pointers as explicit args keep them register-resident. Quick, answers
  a Cluster C open item and the interpreter floor.
- **SK5 iterative comptime kernel** [sketch]. 1845 says comptime recursion segfaults the Zig compiler, so the
  kernel must be an explicit defunctionalised work-stack machine. Confirm the iterative kernel compiles and
  computes the recursive spec (the functional-correspondence check). Load-bearing: SK1 and the whole comptime
  path depend on it.
- **SK4 reachability binder-rule + region-promotion** [sketch]. The executable `N*W` fix (splice-and-drop at
  `Let`/`Lambda` bounding the naming set to `W`, region-promotion bounding the escape set), validated on the case
  that broke the `InScope`-only fix. Retires a known bug.
- **SK2 no-alloc `Handle` + bounded multi-shot** [sketch/spike]. CR1's novelty: multi-shot resumption over a
  host-lent budget with a comptime budget-fit check; probe the continuation representation. The biggest new
  research risk; INCONCLUSIVE is a fine result.
- **BN0 comptime cost cliff** [bench]. 1845/05's measured superlinear finding: specialising the engine to the
  small language table (the comptime sweet spot) versus folding bundled content at comptime (the cliff). This is
  what the three-loci split (content validation to a native `build.zig` step) rests on, and it was owed and never
  measured. Stand up `mock/benches/` for it.

## By subsystem (the full catalogue, so nothing is lost)

### A. Certified generation and the comptime substrate
- SK1 cert-gen core (P1, above).
- SK5 iterative comptime kernel (P1, above).
- **SK6 byte-shift-not-pointer-overlay decode** [sketch] (P2). `@embedFile` alignment forces byte-shift decoding,
  not pointer overlay (1845). Confirm.
- **SK7 panic-free untrusted path** [sketch] (P2). A Zig panic aborts the host across the C ABI (1845), so the
  untrusted path must be panic-free (error unions, not panics). Confirm the discipline holds.
- BN0 comptime cost cliff (P1, above).
- **SP3 the three-loci content-validation `build.zig` step** [spike] (P2). Content validation as a native
  `build.zig` step running the identical kernel (1845). Feasibility of the build-step locus.

### B. The graded (co)modal proof spine
- **SP4 graded judgment composes and typechecks** [spike] (P2, high value). Encode the graded judgment (effect
  graded monad + lease coeffect + binding-time modality + assurance grade) and confirm it composes and the axis
  interactions fall out as typing rules (Granule is the evidence; a vehje-specific encoding). The identity's core.
- **SK8 assurance grade as a binding-time projection** [sketch] (P2). The assurance grade computed from the
  knowledge-source lattice (2001 item 5).
- **SK9 binding-time modality over the knowledge-source join-semilattice** [sketch] (P2). D4; the four sources
  (language-author/bundler/host-loader/runtime) joined, partial bundling as a join.

### C. The effect and handler discipline (Core-re-derivation)
- SK2 Handle + bounded multi-shot (P1, above), which subsumes: effects-as-operations, graded algebraic effects,
  `Permits` as unhandled-operation inclusion, and the AARA-style budget-fit validation (CR1's re-opened resource
  bound).
- **SK10 host-calls and macro expansion as handler instances** [sketch] (P2). Show a host-call and a macro
  expansion as two handler installations of the one discipline (D3).

### D. The lease and reachability axis
- SK4 binder-rule + region-promotion (P1, above).
- **SK11 reachability bitmask + shared-implies-promoted single-pass** [sketch] (P2). The qualifier as a
  fixed-capacity AccessSet-bitmask with the shared-implies-promoted discipline making DAG inference single-pass
  (2001 item 3).
- **SK12 depth-lease floor metatheorem** [sketch/proof] (P2). The degenerate LIFO depth-lease and its
  proven-cheap metatheorem (Calcagno-Helsen-Thiemann restricted to the LIFO chain, 1845).
- **SK13 generational-reference residual** [sketch] (P3). The per-reference dynamic check at the avoidance
  boundary.
- **SK14 Scala-3-style capture/separation as the lease production floor** [sketch] (P3, 2055).
- SP2 avoidance-rate + distributions (P2, corpus-dependent, below).

### E. The compile-stage engine (the relational-fixpoint brain, the biggest under-the-hood novelty)
- **SP5 no-alloc semi-naive relational-fixpoint engine** [spike] (P2, major). The 2055 engine: a no-alloc
  Datalog-with-congruence semi-naive evaluator over the three bounds, lease/eqsat/load-verify as queries,
  leapfrog-triejoin + AGM-bounded joins. Scope to a minimal working evaluator over one rule set (lease). This is
  the compile-stage brain and the largest single piece; a scoped feasibility spike tonight.
- **SP6 no-alloc bounded streaming equality-saturation e-graph** [spike] (P2, major novelty). 2001 item 1:
  slotted + colored e-graph, the graded well-founded PE unfold, bounded by the depth cap, streaming a bounded
  window. The flagged original work. Scope to a minimal saturate-then-extract on a bounded window.
- **SK15 PE-as-extraction-objective** [sketch] (P3). The mix equation as the e-graph extraction preference (2001
  item 2); part of SP6 once SP6 stands.
- **SK16 Perceus exact-meet + in-place reuse** [sketch] (P2, 2055). The collector-free memory model for produced
  values.
- BN3 incremental scaling / differential dataflow (P2, below).

### F. The interpreter, dispatch, and execution
- SK3 dispatch + preserve_none (P1, above).
- **SK17 CFG-of-blocks interpreter** [sketch] (P2). Block/function wire tables, forward-linear-scan within
  straight-line blocks, control at terminators, the depth-cap-bounded frame stack (Cluster C fix). Feeds BN1.
- **SP7 proof-directed / graded strategy selection** [spike] (P3). The bounded evaluator portfolio; native and
  copy-and-patch as one strategy chosen where the grade proves it wins.
- BN1 dispatch shapes (P1-adjacent), BN2 record width (P2), below.

### G. The load and untrusted path
- **SK18 typed structural decode** [sketch] (P2). Parse-don't-validate over the range-typed flat arena: complete,
  linear, the acyclicity-by-construction from children-before-parents. Subsumes the 1315 bounds-checked pass.
- **SK19 tnum load verifier** [sketch] (P3). Tristate-number bounded abstract interpretation (2001 item 4,
  eBPF-shaped), bounded by the lease-lattice height.
- **BN4 SIMD structural decode throughput** [bench] (P3). simdjson-style SIMD over the alignment-and-stride
  predictable arena.

### H. The value transport
- **SK20 value-arena encode/decode** [sketch] (P2). Validate/extend `vehje-runtime-abi`'s `wire.rs`/`encode.rs`:
  fixed-width records, flat child-index pool, byte blob, relative indices, children-first, no back-patching,
  zero-copy round-trip, no-alloc.
- **SK21 reserve/commit sink + cross-chunk lemma** [sketch] (P2). The host-lent sink (in-process two-fn-ptr C-ABI
  and out-of-process stdout pipe), streaming chunks at whole-subtree boundaries, the chunker constrained so
  cross-chunk-implies-promoted holds by construction. The cross-chunk lemma was flagged build-blocking in 1845.

### I. Diagnostics
- **SK22 provenance witness reconstruction** [sketch] (P3). Provenance cheap on the happy path, the derivation
  witness reconstructed lazily on failure (Souffle proof-tree on the provenance-semiring), the six schema
  additions into the wire format.

### J. The owed 1845 widened-experiment arms (a set that was gated to the doc changelist and never ran)
- **BN0 scaling-cost probe** = the comptime cost cliff (P1, above).
- **SK23 differential-testing gate** [sketch] (P2). The kernel run at both binding times, asserting identical
  output; the grade-0 shadow of the proof.
- **SK24 forced-spill round-trip** [sketch] (P3). The streaming spill path exercised end to end.
- **SK25 C-host panic-free arm** [sketch] (P2, folds with SK7). A C host calling across the ABI, confirming no
  panic crosses.
- **BN5 ReleaseFast vs ReleaseSafe axis** [bench] (P3). The safety-check cost, measured.

### K. The two gating experiments (2001, 1845)
- **SP1 semantic-definition coverage** [spike] (P2). Author a Deegen-style semantic definition over a subset
  (twelve Core forms plus one small family), measure whether the primitive vocabulary stays bounded, produce the
  interpreter arm as a byproduct (feeds SK1, BN1, SP5).
- **SP2 reachability avoidance-rate + distributions** [spike] (P2, corpus-dependent). Failure rate plus
  live-binder-width plus arity distributions on a Lua-shaped corpus (`~/Dev/stellar-heritage/` read-only), one
  instrumentation pass; feeds BN2 and the region-id width.

### Deferred (named, not run tonight)
- Native-tier copy-and-patch stencil vs interpreter bench: needs the stencil extraction toolchain built first
  (weeks of object-format work), not overnight-feasible; the gating bench for the native tier, waits for that.
- CHERI hardware-assurance lease target: hardware-dependent, out of scope.
- The `lambda_veh` proof skeleton (point 4): a writing artifact producing no numbers; draft only if the window
  has slack after the sketches, benches, and spikes.

## Run order for the overnight window

P1 first, cheapest-and-most-foundational leading: SK3 (version + dispatch), SK5 (iterative comptime kernel, since
SK1 rests on it), SK1 (the crux), SK4 (the bug fix), then stand up `mock/benches/` and run BN0 (the owed cost
cliff) and BN1 (dispatch shapes) and BN2 (record width). Then SK2 (the big novelty). Then the P2 sketches in
dependency order (SK20/SK21 transport, SK18 decode, SK17 CFG interpreter, SK16 Perceus, SK11 reachability bitmask,
SK9/SK8/SK10 spine pieces, SK6/SK7/SK23/SK25 comptime arms), the P2 spikes (SP4 graded spine, SP5 relational
engine scoped, SP6 e-graph scoped, SP1 coverage, SP2 corpus), and BN3 incremental scaling. P3 items as the window
allows. Each artifact commits as it lands, so the morning has the trail however far the window reaches, and a
morning summary lists every outcome (WORKS/FAILS/INCONCLUSIVE per sketch, the number and findings per bench).
