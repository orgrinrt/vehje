# The procedural document language: senior synthesis and the way forward (Fabian Giesen)

**Date:** 2026-07-19
**Reads:** the three panel files, the artefacts `05` through `09`, the prior-art passes `01` through `08`,
the earlier-round reviews `02` and `03`, and the shipped source itself. Every `file:line` claim below was
re-verified against source rather than taken from a panellist, and where a panellist's citation did not
survive the check, that is stated. Two maintainer corrections received mid-synthesis (the three-context
syntax rule and the typed-reference-field direction) are incorporated and tested rather than assumed.

## The one-paragraph judgement

The design is sound at the spine and buildable, and the panel is right about every bug it names, but all
three experts stopped one level short of the diagnosis. Most of their findings are not independent defects;
they are two root causes seen from six angles. The first is **premature rendering**: `resolve_data`
(`src/registry/resolve.rs:662`) flattens every registry field to target-specific markdown at load time,
which single-handedly produces the substring over-match, the missing absence story, the forbidden inline
iteration, the impossibility of hashing a normalized form, and the `tidy_after_drop` workaround, and which
is flatly incompatible with decision 1's target-decided staging. The second is **missing per-occurrence
identity**: the design carries one identity (the hash-consed node) where it needs two (the deduplicated
value and the positional occurrence), and that one gap is Muratori's span problem, Aaltonen's
occurrence-count problem, and Pesce's review-provenance problem wearing three names; the shipped answer
(the green/red split of Roslyn and rowan) is already documented inside this round's own prior-art file.
The one genuine blocker, the cargo cycle, is real but mis-mechanised in the panel file, and the correct fix
is neither of the ones on the table: split the bootstrap shim out of mockspace and relocate the cons-list
set machinery down to `notko`, after which the workspace's own reuse rules are satisfied instead of
suspended. With those two root causes fixed at the foundation and the cycle broken by layering rather than
duplication, the four settled decisions stand, the check-before-emission guarantee survives conditional
content through a monotone two-level check, and the build order at the end of this file is the honest
sequence.

## Audit of the expert reads

### Muratori (API and accretion)

**Holds, verified against source.**

- Finding 1, the `~` substring over-match, is exactly as stated. `apply_methods` filters with
  `cell.contains(&want)` (`src/registry/resolve.rs:279`), `parse_predicate` is one field, one operator, one
  literal with no conjunction (`resolve.rs:340-346`), and `resolve_data` runs bottom-up before any document
  renders (`resolve.rs:26, 662`), so the cell being matched really is rendered markdown of the shape
  `[world](...)`. The registry really does carry the colliding names: `store` and `store-policy`,
  `plan-lower` / `plan-price` / `plan-schedule`, `ops-async-*` all exist as `{{ crates::... }}` values in
  `task/band_0.toml`. The finding's framing is also right: a query that silently over-matches is worse than
  the prose it replaced, because the prose at least got read.
- Finding 2, absence. Verified: exactly four rows across the registries carry the bare string
  `decided_by = "maintainer"` and every other `decided_by` is a `{{ bench::... }}` or `{{ spike::... }}`
  reference; `options` and `note` are present on some `task` rows and absent on others; `table_cells` folds
  a missing field to the empty string (`resolved.rs:211`, `unwrap_or_default`). The union case is real.
- Finding 3, inline iteration. The `blocked_by` list is real
  (`task/band_0.toml`: three-element arrays) and the value-operation fix
  (`map`/`join` through `Apply`, no `Iter`) is the correct move and consistent with the design's own bar at
  `09_shape.md:203`.
- Finding 5, the two-valued `Policy`. `09_shape.md:102` says `Reduce | Preserve`; `08_decisions.md:26-30`
  already describes a third behaviour. The predicate-over-node-kind fix is right and cheap now.
- The three-year break (family mask computed at parse over all branches, target policy deciding
  reachability later) is a genuine contradiction in the current wording of `08_decisions.md:96-98`, and
  naming it now rather than in year three is the single most valuable thing in his file. My resolution
  differs from his in shape (see Proposals, P5): the check does not need to move wholly inside evaluation.

**Thin.**

- Finding 4 treats the span question as open when this round already contains the shipped answer.
  `prior_art/07_rust_final_encoding_arenas.md` §8 documents rowan's red/green split as the one design that
  gives structural sharing of identical subtrees plus per-position identity, which is precisely the
  span-versus-deduplication conflict he flags. He identified the constraint correctly (spans attach to use
  sites, not nodes) and then left as an open question a thing three production toolchains have answered.
- The "where grammar" observation is correct but underpriced: the predicate grammar is not merely six forms
  to decide, it is the type system's entry point once fields are typed (see the corrections section below).

**Missed.**

- That findings 1, 2, and 3 share one root cause. Each is presented as a separate API gap; all three are
  downstream of `resolve_data` collapsing typed values (lists, absent fields, references) into rendered
  strings before any query can see them. Fix the flattening and finding 1's fix (list-typed `~`), finding
  2's fix (optional projection), and finding 3's fix (list-valued `map`/`join`) stop being three features
  and become one property of a typed value domain.
- The memo-correctness interaction with his own scope chain. He designs `self` resolution via a borrowed
  scope chain (`09_shape.md:174-181`) and separately praises hash-consing, without noticing that the two
  together make any node-id-keyed evaluation cache unsound: the same interned `where(crates ~ self)` node
  evaluates differently per document. Aaltonen inherits this miss (see his audit).

**Wrong.**

- The "hash-consing turns churn into a number computed in nanoseconds" angle, as pitched. Aaltonen's
  rebuttal is correct: root-id comparison detects call churn, not output churn, because registry state is
  not part of node identity. The metric is still valuable, but for a different question than the one he
  attached it to; the adjudication below splits it properly.
- Targets as an authored registry namespace. Pesce's ownership objection stands: a target is code that must
  compile, and its declared family set must be the same fact the trait impl proves, not a TOML row that can
  drift from it. The want behind the idea survives via reflection instead (Proposals, P8).

