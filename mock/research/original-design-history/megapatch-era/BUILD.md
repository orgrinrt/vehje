# Megapatch Build System

> The megapatch is a build artifact, not a hand-maintained file collection.
> Tooling reads all mods in the active modlist, analyzes them, and generates
> the merged output. Hand-editing the generated files is a last resort.
>
> The existing "Universal Patches" on the Workshop are hand-maintained merge
> files by individual modders. They don't touch other mods' source files —
> they provide curated override files. We do the same thing, but automated
> and exhaustive for our specific modlist.

---

## Architecture

```
                    .cache/modlist/mods/
                           │
                    ┌──────┴──────┐
                    │  All 458    │
                    │  mod dirs   │
                    └──────┬──────┘
                           │
                    ┌──────┴──────┐
                    │  ANALYSIS   │  tools/megapatch-analyze.sh
                    │  (read all  │  → conflict map
                    │   mod files)│  → override map
                    └──────┬──────┘  → ID collision report
                           │         → missing reference report
                    ┌──────┴──────┐
                    │  GENERATION │  tools/megapatch-generate.sh
                    │  (produce   │  → merged on_actions
                    │   merged    │  → merged edicts
                    │   files)    │  → merged civics
                    └──────┬──────┘  → merged traits
                           │         → merged game_rules
                    ┌──────┴──────┐  → bridge events
                    │  VALIDATION │  tools/megapatch-validate.sh
                    │  (check     │  → schema compliance
                    │   output)   │  → reference integrity
                    └──────┬──────┘  → ID uniqueness
                           │
                    ┌──────┴──────┐
                    │  megapatch/ │  The generated mod
                    │  common/   │  (committed to VCS as artifact,
                    │  events/   │   source of truth is the tooling)
                    │  ...       │
                    └─────────────┘
```

---

## What the Analyzer Must Detect

### Per-directory analysis

For each `common/` subdirectory, the analyzer must:

1. **Inventory**: which mods contribute files to this directory
2. **Overlap**: which mods write to the SAME filename (last-loaded wins in vanilla)
3. **ID collision**: which mods define entries with the SAME ID (within a file or across files)
4. **Cross-reference**: which entries reference IDs from other mods (dependencies)
5. **Missing references**: which entries reference IDs that don't exist anywhere
6. **Vanilla override**: which mods replace vanilla files vs add new files

### Specific analyzers needed

**148 unique common/ subdirectories found in our 458-mod stack.** The analyzer must
cover ALL of them, not just the commonly discussed ~40. Grouped by analysis type:

#### FIOS directories (First In Only Served — first mod to define an ID wins)
These CANNOT be overridden by a last-loaded megapatch. Conflicts require file
replacement in the conflicting mod, or merging into the FIRST-loaded mod's file.

| Directory | Files | Analysis | Key Questions |
|---|---|---|---|
| `component_templates` | 236 | ID + FIOS | Which mod's component loads first? Do we want that version? |
| `global_ship_designs` | 79 | ID + FIOS | AI designs — first-defined wins. Are our templates loading first? |
| `script_values` | 77 | ID + FIOS | Script value definitions — first wins |
| `events/` (not common/) | 1088 | ID + FIOS | Event ID uniqueness — first event ID defined wins |

#### LIOS directories (Last In Only Served — last mod wins, megapatch CAN override)

**High priority (most conflicts, most mods):**

