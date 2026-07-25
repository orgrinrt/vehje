# Sketch: one operation definition, evaluated at every discharge site

**Date:** 2026-07-26
**Task:** #52
**Round:** `mock/design_rounds/202607260500_topic.the-languageauthor-specialisation-stage.md`

## Hypothesis

A family operation's semantics can be defined once, as a closed primitive
vocabulary emitted as data, and evaluated at every point of the canon's
binding-time lattice without the meaning being written twice. Concretely: the
same bytes drive a Rust `const fn` at Bundler grade, a Zig `comptime`
specialisation at LanguageAuthor grade, and a Zig runtime interpreter at Runtime
grade, and all three agree on every input.

If that holds, the representation is admissible. If any site cannot evaluate the
encoding, the meaning has to be authored again for that site, and the "one
signature, many projections" claim (positive catalogue, Frontier F) fails for
family semantics specifically.

## Why this shape, and why a sketch rather than a bench

Two independent expert reads
(`mock/research/202607260600_semantics-projection-design-space.md`,
`mock/research/202607260700_semantics-projection-second-read.md`) agreed, each
grounded in quoted canon, that a closed primitive vocabulary emitted as data and
specialised at Zig comptime fits the canon. That is the premise this sketch
tests operationally rather than re-argues.

The second read derived the constraint that makes this the first thing to build:
the question is not "how do semantics cross from Rust into Zig". That is one edge
of a four-node lattice, and the tree already contains discharge sites on the Rust
side (`mock/crates/vehje-lower/src/lib.rs:126-128`, the Bundler const-fold FIXME;
`:92-94`, HostLoader load-time lowering). So the admissibility test is: **a
representation must be evaluable by rustc-compiled Rust and by Zig, at comptime
and at runtime, or the meaning gets written twice.**

It is a sketch and not a bench because its primary question needs no timer. Per
`bench-in-bench-harness-never-sketches.md`, a feasibility question with a
WORKS/FAILS outcome is a sketch; the moment a timer is needed it becomes a
harness cell. The timing comparison between the specialised and interpreted arms
is a real bench and follows this, in `mock/benches/`, once there is something
worth timing. The second read classified this as a bench cell; that classification
is the one point where this sketch departs from it, and the departure is the
workspace rule rather than a judgement call.

## The encoding under test

A stack program over a closed vocabulary, deliberately minimal. The question is
whether the projection works at all, not whether the vocabulary is complete;
a vocabulary that is too small to express a real family would still answer the
admissibility question, and a too-large one would confound it with coverage.

```
0x01 n   push argument n
0x10     add     (pop b, pop a, push a + b)
0x11     lt      (pop b, pop a, push a < b as 0/1)
0x12     mul     (pop b, pop a, push a * b)
0x13     sub     (pop b, pop a, push a - b)
```

`add` is therefore `[0x01 0x00, 0x01 0x01, 0x10]`, and that byte string is the
single definition all three sites consume.

## Outcome

Recorded in `findings.md` after the runs. Not yet run.
