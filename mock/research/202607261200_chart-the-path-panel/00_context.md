# Context brief: what vehje's artifacts are, and which of them should exist

## The governing canon

`mock/research/canon/the-soul-of-vehje-positive-catalogue.md` and
`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md` are the
**governing design**. They are ratified with the lead designer in the loop, they
mark their human calls inline ("per op", "op's standing calls"), and they govern.
They are **required reading in full** before you answer anything below.

Alongside them, and also ratified:

- `mock/research/202607260100_op-standing-design-calls.md` (the lead designer's
  seven standing calls, in the designer's own words)
- `mock/design_rounds/202607241615/202607241545_topic.the-vehje-canon.md` and the
  ratification record `op-ratification-answers.md` in that round directory

## The provenance ladder

Everything you read sits on one of three rungs, and you are told which.

**Governing.** The canon above. Defended, not weighed.

**Presumed wrong.** Everything else you will read: research memos, censuses,
design notes, prior expert deliverables, source comments, commit messages, and
all shipped code. These were produced without a recorded human decision. They are
agent output. Where one conflicts with the canon, it is drift and it loses. Where
several agree with each other, that is **not** corroboration: agents copy each
other's framing, so mutual agreement among unratified artifacts is evidence of
shared drift.

**Maximally suspect.** Anything marked as the agent's own call, any note reading
"I decided", "derived call", "my reading". Treat those as the most likely place
the design was broken.

Resolve every conflict by provenance, never by recency, detail, or confidence. A
terse ratified line beats a thorough unratified memo.

## The shipped tree: claims to test, not context

The paths below are **claims**, not a description of what is correct. Read them
and test them. Do not reason within them.

**Rust, `mock/crates/`, 8323 lines across 12 crates:**
`vehje` (392), `vehje-ir` (1451), `vehje-lower` (1101), `vehje-typecheck` (830),
`vehje-resolve` (485), `vehje-codegen` (201), `vehje-fixpoint` (591),
`vehje-signature` (129), `vehje-schedule` (77), `vehje-runtime-abi` (2003),
`vehje-runtime-driver` (917), `vehje-runtime-gen` (146).

**Zig, `mock/runtime-zig/src/`, 2155 lines:**
`runtime.zig`, `value_arena.zig`, `value_image.zig`, `host.zig`,
plus test files.

**A Zig sketch, `mock/research/sketches/202607260900_clause-in-zig/`, 5646 lines,
158 passing tests:** `clause.zig` (lexer, parser, monomorphisation), `check.zig`
(Hindley-Milner inference with trait constraints, associated types, row
polymorphism), `eval.zig` (evaluator), `std.clause` (89 lines of library written
in Clause). By workspace convention a sketch is throwaway audit trail, per
`.claude/rules/bench-in-bench-harness-never-sketches.md` and
`cl-claim-sketch-discipline.md`.

Two specific claims worth testing directly:

- `mock/crates/vehje/src/lib.rs:72` — `pub fn run<T, Families, Effects, S>(...)`
- `mock/crates/vehje-typecheck/src/lib.rs:198` — `pub fn check<'a>(...)` taking an
  `Arena<'a>` and a `NodeRef`

## Explicitly in scope for you to challenge

**Whether any of the above should exist at all, and whether it sits on the correct
side of the design's artifact boundaries, are in scope.** Those are permitted
answers. "This crate should not exist." "This is on the wrong side of the split."
"The sketch is the real artifact and the shipped tree is not." All available.

Report anything the canon does not license **even where it falls outside the
question below**, and do not soften it. Every judgment rides on a citation: name
the canon text and the `file:line` it is violated at.

## The question

Chart what the complete vehje framework and the complete first-party Clause
language actually are under the canon, and what the ordered path to them is from
whatever the current state genuinely is.

Concretely, and in whatever order the canon makes correct:

1. What are the artifacts the canon calls for, and what is each one's boundary?
2. Which of the three bodies of code above belongs to which artifact, and which
   belongs to none?
3. What is missing from the complete set, in dependency order?
4. What has been established by the benches (`mock/benches/`) and the sketches
   (`mock/research/sketches/`), and what has merely been assumed?
5. Where is the state genuinely ambiguous under the canon, such that the call
   belongs to the lead designer rather than to you or to the dispatching agent?

## The standing gate

Before the work above, run the canon gate. If the work, or the state it builds
on, conflicts with the canon, refuse and return early with the conflicting canon
text and the offending `file:line`. If the canon is ambiguous on a point the
answer would depend on, stop on that point and hand the call back rather than
resolving it. An early return is a completed dispatch, not a failed one.
