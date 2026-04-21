# Clause Migration Review: From "Design Complete" to "Actually Ships"

**Date:** 2026-04-16
**Reviewer lens:** Migration and adoption — someone who has shipped large codebase transitions and watched others fail.
**Source material:** `docs/megapatch/design/2026-04-16-mod-language-design-draft.md` (§28 focus), `docs/megapatch/PATCH_AUTHORING.md`, `docs/megapatch/STALE_PATCHES.md`, `.cache/reviews/2026-04-16/typed-dsl-prior-art.md`, Heritage patch corpus (~44 files, 1,058 items), Megapatch patch corpus (~4,530 files, 236 declaration files).

---

## 1. TL;DR

**Verdict: Maybe — with serious scope caveats.**

Phases 1 and 2 will probably succeed because the scope is tight and the project owner is the only user. Phases 3 through 5 are where migrations of this type historically fail. The design is technically sound; the migration plan is optimistic in ways that matter.

The core tension: this is simultaneously a *language design project*, a *compiler implementation project*, a *50,000-line codebase migration*, and a *4,500-patch corpus migration* — all scoped to a part-time solo or near-solo effort. Any one of those is a multi-month commitment. Stacked and sequential, the risk is that the project stalls in Phase 3 with Heritage permanently half-migrated, the compiler permanently "good enough for Bloodline," and the YAML system permanently "temporary but here to stay."

The honest prior-art comparison is TypeScript adoption inside a single company. When Microsoft did it, they had dedicated tooling teams, automated migration paths, and engineering leadership mandates. The equivalent here is: one person, no mandate except self-imposed, competing against the fun of actually playing the game and adding features.

That's not a fatal objection. It's the condition that changes the strategy. The recommendation at the end of this document accounts for it.

---

## 2. Per-Phase Risk Analysis

### Phase 1 — Clause Compiler MVP

**Risk score: MEDIUM**

The acceptance gate is: `clause build crates/hello-world` emits valid Clausewitz that Stellaris can load.

**Top concern: False victory.**

A trivial `.cw` file that emits `scripted_effect = { }` proves the lex/parse/emit loop works. It does not prove:

- Kind inference on ambiguous bodies (95% claim in §9.4 — what are the 5%?)
- The `Bind` sealed-trait enforcement (requires the type checker to be real, not nominal)
- `super.foo()` patch chain resolution (the most novel mechanism in the design)
- Auto-generated naming collision detection across crate boundaries
- The `EmitPlan` stream being correctly consumed by the existing megapatch backend

The gate as written would be satisfied by a compiler that hard-codes the output for the specific test input. That sounds absurd but it's exactly what happens when acceptance criteria are too narrow — you optimize to pass the gate, not to prove the hard things.

**Second concern: M2–M7 are bigger than they look.**

The M-batch task list (M2 lexer → M3 parser → M4 resolver → M5 type checker → M6 transpile → M7 codegen) is a complete compiler frontend implementation. Each task is listed as a discrete unit, but the dependencies are tight: you cannot test M5 without M4, cannot test M6 without M5. Real implementation time for a type checker that handles generics, orphan rule enforcement, sealed traits, and pattern-match exhaustiveness is not "2–4 weeks." M5 alone is where most typed-language implementations spend 60–80% of their effort.

The 4–8 week Phase 1 estimate is plausible only if "minimal typecheck.py" means "validates a trivial struct with one method and no generics." If it means "validates anything resembling real Heritage code," it's 3–6 months.

**What Phase 1 must prove to be non-false:** It should require the hello-world to exercise kind inference (one trigger, one effect, one mixed-body that errors), one generic type, one `impl Trait for Struct`, and one `#[patch]` with `super.foo()`. If all of those work, the hard architectural bets are confirmed.

---

### Phase 2 — Proof of Concept: Bloodline in Clause

**Risk score: HIGH**

This is the phase with the most ways to look like success while hiding failure.

**Concern 1: What is "Bloodline" actually?**

After examining the actual corpus, Heritage's Bloodline subsystem is in `megapatch/heritage/patches/10_heritage_bloodline_vector.yml`. It is not vanilla Clausewitz — it is already YAML-patch-authored content. The "byte-identical" gate means: Clause compiles to output that, after passing through the existing megapatch backend, produces the same `.txt` files as the current YAML-authored version.

