# Making the ambitious version work at scale (Brian Karis)

**Date:** 2026-07-19. Every number below was produced on this machine against the live corpus, in release
mode. The harness reproduces Tatarchuk's figures within noise (13.4-14.4 ms parse against her 14.0; 2.87 ms
anchoring against her 2.3; 202 ms general against her 229), which is the check that we measure the same thing
before I disagree about what it means.

## The judgement in one line

The product is not a product: 132 documents evaluating one query under 132 bindings is a group-by, and
computing it once for all observers instead of once per observer costs **0.053 ms against 2.87 ms and is flat
in document count** (132 to 13,200 documents moves the run from 2.97 ms to 3.10 ms), which removes the
document axis from the cost model entirely rather than making it cheaper.

## Making the product cheap

Tatarchuk's diagnosis is right and her fix is one step short. She converts O(documents x corpus) into
O(documents) with a per-column index. The document axis can be removed outright.

The mechanism is loop inversion. `task::where(crates ~ self)` is not 132 queries. It is **one** query whose
one free variable ranges over a set known before any evaluation runs, because `document.rs:19-23` plans before
it renders and the crate set is the plan. So do not iterate documents and scan rows. Iterate rows once, and
for each reference in a reference-typed field, append that row to the bucket the reference names. One pass
over 148 task rows produces **all 132 answers at once**.

| workload | shape | measured |
|---|---|---|
| naive anchoring, 132 docs x 148 task rows | O(D x R) | 2.871 ms |
| naive general, 132 docs x 2686 rows | O(D x C) | 202.3 ms |
| **inverted anchoring, all docs at once** | **O(R x fanout)** | **0.053 ms** |
| inverted general, index build | O(C) | 3.20 ms |
| indexed probe, 132 docs | O(D) | 0.0020 ms |
| indexed probe, 13,200 docs | O(D) | 0.172 ms |

The scaling table separates the two axes the round has been conflating:

```
corpus   1x (  2686 rows)  docs   1x (  132) : 2.97 ms
corpus   1x (  2686 rows)  docs 100x (13200) : 3.10 ms      <- 100x the documents, +4%
corpus 100x (268600 rows)  docs   1x (  132) : 250.7 ms
corpus 100x (268600 rows)  docs 100x (13200) : 230.1 ms     <- 10,000x the product, same cost
```

Document count is free. Only corpus is priced, once, linearly. Tatarchuk is right that "query breadth is a
design decision, not a corpus property", which is exactly why breadth must stop being a cost axis: the
design's success drives it, so pricing it prices the ambition. The registry is the scene, a document is an
observer, and the index is the acceleration structure every observer shares. A virtualised renderer does not
rasterise the scene per camera for the same reason.

**What it requires, stated rather than absorbed.** Inversion needs the free variable's range enumerable
before evaluation, and the predicate to be equality or membership on a typed reference so it can be a bucket
key. Both hold today with room to spare: 3147 of 3148 registry references are two-segment addresses with zero
parens, and `10_syntax_correction.md` types the reference-bearing fields, which makes the key an id rather
than a substring. Neither is guaranteed by anything written down. **Constrain `where` to indexable predicates
in the grammar, not in a comment**, because a computed or range predicate silently drops back to the 202 ms
path with no diagnostic.

## What is resident, what is computed, what is looked at

Measured: 9360 hits across 132 documents is **70.9 rows per document, 2.64 percent of the corpus**. The naive
loop touches 100 percent per document to find 2.6 percent, and that ratio worsens as the corpus grows while
the per-document working set stays flat, because a document is about one crate no matter how many exist.

**Resident**: the flat snapshot (1304 KB for 2686 rows, 21,428 cells) plus a CSR index in three arrays, key
bytes, offsets, and a row-id pool. That is `arvo-sparse`'s `Csr` shape, fixed-capacity, sized from the
measured corpus, failing loudly at the bound. My over-general index over every cell value is 844 KB; typed
reference fields cut the key population from 13,338 distinct cell values to the 831 distinct references that
exist, which is the difference between indexing prose and indexing addresses.

**Computed**: the group-by, once per corpus generation. Not per document, per occurrence, or per target.

**Looked at**: one contiguous posting slice per document. The measured probe is a binary search over the
sorted key table plus an offset subtraction, **zero allocation, no hashing**, 0.0194 ms for all 132 documents.
That path satisfies the guard directly rather than by promise.

## The parse question, once it is on the critical path

Carmack suspected the parse dominated, Tatarchuk confirmed it at 80 percent of the anchoring run, and both
reasoned about the query side anyway. Measured on the other side:

| path | cost |
|---|---|
| cold: `toml_edit` DOM parse | 13.4 ms |
| cold: row-store build | 3.0 ms |
| cold: index build | 3.2 ms |
| **cold total before any query** | **~19.6 ms** |
| warm: snapshot read, zero-copy walk | 0.41 ms |
| warm: CSR probe, 132 docs | 0.019 ms |
| **warm total** | **~0.43 ms** |

**45x, and the parse leaves the critical path.** This is Tatarchuk's T5-C and Giesen's Q3-B, which she
correctly calls one feature ranked as neither the proof fix nor the performance fix. Stronger: with a
persisted snapshot the whole cold pipeline is a cache-miss path, and every argument in this round about memo
tables and fold-level caching concerns the 20 percent of a number already 45x smaller than measured.

