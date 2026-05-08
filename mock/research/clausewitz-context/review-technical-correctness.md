I now have a thorough picture of all the documents and build tooling. Let me write up the findings.

---

# Megapatch Design Review — Findings

Reviewing: `/Users/orgrinrt/Dev/stellar-heritage/megapatch/DESIGN.md`, `SYSTEMS_MESH.md`, `BUILD.md`, `PATCH_FLOW.md`, `TENETS.md`, plus supporting build files.

---

## CRITICAL

---

### C-1: The core load-order assumption is wrong for component_templates, events, and global_ship_designs

**Files:** `DESIGN.md:26`, `BUILD.md:100-101`, `military-fleet.md:208`

DESIGN.md Principle 1 states flat: "Last-to-load wins. The megapatch must load after every other mod. It overwrites conflicting files with merged versions."

This is false for three of the most consequential file types in the military stack:

- **`component_templates/`** is FIOS (first-in-wins). The load-rules-web-research.txt documents this explicitly: "Component templates, Global ship designs, Script values, Events — FIRST mod to define an entry wins." The ESC NEXT section reinforces: "Component templates are FIOS — ESC's components that load first will take priority."
- **`events/`** is FIOS. The research file specifically notes: "The error log will make it look like it is LIOS, but this is definitely FIOS."
- **`global_ship_designs/`** is FIOS.

**What this breaks:**

1. **Component templates** (merge-tasks.md P0): 20+ files with ESC NEXT + At War/NSC3 conflicts. The megapatch cannot "win" by loading last — if a mod earlier in the load order already defined `component_template_foo`, the megapatch's version of that component is silently ignored. The DESIGN.md's strategy of loading second-to-last and overwriting is the wrong fix here. The megapatch needs to provide its merged component template files in a way that no mod earlier in the load order has already defined those same IDs. Since megapatch loads near the end, earlier mods define their IDs first, and FIOS means the megapatch loses.

2. **Events** (`megapatch_bridge_events.txt`): Every bridge event described in SYSTEMS_MESH.md and the domain docs lives in `events/`. If ANY of those bridge event IDs happen to collide with an earlier mod's event ID (even by accident), the megapatch's bridge event is silently dropped and the earlier mod's version fires instead. The design has no mechanism to detect or prevent this.

3. **`global_ship_designs/`** (`ai-coherence.md Wire 3`, `military-fleet.md:208`): The plan to create per-personality fleet templates in `common/global_ship_designs/megapatch_personality_fleets/` fails for any design ID already claimed by an earlier mod. The ESC Global Ship Designs submod loads before the megapatch; if it defines a design ID the megapatch also uses, ESC's version wins.

**The design must acknowledge this and pivot strategy for FIOS directories.** For component templates, the fix is not to load last and overwrite — it's to ensure IDs are unique (no collision with earlier mods) so FIOS never silently discards them. For events, bridge event IDs must be entirely novel IDs not present in any other mod. Neither the DESIGN.md, BUILD.md, nor SYSTEMS_MESH.md notes this constraint anywhere.

**Concrete impact on merge-tasks.md P0:** The component_templates P0 item cannot be resolved by the current "load last and win" approach. A different resolution is required: either ensure all megapatch component IDs are fresh namespace IDs (like `mp_at_war_esc_carrier_launcher`), or use scripted overrides rather than file overrides.

**Confidence: 100.** FIOS for these three directories is documented in the project's own research file.

---

### C-2: Traits require full file replacement, not per-entry override — the "pick best version" strategy is undefined

**Files:** `load-rules-confirmed.md:57`, `load-rules-web-research.txt:51-53`, `BUILD.md:105-106`

The load-rules-web-research.txt states: "Traits and Strategic Resources require FULL FILE REPLACEMENT (no per-entry override). No individual FIOS/LIOS — the entire file from one mod replaces another."

