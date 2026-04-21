# Performance Review: Megapatch Design

**Reviewer:** Performance Engineer perspective
**Target hardware:** Steam Deck (Zen 2, 4C/8T @ 2.4-3.5GHz, 16GB LPDDR5, SSD)
**Modpack:** 458 mods + megapatch + heritage mod
**Date:** 2026-04-01

---

## Executive Summary

The megapatch design is architecturally ambitious. From a pure design standpoint it
is excellent work. From a performance standpoint, three areas pose serious risk to
late-game playability on Steam Deck: the monthly event budget, the per-leader
variable footprint, and the AI coherence layer's weight multiplication. The trait
collision count (183 collisions across 1930 entries) is a non-issue at parse time.
Save file bloat is a moderate concern that becomes critical if cleanup discipline
slips. Specific cuts are recommended at the end.

---

## 1. Monthly Event Budget

### What the design proposes

The megapatch defines bridge events fired on `on_monthly_pulse_country` across
five domains:

| Bridge Event File | Domain | Purpose |
|---|---|---|
| `megapatch_political_bridge.txt` | Wire 2 | FP faction state -> DPE political crisis |
| `megapatch_war_bridge.txt` | Wire 5 | War outcomes -> FP faction shifts |
| `megapatch_spy_bridge.txt` | Wire 4 | Espionage -> diplomatic consequences |
| `megapatch_planet_bridge.txt` | Wire 4 | Planet events -> political consequences |
| `megapatch_discovery_bridge.txt` | Wire 3 | Discovery -> diplomatic weight |

Additionally, the stability aggregator (`megapatch_stability.txt`) runs monthly
to compute `megapatch_empire_stability`.

### What the existing modpack already fires monthly

From the on_actions map analysis, the **existing 458 mods** already register
**34 `on_monthly_pulse_country` handlers** across the downloaded mods. Every
one of these fires **once per country per month**. With `Spawn More AI Empires`
active and a large galaxy, expect 40-60 countries mid-game.

That means the baseline is already:

    34 handlers x 50 countries x 12 months/year = 20,400 monthly handler invocations per year

Each handler contains condition checks (scripted triggers, flag checks, variable
comparisons). Even fast-exiting handlers (trigger fails immediately) cost ~0.1ms
on Deck-class hardware. Handlers that actually fire events cost 1-10ms depending
on complexity.

### Budget math

**Stellaris tick budget on Steam Deck at speed 3:**
- Target: one in-game day per ~250ms wall-clock (to maintain "playable" speed 3)
- A month is 30 ticks. Monthly pulse fires on one of those ticks.
- That one tick must process ALL monthly handlers for ALL countries.
- Budget for the monthly-pulse tick: ~250ms (same as any tick, but this one is loaded)

**Current load (existing mods):**
- 34 handlers x 50 countries = 1,700 handler evaluations
- At ~0.1ms average (most early-exit on trigger): ~170ms
- Handlers that actually fire: maybe 5% = 85 full event executions at ~2ms = 170ms
- Total: ~340ms -- already OVER budget by ~90ms on the heavy month tick

**Megapatch additions:**
- 6 new monthly handlers x 50 countries = 300 additional evaluations
- The stability aggregator is the expensive one: it reads FP parliamentary state,
  GVP government legitimacy, war outcome modifiers, and diplomatic state.
  That is 4-6 scripted trigger evaluations PER COUNTRY PER MONTH.
- Cost estimate: 6 handlers x 50 countries x 0.3ms (bridge handlers are heavier
  than average because they intentionally read cross-mod state) = 90ms additional

**Result:** The megapatch adds roughly 90ms to an already-overloaded monthly tick.

### Recommendation: CRITICAL

1. **Do NOT run all bridge events monthly.** Stagger them:
   - Political bridge: `on_monthly_pulse_country` (political changes are frequent)
   - War bridge: `on_war_ended` only (not monthly -- wars end discretely)
   - Spy bridge: `on_espionage_operation_completed` only (discrete events)
   - Planet bridge: `on_yearly_pulse_country` (planets change slowly)
   - Discovery bridge: `on_archaeological_site_completed` (discrete events)
   - Stability aggregator: `on_bi_yearly_pulse_country` (6-month interval)

