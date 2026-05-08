**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** vehje (mock/DESIGN.md.tmpl, mock/WORKFLOW.md.tmpl)

# PR-review follow-up: mock-root Tier 1 leakage

PR-review follow-up to round 202605051400. Three Tier 1 leaks survived
the round in the mock-root templates (the per-crate sweep was thorough;
the mock-root pass was lighter):

1. `mock/DESIGN.md.tmpl` references `stellar-heritage` in body prose at
   five sites. `stellar-heritage` is a maintainer-private sibling repo
   on the maintainer's local filesystem; per `documentation-writing.md`
   and `readme-format.md` Tier 1 rules, sibling local-only repo names
   are forbidden in body prose.
2. `mock/DESIGN.md.tmpl` `## See also` block points at `mock/PRINCIPLES.md`
   and `mock/WORKFLOW.md` (Tier 2 paths) from a Tier 1 surface. Either
   drop the bullets or repoint at the rendered `docs/` siblings.
3. `mock/WORKFLOW.md.tmpl` carries "the mockspace under `mock/` carries
   the design… seed topic is `vehje-design-seed`; the seed changelist…"
   contributor-process narrative on a Tier 1 surface. Banned vocabulary:
   `mockspace`, `mock/` path, `seed topic`, `changelist`. `seed` as a
   status label is on `vocabulary.md`'s banned list.

## Decisions

### Decision 1: stellar-heritage references

Strip the literal name. The Python parity-oracle story can stay;
rephrase as "extracted from a Python prototype" / "Python reference
implementation". Any "lives in stellar-heritage until..." claim
becomes "out of scope until the dedicated repo lands" or similar.

### Decision 2: See also bullets

Drop the bullets entirely. The rendered `docs/PRINCIPLES.md` and
`docs/WORKFLOW.md` are next to `docs/DESIGN.md`; readers find them
through the file listing.

### Decision 3: WORKFLOW status section

Replace the "seed topic / seed changelist" sentences with
"Pre-implementation. The compiler and runtime are in design phase;
real crates are stub-shaped while the public surface stabilises."
Drop the `mockspace`, `mock/`, `seed`, `changelist` vocabulary.
