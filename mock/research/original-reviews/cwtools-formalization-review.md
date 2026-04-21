# cwtools Formalization Review
**Date:** 2026-04-16
**Reviewer:** automated agent review of cwtools source
**Purpose:** Extract implementation lessons for the stellar-heritage Clausewitz lexer + token taxonomy

Sources examined:
- `CWTools/Parser/Types.fs` — AST type definitions
- `CWTools/Parser/SharedParsers.fs` — core tokenizer + parser
- `CWTools/Parser/CKParser.fs` — file-entry points
- `CWTools/Process/Process.fs` — Statement→Node/Leaf transformation
- `CWTools/Process/Scopes/Scopes.fs` — scope system
- `CWTools/Process/Scopes/STLScopes.fs` — Stellaris-specific scopes
- `CWTools/Rules/RulesTypes.fs` — CWT type system
- `CWTools/Rules/RulesParser.fs` — CWT config file parser
- `CWTools/Rules/FieldValidators.fs` — semantic validation
- `CWTools/Validation/Validation.fs` — pipeline architecture
- `CWTools/Validation/Stellaris/STLValidation.fs` — Stellaris semantics
- `CWTools/Validation/Stellaris/STLEvents.fs` — event-specific validation
- `CWTools/Game/ResourceManager.fs` — file loading, inline_script inlining
- `CWTools/Game/Stellaris/STLGame.fs` — Stellaris game object
- `CWTools/Utilities/Utilities.fs` — StringTokens, string interning
- `CWTools/Utilities/Position.fs` — range/pos types
- `CWTools/Common/STLConstants.fs` — Stellaris constants
- `cwtools-stellaris-config/config/triggers.cwt` — Stellaris CWT triggers
- `cwtools-stellaris-config/config/effects.cwt` — Stellaris CWT effects
- `cwtools-stellaris-config/config/scopes.cwt` — Stellaris CWT scopes
- Open issues #53, #54, #57, #64, #73, #97 (parser bugs + limitations)

---

## 1. Architecture Summary

