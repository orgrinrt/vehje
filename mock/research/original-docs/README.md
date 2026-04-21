# Clause language documentation (archived seed)

**Active development has moved.** As of 2026-04-19 the Clause language lives in its own repo:

**https://github.com/orgrinrt/clause** — `~/Dev/clause/`

All documents in this directory were ported to `orgrinrt/clause/mock/design_rounds/` as dated seed topics (`202604191300_topic.*`). For current design work, open that repo; do not edit these files expecting them to be authoritative.

## What stays here

- `docs/clause/*.md` — archived design snapshots as of the extraction.
- `tools/clause/` — the Python prototype implementation. It remains running + green (1592 tests) as the **parity oracle** during the Rust rewrite. Bug fixes OK, new features NO.
- Clausewitz/Jomini-specific surface — to migrate later into a future `clause-jomini` extension repo.

## What moved

- Language spec (DESIGN, GRAMMAR, EBNF, GRAMMAR_FRAMEWORK, MANIFEST_SPEC) — seeded into `clause` repo.
- Briefs (generative_pipeline, macros_slice3, scratch_variables, storage_backend) — seeded into `clause` repo.
- Audit (LANG_AUDIT_2026_04) — seeded as state snapshot.
- In-flight task designs (VALIDATOR framework, STRICT2–7, MAC5, T1 inference, etc.) — captured in the initial-seed changelist as gaps for the first `clause` session to consolidate.

## Why the split

The Clause language is game-agnostic. Keeping it in a Stellaris-mod repo constrained the vision (Witcher 3, Sims 4, Lua, and other targets are in scope). Rust compiler + Zig runtime + C ABI gives single-binary distribution — end users installing the toolchain no longer need Python + venv.

See `orgrinrt/clause/mock/design_rounds/202604191301_changelist.initial-seed.md` for the full rationale and the list of gaps the first fresh session needs to close.
