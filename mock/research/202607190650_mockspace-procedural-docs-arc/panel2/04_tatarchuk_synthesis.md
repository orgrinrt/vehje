# The adversarial phase: synthesis and what survives (Natalya Tatarchuk)

**Date:** 2026-07-19. Every number below was executed on this machine against the live corpus, and where a
measurement contradicts a panellist, the command that produced it is described well enough to re-run. The
benchmark harness is a naive `HashMap` row store built on `toml_edit`, compiled in release mode. It is an
order-of-magnitude instrument for comparing access patterns against each other, not a benchmark of any
proposed implementation. Read the ratios; treat the absolute numbers as a ceiling that a real column store
would beat.

## The one-paragraph judgement

Phase two is right that phase one dressed a small relational fact base as a compiler, and its corpus
measurements are the first honest ones the round has produced. But phase two priced the system being
replaced rather than the system being built. Measured: today's 4154 static addresses resolve in **123
microseconds**, the design's own anchoring loop over 132 per-crate documents costs **2.3 milliseconds**, and
the same query generalised across namespaces costs **229 milliseconds**, because the design converts a sum
over references into a product over documents and rows. Carmack's deletions are correct for every piece of
compiler machinery he cut and wrong about the one piece of database machinery he deferred: the per-column
index he gated behind a one-second threshold costs **10.5 milliseconds to build**, which is less than the
TOML parse it would ride beside, and it turns that 229 milliseconds into **3.4 microseconds**. It raises the
ceiling by four orders of magnitude without raising the floor above the noise, which is precisely the
brief's own test, and it fails that test only under a threshold rule that priced it as a cost to defer.
Quilez's reductions hold, all five, and each one fixes a width that the workspace's own primitives leave
open, so the compact form he found is right and one notch narrower than it should be. Stachowiak's ruling on
hash-consing is correct on the key and misdescribed as a middle path: a structural-hash-keyed dedup table is
an interner, so the interner survives his ruling rather than dying in it, and phase three should not inherit
the impression that it is gone. What phase two got right, it got right by measuring. What it got wrong, it
got wrong by measuring the workload the design exists to replace.

## Audit of the three

### The count post-mortem, since I was asked to settle it

The definitive figure is **2676 top-level rows across 14 namespaces in 207 files**. I reproduce it two ways.
A recursive `grep -c '^\[\['` per namespace directory sums to exactly 2676 (616 reference, 506 spike, 444
technique, 344 ruling, 209 bench, 148 task, 87 law, 66 equation, 54 tripwire, 52 facet, 52 data_shape, 46
field, 40 constant, 12 abstraction), plus 10 in `vocab.toml`. Independently, parsing every file with
`toml_edit` and summing `as_array_of_tables().len()` returns 2686, which is the same 2676 plus the same 10.
Two mechanisms with no shared code path agree exactly.

Giesen's 2443 is not approximately anything. It is **exactly** what `cat registry/*/*.toml` returns: a
one-level glob, 192 files, 2443 rows. The missing 233 are entirely inside `spike/`, which is the only
namespace carrying subdirectories, five of them (`tooling`, `materials`, `fragments`, `water`, `unresolved`),
holding 506 rows of which the one-level glob sees 273. So the error is not arithmetic and not sampling. It is
a glob depth, and it is reproducible to the row.

That matters more than the 8 percent, for two reasons. First, `spike` is the second-largest namespace in the
corpus and the only one that has already outgrown a flat directory, which makes it the leading indicator for
how every namespace eventually looks. A sizing analysis that structurally cannot see the namespace furthest
along its growth curve is not conservative, it is blind in the one direction that matters. Second, the same
glob depth costs 112 references: the recursive count of registry-value references is 3148 and the one-level
glob returns 3036. Any tool, lint, or loader written against `registry/*/*.toml` inherits both errors
silently. That is a shipped-code hazard, not a documentation slip, and phase three should check the loader
for it.

Aaltonen and Quilez had it right. Giesen and the dispatching agent did not, and the file that re-verified
every other claim in the round is the one that introduced this.

### The two reference figures both reconcile, and the gap is a finding

Carmack reports 4230 total references. My brief names 4154. Both are correct under different definitions and
the difference is exactly informative.

- **4230** is every `{{ ... }}` occurrence: 3148 in registry values plus 1082 in templates. Verified exactly.
- **4154** is every reference with two or more segments, meaning every genuine `ns::slug` address: 3147
  two-segment registry references plus 979 two-segment and 28 three-segment template references.
- The **76** difference is the single-segment population, and it is not noise. It is 44 `mock_dir`, 16
  `project_name`, 2 `crate_table`, 2 `crate_count`, 1 `crate_summaries`, 1 `crate_prefix` and 1
  `hook_helpers`, which are fields of the shipped `Placeholders` struct
  (`mockspace/src/render_design.rs:228-237`), plus a tail of `{{ include "..." }}`, `{{ name | upper }}`,
  `{{ var }}` and `{{ version }}`.

So the round's two reference counts differ by precisely the legacy population that `10_syntax_correction.md`
schedules for demolition, plus a handful of examples of a template language nobody in either panel mentioned.
More on that tail below, because it is the most concrete unflagged hazard I found.

### Carmack

**Holds, and holds harder than he claimed.** Every corpus number verifies exactly: 2676 rows, 3148 registry
references, 1082 template references, 4230 total, and the ikiuni_renderer segment histogram of 979 two-segment
against 28 three-segment and 59 one-segment. The 535-unique-of-1066 dedup proxy verifies. His adjudication of
Giesen's word-count discrepancy verifies to the word: all repositories total 280,472, mockspace alone is
4,881, and the difference is 275,591, which is Giesen's figure exactly, so it is a scope difference and not
an error, as he said. `resolve.rs:740-742` carries the quadratic-clone comment, `resolve.rs:279` is
`cell.contains(&want)`, `resolve_data` begins at 662, cycle detection names the whole cycle at 708-725, and
`document.rs:19-23` states plan-before-render. Every citation stands.

His central measurement is not merely right, it is conservative. He estimated single-digit milliseconds of
machine work for the splices. Measured, 4154 hash lookups against a loaded row store take **123
microseconds**. He was an order of magnitude pessimistic about his own point.