### Aaltonen (data and scale)

**Holds, verified against source.**

- The interner claims: `ArenaInterner` is a real two-method trait with a const-table-first wrapper
  (`hilavitkutin-str/src/interner.rs:14-19, 40-49`), so the survey's "no heap-free interner exists"
  conclusion is genuinely answered in-workspace.
- The quadratic scar: `resolve.rs:740-742` documents, in a shipped comment, that the whole-registry clone
  per row was quadratic at current corpus size and was fixed by building a per-row view. This is the
  strongest single piece of evidence in the panel that the naive shape of recursive resolution blows up at
  today's scale, not a hypothetical one.
- The occurrence-versus-node cost split is the correct frame and his headline is right: the arena shrinks
  with reuse while the span table, diagnostics, and unmemoized evaluation grow with reuse, and reuse is the
  stated purpose. The design must budget both counts.
- Finding 3's core claim: hash-consing without fold-level memoization is a storage optimization dressed as
  a compute optimization. Verified against the shipped analogue: `table_cells` does an O(rows x fields)
  scan per invocation (`resolved.rs:186-217`) and is invoked fresh at every bare-namespace or `where` site
  (`resolve.rs:442-449, 521`).
- The prior-art citations check out: LMS's `globalDefsCache` (`prior_art/01:39-49`), Typst's bolted-on
  `comemo` convergence (`prior_art/05:112-135`).

**Thin.**

- The corpus numbers are directionally right but not reproducible as stated. My counts: 2443 rows by
  `[[`-count across the fourteen namespace directories plus 10 in `vocab.toml` (his per-namespace figures,
  616 `reference`, 444 `technique`, 148 `task`, verify exactly; his 2676 total does not, off by roughly
  8%), and 275,591 words of `.md.tmpl` across the six repos against his 280,335. Nothing turns on the
  deltas, but a scale analysis should state its counting method, precisely because these numbers are about
  to become sizing inputs.
- The `Contains` arity bench is aimed at the wrong axis. Membership on a cons-list
  (`hilavitkutin-api/src/access.rs:66-70`) is linear trait recursion and will stay cheap at any plausible
  family-set depth; the axis that can actually hurt compile time is the one his own table names but the
  bench does not: the number of distinct target-times-family-set monomorphization instances. Bench that.

**Missed.**

- His own fix for finding 3 is incorrect as stated. "An explicit `NodeId -> Value` memo table" is unsound
  the moment a shared node contains a free variable: `where(crates ~ self)` interns to one node and must
  evaluate to different values under different documents. LMS gets away with sym-keyed caching because its
  binders are globally unique graph nodes, not re-bindable environment entries; this design has a scope
  chain, so it does not. The memo key must include the environment restricted to the node's free variables
  (Proposals, P4). This is the one place where following the panel as written would have shipped a
  correctness bug.

**Wrong.**

- "The variation-seed hash and the query-result cache are the same problem wearing two names"
  (his finding 4). They are two problems that must not share an invalidation key. The evaluation cache must
  invalidate when the registry changes under an unchanged call; the variation seed must do the exact
  opposite, staying fixed when the registry changes, or adding one task row would re-roll the phrasing of
  every fragment in the corpus, which is precisely the churn decision 4 exists to prevent. His observation
  that the cache, as specified, lacks registry-state invalidation is correct and important; his unification
  of the two keys would have destroyed the seed's one required property. The full adjudication is below.

### Pesce (production)

**Holds, verified against source.**

- The cycle is real. `hilavitkutin-api/Cargo.toml` lists `mockspace.workspace = true` under
  `[build-dependencies]` and its `build.rs` calls
  `mockspace::bootstrap::bootstrap_from_buildscript()`. A mockspace dependency on `hilavitkutin-api` is a
  package cycle cargo will refuse.
- The two-population corpus split verifies exactly: by my count `ikiuni_renderer` has 142 templates and
  123,285 words against `arvo`'s 67 and 51,486, and only `ikiuni_renderer` has a registry. The correlation
  between "has a registry" and "has references" is total across the repos, and his inversion (the registry
  is the migration; the language is the cheap last step) is the most consequential production insight in
  the panel.
- Fence-blind validation: `find_registry_refs` skips fenced blocks (`refs.rs:41-48`), and so does the
  resolver itself (`resolve.rs:47-55`). The design's anchoring use case is a fenced `query` block
  (`09_shape.md:152-158`). Wrong queries would today be invisible indefinitely; his one-line fix (narrow
  the skip to fences whose info string is not the query language) must land before the first query is
  authored.
- The branch-pin drift verifies exactly: `arvo`, `viola`, `ikiuni_renderer` pin mockspace at
  `branch = "dev"`; `hilavitkutin`, `vehje`, `ikiuni` at `branch = "main"`
  (`hilavitkutin/mock/Cargo.toml:29`); `dev` is 235 commits ahead of `main` by `git rev-list --count`. Two
  generators are rendering one corpus today. The separately-versioned representation crate
  (`pandoc-types` precedent, `prior_art/03:3-5`) is the right consequence.
- The review question (finding 5) is real and his pinned-render CI is the right half of the answer; the
  other half is provenance attribution (Proposals, P3).

**Thin, and the one overreach.**

- The cycle's mechanism, as cited, does not survive checking. His finding 1 claims the dependency boundary
  "takes out the interner, the node hash-cons table, and the semantic hash at once, since `09_shape.md:46-47`
  rests all three on `Str`". But `hilavitkutin-str` has **no** `build.rs` and **no** mockspace
  build-dependency; I verified its crate directory and manifest directly. The cycle to `Str` exists only
  transitively: `hilavitkutin-str` depends on `arvo` and `arvo-bits`, and those crates do carry the
  mockspace bootstrap build-dependency (verified in `arvo/mock/crates/arvo/Cargo.toml` and
  `arvo-bits/Cargo.toml`). The conclusion survives; the stated mechanism does not, and the difference is
  load-bearing, because the actual edge (a bootstrap shim pulled in as a build-dependency) is severable
  cheaply, while the edge he described would have forced his re-implementation fix. See the adjudication
  and Proposals, P6.
