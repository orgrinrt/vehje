# Carmack Reaction: the Grounded Stack, Reduced to the Machine

**Date:** 2026-07-20
**Status:** worker-fork evaluation, one shot, settles nothing. First-principles soundness and buildability
review of the converged architecture (`mock/design_rounds/202607202001_topic.converged-architecture-shape.md`)
and the grounded stack (`mock/design_rounds/202607202055_topic.attack-meta-attack-adjudication-and-the-grounded-stack.md`),
against the framework constraints in `.claude/CLAUDE.md` (no_std + no_alloc framework crates, no compiler at
the embed site, small Zig runtime over a C ABI, single dev-time static binary, arvo and hilavitkutin as
foundations). The debate trail read in full: `mock/research/202607202038_adversarial-attack-on-the-full-pipeline-proposal.md`,
`..._202607202044_meta-attack-on-the-adversarial-audit.md`, `..._202607202042_better-approaches-bleeding-edge-and-cross-domain.md`;
grounding topics 1315, 1316, 1513, 1845 read for the concrete shapes the stack sits on.

## Verdict

The architecture is sound where it is boring and machine-shaped, and those pieces alone are a complete,
buildable v1: the three-layer artifact model, the flat value-arena with the reserve/commit sink, region tags
on stack discipline, the lease axis as bitmask region inference fused into the resolve walk with a
generational residual exactly where inference fails, the typed structural decode at load, and tiered
execution with the interpreter as the floor. The debate genuinely earned its keep by killing four real
mechanism errors (shared-implies-promoted, PE-as-extraction, tnum-for-structure, the depth-cap pun). The
remaining risk concentrates in one place: the unification layer. The single relational-fixpoint engine, the
equality-saturation compile stage as v1 machinery, the assurance-indexed logical relation treated as the
design's spine, AARA-inferred potentials, and the dynamical fixpoint acceleration are, in that order,
increasingly academic structure that will not survive contact with the repo's own no_alloc lint gates and the
dual-locus reality that every per-script analysis must run inside the composed Zig runtime, not just the Rust
dev-time compiler. Each has a simpler shape that does the same job today. This is a foundation to build on
after one short re-tiering pass, not a redesign.

## What is sound and worth building

The strongest property of the converged shape is that its load-bearing floor reduces to primitive machine
operations with no interpretation gap. Taking the pieces the question names, plus the floor they stand on:

**The three-layer artifact model and the three named codegens** (1513). Language compiler at dev time,
hand-written general engine, composed runtime shipped. Retiring "native" as a tier and making it one output
target removed a phantom that was distorting everything downstream. This is correct and it is the frame that
makes "how does an arriving script reach native" a well-posed question at all. No notes.

**The value-arena and the reserve/commit sink** (1315). Fixed-width records, a flat child-index pool, a byte
blob, relative indices, depth-first children-first emission so parents never back-patch child links,
host-lent memory, chunking at whole-subtree boundaries. This is the Cap'n Proto and rkyv lineage done with
the one simplification those systems never get (no relocation, no schema evolution, one producer one reader).
It is the single best piece of machine design in the arc. The sink as two function pointers plus userdata is
exactly right for the C ABI.

**The lease axis, graded** (1316, converged point 5). The reduction chain is real at every step: the
degenerate LIFO depth-lease is a depth counter and a min-propagation over one bit per operand; the full
reachability qualifier is a fixed-capacity bitmask over live binders with inclusion as AND-compare and join
as OR, the same AccessSet machinery the family and effect axes already run on; the avoidance boundary demotes
exactly the un-placeable reference to a per-reference generation check (one u32 compare at deref). Static
where provable, one cheap dynamic check where not, nothing global, no collector. Ship the degenerate floor
first, grow the lattice. This is the design's best idea and it costs what it claims to cost.

**Perceus-style emission-time finalisation for shared nodes** (2055, the A7 resolution). The exact meet by
referrer count-down on shared roots is the right answer between the leak (promote) and the blowup (copy). One
mechanism note below on how it lands in the arena format, but the shape is correct and bounded.

**The typed structural decode at load** (2055, the A6 resolution). Parse-don't-validate over a flat arena
whose indices are range-typed is complete, linear, branch-predictable, and SIMD-friendly. This displaced an
abstract interpretation that was wrong on both domain and cost bound. The consequence the docs correctly
flag: the arena layout must be alignment-and-stride predictable, decided now. I endorse deciding it now; a
format decision is cheap today and a migration later.

**Three named structural bounds** (2055, the A9 resolution). Environment-width, nesting-depth, input-length
as first-class constants each doing real work. Honest, enforceable, sized at compile time. This is what the
no_alloc story actually rests on, and it is better than the retired one-cap pun precisely because each bound
can be sized to its own job.

