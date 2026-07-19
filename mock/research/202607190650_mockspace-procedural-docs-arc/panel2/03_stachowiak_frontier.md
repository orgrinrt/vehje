# What this design is a generation behind on (Tomasz Stachowiak)

## The judgement in one line

The round built its toolbox from compilers, config languages and document converters and never once opened
the incremental-computation, e-graph or content-addressing literature, and that absence is not decorative:
three of the round's own open questions (the memo key, the variation seed, whether the query side composes)
are exactly the questions that body of work already answered, and the panel's best proposals (Giesen's P4,
Quilez's summary lanes) arrived at roughly the right shape by independent rediscovery, which is the tell.

## The frontier the survey missed

**e-graphs and equality saturation.** [egg](https://dl.acm.org/doi/10.1145/3434304) (Willsey et al., PLDI
2021) formalizes exactly what Quilez's `{fam, hash, free, stage}` summary is doing under a different name:
an **e-class analysis**, domain data attached to each node and combined bottom-up during construction,
required to form a join-semilattice (commutative, associative, idempotent merge). Quilez invented the
mechanism; egg supplies the law it must obey and a checklist to verify each lane against before shipping.
Cost: egg itself is heap-heavy (`HashMap`, union-find with path compression) and unusable in the extractable
crates; only the formal pattern transfers, not the runtime.