The merge-tasks.md (P2 row) says "92 DIFFERENT trait ID collisions" and the resolution is "Megapatch merge (pick best version per trait)." BUILD.md plans to "Consolidate: collect all unique IDs into one file" in `common/traits/megapatch_traits.txt`.

This approach is structurally correct — the megapatch produces ONE merged file. But "pick best version per trait" is not a defined algorithm anywhere in the design documents. For a collision, you have two different definitions of the same trait ID (different balance numbers, different modifiers, different AI weights). The design says you pick the "best" version but provides no decision criteria:

- Which version's numeric values do you keep?
- If Mod A gives trait_foo a +10% research bonus and Mod B gives it +15% with a -5% happiness penalty, which combination is "best"? A simple union of both effects would be double-buffed. Taking one and discarding the other loses the other mod author's intent.
- The design (merge-tasks.md) counts 183 ID collisions in the traits row (conflating the 92 different + some similar). Each one requires a judgment call with actual balance implications.

The automation path described in BUILD.md (auto-collect all unique IDs) only works for non-colliding IDs. For the 92 colliding IDs, BUILD.md says nothing about how the generator decides. This is hand-labeled as an auto-generated file but contains cases that require manual resolution with no workflow defined.

The universal-patches.md (line "Feasibility: HIGH for trait definitions") overestimates feasibility. It correctly notes that slot limits and compatibility are manual, but 92 substantive definition conflicts in the "auto" category is not accounted for.

**Confidence: 95.** The gap between "pick best version" and a workable decision algorithm is real and will halt progress when the first trait collision is encountered.

---

### C-3: BPU's 00_on_actions.txt merge is built on an unconfirmed dependency

**Files:** `load-rules-confirmed.md:93-98`, `DESIGN.md:75-79`, `universal-patches.md:28-39`

BPU (Better Performance & Utilities, 2475302050) is the foundation of the on_actions merge strategy. DESIGN.md section 1.2 says "Start from BPU's optimized version." But load-rules-confirmed.md line 93 explicitly notes: "BPU (2475302050) — Requires Casako's Framework (2466607238) — WE DON'T HAVE THIS."

The document lists three options but does not make a decision. The megapatch's most critical single file — the on_actions merge — is architecturally dependent on a mod that is not in the approved stack. If BPU cannot run without Casako's Framework, the entire BPU-as-base strategy collapses.

The on_actions merge is P0 in the universal-patches.md ("CRITICAL — most common conflict") and the most complex merge in the whole design. Leaving the BPU dependency unresolved means the foundation of the merge is uncertain. If Casako's Framework is not added to the approved stack, the on_actions merge strategy must be redesigned: start from vanilla on_actions, apply BPU's optimizations manually, or drop BPU's optimizations entirely.

**Confidence: 97.** The dependency is documented in the project's own confirmed rules and flagged as unresolved.

---

## WARNING

---

### W-1: Monthly pulse bridge events — performance model is not costed

**Files:** `SYSTEMS_MESH.md:247-261`, `politics-diplomacy-intrigue.md:99-132`, `ai-coherence.md:207-213`

The design proposes multiple bridge events running on `monthly_pulse`:
- Wire 2 (Faction State → Political Events): monthly pulse reading FP parliamentary state + GP political state
- Wire 3 (Faction Happiness → Social Decline): monthly pulse updating `megapatch_empire_stability` variable
- Wire 6 (Government Type → AI Behavior): monthly pulse evaluating GVP government categories
- Planet event bridge: monthly pulse checking planet event flags
- War consequence evaluator: monthly pulse checking war state

These fire once per empire per month. At 10 empires with 50 AI: that's 50 instances of each event per monthly tick, stacked on top of all the other mods' monthly events (More Events Mod, DPE, Social Decline, Planet Flavour, etc. all use monthly_pulse heavily).

The ai-coherence.md does note "The megapatch must not ADD excessive per-tick AI calculations" and "Personality-driven weights are evaluated ONCE per decision cycle, not every tick." But the bridge events described across the domain docs are monthly, not per-decision-cycle. There is no estimate of how many monthly events the megapatch adds, no total budget, and no comparison against what the Steam Deck can sustain.

