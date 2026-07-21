# Systems and codegen frontier: holes in 01 to 03, and novel compositions

**Date:** 2026-07-20
**Status:** Worker-fork deliverable, one shot. Audits `01`, `02`, `03` from a compiler-and-processor lens,
answers them, asks sharper questions, and proposes novel compositions. Settles nothing; several options per
item, none binding. The parent and op decide.
**Role taken:** a bleeding-edge compiler and processor-instruction savant who hates reaching for the
established shape when a composition of frontier pieces makes a better one. The rule is not to ignore the
theory `01` to `03` built on, but to notice where that theory is a generation behind the systems it models,
and where two pieces nobody has fused yet fuse cleanly here.
**Reads:** `01_theory-citation-verification-and-adjacent-threads.md`,
`02_settlements-and-next-architecture.md`, `03_core-nodes-typesystem-and-zig-bridge.md`, and the arc they
continue.

## The lens, and the one-paragraph thesis

`01` to `03` are programming-languages-theory complete. Gradual verification, region and reachability types,
translation validation, initial-algebra folds, binding-time analysis: the design is sound by every standard
those fields set, and the seven calls settle correctly against them. What the three docs systematically
under-weight is the other half of the problem, the systems and codegen half: how you actually generate a fast
runtime from a language definition, and how you actually run untrusted code that arrives at load time, safely,
at speed, with no compiler present. Those are not open research questions. They are two of the most active
frontiers in production compiler engineering, and two systems have shipped answers that are, almost exactly,
vehje's thesis. Deegen generates an interpreter plus a baseline JIT from one semantic definition, at
LuaJIT-competitive speed. Copy-and-patch runs a bytecode stream as native code with microsecond compilation
and no LLVM at the point of use, and it is the CPython 3.13 JIT. Neither is in the trail. Their absence means
the design is reasoning from first principles about problems with shipped, benchmarked answers, and it means
one hole the docs paper over (native execution for arriving scripts) is real, unstated, and closed only by the
frontier the docs skip. This document pokes the holes, then proposes the compositions that close them.

## Part A: the holes

### Hole 1 (the sharp one): "native is just a target" is a category error for arriving scripts, and no doc notices