cwtools is a **parser-combinator pipeline** built on **FParsec** (an F# port of the Parsec family). It does **not** produce a separate token stream — it goes directly from character stream to AST. The parser is recursive-descent built from composable FParsec primitives (`pstring`, `pfloat`, `choice`, `attempt`, `between`). The result is a `ParsedFile` which is a `Statement list`. That list is then **post-processed** by a separate `BaseProcess` class into a typed `Node`/`Leaf`/`LeafValue` tree for semantic analysis.

There are therefore three distinct layers:

1. **Parse layer** (`SharedParsers.fs`): character stream → `Statement list`. No separate tokenization phase. No token stream emitted. Values are resolved immediately into F# union cases (`Bool`, `Int`, `Float`, `String`, `Clause`).
2. **Process layer** (`Process.fs`): `Statement list` → `Node`/`Leaf` tree. This is where the structural semantics live. Scope context is threaded through this transformation.
3. **Validation/rules layer** (`Rules/`, `Validation/`): the typed tree is validated against CWT schema files. This is where `@variable` references, scope chains, and type correctness are checked.

**Key architectural fact for us**: cwtools has no intermediate token stream. This is both its design and its limitation. For our compiler, we are explicitly designing a token stream as the primary IR — this is the right call, and reviewing cwtools confirms it.

---

## 2. Token Model

cwtools does not have a named token enum. Instead, it has a **value type discriminated union** that is populated immediately during parsing. The effective "token kinds" are:

### Value type (from `CWTools/Parser/Types.fs`)

```fsharp
type Value =
    | String of StringTokens      // unquoted bare identifier: foo, bar_baz, prev.from
    | QString of StringTokens     // quoted string: "hello world"
    | Float of decimal
    | Int of int
    | Bool of bool                // EXPLICIT: yes/no are NOT String, they become Bool
    | Clause of Statement list    // { ... } — nested block
```

### Operator type (from `Types.fs`)

```fsharp
type Operator =
    | Equals = 0uy               // =
    | GreaterThan = 1uy          // >
    | LessThan = 2uy             // <
    | GreaterThanOrEqual = 3uy   // >=
    | LessThanOrEqual = 4uy      // <=
    | NotEqual = 5uy             // !=
    | EqualEqual = 6uy           // ==
    | QuestionEqual = 7uy        // ?=
```

### Statement type (from `Types.fs`)

```fsharp
type Statement =
    | CommentStatement of Comment          // # ...
    | KeyValue of PosKeyValue              // key op value
    | Value of range * Value               // bare value (no key) — standalone values in a list
```

### Key parsing

Keys are parsed via `many1SatisfyL isIdChar "id character"`. The `idCharArray` includes:
`letters, digits, _, :, @, ., ", -, ', [, ], !, <, >, $, ^, &, |`

**Critical observation**: `@` is part of the legal identifier character set. This means `@foo` in key position is a `String` value, not a distinct token. The `@` prefix is handled at the **semantic layer**, not at the syntax layer.

### `@[...]` arithmetic expressions

Handled at value-parse time with a specific pattern: `@\[` followed by chars excluding `]` and `\`, terminated by `]`. The content is interned as a string (not evaluated). This means `@[ 0.33 - ( 0.06 * some_var ) ]` becomes a `String` value containing the literal text `@[ 0.33 - ( 0.06 * some_var ) ]`. There is NO separate arithmetic expression node in the AST. Issue #54 (open) is about this failing for subtraction and parenthesized expressions — the parser currently rejects `@[ 0.33 - ( 0.06 * sartek_acg_mod_active ) ]` even though it is valid game syntax.

### Color syntax

RGB and HSV are explicitly handled as clause variants:
- `rgb { R G B }` → `Clause` of three `Int` (or `Float` per issue #53 — currently broken)
- `hsv { H S V }` → `Clause` of three `Float`
- Both uppercase and lowercase variants are explicitly matched

### Boolean handling

`yes` and `no` are parsed with `skipString "yes"` / `skipString "no"` **before** the general string parser, with a lookahead check that the next char is not a valid value character. This means `yes` → `Bool(true)`, `no` → `Bool(false)`, `yes_please` → `String "yes_please"`. This is a principled disambiguation.

### Character encoding

Fixed to Windows-1252 (Latin-1 extended). If parsing fails, the system retries with a fallback encoding via `changeEncoding`. The `magicChar` in `idCharArray` (`š`, `Š`) was added explicitly for these encoding edge cases.

---

## 3. AST Model

After `Process.fs` transforms the `Statement list`, the tree has these node types:

### Node (keyed container)

```fsharp
[<Sealed>]
type Node(key: string, pos: range) =
    member _.KeyId: StringTokens
    member _.Key: string
    member _.AllArray: Child array
    member _.Nodes: Node seq
    member _.Leaves: Leaf seq
    member _.Values: LeafValue seq    // standalone values (no key)
    member _.Tag(key): Value option
    member _.Child(key): Node option
    member _.Trivia: Trivia option
```

### Leaf (key-value pair)

```fsharp
[<Sealed>]
type Leaf =
    member _.KeyId: StringTokens
    member _.Key: string
    member _.ValueId: StringTokens
    member _.Value: Value
    member _.Position: range
    member _.Operator: Operator
    member _.Trivia: Trivia option
```

### LeafValue (standalone value in a list context)

```fsharp
[<Sealed>]
type LeafValue(value: Value) =
    member _.Value: Value
    member _.ValueId: StringTokens
    member _.Position: range
    member _.Trivia: Trivia option
```

### Trivia

Trivia exists as an optional field (`Trivia option`) on leaves and nodes but is not described in detail in the source reviewed. Comments appear to be preserved via `CommentStatement` in the `Statement` layer, but their attachment to nodes in the final tree is unclear — comments may be dropped during the Statement→Node transformation.

### Child discriminated union

```fsharp
type Child =
    | NodeC of Node
    | LeafC of Leaf
    | LeafValueC of LeafValue
    | ValueClauseC of ValueClause    // special multi-value constructs