**Copy-and-patch as a gated tier** (converged point 4), with the direction settled and the commitment gated
on the semantic-definition coverage experiment. The mechanism is proven (CPython 3.13, Deegen). Held as a
per-platform optional tier over an always-present interpreter floor, it is right. Held as "the keystone," it
is over-weighted; see the machine-level section.

**The two gating experiments and their sequencing.** Coverage of the semantic definition over the census
families, and the avoidance rate on real Lua-shaped code. Both are exactly the experiments that de-risk the
two real bets, both are cheap relative to what they gate, and running them before the bridge commitment is
the correct order. The avoidance-rate experiment should be widened by one measurement; see below.

**Differential testing generated from the one semantic definition.** Interpreter arm, stencil, and test from
one source is Deegen-proven and it is the assurance mechanism that will actually exist on day one. Whatever
happens to the logical-relation ambition, this ships.

## What breaks at the machine level

Each failure named, concrete, with the constraint it breaks against.

**1. The relational-fixpoint engine collides with the repo's own no_alloc gates and the dual-locus reality.**
The grounded stack's centerpiece is "lease inference, equality-saturation lowering, and load verification are
three queries on one oatlog-shaped semi-naive relational engine" (2055, grounded-stack section). Reduce it:
oatlog is a Rust proc macro that emits specialized Rust using heap collections. The vehje compile-side crates
are lint-gated no_std + no_alloc (`.claude/rules/lint-forbidden-vehje-ir.md` and siblings). And per 1513, the
per-script analyses run inside the composed runtime for arriving scripts, which is Zig plus generated code,
where a Rust proc macro does not reach at all. So "oatlog-shaped" cannot mean "use oatlog." It means "write
our own AOT relational-engine generator that emits no_alloc code into both loci." That is a full compiler
project standing under the compile stage, and the docs' own original-work item 3 admits the no-alloc
slotted-and-colored variant "remains genuine work" while still placing the engine at the base of the stack.
A base you have not built is not a base. Meanwhile the lease inference already has a conforming design that
needs none of it: hooks fused into the existing resolve walk, single pass, bitmask joins, bounded fixpoint
only across shared roots (1316). Rehosting a fused syntax-directed walk onto a relational engine makes the
common case slower and the implementation an order of magnitude larger, for conceptual unity.

**2. The grounded stack contradicts itself on load verification.** The A6 resolution correctly re-derives the
load check as a typed structural decode, "complete and linear, not an incomplete abstract interpretation"
(2055), and the two-overreaches section even splits it into three mechanisms. Yet the grounded-stack summary
still lists load verification as one of the three queries on the monotone relational-fixpoint engine. A
linear one-shot decode is not a fixpoint query; the document caught value emission over-unified into the
engine and missed that it had done the same thing to the load check two sections earlier. Strike load
verification from the engine's query list. What remains on any shared fixpoint machinery is at most lease
inference's shared-root residual and the optional eqsat stage, which is too little to justify an engine.

**3. Copy-and-patch is runtime code generation as far as the OS is concerned, and no document names it.** The
pitch is "all data, where some of the data is machine-code stencils" (2001, point 4). From the kernel's view,
the patcher writes bytes and then executes them: W^X applies, macOS requires MAP_JIT plus
pthread_jit_write_protect_np toggling, iOS forbids it without an entitlement effectively unavailable to
third parties, consoles forbid it outright, and hardened hosts (the exact embedders a sandboxed scripting
runtime courts) disable executable mapping wholesale. "No compiler at the embed site" is satisfied; "no JIT
permissions at the embed site" is not, and the second is the constraint that actually bites on the platforms
a game-embedded runtime targets. The consequence is not fatal because the design already has the right
structure (tiering with an interpreter floor), but the framing must invert: the interpreter is the product,
present and certified everywhere; copy-and-patch is a per-platform accelerator where the OS permits. It also
cannot be the keystone for a second reason: the measured wins for baseline copy-and-patch JITs on dynamic
languages are real but modest without inline caches and type feedback, and vehje's residual is already
statically checked flat IR, which shrinks the interpreter's overhead the JIT would remove. The per-frame
ceiling for ikiuni is far more likely to be decided by the interpreter's dispatch and value layout than by
the presence of a baseline JIT.

