Now I have a complete picture. Let me produce the full analysis.

---

# Megapatch Domain Wiring Review

Reviewing six domain specification files against the Clausewitz engine rules established in `load-rules-web-research.txt` and the actual collision/conflict data in `collisions_traits.txt` and `conflicts.txt`.

---

## Engine Constraints That Apply to Every Wire

Before scoring individual wires, three hard constraints from the build data shape everything:

**Traits use FULL FILE REPLACEMENT.** The collisions_traits.txt file documents the consequence: Ascension Leader Traits 4.1 and vanilla-adjacent mods share the same trait IDs with different stat values. Whichever mod's file loads last wins entirely. There is no per-entry override. Any wire that assumes "additive" behavior for trait-adjacent files must account for this.

**Civil Wars touches 11+ pop_faction_types files directly.** The conflicts.txt file shows Civil Wars (2434790082) appearing in conflicts for `00_traditionalist.txt`, `00_totalitarian.txt`, `00_supremacist.txt`, `00_progressive.txt`, `00_imperialist.txt`, `00_xenoist.txt`, `00_technologist.txt`, `00_prosperity.txt`, `00_isolationist.txt`, and also `wrb_scripted_triggers.txt` and `fune_diplo_scripted_triggers.txt` (the last two shared with Viable Feudalism). This mod does heavy file replacement, not additive injection. It is not architected to accept external trigger inputs.

**`common/ai_espionage/` is a real directory.** `conflicts.txt` line 1474 shows `common/ai_espionage/spynetworks/00_base_strategies.txt` conflicted between Ariphaos and Chris Covert Operations. The ai_espionage system does exist as data-driven files in Clausewitz, which partially validates Wire 6 in ai-coherence. However the scope of what is controllable there versus what is hardcoded in C++ is still a key question (addressed below).

---

## politics-diplomacy-intrigue.md

### Wire 1: Government Type to Faction Behavior
Rating: **CONCRETE**

The mechanism is accurate. `common/pop_faction_types/` files accept attraction conditions using triggers like `has_civic`, `has_authority`, and `has_government`. Adding new blocks with GVP-specific civic checks that modify `attraction` weight is a legitimate additive operation — each faction type file can have multiple attraction modifier blocks. The spec correctly identifies the file path and notes these are additive.

One caveat: the conflicts.txt data shows `common/pop_faction_types/00_traditionalist.txt` is already a 4-way conflict (Ariphaos, Civil Wars, TNG Faction Personality, PD). The megapatch must own these files as merged versions, not just add a new file alongside them. The spec does acknowledge this in the Load Order Sensitivity table, but the Wire 1 implementation description doesn't make this dependency explicit. Low severity.

### Wire 2: Faction State to Political Events
Rating: **VAGUE**

The spec says DPE fires events based on "vanilla faction state" and the bridge monitors "FP's parliamentary state (coalition strength, deadlock timer)." But the actual mechanism is underspecified in a critical way.

DPE fires its event chains from its own triggers inside its own event files. Since events are FIOS (first in, only served), DPE's event files will win over any same-ID bridge attempt. The megapatch can only influence DPE's chains in two ways: (1) set global flags or scripted variables that DPE's trigger conditions happen to check, or (2) add new event entries via on_actions handlers that fire alongside DPE's events.

The spec proposes `events/megapatch_political_bridge.txt` with monthly pulse events that "set DPE-compatible triggers." What exactly is a "DPE-compatible trigger"? Does DPE actually expose named flags it reads from external sources? If DPE uses internal country flags it sets itself, the bridge cannot inject into those checks — it would need to set those exact same flag names. This requires downloading DPE's event files and auditing what flag names it reads. The spec does not do this. Until that audit happens, this wire cannot be implemented.

Confidence this is vague: **88**.

### Wire 3: Faction Happiness to Social Decline
Rating: **VAGUE with a concrete core**

The scripted_triggers approach (`common/scripted_triggers/megapatch_stability.txt` defining `megapatch_empire_stability`) is a real Clausewitz pattern. Scripted triggers can aggregate multiple condition checks and return a boolean or be used in value calculations. A scripted_variable updated by a monthly pulse is also a legitimate technique.

