# Soul audit: do the two canonical candidates hold against the record

**Author lens:** fresh independent auditor; types, staging, compilers; no continuation of prior panel framing.
**Read in full:** all ten files in this directory; the latter topics of `202607240130` (`202607201315`, `1316`,
`1513`, `202607202001`, `2055`, `2205`, `2330`, `202607210120`, `202607240015`, `202607240100`); the ir-to-ir
topic `202607241330`; `benches/results/RUN_SUMMARY.md` and the `carrier_native_ceiling` cell (FINDINGS.md plus
raw CSVs plus the cell source `carrier/src/bench/native_ceiling.rs`); the two bench audit panels
(`202607211539.../summary.md`, `202607230922.../summary.md`); the novelty audit
(`202607201854_certgen_panel_continuation/04`); and the shipping sources where a claim was checkable
(`vehje-typecheck/src/lib.rs`, `vehje-runtime-gen/src/lib.rs`, `vehje-runtime-abi/src/wire/`,
`vehje-ir/src/hash.rs`, `vehje-lower/src/lib.rs`).

## One-line verdict

The two canonical candidates hold: the identity, the convictions, op's calls, the prior-art commitment tiers,
and the kill catalogue are faithful to the latter topics and to the hard data, and the pair is fit to base a
canonical identity document on, after four corrections (one factual misquote of the consolidation, one
over-pinned bench number, one unify-by-analogy slip in the one-sentence soul's framing, and a handful of stale
or over-confident evidential citations in the prior-art section) plus two additions (the hygiene obligation and
the durable record for op's 2026-07-24 live calls).

## Method

I verified citations against source rather than trusting the forks' cross-audit. Every load-bearing line
citation I checked in the positive enumeration and the inverse landed within a line or two of the quoted text:
the identity proposition (`202607202330:34-35`), the census and spectrum (`:51-65`), the two-artifact addendum
and the LMS-hard rejection (`:160-204`), the handler discipline and CR1 (`202607210120:43-49`, `:92-118`), the
kills (`202607202055:53-104`), the maximal-shape ethos (`202607202205:23-51`, `:140-152`), the three codegens
and the stale native lines (`202607201513:44-85`), the lease and transfer decisions (`202607201316:26-74`,
`202607201315:38-60`), the founding metacompiler sentence (`202607202001:161-164`) and the six-jobs depth-cap
pun it later retired (`:386-388` against `202607202055:95-99`), and the bench topics (`202607240015` throughout,
`202607240100:94-124`, `:266-289`, `:414-431`). The four corrections the negative fork lodged against the
positive fork are real and were applied; I re-verified the depth-cap retirement and the PE-as-extraction
terminator kill directly. The source-state claims also check: `binding: Knowledge::empty()` at
`vehje-typecheck/src/lib.rs:311-312`, zero `vehje_signature` uses in `vehje-typecheck/src/`, `_signature`
ignored at `vehje-runtime-gen/src/lib.rs:108`, `Block`/`BlockTable` defined and never constructed in
`vehje-runtime-abi/src/wire/residual.rs`, and the FIXME'd no-op `MacroExpand`/`Anf` plus the `Saturate`
fallthrough in `vehje-lower/src/lib.rs`. The panels did their reading; the record supports them.

## What holds and is well captured

- **The identity and its history.** Framework-not-a-language, census served equally with the
  authoring/templating majority first-class, the two-artifact model with Rust out of the runtimes, data-not-source,
  the boots-not-steps double rejection (drift and over-correction both). All verbatim-faithful to
  `202607202330` and `202607240100:35-58`. The inverse's B3 (identity errors both directions, the dual-locus
  dissolution) is exactly what the recenter addendum says.
- **The convictions list (positive §2).** Each entry traces to a settled topic decision, not to enthusiasm: the
  graded spine unified by construction, inclusion-not-coverage, the handler discipline, no-GC/no-null
  (`202607201316:40-74`), no-heap-both-sides, the host-lent budget as one shape in three uses (CR1 states the
  unification itself, so this one is by construction, not analogy), strict-by-design, one-truth-many-projections.
  The corrected §2.13 (three bounds, the aesthetic kept, the pun retired) is now right.
