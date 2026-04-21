# Game Design Review — Systems Mesh & Domain Documents

> Reviewer perspective: Game Designer, co-op focus
> Documents reviewed: SYSTEMS_MESH.md, TENETS.md, all 6 domain files, DLC_GUIDE.md
> Date: 2026-04-01

---

## Verdict

The design is architecturally sound. The systems mesh is the right idea —
un-siloing Stellaris's systems through explicit wires between mod outputs and
inputs. The domain documents are unusually well-structured for a modpack.

But the design has blind spots, all of them centered on the same thing:
**Player 2's moment-to-moment experience**. The documents describe a
beautiful system from 30,000 feet. They do not describe what it feels like
to sit on the couch and play it.

Six findings follow, ordered by severity.

---

## 1. Player 2 Depth — THE critical concern

**Rating: INSUFFICIENT as designed. Fixable, but needs explicit attention.**

### The problem

The planet-economy domain document (Player 2's home) describes an experience
arc: Survey, Colonize, Build, Specialize, Wonder, Automate, Focus. That arc
sounds good on paper. In practice, here is what each phase looks like
compared to Player 1:

| Phase | Player 1 (Fleet/Diplo/Research) | Player 2 (Planet/Arch/Politics) |
|---|---|---|
| Early (2200-2230) | Designs first fleet, first contact events, rival emerges, chooses research direction, espionage seeds | Surveys (click science ship, wait), colonizes (click colony ship, wait), builds districts (click, click, click) |
| Mid (2230-2300) | War declarations, fleet battles in real-time, federation negotiations, GC voting, spy operations with consequences | Building queue management, archaeology (click dig, wait 1-5 years), primitive observation (click, wait) |
| Late (2300+) | Crisis response, custodian election, emperor path, galactic-scale diplomacy, fleet redesign for crisis | Automation handles building. Planetary Wonders are one click each. What does Player 2 actually DO? |

Player 1 has **agency density** — frequent moments where they make
consequential choices and see immediate results. Player 2 has **queue
management** — set up a build order, wait, check back, set up another. The
archaeology and politics roles are the saving grace, but they are described
as secondary to planets, not co-equal.

### The deeper issue

Stellaris's planet gameplay is fundamentally a spreadsheet. More districts,
more buildings, more modifiers — these are numerical improvements, not
dramatic moments. The military stack generates EVENTS — battles, war
declarations, crisis arrivals, fleet losses. Events are exciting. Spreadsheet
optimization is not.

The design documents recognize this implicitly: they list 4 automation mods
to reduce planet tedium. But automating tedium does not create depth. It
creates a vacuum. If Ultimate Automation handles building and Better Colony
Automation handles needs assessment, what is Player 2's moment-to-moment
gameplay in year 2280?

### What's missing

The design needs to explicitly define **Player 2's event density** — how
often does something HAPPEN to them that requires a real decision, not just a
queue optimization? Right now the documents describe:

- Planet Flavour events (per-planet, sporadic)
- Guilli's modifier discoveries (per-colony, one-time)
- Archaeological digs (multi-year waits between stages)
- Primitive observation (set and forget)
- Factional Politics parliamentary events (periodic)

Compare to Player 1's event density:
- Spy operation completions (frequent)
- Diplomatic proposals from AI (frequent)
- War status changes (continuous during wars)
- GC resolution voting (periodic)
- Federation management (continuous)
- Fleet encounter events (frequent during exploration/war)

Player 2 has maybe 1/3 the event density. The mods are there — Factional
Politics, Dynamic Political Events, Social Decline, Civil Wars — but the
domain document assigns politics as a BRIDGE between planets and diplomacy,
not as Player 2's primary engagement system.

### Recommendation

**Elevate politics to be Player 2's co-primary domain alongside planets.**
Player 2 should own:
- All faction management (FP parliamentary system, coalition building)
- All internal political events (DPE, Social Decline, civil war prevention)
- Election management (candidate selection, campaign-equivalent mechanics)
- GC resolution DRAFTING and lobbying (while Player 1 handles GC voting as
  part of diplomacy)

This reframes Player 2 from "planet manager who sometimes does archaeology"
to "domestic affairs minister who shapes the empire's internal destiny."
The planet building becomes infrastructure for political power, not the
point in itself.

The heritage mod amplifies this perfectly: succession crises, dynasty
legitimacy, faction loyalty to the ruling house — these are all Player 2's
domain if politics is explicitly theirs.

**Concrete change needed:** The Player 2 Experience Design section in
planet-economy.md needs a parallel "Political Experience Arc" that maps
what political events Player 2 faces at each game phase. The politics
domain document should explicitly state which political systems are
Player 2-owned vs Player 1-owned.

---

## 2. Feedback Loops — Are they FELT?

**Rating: Architecturally correct, experientially unverified.**

### What the design gets right

The cross-domain connections table (SYSTEMS_MESH.md section 8) is excellent.
Twelve explicit wires between systems, each with a concrete implementation
path. The idea that "the output of every mod is the input of another" is the
right design principle.

### What the design doesn't address

Feedback loops only matter if players NOTICE them. The mesh document
describes wires like:

> "Planet → Political: Planet prosperity drives faction happiness"

But how does Player 2 KNOW this happened? If they build a thriving economy
on a PD Ammonia World and that prosperity flows through Social Decline's
stability metric into Factional Politics' satisfaction, what does Player 2
actually see? A green number going up somewhere? A tooltip that says "+2
stability from planetary prosperity"?

The design describes system-level connections but not **player-facing
feedback moments**. For co-op specifically, there need to be moments where:

1. Player 2 does something → Player 1 sees the consequence and says
   "nice, my diplomatic weight just jumped"
2. Player 1 does something → Player 2 sees the consequence and says
   "wait, that war just destabilized three factions"

These moments are the co-op magic. They are when two people on a couch
turn to each other and talk about their shared empire. Without them,
you have two people playing separate games on the same screen.

### Recommendation

For each wire in the cross-domain table, define the **player-facing feedback
moment**:

| Wire | System Event | Player-Facing Moment |
|---|---|---|
| Military → Political | War lost → stability drop | Player 2 gets a popup: "Military Defeat Shakes Parliament — Militarist faction demands inquiry" |
| Planet → Political | Prosperity rises → faction happy | Player 2 sees faction support shift in FP's parliament UI |
| Discovery → Diplomacy | Relic found → opinion shift | Player 1 gets a diplomatic message: "Empire X demands we share the Ancient Archive" |
| Espionage → Political | Spy caught → crisis | Player 2 gets a DPE event: "Foreign Espionage Scandal — Intelligence minister under pressure" |

If a wire doesn't produce a visible, unmissable player moment, it's
infrastructure — useful but not a design feature to highlight. The design
should distinguish between "system wires" (internal plumbing) and
"experience wires" (things players notice and react to).

**Critical for co-op:** Add a new subsection to SYSTEMS_MESH called
"Co-op Conversation Triggers" — moments explicitly designed to make
players talk to each other. "I just found a precursor relic — should we
trade it for alliance leverage or keep it?" "Factions are destabilizing
after that war — should we reform government or suppress dissent?"

---

## 3. Pacing — Front-load risk is real

**Rating: Early game is strong. Mid-game has a gap. Late game has a
vacuum problem.**

### Early game (2200-2230): GOOD

The design creates a natural explore-discover-colonize rhythm. Real Space
makes the galaxy visually interesting, All Anomaly Spawns guarantees
discoveries, PD's 50+ planet types make every colony feel unique. Both
players have clear early tasks.

Risk: 458 mods means a LOT of things spawn at once. Anomalies + events +
primitives + modifiers + dig sites all competing for attention. For a
non-gamer Player 2, the first 30 minutes could be overwhelming. (More on
this in section 6.)

### Mid-game (2230-2300): CONCERN

This is where Stellaris traditionally sags. The design has the right mods
to fill it — Factional Politics' parliamentary system, espionage operations,
federation evolution, archaeological dig chains. But:

- War in Heaven is guaranteed (Guaranteed 100% War in Heaven) — this is
  a MASSIVE mid-game event that completely dominates attention. When it
  fires, Player 1 is consumed with military/diplomatic crisis. What does
  Player 2 do during War in Heaven?
- The political pipeline (section 1 of SYSTEMS_MESH) is the best answer
  — faction upheaval, government reform pressure, potential civil war.
  But only if politics is explicitly Player 2's domain (see section 1 above).

The mid-game gap is a PACING problem, not a content problem. There's enough
content. The question is whether it fires at the right cadence. Dynamic
Political Events needs to fire MORE during crises, not less. Social
Decline's instability should ramp during wars, creating domestic pressure
that Player 2 must manage while Player 1 manages the front.

### Late game (2300+): VACUUM

The design says: "Automation handles routine building. Focus on archaeology
and diplomacy."

But archaeology runs out. Dig sites are finite. Once you've completed the
major chains, the discovery domain dries up. And diplomacy in late-game
Stellaris is either "crisis response" (Player 1's domain) or "galactic
community voting" (periodic, not sustained).

What does Player 2 do in year 2350?

The answer should be: **manage the political consequences of being a
late-game superpower.** A galactic empire with 50+ planets, multiple
species, factions pulling in different directions, vassals testing loyalty,
GC resolutions constraining expansion. This is where Factional Politics,
Social Decline, and Civil Wars should shine — but only if the late-game
political complexity scales with empire size.

### Recommendation

Add an explicit **pacing timeline** to SYSTEMS_MESH that maps when each
system reaches its peak engagement:

```
2200-2220: Exploration + first colonies (both players busy)
2220-2240: First wars + political formation (P1: fleet, P2: factions/building)
2240-2280: Federation era + archaeology golden age (P1: diplomacy, P2: digs + parliament)
2280-2320: War in Heaven + political crisis (P1: military, P2: domestic stability)
2320-2400: Crisis era + imperial politics (P1: crisis fleet, P2: empire cohesion)
2400+:     Endgame scoring / heritage legacy (both players: dynasty legacy)
```

Each phase should have explicit "Player 2 peak engagement" content. If a
phase doesn't have enough for Player 2, that's a design gap to address
before implementation.

---

## 4. Decision Density — Dilemmas vs Optimizations

**Rating: Player 1 has genuine dilemmas. Player 2 mostly has optimizations.**

### What makes a MEANINGFUL choice

A meaningful choice has:
- **Trade-offs** — choosing A means giving up B
- **Uncertainty** — you don't know the optimal answer
- **Consequences** — the choice changes future state
- **Irreversibility** — you can't easily undo it

### Player 1's decision landscape

The military-fleet and politics-diplomacy domains create genuine dilemmas:
- "Do I declare war now with a weaker fleet, or wait and risk the enemy
  getting stronger?" (trade-off + uncertainty)
- "Do I join this federation for tech sharing, or stay independent for
  flexibility?" (trade-off + irreversibility)
- "Do I spy on my ally to learn their plans, risking discovery?"
  (trade-off + consequence)
- "Which war goals do I pursue — total conquest or limited objectives?"
  (consequence + irreversibility)

These emerge naturally from the systems mesh wires. Military choices have
political consequences. Diplomatic choices constrain military options.
Espionage creates risk/reward dynamics.

### Player 2's decision landscape

The planet-economy domain creates mostly optimizations:
- "Should I build a mining district or a research district?" (optimization —
  there's usually a right answer based on current needs)
- "Should I specialize this colony or make it hybrid?" (mild trade-off,
  but Better Colony Automation knows the optimal answer)
- "Should I build the Planetary Wonder now or wait?" (timing optimization)
- "Should I uplift this primitive or observe longer?" (mild trade-off)

The archaeology domain is better — dig sites have branching outcomes with
real consequences. But digs are sparse and slow.

The POLITICS domain, if properly assigned to Player 2, creates genuine
dilemmas:
- "Factions are deadlocked — do I reform government or suppress dissent?"
  (trade-off + consequence + irreversibility)
- "A civil war is brewing — do I appease the rebels or crush them?"
  (trade-off + uncertainty)
- "An election is coming — do I support the popular candidate or install
  my dynasty's heir?" (if heritage is active: deep trade-off)
- "The GC is voting on restricting our expansion — do I lobby against it
  or comply?" (trade-off + consequence)

### Recommendation

For each domain, count the number of **dilemma moments** per game-hour
(not optimizations — genuine dilemmas). If Player 2's dilemma density is
less than half of Player 1's, the design has an engagement imbalance.

The systems mesh wires are supposed to CREATE dilemmas by making systems
interdependent. Wire 7 in the politics domain (Civil Wars from everything)
is the best example — five inputs creating five civil war types. This is
exactly the kind of design that produces dilemmas for Player 2: "The
empire is declining AND factions are deadlocked AND a vassal is disloyal —
which fire do I fight first?"

**The design needs more wires that create Player 2 dilemmas specifically.**
Examples:
- "Planet prosperity attracts immigration, but new species destabilize
  factions" (economy vs politics trade-off)
- "Automating this colony frees you to manage politics, but automated
  colonies produce less" (attention trade-off)
- "This archaeological discovery has political implications — reveal it
  publicly for prestige or suppress it for stability?" (discovery vs
  politics trade-off)

---

## 5. What's Missing for Co-op

### 5a. No communication framework

The design assumes players will naturally coordinate. But Stellaris's UI
doesn't help. Player 2 manages planets and politics. Player 1 manages
fleets and diplomacy. How do they COMMUNICATE about:

- **Dynasty succession:** Heritage mod's succession events affect both
  players. Who decides the heir? What if they disagree?
- **Government reform:** Player 2 manages politics, but government type
  affects Player 1's military doctrine (Wire: Government → Military in
  SYSTEMS_MESH). Who decides to reform?
- **War declaration:** Player 1 commands fleets, but war affects Player 2's
  stability. Does Player 2 get a say in whether to go to war?
- **GC voting:** Both players have stakes. How do they agree on a position?

In CK3 co-op, this coordination happens naturally because the game is
slower and more narrative. Stellaris moves faster and has more simultaneous
systems. The design should identify **mandatory co-op decision points** —
moments where both players MUST discuss before acting.

### 5b. Succession crisis asymmetry

The DLC_GUIDE.md notes that heritage's succession crisis is a core feature.
But succession affects Player 1 and Player 2 differently:

- Player 1 loses their fleet commander / diplomatic envoy when the leader
  changes. This is a MILITARY/DIPLOMATIC disruption.
- Player 2 loses their governor / faction alignment when the leader changes.
  This is a POLITICAL/ECONOMIC disruption.

Both feel the crisis, but in different ways. The design should explicitly
map how succession ripples through each player's domain. A good succession
crisis should force BOTH players into damage control simultaneously, then
force them to coordinate recovery. "You stabilize the factions, I'll
reassure our allies."

### 5c. No shared victory condition

Stellaris's victory conditions are galactic-scale: federation victory,
conquest victory, crisis survival. These are mostly Player 1 achievements.
What is Player 2's victory condition?

If Player 2 owns politics and planets, their "win" should be visible:
- Most stable empire in the galaxy (Social Decline score)
- Richest economy (GDP equivalent)
- Most archaeological discoveries
- Most successful dynasty (heritage score)

But Stellaris doesn't surface these as victories. The design should consider
a "dynasty legacy score" (heritage feature) that values BOTH military
achievement AND domestic prosperity, so both players contribute to the
final score.

### 5d. Turn-taking bottleneck

Both players share one empire in what is effectively a real-time game
(even if you pause constantly). The design doesn't address WHO CONTROLS
THE CLOCK. In practice:

- Player 1 needs time paused during fleet positioning and battle
- Player 2 needs time paused during political events and building decisions
- Both compete for the pause button

This is a UI/workflow problem, not a mod problem. But it affects game
design: systems that produce time-sensitive decisions (fleet battles,
event timers) should not fire simultaneously for both players. The pacing
design should stagger Player 1 and Player 2 crisis moments so they take
turns being "the one who needs the pause button."

---

## 6. Feature Overwhelm — The Non-Gamer Problem

**Rating: HIGH RISK. This is the most likely failure mode of the entire
project.**

### The numbers

458 mods. 50+ planet types. 160+ government configurations. 100+ planetary
modifiers. 125+ technologies from Guilli's alone. Three separate espionage
systems. A parliamentary system. A galactic community overhaul. Hybrid
colony designations. Planetary wonders. Sort orders. Tier numbers.

Player 2 is described as a non-gamer partner. This person has presumably
not played 500 hours of Stellaris. They may not have played any Stellaris.

### The onboarding catastrophe

The design documents contain zero onboarding strategy. The Player 2
Experience Design section in planet-economy.md says: "Survey → Colonize →
Build → Specialize → Wonder → Automate → Focus."

But what does the FIRST HOUR look like? Player 2 opens the planet
management screen and sees:
- PD's expanded planet types (50+ icons they've never seen)
- BPVR's expanded building slots (more slots than vanilla, unfamiliar layout)
- Guilli's planet modifiers (modifiers with names that mean nothing to a
  new player)