Looking at the actual content, `heritage_create_bloodline` alone is a ~100-line if/else_if chain that manually implements indexed array access across 8 slots. This is exactly the kind of code that Clause's `Array<T, N>` abstraction is designed to hide. But "byte-identical output" means the Clause version must emit that same 100-line if/else_if chain, not a better-structured version of it. That's a fundamental tension.

**Concern 2: Byte-identical is probably unachievable.**

The design acknowledges this implicitly — §28 Phase 2 says "byte-identical or diff-approved by user." The "diff-approved" escape hatch is doing a lot of work here. In practice, it means: Clause will emit semantically equivalent but structurally different Clausewitz, and the user will review the diff and say "that looks right." That's fine for Phase 2 purpose-finding, but it means the output is NOT identical, which means the "acceptance gate" is subjective from day one.

Sources of non-determinism that would break byte-identical:
- Auto-generated variable names (`heritage_bloodline_add_member_ok` vs however the current YAML names it). The existing code uses `heritage_tmp_bl_id` as a temp variable. Clause would generate something like `heritage_bloodline_add_member_value`.
- Whitespace and comment differences (Clause adds `# >>` source-map comments; current YAML has none).
- Ordering of emit items within a file (Clause routes by module path; existing YAML routes by declaration order).
- `const` inlining at use sites vs keeping variables around.

"Diff-approved" is the right gate. "Byte-identical" is a red herring that should be dropped from the spec.

**Concern 3: The "smaller, better-structured output" is actually success, not failure.**

If the migrated Bloodline produces correct Clausewitz that is 40% shorter because Clause's `Array<T, N>` properly abstracts the 8-slot if/else chains, that's a win. But it's also a bigger diff to review, harder to verify by inspection, and the user now needs to trust the compiler's array codegen. This is a scope expansion relative to "migrate without changing anything."

Recommendation: accept "diff-approved, in-game functionally verified" as Phase 2's real gate. Drop "byte-identical" from the vocabulary.

**Concern 4: One migrated subsystem + compiler bug = silent corruption of adjacent subsystems.**

The bloodline vector uses `event_target:heritage_registry`. If the compiler's emit for `event_target` is wrong (or right in isolation but wrong when composed with adjacent files), the bug won't surface until Heritage is loaded in-game. The YAML pipeline currently handles this because the backend is battle-tested; the Clause frontend is untested. Phase 2 needs a regression test that runs the full Heritage build and loads it in Stellaris, not just a diff check.

---

### Phase 3 — Heritage Full Migration

**Risk score: HIGH**

This is the longest phase and the one most likely to stall permanently.

**Concern 1: Heritage is already in-flight.**

At the time of this review, Heritage has 44 active YAML patch files representing ~46,000 lines of compiled output, with active development ongoing (situations system, Dawn of Civilization, courtship pipeline). Phase 3 asks for subsystem-by-subsystem migration while also fixing bugs and adding features.

The rebase problem: if you're migrating `heritage_children.yml` to Clause and a bug is discovered in the children system, where does the fix go? Options:
- Fix in the Clause version (requires the Clause version to compile cleanly, which may require the type checker to handle all the children constructs, which may not be ready yet).
- Fix in the legacy YAML version (doubles the maintenance burden; the fix must eventually be re-expressed in Clause).
- Hold the bug until the migration is done (unacceptable for a mod in active use).

The design says "gradual adoption — coexists with YAML patches and raw Clausewitz." But the coexistence story for CROSS-FILE dependencies is underspecified. If `heritage_marriage.yml` (not yet migrated) calls `heritage_bloodline_add_member` (already migrated via Clause), does the call work? The answer is "yes, at the emitted Clausewitz level" — but only if the Clause-emitted function name matches what the YAML version expects to call. If Clause renames anything during migration, every un-migrated caller breaks.

This is the half-migrated codebase problem. It has killed more migrations than any other cause.

**Concrete requirement that the design doesn't guarantee:** During the entire Phase 3 migration window, every emitted symbol name from migrated subsystems must be stable and identical to what they were pre-migration. The compiler must enforce this via `#[name("...")]` annotations or similar. There must be a lint that fires if any migrated symbol's emitted name changes.

**Concern 2: The incentive problem.**

