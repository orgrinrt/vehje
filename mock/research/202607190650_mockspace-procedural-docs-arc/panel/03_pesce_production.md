# The design through the production and maintainability lens (Angelo Pesce)

## Verdict in one line

The design is unbuildable as written because two of its three named reuse targets sit on the far side of a
hard cargo cycle, and the migration it assumes does not exist: 1066 of the 1082 references live in one repo,
and that repo is the only one of nine with a registry at all.

## Where I agree and disagree with Muratori and Aaltonen

Agree with both on findings 1 through 3 of each. The `~` substring bug, missing absence, forbidden inline
iteration, and the missing fold memo are all real and all cheap now.

I disagree with the frame both share: they evaluate the language against the corpus as one population.
It is two. `ikiuni_renderer` holds 142 templates, 123,285 words, 1066 references and the only registry
(2676 rows). `arvo` holds 67 templates and 51,486 words and **2 references**. `hilavitkutin`: 50 files,
52,955 words, 2 references. `vehje`: 55 files, 0. `viola`: 13 files, 0. `ikiuni`: 111 files, 0. Aaltonen
sized the span table to 1082 occurrences and the arena to 280K words as if those met; they do not meet,
they sit in different repos with different problems, and every scale number in his table is really a
statement about one repo.

I also disagree with Muratori's "targets are a registry namespace" as stated, on ownership grounds. See
finding 3.

## What is strong

- **`render` is the single choke point** (`document.rs:254-269`), and the doc comment says why: "This is the
  only place either step runs, which is what stops a generation path from skipping one. Two of them did."
  A codebase that names its own past bug at the site of the fix is one that can absorb a new stage.
- **Resolution already produces a value, not a string** (commit `e37c7e5`). The `query` subcommand landed on
  `dev` two days ago and shares the template grammar, which is exactly the "test at a prompt, paste into a
  document" property `09_shape.md:253` asks for. It is already built.
- **References are stated, never guessed** (`refs.rs:39-52`). The auto-linking version was tried and removed.
  That decision is what makes a third syntax inside the same braces even conceivable.

## Findings

**1. mockspace cannot depend on `hilavitkutin-api` or `hilavitkutin-str`. It is a cargo cycle, not a
boundary.**

`hilavitkutin-api/Cargo.toml:17` lists `mockspace` as a **build-dependency** and `build.rs:2` calls
`mockspace::bootstrap::bootstrap_from_buildscript()`. So mockspace must build before `hilavitkutin-api`.
`08_decisions.md:69` proposes mockspace consume `hilavitkutin-api`'s `AccessSet`; `09_shape.md:43` proposes it
consume `hilavitkutin-str`'s `Str`. Cargo will refuse. This is not the `std`-boundary friction
`09_shape.md:223` names and half-dismisses; that section says mockspace has "no stack dependency at all" and
treats adding one as a naming exercise. It is a build-order impossibility, and it takes out the interner, the
node hash-cons table, and the semantic hash at once, since `09_shape.md:46-47` rests all three on `Str`.

The failing case is `cargo check` in `hilavitkutin/mock` on the day mockspace adds the dep.

Fix, and it is cheap if taken now: the extractable language crates carry their own `Str` and their own
membership traits with **zero stack dependencies**. `access.rs`'s `Contains`/`ContainsAll` is a sealed trait
plus one blanket impl; it is tens of lines, not a substrate. Reimplementing is correct here and reuse is the
error, which inverts the usual workspace rule for exactly one reason: the consumer is upstream of the
producer. Decide it now, because every artefact downstream of `08_decisions.md:69` is written assuming reuse.

**2. The migration is a registry-authoring project, and nothing prices it.**

The reason `arvo` has 2 references across 51,486 words is not authorial discipline. It is that `arvo` has no
`mock/registry/` directory. Only `ikiuni_renderer` does. So for five of six repos, "migrate the prose to
queries" means first extracting the facts from 157,000 words of hand-written prose into rows nobody has
written, in a schema nobody has designed for those domains, and only then writing queries. The language is
the last and cheapest step.