2. **Implement early-exit guards.** Every monthly handler must start with:
   ```
   trigger = {
       has_country_flag = megapatch_has_relevant_state
   }
   ```
   Set/clear flags only when state actually changes. Do not re-evaluate all
   countries every month just to determine nothing changed.

3. **Cap: maximum 2 new monthly_pulse_country handlers from the megapatch.**
   Every additional monthly handler is a 50-country multiplier.

**Moving from 6 monthly handlers to 2 monthly + 4 event-driven saves
approximately 200 handler evaluations per month, or ~60ms per monthly tick.**

---

## 2. Variable Budget Per Leader

### Heritage mod variable layout (from DESIGN.md)

| Data Structure | Slots | Fields/Slot | Variables |
|---|---|---|---|
| Bloodline vector | 8 | 3 (id, pct, ticks) | 24 |
| Bloodline count | - | - | 1 |
| Association vector | 6 | 3 (id, type, role) | 18 |
| Association count | - | - | 1 |
| Previous associations | 4 | 4 (id, type, role, reason) | 16 |
| Previous count | - | - | 1 |
| Misc heritage state | - | - | ~10-15 |
| Cached computed props | - | - | ~10 |
| **Subtotal (heritage)** | | | **~72-87** |

### What the megapatch adds

The bridge variables are mostly on the COUNTRY scope (empire stability, war
consequence state, etc.), not per-leader. The megapatch should add approximately
0-2 variables per leader (e.g., a flag reference for government-type AI weighting).

However, the combined picture matters:

### Memory math

Stellaris variables are stored as key-value pairs in a hash map on each scope.
Each variable consumes approximately:

- Key string: ~32 bytes (interned string reference + hash)
- Value: 8 bytes (64-bit float)
- Hash map overhead: ~24 bytes per entry (bucket pointer, next pointer, hash)
- **Total: ~64 bytes per variable**

**Per leader with full heritage data:**
- 87 variables x 64 bytes = **~5.6 KB per leader**

**Galaxy-wide:**
- Mid-game: ~120 leaders (player 10-15, each AI empire 3-5, x 30 empires)
- Late-game: ~200+ leaders (more empires, more hired leaders, Paragons)
- 200 leaders x 5.6 KB = **~1.1 MB of leader variable data**

**Verdict: ACCEPTABLE.** 1.1 MB is negligible on a 16 GB system. The concern is
not RAM but **iteration cost**: any code that iterates over all leaders and reads
their heritage variables (e.g., "find all leaders with bloodline X") pays
O(leaders x slots) per scan.

### Recommendation: MODERATE

1. **The 8-slot bloodline cap is generous.** In practice, after 4-5 generations
   of outbreeding, old bloodlines dilute below threshold. Consider whether
   `@heritage_bloodline_cap = 6` would suffice. Each slot reduction saves
   3 variables x 200 leaders = 600 variables from iteration and serialization.

2. **Zero cleanup on leader death is essential and already designed.** The design
   correctly notes zeroed variables are excluded from save serialization. This
   discipline must be maintained without exception.

3. **Never iterate all leaders globally.** Any operation that needs "find all
   leaders with bloodline X" must maintain an inverse index (a flag on the
   country, or a registry fleet list) rather than scanning all leader scopes.

---

## 3. Save File Bloat

### Baseline

A vanilla 458-mod Stellaris save in late-game (2400+) is typically 15-40 MB
compressed (100-300 MB uncompressed), depending on galaxy size and pop count.

### What the megapatch and heritage add

**Country-scope variables (megapatch):**
- ~10-20 bridge state variables per country (stability, war consequence, spy state)
- 50 countries x 20 variables x 64 bytes = ~64 KB

**Leader-scope variables (heritage):**
- 200 leaders x 87 variables x 40 bytes (save format is text, ~40 chars per var line) = ~700 KB

**Registry fleets (heritage):**
- Each family, bloodline, association is a hidden fleet with 5-15 variables
- Estimate: 200 families + 100 bloodlines + 50 associations = 350 registries
- 350 registries x 10 variables x 40 bytes = ~140 KB

**Country and global flags (megapatch + heritage):**
- Early-exit flags, state flags, event cooldown flags
- Estimate: 500 flags x 30 bytes = ~15 KB

**Total megapatch + heritage save contribution: ~920 KB uncompressed**

### Risk assessment

