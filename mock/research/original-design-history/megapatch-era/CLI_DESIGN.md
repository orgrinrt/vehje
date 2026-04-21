# Megapatch CLI — Design Document v4

> This document is the complete, self-contained specification for the megapatch CLI refactor. An implementer reading ONLY this document should be able to build the entire system without consulting any other file. All decisions are explicit. All assumptions are stated.

---

## What is the megapatch?

Stellaris is a Paradox grand strategy game. It supports mods via the Clausewitz engine. This project maintains a modpack of 448 mods that play together in multiplayer. When two mods modify the same game entry (a building, a trigger, a ship class), they conflict. The Clausewitz engine silently picks one version — often the wrong one. The megapatch resolves ALL conflicts across all 448 mods and produces output that shadows the conflicting entries with correct, consolidated versions.

The megapatch is NOT a single mod. It produces TWO output mods:
- **megapatch_fios** — loads FIRST in the mod list (position 1)
- **megapatch** — loads LAST in the mod list (position N)

These two mods sandwich all 448 subscribed mods. Conflict-resolved entries in FIOS directories go into megapatch_fios (so they load before any mod can define them). Conflict-resolved entries in LIOS directories go into megapatch (so they load after every mod and override them). Players still subscribe to all 448 mods on Steam Workshop — the megapatch doesn't replace them, it shadows their conflicting text files.

---

## What is this refactor?

The current toolchain is 35 shell scripts (6,800+ lines) with massive duplication, dead code, missing file type coverage, and a broken orchestrator. This refactor consolidates everything into:

- **One CLI entrypoint**: `megapatch <command> [options]`
- **Four shared libraries**: `lib/common.sh`, `lib/clausewitz.sh`, `lib/loadorder.sh`, `lib/validate.sh`
- **Seven subcommand files**: `cmd/build.sh`, `cmd/discover.sh`, `cmd/inspect.sh`, `cmd/scan.sh`, `cmd/modlist.sh`, `cmd/deploy.sh`, `cmd/test.sh`
- **Zero duplication**: every constant, function, and pattern defined exactly once

---

## How the Clausewitz engine resolves conflicts

This is the most important section. Every design decision flows from how the engine works.

### File types

The engine loads plain-text files from mod directories. The text-based file types are:

| Extension | Where | What it contains | Example |
|-----------|-------|-----------------|---------|
| `.txt` | `common/`, `events/`, `prescripted_countries/` | Game scripts: triggers, effects, buildings, events, policies, etc. | `common/scripted_triggers/00_scripted_triggers.txt` |
| `.asset` | `gfx/models/` | Entity definitions: ship model scale, animations, particle references | `gfx/models/ships/humanoid_01/_humanoid_01_ships_entities.asset` |
| `.gfx` | `interface/`, `gfx/` | Sprite sheets, icon definitions, font references | `interface/emex_icons.gfx` |
| `.gui` | `interface/` | GUI layout definitions | `interface/topbar_factions_view.gui` |
| `.shader` | `gfx/FX/` | HLSL-style shader programs in Paradox's custom format | `gfx/FX/pdxmesh.shader` |
| `.fxh` | `gfx/FX/` | Shader include files | `gfx/FX/giga_shaders.fxh` |
| `.yml` | `localisation/` | Localisation strings (YAML format) | `localisation/english/l_english.yml` |

**Binary files** (`.mesh`, `.dds`, `.wav`, `.ttf`, `.bin`) are NOT text, NOT parsed by the Clausewitz script engine, and NOT included in the megapatch output. They don't cause multiplayer desync (they affect only visuals/audio).

**IMPORTANT**: `.asset` and `.gui` files ARE multiplayer desync vectors. Entity properties and button_effects in these files affect game simulation state. They MUST be included in the megapatch output, not left to individual mod subscriptions.

### FIOS vs LIOS vs FULL vs ADDITIVE

The engine resolves duplicate definitions differently per directory:

**FIOS (First In Only Served)**: For directories where the first-loaded definition wins. If mod A loads before mod B, mod A's version of `SMALL_RED_LASER` is used and mod B's is ignored. The megapatch's FIOS output must load BEFORE all other mods (hence megapatch_fios at position 1).

