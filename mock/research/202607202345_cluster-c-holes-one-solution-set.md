# One solution set for the Cluster C holes

**Date:** 2026-07-20
**Phase:** research
**Scope:** the honest-keeper capstone (`202607202250_honest-keeper-capstone-and-diagnostics.md`) found a set of
real holes in Clusters A (`202607202210`) and B (`202607202230`). This document answers all of them as one
coherent solution set, on paper, in the spirit of the identity recenter (`202607202330`): the holes are where we
work, not reasons to drop anything, and the solutions serve the whole framework and its spectrum of consumers,
not only the native and per-frame corner.
**Answers:** the consolidated worklist from Cluster C: the `InScope` reachability-bound fix, the dual-locus
identity divergence, the interpreter control-flow model, the `preserve_none` path, the residual-versus-cheap-
lowering seam, the e-graph extractor, the diagnostics schema, and the bench-gated format decisions.

## Verdict

Every hole is answerable, and the answers reinforce each other rather than stacking. The load-bearing moves are
three. First, the reachability bound is restored by recognising that the escape event and the scope-kill are the
same moment, so the width-`W` in-scope bitmask cannot physically hold an escaped binder and the encoding itself
forces the fix. Second, the dual-locus divergence risk is not real in vehje's actual shape: the general engine is authored
once by hand in Zig and the language-specific parts are validated data statically linked and specialized into
it, so there is one engine rather than two, the per-script analysis runs in that single runtime locus, and the
identity Cluster C worried about already holds by construction. Third, the interpreter is restructured from a single linear
scan into a control-flow graph of straight-line blocks, so the cache-friendly linear scan survives for the
dominant straight-line case while branches, loops, and calls are handled correctly at block boundaries, with the
frame stack bounded by the depth cap the design already carries. The remaining holes (the dispatch convention,
the lowering seam, the extractor, the diagnostics schema, the format measurements) resolve cleanly once those
three are in place.

## 1. The N*W reachability bound: escape and scope-kill are one event

Cluster C is correct that A's `Reach` rules, as written, drop the `InScope` filter, so a node accumulates the
binders of its whole subtree and the closure is `O(N)` per node rather than `O(W)`. The fix is not to bolt
`InScope` onto the propagation rule verbatim, because that would silently discard the escapes the lease axis
exists to detect. The fix is to see that the moment a reached binder is no longer in scope at the parent is
exactly the escape event, and to route it there.

Concretely, the reachability of a node splits into two relations with different bounds:

- `Reach(n, b)`, the live in-scope reachability, stored as the width-`W` lattice-column bitmask over the binders
  in scope at `n`. Propagation keeps only still-in-scope binders: `Reach(p, b) :- Child(p, _, c), Reach(c, b),
  InScope(p, b)`. This is bounded by `W` per node by construction, because the bitmask has exactly `W` bits and
  an out-of-scope binder has no bit to occupy.
- `Escape(n, b)`, the terminal escape events, recorded precisely when a child reaches a binder that is not in
  scope at the parent: `Escape(p, b) :- Child(p, _, c), Reach(c, b), !InScope(p, b)`. An escape is resolved once,
  at the boundary where it happens, into either a proven wider-lease placement or the per-reference generational
  residual (the avoidance boundary). It does not re-accumulate into any node's live set.

So the width-`W` encoding is not just the storage, it is the enforcement: an escaped binder is unrepresentable in
the in-scope bitmask, which forces it to be recorded as an `Escape` and resolved, instead of silently swelling
the reachable set. `Escape` can be `O(number of escaping uses)` in the worst case, but each row is a terminal
event with a one-time resolution, not a growing closure, and in the common case (most values do not escape their
defining region) it is sparse. The `N*W` claim holds for the live reachability that drives the no_alloc frontier,
which is what the bound was ever about.

This also sharpens the lease semantics: a binder leaving scope at a block boundary (section 3) is the single
place `Escape` fires, so the fix and the control-flow model land at the same seam.

The sibling counter-audit sharpens this fix, and the closing commentary addendum folds in the correction: the
`InScope` filter alone is not the whole rule. Without a binder-substitution step at `Let` and `Lambda` it
under-approximates the reach set (a value reaching through a bound variable into a still-live outer binder loses
that reach), so the complete fix is the reachability-type binder rule (splice the bound expression's reach where
the body reaches the variable, then drop the variable) plus a region-promotion bound on the escape set, both
prior art. See the addendum.

