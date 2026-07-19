# Attacking the decomposition (John Carmack)

**Date:** 2026-07-19. Every number below was measured directly against the corpus and source, not inherited
from panel one. Where a panel-one number failed the re-count, that is stated in the last section.

## The judgement in one line

This is a small relational fact base with a template splicer and multi-format value rendering; the design
and all four panel-one files dressed it as a compiler, and nearly every sophisticated mechanism in the round
is a repair to a problem that only exists because of that costume.

## What panel one and the design inherited without earning it

Nobody measured the workload before choosing the representation. I did. The live corpus is 2676 registry
rows (~1.7 MB TOML) and 445 templates (~280K words, ~0.8 MB in the one repo that queries). It contains 1082
template references and 3148 references inside registry field values: 4230 total. **Every single one is a
static address.** Zero method chains. Zero `where`. Zero loops. 979 of 1066 `ikiuni_renderer` template
references are two segments (`ns::slug`), 28 are three, 59 are one. The proposed language adds filters,
iteration, and fragments, so the expression population will grow, but the design's own anchoring query
(`09_shape.md:154`) is eleven tokens. That is the workload: a mostly-static fact base, queried by mostly
tiny expressions, regenerated whole, output diffed in git.

Against that, the inherited apparatus:

1. **A node tree in an arena as the one representation for everything.** Expressions, prose, and query
   results all become interned nodes with `EntityList` children (`09_shape.md:21-26`). Three workloads with
   three access patterns forced into one shape because rustc's HIR uses it. rustc has megabyte functions and
   forty passes that rewrite them. We have eleven-token expressions evaluated once and prose that is never
   rewritten.
2. **Hash-consing, and the repair chain it spawns.** LMS interns constructors because generated numeric
   kernels repeat subexpressions massively. Here the crudest dedup proxy is 535 unique of 1066 template
   refs, on three-token terms; the saving is unmeasurable. But the *costs* cascade: interning shares nodes,
   so spans must move off the node (Muratori finding 4), which demands the red occurrence table (Giesen P2);
   sharing makes a node-keyed memo unsound under the scope chain, which demands free-variable environment
   fingerprints (Giesen P4). Drop the interner premise and the entire chain evaporates: per-occurrence
   nodes carry their own spans inline, and P2 and P4 have nothing left to fix. The green/red split is
   Roslyn machinery for incremental reparse of huge files in an IDE; our largest template is a few KB and
   reparsing it is free.
3. **The check-before-evaluate phase boundary, and P5 as its repair.** Muratori's three-year break is real
   as stated, but both his fix and Giesen's monotone two-level check (parse mask, fast accept, exact
   fallback, dual-route `Checked<T>` evidence) repair a boundary that has no reason to exist. Evaluation is
   pure (it reads a registry) and costs milliseconds. Check the **staged output**, the thing actually about
   to be emitted, and the check is exact by construction, one bitmask AND, before any byte is written.
   Conditional content works on day one. The entire monotone argument is scaffolding for a phase ordering
   inherited from compilers, where checking after evaluation would mean checking after paying seconds of
   work. We pay microseconds.
4. **Incrementality machinery for a batch that costs tens of milliseconds.** ReadSet fingerprints (P3),
   environment-fingerprinted memo tables (P4), comemo-style fixpoint detection, Scribble's four passes.
   Typst needs comemo because relayout is expensive per pass; Salsa exists because rustc analysis is
   expensive. Run Aaltonen's own "first thing to break" through its constants: the `reference` table is 616
   rows by ~8 fields, roughly 5K cell touches, single-digit microseconds per unmemoized scan; 1066
   occurrences of it is single-digit **milliseconds** total. The complexity table was written and the
   constants never plugged in.
5. **Scribble's fixed-point traverse, imported for a dependency structure this system does not have.** The
   real graph is bipartite and acyclic by stage: documents read registry rows; rows read rows (a DAG with
   shipped cycle detection, `resolve.rs:711-724`); cross-document *naming* is already solved by
   plan-before-render (`document.rs:19-23`, `DocIndex`). No document's content depends on another
   document's rendered content, and if one ever should, the system's own thesis says that content belongs
   in the fact base. There is no fixpoint to detect.

## The reduction

