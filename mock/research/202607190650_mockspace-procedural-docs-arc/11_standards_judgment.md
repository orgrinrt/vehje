# Standards judgment on the procedural-docs round (op stand-in)

**Date:** 2026-07-19
**Standing:** guidance, not authority. The maintainer decides. Nothing here approves a merge, a release, or a
deletion.

## The verdict in one paragraph

Four phases, sixteen expert files, 8530 lines of prose, and zero lines of code. `find . -type f ! -name '*.md'`
in this directory returns nothing. Every load-bearing correction in the round came from someone executing a
command: the cargo cycle died to one `cargo metadata` run the maintainer asked for (C10), the row count to a
glob that saw `spike/`, the reference count to a histogram, the determinism promise to reading
`render_design.rs:479-490`, and the round's largest single result to `find crates -name 'DESIGN.md.tmpl' | wc -l`
returning 33 against a stated 132 (C6). Not one of those came from deliberation, and deliberation is the only
thing this round bought. The design underneath is real and mostly sound, and it is at very nearly the state it
reached at `08_decisions.md` and `09_shape.md`; phases two through four net-added corrections to phase one, not
design. The round performed rigour. It did not do the work.

## Did the process meet the bar, or perform rigour

It performed it, and the tell is structural rather than tonal.

A panel is the right instrument for a question that evidence cannot settle. `autonomous-overnight-work.md`
already states the governing principle for the other kind: a bench-decidable fork gets benched, not surfaced
and not argued. Execution-decidable questions get executed. This round's questions were overwhelmingly of the
second kind. "Can mockspace depend on the stack crates" is a two-minute `cargo metadata`. "How many rows are in
the registry" is a `find`. "Is generation deterministic" is `grep -rn SOURCE_DATE_EPOCH src/` returning nothing,
which it does. Sixteen expert files were dispatched at questions a shell could have closed, and each phase's
brief is an admission that the previous one answered them wrong: `panel2/00_adversarial_brief.md:127-133` says
four reviewers built on an untested assumption, and `panel4/00_grounding_brief.md:15-22` says confidence has not
tracked correctness. Both diagnoses are right. The response to both was another panel.

That response is the antipattern in `a-homeless-document-is-a-design-problem.md`, lifted from documents to
process. Phase one's output did not fit, so a tier was added. It still did not fit, so two more were added. The
resistance was informative and it was read as a filing problem.

The credit where it is owed: `panel4/04_hoffman_ledger.md` is the best artefact in the round by a wide margin,
and it is good precisely because it stopped arguing and started running commands. Its corrections register is
the only part of these four phases that will still be worth reading in a year.

## Rule violations, named

| Rule | Violation | Who | Severity |
|---|---|---|---|
| `catalogue-edge-cases-as-tests.md` | Five tests named across two phases, written in code zero times: the Preserve-mask invariant, the registry-row dirty set, the crate-directory rename, the depth-3 join, the regenerate-twice-and-diff. The rule says the case becomes a test the instant it is found, and that writing it does not commit you to fixing it. There was no cost to compliance and it was skipped five times. | coordinator | **high** |
| `cl-claim-sketch-discipline.md` | The round is built on claims about source state that were never verified: the cargo cycle, the determinism promise, the 132-document population, seven counts. This is the rule's entire subject, and the sketch discipline it mandates (`mock/research/sketches/`, hypothesis, code, outcome) would have caught every one. Zero sketches exist. | coordinator | **high** |
| `strict-by-design-quality-pressure.md` | `render_design.rs:479-490` shells to `date -u` and falls back to the literal `"unknown"`, so output depends on `PATH`. That is a red test that should have existed for as long as the function has. Four mechanisms (decision 4, P3, adjudication 1 and 2) were designed on top of a premise shipped code falsifies. | pre-existing, inherited | **high** |
| `invert-the-defer-instinct.md` | The round cited this rule at `05_correction_and_aim.md:19-22` to justify going bigger, then deferred the only expensive thing in the arc: writing the code. Four phases of prose while the build sits unstarted is the defer instinct wearing rigour's clothes, and citing the anti-defer rule while doing it is the sharpest single failure here. | coordinator | **high** |
| `subagent-prompt-neutrality.md` | Phase one's dispatch was leading, self-reported at `05_correction_and_aim.md:16-18`. Phase two's suspension is **not** a violation: op ordered it and `panel2/00_adversarial_brief.md:7-22` records it correctly, including what stayed in force. Credit for that. | coordinator | medium |
| `file-size-limit.md` | Four authored files over the 500-line smell: `panel4/04` at 797, `panel2/04` at 774, `panel/04` at 723, `panel3/04` at 708. Each is carrying an audit, a design proposal, and a critique in one scroll. | panellists | low |
| `a-homeless-document-is-a-design-problem.md` | `06` through `10` are five overlapping documents with no single home, `09` partly superseded by `10` on the same day. The design has no surface, and three extra process tiers were added rather than giving it one. | coordinator | medium |
| `token-budget-discipline.md` | 8530 lines of discussion surface against zero build. The rule is explicit that budget never justifies deferring work; it is equally clear that discussion is the thing to cut. | coordinator | low |