The VAGUE part: "Social Decline reads this instead of (or in addition to) vanilla stability." Does Social Decline actually read vanilla stability through a named scripted trigger you can replace? Or does it read the hardcoded `stability` country value directly via something like `stability < 30`? If it reads the raw `stability` value, the megapatch cannot intercept that without overriding Social Decline's event files directly — which would be a full file replacement, not additive. This needs a Social Decline file audit before implementation can proceed.

The monthly pulse performance cost here is real but manageable for a single aggregator event. The spec correctly notes this as a monthly pulse.

### Wire 4: Espionage to Diplomatic Consequences
Rating: **CONCRETE**

`on_espionage_operation_completed` is a real vanilla on_action in Clausewitz. Adding a handler for it in `common/on_actions/` is the standard way to fire effects after operation completion. Opinion modifiers (`add_opinion`) are global and all diplomacy mods read them. Federation cohesion is a country variable that can be modified via scripted effects. Feudal loyalty in VF is likely a scripted_variable or country modifier, both of which can be set from external event effects.

The implementation path (on_actions handler checking relationship type, then applying modifiers) is sound and achievable without touching any of the three spy mods' files.

One flag: the conflicts.txt shows `common/scripted_effects/pirate_fleet_effects.txt` is contested between Ariphaos and Meaningful Spy Operations. This suggests Meaningful Spy does some file replacement in scripted_effects. If it also places handlers in on_actions for its own operation completion logic, the megapatch on_actions merge must incorporate those.

### Wire 5: Vassal Loyalty Compounding
Rating: **WISHFUL**

This is the most optimistic wire in the document. The spec says the megapatch "reads all three loyalty inputs" and "produces a single merged loyalty value" and "both VF and Theta read the merged value."

That last sentence is the problem. VF and Theta do not "read the merged value" unless their internal loyalty trigger checks are rewritten to call a megapatch scripted trigger instead of their own variables. That requires overriding VF's and Theta's internal files, which means full file replacements for those mods' loyalty calculation logic.

The conflicts.txt shows VF (`common/scripted_triggers/wrb_scripted_triggers.txt` and `fune_diplo_scripted_triggers.txt`) and Civil Wars sharing those exact trigger files — meaning VF already does internal trigger replacement. If the megapatch wants to intercept VF's loyalty checks, it must replace the same files, incorporating VF's logic plus the new merged calculation. This is feasible but it is not the lightweight "composite layer on top" the spec implies. It is effectively rewriting both mods' loyalty systems in the megapatch. The spec describes an output that requires invasive changes but frames it as additive wiring. Implementation will be significantly harder than described.

Confidence this is wishful: **85**.

### Wire 6: Government Type to AI Behavior
Rating: **CONCRETE with a caveat**

`common/ai_budget/` files are data-driven and accept conditional modifiers with trigger blocks. Adding government-type-conditional weight multipliers is a standard technique. `common/personalities/` files similarly accept modifier blocks. Both files are additive in principle — adding new files with new entries doesn't require replacing existing ones, as long as you use unique file names and don't duplicate existing entry IDs.

The caveat: `common/personalities/megapatch_govt_personalities.txt` is described as "creating government-conditional personality overlays that modify the base personality from More AI Personalities." Personalities in Clausewitz are not layered — an empire has one personality. Overlaying a second personality on top of the first is not a native concept. The actual mechanism would be adding `weight_modifier` blocks inside the personality definitions that check government type, making the base personality more or less likely to be assigned. This is achievable, but the "overlay" framing in the spec implies runtime stacking that doesn't exist. The megapatch would instead need to modify personality assignment weights, not apply runtime overlays.

### Wire 7: Civil Wars from Everything
Rating: **WISHFUL**

This is the most problematic wire in the entire document. The conflicts.txt data makes the problem concrete and unavoidable.

Civil Wars directly owns and replaces nine `common/pop_faction_types/00_*.txt` files. These are not additive additions — CW rewrites them wholesale. CW also owns `common/scripted_triggers/wrb_scripted_triggers.txt` and `fune_diplo_scripted_triggers.txt`.

