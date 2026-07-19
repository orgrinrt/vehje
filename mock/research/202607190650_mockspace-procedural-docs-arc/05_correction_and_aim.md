# Correction of record, and what this is actually for

**Date:** 2026-07-19
**Supersedes:** the framing of `04_standards.md`, and the parts of `00_seed.md` and `01_baseline.md` that
rested on it. The technical findings in `02` and `03` stand and are folded in below.

## The correction

The standards review concluded that the ambition was unsupported, and its central evidence was that no
template in the repository uses iteration. That evidence is circular: **no template uses iteration because
iteration does not exist.** Counting the use of a capability in a corpus written before that capability
existed measures the capability's absence, not the need for it.

Three compounding errors, recorded so they are not repeated:

1. **The question was asked in a leading form.** The dispatch asked "is that proportionate, and if not,
   what is the smaller thing that gets most of the value" and so contained its own answer.
   `subagent-prompt-neutrality.md` forbids exactly this, and it was cited elsewhere in the same session.
2. **The workspace's standing rules were violated without being named.**
   `invert-the-defer-instinct.md` states that a scope-feels-large instinct is a training-corpus artefact
   and the signal to start; `prefer-hard-unblocking-work.md` states that the highest-leverage work is
   usually the large foundational piece. Both were broken, and the breaking was dressed as prudence.
3. **A supporting finding was read as a refuting one.** The registry holding prose (`forbids`,
   `why_wrong_looks_right`) was presented as evidence of drift. It is evidence *for* this work: if a row
   carries the argument and a document restates that argument, the restatement is the duplication being
   complained about. The fix is not to strip prose from rows. It is to stop restating rows in documents.

## What this is for

Hand-authored prose across a project of this size is untenable while the design is in flux. The daily cost
is real and present: the same fact is stated in a registry row and again in several documents, and every
change means finding all of them.

**One source of truth, procedurally queried, embedded and formatted per context.** That is the aim, and it
is worth more upfront work than the alternative costs indefinitely.

## The anchoring use case

Stated by the maintainer, and used from here as the thing the design must actually serve rather than as an
illustration:

> for each thing in tasks that mentions crate self, write a table row in format xyz columns

```query
for t in task::where(crates ~ self) {
    "| {t::id} | {t::what} | {t::milestone} |"
}
```

Dropped into any crate document, this yields that crate's pending work. Four requirements follow, each
derived from the example rather than asserted:

| requirement | why it cannot be dropped |
|---|---|
| iteration | per-row output in a chosen shape |
| a binder (`t`) | the row must be nameable in the body |
| document context (`self`) | the document knows which crate it is |
| interpolation | `{t::id}` inside a string literal |

**Where iteration earns itself, precisely:** a filtered projection alone
(`task::where(crates ~ self)::select(id, what, milestone)`) already renders a table without any loop.
Iteration is needed where the per-row output is prose or a chosen shape rather than a table cell. That is a
real boundary, derived rather than assumed.

## `self`, and the registry it implies

`self` is what makes a fragment composable: the same block in any crate document resolves to that crate.
Without it the filter is hand-written per document, which is the duplication again one level up.

That generalises into **a new registry namespace: reusable fragments with declared parameters.** Functions,
expressed in mockspace's existing registry paradigm rather than as a separate mechanism.

A fragment row declares its parameters and its body. A document calls it with arguments. The fragment is
maintained once and every consuming document follows it, which is the same guarantee the registry already
gives facts, extended to the prose that presents them.

This is the decoupling that carries the value: shared prose fragments become registry entries with
templated inputs slotted in, so a phrasing changes in one place.

## Deterministic variation

A fragment used in thirty documents reads as thirty copies of one sentence, because it is. So a fragment
may declare **several phrasings of the same content**, with the choice made deterministically rather than
randomly:

- **stable**: the same document regenerates to the same phrasing, so a diff shows real changes only
- **varied**: different documents get different phrasings, so the corpus does not read as a form letter

**Open: what the seed is.** The document's path was the first suggestion and is fragile, as this session
demonstrated when the document-ordering prefixes shifted (`100_` to `110_`) as soon as the dependency
layers changed. A path-seeded choice would have silently re-rolled every phrasing in the repository.
Candidates worth weighing: the consuming crate's short name, the calling row's own slug, or an explicit
seed field on the call. The requirement is that the seed be stable under exactly the things that should not
change the prose.

## Why the shared IR is the point

The IR is not plumbing shared for tidiness. It is the thing that makes input and output definitions
**independent of each other**.

```
any input grammar  →  one IR  →  any output definition
```

Define a grammar and how it lowers. Define an output and how it folds. Neither knows about the other, so
they mix and match. Write in this DSL and emit Lua, which is vehje's stated aim. Write in Lua, given
someone bothers with the input grammar, and emit C#. The pairs are not enumerated anywhere; they compose.

The shared IR is the only structure that makes that tractable rather than an implementation per pair. In
this immediate use case it looks redundant, because there is one input and the outputs are known. It is not
redundant, because it is what stops the second input or the second output from being a rewrite.

## Output formats are codegen targets

Taken one step further, the markdown, html and plaintext renderings this system needs are **not a match
statement in Rust.** They are output definitions in the same system that emits Lua.

The renderers written by hand earlier in this session (`to_markdown`, `to_terminal`, `to_html`) become
codegen targets, and the `Format` enum dissolves into the same registry of targets that vehje already
carries: `vehje-codegen` has `lookup(name) -> Maybe<&'static dyn CodegenTarget>` and `emit_for` today.

That is the answer to why the IR matters even with a single input language: the *outputs* are already
plural, they already vary by context, and they are already the thing that gets hand-written repeatedly.

## Technical findings that stand

From the two reviews whose subject was engineering rather than justification:

- **Shared passes are the real work.** `Ir<V: Vocabulary>` alone gives one generic type, not one shared
  vocabulary: passes over `Ir<VehjeVocab>` do not run over `Ir<QueryVocab>`. The value depends on a fixed
  expression core that both front ends lower into, with resolve, check and interpret bounded on that core
  and per-language families extending it. This was missing from the plan and is the difference between the
  claim being real and being nominal.
- **The value domain is two-tier.** Fixed scalars and enums as genuine columns; variable text and arrays
  as a `(offset, len)` handle into an aliased byte arena, itself `Copy` so it composes as a column value.
  This is what makes the aliasing mandate and no-alloc iteration stop colliding rather than compete.
- **The scheduler swap is not available and the plan should stop implying it.** `WorkUnit::Read` and
  `Write` are compile-time cons-list types, so a query plan built at runtime can never become a work-unit
  graph. What does transfer is morsel-windowed column access over the fixed record kinds, and the minimal
  runner should be built to that shape now or the later change is a rewrite of the iteration primitive.