Directories: `common/component_templates`, `common/global_ship_designs`, `events`

**LIOS (Last In Only Served)**: For directories where the last-loaded definition wins. If mod A loads before mod B, mod B's version of `habitable_planet` is used. The megapatch's LIOS output must load AFTER all other mods (hence megapatch at position N).

Directories: `common/traits`, `common/technology`, `common/buildings`, `common/scripted_triggers` (entry-level), `common/game_rules`, `common/ship_sizes`, and ~40 more (see full list in rules-fios-lios.txt).

**FULL**: The entire file is replaced wholesale. If two mods provide `section_templates/corvette.txt`, one completely replaces the other — no per-entry merging.

Directories: `common/pop_categories`, `common/pop_faction_types`, `common/section_templates`, `common/ship_behaviors`, `common/ethics`

**ADDITIVE**: Multiple mods can define the same entry name and all definitions coexist. For scripted_triggers, if mod A and mod B both define `my_trigger`, the LAST one loaded is the one that resolves when the trigger is called (effectively LIOS at the entry level), but both files load.

Directories: `common/scripted_effects`, `common/scripted_triggers`, `common/scripted_variables`, `common/inline_scripts`, and ~13 more.

**MERGE (on_actions only)**: Multiple files defining the same on_action key have their event handlers merged additively. The engine collects all `events = { ... }` blocks from all mods for each on_action.

### Corrected classifications (from modder review)

These are WRONG in the current `rules-fios-lios.txt` and must be fixed:

| Directory | Current classification | Correct classification | Why |
|-----------|----------------------|----------------------|-----|
| `common/script_values` | LIOS | **FIOS** | Uses the same internal registry as scripted_triggers — first definition cached, later ones ignored |
| `common/inline_scripts` | ADDITIVE | **FIOS** | `inline_script = "name"` resolves at parse time by finding the first matching file |

### `replace_path` directives

A mod can include `replace_path = "common/traits"` in its `descriptor.mod` file. This tells the engine to **completely discard** every file from every earlier-loaded mod in that directory. This is a nuclear option that silently invalidates our conflict analysis for that directory.

**The current toolchain does not detect `replace_path` at all.** This is a critical gap. The new CLI must scan every mod's `descriptor.mod` for `replace_path` entries during the discover phase.

### Load order

Stellaris loads mods in the order specified by the launcher's load order. Within a single mod, files in a directory are loaded alphabetically. Our naming convention exploits this:

- `00_megapatch_<dir>.txt` — sorts first alphabetically (for FIOS mods)
- `zz_megapatch_<dir>.txt` — sorts last alphabetically (for LIOS mods)

The load order for our setup is:
```
Position 1:  megapatch_fios (our FIOS output)
Position 2:  First subscribed mod
Position 3:  Second subscribed mod
...
Position N-1: Last subscribed mod
Position N:  megapatch (our LIOS output)
Position N+1: Heritage (future, depends on megapatch)
```

---

## Environment configuration

The toolchain needs to know where files live on the local machine. These paths differ per machine and per OS. They are configured via a `.env.local` file at the repo root (gitignored — never committed):

```sh
# .env.local — local machine configuration
# Copy .env.local.example and fill in your paths

# Where Steam Workshop mod content is stored locally.
# macOS default shown. On Linux: ~/.local/share/Steam/steamapps/workshop/content/281990
# On Windows: C:\Program Files (x86)\Steam\steamapps\workshop\content\281990
STEAM_CONTENT="$HOME/Library/Application Support/Steam/steamapps/workshop/content/281990"

# Vanilla Stellaris game files. These are the unmodded base game files.
# Used as a reference baseline for conflict detection.
# NOT shipped with the repo — fetch via: megapatch modlist sync-vanilla
VANILLA_DIR="$HOME/Library/Application Support/Steam/steamapps/common/Stellaris"

# Steam Deck configuration (optional — only needed for deploy/test commands)
DECK_HOST="192.168.1.113"
DECK_USER="deck"
DECK_STELLARIS_DIR="/home/deck/.local/share/Paradox Interactive/Stellaris"

# Stellaris version (auto-detected from VANILLA_DIR if not set)
# STELLARIS_VERSION="4.3"
```