`03` catches the ABI-and-tier contradiction and resolves it by demoting native from a tier to an ordinary
output target (`1513`'s framing). That is right for the taxonomy. It also hides the real problem. A target
that emits machine code needs a machine-code generator at the moment it lowers. For a bundled script (known
at our build time) that generator is our toolchain, and it is fine to invoke Zig or LLVM then. For an arriving
script that wants native speed, there is no compiler at the consumer, because a no-toolchain embed site is
the framework's entire identity. So under the design as written, "native" is silently a bundled-only target,
and arriving scripts are capped at interpretation or an internal bytecode. No doc states this. It is the
single largest unstated limitation in the arc, and it is exactly the limitation the "native is just a target"
sentence makes invisible: calling native a target makes it sound uniform with Lua-transpilation or
value-interpretation, when it is categorically different, because those targets emit data or text a later
stage consumes and native emits executable memory a CPU consumes, and producing executable memory from an
input unknown until load is the one thing the design forbids itself the tools for.

The answer is in Part B, novelty 1: copy-and-patch ships the code generator as data (pre-compiled stencils)
and patches at load with no compiler. It is the only shape that makes native a real tier for arriving scripts,
and it is production-proven.

### Hole 2: runtime generation is treated as unproven first-principles work; Deegen proved it, and its shape contradicts a couple of the docs' leans

`1513` and `1627` reason carefully, from the ground up, about generating a runtime from a language definition,
as if it were a novel move that needs its own soundness argument. It is a novel move for vehje's specific
three-axis-proof purpose, but the core act (compile a language's semantics to a fast VM) shipped in Deegen
(Xu, PACMPL 2026, arXiv 2411.11469; the 2023 workshop invited talk and the LuaJIT-Remake blog series).
Deegen takes one semantic description of the bytecodes and generates an optimized interpreter, a baseline JIT,
and the tier-up logic between them, and the generated Lua VM beats PUC Lua by 179 percent in the interpreter
and is within a third of LuaJIT's optimizing JIT. That is `1513`'s runtime generation, benchmarked, and it
lands three corrections:

- It is the existence proof the arc lacks. The metacompiler thesis is not speculative; bank Deegen as the
  reference that it works and at what speed.
- Its mechanism is not `1627`'s. `1627`'s load-bearing proposal is Zig `comptime` as the specialiser over
  Rust-emitted data tables. Deegen specialises through LLVM at build time and uses copy-and-patch for the JIT.
  So the comptime path is one build-time mechanism, and a different, proven one exists: extract stencils and
  emit specialised code via a real optimizing compiler at our build time, ship the artifact, no compiler at
  the consumer (the Terra `saveobj` shape the synth docs already name). The docs present comptime as the
  answer; it is an answer, and Deegen's is the one with benchmarks.
- Its single-semantic-source shape answers `03`'s pivotal experiment before it runs. `03` names "enumerate the
  L2 primitive vocabulary across the census families" as the experiment that decides the whole bridge. Deegen
  is that vocabulary, realised: a bytecode-semantics DSL rich enough for full Lua, and it did not sprawl, it
  stayed a bounded semantic language. The experiment reframes (Part C) from "enumerate primitives" to "author
  the semantic DSL and measure its coverage," which is a stronger, more direct test.

### Hole 3: the untrusted-load check is a throughput and soundness problem with a living production frontier the docs route around

Arriving scripts, and bytes read back from a spill sink, are untrusted, and `1315`/`02` correctly require a
bounds-and-depth validation before the walk. The docs then treat that validation as a generic given and cite
proof-carrying code (Necula, POPL 1997) as the assurance model. PCC is theoretically exact and effectively
dead in production. The thing that actually ships vehje's safety model, at planetary scale, is the eBPF
verifier: a load-time abstract interpretation that proves an untrusted bytecode program safe (bounds, ranges,
no wild memory) before the kernel will run it, then hands the proven program to a trusted JIT. That is
precisely vehje's shape (prove the arriving artifact safe at load, then run it fast), and its abstract domain
is a published, soundness-proven object: tristate numbers (tnum), with formal soundness and an optimal
multiply now merged into Linux (Vishwanathan et al., CGO 2022; the Agni "Verifying the Verifier" work, CAV
2023). Separately, the throughput half of validating untrusted structured bytes is simdjson (Langdale and
Lemire, VLDB 2019): gigabytes per second of SIMD structural validation of exactly the flat, self-describing,
index-linked shape vehje's value-arena is. The docs give neither. The answer (Part B, novelty 3) is to model
the load check as the eBPF verifier does, borrow the tnum domain proven-not-hand-rolled, run the structural
pass simdjson-style, and, the free composition, bound the verifier's cost by vehje's depth cap, because the
eBPF verifier's number-one scaling pain is path explosion and vehje already carries the finite-height bound
that kills it.

### Hole 4: `01`'s reachability-types bank oversells an unproven, expensive theory as the lease axis's fallback

`01`'s strongest single bank is reachability types as the lease axis's modern grounding and the name for its
inference-failure boundary. Reachability types are real and directly on-axis, and the bank is correct as far
as it goes, but the confirmation this pass returned is that they are research-only through 2026 (OOPSLA and
PLDI, no production implementation), their inference is subtle (the avoidance problem is a paper in itself),
and betting the mutable-Lua fallback on them is betting a shipping guarantee on unshipped research. The docs do
not flag this. The honest frontier picture: the pragmatic fallback for mutable, aliased values is not
necessarily a richer static type system at all; it can be a runtime mechanism. Vehje already has one in the
trail, Vale generational references, currently scoped to a CI oracle (`02` talk 7). Promoting it to an actual
runtime residual for the mutable consumer (a generation check on deref, elided over the immutable regions
exactly as Vale elides them) is a shipping answer, where reachability types are an aspirational one. A second
shipping-adjacent option is revocable capabilities (Typestate via Revocable Capabilities, OOPSLA 2025), which
is the capability reading `02` already reached for, made into a typestate discipline. Give op the pragmatic
fork, not only the research one.

### Hole 5: `02`'s "seven blesses, the field has standard answers" undersells the novelty and hides where the standard answers fail to compose

`02` opens Part I by asserting none of the seven is a gamble, each is the field's standard answer to a
correctly-posed question. That is true call by call, and it is the wrong frame for the whole. No existing
system does all of it together: three static proof axes, plus certified generation, plus no-alloc, plus
arriving scripts, plus a native tier. The research contribution is not any single call; it is the composition,
and the interesting engineering is exactly where the standard answers refuse to compose. Four such seams,
named so they are not lost inside seven routine blesses: native and arriving-scripts do not compose without
copy-and-patch (hole 1); the static lease axis and mutable aliasing do not compose without a runtime residual
or unproven types (hole 4); comptime folding and bundled-content cost do not compose (the measured cost cliff
`02` call 2 correctly moves to a build step); and one kernel over three loci does not compose without a real
differential gate (`02` talk 4's own devil's advocate). Those seams are where vehje is doing something no
shipped system has, and they deserve to be named as contributions rather than dissolved into "the field has an
answer," because the field has an answer to each in isolation and none to their intersection.

## Part B: novel compositions

Each is a composition of pieces that exist, aimed at a seam the holes exposed. Options per item, none binding.

### Novelty 1 (keystone): certified copy-and-patch stencils as the native tier for arriving scripts

The move. At our build time, for each primitive in the family vocabulary (the L2 set `03` and Deegen both
point at), compile a stencil: a machine-code fragment with holes for the operands, live values, and successor
addresses, produced by a trusted optimizing compiler (LLVM or Zig, at our build time only). Ship the stencils
as per-ISA data inside the composed runtime. At the consumer's load time, the runtime selects the stencils for
a lowered script's IR nodes, concatenates them, and patches the holes: this is a bounded relocation pass, a
memcpy plus a fixup table, no compiler present. The result is native code for an arriving script, at
baseline-JIT quality, with zero toolchain at the embed site. This is exactly the CPython 3.13 JIT (Xu and
Kjolstad, arXiv 2011.13127) and Deegen's baseline JIT, lifted to vehje's per-language setting.

What it closes and collapses:

- Hole 1 directly: native becomes a real tier for arriving scripts.
- `03`'s Approach-A-versus-C tension dissolves. Family behavior is no longer a choice between an interpreted
  primitive vocabulary (slow) and generated Zig source (`1627` rejected it). It is a stencil, which is data
  that runs at native speed. "Hybrid data plus narrow behavior" becomes "all data, where some of the data is
  machine-code stencils," which is closer to op's stated all-data taste than the interpreter-plus-escape shape.
- The certification story extends cleanly. The stencils are compiled by a trusted compiler at our build time,
  so they inherit certification 2 (the build-time compile). The load-time patcher is a small, inspectable,
  bounded relocation pass, and this is exactly the eBPF split that ships in the kernel: the verifier proves the
  bytecode safe, then a trusted JIT patches it to native, and the trusted base is {the verifier's soundness,
  the JIT}. Vehje's trusted base becomes {the stencil compiler at build, the patcher, the load verifier of
  novelty 3}, all auditable, none a compiler-at-the-consumer.

Options:

- (a) Full copy-and-patch native tier, every hot family primitive stencilled, the CPython and Deegen shape.
- (b) Stencils for the hot primitives only, interpret the cold rest, tier up on heat. This is Deegen's actual
  two-tier shape and the honest engineering default.
- (c) A WebAssembly baseline instead of native stencils: lower to WASM, ship one tiny embedded runtime with a
  load-time validator (the wasm2c or a copy-and-patch WASM baseline). Trades peak speed for a single portable
  target and a formally-studied safety model (Part D). This is the option that also answers "what is the
  portable native," and it folds novelty 3's verifier in for free because WASM validation is that verifier.

Devil's advocate. Stencils are per-ISA, so the build-time matrix grows with target architectures (real, but
build-time-only, and the census's near-term targets are few). The patcher is a mini-linker and a W-xor-X
memory dance, a genuine TCB item the docs do not currently have (Part C asks about its certification). And
copy-and-patch gives baseline-JIT quality, not optimizing-JIT quality: limited cross-stencil register
allocation, no global scheduling. The honest ceiling is LuaJIT-baseline-tier, roughly a third slower than a
full optimizing JIT on Deegen's numbers. For config, scripting, and lint consumers that is far more than
enough; for ikiuni's per-frame evaluation it is a bench question, not an assumption, and the tiered option (b)
plus the runtime budget (`03` piece 3) is the containment.

### Novelty 2: one semantic definition to interpreter, stencils, and certification tests (Deegen, lifted to many languages)

The move. Author each family's runtime semantics once, as a bytecode-semantics definition (Deegen's single
source of truth). From that one definition, mechanically derive three artifacts: the tier-0 interpreter arm,
the copy-and-patch stencils of novelty 1, and the differential-gate certification tests of `02` talk 7 and
talk 4. This is Deegen's exact contribution (one semantic description, an interpreter and a baseline JIT and
the tier-switch fall out), generalised from Deegen's single-language setting to vehje's multi-language one.

