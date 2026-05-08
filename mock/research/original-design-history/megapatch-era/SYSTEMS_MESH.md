# Systems Mesh — How the Modpack Connects

> This document maps how every major mod system feeds into every other,
> creating emergent gameplay through the megapatch alone — no heritage mod.
> Heritage is a separate layer that loads on top of this integrated platform.
>
> Read TENETS.md first for the philosophy. This is the implementation.

---

## The Core Loop

Vanilla Stellaris has siloed systems. The megapatch's job is to un-silo them.

```
Government shapes → Faction dynamics drive → Elections determine → 
Leaders that command → Fleets that fight → Wars that shift → 
Diplomacy that reshapes → Galactic law that constrains → 
Government reform
```

Every system feeds the next. Here's how each connection works.

---

## 1. The Political Pipeline

```
Government Authority (vanilla + GVP + Stellaris Ascended)
  ↓ determines
Faction Dynamics (Factional Politics + Galactic Politics)
  ↓ shapes
Election Outcomes (vanilla + Gender Politics)
  ↓ triggers
Diplomatic Posture (Federation Overhaul + NGS)
  ↓ affects
Galactic Community Voting (NGS + Hydra GC Resolutions + Forgotten History GC)
  ↓ feeds back into
Government Stability (Social Decline)
  ↓ cycles to
Government Authority reform
```

### What the megapatch wires:

**Government Variety Pack** adds 77 civics and 95 government forms. **Stellaris Ascended** adds 40 post-ascension civics and 27 authorities. Together: ~160 distinct government configurations. But neither mod tells **Factional Politics** about the new government types. The megapatch bridges this:

- GVP/SA government types → FP faction attraction weights (a theocratic government creates religious faction pressure)
- FP's parliamentary mechanics → NGS galactic community voting weights (domestic political strength projects internationally)
- **Gender Politics'** matriarchy/patriarchy civics → faction alignment (gender-based civics create gendered faction dynamics)
- **Social Decline's** instability events → triggered by faction unhappiness from the above, not just vanilla metrics

**Stellar AI** reads all this. The megapatch ensures AI empires with GVP's theocratic government + More AI Personalities' "zealot" personality actually behave theocratically — faction management, religious expansion, ethics enforcement. Not just vanilla AI with a different flag.

**Dynamic Political Events** fires event chains based on faction state. The megapatch wires DPE to read the COMBINED state from FP + GP + Social Decline, not just vanilla faction happiness. A political crisis in DPE should fire because Factional Politics' parliament is deadlocked AND Social Decline's stability is dropping.

---

## 2. The Espionage-Diplomacy Axis

```
Spy Networks (Chris Covert + Whispering State + Meaningful Spy)
  ↓ generate
Intelligence (Nemesis intel system)
  ↓ enables
Targeted Operations (all three mods' expanded operations)
  ↓ produce
Diplomatic Consequences (opinion modifiers + incident events)
  ↓ affect
Alliance Dynamics (Federation Overhaul + Viable Feudalism loyalty)
  ↓ constrain
Espionage Priorities (you spy on rivals, not allies... usually)
  ↓ cycles back to
Spy Network allocation
```

### What the megapatch wires:

Three espionage mods each add their own operations independently. The megapatch ensures:

- **Chris Covert's** deep ops, **Whispering State's** veiled ops, and **Meaningful Spy's** reworked ops all appear in one coherent espionage UI without ID collisions
- Spy network level gates are consistent across all three (level 40 means the same thing regardless of which mod's operation you're running)
- Discovered espionage triggers diplomatic consequences that **Federation Overhaul** responds to (caught spying on a federation ally → federation cohesion drops)
- **Viable Feudalism's** feudal loyalty reacts to espionage between overlord and vassal (spying on your own feudal lord is a betrayal)
- **Stellar AI** + **More AI Personalities** drives which AI empires invest in espionage. An AI "schemer" personality actually uses the expanded espionage toolkit, not just vanilla operations

---

## 3. The Vassal Network

