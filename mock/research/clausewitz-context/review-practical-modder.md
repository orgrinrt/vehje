# Practical Modder Review: Megapatch Design

> Reviewed by: veteran compatibility patcher (5+ years, NSC3/ESC/multi-mod integration experience)
> Date: 2026-04-01
> Scope: All megapatch design docs, conflict analysis, FIOS/LIOS research, trait collisions

---

## Executive Summary

The design documents are ambitious and philosophically sound. The "systems mesh"
vision is the right goal. But the documents consistently underestimate the
mechanical difficulty of Clausewitz modding and overestimate what can be
automated. Several proposed merges are described as if they're data transforms
when they're actually logic rewrites that require reading thousands of lines
of mod code by hand.

This review separates what's straightforward from what will actually hurt,
identifies conflicts that don't need a megapatch at all, and flags the FIOS
trap that the design overlooks entirely.

---

## 1. What's Actually Hard vs What Sounds Hard

### EASY (hours, not days)

**Trait ID collisions (294 collisions across 1930 entries).** The collision
report shows that nearly all collisions are between Psionic/Cybernetic/
Synthetic/Erudite Leader Traits and Ascension Leader Traits 4.1. The pattern
is clear: ALT provides upgraded versions with custom icons and Paragon
inline_scripts. This is a "pick one mod" decision, not a merge. If you want
ALT's versions (which are strictly better -- they add paragon weight scripts
and fix the `offical` -> `official` typo in CLASS), just load ALT after the
individual trait mods, and LIOS handles it. **No megapatch file needed for
most of these.** The only real work is the 3-way collision on species traits:
Ariphaos + Additional Traits + Planetary Diversity on `04_species_traits.txt`.
That needs a manual merge.

**Edict consolidation.** Edicts are additive with unique IDs. The design
correctly identifies this as a clean merge. The only complication is Trade
for Influence reworking vanilla edict costs, which is a single-file override.
Half a day of work.

**Civic consolidation.** Same story. GVP, SA, Hydra, Wiirlak -- each adds
civics with unique IDs into separate files. They don't collide. The real
issue is GVP's government form definitions, not the civics themselves.