Why it is the right answer to `03`'s pivotal question. `03` frames the bridge decision as "does the L2
primitive vocabulary stay bounded or sprawl." Deegen reframes it: the vocabulary is a semantic DSL, and the
question is whether one DSL covers the census. Deegen's DSL covered full Lua without sprawl, which is a strong
prior. And it unifies three of `02`'s separate talks (the three-loci kernel, the harness, and the interpreter)
into consequences of one semantic source, which is the kind of collapse that signals the frame is right.

Options:

- (a) Adopt Deegen's shape directly: semantics in a host language (Deegen uses C++), LLVM at build time, both
  interpreter and JIT generated. Proven, but pulls LLVM into the build-time toolchain.
- (b) A vehje-native semantic DSL over the L2 vocabulary, emitted from the Rust side as data, lowered through
  the copy-and-patch stencils. Fits the all-data taste, keeps LLVM out, costs a DSL design.
- (c) A hybrid: the DSL is data, but a family whose semantics need a primitive the DSL lacks emits a
  generated-behavior stencil, the narrow escape `03` already sized. The escape is now a stencil, not Zig
  source, so it is still data at native speed.

Devil's advocate. Deegen's proven path leans on LLVM at build time, a heavy dependency, though a build-time
one the design already permits (the generator is dev-time; only the artifact ships). A vehje-native DSL is a
real surface to get right, and it does not exist yet. Either way the L2-vocabulary experiment is the
prerequisite, now reframed as "author the DSL, measure census coverage."

