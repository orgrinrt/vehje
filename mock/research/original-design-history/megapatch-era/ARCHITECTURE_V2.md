# Megapatch Build Pipeline — Architecture v2

> Item-based pipeline. Everything targets items, not files.

## Core Principle

The Clausewitz engine resolves conflicts at the **entry ID level**, not the file level. A `scripted_trigger` called `habitable_planet` can be defined in `00_scripted_triggers.txt` by mod A, in `pd_triggers.txt` by PD, and in `gf_triggers.txt` by GF. The engine doesn't care about filenames — it cares about which definition of `habitable_planet` loads last (LIOS) or first (FIOS).

Our pipeline mirrors this: **items in, items out**.

---

## Pipeline Overview

```
   EXTRACT          COLLECT         PATCH          ASSEMBLE        PACKAGE
  ─────────►      ──────────►    ──────────►    ───────────►    ───────────►
  
  vanilla/     ┐                                  megapatch/
  mod_1/       │   items.db     patched.db        output/
  mod_2/       ├─► (all items   (items with       ├─ megapatch/        (LIOS mod)
  mod_3/       │    by ID)       patches           │  └─ common/
  ...          │                 applied)          │     └─ <dir>/
  mod_N/       ┘                                   │        └─ zz_megapatch_<dir>.txt
                                                   └─ megapatch_fios/   (FIOS mod)
                    constants.db  patched           └─ common/
                    (all @vars     constants           └─ <dir>/
                     + defines)                           └─ 00_megapatch_<dir>.txt
```

### Phase 1: EXTRACT — Item-Level Content Database

For every mod in load order + vanilla:

1. **Clausewitz entries**: Extract all top-level `key = { ... }` blocks using `clausewitz-extract.sh --full`. Store each as:
   ```
   {
     id: "habitable_planet",
     directory: "common/scripted_triggers",
     source_mod: 3615026960,
     source_file: "00_scripted_triggers.txt",
     content: "habitable_planet = { ... }",
     line_count: 12
   }
   ```

2. **Global constants**: Extract all `@variable = value` definitions and `NDefine` blocks into a separate constants database. Every `@foo = 123` from every file, every mod. Store as:
   ```
   {
     name: "@tier1cost1",
     value: "1000",
     source_mod: 0,        # 0 = vanilla
     source_file: "00_scripted_variables.txt",
     directory: "common/scripted_variables"
   }
   ```

3. **Load order tracking**: Record which mod defines each item, in load order. For LIOS dirs, the last definition wins by default. For FIOS dirs, the first wins.

Output: `megapatch/generated/items.db` (TSV or JSONL — one line per item definition)
Output: `megapatch/generated/constants.db` (TSV — one line per @variable/define)

### Phase 2: COLLECT — Conflict Resolution

For each unique item ID:
1. If only ONE mod defines it → no conflict, pass through
2. If multiple mods define it → mark as conflict, record all sources
3. Apply default resolution:
   - **LIOS directories**: last-in-load-order definition wins
   - **FIOS directories**: first-in-load-order definition wins
4. Record the "provisional winner" — this is the naive auto-merge result

For constants:
1. Collect ALL definitions across all mods
2. Last-in-load-order wins per key (Clausewitz convention)
3. Output a single unified `megapatch_constants.txt` with every constant

Output: `megapatch/generated/collected/` — one file per directory with all items (provisional winners)
Output: `megapatch/generated/constants_collected.txt` — unified constants file

### Phase 3: PATCH — Apply Decisions

Patches target **items by ID**, not files. Each `.yml` patch specifies:
```yaml
directory: common/scripted_triggers
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

Constant patches:
```yaml
type: constant
name: "@tier1cost1"
value: "1200"
note: "PT values (+20% tech cost for marathon pacing)"
```

Value delegation (resolved at build time):
```yaml
type: constant
name: "@megapatch_faction_slowburn_years"
value: "10"
note: "Central tunable for faction slowburn system"

