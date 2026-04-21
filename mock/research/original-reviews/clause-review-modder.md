# Clause — Modder Perspective Review

**Date:** 2026-04-16  
**Reviewer role:** Stellaris mod author, years of experience, shipped large mods, familiar with cwtools.  
**Design doc reviewed:** `docs/megapatch/design/2026-04-16-mod-language-design-draft.md` (31 sections, ~1700 lines)  
**Supporting docs:** Clausewitz grammar formalization, local-docs findings, online research findings.

---

## 1. TL;DR

Would I actually use this? **Conditionally yes — but only after Phase 2 ships and I can see it work on a real mod.**

The pain points Clause targets are real and have cost real hours. The 5-depth call limit has bitten me. `@variable` collisions across mods are a genuine nightmare. Silent LIOS stomps on buildings files are the reason I stopped trusting my 448-mod setup without a full playtest. The design's instincts are right.

What gives me pause is not the concept but the execution risk. This is a language compiler being built on top of a backend pipeline that itself is still mid-construction (G2-G4 batches). The compiler has to correctly emit every Clausewitz construct that authors throw at it, and Clausewitz has enough undocumented edge cases that "95% unambiguous kind inference" will hit the remaining 5% constantly in a real 50,000-line mod. The type system is genuinely useful but the migration cliff — porting 50,000 lines of hand-authored Clausewitz to a new syntax — is enormous for any existing mod.

The verdict: build it, but build it for YOUR project first. The 448-mod megapatch is exactly the right proving ground. If Phase 2 passes and Bloodline runs byte-identical, the case for porting everything else becomes compelling. If that takes 8 months and the output is fragile, the community will never touch it.

The single most important thing this design gets right: the commitment to gradual adoption (Phase 1-5) and coexistence with raw Clausewitz. If that promise holds in practice, the risk profile is acceptable.

---

## 2. What Works for Modders

### 2.1 The call-graph depth analysis is genuinely valuable

The 5-level parameterized scripted_effect cap is one of the most insidious failure modes in Stellaris modding. It fails silently at depth 6 with no error in the log, no loading crash, just wrong behavior discovered hours into a playtest. The fact that this is a call-graph limit (not a brace depth limit) makes it doubly confusing because modders conflate the two all the time.

Clause's compile-time depth analysis, running on the IR before codegen, would catch this before you ever launch the game. That alone is worth significant friction. The design correctly identifies that `inline_script` bypasses the limit because it's parse-time text substitution, and the compiler's `#[prefer(inline_script)]` hint + auto-promotion for leaf functions directly addresses the RAM-multiplying parameterized-effect problem (§14.7, §1.4 of local-docs). This is not a hypothetical savings: at 1000+ call sites for a popular scripted_effect, you're talking 2GB+ RAM. The compiler emitting inline_script for appropriate leaf nodes is a real performance win for complex mods.

**Verdict:** This is the feature most likely to make a modder say "I would have caught that before the 3-hour playtest." It works.

### 2.2 Auto-namespacing for variables genuinely solves the collision problem

The `@<crate>_<struct>_<field>` naming scheme (§8) addresses a real cross-mod problem. Today, when two mods both define `@pop_size_limit` in their scripted variables, whichever loads first wins with no diagnostic anywhere (FIOS on common/scripted_variables/). Authors have no way to know about the collision until a behavior difference surfaces in-game.

With Clause, every struct field compiles to a namespaced variable name. The `heritage_bloodline_size` pattern means two different mods' bloodline systems can coexist without stomping each other's state. This is not academic — the megapatch already encounters this problem across ~3,984 cross-mod conflicts.

The only caveat: external mod authors who don't use Clause still export flat namespaces. Clause solves the problem for CLAUSE-authored mods interacting with each other. The megapatch still has to handle legacy mods writing to unnamespaced variables. This is correctly scoped as a partial solution, not a total one.

### 2.3 The manifest-managed event ID system is the right approach

Manual event ID management is busywork that scales badly. Today, every mod carves out a namespace like `my_mod.dynasty.0001` through `my_mod.dynasty.0099` by convention, and authors have to manually ensure uniqueness. Conflicts don't show up until in-game behavior breaks.