**Solar system initializers (15 files x 2-3 mods).** The conflict report shows
Ariphaos + All Systems Spawn + Real Space all touching the same initializer
files. But Ariphaos loads first (confirmed rule), Real Space loads after PD
(confirmed rule), and All Systems Spawn should load after both. These are
almost certainly load-order-solvable if RS already accounts for Ariphaos
bugfixes (which, given RS's maturity, it likely does). Download the files
and diff them before assuming a merge is needed.

**Star graphics (40+ files between Real Space and Colour Out of Deep).** These
are between two mods from the same ecosystem (Real Space). Colour Out of Deep
is a visual addon for Real Space. It should load after RS base and its overrides
are intentional. **Not a conflict. Remove from conflict list.**

**NSC3-Downscaled juggernaut .gfx files (12 files).** These are between NSC3
and its own Downscaled compatibility patch. The patch is designed to override
NSC3. Load order solves this. **Not a conflict.**

**Star Wars portrait mod shared files (usage.txt, faction_governments, etc.).**
These are identical shared framework files across the Animated SW portrait
family. The content is the same in each. Load order doesn't matter because
the files are identical. **Not a conflict.**

**At War shared localisation/tech files.** At War: Advanced Ship Sections and
At War: Carrier Improvements share `eac_awss_techs` files. Same author, same
framework. Identical or near-identical content. **Not a conflict.**

### MEDIUM (days of work)

**Faction types (8 files x 4 mods).** Ariphaos, Civil Wars, [TNG] Faction
Personality, and either Viable Feudalism or PD each touch the vanilla faction
type files. This IS a real merge. Each mod modifies different parts of the
faction logic:
- Ariphaos: bugfixes (specific trigger corrections)
- Civil Wars: adds civil war triggers to faction demands
- [TNG] Faction Personality: adds personality-driven behavior modifiers
- VF: adds feudal-related faction demands (for totalitarian, supremacist, etc.)
- PD: adds planet-type habitability faction demands (traditionalist)

The good news: these are largely additive modifications to different blocks
within each faction type. Ariphaos fixes triggers. CW adds demands. TNG adds
AI weights. VF adds demands. They modify different sections of the same file.
A careful 3-way merge should work, but you need to read every file version
line by line. Budget 2-3 days for all 8 faction type files.

**Buildings (20+ files x 2-3 mods).** Plentiful Traditions, Ariphaos, and
Sort Those Buildings all touch vanilla building files. Sort Those Buildings
is supposed to load last among these (it redefines sort order). Ariphaos
loads first (bugfixes). Plentiful Traditions adds building triggers for its
tradition bonuses. The merge is:
1. Start with Ariphaos bugfix version
2. Apply Plentiful Traditions' added triggers
3. Apply Sort Those Buildings' sort modifications
This is tedious but mechanical. Budget 2 days.

**Section templates (corvette/destroyer/cruiser/battleship).** Ariphaos vs
NSC3. NSC3 loads after Ariphaos (confirmed). NSC3's section templates are
comprehensive rewrites, not patches. Check if NSC3 already incorporates
Ariphaos's bugfixes. If so, let NSC3 win via load order. If not, merge the
specific bugfixes into NSC3's version. Budget 1 day to verify + maybe 1 day
to merge.

**Ship behaviors (4 files x 3 mods).** Real Space Ships in Scaling, Ariphaos,
and Simple Fix for Ship Combat Behaviors. This needs careful analysis because
ship behaviors affect combat AI. RS:SiS changes ship positioning for scaled
systems. Ariphaos fixes bugs. Simple Fix adjusts combat engagement ranges.
Budget 1-2 days.

**Personalities (00_personalities.txt x 3 mods).** Ariphaos, Real Space New
Frontiers, and the EthicsSpawnBias-MoreAIPersonalities compatch. If More AI
Personalities adds personalities in its OWN file (not overriding vanilla),
then this is just Ariphaos bugfixes vs RS:NF additions vs compatch fixes.
Load order may solve it: Ariphaos first, then RS:NF, then compatch last.
Verify before merging.

**Sector types (00_core.txt + 01_other.txt x 3 mods).** Civil Wars, Sector
Shaping, and Larger Sectors. All three modify how sectors work. This is a
real merge but small scope -- only 2 files.

**Ascension perks (2 files x 2+ mods).** Need to identify exactly which mods.
The merge-tasks say "Worthy + Better + Expanded." Vanilla perk reworks are
the hardest because each mod might change the same perk differently. You have
to pick one rework per perk and reconcile the balance implications.

### HARD (weeks of work -- here be dragons)

**pop_social_classes.txt (BPU + Factional Politics).** The design says "Take
BPU's performance optimizations, apply FP's parliamentary additions on top.
Both mods change different parts of the same file." This is dangerously
optimistic. BPU doesn't just optimize -- it restructures pop category
processing logic. FP doesn't just add entries -- it modifies the calculation
flow for pop promotion/demotion to support parliamentary class mechanics.
These ARE NOT orthogonal changes to different parts of the file. They're both
rewriting the core pop-processing logic with different goals. You need to
read both versions in their entirety, understand the complete calculation
flow, and produce a version that achieves BPU's performance goals AND FP's
parliamentary mechanics simultaneously. Budget 1 full week minimum.

**WARNING:** BPU requires Casako's Framework (2466607238) which is NOT in
your approved mod stack. The load-rules-confirmed.md notes this. If BPU
doesn't function without Casako's Framework, your entire on_actions merge
strategy (which assumes BPU as the base) needs rethinking. **Resolve this
dependency question before doing ANY work on the on_actions or pop_categories
merges.**

**on_actions merge.** The design says "Start from BPU's optimized version.
Add all on_action entries from other mods." I have serious concerns about
this approach, addressed in Section 4 below.

**Component templates (20+ files x 2+ mods, FIOS).** This is where the
design has a critical blind spot. Addressed in Section 3 below.

**GVP 4.3 compatibility fix.** 77 civics, 95 government forms, multiple
authority types. The author themselves said "do not expect 4.3 update any
time soon." This isn't a bugfix -- it's a port. Every deprecated trigger,
modifier, and game concept reference across dozens of files needs updating.
If even one civic references a removed modifier, the whole mod hangs on load.
Budget 1-2 weeks, and expect to make balance mistakes that only emerge in
testing.

**Civil Wars 4.3 fix.** "Known broken, fork candidate." Fork means maintain.
Maintenance means every Stellaris update, you re-check compatibility. Do you
want to be maintaining a fork of Civil Wars indefinitely? Consider whether
the gameplay value justifies the ongoing cost.

**DPE 4.3 fix.** "Hangs at 80% loading." Could be one bad reference (1 hour
fix) or dozens of deprecated elements (1 week fix). Cannot estimate without
downloading the mod files. The fact that it hangs rather than erroring
suggests a loading loop rather than a single bad reference, which is harder
to diagnose.

---

## 2. What's Load-Order-Solvable vs Merge-Required

Going through the conflict list systematically:

### LOAD ORDER SOLVES IT (no megapatch file needed)

| Conflict | Resolution | Why |
|---|---|---|
| Real Space + Colour Out of Deep (40+ gfx files) | CoD loads after RS base | Same ecosystem, intentional override |
| NSC3 + Downscaled patch (12 juggernaut .gfx) | Downscaled patch loads after NSC3 | Designed to override |
| Star Wars portrait shared files (20+ files) | Any order | Identical files |
| At War shared tech/loc files | Carrier loads after Ship Sections | Same author, shared framework |
| SE Human 2 + Working Fix (12 portrait files) | Fix loads after base | Designed to override |
| Astronomical Emblem + Flags Merged (14 flag dirs) | Merged loads after Astronomical | Consolidated pack |
| UIOD + UIOD submods (No Backgrounds, etc.) | Submods load after UIOD | Standard submod ordering |
| Ariphaos + NSC3 section templates | NSC3 loads after Ariphaos | NSC3 is comprehensive rewrite |
| Ariphaos + NSC3 ship_sizes | NSC3 loads after Ariphaos | NSC3 redefines ship sizes |
| Expanded Gestalts + Nyblax shared files | Either order | Same author shared framework |
| description.txt / credits.txt / Changelog.txt | Any order | Non-gameplay files |
| setup_scenarios (5 galaxy sizes x 5 mods) | Needs specific analysis | Probably last-loaded mod's max empire count wins |
| Destiny Fallen + Darkness portraits | Either order | Same ecosystem |
| Plentiful Traditions + UIOD traditions view | UIOD loads after PT | UIOD overrides UI |
| PD + UIOD PD patch (pd_species_editors.gui) | UIOD PD patch loads after PD | Designed to override |
| UI Flag Mod + Flags Merged (flags/usage.txt) | Flags Merged after UI Flag | Consolidated |

### MERGE REQUIRED (megapatch needed)

| Conflict | Why Load Order Can't Solve | Difficulty |
|---|---|---|
| Faction types (8 files x 4 mods) | Each mod adds different functionality to same file; losing any one breaks features | MEDIUM |
| pop_social_classes (BPU + FP) | Both rewrite core logic | HARD |
| on_actions (BPU override + all handlers) | BPU replaces entire file | HARD (see Section 4) |
| Buildings (unity + pop_assembly x 3 mods) | Ariphaos fixes + PT additions + STB sort all needed | MEDIUM |
| Species traits (04 + 01_habitability x 3 mods) | Three mods adding different traits to same file | MEDIUM |
| Ship behaviors (4 files x 3 mods) | Each mod changes different parameters in same behavior block | MEDIUM |
| Strategic resources (Plentiful + Ariphaos) | Both modify vanilla resources; both changes needed | LOW |
| Sector types (2 files x 3 mods) | Three mods adding different sector functionality | LOW-MEDIUM |
| Districts (urban + arcology x 3 mods) | PT, PD Arcologies, Ariphaos all needed | MEDIUM |
| Personalities (1 file x 3 mods) | May be load-order-solvable; verify first | LOW-MEDIUM |
| topbar resources (4 mods) | All add different resources to topbar | LOW |
| Galaxy shapes (Ariphaos + No Clustered) | Both modify galaxy generation | LOW |
| Espionage events (Ariphaos + Meaningful Spy) | Both modify nemesis espionage events | MEDIUM |
| Hive encounter events (DPE + MEM) | Both provide same event file | MEDIUM |
| Component templates (20+ files, FIOS) | See Section 3 | HARD |
| Megastructures (7 files x 2-3 mods) | Ariphaos + StopAiSpam + Real Space all needed | MEDIUM |
| Deposits (planetary, 3 mods) | PD + RS:NF + RS:NF compat all needed | LOW-MEDIUM |

### NEEDS INVESTIGATION (can't determine without file analysis)

| Conflict | What to check |
|---|---|
| Scripted variables (PT + Ariphaos) | Are they modifying the same variables or different ones? |
| Random names (Ariphaos + Skyhawk + RS) | Probably additive sections, possibly load-order-solvable |
| Component templates (thrusters: RS:SiS + Ariphaos + ESC) | FIOS complicates everything (see Section 3) |
| Ethics (need to identify which 2 mods) | Could be simple or complex |
| Governments (usage.txt is SW portrait shared; faction_govts is SW portrait shared) | Verify identical |
| Prescripted countries (Distant Origin + 4EP3CP) | Probably additive, possibly load-order |

---

## 3. The FIOS Trap: Component Templates

This is the single biggest gap in the design documents.

The FIOS/LIOS research file clearly states:

> **FIOS (First In, Only Served) applies to: Component templates, Global ship
> designs, Script values, Events**

The design documents treat component_templates like every other `common/`
directory -- as if the megapatch can "override" them by loading last. **This
is backwards.** For FIOS content, loading LAST means your version is IGNORED.
The first mod to define a component_template entry wins.

### What This Breaks

The conflict report shows:

```
[3 mods] common/component_templates/00_utilities_thrusters.txt
  Real Space - Ships in Scaling (1915620447)
  Ariphaos Unofficial Patch (1995601384)
  ESC NEXT: Overwrites: Component Progression (2653789292)
```

The merge-tasks file lists "20+ component template files x 2 mods" as P0
critical. But the megapatch loads second-to-last. Under FIOS, its
component_template entries would be ignored in favor of the first mod's.

### The Real Problem

Your military stack relies on ESC NEXT, NSC3, and At War all providing
component_templates. Under FIOS, whoever loads FIRST wins each component ID.
The community solution is: ESC loads before NSC3 (confirmed rule). This means
ESC's components take priority over NSC3's where they share IDs.

But the megapatch loads AFTER all of them. If the megapatch provides a merged
`00_utilities_thrusters.txt`, it will be completely ignored because Real
Space Ships in Scaling already defined those entries first.

### What You Actually Have To Do

For FIOS directories, the megapatch CANNOT work by overriding files. Instead:

1. **Verify that mods use unique filenames.** If ESC uses `esc_components.txt`
   and At War uses `eac_components.txt`, there's no file-level FIOS conflict.
   The conflict only exists when mods provide the SAME filename.

2. **Where filenames collide, the FIRST loaded mod's version wins.** Your
   options are:
   - Modify the first-loaded mod's file directly (breaks mod updates)
   - Rename one mod's file to avoid the collision (hack, but works)
   - Create a compatibility patch that loads BEFORE both mods with the
     merged content (inversesof the megapatch approach)

3. **The `00_utilities_thrusters.txt` conflict is the worst case.** Three
   mods all provide vanilla-named component files. Only one version loads.
   You need to determine what each mod changes in this file, ensure the
   first-loaded version contains all changes, or restructure the files.

4. **Events are also FIOS.** The conflict report shows:
   ```
   [2 mods] events/nemesis_espionage_events.txt
     Ariphaos + Meaningful Spy Operations
   [2 mods] events/mem_hive_encounter.txt
     DPE + More Events Mod
   ```
   Ariphaos loads first. Its version of `nemesis_espionage_events.txt` will
   be the one used. Meaningful Spy's changes to that file will be silently
   ignored. Similarly, whoever loads first between DPE and MEM gets their
   `mem_hive_encounter.txt` used.

5. **Script values are FIOS.** If any mod provides script values with the
   same filename as another mod, first loaded wins.

### Recommended Approach

For component_templates specifically:
- Audit whether the conflicts are at the FILE level or ENTRY level
- If file-level (same filename, different content), the first-loaded version
  must be the merged version, which means patching the earliest-loading mod
- If entry-level (different filenames but same component IDs), FIOS means
  the first-loaded entry wins per ID -- the megapatch approach of "load last
  and override" simply does not work
- Consider creating a "pre-patch" mod that loads FIRST instead of last, for
  FIOS-affected directories only

This fundamentally challenges the "one megapatch loads second-to-last"
architecture for at least component_templates, global_ship_designs, events,
and script_values.

---

## 4. on_actions Reality Check

### The Good News

The on_actions map shows that most mods use their OWN uniquely-named
on_actions files (e.g., `apsr21_on_actions.txt`, `dpe_elections_events.txt`,
`plentiful_traditions_on_actions.txt`). The FIOS/LIOS research says:

> on_actions -- Cannot modify existing entries; new entries with the same name
> merge/append to existing blocks

This means on_actions entries are ADDITIVE. If Plentiful Traditions defines
`on_yearly_pulse` in `plentiful_traditions_on_actions.txt` and APSR defines
`on_yearly_pulse` in `apsr21_on_actions.txt`, BOTH fire. The engine merges
them.

This is critical: **most mods' on_actions DON'T conflict because they use
separate files.** The engine appends handlers from all files that define the
same on_action name.

### The BPU Problem

The design says "BPU does a full file override." If BPU overrides
`00_on_actions.txt` (the vanilla file) with its own optimized version, and
no other mod provides a file named `00_on_actions.txt`, then:

- BPU's version of `00_on_actions.txt` replaces vanilla
- All other mods' separately-named on_action files still load and append

**This means the "merge BPU + all handlers" approach described in the design
may be unnecessary.** If BPU only touches `00_on_actions.txt` and other mods
use their own files, the engine handles the merge for you.

### What Could Actually Break

The problem would be if:
1. BPU REMOVES handlers from `00_on_actions.txt` that other mods expect to
   exist (vanilla handlers that mods depend on)
2. BPU changes the SIGNATURE of an on_action (different scope, different
   parameters)
3. Other mods ALSO provide `00_on_actions.txt` (same filename collision)

Point 1 is the real risk. BPU is a "performance" mod. If it removes vanilla
on_action handlers it deems unnecessary, and another mod fires an event that
expects that handler to have already processed, you get silent failures.

### What I'd Actually Do

1. Download BPU's `00_on_actions.txt`
2. Diff it against vanilla `00_on_actions.txt`
3. List every handler BPU removes or modifies
4. Cross-reference against your 458 mods' event registrations
5. If BPU removes handlers that no mod in your stack uses, you're fine
6. If BPU removes handlers that mods DO use, add those back in a
   `zzz_megapatch_on_actions_restore.txt` file (the `zzz` prefix ensures
   it loads last alphabetically within the directory, and on_actions append)

This approach avoids rewriting BPU's entire file. You only restore what BPU
removed that your stack actually needs. Much less work, much less risk.

### The Casako's Framework Question

BPU requires Casako's Framework (2466607238). Your load-rules-confirmed.md
flags this: "which is NOT in our approved stack." If BPU literally doesn't
load without Casako's Framework, your on_actions strategy collapses. You need
to either:
- Add Casako's Framework to the stack
- Determine BPU works without it (test empirically)
- Drop BPU and use a different performance approach
- Fork BPU's on_actions optimizations into your megapatch directly

**This must be resolved before any on_actions work begins.**

---

## 5. What I'd Actually Do Differently

### Simplification 1: Don't merge what doesn't conflict

The design treats every "mod touches same directory" as a conflict. But
Stellaris loads ALL files in a directory, from all mods. Only SAME-FILENAME
files conflict. If ESC NEXT adds `esc_weapons.txt` and At War adds
`eac_weapons.txt`, both load. No conflict. No merge needed.

The conflict analysis already filters for same-filename collisions. But the
design documents (DESIGN.md, domain docs) talk about merging entire
categories ("Merge all edict definitions," "Consolidate all trait
definitions"). This is unnecessary work. Only merge files that actually
collide on filename.

**Exception:** If you want to do the "systems mesh" bridge work (wiring FP
into Social Decline, etc.), that's NEW code, not merge work. Call it what it
is and prioritize it separately.

### Simplification 2: Treat the "systems mesh" as Phase 2

The design documents mix two completely different kinds of work:

1. **Conflict resolution:** making 458 mods not break each other
2. **Synergy creation:** making mods enhance each other

These have wildly different risk profiles. Conflict resolution is necessary
for the modpack to function at all. Synergy creation is nice-to-have that
adds complexity and maintenance burden.

Ship it in two phases:
- **Phase 1: Conflict resolution megapatch.** Only merged files for actual
  conflicts. Only 4.3 fixes. Only load order documentation. This gets the
  modpack playable.
- **Phase 2: Systems mesh.** Bridge events, cross-mod wiring, AI
  personality-to-system integration. This makes it special.

The design currently treats both as one deliverable. That's a recipe for
never shipping.

### Simplification 3: Drop or defer the hardest 4.3 fixes

- **GVP 4.3 fix:** 77 civics + 95 governments is a massive port. If the
  GVP author doesn't update, consider whether you can use a smaller civic mod
  instead. Hydra's More Civics + Wiirlak Civics might cover 80% of the value
  at 10% of the maintenance cost.
- **Civil Wars fork:** Maintaining a fork is ongoing work. If CW breaks on
  4.3 and the author hasn't fixed it, seriously consider dropping it from the
  v1.0 modlist and revisiting later. Civil war functionality can partially
  come from DPE's political crisis events + Social Decline.
- **DPE 4.3 fix:** This one IS worth investigating because DPE is central to
  the political pipeline. But scope the fix first (download, grep for
  deprecated references) before committing.

### Simplification 4: The "auto-generate merged files" build system is overengineered

The BUILD.md describes a full build pipeline with analyze, generate, validate,
package phases. For a 458-mod megapatch maintained by one person, this is
overkill. The analysis tooling (conflict detection, ID collision reporting)
is invaluable. The generation tooling is premature.

Here's why: every "generated" file will need hand-tuning. The Clausewitz
format is not JSON -- it has inline scripted_effects, conditional blocks,
@variable references, and bracket-scoped logic that no sed/awk pipeline will
correctly merge. You'll generate something, hand-fix it, and then the
next "rebuild" will overwrite your fixes.

What I'd do instead:
- Keep the analysis scripts (conflict detection, ID collision, on_actions map)
- Hand-author every merged file (it's the only way to get it right)
- Use git to track changes (not a build system)
- When a mod updates, diff the new version against what you merged, and
  manually apply the delta

This is how every serious compatibility patch on the Workshop is maintained.
The Universal patches, Merger of Rules, even large patches like the old
StarNet-for-NSC3 -- they're all hand-maintained.

### Simplification 5: The bridge events should use flags and opinion modifiers, not custom scripted triggers

The domain docs describe elaborate "megapatch_stability_aggregator" scripted
triggers and "megapatch_civil_war_evaluator" systems. These are fragile.
Every mod update can change the internal state variables you're reading.

Use the simplest possible wiring:
- **Opinion modifiers** for diplomatic consequences (Stellaris already
  propagates these through all diplomatic calculations)
- **Country flags** for state tracking (set `megapatch_major_defeat` flag
  on war loss, check it in event triggers)
- **Modifier stacking** for economic effects (add modifiers, let the engine
  do the math)
- **on_action event handlers** for timing (fire bridge events in response to
  vanilla on_actions, not custom monthly pulse checks)

Avoid reading other mods' internal variables. Instead, hook into their
OUTPUTS (events they fire, flags they set, modifiers they apply) which are
part of their public API and more stable across updates.

---

## 6. Testing Strategy

### What Breaks First

In order of likelihood:

1. **Game hangs on loading (0-100% progress).** Caused by: malformed
   Clausewitz syntax, circular dependencies, missing required files. The
   loading percentage tells you what system is loading when it hangs. 80% is
   typically events/on_actions. 40-60% is common/ definitions.

2. **Error log spam on game start.** Caused by: missing localisation keys,
   broken cross-mod references (event X references non-existent flag Y),
   deprecated triggers/modifiers. The game runs but the log is full of
   warnings.

3. **Missing content in-game.** Caused by: FIOS/LIOS load order errors where
   the wrong version of a file won. The game runs, no errors, but ESC weapons
   don't appear on NSC3 ships, or AT War sections aren't available for
   certain ship classes.

4. **Late-game crashes.** Caused by: event chains that reference flags set by
   mods whose events were overridden, memory issues from too many concurrent
   event chains, AI calculation overflow from too many personality-modded
   decision weights.

5. **Desync in multiplayer.** Caused by: anything non-deterministic in the
   mod stack, including random weights that differ between hosts, event
   timing that depends on processing order, any mod that uses
   `random_list` without fixed seeds.

### The Error Log Pattern for Clausewitz Conflicts

The game log (usually `Documents/Paradox Interactive/Stellaris/logs/error.log`)
tells you almost everything:

```
[15:23:01][trigger.cpp] Error: Unknown trigger type "has_modifier"
  with value "old_modifier_name" in file "events/dpe_political_events.txt"
```
This means a mod references a modifier that doesn't exist (usually removed
in a game update). Fix: find the correct modifier name.

```
[15:23:01][effect.cpp] Error: Unknown effect type "add_modifier"
  with value "modifier_that_doesnt_exist" in file ...
```
Same pattern, different type. Modifier doesn't exist.

```
[15:23:01][pdx_database.cpp] Duplicate key "leader_trait_age_of_machines"
  in file "common/traits/synthetic_traits.txt"
```
ID collision. Two mods define the same trait. Under LIOS, the second wins.
Under FIOS, the first wins. Either way, one mod's version is silently lost.

```
[15:23:01][trigger.cpp] Error: scope mismatch - expected country,
  got planet in file "common/pop_faction_types/00_traditionalist.txt"
```
A scope error, usually from a mod targeting the wrong Stellaris version.
Common in 4.3 breakage.

```
(no error, but a particular feature doesn't work)
```
The worst case. Often a FIOS/LIOS issue where the wrong file won and the
expected content is missing. No error because the engine doesn't validate
cross-file references at load time.

### Testing Protocol for 458 Mods

**Phase 0: Load test (30 minutes)**
1. Enable all mods with correct load order
2. Start a new game
3. If it loads past 100%, you've cleared the worst obstacles
4. Check error.log immediately -- grep for `Error:` lines
5. Count errors. Zero is ideal. Under 50 is workable. Over 200 means
   something is fundamentally broken.

**Phase 1: Smoke test (2 hours)**
1. Start as a democratic federation builder (tests politics stack)
2. Console `observe` and run 10 years at fastest speed
3. Check: Do AI empires have proper governments? Do factions form? Do events
   fire?
4. Check: Are NSC3 ship classes available in the ship designer? Do ESC
   weapons appear?
5. Check: Do PD planet types appear when colonizing? Do Guilli's modifiers
   spawn?
6. Check error.log again -- new errors from runtime?

**Phase 2: Domain tests (1 day each)**
1. **Military:** Design ships with NSC3+ESC+At War components. Verify all
   sections available. Start a war. Check Fleets Win Wars scoring.
2. **Politics:** Play 50 years. Check FP parliamentary system. Check faction
   formation with GVP government. Check DPE events fire.
3. **Espionage:** Build spy networks. Run operations from all three spy mods.
   Check diplomatic consequences.
4. **Planet:** Colonize 10+ worlds of different PD types. Check Guilli's
   modifiers. Check Planetary Wonders availability. Check Sort Those
   Buildings sort order.
5. **Discovery:** Survey 50+ systems. Check anomaly variety (MEM + APSR).
   Start archaeological digs. Check Relic Trade.
6. **AI:** Observe 10 AI empires for 100 years. Check fleet composition,
   diplomacy behavior, planet management.

**Phase 3: Multiplayer test (half day)**
1. Host + second player connect
2. Play 20 years
3. Check: no desync notifications
4. Check: both players see same galaxy state
5. Check: events fire correctly for both players

**Phase 4: Endgame stress test (half day)**
1. Console `fast_forward 2400`
2. Check: crisis fires with appropriate strength (Dynamic Crisis Strength)
3. Check: no performance collapse (tick speed, especially on Steam Deck)
4. Check: galactic community still functional with 30+ AI empires voting

### What to Fix First

When you find errors, fix in this order:
1. Loading hangs/crashes (game unplayable)
2. Scope errors and missing references (features silently broken)
3. ID collisions in FIOS directories (wrong content winning)
4. ID collisions in LIOS directories (less critical, last loaded usually
   has the better version)
5. Missing localisation (cosmetic but visible)
6. Balance issues (only after everything loads and runs)

---

## 7. Specific Concerns Per Domain Document

### politics-diplomacy-intrigue.md

- **Wire 5 (vassal loyalty compounding)** says "the megapatch reads all three
  loyalty inputs and produces a single merged loyalty value." This requires
  understanding the internal variable names of three separate mods' loyalty
  systems. If any mod changes its variable names in an update, the bridge
  breaks silently. Use country_flags or opinion_modifiers instead of reading
  internal state.
- **Wire 7 (civil wars from everything)** is the most ambitious single wire.
  Five input systems, five civil war types. This is a small mod unto itself.
  Do not attempt in v1.0 of the megapatch.
- **Wire 8 (election integration)** overrides `leader_election_weight` game
  rule. This is a single merge point that everything feeds into. It's the
  right approach architecturally. But get the merge right -- a broken
  election weight crashes every election.

### military-fleet.md

- **Wire 2 (reactor resolution)** between NSC3 and ESC: the community
  solution is already the ESC "Disable NSC Reactors" submod. Use it. Don't
  invent a third approach. This is solved.
- **Wire 4 (Fleet Formation + Downscaled)** is a defines merge. Defines are
  key-value LIOS. The megapatch CAN override these by loading last. This is
  one of the cleanest merges in the entire project. Do this first as a
  confidence builder.

### planet-economy.md

- **Wire 3 (automation awareness)** says "Ultimate Automation's auto-build
  list extended with PD district types." Ultimate Automation likely uses
  generic building triggers, not hardcoded lists. Verify before writing
  patches. This might already work.
- The PD + Guilli's compat patch is already in your approved mod stack. Don't
  re-solve what the compat patch already solves.

### discovery-knowledge.md

- **Wire 1 (narrative coherence)** wants to prevent contradictory precursor
  stories. The simplest approach: Precursor Selection mod (approved) lets
  the player choose. Don't try to make mods' lore consistent via code -- it's
  a creative writing problem, not an engineering problem.

### ai-coherence.md

- **The entire "personality -> system investment" wiring** depends on being
  able to read More AI Personalities' personality type at runtime. If MAP
  stores personality as a standard Stellaris personality type (which it likely
  does), you can check it with `has_ai_personality = personality_name`. If it
  uses custom flags, you need to read their documentation.
- **AI fleet templates per personality** is good in theory but enormous in
  practice. You're talking about creating ship designs for every combination
  of personality x government x ship class x tech level. Start with 3
  personality archetypes (aggressive, balanced, defensive) and expand later.

### universal-patches.md

- The tiering (Tier 1/2/3) is well done. I agree with the priorities.
- The assessment that on_actions "CANNOT universally patch" may be overstated
  per my analysis in Section 4. The engine's append behavior handles most of
  the on_actions merging automatically. The only manual work is restoring
  anything BPU removes.

---

## 8. The Traits Problem: Deeper Than It Looks

The collision report shows 294 potential trait ID collisions across 1930
entries. The collisions_traits.txt shows the detailed diffs. Here's the
real issue:

The FIOS/LIOS research says:

> **Traits and Strategic Resources require FULL FILE REPLACEMENT (no per-entry
> override)**

This means traits don't work like most `common/` entries. There's no per-ID
FIOS or LIOS. The entire file from one mod replaces the entire file from
another when they share a filename.

The collision report shows three critical same-filename conflicts:
- `04_species_traits.txt` (Ariphaos + Additional Traits + PD) -- 3 mods
- `01_species_traits_habitability.txt` (Plentiful Perks + PD Patch + PD) -- 3 mods
- `00_species_traits_imperial.txt` (3 SW portrait mods) -- harmless, identical

For the first two: whichever mod loads last, its ENTIRE file replaces the
others. Not per-trait. The whole file. So if PD's version of
`04_species_traits.txt` adds habitability traits for exotic worlds, and
Additional Traits adds 30 new species traits in the same file, the
last-loaded one wins completely and the other's traits vanish.

This means you MUST produce merged versions of these two files in the
megapatch, containing all three mods' contributions, and the megapatch must
load after all three (which it does, being second-to-last).

For the leader trait collisions (Synthetic/Cybernetic/Erudite/Psionic Leader
Traits vs Ascension Leader Traits), these use DIFFERENT filenames
(`synthetic_traits.txt` vs `alt_synthetic_traits.txt`). Both files load.
The per-entry collision is at the ID level, and since traits use full file
replacement, the question is: does the engine merge entries from different
files in the same directory? The answer appears to be yes for traits -- entries
with the same ID across different files should follow LIOS per-entry (last
file alphabetically wins). But verify empirically. If `alt_synthetic_traits.txt`
loads after `synthetic_traits.txt` alphabetically, ALT's versions win for
colliding IDs, which is probably what you want.

---

## 9. Priority Ranking: What to Do First

1. **Resolve BPU's Casako's Framework dependency.** Everything depends on this.
2. **Investigate FIOS for component_templates.** Determine if your conflicts
   are file-level or entry-level. This changes the megapatch architecture.
3. **Merge the 8 faction type files.** High value, medium difficulty, no FIOS
   complications.
4. **Merge the 2 species trait files.** Required due to full-file-replacement
   behavior.
5. **Merge defines (Fleet Formation + Downscaled).** Clean LIOS merge, quick
   win, high confidence.
6. **Fix DPE 4.3 loading.** Download files, grep for deprecated references.
   Could be quick.
7. **Merge the 3 building files.** Tedious but straightforward.
8. **Set up the correct load order document.** Many "conflicts" disappear.
9. **Test with just correct load ordering and no megapatch.** See how much
   actually breaks vs how much was load-order-solvable.

Only after steps 1-9: tackle on_actions, pop_social_classes, GVP 4.3 port,
and bridge events.

---

## 10. Final Notes

The TENETS and SYSTEMS_MESH documents describe a beautiful vision. The gap
is between that vision and the mechanical reality of Clausewitz modding. The
engine is not designed for 458-mod integration. It's designed for "maybe 10
mods that mostly don't overlap."

The megapatch WILL work. But it will work because someone reads every
conflicting file line by line and hand-authors the merged versions. Not
because a build system auto-generates them. The analysis tooling (which you
already have) is the MVP. The generation tooling is a nice-to-have that
will never be as good as a human modder reading the code.

Ship a minimal conflict-resolution megapatch first. Play with it. Find what's
actually broken in gameplay. Then add bridge events for the connections that
matter most. The systems mesh will emerge iteratively, not from a design
document.