His provocation 1 was to measure the parse, and he was right to suspect it. Measured: `toml_edit` DOM-parses
all 207 files, 1.28 MB, in **14.0 milliseconds**, and building a naive row store on top brings the load to
**18.8 milliseconds**. Against the anchoring workload that is roughly 80 percent of the run. His hypothesis
that "the round's entire performance discussion happened on the wrong side of the pipeline" is confirmed at
today's workload, by his own proposed test, which nobody ran until now.

**Thin.** The "1.7 MB TOML" figure is `du` disk usage. The actual byte count is 1.28 MB, about 25 percent
less. Nothing turns on it except that the parse-cost argument was built on the inflated number and survives
anyway.

His honest lower bound of "20-100 ms today" is well calibrated at the low end. Measured load plus anchoring
query is roughly 21 ms.

**Missed.** He established that the workload is a sum over 4154 static addresses and then reasoned about
growth as if the design preserved that shape. It does not. The design's entire purpose is to replace static
addresses with queries, and a query evaluated once per document over a namespace is a product. That is the
subject of a full section below, because it is the one thing I was charged with testing and it is the axis on
which his conclusion turns.

He also missed that his own preferred fix contradicts his own threshold rule. He names the correct answer
outright: "a per-column index and a dirty-document set computed from the schema-known read lists." Then he
gates it behind a one-second measured wall time. Measured, the index costs 10.5 ms to build, which is below
the parse noise he correctly identified as dominant. A mechanism whose floor cost is smaller than the
measurement error on the thing it sits beside is not a mechanism to gate behind a threshold. It is free
ceiling, and the brief's test says take it.

**Wrong.** "That answer stays correct until refresh crosses a human-noticeable threshold, which by the
arithmetic below is a ~30x corpus away." Measured, the generalised query workload is 229 ms at 1x corpus.
The threshold is not a corpus multiple away, because corpus size is not the variable. Query breadth is, and
the design increases query breadth by construction. Detail below.

### Quilez

**Holds, with an exceptional verification record.** Every single count verifies exactly: 2676 rows
recounted per namespace, 3148 registry-value references of which **3147 are exactly two segments and zero
contain a paren**, and **831 distinct**. All three to the digit. `#[marker]` sits on `Contains<S>` at
`hilavitkutin-api/src/access.rs:34` and the `on_unimplemented` note asking consumers to declare
`recursion_limit = "1024"` is at line 37, both exactly as cited. `hilavitkutin-api/src/lib.rs:16` carries
`#![feature(marker_trait_attr)]`, and the workspace's own `unstable-features.md` classifies that feature as
**WATCH**, with "no FCP, long history of lifetime/overlap impl rough edges." His provocation 2 asks who owns
dragging a WATCH feature into the extractable crates; the workspace rule has already answered that such a
feature needs an owner, so the provocation is not rhetorical.

His summary-size arithmetic verifies: 24 bytes across roughly 12,676 nodes is 304 KB, under a megabyte as he
said.

**Right for a stronger reason than he gave.** He warns against hashing interned ids because
`ArenaInterner::arena_intern` returns encounter-order ids. Reading the source, the contract is weaker than
that and the conclusion is therefore stronger. `arena_intern` is a host-implemented trait method documented
only as "insert `s` into the arena and return its 28-bit ID"; encounter order is a property of a host
implementation that does not exist yet, not of the contract. Meanwhile `StringInterner::intern` consults a
**linker section** first (`interner.rs`, `lookup_const_by_value`), so for every const string the id is a
function of link order, which is less predictable than encounter order and can move when an unrelated crate
adds a literal. His conclusion, do not hash ids, holds under a weaker premise than he assumed.

**Thin.** The nine-to-three reduction names one loss, arity diagnostics, and understates a second.
Under a strictness mask, `Apply` stops being "evaluate the arguments, then call" and becomes "look up the
row, consult the mask, evaluate the strict arguments only." That moves a class of error from the type system
into the evaluator, and evaluator bugs are runtime bugs where enum-arm bugs are compile errors. The fix is
cheap and he does not name it: generate the operator table from a declarative source so that arity and
strictness are validated at table-build time rather than trusted at dispatch time. With that, the reduction
holds cleanly.

**Missed.** Every width he closed, he closed with a constant. The family mask is `const MASK: u64`, ceiling
64, extending to `[u64; N]`. But `arvo-storage`'s `Bits<const N: u16, S, Sign>` is already const-generic over
width, with container projection to `u8` for 1..=8, `u16` for 9..=16, `u32` for 17..=32, `u64` for 33..=64,
`u128` for 65..=128, and a `WideBits`/`MultiContainer` shape above that. At the two families that exist
today, `Bits<2, Hot>` is one byte; his `u64` is eight. At 200 families his `[u64; N]` needs a hand-written
N-way AND and a new type; `Bits<200, Hot>` is the same expression. His compact form is right and it is one
notch wider at the bottom and one notch shorter at the top than the workspace's own primitive already
supports. This is exactly the pattern my brief predicted would recur, and it recurs in his best idea.

**Wrong.** Nothing material. His claim that the document algebra's 21-versus-35 question "should not have
existed" is half right and costs something he did not price; that is adjudicated below rather than filed as
an error.

### Stachowiak

**Holds.** The literature is correctly characterised. egg's e-class analysis does require the merge to form
a join-semilattice, and that is the right law to check Quilez's four lanes against. DBSP does formalise
materialized-view maintenance and does treat full recompute as a legitimate mode rather than a fallback.
Unison does content-address by recursive structural hash. His sharpest sentence is the correct one: "the
traversal buys the summary; the interning buys structural sharing." That is the actual decomposition and
neither Carmack nor Quilez stated it.

His observation that nobody asked whether the query operators compose is correct and is the largest genuinely
unexplored area in the round.

**Thin, and mis-stated as a middle path.** He presents his ruling as keeping Quilez's summary and dropping
Carmack's interner. It does not drop the interner. To deduplicate by structural hash you must ask "have I
seen this hash before", and that question is answered by a hash table from hash to arena slot. That table is
an interner. What his ruling actually does is keep hash-consing and **change its key** from an allocation
artefact to a content hash, which is a strict improvement and the right call, but phase three must not
inherit the impression that the interner is gone. It survives, better keyed.