`lib/common.sh` reads `.env.local` if it exists, falls back to defaults for macOS. The `.env.local.example` file is committed to the repo as a template.

### Vanilla reference files

The repo does NOT ship vanilla Stellaris files (copyright). They must be fetched once and stored at `vanilla/` in the repo (gitignored). The CLI provides:

```
megapatch modlist sync-vanilla
```

This copies all text-based Clausewitz files (`.txt`, `.asset`, `.gfx`, `.gui`, `.shader`, `.fxh`) from the local Stellaris install (configured in `.env.local`) or rsync's from the Steam Deck. Only text files — no binaries.

---

## Directory structure

```
tools/
├── megapatch                    # THE entrypoint — thin POSIX sh dispatcher
├── megapatch/                   # Subcommand and library directory
│   ├── lib/
│   │   ├── common.sh           # .env.local loading, paths, clausewitz_find(),
│   │   │                       # get_mod_name(), for_each_mod(), make_tmp()
│   │   ├── clausewitz.sh       # Clausewitz parser: extract entries, scan dirs,
│   │   │                       # find entries by ID (wraps existing awk parser)
│   │   ├── loadorder.sh        # FIOS/LIOS rules loading, replace_path detection,
│   │   │                       # winner resolution, load order building
│   │   └── validate.sh         # Brace balance checking (comment/quote-aware),
│   │                           # patch success verification, output integrity
│   ├── cmd/
│   │   ├── build.sh            # Build pipeline: discover → extract → constants →
│   │   │                       # assemble → validate. Stages, stamps, --from/--only.
│   │   ├── discover.sh         # Conflict scanning: file-level, ID-level,
│   │   │                       # replace_path detection
│   │   ├── inspect.sh          # Entry investigation: show versions, diff, classify,
│   │   │                       # pair comparison, file analysis
│   │   ├── scan.sh             # Content inventory: planet classes, traits, factions,
│   │   │                       # civics, ship sizes, constants
│   │   ├── modlist.sh          # Modlist management: materialize, download, validate,
│   │   │                       # views, order, pin, check, sync-vanilla
│   │   ├── deploy.sh           # Deployment: rsync to Steam Deck or copy to local
│   │   │                       # Stellaris mod directory
│   │   └── test.sh             # Testing: deploy scenario, collect logs, analyze
│   └── legacy/                 # Dead/superseded scripts moved here for reference
│       └── ...
├── clausewitz-extract           # Thin wrapper that sources lib/clausewitz.sh,
│                                # provides CLI backwards compat for direct use
├── deck-connect.sh              # SSH to Steam Deck (standalone utility, not megapatch)
└── heritage/                    # Heritage mod testing (future, separate concern)
    └── test-scenarios/
```

### Why this structure

- `tools/megapatch` is the CLI entrypoint. It's a POSIX sh script that reads `$1` and sources the corresponding `cmd/*.sh` file.
- `tools/megapatch/lib/` contains sourceable library files. Every `cmd/*.sh` sources all four libs at startup.
- `tools/megapatch/cmd/` contains one file per subcommand group.
- `tools/megapatch/legacy/` is a graveyard for old scripts. Not loaded, not referenced — just preserved for historical context.
- All scripts are POSIX sh (`#!/bin/sh`), not bash. The common lib avoids bash-isms (no arrays, no `${var:2}`, no `[[ ]]`).

---

## Shared libraries

### `lib/common.sh`

This file is sourced by every subcommand. It provides:

```sh
#!/bin/sh
# Load .env.local if present
PROJECT_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
[ -f "$PROJECT_ROOT/.env.local" ] && . "$PROJECT_ROOT/.env.local"

# Paths (from .env.local or defaults)
STEAM_CONTENT="${STEAM_CONTENT:-$HOME/Library/Application Support/Steam/steamapps/workshop/content/281990}"
VANILLA_DIR="${VANILLA_DIR:-$PROJECT_ROOT/.cache/vanilla}"
WORKSHOP_IDS="$PROJECT_ROOT/.cache/modlist/workshop-ids.txt"
MODLIST_INDEX="$PROJECT_ROOT/data/mods.jsonl"
RULES_FILE="$PROJECT_ROOT/data/rules-fios-lios.txt"
BUILD_DIR="$PROJECT_ROOT/megapatch/generated"
PATCH_DIR="$PROJECT_ROOT/megapatch/patches"
OUTPUT_DIR="$PROJECT_ROOT/.dist"

# Find all Clausewitz text files in a directory.
# This is THE function that fixes the .txt-only blindspot.
# Every script that scans mod files MUST use this, never raw `find`.
clausewitz_find() {
    _dir="$1"; shift
    find "$_dir" "$@" -type f \( \
        -name "*.txt" -o -name "*.asset" -o -name "*.gfx" -o \
        -name "*.gui" -o -name "*.shader" -o -name "*.fxh" \
    \) 2>/dev/null
}

# Same but includes localisation .yml files.
clausewitz_find_all() {
    _dir="$1"; shift
    find "$_dir" "$@" -type f \( \
        -name "*.txt" -o -name "*.asset" -o -name "*.gfx" -o \
        -name "*.gui" -o -name "*.shader" -o -name "*.fxh" -o \
        -name "*.yml" \
    \) 2>/dev/null
}

# Check if a file is a Clausewitz text file by extension.
is_clausewitz_file() {
    case "$1" in
        *.txt|*.asset|*.gfx|*.gui|*.shader|*.fxh) return 0 ;;
        *) return 1 ;;
    esac
}

# Mod name lookup from full_index.jsonl. Cached on first call.
_MOD_NAMES_CACHE=""
get_mod_name() {
    if [ -z "$_MOD_NAMES_CACHE" ]; then
        if [ -f "$MODLIST_INDEX" ]; then
            _MOD_NAMES_CACHE=$(jq -r '"\(.id)\t\(.name)"' "$MODLIST_INDEX" 2>/dev/null)
        fi
        _MOD_NAMES_CACHE="${_MOD_NAMES_CACHE:-_empty_}"
    fi
    echo "$_MOD_NAMES_CACHE" | grep "^${1}	" | cut -f2 | head -1
}

# Iterate over all active mod IDs.
# Usage: for_each_mod callback_function
#   callback receives: $1 = mod_id, $2 = mod_dir (full path)
for_each_mod() {
    _callback="$1"
    while IFS= read -r _id; do
        [ -z "$_id" ] && continue
        _dir="$STEAM_CONTENT/$_id"
        [ -d "$_dir" ] && "$_callback" "$_id" "$_dir"
    done < "$WORKSHOP_IDS"
}

# Temp file management. Creates a temp file and registers it for cleanup.
_TMP_FILES=""
make_tmp() {
    _t=$(mktemp "/tmp/megapatch_${1:-tmp}_$$.XXXXXX")
    _TMP_FILES="$_TMP_FILES $_t"
    echo "$_t"
}
cleanup_tmp() { rm -f $_TMP_FILES; }
trap cleanup_tmp EXIT INT TERM
```

### `lib/clausewitz.sh`

Wraps the existing awk-based Clausewitz parser into sourceable functions. The awk core is unchanged — it's proven, handles comments, quotes, and nested braces correctly. The functions are:

```sh
# Extract all top-level entries from a single file.
# Output: FILE\tKEY\tSTART_LINE\tEND_LINE (one line per entry)
clausewitz_extract_entries() { ... }  # wraps the awk

# Extract just entry IDs from a file.
# Output: KEY (one per line)
clausewitz_extract_ids() { ... }

# Extract full content of all entries from a file.
# Output: blocks of text separated by entry markers
clausewitz_extract_full() { ... }

# Scan a directory recursively for all entries.
# Uses clausewitz_find() to find files (not hardcoded *.txt).
clausewitz_scan() { ... }
clausewitz_scan_ids() { ... }

# Find a specific entry by ID across all files in a directory.
clausewitz_find_entry() { ... }  # ENTRY_ID, DIR → file path + line range

# Read the content of a specific entry from a file.
clausewitz_read_entry() { ... }  # ENTRY_ID, FILE → content text
```

### `lib/loadorder.sh`

Consolidates the 3 copies of FIOS/LIOS logic into one:

```sh
# Load the rules file once. Called automatically on first use.
load_rules() { ... }  # parses rules-fios-lios.txt into lookup

# Get the classification for a directory.
# Returns: "FIOS" | "LIOS" | "FULL" | "ADDITIVE" | "MERGE" | "COSMETIC" | "UNKNOWN"
get_dir_type() { ... }

# Determine which mod's version wins for an item.
# For FIOS: lowest load-order position wins.
# For LIOS: highest load-order position wins.
# For ADDITIVE: last definition wins (effectively LIOS at entry level).
get_winner() { ... }

# Build the full load order from workshop-ids.txt.
# Returns: ordered list of mod_id with sequential position numbers.
build_load_order() { ... }

# Scan all mod descriptor.mod files for replace_path directives.
# Returns: list of (mod_id, replace_path_dir) pairs.
scan_replace_paths() { ... }
```

### `lib/validate.sh`

Consolidates the 3 copies of brace validation:

```sh
# Check brace balance in a Clausewitz file.
# Comment-aware (ignores braces in # comments).
# Quote-aware (ignores braces in "strings").
# Returns 0 if balanced, 1 if not.
validate_braces() { ... }  # the existing awk, wrapped

# Check that all patches applied successfully.
# For each replace_in_item patch, verify the find string was matched.
# Returns 0 if all patches applied, 1 if any failed (with error output).
validate_patches() { ... }

# Check all output files for integrity.
validate_output() { ... }  # runs braces + patches + completeness
```

---

## CLI entrypoint

`tools/megapatch` is a thin dispatcher:

```sh
#!/bin/sh
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CMD_DIR="$SCRIPT_DIR/megapatch/cmd"
LIB_DIR="$SCRIPT_DIR/megapatch/lib"

# Source libraries
. "$LIB_DIR/common.sh"
. "$LIB_DIR/clausewitz.sh"
. "$LIB_DIR/loadorder.sh"
. "$LIB_DIR/validate.sh"

case "${1:-}" in
    build)    shift; . "$CMD_DIR/build.sh" "$@" ;;
    discover) shift; . "$CMD_DIR/discover.sh" "$@" ;;
    inspect)  shift; . "$CMD_DIR/inspect.sh" "$@" ;;
    scan)     shift; . "$CMD_DIR/scan.sh" "$@" ;;
    modlist)  shift; . "$CMD_DIR/modlist.sh" "$@" ;;
    deploy)   shift; . "$CMD_DIR/deploy.sh" "$@" ;;
    test)     shift; . "$CMD_DIR/test.sh" "$@" ;;
    validate) shift; validate_output "$@" ;;
    clean)    rm -rf "$BUILD_DIR" "$OUTPUT_DIR" ;;
    *)        echo "Usage: megapatch <build|discover|inspect|scan|modlist|deploy|test|validate|clean>" ;;
esac
```

---

## Build pipeline

```
megapatch build
  Stage 1: discover    → file conflicts + ID conflicts + replace_path scan
  Stage 2: extract     → item database from all mods (items.tsv)
  Stage 3: constants   → @variable collection (constants.tsv)
  Stage 4: assemble    → apply patches, produce per-directory Clausewitz output
  Stage 5: validate    → braces + patch success + completeness check
```

Each stage writes a stamp file (`$BUILD_DIR/.stamp_<stage>`) containing a hash of its inputs. On next run:
- If the stamp exists and the input hash matches → skip the stage
- If the stamp is missing or hash differs → run the stage, invalidate all downstream stamps

`megapatch build --from assemble` — skip stages 1-3 (trusts existing stamps), run from assemble.
`megapatch build --only discover` — run only the discover stage.
`megapatch build --force` — ignore all stamps, rebuild everything.

### What each stage does

**Stage 1: discover**
- Scans all active mods (from `workshop-ids.txt`) for file-level conflicts (same filename in 2+ mods)
- Scans all active mods for entry-level ID conflicts (same entry ID in different files across mods)
- Scans all `descriptor.mod` files for `replace_path` directives
- Uses `clausewitz_find()` (not `*.txt` hardcoded) to find files
- Output: `$BUILD_DIR/discover/{dir-inventory.tsv, file-conflicts.tsv, id-conflicts/*.tsv, replace-paths.tsv}`