920 KB on a 200 MB save is **0.46%** growth. This is negligible for save size.

**Save corruption risk:** Stellaris save corruption is caused by:
1. Running out of disk space during write (not an issue on SSD)
2. Circular references in scopes (not possible with our variable-only approach)
3. Extremely large single entities (e.g., a fleet with 10,000+ ships -- not applicable)
4. Desync in multiplayer (variable mutations not synchronized)

The heritage design's use of flat variables on standard scopes (leader, fleet,
country) is the safest possible storage approach. Registry fleets are a known
pattern used by other mods (e.g., Gigastructures). **Save corruption risk is LOW.**

### Recommendation: LOW RISK

1. **Enforce the zeroing discipline.** Dead leaders MUST have all variables zeroed
   in the `on_leader_death` handler. The design already specifies this. If even
   10% of dead leader variables leak, over a 200-year game you accumulate
   thousands of orphaned variables.

2. **Registry fleet cleanup.** When a family goes extinct, the registry fleet
   must be destroyed. Do not leave empty registries in the save.

3. **Periodic reconciliation (already designed).** The Pattern 4 reconciliation
   pulse catches orphaned data. Ensure it runs at least once per decade.
   Running it annually would be wasteful; decadal is sufficient.

---

## 4. The AI Coherence Layer

### What the design proposes

From `domains/ai-coherence.md`, the megapatch creates personality-driven AI
weight modifiers across ALL subsystems. The Wire 1 table maps 9 personality
types to primary/secondary/de-prioritized system investments.

This means for each AI empire, each decision cycle evaluates:
1. Base Stellar AI weights
2. More AI Personalities personality modifier
3. **Megapatch government-type modifier** (Wire 2 -- 8 government categories)
4. **Megapatch personality-to-system router** (Wire 1 -- 9 personality types)
5. Dynamic AI Scaling progression modifier

### CPU cost analysis

AI budget evaluation runs on a **staggered schedule** -- not every AI every tick.
Stellaris evaluates each AI's economic decisions roughly every 10-30 days
(implementation-dependent, reduced by AI Game Performance Optimisation).

The cost question: how are the megapatch's personality-conditional weights
implemented?

**If implemented as scripted_triggers in ai_budget files:**
Each `ai_budget` entry with a `trigger = { has_personality = ... }` check
costs ~0.05ms per evaluation. With 9 personality mappings x ~6 budget categories
= 54 conditional checks per AI empire per evaluation.

- 50 AI empires x 54 checks x 0.05ms = 135ms per AI budget evaluation cycle
- Budget cycles happen every ~20 days = ~18 times per year
- Annual cost: 135ms x 18 = **2,430ms/year = ~2.4 seconds/year of game time**

At speed 3 on Deck, a game-year takes roughly 10-15 minutes wall-clock in
late-game. 2.4 seconds per year is **~0.3% of total frame time.** This is
acceptable.

**However:** The design also proposes per-personality fleet templates (Wire 3),
personality-conditional diplomatic weights (Wire 5), personality-conditional
espionage weights (Wire 6), and personality-conditional research priorities
(Wire 7). If ALL of these use runtime trigger evaluation:

- 50 AI x (54 budget + 30 fleet + 20 diplo + 20 spy + 30 research) x 0.05ms
- = 50 x 154 x 0.05ms = 385ms per evaluation cycle
- x 18 cycles/year = 6,930ms/year = **~6.9 seconds/year**

This is still only ~1% of total frame time, but it compounds with the existing
AI mod overhead. **The AI coherence layer is the one area where the design's
ambition scales linearly with AI empire count.**

### Recommendation: MODERATE

1. **The AI coherence layer is the design's best feature. Do not cut it.**
   It is what makes 458 mods feel like one game. The CPU cost is manageable.

2. **Use pre-computed templates, not runtime evaluation, for fleet designs.**
   The design already notes this in the Performance Considerations section:
   "Personality-driven weights are evaluated ONCE per decision cycle, not every
   tick." Enforce this. Fleet templates must be static `global_ship_designs`
   entries, not dynamically evaluated.

3. **AI Game Performance Optimisation is load-bearing.** This mod reduces AI
   evaluation frequency. If it breaks on 4.3, the entire AI coherence layer
   becomes a performance problem. Ensure it is tested and functional.