The auto-allocated ID from module path (`heritage::events::dynasty` → `heritage.dynasty.new_heir.0001`) plus the frozen manifest means:
- IDs are stable across builds (manifest file in git)
- The namespace is derived structurally, so two mods with different module paths can't collide
- Reviewable in PR diffs (the manifest is human-readable YAML)

The sham-handler system for save compatibility on renames (§16.4) addresses a real pain point. When you rename a scripted_effect that's been in saves, existing saves reference the old name and silently break on load. The auto-scaffolded sham that wires into `on_single_player_save_game_load` is exactly what authors have to write by hand today, and most don't.

One genuine concern: the design correctly identifies that `on_single_player_save_game_load` does NOT fire in multiplayer. The sham system wires into both hooks. That's right — it's documented in the online research findings (§1.7 of local-docs, G3 in the gotchas). The question is whether the auto-scaffolded sham handles the case where a save was made mid-multiplayer session. This edge case needs explicit coverage in the sham generator.

### 2.4 Orphan rule / coherence as conflict detection

The mechanical enforcement of "two mods can't both impl the same Trait for the same Type without an explicit consolidation" (§10.1) is the most interesting idea in the design from a megapatch perspective. Today, two patch mods defining conflicting overrides for the same vanilla building type produce a silent LIOS stomp. Clause turns that into a compile error.

The `#[patch]` decorator as the only legal orphan-rule bypass, with mandatory `reason` field, means every intentional override is documented. For the megapatch use case (4,500+ cross-mod conflicts), this transforms implicit behavioral choices into auditable code decisions. The consolidate-not-choose principle from the project guidelines is mechanically enforced by the type system.

This is architecturally sound. The question (addressed in §7 below) is whether it scales to 4,500 patches in practice.

### 2.5 Source mapping comments in emitted Clausewitz

The `# >> heritage/bloodline.cw:42` comments in emitted Clausewitz (§18.4, §27.1) are the right answer to the "something broke in-game, now what?" problem. When a modder opens Stellaris' game.log and sees `heritage_bloodline_add_member` failing, they need to find the source. With source mapping, you grep the emit file, find the comment, and jump directly to the `.cw` file. No more tracing auto-generated variable names back to their declaration.

The `source_map.json` structured file (§27.2) enables IDE integrations and more sophisticated debuggers down the line. Getting this right in the initial implementation is important because retrofitting source mapping is painful.

### 2.6 `clause explain` as a unified inspector

The `clause explain <target>` command that handles file:line, item names, AND emit-position lookups (§27.3, §27.4) is the right UX. Today, debugging a megapatch interaction requires tracing through multiple files manually. The ability to ask "what source produced this emitted line?" and "what does this item compile to?" from a single command covers the two most common debugging workflows.

Elm-style error messages (first-person, fix-suggesting) are the right call for Clause's audience. Modders are not compiler engineers. An error like "I can't figure out whether this method is a trigger or an effect — add `#[prefer(scripted_trigger)]` if you're checking a condition" is better than "ambiguous emit context in transpile phase."

---

## 3. What Concerns Modders

### 3.1 The 95% kind inference leaves a 5% landmine field

The design's kind inference (§9.4) claims ~95% unambiguous classification. In an authoring language for a domain with as many edge cases as Clausewitz, that 5% will be encountered regularly. The `#[as_scripted_effect]`/`#[as_scripted_trigger]`/`#[as_script_value]` override decorators exist for this, but authors shouldn't have to think about override decorators routinely.

The edge cases that will surface constantly:
- Mixed trigger/effect patterns. Any method that checks a trigger AND executes an effect (which is the majority of real event code) will hit the "mixed — ambiguous" error.
- `switch = { trigger = ... }` is both a trigger-context block and an effect-dispatching construct. Kind inference will misclassify this without special handling.
- Methods that conditionally return early (common in guard patterns) mix `return Err(...)` effects with pure condition checks.

