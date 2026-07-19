# The claims ledger (Naty Hoffman)

**Date:** 2026-07-19
**Reads:** the phase-four brief and all three phase-four files, all of phase three, all of phase two, all of
phase one including the survey and `prior_art/01` through `08`, and the artefacts `05` through `10`.
**Method:** every claim below that can be executed against was executed against, on this machine, today. Where
I could only check a citation by reading it, that is marked. Where I could not settle a figure, I say so and
give what I measured rather than picking a number. Three figures in this document contradict phase four
itself; one contradicts a correction the coordinator made to phase four.

## The one-paragraph judgement

Phase four did the job it was briefed for and the result is a demotion: of the five things this round has been
calling novel, three have prior art that shipped between eighteen and nineteen years ago, one survives only
after being narrowed from a field to a subfield, and one survives two independent searches intact. The design
is not thereby weakened, but its justification changes shape entirely, and the round has not noticed yet: it
is no longer "we are building something nobody has built", it is "we are applying four established mechanisms
to a domain that demonstrably lacks all four, and composing them in a way nobody has composed them". That is a
defensible and interesting claim and it is a different claim, and it must be restated before it reaches a
design document. Underneath that, the phase found something worse than a citation error: the determinism
promise that three phases of argument rest on is falsified by shipped code, verified, in every generated
document in the workspace, and the commit gate that was supposed to be the compensating control is
client-side, opt-in, and not re-run by any server. Against that, phase four's own factual record is only
partly clean. The reference count is now settled at 6628 by three independent reproductions including mine.
The row count is settled at 2676. But the file count in Fiedler's determinism finding is wrong, the
coordinator's correction of it is also not reproducible, and I found a fifth uncaught count error sitting
directly underneath the round's single largest result: **there is no population of 132 documents in this
corpus**, and 132 is the multiplier every anchoring measurement in phases two and three is stated in.

## Audit of the three

### Wronski (algorithmic grounding)

**Holds.**