That reframes the whole cost. `ikiuni_renderer` did not migrate; it was authored against a registry from the
start, which is why it looks like the design already works. There is no migrated corpus anywhere to learn
from, and the one corpus that queries heavily is the newest and least settled in the workspace.

Fix: price it by doing one. Extract `arvo`'s registry and convert one `DESIGN.md.tmpl`, before the language
exists, using today's `{{ ns::slug }}` mechanism. The number that comes back is the real schedule, and it is
knowable this week.

**3. `{{ }}` carries three namespaces disambiguated by lookup order, and nothing keeps them disjoint.**

Today two things share the braces. `Placeholders` is a fixed 8-field struct (`render_design.rs:228-237`:
`project_name`, `mock_dir`, `crate_count`, `macros_table`, `primary_items`, `crate_layers`, `deep_dives`,
`crate_summaries`), applied first (`document.rs:265`). Registry references are found by "slot zero decides"
(`refs.rs:64-70`). A registry namespace named `deep_dives` would be silently shadowed by the placeholder pass
and never reach the resolver. The design adds queries into the same braces as a third population.

This is the two-sources-of-truth question asked directly: the placeholder field list and the registry
namespace set must stay disjoint by discipline, and nothing checks it. There is also deliberately no escape
for a literal (`render_design.rs:225-227`, "no template needs to write one today"), which stops being true
the moment documentation describes the query syntax.

This is where I disagree with Muratori's targets-as-a-namespace angle. It is a good idea about queries and a
bad idea about ownership: it puts the target list, which is code that must compile, into TOML that nine repos
can edit, and creates a fourth population in the same braces. Keep targets in Rust; the diagnostic
improvement he wants is a `Display` impl over the target set, not a registry table.

Fix: one arbiter that owns the whole brace namespace and errors on collision, plus the escape, before a third
population lands.

**4. A wrong query is invisible, because the validator does not read fences.**