The design acknowledges "edge cases use explicit override decorators" but doesn't quantify how many of those edge cases a typical Heritage event file would encounter. If an author is adding `#[as_scripted_effect]` to one in three methods, the ergonomics break down. This needs empirical validation during Phase 2 (Bloodline migration) — count the annotation rate and report it.

### 3.2 The body syntax tension: Rust-ish vs raw Clausewitz

The design says method bodies "look like Clausewitz with type validation" but the worked example (§19) shows a different reality. The `immediate = { ... }` block of `event NewHeir` mixes:
- Rust-ish `let new_pop = create_pop(...)` with named arguments
- Native Clausewitz `any_pop = { limit = { is_same_value = $who$ } ... }` embedded directly
- Rust `match ... { Ok(idx) => ..., Err(...) => ... }` patterns

This is three different syntactic registers in the same method body. An author writing their first Clause event is going to be confused about when to use Rust-ish syntax, when to use raw Clausewitz, and where the boundary is.

The `name(|p| { p.foo })` lambda syntax for scope-opening (§18.3: `any_pop = |p| { p.is_jobless() }`) is specifically concerning. The lambda here desugars to a scope-opening block, which is Clausewitz's `any_pop = { ... }` pattern. This mapping is non-obvious. A modder who knows Clausewitz but not Rust will not understand `|p|` as "open this scope with p bound to the iterated entity." A modder who knows Rust will understand the lambda syntax but may not understand it desugars to a scope block rather than a function call.

The design notes this was "settled in chunk 2" but settlements in design docs don't mean the UX is actually good. This is something that needs user testing with actual modders before it's locked in.

### 3.3 PascalCase types are a community convention mismatch

Stellaris modding convention is `snake_case` for everything. Event IDs, scripted effect names, building type names, trait keys — all lowercase snake_case, universally. The design introduces PascalCase types (`Bloodline`, `HasFleets`, `Country`, `NewHeir`) which is correct for a Rust-inspired type system, but creates a two-register authoring experience where you write `struct Bloodline` but the compiled output is `heritage_bloodline_add_member`.

This isn't a fatal problem, but it's friction. Authors will need to internalize "PascalCase is Clause syntax, snake_case is what Stellaris sees" as a permanent mental model. The emitted names are correct (auto-generated snake_case), but the source looks different from both raw Clausewitz AND from the debug output they see in game.log. This mental mapping cost compounds when debugging.

Event PascalCase (`event NewHeir for Country`) is particularly jarring because event names in vanilla and mod files are all lowercase (`country_event = { id = heritage.dynasty.0001 }`). A modder reading their Clause source and then reading the emitted Clausewitz will need to translate mentally between the two forms constantly.

### 3.4 The `Registry<Self>` default is too magical for novice authors

The default bind target `Registry<Self>` (§7) is the right choice for complex structs like `Bloodline` that represent logical entities not tied to a vanilla scope. But the consequences — "the compiler allocates a hidden engine entity to hold instances" — are entirely invisible to authors. This is intentional (Tenet 11), but it creates a debugging problem.

When something goes wrong with a Registry-bound struct (and things will go wrong), the author looking at game.log will see references to a hidden country or fleet-rename slots they never authored. Without deep knowledge of how Registry works under the hood, they cannot diagnose the failure. The `clause explain` tool partially addresses this, but only if authors know to use it.

More concretely: if the hidden Registry country is accidentally deleted by an event (common in modding — events delete countries, especially in crises), the entire struct becomes inconsistent with no diagnostic. The design doesn't address this class of failure.

### 3.5 The expect/actual system requires ecosystem buy-in to be useful

The `expect`/`actual` cross-mod runtime contract system (§20.3) is elegant but only works if other mods publish their own `.cw` extern declarations. Today, NSC3, ESC, ACOT, and every other major mod ship raw Clausewitz with zero typed contracts. The `use nsc3::*` pattern requires that someone has written and published an NSC3 extern overlay package.

Until that ecosystem exists, Clause-authored mods interacting with external mods must fall back to raw Clausewitz calls or manually authored extern declarations. The `stellaris-vanilla-spec` crate handles the vanilla surface, but the ~216 conditional mods in the project's own modlist each need their own extern declarations before the type system catches cross-mod errors.