The Steam Deck hosted server concern from the TENETS.md and memory context is real. The current community consensus (load-rules-web-research.txt) already recommends AI Game Performance Optimisation specifically to reduce calculation frequency. Adding 5-8 new monthly pulse events per empire on top of that goes in the opposite direction.

The design needs: a count of proposed monthly events, a per-event trigger cost estimate (checking scripted_triggers is cheap; reading `every_country` for federation members is not), and a decision about which bridges can be moved to event-driven (on_war_ended, on_action hooks) rather than monthly polling.

**Confidence: 90.** The omission of any performance costing for the bridge events is a real gap given the Steam Deck hosting requirement. Not theoretical — monthly polling of complex scripted_triggers at this scale has caused noticeable lag in similar modpacks.

---

### W-2: Faction state is not directly readable from monthly_pulse events the way the bridges assume

**Files:** `SYSTEMS_MESH.md:55`, `politics-diplomacy-intrigue.md:99-113`

Wire 2 describes: "Monitor FP's parliamentary state (coalition strength, deadlock timer) / Monitor GP's political state / When thresholds are crossed, fire custom events that DPE responds to."

The problem is that "FP's parliamentary state" and "GP's political state" are not native Clausewitz variables you can read as `THIS.coalition_strength`. Factional Politics (3322346400) implements parliamentary mechanics through a combination of pop_faction_types modifications, scripted_triggers, scripted_effects, and possibly flags/variables set on the country scope. To read this state from an external event:

1. You must know which specific variables/flags FP actually sets on the country scope. This requires downloading FP's source and auditing it — the design assumes this data is accessible without auditing the mod files.
2. If FP stores parliamentary state as internal scripted_triggers (e.g., `fp_parliament_is_deadlocked`), those triggers ARE accessible from other mods' events. This is the safe path.
3. If FP stores state in local variables within its own events or uses custom GUI/scripted_gui state, that state is NOT accessible externally.

The same applies to GP's political state. The design treats both as readable black boxes, but Clausewitz has no inter-mod API. The bridge can only read what FP/GP expose as country-scope flags or variables.

This is not necessarily fatal — many mods do expose their state via country flags. But the design presents the wiring as a solved implementation problem ("monthly pulse events that read FP/GP flags") when it is actually a research question that requires downloading both mods and auditing their exposed state surface. None of the domain docs note this prerequisite.

**Confidence: 88.** The Clausewitz engine limitation is real. The specific statement "wire FP faction state into Social Decline" is an implementation claim that needs verification before it can be trusted.

---

### W-3: The vassal loyalty merge requires modifying VF and Theta's output reads — may not be possible without forking both mods

**Files:** `politics-diplomacy-intrigue.md:156-167`, `SYSTEMS_MESH.md:104-114`

Wire 5 (Vassal Loyalty Compounding) describes producing a `composite_loyalty` value and then: "Both VF and Theta read the merged value for their mechanics." And: "Overlord's UI displays the merged value."

This cannot work without modifying VF and Theta's source files. Each mod reads its own loyalty variable/trigger for its own mechanics. For VF to "read the merged value" instead of its own, the megapatch must either:

1. Override VF's loyalty scripted_trigger with a new version that reads the composite — which means shipping modified VF files in the megapatch, effectively forking VF for loyalty calculation.
2. Convince VF and Theta authors to add an external hook, which requires upstream cooperation not mentioned in the design.

The phrase "Both VF and Theta read the merged value" implies these mods will transparently accept external loyalty values. That is not how Clausewitz modding works — each mod's internal trigger reads what it wrote. The megapatch can ADD composite loyalty as a parallel value, but making the existing mods' mechanics USE that composite requires overriding their files.

