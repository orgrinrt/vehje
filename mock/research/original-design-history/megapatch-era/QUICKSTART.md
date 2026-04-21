# Megapatch Quick Start

## Full Build (from scratch)

```bash
# 1. Extract all items from all mods
./tools/megapatch-extract-items.sh --all

# 2. Collect constants
./tools/megapatch-collect-constants.sh

# 3. Apply patches → generate manifest
./tools/megapatch-apply-patches.sh

# 4. Assemble output
./tools/megapatch-assemble.sh

# 5. Validate
./tools/megapatch-validate-braces.sh
```

## Single Directory (faster for iteration)

```bash
./tools/megapatch-extract-items.sh --dir common/scripted_triggers
./tools/megapatch-assemble.sh --dir common/scripted_triggers
```

## Adding a New Patch

1. Create a YAML file in `megapatch/patches/<directory>/`
2. Follow the format in `megapatch/patches/PATCH_FORMAT.md`
3. Re-run apply + assemble:
   ```bash
   ./tools/megapatch-apply-patches.sh
   ./tools/megapatch-assemble.sh --dir common/scripted_triggers
   ```

## Writing an Override Patch

```yaml
# megapatch/patches/scripted_triggers/my_patch.yml
patches:
  - directory: common/scripted_triggers
    item: my_trigger_name
    type: override
    content: |
      my_trigger_name = {
          optimize_memory
          # Your Clausewitz code here
      }
    note: "Reason for this override"
```

## Checking Coverage

```bash
# See how many entries are in the manifest
./tools/megapatch-apply-patches.sh --stats

# Count entries per directory
tail -n +2 megapatch/generated/patch-manifest.tsv | awk -F'\t' '{print $1}' | sort | uniq -c | sort -rn
```

## Briefing an Entry

```bash
# See all mod versions of an entry
./tools/megapatch-brief-entry.sh common/scripted_triggers habitable_planet

# Classify a mod pair
./tools/megapatch-classify-pair.sh common/scripted_triggers 1995601384 3615026960
```

## Output Location

The assembled output goes to `.dist/megapatch/common/*/zz_megapatch_*.txt`.
This is the LIOS mod that loads last and overrides all conflicting entries.

## Key Directories

- `megapatch/patches/` — YAML patch files (our edits)
- `megapatch/patches/overrides/` — decision documents (WHY we chose each resolution)
- `megapatch/mesh/` — integration analysis (opportunities for cross-mod synergy)
- `.dist/` — generated Clausewitz output (gitignored)
- `megapatch/generated/` — intermediate data (gitignored)
- `tools/` — all build tools
