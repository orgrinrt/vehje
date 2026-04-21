# Megapatch YAML Patch Format (v2)

> Patches resolve collisions left by the auto-merge engine. Each `.yml`
> file in `megapatch/patches/` is applied after item collection, before assembly.
>
> **v2 Change**: Everything targets ITEMS by ID, not files. An item is a
> top-level Clausewitz entry identified by its key name within a directory.

---

## Patch File Structure

```yaml
# megapatch/patches/scripted_triggers/10_habitable_planet.yml
patches:
  - directory: common/scripted_triggers
    item: habitable_planet
    type: override
    content: |
      habitable_planet = {
          optimize_memory
          OR = {
              habitable_planet_not_urban = yes
              is_urban_planet = yes
              pd_is_planet_class_all = yes
              is_planet_class = pc_hollow
              is_planet_class = pc_apsr_ancient
              is_planet_class = pc_apsr_unique
              is_planet_class = pc_aquatic
              is_planet_class = pc_tidallylocked
          }
      }
    note: "Consolidated: vanilla + GF + PD + APSR + RS + PD AW"
```

### Required Fields

| Field | Description |
|-------|-------------|
| `directory` | Clausewitz directory (e.g., `common/scripted_triggers`) |
| `item` | Entry ID — the top-level key name |
| `type` | Patch type (see below) |
| `note` | Human-readable reason for this patch |

### Optional Fields (type-dependent)

| Field | Used by | Description |
|-------|---------|-------------|
| `mod` | `prefer_mod` | Workshop ID of the mod whose version to use |
| `content` | `override`, `insert_item` | Full Clausewitz content |
| `source_mod` | `inject_field` | Workshop ID to pull the field from |
| `field` | `inject_field` | Field name to inject |
| `find` | `replace_in_item` | Text to find |
| `replace` | `replace_in_item` | Replacement text |
| `name` | `constant`, `constant_ref` | Constant name (e.g., `@tier1cost1`) |
| `value` | `constant` | Literal value |
| `source` | `constant_ref` | Source constant to derive from |
| `multiply` | `constant_ref` | Multiplier for derived value |
| `scope` | `constant` (defines) | Define block scope (e.g., `NGameplay`) |

---

## Patch Types

### 1. `no_override` — Intentionally unpatched

The item was reviewed and needs no patch. Auto-merge or LIOS handles it correctly.
Used to document that an item was reviewed — prevents re-work by future reviewers.

```yaml
  - directory: common/scripted_triggers
    item: allows_slavery
    type: no_override
    note: "A+GF comment/whitespace only — auto-merge handles"
```

### 2. `prefer_mod` — Use a specific mod's version

Select one mod's definition of this item as the winner.

```yaml
  - directory: common/scripted_triggers
    item: is_fallen_empire_machine
    type: prefer_mod
    mod: 3615026960  # Stellaris General Fixes
    note: "GF modernization — is_fallen_empire helper"
```

### 3. `override` — Replace item with hand-crafted content

For consolidation: the merged version that's more complete than any single source.

```yaml
  - directory: common/scripted_triggers
    item: habitable_planet
    type: override
    content: |
      habitable_planet = {
          optimize_memory
          OR = {
              habitable_planet_not_urban = yes
              is_urban_planet = yes
              pd_is_planet_class_all = yes
          }
      }
    note: "Consolidated: vanilla + GF + PD"
```

### 4. `inject_field` — Add/replace a field from another mod's version

Takes the current item and injects a specific field from another mod's definition.
Perfect for Sort Those Buildings (inject `position_priority`) or Stellar AI (inject `ai_weight`).

```yaml
  - directory: common/buildings
    item: building_capital
    type: inject_field
    source_mod: 2798297351  # Sort Those Buildings
    field: "position_priority"
    note: "STB build menu ordering"
```

### 5. `replace_in_item` — Surgical text replacement within an item

```yaml
  - directory: common/scripted_triggers
    item: is_merc_enclave_founding_allowed
    type: replace_in_item
    find: "FLEET_SIZE = 100"
    replace: "FLEET_SIZE = 75"
    note: "Compromise: vanilla 100, A 50"
```

### 6. `delete_item` — Remove an item from output

```yaml
  - directory: common/edicts
    item: gpm_edict_alloy_production
    type: delete_item
    note: "GPM edicts disabled — overpowered, no deficit penalty"
```

### 7. `insert_item` — Add a new item not in any source

```yaml
  - directory: common/on_actions
    item: on_megapatch_dynasty_check
    type: insert_item
    content: |
      on_megapatch_dynasty_check = {
          events = {
              megapatch_bridge.100
          }
      }
    note: "Register megapatch bridge handler"
```

### 8. `constant` — Set a global constant value

For `@variable` definitions and `NDefine` keys.

```yaml
  - type: constant
    name: "@tier1cost1"
    value: "1200"
    note: "PT +20% (vanilla 1000)"
```

For define keys:
```yaml
  - type: constant
    name: "CHANGE_VOTE_COOLDOWN"
    value: "20"
    scope: "NGameplay"
    note: "NGS value"
```

### 9. `constant_ref` — Derive a constant from another constant

Resolved at build time to a literal value.

```yaml
  - type: constant_ref
    name: "@megapatch_faction_slowburn_days"
    source: "@megapatch_faction_slowburn_years"
    multiply: 365
    note: "Derived: years * 365"
```

---

## Resolution Philosophy

**Consolidate, don't choose.** (See TENETS.md)

- `no_override` is for items that are genuinely identical or self-resolving via LIOS.
- `prefer_mod` is for items where one mod is clearly the superset or authoritative source.
- `override` is for items that need content from multiple mods combined.
- `inject_field` is for layered composition — take the base from one mod, inject a field from another.

Patches compose over time — multiple patch files can target the same item.
Applied in filename sort order within a directory.

---

## Resolution Order

When multiple patches target the same item:

1. `override` replaces the item entirely
2. `prefer_mod` selects a specific mod's version
3. `inject_field` inserts fields from secondary sources
4. `replace_in_item` makes surgical text changes
5. `delete_item` removes the item

If both `override` and `prefer_mod` target the same item, `override` wins.

---

## File Organization

```
megapatch/patches/
  constants/                  # Global constants
    tech_costs.yml
    galcom_timers.yml
    faction_slowburn.yml
    
  scripted_triggers/          # Per-directory patch files
    00_no_override_*.yml      # Reviewed, no patch needed
    01_gf_modernization.yml   # Bulk prefer_mod
    10_habitable_planet.yml   # Individual overrides
    11_robot_rights.yml
    ...
    
  scripted_effects/
    ...
  
  game_rules/
    ...
  
  buildings/
    00_no_override.yml
    01_sort_those_buildings.yml  # inject_field batch
    02_gf_base.yml
    ...
```

Numbering convention:
- `00-09` — bulk patterns (no_override, prefer_mod batches)
- `10-29` — individual override patches
- `30-49` — inject_field batches
- `50-69` — replace_in_item surgical patches
- `70-89` — insert_item new content
- `90-99` — delete_item removals
