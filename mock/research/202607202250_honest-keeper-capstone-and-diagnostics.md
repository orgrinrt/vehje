# Honest-keeper capstone: audit of A and B, the diagnostics surface, and the demotion adjudication

**Date:** 2026-07-20
**Phase:** research (Cluster C of the Carmack-rebuttal debate, `202607202205_topic.carmack-rebuttal-full-design-not-retier.md`)
**Scope:** the adversarial keeper-honest pass over Cluster A (`202607202210_design_dual-locus-relational-engine.md`)
and Cluster B (`202607202230_design_runtime-tier-and-interpreter.md`); the full design of diagnostics and binder
provenance as a product surface (problem 6, which neither A nor B designed); and the adjudication of Carmack's
seven demotions against what the debate actually designed.
**Reads:** A, B, the Carmack reaction (`202607202122`), the converged shape (topic 2001), the grounded stack
(topic 2055), the agenda (topic 2205). Grounding fetched: PEP 744, the Zig 0.14.0 release notes.

## Verdict

The maximal shape's keystone holds and this is the debate's real win: A proves the no_alloc, dual-locus
relational engine is buildable, which refutes the one Carmack claim that would have forced a smaller
architecture (that the engine is disqualifyingly large). The runtime layer holds with the framing Carmack
forced and B makes structural, interpreter as the product and native as a gated per-platform accelerator. But
the design is not yet whole, and the agenda's "we reject the demotion" framing is true for far less of Carmack's
seven than the topic implies. Five load-bearing problems survive the two clusters. First, A's `N*W`
reachability bound, the thing that makes the engine no_alloc-tractable at all, rests on a scope-kill the written
`Reach` rules omit; without it the bound is `O(N)` per node, not `W`. Second, the engine's remit is narrower
than "three queries on one engine": the structural decode is a linear front-end outside the engine (A agrees),
and B ships only the residual subset in the runtime, so the unification is real but smaller than the grounded
stack's rhetoric. Third, B's interpreter hot loop is designed only for straight-line dataflow and is silent on
the branches, loops, and script-internal calls that dominate real behavior scripts, which is exactly the piece B
claims decides per-frame identity. Fourth, B's primary dispatch depends on a Zig `preserve_none` calling
convention that stock Zig does not expose as of 0.14.0 (grounded below), so the register-pinning half of the
design is a tool-gated aspiration today, not a holding design. Fifth, A's default two-projection engine-identity
argument leans on "by construction," which is the weakest of its three assurance layers, not the strongest,
because two source backends are a divergence surface that determinism does not close. None of these is a genuine
impossibility. The only true impossibility in the stack, native code generation on iOS and the consoles, the
design already handles by the interpreter floor. The real residue there is a performance floor (un-lowered hot
mods interpreted on the platforms that forbid both stenciling and, per B, runtime lowering), which a cheap
runtime lowering subset should raise. So the maximal shape is defensible and largely whole as a design, but the
honest account of Part 3 is that the debate adopted most of Carmack's re-tiering under a maximalist banner, held
the genuine maximalist line only on the engine's buildability and the e-graph's inclusion, and still owes the
five design gaps above plus the diagnostics schema this document designs.

## Part 1: the audit of A and B

### Cluster A: the dual-locus relational engine

**What holds.** The core thesis is sound and well-grounded: egglog's demonstration that Datalog and equality
saturation are one semi-naive system over a differential database is real prior art, and it is the correct
reason all three analyses can be one evaluator rather than three. Lease inference as transitive reachability
closure is a genuine least-fixpoint, and the Polonius-over-Datafrog precedent for borrow inference as Datalog is
apt (with the caveat that Datafrog is heap-backed, which A acknowledges). The fixed-capacity data-structure
inventory in section 2 is the right shape: sorted-array relations in three generations (stable, delta, new) is
Datafrog's `Variable` made fixed-capacity, leapfrog triejoin is the correct worst-case-optimal join, and the
monotone-derivation argument (each tuple derived at most once, so total insertions are bounded by relation
capacity, no per-round accumulation) is the load-bearing memory fact and it is correct. The generator-not-proc-
macro argument is airtight: oatlog is a proc macro emitting heap Rust and cannot reach the Zig locus, so
"oatlog-shaped" must mean "our own AOT generator," and framing that as the work rather than a disqualifier is
the right posture.

**What is thin.** The e-graph design spends its words on representation (slotted for binders, colored for graded
conditions) and saturation, and almost none on **extraction**. Extraction is where a large part of the no_alloc
difficulty actually lives: picking the best term from a saturated e-graph subject to the effect-ordering edges
(`EffectEdge`) is a constrained optimization, and egg's own extractors range from greedy to ILP. A dismisses it
in one clause ("extraction respects the effect edges"). A bounded, no_alloc, effect-constrained extractor is an
undesigned piece, and it is not obviously cheaper than the saturation it feeds. The streaming-window discipline
(process `WINDOW` nodes, saturate, extract, advance) is named but its interaction with extraction quality across
window boundaries (a rewrite that would have fired across two windows is lost) is unexamined.