**Stage 2: extract**
- Extracts every top-level Clausewitz entry from every text file in every mod
- Records: item_id, directory, source_mod, source_file, line_count, content_hash
- Uses `clausewitz_extract_entries()` from `lib/clausewitz.sh`
- Output: `$BUILD_DIR/items.tsv`

**Stage 3: constants**
- Extracts all `@variable = value` definitions from all mods
- Detects conflicts (same @variable, different values across mods)
- Output: `$BUILD_DIR/constants.tsv`, `$BUILD_DIR/constants_unified.txt`

**Stage 4: assemble**
- Reads items.tsv, determines winner per item using `get_winner()` from `lib/loadorder.sh`
- Reads patch YAML files from `$PATCH_DIR/<directory_slug>/*.yml`
- Applies patches (override, prefer_mod, replace_in_item, inject_field, delete_item, insert_item, no_override, constant)
- Produces Clausewitz output files split into two mods:
  - `$OUTPUT_DIR/megapatch/common/<dir>/zz_megapatch_<dir>.txt` (LIOS entries)
  - `$OUTPUT_DIR/megapatch_fios/common/<dir>/00_megapatch_<dir>.txt` (FIOS entries)
  - Same split for events/, gfx/, interface/, etc.
- Generates `descriptor.mod` for both output mods

**Stage 5: validate**
- Checks every output file for balanced braces (comment-aware, quote-aware)
- Checks every `replace_in_item` patch: was the find string actually matched? If not → hard error.
- Checks completeness: every entry in the conflict manifest has a resolution in the output.

### No passthrough

The output contains ONLY conflict-resolved entries. Files that exist in only one mod and have no conflicts are NOT copied into the output. The underlying mod subscriptions provide these — the megapatch only shadows the conflicts.

**Exception**: If a mod uses `replace_path` for a directory, ALL content for that directory must be in the output (because the engine discards everything from earlier mods in that directory).

---

## Version pinning

Mods update on Steam Workshop without warning. A mod update can break our patches (find strings no longer match), create new unresolved conflicts, or remove entries our patches target.

```
megapatch/versions.lock
```

TSV lockfile recording the state of every mod at build time:

```tsv
MOD_ID	CONTENT_HASH	FILES_HASH	LAST_VERIFIED	STEAM_UPDATED
683230077	a7f3bc2d...	e91d44f1...	2026-04-04	2026-03-15
727000451	b2c9e7a1...	f03a82c5...	2026-04-04	2026-03-28
```

No `VERSION` field — Steam Workshop provides no reliable version metadata API. Detection is via content hashing + the Steam Web API's `TimeUpdated` timestamp (from `ISteamRemoteStorage/GetPublishedFileDetails`, no auth required).

- `megapatch modlist pin` — record current mod states to lockfile
- `megapatch modlist check` — compare current mods against lockfile, report drift
- `megapatch build` — checks lockfile before building, warns on drift, aborts with `--strict`

### Rebuild triggers

| Trigger | How detected | Earliest invalidated stage |
|---------|-------------|---------------------------|
| Mod added or removed from modlist | `workshop-ids.txt` content hash | discover |
| Mod content updated (Workshop push) | Lockfile content hash mismatch | extract |
| Patch YAML file added or modified | `patches/` directory hash | assemble |
| Vanilla Stellaris game update | `vanilla/` directory hash | discover |
| FIOS/LIOS rules changed | `rules-fios-lios.txt` hash | extract |

---

## Full CLI reference