Who is fixing bugs and adding features during Phase 3? If it's the project owner, they face a continuous choice: fix the bug in YAML (fast, familiar, unblocked by compiler readiness) or fix it in Clause (slower, requires compiler to handle the construct, requires understanding the migration state of the subsystem). Under deadline or motivation pressure, YAML wins every time. Each YAML fix is technical debt against the migration.

If it's a future collaborator, they face a worse version of this: they don't know Clause yet, the compiler may have rough edges, and the familiar path (YAML) is always available. The migration stalls.

The design has no mechanism to prevent this. The incentive gap is structural.

**Concern 3: The testing matrix.**

§28 Phase 3 says "subsystem-by-subsystem migration, each step diff-validated." But the diff validation only catches regressions in the migrated subsystem. Interactions between migrated and un-migrated subsystems need integration testing. Heritage has ~44 interdependent subsystems; the test matrix for "3 migrated, 41 un-migrated" is not 3 tests, it's 44 tests (every subsystem must still work in the mixed state).

There is no testing infrastructure described for this. "Loads in-game, plays identically" is the only acceptance criterion, and in-game testing is expensive and slow.

**Concern 4: Ordering difficulties.**

The proposed order (Bloodline → Family → Marriage → Succession → Intrigue → Dawn → events layer) follows logical dependency. But many Heritage subsystems are cyclically interdependent at the scripted-effect level. `heritage_marriage.yml` calls effects from `heritage_family_system.yml`, `heritage_inheritance_engine.yml`, and `heritage_relationships.yml`. Migrating Marriage to Clause before its dependencies are migrated requires either (a) keeping all callee symbols stable, or (b) maintaining two versions of the callee (YAML for the Clause version to call, Clause for the compiler to type-check against).

This is tractable but not trivial. The design doesn't acknowledge the depth of these cross-subsystem wiring dependencies.

---

### Phase 4 — Megapatch Crate Migration

**Risk score: VERY HIGH**

This is the phase the design calls "mostly mechanical." It is not.

**The actual corpus:**

After examining the patch files:
- 4,294 per-item `.yml` files
- 236 `_declarations.yml` files
- 581 files with `replace_in_item` patches (textual find/replace in raw Clausewitz strings)
- 812 files with `inject_field` patches (field injection into structured content)
- 545 files with `override` patches (full content replacement)
- Additionally: 688 compile errors from stale patches that don't even work yet

**The translation problem for each patch type:**

| Current type | Clause equivalent | Translation difficulty |
|---|---|---|
| `prefer_mod` | `#[patch]` impl with `..super` spread | Medium — mechanical but requires reading all source mods |
| `no_override` | No action needed (orphan rule prevents it automatically) | Easy |
| `override` | `#[patch]` impl replacing entire body | Medium — mechanical if the content is complete |
| `inject_field` | `#[patch]` with `..super, new_field: value` | Hard — requires understanding the field position semantics |
| `replace_in_item` | `#[patch]` with `..super, field: new_value` or method body rewrite | VERY HARD — find/replace on raw Clausewitz strings has no direct Clause analog |
| `insert_item` | `#[patch_extend]` | Medium |
| `delete_item` | `#[patch]` with empty body or field omitted | Medium |
| `append_content` | `#[patch_extend]` | Medium |

The `replace_in_item` case is the hard one. The current YAML format allows:
```yaml
type: replace_in_item
find: "max_speed = 6"
replace: "max_speed = 8"
```
This is a textual operation on raw Clausewitz. Clause's `..super` struct spread can only override named fields it knows about. If the field being replaced is nested (inside `ai_weight = { factor = ... }`), it requires the Clause type system to know the nested structure of every vanilla and third-party type. The `stellaris-vanilla-spec` crate covers vanilla — but 581 patches operate on third-party mod content (NSC3, ESC, ACOT buildings with custom fields, etc.). Writing Clause extern declarations for every custom field of every third-party mod is not "mostly mechanical."

**The stale patch problem compounds this.** There are currently 688 patches that don't compile because the source mods changed. These patches are already wrong in YAML form. Translating them to Clause doesn't fix them — it translates broken patches to broken Clause. The Phase 4 migration and the stale-patch remediation effort need to happen together, not sequentially.

**Time estimate: not 375–750 hours.**

The design's "5–10 minutes per patch" is plausible for the simple cases (prefer_mod, no_override, clean override). It is not plausible for:
- replace_in_item patches requiring reading two source mods, understanding what changed, and expressing the change as a field-level override (30–90 minutes each, 581 files)
- inject_field patches with non-trivial positioning semantics (15–45 minutes each, 812 files)
- Stale patches that need to be fixed AND translated simultaneously