The design says "this is one of the trickiest merges" but doesn't call out that it requires file overrides of VF and Theta (not just additive content), which would break if either mod updates.

**Confidence: 85.** The Clausewitz mechanism for sharing state across mods has a clear ceiling. Reading another mod's internal state is possible via flags/variables; making another mod's existing code use YOUR value requires overriding their files.

---

### W-4: `on_espionage_operation_completed` — this on_action may not exist or behave as assumed

**Files:** `politics-diplomacy-intrigue.md:142-148`

Wire 4 says: "The megapatch adds `on_espionage_operation_completed` handlers that: Check if the target is a federation ally → add federation cohesion penalty..."

The Stellaris 4.x on_action for espionage completion is `on_espionage_operation_monthly` or scoped through the espionage event system, not a direct `on_espionage_operation_completed`. The vanilla 4.3 on_actions include `on_covert_op_monthly`, but the exact scope and available triggers differ between what was available in 3.x and 4.x (the Cetus update changed espionage events significantly, which is part of why the DPE mod broke). Additionally, checking "is the target a federation ally" from inside an operation completion handler requires knowing both `ROOT` (the spy empire) and `FROM`/`PREV` scope chains — which are not the same in every operation type across three different spy mods.

The design presents a clean `on_espionage_operation_completed` as if it's a known vanilla hook. The actual available on_action name and its scope variables need to be verified against the 4.3 vanilla on_actions file. Given that DPE broke specifically because on_action names changed in 4.3, this is a real risk.

**Confidence: 83.** The on_action name is stated without citation. Given the 4.3 changes that broke DPE's on_action references, the same risk applies here.

---

### W-5: The `clausewitz-extract.sh` parser mishandles multi-entry blocks (non-braced entries promoted to ID level)

**Files:** `/Users/orgrinrt/Dev/stellar-heritage/tools/clausewitz-extract.sh:64-70`

The awk parser at line 64 contains this branch:

```awk
# Non-@ single-line key = value (no braces)
else if (match(line, /^[[:space:]]*[a-zA-Z_][a-zA-Z_0-9.:-]*[[:space:]]*=/) && index(line, "{") == 0) {
    key = line
    sub(/[[:space:]]*=.*/, "", key)
    ...
    if (mode == "ids") print key
```

This matches ANY `key = value` line at depth 0 that doesn't open a brace. In Clausewitz files, properties like `species_class = "ALL"`, `enabled = yes`, `modifier = { ... }` (split across lines with the opening brace NOT on the same line as the key) all appear at depth 0 in some files. The parser will emit these as top-level IDs.

In trait files specifically, a common pattern is:
```
some_trait = {
    ...
}
modifier = {
    ...
}
```
When `modifier` appears at depth 0 (as a file-level property in some Clausewitz syntax), the parser emits `modifier` as an ID. In the ID collision detection, `modifier` will appear as a "collision" across every trait file, drowning out real collisions.

More concretely: the collision count of 92 (or 183 — merge-tasks.md gives two numbers for the same category, a minor inconsistency) may be inflated or have false positives from this parser behavior. The `megapatch-compare-ids.sh` `extract_block` function (line 46) has the same non-quote-aware substring match on the target ID that may also be fooled by partial ID matches.

Additionally, `extract_block` in megapatch-compare-ids.sh (line 56) counts braces using `split(clean, chars, "")` and iterating character by character — but `clean` was created with `sub(/#.*/, "", clean)` which is NOT quote-aware. A string literal containing `#` (e.g., `color = { r = 0.5 g = 0.5 b = 0.5 }` — no, but `name = "test # value"`) would incorrectly strip part of the content. The clausewitz-extract.sh was already fixed for quote-aware comment stripping; megapatch-compare-ids.sh was not updated to match.

**Confidence: 82.** The inconsistency between the two scripts (one quote-aware, one not) is a concrete defect visible in the code. The false-positive ID issue at depth-0 is a real parser gap.

---

