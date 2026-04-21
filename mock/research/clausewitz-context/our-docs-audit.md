# Our Docs Audit — Clausewitz/Stellaris Scripting Knowledge
**Date:** 2026-04-17
**Purpose:** Identify wrong statements, omissions, inconsistencies, duplications, and outdated
information across all project docs that contain Clausewitz/Stellaris scripting knowledge.
**Output driving:** consolidation into a unified `docs/megapatch/clausewitz-scripting-api-reference.md`.

---

## Executive Summary

The project has accumulated solid, deeply-researched Clausewitz knowledge across a dozen documents.
The research layer (`.cache/reviews/2026-04-16/`) is exemplary — the online-research and cwtools
review docs are among the most thorough community-sourced analyses of the Clausewitz engine I have
seen. However that depth has not yet propagated uniformly. Several critical problems stand out:

**Biggest concerns, in order of impact:**

1. **`?=` operator status is contradicted between three docs.** `clausewitz-local-docs-findings.md`
   calls it "not confirmed in vanilla" and treats it as absent. `clausewitz-online-research-findings.md`
   confirms it via PDX Tools and CK3 docs. `clausewitz-grammar.md` marks it as "Originally thought
   absent; updated" — but the local-docs findings doc (the supposed synthesis) still shows the
   old "not confirmed" position. Any consumer of the local-docs doc will implement wrong behavior.

2. **`inline_script` path inconsistency: `common/inline_tools/` vs `common/inline_scripts/`.** These
   are different directories. Heritage docs consistently say `common/inline_tools/`; megapatch
   compiler and `PLAYTEST.md` say `common/inline_scripts/`. One of these is wrong for Stellaris
   vanilla, and neither doc acknowledges the split. This will silently break inline_script resolution
   in the compiler if the wrong path is used.

3. **The 5-depth limit is described with three different framings** (call-graph cap, brace-nesting
   cap, generic "stack depth") across multiple docs. Only the grammar doc is precise. Heritage
   GUIDELINES.md gets it right. LINT_CATALOG.md slightly muddies it by describing it as "deeper
   than 5 levels" for both trigger brace-depth (fires at 6) and call-graph depth (cap is 5).

4. **`PATCH_FLOW.md` is a design-era document that has been superseded.** It describes a 4-phase
   pipeline (`generated/`, `patches/`, `bridges/`, `output/`) and a patch format discussion
   (Option A/B/C) that do not match the actual item-centric compiler architecture described in
   `PATCH_AUTHORING.md` and `ARCHITECTURE_V2.md`. It will mislead any maintainer who reads it.

5. **Clause language design doc (`mod-language-design-draft.md`) makes several implicit Clausewitz
   claims** — particularly about `inline_script` semantics, `$PARAM$` scope, and scripted_effect
   call-depth — that are accurate, but the doc does not cite sources. If the claims diverge from
   the authoritative research docs, there is no way to detect the divergence.

**Top recommendations:**

- Use `clausewitz-online-research-findings.md` as the single source for operator set, scope rules,
  and iterator semantics. It cites actual sources with URLs.
- Use `clausewitz-grammar.md` as the single source for token taxonomy. It integrates all three
  research docs.
- Resolve the `inline_tools` vs `inline_scripts` directory discrepancy before the compiler
  implements inline_script resolution. Check actual Stellaris vanilla files.
- Retire or clearly date-stamp `PATCH_FLOW.md` as superseded design history.
- Write the unified reference so the `clausewitz-local-docs-findings.md` "open questions" section
  is replaced by resolved facts, and the `?=` contradiction is settled with a single canonical
  statement.

**Overall state:** High quality research, incomplete propagation. The unified reference will save
significant re-lookup time and prevent the contradictions from causing implementation bugs.

---

## Per-Document Audit

---

### 1. `docs/megapatch/design/2026-04-16-clausewitz-grammar.md`

**Summary:** The token taxonomy and grammar formalization design for the new Clausewitz lexer. Target
audience: the compiler implementers who will write G2/G3 batches. Produced by integrating three
research sources (local docs, online research, cwtools review). 923 lines.

**Scripting content:** Comprehensive. Covers the full token taxonomy, all 8 operators, scope
keywords, boolean logic keywords, control flow keywords, inline arithmetic, macro/parameter syntax,
color literals, trivia handling, `@variable` semantics, scope chain rules, known engine constraints
(depth limits, `@[expr]` first-only rule, `$PARAM$` in comments), iterator semantics, performance
constraints, file-format constraints.

**Wrong statements:**

- **§3.2, `OP_DEFAULT`:** States `?=` was "Originally thought absent in vanilla; updated" — the
  inline note is correct but the framing "originally thought absent" is inconsistent with the
  online-research doc which found a concrete Stellaris example (`c:RUS ?= this`). Not wrong per se,
  but the "originally thought absent" phrasing propagated to local-docs-findings (Q5) and may still
  confuse future readers.
- **§3.6, Maximum chain depth note:** References "CK2 documentation: Maximum 4 FROMs as of patch
  2.3" and describes this as "engine-generic." The online-research doc is more careful, noting this
  may be CK2-specific. The grammar doc treats it as authoritative for Stellaris without a caveat
  that Stellaris-specific documentation on this point is absent.
- **§10, Performance constraints, while loop:** "1000-iteration cap (CK3 confirmed; Stellaris
  presumed same)" — this is correct epistemic hedging but should be more prominent. The doc buries
  the uncertainty in parentheses; it should flag this as unconfirmed for Stellaris specifically.
- **§6.3 `[[PARAM]body]`:** The explanation calls these "conditional macro blocks" but does not
  clarify the distinction between `[[PARAM]content]` used in scripted_effects and the similar
  `[[PARAM|text]]` form used in localisation. The online research (§1.7.A) explicitly distinguishes
  these. The grammar doc is silent on this distinction, leaving the impression they are the same
  construct.

**Missing content:**

- No mention of `hidden:` prefix on effect keys (cwtools review §7j). `hidden:add_modifier` is a
  valid construct that strips the `hidden:` prefix before scope resolution. The lexer taxonomy has
  no plan for this.
- No mention of the `type`/`types` keyword syntax (cwtools review §7e). `type NAME = BASE { ... }`
  and `types NAME { ... }` are real Clausewitz syntax visible in `cwtools-stellaris-config` and
  vanilla CWT files. The grammar doc would produce `INVALID` tokens for these.
- No section on `save_event_target_as = name@scope` (dynamic event target syntax, v3.5+). The
  `@scope` suffix on a non-leading position is mentioned in §3.1 (`TARGETED_VAR`) but the save
  syntax is not covered as a distinct case.
- `DEEP_SCOPE_CHAIN` lint trigger threshold: the grammar doc says the linter fires at 4+ `prev.`
  hops but does not explain that the lint threshold (4) and the documented engine hard limit (also 4)
  happen to be the same number by design — making it easy to assume the lint IS the hard limit,
  when in fact the engine might permit 5+.

**Inconsistencies with other docs:**

- **`?=` status:** Grammar doc (§3.2) says "confirmed by cwtools AND online research." Local-docs
  findings doc (Q5) says "unconfirmed, not in vanilla scan." These are from the same batch and
  should agree. The grammar doc is correct; local-docs is stale.
- **Color-space tags:** Grammar doc (§3.7b) lists `rgb`, `hsv`, `hsv360`, `hex` as plain `IDENT`
  tokens not reserved. Local-docs findings (§1.11) suggests `KW_HSV`/`KW_RGB` as reserved keywords.
  These are in tension — grammar doc explicitly says "the lexer doesn't need new token kinds" for
  color tags while local-docs says these should be `KW_HSV`/`KW_RGB`. The grammar doc's resolution
  (IDENT + parser recognizes the keyless-block form) is the right call but creates an inconsistency
  with the local-docs recommendation.
- **`DOT` splitting of `event_target:foo`:** Grammar doc (§3.7, COLON) says COLON is NOT emitted
  standalone — the `event_target:` prefix consumes it as part of `EVENT_TARGET_REF`. Local-docs
  findings (§1.7, §5.4) says the colon SHOULD split: `IDENT COLON IDENT`. The grammar doc
  supersedes the local-docs recommendation here (local-docs correctly notes the grammar doc's
  decision), but the local-docs doc was not updated to reflect the resolution.

**Duplication:**

- The 5-depth call-graph limit, the `@[expr]` first-only rule, and the `$PARAM$`-in-comments hazard
  are each documented in this doc, in `clausewitz-local-docs-findings.md`, in `GUIDELINES.md`,
  in `mod-architecture-insights.md`, and in `clausewitz-tricks.md`. That is 5 copies of each.
  This doc should be canonical for the mechanical constraints; other docs should cross-reference.

**Outdated information:**

- None detected. The grammar doc was freshly written and integrates v4.x-applicable research.
  The `ordered_*` iterator is correctly cited as v3.2+.

**Interesting details to preserve:**

- §10, the two distinct depth limits (brace nesting vs call-graph) clearly distinguished with
  lint references. This is the single best explanation in any project doc.
- The `TARGETED_VAR` token kind (`name@SCOPE` as separate from `@name`) — unique to this doc and
  fixes a real cwtools bug.
- §6.2 "Comment-substitution gotcha" with the lint name `MACRO_PARAM_IN_COMMENT_HAZARD` — explicit
  and actionable.
- The "Verified absent" note on `<>` operator. Exactly the kind of negative fact that saves hours.
- The `prevprev` standalone broken-in-diplomatic-triggers note with the specific lint reference.

---

### 2. `docs/megapatch/design/2026-04-16-mod-language-design-draft.md`

**Summary:** Complete design for "Clause," a Rust-inspired mod authoring language that compiles to
Clausewitz. Not a scripting reference per se, but makes many implicit and explicit Clausewitz claims
throughout. ~2000 lines. Target audience: language design review and M-batch implementers.

**Scripting content:** The Clausewitz content is embedded in motivation, code examples, and
codegen/backend sections. Key claims: `inline_script` semantics, `$PARAM$` scope, scripted_effect
depth cap, `@variable` file-scope vs global distinction, event target persistence, `on_single_player_save_game_load` vs multiplayer, scope navigation, FIOS/LIOS rules.