| Directory | Files | Analysis | Key Questions |
|---|---|---|---|
| `inline_scripts` | 618 | Template collision | Same inline script name from multiple mods? |
| `static_modifiers` | 329 | ID extraction | Modifier ID collisions? Vanilla overrides? |
| `scripted_effects` | 329 | ID extraction | Effect ID collisions? |
| `on_actions` | 305 | Full file diff + handler merge | BPU override + all handler registrations |
| `scripted_triggers` | 289 | ID extraction | Trigger ID collisions? |
| `pop_faction_types` | 243 | Logic diff + additive | 4+ mods per vanilla faction file |
| `buildings` | 189 | ID + upgrade chain | Building ID uniqueness, upgrade chains, slot interactions |
| `traits` | 185 | ID + collision classify | 183 collisions across 1930 entries |
| `technology` | 174 | ID + dependency | Tech prereq chains valid across mods? |
| `governments` | 200 | ID + form matching | 6 mods on usage.txt, authority matching |
| `deposits` | 142 | ID extraction | Planet deposit conflicts? |
| `special_projects` | 125 | ID extraction | Project ID collisions? |
| `section_templates` | 122 | ID + class coverage | NSC3 + At War sections for all ship classes |
| `anomalies` | 116 | ID extraction | Anomaly ID collisions? |
| `decisions` | 111 | ID extraction | Decision ID collisions? |
| `pop_jobs` | 103 | ID + balance | Job ID collisions? Cross-mod job definitions? |
| `traditions` | 98 | ID + category count | Category count vs 64 slot cap |
| `tradition_categories` | 78 | ID + slot count | How many tradition CATEGORIES (trees) vs cap? |
| `megastructures` | 92 | ID extraction | Megastructure ID collisions? |
| `opinion_modifiers` | 91 | ID extraction | Opinion modifier collisions? |
| `ship_sizes` | 84 | ID + define merge | NSC3 + Downscaled + Fleet Formation |
| `colony_types` | 83 | ID extraction | Hybrid Colony Designations + PD colony types |
| `districts` | 81 | ID extraction | PD + BPVR + vanilla district conflicts |
| `edicts` | 71 | ID extraction | Edict ID uniqueness, Trade for Influence rework |
| `ascension_perks` | 70 | ID + rework detection | Worthy/Better/Expanded same-perk reworks |
| `scripted_loc` | 70 | ID extraction | Scripted localisation collisions? |
| `economic_categories` | 64 | ID extraction | Universal Resource Patch coverage |
| `archaeological_site_types` | 56 | ID extraction | Dig site ID collisions? |
| `component_sets` | 53 | ID extraction | Component set definitions |
| `event_chains` | 50 | ID extraction | Event chain ID collisions? |
| `zones` | 47 | ID + slot structure | Universal Zone Patch coverage |
| `policies` | 46 | ID extraction | Policy ID collisions? |
| `armies` | 46 | ID extraction | Army type collisions? |
| `situations` | 42 | ID extraction | Situation ID collisions? |
| `relics` | 41 | ID extraction | Relic ID collisions? |
| `planet_classes` | 41 | ID extraction | PD planet class definitions |
| `defines` | 39 | Key-value merge | Which defines redefined by multiple mods? |
| `game_rules` | 31 | Key-value merge | Which rules redefined? |

**Medium priority (some content, fewer conflicts):**