- **The egg correction is right and I verified it independently.** Willsey, Nandi, Wang, Flatt, Tatlock,
  Panchekha, "egg: Fast and Extensible Equality Saturation," POPL 2021, Distinguished Paper
  ([POPL 2021 programme](https://popl21.sigplan.org/details/POPL-2021-research-papers/23/egg-Fast-and-Extensible-Equality-Saturation),
  [ACM](https://dl.acm.org/doi/10.1145/3434304)). One nuance worth recording because it sharpens the lesson:
  Stachowiak's **URL was correct**; he linked `10.1145/3434304`, which is the POPL paper. Only the venue label
  was wrong. So the error propagated through a correct link that nobody clicked, which is a more specific
  failure than "a bad citation propagated" and a more useful one to design against.
- **The deflation of adjudication 1 is the phase's best single piece of reasoning.** Graefe 1993
  ([ACM Computing Surveys 25(2)](https://dl.acm.org/doi/10.1145/152610.152611)) carries build-on-the-smaller-
  relation as standard material, and his framing is exactly right: an immutable in-memory relation does not
  present a new special case of Selinger-style planning, it removes the condition under which the estimation
  half of Selinger's problem exists at all. Phase three stated the absence of a problem as a discovery.
- **The birthday arithmetic recomputes exactly.** At n = 12,676 and w = 32, n²/2^(w+1) = 160,680,976 /
  8,589,934,592 = 1.87 percent. At w = 64, 4.36e-12. Both match. This is one of the few numbers in the round
  that has now been derived twice from the formula rather than quoted.
- **The DBSP scoping is the methodologically sharpest paragraph in phase four.** "DBSP shows the derivative is
  computable from the *query plan*, not from a schema" is precisely the move the brief asked for: state what
  the source establishes, then mark where the claim leaning on it goes further. Every ledger entry in this
  document tries to copy that shape.
- **Magic sets** (Bancilhon, Maier, Sagiv, Ullman, PODS 1986) is a genuine contribution rather than a
  correction: it is the formal frame for exactly Karis's move, and it composes past the depth-2 case where the
  two-integer chooser stops. Nobody else in four phases opened the Datalog literature for this.

**Thin.**

- **He inherits the round's largest result rather than executing against it, in the phase that exists to stop
  inheritance.** "The measured table is taken as established per the dispatching agent's confirmation." The
  brief names the dispatching agent as having made four verification errors, two while correcting an expert.
  Taking a measurement table as established on that party's confirmation is the exact move the phase forbids.
  What he *did* verify is the technique's preconditions (plan-before-render at `document.rs:19-23`, 3147 of
  3148 references indexable), and that verification is real and valuable. The numbers themselves remain
  unreproduced by anyone.
- **The snapshot entry undersells its own best point.** He correctly observes that the 45x is bought by
  skipping the parse and is not specific to the hand-rolled format, so the Q-B recommendation is a
  dependency-surface argument, not a performance one. Fiedler later supplies the argument that actually
  disqualifies the alternatives (a no-alloc consumer cannot run a validation or fix-up pass at load). Wronski
  was one step from it and stopped at "dependency surface".

**Missed.**

- **The 132-document multiplier.** He re-derived the inversion's preconditions and did not check the
  population the headline is stated over. It is wrong. See the corrections register, C6.
- **His own snapshot finding moots his own memoization entry.** If the snapshot carries the resolved edges and
  the index, the memo-key discussion concerns the 20 percent of a number 45x smaller than measured. He
  establishes both halves and does not put them together, so the round still ranks memo soundness as a
  live design question when it is now a small term inside a small term.

**Wrong.** Nothing material. His one procedural overreach is presenting the variation-seed verdict as settled
after a single search; it is Wyman's independent second search that actually earns that entry, and Wronski
could not have known it was coming.

### Wyman (novelty)

**Holds, and this is the file that changes the design's story.**

- **The four-lineage grounding of pre-emission target checking is the strongest single act of grounding in the
  phase.** SPIR-V `OpCapability` with `spirv-val --target-env`, `javac --release` against `ct.sym`, Rust
  `#[target_feature]` (RFC 2045), and browserslist with `doiuse` / `stylelint-no-unsupported-browser-features`.
  Four independent lineages, one of them (CSS) over a declarative document language, which is as close as the
  adjacent field gets. And he preserves the claim that survives rather than discarding the whole thing: the
  mechanism is established, its application to document conversion is not, and DITA constraint modules and
  Sphinx's `unknown_visit` are correctly excluded with reasons.
- **The Racket correction is a self-contradiction inside the round, uncaught for three phases.**
  `06_prior_art.md` asserts "Racket declines the many-output half entirely" while the round's own
  `prior_art/04_racket_scribble.md` documents `text-render` (322 lines), `markdown-render` (381 lines),
  `html-render` and `latex-render` as mixins over one `render<%>` interface, plus `#lang` supplying many input
  grammars over one expanded core, plus `content?` as a first-class value. The survey contradicted its own
  deep pass and the contradiction was load-bearing under a novelty claim.
- **Re-aiming the relational inversion from egglog to .QL is correct and I verified both citations.** de Moor,
  Verbaere, Hajiyev, Avgustinov, Ekman, Ongkingco, Sereni, Tibble, "Keynote Address: .QL for Source Code
  Analysis," SCAM 2007 ([dblp](https://dblp.org/rec/conf/scam/MoorVHAEOST07.html),
  [PDF](https://codeql.github.com/publications/ql-for-source-code-analysis.pdf)); Avgustinov, de Moor, Jones,
  Schäfer, "QL: Object-oriented Queries on Relational Data," ECOOP 2016
  ([DROPS](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2016.2)). His point that this is
  independent of the venue error is right: egglog unifies Datalog with equality saturation, which is a
  rewriting result, and does not establish storing the tree relationally.
- **The Hofer citation is correct and I tried to correct it and was wrong.** I believed the third author was
  Tillmann Rompf and checked before writing. It is Tillmann Rendel: Hofer, Ostermann, Rendel, Moors,
  "Polymorphic Embedding of DSLs," GPCE 2008, pp. 137-148
  ([ACM](https://dl.acm.org/doi/10.1145/1449913.1449935),
  [Tübingen](https://ps.informatik.uni-tuebingen.de/publications/hofer08polymorphic/)), Most Influential Paper
  at GPCE 2018. Recorded because this round's failure mode is confident correction, and I nearly added an
  instance of it to the file whose subject is that failure mode.
- **6628 reproduced.** Third independent count, mine included. See C2.
- **The "what the prior art learned that this round has not" section is the most actionable content in phase
  four.** Three-state support (MDN carries partial-implementation and version-ranged support; caniuse carries
  "supported with known bugs"), capability dependencies (SPIR-V's implicit declaration, which is why the
  declaration stays short as the vocabulary grows), and `ct.sym` as the shape a versioned external target
  forces. All three are consequences drawn from shipped systems rather than from taste, and all three are free
  now and expensive after the family bitmask locks.

**Thin.**

- **Both of his load-bearing line cites are wrong.** He gives `06_prior_art.md:53` for "nothing found covers
  logic and markup" and `:65` for the Racket sentence. Verified: they are `:52` and `:45` respectively. The
  substance is exactly right and the line numbers are not, in a round whose entire discipline is `file:line`
  and whose central pathology is claims that were never opened. Low stakes, same class.
- **The no-heap generator is named as surviving novelty without being searched.** He performs real searches for
  two claims and asserts this one, calling it "still zero precedent and still the largest unbacked claim". It
  is the largest unbacked claim, and after four phases it remains unbacked in both directions: nobody has
  searched for a precedent and nobody has established there is none. It cannot sit in NOVEL on that basis. See
  U1.

**Missed.**

- **He declines to verify 1007 and it reconciles in one command.** Measured across the workspace: 1082 total
  `{{ }}` occurrences in `.md.tmpl` files, of which 1007 contain `::`. Same population, two definitions, the
  75-occurrence difference being the single-segment placeholder tail. This is precisely the reconciliation
  Tatarchuk already performed for the registry side, and it closes the last open corpus figure.
- **He states that Scribble is the honest picture of the endpoint and does not draw the consequence.** If a
  system with the value model, many renderers and twenty years in production already occupies the niche, and
  its documented failure mode is fidelity diverging silently between backends, then this design's actual
  contribution is **the check**, not the representation. That reframes the whole justification, it is his own
  finding, and he leaves it as a note about a better motivating example.

**Wrong.** Nothing material.

### Fiedler (contracts)

**Holds, and every executable claim in it verified exactly against source and live infrastructure.**

- **The determinism falsification.** Verified: `render_design.rs` `now_rfc3339()` (at :479-490 on the current
  file) runs `Command::new("date").arg("-u")` and on any failure substitutes the literal `"unknown"`, so
  output content depends on `PATH`; `generation_header_md` (:104-123) writes `Generated at: {timestamp}` into
  every generated header; `grep -rn SOURCE_DATE_EPOCH src/` returns nothing. All three confirmed.
- **Leg 1, promoted from assumption to fact.** `gh api repos/orgrinrt/arvo/rulesets` returns
  `trunk-protection`, `enforcement: active`, conditions `["~DEFAULT_BRANCH", "refs/heads/dev",
  "refs/heads/main"]`, rules `deletion`, `non_fast_forward`, `pull_request`. Reproduced exactly.
- **Leg 4, and it is the finding of the phase on the operational side.** Reproduced exactly: the only ruleset
  carrying `required_status_checks` is `main-source-restriction`, and its condition is
  `["refs/heads/main"]`. So no server-side check runs on `dev`. And `bootstrap.rs` `activate()` sets
  `core.hooksPath` via `git config --local`, which is not cloned. The checking is entirely client-side and
  opt-in, and the single server-side leg enforces process only. His reading is correct and his one-line fix
  (add `required_status_checks` on `dev`) is the correct fix.
- **Leg 2, verified including the mechanism.** The generated hook derives `CHANGED_CRATES` from staged paths
  under `$MOCK_DIR/crates/`, and when that is empty passes `--scope infra`; `lint/mod.rs:129-135` then
  `continue`s past every crate not in scope. So staging only registry rows does not merely fail Arntzen's red
  test, it routes to a different scope entirely, exactly as he says.
- **The two-version snapshot correction is the sharpest structural finding in phase four.** Three consumers
  with three different skew tolerances (cache: zero, regenerate silently; interchange: high, refuse loudly;
  no-alloc: zero, cannot upgrade in place) cannot be served by one version field, and SQLite's read-version /
  write-version pair is the right precedent. Giesen's adjudication 5 consolidated correctly and specified one
  field too few.
- **The stronger argument for Q-B-A.** A no-alloc consumer cannot run a validation or fix-up pass at load, so
  rkyv's validation layer is disqualified structurally rather than on dependency-surface taste. This is a real
  improvement on the reasoning the round actually made.

**Thin.**

- **The file count is wrong, and the correction to it does not reproduce either.** See C7. His per-repo figures
  for arvo (27), hilavitkutin (19), vehje (19) and `ikiuni_renderer` (74) match a git-tracked-at-HEAD count
  exactly, so his method was tracked files; he then reported viola as 12 where tracked HEAD gives 0, and
  omitted `ikiuni` (60) and `mockspace` (2) entirely. The claim's direction is certain. Its magnitude is not
  established by him, by the coordinator, or by me, and three defensible populations give three different
  numbers.
- **The SWHID citation overstates by one step.** A SWHID core identifier addresses content, directory,
  revision, release or snapshot; line ranges are a *contextual qualifier* (`;lines=`) layered on the
  identifier, not part of the intrinsic hash. The immutability guarantee is on the blob. This does not damage
  the epoch-pin proposal, which is exactly what `git show <sha>:<path>` provides, but the citation should not
  be read as "SWHIDs hash lines".

**Missed.**

- **He verifies the gate and the determinism failure separately and never composes them.** They interact, and
  the interaction is the actual operational failure: every regeneration dirties every generated document, the
  hook's scope derivation runs over staged paths under `mock/`, and so a contributor either stages hundreds of
  pure-noise diffs or does not stage them at all. Finding 1 does not merely falsify a promise; it makes the
  gate's input meaningless, which is a fifth leg nobody has named.
- **Deleting the timestamp may not be free.** `render_design.rs` carries a predicate testing whether content
  `.starts_with("Generated at:")` (at :99-102), which suggests the field participates in the tool's own
  regeneration or idempotence logic. His provocation 1 (delete the field) is the right destination and it is
  not a one-line change, and someone acting on it as written will find that out at the worst moment.

**Wrong.** The 151 figure. Nothing else material.

### The finding that applies to all three

**None of the three re-ran a single benchmark.** Phase four executed against citations, against shipped source,
and against live GitHub infrastructure, and it did all three well. It did not execute against the measurements,
which are the round's largest single result and the basis of every cost claim in phases two, three and four.
The brief said "execute against factual claims rather than inheriting them; this round's record makes inherited
claims unsafe". The measured table is an inherited claim, and it is the one carrying the most weight. It is
also the one whose stated population I have now shown to be wrong.

---

# The ledger

Claims are grouped by domain and given stable IDs so they can be cited individually. **E** = established,
**N** = novel, **U** = unsupported. "Rests on it" names the design decisions that fall if the claim falls.

**Domains:** A cost model and query evaluation, B identity and caching, C the target guarantee, D
representation, E data and migration, F contracts and determinism, G corpus facts.

## The ledger: ESTABLISHED

### A. Cost model and query evaluation

**E-A1. Loop inversion over an enumerable free variable turns a per-observer scan into one group-by.**
Citation: hash-join and semi-join batching, Graefe, "Query Evaluation Techniques for Large Databases,"
*ACM Computing Surveys* 25(2), 1993, [10.1145/152610.152611](https://dl.acm.org/doi/10.1145/152610.152611);
application-layer twin is the N+1 fix, [DataLoader](https://github.com/graphql/dataloader) (Byron, 2015).
*Covers:* evaluation cost, given an enumerable bound and an indexable predicate.
*Stops at:* render and IO, which stay linear in documents; and at any predicate that is not equality or
membership on a typed key.
*Rests on it:* Q-A (the `where`/`filter` grammar split), T1 (unconditional index), the entire "query cost at
scale" resolution.

**E-A2. Choosing a two-relation join order by comparing two exact cardinalities is the standard build/probe
heuristic, not a new result.** Citation: Graefe 1993, hash-join section. The contrast case is Selinger et al.,
"Access Path Selection in a Relational Database Management System," SIGMOD 1979,
[10.1145/582095.582099](https://dl.acm.org/doi/10.1145/582095.582099), whose dynamic programming exists
because page counts are estimated.
*Covers:* the mechanism, and why estimation is absent here.
*Stops at:* depth 3 and beyond, where a small exact DP is needed and where magic-sets rewriting (Bancilhon,
Maier, Sagiv, Ullman, PODS 1986) is the composing frame the round has not used.
*Rests on it:* adjudication 1, Q-H-A, T7-C. **Note the demotion:** phase three's "removes the entire hard half
of classical query planning" states the absence of a condition as an insight. Restate.

**E-A3. Full recompute is a legitimate mode, not a fallback.** Citation: Blakeley, Larson & Tompa, "Efficiently
Updating Materialized Views," SIGMOD 1986, restated in DBSP.
*Covers:* the mode's legitimacy when incrementalizing costs more than recomputing.
*Stops at:* saying anything about when this corpus crosses over.
*Rests on it:* Carmack's full-refresh reduction, which survives.

**E-A4. A query's derivative is computable statically from the query plan.** Citation: Budiu, Chajed, McSherry,
Ryzhyk, Tannen, "DBSP: Automatic Incremental View Maintenance for Rich Query Languages," VLDB 2023,
[PDF](https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf).
*Covers:* derivative from the plan.
*Stops at:* "derivable from a schema", which is this design's restricted case (typed field reads, no joins) and
is an application of DBSP, not a result DBSP states. It also stops well short of the four-way ground-truth
split, which is the round's own construction.
*Rests on it:* the structural build-the-index-always criterion that replaces Carmack's stopwatch threshold;
adjudication 2's dirty set (partially, see N-A1).

### B. Identity and caching

**E-B1. Content addressing by recursive structural hash.** Citation: [Unison](https://www.unison-lang.org/docs/the-big-idea/).
*Covers:* identity independent of position and encounter order.
*Stops at:* collision handling, which Unison's width makes moot and this design's does not.
*Rests on it:* T2-C, decision 4's seed stability, the churn counter.

**E-B2. Hash-then-verify on insert is the required collision discipline.** Citation: Filliâtre & Conchon,
"Type-Safe Modular Hash-Consing," ACM Workshop on ML 2006,
[10.1145/1159876.1159880](https://dl.acm.org/doi/10.1145/1159876.1159880). In-workspace precedent verified by
phase three at `hilavitkutin-str/src/interner.rs:81-89`.
*Covers:* the discipline and its cost (one branch, almost always taken).
*Stops at:* nothing relevant. This one is clean.
*Rests on it:* T2-C's soundness. Without it, identity-by-hash has a silent wrong-output failure mode.

**E-B3. The birthday bound at the two candidate widths.** n²/2^(w+1); at n ≈ 12,676: 1.9 percent at 32 bits,
4.4e-12 at 64. Derived twice (Tatarchuk, Wronski) and recomputed by me. No citation needed beyond the formula.
*Rests on it:* T2-C's width choice, and the statement that identity is 12 bytes rather than 4.

**E-B4. A memo key that ignores an open term's free variables is unsound.** Citation: Michie, "Memo functions
and machine learning," *Nature* 1968, for the origin; Hammer, Phang, Might, Foster, "Adapton," PLDI 2014, and
Salsa for the modern statement.
*Covers:* the soundness requirement.
*Stops at:* choosing between Giesen's free-variable fingerprint and Quilez's closed-form key, which are the
same principle stated at two levels of ceremony.
*Rests on it:* Q5-C. **Note:** per Wronski's own snapshot finding, this now governs a small term inside a
small term.

**E-B5. Structural sharing plus positional identity is the green/red split.** Citation: Roslyn, and
[rowan](https://github.com/rust-analyzer/rowan) in Rust. Documented in this round's own `prior_art/07` §8.
*Covers:* the representation.
*Stops at:* the no-alloc adaptation, which is ours.
*Rests on it:* P2. Note that T4-C (stream for content) partly supersedes this: a stream with a depth stack
recovers the tree hash, and the red occurrence table survives only for spans and diagnostics.

### C. The target guarantee

**E-C1. Statically refusing a document against a target's declared feature set before emitting is established
practice in four independent lineages.** Citations: [SPIR-V](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html)
`OpCapability` plus `spirv-val --target-env` (shipped since 2015, including the name-the-construct
diagnostic); `javac --release N` against `ct.sym`
([Morling](https://www.morling.dev/blog/the-anatomy-of-ct-sym-how-javac-ensures-backwards-compatibility/));
Rust [`#[target_feature]` RFC 2045](https://rust-lang.github.io/rfcs/2045-target-feature.html); browserslist
plus [`stylelint-no-unsupported-browser-features`](https://www.npmjs.com/package/stylelint-no-unsupported-browser-features)
over [MDN browser-compat-data](https://github.com/3846masa/stylelint-browser-compat).
*Covers:* the mechanism entirely, including refusal-not-degradation and the named-construct diagnostic.
*Stops at:* document conversion, where no instance was found. DITA constraint modules restrict content models
per document type, not per output capability
([OASIS](https://docs.oasis-open.org/dita/v1.2/os/spec/archSpec/configuration-specialization-and-constraints.html));
Sphinx degrades at write time via `unknown_visit`; Pandoc drops silently.
*Rests on it:* decision 2. **This is the demotion that most changes the round's story.** See N-C1 for what
survives.

**E-C2. Target-decided rather than source-annotated interpretation is polymorphic embedding.** Citation: Hofer,
Ostermann, Rendel, Moors, "Polymorphic Embedding of DSLs," GPCE 2008,
[10.1145/1449913.1449935](https://dl.acm.org/doi/10.1145/1449913.1449935), Most Influential Paper GPCE 2018:
pure embedding "forces the DSL designer to commit to a single semantics", and the fix is that interpretations
are chosen by the instance. Tagless-final (Carette, Kiselyov, Shan) carries the same property and the round
cites it.
*Covers:* one term, interpretation chosen by the instance. This is decision 1's core, eighteen years old.
*Stops at:* the reduce-or-residualise policy and the author-declared predicates checked against it.
*Rests on it:* decision 1. Also note: decision 1 frames source annotation as the only alternative, which is
false. Offline partial evaluation's binding-time analysis computes a division without source brackets, and the
round names Jones, Gomard and Sestoft without using them.

**E-C3. A sound over-approximation with an exact fallback is abstract interpretation.** Citation: Cousot &
Cousot, "Abstract Interpretation," POPL 1977.
*Covers:* the two-level shape and its soundness argument.
*Stops at:* Arntzen's Preserve-mask invariant, which is the part with no obvious prior art. See N-C2.
*Rests on it:* P5.

**E-C4. Many input grammars, one representation, many output targets, spanning logic and markup, has shipped.**
Citations: Racket `#lang` plus Scribble's renderer mixins (`scribble/text-render`, `markdown-render`,
`html-render`, `latex-render`, [Racket docs](https://docs.racket-lang.org/scribble/renderer.html)), with
`content?` as a first-class value; noweb (language independent, TeX/LaTeX/HTML/troff back ends); DSSSL (Scheme
logic over SGML markup into RTF, TeX, HTML). The round's own `prior_art/04` documents the Racket case in
detail.
*Covers:* the niche, on all three axes.
*Stops at:* the check. Scribble's documented failure mode is fidelity diverging silently between backends,
which is exactly the gap decision 2 closes.
*Rests on it:* nothing, and that is the point. **The niche framing the round leads with is not a claim it can
make.** `06_prior_art.md:45` is false and must be struck.

**E-C5. Every shared representation needs a typed escape hatch.** Citations: MLIR's
`unrealized_conversion_cast`, Pandoc's `RawBlock`/`RawInline Format`, Typst's `html.elem`, GENERIC's
language-dependent tree codes, Scribble's `convertible?` protocol. All in `06_prior_art.md` and the deep
passes.
*Covers:* that the hatch is structural rather than a leak, and that Scribble's negotiation form dominates
Pandoc's silent-drop form.
*Stops at:* nothing. This survived four phases untouched and is one of the round's cleanest inherited results.
*Rests on it:* the `Raw` constructors in the document algebra; the shape of decision 3's honesty.

**E-C6. A small fixed core is what shipping many-in/many-out systems converged on.** Racket at ~15 expression
forms, Jsonnet at ~15 after desugaring, Pandoc at 14 block plus 19-21 inline, WebAssembly core at four types.
Citations in `06_prior_art.md` and `07_toolbox.md`.
*Covers:* the order of magnitude.
*Stops at:* membership. The survey explicitly does not say which forms.
*Rests on it:* the nine core forms and the 21-constructor algebra, both of which remain this round's own work
with no external authority, as `09_shape.md:6-7` says plainly.

### D. Representation

**E-D1. Content must be a monoid for iteration over markup to work.** Citation: Typst's `#for` joining
iteration results into one value, and Scribble's list accumulation, both in `06_prior_art.md` and the deep
passes.
*Covers:* the requirement and the failure mode without it (degradation to string concatenation).
*Stops at:* nothing needed here. Note that Carmack's stream-concatenation form satisfies the monoid without
`Seq` being a semantic form, which is why `Seq` collapses.
*Rests on it:* `Iter`, the `Seq` collapse, T4-C.

**E-D2. Versioned flat binary formats with mmap access, and how a version field earns forward compatibility.**
Citations: [SQLite file format](https://www.sqlite.org/fileformat2.html) §1.2 read/write version bytes; DWARF's
versioning discipline; [Cap'n Proto evolution rules](https://capnproto.org/language.html#evolving-your-protocol);
FlatBuffers for the zero-copy no-fix-up access pattern.
*Covers:* the discipline, and specifically the **two-number** shape (read version and write version) that
Fiedler correctly demands.
*Stops at:* the format choice, which is a dependency-surface and no-alloc-consumer argument, not a performance
one. The 45x is bought by skipping the parse and any binary snapshot buys it equally.
*Rests on it:* Q-B-A, adjudication 5, and the correction that one version field is one too few.

**E-D3. Initial encoding in an arena with one generic fold is what production Rust compilers do.** Citation:
rustc HIR, rust-analyzer, oxc, swc, per `prior_art/07`. Variable-arity children without a `Vec` is
`cranelift-entity`'s `EntityList` plus `ListPool`.
*Covers:* the representation choice against final encoding.
*Stops at:* the claim that feature checking is free from the type system, which `07_toolbox.md:178-181`
already corrects: free only for terms written in Rust.
*Rests on it:* the arena, `09_shape.md`'s whole spine.

### E. Data and migration

**E-E1. Three-layer per-cell value resolution under a fixed global strength ordering.** Citations: USD's LIVRPS
strength ordering; the CSS cascade's origin model. Grassia's take/refuse split against USD is correctly
reasoned: take per-field resolution and the declared-once ordering, refuse variants/inherits/specialises and
composition-as-a-graph.
*Covers:* the value-resolution model and why a fixed sequence is what makes it debuggable.
*Stops at:* the refusal of composition-as-graph, which is this design's own restriction and should be claimed
as such.
*Rests on it:* adjudication 3, Q-D-A, the per-cell layer lane in the snapshot.

**E-E2. Build-system input classification into distinct ground-truth classes.** Citations: Bazel's
source/generated/config split; [Nix fixed-output derivations](https://nixos.org/manual/nix/stable/language/advanced-attributes)
for the pinned-oracle class.
*Covers:* the practice of classifying inputs by how their truth is established.
*Stops at:* whether the pinned-oracle-to-fixed-output-derivation correspondence is exact, which Giesen flagged
for phase four and which **nobody in phase four checked**. Still open.
*Rests on it:* adjudication 2 and Building-beyond 2 (the stratified dirty set).

**E-E3. Immutable content addressed by commit is available in this workspace.** Git's object model. The
epoch-pin mechanism is `git show <sha>:<path>`, which cannot move.
*Covers:* the immutability of the cited bytes, which is all the citation check needs.
*Stops at:* Software Heritage's SWHIDs, cited as the adjacent named system, address content and not lines; the
`;lines=` form is a contextual qualifier layered on the identifier. So SWHIDs corroborate the addressing
convention rather than establishing line-granular hashing.
*Rests on it:* Q-C-A, Building-beyond 1, the dissolution of Grassia's one declared blocker.

### F. Contracts and determinism

**E-F1. Reproducible builds enumerate the recurring non-determinism sources and specify the timestamp fix.**
Citations: Lamb & Zacchiroli, "Reproducible Builds: Increasing the Integrity of Software Supply Chains,"
*IEEE Software* 39(2), 2022, [arXiv](https://arxiv.org/abs/2104.06020);
[`SOURCE_DATE_EPOCH` spec](https://reproducible-builds.org/specs/source-date-epoch/).
*Covers:* wall clock, iteration order, directory read order, locale, hash ordering, path leakage; and the
standard fix for an embedded timestamp.
*Stops at:* nothing. This is a solved problem with a specification, which is what makes F's failure notable.
*Rests on it:* every claim in the design that a diff is meaningful, that the seed is stable, that the churn
counter measures anything, and that a fingerprint can be trusted. That is four mechanisms on one unstated and
currently false premise.

**E-F2. A check whose execution is controlled by the party being checked carries no proof about that party.**
Citations: Thompson, "Reflections on Trusting Trust," Turing Award lecture, CACM 1984,
[10.1145/358198.358210](https://dl.acm.org/doi/10.1145/358198.358210); Wheeler, Diverse Double-Compiling,
[dwheeler.com/trusting-trust](https://dwheeler.com/trusting-trust/).
*Covers:* why an independent rebuild by a verifier is the answer to an untrusted local build.
*Stops at:* the specific remedy here, which is one ruleset edit rather than a verification programme.
*Rests on it:* the honest statement of the commit-gate claim, and Q3-B's whole value.

**E-F3. Branch protection on the trunk is live, server-side, and enforced.** Verified by me via
`gh api repos/orgrinrt/arvo/rulesets`: `trunk-protection`, active, conditions `~DEFAULT_BRANCH` +
`refs/heads/dev` + `refs/heads/main`, rules `deletion` + `non_fast_forward` + `pull_request`.
*Covers:* that pushes to trunk require a pull request, and that trunk cannot be deleted or force-pushed.
*Stops at:* checking. There is no `required_status_checks` on this ruleset. It enforces process, not content.
*Rests on it:* leg 1 of the commit-gate claim, which the round underrated and which is the only leg that is
actually a proof of anything.

**E-F4. Pre-1.0 upstream owes no stability, deliberately.** Citation: the workspace's own
`no-legacy-shims-pre-1.0.md`.
*Covers:* that the absence of a deprecation window is a decision rather than an oversight.
*Stops at:* mitigation, which is Q-F-A's exact-rev pinning and is policy.
*Rests on it:* Q-F-A, and the honest labelling of Q8's one-directional stability.

### G. Corpus facts (all reproduced by me today)

**E-G1. 2676 rows across 14 namespace directories in 207 files, plus 10 in `vocab.toml`.** Reproduced: per-
namespace `^\[\[` counts summing to 2676 (616 reference, 506 spike, 444 technique, 344 ruling, 209 bench, 148
task, 87 law, 66 equation, 54 tripwire, 52 facet, 52 data_shape, 46 field, 40 constant, 12 abstraction), 207
`.toml` files. Fourth independent reproduction.
*Rests on it:* every sizing argument. A one-level glob is wrong by 233 rows and 112 references and any loader
written against `registry/*/*.toml` inherits both.

**E-G2. 6628 registry-side references, by full prefix histogram.** Reproduced exactly:
3475 `seed::`, 2286 `crates::`, 650 `reference::`, 93 `spike::`, 66 `bench::`, 31 `tripwire::`, 7 `constant::`,
5 `ruling::`, 3 each `vocab::`/`reg::`/`law::`/`equation::`, 2 `technique::`, 1 `facet::`. Row-to-row is 861,
or 867 counting `vocab::`/`reg::`. Command, per Wyman's provocation 4 and Fiedler's publication of it:
`grep -rhoE '\b[a-z_][a-z0-9_]*::' . | sort | uniq -c | sort -rn`.
*Rests on it:* the index sizing, the cycle graph, the snapshot edge tables, adjudication 2's four-way split.
Third reproduction, and the figure is now closed.

**E-G3. 1082 template-side `{{ }}` occurrences, of which 1007 are multi-segment.** Reproduced across the
workspace: `ikiuni_renderer` 1066, `mockspace` 12, `arvo` 2, `hilavitkutin` 2, all others 0. Filtering for
`::` gives 1007. **The round's 1082 and 1007 are the same population under two definitions**, the 75-occurrence
difference being the single-segment placeholder tail. Wyman declined to verify this; it closes here.
*Rests on it:* the four-population split in adjudication 2, and the corpus total, which is 6628 + 1007 = 7635
multi-segment addresses, or 6628 + 1082 = 7710 brace-and-prefix occurrences.

**E-G4. 148 task rows; 142 `.md.tmpl` files; 72 rendered output documents; 33 crates.** All reproduced.
*Rests on it:* the anchoring workload's shape. **And see C6: none of these is 132.**

## The ledger: NOVEL

Four entries. Each states the search that earns it. An entry with no stated search is not in this section; it
is in UNSUPPORTED.

**N-A1. The four-way ground-truth stratification of the dirty set.** Authored templates (per-file content
hash), authored registry rows (per-row hash, dirtying the transitive readership through the 861-edge row-to-row
graph), derived facts (fingerprint of the deriving scan's input; no TOML diff ever appears), pinned oracles
(the pin; never dirties a document).
*Searches performed:* Wronski searched the incremental-computation literature and found DBSP supplies the
*criterion* for when to invalidate, not the classification of *what kind of input* invalidates; he located the
nearest neighbours in build-system input classification (Bazel, Nix). I add that E-E2's correspondence to Nix
fixed-output derivations was flagged as needing a check and has not been checked.
*Closest adjacent work:* Bazel's source/generated/config split, which classifies by provenance but not by
invalidation semantics; DBSP, which computes derivatives but takes the input classes as given.
*Rests on it:* adjudication 2, Building-beyond 2, all three of the owed red tests. This is the round's own
construction and the strongest thing in phase three.

**N-C1. Pre-emission target-feature checking applied to document conversion.** Narrowed from the round's
phrasing, which claims the mechanism rather than the application.
*Searches performed:* Wyman searched static validation of a document against an output format's supported
constructs, DITA specialisation and constraint modules, target-environment validation in shader toolchains,
compiler target-feature checking, and browser-target linting. The mechanism turned up in four lineages; the
application to document conversion turned up nowhere.
*Closest adjacent work:* browserslist plus `doiuse`, because CSS is a declarative document language, which is
as close as the adjacent field reaches. And DITA constraints, which are grammar conformance upstream of any
target.
*Rests on it:* decision 2's claim to be ahead of the prior art. **The wide phrasing fails on first contact with
any reader who has written a shader**; the narrow one is defensible and interesting.

**N-C2. The Preserve-policy mask invariant.** That mask accumulation and the reduce-versus-preserve decision
must be the same predicate over the same node in the same fold, because a `Select` surviving into emitted code
keeps both arms and an exact mask narrowed to the taken arm ships an unsupported construct with no refusal.
*Searches performed:* Wronski grounded the two-level shape in abstract interpretation and explicitly identified
the Preserve subtlety as the part with no obvious prior art, which Giesen had flagged for exactly this check.
No search located a prior statement.
*Closest adjacent work:* JIT guard fast-path/slow-path discipline, which has the shape without the policy
interaction.
*Rests on it:* P5's soundness, and Arntzen's red test 1. The weakest-evidenced of the four, in that the absence
of prior art here rests on one panellist's judgement rather than an enumerated search.

**N-D1. Deterministic phrasing variation seeded by a content hash of the call term.** A content hash of a call
selecting among pre-authored phrasing variants, to keep re-renders diff-stable under a semantic no-op.
*Searches performed:* two, independent, by different panellists who did not share results. Wronski searched
procedural generation with deterministic seeding, hash-based selection in generative text, and NLG
surface-realization template variation. Wyman searched deterministic natural language generation, paraphrase
selection with stable output under regeneration, deterministic text generation with seeds, template variation
in surface realisation, and hash-seeded selection to avoid diff churn. Both negative.
*Closest adjacent work:* [SteadyText](https://github.com/julep-ai/steadytext) and Stable Diffusion seed
practice, which make a *generator* deterministic rather than selecting among pre-authored variants and derive
no seed from a semantic hash; procedural content generation, which hashes a coordinate or entity id to seed a
PRNG for a generated value; content-defined chunking, which is the closest structural analogue and is a
different domain; Dhall's versioned normal form and semantic hash, which supply the hashing half and which the
round cites correctly.
*Rests on it:* decision 4. **Note the awkwardness the round should face:** this is its only cleanly novel
mechanism and its own synthesis lists it as blocked on whether anyone wants it
(`panel3/04_giesen_synthesis.md:390-391`).

**Candidate, not yet earning an entry.** The three-part staging composition (reduce-or-residualise decided by
target policy, locally invertible by an explicit context rule, with author predicates checked against that
policy rather than inferred). Wyman names it as possibly novel and unlocated. No search is stated for the
composition as a whole, only for its first part, which is E-C2. It sits between NOVEL and UNSUPPORTED until
someone searches it.

## The ledger: UNSUPPORTED

Asserted, repeated, ungrounded. These are the dangerous ones, because repetition across phases has given them
the appearance of settled fact.

**U1. That a heap-free generator has no precedent.** Phases repeated: **all four.**
`06_prior_art.md:256` states "Building the generator side without a heap has no precedent found";
`07_toolbox.md:238-246` restates it; Tatarchuk carries it into phase two's "genuinely blocked"; Giesen carries
it into phase three's handoff as "the largest unbacked claim in the design and the first thing a hostile
reviewer should attack"; Wyman names it in phase four as "still zero precedent and still the largest unbacked
claim" **without performing a search**, in the file whose entire subject is that a novelty claim is worth
nothing unless the search behind it is stated. So after four phases this claim is unbacked in *both*
directions: no precedent has been found and no search for one has been described.
*Rests on it:* the entire no-heap constraint's risk assessment, the fixed-capacity arena plan, the
extractable-crate story, and the round's own identification of where the risk sits. This is the single largest
UNSUPPORTED entry and it is load-bearing on the design's most expensive constraint.

**U2. That the generator is deterministic.** Phases repeated: **implicitly all four, explicitly nowhere.**
Fiedler's finding: the promise is not merely unstated, it is **falsified in shipped code**, verified by me.
`render_design.rs` shells to `date -u`, substitutes `"unknown"` on failure so output depends on `PATH`, and
writes the result into every generated header. Zero `SOURCE_DATE_EPOCH` handling.
*Rests on it:* four separate mechanisms. A diff is meaningful (P3's review surface, Pesce's pinned render).
The variation seed is stable (decision 4, and therefore N-D1, the round's only clean novelty). The churn
counter measures something (adjudication 1). The dirty set can trust a fingerprint (adjudication 2, N-A1).
**All four are downstream of a premise that is currently false.** This is the most consequential entry in the
ledger.

**U3. That the commit gate proves anything about what reaches `dev`.** Phases repeated: **two** (Arntzen in
phase three, adopted by Giesen's synthesis as the "honest claim"). Falsified in part by Fiedler and verified by
me: leg 1 (rulesets) is true and enforces *process*; leg 2 (checker parity) is false, since the hook scopes by
staged path prefix and `lint/mod.rs` skips out-of-scope crates; leg 3 (`--no-verify`) is unverifiable policy;
and leg 4, unnamed by anyone through three phases, is that `core.hooksPath` is `git config --local` and is not
cloned, with no `required_status_checks` on the `dev` ruleset. The honest sentence is Fiedler's: "a
contributor with hooks installed is warned about the crates they touched."
*Rests on it:* Q3-B's entire value proposition, the proof-category split's framing, and every "checked at the
gate" claim in the design. One ruleset edit converts it into a real proof carrier.

**U4. That mockspace can hand borrowed views to no-alloc crates without forcing allocation.** Phases repeated:
**two** (Arntzen names the gap; Giesen's adjudication 5 assembles a snapshot that assumes it). No API shape
exists anywhere in four phases. Fiedler searched for a precedent for the no-heap half and found none.
*Rests on it:* adjudication 5, Q-B-A, the whole boundary-contract design, and the extraction story. Related to
U1 and probably resolved by the same work.

**U5. The human adjudication rate.** Phases repeated: **one, honestly labelled throughout.** 98.2 corpus words
per drained row is a volume. Minutes per adjudicated row is unmeasured and, as Giesen says, cannot be derived.
*Rests on it:* every migration schedule quoted in four phases. Correctly flagged, not a failure of the round.

**U6. That vehje is a fit consumer for this representation.** Phases repeated: **two**, labelled as a bet by
Arntzen and kept as one by Fiedler. Keep the label.
*Rests on it:* Q8, the separately-versioned representation crate argument.

**U7. That a boolean feature bit per family is sufficient.** Phases repeated: **three** (`08_decisions.md:96`
onward, unchallenged until Wyman). Not falsified, but every shipped system in the adjacent space discovered
otherwise: MDN carries partial-implementation flags and version ranges, caniuse carries "supported with known
bugs". The day a target supports a construct badly rather than not at all, the mask can only lie or migrate.
*Rests on it:* decision 2's data model, T1's index sizing indirectly, and the `Bits<N, Hot>` width choice.
Cheap to fix now, per Wyman's provocation 2.

**U8. That the family set is flat.** Phases repeated: **three**, never examined. SPIR-V's capabilities have
dependencies, so declaring one implicitly declares what it rests on, which is why the declaration stays short
as the vocabulary grows. This design's set is flat, so every document must name every family transitively.
*Rests on it:* decision 2's authoring ergonomics at scale. Also cheap now.

**U9. The measured cost table.** Phases repeated: **three** (Tatarchuk produced it, Karis extended it, Wronski
accepted it on the coordinator's confirmation, Giesen's synthesis built on it). Every figure comes from one
naive `HashMap`/`toml_edit` harness by one panellist, reproduced by a second panellist within noise but never
by a third party, and **stated over a document population that does not exist** (see C6). The 0.053 ms, 2.87
ms, 202 ms, 13.4 ms, 0.41 ms, 3.2 ms and 10.5 ms figures are all in this class.
*Rests on it:* T1, T5, Q-B, adjudication 1, adjudication 5, the entire "what now works that phase one and two
said could not" section, and Carmack's deleted incremental machinery. This is the largest volume of design
resting on unreproduced measurement in the round.

---

## Corrections register

Every figure and citation this round got wrong, its correct value, and how many phases repeated it before
correction. Reproduced-by-me figures carry the command.

| ID | Claim as stated | Correct value | Phases repeated | Corrected by | Status |
|---|---|---|---|---|---|
| **C1** | Registry rows: 2443 | **2676** plus 10 vocab, 207 files, 14 namespace dirs | 1 (phase-one synthesis) | Carmack + Quilez (phase 2), settled by Tatarchuk two independent ways | **Closed.** Reproduced by me. Cause: a one-level glob missing `spike/`'s five subdirectories. |
| **C2** | Registry references: 5761, then 6623, then 6628 | **6628** | 2 (Grassia's 5761 in phase 3; Giesen's 6623 in the same phase; 6623 carried into the phase-four brief) | Wyman (histogram), reproduced by Fiedler, reproduced by me | **Closed.** Cause, three times running: enumerating prefixes instead of taking a histogram over all of them. |
| **C3** | egg, PLDI 2021 | **POPL 2021**, Distinguished Paper, `10.1145/3434304` | 2 (Stachowiak phase 2, Giesen's phase-three sources list verbatim) | Wronski, confirmed by Wyman, verified by me | **Closed.** Note: the URL was always correct. The venue label was wrong and nobody clicked through. |
| **C4** | egglog establishes the relational inversion | **.QL / CodeQL does**: de Moor et al. SCAM 2007; Avgustinov et al. ECOOP 2016. egglog is a rewriting result | 2 (Stachowiak, then phase-three's framing) | Wyman, both citations verified by me | **Closed.** Independent of C3; fixing the venue does not fix the aim. |
| **C5** | "Racket declines the many-output half entirely" (`06_prior_art.md:45`) | **False.** Scribble ships four render backends over one core; the round's own `prior_art/04` documents them | 3 (survey, then every phase that leaned on the niche framing) | Wyman | **Closed, and the sentence must be struck.** It is load-bearing under a novelty claim. |
| **C6** | "132 per-crate `DESIGN.md.tmpl` files", the anchoring loop's multiplier | **No population of 132 documents exists.** Measured: 33 per-crate `DESIGN.md.tmpl`; 85 templates under `crates/` of all kinds; 142 `.md.tmpl` in the repo; 72 rendered output docs. 132 = 33 × 4 exactly | **3, uncorrected** (Karis's headline, Tatarchuk's "Settled" list, Giesen's synthesis) | **This file** | **Open.** See below. |
| **C7** | 151 committed files carry a generated timestamp; corrected by the coordinator to 610 | **Neither reproduces.** Measured: 201 git-tracked at HEAD across twelve repos; 581 in the working tree across the nine repo roots; 634 across the whole workspace tree | 1 (Fiedler), plus one unreproducible correction | **This file, partially** | **Open.** Direction certain, magnitude not established. |
| **C8** | Extraction ratio 75.7 words per row | **98.2** (203,414 corpus words / 2071 drained rows, not / 2686 all rows) | 1 (Grassia) | Giesen, phase 3 | **Closed.** Grassia committed in one file the denominator error he had diagnosed in another. |
| **C9** | `provenance` is `visibility = "internal"` | The flag is on **`seed_id`**; provenance citations are dropped by frozen-root filtering | 1 (Grassia) | Giesen, phase 3 | **Closed.** An implementer following the original text would look for a flag that is not there. |
| **C10** | The cargo cycle is structural and blocks the work | **It is not.** Package identity includes source id; path-local and git-sourced mockspace are distinct nodes | 1 (phase one entire, including the synthesis, which built a nine-repository restructuring on it) | The maintainer, one question; tested twice since | **Closed.** The `[patch]` collapse is real and is a hard refusal. |
| **C11** | Row-to-row references: 862 | **861** (or 867 counting `vocab::`/`reg::`) | 1 (Giesen phase 3) | Wyman, reproduced by Fiedler and by me | **Closed.** |
| **C12** | Wyman's line cites `06_prior_art.md:53` and `:65` | **`:52` and `:45`** | 0 (phase four only) | This file | **Closed.** Substance correct, cites wrong. |
| **C13** | Arntzen cites `10_syntax_correction.md:14-16` for statically known namespace sizes | Those lines establish typed bare references and say nothing about sizes. The fact is real and differently sourced (the registry is immutable within a run, so cardinalities are read rather than estimated) | 1 (Arntzen) | Giesen, phase 3 | **Closed.** The dissolution survives on the correct ground and is stronger there. |
| **C14** | "1.7 MB TOML" | **1.28 MB** (`du` versus byte count) | 1 (Carmack) | Tatarchuk, phase 2 | **Closed.** The parse-cost argument was built on the inflated figure and survives anyway. |
| **C15** | Template references: 1007 versus 1082 | **Both correct, same population, two definitions.** 1082 total `{{ }}`; 1007 containing `::`; the 75 difference is the single-segment placeholder tail | 3 (used interchangeably without reconciliation; Wyman explicitly declined to verify) | This file | **Closed.** |
| **C16** | SWHIDs content-address to line granularity | The core SWHID addresses content; `;lines=` is a contextual qualifier on the identifier, not part of the intrinsic hash | 0 (phase four only) | This file | **Closed.** Does not damage the epoch-pin proposal. |
| **C17** | Aaltonen's `NodeId -> Value` memo | **Unsound** under the scope chain; the key must be the closed form | 1 (Aaltonen) | Giesen P4, phase 1; simplified by Quilez, phase 2 | **Closed.** The one place following the panel as written would have shipped a correctness bug. |
| **C18** | Giesen's CallHash over resolved interner ids | Interner ids are encounter-order, and via the const linker-section path are *link*-order; hash slug bytes | 1 (Giesen) | Quilez, phase 2; generalised by Stachowiak; sentence written by Tatarchuk | **Closed.** Decision 4's own failure mode, reintroduced one layer down. |

**Counting the record honestly.** The row count was reported four times before settling (2443, 2676, 2686 with
vocab, and the 2676-versus-2686 ambiguity persisting into the phase-four brief). The reference count was
reported four times before settling (1007, 5761, 6623, 6628). Two figures are still open (C6, C7), and one of
them, C6, sits directly under the round's largest result and was introduced in phase three, carried by phase
two's synthesis, and inspected by nobody in phase four. **That is a fifth count error found in the phase that
exists to find them, which is either encouraging or damning depending on whether a sixth turns up.**

### C6 in full, because it is the one that matters

Karis's headline is "132 documents evaluating one query under 132 bindings", measured at 0.053 ms inverted
against 2.87 ms naive. Tatarchuk's phase-two "Settled" section states the population as "132 per-crate
`DESIGN.md.tmpl` files, which is the anchoring loop's multiplier". Measured in `ikiuni_renderer/mock`:

- `find crates -name 'DESIGN.md.tmpl' | wc -l` returns **33**
- `find crates -name '*.md.tmpl' | wc -l` returns **85** (33 DESIGN, 33 README, 19 DEEPDIVE)
- `find . -name '*.md.tmpl' | wc -l` returns **142**
- rendered `docs/*.md` returns **72**
- `ls crates | wc -l` returns **33**, and 33 × 4 = 132

There is no reading of the corpus under which 132 documents exist. **What this does and does not break.**
Karis's conclusion is untouched: his own scaling table runs 132 to 13,200 documents at +4 percent, so the
document axis is free regardless of where on that axis the real corpus sits. What breaks is every *absolute*
per-document figure: the 2.87 ms naive anchoring and Tatarchuk's 2.3 ms are measured against a population
roughly four times the real one, so today's real anchoring cost is nearer 0.7 ms, and the ratio against the
13.4 ms parse moves further in Carmack's favour, not less. The correction strengthens the round's own
conclusions and invalidates its numbers, which is the same shape as C2 and C8. Publish the population
definition alongside the next measurement.

---

## What the ledger implies for the design's justification

**The justification has changed category and the round has not restated it.**

Through three phases, and in `06_prior_art.md`'s framing most explicitly, the design has been justified by a
gap: no surveyed system does many-in and many-out across logic and markup, no system checks before emitting,
and the combination is therefore new. After phase four, that framing does not survive. E-C4 establishes that
Racket plus Scribble occupies the niche on all three axes and has for twenty years, with noweb and DSSSL as
further instances. E-C1 establishes that pre-emission target checking ships in four independent lineages.
E-C2 establishes that target-decided staging is eighteen years old and has a Most Influential Paper award.
E-A1 and E-A2 establish that the query-side results are database standard material. **The design is now
justified by application and composition, not by invention.**

That is not a weaker justification. It is a more defensible one, and it is also a harder one to state well,
because it requires naming what each borrowed mechanism does not cover. The honest form is roughly: *four
mechanisms that are individually standard in compilers, shader toolchains, databases and build systems are
absent, all four, from every document-conversion system surveyed; this design applies them there, under a
no-heap constraint nobody has attempted, and composes them so that one bottom-up summary answers the
inclusion proof, the memo key, the seed and the join order at once.* Each clause of that sentence is
supportable from this ledger. The current framing is not.

**Where a claim's collapse takes a design decision with it.** Named explicitly, so the dependency can be
traced.

- **U2 collapsing takes decision 4 with it, and N-D1 with that.** If output is not byte-reproducible, a
  variation seed that keeps re-renders diff-stable is solving a problem that a wall-clock timestamp has
  already made unsolvable. The round's only cleanly novel mechanism is downstream of a premise that shipped
  code currently falsifies. It also takes P3's provenance-labelled review (a diff whose hunks are attributed
  by cause is meaningless when every hunk is a timestamp), the churn counter in adjudication 1, and the
  fingerprint half of adjudication 2. **Four mechanisms, one premise, currently false.** This is the highest-
  leverage item in the ledger and its fix is the cheapest: gate the field behind `SOURCE_DATE_EPOCH` or delete
  it, with Fiedler's caveat that deletion may not be one line (see the audit).
- **U3 collapsing takes Q3-B's value proposition with it.** Commit-gate compilation was ranked as
  simultaneously the proof fix and the performance fix. The performance half survives untouched (it is the
  snapshot, E-D2). The proof half does not, while checking is client-side and opt-in. The design should either
  land the ruleset edit and keep the claim, or restate the claim as Fiedler's narrower sentence. It should not
  keep the current wording.
- **E-C1's narrowing takes the design's headline framing with it, but not decision 2.** Decision 2 is
  unaffected as a mechanism; what changes is what may be said about it. The wide claim ("nothing checks before
  emitting") fails against SPIR-V; the narrow claim ("no document converter checks before emitting") holds and
  is worth having.
- **E-C4 takes the niche framing with it and leaves the conjunction.** What survives is the combination of the
  niche with pre-emission checking and with no heap on the generator side. Since the third element of that
  conjunction is U1, the surviving novelty claim currently rests on an unsearched assertion. **Fix U1 by
  searching it, or the design's residual novelty claim has no floor.**
- **U1 collapsing in the other direction, if a precedent is found, costs nothing but the claim.** The
  constraint stands on the workspace's own rules regardless. It is the *risk assessment* that depends on U1:
  "no map" is a very different engineering posture from "one prior attempt, here is what it hit".
- **U7 and U8 take nothing today and take a schema migration later.** Both are free now.
- **U9 collapsing takes T1, T5, Q-B and adjudication 1's sequencing with it.** Not their direction, which is
  robust, but their justification, which is stated in milliseconds. Since C6 shows the population those
  milliseconds were measured over is wrong, this is not hypothetical.

**One thing the ledger implies that nobody has said.** With E-C4 established, the design's own best motivating
example is no longer Pandoc. Scribble already has the value model, the many-in axis, the many-out axis and
twenty years of production use, and its documented failure mode is fidelity diverging silently between
backends. That is exactly the thing decision 2 closes. Motivating the design against Pandoc's silent drop
argues against a system that made a deliberate under-expressiveness choice; motivating it against Scribble's
backend divergence argues against a system that made the same choices this design is making and lacked only
the check. The second argument is stronger and it is available.

---

## Handoff: what most needs the coordinator's attention

The coordinator performs a delta evaluation across all four phases next, then a standards review against the
workspace's own rules. In order of how much rides on each.

**1. C6, before anything else in the delta evaluation.** The round's largest single result is stated over a
document population that does not exist. The conclusion survives and the numbers do not. Every table in
`panel2/04` and `panel3/01` that carries a per-document figure needs its population restated, and the next
measurement needs its population definition published alongside it, exactly as the reference-count command now
is. **Do not let this reach a design document; it is the fifth instance of the one error class this round
cannot stop making.**

**2. U2, because it is cheap, verified, and four mechanisms hang off it.** The fix is one environment variable
or one deletion, and until it lands, decision 4, P3, adjudication 1 and adjudication 2 are all built on a
premise that shipped code falsifies. Fiedler's provocation 2 (a regenerate-twice-and-diff test in CI) is the
part that makes the fix durable, and it is owed at the same time. Note his missed interaction: the determinism
failure also poisons the gate's input, which is a fifth leg on the commit-gate claim.

**3. The three claims phase four did not ground, listed so they are not mistaken for grounded.** U1 (the
no-heap generator, named as novel in four phases and searched in none), the three-part staging composition
(named as possibly novel, unsearched), and E-E2's pinned-oracle-to-fixed-output-derivation correspondence
(flagged by Giesen for phase four, checked by nobody). Two of these are one search each. The first is the
design's largest risk and its residual novelty claim's only floor.

**4. C7, and the general lesson under it.** The file count is now on its third irreproducible value. I give
201 / 581 / 634 under three defensible population definitions and cannot get 610. The direction is certain and
the magnitude is not established, which is fine as long as the design document says so rather than picking
one. **The general lesson, which belongs in the standards review:** every count in this round that was
published without its command has been wrong at least once, and every count published with its command
(E-G2 after Fiedler published the histogram) has stayed right. That is a process finding with a one-line fix.

**5. For the standards review specifically.** Three things in this round's own conduct are worth checking
against the workspace rules rather than against the design.

- `subagent-prompt-neutrality.md` was deliberately suspended for phase two, on the maintainer's instruction,
  and the suspension is recorded in `panel2/00_adversarial_brief.md`. That is the rule being followed
  correctly, including its record-keeping. Phase four's brief is neutral and the files show it.
- `catalogue-edge-cases-as-tests.md` is owed and unpaid. Phase three named three red tests (the Preserve-mask
  invariant, the registry-row dirty set, the crate-directory rename) plus a depth-3 join catalogue test, and
  Fiedler adds a regenerate-twice-and-diff test. Five tests, written down across two phases, written in code
  zero times. The rule says the finding is captured the instant it is found, and it has not been.
- `design-is-the-oracle.md` and `canonical-design-outranks-intermediate-rounds.md` are about to become live
  for this arc. This round has produced four phases of intermediate artefacts that disagree with each other in
  eighteen recorded places (the corrections register). When these files are consolidated, **the register is
  the thing that tells a future reader which of the four phases to trust on any given point**, and it should
  travel with the design rather than staying in `panel4/`. A reader in a year who finds `06_prior_art.md:45`
  and does not find C5 will re-derive the wrong conclusion, exactly as the RCM episode in the rules file
  describes.

**6. What I would tell the maintainer in one sentence.** The design survived phase four intact and its story
did not: it is now an application-and-composition argument rather than an invention argument, it has one clean
novelty that its own synthesis cannot say anyone wants, it has one falsified premise with four mechanisms on
top of it, and its numbers are measured over a population that is off by a factor of four.