**4. The stencil toolchain constraint is real engineering, unnamed.** Stencil extraction requires compiling
each operation with a continuation-passing calling convention (the CPython route needs clang's
guaranteed-tail-call and preserve-none machinery), parsing the object files, and extracting relocation
records into patch tables. Zig ships clang, so the toolchain is present at our build, but this is weeks of
grubby object-format work with per-target ABI edge cases, and it lands in the trusted computing base. Budget
it as such in the coverage experiment, not as a byproduct.

**5. Shared-node region tags need one indirection or the no-back-patch property dies.** Emission is
depth-first, children before parents; a shared node is emitted before its later referrers, but under the
Perceus-style exact meet its region is only known when its last referrer is emitted. Writing the region tag
into the node record at emission is therefore impossible without rewriting already-emitted bytes, which the
transfer topic's no-back-patching property forbids and a committed streaming chunk makes physically
impossible. The clean machine answer: nodes carry a region id that indexes a region table, and the table
(small, bounded by live region count, an environment-width quantity) is finalised at chunk close. Nodes are
never touched twice; only the table is. The chunker contract (cross-chunk implies promoted root) already
bounds the deferral window. This is a footnote-sized fix but it must be in the wire format from day one.

**6. The fixed-capacity reachability bitmask has a cliff, and the overflow policy is unspecified.** The
qualifier is a bitmask over live binders, width fixed at compile time of the composed runtime. Real Lua-shaped
code has modules and closures capturing wide environments; a script whose live-binder set exceeds the width
hits the cap. What happens? Hard error under the strict posture, or demotion of the overflowing references to
the generational residual as if avoidance had failed? Either is defensible; silence is not, because the
answer changes the bitmask width you pick and the failure mode modders see. The avoidance-rate experiment
should measure live-binder-set width distribution on the census corpus in the same run; it is the same
instrumentation pass.

**7. The speed layer is garnish and should be struck.** "Nonlinear-dynamical fixpoint acceleration, widening
as limit-prediction, Aitken and vector extrapolation" (2055) over these lattices is a category mistake
dressed as a layer. The lattices in play are fixed-width bitmasks and small finite-height domains; a Kleene
iteration over a 128-bit mask converges in at most 128 joins and in practice a handful, and semi-naive
evaluation is already the correct and complete acceleration for the relational fragment. Numeric sequence
extrapolation has no meaning on a boolean lattice. It is admittedly filed off the correctness path, but a
design document that carries a layer with no machine content teaches the next reader the wrong thing. Delete
it; keep plain widening if a numeric domain (tnum) ever iterates.

## What is missing

**The interpreter's own hot-loop design.** The arc spends thousands of lines on the proof spine and the
compile stage and nearly none on the loop that runs every frame in the flagship consumer: dispatch shape
(Zig `@call(.always_tail)` threaded dispatch versus switch; the difference is 2x on interpreter-bound
workloads and Zig can express both), value-node operand packing (fixed-width records plus a child-index pool
means every operand access is one extra indirection; small-operand inlining into the node record is the
standard fix and it is a wire-format decision, so it interacts with the decide-now SIMD layout), and the
cache behavior of the arena walk. This is where vehje's actual performance identity will be decided, before
any JIT tier exists. It deserves a topic of the same weight the lease axis got.

**Platform capability policy for the native tier.** Which targets get stencils, what the runtime does when
executable mapping is denied at load time (graceful tier-down must be a tested path, not an assumption), and
whether the tier is a build-time or run-time choice per composed runtime. Falls straight out of break 3.

**Diagnostics as a product surface.** Strict-by-default with lease-inference failure as a compile error means
modders will hit lease errors as their first contact with the system. The bitmask tells you inclusion failed;
it does not tell you which binder, bound where, escaped through what. Spans and binder provenance must be
carried through the fused walk from the start; retrofitting provenance onto a bitmask analysis is miserable.
Nothing in the grounded stack mentions error reporting.

**The dual-locus authoring story for the analyses.** Family, effect, and lease checks run in Rust at dev time
for bundled scripts and inside the composed Zig runtime for arriving scripts (1513). Either the general
algorithms are written twice and differentially tested against each other, or written once in a form both
loci consume (generated, or a shared no_alloc core compiled into both). This is the same class of problem as
the point-3 handoff and it is solved the same way, but it is nowhere stated, and it is the strongest
practical argument for keeping every analysis simple enough to write twice.

**A Perceus attribution correction, because precision matters in a design that will be audited again.** The
adjudication calls the reversed-referrer-count finalisation "Perceus verbatim" (2055). Perceus is runtime
reference counting with compile-time-inserted dup/drop and reuse analysis; the design's variant counts only
at compile and emission time on shared roots and ships no runtime counts. It is Perceus-derived and better
fitted to the constraints than the original, which is worth saying, because "verbatim" invites a future
reader to import Perceus's runtime machinery wholesale.

