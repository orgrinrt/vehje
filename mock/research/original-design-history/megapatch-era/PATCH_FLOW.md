# Patch Flow — How Auto-Generation and Manual Patches Compose

> Not everything can be auto-merged. Some things require human judgment,
> custom logic, or creative bridging that no tool can generate. This document
> defines how automated and manual work compose into the final megapatch.

---

## The Problem

Three kinds of work produce the megapatch:

1. **Auto-merge** — collecting all trait definitions into one file (mechanical, deterministic)
2. **Manual patch** — fixing DPE's 4.3 loading hang by changing a specific line (surgical, human judgment)
3. **Bridge logic** — writing events that wire FP faction state into Social Decline stability (creative, new code)

These three can't all live in the same pipeline. Auto-merge gets rebuilt when mods update.
Manual patches target specific mod files that may change. Bridge logic is our original code
that references other mods' data.

---

## The Flow

```
Phase 1: AUTO-GENERATE (deterministic, rebuildable)
  ├── Consolidate: collect all edicts/civics/traits/techs into merged files
  ├── Merge: combine on_actions from BPU + all handlers
  ├── Detect: find conflicts, report them
  └── Output: megapatch/generated/

Phase 2: MANUAL PATCHES (human-authored, versioned)
  ├── Fix: DPE 4.3 loading hang (patch against DPE's specific file)
  ├── Fix: GVP 4.3 compatibility (patches against GVP's files)
  ├── Fix: Civil Wars breakage (patches against CW's files)
  ├── Adjust: pop_social_classes merge (FP + BPU, hand-tuned)
  ├── Adjust: AI weight tuning (personality → system weights)
  └── Output: megapatch/patches/

Phase 3: BRIDGE EVENTS (original code, our creation)
  ├── Wire: FP faction state → Social Decline stability
  ├── Wire: war outcomes → FP faction shifts
  ├── Wire: espionage → diplomatic consequences
  ├── Wire: government type → AI behavior weights
  ├── Init: megapatch_init event (game start setup)
  └── Output: megapatch/bridges/

Phase 4: ASSEMBLE
  ├── Start from: megapatch/generated/ (auto-merge base)
  ├── Apply: megapatch/patches/ (manual fixes on top)
  ├── Include: megapatch/bridges/ (original bridge code)
  ├── Package: into final mod structure
  └── Output: .dist/ (the actual mod that ships)
```

---

## Directory Structure

```
megapatch/
  DESIGN.md
  BUILD.md
  PATCH_FLOW.md
  TENETS.md
  SYSTEMS_MESH.md
  domains/                          # design docs (already written)

  generated/                        # Phase 1 output (gitignored? or committed?)
    common/
      edicts/merged_edicts.txt
      civics/merged_civics.txt
      traits/merged_traits.txt
      ...
    build-report.txt                # what was merged, what conflicted

  patches/                          # Phase 2: human-authored patches (VCS)
    dpe-4.3-fix/                    # one dir per patch
      target: Dynamic Political Events (1227620643)
      files:
        events/dpe_political_events.txt.patch
      notes.md                      # what this fixes and why
    gvp-4.3-fix/
      target: Government Variety Pack (2806903835)
      files:
        common/governments/gvp_governments.txt.patch
      notes.md
    pop-social-classes-merge/
      target: vanilla + BPU + FP
      files:
        common/pop_categories/00_pop_social_classes.txt  # full merged file
      notes.md
    ai-weight-tuning/
      target: megapatch AI behavior
      files:
        common/ai_budget/megapatch_personality_weights.txt
      notes.md

  bridges/                          # Phase 3: original code (VCS)
    common/
      on_actions/megapatch_bridge_on_actions.txt
      scripted_triggers/megapatch_stability.txt
      scripted_effects/megapatch_bridges.txt
    events/
      megapatch_political_bridge.txt
      megapatch_war_bridge.txt
      megapatch_spy_bridge.txt
      megapatch_planet_bridge.txt
      megapatch_discovery_bridge.txt
    localisation/
      english/megapatch_bridges_l_english.yml

  output/                           # Phase 4: final assembled mod (gitignored)
    descriptor.mod
    common/
    events/
    localisation/
```

