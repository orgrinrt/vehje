# Clausewitz Grammar — Formalization (Batch G1)

**Status:** Design doc, not implementation. Output of Batch G1 of the
Grammar Formalization initiative. Implementation tasks G2 + G3 (and
follow-ups) live in the task list.

**Scope note:** this document formalizes the **Clausewitz** surface
syntax (the engine-script language we compile to). The **Clause**
authoring language that produces Clausewitz — and which every author
writes — has its own normative grammar in
[`CLAUSE_EBNF.md`](./CLAUSE_EBNF.md). Don't mix the two; everything
below applies to `.txt` Clausewitz, not `.cse` Clause source.

**Date:** 2026-04-16
**Driver:** the @variable confusion exposed during MD1 — current parsing
treats `@foo` as a regular identifier and downstream the semantic gap
between "scope-bearing variable declaration" and "addressable item"
becomes invisible.

---

## 1. Why this matters

### The exemplar bug

`tools/clause/compiler/clausewitz/parser.py` is a line-by-line
regex scanner using `_IDENT = r"@?[a-zA-Z_][a-zA-Z_0-9.:\-]*"`. The `@?`
makes the prefix optional, so `@barrenBiomassCostCheap` looks identical
to `barrenBiomassCostCheap` to every consumer. Downstream:

- `extract_items.py` adds `@barrenBiomassCostCheap` to the items table as
  if it were a Stellaris entity.
- `prefer_mod` patches against `@barrenBiomassCostCheap` enter the universe
  via `patches_idx`.
- `compile_item` runs `prefer_mod.apply()` which calls
  `extract.extract(mod_id, dir, "@barrenBiomassCostCheap")`.
- The extract sometimes succeeds, sometimes fails — there's no consistent
  semantic for "what does @barren mean here?".

The bug is *not* in any one place. It's in the absence of a representation
that distinguishes "this token is a variable declaration" from "this
token is an item identifier." The grammar lacks the concept.

### The five micro-parsers

The current codebase has at least five independent parsers, each
re-deriving brace structure / identifier shape from scratch:

| File | What it parses | How |
|---|---|---|
| `clausewitz/parser.py` | Top-level entries | regex + line-wise brace counter |
| `clausewitz/vars.py` | File-scope @vars | regex over file head |
| `clausewitz/balance.py` | Brace balance | hand-written counter |
| `compile/ops/replace_in_item.py` | Find/replace targets | string match (no parse) |
| `passes/builtin/optimizers/leaf_inliner.py:_extract_body` | Inline-script body splice | hand-written brace tokenizer |

Each handles 95% of cases and breaks on different edges. None know about
the others. Adding a depth analyzer requires re-deriving brace nesting.
Adding `inline_script` resolution (MD2) means adding a 6th parser.

### What a real frontend gives us

A single tokenizer + IR means:

- **Every pass agrees on what a thing IS.** `@foo` is a `VAR_DECL` or
  `VAR_REF`, never ambiguous.
- **Adding a new analysis is local.** Walk the token stream, no parsing
  re-derivation.
- **Macros plug in cleanly.** `{{ prefer_mod(123) }}` becomes a
  `MACRO_INVOKE` token whose expansion *replaces it in the stream* at a
  defined phase.
- **Coverage is forced.** A token type missing from the lexer means the
  source can't be processed — discovered immediately, not 200 builds
  later.
- **Round-trippable trivia** means we can emit modified files that look
  like the input (preserves comments, blank lines, indentation
  preferences), which matters for diffing patch output against source.

---

## 2. Goal

Replace the regex-and-brace-counter parser stack with a proper compiler
frontend whose primary IR is a **token stream** (with a thin structural
overlay for brace-matched blocks). Every existing parser becomes a thin
function over the token stream.

**Non-goals**: a full AST with typed nodes. Clausewitz isn't typed. The
overlay is just `Block(open_token, [Item, ...], close_token)` and
`Assignment(name_token, op_token, value_token_or_block)`. That's enough
structure for every pass we currently run.

**In-scope**:

- Vanilla Clausewitz syntax (covered in §4).
- Our extensions: `{{ macro(...) }}`-style macro invocations.
- Eventually: a custom patch DSL replacing YAML, parsed by the same
  lexer with a few additional token kinds.

**Out-of-scope (deferred)**:

- Full type-checking of trigger/effect arguments (cwtools handles that;
  we cooperate via lints, don't replace).
- Localization (`.yml`) parsing — different syntax, separate parser
  remains.

---

## 3. Token taxonomy

Every byte of source falls into one of these token kinds. If a future
mod ships syntax we don't recognize, the lexer emits `INVALID(text)`
and the patch fails loudly — that's the safety property.

Categories:

### 3.1 Identifiers

| Kind | Example | Notes |
|---|---|---|
| `IDENT` | `trait_cold`, `country_event`, `pop_job_specialist`, `anti-matter` | The general-purpose identifier. Allowed chars: `[A-Za-z_][A-Za-z_0-9\-]*`. **Hyphens are part of the identifier char set** (cwtools + eug parsers allow it; Stellaris ships IDs like `anti-matter` and `opt-in`). Dotted forms (`country.123`) are split via `DOT` (per §3.7). |
| `QUOTED_IDENT` | `"bow"`, `"part1"` | A string literal *in name position* (LHS of `=`). Lexer emits `STRING`; the parser retags it `QUOTED_IDENT` when it appears in name position. |
| `VAR_DECL_AT` | `@foo` in LHS / top-level | `@`-prefixed identifier in declaration context. cwtools missed this distinction (issue #64) — its semantic layer hacks `n.Key.StartsWith('@')`. We split it at the lexer. *Position-disambiguated by parser; lexer emits `VAR_AT(name)` and parser retags.* |
| `VAR_REF_AT` | `@foo` in value position | Same lexeme as `VAR_DECL_AT`; distinguished by AST position. |
| `TARGETED_VAR` | `ai_weight@ROOT`, `my_flag@root` | An identifier whose name *contains* `@SCOPE` as a separator — entirely distinct semantic from leading `@var`. cwtools issue #64 confirms parsers commonly miss this. The lexer recognizes `IDENT@IDENT` as a single `TARGETED_VAR` token; analyzer splits the parts. |
| `MACRO_PARAM` | `$STORM_NAME$` | A `$NAME$` placeholder. Substituted at inline-script expansion. |
| `MACRO_PARAM_DEFAULT` | `$STAGE|1$`, `$RESOURCE|minerals$` | `$NAME|default$` form — the default value follows a literal pipe. Substitution rules same as `MACRO_PARAM`. |
| `INLINE_EXPR_OPEN` / `INLINE_EXPR_CLOSE` | `@[`, `]` | Wrap inline arithmetic. Inside lives a sub-stream parsed by a tiny arithmetic mini-grammar (§3.11). |
| `CONDITIONAL_PARAM_OPEN` / `CONDITIONAL_PARAM_CLOSE` | `[[PARAM]`, `]` ; `[[!PARAM]` | Conditional macro block in inline_script bodies — content emitted only if PARAM is set (or, with `!`, only if absent). cwtools issue #57 — totally unsupported there. We emit two distinct open kinds (`CONDITIONAL_PARAM_OPEN_POS` for `[[X]`, `CONDITIONAL_PARAM_OPEN_NEG` for `[[!X]`) for clarity. |
| `EVENT_TARGET_REF` | `event_target:formless_country` | A first-class token, NOT `IDENT COLON IDENT`. Recognized by `event_target:` prefix; suffix is the target name. Dotted continuation (`event_target:foo.owner`) splits via `DOT` and produces a SCOPE chain. |
| `VAR_SCOPE_REF` | `var:my_runtime_var`, `value:scripted_value` | First-class for `var:` (runtime variable on a scope) and `value:` (script_value invocation). Pipe-delimited parameters (`value:foo|K|V|`) are tokenized via `PIPE` (§3.7) inside the suffix. |

### 3.2 Operators

| Kind | Example | Notes |
|---|---|---|
| `EQUALS` | `=` | Overloaded: assign / equality / scope-change / activate. Position-disambiguated. |
| `OP_GT` | `>` | Comparison + greater-than-key in `switch` cases. |
| `OP_LT` | `<` | Comparison + less-than-key in `switch` cases. |
| `OP_GE` | `>=` | |
| `OP_LE` | `<=` | |
| `OP_NE` | `!=` | Inequality. |
| `OP_EQ_EQ` | `==` | Explicit equality (cwtools confirmed; semantics vs `=` is community-debated, treat as distinct token; let semantic pass decide). |
| `OP_DEFAULT` | `?=` | "Set if not already set" / "exists then compare". **CONFIRMED** (was previously marked uncertain): cwtools' `QuestionEqual = 7uy` + PDX Tools examples (`c:RUS ?= this`) + CK3 wiki + multiple Stellaris content examples all agree. The jomini-spec + audit research agents both closed this as resolved. |

Verified absent (do NOT add):
- `<>` — appears only in comments / docstrings, not a real operator.
- No documented `&&` / `||` / etc. — the boolean operators are block-form (`AND = { … }`), not infix.

### 3.3 Literals

| Kind | Example | Notes |
|---|---|---|
| `INT` | `5`, `-3`, `+5`, `81477` | 32-bit signed (or 64-bit unsigned for large values). Leading `+` valid, ignored. |
| `FLOAT` | `1.5`, `0.025`, `-0.5` | 32-bit, ~4 decimal digits precision. No scientific notation. |
| `DATE` | `2200.06.21`, `1444.11.11` | `Y.M.D`. Quoted or unquoted. No leap years. Distinguished from `FLOAT` by 3-segment dotted form with no operator separator. |
| `STRING` | `"some text"`, `"grand_archive/space_fauna_upkeep"` | Double-quoted. Confirmed escapes: `\"` and `\\`. Literal newlines allowed inside quotes. `\n`/`\t` not documented (treat as literal backslash-n). |
| `BOOL` | `yes`, `no` | Lexed as own kind with cwtools-style lookahead guard: `yes` followed by an id-char (e.g. `yes_please`) reverts to `IDENT`. |
| `PERCENT` | `5%`, `12.5%`, `-3%` | Numeric literal with a `%` suffix; engine treats as a fraction of 100 for modifier values. Documented in Jomini grammar (HOI4 / CK3) — **Stellaris applicability pending verification via G1.5 harvest**; lexer emits the token conservatively (parsers can fall back to `INT` / `FLOAT` if the %-suffix form proves unused in Stellaris). |
| `NULL` | `none` | Distinct from `no`. Used for "no scope" / "absent" semantic. |

### 3.4 Brackets

| Kind | Example |
|---|---|
| `LBRACE` | `{` |
| `RBRACE` | `}` |

No square brackets except inside `@[ ]` inline expressions (handled by
`INLINE_EXPR_OPEN`/`CLOSE`).

**Implied-object rule (CRITICAL):** `foo { bar = qux }` is equivalent
to `foo = { bar = qux }` — the `=` before a block value is OPTIONAL
in the Jomini grammar. Parsers that require an explicit `=` between
name and block silently misparse files using this form and fall out
of sync for the rest of the file. The parser rule: after a name
token, peek ahead past whitespace/comments — if the next token is
`LBRACE`, synthesize an `EQUALS` position before it. Emitter can
choose to always write the `=` form for consistency.

**Trailing-`;`-absorption rule:** semicolons after quoted-string
values are silently discarded by the engine (common pattern in
older localisation and some ship files). The lexer absorbs a
trailing `;` immediately after a `STRING` token as trivia, not as a
syntactic token. Elsewhere `;` is treated as unexpected-punctuation
and produces a lexer diagnostic.

### 3.5 Reserved words (boolean / control / structural)

These are syntactically `IDENT`s but the lexer tags them as their own
kinds for fast classification by analyzers. Lexer logic: case-fold the
`IDENT.text`, look up in `RESERVED_TABLE` (case-insensitive); on hit,
emit the reserved kind preserving original case in `text`. Both `AND`
and `and` map to `KW_AND` (vanilla uses uppercase, mods often use
lowercase — both valid per online research; cwtools normalizes case-fold
internally via StringTokens).

**Boolean / logical**:

| Kind | Texts | Notes |
|---|---|---|
| `KW_AND` | `AND`/`and` | Implicit when block contains conditions without explicit operator. |
| `KW_OR` | `OR`/`or` | |
| `KW_NOT` | `NOT`/`not` | Multi-condition `NOT` officially unreliable — wiki warns "can cause unforeseen bugs"; use `NOR` explicitly. |
| `KW_NOR` | `NOR`/`nor` | |
| `KW_NAND` | `NAND`/`nand` | Rarer than NOR. |
| `KW_CALC_TRUE_IF` | `calc_true_if` | Counts conditions matching, compares via `amount` field. |

**Control flow (effect side)**:

| Kind | Texts |
|---|---|
| `KW_IF` | `if` |
| `KW_ELSE` | `else` |
| `KW_ELSE_IF` | `else_if` |
| `KW_WHILE` | `while` |
| `KW_BREAK` | `break` |
| `KW_SWITCH` | `switch` |
| `KW_INVERTED_SWITCH` | `inverted_switch` |
| `KW_CASE` | `case` |
| `KW_DEFAULT_CASE` | `default` |
| `KW_HIDDEN_EFFECT` | `hidden_effect` |
| `KW_RANDOM_LIST` | `random_list` |
| `KW_LOCKED_RANDOM_LIST` | `locked_random_list` |

**Control flow (trigger side)**:

| Kind | Texts |
|---|---|
| `KW_TRIGGER_IF` | `trigger_if` |
| `KW_TRIGGER_ELSE` | `trigger_else` |
| `KW_TRIGGER_ELSE_IF` | `trigger_else_if` |
| `KW_HIDDEN_TRIGGER` | `hidden_trigger` |

**Structural keywords (used as field names with special parser semantics)**:

| Kind | Texts | Notes |
|---|---|---|
| `KW_LIMIT` | `limit` | Trigger gate inside `if`/`while`/iterators. |
| `KW_INLINE_SCRIPT` | `inline_script` | Body is a path string OR a parameter block. |
| `KW_OPTIMIZE_MEMORY` | `optimize_memory` | Engine performance hint inside complex triggers. |
| `KW_SCRIPT` | `script` | Used inside `inline_script = { script = "path" … }` block form. |
| `KW_TRIGGER` | `trigger` | Used inside `switch = { trigger = <name> … }`. |
| `KW_COUNT` | `count` | Used inside `while`, iterators, calc_true_if positions. |
| `KW_AMOUNT` | `amount` | Used inside `calc_true_if`. |
| `KW_POSITION` | `position` | Used inside `ordered_*` iterators (v3.2+). |
| `KW_ORDER_BY` | `order_by` | Used inside `ordered_*` iterators. |
| `KW_INVERSE` | `inverse` | Used inside `ordered_*` iterators. |
| `KW_WEIGHT` | `weight` | Used inside `random_*` iterators (weighted selection). |

**CWT-style schema reserved (deferred — only relevant if/when we adopt CWT vocabulary, see §11)**: `type`, `types`, `alias`, `cardinality`, `push_scope`, `replace_scope`, `scope_group`, `severity`. Not lexed as KW_* until G7+.

**Iterators (NOT reserved — pattern-matched at semantic layer)**: `any_*`, `every_*`, `random_*`, `count_*`, `ordered_*` (v3.2+) are regular `IDENT`s whose prefix triggers iterator semantics. The list types are extensible (mods define new ones), so a fixed reserved table is wrong. Lexer treats them as `IDENT`; semantic pass classifies.

### 3.6 Scope words

Scope chain components — `IDENT.text in SCOPE_TABLE` → tag as `SCOPE_*`.
This makes scope-aware analyzers (depth, ownership, prevprev-in-active)
trivial: just walk for `SCOPE_*` tokens.

**Engine-reserved scope navigators** (case-insensitive):

| Kind | Texts | Notes |
|---|---|---|
| `SCOPE_THIS` | `this`/`THIS` | Current scope. |
| `SCOPE_ROOT` | `root`/`ROOT` | Original event/scripted-call scope; preferred over deep PREV chains. |
| `SCOPE_PREV` | `prev`/`PREV` | One stack level up. **DOTTED CHAIN IS CANONICAL** (`prev.prev.prev`). |
| `SCOPE_FROM` | `from`/`FROM` | Triggering-entity scope. **DOTTED CHAIN IS CANONICAL** (`from.from`). |

**Multi-segment forms — DEPRECATED but accepted**:

`prevprev`, `prevprevprev`, `prevprevprevprev`, `fromfrom`, `fromfromfrom`,
`fromfromfromfrom` are accepted by Stellaris but considered legacy.
Lexer tags them as `SCOPE_PREV_DEPRECATED` / `SCOPE_FROM_DEPRECATED`
(distinct kind so a lint can fire). Internal analysis canonicalizes to
`SCOPE_PREV DOT SCOPE_PREV …` form. The `prevprev` standalone form is
specifically broken in diplomatic triggers and `opinion_modifier`
scopes per `linters/builtin/content_syntax.py:174`.

**Maximum chain depth**: 4 hops via concatenation (`prevprevprevprev`).
Beyond 4 requires combining with dot notation. Online research
confirms: "CK2 documentation: Maximum 4 FROMs as of patch 2.3."

**Built-in scope identifiers** — these are NOT engine-reserved scope
navigators (`this`, `root`, `prev`, `from`) but are common scope-typed
identifiers that semantic analysis recognizes. The lexer emits these
as plain `IDENT`; the semantic pass classifies via a known-scope
lookup table (extensible per game version). Examples:
`owner`, `controller`, `planet`, `solar_system`, `capital_scope`,
`starbase`, `space_owner`, `sector`, `species`, `leader`, `fleet`,
`ship`, `army`, `pop`, `country`, `war`, `federation`,
`archaeological_site`, `first_contact`, `espionage_operation`,
`spy_network`, `galactic_object`, `megastructure`, `ambient_object`,
`deposit`, `tile`, `debris`, `design`, `pop_faction`.

(Resolved §9 question: extensibility wins — these are IDENTs filtered
by a known-scope table, not lexer-reserved kinds. Mods add new scope
types via game data; we'd have to ship a new lexer release for every
patch otherwise.)

### 3.7 Punctuation: scope-chain operators + structural

| Kind | Example | Notes |
|---|---|---|
| `DOT` | `.` in `prevprev.owner` | Splits scope chains. **Disambiguation rule**: `.` between two digits → part of `FLOAT`/`DATE`. Otherwise → `DOT` token. cwtools issue #73 is exactly the consequence of NOT splitting (FROM.FROM treated as opaque string, fails to map to FROMFROM scope). |
| `COLON` | `:` in `event_target:foo`, `value:scripted` | NOT emitted as standalone — the `event_target:`/`value:`/`var:` prefixes consume the colon as part of `EVENT_TARGET_REF`/`VAR_SCOPE_REF` tokens. |
| `PIPE` | `|` in `value:foo|K|V|` | Used for parameter lists in `value:` invocations and for `MACRO_PARAM_DEFAULT` separator. cwtools doesn't have a token for this — pipe is part of the opaque string value. We split. |
| `COMMA` | `,` in comma-delimited lists | Rare in Stellaris but legal per the Jomini grammar (PDXTools handles comma-delimited values). Emit as a dedicated token — the parser treats it as a soft separator equivalent to whitespace inside bags / arrays. Absent → treat as `IDENT` char is the previous regex-era misread. |

### 3.7b Color-space tags

Color values are expressed as `tag { components }` — an `IDENT` followed
by an unkeyed `Block` of numeric values. The lexer doesn't need new
token kinds (the tag is a regular `IDENT`, the block is a normal
`Block` per §5), but the parser must accept the keyless-block form.

| Tag | Form | Notes |
|---|---|---|
| `rgb` | `rgb { R G B }` | Components are `INT` 0-255 OR `FLOAT` (cwtools issue #53 — they reject floats; we accept both). RGBA via 4-component form. |
| `hsv` | `hsv { H S V }` | Components are `FLOAT` 0.0-1.0. |
| `hsv360` | `hsv360 { H S V }` | H is 0-360, S/V are 0-100, all `INT`. |
| `hex` | `hex { aabbccdd }` | 8-char hex string; last 2 = alpha. Token-wise the body is an `IDENT` lookalike. |
| `LIST` | `LIST { item_a item_b item_c }` | Ordered array tag — bag of unkeyed scalars. Documented in the Jomini grammar; **Stellaris applicability pending verification via G1.5 harvest** (appears in HOI4 / CK3 files but not yet confirmed in Stellaris content). Tag is a regular `IDENT`; body is a keyless `Block` of `Scalar` items. |

### 3.11 Inline arithmetic inner grammar

Inside `INLINE_EXPR_OPEN` … `INLINE_EXPR_CLOSE` the token stream uses
a tiny arithmetic mini-grammar — the lexer switches modes:

| Kind | Example |
|---|---|
| `ARITH_PLUS` | `+` |
| `ARITH_MINUS` | `-` (binary or unary) |
| `ARITH_STAR` | `*` |
| `ARITH_SLASH` | `/` |
| `ARITH_LPAREN` | `(` |
| `ARITH_RPAREN` | `)` |
| `INT` / `FLOAT` | as in §3.3 |
| `IDENT` | a variable reference — name of an `@var` or `$PARAM$` |
| `MACRO_PARAM` / `MACRO_PARAM_DEFAULT` | `$X$`, `$X|default$` — substituted before arithmetic eval |

cwtools issue #54 is exactly the consequence of treating `@[ … ]` as
an opaque string — they reject `@[ 0.33 - ( 0.06 * sartek_acg_mod_active ) ]`
because their content character set doesn't include `(`, `)`, or `-`.
We avoid by lexing properly.

**Game engine constraint**: only the FIRST `@[ … ]` per scripted_effect
or scripted_trigger body is correctly parsed. Subsequent ones are
silently ignored. This becomes lint
`INLINE_EXPR_MULTIPLE_PER_BODY` (file under LINT_CATALOG Group 2 —
cross-reference scope).

**Operators supported by Stellaris engine**: `+`, `-`, `*`, `/` only
(per online research). Anything else (e.g. `=`) causes string
concatenation rather than arithmetic. The grammar is permissive (we
allow more operators in tokenization to avoid cascading failures);
semantic pass should warn on non-engine-supported ops.

### 3.8 Trivia (preserved but ignored by analyzers by default)

| Kind | Example |
|---|---|
| `WHITESPACE` | spaces, tabs |
| `NEWLINE` | `\n`, `\r\n` |
| `COMMENT` | `# anything to EOL` |

Trivia is *attached* to the next non-trivia token (its `leading_trivia`
list) and the last token of the file gets `trailing_trivia`. This makes
round-tripping cheap: emit `tok.leading_trivia + tok.text` for each
token in order.

**Blank-line attachment rule (refinement over pure Roslyn style):** a
COMMENT followed by 2+ NEWLINEs (i.e. a blank line separating it from
the next construct) attaches to the PRECEDING section as trailing
trivia rather than to the following token's leading trivia. This
matches the `eug` parser convention and produces more natural
attributions — a block of notes above a section isn't silently
re-associated with the next unrelated declaration. Single-blank-line
separators keep the Roslyn "attach to following token" default.

### 3.9 Errors

| Kind | Example |
|---|---|
| `INVALID` | any byte sequence the lexer can't classify |

`INVALID` tokens carry the offending text + a reason string. A patch
that compiles down to a stream containing any `INVALID` is a hard fail
at the gate.

### 3.10 Macro extension (our addition)

Reserved for our patch DSL — vanilla files never produce these.

| Kind | Example |
|---|---|
| `MACRO_OPEN` | `{{` |
| `MACRO_CLOSE` | `}}` |
| `MACRO_BODY` | everything between `{{` and `}}` (re-tokenized by the macro lexer, not the Clausewitz lexer) |

Macros are expanded by a dedicated pass that runs *after* the Clausewitz
lexer but *before* downstream analyzers. The expansion replaces
`MACRO_OPEN MACRO_BODY MACRO_CLOSE` in the token stream with the
macro's output tokens.

---

## 4. Lexer contract

```python
def lex(text: str, *, source_path: str | None = None) -> list[Token]: ...
```

**Token shape**:

```python
@dataclass(frozen=True)
class Token:
    kind: TokenKind
    text: str                    # exact source bytes
    line: int                    # 1-based
    col: int                     # 1-based
    byte_offset: int             # absolute offset into the source string
    leading_trivia: tuple[Token, ...] = ()
    # Optional: source_path attached at file level via an outer wrapper
```

**Properties**:

1. **Lossless round-trip**: `"".join(t.leading_trivia.text + t.text for t in tokens)` reconstructs the source byte-for-byte.
2. **Total**: every byte of input is covered. Unrecognized → `INVALID(text=...)`. No silent drops.
3. **Pure function**: same input → same output. No DB / filesystem access during lex.
4. **Linear time**: single pass with bounded lookahead (max 2 chars). No backtracking on the main path; `attempt`-style backtracking only inside numeric-vs-identifier disambiguation.
5. **Deterministic on errors**: `INVALID` tokens have a `reason` field. Mostly: invalid byte, unclosed string, unclosed `@[`, unclosed `[[PARAM]`.

**Encoding** (per cwtools' fixed-Windows-1252 lesson): auto-detect.
Order: UTF-8 BOM → UTF-8 → Windows-1252 fallback. Vanilla files are
mostly Windows-1252; mod files vary. Localisation `.yml` files require
UTF-8 BOM specifically (per `docs/heritage/mod-architecture-insights.md:778`)
— but those go through a separate parser, not this lexer.

**String interning** (per cwtools' `StringTokens`): identifiers and
quoted strings intern through a per-build `StringResourceManager` that
stores `(lower_case, original_case, quoted_flag)`. The token's `text`
field is an `InternedStr` reference, not a copy. Case-insensitive
comparison via `.lower` is O(1) reference equality. Saves significant
memory + CPU when the same identifier appears 10000s of times.

**Position packing** (per cwtools' `range`): pack `(file_index,
start_offset, end_offset)` into a single 8-byte struct. With ~10000
files in a typical build and ~10MB max file size, this fits with room
to spare. Tokens become much smaller (8 bytes for position vs 24+ for
loose `line, col, offset` triples).

**Pseudocode skeleton** (for reference; real impl in G2):

```python
def lex(text):
    pos = 0
    out = []
    pending_trivia = []
    while pos < len(text):
        if text[pos].isspace():
            pending_trivia.append(_lex_whitespace(text, pos))
        elif text[pos] == "#":
            pending_trivia.append(_lex_comment(text, pos))
        elif text[pos] == '"':
            out.append(_lex_string(text, pos, pending_trivia))
            pending_trivia = []
        elif text[pos] == "@" and text[pos+1:pos+2] == "[":
            out.append(_lex_inline_expr_open(text, pos, pending_trivia))
            ...
        elif text[pos] == "@":
            out.append(_lex_at_ident(text, pos, pending_trivia))
            pending_trivia = []
        elif text[pos] == "$":
            out.append(_lex_macro_param(text, pos, pending_trivia))
            pending_trivia = []
        elif text[pos] in "{}":
            out.append(_lex_brace(text, pos, pending_trivia))
            pending_trivia = []
        elif text[pos] in "<>=!":
            out.append(_lex_operator(text, pos, pending_trivia))
            pending_trivia = []
        elif text[pos].isdigit() or (text[pos] == "-" and text[pos+1:pos+2].isdigit()):
            out.append(_lex_number(text, pos, pending_trivia))
            pending_trivia = []
        elif text[pos].isalpha() or text[pos] == "_":
            tok = _lex_ident(text, pos, pending_trivia)
            out.append(_classify_reserved(tok))  # IDENT → KW_/SCOPE_/BOOL
            pending_trivia = []
        else:
            out.append(Token(INVALID, text=text[pos], reason="unrecognized byte", ...))
            pending_trivia = []
        pos = out[-1].end_pos if out else pos + 1
    return out
```

---

## 5. Light AST overlay

Above the token stream, expose a thin structural view:

```python
@dataclass(frozen=True)
class Block:
    open: Token        # LBRACE
    items: tuple[BlockItem, ...]
    close: Token       # RBRACE
    # Jomini-tape-inspired optimisation: cache the end index
    # (byte position of the close brace, OR the token index of
    # close) on the block itself. Patch-application + IR transform
    # passes can skip an entire block in O(1) instead of walking
    # its items, even when they don't care about the interior. For
    # vanilla events this saves real time on big `option = { ... }`
    # chains that patches never touch. See parsers research agent
    # findings for the Jomini tape reference.
    end_token_index: int | None = None  # index into the source token stream

@dataclass(frozen=True)
class Assignment:
    name: Token        # IDENT, VAR_DECL_OR_REF, QUOTED_IDENT, KW_*, SCOPE_*
    op: Token          # EQUALS, OP_GT, OP_LT, OP_GE, OP_LE, OP_NE
    value: Token | Block | InlineExpr
    # Trivia attached to op + value flows through their tokens.

@dataclass(frozen=True)
class InlineExpr:
    open: Token        # INLINE_EXPR_OPEN
    body: tuple[Token, ...]
    close: Token       # INLINE_EXPR_CLOSE

@dataclass(frozen=True)
class BareValue:
    """A value not introduced by `name =`. E.g. inside `tag = { foo bar baz }`,
    `foo`/`bar`/`baz` are BareValues."""
    token: Token

BlockItem = Assignment | Block | BareValue
```

The structural builder is a thin recursive function over the token
stream. Brace mismatch produces `INVALID` blocks (with the unmatched
brace tagged) so analyzers can report locations.

This is *enough* AST. We don't need typed nodes for triggers vs
effects vs modifiers — those are name-based dispatches that downstream
passes do via dictionaries keyed on the IDENT text.

---

## 6. Macro hooks

The future `{{ run_some_macro(arg, ...) }}` direction (per user
2026-04-16 redirect) plugs in here. There are TWO orthogonal macro
systems that must coexist:

1. **Our `{{ macro }}` system** — token-stream-level. Operates on
   token streams BEFORE downstream analysis. Replaces YAML `prefer_mod`
   etc. with first-class expressions: `@foo = {{ prefer_mod(MOD_ID) }}`.

2. **Engine `$PARAM$` substitution** — text-level, happens at game load
   inside `inline_script` bodies. We do NOT intercept this — the engine
   handles it at runtime. Our lexer emits `MACRO_PARAM` and
   `MACRO_PARAM_DEFAULT` tokens that survive through compilation
   intact, to be substituted at game-load time.

### 6.1 Our `{{ macro }}` expansion

1. Lexer emits `MACRO_OPEN MACRO_BODY MACRO_CLOSE` tokens for `{{ ... }}`.
2. A `MacroExpansionPass` runs over the token stream:
   - Finds `MACRO_OPEN`/`MACRO_CLOSE` triplets.
   - Sub-lexes the body with a macro-grammar lexer (function calls,
     identifiers, literals).
   - Looks up the macro by name, evaluates with arguments.
   - **Replaces** the triplet in the token stream with the macro's
     expansion tokens (same `Token` type, source-position carried
     through for diagnostics).
3. **Iterative expansion** (per cwtools' inline_script model): the
   expansion pass runs up to 5 iterations to handle nested macros.
   Cap matches Stellaris' own inline_script recursion limit. Beyond 5
   → `MACRO_RECURSION_LIMIT` error.
4. Downstream passes see only normal Clausewitz tokens.

Macros become a token-stream-rewrite rule. Composable, testable; the
failure mode is local.

Two specific macros to design first (in their own batch):

- `prefer_mod(MOD_ID)` — emits the chosen mod's value for a @var.
  Replaces today's YAML `prefer_mod` for @var targets.
- `set_value(N)` — explicit override of a @var to a literal.

### 6.2 Engine `$PARAM$` survival

`MACRO_PARAM` and `MACRO_PARAM_DEFAULT` tokens **must NOT be expanded
at compile time** if they appear inside an `inline_script` body — the
engine substitutes them at game load. Our compiler emits them
verbatim.

**Comment-substitution gotcha** (per Stellaris wiki + heritage
guidelines): the engine substitutes `$PARAM$` even inside `# ...`
comment lines. Therefore: we MUST NOT strip comments from inline_script
bodies before they ship. Our trivia handling preserves comments by
default (Roslyn-style attachment), so this is automatic — but the lint
suite should add `MACRO_PARAM_IN_COMMENT_HAZARD` to flag patterns
where a multi-line `$VALUE$` would mangle the comment.

### 6.3 Conditional parameter blocks `[[PARAM]body]`

`[[PARAM] content ]` and `[[!PARAM] content ]` (positive/negative
conditional) — content emitted only if PARAM is set / absent. Lexer
emits `CONDITIONAL_PARAM_OPEN_POS` / `_NEG` and
`CONDITIONAL_PARAM_CLOSE` per §3.1. Engine handles the conditional at
load time; we pass through.

cwtools issue #57 is exactly the consequence of NOT supporting these
— their parser fails on `[[ trade ] set_variable = { name = foo
value = 1 } ]` form. We must.

---

## 7. Patch DSL future

Once the lexer + AST overlay land for Clausewitz, replacing YAML
patches with a custom DSL is straightforward:

```
# Future patch DSL example (NOT current syntax)
patch trait_cold in common/traits {
  override {
    trait_cold = {
      modifier = {
        pop_growth_speed = 0.10
      }
    }
  }
  rationale: "Consolidates GF + Ariphaos thermal trait fixes."
  status: reviewed
}
```

The DSL is parsed by an *extended* lexer with a few additional kinds
(`KW_PATCH`, `KW_OVERRIDE`, `KW_RATIONALE`, ...). Most of the body is
*Clausewitz tokens reused as-is* — that's the win. The patch DSL and
the target language share an IR.

This is its own batch (G4+), not in scope for G1-G3.

---

## 8. Migration plan (overview; details in tasks)

### G1 (THIS DOC) — Design

✅ Token taxonomy + lexer contract + AST overlay sketch.

### G2 — Lexer + compat layer

Implement `clausewitz/lex.py` per §3-§4. Keep `parser.py` working by
having `iter_top_level_entries` consume the token stream internally
(thin shim). All existing tests must pass with the shim in place.

### G3 — Migrate consumers

Rewrite the 5 micro-parsers to consume tokens:

- `cw_vars.FileVars` → walk for `VAR_DECL_OR_REF` at depth 0.
- `clausewitz/balance.py` → count `LBRACE`/`RBRACE` over token stream
  (one-liner).
- `compile/ops/replace_in_item.py` → token-aware find (matches across
  whitespace differences).
- `passes/builtin/optimizers/leaf_inliner.py:_extract_body` → splice
  token range between matching braces.
- Depth analyzer pass → walk `KW_LIMIT`/`KW_AND`/`KW_OR`/`KW_NOT`
  nesting as a token-tree fold.

Delete the regex parsers. Each consumer migration ships independently
with its own regression suite.

### G4 — Macro pass + scripted_variables global resolver

Build `MacroExpansionPass` per §6. Add `GlobalVariableLoader` pass that
indexes `common/scripted_variables/*.txt` and exposes the merged scope
to downstream passes (file-local @vars override globals per vanilla's
own rule).

### G5 — Custom patch DSL (parser only)

Per §7. Doesn't touch downstream — patches enter the same patches_idx
either way.

### G6 — YAML patch retirement

Migrate the corpus from YAML to DSL via an automatic translator (the
DSL is a strict superset semantically). Sunset the YAML loader.

---

## 9. Resolved questions (with research backing)

The original 5 G1-doc questions plus 4 surfaced by research, all
resolved before G2 begins.

1. **Q1: Reserved-word table or pure IDENT?** — RESOLVED: **tagged**,
   case-insensitive. Reserved table as single source of truth (§3.5).
   Online research confirmed both `AND` / `and` appear in vanilla.
2. **Q2: Should `.` and `:` always split?** — RESOLVED: **DOT always
   splits** (with float/date carve-out). **COLON does NOT split** —
   subsumed by `EVENT_TARGET_REF`/`VAR_SCOPE_REF`/`VAR_SCOPE_REF`
   first-class tokens. cwtools issue #73 (FROM.FROM mis-resolution)
   validates the splitting decision; cwtools issues with
   `event_target:` validate the colon-as-prefix-marker decision.
3. **Q3: Lex `yes`/`no` as `BOOL` always or only in value position?**
   — RESOLVED: **always BOOL** with cwtools-style lookahead guard
   (`yes_please` → IDENT). cwtools `Bool` case (§2 of cwtools review)
   confirms this is the right level.
4. **Q4: `INVALID` policy at compile time** — RESOLVED: **hard-fail**
   per Tenet 8. Lexer continues processing (emits `INVALID` token, no
   silent drop) so analysts see all errors; the build gate fails on
   any `INVALID`.
5. **Q5: Trivia attachment policy** — RESOLVED: **next-attached
   (Roslyn-style)**. cwtools' `Trivia option` field on Leaf/Node
   confirms this is the right shape.
6. **Q6 (NEW from local-docs): Case-insensitivity of keywords**
   (`AND` vs `and`) — RESOLVED: case-fold on lookup, preserve original
   case in `text`. Both forms map to the same token kind. Confirmed
   needed by `_STRUCTURAL_KEYWORDS` lowercase + `API_REFERENCE.md`
   uppercase divergence.
7. **Q7 (NEW from online): Maximum PREV/FROM concatenation chain
   length** — RESOLVED: 4 hops (`prevprevprevprev`). Beyond 4 requires
   dot notation. Source: CK2 wiki "Maximum 4 FROMs as of patch 2.3"
   (engine-generic). Token taxonomy lists exactly these 4 forms as
   `SCOPE_PREV_DEPRECATED` / `SCOPE_FROM_DEPRECATED`.
8. **Q8 (NEW from cwtools): `prevprev` standalone token vs
   `prev.prev`** — RESOLVED: lexer emits both forms but tags multi-segment
   forms as `_DEPRECATED`. Canonicalize to `SCOPE_PREV DOT SCOPE_PREV …`
   internally; lint flags the deprecated form. Per
   `linters/builtin/content_syntax.py:174` `prevprev` is broken in
   diplomatic triggers and `opinion_modifier` scopes — actively wrong,
   not just stylistically discouraged.
9. **Q9 (NEW from cwtools): Targeted variables `name@SCOPE`** —
   RESOLVED: distinct token kind `TARGETED_VAR`. Two completely
   unrelated uses of `@`: leading (`@var` = scripted variable) vs
   internal (`name@SCOPE` = targeted variable). cwtools issue #64
   shows what happens when you don't separate them.

---

## 10. Known Clausewitz constraints (engine-level gotchas)

These are runtime / load-time constraints the engine enforces. Every
analyzer/lint pass should be aware of them. Each maps to a specific
lint or warning (some not yet implemented; cross-ref `LINT_CATALOG.md`).

### Depth limits — there are TWO

1. **Brace nesting depth ≥ 6** in trigger/effect directories →
   loading-screen hangs. Raw `{` counter, no semantic interpretation.
   Lint: `TRIGGER_DEPTH_EXCEEDED` (already in catalog,
   `content_syntax.py:65`).
2. **Call-graph depth > 5** for parameterised scripted_trigger /
   scripted_effect chains → silent failures at depth 6. Pre-compiled at
   game launch with ~30 conditional branches per occurrence (RAM
   reason). Source: SLEX docs +
   `passes/builtin/depth_analyzer.py:35`. **`inline_script` BYPASSES
   this limit** because it's parse-time text substitution — that's why
   leaf inlining is an optimization.
3. Plain (non-parameterised) scripted_effects may not have the same cap
   — community inference, unverified.

### `@[ expr ]` constraints

- **Operators**: only `+`, `-`, `*`, `/`. Anything else (e.g. `=`)
  causes string concatenation, silent wrong values.
- **First-only per body**: only the FIRST `@[ expr ]` in a given
  scripted_effect / scripted_trigger body is correctly evaluated.
  Subsequent ones silently break. Lint: `INLINE_EXPR_MULTIPLE_PER_BODY`
  (NEW — not yet in catalog; add via G3e or follow-up).
- Use `script_values` for complex chained arithmetic.

### `$PARAM$` substitution gotchas

- Substitutes inside comments. `# uses $PARAM$ here` will be mangled
  if the substitution is multi-line. Lint:
  `MACRO_PARAM_IN_COMMENT_HAZARD` (NEW).
- `$PARAM|default$` provides a fallback.
- Parameter substitution can occur in keys: `$KEY$ = { … }` generates
  entire key-value pairs dynamically.
- Confirmed in `inline_script` bodies + `scripted_effect` bodies.
  Whether `scripted_trigger` bodies support `$PARAM|default$` is
  unverified.

### Scope chain constraints

- `prev` / `from` chains: max 4 concatenated. Beyond → dot notation.
- Dot-chained scopes do NOT push new PREV entries. After `owner.capital_scope.solar_system = { … }`
  block, `prev` refers to whatever was active *before* the chain, not
  to the intermediate scopes. Lint: `DEEP_SCOPE_CHAIN`
  (`content_syntax.py:222`).
- `prevprev` (single token, deprecated): broken in diplomatic triggers
  and `opinion_modifier` scopes. Lint: `PREVPREV_IN_ACTIVE_CODE`.
- Global event targets (`save_global_event_target_as`) are LOST on
  save/load. Recovery: re-register from
  `on_single_player_save_game_load` (single-player) or
  `on_multiplayer_game_loaded` (multiplayer). Lint:
  `GLOBAL_EVENT_TARGET_NEEDS_REREGISTRATION` (NEW).

### Boolean block gotchas

- `NOT = { A B }` behaves like NOR (not like
  `NOT = { OR = { A B } }`). Officially "can cause unforeseen bugs"
  per the wiki. Lint: `NOT_WITH_MULTIPLE_CONDITIONS` (NEW —
  recommend rewrite to `NOR`).
- `calc_true_if = { amount >= N cond1 cond2 … }` returns true if at
  least N of the listed conditions are true. `amount` supports `>=`,
  `<=`, `>`, `<`, `=`.

### Iterator constraints

- `every_*` is O(n×m) when nested (`every_country { every_leader { … } }`).
  Must NOT appear in monthly pulse events. Lint:
  `NESTED_EVERY_IN_PULSE_EVENT` (NEW).
- `any_*` returns boolean (cheap); `count_*` returns numeric.
- `random_*` supports `weights = { base = float modifier = { … } }`.
- `ordered_*` (v3.2+) deterministic; supports `position`, `order_by`,
  `inverse`.
- These are NOT engine-reserved keywords — pattern-matched on prefix
  at game load against the list system. Mods can define new lists.

### Performance constraints

- **Parameterised scripted_effects**: ~30 effects + ~2MB RAM compiled
  per invocation site. At 1000+ uses, 2GB+ RAM. Use `inline_script`
  for static-text reuse instead.
- **`while` loops**: 1000-iteration cap (CK3 confirmed; Stellaris
  presumed same). No `break` documented for `while` (use `if = { limit
  = { … } break = yes }` pattern in CK3).
- **`optimize_memory`** keyword is an engine hint for complex triggers
  — caches/frees scope allocations aggressively. Most effective on
  branching logic; no benefit on simple one-liners.

### File-format constraints (out of scope for the lexer but worth noting)

- Localisation `.yml` requires UTF-8 with BOM. Plain UTF-8 silently
  fails. (Affects a separate parser, not this one.)
- Vanilla Clausewitz files are Windows-1252; modern mods often UTF-8.
  Auto-detect (per §4).

---

## 11. Lessons from cwtools

cwtools is the most widely-used Paradox-script validator. Reviewing
its source (F#, FParsec-based) was essential context for designing
our compiler. Full review:
`.cache/reviews/2026-04-16/cwtools-formalization-review.md`.

### Worth copying

- **`yes`/`no` lookahead-guarded BOOL parsing** — `yes_please` reverts
  to IDENT. Already in our taxonomy (§3.3).
- **8-operator set** — exactly the right enumeration (§3.2).
- **`StringTokens` interning** — `(lower, original, quoted)` triple,
  reference-equality for case-fold compare. Adopt in §4.
- **Bit-packed `range` positions** — 8-byte struct with file index +
  start/end. Adopt in §4.
- **`{ Root; From: list; Scopes: list }` scope context stack** — right
  shape for analyzers. `PREV` pops `Scopes`. Adopt in G4b
  (GlobalVariableLoader) + future scope-validator passes.
- **Iterative inline_script expansion (max 5 iterations)** with
  `$PARAM$` text substitution per iteration. Adopt the policy + cap in
  G4a (MacroExpansionPass).
- **CWT schema vocabulary** — `alias[class:name]`, `cardinality`,
  `push_scope`, `replace_scope`, `scope_group`. Adopt vocabulary in
  our schema layer when we build one (post-G6); this lets us
  eventually import `cwtools-stellaris-config` files for free.
- **`Trivia option` attached to nodes** — confirms our Roslyn-style
  trivia approach is right.

### Worth avoiding (cwtools' open issues = our test cases)

- **Issue #64** — `@` in idCharArray means lexer can't distinguish
  `@var` decl from `@var` ref from `name@SCOPE` targeted-var. Our fix:
  `VAR_DECL_AT` + `VAR_REF_AT` + `TARGETED_VAR` as distinct tokens
  (§3.1).
- **Issue #54** — `@[ expr ]` opaque string fails on `()`, `-`,
  parentheses. Our fix: proper `ARITH_EXPR` inner grammar (§3.11).
- **Issue #57** — `[[PARAM]body]` conditional blocks unsupported. Our
  fix: `CONDITIONAL_PARAM_OPEN_POS`/`_NEG` tokens (§3.1, §6.3).
- **Issue #73** — `FROM.FROM` not normalized to `FROMFROM`. Our fix:
  always split DOT, canonicalize to dotted form, lint deprecated
  multi-segment form (§3.6, §3.7).
- **Issue #53** — `rgb { 1 0.4 0.6 }` fails on float components. Our
  fix: color clauses accept INT or FLOAT (§3.7b).
- **No token stream emitted** — cwtools goes character → AST directly.
  Cannot perform token-level transformations (macro expansion) without
  re-parsing. Validates our token-stream-as-primary-IR design.
- **No error recovery** — cwtools fails the whole file on first error.
  Our `INVALID` token + continue policy is strictly better.
- **Fixed Windows-1252** — cwtools assumes encoding. Our auto-detect
  (UTF-8 BOM → UTF-8 → 1252 fallback) handles modern mod files.

### CWT schema language — adopt vocabulary, defer implementation

CWT (Clausewitz Type) is a schema language describing valid
Clausewitz syntax. cwtools' `RulesParser.fs` is 66 KB handling 40+
field type variants. We don't need that full surface for G3-G6, but
adopting the **vocabulary** now means we can later import
`cwtools-stellaris-config` files directly when we build our schema
layer. File a follow-up task post-G6 for "CWT schema loader."

---

## 12. What this doc is NOT

- A spec of all of Clausewitz semantics. We capture syntax; semantics
  (what `is_owner` does, what fields `pop_job` accepts) lives in mods'
  cwtools schemas + Paradox's own runtime.
- An implementation plan with line counts. G2/G3 task descriptions
  carry that.
- A deliverable change to the running compiler. No code in `clausewitz/`
  changes from G1.

---

## 13. Cross-references

### Research that fed this doc (G1.5 integration)
- Local docs scan: `.cache/reviews/2026-04-16/clausewitz-local-docs-findings.md`
  — 870 lines, 15 token-design implications, 9 open questions, 10 documented
  gotchas.
- Online research: `.cache/reviews/2026-04-16/clausewitz-online-research-findings.md`
  — 553 lines, ~50 citations from Stellaris wiki / PDX Tools / Paradox dev
  diaries / cwtools. Authoritative source for the operator set, scope rules,
  inline_script semantics.
- cwtools code review: `.cache/reviews/2026-04-16/cwtools-formalization-review.md`
  — 466 lines, F# source review, identifies 7 cwtools bugs we explicitly avoid.

### Project context
- The bug that motivated this doc: this session's transcript + task #10
  (MD1, redirected on 2026-04-16).
- Tenet 7 (consolidate) and Tenet 9 (no shell-out from inside what
  should be a pass) — token IR is the substrate that makes Tenet 9
  enforceable.
- `docs/megapatch/design/compiler-passes.md` — pass framework that the
  lexer + AST will plug into as analyzer/optimizer passes.
- Existing parser code: `tools/clause/compiler/clausewitz/`
  (`parser.py`, `vars.py`, `balance.py`).
- `docs/megapatch/LINT_CATALOG.md` — where the new lints from §10
  ("Known Constraints") get filed: `INLINE_EXPR_MULTIPLE_PER_BODY`,
  `MACRO_PARAM_IN_COMMENT_HAZARD`, `GLOBAL_EVENT_TARGET_NEEDS_REREGISTRATION`,
  `NOT_WITH_MULTIPLE_CONDITIONS`, `NESTED_EVERY_IN_PULSE_EVENT`.

### Source tree pointers for G2 implementation
- New files: `tools/clause/compiler/clausewitz/lex.py`,
  `tools/clause/compiler/clausewitz/ast.py`.
- Migrate (G3a-G3e): `clausewitz/vars.py`, `clausewitz/balance.py`,
  `compile/ops/replace_in_item.py`, `passes/builtin/optimizers/leaf_inliner.py`,
  `passes/builtin/depth_analyzer.py`.

---

## End of design

Implementation starts at G2. See task list.