This is not a design flaw — it's a bootstrap problem every ecosystem language faces. TypeScript had the same problem with `@types/*` packages. But it means the type safety story for cross-mod interactions is incomplete for years, and authors need to understand where the guarantees end.

### 3.6 FIOS vs LIOS routing is invisible to authors

The design's file routing (§22) correctly separates output files by item kind and crate, but doesn't directly surface FIOS vs LIOS semantics. An author writing a `building_type` in Clause needs to understand that buildings use LIOS semantics — last-loaded wins — but the routing rules don't communicate this at the source level.

When two Clause-authored crates both define patches for the same vanilla building, the coherence rule catches it. But when a Clause-authored crate and a raw-Clausewitz mod define competing buildings, the LIOS stomp is still silent. Authors need to understand that `#[file("...")]` escape hatches interact with load order, and that the auto-routing heuristics pick output paths that might not match their intended load-order behavior.

The design mentions `#[file("...")]` is needed for "vanilla-replacement mods where exact filename affects load order" (§22.6). This is accurate but undersells the importance — filename-based load order manipulation is how modders guarantee their changes win. Making this an escape hatch instead of a first-class concern risks modders either misusing it or not using it when they should.

---

## 4. Migration Story

### 4.1 The raw numbers

Heritage is at ~50,000 lines, 43 files, ~300 events. The megapatch is ~4,500 cross-mod patches. Porting either to Clause is measured in person-months, not person-hours.

The migration plan (§28) is realistic in its estimates: Phase 1 (MVP) at 4-8 weeks, Phase 2 (Bloodline PoC) at 2-3 weeks, Phase 3 (Heritage full) at 2-4 months. What it doesn't account for is the ramp-up cost of authoring in a new language during Phase 3. The first 500 lines of Clause will take three times as long as the last 500 lines. Realistic total for Heritage: 4-6 months at part-time pace.

For an existing mod author who didn't build the compiler: the migration cost is pure rework with no new feature value during migration. The only incentive is the long-term maintenance benefits (compile-time errors, refactoring safety, namespace isolation). That's a legitimate incentive but it requires trust that the compiler actually delivers.

### 4.2 The coexistence promise is critical

The design's gradual adoption promise — Clause coexists with YAML patches and raw Clausewitz through Phase 4 — is what makes migration viable. If authors had to port everything before getting any benefit, nobody would migrate. The ability to port Bloodline in Phase 2 while leaving the other Heritage subsystems in raw Clausewitz, and still get type checking on the Clause portion, is the right model.

What happens post-Phase 5 (YAML retirement)? The design says YAML is retired but explicitly notes "localisation `.yml` files still exist." Does raw Clausewitz also survive? The design implies the answer is yes — `#[file("...")]` escape hatch, extern declarations for vanilla, raw Clausewitz entering the backend alongside Clause IR. But this should be stated explicitly. If raw Clausewitz is permanently supported as a coexistence mode, the migration story for existing mods is "port what you want, leave what you don't, get benefits proportional to what you port." That's a viable community adoption story.

If raw Clausewitz is sunset in Phase 5, the story is "port everything or get left behind." That's a community-fracturing move that will kill adoption.

### 4.3 What the megapatch migration actually looks like

The design (§28 Phase 4) says the ~4,500 patches "move to Clause `#[patch]` and `#[patch_extend]` impls." Let's be concrete about what that means for the existing YAML patches:

- `prefer_mod` patches: These become trait impls in the megapatch crate. Each one requires an author to read both source versions, write a consolidated `impl` that's more complete than either, and document the `reason`. At even 5 minutes per patch, that's ~375 person-hours for the prefer_mod corpus alone.
- The 18 buildings and 4 tech that already need consolidation (from the prefer_mod audit) will require the kind of careful per-entry review the project's no-blanket-patches principle demands. These are the hard cases.
- `prefer_mod` patches that are truly formatting-only differences (functionally identical) can be mechanically translated. But the design explicitly requires per-entry review rather than bulk sweeps. This is the right principle but it means no automation shortcut.

