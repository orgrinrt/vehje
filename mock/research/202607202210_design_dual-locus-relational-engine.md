# The single dual-locus no_alloc relational-fixpoint engine

**Date:** 2026-07-20
**Phase:** research (Cluster A of the Carmack-rebuttal debate, `202607202205_topic.carmack-rebuttal-full-design-not-retier.md`)
**Scope:** design the one relational-fixpoint engine that serves lease inference, equality-saturation lowering,
and load verification, under vehje's no_std + no_alloc compile crates, the dual-locus constraint (analyses run
in the Rust dev-time compiler for bundled scripts and inside the composed Zig runtime for arriving scripts), and
the three named structural bounds (environment-width, nesting-depth, input-length) as compile-time constants.
**Answers:** problems 1 and 2 of the rebuttal agenda (the engine generator, and load-verify coherence), and
threads problem 7 (dual-locus authoring), because the engine generator is that problem's answer.
**Source design:** topics 2001, 2055, 1513, 1316, 1315; the Carmack reaction `202607202122`.

## Verdict

The single engine is designable and holds at full strength. It is one ahead-of-time-generated, no_alloc,
semi-naive Datalog-with-congruence evaluator (an egglog subset, in the oatlog and Datafrog lineage but generated
by our own tool rather than a proc macro), specialized per language from one declarative relational-program
record into two source projections, a no_std Rust module for the dev-time locus and Zig source for the runtime
locus. Lease inference is transitive reachability closure, a genuine least-fixpoint (this is exactly what
Polonius already computes as Datalog over Datafrog). Equality-saturation lowering is congruence closure plus
rewrite saturation, a genuine least-fixpoint (this is exactly egglog). The one piece that is not a fixpoint is
the structural core of load verification (index-in-range, acyclicity, depth-bound over the untrusted arriving
value-arena); it is a linear typed decode, proved below to have no fixpoint shape, and it is therefore not a
query on the engine but the safe extensional-fact front-end that feeds the engine. Load verification's genuine
fixpoint content (the arriving value's lease/region validity, and the numeric residual over tristate numbers)
runs as queries on the same engine using the same rule set as dev-time lease inference. So the maximal shape is
kept without a single demotion: one engine under the three fixpoint analyses, with the one non-fixpoint piece
placed where every relational engine already puts its untrusted input handling, in the loader that materializes
the extensional database. Nothing is retired to a research track; the structural decode was never an engine job.

## 1. The input schema: relations, rules, and where each analysis lives

The engine is a bounded evaluator of a Datalog program extended with congruence (equality) and lattice-valued
columns, the egglog feature set restricted to a decidable, capacity-bounded fragment. egglog's own contribution
is that Datalog and equality saturation are one system: it supports term rewriting, congruence closure, and
lattice reasoning, all evaluated semi-naively over a differential database, with a proved equivalence between
semi-naive and naive evaluation ([egglog, Zhang et al., PLDI 2023](https://arxiv.org/pdf/2304.04332)). That
unification is the reason all three analyses can be one engine rather than three: they are three programs over
one evaluator, not three evaluators.

The extensional database (EDB, the base facts) is extracted once from the IR (dev-time, bundled scripts) or from
the decoded value-arena (runtime, arriving scripts), and is the same schema in both cases. Ids are `u32` node
indices; tags are small closed enums. The base relations:

- `Node(n, op)`: one fact per IR node or value-arena node, carrying its family operation tag.
- `Child(p, slot, c)`: the child edges of the term, `slot` a small fixed field index.
- `Binder(b, scope)`: a `Let` or `Lambda` binder `b` whose default region is the node `scope`.
- `VarUse(n, b)`: a variable occurrence `n` resolved to binder `b` (produced by the resolve walk).
- `InScope(n, b)`: binder `b` is lexically live at node `n` (the scope chain the resolve walk already threads).
- `EffectOf(n, e)`: the declared effect grade of node `n` (the family/effect classification vehje already has).
- `EffectEdge(a, b)`: an ordering obligation, `a`'s effect must precede `b`'s read (the LMS-style pinned edge).

The intensional database (IDB, the derived relations) is where each analysis lives. All three are Horn rules
`head :- body`, the body a conjunction of atoms (a join), the head an insert (Datalog) or a union (congruence).

### Lease inference is reachability closure, a genuine fixpoint

The reachability qualifier of a value node is the set of binders it can reach; this is the coeffect grade the
converged design settled on (topic 2001 point 5), and it is a transitive closure, the canonical Datalog
least-fixpoint. Borrow and region inference as Datalog is not novel here: Polonius computes Rust's borrow check
as Datalog rules over the [Datafrog](https://github.com/rust-lang/polonius) engine, and lease inference is the
same shape with the aliasing half dropped (topic 1316: a produced vehje value is immutable, so only
region-validity is needed, not exclusive-mutability).

```
Reach(n, b)      :- VarUse(n, b).                       // a use reaches its binder
Reach(p, b)      :- Child(p, _, c), Reach(c, b).        // a value reaches what its children reach
Escapes(n, b)    :- Reach(n, b), OutlivesRegion(n, b).  // reaches b past b's default region
Placeable(n, b)  :- Escapes(n, b), WiderLeaseExists(n, b).
LeaseResidual(n, b) :- Escapes(n, b), !Placeable(n, b). // the avoidance-boundary residual
```

`Reach` is stored as a lattice column, one bitmask per node over the in-scope binder set, with merge = bitwise
OR; that collapses the closure into the per-node reachability bitmask the converged design wants, and it is the
same AccessSet bitmask machinery the family and effect axes already run on. `LeaseResidual` marks exactly the
references inference cannot place; those become the per-reference generational check at the avoidance boundary
(Vale's mechanism, demoted to the dynamic residual, topic 2001 point 5). The closure is monotone, its height is
the binder-set width, so it converges in at most environment-width rounds. This is a fixpoint by construction.

### Equality-saturation lowering is congruence plus rewrite saturation, a genuine fixpoint

The compile stage (macro expansion, const-fold, CSE, lowering) is one graded equality saturation over a bounded
e-graph, per the grounded correction (topic 2055): binding time prunes the graph as a typing constraint so
ill-staged terms are unrepresentable, partial evaluation is a well-founded graded unfold that terminates by
binding-time grade well-foundedness (not by a saturation cap), and equality saturation runs over the confluent
optimisation identities. The e-graph is the congruence relation; rewrites are unions; extraction respects the
effect edges.

```
Union(id, x)     :- ENode(id, Add, x, z), ENode(z, Zero).   // (add x 0) = x, a rewrite
Eq(a, b)         :- ENode(a, op, xs), ENode(b, op, ys), EqAll(xs, ys). // congruence
Fold(id, k)      :- ENode(id, op, args), AllConst(args), Eval(op, args, k). // const-fold
```

Binders are represented with slots so alpha-equivalence and beta are congruence-native
([slotted e-graphs, Steuwer group, PLDI 2025](https://steuwer.info/files/publications/2025/PLDI-Slotted-E-Graphs.pdf)),
and context-conditioned (graded) rewrites use colors
([colored e-graphs, Singher and Itzhaky](https://arxiv.org/abs/2305.19203)). Saturation is the least fixpoint of
the rule set over the e-graph; egglog proves this is semi-naive-evaluable. This is a fixpoint by construction.

### Load verification is a decode that feeds the engine, plus two residual fixpoints

The structural core of load verification is not a fixpoint; section 5 proves it. What runs on the engine is the
residual: the arriving value's lease/region validity (re-run the reachability rules over the decoded arena's
EDB), and the numeric residual over tristate numbers for bounds the decode could not settle statically.

```
RegionViolation(n) :- Reach(n, b), !RegionOpen(n, b).   // arriving value: same rule set, arriving EDB
Num(n) : Tnum      // lattice column, merge = tnum meet; iterate for the numeric residual
```

The arriving-value lease check is literally the dev-time lease rules run over a different EDB (the untrusted
decoded arena instead of the trusted IR). One rule set, two extensional databases. The numeric residual is a
monotone fixpoint over the tnum lattice
([Vishwanathan et al., CGO 2022](https://arxiv.org/abs/2105.05398)), retained only for the numeric part the
typed decode leaves open. Both are fixpoints; both are queries on the engine.

So of the three named analyses, two are wholly fixpoints (lease, eqsat), and the third (load verification) is a
composite: a non-fixpoint linear decode that materializes the EDB safely, plus fixpoint residuals that are
queries. The single-engine claim covers everything that is genuinely a fixpoint, which is lease inference, eqsat
lowering, and the load residuals.

## 2. The no_alloc data structures for semi-naive evaluation

Three compile-time constants size everything: environment-width `W` (max live binders in any scope),
nesting-depth `D` (max lexical or value-tree depth), input-length `N` (max nodes in a script IR or arriving
value-arena). These are the three genuine bounds the grounded stack settled on (topic 2055, the A9 resolution),
each doing real work rather than the retired one-cap pun.

Semi-naive evaluation needs, per relation, three generations: `stable` (the accumulated closure so far), `delta`
(the tuples derived in the previous round), and `new` (the tuples being derived this round). A rule fires only on
joins where at least one body atom is bound to `delta`, which is the delta-rule rewriting the generator performs
ahead of time; this is Datafrog's `Variable` (recent, stable, to_add) and egglog's differential database. The
load-bearing memory fact: each tuple is derived at most once across the whole run (monotone derivation), so the
sum of all insertions into `new` over the entire fixpoint is bounded by the relation's capacity, and the three
generations together never exceed a small constant times that capacity. No per-round accumulation, no growth with
iteration count.

The concrete structures, all fixed-capacity, all no_alloc:

- **Relation table** `Rel<ARITY, CAP>`: `{ tuples: [[u32; ARITY]; CAP], len: u32 }`, kept sorted by the join key
  order for leapfrog iteration. This is Datafrog's `Relation` (a sorted immutable array) made fixed-capacity.
  Each dynamic relation holds three of these (stable, delta, new).
- **Frontier / worklist**: the `new` table is the frontier for pure Datalog rules; no separate worklist. For
  congruence rebuild, a `dirty` ring of e-class ids to re-canonicalize, `[Id; NODE_CAP]`.
- **Join index**: leapfrog triejoin drives synchronized trie-iterators over the sorted tables without
  materializing intermediates ([Veldhuizen, ICDT 2014](https://openproceedings.org/ICDT/2014/paper_13.pdf)); the
  index is the sort order, and a trie-iterator is a cursor (offset plus level bounds) with no extra storage.
  Where the rule set joins a relation on more than one key order, the generator materializes one sorted copy per
  required order (the number of orders is known ahead of time from the rules, the same index selection Souffle
  and oatlog do), each sized `CAP`.
- **Union-find** (the `Eq` relation, congruence and lease equality): `parent: [Id; NODE_CAP]`,
  `rank: [u8; NODE_CAP]`, path compression in place.
- **Hash-cons / congruence index**: open-addressing table
  `[Option<(OpTag, [Id; MAXAR], Id)>; HCONS_CAP]` mapping (op, canonical children) to e-class id, `HCONS_CAP` a
  small multiple of `NODE_CAP`, `MAXAR` the max op arity (a small closed constant).
- **Lattice columns** (reachability bitmask, tnum): `Reach: [BitSet<W>; N]`, `Num: [Tnum; N]`, indexed directly
  by node id, merged in place. `BitSet<W>` is `ceil(W/64)` u64 words.

Capacity derivations from the three bounds:

| Structure | Capacity | Derivation |
|---|---|---|
| `Node`, EDB | `N` | one per node |
| `Child`, EDB | `N * MAXAR` | at most `MAXAR` children per node |
| `Reach(n, b)` relation | `N * W` | a node reaches only in-scope binders, width `<= W` |
| `Reach` lattice column | `N * ceil(W/64)` words | one bitmask of width `W` per node |
| `ENode` / e-graph | `WINDOW * SAT` | bounded region window times a saturation blow-up cap |
| union-find, hashcons | `NODE_CAP`, `c * NODE_CAP` | one slot per e-node |
| `Num` tnum column | `N` | one tnum per numeric node |

The reachability bound is the key one: a value can only reach binders that were live where it was constructed,
and that set has width at most `W`, so `Reach` is bounded by `N * W` tuples (or `N` bitmasks of width `W`),
independent of `N * N`. This is what makes reachability closure no_alloc-tractable. The e-graph is the one
structure that can grow past `N` under saturation; it is bounded by the streaming-window discipline (process a
bounded window of `WINDOW` nodes, allow a saturation factor `SAT`, and when saturation would exceed `NODE_CAP`,
stop, extract, advance the window). Determinism does not depend on the cap: the confluent static fragment (the
graded partial-evaluation unfold) terminates by grade well-foundedness before any cap bites, and only the
best-effort algebraic-identity saturation runs against the cap.

Termination and cost: semi-naive derives each tuple once, so total work is the sum of relation sizes,
`O(N*W + N*MAXAR + NODE_CAP)`, and each join runs in the AGM worst-case-optimal bound
([Atserias-Grohe-Marx](https://arxiv.org/abs/1210.0481)) via leapfrog triejoin. Convergence is bounded by lattice
height: the reachability bitmask has height `W`, so at most `W` rounds; union-find is near-constant. This is the
"cost bounded by AGM plus finite lattice height" claim made concrete, and it is what the three constants buy.

## 3. The generator: build-time codegen, not a proc macro

A proc macro is disqualified for one hard reason: it emits Rust and runs in the Rust compiler, so it does not
reach the Zig runtime locus at all, where per-script analyses for arriving scripts must run (topic 1513). oatlog
is exactly a proc macro emitting heap-using Rust ([oatlog, Gustafsson, Magnusson, Luque Cerpa,
EGRAPHS 2025](https://github.com/oatlog/oatlog)); "oatlog-shaped" therefore cannot mean "use oatlog." It means
build our own ahead-of-time relational-engine generator that emits no_alloc code into both loci. That is a real
compiler project, and per the maximal-shape mandate it is the work, not a disqualifier.

The generator is one instance of **runtime generation** (topic 1513): the language compiler, having proved the
language's relational program well-formed, emits the specialized engine as part of composing the runtime. It is a
build-time codegen tool in the Rust dev-time layer, run once per language, never shipped, and it emits no
interpreter, compiling the rule theory away exactly as oatlog compiles egglog to specialized Rust and Souffle
compiles Datalog to specialized C++. This is vehje's certified-generation thesis applied to its own compile stage
(topic 2055: the metacompiler metacompiles its own compile stage).

Input description, a **relational-program record** (data, authored once per language as the compile-stage half of
the language's family/effect/lease disciplines):

- relation signatures (name, arity, column types, which columns are lattice-valued and with which merge),
- the rule set (each rule a head plus a conjunction of body atoms with variable positions, egglog-subset),
- the three structural bounds `W`, `D`, `N` and the derived capacities from the table above,
- the index-order requirements (computed by the generator from the rule set, Souffle and oatlog style),
- the stratification and evaluation order (lease closure, then eqsat, then load residuals, or a declared
  interleaving).

Two outputs, the lens projections (topic 2001 point 3):

1. **A no_std no_alloc Rust module** for the dev-time locus. Specialized semi-naive loops over the fixed-capacity
   structures of section 2, monomorphized to the exact relations, rules, and capacities. Compiled into the
   dev-time language compiler, it runs the analyses on bundled scripts and carries the rich diagnostics layer
   (spans, binder provenance) because the compiler's error surface lives in Rust.
2. **Zig source** for the runtime locus. The same specialized loops emitted as Zig, compiled into the composed
   runtime, running the load-verify residuals (and, if arriving scripts get their own lowering stage, the eqsat
   engine) inside the runtime with no compiler present.

Both outputs are deterministic pure functions of the one input record. The generator is the single trusted
producer; the record is the single source. The part with no prior art is not any one ingredient (Datafrog's
structures, Souffle and oatlog AOT specialization, egglog's Datalog-plus-congruence unification all ship) but the
no_alloc, dual-language-projection generator emitting one relational program into both a Rust and a Zig locus.

## 4. Dual-locus identity: one description, two projections, proved to agree

The two emitted engines must be provably the same engine. Three assurance layers, matching the design's
assurance-graded spine (topic 2055 item 2), from strongest guarantee to cheapest-that-always-exists:

**By construction (high grade).** Both outputs are deterministic projections of one content-addressed input
record; neither side serializes the other. A manifest binds record-hash to (Rust-source-hash, Zig-source-hash),
and the manifest is checked at runtime-generation time. This is the lens principle plus proof-carrying on the
data blob (topic 2001 point 3), applied to the engine rather than only to the family/effect typestate. There is
one description, projected twice by one generator; the projection function is shared, so the two engines compute
the same relational closure by the generator's determinism.

**By differential test (grade-0 shadow).** A differential-test harness, itself generated from the same record
(the Deegen-style triple: the two engine projections plus a test driver), runs both engines on a shared corpus
(census scripts plus generated adversarial inputs) and asserts identical relation-closure outputs. This is the
operational proof that the two projections compute the same function, and the assurance-indexed framing makes it
the same object as the by-construction proof at a lower grade (topic 2055 item 2). It always exists on day one,
which is what the Carmack reaction correctly notes about differential testing from one definition.

**Single shared object (assurance-maximal fallback).** Generate the engine kernel once into a language-neutral
C-ABI form (emit Zig, compile to a no_alloc staticlib at runtime-generation time), and have both loci link the
same object: the composed Zig runtime uses it natively, the dev-time Rust compiler FFI-links it. Identity is then
bit-identical, not tested. The cost is that the dev-time diagnostics layer wraps an opaque kernel and must
reconstruct provenance from the kernel's marked-output relations rather than from Rust-native structures. This is
the variant to reach for if differential drift ever appears or a formal identity guarantee is demanded; the
default keeps the two projections so dev-time diagnostics stay rich.

The load-bearing point for the rebuttal's problem 7 (dual-locus authoring): the engine generator is the "written
once, emitted into both loci" answer. The lease, effect, and family analyses are not hand-written twice; they are
written once as the relational-program record and emitted into both loci by the same generator that answers
problem 1. One description, one generator, both loci, and the same mechanism discharges problems 1 and 7
together. That is the maximal-shape win the mandate asked for: the unification is not vocabulary bolted over three
hand-written analyses, it is one generated engine.

## 5. Load-verify coherence: the decode is not a query, the residuals are

The grounded stack contradicted itself: the A6 resolution re-derived the load check as a typed structural decode,
"complete and linear, not an incomplete abstract interpretation" (topic 2055), yet the same document's summary
listed load verification as one of three queries on the monotone fixpoint engine. This resolves definitively:
**the structural decode is not a query on the engine; it is the safe extensional-fact front-end that feeds the
engine.** The load residuals (arriving-value lease validity, numeric residual) are queries.

**Proof that the structural decode is not a fixpoint.** A fixpoint computation iterates a monotone operator to its
least fixed point; its defining property is a derived set that grows across rounds until closure,
`X = X union f(X)`. The structural decode establishes three properties over the untrusted arena, and none has
that shape. Index-in-range (`child_index < arena_size` for every edge) is a per-record comparison, a map over the
records, not a closure. Acyclicity and depth-bound follow for free from the emission discipline: children are
emitted before parents (topic 1315), so a child index is strictly less than its parent index by construction, the
child graph is a DAG by a single monotone-index check, and depth is a running counter bounded by `D`. There is no
join, no recursion, no derived relation that grows. It is a fold, not a fixpoint. This is precisely
parse-don't-validate ([Alexis King, 2019]) over a range-typed flat buffer: an index has type `Fin(arena_size)`,
and the decode is a complete, linear, branch-predictable pass, the simdjson lineage
([Langdale and Lemire, 2019]) and the rkyv/Cap'n Proto validated-archive lineage. Casting it as a fixpoint query
would lose exactly the completeness, linearity, and SIMD-friendliness that make it correct.

**Consequence for the engine's remit.** The engine's query list on the load path loses the structural entry and
keeps two: the arriving-value lease/region validity (the `Reach` closure and `RegionViolation` rule run over the
decoded arena's EDB, the same rule set as dev-time lease inference), and the numeric residual (the `Num` tnum
column iterated to fixpoint for the bounds the decode left open). Both are genuine monotone fixpoints, so both are
legitimate queries.

**Re-justifying the engine on the narrowed remit.** The engine is not justified by load verification alone; it is
justified by lease inference (fixpoint), eqsat lowering (fixpoint), and the two load residuals (fixpoints). The
structural decode being a linear typed decode does not shrink the engine's justification, because that decode was
never an engine job in any relational system: every Datalog engine has a trusted front-end that reads input into
the EDB, and parse-don't-validate over untrusted bytes is exactly that front-end for the arriving-script case.
Placing the decode in the loader is not choosing a smaller architecture; it is putting the one non-fixpoint piece
where relational engines always put untrusted-input handling. The single engine stands over all three fixpoint
analyses, and the loader feeds it.

This also settles the three-mechanisms observation from the grounded stack's two-overreaches section (topic
2055): structural decode (loader, not engine), numeric residual (engine query), and lease residual (engine query)
are three mechanisms for three properties, and the split is clean once the decode is recognized as EDB
materialization rather than a fixpoint query.

## What remains genuinely original work

Every ingredient ships somewhere; the composition is unbuilt. The genuine original work concentrates here, and
this is where the honest-keeper pass should attack.

1. The no_alloc, fixed-capacity, generated (not proc-macro) semi-naive Datalog-with-congruence engine emitting
   one relational program into two language loci (Rust, Zig) from one data record. Datafrog's sorted-array
   relations, Souffle and oatlog AOT specialization, and egglog's unification all ship, but on the heap and into
   one language; the no_alloc dual-projection generator does not exist.
2. The bounded streaming slotted-and-colored e-graph with the graded well-founded partial-evaluation unfold,
   sized by `W`/`D`/`N` with a saturation cap and window discipline. Slotted and colored e-graphs ship on the
   heap; the no_alloc bounded-window variant with binders and graded conditions is genuine systems work.
3. The reachability qualifier as a lattice column of width `W` (one bitmask per node), with the closure bounded
   to `W` rounds and `N*W` tuples, and the avoidance residual (`LeaseResidual`) marked as engine output.
   Reachability types have not been done no_alloc, nor as a lattice column, nor in a production runtime (topic
   2001 named this as original-work item 3).
4. The load-verify recast: the typed decode materializes the EDB, and the arriving-value lease residual is the
   dev-time lease rule set run over the untrusted decoded EDB. One rule set, two extensional databases (trusted
   IR, untrusted arena), is a small but real novelty in how a certified-generation runtime re-uses its own
   analysis at load time.
5. The dual-locus identity mechanism: the manifest-checked two-projection lens plus a differential harness
   generated from the same record, with the single-shared-object variant as the assurance-maximal fallback. A
   generated engine proved the same across a Rust dev-time locus and a Zig runtime locus is not something a
   shipping system does.

## Open questions for the runtime-design and honest-keeper passes

- **How much of the engine ships.** Does the composed runtime run full eqsat lowering on arriving scripts (so the
  bounded streaming e-graph and its `NODE_CAP` ship in the Zig runtime), or only the load-verify residuals (so
  the runtime engine is the much smaller residual-check subset and eqsat is dev-time only)? This decides the
  shipped runtime's size and is the Cluster A/B seam. Cluster B (the interpreter hot loop) must cost the residual
  engine's per-load pass against the per-frame budget for ikiuni.
- **Overflow policy per capacity.** The `W`-width bitmask overflow (a script whose live-binder set exceeds `W`),
  the `NODE_CAP` saturation-cap hit, and any relation `CAP` overflow each need a stated policy: hard error under
  the strict posture, or demotion of the overflowing references to the generational residual as if avoidance
  failed. The answer changes the width chosen and the failure mode modders see (rebuttal problem 8), and the
  avoidance-rate experiment should measure the live-binder-width distribution in the same instrumentation pass.
- **Provenance carriage.** The marked-output relations (`LeaseResidual`, `RegionViolation`) must carry spans and
  binder ids so the dev-time Rust diagnostics layer produces good errors, and the Zig-locus residual engine must
  carry enough provenance for arriving-script errors (rebuttal problem 6). Retrofitting provenance onto a bitmask
  analysis is miserable, so the schema must carry it from the start; this interacts with the single-shared-object
  identity variant, which hides Rust-native structures.
- **Identity assurance level.** Is the manifest check enough, or must the differential harness be a merge gate,
  or is the single-shared-object variant required for a formal guarantee? This is the honest-keeper pass's call
  and it sets how much the by-construction claim can be trusted without the operational shadow.
- **Index selection and stratification tightness.** Are the join key orders and the stratum evaluation order fully
  determined by the generator from the rule set, and are the derived capacities tight rather than
  over-provisioned (no_alloc memory is paid up front, so slack is shipped cost)? The AARA potential framing was
  correctly kept off the critical path (the rebuttal keeps the three bounds as constants), but the honest-keeper
  pass should confirm the sizing derivations in section 2 are tight, not merely safe.
- **The decode-to-EDB boundary as a trusted base member.** The structural decode is trusted (it produces the
  range-typed facts the engine assumes); it belongs in the named trusted computing base alongside the stencil
  compiler, the patcher, and the load verifier. Its correctness (that a decoded fact really is `Fin(arena_size)`)
  is the assumption the engine's soundness rests on for arriving scripts, and it should be stated as such.

## See also

The rebuttal agenda this answers (topic 2205, problems 1, 2, 7); the grounded stack (topic 2055) and converged
shape (topic 2001) it grounds; the Carmack reaction (`202607202122`) whose break 2 (load-verify contradiction)
and break 1 (no_alloc dual-locus engine) are resolved here; the grounding topics 1513 (three codegens, the
runtime-generation locus), 1316 (lease axis, the reachability rules), 1315 (value transfer, the arena the decode
reads).