## 2. Dual-locus identity: the divergence problem dissolves in vehje's actual shape

Cluster C audited a shape vehje does not have. Its concern, that "one generator, two backends" yields two
deterministic engines that can deterministically disagree, presumes the analysis engine is emitted twice, once
as a Rust module and once as Zig source, and that the same per-script analysis runs in a Rust dev-time locus and
a Zig runtime locus that must then be proven to agree. That premise is a drift from the canonical shape settled
in `202607201513` and `..._201534`, and correcting it removes the problem rather than solving it. The neutral
shared-kernel resolution an earlier draft of this section proposed is the wrong move for the same reason: it
invents a second artifact to reconcile where the canonical design already has exactly one engine.

Two facts fix the shape. First, per-script analysis runs in one locus, not two. The Rust side compiles the
LANGUAGE, never a script: it performs runtime generation once per language and proves the language's family,
effect, and lease disciplines sound. Every per-script act (parse, the analyzers, macro expansion, const-eval,
output generation) runs in the composed runtime at load time, for bundled and arriving scripts alike (1513's
vocabulary table is explicit that those are composed-runtime, run-time, per-script operations). There is no
second engine analyzing scripts in Rust, so there are not two script-analysis results to reconcile.

Second, the engine is authored once, by hand, in Zig. The general engine (the interpreter, the analyzers'
general algorithms, the relational evaluator of 2055, the value transport) is hand-written Zig, and the
language-specific parts the Rust side produces are validated DATA (the family table, the effect masks, the
lease-rule schema, the wire layout) statically linked and comptime-specialized into that one hand-authored
engine. This is the certified-generation resolution of `202607201627` and `..._201845` exactly as settled: Rust
emits proven-consistent data, not a Rust engine; the hand-authored Zig engine specializes to it; the Zig
compiler certifies the specialization. There is one engine, hand-written, so there is nothing for a second
backend to diverge from. The property Cluster C wanted, a single engine with identity by construction, is
already the case, which is why generating a separate neutral kernel to "unify two backends" is the wrong shape:
it manufactures the very duplication the canonical design avoids.

So the identity work is not "prove two engines agree." It is the handoff certification already designed as
point 3 of the converged shape (`202607202001`): the emitted data is a projection of the one declarative
language definition, rustc types the data emitter so a well-typed run can only emit consistent data, the Zig
compiler types the hand-authored engine specialized to that data, and a content-addressed manifest checks the
data projection against the canonical definition at comptime-specialize time. Cluster C's real residue narrows
to one honest check: that the emitted data is internally consistent and that the hand-authored engine consumes
it correctly, both certified at our build by the two compilers plus the manifest, with a differential test over
the census corpus as belt-and-suspenders on the data-consumption, not as a proof of agreement between engines
that do not both exist. The corrected reachability rules of section 1 therefore live in exactly one place with no
duplication: they are the language's lease-rule schema (data emitted by Rust) consumed by the one hand-authored
Zig lease evaluator, and Cluster C's `InScope` fix is a fix to that single evaluator and schema, not to two.

## 3. The interpreter is a CFG of straight-line blocks, not one linear scan

Cluster C is right that B's `for n in 0..len` forward scan is correct only for straight-line dataflow and breaks
on branches (it would evaluate both effectful arms), loops (it cannot iterate), and script-internal calls (it has
no return mechanism). The fix keeps B's real win (the cache-friendly linear scan) exactly where it applies and
adds a control-flow layer above it.

The residual is a control-flow graph of blocks. A block is a maximal straight-line dataflow region: a contiguous
node range with backward-only child indices, evaluated by B's forward linear post-order scan, which keeps the two
sequential streams and the L1-hot backward reads unchanged for the dominant case. A block ends in a terminator,
and only the terminator touches control flow:

- Straight-line fall-through: continue into the next block.
- `If` / `Match`: the scrutinee is an already-computed value in the current window; the terminator selects one
  successor block by id and transfers to it. Only the taken arm is evaluated, which fixes the both-arms bug. The
  arm blocks are separate node ranges referenced by id, not inlined into the scan.
