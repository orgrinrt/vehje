# Megapatch Compiler Review — Mod-Toolchain / Build-System Lens

Reviewer: domain expert in modding toolchains (Skyrim / Civ / CK). Scope of
review: the Python pipeline at `tools/megapatch/megapatch_compiler/`, the
patch corpus under `megapatch/patches/` and `megapatch/heritage/patches/`,
the PATCH_AUTHORING / LINTING / PLAYTEST docs, and a representative sample
of patches (ship_sizes, traits `_declarations.yml`, building_bureaucratic_1).

I've been in your shoes. This review is harsh where it needs to be; the
bones of the tool are genuinely good, which is why the gaps are worth
naming.

## What's Good

**The fundamental shape is right.** An LLVM-style pass pipeline with a
scheduler that caches each pass's artifact by input-fingerprint
(`passes/scheduler.py:1-40`) is exactly the architecture a corpus of
~4,500 patches over 450 mods needs to stay iterable. Most modding toolchains
in this space are bash spaghetti; you skipped that entire era.

**The item-centric YAML layout is a genuine DX win.**
`PATCH_AUTHORING.md` captures the key insight: one file per item, with
`_declarations.yml` for bulk declarative sweeps. The shape in
`megapatch/patches/common/buildings/building_bureaucratic_1.yml` — four
patches (inject_field x3, replace_in_item) in apply order with rationale
per patch — reads like a diff-review comment. A six-month-later audit has
real odds of being tractable.

**Conflict-resolution tenets are load-bearing in the linter layer, not
just docs.** `PatchMissingRationaleOnSurgery`
(`linters/builtin/patch_shape.py:341`) pushes back on undocumented
surgery. `PatchReferencesUnknownMod` catches typo-or-removed-mod cases.
The `status` enum with `PATCH_UNKNOWN_STATUS` as an ERROR
(`patch_shape.py:308`) prevents silent typos like `reviwed`.

**Extraction + compile ops are correctly orthogonal and cascading is
supported.** `replace_in_item.apply_on(base_content=...)`
(`compile/ops/replace_in_item.py:108`) lets stacked patches chain without
re-extracting — the right shape for "5 mods touching one building."

**`trace` command is the single best DX feature.** Re-running
`compile_item` with `patches[:0..N]` and diff-ing
(`tools/trace.py:1-12`) guarantees the trace never drifts from the
build. In Skyrim-land we'd kill for this.

**File-scoped `@variable` inlining at extract time**
(`clausewitz/vars.py:1-20`) — you've correctly identified that
`@corvette_build_time` in a mod's local `.txt` can't survive being
embedded in your aggregated file. This is a real Paradox-format gotcha
almost nobody handles.

**Release gating via atomic staging + promotion**
(`build.py:215-231`) with `.dist/` only touched on zero blocking
findings is exactly Tenet 8 made real.

## What's Bad

**CompileUnit separation on paper, leakage in practice.**
`units/builtin.py` cleanly factors megapatch / heritage / heritage_late /
bundle. But the writer (`writer.py:1-17`) still hardcodes an
`inline_scripts/megapatch` assumption for heritage output that Heritage
then ships on top of. More concretely: `_reconstruct_results_from_tree`
(`build.py:486-521`) reads the staged tree back to run tree linters,
loses `source_label`, and the ownership resolver for tree-level findings
degrades to UNKNOWN. That means cross-reference lints CANNOT attribute
ownership correctly for `bundle` builds — and bundle is the very unit
where cross-mod ownership matters most.

**Patch linter / item linter asymmetry in `build.py`.**
`_run_item_linters` is called on an empty list
(`build.py:396: item_findings: list[Finding] = []`). The code
comment concedes pipeline doesn't expose results back. So every
ItemLinter (depth-exceeded, mixed tabs, etc.) is effectively dead for
classic-linter dispatch — surviving only via the pass-based paths. New
contributors writing an ItemLinter per the LINTING.md docs would get
silent no-ops. This is a correctness hole, not just a wart.