The migration tooling mentioned ("automatic translator" from YAML to DSL in Phase 5 pre-retirement) suggests there's a mechanical translation path. But the design's conflict-resolution principles say even the mechanical cases need a `note` demonstrating understanding of what each entry does. A translator that produces Clause syntax with stub `reason` strings that authors must fill in is realistic; a translator that skips that step would violate the design's own principles.

### 4.4 The "two-tier ecosystem" risk

If Clause gets adopted broadly within this project but not by the wider modding community, the megapatch creates a two-tier ecosystem: Clause-aware mods get type-checked interactions with the megapatch crate, and raw-Clausewitz mods get handled via the extern overlay mechanism. This is manageable as long as the extern overlays are maintained.

The risk is maintenance burden: when NSC3 ships a major update, the NSC3 extern overlay needs updating before the type checker can validate interactions with it. If the overlay is stale, authors get "unknown method on NSC3 type" errors that are false positives. This is the TypeScript `@types/*` problem — it's manageable but creates a continuous maintenance obligation.

---

## 5. Missing Features

### 5.1 First-class FIOS/LIOS semantics

Load-order semantics are not surface-level concepts in the current design. There should be a `#[fios]` / `#[lios]` attribute that communicates explicitly which override semantics apply to an output file, with a lint if two items in the same output directory have conflicting intents. Authors today deliberately choose filenames like `!override_buildings.txt` to force load order. Clause should provide a typed equivalent.

### 5.2 Localisation integration

The design explicitly excludes localisation (§4: "Replacing localisation `.yml` files — different parser, different problem"). This is pragmatically correct for v1, but it creates an authoring split: events are authored in Clause but their titles and descriptions are raw `.yml`. Every time you rename an event in Clause, you must also update the `.yml` key manually.

At minimum, the compile-time validation should check that declared event `title = auto` keys exist in the localisation files. A missing loc key for an event is a common silent failure (the game shows a raw key string instead of text). This is a lint-level feature, not a language feature, and it should be in scope for Phase 2 or 3.

### 5.3 Triggered description patterns

Heritage's current design requires `triggered_desc` and triggered GFX for species/ethic/mode-aware variety (from the project memory: "ALL events must use triggered_desc + triggered GFX"). The Clause event model (§15) doesn't mention `triggered_desc` at all. An event system for a mod with this requirement needs:
- A way to declare triggered description variants in the event definition
- Compile-time checking that at least one `triggered_desc` variant exists when the event has mode-conditional content

This is a gap between the design and the project's actual authoring requirements.

### 5.4 On-action registration

Many of Heritage's events wire into `on_action` hooks (`on_leader_added`, `on_pop_created`, etc.). The current design has `#[on_action("yearly_pulse")]` as an event decorator (§21.6). But complex mods often need multiple on_action registrations for the same event and conditional registration (only wire this on_action if feature X is enabled). The current decorator doesn't support conditional on_action registration — that needs `#[cfg(feature = "...")]` + `#[on_action("...")]` composition that isn't shown in the design.

### 5.5 Scripted modifiers and static modifiers

The design covers `struct Modifier<T>` and `HasModifier<T>` (§14.4) but doesn't cover the full complexity of Stellaris modifier definitions. Static modifiers (defined in `common/static_modifiers/`) vs scripted modifiers (defined in `common/scripted_modifiers/`) have different structures and different emitted Clausewitz. The design's Modifier type appears to be a reference to an existing modifier, not a declaration of one. If authors need to DEFINE modifiers (which Heritage does extensively), that authoring surface isn't in the design.

### 5.6 GUI/interface file handling

GFX and interface files are mentioned briefly (§22.5) with decorators like `#[gfx_sprite_type(name = "...")]`. But interface `.gui` files are a completely different syntax from Clausewitz — they have their own grammar. The design doesn't address how Clause interacts with interface files beyond routing. For portrait mods, GFX-heavy mods, or any mod that modifies the UI, this is a significant authoring gap.