The spec says: "the megapatch creates enriched civil war triggers: Civil Wars' base trigger conditions expanded to check all five inputs." But how? The trigger conditions that fire Civil Wars' civil war events live inside Civil Wars' event files (FIOS) and its pop_faction_types files. The megapatch cannot inject new trigger conditions into CW's existing event checks without:

1. Replacing CW's event files with modified versions (requiring ongoing maintenance every time CW updates)
2. Or identifying named flags that CW reads externally (which requires auditing CW's event files to confirm such flags exist)

The spec's fallback — "If not, the megapatch overrides CW's trigger file with the enriched version" — acknowledges this reality but frames it as a fallback. Given what the conflict data shows, this is not a fallback, it is the only path.

Furthermore, the "five distinct civil war types based on which input is dominant" claim assumes CW exposes a way to select civil war type from an external variable. Civil Wars likely fires specific event chains internally based on its own faction state reads. Without CW exposing a `megapatch_civil_war_type` variable that its events branch on, five distinct types cannot be driven from outside. This requires CW source inspection before any implementation claim can be made.

The wire direction is correct and desirable. The implementation description is not achievable as written.

Confidence this is wishful: **92**.

### Wire 8: Election Cycle Integration
Rating: **CONCRETE with a known file collision**

`game_rules/` in Clausewitz accepts a `leader_election_weight` block that is additive — multiple files can add weight modifier conditions and they stack. This is one of the cleaner Clausewitz extension points. Adding GVP civic checks, Gender Politics civic checks, and FP parliamentary state checks all as separate weight modifiers in a single megapatch file is a real pattern.

The spec correctly identifies this as a "critical game_rules merge." The Merger of Rules mod (which sits at the absolute bottom of load order) exists specifically for this type of conflict. The megapatch's game_rules file should be verified against Merger of Rules to ensure they don't fight.

One concrete issue: "Ruler Level System level bonuses" — Ruler Level System is not in the mod list tables in this file. Confirm RLS is an approved mod before building the election weight formula around it.

---

## military-fleet.md

### Wire 1: Ship Design Unification
Rating: **CONCRETE**

Section templates (`common/section_templates/`) reference `ship_size` and accept weapon slot definitions. Adding At War section entries that name NSC3 ship_size IDs is additive — new files, new entries. Verifying ESC weapon component IDs appear in At War's slot filters is an audit task, not a novel implementation. The spec correctly identifies the file paths and the audit requirement.

The FIOS rule for component_templates matters here: ESC components load before NSC3 (load order spec confirms ESC loads before NSC3), so ESC components that define the same slot type as NSC3 will win. This is the intended behavior.

### Wire 2: Reactor/Power Resolution
Rating: **CONCRETE**

The spec honestly presents three options and defers the decision to file analysis. This is the correct approach. The "Disable NSC Reactors" submod exists precisely because the community already identified this conflict. The megapatch needs to make a choice and document it; the spec correctly says that decision is pending file download.

### Wire 3: Carrier Stack
Rating: **CONCRETE**

Component templates (FIOS), section weapon slots, and strike_craft entries are all data-driven files with clear composition rules. At War carrier bay slots accepting ESC component IDs is a trigger condition check in the section template. SCD strike_craft entries being launched by ESC's launch system depends on whether ESC's launch logic references a generic `strike_craft` category or specific IDs — this is an audit item but the path is real.

### Wire 4: Fleet Formation + Downscaled Ships
Rating: **CONCRETE**

`common/defines/` entries are LIOS with full-block replacement required. The spec correctly identifies the need for a merged defines file. The formula "formation distance = Fleet Formation base multiplied by Downscaled scale factor" is speculative in exact values but the approach (single merged file owning both sets of values) is the right one.

### Wire 5: War Outcomes to Political System
Rating: **VAGUE**

`on_war_ended` is a real vanilla on_action. Adding handlers is additive via on_actions. Setting FP flags, Social Decline variables, and NGS diplomatic weight modifiers from those handlers is achievable.

The vague part: the spec says the bridge reads "Fleets Win Wars war_score." Does FWW expose a named scripted_variable or country modifier that encodes the war outcome quality? Or does it just modify internal war score calculations? If FWW works by adjusting the vanilla `war_score` calculation and doesn't set any named output variable, the bridge event can read the result indirectly through the vanilla `last_war` scope. But if FWW's "decisive battle" system works through internal event triggers with no external-readable state, the bridge cannot distinguish a decisive FWW victory from a vanilla total war victory.

This needs a FWW file audit to determine what output state it produces.

### Wire 6: Dynamic Crisis Strength + Full Fleet Stack
Rating: **CONCRETE**

The spec correctly hedges: "If DCS already reads actual fleet_power trigger (likely), this may work automatically." `fleet_power` is a vanilla trigger that reads real computed fleet strength, and if DCS uses it rather than hardcoded estimates, no patch is needed. This is exactly the kind of item the analysis harness should verify. The wire is honest about the uncertainty and defines the correct resolution path.

### Wire 7: AI Fleet Intelligence
Rating: **VAGUE**

The `common/ai_budget/` approach for personality-conditional fleet investment is real. The `common/global_ship_designs/` path for per-personality fleet templates is also real and supported by FIOS (first-loaded templates win, so the megapatch templates should load after ESC's Global Ship Designs to be able to override specific templates while leaving others alone — but this creates a load order dependency the spec doesn't address).

The deeper vague claim: "Schemer personality — lighter fleets, more investment in spy networks." The ai_budget system controls resource allocation across economic categories. "Spy networks" in Clausewitz is funded via the `intel` category or similar. Whether the espionage mods expose their investment logic to the ai_budget system or handle spy hiring through their own event loops is unknown. If spy investment in Chris Covert/Whispering State/Meaningful Spy is driven by their own periodic events rather than the vanilla ai_budget framework, the megapatch ai_budget entries won't control it. See also AI Coherence Wire 6 below.

---

## planet-economy.md

### Wire 1: Planet Type to Building Availability
Rating: **CONCRETE**

Buildings in `common/buildings/` use `potential` trigger blocks. Adding a new file with building entries that include `has_planet_flag` or `is_planet_class` checks for PD planet types is additive. The spec correctly notes this doesn't override Planetary Wonders' files. This is achievable and the file path is correct.

### Wire 2: Zone/District Ecosystem Integration
Rating: **VAGUE**

The spec says "BPVR's expanded slots display PD's custom districts." BPVR's expanded building slot logic is likely in `common/defines/` or UI files — defines are LIOS and the megapatch must own those. If BPVR uses planet type conditions to determine slot counts, adding PD planet type entries is additive. If BPVR sets a global slot count via defines, there's no per-type override mechanism without a defines patch.

"More Specialized Zones' specializations apply to PD planet types" — zone specializations are in `common/colony_types/`. If the colony type entries use generic `planet_class` triggers that default to all classes, PD types are included automatically. If they enumerate specific vanilla planet classes, PD types need explicit additions. Unknown without file audit.

"Verify completeness" is the correct resolution posture but the wire presents this as already solved by existing patches. The Sort Those Buildings PD patches handle building sorting, not zone composition. These are different problems.

### Wire 3: Automation Awareness
Rating: **VAGUE**

The spec honestly says "this may require overriding automation mod files" or "if automation mods use generic district triggers, they may already work." This is not a wire specification — it is a statement that the wire might or might not need to exist. The automation mods' internal logic must be audited before any wiring can be designed. As written, this cannot be implemented.

### Wire 4: Planet Events to Political Consequences
Rating: **CONCRETE**

Adding bridge events to `events/megapatch_planet_bridge.txt` that fire on planet event flags and then modify country-level stability and faction happiness is a real pattern. Planet event flags are set by the source mods' events. Reading `has_planet_flag` and then firing `country_event` to apply consequences is standard Clausewitz scripting. The on_actions for "planet event completed" hooks may be needed depending on how the source mods fire their events, but this is an audit item, not a structural impossibility.

### Wire 5: Pre-FTL Integration Stack
Rating: **CONCRETE**

`common/observation_station_missions/` entries use trigger conditions for planet class and modifier checks. Adding entries that check for PD planet classes and Guilli's modifier flags to modify primitive development is additive. Pre-FTL City Sets matching PD planet types is a visual/initializer concern handled at spawn time. This is achievable.

### Wire 6: Economy to Everything
Rating: **VAGUE bordering on WISHFUL**

The spec says this "is mostly vanilla behavior amplified" and "likely works naturally through vanilla resource/modifier chains." That may be true for basic income flowing into fleet budgets, but the specific claims deserve scrutiny:

"Modded economic output feeds into Social Decline's stability calculations correctly" — Social Decline reads stability or specific modifiers. If PD's exotic districts produce resources that then produce the same base `stability` country value, this flows naturally. But if PD produces novel resources that don't convert to vanilla stability modifiers, Social Decline won't see them. Unknown without file audit.

"The megapatch adds bridge triggers where modded content bypasses vanilla calculation paths" — this is not a specification of a wire, it is a placeholder for future work. There is no mechanism described. This wire should be marked as a design intent pending investigation, not a wiring specification.

---

## discovery-knowledge.md

### Wire 1: Narrative Coherence Across Event Mods
Rating: **VAGUE**

The mutual exclusion approach (`has_planet_flag` checks preventing double-dig on the same world) is concrete and achievable. Adding `megapatch_dig_exclusion.txt` with `potential` conditions is real.

The "shared precursor history flags" part is vague. The spec says "If MEM's precursor chain fires first, ASP/FH/PSP chains reference it." For this to work, MEM's precursor events must set a specific named flag when they complete, and the megapatch bridge must know that flag name and translate it to flags that ASP/FH/PSP check. This requires auditing all four mods' precursor event chains for their internal flag names. The approach is sound if those flags are identifiable and consistent. The spec does not do this audit, so the mechanism is incompletely specified.

Additionally, event files are FIOS. If any of MEM, FH, ASP, or PSP define the same event ID for their precursor entry events, only the first-loaded fires. The spec doesn't address this collision risk.

### Wire 2: Technology Tree Unification
Rating: **CONCRETE**

Tech definitions in `common/technology/` are LIOS-per-entry (each tech ID wins from the last-loaded file). Adding a merged tech file with all unique tech IDs from all mods is the correct approach. The AI research weight adjustment via `common/ai_budget/` is data-driven. Dynamic Technology Tree renders based on the `tier` and `category` fields in tech definitions; as long as those are populated consistently, the visualization follows. The spec correctly identifies the audit requirement (tech ID collision check) and the right file paths.

### Wire 3: Discovery to Diplomacy
Rating: **CONCRETE**

`on_archaeological_site_completed` is a real vanilla on_action. Handlers in `common/on_actions/` that fire after archaeological completion are additive. Adding opinion modifiers and country flags is standard. Relic Trade making newly discovered relics tradeable depends on whether Relic Trade reads a flag or just checks `has_relic` — if the latter, it works automatically when the relic is granted by the dig completion event.

The "GC can debate ownership" claim requires NGS to have a resolution proposal trigger that the megapatch can activate. If NGS provides a way to trigger proposals from events, this is achievable. If GC proposals are only player-initiated, the bridge cannot force it. Audit NGS's proposal mechanics before committing to this.

### Wire 4: Real Space to Everything Physical
Rating: **CONCRETE**

This wire is correctly scoped as primarily a verification task with patching in `solar_system_initializers/` if needed. The existing PD-RS compat patch handles the most critical piece. The spec is honest that this is verification, not novel wiring.

One conflict noted: conflicts.txt shows `common/solar_system_initializers/special_system_initializers.txt` is a 3-way conflict between Ariphaos, All Systems Spawn, and Real Space 4.0. The megapatch must own this merged file.

### Wire 5: Pre-FTL to Planet to Politics
Rating: **CONCRETE**

Bridge events firing after uplift events complete, reading planet class and Guilli's modifiers, then setting faction-visible country modifiers is a standard Clausewitz pattern. The on_action for uplift completion exists in vanilla. The chain from there to FP faction dynamics and Social Decline variables follows the same pattern as other working bridge events in the spec.

---

## ai-coherence.md

### Wire 1: Personality to System Investment
Rating: **VAGUE**

The budget allocation approach in `common/ai_budget/megapatch_personality_weights.txt` is real for resource categories that exist in vanilla's ai_budget framework: minerals, energy, alloys, consumer goods, fleet capacity, etc. Adding personality-conditional multipliers for those categories is achievable.

The critical unverified claim: several "systems" this wire routes to are not vanilla ai_budget categories. "Espionage investment" is not a vanilla economic category in the ai_budget framework. "Archaeology investment" is not a vanilla ai_budget category. "Diplomacy investment" exists partly (influence spending) but federation investment specifics vary.

For each personality-to-system mapping in the table to work, there must be a corresponding ai_budget economic category that controls that system. For the concrete mappings (fleet investment, colonization, research), this is real. For espionage, diplomacy depth, and archaeology, the backing system must be verified.

### Wire 2: Government Type to AI Strategic Posture
Rating: **VAGUE**

Same mechanism as politics Wire 6, which was rated CONCRETE with a caveat. The caveat here is the same: "government-conditional personality overlays" is not a Clausewitz concept. Personalities are assigned once at empire creation based on ethics/civics/authority, not layered at runtime. The actual implementation would be adding weight_modifier blocks to personality definitions that check government type, affecting which personality an AI empire is assigned at game start. This is achievable but it is a static assignment, not a dynamic posture that shifts if the government changes mid-game. The spec implies runtime behavioral adaptation that doesn't exist in the personality system.

### Wire 3: AI Fleet Design
Rating: **CONCRETE**

`common/global_ship_designs/` is a real path. Creating per-personality fleet templates is achievable. The spec is specific about the directory structure and what each template should contain. This is the most implementation-ready wire in the AI domain.

Note the FIOS interaction: Global Ship Designs uses FIOS. ESC's Global Ship Designs templates load first (ESC loads before NSC3 in the stack). The megapatch's personality fleet templates, to be authoritative over ESC's default templates for the same ship classes, must load after ESC. This means the megapatch must sit below ESC in load order for this specific path, which the overall megapatch-at-bottom structure should handle.

### Wire 4: AI Planet Management
Rating: **VAGUE**

Same as planet-economy Wire 3. The spec honestly hedges: "Verify BCA's building logic is extensible to modded content. If not, override files." If Better Colony Automation (BCA) drives planet building through vanilla's colony automation system (which reads `common/colony_automation/` priority scripts), then adding PD-aware priority files is additive. If BCA uses its own internal scripted effects that enumerate specific building/district IDs, those lists need patching, which means replacing BCA's files. Unknown without audit.

### Wire 5: AI Diplomatic Intelligence
Rating: **VAGUE**

`common/diplomatic_actions/` weights are real. Personality-conditional weights on diplomatic actions (forming federations, offering contracts) are achievable via weight_modifier blocks.

The Federation Overhaul federation types: if FO adds new federation types as new diplomatic actions, adding personality-conditional weights for those new actions is additive. If FO reworks the base federation action, the weight entries may conflict. VF feudal contracts as new diplomatic actions — same question. These need file inspection to confirm they are new entries (additive, patchable) vs. reworks of vanilla entries (replacement, requiring merge).

### Wire 6: AI Espionage Usage
Rating: **WISHFUL**

This wire has the same root problem identified in military Wire 7.

The file at `common/ai_espionage/spynetworks/00_base_strategies.txt` (confirmed real by conflicts.txt line 1474) controls spy network investment strategy. This file is contested between Ariphaos and Chris Covert Operations, meaning CCovert already replaces it. The megapatch must own a merged version.

But the deeper issue: the spec says "Schemer personality — high spy network investment, uses advanced operations from all three mods." Chris Covert, Whispering State, and Meaningful Spy each define their own operations. Whether the AI "uses" those operations is controlled by each operation's `ai_weight` block — a per-operation setting inside each operation's definition file. Those definition files are inside each mod. The megapatch would need to edit the AI weight blocks inside all three mods' operation definition files to add personality-conditional weights, meaning it must replace those files too.

The `common/ai_espionage/` strategy file controls which spy networks to build and how aggressively. It does not directly control which individual operations the AI executes. Operation selection is handled per-operation via `ai_weight`. So:

- Personality-conditional spy network investment (how much to invest): achievable via the ai_espionage strategy file
- Personality-conditional operation selection (which specific operations from CCovert/WS/MSO): requires patching each mod's operation definition files

The spec conflates both as achievable from one file. They are not.

### Wire 7: AI Research Prioritization
Rating: **CONCRETE**

`common/ai_budget/megapatch_research_priorities.txt` with personality and government-conditional research weights is achievable. The Clausewitz ai_budget system has research category entries. Multi-conditional weight blocks using `modifier` entries with `factor` values per personality flag are real. Stellar AI itself likely uses exactly this mechanism. This is the cleanest wire in the AI domain.

---

## universal-patches.md

### Assessment Accuracy

The feasibility ratings in the spec are largely accurate and map well onto what the load-rules and conflict data show.

**On_Actions Merge (Tier 1, CRITICAL):** Confirmed critical. The conflicts.txt data shows on_actions-related conflicts across many mods. The rules doc confirms BPU does full replacement of `00_on_actions.txt`. This is accurate.

**Traits Require Full File Replacement:** The load-rules doc explicitly states this: "Traits and Strategic Resources require FULL FILE REPLACEMENT (no per-entry override)." The collisions_traits.txt file demonstrates the consequence — SIMILAR and DIFFERENT collisions with stat differences that will silently lose one version. The spec's assessment (MEDIUM feasibility for definitions, LOW for slots) is accurate.

**Technology Definitions (PARTIALLY patchable):** Correct. Individual tech entries in separate files in `common/technology/` can coexist. AI weights and prerequisites need manual coordination.

**Edicts (GOOD candidate):** Correct. Edict definitions are additive by ID. Trade for Influence's rework of vanilla edict costs is the one non-additive element, correctly flagged.

**Civics (GOOD candidate):** Correct for definitions. The conflicts.txt shows no high-count civic file conflicts (the civic mods add unique IDs). The slot count issue (4EP+3CP) is a defines-level single value, correctly flagged as manual.

**Traditions (PROBLEMATIC):** The 64-slot cap concern is correctly identified. The spec does not confirm whether the sum of all approved tradition mods exceeds 64 trees. This is the highest-risk unverified number in the entire megapatch design.

**Pop Categories (CANNOT universally patch):** Confirmed by conflicts.txt showing `common/pop_categories/02_other_categories.txt` as a 2-way conflict between Plentiful Traditions and Ariphaos, and by the load-rules note that BPU and FP both replace pop category logic.

---

## Summary Table

| Domain | Wire | Rating | Confidence | Primary Risk |
|---|---|---|---|---|
| Politics | W1: Govt → Faction | CONCRETE | — | File merge scope (CW owns faction files) |
| Politics | W2: Faction → DPE | VAGUE | 88 | DPE trigger flag names unaudited |
| Politics | W3: Stability Aggregator | VAGUE | 82 | Social Decline reads raw stability vs named trigger |
| Politics | W4: Espionage → Diplomacy | CONCRETE | — | Merger with MSO scripted_effects needed |
| Politics | W5: Vassal Loyalty | WISHFUL | 85 | VF/Theta internal loyalty files require replacement, not addition |
| Politics | W6: Govt → AI | CONCRETE (caveat) | — | "Overlays" don't exist; use weight_modifiers instead |
| Politics | W7: Civil Wars | WISHFUL | 92 | CW owns faction files, no external trigger API |
| Politics | W8: Election | CONCRETE | — | Verify Ruler Level System is approved mod |
| Military | W1: Ship Unification | CONCRETE | — | Standard section/component audit |
| Military | W2: Reactor | CONCRETE | — | Decision pending file download |
| Military | W3: Carrier Stack | CONCRETE | — | SCD launch system ID audit |
| Military | W4: Formation + Scale | CONCRETE | — | Single merged defines file |
| Military | W5: War → Politics | VAGUE | 82 | FWW output state (flag vs raw war_score) unaudited |
| Military | W6: DCS Scaling | CONCRETE | — | Likely works automatically; verify |
| Military | W7: AI Fleet | VAGUE | 80 | Espionage investment not in ai_budget framework |
| Planet | W1: Planet → Buildings | CONCRETE | — | Additive building triggers |
| Planet | W2: Zone Ecosystem | VAGUE | 83 | BPVR slot logic and colony_type triggers unaudited |
| Planet | W3: Automation | VAGUE | 88 | May require BCA file replacement; unknown |
| Planet | W4: Planet → Politics | CONCRETE | — | Standard on_action + flag pattern |
| Planet | W5: Pre-FTL Stack | CONCRETE | — | Additive observation mission entries |
| Planet | W6: Economy → Everything | VAGUE/WISHFUL | 85 | No mechanism described; placeholder |
| Discovery | W1: Narrative Coherence | VAGUE | 82 | Precursor flag names unaudited across 4 mods |
| Discovery | W2: Tech Tree | CONCRETE | — | ID collision audit required, approach sound |
| Discovery | W3: Discovery → Diplomacy | CONCRETE | — | NGS proposal trigger needs verification |
| Discovery | W4: RS Physical | CONCRETE | — | Verification task, existing patches handle most |
| Discovery | W5: Pre-FTL → Politics | CONCRETE | — | Standard uplift on_action bridge |
| AI | W1: Personality → Investment | VAGUE | 83 | Espionage/archaeology not in ai_budget categories |
| AI | W2: Govt → AI Posture | VAGUE | 80 | "Overlays" are static assignment, not runtime |
| AI | W3: Fleet Design | CONCRETE | — | FIOS load order must be managed |
| AI | W4: AI Planet | VAGUE | 82 | BCA extensibility unknown |
| AI | W5: AI Diplomacy | VAGUE | 80 | FO/VF new vs. replaced actions unaudited |
| AI | W6: AI Espionage | WISHFUL | 88 | Operation selection requires per-op file patches |
| AI | W7: AI Research | CONCRETE | — | Standard ai_budget multi-conditional weights |

---

## Priority Findings for Implementation Planning

**Stop and audit before implementing:**

1. Civil Wars (W7, politics). Download the mod, open its event files and faction type files, determine if it sets any named flags that external events can read. If it does not, the five-input civil war type system requires replacing CW's faction files in the megapatch. The spec's "override CW's trigger file" fallback is actually the only path — the question is how much of CW you end up rewriting.

2. DPE external flag names (W2, politics). Download DPE, search for `has_country_flag` and `has_global_flag` checks in its event triggers. Identify which flags it reads from external sources vs. which it sets internally. This determines whether the bridge events in `megapatch_political_bridge.txt` can actually influence DPE's chains or whether they fire into a vacuum.

3. Social Decline stability trigger (W3, politics). Does SD check `stability` directly or a named scripted trigger? This determines whether the `megapatch_empire_stability` variable can intercept its reads.

4. Espionage operation AI selection (W6, AI-coherence). The ai_espionage strategy file controls network investment. Individual operation `ai_weight` blocks control operation selection. These require separate solutions and the spec does not distinguish them.

5. Tradition slot count (universal-patches). Sum all tradition trees across every approved tradition mod. Confirm the total is below 64. If not, the megapatch cannot fix this — tree count reduction would require removing content from approved mods.

**Wires that can start immediately without audits:**

Politics W1, W4, W6, W8. Military W1, W3, W4. Planet W1, W4, W5. Discovery W2, W4, W5. AI W3, W7.

**Files the megapatch must own as merged versions (not additive, based on conflict data):**

- `/Users/orgrinrt/Dev/stellar-heritage/megapatch/domains/politics-diplomacy-intrigue.md` — Wire 1 notes `pop_faction_types/*.txt` is a 4-way conflict; the megapatch must own merged versions of all nine CW-touched faction files.
- `common/ai_espionage/spynetworks/00_base_strategies.txt` — contested between Ariphaos and Chris Covert; megapatch must merge both.
- `common/scripted_triggers/wrb_scripted_triggers.txt` and `fune_diplo_scripted_triggers.txt` — contested between VF and Civil Wars; megapatch must merge these for vassal loyalty wire to function at all.
- `common/solar_system_initializers/special_system_initializers.txt` — 3-way conflict between Ariphaos, All Systems Spawn, RS 4.0.
- All five `map/setup_scenarios/` size files — 5-way conflicts; megapatch must merge these.