- Loop: a back-edge to a header block. Loop-carried values are block arguments passed on each edge (the SSA
  block-argument discipline, cleaner than phis in a no_alloc setting). A loop activation reuses one fixed window
  per loop-nesting level, so iteration allocates nothing.
- Call: the terminator saves a return record (resume block and index, result slot, caller window base) onto a
  fixed-capacity frame stack, sets up the callee window, and transfers to the callee entry block. Return writes
  the result into the caller's slot and pops. The frame stack is bounded by the depth cap `D`, the same finite
  quantity that already bounds the no_alloc frontier, the untrusted traversal, and the load verifier, so calls
  add no unbounded state.

Results are indexed by window-base plus local node id, so recursion and repeated loop activations reuse one local
id space across activations without collision, and backward child indices stay within a block and within an
activation. The dominance discipline (a used value is computed in a dominating block, still live in the results
window) makes cross-block reads valid without a scan of the intervening blocks.

This costs a wire-format addition, decided now: a block table (each block is a node range, a terminator kind, its
successor block ids, and its block-argument signature) and a function table (entry block, argument signature,
result signature). It is more than a flat node array, but it is the structure that makes control flow correct and
still lets the bulk of execution be the linear scan. It also lines up with the streaming and lease work: block
and function boundaries are whole-subtree boundaries, so they are the chunk boundaries the transfer topic and the
cross-chunk lemma already use, and the block boundary is exactly where `Escape` (section 1) fires.

## 4. Dispatch on Zig 0.14: do not depend on preserve_none for correctness

Cluster C is right that Zig 0.14's reworked `CallingConvention` does not expose `preserve_none` / `ghccc`, so B's
register-pinned tail-call dispatch does not hold on stock Zig today. The resolution separates correctness from
peak performance.