**egglog** ([Zhang et al., PLDI 2023](https://arxiv.org/pdf/2304.04332)) unifies Datalog and equality
saturation by treating e-matching as a relational join. This is the missing frame for the entire query
side (below): `where`/`select`/`count` over typed rows is Datalog with syntax sugar, and pattern search over
the arena is structurally the same join. Cost: egglog's own runtime is GC'd; the transferable part is
treating the operator table as a schema for relational algebra, compiled ahead of time.

**Incremental view maintenance / differential dataflow.** [McSherry et al.'s Differential Dataflow](https://arxiv.org/pdf/2004.05297)
lineage and [DBSP](https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf) (Budiu et al., VLDB 2023) formalize
Carmack's claim that the docs tree is a materialized view and give it a cost model: a query's *derivative*
with respect to an input change, computed statically from the query plan, not measured with a stopwatch.
Cost: both are Rust but heap-heavy (indexed traces, spines); not adoptable directly, only as the criterion
for when to go incremental at all.

**Content-addressed code.** [Unison](https://www.unison-lang.org/docs/the-big-idea/) hashes every
definition by its structure recursively, children's hashes folded into the parent's, so identity never
depends on position or encounter order. This is the fix Quilez's slug-hash patch was reaching for without
naming it.

## The document-as-view question, from the incremental-computation literature

Carmack's framing ("the docs directory is a materialized view, git is its change log") is DBSP's own
vocabulary, and DBSP supplies the missing piece: the criterion for going incremental is not a benchmarked
threshold, it is whether the query's *derivative* is nonzero for a given input delta. For this registry the
derivative is computable statically from the schema, not from instrumentation: a template's holes read a
known, typed set of namespaces and fields at parse time (the same read-set Giesen's P3 wants), so which
documents are dirty after a registry edit is knowable before any evaluation runs. That subsumes Carmack's
"one second, benched" threshold with a structural answer: incrementality buys in exactly where the
derivative touches a small fraction of the read-set graph, and that fraction is a schema property, checkable
at the commit gate (Giesen's own Q3-B), not a runtime guess.

One more thing DBSP names that the round missed: full recompute-and-diff is not a fallback the incremental
literature is embarrassed by, it is DBSP's own batch mode, used precisely when the derivative computation
costs more than the recompute. Git-diff-as-review is not "the incremental design nobody built yet" (Carmack's
phrasing); it is the correct DBSP-shaped answer at this corpus size, arrived at from first principles instead
of from the framework.

## The query side, treated as a query language for the first time

Nobody in either panel asked whether `where`/`select`/`count`/`sort` compose. They are relational algebra:
`t::where(P)::select(cols)` is `σ_P(π_cols(T))`. Predicate and projection pushdown are standard rewrites, and
under egglog's framing they are equality-saturation rewrite rules over the operator table, not hand-written
special cases in the fold. This reverses the direction both panels assumed: the round treats the arena as
primary and the registry as a value family contributing to `Apply`; the relational framing treats the
registry's typed columns as the store and the arena as the **query plan** over it. Quilez's own line, "every
static property is a monoid over the operator table," is one step short of its own conclusion: the operator
table for `where`/`select`/`Project` already is a relational schema, and a compiled join over fixed-size
column arrays needs no heap, since row counts are static at commit-gate time.

## Carmack against Quilez on hash-consing, adjudicated from recent work

Carmack: delete the interner, the dedup saving is unmeasurable (535/1066 unique refs, three-token terms).
Quilez: keep it, because the constructor is the one site where five mechanisms unify into a 24-byte
per-node summary. Both are half right. egg's e-class analysis requires only a join-semilattice merge
computed during traversal; it does **not** require sharing or deduplication to deliver that value. The
traversal buys the summary; the interning buys structural sharing, which this corpus does not need. So:
keep Quilez's bottom-up summary (free, and now checkable against egg's law), drop the encounter-order
interning Carmack objects to, and replace node identity with a **Unison-style recursive structural hash**
computed at the same construction site. That collapses Giesen's CallHash and the node id into one value,
makes dedup a free side effect of structural equality (spanning renders, not resetting per run, since a
content hash is stable across processes the way an arena index is not), and fixes the bug Quilez found in
his own proposal: `hash(fragment_slug, arg_slugs)` hashes names, not resolved structure, so two
semantically-identical but differently-spelled argument expressions still seed differently. A structural
hash of the normalized argument ASTs does not have that hole.

## What a 2026 design does that none of the surveyed systems could

Construction-time recursive structural hashing, in a no-heap fixed-capacity arena, doing triple duty as
node identity, memo key, and variation seed, disciplined by e-class-analysis semilattice laws. Nothing
surveyed does all three with one mechanism: rowan's green/red split ([2018](https://github.com/rust-analyzer/rowan))
predates egg's 2021 formalization and never got a merge-law discipline (which is why Giesen's P4 had to be
discovered by a panelist instead of checked against a known rule); Racket has no hash-consing at all; Dhall's
semantic hash is computed once at normalization, not incrementally during construction. A query language
whose join order, family-inclusion proof, and variation seed all fall out of the same bottom-up summary is
not attempted anywhere in the survey, because no surveyed system was built after e-graphs, egglog, and
content-addressed code existed as a joined body of work, no-heap, from scratch.

## Where panels one and two are wrong

Giesen's CallHash "over resolved ids" was wrong for the reason Quilez caught (arena-order ids are not
stable), but the *general* principle neither named is content-addressing's own rule: an identity hash must
be computed from structure recursively, never from an allocator artifact. Carmack's "delete hash-consing"
overstates its case; the traversal he keeps by necessity is already most of what the interning was buying.
Quilez's slug hash is an improvement but still hashes names rather than resolved structure, and inherits the
exact renumbering fragility the round has already been burned by twice. Neither panel asked whether the
query operators compose, which is load-bearing for every future growth path (Q5, Q7) in Giesen's own
synthesis.

## Open provocations for the synthesiser

1. Replace arena-order node ids with recursive structural hashes computed at construction; bench the unique-node count (535/1066) and confirm dedup spans render runs for free.
2. Write the four summary lanes' merge laws explicitly and check each against egg's join-semilattice requirement before calling the result a monoid.
3. Time-box a compiled, fixed-size join for `where`/`select`/`count` under the relational framing against the current `table_cells` scan; measure whether predicate/projection pushdown is a five-line rewrite or a special case, and let that decide which framing to build against.
4. Cost persisting the content-addressed cache across renders, not only within one run; git already gives durable storage, and Nix and Unison ship exactly this at far larger scale.

Sources: [egg, PLDI 2021](https://dl.acm.org/doi/10.1145/3434304); [egglog, PLDI 2023](https://arxiv.org/pdf/2304.04332); [Differential Dataflow](https://arxiv.org/pdf/2004.05297); [DBSP, VLDB 2023](https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf); [Salsa](https://salsa-rs.github.io/salsa/reference/algorithm.html); [Unison, the big idea](https://www.unison-lang.org/docs/the-big-idea/); [rowan](https://github.com/rust-analyzer/rowan); [WASM component model / WIT](https://component-model.bytecodealliance.org/design/wit.html).
