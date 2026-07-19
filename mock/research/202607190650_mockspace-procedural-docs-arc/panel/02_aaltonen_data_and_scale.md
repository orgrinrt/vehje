# The design through the data-layout and scale lens (Sebastian Aaltonen)

## Verdict in one line

The arena is the right memory model and the interner is real and shippable, but the design counts nodes and
never counts occurrences, and at the corpus that already exists (280K words, 2676 registry rows, 1082 live
references) it is occurrence count, not node count, that will dominate every number that matters.

## Where I agree and disagree with Muratori

Agree on findings 1 through 3: the `~` substring bug, missing absence, and forbidden inline iteration are all
real and all cheap to fix. I disagree with how finding 4 is framed. Muratori treats the span-table cost as a
diagnostics tax paid for total desugaring. It is also a data-layout fact independent of diagnostics: a span
keyed to use-site rather than to node (which his own fix correctly requires) means the span table's size
tracks reference count, not tree size, and reference count is the thing hash-consing was supposed to shrink.
I extend his open question into finding 4 below with real numbers instead of leaving it open.

I disagree harder with the "novel angle" about hash-consing turning variation-seed churn into "a number,
computed in nanoseconds." Root-id comparison catches whether a query's own arguments changed. It says nothing
about whether the query's *answer* changed, because the answer depends on live registry state that the node
identity does not encode. Two runs with byte-identical arena content over a registry where one `task` row
gained a `blocked_by` entry produce the same root id and different rendered tables. The mechanism he is
praising detects source churn, not output churn, and the two are exactly what finding 3 below turns on.

## What is strong

- **The interner is not vaporware.** `hilavitkutin-str/src/interner.rs:14-19` already ships `ArenaInterner` as
  a trait the host implements, with `StringInterner<A>` doing const-table-first lookup and falling back to a
  caller-supplied arena (`interner.rs:40-49`). This is a real answer to prior art's flat "no interner surveyed
  is `alloc`-free" (`prior_art/07:146,169`): the backing store is BYO, so a `heapless`-style fixed buffer slots
  in without touching the interning logic. The design can build the node hash-cons table on the identical
  shape and inherit this, rather than inventing a second interner from scratch.
- **`resolve.rs` has already hit and fixed the exact failure mode this panel worries about.** Line 741-742:
  "build a view over those rather than cloning the whole registry per row. At the row counts a real registry
  reaches, the whole-registry clone was quadratic." That is a documented, shipped, already-repaired O(rows²)
  bug in the current mechanism, at the scale the registries are at today (2676 rows). It is direct evidence
  that the naive shape of this exact problem (recursive resolution over a shared store) blows up at THIS
  corpus size, not at some hypothetical future one, and it is the strongest argument for taking the arena's
  DAG-fold memoization seriously up front rather than discovering the quadratic version first.
- **`Contains<M>` composes without a growth-rate surprise.** `hilavitkutin-api/src/access.rs:70`:
  `impl<H, T: AccessSet, M> Contains<M> for Cons<H, T> where T: Contains<M>`. This is linear recursion in list
  depth per membership check, which is fine; the concern below is arity of use, not shape of the recursion.

## Findings

**1. Node count is bounded, occurrence count is not, and the design only budgets the first one.**

Hash-consing on construction means two structurally identical `Project(task, id)` calls collapse to one
node. It does nothing for the number of *places* that node is read from. The current corpus already shows the
gap: 1082 live `{{ ... }}`-shaped references across 445 templates, 548 of them textually unique (a 49%
collapse rate on the crudest possible proxy: raw text, before any real structural equality is even applied).
Every one of those 1082 occurrences needs its own span (per Muratori's finding 4, correctly placed at the use
site), so the span table is sized to 1082 today and grows with every new document that reuses an existing
fragment, which is the entire point of having fragments. The arena shrinks with reuse; the span table, the
per-occurrence diagnostic context, and (finding 3, below) the per-occurrence evaluation cost all grow with
reuse. The design states the first number and is silent on the second.

**Fix.** Track two counts explicitly in whatever cost model gates this design: unique-node count (small, arena-
bounded) and occurrence count (unbounded, corpus-bounded). Size the span table and any per-call bookkeeping to
the second number, not the first, from the first line of the implementation, not as a later correction.

**2. Tokenization granularity is the single largest lever on arena size, and the design does not name it.**

The Inline row includes `Text`, `Space`, `Break` as separate node kinds (`09_shape.md:82`), which is Pandoc's
own model: prose is not one string, it is a sequence of leaf nodes. The current corpus is 280,335 words across
34,736 lines of `.md.tmpl` text. If tokenization is anywhere near word-granular, that is the node-count floor
before a single query is written: on the order of hundreds of thousands of `Text`/`Space` leaves, each a 4-byte
node id, each (per finding 1) needing a span if it came from a parsed document. At 4 bytes per node id plus an
8-byte span pair, a fully word-granular tokenization of the existing corpus alone costs 3 to 4 MB of pure
bookkeeping against 1.9 MB of source text: bookkeeping larger than the payload. A coarser tokenization (one
`Text` node per run between markup boundaries, which is what Pandoc's reader actually does in practice, not
what its node list implies) drops this by an order of magnitude. `09_shape.md` never states which choice the
fold assumes, and the choice changes the arena's dominant cost by 5 to 10x. This is design work owed now, not
an implementation detail to discover later.

**3. Hash-consing gives a DAG. Nothing in the design says the fold visits it as one.**

