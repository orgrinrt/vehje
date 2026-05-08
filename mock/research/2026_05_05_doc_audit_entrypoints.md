# Vehje entrypoint documentation audit, 2026-05-05

**Date:** 2026-05-05.
**Scope:** `README.md` (repo root, Tier 1), `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` and `mock/WORKFLOW.md.tmpl` (Tier 3, render to `docs/DESIGN.md` / `docs/PRINCIPLES.md`), and the rendered `docs/*` Tier 1 output. Companion to the per-crate audit at `mock/research/2026_05_05_doc_audit.md` (sub-agent).
**Method:** Manual read pass against documentation-writing + writing-style + vocabulary + ai-agent-framing workspace rules.

## Summary

Repo-root `README.md` is in good shape. The mock-root tmpls are clean of em-dashes after PR #26, but `mock/DESIGN.md.tmpl` carries the **largest Tier 3 leakage surface in the workspace**: extensive `stellar-heritage` cross-references (lines 10-13, 59-62), explicit `mock/design_rounds/`, `mock/PRINCIPLES.md.tmpl`, `mock/WORKFLOW.md.tmpl` self-references, and explicit `clause` mentions (line 13: `stellar-heritage/tools/clause/`) that should now be `vehje` post-rename — though the path in stellar-heritage may still be authoritatively named `tools/clause/` since that is the python reference impl directory name, not the renamed crate. This is a substantive question to resolve in the topic file. The rendered `docs/DESIGN.md` mirrors all the leakage in the .tmpl plus the auto-gen-header workspace finding.

## Per-file findings

### README.md (repo root)

Clean. Stand-alone identity, no leakage.

### mock/DESIGN.md.tmpl

