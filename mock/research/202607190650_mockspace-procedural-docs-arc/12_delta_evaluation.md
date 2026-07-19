# Delta evaluation: what four phases actually changed, and what to do

**Date:** 2026-07-19
**Author:** the coordinating agent, guided by `11_standards_judgment.md`.
**Reads with:** `panel4/04_hoffman_ledger.md` for the claims ledger and corrections register.

## The process verdict, accepted

The standards judgment is correct and I am not going to soften it. `find . -type f ! -name '*.md'` in this
round returns **zero**. Forty files, 8653 lines of prose, no code, no sketch, no test, on the subject of
building a language. `mock/research/sketches/` exists and holds prior sketches, so the convention was
available and simply not used.

Worse than the omission is the response to discovering it. Twice the round noticed that its load-bearing
corrections came from execution rather than deliberation (`panel2/00_adversarial_brief.md:127-133`,
`panel4/00_grounding_brief.md:15-22`), and twice the response was to convene another panel. That is the
add-an-intermediary-tier antipattern from `a-homeless-document-is-a-design-problem.md`, applied to process.

My own record: seven verification errors, several while correcting a specialist. The C7 case is the tell.
I overrode Fiedler's 151 with 610, a number I produced but did not define, and 610 does not reproduce under
any definition. I treated my own confident restatement as verification.

The rule I am adopting, from the judgment: **no count leaves my keyboard without the command that produced
it in the same sentence.**

## The pattern under every correction

Seven count disputes occurred. Not one was arithmetic. Every one was an **undefined denominator**:

| dispute | the two denominators |
|---|---|
| rows (four times) | one-level glob against recursive; with and without `vocab.toml` |
| references (four times) | template files against registry data; braced against multi-segment |
| documents | one repository against nine |
| timestamped files | present on disk against git-tracked |
| words per row | all rows against drained rows |

The round could not count because it never said what it was counting. That is a discipline, not a skill, and
it is the cheapest fix available.

## What each phase actually delivered

**Phase one (neutral)** produced the design vocabulary and three structural findings, of which **all three
were false**: the cargo cycle, the absent migration, and the reference population. Its durable output is the
question list, not the answers.

**Phase two (adversarial)** produced the reductions that survive: nine forms toward a smaller core, the
bitmask inclusion check, the repair-chain observation, and the frontier pass that found e-graphs and the
relational framing. It also over-corrected, pricing the design against a corpus measurement of the access
pattern the design exists to replace.

**Phase three (constructive)** produced the round's two best mechanisms: the query inversion from product to
group-by (2.871 ms to 0.053 ms, document count ceasing to be a cost axis), and git-pinned epochs dissolving
the frozen-corpus prerequisite. It also reversed phase one's migration verdict with measurement, and
established that 441 of arvo's rows are derivable at zero authoring cost from files already in the repo.

**Phase four (grounding)** produced the most valuable result: four of five novelty claims are not novel, one
citation propagated wrong through two phases, and the determinism premise underpinning three phases of
argument is falsified in shipped code.

The shape of that progression is worth naming. Each phase's durable contribution came from **measuring or
searching**, and each phase's discarded contribution came from **reasoning about artefacts it had not
opened**.

## The conflicts, adjudicated

**Carmack against Quilez on hash-consing: unresolved, and the round thinks it is resolved.** Stachowiak ruled
for a structural hash replacing encounter-order interning. Tatarchuk then observed that structural dedup
needs a hash-to-slot table, which is an interner, so the ruling restores what it claims to drop. The
substantive question (does the constructor do five jobs or one) is still open, and the identity widening to
64 bits is settled independently and verified.

**Carmack's cost model against the target workload: resolved against Carmack.** Tatarchuk measured it. His
conclusion holds for the anchoring case with two orders of margin and fails the generalisation, because he
priced a sum over references where the design creates a product over documents and rows. Karis then
dissolved the product into a group-by, which makes the disagreement moot in the good direction.

**Relational primary against tree primary: open, and now better grounded than either advocate knew.**
Stachowiak proposed it via egglog, which Wyman found mis-aimed. But CodeQL is the real precedent and it is a
much stronger one: a shipped system whose entire design is the relational inversion. The question is live and
the evidence for it improved while its original argument collapsed.

**AccessSet against bitmask: resolved for the bitmask.** `Contains` is `#[marker]` and its own shipped
diagnostic instructs consumers to raise `recursion_limit` to 1024, with ten implementations in existence.
A `doc & !tgt == 0` check needs no unstable feature. The ceiling Quilez introduced (a mask has a width) is
answered by arvo's `Bits<N, Hot>`, which is const-generic over arbitrary width.

## What survives, what is dead, what is open

**Survives, and is buildable.** The small core (nine forms, or Quilez's further reduction to `Lit | Ref |
Apply` plus an operator table). Families as traits with inclusion rather than coverage. Content as a value,
as a monoid, held by arena index rather than refcount. Typed reference fields. A target as policy plus family
set plus total renderer. The query inversion. The snapshot as one versioned boundary. An explicit frame stack
with a branch quota. A borrowed non-allocating scope chain.

**Dead.** Four of five novelty claims. The justification framing in `06_prior_art.md`, including the false
sentence at line 65. The cargo cycle. The absent-migration finding. The `AccessSet` cons-list recommendation.
Determinism as a premise, until the bug is fixed.

**Open, genuinely.** Whether the relational side or the tree is primary. The exact contract of the
conditional-content check, including Arntzen's unwritten invariant about `Preserve` targets. The proof-category
split. Whether the constructor does five jobs or one.

## One thing the round lost track of

Hoffman notes the variation seed is the only surviving novelty and "the one thing the round cannot establish
anyone wants."

It can. The maintainer asked for it directly when this work started, before any panel convened: several
deterministic phrasings of the same content so a fragment used in thirty documents does not read as thirty
copies of one sentence. Four phases of review lost the requirement's own source and then treated the
requirement as unmotivated.

That is a smaller version of the same failure as everything else here: the answer was in a file nobody
reopened.

## What to do, in order

**1. Fix determinism.** `render_design.rs:480` replaces its `date -u` shell-out with a `SOURCE_DATE_EPOCH`
respecting value, or the timestamp leaves the generated header entirely. This is a bug, not a design
decision, it is the cheapest item in the round, and four mechanisms depend on it: decision 4, the
provenance-labelled review, the churn counter, and the fingerprint half of the dirty set.

**2. Write the five red tests.** Named across two phases, written zero times. `catalogue-edge-cases-as-tests.md`
is a standing rule and compliance costs nothing.

**3. Sketch before designing further.** A working reader plus lowering plus one total renderer over the real
registry. The round has 8653 lines of prose and no evidence any of it compiles.

**4. Correct the record where it will mislead.** `06_prior_art.md:65` is false and carried a novelty claim
through four phases. The corrections register travels with the consolidated design rather than staying in
`panel4/`, per `canonical-design-outranks-intermediate-rounds.md`.

**5. Then decide the two open architecture questions**, relational-primary and constructor-scope, against a
sketch rather than against another panel.

## What this round was worth

Stated honestly, because the process verdict is harsh and the output is not worthless.

The design that exists now is materially better than the one in `09_shape.md`: the inclusion check is
simpler and needs no unstable feature, the cost model is measured rather than assumed, the migration is
known to be tractable with 441 rows derivable for free in one repo alone, and the justification is honest
about resting on application rather than invention.

Every one of those improvements came from someone running a command. None came from the panel structure
itself. The correct lesson is not that the review was worthless but that **its instrument was wrong**: four
phases of expert deliberation bought what perhaps a day of sketching would have bought faster and with more
certainty.