**What is asserted, not proven.** The central tractability claim, the `N*W` reachability bound, does not follow
from the rules as written, and this is the sharpest finding in the audit. A's base propagation rule is
`Reach(p, b) :- Child(p, _, c), Reach(c, b)`: a parent reaches whatever its children reach. Consider a child `c`
that is itself `let y = ... in (use y)`. Then `Reach(c, y)` holds, and the rule propagates `Reach(p, y)` to the
parent `p`. But `y` is a binder local to `c`'s subtree; it is not free in `p`, not in scope at `p`'s position,
and does not escape its own `let`. Propagating it up is both semantically wrong (a well-typed value's reach set
is a subset of the binders in scope at its position, and reachability types maintain exactly that invariant) and
fatal to the bound, because a node's reach set would then accumulate every binder anywhere in its subtree, up to
the subtree's binder count, which is `O(N)`, not `W`. The `N*W` bound holds only if the closure kills
out-of-scope binders at each scope boundary, that is, only with the filter `Reach(p, b) :- Child(p, _, c),
Reach(c, b), InScope(p, b)`. A defines the `InScope(n, b)` relation but does not use it in the propagation rule,
and the bound derivation in section 2 silently assumes the scoped result. This is not a cosmetic omission: the
kill is exactly where the lease mechanism lives. A binder that a child reaches but that is out of scope at the
parent is the escape case, and it must be resolved into a region (avoidance succeeds) or marked as a lease
residual (avoidance fails). A's `Escapes`, `Placeable`, and `LeaseResidual` rules gesture at this, but they are
written as a separate layer on top of an unscoped `Reach`, and the connection (the kill and the escape are the
same event) is not made. The fix is small (add the `InScope` filter, and route the killed-but-still-reached
binder into `Escapes`), but until it is made, the no_alloc tractability that justifies the entire engine is
unproven as written. Everything downstream, the `[BitSet<W>; N]` sizing, the `W`-round convergence, the
`N*W`-vs-`N^2` claim, depends on this one correction.

Two smaller asserted-not-proven points. The negation in the rule set (`!Placeable`, `!RegionOpen`) requires
stratification, and while the given rules happen to be stratifiable (compute `Escapes`, then `Placeable`, then
`LeaseResidual`), A names stratification only as a generator input, not as a well-formedness check the generator
enforces, and semi-naive evaluation interacting with stratified negation over fixed-capacity tables needs the
per-stratum discipline stated. And `Reach` is represented both as a tuple relation (for leapfrog joins) and as a
lattice column (the per-node bitmask that gives the `W`-round bound); keeping the two representations coherent
(every tuple insert also sets the bit) is an unspecified consistency obligation.

**The dual-locus identity, pressure-tested.** The task asks whether "one generator, two projections" is sound or
hides a divergence risk. It hides one, and A mis-grades its own assurance layers. A presents three: by
construction (deterministic projections of one record), differential test (grade-0 shadow), and single shared
object (assurance-maximal fallback), and recommends the two-projection variant as default with "by construction"
as its primary assurance. But the by-construction argument proves the wrong thing. Determinism of the generator
guarantees that the same input record reproducibly yields the same Rust source and the same Zig source. It does
not guarantee that the Rust source and the Zig source **compute the same function**, because "one generator" has
**two backends** (a Rust emitter and a Zig emitter), and A's chosen shape emits "specialized semi-naive loops
monomorphized to the exact relations," that is, specialized code in two languages compiled by two toolchains
(rustc, and Zig-over-LLVM) with independent behavior on integer overflow, evaluation order, and undefined-
behavior corners. The two emitters are precisely the divergence surface, and determinism over divergent backends
gives you deterministic divergence, not agreement. So the three layers are not three grades of one guarantee.
The single shared object is the only one that eliminates divergence (one backend, one object, both loci link it,
identity is bit-identical). The differential test is the real operational assurance for the two-projection
variant. By construction is a coherence story that reduces but does not close the risk, and it is the weakest
link for A's chosen specialized shape, not the strongest. The honest grading: for the two-projection default,
the differential harness must be a merge gate, not an option, and the by-construction claim should be stated as
"reproducible and single-sourced at the front end," never as "the same function." A stronger by-construction
variant exists (emit a shared abstract plan into both loci and a thin trusted loop over it, collapsing toward
the single-object case), but A explicitly chose specialization for speed, which maximizes the divergence surface
and is the right performance choice only if the differential gate is mandatory.

**What A missed.** The extractor (above). The scope-kill (above). And one framing slip in the load-verify proof:
A says children-before-parents holds "by construction" for the arriving arena, but for untrusted input nothing
holds by construction; the decode **checks** that every child index is strictly less than the node's own index,
and that linear check, being **complete** (it rejects every buffer that violates the property), is what makes
acyclicity follow. The proof that the decode is not a fixpoint is otherwise correct and important: index-in-
range is a map, the backward-index check implies acyclicity in one pass, and depth is a running counter read
from the already-filled `depth` array, so the whole decode is a linear fold, and casting it as a fixpoint query
would forfeit the completeness and SIMD-friendliness that make it sound. That conclusion holds; only the "by
construction" phrasing for untrusted input is wrong, and it matters because the decode's trust is the base the
engine's soundness rests on for arriving scripts, so the correct statement is "complete linear validation," not
"assumed by construction."

### Cluster B: the runtime execution layer

