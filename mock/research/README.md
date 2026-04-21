# Clause research — imported context + lessons + port plan

This directory carries the research, design history, and lessons
that inform the Rust rewrite of Clause. It has three layers:

## 1. Original docs, copied in

Pulled from `~/Dev/stellar-heritage/{docs/clause, .archive/{research,design-history}}`
on 2026-04-21. Source-of-record still lives there; these are local
copies so the Rust repo has stable references independent of the
stellar-heritage tree.

- `original-docs/` — canonical Python-era docs (DESIGN, GRAMMAR,
  EBNF, MANIFEST_SPEC, LANG_AUDIT_2026_04, COMPILER_PRUNE_PLAN,
  briefs/).
- `original-reviews/` — third-party-style reviews the author
  commissioned on the Python implementation and language design
  (clause-review-{compiler-impl, language-design, migration,
  modder}, compiler-review-{llvm-lens, mod-domain, python-infra},
  plus rust-traits-research, typed-dsl-prior-art,
  cwtools-formalization-review).
- `original-design-history/pre-clause/` — 2026-04-14 through
  2026-04-16 design seeds before the Python implementation
  diverged.
- `original-design-history/megapatch-era/` — architecture-v2,
  patch flow, lint catalog, cli design, etc. from the
  pre-Clause `.txt`-patch pipeline that Python Clause absorbed.
- `clausewitz-context/` — research on the target language
  (Clausewitz script: Paradox engine's scripted format).
  Includes jomini spec research, parser research, load rules,
  and five domain reviews (game-design, domain-wiring,
  performance, practical-modder, technical-correctness).

These are frame-of-reference only. The Rust port does not
inherit their framing or their decisions by default.

## 2. Author-side research written for the port

- `python-clause-survey.md` — deep survey of the Python
  implementation (2026-04-21): language surface, compiler
  architecture, pass framework, CLI, what's done vs deferred,
  downfalls, features the Rust port must replicate.
- `lessons-learned.md` — what we learned from Python Clause:
  decisions worth inheriting, decisions worth rejecting,
  downfalls, features strictly better in Rust. Reads the survey
  and the reviews together.
- `parity-plan.md` — the roadmap from today's Rust skeleton
  (clause-lex real, everything else stub) to feature parity with
  Python Clause and beyond. Per-crate scope, design-round
  ordering, dependency graph, exit criteria.

## 3. How to use this directory

- When planning a new clause design round: read the relevant
  chapter of `python-clause-survey.md`, check
  `lessons-learned.md` for what to replicate vs what to reject,
  confirm scope against `parity-plan.md`.
- When writing a new feature: peek at the matching Python file
  (paths cited in the survey) for the semantic ground truth,
  then decide the Rust shape from `lessons-learned.md`
  principles. Never mirror Python line-for-line.
- When in doubt about a language-surface detail: the EBNF
  (`original-docs/CLAUSE_EBNF.md`), the grammar doc
  (`original-docs/GRAMMAR.md`), and the language-design review
  (`original-reviews/clause-review-language-design.md`) are the
  reference, not the Python parser's current behaviour.

## 4. What is NOT here

- The Python source code itself. That stays at
  `~/Dev/stellar-heritage/tools/clause/compiler/`. Survey cites
  absolute paths; open them directly when you need them.
- The test corpus. Lives at
  `~/Dev/stellar-heritage/tools/clause/compiler/tests/`. 1511
  test functions across 99 files. Treat as a semantic oracle:
  a passing Python test against the same input should pass
  against the Rust compiler too (where the feature is
  implemented). Deliberate divergence is allowed where the
  survey flags a decision reversal.
- The generated Clausewitz output fixtures. Live at
  `~/Dev/stellar-heritage/tools/clause/compiler/tests/golden/
  outputs/`. Thin today (one file visible) but will grow; when
  the Rust codegen is ready, cross-run both compilers against
  the same `.cse` input and diff the `.txt` outputs.

Do not commit the Python source or the test corpus here. Hard
copies will rot; the originals are a path away.