- Finding 3 (three populations in one brace syntax) is superseded by design: per the maintainer's
  correction, the `{{ }}` surface is legacy, and the proposal is three contexts with two syntaxes
  (reference-only fields carry bare typed references; prose-bearing fields and templates carry the one
  markdown grammar). Assessed against that, his finding converts from a design flaw into scheduled
  demolition work, and the part that survives is real: the `Placeholders` pass
  (`render_design.rs:228-237`), the slot-zero disambiguation (`refs.rs:58-66`), and the deliberate absence
  of a literal escape (`render_design.rs:225-227`) all still exist in shipped code, and the collision
  hazard is live until the legacy surface is removed. The removal belongs in the sequencing, not the
  design.

**Missed.**

- His own degrade-to-last-render proposal has the same ownership defect he correctly called on Muratori's
  targets-as-namespace: it commits machine-generated text into a human-owned source file, next to the query
  it shadows. The want (sentence-level incremental migration) is right; the vehicle should be an
  author-owned fallback instead (Proposals, P7).
- That the typed-reference-field correction makes his "the registry is the migration" finding sharper: once
  reference-bearing fields are typed, migrating a repo is schema authoring plus fact extraction, and the
  schema is machine-checkable from day one. His probe (extract one `arvo` registry by hand) becomes the
  schema-design exercise, not merely a cost estimate.

## Where two experts conflict, adjudicated

**1. Muratori's churn counter against Aaltonen's rebuttal.** Both are half right, and the resolution is
that there are two keys, not one. Define **CallHash** as the semantic hash of the normalized call term
(alpha-normalized, spelling-normalized, nested references resolved to typed ids, per the finding at
`09_shape.md:229-241`; registry *state* excluded). Define **ResultKey** as CallHash combined with a
fingerprint of the registry rows actually read during evaluation. Muratori's root-id comparison is a
CallHash diff: it measures exactly the cost decision 4 accepted (phrasing re-rolls when a call changes) and
it does close that open question with a counter, as he claimed. It says nothing about output churn, as
Aaltonen showed. Aaltonen's demand for registry-state invalidation applies to ResultKey and the evaluation
cache only. Verdict: adopt both metrics under their correct names; reject the framing on each side that
collapsed them into one.

**2. Aaltonen's "same problem, two names" against decision 4.** Adjudicated for decision 4. The variation
seed must be stable under registry edits, or the mechanism defeats its own purpose; the evaluation cache
must be invalidated by them. Two keys, one shared prefix (CallHash), by contract. Aaltonen's underlying
finding (the cache as specified in `09_shape.md` has no invalidation story at all) stands and is fixed in
P3.

**3. Muratori's targets-as-registry-namespace against Pesce's ownership objection.** Pesce wins on
ownership and Muratori wins on the want. Targets stay declared in Rust, where the family set is a type and
the totality obligation is checked. The build then **reflects** those declarations into a generated,
read-only registry namespace, the same way the doc index is generated today. Queries and diagnostics get
Muratori's improvement ("`DefList` is unsupported by `plain`; supported by `markdown`, `html`") from data
that cannot drift because it is derived, and no editable population is added anywhere. Neither expert
proposed the reflection; it is the synthesis of the two objections.

**4. Pesce's two-population framing against the other two's single-corpus arithmetic.** Pesce holds.
Aaltonen's span-table and arena sizing are statements about `ikiuni_renderer` alone, and Muratori's
first-day authoring pains are statements about the one repo that can author queries at all. Every scale
number in this round should carry the repo it is about. This does not weaken either expert's findings; it
scopes them.

## The synthesis: where the lenses intersect

### Root cause A: premature rendering

The maintainer's mid-synthesis hypothesis was that the flattening in `resolve_data` is a common root cause
under several panel findings. Tested against source, it holds, and the blast radius is larger than the
hypothesis stated.

`resolve_data` (`resolve.rs:662-693`) rewrites every registry field, bottom-up, into **rendered markdown
strings** before any document is processed, via `resolve_once` on each field value (`resolve.rs:770`) and
`Resolved::to_markdown` (`resolved.rs:61-74`). Everything downstream sees strings. Consequences, each of
which surfaced in the panel as an independent finding:

- **Muratori 1.** `~` substring-matches over `[world](120_WORLD.md)` because the list of typed references
  was joined and rendered before the predicate ran. On typed values the collision is unrepresentable.
- **Muratori 2.** Absence collapses to the empty string (`resolved.rs:211`) because a string domain has no
  way to say "not present". A typed domain keeps the distinction for free.
- **Muratori 3.** Inline iteration over `blocked_by` is impossible because the list stopped being a list at
  load time; `as_scalar` even documents the join (`resolved.rs:141-146`).
- **The recursive-reference finding** (`09_shape.md:229-241`). The design says the semantic hash must be
  taken over the normalized form after nested references resolve. But the only "normal form" the current
  pipeline produces is markdown, which is target-specific; a semantic hash over it would change whenever a
  renderer's spelling changed. Normalization and rendering must be different operations, and today they are
  one.
- **Decision 1 itself.** Target-decided staging is incoherent while the data every target reads has one
  target's rendering baked in at load time. `resolve_data` is not an implementation detail the new design
  wraps; it is the thing the new design deletes.
- **Pesce's `tidy_after_drop` observation and Muratori's "diagnose the papercut".** The punctuation-repair
  pass (`resolve.rs:86-111`) exists because `Hidden` renders as an empty string inside prose that was
  already flattened; with typed values and staged rendering, the drop happens before punctuation is
  assembled and the pass deletes itself, as both experts noted from different directions.

The boundary of the root cause, stated so it is not over-claimed: it does not cover the cargo cycle, the
migration economics, the fence-blind validator, or the occurrence-identity problems. Those are separate.

