# Making it work: synthesis of the constructive phase (Fabian Giesen)

**Date:** 2026-07-19
**Reads:** the phase-three brief and all three phase-three files, all of phase two, all of phase one
including my own synthesis, and the artefacts `07` through `10`. Every load-bearing claim in the three
phase-three files was re-verified against the corpus and source before this file was written, because this
round's verification record is poor and I am one of the reasons it is poor. Where a claim did not survive
the check, that is stated with the command-level fact that broke it. Where my phase-one work was corrected
by phases two and three, the correction is adopted here without defence.

## The one-paragraph judgement

Phase three did what it was briefed to do: it converted the round's three standing walls into mechanisms
with measured costs, and two of the three conversions hold outright. Karis's loop inversion is correct and
is the round's single largest result, though its headline overstates its own scope by one word: it removes
the document axis from the **query** cost model (0.053 ms against 2.87 ms, flat to 13,200 documents), not
from the render and IO that remain honestly linear in documents. Arntzen's dissolution of the
nested-iteration wall holds, and his two red tests (the Preserve-mask invariant, the registry-row dirty
set) are the most valuable unshipped artefacts in the phase. Grassia's finding that the migration has
already been executed once, at scale, with its receipt in the data, is true and re-prices the round's only
"genuinely blocked" verdict into an engineering schedule, but his own count is wrong in the same direction
he corrected everyone else: the registry-side reference population is not 5761 but **6623** (his two greps
missed 862 cross-namespace row-to-row references, verified by prefix histogram), his 75.7 words-per-row
divides by the wrong denominator (the honest ratio over drained rows is 98.2), and his one declared blocker
(you cannot freeze a live corpus) dissolves against a mechanism the workspace is already sitting inside:
git pins immutable bytes by commit, so a citation into an epoch-tagged blob is verifiable forever while the
working tree moves freely. With that dissolution, nothing in the migration is blocked on design. What
remains genuinely blocked is short and honest: the proof-category split (gate-time is not compile-time, and
the gate claim rests on three operational facts that are policy rather than proof), the human adjudication
schedule (a volume is priced, a rate is not), and the empirical want for phrasing variation. Everything
else on the round's blocked list is merely unexecuted, and this file names each item's cost, what it
forecloses, and who checks it next.

## What I got wrong in phase one, and what it changes

Stated once, plainly, so phase four inherits corrections rather than defences.

**The row count.** My 2443 was exactly a one-level glob (`registry/*/*.toml`), missing 233 rows in
`spike/`'s five subdirectories and 112 references with them. The correct figure is 2676 plus 10 vocab. The
mechanism matters more than the number: any loader, lint, or index written against a one-level glob
inherits both errors silently, and `spike/` is the namespace furthest along the growth curve, so the error
was blind in exactly the direction that matters. Karis's provocation 4 (name the per-file index segment
now, and check the loader for glob depth) is the correct downstream consequence and it stands.

**P6 as a prerequisite.** My nine-repository bootstrap split resolved a cargo cycle that does not exist.
Package identity includes source id; a path-local mockspace and a git-sourced mockspace are distinct nodes,
tested twice (once in the phase-two brief, once independently by Tatarchuk with the real topology). The
split survives only as an optimisation on its own merits: a whole-crate build-dependency to call one
bootstrap function is dishonest sizing regardless of cycles, and the double compile is a real cost across
nine repos. But it gates nothing, and my phase-one Q1 ranking is void. What is genuinely owed from that
episode is one paragraph of workspace policy: the `[patch]` path-override workflow collapses the two
instances and produces a hard cargo refusal (tested), so patching direction must be disciplined before the
first person hits it mid-iteration.

**CallHash over resolved ids.** Specified over interner ids that an encounter-order (or worse,
linker-order) interner assigns, my CallHash would re-roll phrasings corpus-wide on a TOML reorder, which is
the precise failure decision 4 exists to prevent, reintroduced one layer down. Quilez caught it, Stachowiak
generalised it to the content-addressing rule (identity hashes from structure recursively, never from an
allocator artifact), and Tatarchuk stated the sentence the design needed: a recursive structural hash whose
leaves are slug bytes, 64 bits wide, with the hash-to-slot table recognised as the interner it is. Arntzen
then added the piece all three of us owed: the collision discipline on insert (hash then equality), with
in-workspace precedent at `hilavitkutin-str/src/interner.rs:81-89`, verified: the comment reads "by hash,
then by content to rule out 28-bit truncation collisions" and the code checks
`entry.hash == want && str_eq(entry.value, s)`.

