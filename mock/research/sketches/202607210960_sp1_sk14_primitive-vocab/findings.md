# SP1 (+ SK14) findings: the primitive vocabulary stays bounded

**Date:** 2026-07-21 | **Outcome:** WORKS (scoped) | Zig 0.16.0 | artifact `vocab.zig`
**Settles:** SP1 (the 2001/1845 open question: does the primitive vocabulary stay bounded when authoring
semantic definitions), scoped; folds SK14 (Scala-3 capture = the reach coeffect).

## Result
Enumerating the distinct interpreter primitives each of the 12 Core forms needs, plus one arithmetic family and
the lease axis, the union is **25 primitives** (low tens, bounded, matching the survey: Racket 15, Jsonnet 15,
Pandoc 14+19). Adding a SECOND family (a doc family: emit_record, monoid_append, interp_splice) adds **0** new
primitives, it reduces entirely to the existing Core vocabulary. So a Deegen-style semantic definition over the
Core plus families is tractable: the vocabulary stays bounded and families reduce to it rather than sprawling.

## SK14 note (Scala-3 capture = the lease production floor)
Scala-3 capture checking tracks a value's capture set (the capabilities it captures). In vehje this is the lease
coeffect: a produced value's capture set IS its reach set (the binders it captures), the `region_open` /
`region_close` / `reach_or` primitives in the vocabulary, checked against what may escape. It is the same
mechanism SK4/SK11/SK12 validated (reach bitmask / depth-min), approached from the capture-checking angle. No
separate machinery; the lease axis IS the capture floor.

## Scope (honest)
This is a SCOPED enumeration (12 forms + 2 families) that confirms the boundedness hypothesis and the
family-reduces-to-Core property. A FULL coverage test requires authoring the real census families (jomini, lua,
w3, ts4, the doc DSL, polka) and measuring the vocabulary across all of them; that is deferred to when those
consumers are built. The positive signal (bounded vocabulary, families adding ~0) holds for the scoped case and
matches the prior-art survey.

## Design impact
The primitive vocabulary is bounded (~25, low tens), so the semantic definition that generates the interpreter
arm + stencils + differential tests (SK1, BN1, SK23) is tractable and closed. Families reduce to the Core
vocabulary. The lease/capture floor needs no separate machinery beyond the reach coeffect.