A more realistic estimate for the non-trivial patches alone (replace_in_item + inject_field + stale): 1,400+ files at 30–90 minutes average = 700–2,100 hours. At 10–15 hours per week part-time, that's 1–4 years for Phase 4 alone.

**The `replace_in_item` design gap.**

The most direct risk: `replace_in_item` has no clean Clause equivalent, by design. Clause's philosophy is "take the whole struct, override specific fields." But 581 patches are doing surgical string surgery inside nested Clausewitz blocks that the type system doesn't model. Either:

(a) Clause needs a raw-text escape hatch for `replace_in_item` semantics (contradicts the design philosophy)
(b) The affected patches must be rewritten as full `override` patches with all fields spelled out (expensive, requires reading all source mods)
(c) A special `#[patch_replace(...)]` decorator is added to the language (new design work)

None of these options is free. Option (a) defeats much of Clause's value. Option (b) is the bulk of the 700–2,100 hour estimate. Option (c) is adding back complexity that was intentionally removed.

---

### Phase 5 — YAML Retirement and Community Publication

**Risk score: MEDIUM** (if Phase 4 completes; effectively MOOT otherwise)

**Top concern: The YAML loader is not the only consumer of the YAML format.**

Heritage's own 44 patch files are YAML. The megapatch corpus is YAML. Documentation, tooling (trace, feature), and CI all speak YAML. Removing the YAML loader requires ensuring nothing silently depends on it. The acceptance gate (`grep -r '\.yml' crates/` returns zero) is too narrow — it doesn't catch YAML still being used in tooling, documentation, or developer workflows.

**Second concern: Community publication with no community.**

Phase 5 mentions "optionally spin out Clause + stdlib + stellaris-vanilla-spec as a standalone tool repo." This is worth examining honestly: who would adopt Clause? The Stellaris modding community is primarily:
- Experienced Clausewitz authors who have no incentive to learn a new language
- Casual content modders who don't need type safety
- A small cohort of technically sophisticated modders who might benefit

CWTools (the existing typed-validation layer) has been available for years and has decent adoption among serious modders. Clause's value proposition over CWTools is significantly stronger (compile-time vs IDE-only, full language vs validation), but adoption of Clause requires the community to trust the compiler's output, learn new syntax, and accept a tool with limited documentation and no track record.

Publication without community infrastructure (documentation site, examples, package registry for extern declarations of popular mods, Discord/forum presence) is not adoption — it's open-sourcing. Those are different things.

---

## 3. Critical Preconditions

These must be true before starting, and the design hasn't explicitly guaranteed them.

**P1: Emitted symbol names must be migration-stable by default.**

The most dangerous failure mode during Phase 3 is: Clause migrates Bloodline and auto-generates `heritage_bloodline_contains` as the function name, but the legacy YAML files call `heritage_bloodline_check_membership` (the real name in the existing corpus). Every cross-subsystem call breaks. There must be an explicit mechanism (enforced, not optional) for the first migration of any existing symbol to preserve its existing name. This is different from the `#[name("...")]` escape hatch — it needs to be the default behavior during migration, with explicit opt-in to auto-naming.

**P2: A round-trip regression test must exist before Phase 2 starts.**

"Build Heritage in its current YAML form, verify it loads in Stellaris" must be an automated, repeatable test that can be run after every compiler commit. Without this, there's no baseline to regress against, and "diff-approved" has no source of truth.

**P3: Kind inference must be validated against real Heritage bodies before Phase 2.**

The "95% unambiguous" claim for kind inference needs to be empirically validated against the existing Heritage corpus BEFORE committing to Phase 2. If 20% of Heritage's existing scripted_effects would be classified as "ambiguous, add #[prefer]" by the compiler, that's hundreds of manual annotations in Phase 3 that the design doesn't budget for.

**P4: The replace_in_item gap must have an explicit resolution before Phase 4 begins.**

Do not enter Phase 4 without a documented answer to "how does replace_in_item translate to Clause?" The three options (escape hatch, full-rewrite, new decorator) each have different implications for Phase 4 scope and time. This decision should be made when the Phase 3 migration reveals how many existing Heritage patches use text-surgery patterns, not deferred to when Phase 4 is already underway.