**688 stale-patch errors shipped as "doesn't block release."**
`STALE_PATCHES.md:80-97` states plainly: `stats.errors` is distinct
from lint gate; stale patches just don't emit. But 688 integration
patches silently dropping on release is an existential issue for the
product. Tenet 7 ("Consolidate, don't choose") becomes "we intended to
consolidate but a quarter of it silently noop'd." A modder auditing
the build won't see this unless they read `build_manifest.json →
compile_errors[]` and know to grep.

**Patch-authoring onboarding is behind docs, not tooling.** The
`PATCH_AUTHORING.md` + `LINT_CATALOG.md` pair is excellent, but a new
contributor running `megapatch new-patch common/traits/trait_psionic
--type override --edit` gets a file with a `content:` placeholder and
no per-type example template. They have to go back to docs to remember
what override vs replace_in_item vs inject_field does. The scaffold
should include type-specific comment blocks.

**`prefer_mod` without `rationale` is still waved through** for
declarative entries (`_SURGERY_TYPES` excludes `prefer_mod` at
`patch_shape.py:300-305`). The conflict-resolution rule says
formatting-only collapses are the ONLY legal `prefer_mod` without
per-entry review. No lint enforces that — a modder can stack
`prefer_mod` entries into `_declarations.yml` en masse and nothing
fires. That's the blanket-sweep escape hatch the rules forbid.

## What Needs Fixing

### CRITICAL

1. **ItemLinter dispatch is stubbed out.** `build.py:396` — fix
   pipeline to return `list[(Item, CompileResult)]` so `_run_item_linters`
   actually runs. Today, `TriggerDepthExceeded` and every other
   ItemLinter only fires via the pass framework; contributors writing
   classic ItemLinters per `LINTING.md` will produce no-ops.

2. **Tree-linter ownership is UNKNOWN on reconstruction.**
   `build.py:486-521` — reconstruct real `CompileResult.source_label`,
   or plumb CompileResults out of the pipeline. Without this, the
   ownership baseline (`tests/baselines/external_findings.jsonl`) is
   unreliable for tree-level findings — release builds could block on
   external noise mislabeled as first-party.

3. **Stale-patch errors need a release gate escalation path.** 688 is
   not a playtest number; it's "a quarter of megapatch is silently
   dropped." Either: (a) add a hard `FIRST_PARTY_COMPILE_ERROR` policy
   that fails release with a count >0, or (b) downgrade to a visible
   warning stream at the top of the CLI summary (not buried in
   `build_manifest.json`). Current `cmd_build` output mentions
   "Compiler errors: N" but a fresh user won't know 688 means
   "megapatch is 25% broken." The count is equal in impact to a
   blocking finding.

### HIGH

4. **`prefer_mod` without `rationale` is allowed in `_declarations.yml`.**
   Add `PREFER_MOD_IN_DECLARATIONS_WITHOUT_FORMATTING_NOTE` — ERROR
   unless the rationale explicitly contains the string
   `formatting-only` / `byte-identical` / similar whitelist marker.
   Enforces the rule in `.claude/rules/megapatch-conflict-resolution.md`.
   `patch_shape.py:300-305` is the right home.

5. **`@variable` items in `_declarations.yml` shouldn't be compilable
   entities.** 136 of the 688 stale errors come from
   `@variable`-as-item (`STALE_PATCHES.md:34-50`). Filter these at
   ingest (`ingest/extract_items.py`), flag them declarative-only, and
   drop them from the compile universe. Tagged "future fix" in docs;
   it's 20% of the noise.

6. **Parser doesn't handle `inline_script` references.** The Clausewitz
   parser (`clausewitz/parser.py`) treats `inline_script = "foo/bar"` as
   opaque text. No pass resolves the inline_script call graph, so depth
   analysis underestimates when a shallow-looking trigger chain goes
   deep via inline_script (Tenet 8 explicitly calls out depth inlining
   as belonging to the compiler). The `SCRIPTED_X_DEPTH_INLINED` lint
   row is in the catalog but marked "deferred" — this one bites in
   playtest repeatedly in my experience.

### MEDIUM

7. **Patch priority order is surprising for newcomers.**
   `PATCH_AUTHORING.md:241-244` lists:
   `override > replace_in_item > inject_field > prefer_mod > delete_item
   > insert_item > no_override`. But in the patch files patches are
   applied in YAML list order (see `_load_patches_for_item` sorted by
   `source_file, source_line`). These two ordering schemes coexist;
   which one wins for the same item? A trace output note would help,
   and the docs should clarify explicitly.

8. **Localisation handling is a one-off in the writer.**
   `writer.py:111-150` — ad-hoc logic detecting `localisation/<lang>`
   from the directory path. It works, but localisation files have
   their own shape rules (BOM, `l_<lang>:` header, indent-2) that
   deserve a dedicated output op or pass. Right now a bug in the
   re-indent logic silently ships (stripped empty lines, for
   instance).

9. **No cwtools validation gate on release.** `--require-cwtools` is
   off by default (`__main__.py:62`). Given Tenet 8 ("Clean Builds Or
   No Builds") and the existence of `.cwtools/config/*.cwt`, release
   should REQUIRE cwtools to have validated the output. Otherwise the
   gate's guarantee is only "our lints passed" — cwtools catches a
   lot we don't (scope transitions, vanilla vocabulary mismatches).

### LOW

10. **`cmd_migrate_patches` ships as a shippable subcommand but is
    one-shot tooling.** Once the migration is done, this should move
    to `tools/legacy/` or be hidden behind an env var. New
    contributors will see `megapatch migrate-patches` in `-h` and
    wonder.

11. **`deploy/deploy.py:21 DEFAULT_CUSTOM_MODS` hardcodes four
    strings.** If `heritage_late` is renamed or a new unit ships,
    this list needs manual update. Read from the CompileUnit
    registry instead.

## What Could Be Improved

**`megapatch new-patch` should understand conflict context.** Run
`discover` first, check if the item has >1 source mod; if yes, seed
the template with a checklist of `source_mods:` entries pre-filled and
a diff of all source versions beside the `content:` placeholder. Half
the "per-entry review" friction is modders not knowing which mods even
touch an item.

**Add a `megapatch diff-sources <dir>/<item>` command.** Show the
N mod versions of an item side-by-side. A modder shouldn't have to
`cd ~/Library/.../281990/<mod_id>/` and grep manually. This is the
per-entry-review tool; without it, "review each entry individually" is
aspirational.

**Cache semantics need a user-facing story.** The scheduler caches by
input fingerprint per pass — great. But a modder editing one YAML
file expects roughly one-pass rebuild; they don't know what
"dirty passes" means until a stale `.cache/megapatch.sqlite` forces
`megapatch cache clear`. Print cache hit/miss counts in the CLI
summary (the data is already in `pass_reports`), and document the
invalidation story in BUILD.md.

**Patch authoring feedback loop on a single item is still slow.**
`--dir` filter helps, but even that triggers a full pipeline. A
`megapatch compile-item <dir>/<item>` that runs JUST the compile for
one item + its patches + its deps would make inner-loop iteration
seconds not tens-of-seconds.

**Surface per-directory load-order metrics.** `megapatch loadorder`
prints the global order; the follow-up question modders always ask is
"for THIS directory, which mod wins?" Add a third column to the
load-order output based on FIOS/LIOS rule per dir.

**Bundle mode should warn on missing workshop dependencies.** If
`--unit bundle` compiles content that cites mod IDs not downloaded
locally, emit a finding; today it silently extracts nothing and
eventually FAILs EXTRACTION. Pre-flight is cheap.

## Specific Code Locations

- `build.py:396` — ItemLinter dispatch stubbed; `item_findings = []`.
- `build.py:486-521` — Tree linters reconstruct from filesystem, lose
  `source_label`; ownership defaults to UNKNOWN.
- `__main__.py:145` — CLI summary shows `Compiler errors: N` as a
  flat count; no severity escalation for first-party errors.
- `ingest/parse_patches.py:337-341` — status typo warning is added to
  `warnings` but also caught later by `PATCH_UNKNOWN_STATUS`. Pick
  one — dual reporting confuses the build output.
- `compile/ops/prefer_mod.py:30-33` — silently skips when preferred
  mod already LIOS-wins. Correct behavior, but emit a debug-only
  finding so the count of no-op prefer_mods is visible at audit time.
- `clausewitz/parser.py:28` — `_IDENT` regex permits `.` and `:`
  inside identifiers. Stellaris IDs don't actually contain `:` in
  practice; this widens the regex unnecessarily and could cause
  entry mis-parsing on malformed input.
- `linters/builtin/patch_shape.py:300-305` — `_SURGERY_TYPES`
  excludes `prefer_mod`; rationale enforcement skips it. Fix this.
- `writer.py:111-150` — localisation writer is ad-hoc; deserves its
  own op/pass.
- `deploy/deploy.py:21` — `DEFAULT_CUSTOM_MODS` hardcoded; should
  come from the CompileUnit registry.
- `docs/megapatch/STALE_PATCHES.md:80-97` — policy decision to not
  gate on compile errors needs revisiting; 688 first-party errors is
  not "known issues."

---

**Net assessment:** This toolchain is better than 80% of what ships for
modded Bethesda/Paradox games. The pass pipeline, the trace command,
the item-centric YAML, and the tenets-as-linters pattern are all
individually ahead of the curve. The gaps are (a) classic-linter
dispatch incompleteness, (b) compile errors being too quiet for the
damage they represent, and (c) the last-mile of making conflict
resolution discipline fully enforced by the compiler rather than
relying on author diligence. Fix those three and this is a
reference-quality mod-toolchain.
