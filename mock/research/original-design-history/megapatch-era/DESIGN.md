# Megapatch Design

> The megapatch is TWO mods that control both ends of the load order:
>
> Load order: megapatch_fios → [458 mods] → megapatch → heritage
>
> - **megapatch_fios** loads FIRST — controls FIOS directories (component_templates,
>   events, global_ship_designs, script_values) where the first definition wins.
> - **megapatch** loads SECOND-TO-LAST — controls LIOS directories (everything else)
>   where the last definition wins.
> - **heritage** loads LAST, on top of the clean foundation the megapatch provides.
>
> Both mods are generated from the same build pipeline. Together they resolve
> ALL conflicts regardless of the engine's FIOS/LIOS per-directory behavior.
>
> The megapatch's job:
> 1. Resolve file conflicts between mods in our stack
> 2. Fix mods broken on Stellaris 4.3
> 3. Create synergies between mods that don't know about each other
> 4. Provide a clean, integrated platform for heritage to build on
>
> The megapatch contains ZERO heritage content. No bloodlines, no dynasties,
> no families. It is independently useful — someone could use our modlist +
> megapatch without heritage and still get a deeply integrated experience.
>
> Heritage then loads last and hooks into the already-integrated systems
> via on_actions, game_rules, and its own files. Heritage never needs to
> worry about mod X conflicting with mod Y — the megapatch already solved that.

---

## Principles

1. **Control both ends.** megapatch_fios loads first (FIOS dirs), megapatch loads second-to-last (LIOS dirs). Together they control every definition regardless of engine behavior.
2. **Merge, don't choose.** When two mods change the same file, merge both changes. Don't pick one and discard the other.
3. **Minimal footprint.** Only include files that NEED merging. Don't copy unchanged files.
4. **Documented per-file.** Every file in the megapatch has a comment header explaining which mods it merges and why.
5. **Versioned with the modlist.** Megapatch version tracks with `modlists.jsonl` — v1.0.0 megapatch goes with v1.0.0 modlist.

---

## Philosophy: Systems Mesh

The megapatch is not just a compatibility layer. It is a *design* layer that wires
the outputs of one mod system into the inputs of another, creating emergent gameplay
that no individual mod provides.

See **[SYSTEMS_MESH.md](SYSTEMS_MESH.md)** for the full interconnection map covering:
1. The Political Pipeline (government → factions → elections → GC → stability)
2. The Espionage-Diplomacy Axis (spy networks → intel → actions → alliances)
3. The Vassal Network (subjects → loyalty → specialization → contribution)
4. The Military Stack (fleets → wars → outcomes → political consequences)
5. The Planet-Economy Layer (planets → economy → population → factions)
6. The Discovery-Knowledge Arc (exploration → archaeology → relics → advantage)
7. The AI Coherence Layer (all AI mods driving behavior through modded systems)
8. Cross-section connections and DLC multiplier effects

See **[TENETS.md](TENETS.md)** for the design principles.

---

## Category 1: File Override Conflicts (CRITICAL)

These are mods that modify the same vanilla file. Last-loaded wins in Stellaris,
meaning without the megapatch, one mod's changes silently disappear.

### 1.1 `common/pop_categories/00_pop_social_classes.txt`

**Conflicting mods:**
- Factional Politics (3322346400) — adds parliamentary class mechanics
- Better Performance & Utilities (2475302050) — optimizes pop category processing

**Merge strategy:** Take BPU's performance optimizations, apply FP's parliamentary additions on top. Both mods change different parts of the same file — BPU modifies calculation logic, FP adds new category entries.

**Note:** Heritage mod may layer additional pop category modifiers on top in its own load-last position.

### 1.2 `common/on_actions/00_on_actions.txt`

**Conflicting mods:**
- Better Performance & Utilities (2475302050) — full file override for performance
- Every event mod registers handlers in on_actions

**Merge strategy:** Start from BPU's optimized version. Add all on_action entries from:
- More Events Mod, Dynamic Political Events, Fatal Foundations, etc.
- Any mod that registers on_action handlers
- (Heritage mod adds its own on_actions at its own load position — not merged here)

**This is the most complex merge** — on_actions is the central event bus. Must verify all event namespaces survive.

### 1.3 `common/game_rules/*.txt`

**Conflicting mods:**
- Stellar AI, More AI Personalities, Ruler Level System — all may modify game rules

**Merge strategy:** Megapatch merges game rule changes from:
- Stellar AI's AI behavior weights
- Ruler Level System's level-based weights
- More AI Personalities' personality-based weights
- All applied in sequence: vanilla → Stellar AI → RLS → More AI Personalities
- (Heritage adds its own game rule overrides at its own load position)