### 5.7 A migration scaffolding tool

The design has `clause new <kind>` for scaffolding new items (§27.3). There is no `clause migrate <file>` for converting existing Clausewitz files to `.cw` source. A mechanical converter that produces rough Clause syntax from raw Clausewitz — even one that requires manual cleanup — would dramatically lower the migration friction. The converter doesn't have to be perfect; it just needs to produce syntactically valid Clause that compiles to something close to the original.

This is especially important for the megapatch migration. Converting 4,500 YAML patches by hand is the bottleneck. A converter that produces `#[patch]` impls with `// TODO: consolidate` comments and stub `reason` fields would save hundreds of hours.

---

## 6. Comparison: Clause vs Raw Clausewitz + YAML Patches + cwtools

### Raw Clausewitz + YAML patches + cwtools (current state)

**Strengths:**
- Zero new tooling investment: every Stellaris modder already knows this workflow
- cwtools provides schema validation and `event_target:` tracking in most IDEs (VSCode extension is the standard)
- Raw Clausewitz is directly readable by Stellaris — no compile step, immediate iteration
- Community ecosystem: documentation, examples, forum posts all reference raw Clausewitz syntax

**Weaknesses:**
- Typos only surface in playtest, 3 hours in
- Call-graph depth failures are silent
- `@variable` scoping is the root of a documented class of bugs (the `@barrenBiomassCostCheap` exemplar)
- Cross-mod variable collisions have no diagnostic
- Refactoring requires grep + manual verification
- LIOS/FIOS conflicts are silent without the megapatch compiler
- Save compatibility on renames is ad-hoc

**cwtools specifically:**
- Provides useful schema validation but has known gaps: issue #64 (@ disambiguation), issue #54 (`@[expr]` opaque), issue #57 (`[[PARAM]]` unsupported), issue #73 (FROM.FROM normalization). These are documented in the local-docs findings.
- Schema coverage requires the `cwtools-stellaris-config` community repo to be up-to-date, which lags major Stellaris updates
- No call-graph depth checking
- No cross-mod collision detection

**Honest verdict:** cwtools + raw Clausewitz is good enough for small mods with a single author. It breaks down at the scale this project operates at (448 mods, 4,500 conflicts, 50,000+ lines of Heritage).

### Clause (proposed)

**Strengths (if the design delivers):**
- Compile-time depth checking eliminates the silent 5-level failure
- Namespace isolation eliminates cross-mod variable collisions (for Clause-authored mods)
- Coherence check catches silent LIOS stomps at compile time
- Source mapping makes in-game debugging traceable
- Manifest-managed IDs and sham handlers provide save compatibility without manual work
- Refactoring is compiler-guided rather than grep-based
- The kind inference (when it works) reduces boilerplate dramatically
- `clause explain` is better debugging UX than anything cwtools provides

**Weaknesses:**
- Massive authoring investment: learning a new language + new toolchain
- Compiler must be maintained as Stellaris updates break the vanilla spec
- 5% kind-inference failure rate will be encountered constantly in complex mods
- Community ecosystem starts at zero: no tutorials, no forum posts, no examples outside this project
- External mod authors not using Clause create ongoing extern overlay maintenance burden
- The lambda scope-opening syntax is non-obvious
- Performance is unproven at megapatch scale (4,500 patches through the full pipeline)
- Debugging Clause-emitted Clausewitz requires understanding both the source AND the emit model

**Honest verdict:** Clause is better than the status quo for this project's specific use case. It is not clearly better for a standard mod author working alone on a 5,000-line mod. The break-even point where Clause saves more time than it costs to learn is probably around 20,000+ lines of authored content with active cross-mod interactions.

---

## 7. Gate Questions

Before writing the first line of the Clause compiler (M2 — the lexer), the project should answer these questions. Not blocking M2 start technically, but important for avoiding expensive pivots later.

### 7.1 What is the kind-inference failure rate on real Heritage code?