- **The commitment-tier device (married / half-married / heeded, plus the inverse's killed and
  negative-precedent tiers).** This is the pair's best contribution: it encodes "a bench trumps a prior-art
  cite" as a per-citation state, which is exactly the discipline the round actually practised. The placements I
  checked (copy-and-patch demoted, eqsat past the unseated line, tail-threading scoped not blanket, LMS split
  verdict, oatlog heeded-not-aligned) match the record, including the RUN_SUMMARY findings 6, 7, 8 and the
  `202607230922` panel corrections (cse+eqsat bit-identical node counts; the 142x as eqsat-alone pessimization;
  the inverted native cell tags; OOPSLA not PLDI).
- **The kill catalogue (inverse Part B).** Every B1 kill matches `202607202055` verbatim; B3 matches the
  recenter; B5's three topic-drifts-from-bench are real: the consolidation does say "plain switch dispatch...
  beats tail-threading" flatly in the CFG paragraph (`202607240100:118-119`) while its own round's bench topic
  already carried the terminator-threading reservation (`202607240015:286-288`), does crown the e-graph "the one
  genuinely original research piece" (`:100-101`) against RUN_SUMMARY finding 6's park-behind-a-trigger, and does
  stay copy-and-patch-centric (`:112-113`, `:318`) against finding 8's direct-isel reorder. Note in passing that
  the consolidation is internally sloppy here too: it crowns the e-graph "the one genuinely original research
  piece" at `:101` and the assurance-indexed logical relation "the highest-value original work" at `:183-185`,
  two superlatives for two different artefacts, which the soul pair resolves the right way (the relation is the
  prize; the e-graph is parked).
- **The two-fork structure itself.** A positive enumeration and a negative inverse that audit each other, with
  the audits recorded and conceded findings applied inline, is the right shape for an identity document, and the
  concessions were genuine (I checked two of the four against primary source).

## What does not hold, over-claims, or is inconsistent

**F1. The positive fork misquotes the consolidation on the native ceiling.**
`canonical_candidate_the-soul-and-intent-of-vehje.md`, §5.i: "(The consolidation's stale ~2.0x, and Fallin's
inherited 2.0x, are doubly-superseded...)". The consolidation never carried 2.0x; it locked "the native
1.0x-to-1.5x ceiling" (`202607240100:419`), which the retake fork itself VINDICATED. The ~2.0x is Fallin's,
from `202607220300`. The sentence garbles who held what. Low impact on the conclusion, but a canonical
identity document must not misquote the round's own closing statement. Fix the parenthetical.