type: constant
name: "@some_derived_value"
value_from: "@megapatch_faction_slowburn_years"
multiply: 365
note: "Slowburn in days — derived from years at build time"
```

Patch types (updated from v1):

| Type | Target | Description |
|------|--------|-------------|
| `prefer_mod` | item ID | Use specific mod's version of this item |
| `override` | item ID | Replace item content entirely with provided content |
| `inject_field` | item ID + field name | Add/replace a field in the item from another mod's version |
| `replace_in_item` | item ID + find/replace | Surgical text replacement within the item |
| `delete_item` | item ID | Remove item from output entirely |
| `insert_item` | item ID | Add a new item (not in any source) |
| `constant` | @name | Set a global constant value |
| `constant_ref` | @name + source | Delegate constant to another constant's value (with optional math) |

### Phase 4: ASSEMBLE — Generate Output Files

After all patches are applied, assemble items into output files:

1. **LIOS directories** → all items for a directory go into `zz_megapatch_<dirname>.txt` in the `megapatch/` mod. The `zz_` prefix ensures LIOS makes our version authoritative.

2. **FIOS directories** → all items go into `00_megapatch_<dirname>.txt` in the `megapatch_fios/` mod. The `00_` prefix ensures FIOS makes our version authoritative.

3. **On_actions** → special merge: UNION of all event lists from all mods. Output as `zz_megapatch_on_actions.txt`.

4. **Constants** → all constants go into `megapatch_scripted_variables.txt` (or split per category if needed for FIOS/LIOS).

5. **Items that need NO override** (only one mod defines them, no conflict) → NOT included in output. The original mod's file handles it.

6. **Items from the auto-merge that our patch didn't touch** → still included if they're in a file that HAS patched items (because LIOS replaces the entire file, we must include ALL items for that directory, not just patched ones).

### Phase 5: PACKAGE — Mod Structure

```
.dist/
  megapatch/                          # LIOS mod — loads LAST in mod order
    descriptor.mod
    common/
      scripted_triggers/
        zz_megapatch_scripted_triggers.txt    # ALL scripted trigger items
      scripted_effects/
        zz_megapatch_scripted_effects.txt
      game_rules/
        zz_megapatch_game_rules.txt
      buildings/
        zz_megapatch_buildings.txt
      ... (one file per directory)
    events/
      ... (FIOS — handled by megapatch_fios instead)
  
  megapatch_fios/                     # FIOS mod — loads FIRST in mod order
    descriptor.mod
    common/
      component_templates/
        00_megapatch_components.txt
    events/
      00_megapatch_events.txt
```

---

## Constants System

### Problem
Constants (`@variable = value`) are scattered across hundreds of files from dozens of mods. Multiple mods define the same constant with different values. Currently you have to hunt through files to find what `@tier1cost1` is set to.

### Solution: Constants Collector

A new tool: `megapatch-collect-constants.sh`

1. Scans ALL mod files for `@variable = value` definitions (top-level, not inside blocks)
2. Also scans `NDefine` blocks in defines files
3. Outputs a single `constants_collected.txt`:
   ```
   # Auto-generated — all constant definitions across stack
   # Format: @name = value  # source_mod (mod_name) > source_file
   
   @tier1cost1 = 1000          # vanilla > 00_scripted_variables.txt
   @tier1cost1 = 1200          # 1311725711 (Plentiful Traditions) > 00_scripted_variables.txt
   @tier1cost1 = 5000          # 1333526620 (PT Extra Perks) > zz_ascension_perks.txt
   @tier1cost1 = 500           # 1715190550 (Forgotten Queens) > x_scripted_variables.txt
   # WINNER (LIOS): 1200 from Plentiful Traditions (last in load order)
   # PATCHED: 1200 (PT values — +20% tech cost for marathon pacing)
   ```

4. Patch file for constants:
   ```yaml
   # megapatch/patches/constants/tech_costs.yml
   patches:
     - type: constant
       name: "@tier1cost1"
       value: "1200"
       note: "PT values — +20% tech cost"
     - type: constant
       name: "@tier1cost2"
       value: "1800"
     # ... all tier costs
   ```

5. Build output: `megapatch_scripted_variables.txt` with ALL constants, our chosen values, commented with source.

### Value Delegation

```yaml
# megapatch/patches/constants/faction_slowburn.yml
patches:
  - type: constant
    name: "@megapatch_faction_slowburn_years"
    value: "10"
    note: "Central tunable"
  
  - type: constant_ref
    name: "@megapatch_faction_slowburn_days"
    source: "@megapatch_faction_slowburn_years"
    multiply: 365
    note: "Derived — years * 365"