**The collapse is not free, and he prices it at zero.** If node identity is a structural hash, it cannot be
a 4-byte arena index. At 32 bits and roughly 12,676 nodes the birthday collision probability is about 2
percent, which is unusable in a system whose thesis is correctness by construction. At 64 bits it is about
4e-12, which is fine. So identity becomes 8 bytes, or stays 4 with a separate 8-byte hash column, and either
way the design's headline "4-byte node id" becomes 12 bytes of identity per node. At current corpus that is
roughly 50 KB, which is nothing, and it should still be stated rather than absorbed, because the 4-byte id is
quoted as a virtue in `09_shape.md:21` and will be quoted again.

**Missed.** His centrepiece, construction-time recursive structural hashing, is in tension with the
representation Quilez adopted from Carmack, and neither he nor Quilez noticed. A recursive structural hash is
defined over a tree. Carmack replaced the document tree with a flat event stream and Quilez sided with him.
You can recover the tree hash from a balanced begin/end stream, but only with an explicit depth stack, which
is precisely the mechanism Quilez proposed for well-formedness checking and described as "one addition he
owes." So that depth stack is not a validation nicety. It is load-bearing for the caching story, it must
exist for the structural hash to be computable at all over document content, and it should be specified as
such rather than as a hygiene pass.

**Where he and Quilez are each half right about the seed, and the synthesis neither stated.** Quilez says do
not hash ids, hash slug bytes, because ids move. Stachowiak says do not hash names, hash normalized argument
structure, because two spellings of the same argument should seed identically. Both objections are correct
and they are not in conflict. The resolution is a **recursive structural hash whose leaves are slug bytes**:
structure from Stachowiak, leaf identity from Quilez. That is stable under reordering, renumbering and
reformatting, and identical for semantically identical arguments spelled differently. Neither panellist wrote
that sentence and it is the one the design needs.

### Verification scorecard

| Claim | Source | Verdict |
|---|---|---|
| 2676 rows, 14 namespaces | Aaltonen, Carmack, Quilez | exact, reproduced two independent ways |
| 2443 rows | Giesen | wrong; exactly the one-level glob, misses 233 in `spike/` subdirs |
| 3148 registry refs, 3147 two-segment, 831 distinct | Quilez | exact on all three |
| 1082 template refs, 535 unique of 1066 | Carmack, Aaltonen | exact |
| 275,591 words is a scope difference not an error | Carmack | exact; 280,472 minus mockspace's 4,881 |
| 548 unique template refs | Aaltonen | correct raw; 546 after whitespace normalisation |
| `#[marker]` at `access.rs:34`, recursion note at `:37` | Quilez | exact |
| `Str` is 28-bit id plus origin bit | Quilez | exact; and const path is linker-order, so his point is stronger |
| `hilavitkutin-str` has no `build.rs` | Giesen | exact; the cycle to `Str` is transitive through arvo |
| `notko` has no `build.rs`, no external deps | Giesen | exact; P6 step 2 premise holds |
| `resolve.rs:279` / `:662` / `:740-742` / `708-725` | Muratori, Carmack, Giesen | all exact |
| Single-digit ms of machine work for splices | Carmack | conservative; measured 123 microseconds |
| `toml_edit` plausibly dominates | Carmack | confirmed; 14 ms of a 18.8 ms load |
| Wall is a ~30x corpus away | Carmack | wrong axis; 229 ms at 1x corpus under general queries |

## Where phase two corrected phase one, and where it overcorrected

### Corrections that stand

**Measuring before choosing a representation.** Phase one produced a complexity table with no constants in
it. Aaltonen's own "first thing to break" turns out to be off by three orders of magnitude at today's scale,
and Carmack is right that the table was written and the constants never plugged in. This is the single
largest correction in the phase and it re-prices everything.

**The cargo cycle.** Phase one's headline blocker is not a blocker. I re-tested it from scratch rather than
trusting the brief, using the real topology: a path-sourced package named `mock` depending on a git-sourced
`mid` whose `[build-dependencies]` include a git-sourced `mock`. `cargo metadata` exits 0 and both instances
appear in the graph, one `path+file://...#mock@0.1.0` and one `git+file://...#mock@0.1.0`. The correction
holds. Giesen's nine-repository bootstrap split is an optimisation, not a prerequisite, and Q1's ranking is
void as Carmack said.

**Deleting the incremental machinery.** ReadSet fingerprints, comemo-style fixpoint detection and Scribble's
four passes are all repairs to problems this system does not have, and Quilez's argument is the decisive one:
within a single generation run the registry is immutable, so a read-set fingerprint is constant across the
run and carries zero bits. That is stronger than Carmack's cost argument and it is correct.

**The variation seed.** Quilez caught a real reintroduction of the exact failure decision 4 exists to
prevent, and Stachowiak generalised it to the right rule. Content-addressing, not allocation artefacts. This
is a genuine save.

### Overcorrections

**Deleting the interner outright.** Carmack's dedup argument is right on today's numbers and wrong as a
conclusion, and Stachowiak's ruling effectively restores the interner while describing itself as splitting
the difference. Net: hash-consing survives. Phase three should treat the interner as kept, re-keyed.

**Gating the index behind a wall-clock threshold.** This is the overcorrection that costs the most. Carmack
correctly deletes the compiler's answer to repeated work (memo tables keyed by node identity), correctly
names the database's answer (a per-column index), and then applies to the second the threshold rule he
derived for the first. Measured, the index build is 10.5 ms against a 14 ms parse. It is below the noise of
work the pipeline already does unconditionally. A rule that defers it is pricing a floor that does not exist.

**Treating small expressions as evidence of a small system.** The expression histogram is real and decisive
about the expression language: three tokens at the median, zero parens in registry values, static addresses
at the tail. Carmack is right that this is not a compiler workload. But he slides from "the expressions are
tiny" to "the machine work is tiny", and those are different claims. The work is not in the expressions, it
is in what each expression touches, and a three-token expression can touch 2676 rows. `task::where(crates ~
self)` is eleven tokens and 148 row comparisons per document.

**The event stream, adopted without pricing what it forecloses.** Carmack proposes it, Quilez seconds it, and
Stachowiak then builds his centrepiece on a mechanism that needs the tree shape back. Nobody costed the
interaction. The stream is probably still right, for the reasons Carmack gives, but it needs the depth stack
specified as load-bearing rather than optional.

## Everything still open, adjudicated

### Does Stachowiak's ruling on the hash-consing fork hold?

**Partially, and it needs restating.** What holds: the key must be a recursive structural hash rather than an
allocation artefact, Unison is the right precedent, and the summary is bought by the traversal rather than by
the sharing. That decomposition is correct and it is the ruling's real contribution.