**What holds.** The framing inversion is correct and well-grounded: interpreter is the certified floor present
on every platform, native is a per-platform accelerator gated by a build-time capability profile and a run-time
executable-mapping probe. The W^X analysis is accurate; PEP 744 confirms every citation B rests on (no point
writable-and-executable, clang required for `musttail`, the macOS hardened-runtime JIT entitlement, LLVM used
for object parsing and disassembly). The two-gate model is the right structure, and separating platform
capability (the two gates) from optimization heat (a third axis) is a clean decomposition that makes an
unavailable native tier exactly "heat threshold of infinity" with an unchanged interpreter path. The tier-down-
preserves-certified-generation argument is genuinely sound: the guarantee is a property of the residual,
discharged before any execution strategy is chosen, and the interpreter arm and the stencil arm are projections
of one semantic definition (Deegen is the existence proof that interpreter and baseline JIT from one definition
agree), so changing the executor never changes the computed result. This is the strongest single argument in
either cluster and it holds without qualification. The stencil toolchain specification (CPS templates, guaranteed
tail calls, the object-parse-and-relocation-classify pass, per-target relocation tables, the register-indirect-
transfer preference that dodges the `CALL26`/`JUMP26` range limit) is concrete, correct, and correctly placed in
a named TCB with the stencil table content-addressed and manifest-checked rather than trusted by assertion.

**What is thin.** The budget claim, "it is a port of CPython's `Tools/jit`," undersells the delta. CPython's
driver knows CPython's micro-ops statically; the vehje extractor must be parameterized over an arbitrary
generated op set with the certified-generation manifest binding, which B admits is "the one genuinely new piece"
but then frames as cheap. The bigger thin spot is B's own justification for paying the stencil TCB at all. PEP
744, which B cites for the mechanism, also reports that CPython's shipped copy-and-patch JIT is currently "about
as fast as the existing specializing interpreter," not the order-of-magnitude wins B quotes from the OOPSLA 2021
Copy-and-Patch paper. Those OOPSLA numbers are for a from-scratch code generator against a naive interpreter and
against LLVM -O0, not for a baseline JIT against a tuned tail-threaded interpreter over an already-statically-
checked flat IR, which is vehje's actual case. B acknowledges this honestly in one paragraph ("the interpreter
overhead a baseline JIT removes is smaller here") and in its last open question, but the verdict still says the
layer "holds at full strength," and the grounded evidence says the native tier's marginal win over the tuned
interpreter is the least certain quantity in the whole design. That is not a reason to demote it; it is a reason
to state plainly that the native tier's ship-value genuinely gates on a bench that PEP 744's own data suggests
may come back marginal.