### W-6: ESC NEXT loads before NSC3, but the design places megapatch after both — this conflicts with component_templates FIOS

**Files:** `load-rules-confirmed.md:27`, `DESIGN.md:3-6`, `military-fleet.md:62-78`

The confirmed load order has: `ESC NEXT → NSC3 → ... → megapatch`. For LIOS directories this is fine — megapatch loads last and wins. But for `component_templates/` (FIOS), ESC NEXT and NSC3 both define their components before the megapatch. When the megapatch tries to ship merged component templates, those IDs were already claimed by whichever of ESC/NSC3 loaded first.

The military-fleet.md Wire 3 (Carrier Stack) describes: "Verify `common/component_templates/` carrier entries from all three mods. Ensure At War's carrier section weapon_slots accept ESC carrier components." This implies the megapatch will add or modify component_templates entries. But it cannot override existing entries for FIOS directories — it can only add novel IDs. If the carrier component IDs from At War and ESC NEXT are already defined by those mods (loading before megapatch), the megapatch's component_templates file is irrelevant for those IDs.

The "20+ files × 2 mods" P0 component conflict in merge-tasks.md has no feasible solution under the current architecture for the FIOS case. The design needs to explicitly distinguish: for component ID collisions, the resolution is not "megapatch merges them" but rather "ensure no ID collision exists" (different IDs for each mod's components, with bridge scripted_triggers to make them interoperable).

**Confidence: 88.** This is a direct consequence of C-1, but specific enough to the military stack to call out separately given it's a P0 conflict.

---

### W-7: Viable Feudalism ↔ Federation Overhaul conflict is listed as "Megapatch merge" but the resolution path is not specified

**Files:** `load-rules-confirmed.md:53`, `DESIGN.md:204-208`, `politics-diplomacy-intrigue.md:47`

load-rules-confirmed.md notes: "Viable Feudalism | Federation Overhaul | Author says 'almost certainly incompatible' | Megapatch merge."

"Almost certainly incompatible" from the VF author is a strong statement. This typically means the mods touch the same core mechanic files (diplomatic_actions, subject_types, opinion_modifiers, or game_rules) in ways that cannot be trivially merged by appending entries. The DESIGN.md section 3.5 (Diplomacy Stack) mentions "Ensure Viable Feudalism's feudal mechanics don't conflict with Federation Overhaul" but provides no concrete analysis of what the actual conflict is.

Without downloading both mods and identifying which specific files conflict and why the author called it "almost certainly incompatible," this is an open risk being deferred with a label. The megapatch cannot auto-generate a fix for an incompatibility whose root cause is unknown. This is the kind of conflict that can take days to resolve correctly, and it sits at the core of the vassal/federation political pipeline that SYSTEMS_MESH.md treats as fully wired.

**Confidence: 85.** The author's incompatibility warning is a concrete data point that needs investigation before the design can claim this wire (Vassal → Diplomacy → GC pipeline) is achievable.

---

## INFO

---

### I-1: The tradition cap check counts categories correctly but may miss the additive-but-overcapped scenario

**Files:** `megapatch-analyze.sh:238-288`, `universal-patches.md:133-146`

The `cmd_tradition_count` function in megapatch-analyze.sh correctly distinguishes tradition categories from individual traditions and checks against the 64 cap. However: the 64 cap in UIOD applies to tradition CATEGORIES that are rendered in the tradition view. Some tradition mods add traditions to existing categories (vanilla Expansion tree, for example) rather than creating new categories. These do not consume category slots. The script counts categories from `common/tradition_categories/` which is correct, but the cap note in universal-patches.md says "Three mods adding 100+ trees TOTAL may exceed even 64 slots" — this conflates trees with categories vs individual traditions. If the tradition mods mostly add to existing categories, the 64 cap may not be the binding constraint.

**Confidence: 80.** The confusion between "trees" (categories) and "individual traditions" is present in the design docs and could lead to premature alarm or premature confidence, depending on which direction the conflation runs.

---

### I-2: The build pipeline has no incremental dependency tracking — mod updates trigger full regeneration

**Files:** `BUILD.md:149-155`

BUILD.md describes `megapatch-build.sh --changed <mod_id>` for incremental builds: "Only re-analyzes and re-generates files affected by the changed mod." But this requires knowing which generated files depend on which mods. The current analysis scripts scan all mods together and produce unified reports — there's no dependency graph. When a single mod updates, you can't easily determine which of the generated files (`merged_traits.txt`, `merged_civics.txt`, etc.) need to be rebuilt without re-running the full analysis.

This is an implementation detail but becomes a real issue when managing 458 mods with Workshop auto-updates: frequent mod updates mean frequent full rebuilds, and a full build (downloading, analyzing 330+ mods, comparing IDs) could easily take 20-30 minutes.

**Confidence: 80.** The gap between the described incremental build and the implemented analysis tools is visible. Low severity for the current phase but will matter in maintenance.

---

### I-3: Discovery → Diplomacy wire assumes `on_archaeological_site_completed` exists as a hookable on_action

**Files:** `discovery-knowledge.md:122-125`

Wire 3 in discovery-knowledge.md says: "Implementation: `events/megapatch_discovery_bridge.txt` with `on_archaeological_site_completed` handler."

The vanilla Stellaris 4.x on_action for archaeological site completion has changed across versions. In 4.3 (Cetus), the relevant on_action may be named differently or scoped differently than `on_archaeological_site_completed`. The DPE mod broke precisely because on_action names changed in 4.3 — the same risk applies here. The name `on_archaeological_site_completed` is stated as if known, without a citation to the vanilla on_actions file.

**Confidence: 80.** The 4.3 on_action name changes are established context (DPE is broken because of this). Using an on_action name without verifying it against the 4.3 vanilla file is repeating the same mistake.

---

### I-4: Missing from the design — species rights and citizenship types

**Files:** `SYSTEMS_MESH.md`, domain docs (none address this)

The 458-mod stack includes multiple mods affecting pop dynamics, species traits, and uplift. The design covers factions, planet economy, and primitives but has no mention of `common/species_rights/` conflicts. The megapatch-analyze.sh --summary scans this directory (line 64: `species_rights`). If multiple mods modify species citizenship or rights types, these files can conflict in ways that directly affect pop behavior and faction formation — which the political pipeline depends on. This system is not addressed anywhere in DESIGN.md, SYSTEMS_MESH.md, or any domain doc.

**Confidence: 80.** Species rights is a load-order-sensitive system (it's LIOS within common/) that touches the exact same pop/faction pipeline the design invests heavily in wiring. Its omission from the analysis is notable given how many mods in the approved stack touch pop behavior.

---

## Summary of Root Issues

1. **FIOS is the central blind spot.** The design is built on "load last, win everything." That is false for the three directories that matter most for the military stack (component_templates, global_ship_designs, events). Every bridge event in SYSTEMS_MESH.md lives in `events/` and is therefore vulnerable to FIOS. This must be addressed at the architecture level, not just for specific files.

2. **The bridge events assume inter-mod state reading that needs verification.** "Read FP's parliamentary state," "read VF's feudal loyalty" — these are implementation claims that require auditing each mod's exposed state surface. Some mods expose state via country flags (safely readable); others keep it internal. The design treats this as solved when it's a research question.

3. **The 92 trait collision resolution has no algorithm.** "Pick best version" is a placeholder, not a design. For 92 collisions requiring balance judgment, this needs a defined decision process before the build pipeline can implement it.

4. **The on_actions merge depends on an unapproved mod (Casako's Framework) with no resolution.** This is the single most important merge in the entire megapatch.

5. **The clausewitz-extract.sh parser is not consistent with megapatch-compare-ids.sh.** One is quote-aware for comment stripping; the other is not. False positives in the ID collision reports will erode trust in the analysis output.
