# Standards review: procedural documents

**Date:** 2026-07-19
**Reviews:** `00_seed.md`, `01_baseline.md`
**Read alongside:** the registry at `ikiuni_renderer/mock/registry/`, `ikiuni-renderer-boundary/DESIGN.md.tmpl`, `ikiuni_renderer/.claude/rules/reference-syntax.md`

## Verdict

The ambition is sound and about a tenth of the plan serves it; the rest is a language project that found a documentation tool to justify itself.

## What holds

Query-not-restate is correct and already partly shipped. `{{ law::no_rendezvous }}` resolving and being *checked* is the whole win: a reference that points nowhere is reported rather than rendered as something that looks fine. Inline-carries-expressions, fences-carry-statements is the right split if fences are ever built. Vocabulary as a compile-time type parameter is right. The seed is honest about its own weak points and 01 killed a false claim on inspection, which is the discipline working.

## Findings

**1. The registry already holds the prose the split says it must not.** `law.statement`, `law.forbids`, `tripwire.silent_failure`, `tripwire.why_wrong_looks_right`. These are arguments, not facts. The row/document split has already drifted, in the direction nobody was watching, and iteration would industrialise it: a document looping `tripwire` renders paragraphs that were written as documentation and filed as data. Fix the schema question first. Either those fields are documentation and belong in prose that cites a thin row, or they are the canonical statement and the crate document's job shrinks to what the row cannot know. Deciding that is design work worth more than the interpreter.

**2. The one real consumer wants nothing the plan builds.** `ikiuni-renderer-boundary/DESIGN.md.tmpl` is 134 lines of argument with eight references, every one a single-field point lookup. Not one wants a loop, a binding, a conditional, or scope. The documents that want iteration are index documents, and `{{ <ns> }}` already renders a whole namespace table. Convert three documents by hand before writing a lexer. That is stress-test #2 from the seed, and it is the cheapest thing on the list.

**3. The unserved gap is one combinator, not a language.** The seed's own motivating example is `tripwire::where(!enforced_by)` plus `.count()`. Filter and count over the existing table renderer. `reference-syntax.md` already ships postfix methods (`.dir()`, `.count()`, `.first()`) and functions (`pathof`, `sourcesof`). Add a predicate filter to that grammar and you have the demonstrated need. If fenced blocks with real control flow later earn themselves against converted documents, build them then, on evidence.

**4. A document that renders a filtered table is a homeless document.** `a-homeless-document-is-a-design-problem.md` says the tell is that the justification is a document rather than a readership. Here it is a capability rather than a document. Same shape. If a section is a loop, it had no argument in it, and the honest artefact is the registry view: name it, link it, delete the section. The language does not answer the boundary question, it makes the boundary cheaper to cross unnoticed. State the line first: **a document may query a row to avoid restating it, and may not iterate to produce a section it would not otherwise have written.**

**5. The generated document has a silent failure and the plan has no tripwire for it.** Wrong prose reads as wrong. A `where` whose predicate stops matching renders an empty section that reads as nothing to report; a row edited for one reader changes a paragraph in a document nobody opened. This repo has a whole registry kind for exactly this class, with a `why_wrong_looks_right` field, and it was not applied to the proposal. Any iteration ships with: empty-result is a diagnostic rather than empty output, and a row edit reports which documents its text now appears in.

**6. Scope is disproportionate by an order of magnitude.** Twelve parts, a shared IR with vehje, a columnar value domain over `hilavitkutin-api`, a typechecker, an interpreter. Nine repos depend on mockspace. The no-alloc question (open #4) shapes the interpreter and is unanswered, yet P1 to P3 are called independent and unblocking. That is building a front end for a back end that may not exist. Anti-YAGNI does not cover this: the complete design here is *not* known, which is the condition the rule does not reach.

**7. The vehje coupling lost its support and survived anyway.** The seed's argument for building in vehje was shared const evaluation. 01 checked source and found it false. The interpreter is mockspace's need alone. The architecture did not move. Whatever remains of "one IR, two syntaxes" now rests on aesthetics, and open #1 asks whether it is even sound. Decouple: build mockspace's need in mockspace. If a shared core is real, it will be visible after two independent implementations exist, not before.

## Where the reasoning went wrong

Seed line 18: *"Taken seriously, that ambition needs iteration, conditionals, bindings, and scope."* Every later step is earned from that sentence and the sentence is asserted, not derived. No document was converted, no unserved reference was catalogued, no count was taken of what the existing mechanism already covers. From there the growth is orderly and each step follows: language, therefore front end, therefore shared IR, therefore vehje, therefore a value domain, therefore hilavitkutin. Accretion that looks like architecture because the premise was never rechecked.

01 compounds it. Four questions answered by reading source; the two that could falsify the plan (#2 convert a document, #3 is the sharing real) survive to the end untouched. Answering the checkable ones first is efficient and it left the load-bearing ones as sunk-cost hostages.

The framing error underneath: *a template is a program whose output is text* is true and irrelevant. A crate document is an argument. Being expressible as a program says nothing about whether it should be one.

## Before this proceeds

1. Three crate documents converted by hand, fully, and read back. Publish the diff and the reference count.
2. A catalogue of every reference in `ikiuni_renderer/mock/**.tmpl` that the current mechanism cannot express. If it is filter-and-count, ship that and stop.
3. The stated boundary from finding 4, in a rule, before any evaluator.
4. Findings 1 and 5 resolved: what a row's prose fields are for, and what a generated section does when it is empty.
5. Then, and only if 1 and 2 demand it, a scope proportionate to what they demand. Not the twelve parts.

Items 1 to 4 cost a day and can kill or resize the whole thing. That is the work.