```
megapatch <command> [options]

MODLIST MANAGEMENT
  modlist materialize               Resolve JSONL modlist → workshop-ids.txt
  modlist download [--resume]       Download mods via steamcmd
  modlist validate [--fix]          Check full_index.jsonl integrity
  modlist views                     Generate markdown views (approved/rejected/etc.)
  modlist order [--resolve-names]   Analyze/validate load order
  modlist pin                       Record current mod versions to versions.lock
  modlist check                     Compare current mods against lockfile
  modlist sync-vanilla              Fetch vanilla reference files to vanilla/

BUILD PIPELINE
  build                             Full pipeline (all stages)
  build --from <stage>              Resume from a specific stage
  build --only <stage>              Run single stage only
  build --force                     Ignore stamps, rebuild everything
  build --strict                    Abort on lockfile drift (no warnings)

CONFLICT DISCOVERY
  discover                          Full scan (files + IDs + replace_paths)
  discover --files                  File-level conflicts only
  discover --ids [<subdir>]         Entry ID conflicts (optionally filtered)
  discover --replace-paths          Scan descriptor.mod for replace_path directives

CONTENT SCANNING
  scan --content                    Inventory: planets, traits, factions, civics, etc.
  scan --constants                  @variable definitions across all mods

ENTRY INVESTIGATION
  inspect <entry_id> [--dir <D>]    Show all mod versions of an entry
  inspect <entry_id> --diff         Normalized diff between versions
  inspect <entry_id> --classify     Classify diff type (FORMAT/VALUE/LOGIC)
  inspect --pair <m1> <m2> [--dir D]  Compare all entries between two mods
  inspect --file <relative_path>    Show entries in a conflicting file

OUTPUT VERIFICATION
  validate                          Full check: braces + patches + completeness

DEPLOYMENT
  deploy --deck                     rsync output mods to Steam Deck
  deploy --local                    Copy to local Stellaris mod directory

TESTING
  test deploy                       Deploy test scenario to Deck
  test run                          Print manual test workflow
  test collect                      Collect logs/saves from Deck
  test analyze                      Parse collected test artifacts

MAINTENANCE
  clean                             Remove all generated artifacts + stamps
```

---

## Script mapping (old → new)

Every current script maps to exactly one subcommand or library function:

| Current script (in `tools/`) | New location | Notes |
|------------------------------|-------------|-------|
| `megapatch-build.sh` | `cmd/build.sh` | Rewritten, stages from assembler |
| `megapatch-discover.sh` | `cmd/discover.sh` | Merged with discover-ids |
| `megapatch-discover-ids.sh` | `cmd/discover.sh` | Merged with discover |
| `megapatch-extract-items.sh` | `cmd/build.sh` (stage: extract) | Absorbed into build |
| `megapatch-collect-constants.sh` | `cmd/build.sh` (stage: constants) | Absorbed into build |
| `megapatch-assemble.sh` | `cmd/build.sh` (stage: assemble) | Absorbed into build |
| `megapatch-validate-braces.sh` | `lib/validate.sh` | Library function |
| `megapatch-brief-entry.sh` | `cmd/inspect.sh` | Default mode |
| `megapatch-review-entry.sh` | `cmd/inspect.sh --list` | Merged |
| `megapatch-compare-entry.sh` | `cmd/inspect.sh --diff` | Merged |
| `megapatch-classify-pair.sh` | `cmd/inspect.sh --pair` | Merged |
| `megapatch-classify-collisions.sh` | `cmd/inspect.sh --file` | Merged |
| `megapatch-scan-content.sh` | `cmd/scan.sh` | Absorbed |
| `modlist-materialize.sh` | `cmd/modlist.sh materialize` | Absorbed |
| `modlist-download.sh` | `cmd/modlist.sh download` | Absorbed |
| `modlist-resolve-order.sh` | `cmd/modlist.sh order` | Absorbed |
| `resolve-load-order-names.sh` | `cmd/modlist.sh order --resolve-names` | Absorbed |
| `generate-views.sh` | `cmd/modlist.sh views` | Absorbed |
| `validate-index.sh` | `cmd/modlist.sh validate` | Absorbed |
| `test-deploy.sh` | `cmd/test.sh deploy` | Absorbed |
| `test-run.sh` | `cmd/test.sh run` | Absorbed |
| `test-collect.sh` | `cmd/test.sh collect` | Absorbed |
| `test-analyze.sh` | `cmd/test.sh analyze` | Absorbed |
| `clausewitz-extract.sh` | `tools/clausewitz-extract` (wrapper) | Thin wrapper → lib |
| `deck-connect.sh` | `tools/deck-connect.sh` | Unchanged, standalone |

### Moved to `tools/megapatch/legacy/` (dead code, reference only)