### Novelty 3: the depth cap is the load verifier's complexity bound, so compose them into one analysis

The move. `02` treats the depth cap (call 4) and the untrusted-load traversal (call 3, talk 5) as related but
separate, and half-sees that they are the same finite-height fact. Push it all the way. Adopt the eBPF
verifier's abstract-interpretation load check for the arriving-script range, bounds, and lease-residual
verification, over a tnum-style tristate domain (borrowed with its soundness proof from CGO 2022, not
hand-rolled), and let vehje's depth cap bound the verifier's path exploration. The eBPF verifier's dominant
scaling pain is path explosion at branches; vehje's lease lattice is a finite chain whose height is the depth
cap, so the exploration is bounded by construction. One design decision (the cap) buys the no-alloc frontier
bound, the work-stack bound, the untrusted-traversal depth bound (all three `02` already names), and now the
load verifier's termination and cost bound, from the same finite-height number. That is a genuinely novel
composition: the proof-theoretic cap and the systems-frontier verifier's complexity bound are one quantity, so
they are one decision, and the arriving-script check stops being a hand-rolled traversal and becomes a
bounded, sound abstract interpretation with borrowed proofs.

Options:

- (a) Full abstract-interpretation load verifier over tnum plus intervals, the eBPF shape: strongest guarantee,
  heaviest, but the abstract operators are published and proven.
- (b) simdjson-style SIMD structural validation for the bounds-and-shape half, plus a light range check:
  throughput-first, weaker on value ranges, ideal when the arena is huge and mostly structural.