At the bottom, one generation run is four steps:

1. **Load**: parse TOML into typed columns per namespace (the `10_syntax_correction.md` schema: interned
   `ref` ids, `content` fragments, scalars). ~2676 rows, ~10K interned strings.
2. **Settle the data**: resolve row-to-row references bottom-up over the schema-known DAG. 3148 lookups.
   Already shipped, including the quadratic-clone fix (`resolve.rs:740-742`).
3. **Splice**: each template is a sequence of literal byte runs and holes. Runs pass through untouched.
   Each hole evaluates a small expression against the fact store into a typed value; the value renders per
   target and splices. 1066 holes today.
4. **Write, and let git diff be the review surface.** The committed docs directory *is* a materialized
   view of the fact base, and git is its change log. Incremental maintenance of that view, at this refresh
   cost, is full refresh. That answer stays correct until refresh crosses a human-noticeable threshold,
   which by the arithmetic below is a ~30x corpus away, and when it arrives the fix is a per-column index
   and a dirty-document set computed from the schema-known read lists, not evaluator instrumentation.

The document is a view over the fact base. The language is the view definition. That inversion is the whole
reduction: the design put the language at the centre and demoted the data to "a family contributing
values"; the workload says the data is the system and the language is its query surface.

## What survives the reduction, and why it earned it

- **The typed value domain and the deletion of `resolve_data` (P1).** The one genuinely load-bearing fix in
  the round. Not compiler tradition; just not destroying type information at load. Root cause A verifies
  against source exactly as the synthesis states.
- **Typed reference fields and the schema-known reference graph (`10_syntax_correction.md`).** Cycles
  become a load-time graph check; the substring bug class becomes unrepresentable. Earned twice over.
- **Interned 4-byte ids.** A keyed store wants interned keys. `Str` is real
  (`hilavitkutin-str/src/handle.rs:28-30`) and, per the corrected cycle finding, reachable today.
- **The target model and inclusion-not-coverage (decisions 1-3).** The best idea in the design. It survives
  as *one* exact check over the staged output stream, which is stronger and simpler than the two-level
  version.
- **The nine forms as the expression grammar, target-decided staging, total renderers, the explicit frame
  stack with a branch quota.** Expressions get real trees; they are tiny, they get staged and emitted into
  Lua, and structure is what a Preserve target consumes. This is the traditional piece that earned its
  keep, at its actual size: a leaf component, not the centre.
- **Variation seeding by semantic hash (decision 4)**, collapsed to hashing (fragment id, resolved argument
  ids). Same invariance, no CBOR pipeline over three-token calls.
- Muratori's absence/optional projection, list membership, inline `join`; Pesce's fence-aware validation
  and pinned-render review. All become trivial in the typed domain.

## What does not survive, and what replaces it

