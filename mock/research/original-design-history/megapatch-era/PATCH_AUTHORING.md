# Patch Authoring Guide

Day-to-day guide for writing and maintaining patches in the item-centric
layout. Complements the design docs:

- `docs/megapatch/design/2026-04-14-item-centric-patches.md` — layout + schema
- `docs/megapatch/design/2026-04-14-tooling.md` — CLI reference
- `docs/megapatch/LINT_CATALOG.md` — every lint rule

## Where patches live

```
megapatch/patches/
├── common/ship_sizes/
│   ├── _declarations.yml              ← bulk no_override / prefer_mod
│   ├── military_station_small.yml     ← full patch chain for one item
│   └── military_station_large.yml
├── common/traits/
│   ├── _declarations.yml
│   ├── trait_psionic.yml
│   └── …
└── events/
    ├── _declarations.yml
    └── heritage_ceremony_42.yml
```

**One rule:** if the only patches on an item are `no_override` or
`prefer_mod`, the item lives in `_declarations.yml`. If there is any
surgery patch (`override`, `replace_in_item`, `inject_field`,
`insert_item`, `delete_item`, `append_content`), the item gets its own
`<item_id>.yml` with the complete patch chain.

## Per-item file schema

```yaml
# <directory>/<item_id>.yml
directory: common/ship_sizes
item: military_station_small

# Optional file-level defaults — inherited by every entry below
# unless the entry overrides.
feature: station_rebalance
status: reviewed

patches:
  - type: override
    source_mods: [1995601384, 683230077, 1199002146]
    rationale: |
      NSC3 structural base + Theta defense scaling + RAA citadel.
      Theta's fleet_slot_size=2 matches vanilla; NSC3's =1 looks like a typo.
    content: |
      military_station_small = {
          max_speed = 6
          …
      }

  - type: replace_in_item
    source_mods: [3611633]
    status: provisional        # overrides file-level default
    find: "max_speed = 6"
    replace: "max_speed = 8"
    rationale: "Tier 2 tuning — revisit after playtest"
```

### Required fields per type

| type              | required                  | optional                                          |
|-------------------|---------------------------|---------------------------------------------------|
| `override`        | `content`                 | `source_mods`, `status`, `feature`, `rationale`, `depends_on` |
| `insert_item`     | `content`                 | as above                                          |
| `append_content`  | `content`                 | as above                                          |
| `replace_in_item` | `find`, `replace`         | as above                                          |
| `inject_field`    | `field`, `content`        | as above + `after` / `before` for position        |
| `delete_item`     | (nothing beyond `type`)   | `rationale` strongly encouraged                   |
| `no_override`     | — (see declarations)      | —                                                 |
| `prefer_mod`      | `mod` (the winner)        | —                                                 |

### `status` enum

- `reviewed` — signed-off; meets the project's conflict-resolution rules
- `provisional` — works but needs another pass
- `to-revisit` — known-suboptimal; revisit after broader system settles
- `wip` — in progress, do not assume complete

Typos are caught by `PATCH_UNKNOWN_STATUS` (ERROR).

### `feature`

Cross-cutting tag. Free-form string, lowercase, underscores. `megapatch
feature <name>` enumerates every patch tagged with a given feature.
`megapatch feature list` enumerates every tag in the corpus.

Not load-bearing for the compiler — it's an index for developers.

### `source_mods`

List of workshop IDs whose versions of this item were considered when
writing the patch. Useful for "what breaks if this mod updates?" audits
via `megapatch touches <mod_id>` (planned P2 command).

### `rationale`

Multi-line string explaining WHY the patch exists. Required on surgery
patches (`PATCH_MISSING_RATIONALE_ON_SURGERY` WARNING). Write for the
maintainer who has to audit this patch in six months.

### `depends_on`

Patch slug the compiler must apply before this one:

```yaml
- type: inject_field
  depends_on: common/buildings/building_bureaucratic_1#override_1
  field: ai_weight
  content: |
    ai_weight = { factor = 5 }
```

Slug format: `<directory>/<item_id>#<type>_<seq>` where `seq` is
1-indexed per type in the target file. Unresolvable slugs fail
`PATCH_DEPENDS_ON_BROKEN` (ERROR).

## Declarations file schema

```yaml
# <directory>/_declarations.yml
directory: common/scripted_triggers

no_override:
  - item: is_slavery_enabled
  - item: can_use_psionics
    rationale: "Intentional LIOS across 3-mod stack"
  # … dozens per directory typical

prefer_mod:
  - item: can_declare_war
    mod: 1995601384
    source_mods: [1995601384, 683230077]
    rationale: "Ariphaos is authoritative here"
```