| Directory | Files | Analysis |
|---|---|---|
| `species_classes` | 39 | ID extraction |
| `resolutions` | 35 | ID extraction (NGS + Hydra + FH) |
| `country_types` | 35 | ID extraction |
| `starbase_buildings` | 30 | ID extraction |
| `planet_modifiers` | 30 | ID extraction (Guilli's) |
| `message_types` | 30 | ID extraction |
| `personalities` | 29 | ID + AI behavior (5 mods override) |
| `terraform` | 28 | ID extraction |
| `casus_belli` | 28 | ID extraction |
| `war_goals` | 26 | ID extraction |
| `species_rights` | 26 | ID extraction |
| `ship_behaviors` | 23 | ID extraction (3 mods override) |
| `espionage_operation_types` | 20 | ID extraction |
| `federation_laws` | 19 | ID extraction |
| `council_agendas` | 18 | ID extraction |
| `ai_espionage` | 18 | Strategy extraction |
| `colony_automation` | 17 | Rule extraction |
| `strategic_resources` | 14 | ID extraction |
| `map_modes` | 14 | ID extraction |
| `bypass` | 14 | ID extraction |
| `artifact_actions` | 13 | ID extraction |
| `scripted_modifiers` | 13 | ID extraction |
| `bombardment_stances` | 12 | ID extraction |
| `agreement_term_values` | 12 | ID extraction |
| `starbase_modules` | 11 | ID extraction |
| `agreement_presets` | 11 | ID extraction |
| `zone_slots` | 10 | Slot definitions |
| `pop_categories` | 10 | Full file diff (BPU + FP) |

**Low priority (few files, likely no conflicts):**

| Directory | Files | Notes |
|---|---|---|
| `country_limits` | 9 | |
| `tradable_actions` | 8 | |
| `fallen_empires` | 8 | |
| `economic_plans` | 8 | |
| `diplomatic_actions` | 8 | |
| `ai_budget` | 8 | AI spending weights |
| `sector_types` | 7 | |
| `resolution_categories` | 7 | |
| `ethics` | 6 | 2 mods override |
| `star_classes` | 6 | |
| All others with ≤5 files | ~40 dirs | Inventory only, unlikely conflicts |

#### Non-common directories also needing analysis

| Directory | Files | Analysis |
|---|---|---|
| `events/` | 1088 | FIOS event ID uniqueness |
| `localisation/` | 7424 | LIOS key collisions |
| `interface/` | varies | UI widget conflicts |
| `gfx/` | varies | Asset conflicts (if not stripped) |

**Total analysis scope: 148 common/ subdirectories + events + localisation + interface.**
| `common/diplomatic_actions/` | ID extraction | Action ID uniqueness? |
| `common/espionage_operation_types/` | ID extraction | Operation ID uniqueness? Spy network gates consistent? |
| `common/council_positions/` | ID extraction | Position ID uniqueness? |
| `common/situations/` | ID extraction | Situation ID uniqueness? |
| `common/map_modes/` | ID extraction | Map mode ID uniqueness? |
| `interface/` | UI widget conflicts | Multiple mods changing same UI elements? |
| `localisation/` | Key extraction | Localisation key uniqueness? Missing keys? |

---

## What the Generator Must Produce

### Auto-generated files (deterministic, rebuilable)

| Output File | Source | Method |
|---|---|---|
| `common/on_actions/00_on_actions.txt` | BPU base + all mod handlers | Merge: start from BPU, append all handler blocks |
| `common/edicts/megapatch_edicts.txt` | All mod edict definitions | Consolidate: collect all unique IDs into one file |
| `common/civics/megapatch_civics.txt` | All mod civic definitions | Consolidate: collect all unique IDs |
| `common/traits/megapatch_traits.txt` | All mod trait definitions | Consolidate: collect all unique IDs |
| `common/technology/megapatch_tech.txt` | All mod tech definitions | Consolidate: collect all unique IDs, validate prereqs |
| `common/buildings/megapatch_buildings.txt` | All mod building definitions | Consolidate: collect all unique IDs, validate upgrade chains |
| `interface/resource_groups/megapatch_resources.txt` | All mod resource UI entries | Consolidate: ensure all custom resources appear in topbar |
| `localisation/english/megapatch_l_english.yml` | Missing loc keys | Fill: any key referenced but not defined |

### Semi-generated files (base generated, manual adjustments)

| Output File | Generated Part | Manual Part |
|---|---|---|
| `common/pop_categories/00_pop_social_classes.txt` | Base from vanilla + BPU structure | FP parliamentary additions, balancing |
| `common/game_rules/megapatch_rules.txt` | Key-value merge of all mod overrides | Weight tuning, priority ordering |
| `common/defines/megapatch_defines.txt` | Key-value merge, detect conflicts | Conflict resolution (which value wins) |
| `common/pop_faction_types/megapatch_factions.txt` | Additive entries collected | Attraction formula merging |
| `events/megapatch_bridge_events.txt` | Event stubs generated per wire | Bridge logic hand-written |
| `common/ai_budget/megapatch_ai.txt` | Budget categories from all mods | Per-personality weight tuning |

### Validation output (not shipped, build-time only)

| Report | Content |
|---|---|
| `build/conflicts.txt` | All file-level conflicts detected |
| `build/id_collisions.txt` | All ID collisions across mods |
| `build/missing_refs.txt` | All broken cross-mod references |
| `build/tradition_count.txt` | Total traditions vs 64 cap |
| `build/trait_slot_audit.txt` | Trait slots used vs available per leader class |
| `build/tech_prereq_audit.txt` | Cross-mod tech prerequisite chain validation |
| `build/perk_rework_audit.txt` | Same vanilla perk modified by multiple mods |

---

## Build Process

```sh
# Full build from clean state
./tools/megapatch-build.sh

# Which runs:
1. megapatch-analyze.sh --all          # produce analysis reports
2. megapatch-generate.sh               # produce generated files
3. megapatch-validate.sh               # check output integrity
4. megapatch-package.sh                # assemble final mod structure
```

### Incremental build

When a single mod updates:
```sh
./tools/megapatch-build.sh --changed <mod_id>
# Only re-analyzes and re-generates files affected by the changed mod
```

### Determinism guarantee

The same set of input mods MUST produce the same output files.
- No timestamps in generated files (except header comments)
- Entries sorted by ID within each file (deterministic order)
- No random or environmental data in output
- Build is reproducible on any machine with the same modlist + mod files

---

## Technology Choices

The build tooling must be:
- **Shell scripts** for orchestration (user preference: no Python)
- **jq** for JSONL manipulation (already in use)
- **awk/sed** for Clausewitz file parsing (structured text, not JSON)
- **Rust** if any performance-critical analysis is needed (user preference)

Clausewitz syntax is `key = { value }` nested structures. Parsing it properly
requires understanding:
- Brace nesting
- `#` comments  
- String quoting (rare but exists)
- Inline scripts (text substitution blocks)
- `@variable` references

A proper Clausewitz parser in awk or Rust is the foundation of the entire
build system. Without it, we're doing regex matching which will miss edge cases.

---

## Integration with Modlist Tooling

The megapatch build reads from `.cache/modlist/mods/` (the downloaded mod files)
and `modlist/modlists.jsonl` (which mods are in the active modlist). The build
pipeline is:

```
modlist-materialize.sh      → sets up .active/ with the right mods
modlist-download.sh         → downloads mod files via steamcmd
megapatch-build.sh          → analyzes + generates + validates megapatch
heritage-build.sh (future)  → builds heritage mod on top of megapatch
```

The megapatch version in `modlists.jsonl` tracks which megapatch build
corresponds to which modlist version. If the modlist changes, the megapatch
must be rebuilt.