## Over-reach a simpler shape replaces

**The equality-saturation compile stage, as v1 machinery.** The stage must do macro expansion, const-fold,
CSE, and lowering. The grounded correction (binding time prunes as a typing constraint; PE is a well-founded
graded unfold terminating by grade; determinism from confluence) is theoretically right, and none of it
requires an e-graph to implement. The well-founded graded unfold is a recursive specializer with memoization,
a few hundred lines. CSE on a flat arena is hash-consing, which the content-addressed format nearly gives
away for free. Macro expansion is directed rewriting driven by a table; macros are not equations. What the
e-graph buys over this is phase-ordering optimality across algebraic identities, a marginal win for a
front-end lowering to a checked residual, and it costs the no_alloc slotted-and-colored engine that item 1 of
the machine-level breaks shows does not exist. Replacement: fused walk plus hash-consed arena plus graded
specializer plus table-driven macro rewriting on the critical path; the bounded e-graph moves to a bench-gated
research track that must beat the simple pipeline on census workloads before it earns a slot. This also
retires most of re-scored original-work items 1 and 3 (2055) from "owed" to "optional."

**The single relational engine as the base.** Covered in breaks 1 and 2. Replacement: three small conforming
analyses that share the bitmask primitives and the bounds, not an engine. The unification survives as
vocabulary (they are all monotone and all bounded by the same three constants), which costs nothing.

**The assurance-indexed gradual logical relation as "the spine."** As a proof-document north star it is the
right target and genuinely the best idea the attack produced. As the design's spine it is mis-tiered:
step-indexed logical relations at this scale (regions, effects, staging, a generated signature) are an
Iris-scale, multi-person-year mechanization, and no implementation decision can be allowed to wait on it. The
1845 proof skeleton (lambda_veh, T1 staging soundness, T2 gradual guarantee, T3 region soundness, the crux
and cross-chunk lemmas) is the correctly-sized deliverable and it is already scoped as bounded work. Ship
that; hold the unified relation as the consolidation a later pass may fund. The differential-test harness is
the grade-0 shadow that actually exists on day one, which the meta-attack itself proves is the same object at
a lower grade, so nothing is lost by sequencing it first.

**AARA-inferred potentials.** Inferring resource bounds by LP solving inside a no_alloc dev-time compiler,
to replace three constants that are already correct and enforceable, is cost with no consumer. The three
named bounds stay constants. AARA is a fine later research note for tightening the frontier bound if a real
workload ever demands it; it has no place in the foundation.

**The provenance-semiring framing, held with a light grip.** As the observation that binding-time,
assurance, and the grade algebra share semiring structure, it is a good coherence argument for the design
document. The moment it starts dictating implementation (a semiring-annotated engine, homomorphism machinery
in code), it has over-reached: at the machine level the grades are two small enums and two bitmasks, and the
"homomorphism" is a match statement. The converged topic's own scoping rule (grades live in the proof, never
in the runtime types, 2001 point 2) is exactly right; the risk is only that the grounded stack's rhetoric
("the semiring is the joint") erodes that scoping over time. Hold the scoping.

**Brzozowski derivatives as the runtime parser.** Conflict detection between independently authored family
grammars is a dev-time job in the language compiler, where derivatives (or any decidable approximation) are
fine at any cost. The composed runtime then parses one fixed, already-proven-conflict-free grammar, for which
a generated table parser is the fast, boring, no_alloc answer. The adjudication's inclusion-not-coverage
discipline is correct; just keep the detection mechanism at dev time and out of the embed-site hot path.

## Bottom line

Build on it. The floor of this architecture (arena, sink, regions, fused bitmask analyses, typed decode,
tiered execution, the three bounds, the two experiments, differential tests from one definition) is sound,
fast, and conforming with the stated constraints today, and the debate demonstrably improved it by killing
four real errors. What the grounded stack got wrong is not any mechanism but a tiering: it promoted its
unification layer (one engine, one theorem, one semiring) from "coherence argument and eventual proof target"
to "the base," and under the repo's own no_alloc gates and the dual-locus runtime reality that layer is the
one part that cannot be built as described. The pass this needs is short and surgical, not another debate
round: re-tier the engine, the eqsat stage, the logical relation, AARA, and the acceleration layer off the
critical path with the simpler shapes named above in their places; fix the load-verification self-
contradiction; specify the bitmask overflow policy and the region-table indirection; name the W^X constraint
and the platform tier-down path; and give the interpreter hot loop the design attention the proof spine has
been getting. Then run the two experiments exactly as sequenced and start building the floor, which needs
none of the open questions answered.