4. **Consider limiting the personality router to the top 4-5 personality
   archetypes** (conqueror, diplomat, scientist, trader, schemer) rather than
   all 9. The remaining 4 (zealot, expansionist, isolationist, feudalist) can
   fall back to the closest major archetype. This cuts 44% of conditional
   checks.

---

## 5. Trait Collision Resolution

### The numbers

- 1930 total trait entries across 13+ mods
- 294 potential collisions detected
- 183 collisions where LIOS (Last In, Only Served) resolution applies
- Traits require **full file replacement** (no per-entry override)

### Parse-time overhead

**Stellaris trait loading works as follows:**
1. Engine scans all `common/traits/` files across all mods in load order
2. For each file path, LIOS applies: if two mods provide the same filename,
   the last-loaded version wins entirely
3. Within a single file, all trait entries are parsed sequentially
4. If two different files define the same trait ID, the LIOS copy replaces
   the FIOS copy

**Parse overhead of 1930 entries:** Trait parsing is a startup-only cost. At
~0.5ms per trait definition, all 1930 traits parse in under 1 second. This
happens ONCE at game load. **Zero runtime impact.**

**The 183 collisions:** When LIOS resolves a collision, the earlier definition
is simply discarded. The engine does not maintain both copies. There is no
"collision resolution overhead" at runtime -- the game state contains exactly
one definition per trait ID after loading.

**The collisions_traits.txt analysis shows most are SIMILAR (icon/weight
differences) rather than DIFFERENT (mechanical changes).** This means the
LIOS resolution (megapatch version wins) is safe in most cases.

### Recommendation: NON-ISSUE

1. **Parse overhead: negligible.** 1930 traits is well within Stellaris's
   design parameters. The engine handles 2000+ traits without degradation.

2. **Runtime overhead: zero.** Collisions are resolved at load time. The game
   never re-evaluates which trait definition to use.

3. **The megapatch's approach of consolidating into merged trait files is
   correct.** It eliminates the ambiguity of load-order-dependent LIOS
   resolution by providing a single authoritative file.

4. **Watch for trait SLOT limits, not trait count.** The real constraint is
   how many traits a leader can have simultaneously. With Galactic Paragons
   expanding trait slots and 13 trait mods adding options, verify that the
   AI trait picker doesn't thrash (repeatedly gaining/losing traits at the
   slot cap). Trait-slot thrashing causes unnecessary save mutations.

---

## 6. What to Cut (20% Reduction)

If performance budget requires cutting ~20% of megapatch features, cut these
in order of decreasing performance-to-gameplay ratio:

### Cut List (in priority order)

**1. Planet bridge events (monthly) -> move to yearly [SAVE ~30ms/month-tick]**

The `megapatch_planet_bridge.txt` fires monthly to check planet event flags
and route consequences to the political system. Planet state changes slowly.
Moving to `on_yearly_pulse_country` loses 11 months of reactivity but saves
50 country evaluations per month. Alternatively, make it purely event-driven
(fire on specific planet event completions only).

**2. Discovery bridge events (monthly) -> make fully event-driven [SAVE ~30ms/month-tick]**

The `megapatch_discovery_bridge.txt` should ONLY fire on
`on_archaeological_site_completed` and `on_survey_planet` completions. There
is no reason to poll for discoveries monthly -- they are discrete events.

**3. Civil Wars enriched triggers (Wire 7) -> simplify to 2 types [SAVE complexity]**

The design proposes 5 civil war types based on which input is dominant
(faction-driven, decline-driven, vassal-driven, espionage-driven,
military-driven). Each type requires evaluating all five inputs per country.
Cut to 2 types: "political" (faction + decline) and "military"
(war + vassal). Espionage-driven coups are cool but add evaluation cost for
a rare event.

**4. Pre-FTL integration stack (Wire 5 in planet-economy) -> defer to v2 [SAVE dev time, ~10ms]**