The snapshot must carry the index, not just the rows, or the warm run pays 3.2 ms rebuilding what it loaded.
Three flat `u32` arrays serialise and mmap trivially; there is no pointer to fix up.

## Migration: is fact extraction genuinely the expensive half

**No, and the extractor already ships.** Pesce priced arvo at roughly 1100 rows of hand-authoring before a
single query. Measured, for arvo:

| fact class | source | rows | authoring cost |
|---|---|---|---|
| dependency edges | `Cargo.toml` | 34 | zero |
| crate attributes (`#![no_std]`, feature gates) | `lib.rs` | 66 | zero |
| public items | source scan | 341 | zero |
| **derivable with no authoring at all** | | **441** | **zero** |

Roughly 40 percent of the target row count, and not a random 40 percent: it is the class prose restates most.
Measured across arvo's 67 templates: **977 mentions of public items that already exist mechanically, 96
restatements of `no_std` or no-alloc status, 63 dependency sentences, 658 crate-name mentions.** Roughly 1,794
restatements of already-machine-readable facts, against arvo's 2 references today.

The duplication is visible without tooling. `arvo-bits/DESIGN.md.tmpl` states "`#![no_std]`, no alloc" twice
in thirty lines, then gives two different dependency lists ("Depends on `arvo-storage`, `arvo-bits-contracts`,
`arvo-numeric-contracts`. No other workspace deps." and, eight lines later, "Depends on arvo. No other
arvo-family dependencies."). Both are already in `Cargo.toml`. One is wrong. That is the project's thesis
demonstrating itself in the corpus that supposedly has no migration path.

The locating half is shipped: `mockspace/lint-rules/design_doc_source_mismatch.rs` is 286 lines plus a
203-line `type_scanner.rs`, tree-sitter based, and it **already** cross-references DESIGN tables against
source exports as a PUSH_GATE. It answers "does this prose name a type that does not exist". Run it backwards
and it answers "which prose spans restate a fact I can generate". Semi-automatic extraction is that inversion
plus a generator: the tool proposes rows and marks the restating spans, and the human adjudicates rather than
authors. The expensive half was never extraction. It is **schema design for domains with no machine-readable
source**, a smaller and different problem, and the part worth Pesce's probe.

## At 10x and 100x

**At 10x corpus (26,860 rows) the first thing to move is the cold index build**, at 24 ms, and it breaks
nothing because the warm path never pays it. Document count at 10x is free, measured. The first real pressure
is the fixed-capacity arena bound, which is correct behaviour: it fails loudly, sized from a measured corpus.

**At 100x corpus (268,600 rows) the cold index build is 250 ms**, the only term that moved. The fix is that
it is per-file and content-addressed: a per-file postings segment merged at load means a one-file edit
rebuilds one segment, and `spike/` already outgrew a flat directory. Nothing else degrades: 13,200 documents
against a 100x corpus cost the same 230 ms as 132 do, because the document axis is gone.

**What breaks first outside the table**: the 844 KB key population, if the index stays over-general. Indexing
every distinct cell value scales with prose volume, the one input that grows without bound and carries no
queries. Index reference-typed fields only. That is already in `10_syntax_correction.md` and wants stating as
a performance requirement rather than a typing nicety.

## What I could not make work, and the ground I covered

**Nested iteration does not invert in one pass.** The group-by needs the binding's range known before
evaluation. That holds for `self` and any binder over a plan-stage set. It fails when an inner query's binding
comes from an outer row's computed value, which is a join, and a join needs an order chosen. I looked for a
single-pass form and did not find one. This is Tatarchuk's T7 and Stachowiak's relational framing arriving as
a requirement rather than an option: **the first nested query is where a join order must be chosen, and the
operator table has to carry enough for a planner to choose it.** Her T7-C ranking survives my attempt to skip
past it.

**I did not test the no-heap claim under a real implementation.** My harness uses `HashMap` and `Vec`. The
CSR probe is allocation-free as measured and the CSR build is a counting pass plus a fill pass needing no
growth, but "the whole pipeline is no-heap" remains, as Tatarchuk said, a plan and not evidence.

**I did not price schema design for non-Rust domains.** The 441 rows are derivable because arvo is Rust.
`ikiuni`'s 111 templates describe a renderer, and nothing in `Cargo.toml` knows what a froxel is.

## Open provocations for the panellists after me

1. **Constrain `where` to indexable predicates in the grammar.** Equality and membership on typed references
   invert; a computed predicate silently costs 202 ms with no diagnostic. Write the restriction, or write the
   fallback's error message.
2. **Put the index in the snapshot and bench the warm path end to end, including render.** I measured load and
   probe. Nobody has measured a full warm regeneration, and it decides whether anything else here matters.
3. **Run the extractor backwards on arvo.** 441 rows at zero authoring against 1,794 measured restatements. If
   it lands, Pesce's finding 2 is half done, and the remainder is schema design for domains with no
   machine-readable source.
4. **Name the per-file index segment now, not at 100x.** It is the only term that grew in my table, `spike/`
   hits it first, and the loader that reads it is the same one currently wrong by 233 rows through a
   one-level glob.