### 1.4 `common/defines/*.txt`

**Conflicting mods:**
- Fleet Formation Mod — ship_sizes defines for formation spacing
- Downscaled Ships — ship model scale defines
- Galaxy Tweak — galaxy generation defines
- Various performance mods — AI budget defines

**Merge strategy:** Defines are key-value. Multiple mods can set different keys without conflict. Only conflict if two mods set the SAME key to different values. Analysis harness will identify specific key conflicts.

---

## Category 2: Mods Broken on 4.3 (HIGH)

These mods need specific fixes to work on Stellaris 4.3 "Cetus."

### 2.1 Dynamic Political Events (1227620643)

**Issue:** Hangs at 80% loading on 4.3.
**Likely cause:** Reference to removed/renamed on_action, modifier, or trigger.
**Fix approach:**
1. Download mod files
2. Run `modlist-analyze.sh --conflicts` to identify which files reference missing elements
3. Grep for deprecated on_action names (`on_leader_removed`, `on_leader_added`, etc.)
4. Patch the specific file(s) in the megapatch
5. Preserve all event content — only fix the loading issue

### 2.2 Government Variety Pack (2806903835)

**Issue:** Author says "do not expect 4.3 update any time soon." 77 civics, 95 governments.
**Likely cause:** Multiple deprecated triggers/modifiers across many files.
**Fix approach:**
1. Download and analyze
2. This will be a larger patch — many files may reference deprecated elements
3. Focus on making it load without errors; balance can be adjusted later
4. Mark as `default_off` until fully tested

### 2.3 Empires Expanded (2717193796)

**Issue:** Broken on 4.3. Unofficial fix (3688050585) approved.
**Fix approach:** The unofficial fix IS the patch. Ensure it loads after the base mod.

### 2.4 Civil Wars (2434790082)

**Issue:** Known broken on 4.3, fork candidate.
**Fix approach:** Download, analyze breakage scope. May need significant rework.

### 2.5 Machines Robot Expansion Continued (3163759042)

**Issue:** Needs fork/patch for 4.3 rebalance.
**Fix approach:** Download from GitHub source, apply 4.3 compatibility fixes. Machine leader expansion benefits the entire modpack, not just heritage.

### 2.6 Expanded Stellaris Traditions (946222466)

**Issue:** May have been removed from Workshop. Tagged `4.3-untested`.
**Fix approach:** Verify availability first. If available, test on 4.3. If broken, patch or drop.

---

## Category 3: Cross-Mod Synergies (MEDIUM)

These aren't file conflicts — they're opportunities to make mods work BETTER together.

### 3.1 NSC3 + ESC NEXT + At War (Military Triangle)

**The problem:** Three major military overhauls that each add ship components, sections, and weapons. They work independently but don't synergize.