**What is asserted, not proven.** The interpreter hot loop, the piece B rightly calls the per-frame identity, is
designed for straight-line dataflow and is silent on control flow. B's model is `for n in 0..len {
dispatch(node[n]) }`, a single forward linear scan that evaluates every node after its children, with the tail-
call chain being the forward walk itself (handler for `node[n]` tail-calls the handler for `node[n+1]`). This is
correct and beautiful for a pure expression DAG. It breaks the moment there is an `if`, a `while`, or a script-
internal function call, and behavior scripts are made of those. A-normal form, which B invokes, makes evaluation
order explicit but does not remove conditionals or loops; the ANF of `if c then a else b` still branches, and a
loop still has a back-edge. Under the pure forward scan, both arms of a conditional are evaluated as the scan
passes over them, which is wasteful for pure branches and **wrong** for effectful branches (and effects are a
first-class axis here), and a loop cannot iterate at all in one forward pass. The real interpreter must compute
its successor (a branch target, not always `n+1`), must have a mechanism for loop back-edges, and must have a
call/return mechanism (a control stack) for script-internal functions, which B's "no interpreter call stack"
explicitly disclaims. Once the successor is computed rather than `n+1`, the tail-call target is a jump to an
arbitrary handler, which tail-threading handles fine, but the cache story B sells (strictly front-to-back scan,
perfect hardware prefetch, no pointer chasing) degrades to "mostly sequential with data-dependent jumps and
backward loop re-reads." The design is not killed (tail-threaded dispatch over an arena is still the right
shape), but the specific per-frame performance claims are derived from the ideal branch-free case and are
optimistic for control-heavy scripts. This is the single largest hole in B, and it sits in the piece B claims
matters most.

The per-frame budget arithmetic compounds it. B's "1 to 3 ns per node, a hundred-node script is 100 to 300 ns, a
thousand invocations is 100 to 300 microseconds, 1 to 2 percent of frame" conflates **static** node count with
**dynamic** executed node count. A script with a loop of 100 iterations over a 20-node body executes 2000 nodes,
not 20, so the figure is optimistic by whatever the average trip count is, and it assumes the ideal straight-
line dispatch cost with no control-flow mispredicts. The direction is right (load-verify is off-frame,
interpretation is cheap-ish), but the specific 1-to-2-percent number is under-justified and should not be
quoted as a budget until the control-flow model exists and a real behavior-script dynamic instruction profile is
measured.

**The `preserve_none` and Zig risk, grounded.** B flags this as a possibly-fatal open question, and the grounding
confirms it lands unmet today. Zig 0.14.0 reworked `CallingConvention` into a tagged union that "now contains
every major calling convention for every target currently supported by Zig," with options like
`incoming_stack_alignment` and x86 regparm, but the release notes enumerate no `preserve_none`, no `ghccc`, and
no caller-preserves-nothing convention. LLVM has had `preserve_none` since LLVM 19, so the capability exists one
layer down, but Zig does not surface it. `@call(.always_tail)` is a guarantee-or-compile-error primitive (it
either emits the tail call or fails the build), which is good for correctness determinism and is the right basis
for B's "computed-goto and switch as generated fallbacks when the toolchain cannot guarantee the tail call." But
without `preserve_none`, the tail-called handlers use the platform C convention, whose caller-saved and callee-
saved split forces the three walk pointers (node, results, blob) to spill and reload across each dispatch, which
is exactly the scenario B names where "much of the win evaporates." So B's primary dispatch design, tail-call
threading **with** `preserve_none` register pinning, does not hold on stock Zig 0.14.0. It is not fatal: the
mitigations are real (a hot state struct the optimizer is coaxed to keep in registers, inline-asm register
pinning, a Zig upstream ask to surface LLVM's `preserve_none`, or the single-shared-object C or `ghccc` kernel
for the hot dispatch), and per the workspace's fix-the-stack-upstream discipline the clean move is the Zig
upstream ask backed by a sketch on the real target toolchains. But B's verdict ("holds at full strength")
overstates against B's own open question, which correctly calls this "a real risk to the whole dispatch design"
and the first sketch owed. The honest keeper sides with B's open question over B's verdict.

**The 16-byte record and arity-at-most-two.** B flags this as bench-decidable and names 24 bytes as the
alternative, which is the correct posture for a format decision that cannot change after buffers exist in the
wild. Two sharpenings. First, the assumption doing the load-bearing work, "arity at most two dominates because
binops, unops, variable references, literals, comparisons, the arithmetic that dominates a game behavior-script
inner loop," is asserted, not measured, and it is plausibly wrong for a **scripting** language whose primary job
is orchestrating engine and host calls rather than running numeric inner loops; a behavior script that is mostly
`call this system, call that system, dispatch to that handler` is arity-heavy, and then the `POOL_SPAN`
indirection is on the hot path and 16 bytes buys little over 8. Second, the four-way reuse B is proudest of (one
record serving interpreter operand encoding, wire format, SIMD-decode input, and eval order) couples four
different workload profiles to one width decision; the profile that wants 16 for the eval-order arithmetic case
is not obviously the profile the value-wire case or the call-heavy case wants, so the four-way reuse is a four-
way compromise, not a free win. B's deferral to a bench is right; the correction is that the bench must run
before any format lock and B's lean toward 16 should be held as provisional, and the arity distribution measured
on real ikiuni behavior scripts (which are likely call-heavy, not arithmetic-heavy) rather than assumed.

### The A/B seam adjudications

**"How much of the engine ships" (A's open question, B's resolution).** A raised it; B resolved it by shipping
only the load-verify residual subset (decode plus lease residual plus numeric residual) in the runtime, keeping
full eqsat lowering dev-time-only, and interpreting arriving scripts un-lowered with heat-based stenciling for
hot loops. This is defensible for shipped-runtime size and it correctly closes several of A's open questions in
passing (B states the overflow policy, dev-time hard error and load-time demotion to the generational residual,
and the region-id width, `flags` byte when `W` is at most 256 else a parallel `[u16; N]`). But B's binary choice
(full eqsat engine in the runtime, or no lowering at all) leaves a real performance floor exactly where the
design most wants mods to run well. On iOS and the consoles, B's own primary targets, native is forbidden, so an
arriving hot mod loop gets neither stenciling (platform-forbidden) nor lowering (not shipped), and runs as plain
interpretation of un-optimized, un-lowered code. B names this in its last open question but does not resolve it,
and its claim that "heat-based stenciling is the larger win for a hot mod loop than eqsat lowering would be" is
both asserted and, on the platforms that forbid stenciling, vacuous. The adjudication: the seam should not be
binary. The runtime should ship a **cheap lowering subset**, const-fold plus common-subexpression elimination
via the content-addressed hash-cons the flat arena nearly gives for free, which is exactly Carmack's "hash-
consing plus a specializer" and is a few hundred lines, not the heavy no_alloc slotted-and-colored e-graph. That
raises the iOS and console mod floor without shipping the e-graph, and it is the maximal thing that holds where
full-eqsat-in-runtime is too heavy and zero-lowering leaves mods slow. So: dev-time gets full eqsat for bundled
scripts; the runtime on all platforms gets decode plus lease residual plus numeric residual (mandatory
verification) plus the cheap const-fold-and-CSE lowering subset; the runtime on native-permitted platforms adds
heat-based stenciling on top. This is a three-way split neither A nor B took, and it is the honest resolution of
the seam.

**Where A and B agree and it holds.** B's budgeting of A's residual engine as a once-per-load, off-frame cost is
consistent with A and is correct: the structural decode is `O(N)` linear, the arriving-value lease residual is
bounded to `W` rounds and `N*W` tuples (modulo the scope-kill correction above), and the numeric residual is
bounded by tnum lattice height, all sized by the three structural constants and paid at level or mod load, not
per frame. The per-frame budget sees only the interpreter walk. That decomposition is sound.

## Part 2: diagnostics and provenance as a product surface

Neither A nor B designed this, and the Carmack reaction is right that it is a product surface, not an
afterthought: strict-by-default with lease-inference failure as a compile error means a modder's first contact
with vehje is a lease error, and a bitmask that says "inclusion failed" without naming which binder, bound
where, escaped through what, is a hostile first contact. The design below carries provenance from the schema
outward, produces good errors at the dev-time Rust locus, produces the best-available error at the Zig runtime
locus with no compiler present, and survives the single-shared-object engine variant. The governing principle,
which makes it no_alloc-compatible, is that **provenance is carried cheaply on the happy path and the derivation
witness is reconstructed lazily on the failure path**, over the relation tables that are still resident, in work
bounded by derivation depth. This is the shipping discipline of Souffle's provenance subsystem (on-demand proof-
tree reconstruction rather than eager storage) grounded on the provenance-semiring foundation (Green,
Karvounarakis, Tannen, 2007) that the grounded stack already identified as the joint of the design, and it is
the place that theory earns a real implementation home rather than the light grip Carmack allotted it.

### 2.1 The schema, decided now

Six additions, all fixed-width, all sized by the three structural constants, so they are paid up front as
known no_alloc cost and none is retrofitted onto a bitmask later.

A **span table** parallel to the node arena, `SpanId -> { file_id: u16, byte_start: u32, byte_len: u16 }`,
indexed by node id. It is populated in the same pass that already runs: topic 2001's "the parse is the first
graded computation" attaches the initial grades as synthesized attributes bottom-up, and the source span is one
more synthesized attribute attached at the same visit, so spans ride the existing one-pass L-attributed
evaluation with zero extra passes. On the dev-time locus the span points into the mod author's source. On the
runtime locus it points into the residual's optional debug section (2.5).

A **binder-site table**, `BinderId -> SpanId`, built by the fused resolve walk as it threads the scope chain. The
walk already visits every `let` and `lambda` binder; recording the binding-site span per binder is `O(1)` per
binder and no new pass. This is the "which binder, bound where" anchor of every lease error.

A **provenance array parallel to each engine relation**, sized at that relation's `CAP`, each entry a fixed
record `{ rule_site: u16, premise_key: u32 }`. `rule_site` names which Horn rule produced the tuple (a small
const index over the rule set). `premise_key` is the key of the single recursive premise that fired the rule.
The reach rules have exactly one recursive premise (`Reach(p, b) :- Child(p, _, c), Reach(c, b)` recurses only
on `Reach(c, b)`; the `Child` fact is recoverable from `(p, c)`), so one back-pointer plus the rule site is
enough to walk the reach chain backward from any tuple to the `VarUse` that introduced the binder. For rules
with two recursive premises, the record widens to two `premise_key` slots; the maximum is a small closed
constant known from the rule set, so the width is fixed at generation time. Eight to twelve bytes of provenance
per tuple, inside the `N*W` constant factor.

The **violation relations carry the reconstruction seed inline**. `LeaseResidual(n, b)` carries `(n's SpanId,
b's BinderId, rule_site)`; `RegionViolation(n)` carries `(n, region_id, rule_site)`; the decode's structural
rejections and the numeric residual's bound violations carry `(node_id, kind, offending_value)`. The seed is the
one tuple from which the witness is reconstructed.

The **residual's optional, strippable debug section** (2.5), the wire-format analogue of DWARF: a node-id to
source-coordinate map plus original identifier names, present in dev and debug residuals, stripped in shipping
residuals.

The **engine C ABI is provenance-complete** (2.6): in every identity variant the engine exposes not just the
verdict but the marked-output relations with their provenance and a backward-query accessor, so the diagnostics
layer never needs Rust-native structures.

### 2.2 Provenance through the fused resolve walk and the engine

The resolve walk produces the base facts (`VarUse`, `InScope`, the binding sites) and attaches the two human-
meaningful anchors as it goes: each binder's binding-site span (where `let x` or `lambda x` appears) and each
variable occurrence's use-site span. Both fall out of the single pass the walk already runs. As the engine
derives tuples, semi-naive evaluation already knows, at each rule firing, which premise tuple triggered it (that
is the `delta` binding that makes the rule fire); recording that premise's key and the rule site into the
provenance array at the moment of insert is one store per derived tuple, on the happy path, and nothing more.
No derivation trees are built eagerly. The happy path pays one fixed-width provenance store per tuple and no
reconstruction.

### 2.3 Lazy reconstruction of the derivation witness

When a violation tuple appears, the diagnostics layer reconstructs the witness for that one tuple by following
`premise_key` back-pointers over the resident relation tables. From `LeaseResidual(n, b)`, read the `Reach(n, b)`
tuple's `premise_key = c1` (the child that propagated the reach), then `Reach(c1, b).premise_key = c2`, and so on
until the terminal `VarUse(mk, b)` that first introduced `b` into the reach set. Each hop's node has a span. The
result is a span-annotated escape path: the value at `n` escapes the scope of binder `b` (bound at `b`'s span);
`b` enters this value at use-site `mk`'s span, and flows through the nodes `c1, c2, ...` (each with a span) up to
`n`. That reconstructs "which binder, bound where, escaped through what" in full. The reconstruction is a bounded
backward walk (its length is at most the reach-chain length, itself bounded by nesting depth `D`), it runs only
on the failure path, it allocates nothing (it reads resident tables and writes into a fixed-capacity path buffer
sized by `D`), and it costs zero on the happy path. This is the how-provenance polynomial of provenance-semiring
Datalog, computed lazily for one output rather than stored for all.

### 2.4 The dev-time Rust diagnostics layer

On the Rust locus the marked relations, the provenance arrays, and the span table are all Rust-native, so the
layer is a renderer over the reconstructed witness. It slots into CLAUDE.md's existing error-variant discipline
(descriptive variants carrying span, phase, expected, actual): the variant is `LeaseEscape { escape: Span,
binder_site: Span, flow: BoundedPath<D>, remedy: Avoidance }`. It renders a multi-span diagnostic in the rustc
idiom: a primary span at the escape site `n` ("this value escapes the scope of `x`"), a secondary span at the
binder site ("`x` is bound here"), and a labeled note chain along the flow path ("reaches `x` through here", "and
here"). Because reachability types give a concrete avoidance operation, the error offers a real remedy: either
"the value must not reference `x`; move the binding outward" when avoidance is structurally possible, or "this
reference falls to a per-reference generational check (one `u32` compare at deref)" when the generational
residual is the graded fallback. Offering the tier-down as a named, costed suggestion is the strict-but-humane
posture, and it is only expressible because the lease axis was designed with the generational residual as a
first-class low-assurance grade rather than a failure.

### 2.5 The Zig runtime locus, no compiler present

This is the hard case. An arriving mod is the **serialized residual** (the arena), not source text; its source
was compiled elsewhere. So spans-into-source do not exist at the runtime locus by default, and the Rust
diagnostics layer is not present. Two mechanisms cover it.

First, the **optional debug section** in the residual. When the mod author's dev-time compiler emitted the
residual, it optionally embedded a node-id to source-coordinate map plus identifier names, exactly as a native
toolchain embeds DWARF. The runtime residual engine, on a violation, reads this section to render "node `n` (was
`foo` at `mymod.veh:42:7`) escapes region `R`." With the section stripped (a shipping residual), it renders the
structural error "node `n` (op `Apply`) escapes region `R`; rebuild the mod with debug info for source
locations." The verdict-plus-structural-seed is always available; the source anchoring is present exactly when
the debug section is.

Second, the **reconstruction logic is the same query, generated into both loci**. The backward witness walk of
2.3 is itself a bounded query over the same relations the forward engine derives, so it is emitted from the same
relational-program record by the same generator that answers problem 1, into both the Rust and the Zig locus. It
is not hand-written twice. The Zig-side reporter runs the identical reconstruction over its resident residual
tables and produces the same span-annotated escape path (mapped through the debug section when present). This is
the direct payoff of problem 7's "written once, emitted into both": the diagnostics reconstruction is one more
projection of the one description, so a mod that fails load verification gets a real escape-path error from the
runtime with no compiler, no Rust, and no source, using only the residual, its optional debug section, and the
generated reconstruction query.

Third, the **host-configuration case is a distinct diagnostic**. When a violation is the host's fault rather than
the mod author's (the host chose an environment-width `W` too small and the mod overflowed it, which B's overflow
policy demotes to the generational residual rather than bricking), the runtime emits a warning keyed to the host,
not an error keyed to the mod: "mod exceeds this runtime's environment-width `W = 64`; affected references
demoted to dynamic checks." Never-brick-always-degrade (B's principle) extends to the diagnostic: the mod loads,
the host is told what it cost, and the mod author is not blamed for the host's width choice.

### 2.6 The single-shared-object variant

The task flags that A's assurance-maximal single-shared-object variant hides Rust-native structures, so the
dev-time diagnostics layer "must reconstruct provenance from the kernel's marked-output relations rather than
from Rust-native structures." The design makes that a feature, not a degradation, by making the engine's output
interface provenance-complete in **every** variant. The kernel's C ABI exposes not just a bitmask verdict but a
`violations()` accessor returning an array of seeds `{ node_id, binder_id, region_id, rule_site }`, and a
`provenance_of(relation, tuple_key) -> { rule_site, premise_key }` accessor that the backward reconstruction of
2.3 drives across the ABI. So the diagnostics layer, on either side and in every identity variant, runs the same
reconstruction by calling the kernel's provenance accessors; it never needs the emitter's internal Rust
structures. The cost is a slightly wider kernel ABI (the two provenance accessors), which is a clean designed
cost, not the "diagnostics get worse" hand-wave A left it as. The principle generalizes: **the engine's output
contract is the verdict plus the provenance-bearing marked relations plus a backward-query accessor, in every
variant**, so provenance is a first-class engine output designed into the C ABI from the start, and the single-
shared-object variant loses nothing on diagnostics. This also resolves A's open question that named exactly this
interaction.

## Part 3: the demotion adjudication

The agenda rejects Carmack's "build the floor, re-tier the unification layer" as YAGNI. The honest account,
comparing what A and B actually designed against Carmack's seven specific re-tierings, is that the debate held
the genuine maximalist line on one-and-a-half of the seven and, on the other five-and-a-half, **adopted**
Carmack's re-tiering while describing it as designing the maximal shape. That is not a failure. Carmack's
re-tierings on those items were placement and sequencing corrections, not demotions of ambition, and a design
that agrees with them is not a smaller design. But the "we reject the demotion" framing should be corrected so a
future reader does not mistake "we refuted all seven" for what happened. The seven, each marked.

**1. The single relational engine as the base. Answered by design, and this is the real win, but narrowed.** A
proves the no_alloc dual-locus generator is buildable, which refutes Carmack's disqualifying claim ("a full
compiler project standing under the compile stage," treated as too large to build). That refutation is genuine
and it is the debate's central achievement. But two narrowings stand, and one is Carmack's mechanism catch
vindicated. The structural decode is not a query on the engine (A agrees with Carmack completely; the decode is
a linear front-end that materializes the EDB), so Carmack's "strike load verification from the engine's query
list" is correct and adopted. What Carmack got wrong is the conclusion "what remains is too little to justify an
engine": lease inference (a genuine fixpoint) and eqsat (a genuine fixpoint) do justify the engine at dev time.
The residual open point: whether **lease** should run on the engine or as Carmack's fused syntax-directed walk is
a live performance question A does not settle. A concedes Carmack's charge that "rehosting a fused walk onto a
relational engine makes the common case slower" and refutes only the buildability, not the speed. Per the
workspace's own bench-decided-forks discipline, lease-on-engine versus fused-walk on the common tree case is a
bench, not a first-principles win, and A should claim only that the engine is unified and buildable, never that
it is faster. Verdict: answered by design (engine buildable, refuting the disqualifier); Carmack's load-verify
catch adopted; the lease-placement is a bench fork, not a settled maximalist win.

**2. The equality-saturation compile stage as v1 machinery. Answered by design for the specializer half; the
e-graph half is a legitimate bench-decided fork, not a YAGNI demotion.** The grounded stack itself (2055, the A3
resolution) already split the compile stage into a graded well-founded partial-evaluation unfold (a recursive
specializer with memoization, which **is** Carmack's specializer) plus equality saturation over only the
confluent algebraic identities. So the specializer half is agreed by both. The delta is whether the confluent-
identity optimization runs as a bounded streaming slotted-and-colored e-graph (A's design, which A itself lists
as the hardest unbuilt piece with an undesigned extractor) or is deferred to a bench-gate against the simpler
hash-consing-plus-specializer pipeline (Carmack). This is precisely a "which is faster" fork, and the
workspace's bench-in-harness rule says such forks are decided by benches on real workloads, not by first
principles and not by declaring the maximal shape. So Carmack's "bench-gate the e-graph until it beats the simple
pipeline" is not YAGNI; it is the workspace's own discipline correctly applied. The design is owed (A sketches
it); the commitment is bench-gated, and the baseline the e-graph must beat is Carmack's simpler pipeline.
Verdict: answered by design for the specializer; the e-graph is a bench-decided fork, and labeling that fork a
YAGNI demotion mislabels the workspace's own bench discipline.

**3. The assurance-indexed gradual logical relation as the spine. Carmack's demotion survives, is correct, and
the debate quietly adopted it.** Carmack's position: ship the 1845 proof skeleton (bounded, scoped), hold the
unified logical relation as a north star a later effort may fund, and let no implementation decision wait on it,
because a step-indexed logical relation at this scale is an Iris-scale multi-person-year mechanization. A agrees
in substance: A makes the **differential test** the day-one operational assurance ("the assurance mechanism that
will actually exist on day one") and treats the full logical relation as the assurance-maximal fallback that
nothing waits on. That is exactly Carmack's re-tiering. Neither A nor B blocks on the logical relation; both
ship the differential test. Verdict: genuine survival, and correct; the maximal shape does not require the
logical relation built before implementation, only targeted, which is precisely what Carmack said. The "reject
the demotion" framing does not hold here.

**4. AARA-inferred potentials. Adopted, exactly as Carmack said.** The grounded stack's A9 resolution already
retired AARA for three genuine structural constants (environment-width, nesting-depth, input-length), and A
explicitly keeps the three bounds as constants and AARA off the critical path. Carmack's "AARA to three
constants" is the shipped position. Verdict: adopted; not rejected, not survived, simply kept demoted.

**5. The provenance-semiring framing. Adopted for the forward engine; given a real home in diagnostics.**
Carmack: fine as a coherence argument, over-reaches the moment it dictates a semiring-annotated engine in code,
where the grades are two small enums and two bitmasks and the homomorphism is a match statement; hold the
scoping (grades in the proof, never in the runtime types). Neither A's concrete relations-and-bitmasks engine nor
B's concrete records carry semiring machinery, so the scoping Carmack demanded is kept in implementation. But
"held with a light grip" undersells it: the semiring is the correct and load-bearing theory for the **diagnostics
provenance** of Part 2 (the how-provenance polynomial reconstructed lazily for a violation witness). Verdict:
Carmack's demotion adopted for the forward engine; the semiring finds its genuine, non-decorative implementation
home in the diagnostics layer, which is a refinement of Carmack's position, not a rejection of it.

**6. Brzozowski derivatives as the runtime parser. Adopted, uncontested, correct.** Carmack: cross-family grammar
conflict detection is a dev-time job (derivatives, or any decidable approximation, at any cost), and the composed
runtime parses one fixed, already-proven-conflict-free grammar with a generated table parser (fast, boring,
no_alloc). Neither A nor B needs derivatives at the embed site, and the placement is plainly right. Verdict:
adopted and uncontested; a placement correction, not an ambition demotion.

**7. The nonlinear-dynamical fixpoint acceleration (the speed layer). Adopted (struck), exactly as Carmack
said.** Carmack: strike it as a category mistake (numeric sequence extrapolation has no meaning on a boolean
lattice; a Kleene iteration over a fixed-width mask converges in at most its width and in practice a handful;
semi-naive is already the correct and complete acceleration for the relational fragment); keep plain widening
only if a numeric domain iterates. A uses semi-naive evaluation and bounds convergence by lattice height (the
reach bitmask in `W` rounds, tnum by lattice height), never mentions the speed layer, and keeps plain widening
for the numeric residual. That is Carmack's position verbatim. Verdict: adopted (struck).

**Genuine impossibilities.** Exactly one, and the design already handles it: native code generation on iOS and
the consoles is a hard platform prohibition (executable mapping forbidden to third parties), proven by the
platform capability model itself, and the design correctly does not attempt native there, falling to the
interpreter floor. That is an impossibility contained by design, not a design that fails. The one residue is not
an impossibility but a performance floor: un-lowered hot mods interpreted on those platforms (native forbidden,
and, per B's binary seam resolution, no runtime lowering shipped), which the Part 1 seam adjudication resolves by
shipping the cheap const-fold-and-CSE lowering subset into the runtime. The `preserve_none` gap and the
interpreter control-flow gap are design gaps and tool-gated dependencies, not impossibilities; both have named
holding designs.

**The honest summary of Part 3.** Of Carmack's seven, the debate refutes the strong disqualifying form of one
(the engine is buildable no_alloc dual-locus, which A proves) and designs the specializer half of a second. On
the other five (the logical-relation spine, AARA, the provenance-semiring scoping, the runtime parser, and the
speed layer) the debate adopted Carmack's re-tiering in substance, because those five were sequencing and
placement corrections that a maximal design agrees with rather than ambitions to defend. The e-graph is neither
rejected nor adopted but correctly recast as a bench-decided fork per the workspace's own rule. The agenda's
"the re-tiering is YAGNI and we reject it" is true for the engine's buildability and the e-graph's commitment;
it is theatrical for the five items where "keep the maximal shape" and "adopt Carmack's placement" produce the
same artifact. Stating that plainly is the honest-keeper's job, and it costs the design nothing: the keystone
holds, and holding it was worth the whole arc.

## What genuine design work remains before this is a build target

The engine's `Reach` closure must be corrected to filter by `InScope`, the scope-kill must be routed into the
`Escapes`/`LeaseResidual` mechanism as the single event it is, and the `N*W` bound re-derived over the scoped
closure. Without this the no_alloc tractability that justifies the engine is unproven. This is small and
load-bearing.

The no_alloc, bounded, effect-constrained e-graph **extractor** must be designed. A designed representation and
saturation; extraction under the `EffectEdge` ordering constraints, within the fixed-capacity budget, across
streaming-window boundaries, is undesigned and is where much of the remaining difficulty sits.

The interpreter's **control-flow model** must be designed: computed-successor dispatch for conditionals, a
loop-back-edge mechanism, and a bounded control or call stack for script-internal functions, with the cache
story re-derived honestly (it degrades from perfect-sequential to mostly-sequential with data-dependent jumps),
and the per-frame budget re-estimated on dynamic (not static) executed node counts from real behavior scripts.

The `preserve_none` path must be resolved: a Zig upstream ask to surface LLVM's `preserve_none` in
`CallingConvention` (the capability exists one layer down, and 0.14.0 just made the enum extensible), or a
committed single-shared-object C or `ghccc` kernel for the hot dispatch, or the hot-struct-in-registers fallback.
Per B's own last open question, this is the first sketch owed on the runtime arc, on the real target toolchains
including the consoles.

The **cheap runtime lowering subset** (const-fold plus CSE via the content-addressed hash-cons) must be added to
the shipped runtime for arriving scripts, resolving the "how much engine ships" seam as a three-way split rather
than B's binary, so hot un-lowered mods on iOS and the consoles are not stuck at un-optimized interpretation.

The **diagnostics and provenance schema of Part 2** must land in the wire format now: the span table, the
binder-site table, the per-relation provenance arrays, the violation seeds, the residual's optional debug
section, and the provenance-complete engine C ABI (`violations()` plus `provenance_of()`). Retrofitting
provenance onto a bitmask analysis is miserable, so this is a day-one format decision, and the reconstruction
query is generated into both loci by the same generator as the forward engine.

The **engine-identity assurance level** must be set: for the default two-projection variant the differential
harness must be a merge gate (not optional), because by-construction over two source backends is deterministic
divergence, not agreement; the single-shared-object variant is required only if a formal identity guarantee is
demanded, and it is now diagnostics-capable via the provenance-complete ABI.

The **bench forks the design legitimately rests on** must run before any format or commitment lock, per bench-in-
harness discipline: the e-graph versus the graded-unfold-plus-hash-cons pipeline on census workloads; the native
tier versus the tail-call interpreter on real ikiuni behavior scripts (PEP 744's evidence that CPython's own
copy-and-patch JIT is currently only interpreter-speed suggests this may return marginal, which re-weights how
much stencil-TCB budget is worth paying up front); the 16-versus-24-byte record on real arity distributions from
call-heavy behavior scripts; lease-on-engine versus the fused walk on the common tree case; and the avoidance-
failure rate plus the live-binder-width distribution in one instrumentation pass.

## Grounding and sources

Two primary sources were fetched and verified against the clusters' citations. [PEP 744, JIT Compilation
(CPython 3.13 copy-and-patch)](https://peps.python.org/pep-0744/) confirms B's W^X claim ("at no point is the
data both writable and executable"), the clang-required-for-`musttail` claim, the macOS hardened-runtime JIT
entitlement, and LLVM used for object parsing and disassembly; it also states CPython's own JIT is currently
"about as fast as the existing specializing interpreter," which qualifies B's performance justification for the
native tier. The [Zig 0.14.0 release notes](https://ziglang.org/download/0.14.0/release-notes.html) confirm the
`CallingConvention` rework to an exhaustive tagged union with no `preserve_none`, `ghccc`, or caller-preserves-
nothing convention, which lands B's flagged dispatch risk as unmet on stock Zig today. WebSearch budget was
exhausted for this session, so the following claims are taken from the clusters' own cited sources and my
knowledge of them rather than re-fetched, and are flagged as such: the egglog PLDI 2023 semi-naive-plus-
congruence unification (A's single-engine keystone, consistent with the paper), the Copy-and-Patch OOPSLA 2021
and Deegen arXiv:2411.11469 performance figures (B's citations, consistent with those papers' abstracts), and
the Datafrog/Polonius borrow-inference-as-Datalog precedent (apt, with A's own caveat that Datafrog is heap-
backed). The two verified fetches are the load-bearing ones for Part 1's B audit and Part 3's impossibility
adjudication; the unverified ones support claims that are corroborative rather than decisive.

## See also

The agenda this answers (topic 2205, problem 6 designed here in full, problems 1 through 8 audited), Cluster A
(`202607202210`) and Cluster B (`202607202230`) audited above, the Carmack reaction (`202607202122`) whose seven
re-tierings are adjudicated in Part 3, the converged shape (topic 2001) and grounded stack (topic 2055) the arc
rests on.