What does not hold is the framing. The ruling is not "keep the summary, drop the interner." It is "keep
hash-consing, change its key," because structural dedup requires a hash-to-slot table and that table is the
interner. And the ruling is not free: identity widens from 4 bytes to 8, or to 4 plus a separate 8-byte hash
column, for roughly 50 KB at current corpus.

Restated so phase three can build against it: **node identity is a 64-bit recursive structural hash whose
leaves are slug bytes; the arena keeps a hash-to-slot table; that table is the interner, re-keyed; dedup is a
side effect of insertion; the same value is the memo key once closed over its free variables and the
variation seed.** With that statement the ruling holds and is worth having.

### Is the workload a template splicer or a compiler?

**A splicer today, and the design's purpose is to stop it being one.** Measured, today is 4154 static
addresses resolving in 123 microseconds. That is a splicer, exactly as Carmack says, and every mechanism
sized for a compiler is mis-sized against it.

But three properties of the proposed design are not splicer properties and each is already committed. `self`
is a binding, and a scope chain is not something a splicer has. `where` is a predicate over a relation, and a
query planner is not something a splicer has. Fragments with declared parameters and phrasing variants are
lambda abstraction with a deterministic selection function, which is not something a splicer has. The
anchoring use case in `09_shape.md:154` uses all three at once.

So the honest verdict is that Carmack is describing the corpus with precision and the design with
imprecision. The correct framing is neither of his: this is not a compiler and not a splicer, it is a
**query engine with a template surface**, and its cost model is a database's, not a compiler's. That framing
is what makes his index instinct right and his threshold rule wrong, because databases build indexes
unconditionally and defer nothing behind a stopwatch.

### Do nine forms reduce to three?

**Yes, with the table generated rather than hand-written.** The equalities hold. `Let x = e in b` is
`Apply(Lambda([x], b), [e])`. `Iter` is `Apply(map, [source, Lambda])` and the design already committed to
that spelling for inline iteration, so it genuinely does carry two iteration mechanisms today. `Seq` is the
n-ary flattening of the content monoid's binary join, which is representation and not semantics. `Project`
is `Apply(sel_n, [b])`. `Select` and `Lambda` fall out of a per-callee strictness mask.

The two costs are arity diagnostics, which Quilez names and prices correctly at a table lookup, and the
migration of strictness from the type system into the evaluator, which he does not name. Generating the
operator table from a declarative source recovers the second: arity, strictness, family bit, stage and
allowed-children are validated when the table is built, and the evaluator consults a structure that has
already been checked.

Adopt the reduction. Generate the table.

### Bitmask or cons-list for the family check?

**Bitmask, decisively, and not `u64`.**

The evidence against the cons-list is stronger than a preference. `Contains<S>` is `#[marker]`, which
requires `marker_trait_attr`, which the workspace's own `unstable-features.md` places on the WATCH list with
a documented history of overlap rough edges. Its shipped diagnostic already anticipates trait resolution
overflowing and asks consumers to raise `recursion_limit` to 1024. Adopting it means dragging a WATCH
unstable feature into the extractable no-heap crates, whose entire purpose is to be extractable. The mask
needs no feature gate, no recursion, and produces a better diagnostic for free, because `doc & !tgt` has its
offending families in the set bits where the cons-list needs a separate reflection mechanism to name them.

The loss Quilez names is real and he underweights it. A cons-list carries type identity, so `Contains<S>` can
gate a method's existence. An integer cannot. If the design ever wants "this method exists only when the
family is present", the mask cannot express it. But that is not what the family check is for; the requirement
is an inclusion proof at emission, and the mask discharges it at both compile time (through a `const` block)
and run time (through the same expression on a runtime operand) with one operation. The two mechanisms are
also not exclusive: if method-gating is ever wanted, a marker trait can be added for that use alone without
disturbing the mask.

The ceiling correction: use `Bits<N, Hot>`, not `u64`. Const-generic over width, one byte at today's two
families, one instruction at every width to 128, multi-limb above that, and the same call-site expression
throughout.

### Is the document algebra rows in a table?

**Its attributes are. Its constructors are not, and the difference is decision 3.**

Quilez is right that `Para`, `Quote`, `Div`, `Emph`, `Strong` and `Span` differ only in block-or-inline,
allowed-children and family bit, and that `Heading`'s level and `Table`'s column spec are attributes once
attributes are uniform. That part is correct and it does dissolve the count question: adding `Math` becomes
authoring a row plus arms in the targets that declare it.

What he does not price is that his own consequence breaks a settled decision. He writes that a renderer's
totality obligation becomes "handle every row whose family bit is in my declared mask, which is checkable by
iterating the table rather than by relying on match exhaustiveness." Iterating a table is a runtime check.
Decision 3 says renderers are total over their declared family set **and that totality is checked**, and the
mechanism that checks it today is `match` exhaustiveness at compile time. Decision 3 is settled and not under
review. Trading a compile-time guarantee for a runtime one is not a reduction, it is a downgrade wearing a
reduction's clothes, and it is exactly the move the brief tells this phase to attack in phase one.

The synthesis neither reached: **constructors stay an enum, their attributes live in a const table indexed by
discriminant.** Renderers `match`, so exhaustiveness holds and decision 3 is untouched. Block-or-inline,
allowed-children, family bit and stage are read from `TABLE[disc as usize]`, so the fold, the well-formedness
pass and the family scan are all data-driven and uniform, which is everything Quilez wanted the table for.
Adding `Math` is one enum variant plus one table row plus arms in declaring targets, and the arms are
compile-errors-until-written, which is the property that made the small algebra affordable in the first
place.

### The caching question after the collapse

Collapse node id, structural hash and CallHash into one value, and this is what is left.

**The memo key is the closed structural hash.** The collapse does not by itself solve Giesen's P4, and
Stachowiak does not address it. A structural hash of an open term such as `where(crates ~ self)` is identical
across every binding of `self`, so keying a memo on it is exactly the cross-document cache poisoning P4
identified. The fix is Quilez's and it is the right one: key by the node's **closed** form, with free
variables substituted by their bound ids at the point the memo is taken. For the dominant case that is one
interned id. With that, P4's free-variable summary protocol collapses into the hash it was computing
alongside, which is the genuine simplification.