The maintainer's second correction extends the reach one step further, and the extension holds: under the
three-context rule, a prose-bearing registry field (`what`, `note`, a fragment body) is a document fragment
in the same grammar as a template. That means such a field lowers into the same arena, carries its own
family set, and participates in the target-support check exactly as an embedded fragment does. The current
flattening forecloses that too: a field pre-rendered to markdown can never report which families it uses.
One consequence worth stating explicitly because nobody has: **the family set of a document is the union of
its own set and the sets of every fragment and prose field it transitively embeds**, and that union is only
computable during evaluation, which independently confirms the resolution of Muratori's three-year break
(P5): the exact mask is an output of the staging fold, not of the parse.

### Root cause B: missing per-occurrence identity

Three findings are one concept. Muratori: spans must attach to use sites because interned nodes are shared
(his finding 4 and his first open question). Aaltonen: the span table, diagnostics, and unmemoized
evaluation cost are occurrence-bounded while the arena is node-bounded (his findings 1 and 3). Pesce: a
reviewer needs to know which occurrence of which call produced a changed hunk (his finding 5). The design
currently has one identity, the hash-consed node id, and needs two:

- **Value identity** (green): the deduplicated, position-free node. Small, arena-bounded, the correct key
  for structural equality, CSE, and (environment-qualified, see P4) evaluation memoization.
- **Occurrence identity** (red): this appearance of that value, at this position, under this parent. Corpus-
  bounded, the correct key for spans, diagnostics, and review provenance.

This is not an invention; it is Roslyn's and rowan's green/red split, documented in this round's own
`prior_art/07` §8 as the one representation giving structural sharing plus positional identity at once. The
panel walked past its own prior art. What is ours to do is the no-alloc adaptation (P2), which is easier
than the general case because a generation-time arena has one lifetime and the red layer never needs
mutation.

### The corrections, tested

The typed-reference-field direction verifies against source and closes more than it was offered for.
`ikiuni_renderer/mock/mockspace.toml` already declares per-field types (`[[registry.namespace.field]]`,
`name = "crates"`, `type = "string[]"` at lines 427-430) and already declares reference roots
(`[ref.roots.seed]` at line 383). Two distinct reference target kinds exist in the data, exactly as the
correction said: namespace rows (`{{ crates::world }}` in `task/band_0.toml`) and root-relative source
lines (`provenance = ["seed::ROADMAP::98"]`). So the schema surface for `type = "ref<crates>[]"` and
`type = "ref<seed>[]"` is an extension of a mechanism that is already there, not a new mechanism, and once
it lands, Muratori's finding 1 is not fixed but dissolved: membership on typed ids cannot substring-collide,
and the checker can verify at load time that every reference in a typed field targets its declared
namespace, which no amount of operator redesign over strings could do.

## Proposals: building beyond the experts

### P1. A typed registry value domain, and the deletion of `resolve_data`

Registry loading stops producing strings and starts producing typed values against the declared schema:
scalars, `Ref<Ns>` ids (interned, 4-byte), lists of them, and prose fields parsed by the one markdown
grammar into the same arena as templates. `resolve_data` and the `Resolved` enum's role as the value domain
are deleted outright, per the pre-1.0 no-shims rule; the eight-variant `Resolved`
(`resolved.rs:18-57`) survives only as the `query` subcommand's presentation layer until the target model
subsumes it. Rendering becomes what it should always have been: the last fold, per target, over typed
values. The earlier-round finding that check-time rewrites `Project` into concrete accesses
(`02_giesen_ir.md`, finding 2: `StaticRef` / `FieldLoad` / `ColumnProject`) composes directly: the field's
declared type is what the projection rule consults, `Project` on an optional field yields the optional, and
the registry family contributes `or(default)`, `has`, `map`, `join` as value operations, exactly the shape
Muratori asked for, now grounded in schema rather than convention. The two-tier storage split from
`03_aaltonen_representation.md` finding 1 (scalar columns plus copyable offset handles into a byte arena)
is the storage layout underneath; it was right then and nothing in the panel disturbs it.

Cost: this is the large lift of the project, but it is the same lift the language needs anyway; there is no
separate "fix the current resolver" work item, because the current resolver's core is the thing being
replaced. Risk: migration of the 1000-plus existing references; mitigated because the typed loader can
accept the legacy `{{ }}` spelling during the demolition window while the validator reports it.

### P2. The green/red arena

Two parallel structures, one allocation each, both index-addressed:

- **Green**: the hash-consed node arena as designed (`09_shape.md:21-26`), 4-byte ids, `EntityList` child
  lists, constructor-time interning. Unchanged.
- **Red**: an occurrence table built in parse order: `(green_id: u32, span: (u32, u32), parent_occurrence:
  u32)`. Twelve to sixteen bytes per occurrence. Diagnostics, the family-mask refusal message, and review
  provenance address occurrences; evaluation and memoization address green ids. At the current corpus
  (roughly one thousand live references, growing with fragment reuse) the table is tens of kilobytes;
  at a hundred-fold growth it is megabytes, and it is the one structure whose growth is linear in corpus
  rather than in unique structure, so it is stated in the cost model as Aaltonen demanded: **budget the
  arena by unique-node count and the red table by occurrence count, and size the fixed backing of each
  separately.**

Tokenization is fixed at the same time, answering Aaltonen's finding 2: `Text` is run-granular (one node
per run between markup boundaries, which is what Pandoc's reader does in practice per his own citation),
never word-granular. That choice moves the node-count floor for the existing corpus from hundreds of
thousands of leaves to tens of thousands, and it is now stated rather than implied.

### P3. The two-key contract, and provenance-attributed review

The caching and variation story becomes one contract with two keys and three consumers:

- **CallHash** = semantic hash (Dhall's normalize, alpha-normalize, canonically encode, hash, per decision
  4) of the call term after typed-reference normalization. Excludes registry state by construction.
- **ReadSet** = the registry rows and fields actually read during evaluation of the call, tracked by the
  fold (the loader computes one content hash per row at load; the read set is row ids plus those hashes).
- **ResultKey** = CallHash x ReadSet fingerprint.