Take a representative 500-line sample of current Heritage `.txt` files (events, effects, triggers). Manually classify each function into trigger / effect / value / ambiguous. Count the ambiguous cases. If more than 10% require explicit override decorators, the inference system needs more heuristics before authors can comfortably use it. This can be done before the compiler exists by reading the grammar doc's inference rules and applying them by hand.

### 7.2 Does the lambda scope-opening syntax survive user testing?

The `name(|p| { p.foo })` → `name = { foo = ... }` mapping (§18.3, §24.4) is the most syntactically novel element of Clause. Find three modders who don't know Rust. Show them this syntax. Ask them what it does. If none of them can correctly describe it in 5 minutes, revise the syntax before it's locked into the compiler.

Alternative worth considering: make the scope-opening explicit with a keyword. `name with p { p.foo }` is less Rust-idiomatic but more self-documenting for a Clausewitz-familiar author.

### 7.3 What is the plan for external mod extern overlays?

The design mentions `nsc3-compat` as a future crate. Who writes it? Who maintains it when NSC3 updates? If this project writes it and NSC3 doesn't adopt Clause, you own an N-mod compatibility burden indefinitely. Establish the maintenance model before the feature is depended upon.

### 7.4 What happens when the Stellaris vanilla spec is wrong?

The `stellaris-vanilla-spec` crate is derived from cwtools-stellaris-config and trigger_docs logs. When a Clause author calls a vanilla effect that the spec declares incorrectly (wrong scope, wrong parameter type, missing variant), they get a compile error for valid code. This is a false-positive compiler error. What is the workflow for reporting and fixing spec bugs? How fast can they be patched? A single incorrect spec entry blocking the build of a large mod for days is a tooling-trust killer.

### 7.5 Is byte-identical output actually achievable for the Bloodline PoC?

The Phase 2 acceptance gate is "byte-identical output vs the existing Clausewitz." This is a hard target. The emitted Clausewitz will have:
- Different variable naming (auto-generated vs manually named)
- Source-mapping comments that don't exist in the hand-authored version
- Potentially different file layout (compiler-routed vs author-placed)

"Byte-identical" is probably not achievable — "semantically identical and diff-approved" is more honest. Clarifying the acceptance gate before Phase 2 starts avoids a false-failure scenario where the Clause-emitted output is correct but not byte-for-byte equal.

### 7.6 What does the compilation performance look like at megapatch scale?

The megapatch backend currently handles ~448 mods worth of Clausewitz through the pass framework. Adding the Clause frontend (lex → parse → typecheck → transpile → codegen) for 50,000 lines of Heritage content plus 4,500 patch definitions needs to complete in a time that doesn't break the author's iteration loop. If `clause check` takes 30 seconds for a type-check-only run on Heritage, it's usable. If it takes 5 minutes, it's not. Get a runtime estimate on the Bloodline subsystem (~1,000 lines) during Phase 2 and extrapolate.

### 7.7 Who maintains Clause when this project goes dormant?

Every large modding tool that doesn't get community adoption dies when its author moves on. The design mentions "optionally spin out to standalone repo" in Phase 5. What's the contingency if the author stops working on Stellaris before Phase 5? Are the compiler artifacts (emitted Clausewitz in `.dist/`) sufficient to keep the mod loading even if the Clause toolchain can't be run? The answer should be yes — Clause compiles to raw Clausewitz which Stellaris loads without knowing Clause exists. But this should be explicitly verified: can you freeze the `.dist/` output and keep a mod working without ever re-running `clause build`?

---

## 8. Specific Answers to the Brief's Questions

### 5-depth call-graph limit: does Clause's compile-time depth analysis actually help?

Yes, materially. This is the most unambiguous win in the design. The depth_analyzer already exists in the megapatch pipeline; Clause's frontend gives it richer call-graph information than the regex-based approach. The `inline_script` auto-promotion makes the optimization path automatic instead of manual. Real win.

### Variable name collisions: does auto-naming fix this?

For Clause-authored mods, yes. For interactions with legacy raw-Clausewitz mods, no. Partial solution correctly scoped.

### Event ID conflicts: does manifest-managed IDs work?