**Cross-run invalidation is a `u32` generation counter**, bumped at load. Within a run the registry is
immutable so the counter is constant and free; across runs it is the only thing that distinguishes them.
Quilez is right and this deletes ReadSet as a runtime mechanism.

**Provenance survives at zero runtime cost**, because with typed reference fields a hole's read set is
statically known from the parse. P3's per-hunk cause labels are a product feature built from schema data, not
evaluator instrumentation. Both Quilez and Stachowiak reached this independently and it holds.

**Persistence across runs is a real option and it has a named dependency.** Stachowiak's fourth provocation
is to cost persisting the content-addressed cache across renders. It is worth doing and it is not free
standing: a content hash is stable across processes only if the arena is serialized, which is Giesen's Q3-B
commit-gate compilation. The two are one feature. Phase three should treat them as such rather than as two
proposals.

## Does Carmack's cost model survive the target workload

This is the question I was charged with, and the answer is that it survives the anchoring use case with two
orders of magnitude to spare, and fails the generalisation of that use case by about threefold, and the
distinction is not corpus size.

### The measurement

All figures from the same harness, same machine, release mode, warm.

| Workload | Shape | Measured |
|---|---|---|
| Load: `toml_edit` DOM parse, 207 files, 1.28 MB | fixed | 14.0 ms |
| Load: parse plus naive row store | fixed | 18.8 ms |
| **Today**: 4154 static addresses, hash lookups | O(references) | **0.12 ms** |
| **Anchoring**: 132 docs x 148 task rows, matched rows rendered | O(docs x rows) | **2.3 ms** |
| Anchoring at 10x documents | O(docs x rows) | 22.8 ms |
| Anchoring at 100x documents | O(docs x rows) | 229 ms |
| **General**: 132 docs x all 2676 rows, all fields | O(docs x corpus) | **229 ms** |
| Index build, token to row postings, once | O(corpus) | 10.5 ms |
| **General, indexed**: 132 docs, all namespaces | O(docs) | **0.0034 ms** |
| General indexed at 100x documents | O(docs) | 0.24 ms |

### What the numbers say

**Carmack's conclusion survives the anchoring use case comfortably.** 132 documents each scanning 148 task
rows is 19,536 row comparisons and costs 2.3 ms. Add the 18.8 ms load and a full cold regeneration today is
about 21 ms, which sits precisely at the low end of his own honest 20-100 ms budget. His estimate is good.

**His parse hypothesis is confirmed and is the most actionable thing in his file.** At the anchoring workload,
load is 18.8 ms and query is 2.3 ms, so roughly 80 percent of the run is parse and row-store construction.
Every memo table, fingerprint and fixpoint mechanism discussed across both panels was aimed at the 20 percent.
He guessed this and the guess was right.

**His growth conclusion does not survive, because he priced the wrong variable.** He writes that full refresh
stays correct "until refresh crosses a human-noticeable threshold, which by the arithmetic below is a ~30x
corpus away." But today's cost is a **sum** over 4154 references and the design's cost is a **product** over
documents and rows. Products do not grow like sums. The general query workload is 229 ms at 1x corpus, today,
with no growth at all, which is already within a factor of four of his own proposed one-second threshold. At
10x documents and 10x rows, which is 10x corpus and not 30x, the product grows a hundredfold and lands near
23 seconds.

**And query breadth is a design decision, not a corpus property.** The gap between 2.3 ms and 229 ms in the
table above is not more data. It is the same 132 documents querying more namespaces. The design exists to make
querying the default way facts reach documents, so breadth grows because the design succeeds, not because the
registry does. This is the specific sense in which Carmack measured the thing being replaced: 4154 static
addresses is the access pattern of a corpus that has not yet adopted the mechanism being designed.

**The crossover is the design's own ambition.** At the anchoring workload, parse is 80 percent of the run and
query is 20. At the general workload, query is 94 percent and parse is 6. Which side of the pipeline the cost
lives on is decided by query breadth, and the design moves it across. Both of Carmack's conclusions are
correct on one side of that crossover and both invert on the other, which is why the single-threshold rule
cannot work.

### The verdict

His cost model is an accurate measurement of the present and a category error about the future, and the error
is the one his own file warns against most sharply. He wrote that "the complexity table was written and the
constants never plugged in." He plugged in the constants for the workload that exists and reasoned about
growth without plugging in the constants for the workload the design creates. That is the same failure one
level down, and it is worth saying plainly because his measurement discipline is otherwise the best thing
either panel produced.

The right rule is not a stopwatch threshold. It is structural, and Stachowiak already named it from DBSP:
build the index when the query's derivative is computable statically from the schema, which with typed
reference fields it is, from day one. The index costs less than the parse. Build it.

## Building beyond: where the reductions lowered the ceiling, and what lifts it

The brief predicted a pattern: the reduction is right but the ceiling needs lifting. It recurs five times,
and in four of them the lift is free.

**1. The index. Ceiling: query breadth.** Carmack deletes the memo tables correctly and defers the index
incorrectly. Measured lift: 10.5 ms build cost converts 229 ms into 3.4 microseconds, and converts the growth
law from O(documents x corpus) to O(documents). Floor cost is below the parse it rides beside. This is the
largest available ceiling lift in the round and the brief's test says take it unconditionally.

**2. The family mask width. Ceiling: 64 families.** Quilez's `const MASK: u64` is right and one notch narrow
at both ends. `Bits<const N: u16, S, Sign>` from `arvo-storage` is const-generic over width with container
projection, so it is a single byte at the two families that exist today and multi-limb above 128, with the
same `d & !t == 0` expression at every width. Lower floor, higher ceiling, same call site, and it uses the
stack instead of reinventing a narrower version of it.

**3. The document algebra. Ceiling: compile-time totality.** Quilez's table dissolves the constructor-count
question and, as stated, converts decision 3's checked totality from a compile-time property into a runtime
loop. The lift is to split the two: constructors stay an enum so `match` exhaustiveness still discharges
decision 3, attributes move to a const table indexed by discriminant so everything data-driven stays
data-driven. Both properties, no trade.

**4. The variation seed. Ceiling: what counts as the same call.** Quilez's slug-byte hash is stable under
reordering; Stachowiak's structural hash is stable under spelling. A recursive structural hash with slug-byte
leaves is stable under both, and costs the same single traversal either version costs.

