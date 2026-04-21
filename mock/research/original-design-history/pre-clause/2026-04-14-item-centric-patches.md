# Item-Centric Patch Organization — Design

Date: 2026-04-14
Status: proposed
Supersedes: the topic-file organization under `megapatch/patches/<topic>/*.yml`.
Feeds: migration + tooling implementation tasks (created as follow-ups).

Input: `docs/megapatch/sessions/2026-04-14-patch-redesign-pain-points.md`.

Scope: reorganize the YAML patch tree around items, add structured metadata,
compress bulk declarations. **YAML stays** — a parallel grammar project (#263)
was considered and dropped as low-ROI once organization is fixed.

---

## Headline

Two-tier layout per patch tree:

```
megapatch/patches/
├── common/ship_sizes/
│   ├── _declarations.yml         ← bulk no_override / prefer_mod (~40 entries)
│   ├── military_station_small.yml ← full patch chain for one item
│   └── military_station_large.yml
├── common/traits/
│   ├── _declarations.yml
│   ├── trait_psionic.yml
│   └── ...
└── events/
    ├── _declarations.yml
    └── heritage_ceremony_42.yml
```

Rule: **if an item only has declarative patches (`no_override` or
`prefer_mod`), it lives as one line in `_declarations.yml`. If it has any
real surgery (override, replace_in_item, inject_field, insert_item,
delete_item, append_content), it gets its own `<item_id>.yml` with the full
patch chain.**

Expected layout scale (from current corpus):
- 240 directories → 240 `_declarations.yml` files (avg ~38 entries each)
- 9246 purely-declarative items → collapsed into the declarations tables
- 4292 surgery items → 4292 per-item files

Compared to today's 455 topic-files with tangled contents, this is both more
files (4500 vs 455) AND fewer per-file lines (small files average ~30–80
lines vs today's 891-line monsters). The right number of files is the
number of items, not the number of feature topics.

---

## Decisions

### Q1: Cross-cutting concerns → **tag field + CLI, no separate authoring dir**

Each patch entry gets an optional `feature: <slug>` field. Tooling
(`megapatch feature <name>`) lists every patch tagged with that feature.
No separate `features/` directory; the authoritative source is still the
per-item files. The tag is an index, not a coordination point.

**Rejected alternative**: features/ dir that "contains" patches and
materializes into per-item files. Double-source-of-truth risk + rebuild
step; not worth the friction for our scale.

If multiple tags per patch become necessary later, `feature: X` grows to
`features: [X, Y]`. Start with single-tag for simplicity.

### Q2: Bulk declarations → **tiered layout with `_declarations.yml` per directory**

68% of patches are `no_override` / `prefer_mod`. Exploding them into ~9246
single-patch files would inflate `git status` noise and churn on every mod
audit. Compress into:

```yaml
# common/ship_sizes/_declarations.yml
directory: common/ship_sizes

no_override:
  - item: is_galactic_market
  - item: can_build_mining_station
    rationale: "Vanilla behavior preserved across 4-mod stack"

prefer_mod:
  - item: science_ship_hangar
    mod: 1995601384
    rationale: "Ariphaos has the most complete hangar module definition"
  - item: military_station_medium
    mod: 683230077
    source_mods: [1995601384, 683230077, 1199002146]
```

Schema differences vs per-item files:
- `type` is implicit (the section it's under)
- Entry is a single mapping with minimal fields
- `directory` declared once at top, inherited by every entry

The `_` prefix keeps the file alphabetically first in dir listings and
visually distinguishes it from per-item files.

Per-item file still wins if the item has ANY surgery. Example: if
`military_station_small` has both a `prefer_mod` AND an `override`, it goes
entirely into `military_station_small.yml` — never split across files.

### Q3: Structured metadata → **typed fields, replace free-form `note:`**

New per-entry schema:

```yaml
# common/ship_sizes/military_station_small.yml
directory: common/ship_sizes
item: military_station_small

patches:
  - type: override
    source_mods: [1995601384, 683230077, 1199002146, 3011634662]
    status: reviewed           # reviewed | provisional | to-revisit | wip
    feature: station_rebalance # optional cross-cutting tag
    rationale: |
      NSC3 structural base + Theta defense scaling + RAA citadel.
      Theta's fleet_slot_size=2 matches vanilla; NSC3's =1 looks like a typo.
    content: |
      military_station_small = {
          max_speed = 6
          ...
      }

  - type: replace_in_item
    source_mods: [3611633]
    status: provisional
    find: "max_speed = 6"
    replace: "max_speed = 8"
    rationale: "Tier 2 tuning — revisit after playtest"
```

Required fields per patch type:
| type | required | optional |
|---|---|---|
| `override` / `insert_item` / `append_content` | `content` | `source_mods`, `status`, `feature`, `rationale`, `depends_on` |
| `replace_in_item` | `find`, `replace` | as above |
| `inject_field` | `field`, `content` | as above + `after`/`before` for positional injection |
| `delete_item` | (nothing beyond `type`) | `rationale` strongly encouraged |
| `no_override` / `prefer_mod` | (see declarations schema Q2) | — |

Deprecated: the old free-form single-line `note:` field. Migration maps:
- `note: "CONSOLIDATED ..."` → `rationale: "CONSOLIDATED ..."`
- `mods_considered: [...]` → `source_mods: [...]`
- multi-line notes that also pack status signals ("REVIEWED: ...") → split
  into `status: reviewed` + `rationale: ...`.

Enum for `status` is strict (validated by PatchLinter). Free-form `rationale`
is multi-line-friendly.

### Q4: Apply order → **lexical default + optional `depends_on` for cross-file deps**

Within one item file, patches apply top-to-bottom in list order. The
developer reading the file sees exactly the order. No `order:` integer
field — adding patches in the middle just means inserting a YAML list
entry in the right place.

Across files (rare — almost all dependencies are same-item), introduce
`depends_on: <patch_id>`:

```yaml
patches:
  - type: inject_field
    field: ai_weight
    depends_on: common/buildings/building_bureaucratic_1#override_1
    content: |
      ai_weight = { factor = 5 }
```

Patch IDs (Q8 below): slug-form. `#` separator makes them easy to grep.
Only emit IDs when something actually depends on a patch — auto-derived
lazily, not in every file header.

### Q5: Insert_item location → **file named after the created item, in the target dir**

If we're defining a new `my_heritage_trigger` in `common/scripted_triggers/`,
the file is `common/scripted_triggers/my_heritage_trigger.yml` with a
single `type: insert_item` patch entry. Same pattern as items we're
modifying — the file name tracks the compiled-output symbol.

### Q6: Per-mod patch trees → **keep existing separation**

`megapatch/patches/`, `megapatch/heritage/patches/`,
`megapatch/heritage_late/patches/` are the first-party mods. Each gets its
own item-centric tree. If both heritage and megapatch want to patch
`common/traits/trait_psionic`, they each have their own
`common/traits/trait_psionic.yml` in their own tree. The canonical load
order determines which mod's patches apply when.

No `mod:` field inside per-item files to disambiguate ownership — the tree
root encodes it.

### Q7: Migration → **one-shot tool, atomic commit**

Build `megapatch migrate-patches` subcommand that:
1. Reads `patches` table (already authoritative — DB-native since #241).
2. Groups by `(directory, item_id)`.
3. For each group:
   - If ALL patches are `no_override` / `prefer_mod`, emit into
     `_declarations.yml` of that directory.
   - Otherwise emit `<item_id>.yml` with full patch list.
4. Collapse `mods_considered` → `source_mods`, `note` → `rationale`.
5. Attempt to detect status signals in old notes (`REVIEWED:`, `TODO:`,
   `PROVISIONAL:`) and populate `status:`; leave unset if ambiguous.
6. Attempt to detect feature tags from the old source_file name
   (`mesh/105_political_system_deep_integration.yml` →
   `feature: political_integration`). Best-effort; leave unset if unclear.

Round-trip gate: after migration, run `parse-patches` against the new
tree, compare DB rows to a snapshot of the old DB. Must be byte-identical
in `(item_id, directory, type, mod, content, find, replace, field)`. The
legacy `note` and `mods_considered` columns are migrated to new fields
and may legitimately differ; all other columns must match.

Atomic deployment:
- Migration tool writes to `megapatch/patches_v2/` initially.
- Once round-trip passes, commit.
- In a follow-up commit: delete old `megapatch/patches/`, rename
  `patches_v2/` → `patches/`, update the compiler's `patch_dir` if needed
  (it shouldn't — same path).
- Rollback: revert both commits.

### Q8: Patch IDs → **derived slug, lazy**

Patches get a stable slug derived from `(directory, item_id, type, seq)`
where seq is the ordinal of that patch in its item file (1-based per type).
Example: `common/ship_sizes/military_station_small#override_1`.

The slug is NEVER written into the YAML. It's derived on demand by
tooling. The sequence resets per type so re-ordering within a file keeps
IDs stable unless you insert a same-type patch earlier.

`depends_on` references use this slug. If a file reorders and breaks a
reference, migration tooling surfaces the break — we lose no correctness,
just an error message.

Not doing: content-hash-based IDs. They're stable across reorderings but
unreadable in commit messages and hard to type.

---

## The final file schemas (canonical)

### Per-item file

```yaml
# <directory>/<item_id>.yml
directory: common/ship_sizes
item: military_station_small

# Optional top-level metadata. Applies to every patch below unless
# overridden on the individual entry.
feature: station_rebalance          # default feature tag for all entries
status: reviewed                    # default status

patches:
  - type: override
    source_mods: [1995601384, 683230077]
    rationale: |
      Multi-paragraph rationale.
      Multiple lines allowed.
    content: |
      military_station_small = {
          max_speed = 6
          ...
      }

  - type: replace_in_item
    source_mods: [3011634662]
    status: provisional              # overrides file-level default
    find: |
      max_speed = 6
    replace: |
      max_speed = 8
    rationale: "Per playtest feedback"
```

### Declarations file

```yaml
# <directory>/_declarations.yml
directory: common/scripted_triggers

no_override:
  - item: is_slavery_enabled
  - item: is_machine_empire
  - item: can_use_psionics
    rationale: "Intentional LIOS across 3-mod stack"
  # ... dozens more per directory

prefer_mod:
  - item: can_declare_war
    mod: 1995601384
    source_mods: [1995601384, 683230077]
    rationale: "Ariphaos is authoritative here"
  - item: is_xeno_compatibility_enabled
    mod: 683230077
```

### Top-level doc convention

Every item file starts with a comment line naming the item (helpful when
greppping file contents). The YAML headers (`directory:`, `item:`) are
load-bearing — the parser uses them, so redundancy with the file path is
intentional (catches mispaths).

---

## What this fixes (vs the pain points)

| Pain | Fix |
|---|---|
| 1. File boundaries don't match item boundaries | Per-item file — one file, one item, one chain. |
| 2. YAML + Clausewitz is two grammars | Unchanged; YAML block scalars still contain Clausewitz. But each block is small (1 item at a time), so the envelope overhead is tolerable. |
| 3. Apply order is invisible | Top-to-bottom in the list is explicit. `depends_on` for cross-file cases. |
| 4. Big monolithic files (891-patch `gfx_ships`) | Declarations go into `_declarations.yml`; surgery spreads across per-item files. No single file gets huge. |
| 5. Cross-cutting concerns are invisible | `feature:` tag + `megapatch feature <name>`. |
| 6. `note:` is underpowered | Replaced by `rationale`, `status`, `source_mods`, `feature`. |
| 7. `replace_in_item` is brittle | Unchanged — AST-level patches deferred as research. The per-item layout makes brittle replaces easier to find (they're in one file) but doesn't eliminate brittleness. |
| 8. No stable patch IDs | Derived slugs on demand. |

Pain 2 and 7 are accepted-as-is; the rest are resolved. Both deferrals
are revisitable if the new layout exposes them as more painful than
expected.

---

## Tooling required

Identified in #264. Mandatory for #262 to be usable:

1. **`megapatch migrate-patches`** — one-shot migration with round-trip
   verification. Without this we can't ship the reorg.
2. **`megapatch trace <dir>/<item_id>`** — show every patch touching an
   item with cumulative state after each step. Developer's daily tool.
3. **`megapatch new-patch <dir>/<item_id> [--type T]`** — scaffold a new
   patch entry into the right file, creating it if missing.

Nice-to-have:

4. **`megapatch feature <name>`** — list every patch tagged with a feature.
5. **`megapatch fmt`** — canonical field order + indentation.
6. LSP/editor support — deferred.

---

## Implementation tasks (to create after review)

Keep ports small and independently committable:

1. **New YAML schema + parser extension** in `ingest/parse_patches.py`:
   read the new per-item and declarations layout, produce the same
   `patches` table rows. Support BOTH the old and new layout briefly
   during migration window.
2. **Migration tool**: `megapatch migrate-patches [--dry-run] [--out DIR]`.
   Reads DB, writes new layout, round-trip verifies.
3. **PatchLinter updates**: new lints for schema violations
   (unknown `status` value, `rationale` missing on surgery patches,
   `feature` value not in a known-features registry).
4. **CLI tooling**: `trace`, `new-patch`, `feature`.
5. **Documentation**: schema reference + authoring guide in
   `docs/megapatch/PATCH_AUTHORING.md`.
6. **Atomic cut-over**: run migration, commit output, delete old tree,
   rename.

---

## Open questions deferred

- **AST-level patches** (Pain 7): genuinely research-heavy. Mod source is
  heterogeneous; parsing every mod's Clausewitz AST to allow "set field
  `max_speed` to 8" declaratively requires a robust Clausewitz AST that we
  don't have. Revisit after reorg proves the grammar pain is still acute.
- **Localisation patches**: we don't patch localisation today, so the
  schema doesn't cover it. If we ever do, add a `type: localisation_patch`
  with `(key, language, value)` entries.
- **Artwork patches**: artwork is produced by `process-artwork.sh` (now a
  Python pipeline). Out of scope for patch reorganization.
