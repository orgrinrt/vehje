# Megapatch-era design docs — SUPERSEDED 2026-04-17

These documents describe the pre-Clause **megapatch** workflow: a Python compiler + YAML patch corpus whose purpose was to integrate ~458 third-party Stellaris mods into one coherent playset.

## What superseded them

On **2026-04-17** the project pivoted to **standalone Clause-first**: no third-party mods ship at runtime. All gameplay is authored in our own Clause crates on top of vanilla Stellaris + DLCs.

Current authoritative design lives in:
- **`docs/clause/DESIGN.md`** — Clause language + compiler + workspace architecture (pivot documented in §26)
- **`docs/TENETS.md`** — project tenets (pivot-amended)
- **Memory entries** at `~/.claude/projects/-Users-orgrinrt-Dev-stellar-heritage/memory/`:
  - `project_direction_standalone_20260417` — the pivot itself
  - `project_crate_workspace` — foundation/shared/content tier taxonomy
  - `feedback_think_fresh_not_catalogue`, `feedback_conventions_not_mandates`, `feedback_env_specific_playset_compile`, `feedback_cross_cutting_domains`, `feedback_externs_demand_driven` — principles

## What's in this directory

| File | Pre-pivot purpose | Current status |
|---|---|---|
| `DESIGN.md` | Megapatch as a two-bookend mod (fios → 458 mods → lios → heritage) | Superseded by `docs/clause/DESIGN.md` §26 |
| `ARCHITECTURE_V2.md` | Item-based pipeline architecture | Superseded — Clause compiler uses IR + pass framework (DESIGN.md §24, §32) |
| `BUILD.md` | Megapatch build system | Superseded — Clause workspace build (DESIGN.md §26.9, task W6) |
| `CLI_DESIGN.md` | `megapatch` CLI design | Superseded — `clause` CLI (DESIGN.md §27.3, task M10) |
| `PATCH_FLOW.md` | YAML patch flow through the megapatch compiler | Superseded — Clause `#[patch]` + `super.foo()` (DESIGN.md §23) |
| `PATCH_AUTHORING.md`, `PATCH_FORMAT_REF.md` | YAML patch authoring | Superseded — patches are first-class Clause (DESIGN.md §23) |
| `QUICKSTART.md` | First-time megapatch contributor guide | Superseded — a Clause quickstart will land with M10 (CLI) |
| `AUTOPATCH_CONCEPT.md` | Auto-conflict-resolution concept | Moot post-pivot — no third-party conflicts to resolve |
| `COMPILER_EXTENDING.md`, `compiler-passes.md` | Megapatch compiler pass framework | Principles carried into Clause (DESIGN.md §32); specific megapatch pass inventory is historical |
| `LINTING.md`, `LINT_CATALOG.md` | Megapatch lint catalog | Principles carried into Clause (Tenet 8); specific lint inventory evolves with Clause |
| `PATCH_FORMAT_REF.md` | YAML patch format reference | Superseded — Clause patch attributes (§23) |
| `STALE_PATCHES.md` | Detection of outdated patches against live mod sources | Moot post-pivot — no third-party mod sources to track |
| `SYSTEMS_MESH.md` | Cross-mod system integration diagram | Superseded — integration now happens across *our own* crates (DESIGN.md §26.6 cross-cutting) |

## Historical research preserved

- `.archive/research/modlist-audit/` — 466-mod audit + patch-signals cross-reference (preserved as research context informing our `_expanded` crate concepts per `feedback_think_fresh_not_catalogue`).
- `megapatch/` (at repo root) — YAML patch corpus, kept read-only as concept reference.

## When to read these

- **Implementation reference**: Never. See `docs/clause/DESIGN.md`.
- **Understanding decisions**: Useful for tracing why certain Clause design choices exist (many principles — pass framework, lint-first, item-based IR — carried forward from megapatch).
- **Concept archaeology**: If a current design doc reveals a gap, the pre-pivot docs may have explored the territory.

Do not update the files in this directory. They are frozen snapshots of the pre-pivot design.