**P5: Heritage must not be in active feature development during active migration of a subsystem.**

Concurrent feature development and migration of the same subsystem is not viable. The migration window for a specific subsystem should be a bounded sprint: no new features added to that subsystem until migration is complete and validated. This requires deliberate scheduling that the design doesn't mention.

---

## 4. Scope Reduction Options

If the full migration is too ambitious, here are valuable smaller versions, ordered by cost-to-value:

**Option A: Phase 1 only, language as authoring layer for NEW Heritage content.**

Don't migrate existing content. Use Clause for all new Heritage subsystems going forward. Let legacy YAML coexist indefinitely. This gives the project the type-safety and abstraction benefits for new work while avoiding the migration risk entirely. Cost: Phase 1 (4–8 weeks). Value: substantial — new features get full compiler benefits immediately.

**Option B: Phase 1 + Phase 2, treat as R&D.**

Implement the compiler and migrate Bloodline. Stop there. Use the result to inform decisions about whether Phase 3 is worth it. If Bloodline migration takes 3 months and reveals 40 compiler bugs, the evidence is in. If it takes 2 weeks and works cleanly, Phase 3 looks more viable. The Phase 2 result is a decision gate, not a commitment to proceed.

**Option C: Phase 1 + Phase 2 + selective Phase 3 (new subsystems only).**

Migrate Bloodline (Phase 2), then write all future Heritage subsystems in Clause. Don't migrate the 43 existing subsystems. The existing YAML content becomes "legacy, maintained but not migrated." Over time, if a legacy subsystem needs significant rework, rewrite it in Clause at that point. The migration happens organically rather than as a forced march.

**Option D: Skip Phase 4 (megapatch migration) entirely.**

This is the most significant scope reduction. The megapatch corpus is 4,294 files of cross-mod conflict resolution authored against 448 external mods that change without notice. The 688 stale patches are an ongoing maintenance problem that Clause migration doesn't fix — it just adds another layer of complexity to each fix. A better investment might be: keep the YAML format for megapatch patches, invest Phase 4 effort in stale-patch remediation and better automation tooling, and use Clause exclusively for Heritage. The megapatch crate is a different authoring task (resolving external conflicts) from Heritage (authoring first-party content). These may not benefit equally from the same language.

---

## 5. Adoption Dynamics

