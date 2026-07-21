# Better approaches: bleeding-edge and cross-domain upgrades to the full-pipeline proposal

**Date:** 2026-07-20
**Status:** Worker-fork deliverable, one shot. Explores better approaches for each choice in the converged
full-pipeline proposal, from recently-released (2023 to 2026) research and from cross-domain lessons (databases
and dataflow, NN/ML, nonlinear dynamics). Settles nothing; every suggestion is an option to attack, several per
choice, with the specific fold into vehje named. Claims are web-verified this pass; sources at the end.
**Reads on:** the converged shape in `design_rounds/202607202001_topic.converged-architecture-shape.md` (the
five calls, the reachability-lease fit, and "The bleeding-edge full-pipeline proposal"), the five continuation
docs (`202607201854_certgen_panel_continuation/01`..`05`), and the synth family.

## One-paragraph thesis

The proposal is theory-complete on the pieces it names, but several of its choices reach for the 2021-era
version of an idea when a 2024-to-2026 result is both stronger and a better fit, and three whole cross-domain
bodies (database provenance and worst-case-optimal join, precise reference counting with reuse, and automatic
amortized resource analysis) answer needs the proposal currently hand-rolls. The load-bearing upgrades: the
equality-saturation stage needs **slotted e-graphs** (vanilla e-graphs cannot represent binders, and vehje's IR
is a lambda calculus with binders) and should compile the theory ahead-of-time with **oatlog**, not interpret
**egglog**, which is the same metacompiler move vehje already makes for its runtime, applied to its own compile
stage; the lease axis should ground on **Scala 3 capture checking** (a production-shipping reachability
discipline, which partially retires the "reachability is research-only" worry); runtime generation has a lighter,
theme-consistent alternative in **weval** (partial evaluation of the interpreter, shipped, no runtime codegen);
the memory model composes with **Perceus reuse** (reachability-proven uniqueness becomes in-place update); the
assurance-from-binding-time move is an instance of **provenance semirings**; and the single depth cap generalises
to **automatic amortized resource analysis** potentials. The genuinely-new-frontier ask (nonlinear computing)
has one honest, non-hardware thread: the design's fixpoints are dissipative flows, and nonlinear-dynamical
acceleration gives convergence-rate bounds, not just termination. Analog, neuromorphic, and reservoir computing
do not fit a deterministic no-alloc runtime, and that is said plainly so the door is not left ajar.

## The oatlog-versus-egglog branch (asked for explicitly)