Consumers: (1) the **variation seed** is CallHash, exactly decision 4, now with the stability property
provable rather than hoped; (2) the **evaluation memo** and any cross-run cache key on ResultKey, which
closes Aaltonen's finding 4 correctly; (3) the **review surface** runs Pesce's double render (one pinned,
one live) and then labels every changed hunk of the pinned diff by cause: CallHash changed (an author
edited a call), ReadSet fingerprint changed (a fact changed in the registry, and the responsible rows are
named because the read set holds their ids), or neither (a generator change; flagged loudly, because that
is either a renderer fix or a bug). The reviewer reads a diff in which every hunk says why it exists.

Homework on novelty: read-tracked invalidation is Salsa, comemo, and Adapton; input-attributed difference
explanation is [nix-diff](https://github.com/Gabriella439/nix-diff), which recursively descends shared
inputs to name the root cause of a derivation change. Neither, nor anything a search surfaced, applies the
combination to rendered-prose review with per-hunk cause labels, and nothing surfaced does seeded
deterministic phrasing variation keyed to a semantic content hash at all; the nearest neighbours are NLG
surface realizers (no stability contract) and content-spinning tools (seeded, no semantics). The
combination is ours.

### P4. The memo key that is actually sound

The fold memoizes by **green id plus environment fingerprint of the node's free variables**, not by green
id alone. Cheap by construction: the interning constructor computes each node's free-variable summary as
the union of its children's minus its binders, O(1) per construction, stored beside the node. A closed node
(empty summary) memoizes globally; an open node keys by the values of exactly its free variables, which for
the dominant case (`self`) is one interned id. The shared-fragment cost Aaltonen worried about then lands
where it truthfully belongs: a `self`-dependent query evaluates once per distinct binding of `self`, which
is the actual amount of sharing that exists, and byte-identical calls across documents share one entry.
Without the environment qualifier, the memo is a cache-poisoning bug across documents; with it, hash-consing
finally delivers the compute saving its headline promised. This correction is the price of admission for
Aaltonen's otherwise-correct finding 3, and it must be stated as a requirement next to his.

### P5. The monotone family check: cheap where possible, exact where needed

Resolution of Muratori's three-year break that keeps the typestate and the phase boundary for the common
case:

1. **Parse-time mask** (as designed, `08_decisions.md:96-98`): a bitmask over every construct present in
   any branch. This is a sound over-approximation of every possible staged mask.
2. **Fast accept**: if the parse mask is within the target's declared mask, emission proceeds; the
   over-approximation makes this sound, and it is one bitwise operation, unchanged from the design.
3. **Exact path**: only when the fast accept fails, the staging fold (which runs anyway) accumulates the
   exact mask over reachable, reduced content, including the transitive union over embedded fragments and
   prose fields from root cause A. If the exact mask fits, emit; if not, refuse, naming the construct and
   its occurrence span (P2) and, via P8, the targets that do support it.

`Checked<T>` keeps its single public constructor; internally its evidence is an enum of the two proof
routes. The three-year feature ("this section only renders for the internal target") then works on the day
it is asked for: the internal-only branch's `Table` fails the fast accept against `plain`, the exact pass
sees the branch is untaken under `plain`'s policy, and the document emits. No rewrite, no weakened
guarantee, and the guarantee's statement stays honest: inclusion is proven against what the target will
actually be handed, which is the only version of the claim that was ever true.

### P6. Break the cycle by layering, not duplication

The cycle's true shape, established above: mockspace is the build-root of the entire stack because every
stack crate's `build.rs` build-depends on **the whole mockspace crate** to call one bootstrap function
(`hilavitkutin-api/build.rs`, `arvo/build.rs`), while the language work needs mockspace to consume crates
from the top of that same stack. The minimal severing move is to make the build-dependency honest about its
size:

1. **Extract `mockspace-bootstrap`**: a zero-dependency crate carrying `bootstrap_from_buildscript` and the
   hook/alias generation (`src/bootstrap.rs` is file IO and `env!` paths; it is self-contained today).
   Stack crates repoint their `[build-dependencies]` to it. Mechanical, one small PR per repo.
2. **Relocate the cons-list set machinery to `notko`**: `Empty`, `Cons`, `Contains`, `ContainsAll`,
   `Concat` (`hilavitkutin-api/src/access.rs:48-90`) are foundation primitives, not engine machinery; the
   workspace's own vocabulary already names cons-lists as a workspace-level concept. `notko` has no
   `build.rs` and zero dependencies (verified), so nothing it is moved into can cycle. `hilavitkutin-api`
   rebinds to the relocated types outright, no shims, per the pre-1.0 rule. The engine's `AccessSet`
   remains its own sealed trait over them.
3. The language crates then depend on `notko` (sets, fallibility), `arvo` (numerics), and
   `hilavitkutin-str` (`Str`, the interner contract) with no cycle, because the only back-edges left in the
   graph point at the zero-dependency bootstrap shim.

This dominates Pesce's fix (re-implement `Str` and the membership traits with zero stack dependencies) on
every axis that matters here: it keeps one implementation of each primitive, it satisfies
`use-the-stack-not-reinvent` instead of suspending it, and it fixes the layering for every future consumer
that wants to sit below the stack's build-root, of which this project is merely the first. His fix remains
the fallback if the bootstrap split hits an unforeseen wall, and his instinct that *something* structural
had to give was right; it is the tool's layering that gives, not the reuse rule.

### P7. Author-owned fallback, not committed render cache

Sentence-level incremental migration, Pesce's goal, through a construct rather than a cache:

````markdown
```query
for t in task::where(crates ~ self) { ... }
``` else ```
The store crate currently owes three walking-skeleton items.
```
````

(Concrete syntax to be settled with the grammar; the semantics are the point.) The fallback block is the
existing prose, kept by the author, rendered verbatim while the query resolves to nothing or its namespace
is not yet extracted, and superseded the moment the query resolves. A lint counts sites where a query
resolves but a fallback still exists: that count is Pesce's "the tell that half-migration happened" made
mechanical, per repo, for free. Ownership stays clean (no machine-written bytes inside human-owned source),
the diff at migration time is an explicit reviewed replacement, and `tidy_after_drop`'s reason to exist
disappears here too.

### P8. Reflect the tool into its own registry

The build generates a read-only namespace from the Rust target declarations: one row per target, its
declared family set, its policy, its cluster. Diagnostics query it (adjudication 3). Then go one step
further than either expert: reflect the **language itself** the same way, one generated namespace each for
the core forms, the families, and the document algebra constructors. The documentation of this system is
then written in this system against generated rows that cannot drift from the source, which is the
project's one-source-of-truth thesis applied to the project. Compilers print target lists and dialect docs;
none surveyed feed them into a queryable registry consumed by the same document language, so this
composition is available and cheap: it is the doc-index generation pattern that already exists, pointed at
new data.

## Open questions, each with at least three costed options and a recommendation

### Q1. The dependency layering (the cycle)

- **A. Bootstrap split plus notko relocation (P6).** Effort: small-medium; one new tiny crate, nine repos'
  build-dep bump, one type relocation with mechanical rebinds. Risk: low; every step is mechanical and
  independently verifiable by `cargo check` across the workspace. Payoff: cycle gone, reuse rules intact,
  layering fixed for every future below-the-build-root consumer. Forecloses: nothing.
- **B. Language crates re-own `Str` and the set machinery, zero stack deps (Pesce).** Effort: small.
  Risk: permanent double implementation of two foundation primitives, drift between them, and a standing
  suspension of the workspace reuse rule that will be cited as precedent. Payoff: unblocks immediately
  without touching other repos. Forecloses: sharing the interner's const-table path and any future
  arvo-typed surface in the language crates without a second migration.
- **C. Vendor-by-copy with a sync lint.** Effort: small now, unbounded later. Risk: the copies *will*
  drift; a lint that diffs vendored source against upstream is a standing tax. Payoff: none over B.
  Forecloses: clean extraction.
- **D. Host the language crates inside the hilavitkutin repo.** Effort: small. Risk: couples the language's
  release cadence to the engine's, the most active repo in the workspace; contradicts the extraction aim.
  Payoff: no new repo. Forecloses: independent versioning, the `pandoc-types` property Pesce established a
  concrete need for.

Ranking: **A > B > D > C. Recommend A.** B is the honest fallback if the bootstrap split surfaces a hidden
dependency; time-box the split attempt to confirm.

### Q2. Where the family check runs

- **A. Parse-time mask only (the design as written).** Effort: none. Risk: the three-year break arrives
  with the third target; conditional content refuses documents whose offending branch is unreachable.
  Payoff: simplest statement. Forecloses: target-conditional content, which decision 1's own hybrid
  paragraph already promises.
- **B. Check wholly inside the staging fold.** Effort: medium. Risk: gives up the cheap pre-check; every
  emission pays the exact bookkeeping; `Checked<T>` loses its clean phase story, as Muratori noted.
  Payoff: exact everywhere. Forecloses: refusing obviously-bad documents before evaluation starts.
- **C. Monotone two-level (P5): parse mask as sound over-approximation with fast accept, exact mask in the
  staging fold on fast-accept failure.** Effort: medium-small; the fold accumulates a bitmask it is already
  in a position to compute. Risk: low; soundness of the fast path is a two-line argument from
  over-approximation. Payoff: exactness only where needed, phase boundary preserved for the common case,
  refusal messages carry occurrence spans. Forecloses: nothing.

Ranking: **C > B > A. Recommend C.**

### Q3. Ahead-of-time template compilation (the open question from `09_shape.md:264` and `08_decisions.md:171`)

- **A. Parse templates at every render (status quo of the design).** Effort: none. Risk: the family check
  stays a run-time event in the render path; authoring errors surface latest. Payoff: simplest. Forecloses:
  nothing, but leaves the decision-2 gap open permanently.
- **B. Compile at the commit gate: parse, type-check against the registry schema, compute masks, serialize
  the green/red arenas (flat `u32` tables serialize trivially) keyed by content hash.** Effort: medium.
  Risk: low; the gate already parses everything it checks. Payoff: an authoring error becomes a commit-gate
  refusal, which in this workspace *is* the compile error, since the gate is where discipline already
  bites; renders load pre-checked arenas and start at the fold. The decision-2 gap closes in the sense that
  matters: no unchecked document can reach a render, because unchecked documents cannot reach the repo.
  Forecloses: nothing; A remains the cold-start path for uncommitted work.
- **C. Codegen templates into Rust statics via `build.rs`, making the family proof a genuine rustc
  compile-time bound.** Effort: large. Risk: every template edit becomes a rustc rebuild of a generated
  crate; authoring latency couples to the compiler; the generated surface must itself be maintained.
  Payoff: the pure static proof, closing decision 2's caveat in the strongest sense. Forecloses: fast
  authoring iteration, and it drags document data into the build graph, re-creating a small cousin of the
  cycle problem.

Ranking: **B > A > C. Recommend B.** The hour of thought the shape document asked for lands here: the gap
between "compile error" and "gate error" is immaterial in a workspace whose entire quality model is
gate-enforced, so C's costs buy a distinction without a difference.

### Q4. The variation seed's ergonomics (decision 4's open condition)

- **A. CallHash default, optional `seed = <expr>` argument, zero syntax when unused.** Effort: small.
  Risk: the accepted cost stands (argument changes re-roll phrasing), now measured by the CallHash churn
  counter rather than guessed. Payoff: decision 4 as settled, with its open ergonomic condition satisfied
  literally. Forecloses: nothing; the override is the escape hatch.
- **B. Fragment-identity hash only (exclude arguments).** Effort: small. Risk: two calls to the same
  fragment in one document choose the same variant, which defeats the corpus-reads-as-a-form-letter goal in
  the aggregate case; loses per-call variety exactly where fragments repeat most. Payoff: maximal stability
  under argument churn. Forecloses: per-call variation without explicit seeds everywhere.
- **C. Seed from the calling row's slug or crate name (positional).** Effort: small. Risk: this is the
  document-path mistake one renaming away; the round already watched ordering prefixes shift
  (`05_correction_and_aim.md:89-94`). Payoff: stable under argument changes, varied per site. Forecloses:
  stability under the renames that demonstrably happen.

Ranking: **A > B > C. Recommend A**, with the CallHash churn counter (adjudication 1) run in CI from day
one so the accepted cost is a monitored number, and with B available as a per-fragment declaration
(`vary_by = "identity"`) if a fragment's arguments prove churn-heavy in practice; that per-fragment knob is
additive later and costs nothing now.

### Q5. The fold memo's shape

- **A. No memo.** Effort: none. Risk: occurrence-multiplied re-evaluation; the shipped `table_cells` cost
  at every call site, already visible at today's scale per the quadratic scar. Payoff: none. Forecloses:
  the compute half of hash-consing's promise.
- **B. `NodeId -> Value` (Aaltonen's fix as written).** Effort: small. Risk: **unsound** under the scope
  chain; cross-document cache poisoning through shared open terms. Payoff: none that survives correctness.
  Forecloses: correctness.
- **C. `(green id, free-variable environment fingerprint) -> Value` with constructor-time free-variable
  summaries (P4).** Effort: small-medium. Risk: low; the summary is O(1) per construction. Payoff: sound
  memoization delivering exactly the sharing that exists. Forecloses: nothing.
- **D. Full read-tracked dependency memoization (comemo-shaped) over evaluation and cross-document
  passes.** Effort: large. Risk: building general incremental-computation machinery for a batch generator.
  Payoff: subsumes C and powers the Scribble-style cross-document fixed point's convergence detection,
  which `09_shape.md:128-135` already plans to borrow from Typst. Forecloses: nothing; C is its inner loop.

Ranking: **C now, D as the already-planned growth path, B never, A never. Recommend C**, with the read-set
tracking from P3 implemented alongside it (the two share the tracked-read plumbing, so C plus P3 is most of
the way to D when the cross-document pass lands).

### Q6. The document algebra's membership (`Note`, `Math`, and the list)

- **A. Ship the proposed twenty-one, with red catalogue tests naming the dropped constructors.** Effort:
  none beyond the tests. Risk: a genuinely needed constructor arrives late; mitigated because adding a
  node-contributing constructor is exactly the priced, justified act the family rule exists to gate.
  Payoff: smallest total fold surface; every renderer's totality obligation stays minimal. Forecloses:
  nothing, by design.
- **B. Add `Note` and `Math` now.** Effort: small (two constructors, three renderer arms each). Risk:
  carrying constructors the corpus does not use; checked against the data, the `equation` namespace's 66
  rows carry prose descriptions (`computes`, `used_by`) and references, not math markup, and no footnote
  syntax surfaced in the templates, so today's evidence for both is thin. Payoff: insurance. Forecloses:
  nothing, but pays fold arms in every renderer forever.
- **C. Adopt Pandoc's full thirty-five.** Effort: medium. Risk: fourteen constructors with no current or
  foreseeable use, each a permanent totality obligation on every renderer; Pandoc carries them because it
  serves thirty input formats, which this does not. Payoff: maximal expressiveness. Forecloses: the small
  total fold that makes hand-written renderers cheap, which is the design's own cost model.

Ranking: **A > B > C. Recommend A**, with the workspace's catalogue-edge-cases discipline doing the real
work: one ignored red test per dropped constructor, asserting the intended lowering, so the day `Math` is
needed the gap is a one-flip test rather than a discovery.

### Q7. What a registry query means in code destined for a runtime target (the effect intent)

- **A. Always generation-time: queries reduce under every policy, and the reduced value must lower to a
  target literal.** Effort: small. Risk: silently freezes a fact into emitted code with no marker that it
  was live data once; the "either exactly right or silently wrong" problem the toolbox named
  (`07_toolbox.md:216-219`) decided by fiat. Payoff: simplest semantics. Forecloses: ever shipping live
  data access without a semantics change.
- **B. Forbidden in Preserve-policy targets unless annotated.** Effort: small. Risk: friction on the
  common, correct case (most queries in emitted code really do want the generation-time value); annotations
  accrete. Payoff: nothing silent. Forecloses: fragment reuse across markdown and code targets without
  per-use annotations, which cuts against the whole point of fragments.
- **C. The family declares its stage (P-shaped: `Registry` is a generation-stage family by declaration);
  the checker enforces cross-stage persistence, meaning a reduced value entering residual code must be
  representable as a target literal, with rows and tables required to be projected to scalars or rendered
  fragments first, and a named diagnostic otherwise.** Effort: medium. Risk: low; this is decision 1's
  declared-predicate machinery (`08_decisions.md:32-34`) applied by the family author once rather than by
  every call site. Payoff: the silent-wrongness class is eliminated structurally; the design's spine ("a
  target declares, everything else is a proof obligation against the declaration") gains its fourth face,
  as the decisions file almost said. Forecloses: nothing; see D.
- **D. C now, plus a later additive `RegistrySnapshot` family whose stage is residual: targets that want
  live data at their own runtime declare support for it, and the emitter serializes the queried slice
  beside the emitted code.** Effort: C plus a later medium. Risk: scope; only worth it when a real consumer
  (a Lua mod reading design constants at load, which is vehje's world) asks. Payoff: the door C leaves
  open, opened. Forecloses: nothing.

Ranking: **C > D-later > B > A. Recommend C**, recording D as the anticipated extension so the family
namespace is named with it in mind.

### Q8. Where the language crates live

- **A. In the mockspace repo, own cargo workspace, with a lint enforcing zero dependencies on mockspace
  itself from day one; extract to their own repo at the first external consumer (vehje).** Effort: none
  now, mechanical later (the lint guarantees `git mv` is the whole extraction). Risk: low; the lint is the
  load-bearing part. Payoff: no tenth repo and no second release cadence for a maintainer running nine,
  until a consumer exists that needs one. Forecloses: nothing, provided the lint lands with the first
  crate.
- **B. Own repo immediately.** Effort: small but real (rulesets, branch policy, pin management,
  a tenth entry in every cross-repo chore). Risk: cadence overhead paid before any consumer benefits.
  Payoff: the `pandoc-types` versioning property from day one; the 235-commit branch-drift finding shows
  the workspace does feel representation-version skew. Forecloses: nothing.
- **C. Inside the vehje repo.** Effort: small. Risk: couples the document language to the language project
  whose stall history the seed itself documents (`00_seed.md:210-227`); mockspace would then pin vehje,
  inverting the intended dependency direction. Payoff: co-location with the eventual IR consumer.
  Forecloses: mockspace moving independently of vehje's cadence, which the last two months of history say
  it must.

Ranking: **A > B > C. Recommend A.** The drift Pesce found argues for B eventually; "eventually" is spelled
"first external consumer", and the lint makes the move date a rename rather than a project.

### Q9. The migration probe's sequencing (Pesce's step 3)

- **A. Probe before the language is specified.** Effort: days. Risk: the schema gets designed against one
  repo's domain and the language then over-fits it; also serializes the schedule. Payoff: a real cost
  number earliest. Forecloses: nothing, but delays everything behind a manual exercise.
- **B. Probe after the language ships.** Effort: same, later. Risk: the language's registry-facing
  decisions (field types, reference kinds, prose-field semantics) get locked against a single corpus,
  `ikiuni_renderer`, the newest and least settled repo, which is exactly the over-fit Pesce warned about.
  Payoff: probe runs with better tooling. Forecloses: schema feedback into the type-system design, which is
  where it is worth the most.
- **C. Run the probe concurrently as the schema-design exercise: extract `arvo`'s registry by hand against
  the typed-reference field schema while the language crates are being built, using today's mechanism for
  rendering.** Effort: the same days, overlapped. Risk: some rework if the schema shifts under it; small,
  because the schema surface (typed fields, two reference kinds) is the part already settled by the
  correction. Payoff: the cost number, the second corpus, and live schema feedback into the check rules,
  all before the language's registry semantics lock; Pesce's "before gives a real schedule and risks
  specifying against one repo's schema" objection dissolves because the probe *is* the second repo's
  schema.

Ranking: **C > A > B. Recommend C.**

## The bottom line for the maintainer

**What is ready and stands.** The four decisions of `08_decisions.md`, unchanged. The nine core forms,
with the typed-projection amendment (projection yields the field's declared type, optional fields yield an
optional) and no new forms; every panel finding that looked like a missing form resolved into a value
operation, which is the family rule working. The document algebra at twenty-one with red catalogue tests
for the dropped constructors. The inclusion-not-coverage spine, now stated in the version that survives
conditional content.

**What must land first, in order, because everything else builds on it.**

1. **The bootstrap split and the notko relocation (P6/Q1-A).** Nothing that consumes the stack can be
   written until the cycle is dead, and this fix also repairs the layering permanently. Smallest hard
   piece, largest unblocking.
2. **Fence-aware validation and the legacy-surface demolition** (Pesce's finding 4 fix; retirement of the
   `Placeholders` pass and the `{{ }}` brace surface per the three-context correction). Both are small,
   and both are impossible to retrofit cleanly after the first real query is authored against the old
   rules.
3. **The typed registry schema and the deletion of `resolve_data` (P1).** This is the root-cause fix; the
   substring bug, the absence gap, the inline-iteration gap, and the normalization-before-hashing
   requirement all fall out of it rather than being fixed severally. The `arvo` extraction probe runs
   concurrently as the schema exercise (Q9-C).
4. **The green/red arena, the core, and the fold with the sound memo key (P2, P4) and the monotone mask
   (P5).** The language proper, built once, on the two identities and the two-level check, so neither
   root cause is rebuilt into the new foundation.
5. **The two-key contract and the review pipeline (P3), the commit-gate compilation (Q3-B), and the
   reflected namespaces (P8).** The operational layer that makes the thing trustworthy in production:
   caches that invalidate correctly, prose variation that never lies in a diff, refusals that name their
   occurrence and their alternatives, and documentation of the tool written in the tool against data that
   cannot drift.

**The risk to watch** is the one the toolbox named and no panellist dwelt on: the heap-free generator has
no shipping precedent (`07_toolbox.md:238-246`), and the pieces above lean on exactly the machinery
(fixed-capacity arenas, the red table's occurrence growth, the byte-arena value tier) where that novelty
lives. The mitigations are already in the plan: both counts budgeted separately, fixed backings sized from
measured corpus numbers with stated methods, and the `Contains`-monomorphization bench pointed at the axis
that can actually hurt. Where a capacity has to be guessed, guess loudly, fail loudly at the bound, and
record the bound as a registry constant so the documents about the system report it. That, too, is the
thesis: one source of truth, queried, correct by construction.

## Sources consulted beyond the round's artefacts

Verified in-workspace: `mockspace/src/registry/{resolve,resolved,refs}.rs`, `mockspace/src/document.rs`,
`mockspace/src/render_design.rs`, `mockspace/src/bootstrap.rs`, `mockspace/Cargo.toml`,
`hilavitkutin/mock/crates/{hilavitkutin-api,hilavitkutin-str}/`, `hilavitkutin/mock/Cargo.toml`,
`arvo/mock/crates/{arvo,arvo-bits}/Cargo.toml`, `notko/mock/crates/notko/`,
`hilavitkutin-api/src/access.rs`, `hilavitkutin-str/src/interner.rs`,
`ikiuni_renderer/mock/mockspace.toml`, `ikiuni_renderer/mock/registry/` (row counts and field shapes),
per-repo template counts by `find`/`wc`, branch pins by manifest grep, `git rev-list --count main..dev` in
mockspace. External: [nix-diff](https://github.com/Gabriella439/nix-diff) (input-attributed derivation
difference explanation; the nearest prior art to P3's provenance-labelled review and cited as such).
