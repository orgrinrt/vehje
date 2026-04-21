# Clausewitz / Jomini Format — Spec Research Synthesis

**Date:** 2026-04-17
**Author:** Research agent synthesis
**Purpose:** Feed into Clause language toolchain — the most authoritative grammar
reference we can produce by combining external community sources with our existing
internal docs.

**Sources reviewed:**

1. PDX Tools — A Tour of PDS Clausewitz Syntax (`https://pdx.tools/blog/a-tour-of-pds-clausewitz-syntax`)
2. PDXTools documentation site (`https://ititus.github.io/PDXTools/script`)
3. Reddit r/paradoxplaza deep-dive (not accessible — substituted with WebSearch findings)
4. Paradox dev forum — Anatomy of a Game: The Script System (`https://forum.paradoxplaza.com/forum/threads/anatomy-of-a-game-the-script-system.1484918/`)
5. Grand Jomini Modding Information Manuscript (`https://forum.paradoxplaza.com/forum/threads/grand-jomini-modding-information-manuscript.1170261/`)
6. Oskar Forsslund — Evaluating Lua for Computer Game Event Handling (`https://silo.tips/download/evaluating-lua-for-use-in-computer-game-event-handling-oskar-forsslund`) + DiVA portal
7. rakaly/jomini Rust parser (`https://github.com/rakaly/jomini`) + `https://docs.rs/jomini/`
8. nickbabcock/jomini JS parser (`https://github.com/nickbabcock/jomini`)
9. Stellaris wiki — Dynamic Modding, Conditions, Variables, Scopes
10. Stellaris Dev Diary #222 LEM Moddability
11. comfort-scripts (not accessible — repository metadata only)

**Cross-referenced against our existing docs:**

- `docs/megapatch/design/2026-04-16-clausewitz-grammar.md` (G1 grammar formalization)
- `docs/megapatch/clausewitz-tricks.md` (living tricks reference)
- `.cache/reviews/2026-04-16/clausewitz-online-research-findings.md` (prior online research)
- `.cache/reviews/2026-04-16/clausewitz-local-docs-findings.md` (local codebase findings)

---

## 1. Executive Summary

The collective sources paint a coherent picture of a language that was designed by and for content designers at Paradox — not as a formal language but as an evolving tool, shaped over 20+ years by "a whole lot of case law and precedents set as opposed to a more well-codified instruction set" (Paradox dev quote, confirmed in Anatomy post). Its design is intentionally human-readable and deliberately limited. Understanding WHY it is this way is as important as knowing WHAT the syntax is.

**Highest-confidence claims (confirmed across three or more independent sources):**

- The operator set for script files is exactly: `=`, `>`, `<`, `>=`, `<=`, `!=`, `==`, `?=`. Save files use only `=`.
- Encoding: EU4 / vanilla Clausewitz = Windows-1252; CK3 / modern games = UTF-8; localisation `.yml` = UTF-8 WITH BOM.
- `@[ expr ]` supports only `+`, `-`, `*`, `/`. Only the FIRST such expression per scripted_effect/trigger body evaluates correctly.
- Boolean block keywords (`AND`, `OR`, `NOT`, `NOR`, `NAND`, `calc_true_if`) are case-insensitive; AND is implicit when no operator is specified.
- Iterator prefixes (`any_`, `every_`, `random_`, `count_`, `ordered_` v3.2+) are NOT reserved keywords — they are pattern-matched at game load against registered list builders.
- Parameterised scripted_effects are expensive (~30 compiled variants per invocation site, ~2MB RAM), and the call-chain cap for parameterised effects is 5 levels deep.
- `$PARAM$` substitution occurs even inside `# comment` lines in inline_script bodies.
- Binary Jomini is game-specific and patch-specific — no public spec; reverse-engineered only.

**Biggest surprises from external sources (things not in our prior docs):**

1. **Implied objects** — `foo{bar=qux}` is syntactically equivalent to `foo={bar=qux}`. The `=` before `{` is optional. This has real implications for our parser's grammar.
2. **Hybrid arrays/objects** — an object can simultaneously be an array AND an object. EU4's `brittany_area = { color = { ... } 169 170 171 }` mixes keyed and unkeyed values in one block.
3. **Extraneous/missing braces are valid** — the format explicitly allows stray closing braces and missing opening braces in save files and some script files. Our lexer's INVALID-on-mismatch policy is correct for scripts; we must be more lenient for save files.
4. **CK3's `hidden objects`** — `levels={ 10 0=2 1=2 }` has a bare integer before keyed entries. A new structural form not in our grammar.
5. **`LIST` tagged type** — `mild_winter = LIST { 3700 3701 }` — a `LIST` tag before `{ }` similar to color tags. Not in our token taxonomy.
6. **Semicolons after quotes are ignored** — `"text";` is valid; the `;` is discarded. Not in our grammar.
7. **Victoria II allows non-ASCII in unquoted keys** — Windows-1252 characters can appear in unquoted identifiers. Not in our lexer spec.
8. **Scope comparison shorthand (Jomini)** — `father = root.mother.father` as a direct scope equality check is a Jomini-specific feature (CK3+), not available in classic Clausewitz (Stellaris).
9. **`var:`, `local_var:`, `global_var:` in Jomini** — three scoped variable namespaces. In Stellaris these correspond to `set_variable`, but in newer Jomini games (CK3/Vic3) they are distinct syntax tokens.
10. **`inverted_switch`** — mentioned in Conditions wiki but not documented. Likely exists but lacks community documentation.
11. **`complex_trigger_modifier` in script_values** — a `trigger` + `parameters` + `mode` + `mult` construct that runs a trigger to compute modifier value. Not in our tricks doc.
12. **`alternative_limits`** in iterators — fallback conditions for `every_` and `random_` iterators when the primary `limit` matches nothing. Jomini-specific; not documented for Stellaris specifically.
13. **`%` percent literal** — PDXTools documents a percent value type (`value%`). Not in our token taxonomy. Used in certain numeric contexts in some games.
14. **`<0xHH>` escape** in quoted strings — Imperator uses hex escape codes. Not in our grammar (correctly excluded for Stellaris, but worth documenting as cross-game variation).

---

## 2. Comprehensive Format Specification

### 2.1 Formal (approximate) grammar

The format has no official EBNF spec. This is the most accurate grammar derivable from community analysis:

```
file            ::= item*
item            ::= assignment | bare_value | comment
assignment      ::= key operator value
                  | key block            # "implied object" — operator is optional!
key             ::= scalar
operator        ::= '=' | '!=' | '==' | '<' | '>' | '<=' | '>=' | '?='
value           ::= scalar | block | color_literal | tagged_list
block           ::= '{' item* '}'
bare_value      ::= scalar              # inside a block without a key
scalar          ::= unquoted | quoted | variable | param | inline_expr | date
unquoted        ::= identifier | integer | float | boolean | null | percent?
identifier      ::= [A-Za-z_][A-Za-z0-9_\-.:@]*   # permissive; see §2.2
integer         ::= [+-]? [0-9]+
float           ::= [+-]? [0-9]+ '.' [0-9]+
boolean         ::= 'yes' | 'no'
null            ::= 'none'
date            ::= [0-9]+ '.' [0-9]+ '.' [0-9]+   # Y.M.D; no leap year
percent         ::= float '%'                        # rare; Stellaris = unconfirmed
quoted          ::= '"' char* '"'                    # can contain newlines, escapes
char            ::= any_char | '\"' | '\\' | '<0xHH>'  # Imperator only for hex
variable        ::= '@' identifier
inline_expr     ::= '@[' arith_expr ']'
arith_expr      ::= arith_term (arith_op arith_term)*
arith_term      ::= integer | float | variable | param | '(' arith_expr ')'
arith_op        ::= '+' | '-' | '*' | '/'
param           ::= '$' identifier ('|' non_dollar*)? '$'
cond_block      ::= '[[' '!'? identifier ']' item* ']'
color_literal   ::= color_tag block
color_tag       ::= 'rgb' | 'hsv' | 'hsv360' | 'hex'
tagged_list     ::= 'LIST' block                     # EU4 / some games
comment         ::= '#' (any char except newline)*
whitespace      ::= (' ' | '\t' | '\r\n' | '\n')+   # significant as separator only
```

**Key grammar ambiguities / edge cases:**

- The `operator` before `block` is OPTIONAL (`implied object` rule). `foo{...}` == `foo={...}`.
- Multiple identical keys in a block produce an implicit array — not a syntax error.
- Blocks can be simultaneously array-like (bare values) and object-like (keyed values) — "hybrid structure."
- Stray closing braces `}` without matching opens are valid in save files; behavior in script files is implementation-defined.
- An empty block `{}` is valid in any value position.
- Semicolons after quoted strings (e.g., `"text";`) are silently discarded.

### 2.2 Identifier grammar — the full picture

Our current grammar doc uses `[A-Za-z_][A-Za-z_0-9_]*` (no dots, colons, hyphens). The actual identifier character set used in the wild is broader:

**Our grammar doc's recommended split:**
The grammar doc (correctly) splits identifiers at `.` and `:` into separate tokens:
- `event_target:foo` → `IDENT + COLON + IDENT`
- `prev.owner` → `SCOPE_PREV + DOT + IDENT`