```

At build time, `@megapatch_faction_slowburn_days` becomes `3650` in the output. Clausewitz requires literal values — no runtime references. But our PATCH files can use references for maintainability.

---

## Patch File Organization

```
megapatch/patches/
  constants/
    tech_costs.yml
    faction_slowburn.yml
    district_costs.yml
    ship_sizes.yml
    galcom_timers.yml
  
  scripted_triggers/
    habitable_planet.yml           # one patch per item (or group)
    robot_rights_family.yml        # related items can share a file
    ship_component_triggers.yml
    gpm_planet_classes.yml
    gpm_planet_types.yml
    ...
  
  scripted_effects/
    generate_start_pops.yml
    pirate_fleet_effects.yml       # batch of 36 pirate entries
    ...
  
  game_rules/
    species_can_live_on_planet.yml
    ...
  
  buildings/
    sort_those_buildings.yml       # inject_field for all STB entries
    pt_tradition_modifiers.yml     # inject_field for all PT entries
    ...
  
  pop_faction_types/
    traditionalist.yml
    imperialist.yml
    ...
```

Each `.yml` file contains one or more patches for related items. Patches compose — multiple files can patch the same item (applied in filename sort order).

---

## Updated Patch Format (v2)

```yaml
# megapatch/patches/scripted_triggers/habitable_planet.yml
---
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

  - directory: common/scripted_triggers
    item: habitable_planet_not_urban
    type: prefer_mod
    mod: 3615026960
    note: "GF version — vanilla-identical logic, condensed format"
```

Key changes from v1:
- `target` (file path) → `directory` + `item` (item-based targeting)
- `entry` → `item` (clearer naming)
- `replace_in_entry` → `replace_in_item`
- New: `constant` and `constant_ref` types
- New: `insert_item` for adding entirely new items
- Patches can target items regardless of which source file defines them

---

## FIOS Handling

FIOS directories (events, component_templates, global_ship_designs) need the FIRST-loaded definition to win. Our pipeline handles this by:

1. **Extract phase**: marks each item with its directory's FIOS/LIOS classification
2. **Collect phase**: for FIOS dirs, the first-in-load-order definition is the provisional winner (opposite of LIOS)
3. **Assemble phase**: FIOS items go into `megapatch_fios/` mod with `00_` prefix filenames
4. **Load order**: `megapatch_fios` loads FIRST in the mod list (before all other mods). Its `00_` prefixed files ensure it wins in FIOS resolution.

For items in FIOS directories that we DON'T need to override, we don't include them — the original mod's version (whichever loads first) is fine.

---

## Migration from v1

The existing decision documents (4,134 entries across 20 directories) are the intellectual foundation. The migration is:

1. Each "NO OVERRIDE NEEDED" decision → no patch file needed
2. Each "GF VERSION" / "mod X VERSION" → `prefer_mod` patch
3. Each "CONSOLIDATE" → `override` patch with hand-crafted content
4. Each "VALUE CHOICE" → `constant` patch
5. Bulk patterns (422 comment-only, etc.) → batch `prefer_mod` patches

The decision docs don't change — they document WHY. The patch files express HOW.