**Wrong statements:**

- **§14.7 (implicit), inline_script promotion:** The doc refers to "Clause emits one per body" for
  `@[expr]`, which is correct. However it relies on `inline_script` for RAM-frugal emit without
  specifying the output directory. Given the `inline_tools` vs `inline_scripts` inconsistency in
  other docs (see Finding #2 in cross-doc section), this is a latent bug in the codegen spec.
- **§6, crate model:** The example `common/inline_scripts/auto/` (in §28 implicitly, via
  `compiler-passes.md`) conflicts with Heritage docs saying `common/inline_tools/heritage/`. The
  Clause doc uses `inline_scripts/auto/` as its emit path — this is the megapatch compiler path,
  not the Stellaris vanilla path. The confusion may not matter if the compiler always handles
  path routing, but it is undocumented.

**Missing content:**

- The doc describes `super.foo()` patch method overrides and `..super` struct spreads as the patch
  mechanism, but does not document what happens when a Clausewitz-targeted patch writes to a field
  that is a `$PARAM$` substitution in vanilla (i.e., the key is dynamic at game-load time). This
  is an edge case the grammar doc covers but Clause does not address.
- No section on how Clause handles the `hidden:` prefix on effects (cwtools §7j) — if a Clause
  method compiles to a hidden effect, the emit strategy is unspecified.
- The `?=` operator is not mentioned in any Clause code example. Since Clause generates Clausewitz,
  the question of whether the transpiler ever emits `?=` for optional-scope checks is unaddressed.

**Inconsistencies with other docs:**

- `§14` describes `Cached<T>` as using a flag+variable pattern. `clausewitz-tricks.md`
  "Trigger memoization via flags" documents the same pattern correctly. No cross-reference.
- The "Tenet 11" and "Tenet 12" defined in this doc (§4.5) have numbers that continue from
  `TENETS.md` which defines Tenets 1-10. These are compatible but not referenced across docs,
  leaving the tenet numbering ambiguous for any reader of `TENETS.md` alone.

**Duplication:**

- Gotcha documentation (depth cap, `$PARAM$` in comments, inline_script path substitution,
  `every_*` O(n×m)) is repeated in motivation sections without cross-references.

**Outdated information:**

- None detected. The doc post-dates the research synthesis and is v4.x aware.

**Interesting details to preserve:**

- The `StringStorage` via hidden fleet rename trick explicitly codified as a Clause backend —
  unique among all project docs.
- `Cached<T>` as the stdlib type that auto-generates the flag-memoization pattern — the most
  concrete documentation of that pattern.
- The orphan rule as conflict detection — mechanically enforcing Tenet 7 via the type system.
  Worth preserving in the unified reference as the "why coherence matters" motivation.

---

### 3. `docs/megapatch/design/2026-04-16-mod-language-design-startpoint.md`

**Summary:** The "strawman" predecessor to the final Clause design. Explicitly marked "NOT the
design." Kept as reference. Contains the original motivation analysis and early trait system sketch.
Target audience: design history / context.

**Scripting content:** Thin. The scripting claims are embedded in motivation examples. Does not
attempt to be a comprehensive reference.

**Wrong statements:**

- The file extension is `.mp` in the startpoint but `.cw` in the final design draft. Since the
  startpoint is explicitly "not the design," this is not technically wrong but will confuse readers
  who encounter both docs.
- Early section uses `#[clausewitz_trigger("name")]` decorator syntax which was later revised to
  `extern; fn ... #[as_scripted_trigger]` in the final design. If anyone uses the startpoint as
  reference, they will author against a superseded decorator API.

**Missing content:** N/A — document is a historical reference, not a specification.

**Inconsistencies with other docs:** Most inconsistencies are resolved in the final design draft.
The startpoint was explicitly not updated after the design was finalized.

**Duplication:** Substantially duplicated by the final design draft. Should be referenced
but not maintained in parallel.

**Outdated information:** The working title "Modlang" and file extension `.mp` are superseded.
The decorator API has changed.

**Interesting details to preserve:**

- The "six things broken about authoring directly in Clausewitz" enumeration (§1) — the clearest
  articulation of WHY Clause exists. Worth preserving as motivation text in a reference doc.
- The early observation that "file location is a design decision that authors shouldn't make" is
  a clean statement of a principle that otherwise only appears in passing in the final design.

---

### 4. `docs/megapatch/clausewitz-tricks.md`

**Summary:** A living catalog of Clausewitz scripting patterns, organized by category (storage
primitives, computation, persistence, AI, scheduling, performance). Source of truth for patterns
the Clause compiler auto-selects as bind backends. Target audience: compiler implementers and mod
authors.

**Scripting content:** Dense. Covers all major engine storage primitives, computation tricks,
persistence patterns, scheduling approaches, performance trade-offs, and a full gotcha section with
confidence levels.

**Wrong statements:**

- **`@[expr]` section:** States "`@[ x + y ]` evaluates at parse time." This is not precisely right.
  `@[ expr ]` in file-local `@variable` declarations evaluates at parse time. In scripted_effect
  or scripted_trigger bodies, the grammar doc and research docs describe it as evaluating at
  game-load time (not parse time). The distinction matters: parse time means during the Clausewitz
  parser's pass; game-load time means when Stellaris loads the script. These docs use "parse time"
  and "load time" inconsistently throughout. The online research calls `@[ ... ]` "inline math syntax
  is `@[ <expr> ]`" without calling it either parse-time or load-time. However, `common/scripted_variables/`
  entries are explicitly "evaluated at game-load time to integer or float literals" (online research
  §1.2.D). "Parse time" appears to be an informal usage but could mislead implementers.
- **"Recursion-tolerant inline_script chains" section:** States "inline_script can chain up to 5
  iterations." This is accurate (cwtools ResourceManager.fs §5e), but the framing "recursion-tolerant"
  is slightly misleading. The 5-iteration cap is for nested `inline_script` calls, not for recursion
  in the computer-science sense. An inline_script cannot call itself; the 5 iterations are for
  chains like A includes B includes C. The word "recursion" will confuse implementers familiar with
  the 5-level call-graph cap for scripted_effects, which IS a genuine recursion/call-stack limit.
- **Storage section, "Variables set to 0 are NOT serialized" is absent.** The GUIDELINES.md
  documents this as a critical rule, but clausewitz-tricks.md does not mention it under the
  `set_variable / change_variable` entry. An author reading only clausewitz-tricks.md will not
  know to zero out variables for save health.

**Missing content:**

- No entry for `script_values` operations (the full set: `set`, `weight`, `add`, `subtract`,
  `factor`, `mult`, `multiply`, `divide`, `modulo`, `round_to`, `max`, `min`, `pow`, `round`,
  `ceiling`, `floor`, `abs`, `square`, `square_root`). The section mentions script values exist
  but does not enumerate the operations. API_REFERENCE.md §12.4 is the only complete listing.
- No entry for `exists` guard pattern before scope transitions. This is documented in
  API_REFERENCE.md §1.4 as a critical rule but absent from tricks.
- No entry for `pre_triggers` — the "free performance" C++-level check available on planet, pop,
  system, starbase, and leader events. GUIDELINES.md §1 covers this thoroughly.
- No section on localization-side scripting (`$variable$` substitution, `[Root.GetName]` accessors).
  These are scripting constructs used alongside event scripting.
- No discussion of `hidden_effect = { ... }` and `custom_tooltip` patterns — important for
  maintaining clean UI while executing effects.
- No entry for `assert_if` and `log` debug effects (documented in API_REFERENCE Appendix A).

**Inconsistencies with other docs:**

- Confidence tag "primary" for `[[PARAM] body ]` conditional blocks, citing "wiki-documented;
  cwtools missed it." The online research (§1.6.C) confirms this as a real feature but traces
  its origin to EU4 (Dharma) and notes it's used "across Clausewitz games including Stellaris."
  The wiki confirmation is for Stellaris specifically. Fine, but the citation should be more precise.

**Duplication:**

- "Global event targets LOST on save/load" appears here and in GUIDELINES.md, mod-architecture-insights,
  API_REFERENCE §1.5, and local-docs-findings G3. Five copies.
- "Every_* iterators are O(n×m)" appears here and in GUIDELINES.md, mod-architecture-insights,
  and local-docs-findings G8. Four copies.

**Outdated information:**

- The `Situations` entry says "Stellaris 3.5+." The API_REFERENCE says `situation` scope is
  v3.3+. This is a minor version disagreement. The situation SYSTEM existed earlier than
  situation_event type. Should be clarified: situations as an engine concept were introduced
  in 3.3 (Stellaris Dev Diary #240); Clause targets the current engine.

**Interesting details to preserve:**

- "Custom_count_modifier abuse" for encoding count-like state via modifier names — unique to this
  doc, not documented elsewhere in the project.
- "Flag-set arithmetic (binary-search counter)" — elegant; novel in project context.
- "Dynamic personality switching via changing country_type at runtime" — confirmed-primary and
  not in other docs.
- "Situation auto-progress as cheap monthly ticker" — community-reported but concrete.
- The `TraitCarrier<T>` cross-reference for dynamic flag names (`is_friend_of_@root`).
- The explicit "wishlist" section documenting what Clausewitz CANNOT do — prevents wasted effort.

---

### 5. `docs/megapatch/LINT_CATALOG.md`

**Summary:** Catalog of all lint rules enforced by the megapatch compiler. Organized by group
(structural, cross-reference, type/value, lifecycle, deprecated syntax, scope/flow, patch shape,
performance/hygiene). Living document. Target audience: compiler implementers, patch authors.

**Scripting content:** Clausewitz-specific rules throughout — particularly Group 1 (structural
depth limits), Group 3 (scope transitions), Group 5 (deprecated syntax), Group 6 (scope chains).

**Wrong statements:**

- **`TRIGGER_DEPTH_EXCEEDED`, Group 1:** Description says "nested deeper than 5 levels." The
  grammar doc is explicit that the brace-nesting lint fires at depth ≥ 6 (threshold is 6, not 5),
  while the call-graph cap is 5 frames. By describing the rule as "deeper than 5" the catalog
  is off by one on the brace-depth lint — or it's conflating the two distinct limits. The linter
  code in `content_syntax.py` uses `max_depth = 6` (fires at 6). "Deeper than 5" and "at 6 or
  deeper" are the same condition, but the wording "deeper than 5 levels" implies the trigger fires
  for depth 6 or more, which is correct. Actually this is consistent. The confusion is that "depth
  > 5" equals "depth >= 6." Not actually wrong, just potentially confusing given the parallel
  call-graph cap of exactly 5.
- **Group 5 `DEPRECATED_RANDOM_POP`:** States "semantics changed; use `random_owned_pop`." This
  is partially correct but `random_owned_pop` may itself be deprecated in favor of
  `random_owned_pop_group` in v4.0+ (Pop Groups update). The catalog does not note this.
  API_REFERENCE §1.1 references Pop Groups (4.0+) but the lint catalog was not updated.

**Missing content:**

- `INLINE_EXPR_MULTIPLE_PER_BODY` — flagged as "NEW" in grammar doc §10 but not yet in the
  catalog. The grammar doc explicitly notes "add via G3e or follow-up."
- `MACRO_PARAM_IN_COMMENT_HAZARD` — flagged as "NEW" in grammar doc §10. Not in catalog.
- `GLOBAL_EVENT_TARGET_NEEDS_REREGISTRATION` — flagged as "NEW" in grammar doc §10. Not in catalog.
- `NOT_WITH_MULTIPLE_CONDITIONS` — flagged as "NEW" in grammar doc §10. Not in catalog.
- `NESTED_EVERY_IN_PULSE_EVENT` — flagged as "NEW" in grammar doc §10. Not in catalog.
- No lint for "using `prevprev` in diplomatic trigger context" distinct from the general
  `PREVPREV_IN_ACTIVE_CODE`. The grammar doc and local-docs note that `prevprev` is specifically
  broken in diplomatic triggers and `opinion_modifier` scopes — a targeted lint would be more
  actionable than a blanket advisory.
- No lint for "reading an unset variable without `is_variable_set` guard." GUIDELINES.md calls
  this a hard rule but there is no lint for it.
- No lint for `MTTH` usage (`mean_time_to_happen`). GUIDELINES.md §1 calls this a hard rule ("NEVER
  use MTTH") but the catalog has no lint entry for it.
- No lint for missing `is_triggered_only = yes` on events. GUIDELINES.md §1 calls this mandatory.

**Inconsistencies with other docs:**

- Group 5 deprecated syntax section is sparse (4 entries with "extensible" note). The
  GUIDELINES.md §4 hard rules and PATCH_FLOW.md both identify more deprecated patterns.

**Duplication:**

- The catalog references `PREVPREV_IN_ACTIVE_CODE` and `DEEP_SCOPE_CHAIN` which are also described
  in grammar doc §10, local-docs G2/G9, GUIDELINES.md §3. The catalog is the correct owner; other
  docs should cross-reference it rather than re-documenting.

**Interesting details to preserve:**

- The clear severity ladder (ERROR/WARNING/INFO) and its build-gate implications.
- The "cwtools? yes/partial/no" column — invaluable for deciding what to implement vs delegate.
- Group 7 patch-shape lints — unique to this doc, not covered by any scripting reference.
- The explicit prioritization / implementation order at the bottom.

---

### 6. `docs/megapatch/LINTING.md`

**Summary:** Framework and workflow guide for the linter system. How to write lints, how to
manage baselines, when to use which lint scope. Target audience: compiler contributors.

**Scripting content:** Minimal. This is a process doc, not a scripting reference. The one
scripting implication is the correct statement that release builds must have zero warnings/errors.

**Wrong statements:** None detected for scripting content.

**Missing content:** N/A for a process doc.

**Inconsistencies:** The severity ladder described here (ERROR/WARNING/INFO with specific build-gate
behavior) is consistent with LINT_CATALOG.md. No inconsistencies found.

**Interesting details to preserve:**

- The ownership attribution model (FIRST_PARTY / EXTERNAL / UNKNOWN) and its fail-safe default
  (treat UNKNOWN as FIRST_PARTY).
- The external findings baseline pattern — relevant to understanding which Clausewitz quirks are
  "known external problems" vs "our bugs."

---

### 7. `docs/megapatch/PATCH_AUTHORING.md`

**Summary:** Day-to-day guide for writing patches. Covers the item-centric YAML format, field
schemas, workflows, and lints. Target audience: patch authors. Current and accurate for the
item-centric compiler.

**Scripting content:** The scripting knowledge here is procedural — how to write patches that
target Clausewitz content — rather than declarative scripting facts. The most relevant Clausewitz
claim is the `@variable` note in STALE_PATCHES.md (cross-referenced) about `@variable` items not
being addressable as Items.

**Wrong statements:** None for Clausewitz scripting facts. The guide accurately describes the patch
mechanics.

**Missing content:**

- No guidance on how to patch content that uses `$PARAM$` substitution in keys. If a vanilla item
  has `$KEY$ = { ... }` where the key is a parameter, a `replace_in_item` patch cannot target it
  by key name.
- No mention of the FIOS/LIOS implications for different `directory:` targets. A patch author needs
  to know whether their target directory is FIOS or LIOS to understand when their `prefer_mod` will
  be effective.

**Interesting details to preserve:**

- The patch priority ordering: `override > replace_in_item > inject_field > prefer_mod > delete_item
  > insert_item > no_override`.
- The `depends_on` slug format and when to use it.

---

### 8. `docs/megapatch/TENETS.md`

**Summary:** Project principles. Not a scripting reference. The relevant tenet for scripting is
Tenet 8 (Clean Builds Or No Builds) with its explicit callout of "loading-screen infinite loops
from depth-5 trigger nesting" as a compiler-catchable class of bug.

**Scripting content:** The depth-5 mention in Tenet 8 is the one scripting fact. It correctly
identifies the issue but does not distinguish brace-depth from call-graph depth (that precision
belongs in the lint catalog).

**Wrong statements:** Tenet 8 says "depth-5 trigger nesting" which is a natural-language description
of the call-graph limit, but the exact threshold varies: brace-depth lint fires at ≥6, call-graph
cap is 5 frames. Calling it "depth-5" is close enough for a tenet document.

---

### 9. `docs/megapatch/STALE_PATCHES.md`

**Summary:** Analysis of the 688 compile errors from the current patch corpus. Documents root
causes and fix workflows. Mentions the `@variable` items as a specific failure pattern.

**Scripting content:** The `@variable` section is the main scripting fact: "`@variable` is a
Clausewitz file-scope constant, not an Item. The compile engine can't extract them from source mods
because they're not addressable as entries."

This is correct and well-stated. Cross-references to grammar doc's handling of `@variable`
(VAR_DECL_AT) is the right forward-looking fix.

**Wrong statements:** None for scripting content.

**Missing content:** The doc notes that traits patches fail on `modifier = { <job>_jobs_bonus_workforce_mult = ... }` due to a "4.x patch" changing the vanilla format. This should be more specific: the Stellaris 4.0 Pop Groups update changed job modifier field names. The document correctly identifies this as a version-change stale patch but doesn't name the v4.0 pop group change as the cause.

**Interesting details to preserve:**

- The three distinct categories of stale patches (FIELD_NOT_FOUND, EXTRACTION_FAILED, FIND_NOT_MATCHED)
  with their root causes are valuable taxonomy for understanding Clausewitz content evolution.

---

### 10. `docs/megapatch/PATCH_FLOW.md`

**Summary:** Early design document (design-era, pre-compiler) describing a 4-phase pipeline for
the megapatch. Describes three patch format options (git-diff, semantic YAML, full file override)
and recommends Option B + C. This document has been SUPERSEDED by the item-centric compiler
architecture.

**Scripting content:** The patch format options are presented as if they describe how the
compiler works. They do not match the current item-centric YAML format in `PATCH_AUTHORING.md`.

**Wrong statements:**

- The entire "Phase 1 through Phase 4" pipeline description does not match the current architecture.
  The compiler does not produce `generated/`, `patches/`, `bridges/`, and `output/` directories in
  the described way.
- The "Option A: Git-style unified diff patches" is described as viable; the current compiler
  uses semantic YAML patches and this option was rejected. Recommending both Option A and B
  contradicts the actual implementation.
- The VCS table saying `generated/` is not committed may or may not reflect current practice
  (the `ARCHITECTURE_V2.md` describes a different build model).
- The patch format YAML example uses `target_mod`, `find`, `replace`, `all: true` — this is NOT
  the current patch schema which uses `directory`, `item`, `type`, `content`, etc.

**Interesting details to preserve:**

- The taxonomy of "three kinds of work" (auto-merge, manual patch, bridge logic) as a conceptual
  model for the megapatch's structure remains valid even if the implementation has changed.
- The "when to rebuild" table is still accurate at a high level.

**Severity: HIGH.** This doc will actively mislead any new contributor who reads it and tries to
understand the patch system. Should be either updated to reflect current architecture or clearly
dated/archived.

---

### 11. `docs/heritage/API_REFERENCE.md`

**Summary:** Authoritative reference for Stellar Heritage mod development. Covers Stellaris 4.3.x
(Cetus) scopes, on_actions, APIs by system domain, dynamic modding, and moddable directories.
3000+ lines. Target audience: Heritage mod developers.

**Scripting content:** Extensive. Everything from scope transitions, on_actions, variables, flags,
events, scripted effects/triggers/variables/values, inline scripts, GUI modding, and the full
`common/` directory catalog.

**Wrong statements:**

- **§12.5, inline scripts:** States inline scripts are "Defined in `common/inline_tools/`." The
  megapatch compiler emits to `common/inline_scripts/`. These are two different paths. The Stellaris
  wiki uses `common/inline_scripts/` in examples. GUIDELINES.md §3 says "placed in
  `common/inline_tools/`." This inconsistency is HIGH severity — it means the compiler and the API
  reference are pointing to different output directories. One of them is wrong for actual Stellaris.
  The Stellaris wiki convention (and PLAYTEST.md's reference to `common/inline_scripts/heritage/`)
  suggests `common/inline_scripts/` is the vanilla-consistent path.
- **§1.2, PREV stackable forms:** States "max 4" via parenthetical after listing `prevprev`,
  `prevprevprev`, `prevprevprevprev`. This is consistent with the grammar doc. Good.
- **Appendix B, `calc_true_if`:** States `amount = <int>` but the online research (§1.5) and
  grammar doc (§3.5) document that `amount` supports `>=`, `<=`, `>`, `<`, `=` — not just
  integer equality. The Appendix B entry uses `amount = <int>` suggesting only equality, which
  is incomplete.

**Missing content:**

- No documentation of `exists = <scope>` guard pattern — mentioned in §1.4 but not explained as
  the standard pattern for conditional scope access.
- No documentation of `trigger_if` / `trigger_else_if` / `trigger_else` in Appendix B (only
  Appendix A covers the effect-side `if`/`else_if`/`else`).
- No documentation of the `hidden:` prefix on effect keys.
- No documentation of the `optimize_memory` keyword — mentioned in GUIDELINES.md but absent here.
- The `value:name|PARAM1|val1|PARAM2|val2|` pipe-delimited parameter syntax in §12.4 is mentioned
  but the full pipe-parameter mechanics are not documented.
- `on_multiplayer_game_loaded` is absent from the on_actions table (§2.1). This is critical for
  any system that re-registers global event targets in multiplayer.

**Inconsistencies with other docs:**

- `common/inline_tools/` (this doc and DESIGN.md and GUIDELINES.md) vs `common/inline_scripts/`
  (CLI_DESIGN.md, PLAYTEST.md, compiler-passes.md). This is the inline_script path conflict.
- Appendix B shows `calc_true_if = { amount = <int> }` (equality only), but grammar doc §3.5
  explicitly documents that `amount` supports all six comparison operators.
- GUIDELINES.md §3 mentions `inline_scripts (3.5+)` for code reuse but `clausewitz-tricks.md`
  says "Situations (3.5+)" for the Situation store — the API_REFERENCE §1.1 has `situation` scope
  as v3.3+. These version numbers are for different features (situations as engine concept vs
  situations as a Clause backend).

**Duplication:**

- The scope transitions table (§1.1) is the most comprehensive scope reference in the project.
  Should be THE canonical source. Grammar doc §3.6 has a shorter list.
- On_action tables (§2.x) are unique to this doc and should be preserved.
- Variable/flag rules in §8 duplicate GUIDELINES.md §2.

**Interesting details to preserve:**

- The complete on_actions tables with THIS/FROM/FROMFROM contexts — unique in the project.
- The "important" note in §2.3 that there is no `on_leader_added` or `on_leader_removed` — only
  `on_leader_spawned`, `on_leader_fired`, `on_leader_assigned` etc. This is the kind of negative
  fact that prevents hours of debugging.
- The complete `common/` directory catalog (§18) — comprehensive and seemingly accurate.
- The `pop_group_event` type (v4.0) — the only doc that consistently documents v4.0 changes.

---

### 12. `docs/heritage/GUIDELINES.md`

**Summary:** Heritage mod's hard rules for scripting. Covers performance, variables/flags,
scripted effects/triggers, compatibility, multiplayer, save health, GUI, file format, and
development workflow. Target audience: Heritage mod developers.

**Scripting content:** Dense, rule-oriented. Contains the best summary of performance rules in the
project, including pre-triggers, event polling hazards, and iterator cost hierarchy.

**Wrong statements:**

- **§3:** "Use `inline_scripts` for code reuse. Placed in `common/inline_tools/`, they perform
  text substitution at load time." This is the same `inline_tools` vs `inline_scripts` conflict.
  The vanilla Stellaris path for inline scripts is `common/inline_scripts/`. Using `common/inline_tools/`
  will fail unless this is a project-specific convention that overrides the vanilla directory.
  If it IS a project convention (pointing to a different directory that the compiler routes
  through), this distinction must be documented. Currently it creates a silent wrong expectation.
- **§3:** "Scripted effects have a hard stack depth limit of 5." Correct for the call-graph limit.
  But the companion brace-depth limit (fires at ≥6) is absent from this statement, creating the
  impression there is only one kind of depth limit.
- **§4, LIOS/FIOS table:** "No override: traits, terraform, strategic_resources — cannot be
  individually overwritten; require full file replacement." The local-docs findings (§1.15) cites
  the rules.py compiler as mapping this — which matches. However, the online research confirms
  that `scripted_variables` are FIOS (not mentioned in GUIDELINES.md's table). This is a gap.

**Missing content:**

- No discussion of `ordered_*` iterators (v3.2+) — mentioned in grammar doc and online research
  but absent from GUIDELINES.md.
- No discussion of `calc_true_if` for counting trigger conditions.
- No discussion of `?=` operator — confirmed in cwtools and PDX Tools but not documented anywhere
  in Heritage-specific docs.
- No entry for `trigger_if`/`trigger_else`/`trigger_else_if`.
- No discussion of how file-local `@variable = @[ expr ]` form is valid (documented in cwtools
  review §7b) — some scripts use arithmetic in variable declarations.

**Interesting details to preserve:**

- "Triggers short-circuit. Place triggers most likely to fail FIRST in AND blocks" — the best
  summary of this optimization rule in the project.
- The `pre_triggers` section is the most thorough in any project doc.
- The "Engine Cost Hierarchy" (§1, cheapest to most expensive) — unique to this doc and extremely
  useful. Should be in the unified reference.
- The full variable range: "Fixed-point, -2,147,483.648 to 2,147,483.647 with 0.001 precision"
  — the only place this is documented in the project.

---

### 13. `docs/heritage/mod-architecture-insights.md`

**Summary:** Research on top Stellaris mods' architecture patterns. Source: GitHub analysis of
Gigastructural Engineering, EST, Planetary Diversity, DPE, More Events Mod. Target audience:
Heritage mod developers learning from existing mods.

**Scripting content:** The Clausewitz-specific sections are §13 (Mistakes to Avoid) and scattered
throughout the mod-analysis section. The key gotchas (items 1-9 of §13) map directly to the
grammar doc constraints.

**Wrong statements:**

- No definitively wrong statements found for Clausewitz scripting facts. The content is
  observation-based (what mods actually do) rather than specification-based.
- Item 9 of §13 (Subtle Mistakes): "Not using UTF-8 with BOM for localization — plain UTF-8
  silently fails." This is correct. But §8 of GUIDELINES.md says "Script files: UTF-8." It does
  NOT specify that script files DON'T need BOM. The BOM is required for localisation (`.yml`) but
  NOT for script files (`.txt`). Both statements are individually correct but a reader comparing
  them could misread the BOM requirement as applying to script files too.

**Missing content:**

- No documentation of the `~~` ASCIIbetical load-order prefix trick mentioned in §1 (from
  Scripted Trigger Undercoat) — the insight is noted but not explained in depth.
- The mod analysis mentions Gigastructural Engineering uses `giga_compat_triggers.txt` for
  cross-mod compatibility triggers but doesn't explain the scripting pattern for conditional
  cross-mod compatibility (checking `has_mod` or `has_country_flag = <mod_init_flag>`).

**Interesting details to preserve:**

- §1 namespacing prefix table (organized by mod) — most complete namespacing reference in project.
- The "load-order control files" pattern: `00_` for overridable, `zzz_` for last-word.
- The `est_options.txt` configuration-via-scripted-triggers pattern.
- §9 (Subtle Mistakes) items 1-9 — a clean checklist that should transfer to the unified reference.
- The variable range and data-budget calculation (§2 of GUIDELINES.md, cross-referenced) is unique.

---

### 14. `docs/megapatch/design/compiler-passes.md` (additional, relevant doc)

**Summary:** Describes the compiler pass architecture. Contains scripting-relevant details about
what the passes operate on.

**Scripting content (relevant):**

- References `common/inline_scripts/` as the output directory for inlined leaves — consistent with
  the megapatch compiler perspective. This is the third form of the inline_scripts path conflict.

---

### 15. `.cache/reviews/2026-04-16/clausewitz-local-docs-findings.md`

**Summary:** Synthesis of Clausewitz syntax knowledge from project code and docs (before the
research phase). 870 lines. Produced as input to grammar formalization. Contains confirmed
semantics, documented gotchas, implicit assumptions in code, and open questions.

**Scripting content:** Thorough summary of what the codebase knows. The 15 "token-design
implications" are organized from a compiler perspective.

**Wrong statements:**

- **§1.9, `?=` operator:** States "not confirmed in vanilla; might appear in some mods." This is
  the stale pre-research position. The grammar doc (produced after this doc) confirmed `?=` via
  PDX Tools and cwtools. The open question Q5 was RESOLVED but the local-docs doc was not updated.
  Any consumer of this doc will see the old "unconfirmed" state.
- **§5.4, Token-design implication:** "COLON and DOT should be separate tokens — splitting
  `event_target:foo` into `IDENT COLON IDENT`." The grammar doc RESOLVED this differently:
  COLON is NOT emitted separately; `event_target:` is consumed as a first-class `EVENT_TARGET_REF`
  token. The local-docs suggestion was superseded by the grammar doc's decision but not updated.
- **§1.10 (implied):** States `_STRUCTURAL_KEYWORDS` in `undefined_refs.py:439` uses lowercase
  `"and", "or", "not"` suggesting they are case-sensitive. The grammar doc §3.5 resolved that both
  `AND`/`and` map to `KW_AND` via case-insensitive lookup. The local-docs correctly identifies
  this as "Open Question Q1" but does not resolve it.

**Missing content:**

- The open questions (Q1-Q9) were mostly resolved in the grammar doc, but the local-docs doc was
  not annotated with the resolutions. A future reader sees open questions that have been answered.

**Interesting details to preserve:**

- The "five micro-parsers" table (§A2) — best summary of the current parser architecture's
  fragmentation. Essential for understanding why the grammar formalization is needed.
- The `_strip_noise` duplication finding (§A3) — concrete technical debt.
- The `_CALL_RE` assumption analysis (§A4) — explains a subtle false-positive source in the call
  graph analyzer.
- The `RULE_FULL = "FULL"` synonym note (§A6) — easy to miss, important for compiler correctness.

---

### 16. `.cache/reviews/2026-04-16/clausewitz-online-research-findings.md`

**Summary:** Online research synthesis with URL-cited sources for Clausewitz/Stellaris scripting.
553 lines. Covers scope rules, operators, boolean blocks, inline_script, macro constructs, literals,
iterators, switch blocks, control flow, and modder gotchas. Authoritative.

**Scripting content:** Most comprehensive and best-cited Clausewitz reference in the project.

**Wrong statements:**

- **§1.3.A, PREV chain:** States chains are "literal keyword identifiers, not dot-chained."
  This is accurate for the concatenated form (`prevprevprev`) but the same section correctly notes
  dot notation also works (`owner.capital_scope.solar_system`). No actual contradiction, but the
  framing could be clearer that BOTH forms are valid.
- **§1.5, `NOT = { A B }` behavior:** States "community consensus is that multiple conditions in
  `NOT = { ... }` behave as NOR." The §2 contradictions section hedges that "wiki says 'can cause
  unforeseen bugs' (implying unreliable)." The grammar doc treats this as authoritative wiki
  warning. The online research doc is appropriately uncertain here; the unified reference should
  cite the wiki's own wording.
- **§1.6.E, `inline_script` placement restrictions:** Says "One inline_script per file; file name
  determines the invocation path." This is misleading. Each inline_script FILE contains one script
  body, but a single Clausewitz script file (e.g., a scripted_effect file) can reference multiple
  inline_scripts via multiple `inline_script = path` calls. The statement should be "each inline
  script is its own file in `common/inline_scripts/`."

**Missing content:**

- No documentation of `on_multiplayer_game_loaded` on_action. This is mentioned in the grammar
  doc (§10) as needed for global event target re-registration in multiplayer, but the online
  research section only mentions `on_single_player_save_game_load` in the scope discussion.
- The `hidden:` prefix (cwtools §7j) was not found by the online research — this appears to be an
  underdocumented engine feature.

**Interesting details to preserve:**

- §1.7.B "Dynamic flag/keyword construction" — `set_leader_flag = is_friend_of_@root` producing
  `is_friend_of_140`. The only source-cited documentation of this pattern in the project.
- §1.7.C "Dynamic identifier construction with `$PARAM$` in keys" — parameter substitution in KEY
  positions. Confirmed but easy to miss.
- §1.6.E "Placement restrictions" for inline_script — not documented well elsewhere.
- The complete token taxonomy summary table at §5 — most structured token summary in the project,
  even if superseded by the grammar doc.
- §2 "Contradictions" section — valuable epistemic hygiene, should be reflected in the unified
  reference's confidence ratings.

---

### 17. `.cache/reviews/2026-04-16/cwtools-formalization-review.md`

**Summary:** Code review of cwtools source (F#, FParsec). Documents cwtools' architecture,
identifies 7 parser bugs as "tests for our lexer," and provides lessons to copy vs avoid.

**Scripting content:** Indirect — focuses on cwtools' representation of Clausewitz syntax rather
than Clausewitz scripting directly. High technical value for the lexer implementers.

**Wrong statements:**

- **§2, Key parsing:** States `idCharArray` includes `|` as a legal identifier character. This is
  accurate for cwtools but incorrect for our design: the grammar doc explicitly defines `|` as a
  `PIPE` token in `value:name|K|V|` contexts. The cwtools behavior of treating `|` as part of an
  identifier is a limitation we're explicitly fixing.
- **§7e, `type`/`types` keyword note:** Says these appear in `clause.txt` test file and suggests
  emitting `TYPE_DECL`/`TYPES_DECL` reserved tokens. The grammar doc §3.5 defers these to G7+
  ("CWT schema reserved — deferred"). The cwtools review's recommendation is more aggressive than
  the grammar doc's current plan. Not strictly wrong but creates an expectation gap.

**Missing content:** None significant — this doc is appropriately scoped.

**Interesting details to preserve:**

- The `ScopeContext = { Root: Scope; From: Scope list; Scopes: Scope list }` model for scope
  tracking — the most concrete specification of how scope state should be maintained.
- The `StringTokens` interning specification — copy-worthy for the G2 lexer.
- The 8-byte bit-packed range type — copy-worthy for G2.
- The complete "worth copying vs worth avoiding" table in §9.

---

## Cross-Document Findings

---

### Inconsistencies Between Our Docs

**I-1 (Critical): `?=` operator status**

| Doc | Claim |
|-----|-------|
| `clausewitz-local-docs-findings.md` §1.9, Q5 | "not confirmed in vanilla; treat as unconfirmed" |
| `clausewitz-online-research-findings.md` §1.4 | Confirmed present; `c:RUS ?= this` example; CK3 docs confirm |
| `clausewitz-grammar.md` §3.2 | "Confirmed by cwtools QuestionEqual = 7uy AND online research" |

**Resolution:** Grammar doc is correct. `?=` IS in the Clausewitz operator set, confirmed by multiple sources. Local-docs is stale. All future docs must treat `?=` as a first-class operator.

---

**I-2 (Critical): `inline_script` output directory**

| Doc | Path |
|-----|------|
| `docs/heritage/API_REFERENCE.md` §12.5 | `common/inline_tools/` |
| `docs/heritage/GUIDELINES.md` §3 | `common/inline_tools/` |
| `docs/heritage/DESIGN.md` | `common/inline_tools/heritage/` |
| `docs/megapatch/CLI_DESIGN.md` | `common/inline_scripts/` |
| `docs/megapatch/PLAYTEST.md` | `common/inline_scripts/heritage/` |
| `docs/megapatch/design/compiler-passes.md` | `common/inline_scripts/` |
| Stellaris wiki (online research) | `common/inline_scripts/` (implicit from examples) |

**Resolution needed:** The Stellaris vanilla convention is `common/inline_scripts/`. The Heritage docs use `common/inline_tools/`. If Heritage intentionally uses a non-standard directory, this must be documented explicitly and the compiler must route correctly. If it's a documentation error in Heritage docs (likely: the compiler emits to `common/inline_scripts/` and Heritage docs were written before they were confirmed against vanilla), all Heritage docs need to be corrected.

**Recommended fix:** Confirm from vanilla Stellaris files which directory is correct. Update whichever set of docs uses the wrong path.

---

**I-3 (High): Depth limit description precision**

| Doc | Description |
|-----|-------------|
| `LINT_CATALOG.md` Group 1 | "deeper than 5 levels" for both brace-depth lint and call-graph |
| `clausewitz-grammar.md` §10 | Explicitly distinguishes: brace-depth fires at ≥6; call-graph cap is 5 frames |
| `docs/heritage/GUIDELINES.md` §3 | "hard stack depth limit of 5" |
| `clausewitz-tricks.md` | "5-level call-graph depth cap" (correct framing) |

**Resolution:** Grammar doc is most precise. The two limits are:
1. Brace nesting: lint fires at depth ≥ 6 (i.e., threshold is inclusive 6 or exclusive 5 — same condition).
2. Call-graph: cap is 5 frames; depth 6 silently fails.

Both thresholds happen to share the number "5" (< 6 for brace, ≤ 5 for call-graph) but are completely different constraints. Every doc describing these should use grammar doc §10 language.

---

**I-4 (High): `calc_true_if` `amount` operator support**

| Doc | Claim |
|-----|-------|
| `docs/heritage/API_REFERENCE.md` Appendix B | `amount = <int>` (equality only implied) |
| `clausewitz-online-research-findings.md` §1.5 | `amount` supports `>=`, `<=`, `>`, `<`, `=` |
| `clausewitz-grammar.md` §3.5 | `amount` supports `>=`, `<=`, `>`, `<`, `=` |

**Resolution:** API_REFERENCE Appendix B is incomplete. `calc_true_if` supports all six comparison operators for `amount`, not just equality.

---

**I-5 (Medium): Deprecated `prevprev` scope**

| Doc | Claim |
|-----|-------|
| `clausewitz-grammar.md` §3.6 | `prevprev` broken in diplomatic triggers and `opinion_modifier` scopes |
| `clausewitz-local-docs-findings.md` §1.6 | Same, citing `content_syntax.py:174` |
| `docs/heritage/GUIDELINES.md` §3 | "PREV chains are fragile and limited to 4 levels" (general warning, no specific broken contexts) |
| `clausewitz-tricks.md` gotchas | "broken in diplomatic triggers and `opinion_modifier` scopes" |

**Resolution:** Grammar doc and tricks doc are most specific. GUIDELINES.md's general warning is insufficient — it should name the specific broken contexts.

---

**I-6 (Medium): `NOT = { A B }` behavior reliability**

| Doc | Claim |
|-----|-------|
| `clausewitz-online-research-findings.md` §1.5, §2.E | Wiki says "can cause unforeseen bugs"; community says "acts as NOR" — in tension |
| `clausewitz-grammar.md` §10 | "Officially 'can cause unforeseen bugs' per the wiki" |
| `clausewitz-tricks.md` | "Multiple conditions in `NOT` evaluate as NOR, not as `NOT (A OR B)`" |
| `clausewitz-local-docs-findings.md` §1.10 | Cites both wiki warning and NOR behavior |

**Resolution:** The online research correctly identifies the tension. The authoritative source (Stellaris wiki) says "can cause unforeseen bugs" which is a warning about unreliable behavior, not a guarantee of NOR semantics. The unified reference should say: "wiki warns `NOT = { A B }` can cause unforeseen bugs; in practice behaves like NOR (none must be true), but this is implementation-specific. Always use `NOR` for multi-condition negation to be safe."

---

**I-7 (Low): `inline_script` path as "script" field name**

| Doc | Claim |
|-----|-------|
| `docs/heritage/API_REFERENCE.md` §12.5 | Block form: `script = path/to/script` |
| `clausewitz-online-research-findings.md` §1.6.A | Block form: `script = test_basic_policy` |
| Both agree | The `script` key inside inline_script block form |

No contradiction here — both agree the keyword is `script`. Documented for completeness.

---

### Topics Covered by No Doc

**G-1: `on_multiplayer_game_loaded` on_action.**
Critical for multiplayer save-load global event target re-registration. Mentioned in grammar doc
§10 and clausewitz-tricks.md "Global event targets LOST on save/load" but not in the API_REFERENCE
on_action tables or GUIDELINES.md §5 (Multiplayer Safety).

**G-2: `hidden:` prefix on effect keys.**
`hidden:add_modifier` strips `hidden:` before scope resolution. Documented only in cwtools review
§7j. Not in API_REFERENCE, GUIDELINES, grammar doc, or tricks catalog.

**G-3: `type`/`types` keyword syntax (CWT-style).**
`type NAME = BASE_TYPE { ... }` appears in vanilla CWT files. The grammar doc defers to G7+.
No doc explains what the lexer should do with these in the interim.

**G-4: `==` operator precise semantics.**
PDX Tools confirms `==` is a distinct operator. Online research §2.D says "semantics unclear; may
be explicit exact equality vs `=` as `>=` in numeric triggers." No doc resolves this. The grammar
doc lists it as `OP_EQ_EQ` with note "semantics vs `=` is community-debated." The unified reference
should document: `==` exists, exact semantics unverified, treat as distinct operator, let semantic
pass decide.

**G-5: `while` loop iteration limit in Stellaris specifically.**
Online research §2.A confirms 1000 limit for CK3 but marks Stellaris as "likely same, unconfirmed."
Grammar doc says "1000-iteration cap (CK3 confirmed; Stellaris presumed same)." No doc has a
Stellaris-specific confirmation.

**G-6: Non-parameterized scripted_effects and the depth cap.**
Online research §2.F: "inference: depth limit only for parameterized effects." Grammar doc says
"plain (non-parameterised) scripted_effects may not have the same cap — community inference,
unverified." No doc has authority to confirm or deny. Should be documented as unverified.

**G-7: `$PARAM|default$` in `scripted_trigger` bodies specifically.**
Local-docs Q6 flags this. Grammar doc §10 leaves it unverified. The unified reference should
document: confirmed for `inline_script` and `scripted_effect` bodies; `scripted_trigger` with
parameter defaults is unconfirmed.

**G-8: Whether PREV chain beyond 4 is engine-hard-limited or just lint-advisory.**
Grammar doc lists `prevprevprevprev` as max concatenated form. Online research §1.3.A cites the
Stellaris wiki and CK2 docs for max 4. But no source says the engine hard-errors on a 5th level —
it may just be undefined behavior or return the same scope as level 4. Should be documented clearly.

**G-9: Variable range precision for script_values.**
GUIDELINES.md documents variable range as "Fixed-point, -2,147,483.648 to 2,147,483.647 with
0.001 precision." Script values may have different precision characteristics. Not documented.

**G-10: `scripted_variables` FIOS behavior.**
Online research §1.2.B explicitly states scripted variables use FIOS. GUIDELINES.md's LIOS/FIOS
table omits scripted_variables. An important cross-mod conflict source.

---

### Topics Over-Covered (Should Collapse)

**O-1: The `@[expr]` first-only limitation.**
Documented in: grammar doc §10, clausewitz-local-docs findings G4/§1.3, GUIDELINES.md §3,
mod-architecture-insights §13 item 5, clausewitz-tricks.md, clausewitz-online-research §1.2.C,
API_REFERENCE §12.6. Seven copies. The unified reference should be canonical; others should
cross-reference.

**O-2: `$PARAM$` substitutes inside comments.**
Documented in: grammar doc §6.2/§10, local-docs G5, GUIDELINES.md §3, mod-architecture-insights
§13 item 6, clausewitz-tricks.md, online research §1.6.D. Six copies.

**O-3: Global event targets lost on save/load.**
Documented in: grammar doc §10, local-docs G3/§1.7, GUIDELINES.md §2, mod-architecture-insights
§13 note, clausewitz-tricks.md, API_REFERENCE §1.5. Six copies.

**O-4: Parameterized scripted_effect RAM cost (~2MB per invocation).**
Documented in: grammar doc §10, local-docs G7, GUIDELINES.md §3, mod-architecture-insights §13
item 4, clausewitz-tricks.md, clause design draft (motivation). Five copies.

**O-5: 5-level call-graph depth cap.**
Documented in: grammar doc §10, local-docs G1/§1.4, GUIDELINES.md §3, mod-architecture-insights
§13 note, clausewitz-tricks.md, clausewitz-online-research §1.1. Six copies.

---

## Severity-Ranked Issue List

### Critical

**C-1.** `?=` operator is marked "unconfirmed" in `clausewitz-local-docs-findings.md` Q5 but
confirmed in `clausewitz-grammar.md` and `clausewitz-online-research-findings.md`. The local-docs
doc is used as a design input. Any implementer reading it will treat `?=` as absent from the
lexer's operator set.
**Fix:** Mark Q5 as resolved in local-docs, or simply ensure the grammar doc supersedes it
everywhere. Unified reference must include `?=` as confirmed.

**C-2.** `inline_script` output directory conflict: `common/inline_tools/` (Heritage docs) vs
`common/inline_scripts/` (compiler docs). If the compiler emits to `inline_scripts/` but Heritage
scripts reference files at `inline_tools/`, the references will silently fail at game load.
**Fix:** Check Stellaris vanilla; confirm one canonical path; update all Heritage docs to match.
The compiler and PLAYTEST.md convention (`inline_scripts/`) is almost certainly correct.

**C-3.** `PATCH_FLOW.md` describes a superseded 4-phase pipeline and a patch format (Option B
YAML schema) that does not match the current item-centric compiler. New contributors reading this
doc will be misled about how the compiler works.
**Fix:** Add a prominent "SUPERSEDED" header with a pointer to `PATCH_AUTHORING.md` and
`ARCHITECTURE_V2.md`. Do not delete (valuable design history) but clearly date-stamp.

**C-4.** `calc_true_if` `amount` field: API_REFERENCE Appendix B implies equality-only (`amount = <int>`)
but `amount` supports all six comparison operators per wiki, grammar doc, and online research.
**Fix:** Update Appendix B to show `amount >= N` as the canonical example and document all
supported operators.

### High

**H-1.** Five new lints referenced in grammar doc §10 (`INLINE_EXPR_MULTIPLE_PER_BODY`,
`MACRO_PARAM_IN_COMMENT_HAZARD`, `GLOBAL_EVENT_TARGET_NEEDS_REREGISTRATION`,
`NOT_WITH_MULTIPLE_CONDITIONS`, `NESTED_EVERY_IN_PULSE_EVENT`) are not in `LINT_CATALOG.md`.
**Fix:** Add all five to the catalog before G3e tasks begin.

**H-2.** `LINT_CATALOG.md` Group 5 deprecated syntax does not include a lint for MTTH or for
missing `is_triggered_only = yes`. Both are described as hard rules in GUIDELINES.md.
**Fix:** Add `DEPRECATED_MTTH` (ERROR) and `MISSING_IS_TRIGGERED_ONLY` (ERROR) to the catalog.

**H-3.** `on_multiplayer_game_loaded` is absent from API_REFERENCE on_action tables but is a
required hook for multiplayer save-load global event target recovery.
**Fix:** Add to API_REFERENCE §2.1 global pulses table.

**H-4.** Local-docs findings doc's "Open Questions" section was never updated to mark resolved
questions as resolved. Q1, Q2, Q3, Q5, Q6, Q7, Q8, Q9 all have answers in grammar doc.
**Fix:** Either annotate each Q with "[RESOLVED: see grammar doc §X]" or note at document head
that questions were resolved in the grammar doc.

**H-5.** `clausewitz-grammar.md` §3.6 scope words table is missing `tile` and `design` from the
built-in scope identifiers list (both appear in API_REFERENCE §1.1 scope types table and in the
online research §1.3.E scope keywords list).
**Fix:** Minor but worth adding to avoid `INVALID` tokens for scripts using these scopes.

**H-6.** `hidden:` prefix on effect keys (documented only in cwtools review §7j) is not in any
doc that compiler implementers or mod authors would read. If a vanilla or mod script uses
`hidden:add_modifier`, the current lexer plan has no token type for it and would treat it as an
unknown identifier.
**Fix:** Add to grammar doc §3.1 as a special token prefix form, add to clausewitz-tricks.md,
and add a note in GUIDELINES.md.

### Medium

**M-1.** `clausewitz-grammar.md` §3.7b color-space tags disagree with local-docs §1.11 on
whether `hsv`/`rgb` should be reserved keywords or plain IDENTs. Grammar doc says IDENT (correct),
local-docs says KW_HSV/KW_RGB (superseded). The unified reference should settle this with the
grammar doc's reasoning.

**M-2.** `PATCH_FLOW.md` describes `megapatch/generated/` directory as existing in VCS (with a
"commit but with GENERATED header" decision). Current compiler outputs to `.dist/` which is
gitignored. The decision recorded in PATCH_FLOW.md does not match current practice.

**M-3.** The `inline_script` placement restrictions (cannot be used inside plain lists; "unexpected
token" in unsupported locations) are documented in online research §1.6.E but absent from
GUIDELINES.md and API_REFERENCE.

**M-4.** `clausewitz-tricks.md` says `@[ x + y ]` "evaluates at parse time." The correct phrasing
is "evaluates at load time" (game engine load, not the Clausewitz file parser run time). This
matters for understanding when `@[ ... ]` expressions with `$PARAM$` inside are evaluated
(parameter substitution happens first, then arithmetic is evaluated).

**M-5.** The Situations Clause backend in `clausewitz-tricks.md` says "Stellaris 3.5+." The
API_REFERENCE correctly says `situation` scope is v3.3+. If Heritage targets Stellaris 3.3+
situations, the tricks doc is misleadingly conservative.

**M-6.** Heritage `GUIDELINES.md` §4 LIOS/FIOS table omits `scripted_variables` (FIOS) and
`game_rules` (appears to be LIOS). Online research and CLI_DESIGN.md have more complete tables.

**M-7.** Seven project docs use the word "parse time" and "load time" interchangeably for
`@[ expr ]` evaluation timing. A unified reference must pick one term and use it consistently,
or explain the distinction (Clausewitz parser load = Stellaris game engine initialization).

**M-8.** The "recursion-tolerant" label for inline_script chaining in clausewitz-tricks.md is
technically inaccurate. "5-iteration inline_script chain expansion" is more accurate terminology.

### Low

**L-1.** `TENETS.md` uses "depth-5 trigger nesting" informally. Fine for a tenet document but
should not be cited in scripting references.

**L-2.** `mod-language-design-startpoint.md` uses file extension `.mp` and decorator syntax that
differs from the final design draft. Should have a header redirect.

**L-3.** `clausewitz-tricks.md` "Confidence: primary" for `[[PARAM] body ]` could be more specific:
"confirmed for scripted_effects and inline_scripts; conditionals in scripted_triggers unconfirmed."

**L-4.** `GUIDELINES.md` §8 says "Script files: UTF-8" without clarifying the BOM requirement is
NOT needed for script files (only for localisation). Should say "Script files: UTF-8 (no BOM)."

**L-5.** Grammar doc and local-docs docs use "load order" to mean two different things: (1)
Stellaris mod load order (which mod loads first) and (2) file-system load order (which file in a
directory is processed first). The unified reference should distinguish these as "mod load order"
and "file load order."

**L-6.** Multiple docs call `random_list` and `locked_random_list` without mentioning that
`random_list` is non-deterministic (different results between game runs) while `locked_random_list`
is deterministic per-instance (same seed = same result). This distinction matters for multiplayer.

---

## Proposed Unified Reference Structure

**Target file:** `docs/megapatch/clausewitz-scripting-api-reference.md`

This document should supersede scattered Clausewitz knowledge across all current docs. Each section
below includes what to include and which existing doc is the canonical source.

---

```
# Clausewitz Scripting API Reference
# Stellaris Dialect — Project Canonical Reference

> This document supersedes clausewitz-local-docs-findings.md (resolved questions),
> clausewitz-online-research-findings.md (integrate citable facts), and the 
> grammar-doc §10 "known constraints" section (for scripting behavior — the grammar
> doc remains authoritative for lexer/token taxonomy).

## 0. How to Use This Reference

Short orientation: what is Clausewitz scripting, what does this doc cover, what does it NOT cover.

## 1. Syntax Fundamentals

### 1.1 Token Types (canonical reference: clausewitz-grammar.md §3)
- Identifiers (IDENT, VAR_DECL_AT, VAR_REF_AT, TARGETED_VAR)
- Operators (full 8-operator set with confirmed presence/semantics)
- Literals (INT, FLOAT, DATE, STRING, BOOL, NULL)
- Brackets and punctuation (DOT, PIPE, braces)
- Trivia (COMMENT, WHITESPACE)
- Special forms (EVENT_TARGET_REF, VAR_SCOPE_REF, MACRO_PARAM, INLINE_EXPR)

### 1.2 Identifier Grammar
- Legal characters, dotted forms, quoted-key forms
- Case sensitivity rules (keywords: case-insensitive; string values: case-sensitive)
- `@variable` prefixes (file-local vs global distinction)
- Hyphen and special characters in mod IDs
- Dynamic identifiers (`set_flag = name@root` form)

### 1.3 Operators
All 8 operators with confirmed sources:
- `=` (overloaded: assignment / equality / scope-change / activation)
- `>`, `<`, `>=`, `<=`, `!=` (numeric comparisons)
- `==` (explicit equality — exists, exact semantics unverified)
- `?=` (exists-then-compare — confirmed via cwtools QuestionEqual + PDX Tools + CK3 wiki)
- Absent: `<>` (not a real operator)

### 1.4 Literals
- Integer forms (32-bit signed, 64-bit unsigned, leading +/-)
- Float forms (32-bit, ~4 decimal digits, no scientific notation)
- Date forms (Y.M.D, no leap years)
- String forms (double-quoted, confirmed escapes: \" and \\, literal newlines permitted)
- Boolean: `yes`/`no` (with lookahead guard for yes_please → IDENT)
- Null: `none` (distinct from `no`)
- Color literals: `rgb { R G B }`, `hsv { H S V }`, `hsv360 { H S V }`, `hex { RRGGBBAA }`
  (INT or FLOAT components; alpha via 4th component or hex 8-char)

### 1.5 Block Structure
- Key-value assignments (name op value, name op block)
- Bare value lists (unkeyed blocks)
- Top-level scalar assignments (including @variable declarations)
- Brace matching and depth constraints

## 2. Variables and State

### 2.1 File-Local @variables
- Syntax: `@name = value` at file head
- Scope: file-only; cannot be referenced outside the declaring file
- Format: only numeric values (int/float) — no string @vars
- Ordering: must precede first block opener
- Word-boundary matching rule (sort longest names first to prevent prefix collisions)
- `@var = @[ expr ]` form: arithmetic expression as declaration value

### 2.2 Global Scripted Variables (common/scripted_variables/)
- Available across all files after game load
- FIOS load order: first definition wins; name collision produces log error
- Cannot be individually overridden by load-order mechanics
- Common cross-mod conflict source

### 2.3 @[ Arithmetic Expressions ]
- Operators: `+`, `-`, `*`, `/` only (other operators cause string concatenation)
- FIRST-ONLY LIMIT: only the first @[ ... ] per scripted_effect or scripted_trigger body is
  evaluated; subsequent ones silently wrong
- Parameters inside @[ ]: `@[ $COUNT|1$ * 10 ]` — parameter substituted before arithmetic
- Evaluates at game-load time (not Clausewitz-parser parse time)

### 2.4 Script Values (common/script_values/, v3.3+)
- Full operations list: set, weight, add, subtract, factor, mult, multiply, divide, modulo,
  round_to, max, min, pow, round, ceiling, floor, abs, square, square_root
- Invocation: `value:my_value` or `value:my_value|PARAM1|val1|`
- Pipe-delimited parameter form
- `complex_trigger_modifier` form for conditional arithmetic

### 2.5 Runtime Variables (set_variable / check_variable)
- Numeric only: fixed-point, -2,147,483.648 to 2,147,483.647, 0.001 precision
- Variables set to 0 are NOT serialized (implicit cleanup)
- Must guard reads with `is_variable_set`
- Scope: country, planet, pop, leader, fleet, etc.

### 2.6 Flags
- Binary presence/absence per scope
- Cheaper than variables for boolean state
- Timed flags: `set_timed_flag` for self-cleaning state
- Dynamic flag syntax: `set_flag = name@scope` (produces `name_<scope_id>`)

## 3. Scopes

### 3.1 Scope Reference Keywords
- THIS, ROOT, PREV (+ prevprev, prevprevprev, prevprevprevprev — max 4 concatenated)
- FROM (+ fromfrom, fromfromfrom, fromfromfromfrom — max 4 concatenated)
- Dot-notation form (`prev.prev`, `from.from`) — preferred over concatenated
- DEPRECATED: prevprev as standalone token (broken in diplomatic triggers and opinion_modifier)

### 3.2 Built-in Scope Identifiers
Full table from API_REFERENCE §1.1 and online research §1.3.E:
country, planet, fleet, ship, pop, species, galactic_object, sector, army, federation/alliance,
war, starbase, megastructure, deposit, archaeological_site, pop_faction, first_contact,
espionage_operation, spy_network, situation, agreement, ambient_object, debris, design...

### 3.3 Scope Navigation
- Dot-chain notation: `owner.capital_scope.solar_system = { ... }` — does NOT push PREV context
- Dot-chaining limitation: cannot dot-chain into scope-changing triggers/effects
- exists guard: `exists = owner` before `owner = { ... }` for optional scopes

### 3.4 Event Targets
- Save: `save_event_target_as = name` (event-chain local)
- Save global: `save_global_event_target_as = name` (global — LOST on save/load)
- Access: `event_target:name = { ... }`
- Dynamic: `save_event_target_as = name@root` (appends scope ID, v3.5+)
- Global targets must be re-registered in:
  - `on_single_player_save_game_load` (single-player only — does NOT fire in multiplayer)
  - `on_multiplayer_game_loaded` (multiplayer)

### 3.5 Scope Context Model (for compiler/analyzer implementers)
Based on cwtools §5f:
`{ Root: Scope; From: list[Scope]; Scopes: list[Scope] }`
PREV pops Scopes stack.

## 4. Boolean and Control Flow

### 4.1 Logical Trigger Operators
- `AND = { ... }` — implicit when no operator is present
- `OR = { ... }`
- `NOT = { ... }` — SINGLE condition intended; WARN: multiple conditions behave like NOR
  (wiki: "can cause unforeseen bugs"; community: acts as NOR — treat as unreliable)
- `NOR = { ... }` — explicit multi-condition negation
- `NAND = { ... }` — not all must be true
- `calc_true_if = { amount >= N ... }` — amount supports >=, <=, >, <, = operators
- Case-insensitive: `AND` = `and` = `And` (all valid, lexer normalizes)

### 4.2 Control Flow — Effects
- `if = { limit = { ... } ... }` / `else_if` / `else` (added v2.1.0)
- `switch = { trigger = <key> <value> = { ... } default = { ... } }` — first match wins
  (comparison operators `>`, `<`, etc. valid as case keys for numeric triggers)
- `while = { count = N ... }` / `while = { limit = { ... } ... }` — iteration cap: CK3=1000,
  Stellaris presumed same (unconfirmed)
- `break = yes` — stops remaining effects in block
- `random_list = { <weight> = { ... } }` — non-deterministic
- `locked_random_list = { ... }` — deterministic per-instance (same seed = same result)
- `hidden_effect = { ... }` — no tooltip
- `log = "..."` / `assert_if = { ... }` — debug effects

### 4.3 Control Flow — Triggers
- `trigger_if = { limit = { ... } ... }` / `trigger_else_if` / `trigger_else`
- `hidden_trigger = { ... }` — no tooltip
- `optimize_memory` — engine hint for scope-allocation caching in complex triggers

## 5. Iterators

### 5.1 Four Standard Iterator Variants
Each "list" in the game system generates:
- `any_<X>` — TRIGGER: boolean "any member matches" (scope change into member)
- `every_<X>` — EFFECT: execute on ALL matching members
- `random_<X>` — EFFECT: execute on ONE randomly selected matching member
  (supports `weights = { base = float modifier = { ... } }`)
- `count_<X>` — TRIGGER: count matching members, returns number

Plus (v3.2+):
- `ordered_<X>` — EFFECT: deterministic iteration; `position`, `order_by`, `inverse`

### 5.2 Iterator Performance Rules
- Iterators are NOT reserved keywords — engine pattern-matches on prefix; mods can define new lists
- `every_*` is O(n): nested → O(n×m); MUST NOT appear in monthly pulse events
- `any_*` for boolean checks; `count_*` for counts; `every_*` only when applying effects
- Always use the narrowest available scope (`every_owned_leader` vs `every_country_leader`)

## 6. Inline Scripts and Parameters (v3.5+)

### 6.1 Directory and Invocation
- Location: `common/inline_scripts/` (vanilla convention; verify project-specific path)
- String form: `inline_script = "path/to/script"` (no extension, relative to common/inline_scripts/)
- Block form: `inline_script = { script = "path" KEY = value ... }`
- NOT supported in all contexts (plain-item lists reject it; unsupported → "unexpected token")

### 6.2 Parameter Substitution ($PARAM$)
- `$PARAM$` substituted textually before parsing; applies to ALL occurrences including comments
- Default values: `$PARAM|default$` — confirmed for inline_script and scripted_effect bodies;
  unconfirmed for scripted_trigger bodies
- Parameters in KEY position: `$KEY$ = value` generates dynamic key-value pairs
- Substitution is iterative (up to 5 iterations for nested inline_scripts)

### 6.3 Conditional Parameter Blocks
- `[[PARAM] content ]` — include content only if PARAM is supplied (not absent/no)
- `[[!PARAM] content ]` — include content only if PARAM is absent or no
- Confirmed for scripted_effects and inline_scripts; EU4 origin, used across Clausewitz games

### 6.4 Inline_Script Performance
- Parse-time text substitution: NO per-call RAM cost (vs ~2MB per parameterized scripted_effect use)
- Parameterized scripted_effects: ~30 compiled branches × ~2MB RAM per invocation site
  → prefer inline_scripts for code reuse

## 7. Scripted Effects and Triggers

### 7.1 Call-Graph Depth Cap (parameterized)
- Hard cap: 5 frames (A→B→C→D→E; depth 6 silently fails — no error, wrong behavior)
- Applies to parameterized scripted_effects; non-parameterized may not have same cap (unconfirmed)
- inline_script BYPASSES this cap (parse-time substitution, not call-stack)
- `inline_script` promotion is the recommended mitigation for deep call chains

### 7.2 Brace-Nesting Depth
- Lint fires at brace depth ≥ 6 in trigger/effect directories
- Correlated with loading-screen hangs; PDX Tools: "can be infinitely deep" per engine design
  but practical hangs occur at high depths

### 7.3 Scripted Variable Scoping
- File-local @vars in scripted_effect/trigger files: file-scoped only; inline before emitting
  extracted items to new files
- Global: `common/scripted_variables/` — FIOS; compiler must NOT inline these (other mods override)

## 8. File Format

### 8.1 Encoding
- Vanilla script files: Windows-1252
- Modern mod files: often UTF-8
- Auto-detect strategy: UTF-8 BOM → UTF-8 → Windows-1252 fallback
- Localisation (.yml): UTF-8 WITH BOM required; plain UTF-8 silently fails
- Script files (.txt): UTF-8 (no BOM needed)

### 8.2 Indentation
- Vanilla convention: tabs (not spaces)
- Mixing tabs/spaces: does not cause parse failure but produces subtle visual bugs in error output

### 8.3 Load Order
- LIOS (Last In Only Served): buildings, districts, technologies, traditions, interface, localisation
- FIOS (First In Only Served): component_templates, event_chains, global_ship_designs, events,
  scripted_variables, inline_scripts
- No override (full file replacement only): traits, terraform, strategic_resources
- File ordering within a directory: ASCIIbetical; `~` prefix loads first, `!` prefix loads last

## 9. Performance Reference

### 9.1 Engine Cost Hierarchy (cheapest to most expensive)
1. Flags (`has_flag`) — C++ boolean/existence
2. Scope type checks, pre_triggers — C++ level
3. Traits (`has_trait`) — C++ list lookup
4. Variables (`check_variable`) — scope lookup + comparison
5. Direct scope traversal — single link
6. Dot-chain traversal — multi-link, no PREV push
7. `any_*` triggers — iterates full list, returns boolean
8. `every_*` effects — iterates full list, applies effects
9. Explicitly expensive: `can_access_system`, `check_economic_production_modifier_for_job`
10. Cross-empire iteration — O(n×m), event-only

### 9.2 Pre-Triggers
- Evaluated at C++ level before any script trigger (essentially free)
- Available on planet, pop, system, starbase, leader events
- Available: `has_owner`, `is_homeworld`, `is_ai`, `is_idle` (leader), etc.
- Always use pre_triggers on eligible event types

### 9.3 Short-Circuit Ordering
- AND blocks: put most-likely-to-fail check FIRST
- OR blocks: put most-likely-to-succeed check FIRST
- `is_ai = no` as first check skips 99% of entities in most contexts

### 9.4 Pulse Event Performance Budget
- `every_*` nested iterators MUST NOT appear in monthly pulse events
- Script values in GUI recalculate every frame — keep calculations lightweight
- Use `hide_window = yes` on computation-only events

## 10. Engine Gotchas Reference

Numbered reference for cross-linking from lints, compiler passes, and other docs:

| # | Gotcha | Severity | Lint |
|---|--------|----------|------|
| G1 | `@[ expr ]` first-only per body | ERROR | INLINE_EXPR_MULTIPLE_PER_BODY |
| G2 | `$PARAM$` substitutes in comments | WARNING | MACRO_PARAM_IN_COMMENT_HAZARD |
| G3 | `save_global_event_target_as` lost on save/load | ERROR | GLOBAL_EVENT_TARGET_NEEDS_REREGISTRATION |
| G4 | `prevprev` standalone broken in diplomatic/opinion_modifier scopes | WARNING | PREVPREV_IN_ACTIVE_CODE |
| G5 | 5-frame call-graph cap on parameterized scripted_effects | ERROR | depth analyzer |
| G6 | `NOT = { A B }` behaves like NOR (unreliable per wiki) | WARNING | NOT_WITH_MULTIPLE_CONDITIONS |
| G7 | Nested `every_*` iterators O(n×m) in pulse events | WARNING | NESTED_EVERY_IN_PULSE_EVENT |
| G8 | Dot-chain scoping does NOT push PREV context | INFO | DEEP_SCOPE_CHAIN (partial) |
| G9 | Non-parameterized scripted_effects depth cap: unconfirmed | - | - |
| G10 | `inline_script` not supported in plain-item lists | ERROR | - |
| G11 | `on_single_player_save_game_load` does NOT fire in multiplayer | ERROR | - |
| G12 | Variable set to 0 = not serialized (use for cleanup) | INFO | - |
| G13 | Variables read without `is_variable_set` guard → errors | WARNING | - |
| G14 | 1000-iteration `while` limit (CK3 confirmed; Stellaris unconfirmed) | WARNING | - |
| G15 | `scripted_variables` FIOS: first definition wins, no override | WARNING | - |

## 11. Unresolved / Unconfirmed Facts

Document facts the project knows are uncertain. Each should link to where investigation should
go to resolve them.

| # | Question | Source | Status |
|---|----------|--------|--------|
| U1 | Does the 5-frame depth cap apply to non-parameterized scripted_effects? | online-research §2.F | Unverified |
| U2 | Does Stellaris have the 1000-iteration while loop cap (confirmed CK3)? | online-research §2.A | Unverified |
| U3 | What are the exact semantics of `==` vs `=`? | online-research §2.D | Unverified |
| U4 | Does `$PARAM|default$` work in scripted_trigger bodies? | local-docs Q6 | Unverified |
| U5 | Does the 4-hop prev/from limit produce a hard engine error or undefined behavior? | local-docs Q2 | Unverified |
| U6 | Does `while` support both `count` and `limit` simultaneously? | online-research §4.9 | Unverified |
| U7 | What exact Stellaris version added `trigger_if`/`trigger_else`? | online-research §4.12 | Unverified |

## Appendix A: Reserved Keywords Quick Reference
(All token kinds from grammar doc §3.5)

## Appendix B: Scope Types Quick Reference
(From API_REFERENCE §1.1, kept in sync)

## Appendix C: Moddable Directory Reference with FIOS/LIOS Rules
(From GUIDELINES §4 + CLI_DESIGN.md, authoritative merged table)

## Appendix D: On_Actions Reference
(From API_REFERENCE §2.x — preserve THIS/FROM/FROMFROM columns)
```

---

**Key decisions for the unified reference:**

1. Grammar doc remains authoritative for **lexer/token taxonomy**. Unified reference covers
   **scripting behavior** (what the tokens mean semantically, how the engine uses them).
2. `clausewitz-online-research-findings.md` is the citation source for all facts with external
   URLs — the unified reference should cite those URLs directly.
3. The API_REFERENCE on_actions tables are unique and high-value; import them wholesale.
4. All "gotcha" content collapses into §10 with lint cross-references.
5. Unresolved/unconfirmed facts belong in §11, not scattered as parenthetical hedges.
6. The `inline_tools` vs `inline_scripts` conflict MUST be resolved before §6 is written.