**What drives adoption (for languages in Clause's position):**

- **The compiler solves a problem you feel every day.** CWTools already provides some validation, but scope mismatches still only surface at runtime. Every time a Heritage developer spends 2 hours debugging a "loading screen hang" that turned out to be a scope mismatch, that's an advertisement for Clause. The compiler needs to visibly catch something painful before developers will trust it with migration work.
- **The first migration is fast and clean.** If Bloodline migration takes 2 days and produces output that works perfectly the first time, Phase 3 enthusiasm is high. If it takes 3 weeks and requires 12 compiler bug fixes, Phase 3 seems daunting.
- **Tooling matches the existing workflow.** The `clause explain` inspector, the `clause check` fast-feedback mode, and source-map comments are good. The IDE/LSP (M12) is deferred; that's fine for Phase 1 but starts to hurt in Phase 3 when the migrated corpus is large enough that navigation by file becomes unwieldy.
- **Error messages catch the right thing.** Elm-inspired error messages work when the error is genuinely what the compiler says. If the compiler says "I found a scope mismatch" when the real problem is a missing `extern` declaration for a third-party mod, the developer's trust erodes quickly.

**What kills it:**

- **Half-migrated state that lasts more than a few months.** If Heritage is 40% Clause and 60% YAML for six months, both maintainers and any new collaborators will have a "which version do I touch?" question for every task. The answer "check the status" is friction that compounds.
- **A compiler bug that produces silent incorrect output.** Type errors that prevent compilation are safe. A bug that compiles successfully but emits wrong Clausewitz, causing in-game bugs that take hours to trace back to the compiler, is trust-destroying. The coverage requirements for Phase 2 need to be extremely high on this axis.
- **Stellaris updating and breaking the vanilla spec.** The `stellaris-vanilla-spec` crate's update story (§25.4) requires re-parsing trigger_docs and running the validation pipeline after every major Stellaris patch. If a Stellaris update breaks the spec and the spec update lags behind, the compiler produces wrong output for anyone on the new version. This is the same problem that makes roblox-ts difficult to maintain — game updates are outside your control.
- **The YAML format being "good enough."** Every week the project runs smoothly on YAML patches is evidence that the migration isn't urgent. Motivation to do painful migration work is highest when the current system is visibly breaking. With 688 stale patches, that pressure exists, but the stale patches don't block the build — they just mean some features aren't landing. That's survivable.

---

## 6. Recommended Go/No-Go Triggers

### Phase 1 → Phase 2 gate

Go if:
- The compiler correctly type-checks a test file with at least: one struct with generics, one trait impl with kind inference, one `#[patch]` with `super.foo()`, one mixed-body that produces an ambiguous-kind error.
- The `EmitPlan` stream integrates with the megapatch backend and the backend's existing passes (conflict resolution, lint) run over it without errors.
- A regression test exists that builds Heritage from YAML, loads in Stellaris, and records a baseline.

No-go if:
- The Phase 1 implementation took more than 2x the estimated time (suggests the compiler complexity is being systematically underestimated).
- The type checker handles only the examples in the design doc and fails on constructs that appear in the actual Heritage corpus.

### Phase 2 → Phase 3 gate

Go if:
- Bloodline migration completes in under 6 weeks from Phase 1 acceptance.
- The compiled Heritage (with Clause Bloodline + YAML everything else) loads in Stellaris and Bloodline functions identically in a play session.
- No compiler bugs were discovered that required design-level changes (i.e., the Phase 1 architecture held up under real content).
- A count of Heritage constructs that would require `#[prefer]` annotations or manual disambiguation has been done and is manageable (<5% of items).

No-go if:
- Bloodline migration revealed that `replace_in_item`-style patterns appear in Heritage and have no Clause equivalent (this is a Phase 4 design blocker that needs resolution before committing to Phase 3).
- The byte-identical or diff-approved output required non-trivial manual intervention on more than 20% of emitted items.
- Any compiler bug produced valid-but-wrong Clausewitz (rather than compile error) during Phase 2.

### Phase 3 → Phase 4 gate

Go if:
- At least 5 Heritage subsystems have been migrated cleanly.
- The mixed-state (Clause + YAML coexistence) workflow has been stable for at least 4 weeks with active Heritage development happening.
- The `replace_in_item` gap has a documented resolution that has been validated on at least 10 real patches.
- Time-to-migrate per subsystem has been measured and is consistent with Phase 4 estimates.

No-go if:
- Phase 3 has stalled: fewer than 2 subsystems migrated after 3 months of active work.
- Feature development on un-migrated subsystems has required YAML fixes that create divergence between the "what Clause understands" and "what YAML actually does" versions.
- The compiler required design-level changes (not just bug fixes) to handle real Heritage content.

### Phase 4 → Phase 5 gate

Go if:
- All of Phase 4 is actually complete (not "mostly done, a few hundred patches left").
- The YAML loader removal has been tested in a branch build with all patches in Clause.
- Community interest in Clause has been assessed (is there anyone besides the project owner who would use a published version?).

No-go if:
- More than 100 patches remain in YAML for any reason.
- The `stellaris-vanilla-spec` crate is more than one Stellaris major version behind.

---

## 7. Governance and Sustainability

If Clause lives for 10 years, the questions that arise are not technical — they're about who makes decisions when circumstances change.

**The single-owner problem.** Right now Clause is implicitly owned by the project's owner. Design decisions happen in conversation. The design doc says "community publication as Phase 5"; but community publication without governance infrastructure means either (a) the original owner reviews all PRs forever (doesn't scale), or (b) anyone can merge anything (breaks backwards compatibility). Languages need governance before they have users, not after.

**Breaking changes.** Clause is currently at "design complete, implementation starting." It has no version history and no users. But as soon as Heritage is migrated and relies on specific Clause semantics, any compiler change that changes behavior is a breaking change. The manifest system handles save compatibility for players, but there's no equivalent for "compiler changed how it names things and now all my emitted symbols are different." The `clause_edition = "2026"` field in `mod.toml` gestures at this but doesn't specify what the upgrade path looks like.

**Stellaris dependency.** Clause's `stellaris-vanilla-spec` is versioned against Stellaris releases. Stellaris 4.0 → 5.0 could rename scopes, remove triggers, or restructure the scope graph. Every major Stellaris update requires a `stellaris-vanilla-spec` update before the compiler is usable on the new version. If that update takes 2 weeks, the project is blocked on the new version for 2 weeks. If it takes 2 months (a large restructure), that's a significant disruption. There needs to be a fast-path for "Stellaris just patched, validate the spec quickly" that doesn't require the full ingest pipeline.