egglog and oatlog implement the same language (a Datalog-plus-equality-saturation surface), but egglog is an
interpreter of that language and oatlog is an ahead-of-time compiler for it: oatlog embeds an equality-saturation
theory into a Rust application as a procedural macro, compiling the ruleset (as a relational database with
semi-naive evaluation) into specialised Rust at build time. The measured consequence is decisive for vehje's
regime: oatlog is over 100x faster than egglog for very small e-graphs, tapering to 2x at a million e-nodes, and
it is faster than egglog for everything up to about 100,000 e-nodes. Vehje's compile stage never wants a giant
e-graph; it wants a bounded, streaming, no-alloc frontier (the proposal's original-work item 1), which is exactly
the small-e-graph regime where oatlog's advantage is largest.

The deeper point is that oatlog-not-egglog is vehje's own thesis applied to its own compile stage. egglog ships
an interpreter of a rewrite language; oatlog compiles the rewrite language away into specialised code at build
time and ships no interpreter. That is precisely the certified-generation and runtime-generation move (1627,
1513, Deegen): the language definition is compiled into the composed runtime, not interpreted at the consumer. So
the lowering theory (the language's rewrite, const-fold, and macro rules) should be an oatlog-style
proc-macro-compiled saturation engine, generated into the composed runtime as one more projection of the one
source, rather than an egglog interpreter carried at runtime. Three concrete gains fold out: the AOT, specialised,
whole-ruleset-optimised engine over a bounded window is far more amenable to fixed-capacity, no-alloc allocation
than a general interpreter (it de-risks original-work item 1); oatlog is already a Rust proc macro, so it lives on
the Rust compile side where vehje's generation lives; and oatlog's relational-database backing (semi-naive
evaluation over a worst-case-optimal join, the AGM-bound cost model below) gives the compile-stage cost a
database-theory bound rather than a hand-rolled traversal budget. The one caution: oatlog is young (EGRAPHS 2025,
Chalmers) and egglog-compatible, so the sane path is to author the theory in the egglog language (portable) and
compile it with oatlog, keeping egglog as the reference semantics and prototype interpreter and oatlog as the
shipped engine. That mirrors the reference-interpreter-plus-compiled-artifact discipline vehje already uses.

## Per-choice upgrades

### The spine: ground the binding-time and assurance grades in provenance semirings

The proposal's novel move is assurance computed as a projection of the binding-time knowledge-source lattice.
That move is an instance of a settled, twenty-year-old algebra from databases: provenance semirings
(Green-Karvounarakis-Tannen, PODS 2007). A provenance semiring annotates each value with a semiring element
recording what it depends on, and different semiring homomorphisms recover different facts from one computation
(which inputs, how many derivations, trust level, cost). The knowledge-source lattice is a provenance semiring,
and "the same computation under a different semiring interpretation yields the static proof, the dynamic residual,
or the cost bound" is the semiring-homomorphism story, which is exactly what the proposal wants assurance-as-a-grade
to be. The fold: make the assurance grade a provenance-semiring annotation and the static-versus-dynamic choice a
choice of semiring, so the guarantee inventory is computed by a homomorphism rather than a bespoke lattice. This
also connects to the compile stage, since egglog and oatlog are relational engines and provenance semirings are
the provenance theory of relational queries, so the compile stage and the assurance algebra share a semiring
foundation. Grounding evidence that the graded dependent story is implementable: Idris 2 ships quantitative type
theory, and graded modal dependent type theory (Moon-Eades-Orchard, ESOP 2021) is the dependent extension.

### The one source: an oatlog-style relational theory, projected

Beyond the oatlog point above, the one-source shape is strongest as a relational or Datalog theory (the egglog
language), because then the grammar (an attribute grammar is a relational specification), the families and their
effects (relations), the rewrite and lowering rules (eqsat rules), and the projections (the Rust typestate, the
Zig consts, the interpreter, the stencils) are all views or materialisations of one relational theory. This is
the language-workbench dream (Spoofax, MPS) realised on a modern relational-plus-eqsat substrate. The fold: the
one declarative source is a relational theory in the egglog language; oatlog compiles it; the projections are its
materialisations.

### Stage 1 input: consider derivative-based or incremental parsing over vanilla scannerless PEG

Scannerless PEG is a fine v1, but two recent shapes fit a scripting host better. Derivative-based parsing
(Might-Darais-Spiewak 2011, and "Parsing with Zippers," Darragh-Adams, ICFP 2020) is compositional and naturally
incremental, and the zipper formulation is efficient. Tree-sitter's incremental GLR is the production-proven
incremental reparser (reparse only the changed region), which is what a REPL or hot-reloaded scripting consumer
wants. PEG also has a real hazard the graded-IR emission would rather avoid: ordered choice silently masks
grammar ambiguity, where GLL, GLR, and derivative parsers surface it. The fold: keep "the parse emits a graded IR
in one pass as synthesised attributes," but consider a derivative or incremental-GLR engine so a script edit
reparses incrementally, which composes with the streaming and REPL story. This is a soft upgrade, not a
correction; scannerless PEG can ship first.

### Stage 2 lowering: slotted e-graphs (a correctness fix), colored e-graphs, RVSDG, and the extraction rim

The equality-saturation stage carries the most consequential upgrade, and one of them is a correctness fix rather
than an optimisation.

- **Slotted e-graphs are required, not optional.** Vanilla e-graphs (egg, egglog, oatlog as-is) represent terms
  up to structural equality, but a value language with binders needs alpha-equivalence, and plain names or de
  Bruijn indices both reduce sharing and force renaming that can blow up the e-graph. Slotted e-graphs
  (Schneider-Steuwer, EGRAPHS 2024; "Slotted E-Graphs: First-Class Support for (Bound) Variables," PLDI 2025;
  "E-Graphs With Bindings," Tiurin, 2025) make bound variables a built-in by parameterising e-classes with slots,
  so renaming is just a different instantiation. Vehje's IR is a lambda calculus with binders (Let, Lambda,
  Match), so the lowering e-graph must be slotted or it mis-shares and explodes on exactly the binder-heavy code a
  real language produces. This is the single most load-bearing correction in this document.
- **Colored e-graphs for the graded, context-conditioned rewrites.** Colored e-graphs (Singher-Shachar, "Colored
  E-Graph: Equality Reasoning with Conditions," arXiv 2305.19203, EGRAPHS 2024) and relational contextual
  equality saturation (Hou et al., arXiv 2507.11897, 2025) support multiple layered equivalence relations, a base
  plus context-sensitive colors, memory-efficiently. The proposal's binding-time-and-effect-conditioned rewrites
  are exactly context-conditioned rewrites, so the graded conditions are colors and the graded extraction is
  contextual eqsat, which is a named, memory-efficient mechanism rather than duplicating the e-graph per context.
- **RVSDG as the eqsat IR.** A Regionalised Value-State Dependence Graph carries data, control, effect, and
  region dependencies as first-class nested-region structure, and running equality saturation over RVSDG (the
  RVSDG-plus-eqsat prototype; "E-Path: Equality Saturation for Control-Flow Graphs," arXiv 2605.28694, 2026;
  "Equality Saturation for Optimizing High-Level Julia IR," 2025) sidesteps the known difficulty of doing eqsat
  over arbitrary control-flow graphs. RVSDG's nested regions are the lease region nesting, so the residual IR,
  the effect ordering (which the proposal otherwise gets from A-normal form), and the lease regions can be one
  RVSDG rather than an A-normal-form term plus separate region tags. The fold: consider RVSDG as the residual IR
  and eqsat over it, with the regions doubling as lease regions.
- **The extraction rim is the legitimate ML entry, kept deterministic.** Optimal e-graph extraction is NP-hard,
  and the frontier is a differentiable or learned cost model: SmoothE (differentiable e-graph extraction, ASPLOS
  2025) and e-boost (heuristic warm-start plus exact solve, ICCAD 2025), with LLM-guided schedule synthesis for
  eqsat (arXiv 2604.17364, 2026) as the offline strategy-search frontier. This is exactly the optimisation-policy
  rim the NN/ML fork earlier identified as the one legitimate ML entry: a fixed, learned cost model compiled to a
  deterministic arvo fixed-point evaluation preserves determinism (the learning is offline and frozen), and it is
  bench-gated behind a swappable analytic heuristic. So extraction can be learned without breaking the
  deterministic-runtime identity, provided the model is frozen and evaluated in fixed-point.
- **Cost is governed by worst-case-optimal join.** egglog and oatlog back their matching with a worst-case-optimal
  join whose cost is the AGM bound of the query ("Parameterized Complexity of Running an E-Graph," UW PLSE 2025).
  This is the database-theory bound for the compile-stage cost, and it is the principled replacement for a
  hand-rolled traversal budget; it also tells you which rule shapes are cheap and which are cardinality bombs.

### Stage 3 lease: ground on Scala 3 capture checking, a production reachability discipline

The proposal grounds the lease on reachability types (research-only, per 05's caution). The stronger grounding is
that a production compiler already ships a reachability discipline: Scala 3 capture checking records, in a value's
type, the capture set of capabilities it can reach, with an empty capture set meaning pure and retaining nothing,
which is nearly verbatim the reachability-qualifier-as-AccessSet-bitmask idea. The theory is now well-formalised:
System Capless (existential and universal quantification of capture sets) and System Reacap (the surface language
with reach capabilities, translated into Capless), with reach capabilities `x*` added by Xu and Odersky (2024) to
handle mutable variables, and a production re-implementation applied to Scala's async library and standard
collections. Critically, capture checking (Odersky, EPFL) and reachability types (Rompf, Purdue) are converging:
"Ergonomic and Expressive Capture Tracking over Generic Data Structures" (OOPSLA 2025) is co-authored by Bračevac,
a reachability-types author. So the lease axis should track both as one lineage, ground its production-facing
claims on capture checking (which ships), and take reachability types as the sharper aliasing-and-separation
variant for the mutable-Lua case. The avoidance boundary maps onto capture checking's boxing: a boxed capture is
exactly a captured capability whose reach is not expressible in the current context, which is the avoidance
moment, and box/unbox is the discipline for it. This materially de-risks point 5: the mutable-consumer story is no
longer betting solely on unshipped inference, because a production system's capture-set discipline is the floor.

### Stage 4 runtime generation: weval as a lighter, theme-consistent alternative to Deegen

Deegen generates an interpreter and a JIT from a semantic definition via LLVM at build time. There is a lighter,
just-shipped alternative that is more consistent with the proposal's own partial-evaluation theme: weval (Fallin,
"Partial Evaluation, Whole-Program Compilation," PLDI 2025), which partially evaluates a Wasm interpreter into a
compiler with no runtime code generation, using the interpreter program counter as the context in a
context-sensitive constant-propagation, and which ships in StarlingMonkey and SpiderMonkey to provide ahead-of-time
compilation on Wasm-first platforms where runtime codegen is forbidden. That is vehje's exact constraint
(no toolchain at the embed site) and vehje's exact move (the first Futamura projection), shipped. So runtime
generation has three AOT, no-runtime-codegen options to weigh: Deegen (LLVM, interpreter-plus-JIT from semantics),
weval (partial-evaluate the interpreter, no LLVM), and copy-and-patch (pre-compiled stencils). weval is the
lightest and the most consistent with "partial evaluation everywhere" (it makes stage 4 and stage 2 the same
idea at two scales), and it removes the LLVM build dependency Deegen carries; copy-and-patch remains the one that
gives per-op native stencils for the tiered JIT. The fold: treat these as a costed fork resolved by the pivotal
experiment, and note weval as the option that unifies runtime generation with the compile-stage partial
evaluation rather than standing apart from it.

### Stage 5 load verifier: incrementalise it across scripts

The eBPF-and-tnum load verifier is the right model; the upgrade is to incrementalise it. A runtime loads many
scripts that share structure, and incremental or differential abstract interpretation reuses analysis results
across near-identical inputs, so the verifier need not re-prove a shared prelude every load. The relational
framing (egglog, oatlog) applies to the verifier's matching as well, so the load verifier and the compile-stage
engine could share a relational substrate. The fold: cache and reuse verifier state keyed by content hash of
shared script fragments, bounded by the same depth cap.

### Stage 6 sink: an algebraic-effect handler, not an iteratee

The reserve-commit sink is formalised as an iteratee (2012). The modern, production formalisation of a one-shot,
bounded, pull-based consumer is an algebraic-effect handler with a one-shot continuation, as shipped in OCaml 5's
effect handlers (2022). One-shot effects are literally the one-shot continuations that the proposal's streaming
lemma already rests on, so modelling the sink as a handler of the language's own effect signature unifies the
value-transfer sink with the algebraic-effects door (stage 0) and gives the one-shot-continuation theory a
production home. The fold: the streaming sink is a handler instance, closing the loop with the effect signature
the whole language is defined over.

### Memory model: compose reachability-proven uniqueness with Perceus reuse

The region-tag, no-collector model is right, and it composes with a production result the proposal does not name:
Perceus (Reinking-Xie-de Moura-Leijen, PLDI 2021, shipped in Koka), precise reference counting with reuse
analysis that guarantees in-place updates, enabling the functional-but-in-place paradigm where a purely functional
program mutates in place when a value is provably unique. Reachability is exactly the analysis that proves
uniqueness (a value reachable from only one binder is unique), so reachability-proven uniqueness drives Perceus-style
in-place reuse in the value-arena without breaking the immutable-value model: the same reachability information
that places the region tag also licenses the in-place update. The fold: where reachability proves uniqueness, the
runtime reuses the slot in place (FBIP); this is one more use of the lease information already computed, and it is
grounded in a shipping language rather than invented.

### The depth cap: generalise to automatic amortized resource analysis potentials

The depth cap doing six jobs is elegant but blunt (a single worst-case constant). Automatic amortized resource
analysis (Hofmann-Jost 2003; polynomial potentials 2010; exponential 2020; "Automatic Linear Resource Bound
Analysis for Rust via Prophecy Potentials," 2025; "Potential Functions as Types," arXiv 2607.08547, 2026) infers
compositional, non-asymptotic resource bounds by attaching a potential function to each type and solving linear
constraints. Two things make it a fit rather than a flourish: AARA's "potential as a type annotation" is a
graded or coeffect annotation, so it folds directly into the graded spine (the resource grade is a potential
function), and the recent Rust-targeted AARA shows the technique reaching a systems language. The fold: the depth
cap is the degenerate constant-potential case; a proper AARA potential bounds total work, frontier size, and
memory tighter than the worst-case cap, and it is computed by the same linear-constraint discipline the family and
effect inclusion already use. This also supplies the tighter-than-worst-case frontier bound the proposal wanted
from the speculative parabolic-stability idea, by a proven, implementable route.

## The cross-domain lessons, consolidated

- **Databases.** Provenance semirings are the assurance-and-binding-time grade algebra; worst-case-optimal join
  and the AGM bound are the compile-stage cost model (and oatlog and egglog already run on them); the whole
  compile pipeline is a relational-plus-eqsat query engine, which is a mature, benchmarked substrate rather than a
  bespoke pass pipeline.
- **Precise reference counting.** Perceus and functional-but-in-place turn reachability-proven uniqueness into
  in-place reuse, a shipping technique that composes with the lease axis for free.
- **Resource analysis.** AARA generalises the depth cap to inferred potential-function bounds that are themselves
  graded annotations, folding into the spine.
- **NN/ML, kept in its lane.** The only sound ML entry is the e-graph extraction cost model (SmoothE, e-boost,
  LLM-guided schedule synthesis), and only as a frozen, offline-learned model evaluated in deterministic arvo
  fixed-point and bench-gated behind an analytic heuristic. ML never touches the correctness path (grades,
  verifier, lease), consistent with the earlier NN/ML fork's conclusion.
- **Nonlinear dynamics, honestly scoped.** Analog, neuromorphic, reservoir, and thermodynamic computing do not
  fit a deterministic no-alloc runtime, and that is stated so the door is not left ajar. The one real thread is
  that every heavy analysis here (lease inference, equality saturation, the load verifier) is a monotone Kleene
  iteration to a least fixpoint, that is, a discrete dissipative flow, so nonlinear-dynamical acceleration applies:
  widening as limit-prediction, Aitken and vector extrapolation ("Convergence acceleration as a dynamical system";
  "Abstract Fixpoint Computations with Numerical Acceleration Methods," arXiv 1006.3159), and recent fixed-time
  convergent nonlinear flows ("Breaking the Convergence Barrier: Optimization via Fixed-Time Convergent Flows,"
  arXiv 2112.01363), which can accelerate convergence and, more valuably, yield provable convergence-rate bounds
  rather than mere termination. This is the disciplined form of the 08b parabolic-stability idea: the fixpoints
  are dissipative, and stability theory bounds their rate. It is an optimisation-and-bounds thread off the
  correctness path, not a hardware or ML bet.

## The upgrades ranked by leverage

Load-bearing (a correction or a production-grounding that changes the design): slotted e-graphs for stage 2 (a
correctness fix for binders); oatlog-not-egglog (the compile stage is vehje's own AOT thesis, and the no-alloc
bounded e-graph is far more tractable AOT-specialised); Scala 3 capture checking as the production floor under the
lease axis (retires much of the research-only risk). High-value folds (compose a shipping technique for free):
Perceus reuse from reachability-proven uniqueness; AARA potentials generalising the depth cap; provenance
semirings as the assurance algebra. Theme-unifying options (make two parts one idea): weval for runtime generation
(unifies with compile-stage partial evaluation); the algebraic-effect-handler sink (unifies with the effect
signature); RVSDG as the eqsat IR (unifies the residual IR, effect ordering, and lease regions). Softer or
scoped: colored e-graphs for context-conditioned rewrites; incremental abstract interpretation for the verifier;
derivative or incremental-GLR parsing; the deterministic ML extraction rim; the nonlinear-dynamical fixpoint-rate
thread. None reopens the five converged calls or the reachability decision; each sharpens how a stage is built.

## Sources

E-graphs and binders: Schneider, Steuwer, Slotted E-Graphs, EGRAPHS 2024
(https://steuwer.info/files/publications/2024/EGRAPHS-2024.pdf) and PLDI 2025
(https://steuwer.info/files/publications/2025/PLDI-Slotted-E-Graphs.pdf); Tiurin et al., E-Graphs With Bindings,
2025 (https://arxiv.org/pdf/2505.00807). Colored and contextual eqsat: Singher, Shachar, Colored E-Graph, arXiv
2305.19203 (https://arxiv.org/abs/2305.19203); Hou et al., Towards Relational Contextual Equality Saturation,
arXiv 2507.11897 (https://arxiv.org/pdf/2507.11897). oatlog: Gustafsson, Magnusson, Luque Cerpa, Oatlog, EGRAPHS
2025 (https://pldi25.sigplan.org/details/egraphs-2025-papers/8/Oatlog-A-performant-ahead-of-time-compiled-e-graph-engine ;
https://github.com/oatlog/oatlog). egglog and WCOJ: Parameterized Complexity of Running an E-Graph, UW PLSE 2025
(https://uwplse.org/2025/06/16/egraph-complexity.html). RVSDG and eqsat: E-Path: Equality Saturation for
Control-Flow Graphs, arXiv 2605.28694 (https://arxiv.org/pdf/2605.28694); Equality Saturation for Optimizing
High-Level Julia IR, 2025 (https://arxiv.org/html/2502.17075v1). ML extraction: SmoothE, ASPLOS 2025
(https://www.csl.cornell.edu/~yc2632/data/smoothe_asplos2025_final.pdf); e-boost, ICCAD 2025
(https://arxiv.org/pdf/2508.13020); LLM-Guided Strategy Synthesis for Scalable Equality Saturation, arXiv
2604.17364 (https://arxiv.org/pdf/2604.17364).

Capture checking and reachability: Scala 3 Capture Checking
(https://docs.scala-lang.org/scala3/reference/experimental/cc.html); What's in the Box: Ergonomic and Expressive
Capture Tracking over Generic Data Structures, OOPSLA 2025 (https://arxiv.org/pdf/2509.07609 ;
https://bracevac.org/assets/pdf/oopsla25full.pdf); Formalizing Box Inference for Capture Calculus, arXiv
2306.06496 (https://arxiv.org/pdf/2306.06496).

Runtime generation: Fallin, Partial Evaluation, Whole-Program Compilation (weval), PLDI 2025
(https://cfallin.org/pubs/pldi2025_weval.pdf ; https://cfallin.org/blog/2024/08/28/weval/ ;
https://github.com/bytecodealliance/weval).

Memory and reuse: Reinking, Xie, de Moura, Leijen, Perceus: Garbage Free Reference Counting with Reuse, PLDI 2021
(https://xnning.github.io/papers/perceus.pdf ; https://koka-lang.github.io/koka/doc/book.html).

Resource analysis: Hofmann, Jost, and successors; Exponential AARA
(https://link.springer.com/chapter/10.1007/978-3-030-45231-5_19); Automatic Linear Resource Bound Analysis for
Rust via Prophecy Potentials, 2025 (https://arxiv.org/pdf/2502.19810); Potential Functions as Types, arXiv
2607.08547 (https://arxiv.org/pdf/2607.08547).

Provenance semirings: Green, Karvounarakis, Tannen, Provenance Semirings, PODS 2007. Graded dependent types:
Moon, Eades, Orchard, Graded Modal Dependent Type Theory, ESOP 2021; Idris 2 (quantitative type theory).

Parsing: Darragh, Adams, Parsing with Zippers, ICFP 2020; tree-sitter incremental GLR.

Effect handlers: OCaml 5 effect handlers (one-shot continuations), 2022.

Nonlinear-dynamical fixpoint acceleration: Abstract Fixpoint Computations with Numerical Acceleration Methods,
arXiv 1006.3159 (https://arxiv.org/pdf/1006.3159); Convergence acceleration as a dynamical system
(https://www.sciencedirect.com/science/article/abs/pii/0168927494000204); Breaking the Convergence Barrier:
Optimization via Fixed-Time Convergent Flows, arXiv 2112.01363 (https://arxiv.org/abs/2112.01363).

## Amendment (2026-07-20): evaluating the adversarial attack and its meta-attack

Two later forks stress-tested the same proposal from other angles: an adversarial attack
(`202607202038_adversarial-attack-on-the-full-pipeline-proposal.md`, a math-and-instruction-set veteran that
tries to break each mechanism and compose a replacement where it lands) and a meta-attack of that attack
(`202607202044_meta-attack-on-the-adversarial-audit.md`, a novelty-driven theorist that audits the attack's own
constructions). This amendment evaluates both against the cross-domain bank above, defends what deserves it,
names where each of the three (the original proposal, the attack, the meta-attack) is wrong, and, the reason it
belongs on this document specifically, shows that nearly every better construction the meta-attack reached for is
already realised in a shipping or verified-frontier system I banked, which both de-risks those constructions and
collapses several of them onto one substrate. Two places even the meta-attack overreaches, and one deeper
unification that ties the pieces all three left disjoint, close the amendment.

### The verdict in one paragraph

The attack is correct on its concrete mechanism failures (the packrat memo is not depth-bounded; tnum is
non-relational and the load cost is program-size; shared-implies-promoted is the ML Kit leak; "partial evaluation
equals extraction" is a false label) and the meta-attack is correct that the attack's fixes retreat to the
established shape and three of them reintroduce the problem they escaped. What neither did, because both argued on
paper abstraction, is notice that the meta-attack's superior constructions are not aspirations: capture-and-separation
checking, slotted and colored e-graphs, oatlog's semi-naive relational engine, simdjson's parse-don't-validate
decode, Perceus precise reference counting, and automatic amortized resource analysis are shipping or verified
systems that each realise one of the meta-attack's fixes. So the adjudication is not "whose paper argument
wins"; it is that the meta-attack picked the right shapes and the systems literature already built most of them.
The original's structural instincts hold better than the attack credited, for the same reason: the unifications it
named badly are the substrates those systems run on.

### Where the bank resolves the attack-versus-meta disputes

- **A1, the effect-coeffect interaction, resolves in production, not in category theory.** The attack demanded a
  Beck distributive law, found none in general, and ceded the mutable fragment to a dynamic residual. The
  meta-attack correctly reframed it as graded def-use (a read then a write is sequential composition, not
  distribution) and named graded state or a graded Freyd category. The bank closes it: that graded def-use
  discipline ships as Scala 3's capture checking plus separation checking, where a value's capture set is the
  read (its reachability coeffect) and separation is the disjointness/aliasing judgment, and reach capabilities
  `x*` (Xu-Odersky 2024) plus System Reacap are exactly retention-through-a-boundary tracked as a graded
  capability. So the meta-attack is right that no distributive law is owed, and the shipping instance of its
  "graded state" resolution is capture-plus-separation checking. The residual boundary the meta-attack still
  admits (imprecise cyclic def-use) is precisely the reachability-avoidance boundary the converged topic already
  carries, so the two accounts are one.
- **A3, the compile stage, resolves with slotted and colored e-graphs plus oatlog.** Attack and meta-attack
  agree on the destination (binding time prunes the graph as a typing constraint so ill-staged terms are
  unrepresentable, honouring 1627; equality saturation runs over the confluent optimisation fragment; partial
  evaluation is a well-founded graded unfold that terminates by grade, not by cap), and the meta-attack gestured
  at "2024-2025 e-graphs with binders" without specifics. The bank supplies them and verifies them: slotted
  e-graphs (EGRAPHS 2024, PLDI 2025) are the binder-and-alpha-equivalence fix that classical eqsat lacked,
  colored e-graphs (arXiv 2305.19203) are the context-conditioned rewrites the graded conditions need, and oatlog
  (EGRAPHS 2025) is the ahead-of-time-compiled, semi-naive relational engine that makes the bounded no-alloc
  version tractable. The meta-attack's "determinism from confluence on the static fragment, not from saturation
  order" is then real, because a confluent, terminating rule set over a bounded input has an order-independent
  normal form, and oatlog's whole-ruleset AOT compilation is where that confluence is enforced and specialised.
- **A6, the load verifier, resolves with parse-don't-validate and simdjson.** The attack was right that tnum is
  non-relational and cost is program-size, and its relational-domain fix stays inside incomplete abstract
  interpretation. The meta-attack's parse-don't-validate refinement-typed decode is the right move, and the bank
  shows it is not hypothetical: simdjson is a complete, linear-time structural decoder of untrusted flat data
  that either produces a valid structure or fails, which is exactly the typed decode the meta-attack described,
  and it is the reason the amendment earlier flagged the arena-SIMD-format decision as time-sensitive. tnum is
  retained for the numeric residual only. Types and SIMD are the same mechanism from two angles.
- **A7, sharing, resolves with Perceus.** This is the attack's sharpest catch (shared-implies-promoted is the ML
  Kit leak) and its own fix (copy every leased shared value) is the meta-attack's sharpest counter (exponential
  blowup on the huge nested values the transfer model exists to carry). The meta-attack's construction, an exact
  meet of referrers' reach-sets by a reversed referrer count on shared roots finalised at emission, "reference
  counting as a compile-time and emission-time finalisation," is Perceus (Koka, PLDI 2021) verbatim: precise,
  garbage-free reference counting computed at compile time with drop finalisation. So the meta-attack's fix is a
  shipping algorithm, and it composes with the reuse point earlier in this document (reachability-proven
  uniqueness driving in-place update is the same Perceus machinery). Perceus is the exact-meet-without-a-collector
  the reachability paradigm promised and both original and attack walked away from.
- **A8, the streaming substrate, resolves with the relational engine already banked.** The attack correctly
  struck "one one-shot coroutine" (eqsat is multi-shot and re-entrant), and the meta-attack recovered a real
  unification (semi-naive, differential-dataflow fixpoint over a monotone lattice with a bounded working set,
  Abadi-McSherry-Murray). The bank shows this is not a fresh construction to build: oatlog and egglog are
  relational engines with semi-naive evaluation over a worst-case-optimal join, so the lease inference, the
  equality saturation, and the load verifier (three monotone fixpoints) are queries on one relational-fixpoint
  substrate whose cost is the AGM bound. And the nonlinear-dynamical fixpoint acceleration this document banked
  applies uniformly to all three (they are Kleene iterations, that is dissipative flows), so widening and
  extrapolation accelerate the shared substrate, tying two of the bank's finds to the meta-attack's A8.
- **A9, the resource bounds, resolves with AARA.** The meta-attack's correction (not one depth cap doing six
  jobs but three genuine bounds, environment-width, nesting-depth, and input-length, each doing two) is right,
  and AARA (automatic amortized resource analysis, potential functions as types) is the theory that makes those
  three bounds inferred and composable rather than hand-counted, and a potential is itself a graded annotation, so
  the three bounds fold into the graded spine as inferred potentials rather than named constants.
- **A4, the parser, resolves with derivatives plus the inclusion axis.** The meta-attack's fix (grammar
  composition is inclusion-not-coverage with declared-and-detected conflicts, mechanised by Brzozowski
  derivatives over a family lattice, since derivatives compose) is exactly the derivative-based parsing this
  document flagged (parsing with zippers), applied through vehje's own inclusion axis. Neither PEG's silent
  shadowing nor GLL's silent ambiguity; a cross-family syntax clash is a declared error, the same discipline as
  `Supports` and `Permits` everywhere else.
- **C5, the prize, resolves onto the spine grounding.** Both forks agree the graded step-indexed logical relation
  (Ahmed, Iris, RustBelt) is the real unification, and the meta-attack sharpened it (index it by the assurance
  grade so proof and differential test are one object at different grades, parametric over the generated
  signature as the fourth functor). The bank connects: the assurance grade that indexes the relation is a
  provenance-semiring annotation, so the assurance-indexed gradual logical relation is a semiring-graded logical
  relation, which is what ties C5 to the spine grounding this document proposed.

### Where even the meta-attack overreaches

- **A8: the value stream is not a fixpoint, and the attack was half-right to isolate it.** The meta-attack folds
  value emission into the differential-dataflow unification as "the degenerate one-shot fixpoint." That
  over-unifies. Differential dataflow is incremental maintenance of a monotone computation under changing inputs;
  the lease inference, the equality saturation, and the load verifier genuinely are that (monotone accumulation
  to a least fixpoint over a lattice). Value emission is a linear streaming fold, not a monotone accumulation to
  a fixpoint, so it shares the bounded-frontier discipline (the one-shot linear-continuation property the attack
  correctly isolated) but not the fixpoint substrate. The clean statement separates the two axes the two forks
  each got half of: the bounded-frontier one-shot discipline covers all four stages, the monotone-fixpoint
  relational substrate covers the three analyses, and value emission is where they coincide only on the frontier
  axis. Neither fork stated it cleanly.
- **A6: typed decode does not subsume the lease residual.** The meta-attack's parse-don't-validate decode
  completely and linearly settles the structural safety of the arena (index-in-range, acyclicity,
  depth-boundedness) and tnum settles the numeric residual, but it implies these two cover the load check, and
  they do not. The lease and region validity of an arriving value (the reachability residual, whether a reference
  outlives its region) is not a structural typing property of the wire format nor a numeric one; it is the
  gradual dynamic residual of the lease coeffect, discharged by the per-reference generational check at the
  avoidance boundary. So the arriving-value load check is three mechanisms, not the two the meta-attack lumps:
  typed structural decode (simdjson), numeric abstract interpretation (tnum), and the lease residual
  (generational check). Folding the third into the first is the meta-attack's one blind spot.

### The deeper unification the three left disjoint

Reading the two forks against the bank surfaces a unification larger than any of the four "one X does N" claims
the attack demolished, and it is grounded rather than asserted. The compile-and-verify side of vehje is one
relational-fixpoint engine with a semiring-graded provenance annotation, and the three analyses (lease inference,
equality-saturation lowering, load verification) are queries on it. The evidence is concrete and cross-domain:
oatlog and egglog make the engine a relational database with semi-naive evaluation and a worst-case-optimal join
(so the substrate and its cost model are the database frontier, not a bespoke pass pipeline); provenance
semirings are the provenance theory of exactly that relational substrate (so the assurance-and-binding-time grade
is a provenance annotation on the same engine); capture-and-separation checking is itself a dataflow analysis
expressible as such a query (so the lease axis lives on the same engine); and the graded logical relation of C5,
indexed by that provenance grade, is the proof layer over it. The stack, top to bottom: a semiring-graded logical
relation (the proof), over a graded (co)modal judgment whose grades are provenance-semiring elements (the spine),
computed by a semi-naive relational-fixpoint engine (the substrate, oatlog-shaped), accelerated by
nonlinear-dynamical fixpoint methods (the speed), with slotted-and-colored e-graphs as the binder-and-context-aware
instance of the engine and Perceus and AARA as the emission-time finalisation and the resource accounting. That
is the shape all three forks were circling: the original named it as four coincidental synergies, the attack
struck three of them as puns, and the meta-attack recovered the substrate (differential dataflow) and the proof
(graded logical relation) separately without joining them through the provenance semiring that makes them one
engine and one grade.

### What survives across all three

The consensus core, held by the original and unbroken by attack or meta-attack: grading as the organising idea
(scoped); reachability-as-coeffect-grade realised as a fixed-capacity AccessSet bitmask, with capture-and-separation
checking as its production grounding; the degenerate depth-lease as the proven floor; copy-and-patch as the native
tier; effect-constrained equality saturation over the confluent fragment, on slotted-and-colored e-graphs; and the
five converged directions and the reachability decision, which nothing here reopens. The three sharpest results to
carry forward: Perceus is the exact-meet answer to the A7 leak (not promote, not copy); the relational-fixpoint
engine with a provenance-semiring grade is the substrate under lease, eqsat, and load-verify at once; and the
assurance-indexed gradual logical relation, parametric over the generated signature, is the spine, with the
differential test as its grade-0 shadow.

### Amendment sources

New to this amendment (the rest are in the sources above). Scala 3 separation checking alongside capture
checking: Introduction to Scala 3's Capture Checking and Separation Checking
(https://tanishiking.github.io/posts/introduction-to-scala-3s-capture-checking-and-separation-checking/), with
System Reacap and reach capabilities from the capture-checking sources above. Differential dataflow: Abadi,
McSherry, Murray, Isard, Differential Dataflow, CIDR 2013
(https://www.microsoft.com/en-us/research/publication/differential-dataflow/). Provenance semirings, oatlog and
the AGM bound, Perceus, AARA, slotted and colored e-graphs, simdjson, derivative parsing, and the
nonlinear-dynamical acceleration references are all cited in the sources above.