---

## Patch Format

For Phase 2 manual patches, we have options:

### Option A: Git-style unified diff patches
```
--- a/events/dpe_political_events.txt
+++ b/events/dpe_political_events.txt
@@ -142,7 +142,7 @@
     trigger = {
-        on_leader_removed = yes
+        on_leader_fired = yes
     }
```
**Pro:** Standard format, `patch` command applies them, git-native
**Con:** Fragile — if the mod updates and line numbers shift, patch fails

### Option B: Semantic patches (search-and-replace rules)
```yaml
# dpe-4.3-fix.yaml
target_mod: 1227620643
patches:
  - file: "events/dpe_political_events.txt"
    find: "on_leader_removed"
    replace: "on_leader_fired"
    all: true
  - file: "events/dpe_government_events.txt"
    find: "has_modifier = old_modifier_name"
    replace: "has_modifier = new_modifier_name"
```
**Pro:** Survives minor mod updates (search by content, not line number)
**Con:** Custom format, need our own apply tool

### Option C: Full file overrides
Just ship the patched version of the file in `patches/dpe-4.3-fix/events/dpe_political_events.txt`.
**Pro:** Simplest. No patch application logic needed.
**Con:** Entire file duplicated. When mod updates, must re-diff and re-patch from scratch.

### Recommendation: Option B (semantic patches) with Option C fallback

- Semantic patches for surgical fixes (rename triggers, swap modifier names)
- Full file overrides for complex merges (pop_social_classes where the whole file is rewritten)
- A simple `megapatch-apply-patches.sh` that reads YAML rules and applies them via sed/awk
- When a patch fails to apply (target text not found), it flags the failure and falls back to the full override if one exists

---

## What Goes in VCS vs What's Generated

| Directory | In VCS? | Why |
|---|---|---|
| `megapatch/domains/` | YES | Design docs, permanent |
| `megapatch/patches/` | YES | Human-authored, source of truth |
| `megapatch/bridges/` | YES | Our original code, source of truth |
| `megapatch/research/` | YES | Human-authored research, reviews, confirmed rules |
| `megapatch/generated/` | NO (.gitignore) | Regenerable analysis output (conflicts, IDs, patterns) |
| `.dist/` | NO (.gitignore) | Assembly artifact, rebuilt from generated + patches + bridges |

The `generated/` question is interesting. Arguments for committing:
- Reproducibility: anyone can see exactly what shipped
- Review: changes in generated output are visible in git diff
- No build step needed to just use the megapatch

Arguments against:
- Noise: regeneration creates large diffs that obscure real changes
- Confusion: someone might edit generated files thinking they persist

**Decision:** Commit generated files but with a header comment saying "GENERATED — do not edit, will be overwritten by megapatch-build.sh"

---

## When to Rebuild

| Trigger | Action |
|---|---|
| Mod in modlist updates (new Workshop version) | Re-download, re-analyze, re-generate, re-apply patches |
| New mod added to modlist | Re-analyze, re-generate (new entries merged in) |
| Mod removed from modlist | Re-generate (entries removed) |
| Manual patch changed | Re-apply patches, re-assemble |
| Bridge code changed | Re-assemble (bridges are included directly) |
| Modlist version bumped | Full rebuild (tag the output) |

---

## Applying Patches to Mod Source Files

An important distinction: some patches target **mod source files** (fixing DPE's broken event),
while others target **our generated files** (tuning AI weights in the merged output).

For mod source patches:
1. The patch is defined in `patches/dpe-4.3-fix/`
2. The apply script reads the mod's files from `.active/mods/1227620643/`
3. Applies the semantic patch rules
4. Writes the patched file to `output/` (never modifies the source in .active/)
5. The output file overrides the mod's original in the load order (megapatch loads later)

For generated file adjustments:
1. Phase 1 generates the base file
2. Phase 2 patches can target generated files too
3. The patch applies to `generated/` output, producing the final in `output/`

This means `output/` is always: `generated/ + patches/ + bridges/` assembled together.