- (c) Two-speed: a process-private trusted-local sink skips the verifier (the `02` talk 5 exemption); the
  untrusted path runs the full (a). The cap makes both terminate.

Devil's advocate. A real abstract interpreter at load is more machinery than a bounds-checked memcpy walk, and
for a tiny arriving script it is overkill; the mitigation is that its cost is bounded by the cap and paid only
on the untrusted path, and the domain is borrowed, not invented. simdjson-style validation needs the arena
layout to be SIMD-friendly (aligned, predictable), which is a format constraint to accept up front, and one
the flat index-linked value-arena is already close to.

### Novelty 4: equality saturation for the macro, const-fold, and CSE stage, with effect-edge-constrained extraction

The move. The docs bank LMS's hand-rolled CSE and its effect-dependency edges. The frontier for the IR-to-IR
macro-expansion plus const-fold plus common-subexpression stage is equality saturation: build an e-graph of
equivalent rewrites and extract the best, rather than applying rewrites destructively in an order that can
foreclose a better one (egg, PACMPL 2021; egglog, PLDI 2023, which adds a Datalog side; Oatlog, an
ahead-of-time-compiled e-graph engine, EGRAPHS 2025). The specific composition for vehje: run the compile-stage
rewriting as equality saturation, and constrain extraction by the LMS effect edges (do not extract a form that
reorders an effect past a dependent read). egglog's Datalog side expresses exactly those ordering constraints,
so effect-aware extraction is not a bolt-on, it is a natural egglog rule set. This supersedes hand-rolled CSE
with rewriting that is optimal within its rule set, and it unifies const-fold, macro expansion, and CSE into
one saturate-then-extract stage instead of three passes.

Options:

- (a) Full egglog-style saturate-and-extract at the compile stage, effect edges as extraction constraints.
- (b) A bounded, fixed-capacity e-graph (a research contribution in itself, since egg and egglog allocate):
  the compile stage runs over host-lent memory where a bounded bump arena is already the discipline (`03`
  piece 6), so a fixed-capacity e-graph is plausible and would be the no-alloc-native version.
- (c) Keep LMS edges for scheduling, use an e-graph only for the const-fold sub-stage, where the payoff is
  clearest and the graph is smallest.

Devil's advocate. egg and egglog allocate, and a no-alloc bounded e-graph is unproven; option (b) is real
research risk. Equality saturation can blow up on a rich rule set, so the rule set must be curated and the
saturation budgeted, which is the same budget discipline the runtime already needs. For a first cut, (c) buys
most of the value at least risk.

### Novelty 5 (speculative, systems-frontier): the lease-as-capability made hardware-real on CHERI

The move. `02` reads the lease bit as substructural (affine consume versus unrestricted link) and the sink as
an object-capability, and both readings point at the same hardware: CHERI, where a pointer is an unforgeable
capability carrying bounds and permissions the CPU enforces. A CHERI output target could make the lease axis
hardware-backed: a region is a capability with bounds, closing a region revokes the capability, and a
use-after-close faults in silicon rather than relying on the static proof alone. CHERI is past research
(Morello ships on a Neoverse N1; the CHERI Alliance formed in 2024; CHERIoT targets embedded), and the exact
cautionary-and-enabling reference is "Pitfalls in VM Implementation on CHERI: Lessons from Porting CRuby"
(arXiv 2603.05645), plus a verified CHERI C temporal-safety memory model (CPP 2025). Because vehje's target
model is plug-in, a CHERI target is additive, not a core commitment, and it is the kind of result that reads
as a contribution: a metacompiler whose lease proof is optionally enforced by capability hardware, closing the
gap the docs admit (Zig has no lifetime types, so the lease axis is the least statically-Zig-provable, `03`
piece 5) with a hardware backstop instead of only a residual runtime tag check.

Devil's advocate. CHERI is not on any census consumer's near-term target list, VM-on-CHERI has real porting
pitfalls (the CRuby paper is a list of them), and this is a speculative bank, not a plan. It earns its place as
the answer to "what would make the lease axis's weakest link strong," recorded so the option exists, not
scheduled.

