# Grounding the algorithmic claims (Bart Wronski)

## The judgement in one line

Every headline mechanism in phase three names a real, decades-old database or incremental-computation
result and the round mostly cites it correctly; the actual gaps are one wrong venue, one overclaimed
"novel" that is a fifty-year-old join heuristic in new clothes, one extrapolation past what its cited paper
proves, and a genuinely unsearched corner (the variation seed) that stays genuinely unsearched after I
searched it.

## Stress test of phase three

**The query inversion survives as stated, narrowed exactly as Giesen narrowed it.** Loop inversion over an
enumerable free variable, bucketing 148 task rows once instead of scanning them 132 times, is sound: it
requires the free variable's range known before evaluation (true, `document.rs:19-23` plans before render)
and the predicate reducible to an equality/membership key (true for 3147 of 3148 registry references). The
0.053 ms figure is the query side only; render and IO stay linear, as Giesen already corrected. Nothing in
phase three's numbers needed re-deriving; they check out against the technique's own preconditions.

**Arntzen's nested-iteration dissolution overclaims its novelty, not its correctness.** "A two-relation join
order chosen by comparing two exact integers" is not a new inference about query planning. It is the oldest
hash-join heuristic there is: build the hash table on the smaller relation, probe with the larger
(Goetz Graefe, "Query Evaluation Techniques for Large Databases," *ACM Computing Surveys* 25(2), 1993,
[dl.acm.org/doi/10.1145/152610.152611](https://dl.acm.org/doi/10.1145/152610.152611); the build/probe-side
choice by relation size is standard material in that survey's hash-join section). Selinger's System R
optimizer needs dynamic programming and cost estimation because its inputs are page counts on disk that
change and cannot be read exactly at plan time (Selinger et al., "Access Path Selection in a Relational
Database Management System," SIGMOD 1979,
[dl.acm.org/doi/10.1145/582095.582099](https://dl.acm.org/doi/10.1145/582095.582099)). An in-memory,
immutable, fully-loaded relation has no such uncertainty; that is not a discovery about join planning, it is
the precondition under which the estimation half of Selinger's problem does not arise at all. The
mechanism holds; "removes the entire hard half of classical query planning" (phase-three synthesis,
adjudication 1) states as insight what is actually the absence of the reason the hard half exists.

**The monotone two-level family check is standard abstract interpretation, not underprized, correctly
adopted.** Sound over-approximation with an exact fallback on failure is Cousot and Cousot's original
framing (Cousot & Cousot, "Abstract Interpretation," POPL 1977). Arntzen's Preserve-mask invariant (mask
accumulation and staging must share one predicate over one node) is real and is exactly the kind of
soundness bug abstract-interpretation discipline exists to catch when the abstraction and the concrete
semantics are computed by two different passes instead of one. This is grounding the round should claim
plainly rather than treat as its own finding.

## The claims ledger

**Query inversion / loop inversion.** ESTABLISHED. Database term: hash-join / semi-join batching, and its
application-layer twin, the N+1 antipattern fix (Facebook's DataLoader, Lee Byron, 2015,
[github.com/graphql/dataloader](https://github.com/graphql/dataloader)). Boundary: covers evaluation cost
only, requires an enumerable bound and an indexable predicate; the round's own grammar restriction
(`where` vs `filter`) correctly encodes that boundary.

**Incremental evaluation / dirty set.** ESTABLISHED for the concept, EXTRAPOLATED for the claim as stated.
DBSP (Budiu, Chajed, McSherry, Ryzhyk, Tannen, "DBSP: Automatic Incremental View Maintenance for Rich Query
Languages," VLDB 2023, [vldb.org/pvldb/vol16/p1601-budiu.pdf](https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf))
formalizes a query's derivative as a mechanical circuit transform and treats full recompute as a legitimate
mode when incrementalizing costs more than recomputing. That full-recompute-is-a-mode point is older than
DBSP and belongs first to Blakeley, Larson & Tompa, "Efficiently Updating Materialized Views," SIGMOD 1986.
What DBSP does **not** establish is that a derivative is "derivable from a schema" for an arbitrary system;
it shows the derivative is computable from the *query plan*. This design's plan happens to be schema-shaped
because its queries are typed field reads with no joins, so the extrapolation is reasonable, but it is an
application of DBSP to a restricted case, not a result DBSP itself states. The stratified four-class dirty
set (authored template, row-to-row, derived-namespace, pinned-oracle) is the round's own construction; DBSP
supplies the criterion for *when* to invalidate, not the four-way split of *what kind of input* invalidates,
which is closer to build-system input classification (Bazel's source/generated/config split; Nix's
fixed-output derivations map cleanly onto the pinned-oracle class, as Giesen's synthesis already notes).

**Identity, hashing, interning.** ESTABLISHED, correctly cited. Structural content-addressing: Unison
(unison-lang.org/docs/the-big-idea). Hash-consing with a verified equality check on collision: Filliâtre &
Conchon, "Type-Safe Modular Hash-Consing," ACM Workshop on ML, 2006
([dl.acm.org/doi/10.1145/1159876.1159880](https://dl.acm.org/doi/10.1145/1159876.1159880)), which is
precisely the hash-then-content-verify discipline the round found already shipped at
`hilavitkutin-str/src/interner.rs:81-89`. rowan's green/red split (rust-analyzer) is real and correctly
attributed. The collision arithmetic checks out on recomputation: birthday-approximation collision
probability is n²/2^(w+1); at n≈12,676 nodes and w=32 that is (12,676²)/(2·2^32) ≈ 1.9%, matching the
round's "~2%"; at w=64 it is ≈4.4×10⁻¹², matching "~4e-12". This is standard birthday-bound arithmetic, not
a result requiring a citation beyond the formula itself.

**Memoization soundness.** ESTABLISHED, and Quilez's resolution is the textbook one, not a novel discovery.
A memo key that ignores an open term's free variables is unsound; this is folklore going back to Michie's
original memo functions (1968) and is stated formally as "queries keyed by all their inputs" in every modern
incremental-computation system, e.g. Adapton (Hammer, Phang, Might, Foster, "Adapton: Composable,
Demand-Driven Incremental Computation," PLDI 2014) and in this round's own citation of Salsa. Quilez is
right that Giesen's free-variable-fingerprint (P4) is unnecessary machinery once the key is the node's
*closed* form (substituting bound values at the point the memo is taken); that is the same principle stated
more simply, not a different one. Aaltonen's original `NodeId -> Value` proposal is the unsound version this
literature exists to warn against.

**Deterministic variation seeded by a content hash.** Searched directly (procedural generation + deterministic
seeding + hash-based selection in generative text + NLG surface-realization template variation; no
prior-art hit). NOVEL, as claimed, in the specific combination: a content hash of a *call term* selecting
among pre-authored *phrasing variants* to keep re-renders diff-stable under a semantic-no-op. The closest
neighbors: procedural-content-generation hashes a spatial coordinate or entity id to seed a PRNG for a
generated *value* (terrain, loot tables, roguelike flavor text keyed by entity id), not a discrete choice
among hand-authored sentences; NLG surface realizers vary phrasing without any stability contract across
regenerations. Content-defined chunking (rolling-hash-determined stable boundaries in storage
deduplication) is the closest structural analogue outside NLG, and it is a different domain entirely. The
combination stays unclaimed by anything found.

**The snapshot.** ESTABLISHED as a category (versioned flat binary formats with mmap access are decades old),
the specific format choice is sound and correctly argued against the alternatives. SQLite's file format
(fixed header, format-version bytes at defined offsets,
[sqlite.org/fileformat2.html](https://www.sqlite.org/fileformat2.html)) and DWARF's versioning discipline
are the right precedent for "how a version field earns forward compatibility," and Cap'n Proto / FlatBuffers
establish the zero-copy, no-pointer-fixup access pattern the recommendation (A: hand-rolled flat `u32`
tables) declines to adopt as a dependency while still copying their layout discipline. Karis's 0.41 ms
warm-read figure against 13.4 ms parse is consistent with what any of these formats would deliver; nothing
about the 45x figure is specific to the hand-rolled choice over rkyv or FlatBuffers, it is specific to
skipping the parse, which any binary snapshot buys equally. The format choice is a dependency-surface
argument, correctly reasoned, not a performance argument, slightly overstated as one.

## Corrections to numbers and mechanisms

**egg's venue is wrong throughout the round.** Stachowiak cites "egg, PLDI 2021" and Giesen's synthesis
repeats it verbatim in the sources list. The paper is Willsey, Nandi, Wang, Flatt, Tatlock, Panchekha, "egg:
Fast and Extensible Equality Saturation," **POPL 2021** (Distinguished Paper), published as
[dl.acm.org/doi/10.1145/3434304](https://dl.acm.org/doi/10.1145/3434304). It was reprinted in PLDI 2022's
SIGPLAN Research Highlights track, which is where "PLDI" enters the citation trail; that is a reprint venue,
not the paper's origin. Fix the citation before it propagates further; a fourth panellist has now repeated
someone else's uncorrected attribution, the exact failure mode phase four exists to catch.

**egglog and DBSP citations are correct as given.** Zhang, Wang, Flatt, Cao, Zucker, Rosenthal, Tatlock,
Willsey, "Better Together: Unifying Datalog and Equality Saturation," PLDI 2023
([arxiv.org/abs/2304.04332](https://arxiv.org/abs/2304.04332)); Budiu et al., DBSP, VLDB 2023, confirmed
above. Both verify.

**"An immutable loaded registry reads cardinalities instead of estimating them" is real but not a novel
special case of query planning**, per the ledger entry above; it is the absence of the reason cost-based
optimization exists, not a new instance of it. State it as "classical join planning does not apply here,"
not as a finding about classical join planning.

## What the literature offers that this round has not used

**Magic sets.** Bancilhon, Maier, Sagiv, Ullman, "Magic Sets and Other Strange Ways to Implement Logic
Programs," PODS 1986. This is the formal Datalog-family treatment of exactly Karis's move: rewriting a
query so a whole *set* of bindings for a free variable is evaluated together instead of once per binding.
It would give the design a principled way to generalize the `where`/`filter` split to deeper nesting (T7)
beyond the ad hoc two-integer chooser, because magic-sets rewriting composes past two relations where the
hand-rolled chooser stops.

**Blakeley, Larson & Tompa (1986)**, named above, is the earlier and more direct citation than DBSP for "full
recompute is a legitimate mode, not a fallback to be embarrassed by," which the round wants and currently
only sources from DBSP's modern restatement.

## Open provocations for the panellists after me

1. Fix the egg citation (POPL 2021, not PLDI 2021) everywhere it has propagated before it locks into a
   design doc that a future reader trusts without checking.
2. Restate adjudication 1's join-order claim as "classical join planning's hard half does not apply to an
   immutable in-memory relation," not as a discovered special case worth a citation of its own; cite Graefe
   1993 for the underlying build/probe heuristic if a citation is wanted at all.
3. Read magic-sets rewriting against T7 (depth-3+ nested queries) before building a bespoke planner; it may
   be the exact frame Stachowiak's relational-algebra provocation was reaching for and it composes further
   than the two-integer chooser does.
4. The variation-seed novelty verdict stands after a real search; if anyone finds a closer neighbor than
   content-defined chunking or PCG value-seeding, it should overturn this ledger entry specifically, not the
   whole claim.