The primitive enrichment system (matching PD planet types to primitive
development paths, Guilli's modifier interaction) is flavor, not core
gameplay. Primitives are a niche mechanic that most players encounter
5-10 times per game. The megapatch wiring cost (observation station mission
overrides, event checks) is not justified for v1.

**5. Personality router: reduce from 9 to 5 archetypes [SAVE ~44% AI checks]**

Map zealot->conqueror, expansionist->diplomat, isolationist->scientist,
feudalist->diplomat. Keeps the 5 most distinctive AI behaviors; the remaining
4 are shades of the main 5 anyway.

**6. Narrative coherence (Wire 1 in discovery) -> defer to v2 [SAVE complexity]**

The precursor history flag system and dig-site mutual exclusion triggers add
per-planet flag checks on dig site events. In practice, the chance of two
mods spawning contradictory dig sites on the same planet is low enough to
accept. Mutual exclusion triggers can be added post-launch if reports emerge.

### What NOT to cut

- **The stability aggregator (Wire 3 in politics).** This is the backbone of
  the systems mesh. Without it, Social Decline runs on vanilla metrics and
  the entire political pipeline loses its cross-mod integration.

- **The AI coherence layer (Wires 1-2 in AI domain).** This is the single
  highest-value feature. AI that uses modded systems is what makes the modpack
  feel integrated vs. a random pile of mods.

- **The on_actions merge.** This is not optional. Without it, BPU's override
  silently destroys other mods' event registrations.

- **The military stack unification (ship design + reactor + carrier).** Without
  this, the three military mods are three parallel games. The merge is mostly
  file-level (section_templates, component_templates) with no runtime cost.

- **Espionage -> diplomatic consequences (Wire 4).** This is event-driven
  (fires on operation completion only), so its runtime cost is near zero.
  It is also one of the most satisfying cross-mod interactions: getting caught
  spying on a federation ally has real consequences.

---

## Summary: Performance Risk Matrix

| Feature | Risk | Runtime Cost | Action |
|---|---|---|---|
| Monthly bridge events (6) | HIGH | ~90ms/month-tick | Reduce to 2 monthly, 4 event-driven |
| Per-leader variables (87) | LOW | ~1.1 MB RAM | Acceptable; enforce cleanup |
| Save file growth (~920 KB) | LOW | <0.5% growth | Acceptable; enforce zeroing |
| AI coherence weights | MODERATE | ~7s/game-year | Reduce personality types 9->5 |
| Trait collisions (183) | NONE | Load-time only | Non-issue |
| Stability aggregator | LOW | ~15ms/bi-yearly | Move from monthly to bi-yearly |

### Steam Deck Specific Notes

- **Thermal throttling:** The Deck throttles CPU at ~90C. Late-game Stellaris
  with 458 mods will thermal throttle. Every millisecond saved extends the
  window before throttle. The monthly event budget is the highest-leverage
  optimization target.

- **RAM:** 16 GB shared between CPU and GPU. Stellaris on Deck with 458 mods
  will use 6-10 GB. Our ~1.1 MB of leader variables is noise. The loading
  screen texture mods (Backgrounds+ Combined, Cinematic, More Vanilla) are a
  bigger RAM concern than any megapatch variable.

- **SSD I/O:** Save file writes are fast on Deck's NVMe. The 920 KB addition
  to save files is invisible. Save time is dominated by the base game's
  serialization of pop jobs, trade routes, and AI state.

- **Speed 5 is dead.** On Deck with this modpack, speed 5 will not be
  meaningfully faster than speed 3 in late game. The monthly tick is the
  bottleneck. Inform users that speed 3 is the target and speed 5 is cosmetic
  after 2350.

---

## Appendix: Monthly Handler Inventory (Existing Mods)

From `on_actions.txt` analysis, mods already registering `on_monthly_pulse_country`:

| Mod | Handler Count | Complexity |
|---|---|---|
| Expanded Stellaris Ascension Perks | 1 | Medium (perk checks) |
| Plentiful Traditions | 1 | High (tradition state) |
| Reworked Advanced Ascension | 1 (via global monthly) | Medium |
| Dynamic Political Events | Multiple (via init) | High |
| Factional Politics | 1+ | High (parliamentary state) |
| Various event mods | ~20+ | Low-Medium |
| Heritage mod (future) | 2-3 | Medium (bloodline events) |
| **Megapatch (proposed)** | **6** | **Medium-High** |

Total with megapatch: **~40+ monthly_pulse_country handlers**.
Total without staggering optimization: **2,000+ evaluations per monthly tick**.

The staggering recommendation (point 1 in section 1) is the single most
impactful optimization in this review.