What these three corrections change together: my phase-one synthesis's build order is obsolete in its first
step (the bootstrap split is no longer step 1), its identity machinery is replaced wholesale (T2-C per
Tatarchuk, plus Arntzen's insert check), and its cost model is superseded by measurement at every point
where I asserted a magnitude. The parts of phase one that survive are the root-cause analysis (premature
rendering; per-occurrence identity), the monotone two-level check (narrowed by Arntzen below), P3's
provenance-labelled review, P7's author-owned fallback, and P8's reflection, all of which phases two and
three either adopted or sharpened rather than overturned.

## Audit of the three

### Karis (scale)

**Holds, with an excellent verification record where I could re-check.** The inversion mechanism is
correct: `document.rs:19-23` does plan before render, so the free variable's range is known before
evaluation, and one pass over 148 task rows bucketing by reference-typed field genuinely produces all 132
answers at once. The measured table is taken as established per the dispatching agent's confirmation
(0.053 ms inverted anchoring; 3.10 ms at 100x documents; warm snapshot ~0.43 ms against ~19.6 ms cold of
which `toml_edit` is 13.4 ms). The extractor claims verify exactly: `lint-rules/src/
design_doc_source_mismatch.rs` is 286 lines, `lint-rules/src/type_scanner.rs` is 203, and the duplication
he cites in `arvo-bits/DESIGN.md.tmpl` is real and worse than decorative: lines 26 and 31 both state
"`#![no_std]`, no alloc", and lines 28 and 33 carry two contradictory dependency lists ("Depends on
`arvo-storage`, `arvo-bits-contracts`, `arvo-numeric-contracts`" against "Depends on arvo. No other
arvo-family dependencies"), both restating `Cargo.toml`, at most one correctly. The project's thesis
demonstrated in the corpus said to have no migration path, exactly as he framed it.

**Thin.**

- **The headline claim "document count is free" is scoped narrower than its wording.** What was measured is
  the query side. Rendering 13,200 documents and writing 13,200 files is linear in documents with a real
  constant (at even half a millisecond of render and IO per document, 13,200 documents is several seconds
  of wall time that no index removes). The correct statement, and the one phase four should hold him to:
  the inversion removes the document axis from **evaluation**; documents still pay render and IO linearly,
  and nobody has measured a full warm regeneration end to end, which his own provocation 2 admits. The
  inversion's value is untouched by this scoping; the slogan is not.
- **The 441 derivable rows are a claim about Rust repositories.** He states this himself for `ikiuni`'s
  froxels, but the audit should sharpen it: the three derivable classes (dependency edges, crate
  attributes, public items) exist because cargo and rustc are machine-readable oracles. The generalisation
  is not "40 percent of rows are derivable" but "rows whose ground truth is a machine-readable artifact are
  derivable, and Rust repos have three such artifact classes." Other domains have different oracles
  (Grassia's frozen seed is one), and some have none.

**Missed.**

- **His own extractor-backwards output belongs in Grassia's derived layer, which changes the migration
  arithmetic permanently rather than once.** Karis prices the 441 rows as authoring saved at migration
  time. Under Grassia's stored-versus-computed rule (correct, adjudicated below), those rows are never
  authored at all, at migration or ever: they are derived at load like the `crates` namespace, with the
  source tree as ground truth. The saving is not 441 rows of typing once; it is 441 rows of *maintenance*
  forever, plus the drift class (the `arvo-bits` contradiction he found) becoming unrepresentable for that
  fact class. He measured the right thing and filed it under the wrong ledger.
- **The registry-side reference population.** His index sizing and key-population analysis (844 KB
  over-general, 831 distinct references) is computed over the braced reference population. Grassia's
  finding, corrected below, means the resolution pass, the cycle graph, and the snapshot's edge tables face
  roughly 6.6x the reference volume the round had been pricing. None of his conclusions invert (the volumes
  are still trivially small), but the snapshot he specifies must carry resolved row-to-row edges or the
  warm path silently re-pays the resolution he benched away.

**Wrong.** Nothing material beyond the scoping above. His honest section is honest: the nested-iteration
wall he reported was real as he measured it and Arntzen's dissolution does not contradict his measurement,
it adds the inference he stopped short of.

### Arntzen (guarantees)

**Holds.** The Preserve-mask invariant is the sharpest single correctness finding in the phase: under a
`Preserve` policy, a `Select` surviving into emitted Lua keeps both arms as code, so the exact mask must be
the parse mask restricted to the preserved subtree, not the reduced-branch mask, or an unsupported
construct in the untaken arm ships with no refusal. This is a genuine hole in my P5 as I wrote it (I
specified the exact pass over "reachable, reduced content" without conditioning on policy), and his
statement of the fix is the correct one: mask accumulation and the reduce-versus-preserve decision must be
the same predicate over the same node in the same fold. His narrowing of the guarantee's wording also
stands: decision 2's "prove that what a document contains is within what the target declares"
(`08_decisions.md:59-60`) should be restated as proving inclusion of what this target, under its own
staging policy, will actually be handed. I wrote that sentence as a synthesis aside; he is right that it
belongs in the decision's own text, and right about why (the original phrasing is what invited the
three-year misreading).

The hash-then-verify insert discipline holds with verified precedent (above). The boundary-contract section
holds as accounting: three edges, none specified, and the honest claim for the commit gate ("no document
that reaches `dev` through the standard flow is unchecked") correctly names its three operational
dependencies as policy rather than proof.

**Thin.**

- **The citation for "statically known namespace sizes" over-claims.** He cites
  `10_syntax_correction.md:14-16` for namespaces being "fixed tables, known the moment loading finishes."
  Those lines establish that reference-only fields carry typed bare references; they say nothing about
  sizes. The fact he needs is real but differently sourced: the registry is immutable within a run, so
  after load every cardinality (including the cardinality of any intermediate filtered set) is an exact
  known integer, not an estimate. The dissolution survives on the correct ground, and is actually stronger
  there: classical join planning is hard because cardinalities must be *estimated*; here they are *read*.
- **His own headline miscounts his own findings.** "Four of the five reported guarantee failures dissolve
  with a stated invariant; the fifth (nested iteration) dissolves too" does not leave room for the
  proof-category split, which his own section says does **not** dissolve and should be documented as real.
  The body is honest; the one-line judgement is one failure short.
- **The collision-branch cost at 100x is flagged by him and priced by nobody**, including him. It is a
  hash-table probe already paid plus one structural equality on the rare collision path; sub-microsecond
  per insert is almost certainly right, and it should be a number in the implementing CL, not an adjective.

**Missed.**

- **The dirty set's ground truth is not just registry rows.** His red test (edit a row inside a cached
  document's transitive read set, template untouched, confirm the gate re-checks) watches authored TOML.
  Verified against source: the single largest reference population, 2286 `crates::` references, resolves
  against a namespace with **zero authored rows**, whose ground truth is a filesystem directory probe
  (`RESERVED_ROOTS` includes `crates` in `mockspace/src/registry/model.rs`, and resolution is
  `cfg.crates_dir.join(&name); if dir.is_dir()` at `resolve.rs:359-368`). Rename a crate directory and 2286
  references re-resolve with no TOML diff anywhere. Karis's 441 derived rows add source files to the same
  class. His test as written passes while the gate stays blind to the largest live dependency class. The
  stratified dirty set is specified in Building beyond, below.
- **The boundary contract he declares undesigned is the same artefact two other panellists half-specified.**
  Karis's persisted snapshot (rows plus CSR index, three flat `u32` arrays, mmap, no pointer fix-up) and
  Grassia's "the snapshot is the genuine interchange boundary" are, jointly, the design of the
  mockspace-to-language-crates edge he says has no API shape: mockspace owns the `std` side that builds and
  persists the snapshot; the no-alloc crates consume borrowed views over its mapped bytes. He is right that
  nobody wrote it down as a contract; he is wrong that nothing in the round designs it. It needed
  assembling, not inventing, and it is assembled below.

**Wrong.** Nothing found that survives as a material error. This is the cleanest file of the three.

### Grassia (ingest)

**Holds, and the central finding re-prices the round.** Verified directly: `ikiuni_renderer/mock/research/
seed/` is 7 files and 203,414 words (`wc -w`, exact); `mockspace.toml` declares it a reference root with
`frozen = true` and the comment stating the oracle rationale (lines 380-391, matching his 383-389); 2070
rows carry `provenance` by my count against his 2071 (one row, immaterial); the drained-not-moved cursor
(`seed_id`, `visibility = "internal"`) exists in the namespace declarations exactly as described. The
schema-evolution evidence verifies to the digit: 1212 lines, 15 `[[registry.namespace]]`, 144
`[[registry.namespace.field]]` slots, `crates` declared 14 times, and `vocab` declares `note` **twice**
(lines 424 and 448), with the first duplicate's description being row-level content pasted into the schema
("Audio is a lane in passing but composes as reserved cadence..."), which is a stronger form of his finding
than he claimed: not just a fork, but content in the wrong tier, shipped, unnoticed. The `crates`-namespace
facts verify: reserved root, zero authored rows, 2286 references, directory-probe resolution. The
composition section's take/refuse split on USD is correctly reasoned and the three-layer strength ordering
is right (see Adjudications).

**Thin.**

- **The 75.7 words-per-row ratio divides by the wrong denominator, committing the error he corrected in
  Pesce.** 203,414 / 2686 uses *all* rows, but 615 of them carry no provenance and were not drained from
  the seed. The honest extraction ratio is corpus words over drained rows: 203,414 / 2071 = **98.2 words
  per row**. Applied to arvo's 51,486 words that is ~525 rows rather than his ~680, before subtracting the
  derivable class. The direction of his argument survives (the residual hand-adjudicated population is a
  couple of hundred rows, not Pesce's ~1100); the number he shipped repeats the denominator sin one file
  after diagnosing it.
- **The `provenance` mechanism attribution is imprecise.** He states provenance "is `internal = true`,
  dropped on the way out." Verified: no `provenance` field declaration carries a visibility attribute; the
  `visibility = "internal"` flag sits on `seed_id` (15 occurrences). Provenance citations are dropped on
  output by the frozen-root filtering described in the `[ref.roots.seed]` comment ("citations stay in the
  source... they are dropped on the way out. Filtering is per item"). His conclusion is untouched (the
  field is source-facing and semantically taken, so derivation provenance needs its own axis); the
  mechanism he cites for it is the wrong one, and an implementer following his text would look for a flag
  that is not there.

**Missed.**

- **His own count undercounts, by the same class of error he corrected.** His 5761 is two greps
  (`crates::`, `seed::`). The registry's braced reference population is 3148 (matching Quilez's count),
  of which 2286 are `crates::` and **862 are cross-namespace row-to-row references** his greps miss:
  650 `reference::`, 93 `spike::`, 66 `bench::`, 31 `tripwire::`, 7 `constant::`, 5 `ruling::`, 3 `law::`,
  3 `equation::`, 2 `technique::`, 1 `facet::` (verified by prefix histogram). The full registry-side
  population is 3148 braced plus 3475 seed citations = **6623**, and with the 1007 template references the
  corpus total is 7630. The 862 he missed are the most consequential subset for the arguments he was
  making: they are the live row-to-row dependency edges that feed cycle detection and the transitive dirty
  set, the exact populations he told the synthesiser to re-run. The round priced the wrong population by
  6.6x, not 5.7x.
- **The dissolution of his own blocker.** He searched content-addressed spans, diff-tracked cursors, and
  heading citations for a way to let the oracle move under a partial drain, and correctly found each
  weakens the citation check. He did not search the mechanism the whole workspace lives inside: **git
  already stores immutable snapshots addressed by commit**. Declare the extraction oracle as a pinned
  epoch (`pin = "<tag-or-sha>"` on the ref root); a citation resolves against `git show <sha>:<path>`,
  which is immutable forever, line-verifiable forever, while the working tree moves freely. Epoch
  advancement is a deliberate act that diffs the two pinned blobs and re-opens exactly the citations whose
  cited spans changed, which is the incremental re-adjudication frontier his `seed_id` cursor already
  models. `frozen = true` becomes the degenerate case (epoch = forever). Full treatment in Building
  beyond; this dissolves the "single unsolved prerequisite" his file ends on.

**Wrong.** The 5761 figure as a total (undercount, above) and the 75.7 ratio (denominator, above). Both
errors *understate* his own case, which is worth noting: the corrected numbers make the data-side
population larger and the extraction cheaper per row of prose than he claimed.

## Adjudications

**1. Karis's "genuinely blocked" nested iteration against Arntzen's dissolution: Arntzen wins, on
corrected ground, with one boundary.** Karis's measurement was right: an inner binding sourced from an
outer row's computed value is a join, and a join needs an order. Arntzen's inference completes it: within a
run the registry is immutable, so by the time the gate (or the load) plans, every relation's cardinality,
including any intermediate filtered set's, is an exact readable integer, and a two-relation order choice is
a comparison of two integers computed once. The boundary worth stating: this is exact for the two-relation
case, and at depth three or more, join ordering becomes a small dynamic program over exact cardinalities
(still cheap, still deterministic, no estimation, which removes the entire hard half of classical query
planning). The corpus histogram says nesting depth today is zero, so the two-integer chooser covers the
frontier, and a catalogue red test pins the depth-3 case (`catalogue-edge-cases-as-tests.md`). Adopt
Arntzen's recommendation over Tatarchuk's T7-C sequencing on the narrow point: build the two-integer
chooser alongside T1's index, because its absence has a silent quadratic failure mode and its cost is the
lookup T1 already performs. Tatarchuk's T7-C survives for everything else (the operator table carries
algebraic metadata; pushdown waits).

**2. Grassia's data-side population against Karis's inversion and Arntzen's dirty set: all three compose,
under the corrected four-way split.** The 7630 references are not one population with one story. They are
four, each with its own invalidation semantics, and no panellist stated the split:

| population | count | ground truth | dirties |
|---|---|---|---|
| template references | 1007 | authored `.md.tmpl` | the containing document, on file edit |
| row-to-row references | 862 | authored TOML rows | every transitive reader, on row edit |
| `crates::` references | 2286 | filesystem directory listing | every reader, on directory rename/add/remove |
| `seed::` citations | 3475 | pinned immutable corpus | never (verify pin instead) |

Consequences, one per panellist: Karis's snapshot must carry the resolved row-to-row edge list (862 edges
plus the derived-namespace resolutions) or the warm path re-pays resolution; Arntzen's red test must gain a
second case (rename a crate directory, no TOML diff, confirm the gate re-checks the 2286-reference
readership) and a third (advance the seed pin, confirm only citation verification re-runs, no document
dirties); Grassia's "re-run every scale argument" instruction is discharged by observing that 3475 of the
6623 data-side references are frozen-oracle citations that never participate in invalidation, so the live
dependency graph is ~4155 edges (862 + 2286 + 1007), comfortably inside every measured budget.

**3. Grassia's provenance finding against the layer-tag need: adopt his split, with the tag in the
snapshot, not the schema.** `provenance` means source citation, is source-facing, and is dropped on output;
derivation provenance is a different axis. His per-cell layer tag (three values: `derived`,
`schema-default`, `authored`, set by the resolver, never authored) is correct, and the right home falls out
of the snapshot contract below: a two-bit lane per cell in the snapshot's cell table, populated by the
resolution pass, never present in TOML at all. The reader-facing consequence he states is right and cheap:
a hand-edit to a derived cell is refused with a diagnostic naming the owning source. This is the CSS
cascade's origin model and USD's strength ordering at their shared core, which is phase four's citation to
pin.

**4. Karis's "schema design is the expensive half" against Grassia's "it is priced by an executed
instance": both, at different layers, and the residual is named.** Grassia priced extraction volume (98.2
corpus words per drained row, corrected) and demonstrated the schema *shape* (per-domain namespaces plus
shared field vocabulary). What the executed instance also prices, via its own defects, is unsupervised
schema evolution: 51 percent of field slots are copy-paste of five shared fields, already forked in
description five ways, with one duplicated declaration shipped. So schema design is neither unpriced
(Karis) nor free (Grassia's framing tends there): it is priced as *cheap when the shared-field library
exists and expensive in silent drift when it does not*. The library is therefore sequenced before the
`crates` retype (Grassia's provocation 3, adopted). The genuinely unpriced residual, stated for phase
four: human adjudication rate. 98.2 words per row is a volume; nobody has measured minutes per adjudicated
row, and only Pesce's probe, now redefined as the arvo epoch-pinned drain, produces that number.

**5. Arntzen's boundary gap against Karis's snapshot and Grassia's interchange question: one artefact, and
it is the load-bearing one.** Adjudicated: the snapshot is simultaneously Karis's 45x warm path, Grassia's
interchange format, and Arntzen's mockspace-to-language-crates API shape. It must be specified as a
versioned contract, not an implementation convenience. Contents, assembled from the three files plus the
corrections above: the typed cell table (post-resolution, post-derivation), the per-cell layer lane
(adjudication 3), the CSR reference index (Karis, per-file segments per his provocation 4), the resolved
edge lists (adjudication 2), exact cardinalities per namespace and per indexed key (adjudication 1's
chooser input), the structural-hash identity column (Tatarchuk T2-C), per-ground-truth-class fingerprints
(authored-file content hashes; derived-class source fingerprints; the seed pin), a generation counter, and
a version field. Requirements: mmap-able, zero pointer fix-up, borrowed views on the no-alloc side. Format
options are costed in the open questions.

**6. The `where` grammar restriction (Karis's provocation 1): adopted, as two spellings plus a gate
diagnostic.** The inversion holds only for equality or membership on typed keys; a computed predicate
silently costs the 202 ms path. This is the same resolution as Muratori's finding 1 (membership and
substring are two questions, two spellings): `where` accepts indexable predicates only, checked
grammatically; an explicit second spelling (`filter`) carries arbitrary predicates at documented scan cost;
the gate emits a cost note when a `filter` scans a namespace above a stated row threshold. Nothing is
foreclosed, and the fast path cannot be silently lost.

**7. What phase three's silence ratifies.** Tatarchuk's T2-C (4-byte arena index plus 8-byte structural
hash column), T3-A (enum constructors, const attribute table), T4-C (stream for content with the depth
stack load-bearing, trees for expressions), and T6-A-plus-C (brace arbiter plus the `@` sigil) were
disturbed by nothing in phase three and are carried as settled. T5 is upgraded: Karis's measurements
collapse Tatarchuk's "C first, then B" into the single snapshot artefact of adjudication 5, with the index
and identity column aboard from the first version.

## What now works that phase one and two said could not

Each entry states the mechanism, what it now costs, and what it forecloses.

**Conditional content under the family check** (phase one: my three-year-break finding; phase two: Quilez's
single-expression collapse). Works via the monotone two-level check, now with Arntzen's two corrections:
the guarantee is restated as target-relative, and the exact mask under `Preserve` is computed by the same
predicate that decides preservation, in the same fold. Costs: one mask accumulation lane in a fold that
runs anyway; one bitwise AND on the fast path (`Bits<N, Hot>`, per Tatarchuk's width correction). Forecloses:
nothing. Owed before ship: Arntzen's red test 1 (Preserve-target document, unsupported construct in the
untaken arm of a preserved `Select`, must refuse).

**Nested iteration** (phase three's own Karis, dissolved by Arntzen). Works via the two-integer order
chooser over exact load-time cardinalities, built alongside T1. Costs: cardinalities carried in the
snapshot (bytes), one comparison per nested query at plan time. Forecloses: nothing at depth 2; depth 3+
needs the small exact DP, pinned as a catalogue red test until someone writes a depth-3 query.

**Query cost at scale** (phase one: Aaltonen's "first thing to break"; phase two: Carmack's threshold
deferral, Tatarchuk's product warning). Works via inversion plus the unconditional index: the query side is
flat in documents and linear once in corpus. Costs: the grammar restriction of adjudication 6 (the honest
price, an expressiveness constraint stated in the grammar rather than absorbed); index build 3.2 ms cold,
zero warm (snapshot-resident); per-file segments at 100x. Forecloses: computed predicates on the fast path
(available via `filter` at scan cost, never silently).

**The parse wall** (phase two: Carmack's provocation 1, Tatarchuk's 80 percent measurement). Works via the
snapshot: ~0.43 ms warm against ~19.6 ms cold, and the cold path remains as fallback. Costs: the snapshot
contract of adjudication 5 must be specified and versioned; the invalidation stratification of
adjudication 2 must be implemented or the warm path serves stale derived cells. Forecloses: nothing; but it
promotes the snapshot format to a decision with decades of consequence (Grassia is right), hence Q-B below.

**The migration** (phase one: Pesce's finding 2, my "genuinely blocked" carry; phase two: untouched;
Tatarchuk: "the only finding no mechanism dissolves"). Works, as a composition of three mechanisms: the
derived layer (Karis's 441 rows never authored, maintained by construction), the drain (Grassia's executed
mechanism: freeze, cite, adjudicate over a diminishing frontier with the `seed_id` cursor), and the
epoch-pin dissolution of the live-corpus blocker (Building beyond, mechanism 1). Costs: per repo, roughly
a couple hundred hand-adjudicated rows (98.2 words/row over the non-derivable prose residue) at an
unmeasured adjudication rate; the shared-field library first; schema vocabulary per domain. Forecloses:
nothing. The remaining unknown is a rate, not a mechanism, and the arvo probe produces it.

**The cargo cycle** (phase one: Pesce's headline, my P6; phase two: dissolved). Works today with no
restructuring; the `[patch]` hazard is a hard refusal, tested. Costs: the one-paragraph workflow rule,
still unwritten, now the oldest owed artefact in the round. Forecloses: `[patch]`-based local iteration
on mockspace itself against a stack repo, which is exactly what the rule must say.

## What is genuinely blocked, and what is only unexamined

**Genuinely blocked, with the reason stated:**

1. **The proof-category split.** Compile-time (`const` block on statically known terms) and gate-time
   (checker at commit) remain different categories of guarantee. Arntzen searched for a dissolving
   formulation and found only Q3-C (codegen templates into rustc-checked statics), which costs authoring
   latency coupled to a rebuild; I reached the same wall in phase one from the other side. The honest
   claim is operational: "no document reaching `dev` through the standard flow is unchecked," resting on
   branch-protection rulesets, hook identity with the render-time checker, and the `--no-verify`
   prohibition, all of which are workspace policy (`branch-pr-flow.md`,
   `strict-by-design-quality-pressure.md`), not proof. Blocked because the distinction is real, not because
   nobody looked. Document it as real; do not argue it away.
2. **The adjudication rate.** Extraction volume is priced; human minutes per adjudicated row are not, and
   cannot be derived, only measured. Blocked on executing the arvo probe.
3. **Whether variation-seeded phrasing is wanted.** Empirical, answered by real documents plus the churn
   counter (now free: a comparison of structural hashes). Blocked on usage, by nature.

**Only unexamined, with the examination named:**

1. **The no-heap pipeline under a real implementation.** Karis's harness is `HashMap` and `Vec`; the CSR
   probe is allocation-free as measured and the CSR build needs no growth, but pipeline-wide no-heap is a
   plan. The toolbox's "no shipping precedent" (`07_toolbox.md:238-246`) stands. This is buildable with
   fixed capacities sized from now-measured numbers, failing loudly at the bound; it is unexecuted, not
   blocked.
2. **The full warm end-to-end regeneration bench, render and IO included.** Decides the honest scope of
   "document count is free" and whether anything after the snapshot matters. Karis's provocation 2;
   nobody ran it.
3. **The extractor run backwards on arvo.** 441 derivable rows and 1,794 measured restatements; the tool
   inversion is described, not executed.
4. **The epoch-pin probe** (Building beyond, mechanism 1) on arvo's live corpus. Replaces Grassia's
   freeze-a-live-corpus probe and Pesce's step 3.
5. **The two red tests plus the two new dirty-set cases** (adjudication 2). Written down, not written.
6. **The shared-field library and the vocab duplicate lint.** Specified by Grassia; the duplicate is live
   in shipped schema today.
7. **The fourth brace population** (Tatarchuk's 38 inline-span sites). A parser change plus a lint,
   untouched by phase three, still cheap, still getting more expensive with every authored document.
8. **The depth-3 join and the operator-table algebraic metadata.** One catalogue test and one table shape.
9. **The `[patch]` direction rule.** One paragraph.

## Building beyond the three

Four mechanisms none of the three stated, each assembled from parts two or more of them supplied.

### 1. Epoch-pinned extraction oracles: the freeze dissolves into git

Grassia's blocker: `frozen = true` requires a finished corpus; every repo but one has a live corpus; every
softening he tried (span hashes, heading citations) weakens the citation check. The dissolution: the check
never needed the *working tree* frozen. It needs the *cited bytes* immutable, and the workspace's version
control already provides immutable bytes addressed by commit.

Mechanism: a ref root may declare `pin = "<tag-or-sha>"` instead of (or as the general form of)
`frozen = true`. A citation `seed::FILE::LINE` resolves against the pinned blob (`git show <sha>:<path>`),
which cannot move, so the line check is as honest as against a frozen directory. The live file moves
freely; authors keep editing. Advancing the pin is a deliberate act: diff the old and new pinned blobs,
compute which cited lines fall in changed hunks, and re-open exactly those citations for re-adjudication.
That diff-driven frontier is the same incremental cursor `seed_id` already implements for "new rows";
this extends it to "moved facts." `frozen = true` becomes the degenerate pin that never advances.

Costs: citation verification shells to git (or reads the odb; either is milliseconds over thousands of
citations); citations can only target committed content (correct: the gate is where checking lives anyway);
the pin is one more input in the seed-class fingerprint of the snapshot. Forecloses: citing uncommitted
prose, which nothing should do. What it buys: the single unsolved prerequisite for extraction beyond one
repository, gone. Phase four should ground this in prior art: it is stand-off annotation against a fixed
text (the NLP annotation literature) plus content-addressed persistent identifiers for source lines
(Software Heritage's SWHIDs are the closest named system).

### 2. The stratified dirty set: ground truth classes, not file types

Assembled from Arntzen's dirty-set gap, Grassia's derived-namespace analysis, Karis's derived rows, and the
corrected reference histogram. The commit gate's invalidation input is not "changed `.md.tmpl` files" and
not even "changed files": it is four ground-truth classes, each fingerprinted its own way in the snapshot:

- **Authored templates**: per-file content hash; a change dirties that document.
- **Authored registry rows**: per-row content hash (the loader already computes these for P3); a change
  dirties the row's transitive readership through the 862-edge row-to-row graph plus template readers.
- **Derived facts** (`crates::` today; Karis's 441 tomorrow): a fingerprint of the deriving scan's input
  (directory listing hash; source-scan digest). A change re-derives and dirties the readership; a TOML
  diff never appears. This is the class Arntzen's test misses and the class carrying the most references.
- **Pinned oracles**: the pin itself. Never dirties documents; a pin advance re-opens citations, not
  renders.

The snapshot records all four fingerprints; the gate compares and computes the dirty set structurally,
which is Stachowiak's DBSP derivative criterion made concrete against this corpus's actual dependency
classes. Cost: four fingerprint computations already individually cheap (the dominant one, per-row hashing,
rides the load pass). Forecloses: nothing. Owed: the three red tests of adjudication 2.

### 3. The snapshot as the system's one boundary contract

Adjudication 5 assembled it; stated here as the design rule that makes it load-bearing: **everything that
crosses a boundary in this system crosses inside the snapshot.** mockspace (std) to language crates
(no-alloc): borrowed views over the mapped snapshot. Cold to warm: the snapshot. This run to next run: the
snapshot (generation counter, fingerprints). mockspace to vehje, eventually: the snapshot, versioned. That
one artefact carries the rows, the layer lanes, the index segments, the edges, the cardinalities, the
identity hashes, and the fingerprints; every mechanism adjudicated above reads its inputs from it and no
mechanism needs a second channel. The alternative (each mechanism growing its own cache file and its own
invalidation) is the two-caches gap of phase one reborn at the file level. One artefact, one version
field, one invalidation story.

### 4. Derived facts close the loop on the thesis

Stated because it upgrades the project's own claim. With Karis's extractor inverted and Grassia's layer
model adopted, the fact classes prose restates most (dependency edges, item inventories, attribute flags)
are not migrated into the registry; they are *derived through* it from the artifacts that already can't
lie (`Cargo.toml`, the source tree). The `arvo-bits` contradiction Karis found is then not fixed but made
unrepresentable: there is no authored cell to fork. The registry stops being a second place facts are
written and becomes the typed junction between ground truths and documents. That is the one-source-of-truth
thesis in its strongest form, and none of the three quite said it.

## Open questions, each with at least three costed options and a recommendation

### Q-A. What may `where` accept?

- **A. Indexable predicates only in `where` (equality/membership on typed keys); a distinct `filter`
  spelling for arbitrary predicates at scan cost; gate cost-note above a row threshold.** Effort: small
  (grammar plus one diagnostic). Risk: low; two spellings to document. Payoff: the fast path is
  grammatically guaranteed; scans are visible and priced. Forecloses: nothing (everything expressible,
  nothing silent).
- **B. One spelling; planner picks index or scan; gate warns on scans.** Effort: small-medium. Risk: the
  performance contract lives in a diagnostic, not the grammar; authors learn costs by warning fatigue.
  Payoff: simpler surface. Forecloses: nothing technically, but the silent-degradation channel stays open
  if the warning is ever tuned down.
- **C. One spelling, no restriction, no diagnostic.** Effort: none. Risk: the 202 ms path returns
  unannounced with the first computed predicate; Karis's exact warning. Payoff: none. Forecloses: the
  cost model's predictability.

Ranking: **A > B > C. Recommend A.** Same shape as the membership/substring split already settled for
Muratori's finding 1; consistency argues for it as much as performance.

### Q-B. The snapshot format

- **A. Hand-rolled flat sections: little-endian `u32`/`u64` tables, section directory, version field,
  alignment for mmap.** Effort: medium. Risk: we own every byte and every mistake; mitigated because
  every table is already flat by design (CSR arrays, cell table, hash column) and there is genuinely no
  pointer to fix up. Payoff: zero dependencies in the extractable crates, exact fit, borrowed views are
  just typed slices, format stability under our control (the SQLite-file-format discipline: a version
  field and a spec document). Forecloses: nothing.
- **B. rkyv.** Effort: small. Risk: a serialization framework's derive machinery and its versioning
  story inside crates whose purpose is to be dependency-light and extractable; rkyv's validation layer
  is where its complexity lives. Payoff: zero-copy access with less hand work. Forecloses: full control
  of layout evolution; adds a load-bearing external dependency at the system's one boundary.
- **C. Cap'n Proto / FlatBuffers.** Effort: medium (codegen toolchain). Risk: external schema compiler
  in the build; generated accessors shaped for general graphs where we have flat tables. Payoff:
  battle-tested evolution rules. Forecloses: the no-external-toolchain property of the build.
- **D. serde + bincode (owned deserialization).** Effort: smallest. Risk: deserialization allocates and
  copies, which forfeits the mmap/borrowed-view property that makes the boundary contract work for the
  no-alloc side. Payoff: quickest first version. Forecloses: the boundary contract itself; disqualifying.

Ranking: **A > B > C > D. Recommend A.** The data is flat `u32` tables by construction; a framework would
be imposing generality on a format that has none to need. Phase four should stress the evolution story
(the version field's semantics, what a reader does with a newer snapshot) against named prior art
(SQLite's file format, DWARF's versioning).

### Q-C. The extraction oracle for live corpora

- **A. Epoch-pinned git blobs (Building beyond, mechanism 1): `pin = "<sha>"`, citations verified against
  the pinned blob, pin advances diff-driven.** Effort: small-medium (resolver reads the odb; pin-advance
  tooling computes the re-adjudication frontier). Risk: low; git's immutability is the strongest
  guarantee available in this workspace. Payoff: extraction proceeds against any live corpus with
  citation checking at full strength; the incremental frontier is computed, not guessed. Forecloses:
  citing uncommitted text.
- **B. Content-addressed span hashes (cite a hash of the cited lines).** Effort: small. Risk: a hash
  verifies the span still exists somewhere but not where, and silently breaks on any edit including
  trivial rewording, forcing re-adjudication noise without locating the change. Payoff: no git coupling.
  Forecloses: the diff-driven frontier; Grassia already found this weakens the check.
- **C. Heading citations for live roots (the `mockspace.toml` comment's own recommendation).** Effort:
  none. Risk: heading-granular precision; an edit under an unchanged heading repoints silently, the
  exact dishonesty the frozen-root comment warns about. Payoff: survives edits. Forecloses: line-level
  verifiability, which is what makes extraction auditable.
- **D. Literal freeze (fork the prose into a frozen copy, keep editing the live one).** Effort: small.
  Risk: two copies of 51K words divergent from day one; the two-sources-of-truth failure the project
  exists to remove, self-inflicted. Payoff: none over A. Forecloses: honesty.

Ranking: **A > B > C > D. Recommend A**, and run it as the arvo probe (Q-E) rather than as a separate
exercise.

### Q-D. Where derived facts are computed

- **A. Derived at snapshot build (gate/load), stored in the snapshot with the deriving input's
  fingerprint; never in TOML.** Effort: small-medium. Risk: low; the `crates` namespace already works
  this way minus the fingerprint. Payoff: warm path serves derived cells at zero cost; staleness is
  structural (fingerprint mismatch forces re-derivation); hand-edit refusal per adjudication 3.
  Forecloses: nothing.
- **B. Derived lazily at first query.** Effort: medium (presence checks in the query path). Risk: the
  hot path grows a derivation branch; cold-vs-warm asymmetry leaks into query latency. Payoff: avoids
  deriving unqueried classes, worth ~nothing at measured volumes. Forecloses: nothing, buys nothing.
- **C. Materialised into authored TOML by a generator, with a lint against hand edits.** Effort: small.
  Risk: the derived copy is now a second source of truth that drifts between generator runs; the
  `arvo-bits` contradiction class returns with a lint standing guard instead of being unrepresentable.
  Payoff: rows visible in the authored files. Forecloses: the by-construction guarantee, which is the
  point.

Ranking: **A > B > C. Recommend A.**

### Q-E. The arvo migration probe, redefined

- **A. Derive-first, then epoch-pinned drain: run the inverted extractor for the 441 derivable rows into
  the derived layer; pin arvo's prose at an epoch; drain the residual (~a few hundred rows at 98.2
  words/row) with tool-proposed rows and human adjudication; measure minutes per row.** Effort: days,
  measured as they pass. Risk: low; every mechanism is specified above. Payoff: the adjudication rate
  (the last unpriced number), the second corpus, the shared-field library exercised on a second domain,
  and live schema feedback before the language's registry semantics lock. Forecloses: nothing.
- **B. Drain-first without the derived layer (Pesce's original step 3 shape).** Effort: days, more of
  them. Risk: hand-authors the 441 rows the extractor derives, then either maintains them by hand
  forever or migrates them to the derived layer later (twice the touch). Payoff: probe runs without
  extractor work. Forecloses: nothing, but wastes the phase's own findings.
- **C. Language-first, probe after.** Effort: same, later. Risk: registry-facing semantics lock against
  a single corpus (the round's own repeatedly-identified over-fit hazard); the rate number arrives after
  it can no longer change the design. Payoff: probe runs with better tooling. Forecloses: schema
  feedback into the type vocabulary, where it is worth the most.

Ranking: **A > B > C. Recommend A.**

### Q-F. Upstream stability for the language crates (Arntzen's provocation 4)

- **A. Accept one-directionality in writing; pin arvo/notko/hilavitkutin-str by exact rev or tag; bump
  deliberately with a build-and-test pass, exactly the workspace's own nightly-toolchain-pin discipline
  applied to crates.** Effort: small. Risk: the pins lag upstream; deliberate bumps cost a verification
  pass each. Payoff: upstream churn cannot land uninvited; the language crates offer their consumers the
  stability Q8 wants without demanding it from upstream. Forecloses: nothing; the pin is a policy, not a
  structure.
- **B. Demand a deprecation-window contract from arvo and hilavitkutin-str.** Effort: none here, real
  there. Risk: contradicts the workspace's settled pre-1.0 no-shims rule for the whole stack to serve
  one consumer; the maintainer has ruled that direction before. Payoff: real windows. Forecloses: the
  stack's pre-1.0 velocity; not this round's call to make.
- **C. Vendor the consumed surfaces.** Effort: small now, unbounded later. Risk: drift; already rejected
  in phase one's Q1 for the same reason. Payoff: none over A. Forecloses: clean reuse.

Ranking: **A > B > C. Recommend A**, recorded in writing as Arntzen asked: Q8's stability is
one-directional, and the pin is the mitigation.

### Q-G. The shared-field library's shape (before the `crates` retype)

- **A. Declared once in each repo's `mockspace.toml` under a `[registry.fields]` section; namespaces
  include by name; mockspace validates single declaration and reports per-field population counts;
  retypes declare old and new type and must pass the new validator over every populated cell before
  landing.** Effort: small-medium. Risk: low; twenty lines of resolver per Grassia's own sizing. Payoff:
  73 of 144 slots collapse to 5 declarations plus citations; the shipped `vocab` duplicate becomes a
  load error; the retype-checked-against-data rule closes the unsafe act nothing today prevents.
  Forecloses: nothing.
- **B. Builtin field definitions inside mockspace itself.** Effort: small. Risk: field vocabulary
  becomes tool-owned; nine repos' domain schemas negotiate with mockspace releases for a description
  edit. Payoff: zero per-repo declarations. Forecloses: per-repo field semantics, which the domains
  genuinely vary on (the five `crates` descriptions are partly real variance, partly drift; B cannot
  tell them apart).
- **C. Keep per-namespace copies, add a description-fork lint.** Effort: smallest. Risk: the lint
  guards the fork instead of removing it; 14-per-repo declarations forever; the retype still touches
  every copy. Payoff: no schema change. Forecloses: the one-edit property.

Ranking: **A > B > C. Recommend A**, sequenced before the `crates` `string[]` to `ref<root>[]` retype,
with the retype's data-validation rule (1658 cells, a millisecond) as its first exercise.

### Q-H. What the operator table must carry for the planner (Arntzen and Tatarchuk, merged)

- **A. Now: arity, strictness, family bit, stage, allowed-children (already settled), plus
  indexability-class per predicate operator and the cardinality-source hook the two-integer chooser
  reads. Later, additively: commutes-with/distributes-over for pushdown.** Effort: small now. Risk: low;
  additive columns on a generated table. Payoff: the chooser is buildable alongside T1 (adjudication 1);
  the T7-A relational future needs no table migration. Forecloses: nothing.
- **B. Full algebraic metadata now (pushdown rules included).** Effort: medium. Risk: speculative
  columns validated by no consumer; the exact over-building the phase-two guard warns against, since
  pushdown has no measured payer yet. Payoff: none until a rewrite engine exists. Forecloses: nothing,
  but spends now what is free later.
- **C. No planner metadata; chain evaluates left to right (design as written).** Effort: none. Risk: the
  first nested query picks an accidentally quadratic order with no diagnostic; Arntzen's named silent
  failure. Payoff: none. Forecloses: the dissolution adjudication 1 just bought.

Ranking: **A > B > C. Recommend A.**

## Handoff to phase four

Phase four stress-tests this phase and grounds surviving claims in named prior art. In order of how much
is riding on each:

**Load-bearing and unproven (execute against these first):**

1. **The no-heap pipeline.** Still zero shipping precedent (`07_toolbox.md:238-246`), and every mechanism
   this phase adopted assumes fixed-capacity arenas failing loudly at bounds now sized from real
   measurements. Nothing in three phases has written a line of it. This is the largest unbacked claim in
   the design and the first thing a hostile reviewer should attack.
2. **The full warm regeneration, render and IO included.** Decides the true scope of "document count is
   free." Until it runs, Karis's headline is a claim about 20 percent of the pipeline.
3. **The snapshot contract.** Adjudication 5 assembles it from three partial specifications; nobody has
   written the section layout, the version semantics, or the reader-side view types. It is the system's
   one boundary and currently exists only in prose.
4. **The commit-gate honest claim.** "No document reaching dev through the standard flow is unchecked"
   rests on rulesets, hook identity, and `--no-verify` discipline. Phase four should state what breaks it
   and whether any leg can be hardened from policy toward mechanism.
5. **The adjudication rate.** Only the arvo probe (Q-E-A) produces it; every migration schedule quoted in
   four phases is a volume estimate wearing a schedule's clothes until it runs.

**Claims needing a citation or a novelty verdict:**

- **The loop inversion** is not novel and should not be presented as such: it is the batching fix to the
  N+1 query problem, an inverted index, and group-by pushdown, standard in databases and in GraphQL
  dataloaders. Phase four names the canonical citation and, in doing so, inherits the literature's known
  edge cases for free (skewed fanout, key explosion), which map onto Karis's over-general-index warning.
- **The two-integer join order over exact cardinalities**: Selinger et al.'s System R planning is the
  ancestor; the novelty claim to test is narrower and real: classical planners *estimate* cardinalities,
  while an immutable loaded registry *reads* them, deleting the estimation half. Verify nobody has named
  this special case before claiming it.
- **Epoch-pinned line citations** (Building beyond, 1): ground in stand-off annotation (LAF/GrAF, brat)
  and Software Heritage SWHIDs (content-addressed persistent identifiers down to line granularity). The
  combination with a diff-driven re-adjudication frontier may be genuinely novel; verdict wanted.
- **The three-layer per-cell resolution** (derived < schema-default < authored): USD's LIVRPS strength
  ordering and the CSS cascade's origin model are the ancestors; the refusal of composition-as-graph is
  the design's own restriction and should cite both while claiming only the restriction.
- **The stratified dirty set by ground-truth class**: DBSP's static derivative criterion (already cited
  by Stachowiak) is the frame; build-system input classes (Bazel's source/generated/config distinction,
  Nix's fixed-output derivations for the pinned-oracle class) are the practice to cite. The pinned-oracle
  class mapping onto fixed-output derivations is close enough that phase four should check whether the
  correspondence is exact.
- **Hash-then-verify interning**: Filliâtre and Conchon's type-safe modular hash-consing is the standard
  citation; the in-workspace precedent is verified at `hilavitkutin-str/src/interner.rs:81-89`.
- **The monotone two-level family check**: a sound over-approximation with an exact fallback is abstract
  interpretation's shape and also the fast-path/slow-path discipline of every JIT guard; phase four picks
  the citation that carries the Preserve-policy subtlety, because Arntzen's invariant (mask and staging
  as one predicate) is the part with no obvious prior art and the part most worth a fresh-eyes proof.
- **The snapshot format**: SQLite's file-format discipline and DWARF versioning for evolution; rkyv,
  Cap'n Proto, FlatBuffers as the frameworks Q-B declined, with the decline's reasoning stress-tested.

**Corrections phase four must not re-import:**

- 2676 rows (2686 with vocab), 14 namespaces plus vocab, 207 files; one-level globs are wrong by 233 rows
  and 112 references.
- The reference corpus is 7630 in four populations (1007 template, 862 row-to-row, 2286 derived-namespace,
  3475 pinned-oracle), not 1007 and not 5761; 3475 of the data-side references never participate in
  invalidation.
- The extraction ratio is 98.2 corpus words per drained row (2071 drained rows), not 75.7 over all rows.
- The cargo cycle does not exist absent `[patch]`; the `[patch]` collapse is a tested hard refusal.
- `provenance` carries source citations dropped by frozen-root filtering; the `visibility = "internal"`
  flag is on `seed_id`, not `provenance`.
- Node identity is 4-byte arena index plus 8-byte structural hash with slug-byte leaves and verify-on-
  insert; the interner survives, re-keyed; "4-byte node id" quoted alone is no longer the design.