`directory:` is declared once at file top and inherited by every entry.
Type is implicit from the section (`no_override:` vs `prefer_mod:`).

## Common workflows

### Add a new patch

```bash
megapatch new-patch common/traits/trait_psionic \
    --type override \
    --feature psionic_overhaul \
    --source-mods 1995601384,2798297351 \
    --rationale "consolidate pan-dim psionic branches" \
    --edit
```

Creates or appends into `megapatch/patches/common/traits/trait_psionic.yml`
with TODO placeholders for `content:`. `--edit` opens `$EDITOR` on the
result.

Declarative patches route automatically to `_declarations.yml`:

```bash
megapatch new-patch common/scripted_triggers/is_xeno_compat \
    --type no_override \
    --rationale "LIOS OK, 3-mod stack agrees"
```

### Debug what a patch actually does

```bash
megapatch trace common/buildings/building_bureaucratic_1
```

Shows every patch applied to the item in apply order with unified diffs
between compile steps. Add `--show full` for post-patch content or
`--show last` for just the final compiled output.

### Audit a cross-cutting feature

```bash
megapatch feature psionic_overhaul              # group by item
megapatch feature psionic_overhaul --show tree  # collapsed by top-level dir
megapatch feature psionic_overhaul --status-filter wip
megapatch feature list                          # every feature + counts
```

### Add a bulk declarative sweep

Edit `<dir>/_declarations.yml` directly — appending a list entry is
cheap. `megapatch new-patch --type no_override <dir>/<item>` also works
for single-item additions.

## Linters that will fire

The full catalog is in `docs/megapatch/LINT_CATALOG.md`. Schema-specific
rules that new patches commonly trip:

| rule                                       | severity | fix                                    |
|--------------------------------------------|----------|----------------------------------------|
| `PATCH_UNKNOWN_STATUS`                     | ERROR    | use one of the 4 canonical statuses    |
| `PATCH_MISSING_RATIONALE_ON_SURGERY`       | WARNING  | add `rationale:`                        |
| `PATCH_DECLARATIONS_INVALID_TYPE`          | ERROR    | move surgery out of `_declarations.yml` |
| `PATCH_DECLARATIONS_ITEM_HAS_SURGERY`      | ERROR    | item belongs in one file only          |
| `PATCH_DEPENDS_ON_BROKEN`                  | ERROR    | fix the slug or rename the dep target  |
| `PREFER_MOD_MISSING_MOD`                   | ERROR    | add `mod: <winner_id>`                 |
| `OVERRIDE_MISSING_CONTENT`                 | ERROR    | either add content or switch to `delete_item` |

Release builds block on first-party findings of any severity; dev
builds report but continue.

## Conflict-resolution discipline

Inherited from `.claude/rules/megapatch-conflict-resolution.md`:

- **Consolidate, don't choose.** Take the best of every source; don't
  just pick one mod.
- **Per-entry review.** Never sweep "prefer_mod X for all items in
  this dir" without per-item rationale.
- **Value disagreements** get a provisional value with `# TODO:`
  comment referencing both originals.
- **Formatting-only differences** are the only case where
  `prefer_mod` without per-entry review is acceptable.

## Patch IDs

Slugs are derived, not written. `<directory>/<item_id>#<type>_<seq>`
where seq is 1-indexed per type in the patch file. You'll see them in:

- `trace` output
- `depends_on:` references
- lint findings
- commit messages when useful

Don't try to write the slug into a YAML file — it isn't a field;
authoring tools infer it.

## When things go wrong

- **Your patch doesn't fire** — check `trace` for the item. Most
  likely: another higher-priority patch ran first. Priority order:
  `override > replace_in_item > inject_field > prefer_mod > delete_item
  > insert_item > no_override`.
- **`PATCH_UNKNOWN_DIRECTORY`** — the patch targets a directory not
  declared in `data/rules-fios-lios.txt`. Add the rule.
- **Round-trip mismatch after `migrate-patches`** — shouldn't happen;
  if it does, read the mismatch report. Known causes: case-insensitive
  FS collisions, tabs inside content that PyYAML can't block-scalar.
  See the migration module's docstring for the fixes.

## Adding a new lint

See `docs/megapatch/LINT_CATALOG.md` § "Adding a new rule".
