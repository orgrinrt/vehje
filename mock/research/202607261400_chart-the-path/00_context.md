# Context brief: what the complete vehje is, and the ordered path to it

## The governing canon, named exactly

`mock/research/canon/the-soul-of-vehje-positive-catalogue.md` and
`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md`.

**Two files. Eight sections each. That is the entire canon.** It is ratified with
the lead designer in the loop, marks its human calls inline ("per op", "op's
standing calls"), and governs. Required reading in full before you read any code.

Also ratified, and governing:

- `mock/research/202607260100_op-standing-design-calls.md`, ten standing calls in
  the designer's own words, including the 2026-07-26 artifact-split calls.
- `mock/design_rounds/202607241615/op-ratification-answers.md`, the ratification
  record.

## A trap that has already corrupted one expert panel

`mock/design_rounds/202607241615/202607241545_topic.the-vehje-canon.md` is the
round that **produced** the canon. It is **not the canon**. It has twelve
sections; the catalogues have eight.

Its sections 1 to 10 carry the same substance as the catalogues. **Its section 11
does not.** Section 11 is titled "The path forward: what this canon commits the
coming rounds to build", it is a work agenda, and it has no counterpart in either
catalogue (grep `fold_core` or `check-to-signature` in `mock/research/canon/`:
zero hits in both files).

That agenda commits to wiring the binding-time axis into the Rust check pass,
giving `fold_core` a node-creating catamorphism shape, wiring the Rust check to
the signature, and CFG lowering downstream of `Anf`. Every item builds out a Rust
per-script pipeline. The catalogues forbid exactly that.

A previous panel member cited "canon §11" in good faith, because the dispatching
agent's own notes labelled that round as canon. **Any citation to "canon §9",
"§10", "§11", or "§12" is false by construction.** If you find yourself reaching
for that document, you are reading an intermediate artifact, and
`canonical-design-outranks-intermediate-rounds.md` says it loses to the
catalogues.

## The provenance ladder

**Governing.** The canon and the ratification records above. Defended, not
weighed.

**Presumed wrong.** Everything else: research memos, censuses, design notes,
prior expert deliverables, source comments, commit messages, `.shared/state/`
notes, and all shipped code. Produced without a recorded human decision. Where
one conflicts with the canon it is drift and loses. Where several agree with each
other that is **not** corroboration; agents copy each other's framing, so mutual
agreement among unratified artifacts is evidence of shared drift.

**Maximally suspect.** Anything marked as an agent's own call.

Resolve conflicts by provenance, never by recency, detail, or confidence.

## Already ruled, and not reopenable

The lead designer ruled on 2026-07-26, reading the canon directly, that the Rust
per-script compile pipeline (roughly 3,700 lines) is forbidden and comes out. The
canon needs no amendment for this: `positive:45` reads "a dev-time Rust language
compiler (emits validated data and structural proofs, **never sees an end-user
script, never ships**) and a shipped Zig composed runtime (one hand-authored
engine comptime-specialised to the Rust-emitted data)", and `negative:38` kills
the dual locus by name.

Treat that as settled input. Your job is not to relitigate it but to chart what
replaces it and in what order.

## The shipped tree: claims to test, not context

Read these and test them. Do not reason within them.

- **Rust**, `mock/crates/`, 8,323 lines, 12 crates.
- **Zig runtime**, `mock/runtime-zig/src/`, 2,155 lines.
- **A Zig sketch**, `mock/research/sketches/202607260900_clause-in-zig/`, 5,646
  lines, 158 passing tests (`zig test eval.zig`). A sketch is throwaway audit
  trail by workspace convention, not a shipped artifact.
- **Benches**, `mock/benches/`.
- **A prior panel**, `mock/research/202607261200_chart-the-path-panel/`. Its
  brief carried the mislabelling described above. Read it as evidence, weigh it
  knowing that.

Whether any of it should exist, and whether it sits on the correct side of the
design's artifact boundaries, **are in scope for you to challenge, in those
words**. Report anything the canon does not license even where it falls outside
the question below, and do not soften it. Every judgment rides on a citation:
name the canon text and the `file:line` it is violated at.

## The question

Chart what the complete vehje framework and the complete first-party Clause
language are under the canon, and the ordered path from whatever the current
state genuinely is to that.

1. What artifacts does the canon call for, and what is each one's boundary?
2. What is missing from the complete set, **in dependency order**? Name what
   blocks what, and what the keystone is.
3. What has been established by the benches and sketches, and what has merely
   been assumed?
4. Which steps are provable now by a sketch, and which cannot be until an earlier
   step lands?
5. Where is the canon genuinely silent, such that the call belongs to the lead
   designer rather than to you or the dispatching agent? Silence is not
   permission.

## The standing gate

Before the work above, run the canon gate. If the work, or the state it builds
on, conflicts with the canon, refuse and return early with the conflicting canon
text and the offending `file:line`. If the canon is ambiguous on a point the
answer depends on, stop on that point and hand the call back rather than
resolving it. An early return is a completed dispatch, not a failed one.