**F2. "~1.5x over the best interpreter is the honest ceiling" is pinned harder than the committed evidence
supports, and the "doubly-stale" dismissal of the 2.0x is unfair.** I went to the cell. The committed
`benches/results/carrier_native_ceiling/FINDINGS.md` (the post-C1-fix, FFI-opaque measurement) reports the
switch interpreter at 2.01x-2.05x of native, stable across N=64..16384, and states in its own words: "the
honest ratio is ~2.0x" and "~2x is the floor, not the typical case" because the madd stream is maximally
predictable (the interpreter's best case; on unpredictable dispatch the ratio grows). RUN_SUMMARY finding 10's
~1.49x comes from the PMU cell (`carrier/src/bench/native_ceiling.rs`), whose interp variant is the switch
interpreter over a predecoded program, sizes stopping at 1024 by the stream regime's cost design, on the same
maximally predictable stream. So the record holds two live, baseline-and-workload-indexed numbers (~1.5x and
~2.0x on near-identical madd cells, spread unexplained anywhere in the record), plus the cell's own floor-not-
typical caveat, plus RUN_SUMMARY's own framing rule that small-N flatters interpreters. Fallin's canon-recheck
already named this an owed reconciliation (his finding 4 and open item 5); the retake fork closed it by
argument (baseline dependence), not by the owed check, and the positive fork then flattened it to a single
pinned scalar. The direction is safe under every number on record (native is a modest scalar multiplier;
the 10x class needs vectorisation; this survives 1.5x, 2.0x, and even the Giesen 10.7x toy). The canonical
statement should be the range with its index: roughly 1.5x to 2x for scalar code depending on interpreter form
and dispatch predictability, growing on branchy real programs, with the 10x class reserved for vectorisation.
Do not canonise "~1.5x is the honest ceiling" as a bare scalar, and drop "doubly-superseded": the 2.0x is the
switch-baseline number the carrier cell itself reproduces, not a stale intermediate.

**F3. §8's "the four commitments are its shadows" is unify-by-analogy, the exact move §9.A celebrates the
round for refusing.** The positive fork's own §9.A quotes the novelty audit: "a filing system gives you one
vocabulary; it does not give you one theorem" (`04:43-46`), and records that the graded spine was accepted only
when unified by construction. §8 then asserts that "one signature projected", "the proof compiled away", "two
artifacts", and "native never the driver" are four projections of the one handler-discipline idea. Two of the
four are not derivable from it: the census identity (templating first-class, ikiuni a consumer not the reason)
is op's identity call grounded in who the consumers are (`202607202330:51-65`), and native-never-the-driver is
grounded in that call plus the bench evidence, not in binding-time-directed discharge. A hypothetical vehje
with the identical handler discipline but a game-runtime-first identity is coherent, which proves the
commitment is independent. This matters operationally because the document nominates §8 as "the fastest test"
for drift: a proposal recentering the project on one hard consumer could pass the §8 test while violating the
soul. Fix: state the drift test as §1 plus §8 jointly (identity and mechanism), and reword the shadows claim
to "aligned with" or "carried by", not "projections of". The one-sentence soul itself is fine as a mnemonic.

**F4. Three stale or over-confident evidential citations in the positive fork's §6.**
(a) "the register VM ... bench-confirmed, ~1.9x over a stack VM" cites the superseded composite: RUN_SUMMARY
finding 4 corrected it to 1.41x-3.06x per profile after the panel found the 1.91x was a three-axis composite
burdening the register side (`202607230922.../summary.md`, magnitude corrections). The fork's own §5 preamble
("the later, more rigorous re-measure governs") mandates the corrected range.
(b) "NaN-boxing ... married on the evidence": RUN_SUMMARY finding 5 marks the nan-boxing result "one-shape
choice, low-to-medium confidence". Married-on-the-evidence overstates a low-to-medium-confidence cell; say
"bench-leaning" or carry the confidence tag.
(c) The tnum entry ("the eBPF-verifier-shaped bounded abstract interpretation over tristate numbers ... for
the untrusted-load check") omits the scoping the round's own kill mandates: tnum-for-structure was killed and
tnum retained for the numeric residual only (`202607202055:71-78`; `202607240100:122-124` makes the load
verifier three mechanisms with the typed decode primary). As written the half-married entry re-widens a
narrowed cite.

**F5. Op's 2026-07-24 live calls (§4.9) have no durable record, and one standing topic contradicts them.**
The single-arena-recursive default, the double-buffer bench fork, and the precedence rule exist only in this
panel directory's prose. The `202607241330` topic still states the two-arena fresh-emit shape and the unbounded
"until a run produces no change" fixpoint (`202607241330:17-38`), both superseded by the live correction and by
the round's own locked bound (`202607240100:96`). Per the supersede-forward discipline the pair itself
canonises (§7), the fix is a short superseding topic recording the live calls, before or with the canonical
identity document. Canon resting on chat memory while a locked topic states the opposite is exactly the drift
seed this whole exercise exists to prevent.

**F6. The terminator convergence is right but under-specified, and one distinction must be stated or the next
reader re-litigates the kill.** Both forks converge on "binding-time well-foundedness, not a cap"
(`202607202055:60-70`). Three sharpenings the canonical statement needs. First, binding-time grade
well-foundedness terminates the stage dimension; it does not by itself terminate within-stage recursion, since
a family macro can emit another same-stage macro call. The within-stage measure is the condition the idealistic
synthesis names (a macro may not re-introduce an operation of its own family at the same or later binding
time), and it needs a checkable home, which is Fallin's `binding_time_ceiling` on `vehje-signature::Operation`
plus a per-operation productivity rule, not a convention. Second, distinguish what the kill killed:
cap-as-semantics (which residual you get depends on the cap; that is the non-determinism `202607202055:64`
names) from budget-as-diagnosed-failure (a deterministic, named error at arena exhaustion). The second is not
merely permitted but mandated by the round's own evidence ("the bounded streaming window as a hard
termination-and-memory safety mechanism", `202607240015:126`), and a well-founded unfold can still exceed a
caller-lent arena, so the budget diagnostic is needed regardless of the termination proof. Third, the
diagnostic must be distinguishable from passthrough at the `FamilyExpand` seam (Rompf finding 6's three-way
result), or exhaustion reads as a truncated program. State all three and the fuel-cap dispute dissolves.

**F7. Macro hygiene is absent from both candidates, and it is soul-shaped.** The panel's unanimous largest
open soundness item (Rompf finding 4, uncontested by every subsequent reader) does not appear in either
enumeration. The soul already contains "illegal states unrepresentable" and "strict by design"; hygiene is that
conviction applied to generative discharge (fresh binders by construction, capture structurally impossible
because resolve runs after expansion). One sentence in §2 alongside the handler discipline, plus the red
capturing-macro test per the catalogue discipline. Leaving it out of the identity document risks a consumer-
facing macro system shipping with the one hole every macro system in history shipped with first.

**F8 (minor, consolidation-side, carried for the fix list).** The consolidation names the manifest and the
fixpoint dedup as consumers of the one structural hash (`202607240100:267-268`). Shipped source says the
opposite, correctly: `vehje-ir/src/hash.rs:1-14` states the word-fold is interner-local and that cross-artifact
identity is the byte-image hash's job, and `vehje-runtime-gen` hashes the byte image. Rompf's addendum finding
0 is verified; the consolidation prose is the unsound-if-built-as-written statement and owes the correction
(within-stage identity structural, cross-stage identity byte-image). Soul-adjacent because "one truth, many
projections" is only sound with the identity boundary stated.

## From the other panellists: carry and challenge

- **Carry (Rompf):** the binding-time pinning of the line is the correct answer to the panel's (B) question
  and deserves canonical wording: AOT at LanguageAuthor and Bundler, load-time compilation (not profile-guided
  JIT) at HostLoader, interpretation a Runtime execution form; const-inlining mandates a specialization stage
  at the binding time the constant becomes known, never a native step; fully-interpreted is coherent per-stage
  and incoherent across the pipeline. That an independent staging read reconstructed the canon's own axis from
  code alone is the strongest corroboration the axis is right. Also carry finding B (the IR-to-IR rebuild is a
  catamorphism; `fold_core`'s visitor shape cannot carry it) and the `Interp`-as-first-staging-site suggestion.
- **Carry (Fallin):** the CFG compounding-drift chain (sketch proves execution, topic cites it as the whole
  mechanism, CL verifies surface, two notes re-certify past the gap) is the best process finding in the round
  and generalises: an audit that enumerates FIXMEs cannot see an unmarked gap; the second-direction audit
  (walk every "settled" bullet against source) is owed. Also the ANF-to-CFG dependency chain, and the
  `RelationSet::insert` three-way result mirroring `FamilyExpand`'s.
- **Challenge (retake fork):** the bench-supremacy re-adjudication is largely correct and its three
  drift-from-bench corrections stand, but "Fallin cited a doubly-stale intermediate" is wrong per F2 above; the
  numbers are baseline-indexed, not a supersession chain, and the owed 220300 reconciliation was closed by
  argument rather than by the check its own meta-review names as residual.
- **Carry (changelist-drift fork):** the process lesson that both live wounds are one CL-verification weakness
  (surface existence standing in for mandate satisfaction) belongs in whatever CL-discipline doc the claim
  verifier lands in; it is the mechanical fix that prevents the next cascade.
- **Note (idealistic synthesis):** the one-reducer thesis is attractive and correctly kept out of the soul
  documents (it is a forward proposal, not settled intent). One coupling it asserts too quickly: that op's
  single-arena correction "dissolves" the `NodeRef` arena-branding hole. True only if single-arena wins the
  arena-strategy bench fork; if double-buffer benches better (a live candidate per op's own call), two same-
  shaped arenas coexist and Fallin's finding A is load-bearing again. The branding decision and the arena-
  strategy bench are one fork, not two; nobody in the directory names that coupling.

## Open questions handed forward

1. The native-ceiling reconciliation, done as a measurement, not an argument: one cell, both baselines (switch
   and predecoded-best), predictable and unpredictable op streams, sizes past the predictor capacity where the
   regime permits, and the canonical range recorded with its index (F2). Until then, canon carries the range.
2. The superseding topic for op's 2026-07-24 live calls (single-arena recursive default, double-buffer as a
   bench fork, benches-over-topics-over-changelists precedence), closing the contradiction with `202607241330`
   (F5).
3. The terminator statement in three layers (stage well-foundedness; the per-operation within-stage condition
   as signature data; the budget diagnostic distinct from passthrough), so the fuel-cap debate never reopens
   (F6).
4. Whether the drift test canonises as §1 plus §8 jointly, and the reworded shadows framing (F3).
5. The hygiene sentence in §2 and the red capturing-macro test (F7).
6. The consolidation corrections owed independently of the soul pair: the hash-identity boundary (F8), the
   terminator-threading reservation, the e-graph re-framing, and the direct-isel reorder in the "settled" list,
   filed as the doc round the retake fork already specifies.
7. The arena-strategy bench fork carries the `NodeRef` branding decision with it; whichever shape wins, decide
   branding in the same act.