- **The one-representation arena tree** is replaced by **three representations matched to access
  patterns**. Facts: struct-of-arrays columns per namespace (interned slug column; ref lists as
  offset+length into one ref-id pool; content fields as spans into a text arena), fixed-cap, sized from the
  measured corpus, failing loudly at the bound. Expressions: small per-hole trees, spans inline on each
  node, no interning, no occurrence table. Prose and query output: a **flat event stream** (the
  pulldown-cmark shape): the 21 document constructors survive as event kinds, nesting is balanced
  begin/end events, the monoid join the design needs for `Iter` is stream concatenation, the family mask is
  a scan of event kinds, and every renderer is a single pass. Trees earn their place only where content is
  rewritten after construction (Pandoc's filter ecosystem); this design has total renderers and no filters,
  so the stream is strictly simpler and strictly more no-heap friendly than an arena of `Para` nodes.
- **Hash-consing, the green/red split, and the environment-fingerprint memo**: deleted with their premise.
  If profiling ever shows repeated evaluation mattering, the first answer is a hash index on the queried
  column (the database answer), not a memoized fold (the compiler answer).
- **The monotone two-level family check (P5)**: replaced by one exact check over the staged stream.
  `Checked<T>` keeps its single constructor; its evidence is one pass, not an enum of proof routes.
- **Scribble's four passes and fixpoint reuse**: replaced by the two-stage DAG the system actually has.
- **P3's read-tracked provenance**: kept as a *product* feature (per-hunk cause labels are genuinely good),
  built the cheap way: references are static, so a hole's read set is its reference list, read off the
  parse; `where` holes record namespace plus filtered column. Regeneration is cheap enough to attribute by
  re-rendering against the previous registry state from git when a hunk needs explaining.

## The honest lower bound

For the real run (`ikiuni_renderer`: 2676 rows, 142 templates, 72 output docs): read ~2.5 MB, perform ~4.2K
hash lookups, render 14 namespace tables (~1.7 MB of cell traffic), write ~2.5 MB. That is a few passes
over roughly 10 MB of memory: **low single-digit milliseconds of machine work**, plus file IO. The one real
cost is TOML parsing: `toml_edit` is a format-preserving DOM parser and plausibly dominates at 1.7 MB;
measure it, and if it is the wall, a binary row cache keyed by file hash buys more than every memo table
discussed in this round combined. Honest budget for a cold full regeneration: 20-100 ms today. The proposed
design is not *slow*; its mechanisms are individually cheap. It is off in a different way: it spends fixed
complexity, in extractable no-heap crates where complexity is most expensive to build, against costs that
measure two to three orders of magnitude smaller than the panel's framing implied, while the actual wall
(parse) goes unpriced.

## Where panel one is wrong

- **The cargo cycle is not a constraint.** Pesce's finding 1 ("unbuildable as written"), the headline of
  his file, and Giesen's opening plus P6's nine-repo bootstrap split, all rest on a check nobody executed.
  Executed: cargo keys packages by name, version, *and source id*; `arvo/mock/Cargo.toml:48` pins mockspace
  by git branch, so a path-local mockspace depending on git-sourced `hilavitkutin-str` resolves cleanly
  with both instances in the graph. What survives is real but small: a double compile, a lagging bootstrap
  instance, and one sharp workflow rule owed: the sanctioned `[patch]` path-override workflow would
  collapse the two instances and make the cycle real, so patching direction must be disciplined. P6 becomes
  an optimisation to consider on its merits, not a prerequisite; Q1's ranking is void.
- **The synthesis's audit arithmetic is itself the error it alleges.** Giesen: "2443 rows by `[[`-count...
  his 2676 total does not [verify], off by roughly 8%" (`04_giesen_synthesis.md:115-117`). Direct count:
  exactly **2676** rows across the fourteen namespaces (616 reference, 506 spike, 444 technique, 344
  ruling, 209 bench, 148 task, 87 law, 66 equation, 54 tripwire, 52 facet, 52 data_shape, 46 field, 40
  constant, 12 abstraction) plus 10 vocab. Aaltonen's total verifies exactly. The word-count "discrepancy"
  is a scope difference, not an error: 280,472 across eight repos minus mockspace's 4,881 is Giesen's
  275,591. The file that re-verified every claim introduced the round's only wrong counts.
- **Aaltonen's finding 3 urgency fails its own constants**, as computed above. The frame (budget
  occurrences separately from nodes) is right; the "first thing to break" conclusion is off by orders of
  magnitude at any plausible growth.
- **The panel's shared, unexamined assumption**: that the expression workload is compiler-shaped. Nobody
  produced the histogram. It is three tokens at the median, static addresses at the tail, and that one
  measurement re-prices every mechanism in the round.

## Open provocations for the panellists after me

1. **Measure the parse.** Time `toml_edit` on the 1.7 MB registry. If it is over half the run, the round's
   entire performance discussion happened on the wrong side of the pipeline.
2. **Kill or confirm the event stream.** Find one required product behaviour (not an inherited one) that
   needs random access or post-construction rewriting of document content. Filters do; total renderers do
   not. If none exists, the tree is dead and the no-heap story gets simpler.
3. **Write the `[patch]` direction rule.** The cycle sidestep holds only while nobody patches mockspace
   into a stack repo's graph. That is one paragraph of workspace policy owed now, before the first person
   hits it mid-iteration.
4. **Name the threshold.** At what measured full-regeneration wall time does incremental machinery buy in?
   Write the number down (I propose one second) and gate every cache, memo, and fingerprint proposal on a
   bench crossing it. Below the line, determinism plus full refresh plus git diff *is* the incremental
   design.