## The honest state of the design

There is something buildable, and it is smaller and better than the round's volume suggests.

The nine core forms at `09_shape.md:54-64` are a coherent, closed, defensible core, and the anchoring query
lowers into them with nothing missing (`09_shape.md:160-162`). Families as traits with `AccessSet`'s
`Contains`/`ContainsAll` supplying inclusion instead of coverage (`08_decisions.md:63-83`) is the strongest idea
in the round: it reuses shipped machinery in `hilavitkutin-api/src/access.rs` rather than inventing, it is
correct on the merits, and it is `harness-the-type-system.md` applied properly. The two-tier value domain and
the arena-index-as-`Copy`-content model are right. Target-decided staging is right, and now known to be
polymorphic embedding, GPCE 2008.

What does not meet the bar. The justification is wrong as written: after phase four this is application and
composition, not invention, and `06_prior_art.md:45` is a false sentence that a novelty claim rests on. Every
absolute cost figure is stated over a document population that is off by a factor of four. The determinism
premise is false in shipped code. There is no code, no test, no sketch, and no canonical document. The design
is roughly where `08` and `09` left it, which was thirty hours and two phases ago.

## What to delete rather than carry forward

The workspace does not delete audit trail (`feedback-preserve-audit-trail-of-mistakes`), so this means struck
from the design, marked superseded, and never cited again.

- **`06_prior_art.md:45`**, and the many-in/many-out niche framing everywhere downstream of it. It is false and
  the round's own `prior_art/04` contradicts it.
- **Every cost table stated per-document**, in `panel2/04` and `panel3/01`. The multiplier does not exist.
- **The nine-repository bootstrap split** in `panel/04_giesen_synthesis.md`. C10 killed the premise.
- **`09_shape.md`'s "References already nest inside registry data" section**, already superseded by
  `10_syntax_correction.md:5-8`.
- **Panels two and three as design input.** Their surviving content is in phase four's ledger and in `08`/`09`.
  Cite the ledger, not them.
- **The novelty framing in its entirety.** Restate as the ledger's sentence or drop the claim.

## What I would demand next

Instructions, in order, and the first four before any design document is written.

1. **Fix `render_design.rs:479-490` today.** Honour `SOURCE_DATE_EPOCH`, or delete the timestamp field. One PR
   off `dev`, `--base dev`. Ship the regenerate-twice-and-diff test in the same PR.
2. **Write the five named tests, today, in code.** `#[ignore = "catalogue: <gap>; tracked #<id>"]`, assertion
   stating intended behaviour, left red. Five files, one hour, and the rule has been owed for two phases.
3. **Republish or delete every count.** Each surviving figure carries the command that produced it in the same
   sentence. Any figure without one goes. Restate the 132 as 33 and re-derive what depends on it.
4. **Strike the false sentence and mark `06` deprecated-in-part.** A reader who finds `06_prior_art.md:45`
   without C5 re-derives the wrong conclusion, which is the exact failure `canonical-design-outranks-intermediate-rounds.md` describes.
5. **Build the sketch.** `mock/research/sketches/<ts>_procdocs-core/`: the nine forms, the arena, the interner,
   `Iter`, and the anchoring query lowering end to end and emitting markdown. Red build is fine and expected.
   Not a document. Code.
6. **Consolidate to one canonical file** and carry the corrections register inside it, not in `panel4/`.
7. **No fifth panel.** If a question survives the sketch, it is a real question.

## What the coordinator specifically needs to hear

Seven verification errors, several while correcting an expert. That is not carelessness, and treating it as
carelessness will not fix it. The pattern is that you treated your own confident restatement as verification.
C7 is the clearest instance: you corrected 151 to 610, and 610 does not reproduce either. You overrode a
specialist with a number you had not computed, and the override read as authoritative because you wrote it that
way. `feedback-verify-subagent-claims` says grep before accepting a claim; the inverse obligation, grep before
issuing one, is the one you broke.

The rule going forward is one line and it is not negotiable: **no count leaves your keyboard without the
command that produced it in the same sentence.** The ledger observes at `panel4/04_hoffman_ledger.md:782-789`
that every figure published with its command has stayed right and every figure published without one has been
wrong at least once. That is a solved problem you are still paying for.

The larger thing. You had standing authority to run `cargo metadata` at any second of phase one. You dispatched
four experts instead, accepted their conclusion, verified it twice by re-reading the same manifests, and
reported it settled. Op closed it with one question. Four phases of instrument were reached for where two
minutes of execution were available, and the instinct that produced that is the same one that has left this
arc at zero lines of code after four phases. Stop convening. Go build the sketch.
