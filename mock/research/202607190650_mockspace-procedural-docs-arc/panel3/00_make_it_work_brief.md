# Panel three: make it work as intended

**Date:** 2026-07-19
**Format:** sequential and cumulative. Each panellist reads this brief, all of panel one, all of panel two,
and every prior panel-three file.
**Runs:** after panel two closes, before the dispatching agent's delta evaluation.

## The stance

Panel one was neutral. Panel two was adversarial against panel one's conservatism. **Panel three is
constructive, and its premise is that the thing works.**

Where panels one and two concluded something cannot be done, that conclusion is your **starting point, not
your verdict**. Your job is to find the way. The maintainer's instruction, verbatim in substance: find ways
to make this all work as intended, even if the experts before resolve that it cannot.

This is not optimism as a mood. It is a different search. A reviewer asked "does this work" stops at the
first blocker and reports it. You are asked "how does this work", which means every blocker is a subproblem
with an unexplored solution space behind it, and the reviewer who stopped there simply did not look.

## Two kinds of answer, in preference order

**Dissolution beats satisfaction.** The best answer to a constraint is showing it does not need to exist.
Ask what the constraint is actually protecting, whether that thing is still needed, and whether it can be
obtained another way entirely. A constraint that everyone has been routing around is often load-bearing on
nothing.

**Clever satisfaction beats reduced ambition.** If the constraint is real, satisfy it in a way that costs
nothing that matters. Type-level proof, const evaluation, generated code, a different decomposition, an
inverted dependency, a mechanism from an unrelated field.

**Reduced ambition is not an answer.** A proposal that quietly drops part of the intent has failed no matter
how clean it looks. Say plainly that you could not find the way, and say exactly what you tried, rather than
shrinking the target to fit a solution.

## What must survive intact

**The intent.** One source of truth, procedurally queried, embedded and formatted per context, so a fact is
maintained in exactly one place. Many input grammars into one representation; many output targets out of it;
program logic and document markup both.

**Performance and efficiency.** No heap in the extractable crates, no dynamic dispatch, no allocation in hot
paths, and a generation run whose cost does not grow badly with registry size, document count, or target
count. A solution that raises the floor to raise the ceiling is the trade nobody asked for.

**Scale.** This is the foundation of something that grows. A fix that works while the thing is small is not
a fix.

## Named targets

These are the places panels one and two reported walls. Attack whichever your lens fits, and treat the
reported wall as unverified.

**The cycle discipline.** Panel one called the mockspace-to-stack dependency structural. It was not, and the
correction is recorded in `panel2/00_adversarial_brief.md`. What survives is narrower: a `[patch]` override
of mockspace itself would collapse two package instances into one. The maintainer's read is that clever
answers exist here, possibly ones that dissolve the need entirely. Ask what the build-dependency bootstrap
is actually buying, whether it must be a build-dependency to buy it, and whether the whole edge can go away.

**The conditional-content check.** A document with target-guarded content is refused by a target that would
never evaluate the guarded branch, because the family set is computed unconditionally at parse time. Giesen
proposed a monotone two-level check. Is there a shape where the question does not arise?

**The parsed-document proof gap.** Family-set proofs are compile-time for terms written in Rust and runtime
for templates read as data. Can templates be compiled ahead of time into a checked form, or is there a third
answer that makes the distinction irrelevant?

**The migration economics.** 1007 of 1007 references live in one repository, which is also the only one with
a registry, and it was authored that way from the start rather than migrated. Roughly 280,000 words of
hand-authored prose exist elsewhere with no migration path proposed. Is fact extraction genuinely the
expensive half, or is that a failure of imagination about what can be mechanised?

**The two-caches correctness gap.** A variation-seed hash and a fold-level memo over the same registry state
invalidate independently. Giesen split them into a `CallHash` and a `ResultKey`. Is two the right number?

**Anything panel two declares impossible.** Panel two is briefed to attack conservatism, which makes its own
"impossible" verdicts the most interesting ones to test.

## The guard

Do not manufacture a solution. A mechanism that technically satisfies a constraint while making the system
worse is not what is being asked for, and neither is a proposal whose cost is hidden in a place nobody
looked. State the cost of every proposal, including the ones you like.

If you genuinely cannot find a way, say so with the search you performed. An honest "I could not, here is the
ground I covered" is worth more than a fix that does not survive contact.

## Output

Write to the path named in your dispatch. Cite `file:line`. Execute against claims rather than inheriting
them; panel one produced at least three "cannot" verdicts that did not survive being run. Concrete beats
concerned, and a mechanism beats a direction.
