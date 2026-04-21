# Megapatch Lint Catalog

Living catalog of every Stellaris/Clausewitz scripting rule we want enforced as a
build-blocking lint. Each rule has a stable `rule_id`, a default severity, a
short description, and a sketch of detection. Implementation lives in
`tools/megapatch/megapatch_compiler/linters/builtin/`.

**Severity ladder** (matches `Severity` enum):
- `ERROR` — the game will reject this, crash, or silently corrupt save state.
- `WARNING` — likely correctness issue or deprecated; game tolerates but result is wrong.
- `INFO` — code smell or non-blocking advisory.

**Ownership** is decided per-finding by the OwnershipResolver. First-party
findings always block release builds; external findings can be allowlisted in
`tests/baselines/external_findings.jsonl`.

**cwtools coverage** — for each rule, "cwtools? yes/no" indicates whether
`.cwtools/config/*.cwt` already enforces it. `.cwtools/config/triggers.cwt`
defines 837 vanilla triggers; `effects.cwt` defines 731 vanilla effects;
`scopes.cwt` defines the scope graph; `scope_changes.cwt` defines valid scope
transitions. Where cwtools handles a rule cleanly, we DELEGATE rather than
re-implement. Where cwtools is unavailable (CI without dotnet), our linters
should at least catch the high-severity cases.

---

## Group 1 — Structural

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `BRACE_UNBALANCED` | ERROR | Compiled item has unmatched `{` / `}`. Already enforced inline by `clausewitz/balance.py` during compile; surface here as a lint too. | yes |
| `TRIGGER_DEPTH_EXCEEDED` | ERROR | Trigger block nested deeper than 5 levels (loading-screen hang). Existing `TriggerDepthExceeded` in `content_syntax.py`. Extend to follow scripted_trigger inlining. | partial |
| `EFFECT_DEPTH_EXCEEDED` | ERROR | Same as above for effect blocks. New sibling lint. | partial |
| `SCRIPTED_X_DEPTH_INLINED` | ERROR | A scripted_trigger/effect, when inlined at its call site, pushes the call site over the depth limit. | no |
| `MIXED_TABS_AND_SPACES` | INFO | Mixed indentation breaks downstream tooling; flag in first-party files. | no |

## Group 2 — Cross-reference (require Vocabulary loader, task #250)

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `UNDEFINED_SCRIPTED_TRIGGER` | ERROR/WARN | Reference to scripted_trigger that doesn't exist in vocabulary. Existing class deferred per prior session; ship in #251. | yes |
| `UNDEFINED_SCRIPTED_EFFECT` | ERROR/WARN | Same for scripted_effects. | yes |
| `UNDEFINED_TRAIT` | ERROR | `has_trait = X` / `add_trait = X` where X isn't in `common/traits/`. | yes |
| `UNDEFINED_ETHIC` | ERROR | `has_ethic = X` / `shift_ethic = X` references unknown ethic. | yes |
| `UNDEFINED_CIVIC` | ERROR | `has_civic = X` references unknown civic. | yes |
| `UNDEFINED_ORIGIN` | ERROR | `has_origin = X` references unknown origin. | yes |
| `UNDEFINED_BUILDING` | ERROR | Unknown `has_building = X` or `build_building = X`. | yes |
| `UNDEFINED_DISTRICT` | ERROR | Unknown district reference. | yes |
| `UNDEFINED_TECHNOLOGY` | ERROR | `has_technology = X` / `give_technology = { tech = X }`. | yes |
| `UNDEFINED_EVENT` | ERROR | `country_event = X.N` (call-site) where event id not registered anywhere. | yes |
| `UNDEFINED_ON_ACTION` | WARNING | `on_<action_name> = { events = ... }` where on_action isn't a known Stellaris hook (changes per version). | partial |
| `UNDEFINED_LOCALISATION_KEY` | WARNING | Quoted string in known text-position field (`name`, `desc`, `fail_text`, `custom_tooltip`) not present in any `localisation/**/*.yml`. | yes |
| `UNDEFINED_MODIFIER` | WARNING | `add_modifier = { modifier = X }` references unknown static_modifier. | yes |

## Group 3 — Type / value

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `INVALID_SCOPE_TRANSITION` | WARNING | Scope change (e.g., `every_owned_pop = { ... }` inside Country scope) is invalid per `scope_changes.cwt`. | yes (delegate) |
| `NUMERIC_OUT_OF_RANGE` | WARNING | Field with documented range (e.g., `factor` accepts 0..N) gets out-of-range value. | partial |
| `INVALID_AI_WEIGHT_SHAPE` | WARNING | `ai_weight = { ... }` block missing required `factor` or with malformed `modifier` entries. | yes |
| `MISSING_REQUIRED_FIELD` | ERROR | Item type whose required field is absent (e.g., trait without `cost`). | yes |