```
Subject Creation (Overlord + Viable Feudalism)
  ↓ establishes
Loyalty Dynamics (Overlord + VF feudal loyalty + Theta Vassals)
  ↓ with
Subject Specialization (Overlord specialist subjects)
  ↓ affecting
Military Contribution (At War defense + Expanded War Goals)
  ↓ feeding
Overlord Power (diplomatic weight + galactic standing)
  ↓ reinforcing or undermining
Subject Loyalty
```

### What the megapatch wires:

**Overlord** adds specialist subjects. **Viable Feudalism** adds feudal loyalty. **Theta Vassals** expands vassal interactions. But they don't talk to each other:

- VF's feudal contract loyalty + Overlord's specialist loyalty = compounding loyalty for vassals that are both specialized AND feudally contracted (megapatch merges the loyalty calculation)
- Theta Vassals' expanded interactions trigger in response to VF's feudal contract violations
- A Scholarium vassal with Overlord specialization + At War's defense platforms + VF feudal loyalty = a well-defined, mechanically coherent vassal state
- **Vassal Antispam** prevents AI from snowballing vassals, but the megapatch ensures the vassals that DO form are meaningful (proper specialization, proper loyalty mechanics)
- AI overlords (Stellar AI) use specialist subjects strategically, not randomly

---

## 4. The Military Stack

```
Fleet Composition (NSC3 + ESC NEXT + At War + Fleet Formation + Downscaled)
  ↓ determines
War Capability (Expanded War Goals + Dynamic Crisis Strength)
  ↓ enables
Military Actions (wars, crisis response)
  ↓ produce
War Outcomes (Fleets Win Wars scoring)
  ↓ generate
Political Consequences (faction shifts, GC responses)
  ↓ influencing
Military Investment (traditions, tech priorities)
  ↓ cycling back to
Fleet Composition
```

### What the megapatch wires:

**NSC3** adds ship classes. **ESC NEXT** adds weapons. **At War** adds ship sections, carriers, defense platforms, starbases. Each mod is designed independently. The megapatch unifies them:

- At War's advanced ship sections are available for NSC3's ship classes (not just vanilla classes)
- ESC NEXT's weapons fit into At War's carrier bays
- ESC NEXT's "Disable NSC Reactors" submod resolves the power source conflict — megapatch ensures this is configured correctly
- **Fleet Formation** spacing + **Downscaled Ships** scaling work together (formation math accounts for scaled model sizes)
- AI ship designs (**ESC NEXT: Global Ship Designs** submod) incorporate all three mods' components, not just one
- **Strike Craft Diversity** gives fighters variety that ESC NEXT's carriers can deploy through At War's carrier improvements
- **Fleets Win Wars** adjusts war score so decisive battles matter — this feeds back through political consequences via **Social Decline** and **Factional Politics** (losing a major war destabilizes the government)

The military mods' OUTPUT (war results) becomes the political mods' INPUT (stability, factions, GC). This is the key cross-domain wire.

---

## 5. The Planet-Economy Layer

```
Planet Types (PD 50+ types + Guilli's modifiers + Planet Flavour + APSR)
  ↓ provide
Building/Zone Framework (BPVR + Hybrid Colony Designations + More Specialized Zones)
  ↓ supporting
Economic Output (vanilla economy + Planetary Wonders)
  ↓ driving
Population Dynamics (Pop Growth Corrections + species traits)
  ↓ generating
Faction Activity (Factional Politics + faction personality)
  ↓ feeding
Political Events (DPE + Social Decline)
```

### What the megapatch wires:

The planet player manages 50+ PD planet types with Guilli's modifiers, builds in BPVR's expanded slots, uses Hybrid Colony Designations for mixed-purpose worlds. But without the megapatch:

- **Guilli's Planet Modifiers** doesn't know about **PD's** custom planet classes — the compat patch (approved) bridges this, but the megapatch ensures the combined result feeds into **Planet Flavour's** event system
- **BPVR's** expanded building slots need to work with **PD's** custom districts AND **Sort Those Buildings'** auto-sort AND **Tier Numbers: Buildings** visual indicators — all reading the same building data
- **Hybrid Colony Designations** mixed-purpose worlds need the AI to understand them — **Stellar AI** + **Better Colony Automation** must handle modded colony types, not just vanilla
- **Planetary Wonders** endgame buildings on specific PD planet types — the megapatch ensures wonder availability matches PD's expanded planet type list
- The economic output of all this drives **Factional Politics** satisfaction (wealthy empire = happy factions = stable politics) through **Social Decline's** stability metrics

**Ultimate Automation** and **Automate Infrastructure** handle the tedium. The megapatch ensures automation respects modded content — auto-building doesn't ignore PD districts or Planetary Wonders.

---

## 6. The Discovery Arc

```
Exploration (Real Space + Hypothetical Stars + All Systems Spawn)
  ↓ finds
Anomalies (All Anomaly Spawns + APSR + More Events)
  ↓ leading to
Archaeological Digs (Ancient Relics + Archaeology Story Pack)
  ↓ yielding
Relics & Artifacts (relic system + Relic Trade + artifact QoL)
  ↓ providing
Knowledge (intel + technology + precursor lore)
  ↓ enabling
Strategic Advantage (tech bonuses + diplomatic leverage + relics)
  ↓ fueling
Further Exploration (Distant Stars L-Gates + astral rifts)
```

### What the megapatch wires:

**Real Space** makes the galaxy physically interesting. **More Events Mod** fires anomalies. **Archaeology Story Pack** and **Forgotten History Precursors** create dig chains. **Relic Trade** lets empires trade relics.