For mods that use Clause, yes. The cross-mod compatibility question is whether other mods can still reference `heritage.dynasty.new_heir.0001` from raw Clausewitz — and the answer is yes, because the emitted Clausewitz uses the allocated ID. The only risk is ID instability if the manifest is corrupted or the module path changes. The manifest is compiler-managed and git-tracked, so this is low risk if the author reviews manifest diffs in PRs.

### @variable confusion: does Registry<Self> default + extern vanilla scopes address it?

Mostly yes for the `@variable`-as-item confusion. The compiler knows a struct field named `size` compiles to `@heritage_bloodline_size`, not to a top-level item named `heritage_bloodline_size`. The confusion documented in the grammar doc (items like `@barrenBiomassCostCheap` entering the items table) cannot happen in Clause because the author never writes bare `@` declarations.

The residual issue: when reading emitted Clausewitz, the `@heritage_bloodline_size` variables look like file-scoped constants, but they're actually compiler-allocated global variables. This is a documentation/training issue, not a design flaw.

### inline_script 2MB-per-effect issue: does `#[prefer(inline_script)]` deliver measurable savings?

Potentially yes, but this needs empirical validation. The theoretical argument is sound: converting leaf scripted_effects to inline_script eliminates the ~30-branch pre-compilation overhead. The compiler's auto-promotion for appropriate leaves is the right heuristic. But "measurable savings at scale" requires a benchmark on the actual megapatch output. The existing `leaf_inliner.py` provides a baseline for comparison.

### Save/load compatibility on renames: does the sham-handler system cover real-world scenarios?

For clean renames (same structure, new name), yes. For structural changes (field removed, type changed, logic inverted), the auto-scaffolded sham covers the trivial case but requires author extension for non-trivial migrations. The workflow (build warning → author adds `#[supersedes]` annotation → compiler writes sham) is the right friction: it catches the case without being fully automatic.

The gap: multiplayer save compatibility. The design wires shams into both `on_single_player_save_game_load` and `on_multiplayer_game_loaded`, which is correct. But the timing of when these fire relative to game state initialization matters — if a sham migrates variables before the game has initialized scopes, it can fail silently. This needs testing on the Bloodline subsystem before being declared production-ready.

### Does the Clause patch model scale to 4,500 patches?

Syntactically, the `super.foo()` + `..super` struct-spread model (§23.1, §23.2) is more expressive than YAML and enables the consolidate-not-choose principle mechanically. Ergonomically, writing 4,500 `#[patch]` impls is still 4,500 individual review tasks. Clause doesn't eliminate the work; it makes the work more structured and more auditable.

The question of whether it's more ergonomic than YAML for this volume: probably yes for complex patches (method overrides with multi-impl consolidation), probably neutral for simple value patches. The real gain is that every patch has machine-verifiable structure, so regressions are caught at compile time rather than in playtest.

### What common mod patterns are awkward or impossible in Clause?

- GFX-heavy portrait mods: no first-class portrait/mesh/shader authoring. Clause routes GFX declarations (§22.5) but portrait mods require asset pipeline integration that's out of scope.
- Localisation-only mods: explicitly out of scope (§4). Clause doesn't touch `.yml`.
- Mods that use aggressive `#file` load-order manipulation: the escape hatch `#[file("...")]` exists but is lint-warned as unusual. Mods that depend on load order as a feature (common in LIOS directories) will get lint noise.
- Mods that need `[[PARAM]body]` conditional inline_script blocks: the grammar doc confirms Clause must pass these through verbatim (the engine handles them at load time). But authoring them in Clause bodies isn't shown. If a method body needs a conditional parameter block, the author has to drop to raw Clausewitz syntax, which breaks the Rust-ish body model.
- Mods that reference vanilla entities by their raw string keys (e.g., `modifier = "trait_natural_engineers"` in an event body) without going through the type system: the extern spec covers declared types, but ad-hoc string references to vanilla content will produce warnings until the spec covers that specific item.

---

*Review complete. Core recommendation: build Phase 1-2, get empirical data on kind-inference failure rate and compilation performance, answer the gate questions before committing to Phase 3.*
