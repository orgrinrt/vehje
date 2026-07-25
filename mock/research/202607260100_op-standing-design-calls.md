# op's standing design calls for the vehje arc

**Date:** 2026-07-26
**Status:** standing directives, not a design round artifact
**Scope:** the whole vehje framework arc and the first-party Clause language

These are op's own words and the calls they carry, written down so they survive
compaction and so no later round re-litigates them. They are canon on the
intent axis. Where a call names a specific mechanism, the mechanism is a
proposal and the measurement decides; where it names a property, the property
is a mandate. That split is itself one of the calls (call 4 below).

## 1. The completeness bar is the full pre-redesign grammar

> "Real language and running isn't the bar, but completeness for the 'vehje the
> language' that was specced long time ago, before we did the re-design and
> updating and changing the identity wholesale. The old syntax for the vehje
> authoring lang is still the benchmark and dogfood target, not any arbitrary
> lang. So nothing is done until we can express the full intended lang,
> generics, macros, assoc types and such, as intended."

The normative grammar is `mock/research/original-docs/CLAUSE_EBNF.md`. A
runnable subset is not a milestone worth claiming. Generics, macros, associated
types, and traits are in scope, not deferred surface. The census that walks the
grammar against the Core is `202607251530_language-completeness-census.md`.

## 2. The runtime is computationally self-sufficient

> "Of course the runtime should be standalone sufficient to compute and do all
> of it. Host should only basically configure the runtime and give it the
> allocations, nothing more. It does receive the values, but it should *NOT*
> have to give the simple things like fucking arithmetics..."

The host's job is configuration, lent allocations, and receiving results. That
is the whole of it. The host does not supply arithmetic, comparison, or any
other elementary computation, and a design that requires it to is backwards.

This corrects the shape that had shipped: arithmetic lowered to
`Raw(ARITH, ...)` and crossed the C ABI to a host-provided handler, which meant
the language's elementary operators existed only as a function in a test file.
The correction is not "move the handler into the runtime as a default"; it is
that computation belongs to the runtime and never depended on the host.

Note the boundary this does NOT move: the framework still owns no family. vehje
the framework is generic over families. The first-party Clause runtime is a
different artifact from the framework, and it ships its own computation. The
separation of concerns survives; what changes is which side of it arithmetic
was on.

## 3. Clause ships a standard library, written in Clause

> "We need a stdlib to ship with clause of course, it's not complete until that
> is true. And we have to write it *in* clause, so it dogfoods this thing"

Two requirements, both load-bearing:

- Completeness includes a standard library. Without one the language is not
  done, regardless of what the compiler and runtime can do.
- The stdlib is authored in Clause, not in Rust or Zig. Writing it in Clause is
  how the language proves it can carry real code, and every gap it hits is a
  finding about the language rather than a missing convenience.

The stdlib is therefore both a deliverable and the acceptance test for calls 1
and 2. It cannot be written until the language expresses enough to write it,
and it cannot run until the runtime computes on its own.

## 4. Intent is canon; the named vehicle is bench-decided

> "We don't just want to make a naive runtime. We want to bake in all the
> optimisations we know will be beneficial to vast majority of cases, and make
> it opt-out for those that want to hand-bake them. Host-config is again the
> thing that controls this, but we should give sensible and ergonomic defaults
> either way."

> "if string interning as such would be *less* efficient and optimal than
> something else, then obviously we'd do the better one. When in doubt, reach
> for benches. We have a brilliant harness to design and run benches on!"

The mandate is the property: known wins are on by default, opt-out by host
config, ergonomic either way. The named candidates (automatic string interning,
interning of equality-heavy compounds, active deduping, aliasing and
reclamation as values leave scope) are proposals to measure, not a
specification to implement. Two of the four have already been refused by
measurement; see `202607251610_baked-in-optimisation-mandate.md`.

## 5. Every bench runs in the harness

> "Even for zig-authored benches, you *NEED* to plug them into the harness.
> It's crucial that we get all data as cvs results, analysable"

Applies regardless of which language authored the measured code. A standalone
timing loop is not a bench: it produces numbers nobody can re-run, compare, or
audit. The harness gives per-variant cdylib isolation, a shared realistic
workload, calibrated repetition, and a committed CSV trail.

## 6. Optimal over fast

> "we aren't about 'fastest', we are about 'most ideal and optimal'."

Sequencing follows the ideal shape, not the shortest path to something running.
This is why a stopgap that greens a gate is worse than the red gate it would
have replaced.

## 7. No bylines

No `Co-Authored-By` trailer on any commit, PR body, PR comment, issue body, or
issue comment. op's reasoning: the design is human-authored and human-overseen,
and the agent is executing it mechanically, so the byline does not stand. The
`Generated with` advertising suffix is forbidden everywhere for the same
reason.

## How these interact

Calls 1, 2, and 3 compose into one acceptance condition rather than three
separate ones: **a standard library, written in Clause, exercising the full
grammar, running on a runtime that computes without host help.** Each of the
three is a prerequisite for that sentence being true, and none of them is
individually sufficient.

Calls 4, 5, and 6 govern how the work gets done rather than what gets built.

## See also

`canon/the-soul-of-vehje-positive-catalogue.md` and
`canon/the-inverse-of-vehje-negative-catalogue.md` (the identity oracle),
`202607251530_language-completeness-census.md` (the grammar walk),
`202607251610_baked-in-optimisation-mandate.md` (call 4's measurements),
`202607252100_handle-clause-representation.md` (the effect discharge design),
`original-docs/CLAUSE_EBNF.md` (call 1's normative grammar).