**The self-hosting question.** §26.1 notes "Python codebase, eventually self-hosted in Clause." Self-hosting is a milestone that legitimizes a language but also creates a circular dependency: improving the compiler requires the compiler to compile itself. This is a Phase 5+ concern, but the decision to self-host should be made intentionally, not by drift. If the compiler stays in Python indefinitely, that's fine — but it should be a choice.

**Long-term sustainability** for a 10-year language requires:
- A versioned specification (not just a design doc) so that behavior can be tested against a reference.
- A conformance test suite that any new compiler implementation must pass.
- A changelog discipline that distinguishes bug fixes from behavior changes.
- At least one other person who can review the compiler for correctness, not just features.

None of these exist yet, and all of them are reasonable to defer to post-Phase-2. But they should be on the roadmap.

---

## 8. Failure Modes

The five most likely failure modes, with recovery plans:

**FM1: Perpetual Phase 3 half-migration (most likely).**

Heritage reaches 40–60% Clause migration, the compiler is "good enough for what's migrated," and forward velocity on new features (which the project owner cares more about) takes priority over completing migration of legacy subsystems. The mixed state becomes permanent.

Recovery: Before starting Phase 3, define a hard stop date. If migration isn't complete by then, either (a) revert all Clause content back to YAML (use the byte-identical baseline from Phase 2), or (b) declare the mixed state permanent and document which subsystems are Clause and which are YAML. Option (b) is survivable if the coexistence story is stable; option (a) is the real recovery.

What makes this failure mode durable is that it's not obviously bad. The migrated subsystems work. The unmigrated ones work. Nothing is broken. The failure is opportunity cost — the type safety benefits don't apply to the unmigrated 60%, and the compiler must be maintained to support both formats forever.

**FM2: Compiler bug causing valid-but-wrong Clausewitz (highest trust risk).**

The compiler emits Clausewitz that is syntactically valid but semantically wrong (wrong variable name, wrong scope, missing side effect). The bug is subtle and only surfaces in-game after an hour of play. The author cannot distinguish whether the bug is in their Clause code or in the compiler.

Recovery: This is why the regression test suite (against a known-good Clausewitz baseline) is non-negotiable. Every compiler change must be validated against the full Heritage build's output, not just "does it compile." Add property-based tests that validate semantic properties of emitted Clausewitz (variable references are defined before they're used, scope transitions are valid, etc.).

**FM3: Stellaris major version breaks the vanilla spec.**

Stellaris 5.0 restructures how scripted effects are declared, or renames the scope graph. The `stellaris-vanilla-spec` crate needs a significant rewrite. During the update window, the compiler either refuses to build (because the spec is out of date) or builds with wrong types (if the old spec is used).

Recovery: Design the spec crate to be "last known good" by default — the compiler uses the most recent spec that was validated against the current Stellaris version, not the latest spec. On Stellaris update, the old spec remains valid until the new one is confirmed. Add CI that checks the spec against a freshly-dumped trigger_docs on every Stellaris patch.

**FM4: `replace_in_item` has no Clause equivalent for a large class of patches.**

Phase 4 begins and the project discovers that 400+ patches use `replace_in_item` patterns (nested field surgery) that require either full-override rewrites or a new language feature. Neither option is fast. The Phase 4 estimate expands from "2–4 months" to "1–2 years."

Recovery: This should be discovered during Phase 3, when Heritage's own patches are migrated. If Heritage has no `replace_in_item` patches (it currently uses only `insert_item`), then Phase 4 faces this problem fresh. The pre-Phase-4 gate must include "audit the 581 replace_in_item files and categorize the translation difficulty." If more than 200 are non-trivial, add a `#[patch_replace(find = "...", replace = "...")]` escape hatch before starting Phase 4 in earnest.

**FM5: Contributor burnout on Phase 4.**