The megapatch ensures these aren't isolated event chains:
- Anomalies from MEM can reference Real Space's unique system types (an anomaly in a binary star system knows it's in a binary system)
- Archaeological discoveries from Archaeology Story Pack can reference Forgotten History's precursor lore (consistent galactic history, not contradictory mod narratives)
- **APSR's** planetary resources interact with Guilli's planet modifiers (a resource-rich planet with a Guilli's "ancient ruins" modifier is a natural dig site location)
- **Relic Trade** relics have diplomatic weight that **Federation Overhaul** and **NGS** respond to (trading a powerful relic is a diplomatic event, not just an inventory swap)
- **Pre-FTL observation** (First Contact) events feed into the discovery arc — primitives on worlds with specific PD types or Guilli's modifiers have thematically appropriate development

---

## 7. The AI Coherence Layer

```
Stellar AI (core strategic behavior)
  + More AI Personalities (character diversity, 24+ personalities)
  + Dynamic AI Scaling (progression balance)
  + AI Ship Building Priority Fix (fleet efficiency)
  + AI Game Performance Optimisation (computation efficiency)
  + Interesting Empires + Rising Empires + Empires Collection (160+ handcrafted AI empires)
  + Faction Personality [TNG] (faction behavior diversity)
  + Spawn More AI Empires (galaxy population)
  + No Clustered Starts (spatial fairness)
  + AI Use Custom Shipsets (visual variety)
  + Better Colony Automation (planet AI)
  + Habitat AI Colonisation Fix (habitat logic)
```

### What the megapatch wires:

The AI stack is the invisible backbone. Without it, AI plays vanilla while the player plays modded. The megapatch ensures:

- **AI personality → modded behavior.** A "trader" personality (More AI Personalities) focuses on economy mods (MegaCorp mechanics, Trade for Influence). A "conqueror" focuses on military (NSC3 ship designs, At War defense). A "schemer" focuses on espionage (all three spy mods). AI personality doesn't just affect diplo attitude — it drives which systems the AI invests in.
- **AI ship designs use all military mods.** NSC3 classes + ESC NEXT weapons + At War sections. Not vanilla designs with modded weapons bolted on. The Global Ship Designs submod handles this but the megapatch ensures it covers the full military stack.
- **AI empire variety is real.** 160+ handcrafted empires from Interesting + Rising + Empires Collection, each with proper GVP/Stellaris Ascended government forms, appropriate ethics/traits/civics. Every playthrough has a unique galactic cast.
- **AI manages modded planet systems.** Better Colony Automation + Stellar AI ensures AI doesn't ignore PD custom districts, Hybrid Colony Designations, or Planetary Wonders. AI builds smartly on modded content.
- **Dynamic AI Scaling** applies to ALL modded systems. As the game progresses, AI gets better at using expanded traditions, expanded ascension perks, expanded espionage, expanded diplomacy. Not just vanilla scaling.

**The result:** When the player encounters an AI empire, it feels like encountering a real civilization with its own political dynamics, military doctrine, and diplomatic personality. Not a stat block running on a different game.

---

## 8. Cross-Domain Connections

These are the links between the seven systems. Each is a megapatch wire.

| From → To | What Flows | Concrete Implementation |
|---|---|---|
| Political → Military | Government type affects military doctrine | GVP/SA civics → Stellar AI fleet composition weights |
| Military → Political | War outcomes affect political stability | Fleets Win Wars results → Social Decline + Factional Politics |
| Political → Espionage | Government instability increases spy vulnerability | Social Decline instability → spy network cost reduction for attackers |
| Espionage → Political | Discovered spies trigger political events | Spy operation outcomes → DPE political crisis triggers |
| Diplomacy → Vassal | Alliance type affects vassal loyalty | Federation membership → VF/Overlord loyalty modifiers |
| Vassal → Military | Vassal specialization affects war capability | Overlord Bulwark + At War defense → combined fleet defense |
| Planet → Political | Planet prosperity drives faction happiness | Guilli's modifiers + PD prosperity → Factional Politics satisfaction |
| Discovery → Diplomacy | Shared discoveries improve relations | Relic Trade + Archaeological finds → opinion modifiers |
| AI → Everything | AI uses all systems coherently | Stellar AI + personality → drives all subsystem decisions |
| Military → Diplomacy | Military strength affects diplomatic weight | NSC3 fleet power → NGS/Federation Overhaul diplomatic calculations |
| Espionage → Discovery | Spy networks reveal hidden information | Spy intel → archaeological site discovery probability |
| Planet → Discovery | Rich planets attract investigation | APSR resources + Guilli's modifiers → anomaly spawn weights |

---

## DLC Multiplier Effects

Each DLC amplifies existing mod connections:

**Galactic Paragons** × Political Pipeline = council positions are political prizes. Every council seat change is a political event that Factional Politics responds to, that GVP government type constrains, that NGS galactic community notices.

**Nemesis** × Espionage Axis = custodian elections become political power plays using NGS + Federation Overhaul diplomatic weight. The spy mods' operations gain galactic-scale stakes when targeting the custodian/emperor.

**Overlord** × Vassal Network = specialist subjects + VF feudal loyalty + Theta expanded interactions = a fully realized feudal system where AI overlords manage their subjects strategically.

**Federations** × Diplomacy = galactic community resolutions from NGS + Hydra + Forgotten History create a rich legislative environment. Federation types from Federation Overhaul interact with VF's feudal mechanics.

**Ancient Relics** × Discovery Arc = dig sites feed into the knowledge pipeline. Relics from the archaeology mods flow through Relic Trade into diplomatic leverage.

**First Contact** × Discovery = pre-FTL observation feeds into the planet management pipeline (PD planet types, Guilli's modifiers affect primitive development).

---

## What Makes This Different

Other modpacks say: "we include 200 mods that work together."
They mean: "we tested that they don't crash."

We say: "our 458 mods are one system."
We mean: "the output of every mod is the input of another. The player
experiences one coherent galaxy, not 458 independent feature additions."

The megapatch is where this promise becomes real. It stands alone as
a deeply integrated modpack experience. Heritage then loads on top as
a separate mod, adding dynasty meaning to an already-functioning system mesh.

```
[458 mods] → megapatch (integration) → heritage (dynasty meaning)
```