### Novelty 6 (the SIGGRAPH-and-GDC-native framing op asked for): the output spectrum is MLIR dialects, the inclusion model is SPIR-V capabilities, specialisation is uber-shader partial evaluation

Not a new mechanism, a frame that lets a rendering-and-systems audience recognise vehje's abstractions as the
ones they already ship. The output-generation spectrum (one IR, many targets, inclusion-not-coverage) is
MLIR's dialects lowered to many backends, at production scale (Lattner et al., CGO 2021; IREE as the
model-to-many-hardware existence proof), and the family axis is precisely an MLIR dialect. The
inclusion-not-coverage proof (a target declares `Supports` and `Permits` and refuses the rest by name) is the
SPIR-V `OpCapability` model checked by `spirv-val` against a target environment, which the synth prior-art
already cites and which is the SIGGRAPH-native precedent for exactly vehje's inclusion axis. And the whole
Futamura story (a compiler is a specialised interpreter; native is how much you residualise) is, in a
rendering audience's own vocabulary, uber-shader specialisation: partial-evaluating a general shader over its
static parameters to a specialised variant, which every engine ships and every GDC rendering talk assumes.
Bank MLIR and IREE as the production-scale existence proof of the output spectrum, SPIR-V `OpCapability` as the
inclusion precedent, and Slang's multi-target IR (SPIR-V, HLSL, and more from one front end) as the closest
single-repo analogue of vehje's one-IR-many-targets shape. This frame is what makes a systems or rendering
reviewer trust the design on sight: they are the abstractions that reviewer already builds on.

## Part C: sharper questions back to the docs

- If native-for-arriving-scripts is copy-and-patch, what certifies the patcher? It is a new TCB item the arc
  does not have. The eBPF answer is the clean split: the load verifier is the proof, the JIT-patcher is
  trusted and small and audited, and the two together are the trusted base. Adopt that split explicitly rather
  than folding the patcher silently into "certified generation."
- Does `03`'s L2-vocabulary experiment subsume into "author the Deegen-style semantic DSL and measure census
  coverage"? That is a stronger, more direct experiment than enumerating primitives, and it produces the
  interpreter and stencils as a byproduct rather than as separate later work.
- Is `1627`'s comptime specialiser actually the right build-time mechanism, or is LLVM-at-build-time (Deegen's
  proven path) better for stencil extraction, with comptime kept for the `@Type` reification it alone can do?
  The docs never compare the two; Deegen suggests a two-tool build (LLVM for the native tier, comptime for the
  type-gen), which matches the three-loci split `02` already blessed.
- The lease mutable-consumer fallback: research reachability types, versus runtime generational references,
  versus revocable capabilities. Which does op want to ship on? A defensible split is generational references
  as the shipping fallback (a runtime residual, elided over immutable regions) with reachability types as the
  aspirational static story, so the framework ships without betting on unproven inference.
- Is the value-arena laid out for SIMD structural validation (aligned, predictable strides) so the untrusted
  load check can run at simdjson throughput, or does the format need a constraint added now to allow it later?
  This is a cheap decision now and an expensive format change later.

## Part D: banked, with reachable locations

Systems and codegen frontier (new to the arc):

- Copy-and-patch compilation. Xu and Kjolstad, "Copy-and-Patch Compilation," OOPSLA 2021, arXiv 2011.13127,
  https://arxiv.org/pdf/2011.13127 . Now the CPython 3.13 JIT (bytecode copy-and-patch). Lands on: novelty 1,
  the native tier for arriving scripts.
- Deegen. Xu, "Deegen: A JIT-Capable VM Generator for Dynamic Languages," PACMPL 2026, arXiv 2411.11469,
  https://arxiv.org/pdf/2411.11469 ; the LuaJIT-Remake blog series, https://sillycross.github.io . The
  existence proof that runtime generation works at LuaJIT-competitive speed from one semantic definition.
  Lands on: hole 2, novelty 2.