| Script | Why dead |
|--------|----------|
| `megapatch-apply-patches.sh` | Produces patch-manifest.tsv that nothing reads |
| `megapatch-collect-items.sh` | Logic duplicated verbatim in megapatch-assemble.sh |
| `megapatch-compare-ids.sh` | Uses old `ids_<subdir>.txt` format, superseded by classify-pair |
| `megapatch-analyze.sh` | Superseded by discover.sh + scan-content.sh |
| `megapatch-merge.sh` | v1 file-centric architecture, replaced by v2 item-centric assembler |
| `megapatch-patch.sh` | v1 patch applier, replaced by v2 assembler's integrated YAML parser |
| `megapatch-common.sh` | Written but never sourced by any script. Replaced by lib/common.sh |
| `modlist-analyze.sh` | Targets old symlink directory, superseded by scan-content.sh |
| `mod-download.sh` | SSH-to-Deck downloader, superseded by modlist-download.sh |
| `mod-download-local.sh` | Local steamcmd downloader, superseded by modlist-download.sh |

---

## `descriptor.mod` generation

Both output mods need proper Clausewitz mod descriptors:

```ini
# megapatch_fios/descriptor.mod
name="Stellar Heritage — Megapatch FIOS"
path="mod/megapatch_fios"
tags={ "Total Conversion" }
supported_version="4.3.*"
thumbnail="thumbnail.png"
```

```ini
# megapatch/descriptor.mod
name="Stellar Heritage — Megapatch"
path="mod/megapatch"
tags={ "Total Conversion" }
supported_version="4.3.*"
thumbnail="thumbnail.png"
dependencies={ "Stellar Heritage — Megapatch FIOS" }
```

`supported_version` is derived from the vanilla reference files (detected game version) or overridden in `.env.local`. The `dependencies` field on the LIOS mod ensures the launcher enforces that megapatch_fios is always present when megapatch is loaded.

---

## Implementation order

Each step produces a working, testable increment:

1. Create `.env.local.example` + `.gitignore` additions + directory structure (`tools/megapatch/{lib,cmd,legacy}`)
2. Write `lib/common.sh` — .env.local loading, all paths, clausewitz_find, get_mod_name, for_each_mod, make_tmp
3. Write `lib/clausewitz.sh` — extract the awk parser from clausewitz-extract.sh into sourceable functions
4. Write `lib/loadorder.sh` — FIOS/LIOS rules loading (from rules file, not hardcoded), replace_path scanning, winner resolution, load order building
5. Write `lib/validate.sh` — brace validation + patch success checking
6. Write `tools/megapatch` (dispatcher) + `cmd/build.sh` (stage-based pipeline using the four libs)
7. Write `cmd/discover.sh` — merge megapatch-discover.sh + megapatch-discover-ids.sh + add replace_path scanning
8. Write `cmd/inspect.sh` — merge brief-entry + review-entry + compare-entry + classify-pair + classify-collisions
9. Write `cmd/scan.sh` — merge scan-content + collect-constants
10. Write `cmd/modlist.sh` — absorb modlist-materialize, modlist-download, validate-index, generate-views, resolve-load-order + add pin/check/sync-vanilla
11. Write `cmd/deploy.sh` + `cmd/test.sh` — absorb test-deploy, test-run, test-collect, test-analyze
12. Fix `rules-fios-lios.txt` — correct script_values→FIOS, inline_scripts→FIOS
13. Move dead scripts to `tools/megapatch/legacy/`
14. Verify: run `megapatch build` end-to-end, compare output hash to previous build
15. Remove old scripts from `tools/` root (after verification passes)

---

## Non-goals

These are explicitly out of scope for this refactor:

- **Rewriting the Clausewitz awk parser** — the core parser works. We wrap it, not rewrite it.
- **Changing the patch YAML format** — existing patches continue to work unchanged.
- **Changing the output directory layout** — `.dist/{megapatch,megapatch_fios}` stays.
- **Rewriting the assembler's internal awk** — the 800-line awk in megapatch-assemble.sh stays. We source it as a stage in the build pipeline.
- **GUI or mod browser** — no UI. CLI only.
- **Steam Workshop publishing** — no upload. Distribution is manual (rsync/copy).
- **Heritage mod code** — untouched. Heritage is a separate top-level concern.
- **New patch types** — the existing 8 patch types are sufficient.