- **Tier 3 leakage, line 1.** `# vehje. Architecture Design`. The period at end of "vehje." reads odd; consider `# Vehje: Architecture Design` for cleaner heading flow.
- **stellar-heritage Tier 3 leakage, line 10.** `**Initial seed.** This repo was extracted from the stellar-heritage` — open paragraph names the maintainer-private repo. End consumer landing on the rendered `docs/DESIGN.md` has no idea what `stellar-heritage` is.
- **stellar-heritage + clause Tier 3 leakage, line 13.** `stellar-heritage/tools/clause/`. Names the maintainer-private path AND uses the old `clause` name. If the path is authoritative for the python reference impl (which keeps the `clause` name), the framing needs to clarify that for the public reader: "the python reference implementation, named `clause` in its source location, sits in a separate repo" or similar. If `tools/clause/` has been renamed in the source repo to match, update.
- **Tier 3 leakage, line 15.** `mock/design_rounds/202604191300_topic.vehje-design-seed.md (3095 lines). Grammar is in the grammar seed. See the changelist mock/design_rounds/202604191301_changelist.initial-seed.md`. Names `mock/design_rounds/`, `changelist`, and a 3095-line internal-only doc. Replace with public-friendly framing.
- **stellar-heritage Tier 3 leakage, lines 59-62.** Three list bullets each name `stellar-heritage` as the resting place for not-yet-extracted code (`Clausewitz-specific code, bind targets, content`, `Python reference implementation`, `Deploy / playset / artgen orchestration`). The reader on crates.io has no business hearing about a local sibling repo. Reframe: "Clausewitz-specific code, bind targets, content. Will live in a future `vehje-jomini` companion crate when extracted." (Drop the stellar-heritage mention; the reader does not need to know where the unexported code currently sits on the maintainer's filesystem.)
- **`.tmpl` self-references, lines 66-68.** `mock/design_rounds/. 11 seed topics + 1 changelist. Fresh session starts here.` / `mock/PRINCIPLES.md.tmpl. language principles.` / `mock/WORKFLOW.md.tmpl. design → validate → implement discipline.` All three name `.tmpl` extensions and `mock/design_rounds/`. Replace with `Per-language principles live in PRINCIPLES.md.` etc.

### mock/PRINCIPLES.md.tmpl

(Sub-agent will cover.) Sample-checked: line 255 names `design rounds` in prose. Same pattern as arvo / hilavitkutin.

### mock/WORKFLOW.md.tmpl

- **Mockspace leakage, line 26.** `**Current phase: design.** The mockspace validates design via cargo`. Same pattern as arvo / hilavitkutin.
- **`.tmpl` references, line 28.** `every DESIGN.md.tmpl agrees with`.
- **cargo mock leakage, line 33.** `Regenerate. cargo mock to produce generated docs.`
- Same audience-decision question as arvo / hilavitkutin: is this Tier-1-rendered? If yes, rewrite. If contributor-only, no fix.

### docs/DESIGN.md (rendered Tier 1)

Inherits all the .tmpl leakage above, plus the workspace-wide auto-gen header. Will resolve after .tmpl fixes land + `cargo mock` regenerates.

### docs/PRINCIPLES.md (rendered Tier 1)

- **saalis Tier 1 leakage, line 88.** `hilavitkutin-api's 22 access/context/platform traits, saalis's`. The source `mock/PRINCIPLES.md.tmpl` likely has this verbatim. Replace `saalis` with a generic phrasing.
- **design rounds reference, line 264.** `exceptions with justification in design rounds.` Inherited from .tmpl. Replace with "exceptions with justification" alone, or "exceptions with documented justification".

### docs/STRUCTURE.GRAPH.dot, .png, .svg

Auto-generated assets; not checked in detail. The `STRUCTURE.GRAPH.png` and `.svg` are graph-shape files where the shape IS the content (per writing-style ASCII-diagram rule, real rendered images are allowed). No findings expected.

## Cross-cutting patterns

1. **`stellar-heritage` references are pervasive.** Multiple Tier 3 leakage sites in `mock/DESIGN.md.tmpl` name `stellar-heritage` as the resting place for not-yet-extracted code. The framing needs a complete pass: every mention of `stellar-heritage` is leakage of the maintainer's local-only repo to a Tier 1 reader.
2. **`clause` vs `vehje` post-rename ambiguity.** Line 13 names `stellar-heritage/tools/clause/`. The python reference implementation's directory name may still legitimately be `clause/`; this is a content question, not a pure rename drift.
3. **Same mock-root tmpl Tier 3 leakage pattern as arvo / hilavitkutin.** `design_rounds`, `.tmpl`, `cargo mock`, `mockspace` named throughout. Same mechanical fix.
4. **Title flow odd post-em-dash-replacement.** `# vehje. Architecture Design` with a period instead of em-dash reads stilted; consider colon.
5. **`saalis` reference in rendered docs/PRINCIPLES.md** indicates the .tmpl source still has it. Sample finding for the per-crate audit to confirm and the topic to address.

## Suggested topic-file scope

Vehje entrypoint cleanup wants:

1. **Strip `stellar-heritage` references from `mock/DESIGN.md.tmpl`.** Reframe the seed paragraph and the lists at lines 59-62 to talk about future `vehje-jomini` and "the python reference implementation" without naming the maintainer-private repo.
2. **Resolve `clause` reference at line 13.** Decide whether `stellar-heritage/tools/clause/` is current authoritative (python ref impl directory name), and if so, frame the reference around "the python reference implementation, named clause in its source repo," or remove entirely.
3. **Strip mock-root tmpl Tier 3 leakage.** `design_rounds`, `.tmpl`, `cargo mock`, `mockspace` references throughout DESIGN.md.tmpl, PRINCIPLES.md.tmpl, WORKFLOW.md.tmpl.
4. **Title flow fix.** `# vehje. Architecture Design` → `# Vehje: Architecture Design`.
5. **`saalis` reference in PRINCIPLES.md.tmpl.** Replace with generic phrasing.
6. **`mock/WORKFLOW.md.tmpl` audience decision.** Same as arvo / hilavitkutin.

The vehje doc round needs one topic file with one doc CL covering items 1-6. The src CL is unnecessary at this scope unless the per-crate audit surfaces rustdoc drift.