## Group 4 — Lifecycle (variables + symbols)

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `VARIABLE_USE_BEFORE_DEFINITION` | WARNING | `@var` referenced earlier in file than its `@var = X` declaration. | no |
| `CIRCULAR_VARIABLE` | ERROR | `@a = @b` while `@b = @a` (direct or transitive). | no |
| `UNUSED_VARIABLE` | INFO | `@var` defined in a file and never referenced. | no |
| `UNUSED_FIRST_PARTY_SCRIPTED_TRIGGER` | INFO | First-party scripted_trigger never called anywhere in the compiled tree. Existing `cross_references.py`. | no |
| `UNUSED_FIRST_PARTY_SCRIPTED_EFFECT` | INFO | Same for scripted_effects. Existing. | no |

## Group 5 — Deprecated syntax

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `DEPRECATED_IS_MEGACORP` | WARNING | `is_megacorp = yes` → use `has_authority = auth_corporate`. | partial |
| `DEPRECATED_RANDOM_POP` | WARNING | `random_pop = { ... }` semantics changed; use `random_owned_pop`. | partial |
| `DEPRECATED_SET_POP_FACTION` | WARNING | Use `set_pop_flag` instead. | no |
| `DEPRECATED_OLD_JOB_PRIORITY_SYNTAX` | WARNING | Format pinned to current Stellaris version. | no |
| (extensible — add rows per version) | | | |

## Group 6 — Scope & flow smells

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `DEEP_SCOPE_CHAIN` | WARNING | `prev.prev.prev.prev` or longer chain (default threshold 4). Almost always a sign the script lives in the wrong scope. | no |
| `PREVPREV_IN_ACTIVE_CODE` | INFO | `prevprev` keyword usage outside debug/temp code. Existing. | no |

## Group 7 — Patch shape (PatchLinter scope, syntax-coupled)

These run on patch input pre-compile and are **coupled to patch syntax**. They
will be rewritten when the patch DSL changes (#263). Existing rules in
`patch_shape.py`:

| rule_id | severity | description |
|---|---|---|
| `PREFER_MOD_MISSING_MOD_ID` | ERROR | `prefer_mod` patch without a `mod` field. |
| `PREFER_MOD_WITHOUT_FORMATTING_RATIONALE` | ERROR | `prefer_mod` patch whose rationale doesn't signal a formatting-only collapse (e.g. `byte-identical`, `formatting-only`, `identical hash`). Substantive winner-picks should be real consolidations per Tenet 7. |
| `REPLACE_IN_ITEM_EMPTY_FIND` | ERROR | `replace_in_item` with empty `find`. |
| `OVERRIDE_MISSING_CONTENT` | ERROR | `override` patch with no content. |
| `INSERT_ITEM_MISSING_CONTENT` | ERROR | `insert_item` patch with no content. |
| `INJECT_FIELD_MISSING_ARGS` | ERROR | `inject_field` patch missing `field` or `content`. |
| `PATCH_UNKNOWN_DIRECTORY` | WARNING | Patch targets a directory not in `rules-fios-lios.txt`. |
| `PATCH_REFERENCES_UNKNOWN_MOD` | WARNING | `mod` field doesn't match any modlist entry. |

## Group 8 — Performance / hygiene (lower priority)

| rule_id | severity | description | cwtools? |
|---|---|---|---|
| `REDUNDANT_NESTED_AND` | INFO | `AND = { AND = { ... } }` — flatten. | no |
| `EMPTY_LIMIT_BLOCK` | INFO | `limit = {}` is a no-op; usually a bug. | no |
| `MULTIPLE_ALWAYS_NO_TRIGGERS` | WARNING | An `always = no` short-circuits everything below it; lint hint. | no |
| `MAGIC_NUMBER` | INFO | Numeric literal outside common patterns (0/1/100) where an @variable would be clearer. | no |

---

## Implementation order (mapped to tasks)

1. **#249** (this doc) — done.
2. **#259** test harness — share fixture across every per-lint test.
3. **#250** vocabulary loader — symbol table cached on LintContext.
4. **#258** depth lints — extend existing.
5. **#251–#254** cross-reference lints — Group 2 in parallel; each is a thin
   walker over compiled output, looking up against Vocabulary.
6. **#255** variable lifecycle — single per-file pass.
7. **#256** scope chain — regex-based.
8. **#257** deprecated syntax — table-driven.
9. **#260** baseline regen + triage — final cleanup pass once everything ships.

## Adding a new rule

1. Pick the next free `rule_id` in this catalog (consistent prefix per group).
2. Add a row to the appropriate group table.
3. Implement in `linters/builtin/<group>.py`. One file per group keeps related
   rules together and minimizes import churn.
4. Add tests via the harness.
5. Update `tests/baselines/external_findings.jsonl` if external mods trigger
   the new rule.
6. Append to `docs/megapatch/LINTING.md` ("How to add a lint" section).