- The eBPF verifier and tristate numbers. Vishwanathan et al., "Sound, Precise, and Fast Abstract
  Interpretation with Tristate Numbers," CGO 2022, https://doi.org/10.1109/CGO53902.2022.9741267 ; "Verifying
  the Verifier: eBPF Range Analysis Verification" (Agni), CAV 2023,
  https://people.cs.rutgers.edu/~sn349/papers/agni-cav2023.pdf . The production home of load-time
  abstract-interpretation safety verification of untrusted bytecode. Lands on: hole 3, novelty 3.
- simdjson. Langdale and Lemire, "Parsing Gigabytes of JSON per Second," VLDB Journal 2019,
  https://arxiv.org/abs/1902.08318 . SIMD structural validation of untrusted flat data at GB/s. Lands on:
  hole 3, novelty 3 option (b).
- WebAssembly as a sandboxing substrate. "Provably-Safe Multilingual Software Sandboxing using WebAssembly"
  (CMU, Parno et al.), https://www.andrew.cmu.edu/user/bparno/papers/wasm-sandboxing.pdf ; RLBox plus wasm2c in
  Firefox 95, https://hacks.mozilla.org/2021/12/webassembly-and-back-again-fine-grained-sandboxing-in-firefox-95/ ;
  Wasmtime security model, https://docs.wasmtime.dev/security.html . Lands on: novelty 1 option (c), the
  portable native tier with a formally-studied load validator.
- Equality saturation. "egg: Fast and Extensible Equality Saturation," PACMPL 2021,
  https://dl.acm.org/doi/10.1145/3434304 ; egglog (Datalog plus eqsat), PLDI 2023; Oatlog (AOT-compiled
  e-graph engine), EGRAPHS 2025. Lands on: novelty 4, the compile-stage macro, const-fold, and CSE.
- CHERI and Morello. CHERI Alliance (2024); "Pitfalls in VM Implementation on CHERI: Lessons from Porting
  CRuby," arXiv 2603.05645, https://arxiv.org/pdf/2603.05645 ; "A CHERI C Memory Model for Verified Temporal
  Safety," CPP 2025. Lands on: novelty 5, the hardware-backed lease axis.
- Revocable capabilities. "Typestate via Revocable Capabilities," OOPSLA 2025,
  https://dl.acm.org/doi/10.1145/3808323 . Lands on: hole 4, the mutable-consumer lease fallback as a typestate
  discipline.
- MLIR and the rendering-audience frame. Lattner et al., "MLIR: Scaling Compiler Infrastructure for Domain
  Specific Computation," CGO 2021, https://arxiv.org/abs/2002.11054 ; IREE; SPIR-V `OpCapability` and
  `spirv-val` (already in the synth prior-art); Slang's multi-target IR. Lands on: novelty 6, the output
  spectrum and inclusion model as production-scale precedent.

Cross-referenced from the arc (relied on above):

- Vale generational references (`02` talk 7): promoted from CI oracle to shipping runtime residual, hole 4,
  novelty 1's certification split.
- The depth cap as finite lattice height (`02` call 4): the free bound on the load verifier, novelty 3.
- LMS effect edges (synth purity doc): the extraction constraints for the e-graph, novelty 4.
- Terra `saveobj`, the generator-is-dev-time-only shape (synth staged-metaprogramming doc): the certification
  model for build-time stencil compilation, novelty 1.
- Object-capability (Miller 2006, `02` talk 5) and substructural typing (Girard, Wadler, `02` call 1): the two
  readings that point at CHERI, novelty 5.

## Closing

The arc is programming-languages-theory sound, and `01` to `03` prove it. Where it is thin is the systems and
codegen frontier, and there the finding is not that the design is wrong but that it is reasoning from first
principles about two problems that have shipped, benchmarked answers: generate a fast runtime from a semantic
definition (Deegen), and run untrusted code safely at load with no compiler present (eBPF, WebAssembly). The
single keystone move is certified copy-and-patch: it closes the one hole the docs make invisible (native for
arriving scripts), it collapses the data-versus-behavior tension into all-data-where-some-data-is-machine-code,
and it slots into the eBPF-shaped verify-then-patch split the design half-has already. Everything here is an
option, none binding, and the one to put in front of op first is the copy-and-patch native tier, because it is
the piece the current design silently cannot build and the frontier already did.