Correctness never depended on `preserve_none`: tail-call threading with `@call(.always_tail)` and the walk state
(node pointer, results pointer, blob pointer, window base) carried in an explicit hot-state struct is correct on
any Zig, and `always_tail` alone bounds the C stack. `preserve_none` is a performance win (it stops a cold op
spilling the hot path's registers); without it the compiler may spill the hot-state struct across dispatch, which
costs some cycles, not correctness. So the primary dispatch ships as `always_tail` threading over the hot-state
struct, correct today, and the `preserve_none` win is treated as upside, not a dependency.

Three moves make the upside reachable without blocking on it: sketch the dispatch on the real toolchain and
measure the actual spill cost (it is often smaller than feared, because the hot-state struct is a few pointers
and LLVM keeps small structs in registers under `always_tail` anyway); pursue a `preserve_none`-equivalent
calling convention upstream, since Zig lowers through LLVM which has `ghccc`, so the convention is an additive Zig
feature, not a missing capability; and keep the generated `switch` and computed-goto-style fallbacks (already in
B's design) for any target whose toolchain guarantees neither. Because native and peak dispatch are a
keystone-among-many and not the lone purpose, an interpreter that is good today on stock Zig and becomes maximal
when the convention lands is the right posture, not a blocker.

## 5. The lowering seam: a three-way split that also serves the templating consumers

Cluster C is right that B's binary "ship the load-verify residual subset only" leaves a hot un-lowered mod at
un-optimised interpretation on exactly the platforms that also cannot stencil it. The resolution is the three-way
split C gestured at, and it falls out naturally along the output-generation spectrum:

- Full equality-saturation lowering (macro expansion, const-fold, CSE, algebraic identities, the graded PE
  unfold) stays dev-time for bundled scripts, where the heavy e-graph can take its time.
- A cheap runtime lowering subset runs at load for arriving scripts: const-fold plus CSE as a bounded single
  bottom-up pass with hash-consing, a fixed rule set, no unbounded saturation, so it is no_alloc and cheap. This
  is the high-value core of the e-graph (fold what is now known, share common subexpressions) without the
  research-risk of the streaming e-graph on the load path.
- Heat-based native acceleration (the copy-and-patch stencils) applies on top where the platform permits it, as
  the accelerator it is.

This split is the one that repays the identity recenter directly. The cheap load-time const-fold-plus-CSE subset
is exactly what the authoring and templating consumers (the mockspace columnar-query and procedural-document DSL,
polka-dots' `.polka`) need: it folds the load-time-known configuration away and shares repeated structure at the
emit stage, which is where their speed concern lives and where load-time bloat is prevented, and it does so with
no dependence on the native tier at all. The same subset also keeps a hot arriving mod on a per-frame consumer
from running fully un-optimised where stencils are unavailable. One cheap pass serves both ends of the census.

## 6. The e-graph extractor: dev-time cost-model extraction, not a blocker

Cluster C lists the extractor as owed. With the seam resolved (section 5), the full e-graph is dev-time, so the
extractor is not on any hot path and can be the standard algorithm. Extraction is bottom-up cost-model selection
over the bounded e-graph window: each e-class picks its minimum-cost representative, where the cost function
encodes the two constraints already named in 2001 and 2055, respect the effect-ordering edges (never extract a
form that reorders an effect past a dependent read) and prefer the maximally-residualised form at the known
binding-time grades (the partial-evaluation-as-extraction-objective move). This is egg's linear extraction adapted
to the bounded window, a known algorithm rather than novel work; the only genuinely novel piece remains the
no_alloc bounded streaming e-graph itself, which stays flagged as original work and dev-time, where its cost is
affordable. The cheap runtime subset (section 5) needs no extractor at all, being a single directed pass.

## 7. Diagnostics and provenance: adopt Cluster C's Part 2 and land the schema now

Cluster C's Part 2 is already a full design, not a hole: provenance carried cheaply on the happy path, the
derivation witness reconstructed lazily on failure (the Souffle proof-tree technique on the provenance-semiring
foundation), with the reconstruction query generated into both loci by the same generator as the forward engine.
The solution is to adopt it and land its six schema additions in the wire format in the same pass as the block
and function tables from section 3: the span table, the binder-site table, the per-relation provenance arrays,
the violation seeds, the optional residual debug section, and the provenance-complete engine C ABI. The last of
these is what makes the one hand-authored engine of section 2 lossless for diagnostics, so sections 2 and 7 are
one decision: the engine emits the provenance-complete relations, and rich errors are reconstructed from them
wherever the analysis runs, with no separate prover present.

## 8. The format decisions that are measurements, not paper calls

Three of Cluster C's and B's open items are genuinely bench questions and are resolved as measure-then-lock, with
provisional values chosen so work can proceed and the experiment as the confirm-gate:

- Node record width. Provisional 16 bytes with two inline operands, on the arity-at-most-two assumption; the
  gating measurement is the real arity distribution across the census families (not ikiuni alone), and if calls
  and record construction are hotter than assumed the width goes to 24 with three inline operands. Measured in the
  same instrumentation pass as the coverage experiment.
- Region-id column width. Provisional `flags` byte when the live-region count is at most 256, else a parallel
  `[u16; N]` column; the gating measurement is the live-binder and live-region width distribution on real
  census code, measured in the same pass as the reachability avoidance-rate experiment.
- Whether the native tier beats the cheap-lowered interpreter by enough to justify the stencil trusted base. PEP
  744's note that CPython's copy-and-patch JIT is currently only interpreter-speed is the warning; the gating
  bench is stencil-versus-cheap-lowered-interpret on census workloads, and the honest posture is that native is a
  keystone-among-many, so a modest win still earns the accelerator on the platforms that permit it, and no win
  means the interpreter-only builds lose nothing.

These are the bench forks that must run before the format locks; they are not solved on paper, they are
scheduled against the two gating experiments the round already owes.

## The unifying thread

The three load-bearing moves interlock. The block-CFG model (3) gives the escape event (1) its precise firing
point (the block boundary where a binder leaves scope), gives the cheap runtime lowering (5) its unit of work
(const-fold and CSE per block), and gives calls a frame model bounded by the depth cap the design already leans
on sixfold. The one hand-authored Zig engine (2) is where the corrected reachability rules (1) live, as the language's
lease-rule schema consumed by a single evaluator rather than duplicated across backends, and the provenance ABI
(7) makes that engine's errors rich, so the diagnostics design and the dissolved identity concern collapse into
one small decision about certifying the data handoff. The dispatch posture (4) and the lowering seam (5) both take the recentered stance
that native and peak performance are a real ceiling and a genuine keystone, not the lone purpose, so neither
blocks the framework, and the cheap-lowering subset repays the templating consumers directly. What stays
genuinely open is measurement, not design: the three bench-gated format decisions (8), the upstream Zig calling
convention (4), and the exact wire encoding of the block and function tables (3), which is design work with a
clear shape rather than an unanswered question.

## Addendum A: solution 3 is a portfolio of strategies, each used only where it provably wins

Op's extension of solution 3 is the correct generalization, and it corrects a residual over-claim in section 3:
the backward linear scan is not the one ruling evaluation model with control flow bolted on. It is one strategy
in a portfolio, and it is the optimal choice precisely in the regions where its precondition provably holds
(straight-line dataflow with backward-only child indices). Other region shapes call for other strategies, and
the runtime uses each only where that strategy is the winner. The block-CFG structure of section 3 is then not
"the model," it is the substrate over which per-region strategy selection happens.

The pieces of this pattern have names in the literature. Partitioning a program into regions and compiling or
evaluating each independently, potentially by a different method, is region-based compilation (Hank, Hwu, Rau,
MICRO-28, 1995; the superblock lineage, Hwu et al., 1993); the straight-line region evaluated by the linear scan
is exactly a region in that sense. Selecting the winning strategy on the fly, from runtime-observed shape rather
than static structure alone, is trace selection (Bala, Duesterwald, Banerjia, Dynamo, PLDI 2000; the tracing-JIT
lineage, Gal et al., PLDI 2009); that is op's "or in fly" case. Resolving a branch by pre-evaluation where its
predicate is statically known, and leaving it as a residual branch where it is not, is the standard partial-
evaluation and binding-time treatment of conditionals (Jones, Gomard, Sestoft, 1993): a static predicate is
folded so only one arm is lowered, a dynamic predicate becomes a runtime branch; that is op's "pre-evaluated
either statically." The meta-principle, a set of methods where each is applied only where it beats the others,
is algorithm selection (Rice, 1976) and algorithm portfolios (Gomes, Selman, Artificial Intelligence, 2001).

What vehje adds, and where the name is owed, is that the selection is proof-directed and is itself a graded
discharge. The graded (co)modal system that proves family, effect, and lease also carries, per region, the grade
that names which evaluation strategy is legal and optimal there: a region proven straight-line dataflow carries
the grade that selects the backward linear scan; a region with a binding-time-known branch predicate is folded
by partial evaluation at lowering; a region whose predicate or loop shape is dynamic carries a grade that
selects the runtime block-transfer or trace strategy. The selection follows the same static-or-dynamic discharge
as every other axis in the design: where the grades prove the winning strategy from statically-known inputs, it
is chosen at lowering (static, folded, zero runtime decision); where they cannot, the choice is a bounded
dynamic residual decided on the fly, the same gradual shape (discharge at the earliest binding time the inputs
are known) that governs the whole spine. Strategy selection is therefore not a new subsystem; it is one more
coordinate of the one graded judgment, which is why it composes rather than stacks.

The proposed name, since the composition has no single existing one: proof-directed strategy selection (or,
naming the mechanism, graded strategy selection), with the existing terms above naming its parts. The framing
repays two commitments. It fits vehje's identity, the type system as the verification layer: the proof does not
only certify safety, it selects the optimal evaluator per region, so the grades earn their keep at runtime and
not only at the gate. And it fits the identity recenter: native and copy-and-patch are one strategy in this
portfolio, chosen in the regions where they provably win (hot, stencil-permitted, worth the trusted base), not a
ruling path, which is exactly the keystone-among-many placement the recenter demanded. The output-generation
spectrum (a target is a point on the interpret-to-transpile line) is this same idea one level up; the portfolio
is that spectrum applied per region to how a single target evaluates.

One honest boundary: a portfolio is a win only if the selection is cheap and the strategy set is bounded. The
selection must be a static grade lookup or a bounded on-the-fly test, never a search, and the strategy set is a
fixed, small vocabulary (linear scan, folded branch, block-transfer branch, bounded-loop forms, frame-transfer
call, stencil-accelerated hot region), each with a proven precondition, so that "use the winner" is a
constant-time dispatch on a grade rather than an open optimization problem at runtime. That boundedness is what
keeps the portfolio no_alloc and its selection off the hot path, and it is the condition under which the
principle is a win rather than a cost.

## Addendum B: reading the two sibling fork deliverables

Two other forks delivered alongside this one: `202607202355_counter_audit_of_cluster_c_holes_and_arguments.md`
(an adversarial attack on Cluster C's conclusions) and
`202607202358_reconcile-grounded-stack-and-rebuttal-to-identity.md` (a reconciliation of the grounded stack and
the rebuttal with the identity recenter). This addendum comments on both, gives the counter-audit the extra
scrutiny it warrants, states the solutions to the problems all three passes now confirm, and adds my own read.

### The convergence is the signal, and it is real

Four independent passes now sit over the same holes: the honest-keeper (Cluster C) found them, this document
filled them, the counter-audit attacked the fills, and the reconciliation mapped them to the identity. On the
three load-bearing holes they land on one shape: the reachability qualifier stays a width-`W` in-scope bitmask
with escapes routed to a separate terminal relation; the analysis engine is one thing that both loci use rather
than two backends tested for agreement; and the interpreter keeps the linear scan within straight-line regions
and handles control flow only at region boundaries. Convergence under independent adversarial attack is the same
signal the whole arc has trusted, and here it says the design is closer to whole than Cluster C's five-surviving-
problems tally implied. The counter-audit's own net (three of C's five are answered by prior art or the canonical
design, not by unbuilt work) is the same conclusion from the adversarial side.

### Extra scrutiny of the counter-audit: two sharp catches, two calibrations

The counter-audit is the sharpest of the three, and two of its catches are genuine corrections I accept.

Its Finding 1 shows my solution 1 was half-right and I take the correction. Filtering the propagation by
`InScope` and routing out-of-scope reaches to `Escape` bounds the naming set to `W`, but without a
binder-substitution rule at `Let` and `Lambda` it under-approximates: when the bound variable itself leaves
scope, dropping it without splicing the bound expression's reach loses any reach that flowed through it into a
still-live outer binder, judging safe what is not. The complete rule is the reachability-type binder rule (Bao,
Wei, Bracevac, Jiang, He, Rompf, "Reachability Types," OOPSLA 2021, the avoidance operation; the polymorphic
successor, Wei et al., POPL 2024), which is the very paradigm 2001 adopted, so the correct fix falls straight out
of the chosen lease theory. The counter-audit also catches that `N*W` bounds only the naming set, and the escape
set needs its own bound from region promotion (Tofte-Talpin region count plus the shared-implies-promoted
discipline of 2001). Both are prior art; both are right; solution 1's fix is superseded by the binder-rule-plus-
promotion-bound version, and section 1 now points here.

Its Finding 2 independently reaches the correction op made to this document's own solution 2, from a different
direction, which is worth stating because two passes converging on it is strong. The counter-audit shows that
Cluster A's "one generator emitting specialised Rust source and specialised Zig source" is itself the drift the
identity recenter warns about at the framing level, reappearing at the engine level: it re-imported the
LMS-hard "Rust metaprogram emitting foreign source" path that 1627 explicitly rejected. The canonical shape is
one hand-authored engine specialised by Zig comptime to Rust-emitted validated data, a single backend, which the
dev-time locus runs by linking the compiled engine; the divergence dissolves by returning to it, and that is less
engineering, not more. This is exactly the amendment now in solution 2, and the counter-audit's framing (the
canonical resolution, not an exotic fallback) is the right one.

Two places the counter-audit is a touch optimistic, worth calibrating rather than disputing. It calls the
interpreter control-flow fix "standard and cheap" and stops at "structured dispatch over the Core forms that
already exist." The concept is textbook and it is right that the eleven Core forms (`If`, `Match`, `Iter`,
`Interp`, `Apply`) mean control flow is first-class rather than an unmodeled surprise, so "the single largest
hole" was inflation. But the no_alloc streaming encoding of it is genuine design work, not free: block
arguments for loop-carried values, window reuse under the backward-index invariant, the depth-cap-bounded frame
stack, and the wire-format block and function tables are the real content, which section 3 supplies. Standard in
concept, real in encoding. And its Zig-version catch correctly deflates Cluster C (the `preserve_none` absence
was measured against 0.14.0 while 1845 pins 0.16.0, and the core register-pinning of the three walk pointers is
achieved on stock Zig by guaranteed tail calls plus explicit-argument passing, per Haberman's musttail result
which predates the convention), but it does not establish that the pinned version actually exposes
`preserve_none`; it asserts the trajectory. So the foundation holds on stock Zig today for the three core
pointers, `preserve_none` is the optimization tier for heavier handlers, and a real sketch on the pinned
toolchain is still owed, which is where solution 4 already lands. Its PEP 744 deflation is a clean scrutiny
point I agree with: CPython's copy-and-patch JIT is a deliberately first-generation baseline with no inline
caches or type feedback, so citing its current interpreter-speed is a thumb on the scale in both directions, and
the honest statement is solution 8's "must bench."

### The solutions to the problems all three passes confirm

Consolidated, taking the sharper version wherever the passes differ:

- The `N*W` reachability bound: the reachability-type binder rule (splice-and-drop at `Let` and `Lambda`) plus
  the region-promotion bound on the escape set, both prior art, superseding solution 1's `InScope`-only fill.
- The dual-locus identity: dissolved by returning to the canonical single hand-authored comptime-specialised
  engine (solution 2 as amended), with the differential test kept only as defence in depth over the data
  handoff.
- The interpreter control flow: structured dispatch over the existing Core forms, encoded as section 3's
  CFG-of-straight-line-blocks, with the per-frame budget re-derived on dynamic executed node counts rather than
  static counts (the counter-audit's genuine addition on Finding 3).
- Diagnostics: land Cluster C's Part 2 schema now, with the counter-audit's multi-premise refinement (a join
  rule with two recursive premises makes the reconstruction path buffer a bounded path tree sized by depth times
  max-recursive-arity, still finite, still failure-path-only).
- The untrusted-load decode: state it as a complete linear validation for arriving artifacts, not "by
  construction"; children-before-parents is a property the decode checks for untrusted input, and is by
  construction only for trusted bundled emission.
- The `preserve_none` path: an optimization tier, not a foundation; the core dispatch is buildable on stock Zig
  now, the sketch on the pinned toolchain is owed, and the convention is a fix-the-stack-upstream ask.
- The nonlinear-dynamical speed layer: struck, on which all three passes and both prior experts agree. It is
  empty on the finite bitmask lattices in play (Kleene iteration converges in at most the lattice height, and
  semi-naive evaluation is already the complete acceleration), and carrying an empty frontier-sounding layer
  corrodes the credibility of the genuine novelty around it.

### The reconciliation's contribution, and my net read

The reconciliation earns its keep by making op's recenter concrete on the tech: it shows 2055's engine is the
compile-stage brain whose primary beneficiary is the authoring and templating majority, not the native minority,
because equality-saturation lowering is macro expansion, constant folding, and common-subexpression elimination
unified, which is exactly the compile-and-emit stage a procedural-document DSL or a dotfile-templating language
runs to turn template shards and cascades into configured data. The load-time speed those consumers need so mod
and content stacks do not bloat comes directly from that engine being ahead-of-time compiled and from the cheap
const-fold-plus-CSE subset (solution 5). That reframing vindicates the most-attacked piece of 2055 in the
recenter's own terms, and it names the owed topics the narrow focus never wrote: the transpilation end of the
output spectrum, the authoring-and-templating consumer story, and the input-side grammar composition (Brzozowski-
derivative inclusion-not-coverage, the framework-of-many-DSLs identity applied to syntax).

My net across all three passes matches the reconciliation's independently: take the counter-audit's sharper
versions where a sibling conflicts (the binder-rule-plus-promotion `N*W` fix, and the return-to-canonical single
comptime engine, both now folded in); keep this document's constructive designs where they do not conflict (the
CFG-of-blocks encoding, the three-way lowering seam whose cheap subset serves the templating consumers, the
bench-gated format decisions); treat the diagnostics design as landed with the multi-premise refinement; and
strike the nonlinear speed layer. The honest whole is that the holes were real, the fixes are mostly prior art or
the round's own canonical design rather than unbuilt invention, and the design is closer to whole than the raw
count suggested, which is the recenter's claim reached from the technical side.
