# The design through the API and accretion lens (Casey Muratori)

## Verdict in one line

The core is right-sized and the target contract is the best idea in the document, but the author-facing
surface has no story for absence, no story for inline iteration, and no story for source provenance, and
those three are what a real registry makes you reach for on the first day.

## What is strong

- **Inclusion instead of coverage** (`08_decisions.md:60`). This is the correct inversion and it is worth
  more than the document claims. Pandoc's writers drop silently because totality forced them to accept
  everything (`08_decisions.md:78`); declaring a smaller set turns a silent degradation into a named refusal.
  Everything downstream of this call gets better.
- **Value-contributing families as the default** (`09_shape.md:32-38`). The rule that a family wanting new
  node kinds must justify itself is the single line that keeps the fold from rotting over ten years. The
  registry family passing that test (`09_shape.md:206`) is the proof it is a real bar and not a slogan.
- **`Resolved` already separates lookup from rendering** (`registry/resolved.rs:1-12`, `to_markdown` at 61,
  `to_terminal` at 82). The shipped code already learned the lesson the design is generalising, and
  `resolve_typed` (`registry/resolve.rs:456`) is already the one-lookup-two-renderers shape. This is not a
  speculative architecture; it is an extraction of something working.
- **Cycle detection names the whole cycle, not the re-entry point** (`registry/resolve.rs:713-724`). That is
  a failure path someone actually thought about, and it is the standard the rest of the failure paths below
  do not yet meet.

## Findings

**1. The anchoring use case is silently wrong on the real data.**

`09_shape.md:154` is `task::where(crates ~ self)`. Today `~` is `cell.contains(&want)`
(`registry/resolve.rs:279`) over a cell that has already been resolved to rendered markdown
(`resolve_data` at 662 runs before `table_cells`). So the cell is literally
`[world](120_WORLD.md), [store](...)`. The real crate names in `ikiuni_renderer/mock/registry/task/`
include `store` and `store-policy`, `audio` and `audio-dsp`, `plan-lower`/`plan-price`/`plan-schedule`,
`ops-async-constraint`/`ops-async-constitutive`. `where(crates ~ store)` returns the `store-policy` rows.
It also matches anything whose *link target* contains the substring. The document that is supposed to be
correct by construction is wrong, and nothing reports it, because a substring match cannot fail.

The cost is the whole premise. The reason to query instead of restating is that the query cannot drift; a
query that quietly over-matches is worse than the prose it replaced, because prose is at least audited by a
human reading it.

Fix: `crates` is a list, so `~` must be membership over list elements, not substring over a joined rendered
string. That means resolution must keep list-ness (`Resolved::List` exists at `resolved.rs:47` and is
already thrown away by `as_scalar` at 141), and `~` must be defined on `List × Ref`. Substring stays
available as a distinct operator with a different spelling, so the two questions are not one.

**2. There is no form for absence, and the data is full of it.**

The nine forms (`09_shape.md:54-64`) have no way to ask whether a field is present. The real rows need it
constantly: `options` is present on some `task` rows and not others, `note` likewise, `blocked_by` likewise.
`decided_by` is worse than optional, it is a heterogeneous union: four rows carry the bare string
`"maintainer"` and the rest carry `{{ bench::… }}` or `{{ spike::… }}` references.

Today `Project` on a missing field returns `None` and the reference is left literally in the output
(`registry/resolve.rs:10-12`), which is correct for a typo and exactly wrong for a field that is legitimately
absent. The author's only recourse is to not write the sentence. `Select` cannot help, because there is
nothing it can test.

This is the papercut that becomes a grievance. Every fragment that mentions `note` or `blocked_by` has to be
written twice or not at all.

Fix: `Project` yields an optional value and the registry family contributes `or(default)`, `has`, and a
`Select` that accepts the optional directly. No new core form, which is the bar `09_shape.md:203` sets. The
union case (`decided_by`) additionally needs the type rule at `Project` check time (`09_shape.md:60`) to
admit a sum, or it will reject real data that resolves fine today.

**3. Inline iteration is forbidden and immediately necessary.**

The two-surface split (`09_shape.md:249-251`) reasons that "a loop cannot render mid-sentence, so control
flow is inherently block-level." The reasoning is sound about *control flow* and wrong about *iteration*.
`blocked_by = ["walking_skeleton", "geometry_channel_distance_retype", "depth_channel_reverse_z_infinite_far"]`
wants to render as "blocked by X, Y and Z" inside a sentence. That is iteration producing inline content,
mid-sentence, which the split forbids and which the data demands on the first document.

The author's workaround is to break the sentence into a bullet list, which changes the prose to suit the
tool. That is the tool fighting its user in month one, not year three.

Fix: keep the split exactly as stated, and make inline iteration a *value* operation rather than a control
form: `blocked_by::map(fragment)::join(", ", " and ")` is `Apply` over the registry family, needs no `Iter`,
and stays inside a code span. This is the same move as finding 2 and the same move the design already made
for `where`/`select`/`count`. The split survives because the case that seemed to violate it was never
control flow.

**4. Total desugaring buys a small core and pays in diagnostics, and the bill is not accounted for.**

`09_shape.md:66` adopts Jsonnet's discipline: nothing survives as sugar. `prior_art/02_jsonnet_dhall.md:117`
records the documented cost of exactly that choice: "errors can point at desugared code." The design takes
the mechanism and does not take the mitigation.

Concretely: an author writes `task::where(crates ~ self)::select(id, what)` and gets an arity or type error
reported against `Apply(Project(Apply(Project(task, where), …), select), …)`. There is no line, no column,
and no span, because a 4-byte node id (`09_shape.md:21`) carries none. For a documentation tool whose users
are writing prose and not debugging compilers, that is the difference between a usable tool and one people
stop using.

