# Autopatch — Self-Generating Patches from Content Scans

## Problem

Some triggers/effects need to list ALL instances of a category across ALL mods
(e.g., "every district that produces energy"). These lists go stale when mods
update. Maintaining them manually is error-prone and tedious.

## Solution

An `autopatch` is a patch type that regenerates its content from tooling output
every build cycle. The patch definition says WHAT to scan for. The build system
produces the actual Clausewitz code.

## Format

```yaml
target: common/scripted_triggers/megapatch_energy_infrastructure.txt
type: autopatch
note: "All primary energy-producing districts, zones, and buildings from all stack mods"

generator:
  # What to scan
  scan: districts,zones,buildings
  
  # How to filter (applied to content-scan output)
  filter: "primary_resource = energy"
  
  # Template for output (handlebars-style)
  template: |
    has_any_generator_district_or_building = {
        OR = {
            {{#each districts}}
            has_district = {{id}}
            {{/each}}
            {{#each zones}}
            has_zone = { zone = {{id}} }
            {{/each}}
            {{#each buildings}}
            has_building = {{id}}
            {{/each}}
        }
    }
```

## Build Integration

```
megapatch-build.sh:
  1. discover
  2. merge
  3. patch (manual YAML patches)
  4. autopatch ← NEW: run content scans, generate autopatch outputs
  5. validate
  6. package
```

The autopatch phase:
1. Reads all `type: autopatch` patch definitions
2. Runs the specified scan (uses `megapatch-scan-content.sh`)
3. Applies the filter to scan results
4. Fills the template with filtered results
5. Writes the output file

## First Candidates

### has_any_generator_district_or_building
Scan all districts/zones/buildings → filter primary energy producers → list them.

### has_any_farming_district_or_building
Same pattern → filter primary food producers.

### has_any_mining_district_or_building
Same pattern → filter primary mineral producers.

### habitable_planet (partial)
Scan all planet_classes → filter `colonizable = yes` → add non-vanilla planet classes.
(The `pd_is_planet_class_all` trigger handles PD, but other mods' planets need listing.)

### habitable_planet_not_urban
Scan all planet_classes → filter `colonizable = yes` AND NOT urban/relic/city/hive/machine/structure → list non-vanilla habitable planet classes.

## Why Not Just Use Scripted Triggers From Each Mod?

Mods define their OWN content but don't update VANILLA triggers to include it.
PD defines `pd_is_planet_class_all` for its own use, but vanilla's `habitable_planet`
doesn't call it. The autopatch bridges this gap automatically.

## Future Extensions

- `scan: civics` with `filter: "is_homicidal = true"` → auto-generate the
  `is_homicidal` belt-and-suspenders civic list
- `scan: star_classes` with `filter: "dangerous = true"` → auto-generate
  `system_blocks_sensors` star class list
- `scan: traits` with `filter: "category = leader"` → auto-generate trait
  compatibility lists for Heritage