**Megapatch synergies:**
- Ensure At War's advanced ship sections work with NSC3's ship classes
- Ensure ESC NEXT's weapons are available for At War's carrier bays
- Unify reactor/power source system (ESC NEXT's Disable NSC Reactors submod handles part of this)
- Ensure AI ship designs use all three mod's components properly (Global Ship Designs submod)

### 3.2 Tradition Stack (More Traditions + Plentiful + Expanded ST)

**The problem:** Three tradition mods adding 100+ trees total. Possible ID collisions, UI overflow, AI pick weighting.

**Megapatch synergies:**
- Verify no tradition ID collisions (analysis harness `--traits` adapted for traditions)
- Ensure UIOD More Tradition Categories 64 can display all trees
- Adjust AI tradition pick weights so AI doesn't ignore modded traditions
- Verify adoption cost scaling is consistent across all three mods

### 3.3 Planet Stack (PD + Guilli's + Planet Flavour + WP Features + BPVR)

**The problem:** Multiple mods modifying planet views, planet modifiers, district types, building slots.

**Megapatch synergies:**
- Ensure Guilli's modifiers appear correctly alongside PD's planet types
- Ensure Planet Flavour events respect PD's custom planet classes
- Ensure BPVR's building slots work with PD's custom districts
- Verify Sort Those Buildings works with all planet mods' custom buildings
- Ensure Universal Zone Patch covers all zone interactions

### 3.4 AI Stack (Stellar AI + AI Game Perf + Dynamic AI Scaling + More AI Personalities)

**The problem:** Multiple AI behavior mods that may override the same AI decision files.

**Megapatch synergies:**
- Determine which AI files each mod touches
- Stellar AI for strategic behavior + AI Game Perf for computational efficiency
- Dynamic AI Scaling for difficulty progression
- More AI Personalities for variety
- Merge AI budget files if conflicting
- Ensure AI Ship Building Priority Fix works alongside Stellar AI's fleet logic

### 3.5 Diplomacy Stack (Federation Overhaul + NGS + Galactic/Factional Politics + Viable Feudalism)

**The problem:** Multiple diplomacy overhauls touching different aspects but potentially conflicting on diplomatic action definitions or opinion modifiers.

**Megapatch synergies:**
- Ensure Federation Overhaul's federation types work with NGS's galactic community
- Ensure Factional Politics' parliamentary system works alongside Galactic Politics
- Ensure Viable Feudalism's feudal mechanics don't conflict with Federation Overhaul
- Merge opinion modifier files if multiple mods add to the same file

### 3.6 Espionage Stack (Chris Covert + Whispering State + Meaningful Spy)

**The problem:** Three espionage mods that may define overlapping operation types.

**Megapatch synergies:**
- Verify no operation ID collisions
- Ensure spy network level gates are consistent
- Ensure all three mods' operations appear in the espionage UI
- (Heritage's espionage operations layer on top at heritage load position)

---

## Category 4: DLC-Mod Synergies (MEDIUM)

### 4.1 Galactic Paragons + Ruler Level System

Both modify the leader progression system. Ensure:
- RLS's ruler levels work with Paragons' veteran classes
- Verify Paragons' council + CPD's custom positions + RLS's level system coexist
- No trait slot overflow from combining Paragons + RLS traits

### 4.2 Nemesis Espionage + Espionage Mods

Ensure the three espionage mods' operations work alongside vanilla Nemesis operations without operation type overflow. (Heritage adds its own operations at its load position.)

### 4.3 Overlord Vassals + Theta Vassals + Viable Feudalism

Three layers of vassal mechanics:
- Overlord (DLC): specialist subjects, holdings, agreements
- Theta Vassals: expanded vassal interactions
- Viable Feudalism: feudal authority and loyalty
Ensure all three layers compose correctly. Holdings, loyalty, agreement terms should stack, not conflict. (Heritage adds its own vassal mechanics at its load position.)

### 4.4 Federations GC + NGS + Hydra's More GC Resolutions + Forgotten History GC

Multiple mods adding galactic community content:
- NGS overhauls the base system
- Forgotten History adds GC resolutions
- Hydra adds more resolutions
- Forgotten History adds GC resolutions

Ensure all resolution categories and individual resolutions coexist. Verify the GC UI can handle the expanded resolution count.

---

## Category 5: Base Game Fixes (LOW)

Things that are technically broken in vanilla or that our mod stack makes worse.

### 5.1 Ariphaos + Stellaris General Fixes + Dux Overlap

Both Ariphaos and General Fixes override vanilla files for bug fixes. Dux fixes are more targeted. Ensure:
- No three-way conflict on the same vanilla file
- If Ariphaos and General Fixes both fix the same bug differently, pick the better fix
- Dux fixes that are redundant with the above can be safely ignored (harmless double-fix)

### 5.2 Load Screen Consolidation

Multiple loading screen mods (Backgrounds+ Combined, Cinematic, More Vanilla, etc.). Not a code conflict but:
- Ensure no duplicate image filenames
- Curate the total count (too many loading screens = memory bloat)

---

## Implementation Workflow

1. **Download all mod files** (in progress)
2. **Run `modlist-analyze.sh --all`** to get the full conflict/override map
3. **Prioritize by category** (file conflicts first, then 4.3 fixes, then synergies)
4. **Per-file merge process:**
   a. Read vanilla file
   b. Read each mod's version of the file
   c. Identify what each mod changes vs vanilla
   d. Merge all changes into one file
   e. Add header comment documenting the merge
   f. Place in `megapatch/common/` (or appropriate directory)
5. **Test in-game** — load megapatch last, verify no errors
6. **Version and commit** — megapatch version tracks modlist version

---

## File Structure

```
megapatch/
  DESIGN.md                          — this file
  descriptor.mod                     — Stellaris mod descriptor (loads last)
  common/
    on_actions/
      00_on_actions.txt              — merged BPU + all event registrations
    pop_categories/
      00_pop_social_classes.txt      — merged FP + BPU
    game_rules/
      megapatch_rules.txt            — merged game rule overrides (AI + RLS + personalities)
    defines/
      megapatch_defines.txt          — merged formation + scaling + perf defines
  events/
    megapatch_bridge_events.txt     — cross-mod bridge events (synergy wiring)
  localisation/
    english/
      megapatch_l_english.yml
```