Fix: the arena carries a parallel span table indexed by node id. It is one `u32` pair per node, it is
allocated once, and it is the thing that makes every other error message in this document actionable.
Decide it now, because retrofitting spans through a hash-consing interner is unpleasant: identical subtrees
deduplicate (`09_shape.md:24-25`) and therefore share a node id, so span must be attached at the *use site*,
not the node. That constraint is invisible until you try, and it is cheap now.

**5. `Policy` is two-valued in one file and three-valued in the other.**

`09_shape.md:102` says `Policy: Reduce | Preserve`. `08_decisions.md:26-30` establishes that a Lua target
preserves control flow *by default* while an interpolation hole inside it is generation-time. That is a
third policy, "preserve control flow but reduce interpolation," and it is the one every real code-generation
target wants. A two-valued enum in a design that opened everything else is the one closed thing left, and
the closed thing is already wrong.

Fix: policy is a predicate over node kind, not an enum. `Reduce` and `Preserve` become the two constant
predicates and the Lua case becomes expressible without a third variant being invented under pressure later.

## The three-year feature request that breaks this

**"This section only renders for the internal target."**

It arrives the moment there are three targets, and it breaks the family-inclusion proof, which is the spine
(`08_decisions.md:157`).

The proof for parsed documents is a family bitmask "computed during parsing" checked against the target's
const bitmask (`08_decisions.md:96-98`). Parsing is unconditional: it sees every branch. So a document with
a `Select` whose internal arm emits a `Table` and whose public arm emits a `Para` carries `Table` in its
mask permanently. The plain-text target that legitimately cannot express tables refuses the document, even
though under its own policy the table branch is unreachable and never evaluates.

The fix is to compute the mask over *reachable* branches, and that is the break: reachability is decided by
staging, staging is decided by the target's policy, and the check is supposed to gate emission which happens
after staging. The check must move inside evaluation, and once it does, `Checked<T>`'s single-constructor
typestate (`08_decisions.md:98`) no longer describes a phase boundary. The guarantee does not disappear, but
its shape changes, and it changes at the exact point where the most code has already been written against
it.

Name it now and the resolution is cheap: the mask is per-node, not per-document, and the check is a fold
that runs with staging rather than before it. Discover it in year three and it is the rewrite.

## What is sized for a small tool

- **The `where` predicate grammar.** One field, one operator, one literal (`registry/resolve.rs:340-346`).
  There is no conjunction. The real query in `09_shape.md:194` already needs one:
  `where(crates ~ crate, milestone ~ band)`. Comma-as-`and` will be added under pressure and then `or` will
  be wanted and there will be no precedence story. Decide the predicate grammar as a grammar now; it is six
  forms and it will otherwise accrete as string splitting.
- **`Registry` is `BTreeMap<String, Row>` with `String` field values.** At the real scale already present
  (2500 rows across fourteen namespaces, 616 in `reference`, 444 in `technique`) this is fine. The design's
  interner (`09_shape.md:43`) fixes it. The thing that does not get fixed by the interner is that a *field
  value* is a string even when it is a list, which is what finding 1 is downstream of. Type the values.
- **Method dispatch is a `match` on `(name, &value)` with `_ => return None`**
  (`registry/resolve.rs:270-331`). An unknown method and a method applied to the wrong kind produce the same
  nothing. Two failures, one message, neither actionable.

## Novel angles worth stealing

**Targets are a registry namespace.** The design already has an open family set, a declared support set per
target, and a query language over registry rows. Make the target list a namespace and all three connect: a
target row declares its family set as data, "which targets can render this document" becomes an ordinary
query in the language being designed, and the refusal diagnostic for an unsupported construct can *name the
targets that do support it* by querying rather than by a hand-maintained message table. Adding a target
becomes a row plus a renderer impl, and the checker's error messages improve for free as targets are added.
No surveyed system does this because none of them have a queryable registry sitting next to the target
table. This one does, and it is already built.

**Hash-consing turns the variation-seed question from empirical into measurable.** Decision 4 accepts an
unknown cost, that changing a fragment argument re-rolls phrasing, and says "real documents will answer"
(`08_decisions.md:146`). But node construction goes through an interner (`09_shape.md:24`), so structural
equality is a 4-byte comparison and "did this document actually change" is a root-id comparison. Regenerate
against the previous run's root ids and the churn is a *number*, computed in nanoseconds, gateable in CI
before anyone reads a diff. The open question closes with a counter instead of a season of observation, and
the mechanism is already paid for.

**Diagnose the papercut, not the reference.** `tidy_after_drop` (`registry/resolve.rs:86-111`) is a
punctuation repair pass for references that resolve to nothing. It is careful, well-reasoned code for a
problem that only exists because absence has no representation (finding 2). When absence is a value, an
author writes the fallback and the repair pass deletes itself. Worth noting because it is the clearest
signal in the shipped source of what the language is missing: the codebase built a workaround good enough
that the gap stopped being visible.

## Open questions for later panellists

- **Spans against interning.** A span table indexed by node id conflicts with deduplication, so spans belong
  to use sites and the arena needs a second index. That is a data-layout question with an ergonomics forcing
  function, and the two of you should not answer it independently.
- **Where the reachability check runs.** Moving family checking inside staging fixes the three-year break
  and costs the clean phase boundary that makes `Checked<T>` easy to explain. Both are real; the tradeoff is
  between a guarantee that is simple to state and one that is true under conditional content.
- **Whether `Resolved`'s eight variants survive as the value domain, or dissolve into families.** Typst
  carries 28 (`07_toolbox.md:18`), this carries eight, and the design does not say which of the two numbers
  it is aiming at. That is a layout question as much as an API one.