Phase 4 is 4,294 files. Even at 5 minutes per file, that's 357 hours. At 10–15 hours per week, that's 6–9 months of Phase 4 alone. The work is repetitive, low-creativity, high-attention (can't zone out — each patch requires reading source mods). Burnout risk is high.

Recovery: Automate as much of the mechanical translation as possible before starting Phase 4 manually. A `megapatch migrate-to-clause <patch_file>` command that translates the unambiguous cases (prefer_mod → `..super` spread, no_override → no-op, clean override → full replacement) could handle 30–50% of files automatically. The non-trivial cases (replace_in_item, inject_field with positioning) require human review but are isolated. Build the automation first; estimate what percentage it handles; then make a data-driven decision about Phase 4 scope.

---

## 9. Incentives

**Why would anyone (including future-self) migrate a Heritage subsystem rather than keep authoring in YAML?**

The design's value proposition is real: scope mismatch errors at compile time (not loading-screen hang), refactoring safety, abstraction (write `self.bloodline.add_member()` not `heritage_create_bloodline = { ... }`). These benefits are genuine. But they're only felt while writing new code in Clause. The benefits of migrating existing, working code are:

- The existing code gets type-checked going forward (catches bugs on modification).
- Refactoring existing code becomes safe.
- The code becomes more readable.

These are real but diffuse. They don't create a sharp incentive to do the migration today rather than next month.

**What tooling/features would make Phase 3 happen?**

Before Phase 2 acceptance, Clause needs:

1. `clause check` that is fast (< 5 seconds for Heritage) and runs on commit. Every YAML bug hunt that takes 30 minutes and would have been caught by `clause check` in 3 seconds is a convert.
2. A `clause migrate` command that takes a YAML patch file and outputs a `.cw` stub with the unambiguous parts translated and TODOs for the ambiguous parts. Reduces the activation energy per subsystem from "learn Clause well enough to write it from scratch" to "review and complete a generated stub."
3. Source-map comments that are actually useful in a specific debugging scenario. The first time a Heritage developer finds a bug in emitted Clausewitz, traces it back to the `.cw` source in under 2 minutes using the source map, and would have spent 45 minutes tracing it in raw YAML — that story spreads.
4. At least one non-trivial example (Bloodline) where the Clause version is visibly more readable than the YAML version. The Phase 2 subsystem choice matters for adoption: Bloodline's `heritage_create_bloodline` with its 100-line if/else_if chain is exactly the kind of code that Clause's `Array<T, N>` makes dramatically shorter. Show that diff prominently.

---

## 10. Risk-Weighted Recommendation

**Commit to Phase 1 and Phase 2. Treat Phase 2 outcome as a binary decision gate for Phase 3.**

Do not commit to all five phases now. The design is sound; the migration risk is in execution, not design. The right strategy is to prove the hard architectural bets (Phase 1) and validate the migration workflow on real Heritage content (Phase 2) before committing to the much larger Phase 3 and Phase 4.

**The single most important precondition for Phase 1 success:**

Define the Phase 1 acceptance gate such that it forces the hard parts to be real, not deferred. Specifically: the hello-world test file must contain a `#[patch]` impl with `super.foo()` that produces correct output when consumed by the megapatch backend. This is the most novel mechanism in the design and the most likely source of architectural revision. Discovering it doesn't work in Phase 1 (4–8 weeks) is much cheaper than discovering it in Phase 3 (6+ months in).

**On Phase 4 specifically:**

Given the `replace_in_item` design gap and the 581 affected files, Phase 4 as designed is likely to take 3–5x longer than estimated and requires a design decision that hasn't been made. Before committing to Phase 4, add to the Phase 3 acceptance gate: "100 megapatch patches of each surgery type have been translated to Clause and the time per patch has been measured. The `replace_in_item` translation strategy is documented and validated."

**On community adoption:**

Clause doesn't need third-party adoption to be valuable. The Heritage + Megapatch use case is self-justifying if the migration succeeds. External adoption is upside, not a requirement. Do not let the "community publication" goal in Phase 5 influence Phase 1–4 priorities. Build what's needed for the first-party use case; generalize only if the first-party use case is solid.

**Honest assessment:** The most likely outcome is that Phases 1 and 2 succeed, Phase 3 reaches ~50–70% completion before stalling, and Phase 4 gets started but never fully completes. That outcome is still net positive over "never started": the Clause-authored subsystems benefit from type safety, the migration tooling exists, and future development can use Clause from the start. The goal should be to make that partial outcome as valuable as possible — by ensuring the mixed YAML/Clause coexistence is a stable, supported state rather than an accidental failure mode.