`find_registry_refs` skips fenced blocks entirely (`refs.rs:41-48`), deliberately, so a document describing
the syntax does not trip the check. `validate.rs:306-312` and `resolve.rs:42-48` do the same. The design's
anchoring use case is a ```query fence (`09_shape.md:152-158`).

So the one check that catches a bad reference is structurally blind to the surface the design is built on.
Combined with Muratori's finding 1 (a substring `~` cannot fail) the answer to "what does a wrong query look
like before someone notices" is: **it looks like a correct document, indefinitely.** The dangling-reference
lint reports clean.

Fix: fences carrying queries are parsed, not skipped; the skip narrows to fences whose info string is not a
query language. One line, and it must land before the first query is authored, not after.

**5. Review of a generated document is not defined, and variation seeding makes it worse.**

Rendered output is committed, so the diff is the review surface. Decision 4 accepts that changing any fragment
argument re-rolls phrasing (`08_decisions.md:145-147`). So a semantic no-op produces prose churn in the diff,
and the reviewer cannot separate "a fact changed" from "the seed re-rolled". Aaltonen is right that root-id
comparison does not close this, because the answer depends on registry state the node identity does not
encode.

Fix: CI renders twice, once with variation pinned. The pinned render is the review diff and shows only fact
changes; variation lands unreviewed because it is by construction not a fact change.

## The migration question, answered concretely

There is no incremental path through the corpus, because the corpus is not the unit. The unit is the repo,
and a repo is binary: it has a registry or it has 51,000 words of prose and two references. Half-migration
within `ikiuni_renderer` is genuinely incremental and already happening (44 of its 142 templates still carry
zero references). Half-migration *across* repos is not incremental at all; it is five separate
fact-extraction projects that happen to share a tool.

Realistic cost, from the only evidence available: `ikiuni_renderer` carries 2676 rows against 123K words,
roughly one row per 46 words. Applying that to `arvo` and `hilavitkutin` alone implies ~2200 rows to author,
schema-designed per domain, before a single query is written.

## The half-landed failure mode

`ikiuni_renderer` is queried and correct by construction. The other five stay prose. This is the likely
outcome, and it is worse than either end state, for a reason specific to a workspace this size: the tool now
enforces two documentation dialects. The Tier 1 leakage checks, the dangling-reference lint, and the
doc-source-mismatch lint all mean different things depending on which repo you are in, and the maintainer
carries both paths forever. The contributor to `arvo` learns nothing transferable; the contributor to
`ikiuni_renderer` learns a language whose only corpus is the least stable repo in the workspace.

The tell that this has happened: `ikiuni_renderer`'s reference count keeps climbing and `arvo`'s stays at 2
for six months. That metric is free to watch and it is the one that matters.

## Honest sequencing

1. **Break the cycle.** Nothing else can be built until the language crates own their `Str` and their
   membership traits. Finding 1.
2. **One brace arbiter plus the escape, and fence-aware validation.** Findings 3 and 4. Both are small, both
   are impossible to retrofit cleanly once a third population is authored against the old rules.
3. **Extract one non-`ikiuni_renderer` registry by hand.** Finding 2. This is the schedule probe and it
   answers whether steps 4 onward are worth doing at all.
4. **Then the language**, with Muratori's absence, list-valued `~`, and inline-iteration fixes folded in from
   the first line, and Aaltonen's fold memo stated as a requirement.

Steps 1 and 2 are worth doing even if step 3 says stop, which is the property that makes this sequence
honest.

## Novel angles worth stealing

**The registry is the migration; the language is not.** The correlation between "has a registry" and "has
references" is total across nine repos, and it explains the corpus better than any authoring story. That
inverts the project: the expensive, uncertain, human part is fact extraction, and it is not a language problem
at all. Build the extraction assist first (scan prose for repeated noun phrases and cross-document restatement,
propose candidate rows), and the language becomes the cheap part it should be. Nothing in the prior art
survey covers this because every surveyed system arrived with its data model already given.

**Make an unresolved query degrade to its last rendered text, not to nothing.** Commit the previous render
next to each query as a cache. A query whose registry rows are not yet extracted renders the existing prose
with a staleness marker, which means a half-migrated document behaves exactly like today's corpus instead of
like a broken build. This is what makes migration genuinely incremental at the *sentence* level rather than
the repo level, and it is the only mechanism I can see that lets the five prose repos move at all without a
big-bang registry project each. It also deletes `tidy_after_drop` (`resolve.rs:86-111`) for the same reason
Muratori identified.

**Version the representation separately from the tool, as `pandoc-types` does** (`prior_art/03:3`). Today six
consumers pin mockspace by **git branch**: `arvo`, `viola`, `ikiuni_renderer` on `dev`; `hilavitkutin`,
`vehje`, `ikiuni` on `main`; and `dev` is 235 commits ahead. The same corpus is rendered by two different
generators depending on which directory you are in, with no version negotiation anywhere. A separately
versioned representation crate lets the tool move without moving the document semantics under six repos at
once.

## Open questions for the synthesiser

- **Whether the language crates live in the mockspace repo or their own.** Own repo gives independent
  versioning and a clean extraction story for vehje; mockspace repo avoids a tenth repo and a second release
  cadence for a maintainer already running nine. Both are consistent with finding 1's cycle break.
- **Whether the degrade-to-last-render cache is committed or derived.** Committed makes half-migration work
  and makes the diff honest; it also puts generated text in git next to its query, which is the duplication
  the project exists to remove.
- **Whether step 3's extraction probe runs before or after the language is specified.** Before gives a real
  schedule and risks specifying against one repo's schema; after risks building a language for a corpus that
  never arrives.
