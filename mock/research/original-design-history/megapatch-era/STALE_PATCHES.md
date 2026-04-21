# Stale Patches Audit

**Status:** 688 compile errors from the corpus as of the #291 landing.

These are patches whose references no longer resolve against the current
source mods — either the source mod updated its content and the patch
hasn't been refreshed, or the patch was shaped around something that
the compile engine can't handle in its current form.

## How to see the current list

```
rm -rf .debug
python3 -m megapatch_compiler build --unit megapatch --mode debug
python3 -c "
import json
m = json.load(open('.debug/build_manifest.json'))
for e in m['compile_errors']:
    print(f'{e[\"category\"]:20} {e[\"source_file\"]}:{e[\"source_line\"]}')
"
```

Or read `.debug/build_manifest.json → compile_errors[]` directly.

## Breakdown (from the most recent audit)

| Category | Count | Root cause |
|---|---|---|
| FIELD_NOT_FOUND | 451 | inject_field targeting a field name the source mod no longer uses (or never had) |
| EXTRACTION_FAILED | 202 | winner source couldn't produce extractable content — most in `_declarations.yml` for `@variable` items |
| FIND_NOT_MATCHED | 35 | replace_in_item find-string doesn't appear in the winner source (mod-upstream changed) |

## Known patterns

### `@variable` items in `_declarations.yml`

136 errors from files like `common/terraform/_declarations.yml`,
`common/diplo_phrases/_declarations.yml`,
`gfx/portraits/portraits/_declarations.yml`.

Root cause: these files declare `no_override` / `prefer_mod` patches
for `@variable` references, but `@variable` is a Clausewitz file-scope
constant, not an Item. The compile engine can't extract them from
source mods because they're not addressable as entries.

**Fix path:** either (a) filter out `@variable`-prefixed items at
ingest so they're recorded as reviewed-and-skipped without being
compiled, or (b) rewrite these as documentation/comments rather than
patches. The existing `no_override` entries effectively achieve (a)
already — their `rationale` captures the review decision.

### trait patches targeting old field names

16 errors in `common/traits/trait_*.yml` with `FIND_NOT_MATCHED` on
`modifier = { <job>_jobs_bonus_workforce_mult = ... }` — the Stellaris
vanilla trait system moved to `pop_job_<job>_add` or similar in a
4.x patch. These patches need to be refreshed against the current
vanilla trait format.

### inject_field against evolving component templates

~100 errors in `common/component_templates/*.yml` with FIELD_NOT_FOUND
on `position_priority`, `ai_weight`, etc. — the source mods (NSC3,
ESC, ACOT) have reshuffled these fields in recent updates. Each
patch needs a source-mod review to pick the current field name or
switch to literal-content mode (`field + content` instead of
`field + mod`).

### archaeological_site_types + anomalies find-patterns

36 errors from `find_not_matched` on `is_planet_class` / `OR` blocks.
The source mods (MEM, most of them) reformatted these blocks
— typically whitespace or key reordering. The patches' find strings
need updating.

## Fix workflow

1. Pick a patch file from the list.
2. Open the source mod at `~/Library/.../steamapps/workshop/content/281990/<mod_id>/`.
3. Read the current content of the item the patch targets.
4. Update the patch to match current shape, or mark as obsolete and
   delete.
5. Rebuild; verify the error count drops by however many patches in
   that file you fixed.

## Why they don't block release

`stats.errors` is a distinct gate from `findings`. A stale patch
doesn't fail the lint gate — its item just doesn't emit. The rest of
the build runs correctly. Release builds with these errors produce a
valid `.dist/megapatch/` that works in-game; the fixes-by-mods that
those patches implemented just don't land until the patch is
refreshed.

That said, each error IS an integration feature we intended to ship
— so the fix path matters for playtest quality, even if the build
technically passes.