- Tier Numbers on buildings (numbers on icons they don't recognize)
- Sort Those Buildings auto-sorting into an order they didn't choose

This is information overload. A non-gamer will not read tooltips for 50
planet modifiers. They will feel lost and disengaged.

### The "what do I click" problem

Experienced Stellaris players have internalized decision heuristics:
"Early game, build alloy foundries. Mid-game, specialize colonies."
A non-gamer has none of these. The design adds 458 mods of new content
without any scaffolding for someone who doesn't have the base game
internalized.

Ultimate Automation and Better Colony Automation help — but they also
remove agency. If the automation makes good decisions, Player 2 has
nothing to do. If it makes bad decisions, Player 2 can't fix them
because they don't understand the system.

### What the design should address

**1. A progressive disclosure strategy.** Not all systems should be
visible from game start. The design should define which mods' content
appears when:
- Year 1: Vanilla-equivalent planet management. PD planet types visible
  but not overwhelming. Basic building. No hybrid designations.
- Year 10: First Guilli's modifiers appear. First archaeological dig.
  Player 2 has learned basic building by now.
- Year 30: Specialized zones unlock. Hybrid designations become relevant.
  Parliamentary politics from FP starts mattering.
- Year 50+: Planetary Wonders, advanced politics, automation options.

This isn't about hiding content — it's about not throwing everything at
a new player simultaneously.

**2. A "Player 2 cheat sheet."** A one-page reference that says:
- "When you see [green planet modifier], it means your colony is good at
  [thing]. Build [specific building]."
- "When a faction popup appears, here's what the three buttons mean."
- "When the parliament is deadlocked, here are your options."

The design can't rely on Stellaris tooltips because tooltips assume game
knowledge that Player 2 doesn't have.

**3. Rogue Servitor as training wheels.** The DLC_GUIDE.md suggests
Rogue Servitor for a non-gamer partner: "low military pressure, focus
on building and caretaking." This is smart but contradicts the dynasty
mod's core concept (bio-trophy pampering has no dynasty politics).

A better first-campaign approach: play a Xenophile Democracy where Player 2
manages a small, stable empire's planets and politics. Low external threat
(Player 1 handles diplomacy to avoid wars). Player 2 learns building and
faction management in a low-pressure environment. SECOND campaign: ramp up
complexity with the full stack.

**4. Automation as scaffolding, not replacement.** Ultimate Automation
should be configured to SUGGEST actions, not auto-execute them, at least
for Player 2's early experience. "Your colony on Vega III needs a research
lab. [Build it?]" This teaches decision-making while preventing the
"stare at screen, don't know what to do" paralysis.

### The honest risk assessment

If Player 2 sits down for their first session and is overwhelmed within
20 minutes, the entire co-op concept fails. No amount of elegant systems
mesh design matters if one of the two players isn't having fun. This is
the failure mode that needs the most design attention and gets the least
in the current documents.

---

## Summary of Recommendations

| # | Issue | Severity | Recommendation |
|---|---|---|---|
| 1 | Player 2 depth | HIGH | Elevate politics to co-primary domain. Define political experience arc per game phase. |
| 2 | Feedback visibility | MEDIUM | For each wire, define the player-facing moment. Add "Co-op Conversation Triggers" section. |
| 3 | Pacing gaps | MEDIUM | Create explicit pacing timeline. Ensure each phase has Player 2 peak engagement content. |
| 4 | Decision density imbalance | MEDIUM | Count dilemma moments per player per hour. Add wires that create Player 2 dilemmas. |
| 5 | Co-op communication gaps | HIGH | Define mandatory co-op decision points. Map succession crisis per-player impact. |
| 6 | Feature overwhelm / onboarding | CRITICAL | Progressive disclosure strategy. Player 2 cheat sheet. First-campaign difficulty ramp. |

### What the design gets RIGHT

- The systems mesh architecture is genuinely good. Un-siloing Stellaris is
  the correct goal and the wiring approach is sound.
- The AI coherence layer is thorough. Personality-driven AI behavior across
  all modded systems is ambitious and, if executed, would be exceptional.
- The civil war wire (5 inputs, 5 types) is the best single design element
  in the documents. More wires should aspire to this level of emergent
  richness.
- The "megapatch stands alone" principle is correct. Separating integration
  from heritage content is the right architectural decision.
- The DLC multiplier analysis is excellent. Each DLC amplifying existing
  connections rather than adding isolated features shows genuine design
  thinking.

### What needs work BEFORE implementation

1. Player 2's political role needs explicit ownership definition
2. Onboarding strategy for a non-gamer needs to exist AT ALL
3. Co-op decision points need enumeration
4. Late-game Player 2 engagement needs a concrete plan
5. Feedback visibility needs per-wire specification

None of these require new mods. They require clearer design intent about
how existing mods serve the co-op experience. The pieces are all there.
The assembly instructions are missing.