A shared subtree, once interned, is one node referenced from many parents. Folding that DAG two ways gives
wildly different costs: memoize by node id (visit each unique node once, LMS's actual discipline via
`globalDefsCache`, `prior_art/01:39-49`) or walk the tree structure and re-execute at every occurrence
(Typst's actual discipline, which is why it needs `comemo` as a separate memoization layer bolted on top,
`prior_art/05:118-135`, rather than getting it from the representation). `09_shape.md` claims the interner
buys CSE "as a property of the constructor," which is true for construction and silent on evaluation. The
shipped registry family is a concrete instance of exactly this: `table_cells` (`resolved.rs:186-214`) does an
O(rows x fields) scan building a namespace's full table, and it is invoked fresh, unmemoized, at every call
site that touches a bare namespace or a `where` (`resolve.rs:442-449`). If the arena's `Apply(Project(task,
where), [...])` deduplicates to one node (because the query text is byte-identical across the many crate
documents calling the same fragment) but the fold does not cache by node id, the O(rows x fields) work runs
once per occurrence, not once per unique query, which is the same cost the current unmemoized `table_cells`
already pays today at 148 to 616 rows per namespace. Hash-consing without fold-level memoization is a storage
optimization dressed as a compute optimization.

**Fix.** The fold needs an explicit `NodeId -> Value` memo table, sized to unique nodes (small), checked
before evaluating any node the DAG has already computed. This is not exotic; it is what makes hash-consing
worth doing at all. State it as a requirement, not an implication.

**4. The variation-seed hash and the query-result cache are the same problem wearing two names.**

Decision 4's semantic hash detects when a fragment *call* changed. Finding 3's fold memo detects when a
fragment's *evaluation* can be reused. Both need to invalidate the instant the registry itself changes
underneath an unchanged call, and neither, as specified, does: the semantic hash is over the call term, not
over the registry state the call reads, so a `task::where(crates ~ store)` call with identical arguments and
a changed `task` table hashes identically and would (per the "novel angle" praised in Decision section) read
as "no churn" when the rendered table is in fact different. This is a correctness gap in the caching story,
not a performance one, and it gets more not less important as fold-level memoization (finding 3) is added,
since two independent caches keyed on the wrong invalidation signal compound rather than cancel.

## The complexity table

| Operation | Cost | Term currently small | Term that grows |
|---|---|---|---|
| Node construction (interned) | O(1) amortized, hash lookup | unique-node count (~thousands) | none, by design |
| Whole-registry resolve pass | O(rows + edges), post-fix | 2676 rows, 14 namespaces | rows, as registries accrete |
| `table_cells` per query | O(rows-in-namespace x visible fields) | 616 rows (`reference`), ~8 fields | occurrences x namespace size, if unmemoized (finding 3) |
| Span table | O(occurrences), not O(nodes) | 1082 today | every fragment reuse, unboundedly |
| `Contains<M>` check | O(cons-list depth) | 10 WorkUnit instances measured in hilavitkutin today | family-set arity per document |
| Target x family-set monomorphization | one codegen instance per distinct type combination actually used | 3 target paths (`09_shape.md:114`) | number of distinct family-set combinations documents declare |
| Fold over a DAG, unmemoized | multiplicative in fan-in (each shared node re-evaluated per parent) | low fan-in today (49% raw-text dedup) | fan-in, as fragment reuse is the entire stated purpose |

## What breaks first as this grows

Not the arena. The arena is small, `Copy`, and 4 bytes is genuinely 4 bytes. What breaks first is the fold's
missing memo table (finding 3), because it is invisible at today's scale (few hundred documents, moderate
reuse) and becomes the dominant cost exactly when the design succeeds at its own stated goal: more fragments,
called from more per-crate documents, each call structurally identical and therefore already sharing one
arena node, each occurrence re-walking the same O(rows) registry scan because nothing said not to. The second
thing to break is the span table (finding 1), for the identical reason: it is sized by occurrence, fragments
are meant to multiply occurrences, and nobody has stated a bound.

## Novel angles worth stealing

**Steal the hilavitkutin AccessSet reuse, but bring its own scaling evidence with it.** The design correctly
notices `Contains`/`ContainsAll` is the identical mechanism for family sets that it already is for WorkUnit
store access (`08_decisions.md:69-70`). Before trusting it at whatever family-set arity documents end up
declaring, measure hilavitkutin's own compile time at its current 10 `impl WorkUnit` instances against a
synthetic bench with 50 and 200, because the reused trait shape has never been arity-tested in the codebase
it is borrowed from and there is no in-house evidence it stays cheap past a handful of set elements. This is a
bench decision (per the workspace's own standing rule for algorithm-branch forks), and it is one bench, not a
research question, and the answer transfers to both consumers of the mechanism at once.

## Open questions for later panellists

- **Whether the fold memoizes by NodeId or walks structurally.** This is not a style choice, it is the
  difference between the interner's headline claim being true or being storage-only. Someone has to pick, and
  it changes both the cost model and the API contract for a `Rows` value produced by a `Project`/`Apply` chain
  shared across many `Iter`/`Seq` parents.
- **Whether tokenization is word-granular, run-granular, or line-granular.** Names an order-of-magnitude
  decision the shape document currently leaves implicit in a node-kind list rather than stating outright.
- **Whether the semantic-hash cache and the fold's evaluation memo share one invalidation key or two.**
  Finding 4's gap gets worse, not better, once both caches exist independently; whichever panellist owns
  maintainability should weigh in on whether two independently-invalidated caches over the same registry are
  a maintenance hazard on their own terms, separate from the correctness gap named here.
