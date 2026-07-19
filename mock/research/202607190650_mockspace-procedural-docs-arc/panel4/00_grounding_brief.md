# Panel four: stress test, and ground every surviving claim

**Date:** 2026-07-19
**Format:** sequential and cumulative. Each panellist reads this brief, all of phases one through three, and
every prior phase-four file.
**Runs:** after phase three closes, before the dispatching agent's delta evaluation and the maintainer's
standards review.

## Why this phase exists

Three phases have produced a large body of confident assertion. The factual record is poor.

Four panellists produced four different registry row counts before the definitive figure (**2676** top-level
rows across 14 namespaces in 207 files, with `spike/` holding 506 across five subdirectories a naive glob
misses) was established by counting. Three separate "this cannot be done" verdicts dissolved when someone
executed against them rather than reasoning about them, including one that a senior synthesiser accepted and
built a nine-repository restructuring on top of. The dispatching agent made four verification errors, two of
them while correcting an expert, and reported each with confidence.

**Confidence has not tracked correctness in this round.** Citations are the correction. A claim with a paper
behind it can be checked by a reader in a year; a claim that sounded authoritative cannot.

## The two jobs

**Stress-test phase three.** It was briefed to make the design work as intended even where earlier phases
concluded it could not. That stance finds real answers and also manufactures plausible ones. Take its
proposals apart. Where a phase-three mechanism dissolves a constraint, verify the constraint is genuinely
dissolved rather than relocated somewhere nobody looked. Continue the brainstorm where it left something
unfinished, and find new answers where it found none.

**Ground every surviving claim.** This is the more important job. Every load-bearing claim that survived
three phases of scrutiny gets sorted into one of three buckets, explicitly, by name:

**ESTABLISHED.** Prior art exists. Name it: paper, talk, blog post, book chapter, shipped system, with a URL
and enough specificity that a reader can find the exact result. State what the source actually establishes,
which is often narrower than the claim leaning on it. Where a surveyed system is cited, cite the primary
source rather than this round's summary of it.

**NOVEL.** No prior art found after real searching. Say so explicitly, say what search you performed, and
name the closest adjacent work and how it differs. A novelty claim is only worth something if the search
behind it is stated. The design targets a combination the phase-one survey could not find in any single
system, so genuine novelty is expected in places; the point is knowing exactly where.

**UNSUPPORTED.** Asserted during this round, no grounding found, not obviously novel either. These are the
dangerous ones, because they have been repeated across phases and have accumulated the appearance of
settled fact through repetition alone. Name them plainly.

## Claims that specifically need grounding

Not exhaustive. Add what you find.

The e-class analysis framing of the bottom-up summary, and whether egg's result actually covers the use made
of it. Structural hashing as node identity, and Unison's actual scheme versus what this round attributes to
it. The claim that relational algebra is the primary side and the tree serves it, via egglog. DBSP and
differential dataflow as the basis for an incrementality criterion derivable from a schema. The claim that
no system statically checks a document against its target's feature set before emitting, which is a
novelty claim made twice and never searched properly. The monoid requirement on content and whether Typst's
joining semantics is the right citation. The claim that a bitmask inclusion check is equivalent in strength
to type-level set inclusion. The staging model, target-decided rather than source-annotated, against the
multi-stage programming and tagless-final literature. Bounded evaluation by branch quota rather than
totality. The two-tier value domain. Deterministic phrasing variation seeded by a content hash, which
appears to be genuinely unsearched.

## What good grounding looks like

A citation that names the specific result, not the general area. "Partial evaluation is well studied" is
not grounding. "Jones, Gomard and Sestoft establish X in chapter N, which covers the reduce-or-residualise
decision but explicitly not multi-target emission" is grounding, because it also marks the boundary of what
the source supports.

Where a claim is supported only partially, say which part. Where two sources conflict, say so. Where the
round has cited something incorrectly, correct it; at least one attribution in this round is already
suspected loose.

## The constraints, unchanged

The intent: one source of truth, procedurally queried, embedded and formatted per context. Many input
grammars into one representation, many output targets out, logic and markup both. No heap in the extractable
crates, no dynamic dispatch, no allocation in hot paths, and cost that does not grow badly with registry
size, document count or target count. The foundation of something that grows.

## Output

Write to the path named in your dispatch. Cite with URLs. Execute against factual claims rather than
inheriting them; this round's record makes inherited claims unsafe. Where you cannot ground something,
say so plainly rather than reaching for an adjacent citation that does not quite cover it, which is the
specific failure mode this phase exists to prevent.