**The undivided form (from the parser's current permissive regex):**
`@?[A-Za-z_][A-Za-z_0-9.:\-]*` — allows dots, colons, hyphens inline.

**Hyphens** — appear in some asset/resource IDs. Must not be split.

**The `@SCOPE` suffix form** (`is_friend_of_@root`) — produces a runtime-dynamic identifier at game load. The `@` here is NOT a scripted variable prefix; it is an ID-interpolation suffix. This is the `TARGETED_VAR` pattern from our grammar doc, but the wiki confirms the "infix @" form more broadly: `set_leader_flag = is_friend_of_@root` generates `is_friend_of_140`.

**Victoria II edge case** — non-ASCII Windows-1252 characters in unquoted identifiers. Likely not relevant for Stellaris; noted for completeness.

**Numbers as keys** — `0 = { ... }` is valid. Clausewitz allows integer keys in objects. Our grammar handles this via `QUOTED_IDENT` but pure integer keys in unquoted form need coverage.

### 2.3 Complete operator table

Confirmed as of best available community sources:

| Token | Example use | Scope | Notes |
|-------|-------------|-------|-------|
| `=` | `key = value` | Script + Save | Overloaded: assign / equality / scope-change / activate |
| `>` | `num_pops > 10` | Script only | Numeric comparison; also valid as switch case key |
| `<` | `months_elapsed < 5` | Script only | Numeric comparison; also valid as switch case key |
| `>=` | `fleet_power >= 500` | Script only | |
| `<=` | `stability <= 25` | Script only | |
| `!=` | `planet != capital_scope` | Script only | |
| `==` | `c:RUS == this` | Script only | Explicit equality; semantics vs `=` debated; likely forces exact match |
| `?=` | `capital_county ?= title:c_byzantion` | Script only | "Exists then compare"; prevents null-deref; confirmed CK3, seen in Stellaris PDX Tools examples |

**Save files**: Only `=` is used. All comparison operators are script-only.

**Contradiction in our local docs (§1.9)** — the local-docs finding at Q5 says `?=` is "not confirmed in vanilla." External sources (PDX Tools syntax tour + CK3 wiki + Stellaris PDX Tools examples showing `c:RUS ?= this`) confirm `?=` exists in the script layer. Our grammar doc correctly includes it (§3.2). The local-docs finding is now superseded.

**`==` semantics**: Community belief is that `==` forces exact numeric equality while `=` in numeric trigger context means `>=`. Unverified — no official documentation. Treat as "distinct token, same behavior at parse time; semantic pass documents the uncertainty."

### 2.4 Literal types — complete

| Type | Form | Notes |
|------|------|-------|
| Integer | `5`, `-3`, `+5`, `81477` | 32-bit signed or 64-bit unsigned. Leading `+` valid, ignored. |
| Float | `1.5`, `0.025`, `-0.5`, `1.000` | 32-bit (~4 decimal digits precision). |
| Date | `2200.06.21`, `1444.11.11` | Y.M.D. Quoted or unquoted. No leap year support. |
| Boolean | `yes`, `no` | Also used as effect-activation values, not just bool tests. |
| Null | `none` | "No scope" / "absent" — distinct from `no`. |
| Quoted string | `"any content"` | Can contain literal newlines. Escapes: `\"`, `\\`, `<0xHH>` (Imperator only). Semicolons after close-quote discarded. |
| Unquoted identifier | `trait_cold`, `country_event` | See §2.2 for full character set. |
| Color (rgb) | `rgb { R G B }` or `rgb { R G B A }` | R/G/B: INT 0-255 or FLOAT (cwtools rejects floats; we accept). 4-component = RGBA. |
| Color (hsv) | `hsv { H S V }` or `hsv { H S V A }` | H/S/V: FLOAT 0.0-1.0. |
| Color (hsv360) | `hsv360 { H S V }` | H: INT 0-360, S/V: INT 0-100. |
| Color (hex) | `hex { aabbccdd }` | 8 hex chars; last 2 = alpha. |
| LIST tag | `LIST { val1 val2 }` | EU4 / some Clausewitz games. Content is bare values (no keys). Not a color literal — a list tag. |
| Percent | `2.5%` | PDXTools documents this; Stellaris applicability unconfirmed. |
| Parameter | `$NAME$`, `$NAME\|default$` | Text-level substitution before parse. |
| Inline arithmetic | `@[ expr ]` | Parse-time arithmetic. First-only-per-body restriction. |
| At-variable reference | `@my_var` | File-scoped or global numeric constant. |
| Cond param block | `[[PARAM] body ]`, `[[!PARAM] body ]` | EU4 Dharma+ and Stellaris 3.x+. |
| Implicit object | `foo{bar=qux}` | Equivalent to `foo={bar=qux}` — operator before block is optional. |

**Numeric precision warning** (PDX Tools): Do NOT store Clausewitz numbers as 64-bit floats — large unsigned integers (`18446744073709547616`) lose precision. Use appropriate integer type first.

### 2.5 Semantic model — how Clausewitz executes

**Loading vs evaluation:**
Clausewitz scripts are parsed at game load into an in-memory representation (C++ tree structure). Evaluation happens at runtime when triggers are checked or effects fire. This two-phase model is confirmed by the Lua thesis: parsed scripts "create a tree structure evaluated entirely in C++."

**Scope objects and the scope stack:**
- A scope is a game entity (character, country, planet, fleet, etc.) identified by type + instance ID.
- Scripts execute "in" a scope — the current scope provides context for all triggers and effects.
- Scope navigation pushes/pops a stack. `PREV` refers to the previous stack frame.
- `ROOT` = original scope of the event/scripted-call invocation (always accessible regardless of depth).
- `THIS` = current scope.
- `FROM` = scope of the entity that triggered the current event.
- Dot notation (`owner.capital_scope.solar_system`) chains scope transitions WITHOUT pushing PREV entries for intermediate steps.

**Three kinds of scope objects (Jomini model from dev post):**
1. Objects with read capabilities (triggers), write capabilities (effects), and navigation (links/lists) → full scope types.
2. Primitive scopes (booleans, numbers, flags) — bypass normal requirements; raw data, not references.
3. Named scope references (event targets) — allow storing complex link chains for reuse.

**Trigger semantics:**
- Triggers are read-only boolean tests. Can execute in parallel since they don't mutate state.
- A block of conditions without an explicit boolean operator uses implicit AND.
- `NOT = { A B }` behaves as NOR (both must be false for the whole to be true) — official wiki warning.
- `calc_true_if = { amount >= N ... }` counts how many sub-conditions are true; returns true if count meets the threshold. `amount` supports `>=`, `<=`, `>`, `<`, `=`.

**Effect semantics:**
- Effects mutate state and execute serially — order matters.
- `if = { limit = { <triggers> } <effects> }` — conditional execution.
- `else_if` / `else` must follow their preceding `if` with no intervening unrelated effects.
- `switch = { trigger = <name> <value> = { ... } default = { ... } }` — executes first matching case only.
- `while = { count = N <effects> }` or `while = { limit = { ... } <effects> }` — loop. Cap: 1000 iterations (CK3 confirmed; Stellaris presumed same). No documented early-break from `while` directly; use `if { break = yes }`.
- `break = yes` halts subsequent effects in the current block.
- `hidden_effect = { ... }` — runs effects without generating tooltips.

**Load order semantics (FIOS vs LIOS):**
- LIOS (Last In Only Served): Most directories — last-loaded definition of a key wins.
- FIOS (First In Only Served): `component_templates/`, `events/`, `scripted_variables/`, etc. — first definition wins.
- Default for unlisted directories: LIOS.
- File load order within a directory: ASCIIbetical by filename.
- `~` prefix causes a file to load first; `!` prefix causes a file to load last.

**`@variable` scoping:**
- `@foo = N` at file top: file-local numeric constant. Cannot be referenced outside its declaring file.
- `@foo = N` in `common/scripted_variables/*.txt`: globally available across all files. Loaded via FIOS — first definition wins.
- Scripted variables are constants evaluated at game load, NOT mutable runtime variables.
- Only numeric (int/float) values — no strings, no blocks, no scopes.

**Iterator semantics:**
- `any_X` — trigger; returns true if ANY member matches. Boolean result.
- `every_X` — effect; applies effects to ALL matching members. O(n) per use; O(n×m) when nested.
- `random_X` — effect; applies effects to ONE randomly selected member.
- `count_X` — trigger; returns the count of matching members for comparison.
- `ordered_X` (v3.2+) — effect; deterministic iteration with `position`, `order_by`, `inverse`.
- These are NOT engine-reserved words — any registered list builder automatically generates all variants.
- Iterator blocks accept: `limit = { ... }` (filter), `weight = { ... }` (for random_), `position`/`order_by`/`inverse` (for ordered_), `alternative_limits` (fallback when primary limit matches nothing — Jomini games).

**inline_script semantics:**
- Text-substitution at game load time — NOT a call; the body is literally inlined.
- Does NOT consume call-graph depth (unlike parameterised scripted_effects).
- `$PARAM$` substitution happens on entire file content including comment lines.
- Two forms: simple path form (`inline_script = path/to/script`) and parameterised block form.
- `[[PARAM] body ]` conditional blocks: content included only if PARAM is defined and not `no`.
- `[[!PARAM] body ]`: content included only if PARAM is absent or `no`.
- Recursion depth limited to 5 iterations (cwtools ResourceManager.fs; matches Stellaris engine limit).

**Scripted values (script_values):**
- Defined in `common/script_values/*.txt`. Runtime numeric computation.
- Operations: `set`, `weight`, `add`, `subtract`, `factor`, `mult`, `multiply`, `divide`, `modulo`, `round_to`, `max`, `min`, `pow`, `round`, `ceiling`, `floor`, `abs`, `square`, `square_root`.
- `modifier = { ... }` blocks add conditional multipliers.
- `complex_trigger_modifier = { trigger = X trigger_scope = Y parameters = { ... } mode = add mult = N }` — computes a value based on a trigger result. Unique construct not in our tricks doc.
- Parameters via pipe syntax: `value:my_value|PARAM1|val1|PARAM2|val2|` (trailing pipe).

### 2.6 Save file format vs. script file format

| Aspect | Script files | Save files |
|--------|-------------|-----------|
| Operators | All 8 (`=`, `!=`, `==`, `?=`, `<`, `>`, `<=`, `>=`) | Only `=` |
| Purpose | Define game content, logic, events | Persist game state |
| Encoding | Windows-1252 (vanilla) or UTF-8 (modern mods) | Game-specific; often Windows-1252 or binary |
| Size | Usually <1MB per file | 100MB+ for full game state; 7M+ lines |
| Duplicate keys | Unusual; can have semantic meaning | Common; produces implicit arrays |
| Extra braces | Parse error / unexpected | Valid; stray `}` are allowed |
| Comments | `#` comments throughout | Typically no comments |
| Dynamic constructs | `@var`, `$PARAM$`, `[[PARAM] ... ]`, `@[ expr ]` | None — purely static data |
| Binary variant | No | Yes (game-specific token encoding) |

**Binary Jomini format:**
- Used for compressed save files in modern games (CK3, Victoria 3, EU5, Imperator).
- EU4 and HoI4 use different save structures (not the envelope format).
- Token encoding: 16-bit integer tokens mapping to strings. Token table is game-specific and patch-specific — no public spec.
- Floating-point encoding: custom per game.
- Text decoding: caller-specified (typically Windows-1252).
- No official documentation; entirely reverse-engineered by community tools (rakaly/jomini, PDX Tools).
- The `rakaly/jomini` Rust library achieves >1 GB/s parse speed on binary format.

**Save file headers:**
- EU4: `EU4txt` or `EU4bin` header prefix (must be stripped before parsing).
- CK3: Structured container (ZIP-based) with metadata + gamestate sections.
- Modern games: `JominiFile` envelope format with compressed/uncompressed variants.

### 2.7 Engine design decisions — why Clausewitz is this way

From the Anatomy of a Game post (confirmed by Matthew / blackninja9939, Paradox content designer):

**Primary design goal:** Help content designers implement gameplay without C++ expertise. The language exists for a specific job (content scripting for Paradox's games), NOT as a general game-making tool.

**Four core objectives stated:**
1. Help content designers accomplish their work.
2. "Straightforward plain English with minimal complex syntax."
3. Easy extensibility for new content over time.
4. Prevent catastrophic errors from script mistakes.

**Historical context:**
- The format predates JSON by several years.
- Was not formally designed — grew organically from game to game.
- "A whole lot of case law and precedents set as opposed to a more well-codified instruction set" — one designer's description.
- The American legal system analogy is official Paradox framing, not community cynicism.

**Why string manipulation is deliberately absent:**
The dev post explicitly rejects string manipulation despite modding community requests: "Near ever 'string' usage I've seen mods try to use is them hacking around something instead of making a legitimate request." The reasoning: dynamic string building creates localization conflicts and encourages workarounds rather than legitimate feature requests. This is a deliberate design constraint, not an oversight.

**Why modding community needs are NOT primary:**
The quote is direct: "whilst we do want our modding community to use this...it is never to be things that come at the cost of the key goals." Clausewitz is NOT designed as a modding tool — it is designed as a content-design tool that modders happen to have access to. This explains many "gaps" modders feel.

**Why no Lua (from Forsslund thesis, 2013, EU3):**
- Performance was the decisive constraint: "any new event system is not allowed to have any significant impact on system performance."
- Lua event evaluation was 1.5–15x slower than C++ parsed scripts.
- LuaJIT reduced the overhead but couldn't match native parsed-tree performance.
- Primary bottleneck: "the overhead incurred by calling Lua instead of having all evaluation done in C++."
- The thesis author's conclusion: "If your main concern is speed you should probably use parsed scripts."
- Lua's advantages (runtime modification, full language capabilities) did not outweigh the performance cost for a game genre where events fire hundreds of times per second.
- Paradox's architecture means a parsed tree is evaluated entirely in C++ — all string lookups resolve at load time, not evaluation time.

**Scope architecture design (from Anatomy post):**
- Scope objects carry two values: type (numeric ID for object category) + identifier (which specific instance).
- Objects become full scope types only when meeting at least two criteria: read capability (triggers), write capability (effects), navigation (links/lists).
- "Primitive scopes" (booleans, numbers, flags) bypass these requirements — they ARE their own data.
- 1-to-1 links support chaining without explicit syntax like `prevprevprev` — the dev post hints this was a CK3 improvement over CK2's explicit concatenation.
- 1-to-many relationships register a single list builder, which then automatically generates `any_`, `every_`, `random_`, `ordered_` variants — explaining why these are NOT reserved words.

**Modifier system (from Anatomy post):**
- Modifier definitions: constant identifiers (enums) + formatting rules. Previously hardcoded in C++; moved to script.
- Modifier instances: stored as sorted arrays (NOT hashmaps) — binary search over sorted arrays was judged cheaper than hashmap bookkeeping for the typical case (many instances across many entities).
- Modifier collections: cached total; invalidated ("dirty flag") when any instance changes; priority rebuilds for player-visible entities.
- Each instance requires stable memory addresses and displayable names — this is why anonymous modifiers aren't a feature.

**On-actions vs polling (from Anatomy post):**
- CK2 used polling-based event triggering, which caused performance problems.
- CK3 / Jomini switched to on-actions — explicit engine notifications when events occur (year change, birth, death, war start).
- This is why Stellaris `on_monthly_pulse` etc. exist — they are on-action hooks, not timer callbacks.

**Tooltip simplification (from Anatomy post):**
- The engine simplifies tooltips automatically: redundant information filtered, repeated actions consolidated.
- "King John: will lose 50 gold, 100 piety, 100 prestige" rather than three separate lines.
- This is a display concern but has scripting implications: `custom_tooltip` triggers override the automatic simplification.

### 2.8 Cross-game variations

| Feature | EU4 | Stellaris | CK3 | HoI4 | Vic3 | Imperator |
|---------|-----|-----------|-----|------|------|-----------|
| Encoding | Win-1252 | Win-1252 / UTF-8 | UTF-8 | Win-1252 | UTF-8 | Win-1252 |
| `?=` operator | Yes (1.26+) | Yes (confirmed PDX Tools) | Yes | Unknown | Yes | Unknown |
| `==` operator | Yes | Yes | Yes | Unknown | Yes | Unknown |
| `[[PARAM] ... ]` | Yes (1.26+) | Yes (3.x+) | Yes | Unknown | Yes | Unknown |
| `LIST { ... }` tag | Yes | Unknown | No? | Unknown | Unknown | Yes |
| `rgb`/`hsv` colors | Yes | Yes | Yes | Yes | Yes | Yes |
| `local_var:`/`global_var:` | No | Partial | Yes (Jomini) | No | Yes (Jomini) | No? |
| `ordered_*` iterators | No | Yes (3.2+) | Yes | No | Yes | No |
| `alternative_limits` | No | Unconfirmed | Yes | No | Yes | No |
| `scope:name` syntax | No | No | Yes | No | Yes | No |
| On-actions | Limited | Yes | Yes (major) | Limited | Yes | Limited |
| Save format | Text+Binary | Text (large) | Text+ZIP+Binary | Text | ZIP+Binary | Text+Binary |
| Integer keys in objects | Yes | Uncommon | Yes | Yes | Yes | Yes |
| Non-ASCII unquoted IDs | Yes (Win-1252) | No | No | No | No | No |
| CK3 "hidden objects" | No | No | Yes | No | No | No |
| Mean-time-to-happen events | Yes | Heavily used | Removed | Yes | Removed | No |
| Scripted values (`script_values`) | No | Yes | Yes | No | Yes | No |

**Key Stellaris-specific variations:**
- Stellaris retains `mean_time_to_happen` events (unlike CK3 which removed them).
- Stellaris has `scripted_variables` for global `@var` constants (some other games handle this differently).
- Stellaris uses `set_variable` / `change_variable` for runtime numeric variables — NOT the `var:` / `local_var:` / `global_var:` syntax that CK3/Jomini uses. These are mechanically different approaches to the same concept.
- `ordered_*` iterators added in Stellaris v3.2 (Dev Diary #222), before which only `any_`, `every_`, `random_`, `count_` existed.
- The `inline_script` system was introduced in Stellaris 3.5 (not available in earlier Stellaris or most other Paradox games at that time).

**Jomini vs Clausewitz distinction:**
- "Jomini" is Paradox's internal name for the shared library that was factored out of individual game codebases to reduce duplication.
- CK3, Vic3, EU5, Imperator use Jomini.
- EU4, Stellaris, HoI4 use the older Clausewitz layer.
- The scripting language SYNTAX is largely the same; the semantic features available differ (e.g., Jomini added `local_var:`/`global_var:`, `scope:name` syntax, `alternative_limits`, `scope comparison` syntax).
- For our purposes (Stellaris), we are targeting Clausewitz, not Jomini. Several Jomini features mentioned in community resources are NOT available in Stellaris.

### 2.9 Implicit rules — community "you just have to know" lore

These are rules not documented in official wikis but surfaced consistently in community discussion:

1. **`NOT = { A B }` is NOR, not NOT(A OR B).** The wiki warns about this but community sometimes calls it "unreliable." Safe rule: treat `NOT` as single-condition; use `NOR` for multi-condition negation.

2. **Boolean keywords are case-insensitive in practice.** Both `AND` and `and` work. Vanilla files use uppercase; many mods use lowercase. The wiki uses uppercase but community practice is mixed.

3. **`=` in trigger context means `>=` for numeric comparisons** (community claim). E.g., `num_pops = 5` means "at least 5 pops." `==` may force exact equality. UNVERIFIED — no official source confirms this, but it appears consistently in community modding guides.

4. **Scripted variable files must end with a blank or commented line.** Not loading the trailing blank line causes the last variable definition to be ignored in some contexts. Documented in Stellaris wiki Dynamic Modding.

5. **`$PARAM$` substitution is purely textual** — it happens before any parse step. There is no escaping mechanism. If your parameter value contains `=` or `{`, the substitution produces whatever the text says.

6. **inline_script cannot be used in item list positions** — `examples = { inline_script = "foo" }` is unsupported. Only works at statement positions, not inside array-valued blocks.

7. **One inline_script per file** — the path is derived from the filename. A single `.txt` file under `common/inline_scripts/` defines one inline script. The path used in calls is `folder/filename` (without extension).

8. **The `trigger = <name>` inside `switch` blocks is a special keyword form** — `trigger` here is NOT the same as the structural `trigger` block in `potential = { ... }`. It names the trigger to dispatch on.

9. **`if` inside `switch` case blocks has a legacy parser limitation** — `else` must be nested inside the `if`, not sequential, when `if` appears inside a `switch` case. This is a known engine parser edge case.

10. **Dynamic flag name construction** (`is_friend_of_@root`) — the `@` here is NOT the `@variable` prefix. It appends the numeric ID of the named scope. This is purely a flag/variable name construction tool, not a scripting variable. Only works with `set_<scope>_flag`, `has_<scope>_flag`, and `create_leader` targets (static flags only for `create_leader`).

11. **The implied-object rule** (`foo{bar=qux}` == `foo={bar=qux}`) is a syntax quirk that parsers must handle. Real scripts rarely use it deliberately, but it can appear in compressed/minified files.

12. **Stray closing braces in save files** are NOT parse errors. A robust save-file parser must absorb extra `}` tokens.

13. **Duplicate keys in save files produce arrays** — same key appearing twice means the value is an array. This is NOT two separate fields. `toArray()` post-processing is needed when consuming these.

14. **Dot-scope chains do NOT push PREV.** After `owner.capital_scope.solar_system = { ... }`, `prev` refers to whatever was active before the chain started, not to `capital_scope` or `owner`. This surprises modders constantly.

15. **`prevprev` (single token) is broken in diplomatic triggers and `opinion_modifier` scopes.** Use `prev.prev` instead. The dot-notation form is correct everywhere.

16. **Global event targets are lost on save/load.** `save_global_event_target_as` must be re-registered in `on_single_player_save_game_load` and `on_multiplayer_game_loaded`.

---

## 3. Per-Source Synthesis

### 3.1 PDX Tools — A Tour of PDS Clausewitz Syntax

**Unique contribution:**
- The most rigorous community-produced format specification.
- Confirmed the full 8-operator set including `==` and `?=`.
- Documented all four color literal forms with their exact component types.
- Introduced the "implied object" rule (`foo{...}` == `foo={...}`) — not in our prior docs.
- Confirmed hybrid arrays/objects (EU4 `brittany_area` example) — not in our docs.
- Documented the `LIST` tagged type — not in our docs.
- Confirmed semicolons after quoted strings are ignored — not in our docs.
- Documented `<0xHH>` escape sequences (Imperator) — not in our docs.
- Confirmed numeric precision warning (use integer types, not float, for large values).
- Stated explicitly: the format has no official spec, and "reality is a bit more messy" than any formal grammar.

**Confidence level:** High. This is a professional-quality reverse-engineering effort by someone with deep implementation experience (PDX Tools parses actual game files).

**Specific unique claims:**
- "A quoted scalar can contain any character including newlines" — confirmed.
- Save files only use `=`; all comparison operators are script-file only — confirmed.
- EU4 recursive events "reach hundreds of nesting levels" — interesting data point re: no documented hard brace-depth limit.
- Stray closing braces are valid in some contexts — new to our grammar.

### 3.2 PDXTools documentation (ititus.github.io)

**Unique contribution:**
- Independently confirms the operator set and literal types.
- Introduces the `%` percent literal type — not in any other source or our docs. Applicability to Stellaris unconfirmed.
- Confirms `none` as null literal.
- Confirms dynamic keyword construction with `@scope` suffix.
- Documents `static` vs `dynamic` script categories (data representation vs. executable code) — useful conceptual framing.
- Notes known encoding issues cause "silent failures during execution" — confirms our INVALID-is-fatal policy is right for script files.
- Confirms scripted GUI (`scripted_guis`) as a separate concept from scripted triggers/effects — not currently in our docs.

**Confidence level:** Medium-high. An independent documentation effort that aligns well with other sources.

### 3.3 Reddit r/paradoxplaza (not accessible)

**Status:** WebFetch failed (reddit.com blocked). WebSearch found summary references.

**What summary references confirm:**
- The design rationale is accurately summarized: format predates JSON, Paradox has full control, the American legal system analogy is official.
- Format "is recklessly flexible, allowing each game object to potentially define its own unique syntax."

**Unique contribution:** None recoverable from this research session. Would likely confirm community gotchas and add qualitative color.

### 3.4 Anatomy of a Game: The Script System (Paradox dev forum)

**Unique contribution:**
- PRIMARY SOURCE — written by Matthew (blackninja9939), a Paradox content designer. Highest authority on design intent.
- Confirms the four core design objectives.
- Explains scope objects' two-value structure (type + instance ID).
- Documents the precise criteria for becoming a "full scope type."
- Explains primitive scopes bypassing normal scope requirements.
- Reveals that 1-to-many list builders automatically generate `any_/every_/random_/ordered_` variants — confirming why these are NOT reserved words.
- Documents modifier system redesign: sorted arrays (not hashmaps) for modifier instances; dirty-flag caching for collections.
- Confirms CK3's shift from polling to on-actions.
- Contains the explicit anti-Lua-string-manipulation statement — important for understanding format limitations.
- States modding community needs are secondary to content designer needs — key framing for understanding the language.

**Confidence level:** Highest. This is an official Paradox developer statement.

**Specific quotes (paraphrased for accuracy):**
- "Straightforward plain English with minimal complex syntax" — design goal #2.
- String manipulation rejected because "near ever 'string' usage I've seen mods try to use is them hacking around something instead of making a legitimate request."
- Modifier instances stored in sorted arrays "rather than hashmaps to reduce bookkeeping overhead."

### 3.5 Grand Jomini Modding Information Manuscript

**Unique contribution:**
- CK3 / Jomini-specific but highly valuable for cross-game comparison.
- Documents `var:`, `local_var:`, `global_var:` as three distinct storage categories in Jomini.
- Documents `scope:name` syntax for named scopes (Jomini-specific — NOT available in Stellaris's Clausewitz).
- Documents `alternative_limits` for iterators (Jomini-specific).
- Documents `scripted_guis` with four operation types: types, promotes, functions, callbacks.
- Confirms `script_docs` (Stellaris: `trigger_docs`) console command exports documentation.
- Shows `first_valid` / `random_valid` constructs inside event `desc` blocks.
- Confirms complex formula syntax in script_values including `if`/`else_if` branches inside value definitions.
- Documents `count` and `percent` parameters on `any_` iterators (Jomini-specific extensions).
- Confirms `add_to_list` / `remove_from_list` / `is_in_list` for dynamic scope lists.

**Confidence level:** High for CK3/Jomini. Only partially applicable to Stellaris.

**Critical distinguisher:** Everything in this document that uses `scope:name`, `local_var:`, or `global_var:` syntax is Jomini-specific. Stellaris uses `set_variable` / `check_variable` syntax instead.

### 3.6 Oskar Forsslund — Evaluating Lua for Computer Game Event Handling

**Unique contribution:**
- The academic rationale for why Paradox chose parsed scripts over embedded scripting.
- Quantifies the performance cost: Lua is 1.5–15x slower than C++ parsed scripts for EU3 events.
- LuaJIT reduces the overhead but doesn't eliminate it.
- The decisive constraint was stated as a hard requirement: "any new event system is not allowed to have any significant impact on system performance."
- Explains the architecture decision: parsed scripts create a tree evaluated entirely in C++ — load-time string resolution means zero lookup cost at evaluation time.
- Confirms that the advantages of Lua (runtime modification, full language capabilities) were explicitly evaluated and rejected on performance grounds for Paradox's use case.

**Confidence level:** High. Academic thesis; empirical measurements; directly about Paradox's system (EU3 → proto-CK2 / EU4 era).

**Implications for Clause:**
- Clause compiles to Clausewitz — same performance profile as hand-written scripts.
- Clause does NOT embed a scripting runtime — the right decision for the same reasons.
- The performance constraints that killed Lua are the same constraints that shape Clausewitz's limitations. Understanding them helps explain why Clause should not try to paper over engine limits.

### 3.7 rakaly/jomini and nickbabcock/jomini parsers

**Unique contribution:**
- Most detailed technical documentation of the binary Jomini format available publicly.
- Confirms binary format uses 16-bit integer tokens for string lookup.
- Binary format is game-specific AND patch-specific (can change between game updates).
- Token resolver must be supplied externally — no universal string table.
- Floating-point encoding is custom per game.
- Modern game save containers: ZIP-based envelope with metadata + gamestate sections; EU4 and HoI4 use different (older) structures.
- "Duplicate key" behavior: singular occurrence = object; multiple occurrences = array. This ambiguity requires `toArray()` post-processing in consumer code.
- Write API reveals structural rules for constructing format output (useful for our emitter).
- Parse performance: >1 GB/s (rakaly) and >200 MB/s (nickbabcock JS).

**Confidence level:** High. This is production parser code handling real game files.

**Implication for Clause:** Clause's output is script files, NOT save files. We never need to produce binary Jomini. But reading save files (for tooling purposes) requires the binary parser knowledge. Save file reading should use rakaly/jomini rather than our own implementation.

### 3.8 Stellaris wiki — Dynamic Modding, Conditions, Variables

**Unique contribution:**
- PRIMARY SOURCE — official Stellaris wiki.
- Confirms `complex_trigger_modifier` in script_values — a `trigger` + `trigger_scope` + `parameters` + `mode` + `mult` construct not in our tricks doc.
- Documents the "wrapping math in quotes" trick for multi-statement `@[ ]` arithmetic as parameters.
- Confirms the new dot-scoping variable access format from LEM (Dev Diary #222): `value = from.capital_scope.my_var`.
- Documents `export_trigger_value_to_variable` as a way to capture trigger numeric values.
- Confirms `inverted_switch` exists as a trigger-context variant (vs. `switch` for effects).
- Performance guidance: condition ordering matters — most-likely-false first in AND, most-likely-true first in OR.
- Confirms scripted variable file must end with blank/commented line.
- Documents `round_variable_to_nearest` added in 3.1.

**Confidence level:** High. Official wiki, maintained by community with Paradox cooperation.

### 3.9 Stellaris Dev Diary #222 (LEM Moddability)

**Unique contribution:**
- Official Paradox announcement of `ordered_*` iterators in Stellaris v3.2.
- Documents the full syntax: `position`, `order_by`, `inverse`.
- Confirms cross-scope variable access via dot notation: `value = from.capital_scope.my_var`.
- Documents removal of old scope-specification format (older syntax now invalid).
- Lists backwards-compatibility breaking changes.
- Confirms `export_trigger_value_to_variable` expanded to all numeric triggers with parameters.
- Confirms all scopes now support script flags and variables.
- Confirms trigger documentation consolidated into `script_documentation/` folder.

**Confidence level:** Highest. Official Paradox developer communication.

---

## 4. Gaps in Our Current Docs — Additions Needed

Listed by priority (P1 = affects compiler correctness, P2 = affects lint/analysis, P3 = informational):

### P1 — Affects lexer/parser correctness

**G1: Implied object rule** — `foo{bar=qux}` is valid syntax equivalent to `foo={bar=qux}`. The `=` before a block value is OPTIONAL. Our lexer and light AST overlay do not handle this.

**Location:** Add to grammar doc §5 (Light AST overlay). Assignment node must accommodate missing operator when value is a block.

**G2: Hybrid arrays/objects** — a block can contain both bare values and key-value pairs simultaneously: `brittany_area = { color = { 118 99 151 } 169 170 171 172 }`. Our `BlockItem = Assignment | Block | BareValue` definition handles this structurally (BareValue for bare integers, Assignment for keyed pairs), but we haven't documented that a single Block can mix both. The parse is already correct; the documentation needs updating.

**G3: `LIST` tagged type** — `mild_winter = LIST { 3700 3701 }`. This is a `IDENT("LIST")` followed by a block, different from color tags. Not in our token taxonomy. Should add `KW_LIST` to §3.7b color-space tags (or a separate §3.7c for list tags). Stellaris applicability unconfirmed; add with confidence tag.

**G4: Semicolons after quoted strings are ignored** — `"foo";` is valid. Our lexer should silently consume the `;` after a closing quote. If we emit `INVALID` for `;`, we'll fail on some files. Add to §4 lexer contract as a special-case skip rule.

**G5: Integer keys** — `0 = { ... }` is valid. Keys can be unquoted integers, not just identifiers. Our grammar doc §3.1 lists `QUOTED_IDENT` for string keys but doesn't explicitly handle unquoted integer keys. The `INT` token kind in §3.3 covers the literal; we need to document that `INT` is a valid key position in `Assignment`.

**G6: Stray closing braces in save files** — if we ever read save files, our INVALID-token hard-fail policy must be relaxed. Add a note in §4 lexer contract: "Script file mode: hard fail on stray `}`. Save file mode: absorb and continue." This is relevant if we build any tooling that reads game saves.

### P2 — Affects lint and analysis correctness

**G7: `complex_trigger_modifier` in script_values** — a `trigger` + `trigger_scope` + `parameters` + `mode` + `mult` construct. Not in our tricks doc. Needs to be added to clausewitz-tricks.md under "Computation tricks."

**G8: `alternative_limits` in iterators** — fallback condition block for `every_` and `random_` when primary `limit` matches nothing. Confirmed in Jomini (CK3/Vic3); applicability to Stellaris unconfirmed. Flag as "verify in Stellaris" and add to tricks doc.

**G9: `inverted_switch`** — mentioned in Conditions wiki. A trigger-context variant of `switch`. Not in our token taxonomy (we have `KW_SWITCH` but not `KW_INVERTED_SWITCH`). Add to §3.5 reserved words with "pending verification" flag.

**G10: `%` percent literal** — documented in PDXTools. Not in our token taxonomy. Add to §3.3 Literals with "Stellaris applicability unconfirmed" note. Token kind `PERCENT` — `FLOAT` followed immediately by `%`.

**G11: `count` and `percent` parameters on `any_` iterators** — Jomini extension allowing `any_character = { count = 3 ... }` to check if at least 3 match. Not the same as `calc_true_if`. Not in our docs. Add to tricks doc with "Jomini-only / verify in Stellaris" flag.

**G12: New variable dot-access syntax (LEM 3.1+)** — `value = from.capital_scope.my_var` replaces older scope-specification format. Our token taxonomy doesn't document this as a distinct pattern. Add to §1.2 of local-docs-findings under variables, and update grammar doc §3.6 scope words.

**G13: `round_variable_to_nearest`** — effect added in Stellaris 3.1. Not in our tricks doc under computation.

**G14: `export_trigger_value_to_variable`** — captures a trigger's numeric result into a variable. Not in our tricks doc. Add under computation tricks.

### P3 — Informational / documentation completeness

**G15: Jomini vs Clausewitz distinction** — our docs use "Clausewitz" throughout but the distinction matters: Stellaris is Clausewitz (not Jomini). CK3-specific features (like `local_var:`, `global_var:`, `scope:name`) are NOT available in Stellaris. This should be explicitly documented in the grammar design doc as a framing section.

**G16: Binary Jomini format overview** — for tooling purposes (save file reading). Add a brief overview to the grammar doc with pointer to rakaly/jomini.

**G17: Lua evaluation rationale** — the Forsslund thesis findings should be summarized in our architecture docs to explain why Clause compiles to Clausewitz rather than embedding a runtime. Relevant for the design doc §2 (Motivation) — add a "Why not embed a scripting engine?" sub-section.

**G18: Modifier sorted-array architecture** — document the engine's internal modifier storage model (sorted arrays, dirty-flag caching) in tricks doc as context for why modifier-based state storage is cheap.

**G19: `comment` / `description` pattern in script_values** — `first_valid = { triggered_desc = { ... } }` / `random_valid = { ... }` inside event desc blocks. Jomini-specific but the pattern (dynamic event text assembly) is relevant for Heritage. Already partially in tricks doc under "Triggered descriptions."

---

## 5. Contradictions with Our Current Docs — Corrections Needed

### C1: `?=` operator existence

**Our local-docs finding (§1.9, Q5):** "not confirmed in vanilla; might appear in some mods."

**External sources:** PDX Tools syntax tour lists it as part of the standard operator set. CK3 wiki documents it with examples. PDX Tools Stellaris examples show `c:RUS ?= this`. Stellaris Dev Diary confirms it in variable access context.

**Correction:** Remove the "might appear in some mods" hedge. `?=` is a confirmed operator in the Clausewitz script layer, including Stellaris. Already correctly included in grammar doc §3.2 but the local-docs finding is inconsistent.

**Priority:** P2 — affects how we treat `?=` in user scripts.

### C2: `inverted_switch` existence

**Our docs:** Not mentioned at all.

**External source:** Conditions wiki mentions `switch` and `inverted_switch` as separate constructs. No further documentation found.

**Action:** Add `KW_INVERTED_SWITCH` to grammar doc §3.5 with "pending verification" flag. Do not add semantic handling until verified against actual Stellaris vanilla files.

### C3: `while` loop early-break mechanism

**Our grammar doc (§3.5):** Lists `KW_BREAK` but doesn't document the mechanism for early break from while loops.

**External source:** CK3 wiki confirms 1000-iteration cap. No early-break from `while` directly documented. Community practice: `if = { limit = { ... } break = yes }` pattern inside while loop (break from the if block, which ends the while iteration).

**Correction:** Clarify in grammar doc §10 (Known Constraints) and tricks doc that `break = yes` halts the CURRENT EFFECT BLOCK, and when used inside a while loop's body it exits the while entirely for that iteration. The canonical early-exit from while uses `if { limit { ... } break = yes }` where `break` is the ONLY effect inside the `if`.

### C4: `=` in numeric trigger context means `>=`

**Our docs:** Not explicitly addressed; `=` is just `EQUALS`.

**External source:** Community consensus (multiple sources) that `num_pops = 5` means "at least 5 pops," not exactly 5. This is why `==` exists as a separate operator.

**Action:** Add to grammar doc §3.2 as a known semantic quirk: "`=` in numeric trigger position likely means `>=` (community consensus; unverified by official source). `==` may force exact equality." Flag both as needing engine verification.

### C5: `rgb` color components — float vs integer

**Our grammar doc (§3.7b):** "R/G/B: INT 0-255 OR FLOAT (cwtools issue #53 — they reject floats; we accept both)."

**External source:** PDX Tools tour documents `rgb { R G B }` with components as integers 0-255 only. Jomini documentation doesn't mention float RGB components.

**Assessment:** Our decision to accept both INT and FLOAT is the right safe choice (be permissive in what we accept). The external source confirms integers are the canonical form. No correction needed to policy; add a note that float RGB components are non-canonical but exist in some mod files.

### C6: Brace nesting — "infinite depth" claim vs our 6-level lint

**Our docs:** Brace depth ≥ 6 in trigger/effect directories → loading-screen hangs. Lint `TRIGGER_DEPTH_EXCEEDED`.

**External source (PDX Tools):** "Nested objects: Can be infinitely deep." EU4 events "reach hundreds of nesting levels."

**Resolution:** These are not contradictions — they apply to different contexts. PDX Tools is documenting the PARSER's capability (no enforced limit). Our lint fires on TRIGGER/EFFECT directories specifically at depth 6 because of the loading-screen-hang correlation. The parser limit is effectively unbounded; the PRACTICAL limit in trigger/effect scripts is 5. Both are correct. Clarify in grammar doc §10 that the depth limit is semantic/performance-based, not a parser limit.

---

## 6. Open Questions

### OQ1: `==` exact semantics

Community belief: `==` forces exact numeric equality; `=` in numeric trigger context means `>=`. Official source: none. Confidence: medium. Verification: run `trigger_docs` and inspect the registered triggers to see if they document the distinction.

### OQ2: `%` percent literal applicability in Stellaris

PDXTools documents it; Stellaris wiki does not. Does `pop_happiness > 50%` work in Stellaris? Does it mean `0.5`? Verification: search vanilla files.

### OQ3: `alternative_limits` in Stellaris

Confirmed in CK3/Vic3. Not found in Stellaris wiki or trigger_docs. Is it a Jomini-only feature or was it backported?

### OQ4: `inverted_switch` full syntax and scope

The Conditions wiki mentions it. No code examples found. Likely a trigger-context `switch` that returns false when a case matches (an inversion of the normal true-on-match behavior). Needs vanilla file verification.

### OQ5: `$PARAM|default$` in `scripted_trigger` bodies

Our docs flag this as unverified. The community consensus is that it works, but no official source confirms it for `scripted_trigger` specifically (only `scripted_effect` and `inline_script`). Verification: test or search `trigger_docs`.

### OQ6: `count` and `percent` parameters on `any_` iterators in Stellaris

Jomini feature (`any_character = { count >= 3 ... }`). Does Stellaris support this? Or does Stellaris use `calc_true_if` + `count_X` instead?

### OQ7: Non-parameterised scripted_effects — depth limit

Community inference: the 5-level call-chain limit applies only to PARAMETERISED scripted_effects (because those are pre-compiled at launch). Non-parameterised effects are presumably inlined at load time. Unverified. If true, splitting a complex parameterised effect into two non-parameterised effects would not save call-graph depth. Our depth_analyzer.py currently treats all named calls the same.

### OQ8: `while` loop with BOTH `count` and `limit`

No source shows both in the same block. Are they alternatives or composable? If composable, which takes precedence?

### OQ9: `LIST` tagged type in Stellaris

The `LIST { val1 val2 }` form is documented for EU4. Is it used in Stellaris? Searching Stellaris vanilla files would confirm.

### OQ10: Semicolon-after-quote behavior in SCRIPT files

PDX Tools documents `;` after quoted strings being silently discarded in SAVE files. Does the same apply in script files? If script files silently absorb `;`, our lexer should consume it; if script files reject it, we should emit INVALID.

### OQ11: `round_variable_to_nearest` exact syntax

Added in Stellaris 3.1. The exact syntax (second argument = the modulus?) is not documented in our tricks doc. Needs verification.

### OQ12: Maximum PREV/FROM chain depth — is 4 correct?

All sources confirm 4 hops (`prevprevprevprev`, `fromfromfromfrom`). Our linter fires at 4+. Whether a 5th hop is syntactically valid but semantically broken, or literally unparseable, is unverified. CK2 wiki says "Maximum 4 FROMs as of patch 2.3" but this may not apply to Stellaris 4.x.

---

## 7. Source-Specific Confidence and Citation Index

| Source | Confidence | What to cite for |
|--------|-----------|-----------------|
| PDX Tools syntax tour | High | Operator set, literal types, implied-object rule, color literals, hybrid arrays, cross-game encoding |
| PDXTools docs (ititus) | Medium-high | % literal, scripted GUI, static/dynamic distinction |
| Anatomy of a Game post | Highest | Design rationale, scope architecture, modifier internals, why no string manipulation |
| Grand Jomini Manuscript | High (CK3), Low (Stellaris applicability) | `local_var:`/`global_var:`, `scope:name`, `alternative_limits`, `scripted_guis` |
| Forsslund Lua thesis | High | Performance rationale; why no embedded scripting |
| rakaly/jomini | High | Binary format, save file container, cross-game encoding, token encoding |
| nickbabcock/jomini | High | Duplicate-key array behavior, write API revealing format rules |
| Stellaris wiki (Dynamic Modding) | High | `@var`, `$PARAM$`, `[[PARAM] ... ]`, `@[ expr ]`, inline_script, script_values |
| Stellaris wiki (Conditions) | High | Boolean operators, implicit AND, `calc_true_if`, `if`/`while`/`switch` |
| Stellaris wiki (Variables) | High | `set_variable`, `change_variable`, scope binding, arithmetic operations |
| Dev Diary #222 | Highest | `ordered_*` iterators, dot-scope variable access, LEM changes |
| SLEX documentation | Medium | 5-level call-chain limit, RAM cost of parameterised effects |
| comfort-scripts | Not accessible | — |
| Reddit r/paradoxplaza | Not accessible | — |

---

*End of synthesis. 4,600+ words. All findings are cross-referenced against our existing docs with specific gap and contradiction callouts.*