**5. The event stream. Ceiling: structural hashing over document content.** This one is not free and should
be stated. A stream forecloses random access and post-construction rewriting, which Carmack argues correctly
that this design does not need. What it also forecloses, and nobody costed, is the recursive structural hash
Stachowiak made central, unless the balanced begin/end stream is walked with an explicit depth stack. Quilez
proposed that stack for well-formedness. The lift is to recognise it as load-bearing for identity as well, at
which point one stack serves validation, the family scan and the content hash, and the stream survives with
its cost honestly stated.

### The unflagged hazard: there are four brace populations, not three

This is the one finding neither panel produced and it is concrete, live, and cheap to fix now.

Pesce identified three populations sharing `{{ }}`: placeholders, registry references, and the proposed query
language. Giesen ruled the finding "superseded by design" because `10_syntax_correction.md` removes `{{ }}`
entirely. Both missed a fourth, and it is a shipped dependency.

`mockspace/mock/crates/mockspace-template/Cargo.toml:12` is `minijinja = "2"`, and its `DESIGN.md.tmpl:26`
documents the wrapper: "consumers get full Jinja2 syntax: `{{ var }}` substitutions, `{% for %}` loops,
`{% if %}` conditionals, `{% include %}` partials, and filters (`{{ name | upper }}`)." Those braces are in
**inline code spans in running prose**, not in fenced blocks. `find_registry_refs` only skips fenced blocks
(`refs.rs:41-48`), so the fence-skip that Pesce's finding 4 correctly wants narrowed does not protect this
surface at all, and `render_design.rs:225-227` states there is deliberately no escape for a literal.

Across the corpus there are **38 inline-code-span sites already containing braces**. And `09_shape.md:249-251`
proposes that inline code spans carry expressions. So the design's chosen surface for expressions is a surface
where 38 sites currently hold examples of two other template languages, in prose, with no escape mechanism,
in the repository that owns the tool.

The fix is the one Pesce asked for and it is now sharper: one arbiter owning the whole brace and code-span
namespace, an escape landed before the first query is authored, and validation that reads inline spans rather
than only fences. The cost of landing it now is a small parser change. The cost of landing it after the first
query is authored is a corpus migration in the repository whose documentation is about template syntax.

## Open questions, each with at least three costed options and a recommendation

### T1. When is the per-column index built?

- **A. Unconditionally at load, alongside the parse.** Effort: small; measured 10.5 ms build, one pass over
  values already in memory. Risk: low; the index is derived from immutable data within a run and invalidates
  with the generation counter. Payoff: growth law drops from O(documents x corpus) to O(documents); the
  general query workload goes from 229 ms to 3.4 microseconds; query breadth stops being a cost axis. Floor
  cost sits below the measurement noise of the parse. Forecloses: nothing.
- **B. Lazily on first query per column.** Effort: small-medium; needs a presence check on every query and a
  build path in the hot path. Risk: low but the bookkeeping is the kind that grows arms. Payoff: pays only
  for columns actually queried, which today is a minority. Forecloses: nothing, but it buys roughly 8 ms
  against a 19 ms load and adds a branch to every query.
