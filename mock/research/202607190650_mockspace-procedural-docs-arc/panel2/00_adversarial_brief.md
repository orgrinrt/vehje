# Panel two: the adversarial phase

**Date:** 2026-07-19
**Format:** sequential and cumulative, same as panel one. Each panellist reads this brief, the panel-one
files, the panel-one synthesis, and every prior panel-two file.

## A deliberate exception to prompt neutrality, on the maintainer's instruction

`subagent-prompt-neutrality.md` governs every sub-agent dispatch in this workspace and forbids prompts that
telegraph the answer the dispatcher wants. **Panel two intentionally breaks that rule.** The maintainer
ordered the exception, and it is recorded here so a future reader does not mistake it for the rule being
forgotten.

The reasoning: neutrality buys independence, and independence is what panel one was for. That value has been
collected. What panel one cannot supply is adversarial pressure against its own shared assumptions, because
those assumptions are precisely what its members had in common. A second neutral panel would mostly
re-derive the first. So panel two is briefed to be partisan, and the partisanship is aimed at a named and
specific target rather than at a conclusion.

Panel two prompts therefore state what to attack. They do not state what to conclude. That distinction is
the only part of the neutrality rule still in force here: no panel-two prompt says which design is right.

## The prior assumed of panel one

Panel one is assumed to have been **overly eager to apply YAGNI, and to have fallen back on tradition and
the classics**. Specifically, expect and attack these moves wherever they appear in panel one's files or in
the artefacts it was reviewing:

**Prior art as authority.** "Pandoc does it this way, therefore." "Racket froze at fifteen forms, therefore
fifteen." A shipped system is evidence that an approach *can* work, never evidence that it is optimal, and
every system in the survey was built under constraints that are not ours: a heap, a garbage collector, a
different decade's hardware, a different problem. The survey found nothing with the properties this design
targets. That is the licence to invent, and treating the survey as a menu wastes it.

**Cost arguments that price the build and ignore the ceiling.** "That framework is 4600 lines, so do not
build it" prices construction and says nothing about what the cheaper thing forecloses. The relevant question
is what each option's ceiling is and what breaking through it later costs, not what the floor costs today.

**Small-first reasoning.** "Start simple and grow." "This is enough for the current use case." "Defer until
a real need appears." The workspace has standing rules against exactly this instinct
(`invert-the-defer-instinct.md`, `prefer-hard-unblocking-work.md`), and the maintainer has already corrected
one round of this work for it. A design that is right only while the thing stays small is the wrong design.

**Traditional decomposition.** Reaching for the shapes a compiler course teaches (a tree, a fold, passes over
it, a symbol table) because they are the shapes one reaches for. Some will be right. None should survive
unexamined merely because they are standard.

**Conservatism disguised as engineering judgement.** Phrasings like "the pragmatic choice", "well understood",
"proven", "the standard approach" are flags. Each may be correct. Each should have to earn it against a
better alternative that was actually considered rather than against nothing.

## The mandate

Find the **optimal** answer. Not the safe one, not the maximal one.

The optimal answer sacrifices nothing on three axes at once:

**Intent.** One source of truth, procedurally queried, embedded and formatted per context, so a fact is
maintained in exactly one place. Many input grammars into one representation, many output targets out of it,
covering both program logic and document markup. Any proposal that quietly shrinks this ambition has failed
regardless of its other merits.

**Performance and efficiency.** No heap in the extractable language crates, no dynamic dispatch, no
allocation in the hot paths, and a generation run whose cost does not grow badly with registry size,
document count, or target count. A novel design that is elegant and slow is not the answer either.

**Scale.** The design is the foundation of something that grows: more grammars, more targets, more families,
larger registries, more documents, more authors, years of accretion. Judge every choice as that foundation.

## The guard against the opposite failure

Attacking YAGNI is not licence to over-engineer, and the maintainer's constraint is explicit that efficiency
is not negotiable. Generality that costs nothing at runtime is free and should be taken. Generality that
costs allocation, indirection, or a layer of ceremony has to pay for itself.

So the test for any proposal here is: does it raise the ceiling **without** raising the floor? Mechanisms
that do (monomorphisation, type-level proofs, const evaluation, arena indices, generated code) are the ones
worth inventing around. Mechanisms that raise the ceiling by adding runtime cost or authoring friction are
the trade this brief is not asking for.

## The questions

**Where did panel one settle for the traditional answer, and what is the better one?** Name the specific
finding or recommendation, name what it defaulted to, and give the alternative concretely enough to build.

**What in the design or in panel one's critique is sized for a small tool?** Not "this could be bigger" but
"this specific choice has a ceiling, here is where it hits, here is what breaking it costs."

**What is the genuinely novel answer nobody has built?** The survey found no system with these properties.
Something has to be invented. Propose it, cost it, and say what it forecloses.

**Where is panel one simply wrong?** On facts, on mechanisms, on what a cited system actually does.

## Output

Write to the path named in your dispatch. Cite `file:line` for claims about the artefacts, panel one's files,
or the source. Concrete beats concerned. A proposal that could be built beats an objection that could be
raised.

## A worked example of the failure mode, from inside panel one

Added 2026-07-19, after panel one closed and before panel two's first dispatch.

Pesce (panel one, finding 1) reported that mockspace cannot depend on the stack crates, because `arvo`,
`arvo-bits` and `arvo-refit` all carry `mockspace` as a build-dependency, so mockspace must build first.
Giesen's synthesis accepted the conclusion, corrected the mechanism, and designed a nine-repository
bootstrap-split to resolve it. The dispatching agent verified the build-dependencies, confirmed the
conclusion twice, and reported it to the maintainer as settled.

The maintainer asked one question: if the dependencies are pinned to git refs rather than the local
worktree, is it cyclic at all?

It is not. Cargo identifies a package by name, version **and source id**. A local path `mockspace` and a
git `mockspace@dev` are distinct nodes. Tested directly by resolving a scratch package named `mockspace`
depending on git-sourced `hilavitkutin-str`: `cargo metadata` exits 0 and the graph contains both
`mockspace 0.1.0 | LOCAL PATH` and `mockspace 0.1.0 | git+ssh://.../mockspace.git?branch=dev`.

So `hilavitkutin-str`, `hilavitkutin-api` and the arvo crates are all reachable from mockspace today, with
no restructuring. The bootstrap-split is an optimisation, not a prerequisite.

Three real costs survive and should be weighed rather than dismissed: mockspace compiles twice; the
mockspace instance that bootstraps arvo lags the local one; and, load-bearing, the `[patch]` path-override
workflow that `workspace.md` documents for cross-repo iteration would collapse the two instances into one
and make the cycle real. The sidestep holds only while nobody uses a workflow the workspace already
sanctions.

**Why this is in the brief.** Four reviewers, three of them senior and one of them explicitly tasked with
verification, treated an untested assumption as a structural constraint and built increasingly sophisticated
work on top of it. Nobody ran the two-minute check. This is the exact move panel two is charged with
attacking, and it occurred inside the panel that was supposed to catch it.

Treat every "cannot" in panel one's files as unverified until you test it. A constraint that has not been
executed against is a hypothesis wearing a conclusion's clothes.