```

### No dedicated @[...] node

As noted in the token model section, `@[ expr ]` arithmetic is stored as a `String` value. There is no `ArithmeticExpr` node. This is a known limitation.

### Dotted keys (scope chains)

Keys like `prev.from.planet` are stored as a single `String` token containing the dots. They are **not decomposed at parse time** into a sequence of scope-traversal operations. The scope chain is resolved at the **validation layer** by calling `changeScope` which internally splits on `.` and resolves each segment. This separation of concerns is important to note.

---

## 4. Parser Approach

**FParsec parser combinators**, recursive descent, with explicit backtracking via `attempt`. Not hand-written in the traditional sense, but not a generated parser either.

### Key patterns

- **Forward reference** for mutual recursion: `let keyValue, keyvalueimpl = createParserForwardedToRef()`. This is how `value` and `keyValue` can be mutually recursive.
- **Lookahead-based dispatch** in `valueCustom`: the first character determines which branch to try (`{` → clause, `"` → quoted string, digits/`-` → int/float attempt, known prefixes like `rgb`/`hsv`/`yes`/`no` → explicit matches, `@[` → arithmetic snippet, else → unquoted string).
- **`attempt` for backtracking**: int parse is attempted; on failure, float is attempted; on failure, falls through to string. This means `1` is Int, `1.5` is Float, `1abc` is String.
- **Whitespace is consumed eagerly** after every token: `str s = pstring s .>> ws`. There is no separate whitespace token.
- **Error recovery**: uses `betweenL` (a custom version of FParsec's `between`) that provides contextual error messages about unclosed delimiters. There is no "skip to next balanced brace" error recovery — a parse failure returns `ParsedFile []` (empty) via `getSuccess`.

### No separate lexer phase

This is the most important architectural fact: there is no tokenizer producing a token stream. The parser reads directly from the character stream. This makes it impossible to emit a token stream for other consumers, makes it hard to add new syntax, and means every pass must re-parse from source.

---

## 5. Worth Copying

### 5a. `yes`/`no` → `Bool` disambiguation

cwtools correctly recognizes `yes` and `no` as boolean values distinct from string identifiers, with a lookahead guard that prevents `yes_please` from being misidentified. Our lexer should emit `BOOL_LIT(true)` and `BOOL_LIT(false)` as distinct token types, not `IDENT("yes")`.

### 5b. Operator taxonomy

The 8-operator set (`=`, `>`, `<`, `>=`, `<=`, `!=`, `==`, `?=`) is comprehensive and correct for Clausewitz. Our token taxonomy should match this exactly. The `?=` (QuestionEqual) is the "set if not already set" operator — easy to miss.

### 5c. String interning via `StringTokens`

The `StringTokens` struct pairs a lowercase-normalized token with the original-case token plus a `quoted: bool` flag. This is efficient and elegant. The `StringResourceManager` uses concurrent dictionaries for thread-safety. **For our lexer**: intern identifiers using a similar dual-token scheme. The case-insensitive comparison is critical because Clausewitz is case-insensitive for keys (but not for some string values).

### 5d. Position packing (range type)

The `range` type packs file index (16 bits), start/end position into 8 bytes total using bit manipulation. This is compact and fast. Our IR should do the same — attach a `Span` or `Range` to every token.

### 5e. Iterative inline_script resolution (max 5 iterations)

From `ResourceManager.fs`: inline_script expansion runs up to 5 iterations to handle nested `inline_script` references. Each iteration clones the script node and applies `$param$` → value substitution on all keys and values. **For our lexer/compiler**: inline_script should be resolved at a post-parse phase, not at lex time. The `$PARAM$` substitution is string-level (before re-parsing) — the parameters replace text in the cloned script body, which is then re-parsed. Our compiler should follow the same model: expand → re-tokenize → re-parse.

### 5f. Scope context as a stack

From `Scopes.fs`:
```fsharp
type ScopeContext =
    { Root: Scope
      From: Scope list
      Scopes: Scope list }
```

The `From` list is the historical "FROM chain" (FROMFROM, FROMFROMFROM etc.), and `Scopes` is the active traversal stack. `PREV` pops the `Scopes` stack. This is the correct model. Our compiler's scope validator should use exactly this structure.

### 5g. CWT `alias_name` / `alias_match_left` pattern

From `effects.cwt`:
```
alias[effect:if] = {
    limit = { alias_name[trigger] = alias_match_left[trigger] }
    alias_name[effect] = alias_match_left[effect]
}
```

This declares that inside `if`, a `trigger` alias-named key can appear whose value matches the trigger alias. This is a principled way to declare "any trigger is valid here, any effect is valid here" without enumerating them. The CWT format is a **schema language**, not a validator itself — it describes what valid syntax looks like, and the validator checks the tree against the schema. This is relevant if we build a schema layer.

### 5h. `event_target:` as a special prefix

From `Scopes.fs` and `STLScopes.fs`: the string `"event_target:"` is explicitly recognized as a prefix that transitions to `AnyScope`. Same for `"var:"`. These are not parsed as scoped identifiers — they are string-prefix matches in the semantic layer. Our lexer should emit `EVENT_TARGET_REF` as a distinct token type for `event_target:foo`, rather than treating it as a bare identifier.

### 5i. Trivia (comments) as optional attachment

Comments are preserved in the `Statement` layer as `CommentStatement`. The downstream tree has a `Trivia option` field on nodes/leaves, suggesting comments can be attached to their following node. This is the right approach for round-trip fidelity. Our design doc's "trivia-carrying IR" approach is validated.

---

## 6. Worth Avoiding

### 6a. No token stream emitted

The single biggest structural weakness of cwtools is that parsing goes directly from character stream to AST with no intermediate token representation. This means:
- No other tool can consume a token stream
- Adding syntax variants requires modifying the parser
- Error recovery is all-or-nothing (empty result on failure)
- Cannot perform token-level transformations (macro expansion, `prefer_mod` injection) without re-parsing

**Our approach of emitting a token stream as primary IR is the correct alternative.**

### 6b. `@` prefix handled at semantic layer, not lexer

Because `@` is in `idCharArray`, the lexer has no way to distinguish `@var = 100` (declaration) from `value = @var` (reference). Both are parsed as `String` tokens starting with `@`. The semantic check (`n.Key.StartsWith('@')`) in `STLValidation.fs` is a string-level hack. **We must distinguish `VAR_DECL_AT` (key position) from `VAR_REF_AT` (value position) as distinct token types.** Our grammar doc already specifies this.

### 6c. `@[...]` stored as an opaque string

Arithmetic expressions inside `@[...]` are stored as literal strings — there is no AST representation of the expression. Issue #54 (open, filed 2023) shows that even simple arithmetic like `@[ 0.33 - ( 0.06 * var ) ]` fails to parse because the character set for the content is too restrictive (doesn't allow parentheses). **We must lex `@[...]` as a proper `ARITH_EXPR` token and define the inner grammar** (at minimum: float literals, integer literals, variable references, `+`, `-`, `*`, `/`, `(`, `)`).

### 6d. Dotted scope chains as opaque strings

`prev.from.planet` is a single `String` token. Scope chain resolution is deferred to the semantic layer which splits on `.`. This creates a semantic/lexer impedance mismatch: the lexer has no knowledge of scope navigation. **Our design should emit `SCOPE_NAV` tokens for dotted traversals**, or at minimum flag the token as containing dots so the parser can decompose it into a chain of scope steps. Issue #73 (FROM.FROM scoping incorrectly) and issue #63 (scope stack display) are downstream consequences of this.

### 6e. RGB/HSV float parsing broken

Issue #53 (open, 2023): `rgb { 1 0.4 0.6 }` fails because the RGB parser expects integers, not floats. The Paradox engine accepts floats here. **Our parser must accept either int or float in color clauses.**

### 6f. No real error recovery

Parse failure returns an empty `ParsedFile []`. There is no "skip-and-resync" logic. This means a single syntax error prevents the entire file from being processed. For a mod compiler processing hundreds of files, this is a significant gap. **Our lexer should emit `ERROR` tokens for unrecognized input and continue**, allowing the parser to produce a partial tree with error nodes.

### 6g. Scope validation not connected to scope traversal in keys

From issue #73: `FROM.FROM` (dotted notation) is not correctly mapped to the `FROMFROM` scope constant defined in the STL scopes. The validator handles `FROMFROM` as a single-token scope name but fails to resolve the dotted `FROM.FROM` form that is also valid game syntax. **Our compiler should normalize both forms to the same internal scope representation during parsing.**

### 6h. Targeted variable syntax not fully supported

Issue #64: `ai_attitude_allied_weight@ROOT` (targeted variables — a variable scoped to a specific object) fails validation because the semantic layer expects the `@` to be a scripted variable prefix, not a scope-targeting separator. These are two entirely different uses of `@` in Clausewitz: `@foo` = scripted variable reference, `my_var@SCOPE` = a targeted variable whose name includes the scope. **These two uses require distinct token types in our taxonomy**: `VAR_REF_AT` (value starts with @), `TARGETED_VAR` (contains @ in non-leading position as scope separator).

### 6i. `[[parameter]]` conditional blocks not supported

Issue #57 (open, 2023): The `[[param]...content...]` syntax used in scripted_effects (conditional blocks that execute only if the parameter was passed) is not supported at all. This is a Crusader Kings III and also-applicable Stellaris syntax where `[[ trade ] set_variable = { name = foo value = 1 } ]` means "execute the block only if `trade` parameter was supplied." **Our lexer must handle `[[PARAM]BODY]` as a `CONDITIONAL_PARAM_BLOCK` token or structured node.**

### 6j. Fixed Windows-1252 encoding

The encoding assumption (Windows-1252) is baked in at `CKParser.parseFileInner`. Files using other encodings fail silently or produce mojibake. While most Stellaris files are Windows-1252 or UTF-8, this is worth flagging. **Our lexer should auto-detect encoding** (check for UTF-8 BOM, then fall back to Latin-1/1252).

---

## 7. Edge Cases / Weird Syntax Discovered

These are concrete, empirically-discovered syntax cases that cwtools has encountered, either as working features or as open bugs.

### 7a. `@[ expr ]` arithmetic with complex subexpressions

From issue #54 (Stellaris, open):
```
modifier = { weapon_damage_mult = @[ 0.33 - ( 0.06 * sartek_acg_mod_active ) ] }
```
The game engine accepts this. cwtools does not. The `@[...]` parser in `SharedParsers.fs` uses a character whitelist that excludes `(`, `)`, and apparently `-` in some positions. Any Clausewitz arithmetic expression can contain: float literals, integer literals, variable names, `+`, `-`, `*`, `/`, `(`, `)`, and whitespace.

### 7b. `@[ ( 333 / 768 ) + 0.001 ]` in variable declarations

From issue #53 (Vic3, but applies to Stellaris Coat-of-Arms):
```
@canton_scale_cross_x = @[ ( 333 / 768 ) + 0.001 ]
```
This is a **variable declaration** whose value is an arithmetic expression. Our lexer must handle `@VAR_NAME = @[ expr ]` as a valid declaration form, not just `@VAR_NAME = literal`.

### 7c. Float values in `rgb {}` blocks

From issue #53:
```
todo_purple = rgb { 1 0.4 0.6 }
```
cwtools only accepts integer components in RGB. The game accepts floats in some color-space variants. **Color clauses can have either integer or float components.**

### 7d. Bare standalone values in list context

From the test file `simple.txt` and the `Value(range, Value)` Statement variant:
```
label = {
    valuea
    valueb
}
```
A block can contain bare values (no key, no operator). These become `LeafValue` nodes. Our grammar must accommodate `CLAUSE_BODY ::= (KEY_VALUE | BARE_VALUE | COMMENT)*`.

### 7e. Key-value pairs without operators (type blocks)

From `clause.txt` test file:
```
types test
{
    values
    values
}
type blah = blah2 {
    values2 values2
}
```
This is the Clausewitz "type definition" syntax (`type NAME = BASE_TYPE { ... }` and `types NAME { ... }`). These are not key-value pairs in the normal sense. They appear to be parsed as special statement forms. **Our lexer should emit `TYPE_DECL` for the `type` keyword and `TYPES_DECL` for the `types` keyword as reserved forms.**

### 7f. `?=` operator (conditional assignment)

The `QuestionEqual = 7uy` operator in `Types.fs` is `?=`. This is the "set if not already set" or "set if matches" operator. It is valid Clausewitz syntax that most parsers miss.

### 7g. Multi-segment inline_script parameter substitution

From `ResourceManager.fs`: inline_script parameters use `$PARAM$` format. The substitution is recursive (up to 5 iterations). Parameters can appear in:
- Keys: `$param_key$ = value`
- Values: `some_key = $param_value$`
- Nested inline_script references: `inline_script = "path/$variant$/file"`

This means `$...$` inside identifiers is valid syntax that must survive lexing intact until the inline_script expansion phase.

### 7h. `var:` prefix on scope transitions

From `STLScopes.fs`: `"var:"` prefix (lowercase) triggers `AnyScope` resolution — it's a targeted variable access in scope context. This is distinct from the `@var` scripted variable. `var:my_variable` references a runtime variable value stored on an object. **Our taxonomy needs `VAR_SCOPE_REF` for the `var:` prefix form.**

### 7i. `event_target:` and `global_event_target:` prefixes

From `STLEvents.fs` and `STLScopes.fs`: strings beginning with `event_target:` or `global_event_target:` are event target references, not scope traversals. They are validated by matching the suffix against `save_event_target_as` definitions. **These are distinct token types from regular identifiers.**

### 7j. The `hidden:` prefix on effects

From `STLScopes.fs`: scope keys can be prefixed with `"hidden:"` which is stripped before scope resolution. This is an engine-level hint that the effect is hidden from tooltips. `hidden:add_modifier` is the same scope-resolution as `add_modifier`.

### 7k. Scope-targeting syntax with `@` in variable names

From issue #64: `ai_weight@ROOT` is a targeted variable — the `@` separates the variable base name from the scope it's targeted to. This is entirely distinct from `@ai_weight` (scripted variable reference). The two `@` syntaxes coexist and cannot be distinguished without knowing whether `@` is the first character or is internal to the name.

---

## 8. CWT Config Format Relevance

**Recommendation: Yes, study the CWT format. No, do not implement it wholesale now.**

### What CWT is

CWT is a **schema language** for Clausewitz script files. It describes what valid syntax looks like, using the same key-value syntax as Clausewitz itself. Key constructs:

```
# From effects.cwt:
alias[effect:create_starbase] = {
    ## cardinality = 0..1
    owner = scope_group[target_country]
    ## cardinality = 1..1
    size = <ship_size.starbase>
    ## push_scope = starbase
    effect = {
        alias_name[effect] = alias_match_left[effect]
    }
}
```

CWT metadata annotations (in `##` comments) specify:
- `cardinality = min..max` — how many times a field may appear
- `push_scope = TYPE` — this block changes the active scope to TYPE
- `replace_scope = { this = X root = Y }` — override scope context
- `severity = warning/error` — what to emit on violation

CWT field types include: `bool`, `int`, `float`, `scalar`, `localisation`, `filepath`, `scope[type]`, `scope_group[group]`, `<type_name>` (game object reference), `enum[name]`, `value_field`, `variable_field`, `alias_name[X]`, `alias_match_left[X]`.

### Why it matters for us

Our compiler already plans a schema layer (the `Rules/` layer concept). The CWT format is a proven design for this. We should adopt its vocabulary:
- `alias[class:name]` for declaring polymorphic groups (all effects, all triggers)
- `push_scope` and `replace_scope` annotations as the mechanism for tracking scope changes through blocks
- `cardinality` as the field occurrence model
- `scope_group[name]` for sets of valid scope types

### Why not wholesale

1. cwtools-stellaris-config is maintained separately and trails the game by months on new content. We cannot rely on it being complete for modded content.
2. The CWT parser in cwtools is over 66 KB (RulesParser.fs) and handles 40+ field type patterns — significant complexity.
3. For our use case (conflict resolution + patch generation), we primarily need: scope tracking, variable resolution, and type identity — not full field-level cardinality checking.

**Concrete recommendation**: Design our schema layer using CWT-compatible vocabulary (`push_scope`, `cardinality`, `alias`, `scope_group`) so we can eventually import cwtools-stellaris-config files directly. But implement only the subset we need for batch G3/G4, and extend incrementally.

---

## 9. Summary: What We Copy, What We Avoid, What We Add

| Aspect | cwtools approach | Our approach |
|--------|-----------------|--------------|
| Token stream | None (direct to AST) | Primary IR: emit token stream |
| `@` prefix | In idCharArray; semantics at validation layer | Distinct tokens: `VAR_DECL_AT`, `VAR_REF_AT`, `TARGETED_VAR` |
| `yes`/`no` | Explicit `Bool` case with lookahead guard | `BOOL_LIT` token, same guard |
| Operators | 8-op enum, correct | Copy exactly |
| `@[...]` | Opaque string, fails on `()` and `-` | `ARITH_EXPR` token with proper inner grammar |
| Scope chains | Opaque string, split at semantic layer | Token-level: emit `SCOPE_NAV` chain or decompose |
| `FROM.FROM` | Fails (issue #73) | Normalize dotted form to canonical at parse time |
| `[[param]body]` | Unsupported (issue #57) | `CONDITIONAL_PARAM_BLOCK` token |
| `$PARAM$` in names | Survives to expansion phase | Same: emit as-is, expand in post-lex phase |
| `event_target:` | String prefix check at semantic layer | `EVENT_TARGET_REF` token |
| `var:` prefix | String prefix check at semantic layer | `VAR_SCOPE_REF` token |
| `hidden:` prefix | Stripped at semantic layer | Pass through; strip in semantic phase |
| RGB/HSV colors | Int-only (floats break, issue #53) | Allow int OR float in color component |
| Error recovery | Fail-fast, empty result | Emit `ERROR` token, continue |
| Encoding | Fixed 1252 with retry | Auto-detect UTF-8 BOM, fall back to 1252 |
| inline_script | Iterative expansion, max 5, `$P$` substitution | Same model, copy exactly |
| Scope context | `{ Root; From: list; Scopes: list }` stack | Copy this structure |
| String interning | `StringTokens` (lower, normal, quoted) | Copy: intern with case-fold + quoted flag |
| Position | Bit-packed range (file, line, col) | Copy: `Span { file_id, start, end }` |
| Comments | `CommentStatement` in Statement list | Trivia tokens attached to following node |
| CWT schema | 40+ field types, alias system | Adopt vocabulary, implement subset |

---

*End of review. Next step: use these findings to extend the token taxonomy in `docs/megapatch/design/2026-04-16-clausewitz-grammar.md` with the edge-case token kinds identified here (TARGETED_VAR, CONDITIONAL_PARAM_BLOCK, VAR_SCOPE_REF, EVENT_TARGET_REF, ARITH_EXPR inner grammar, `?=` operator, `type`/`types` keywords).*