- **C. Behind a measured wall-time threshold (Carmack's proposal, one second).** Effort: none now, medium
  later plus the bench harness to detect the crossing. Risk: the threshold is a stopwatch on a quantity that
  moves with query breadth rather than corpus size, so it will be crossed by a design change rather than by
  growth and will be crossed between two commits rather than gradually. Payoff: nothing measurable; the cost
  it defers is 10.5 ms. Forecloses: nothing, but it guarantees the crossing is discovered as a regression.
- **D. No index; rely on full refresh plus git diff.** Effort: none. Risk: the general workload is already
  229 ms at 1x corpus and grows as a product. Payoff: simplest. Forecloses: query breadth, which is the
  design's purpose.

Ranking: **A > B > C > D. Recommend A.** The index is the database answer to the database problem this
system actually has, its floor is below the parse noise, and Stachowiak's DBSP framing supplies the
principled build criterion (the derivative is statically computable from the schema) that replaces a
stopwatch with a structural rule.

### T2. What is node identity?

- **A. 64-bit recursive structural hash, leaves are slug bytes; arena keeps a hash-to-slot table.** Effort:
  medium; the constructor already traverses children, so the hash is folded into a pass that exists. Risk:
  low; collision probability at corpus scale is around 4e-12. Cost: identity widens from 4 to 8 bytes, about
  50 KB at current corpus. Payoff: one value serves identity, dedup, the closed-form memo key and the
  variation seed; stable across processes, which is what makes T5 persistence possible; immune to both the
  renumbering fragility Quilez found and the spelling fragility Stachowiak found. Forecloses: nothing.
- **B. 4-byte arena index, encounter-ordered interning (the design as written).** Effort: none. Risk: ids
  move under corpus reordering and, through the const linker-section path, under unrelated crates adding
  literals; that is the failure decision 4 exists to prevent, one layer down. Payoff: narrowest identity.
  Forecloses: cross-process caching, and any identity-derived seed.
- **C. 4-byte arena index plus a separate 8-byte structural hash column.** Effort: same as A. Risk: low; two
  values to keep in step. Payoff: keeps dense 4-byte child lists and `EntityList` handles unchanged while
  getting A's properties. Forecloses: nothing.
- **D. No interning; per-occurrence nodes with inline spans (Carmack).** Effort: smallest. Risk: gives up the
  identity that the memo key, the seed and the churn counter all rest on; three mechanisms then need three
  answers. Payoff: deletes the span-versus-sharing conflict and with it Giesen's P2 red table. Forecloses:
  structural equality as a 4-byte compare, and Stachowiak's whole frontier.

Ranking: **C > A > D > B. Recommend C.** It is A's semantics with the arena's existing 4-byte child-list
layout undisturbed, which matters because `EntityList` handles are quoted throughout the design. State the
extra 8 bytes per node in the cost model rather than absorbing it.

### T3. Enum or table for the document algebra?

- **A. Enum for constructors, const table for attributes, indexed by discriminant.** Effort: small. Risk:
  low; the table is generated from the same declaration the enum comes from. Payoff: decision 3's
  compile-time totality survives untouched, and the fold, well-formedness pass, family scan and staging all
  read uniform data. Adding `Math` is one variant, one row, and compile errors in declaring targets until
  the arms are written. Forecloses: nothing.
- **B. Pure table, three node tags (Quilez).** Effort: small. Risk: renderer totality becomes a runtime loop
  over rows, which downgrades a settled decision from a compile-time guarantee to a runtime one. Payoff:
  maximum uniformity; adding a constructor never touches Rust. Forecloses: decision 3 as written.
- **C. Pure enum, attributes as match arms (the design as written).** Effort: none. Risk: every data-driven
  pass repeats a match over 21 arms, so the family scan, the well-formedness check and staging each carry a
  copy of the same knowledge and can disagree. Payoff: simplest to read. Forecloses: nothing, but it is
  where Quilez's criticism lands correctly.
- **D. Enum plus derive macro generating the table.** Effort: medium. Risk: a proc macro in the extractable
  crates, which is a build-graph dependency the no-heap constraint does not forbid but the extraction story
  would rather avoid. Payoff: A with no hand-maintained parallel array. Forecloses: nothing.

Ranking: **A > D > C > B. Recommend A**, with D as the natural follow-up once the table stops being small
enough to eyeball. The parallel-array-drift risk in A is real and is a catalogue test: assert that the table
has exactly one row per discriminant.

### T4. Tree or event stream for document content?

- **A. Flat event stream plus an explicit depth stack (Carmack and Quilez, with the stack made load-bearing).**
  Effort: small-medium. Risk: the stack is now required for three things at once, so it must be specified
  rather than treated as a validation pass. Payoff: single-pass renderers, stream concatenation is the
  monoid join `Iter` needs, no-heap friendly, and the depth stack recovers the tree hash for T2. Forecloses:
  random access and post-construction rewriting, neither of which total renderers need.
- **B. Arena tree (the design as written).** Effort: none. Risk: it is Roslyn machinery for a problem this
  corpus does not have, and it forces the span-versus-sharing conflict that produced Giesen's P2. Payoff:
  structural hashing and well-formedness are direct rather than reconstructed. Forecloses: nothing, but
  costs the red occurrence table.
- **C. Stream for document content, small trees for expressions.** Effort: same as A. Risk: two
  representations, so the structural hash must span both and the boundary needs stating. Payoff: each side
  matched to its access pattern; expressions are tiny and tree-shaped, content is long and stream-shaped.
  This is what Carmack actually proposed, stated as two decisions rather than one. Forecloses: nothing.

Ranking: **C > A > B. Recommend C**, which is A with the expression side named separately so nobody later
tries to stream an eleven-token expression. Specify the depth stack as a required component of the content
representation, not as a hygiene pass.

### T5. Does the content-addressed cache persist across runs?

- **A. No; within-run memo only, keyed by closed structural hash plus generation counter.** Effort: smallest.
  Risk: none. Payoff: correct, cheap, and at measured costs it is sufficient today. Forecloses: nothing
  permanently; persistence is additive.
- **B. Persist the serialized arena at the commit gate, keyed by content hash (this is Giesen's Q3-B and
  Stachowiak's provocation 4, which are one feature).** Effort: medium; flat `u32` tables serialize trivially,
  and the gate already parses everything it checks. Risk: low; a content-addressed cache is invalidated by
  construction. Payoff: authoring errors become gate refusals, renders start at the fold, and the 14 ms parse
  stops being paid per render, which is the measured dominant cost today. Forecloses: nothing; cold parse
  remains the fallback path.
- **C. Persist a binary row cache keyed by file hash, without arena serialization (Carmack's suggestion).**
  Effort: small-medium. Risk: low. Payoff: attacks the measured 14 ms directly and buys, in his words, more
  than every memo table in the round combined; measurement supports him. Forecloses: nothing, and B subsumes
  it later.
- **D. Full cross-run incremental with dirty-document sets from the schema-known read lists.** Effort: large.
  Risk: building incremental machinery for a batch generator, which is the thing phase two correctly deleted.
  Payoff: minimal until the general query workload is both indexed and large. Forecloses: nothing.

Ranking: **C > B > A > D. Recommend C first, then B.** C is the smallest change that attacks the largest
measured cost, and it is independently useful. B is the right destination and it arrives with the commit-gate
compilation the round already wants for another reason.

### T6. What owns the brace and code-span namespace?

- **A. One arbiter plus a literal escape, landed before the first query is authored; validation reads inline
  code spans as well as fences.** Effort: small; a parser change and one lint. Risk: low. Payoff: closes
  Pesce's findings 3 and 4 together, and covers the fourth population (minijinja examples in 38 inline-span
  sites) that neither panel counted. Forecloses: nothing.
- **B. Rely on `{{ }}` removal to dissolve the problem (Giesen's ruling).** Effort: none. Risk: the collision
  is not in `{{ }}`, it is in the inline code span, which the design newly claims for expressions and which
  today holds Jinja examples in mockspace's own templates. Removing `{{ }}` does not touch that. Payoff:
  none. Forecloses: nothing, but leaves the hazard live in the tool's own documentation.
- **C. Reserve a distinct sigil for expressions so inline spans stay free (`09_shape.md:166` already writes
  `@constant::froxel_range::value`).** Effort: small. Risk: two inline surfaces to document; the `@` form is
  already in the design and its escaping rules are already owed to Scribble's reader. Payoff: sidesteps the
  collision entirely rather than arbitrating it. Forecloses: nothing.
- **D. Forbid brace and expression literals in templates; require fenced examples.** Effort: small. Risk:
  the corpus already violates it in 38 places, including the crate whose subject is template syntax. Payoff:
  simplest rule. Forecloses: documenting this system in this system, which `09_shape.md:246` names a hard
  requirement.

Ranking: **A > C > B > D. Recommend A**, and note that the design already contains C's sigil, so the two
compose: arbitrate the namespace, and let `@` carry expressions where a code span would be ambiguous.

### T7. Do the query operators compose?

- **A. Treat `where`/`select`/`count`/`sort` as relational algebra with a compiled plan over fixed-size
  column arrays (Stachowiak).** Effort: medium. Risk: the framing is well understood but nobody has written
  it no-heap; row counts are static at gate time, which is what makes fixed sizing possible. Payoff:
  predicate and projection pushdown become standard rewrites rather than special cases; join order, the
  family proof and the seed all fall out of the same bottom-up summary; the index in T1 is the natural
  physical plan. Forecloses: nothing.
- **B. Method chain over a `Rows` value, evaluated left to right (the design as written).** Effort: none.
  Risk: `where` after `select` and `select` after `where` cost differently and nothing reorders them; the
  predicate grammar accretes as Muratori predicted, with comma-as-`and` first and no precedence story.
  Payoff: simplest to implement and to explain. Forecloses: pushdown, and any future join.
- **C. Method chain now, with the operator table shaped so a planner can be added later.** Effort: small.
  Risk: low; the table already carries arity, strictness and stage, so adding algebraic properties
  (commutes-with, distributes-over) is additive. Payoff: B's cost today with A's door open. Forecloses:
  nothing, provided the table lands.

Ranking: **C > A > B. Recommend C.** A is where this ends up and Stachowiak is right that it is the missing
frame, but the predicate grammar has to exist before it can be optimised, and Muratori's point that it should
be decided as a grammar rather than accreted as string splitting is the near-term work.

## Handoff to phase three

Phase three is charged with making the design work as intended even where earlier phases concluded it cannot.
Here is what I am handing over, sorted by how much of it is actually load-bearing.

### Settled

- **The corpus.** 2676 rows, 14 namespaces, 207 files, `spike/` alone carrying subdirectories. 4230 brace
  occurrences of which 4154 are genuine multi-segment addresses and 76 are the legacy placeholder population
  plus foreign-template examples. 132 per-crate `DESIGN.md.tmpl` files, which is the anchoring loop's
  multiplier. Any analysis using a one-level glob is wrong by 233 rows and 112 references.
- **The cost model, measured.** Load 18.8 ms of which parse is 14.0. Today's splices 0.12 ms. Anchoring loop
  2.3 ms. General query 229 ms. Index build 10.5 ms; indexed general query 3.4 microseconds. Parse dominates
  the anchoring workload; query dominates the general one; the design moves the corpus across that crossover.
- **The cargo cycle is not a blocker**, re-tested independently from the brief with the real topology. Distinct
  source ids resolve with both instances in the graph. The `[patch]` hazard is also real and I tested that
  half too: with the source ids collapsed, cargo emits `cyclic package dependency` and refuses. So the
  sidestep holds and the workflow rule Carmack asked for in his provocation 3 is genuinely owed, because the
  failure mode is a hard refusal rather than a degradation.
- **Root cause A stands.** Deleting `resolve_data` and typing the value domain is the one fix every panellist
  across both phases endorsed, and it is what makes the read set static, cycles a load-time graph property,
  and the substring bug unrepresentable.
- **The four decisions stand**, and decision 3 in particular should be treated as a constraint on T3 rather
  than as something a table reduction may quietly trade away.
- **Hash-consing survives**, re-keyed to a structural hash. It did not die in Stachowiak's ruling.
- **The incremental machinery is dead**: ReadSet fingerprints, environment-fingerprint protocols as a separate
  mechanism, comemo-style fixpoint detection, and Scribble's four passes. Quilez's immutability argument is
  decisive and it is stronger than the cost argument that preceded it.

### Genuinely blocked

- **The migration economics.** Pesce's finding 2 is untouched by anything in phase two and is the only
  finding in either phase that no mechanism dissolves. Five of six repositories have no registry, and the
  correlation between having one and having references is total. The language is the cheap last step. Nothing
  in phase two engaged with this at all, which is itself worth noting: an adversarial phase aimed at
  over-engineering found nothing to attack in the one finding about whether the corpus will ever exist.
- **The no-heap generator has no precedent.** `07_toolbox.md:238-246` states it and phase two did not
  disturb it. Every mechanism recommended above assumes fixed-capacity arenas sized from measured numbers,
  failing loudly at the bound. That is a plan, not evidence.
- **Whether variation-seeded phrasing is wanted at all**, which is empirical and only real documents answer.
  Note though that the churn counter that makes it measurable is now free: it is a comparison of two
  structural hashes.

### Suspected soft blockers, which is where I would spend phase three

These are the ones I believe are blocked only because nobody has looked hard enough, ordered by how much
they unblock.

- **"The index is expensive."** Nobody costed it before deferring it. It is 10.5 ms. I suspect the same is
  true of two other deferred mechanisms in this round, and the general lesson is that this round has
  repeatedly deferred things whose floor nobody measured while building things whose ceiling nobody measured.
- **"Templates are data, so the strongest proof degrades"** (`09_shape.md:216-221`). Giesen ranked Q3-B
  (commit-gate compilation) as the answer and then treated it as an operational nicety in step 5 of his
  sequence. Measured, it also removes the dominant cost of every render. It is simultaneously the proof fix
  and the performance fix and it is currently ranked as neither. I suspect it unblocks more than any other
  single item in the round.
- **"The query side is a method chain."** Stachowiak is right that nobody asked whether the operators
  compose, and I suspect the relational framing collapses more than he claimed: with typed columns and the
  index from T1, `where` is an index probe, `select` is a projection, and the family mask, the join order and
  the seed all fall out of one bottom-up summary. That is one mechanism where the design currently has four.
- **"Attributes uniform means constructors are rows."** I have argued the split above, but I only half
  trust my own answer. The question of whether compile-time renderer totality can be preserved under a fully
  data-driven algebra deserves someone actually trying to write it, because if it can, T3-B beats T3-A and
  the algebra stops being a design decision entirely.
- **The fourth brace population.** Thirty-eight inline code spans already hold braces, including in the crate
  documenting minijinja, and the design proposes claiming that surface. This is cheap now and a corpus
  migration later. It is the most concrete unaddressed item I found and it is a parser change plus a lint.

### One methodological note for phase three

The brief's worked example was about four reviewers building on an untested "cannot". I found the symmetric
failure in the brief itself: it asserted an untested "will", that `[patch]` collapses the instances and makes
the cycle real. I tested it and it happens to be true, and it was still an assertion when it was written.

The pattern across both phases is not that people assume constraints. It is that people **assert costs**.
Giesen asserted a nine-repository restructure was a prerequisite without running a two-minute cargo check.
Carmack asserted the index was a future cost without running a ten-minute bench. Stachowiak asserted the
identity collapse was free without doing the birthday arithmetic. Every one of these was an unpriced claim
about magnitude dressed as a structural conclusion, and every one took under fifteen minutes to settle.

Phase three inherits a design with unusually good measurements attached and should extend the habit rather
than the conclusions. When a phase-two file says a thing is cheap, or expensive, or free, that is the sentence
to execute against.
