# Clausewitz Parsers Research — Cross-Implementation Analysis

**Date:** 2026-04-17
**Reviewer:** automated agent research
**Purpose:** Extract implementation lessons for the stellar-heritage Clause lexer and parser
**Feeds into:** G2 (lexer implementation), grammar spec `docs/megapatch/design/2026-04-16-clausewitz-grammar.md`

Prior art already reviewed and available:
- `cwtools-formalization-review.md` — F#/FParsec parser-combinator (reference baseline)

Repos studied:
1. `https://github.com/mmyers/eug` — Java, JFlex-generated DFA scanner + hand-written recursive descent parser
2. `https://github.com/davidvontamar/clausewitz-interpreter` — C#/.NET, hand-written character-by-character tokenizer
3. `https://github.com/iTitus/PDXTools` — Java (Kotlin+Java), hand-written tokenizer + recursive descent
4. `https://github.com/nickbabcock/jomini` — Rust/WASM compiled to JavaScript; tape-based parser
5. `https://github.com/LokiSharp/klausewitz` — Kotlin/Java, ANTLR4 grammar-based (mislabeled TypeScript in GitHub)
6. `https://github.com/zipsegv/pdxparser` — D language, single-file hand-written recursive descent

---

## 1. Executive Summary

The state of existing Clausewitz parsers breaks into two clear tiers.

**Tier 1 — production quality**: jomini (Rust/WASM) stands alone. It is the only parser with explicit binary format support, a tape-based IR (not just a tree), formal encoding selection, >200 MB/s throughput, and a test suite with real-world save-file corpus. It powers the pdx.tools EU4 save analyzer and has been battle-tested at scale. PDXTools (Java) is a distant second: meticulous about encoding details (explicit UTF-8 BOM handling), rich value types (percent, color, date, null), and has a patch-database concept implying some understanding of multi-source merging.

**Tier 2 — scripting/utility quality**: eug (Java/JFlex), clausewitz-interpreter (C#), pdxparser (D), and klausewitz (ANTLR4/Kotlin). These are capable enough for simple file reading but make no attempt at save-file binary formats, have minimal or no error recovery, and are generally limited in token expressiveness (most collapse @-prefix variables, dotted chains, and operators-in-key-position into generic string tokens).

**Biggest surprises**:

1. **klausewitz is Kotlin/Java with ANTLR4 — not TypeScript**. The GitHub language stats are misleading because the grammar file is tiny but the repo is 100% Kotlin/Java. The grammar (`Clausewitz.g4`) is remarkably thin (34 lines) and includes `<>` as a valid operator — which is not real Clausewitz syntax. It also misses `!=`, `==`, and `?=`.

2. **jomini's tape model is architecturally closest to what we want** but is oriented toward read-only deserialization, not round-trippable transformation. It does not preserve trivia (whitespace, comments). Our use case (patch compilation, round-tripping) needs something beyond what jomini provides.

3. **All six parsers treat `@`-prefixed tokens as opaque strings** (or lump them into the identifier character set), reproducing the exact cwtools bug we are explicitly fixing. This is a universal blind spot.

4. **None of the six implement `[[PARAM]body]` conditional blocks**. Only jomini explicitly handles `$PARAM$` parameter tokens (calling them `Parameter` and `UndefinedParameter`). Everyone else ignores the entire inline_script parameter system.

5. **Error recovery is primitive everywhere except eug**. eug has the most mature approach: a `ParserSettings.tryToRecover` flag that, when set, allows the parser to produce a partial tree rather than throwing. It also has a `NEWLINE` token type (unique among the six) that enables comment-tracking across blank lines — useful for our own round-trip comment preservation.

6. **PDXTools explicitly has inline-math TODO comments** ("TODO: correctly tokenize @[...]") and disabled math operators in the tokenizer (`isMathOperator` always returns false). They knew the problem exists and punted.

7. **clausewitz-interpreter (C#) is the only parser that explicitly throws on `==`** (`throw new SyntaxException("Unexpected token '=='.")`), treating `==` as an error rather than an operator. This is technically wrong for modern Stellaris; `==` is a valid comparison operator.

8. **PDXTools handles comma-separated list values** (`a = { 1, 2, 3 }` — comma as list separator). None of the other parsers do. This syntax appears in some Paradox games and in some define files.

---

## 2. Per-Parser Deep Dives

### Parser 1: eug (Java / JFlex DFA scanner)

**Repository:** `https://github.com/mmyers/eug`
**Key files:** `eugFile/src/eug/parser/EUGScanner.java`, `TokenType.java`, `CWordFile.java`, `ParserSettings.java`

#### Language and approach
Java. Two-phase: a **JFlex-generated DFA scanner** (`EUGScanner`) handles tokenization, then a **hand-written recursive descent parser** (`CWordFile`) builds the AST. The scanner is generated from a `.flex` grammar (not checked in, only the generated Java). The DFA is baked as compressed packed tables. This is the most "classically correct" scanner architecture of the six — separate lexer/parser phases with a formal grammar for the lexer.

#### Token model
`TokenType` enum has exactly 9 kinds:
- `EOF`, `ULSTRING` (unquoted string), `DLSTRING` (double-quoted string), `EQUALS`, `LBRACE`, `RBRACE`, `COMMENT`, `IDENT`, `NEWLINE`

`IDENT` is a special token for the `key =` pattern — the scanner uses a two-state DFA (`YYINITIAL` → `HAS_IDENT`) where after seeing `word =` it emits `IDENT` containing both the key and the `=` operator in a single token (extracted by `getIdent()` which scans for the non-operator part). **This is architecturally unusual**: the scanner has absorbed operator detection into a stateful key-recognizer, not a separate operator token. This means `IDENT` is not a pure token type but a combined "key-plus-operator" unit.

Critically: **the scanner has only `EQUALS` as an operator token**. Comparison operators (`>`, `<`, `>=`, `<=`, `!=`) are not in `TokenType`. They are treated as unquoted strings if they appear at all. This is fine for save files (which rarely have comparison operators in key position) but wrong for script files.

`NEWLINE` is a unique contribution: eug emits newline tokens rather than discarding them. This enables the comment-attachment logic in `CWordFile` — comments separated by 2+ newlines are associated with the root node, while comments attached on the same or adjacent line associate with the next construct. This is the most mature comment-handling of all six parsers reviewed.

#### AST shape
`GenericObject` (keyed container with `name`, parent pointer, children), `GenericList` (ordered list of strings), `ObjectVariable` (name=value leaf). The tree is mutable — nodes can be added, removed, modified. The root is a synthetic nameless `GenericObject`. The distinction between a `GenericList` and a `GenericObject` is determined dynamically during parsing: the parser does a one-token lookahead after `{` to decide whether the contents are pure-value (list) or key-value (object). If the list turns out to contain operators mid-stream, it is retroactively converted to an object.

No typed value nodes: everything is stored as strings. No `INT`, `FLOAT`, `BOOL` tokens; those are up to consumers to interpret.

#### Encoding handling
`InputStreamReader` with `"Cp1252"` (Windows-1252) hardcoded in `openInStream`. No UTF-8 detection, no BOM handling. The scanner itself uses Java's character-based Reader interface, so any encoding the Reader is configured for is passed through. But `openInStream` always passes Cp1252. String streams (`loadFromString`) implicitly use platform encoding.

#### Error recovery
Configurable via `ParserSettings`. `tryToRecover=true` (the default) catches `ParserException` and returns a partial tree. `warningsAreErrors=false` (default) means unexpected tokens emit warnings to stderr but do not abort. The `warn()` method logs and optionally throws. The parser can recover from unmatched `}` (pops to root), unexpected single tokens (creates a pseudo-node with value "1" if `allowSingleTokens=true`), and mixed list/object content (converts list to object).

`allowLists=true` enables the list/object heuristic. `allowSingleTokens=false` (default) means bare unquoted values in non-list context cause warnings. These settings make the parser adaptable to different game dialects.

#### Save vs. script files
Same parser for both. Zip/PK detection in `openInStream`: if the file starts with "PK", it opens it as a zip and reads all entries in sequence via `SequenceInputStream`. Modern Paradox save files are zipped; eug handles this transparently.

#### Edge cases solved
- List vs. object ambiguity: one-token lookahead, retroactive conversion.
- Inline comments: `readInlineComment()` peeks ahead after each node close.
- Zip save files: transparent unzip.
- Multi-entry zip saves: concatenates all entries (handles the three-entry modern format).
- Comments attached to constructs: NEWLINE-counting for multi-blank-line separation.

#### Performance
DFA scanner over a `BufferedReader` with 64 KB buffer (or `Math.min(65536, file_length)`). No interning. Pure Java object allocation per node. The tree is mutable and linear in memory (no deduplication). For large save files, this is O(n) time but O(n) memory with no reuse. Timing info is optionally printed (`printTimingInfo`).

#### Known limitations
- Only `EQUALS` as operator. Comparison operators are not parsed.
- No type information (everything is a string).
- No support for `@variables`, `$params$`, `event_target:`, `var:`, `@[expr]`.
- Hardcoded Windows-1252.
- `IDENT` token fuses key and `=` into one unit (prevents clean separation of key from operator).

#### Uniquely useful
- `NEWLINE` token + blank-line-counting comment attachment: the most principled comment association logic of any parser reviewed. Our trivia model should replicate this boundary-detection behavior.
- `ParserSettings` as a configuration object: clean API for controlling strictness, comment handling, and list-vs-object heuristics. Our `ParseOptions` should offer comparable knobs.
- Zip/PK detection: useful for handling Stellaris multiplayer saves (which are zipped).

---

### Parser 2: clausewitz-interpreter (C# / hand-written character tokenizer)

**Repository:** `https://github.com/davidvontamar/clausewitz-interpreter`
**Key files:** `Tamar.Clausewitz/Interpreter.cs`, `Constructs/Token.cs`, `Constructs/Binding.cs`, `Constructs/Clause.cs`, `Constructs/Operators.cs`, `Constructs/Pragma.cs`

#### Language and approach
C#/.NET. **Single-pass hand-written character-by-character tokenizer** (`Interpreter.Tokenize`) followed by a **single-pass token-stream parser** (`Interpreter.InterpretText`). No separate lexer phase — tokenization and interpretation are both manual. The tokenizer uses a `goto concat` pattern that resembles Duff's device: control flow jumps to character-concatenation code on the "normal character" path. Two passes but both are linear.

This is one of two parsers (with eug) that has an explicit separate tokenization function returning a list of `(string token, int line)` pairs before the interpretation loop runs. That two-phase structure means the raw tokens are accessible independently of the AST, which is architecturally cleaner than cwtools.

#### Token model
The tokenizer produces raw `(string, int)` pairs — no dedicated enum. However, the **operator set** is defined as `Operators` enum with 7 members:
```csharp
Equals, NotEquals, QuestionEquals, GreaterThan, LessThan, GreaterThanOrEqual, LessThanOrEqual
```
This is correct except for one critical bug: the `==` operator causes a `SyntaxException("Unexpected token '=='.")`. The parser explicitly rejects `==`. This is wrong for modern Stellaris. The comment says this is intentional design, not an oversight.

`?=` (`QuestionEquals`) is recognized — only the second parser after cwtools in this survey to include it.

`ValuePattern = @"[a-zA-Z0-9_.:""-]+"`: this regex allows `:`, `@`, `-`, `.`, `"` as identifier characters. The `@` is embedded in the valid character set, exactly reproducing the cwtools issue #64 where `@var` and `name@SCOPE` are indistinguishable.

#### AST shape
Three construct types inheriting from a base `Construct`:
- `Clause` — named or unnamed block with `Operator`, child `Constructs`, `Bindings`, `Clauses`, `Tokens`, plus `Comments` and `EndComments`
- `Binding` — single key-op-value leaf (all strings). Op is `Operators` enum.
- `Token` — a standalone value with no operator (bare value in list context)

`Pragma` is a unique addition: comment-embedded control directives (`# @indent`, `# @unindent`, etc.) that affect how the translator renders constructs back to text. This is a rudimentary configuration-via-comments system for the serializer.

The `Clause.IsIndented` property uses a heuristic: blocks with >20 items and only `Token` children (bare value lists) are formatted inline rather than one-per-line. This is a practical serialization heuristic for lists like `{ 1 2 3 4 ... }`.

#### Encoding handling
No encoding logic whatsoever. The `Tokenize` method takes a `string` — whoever calls it is responsible for encoding. No UTF-8 BOM detection, no Windows-1252 handling. Encoding is entirely deferred to the consumer.

#### Error recovery
Fail-fast. `SyntaxException` is thrown on:
- Missing opening `{` for a clause
- Missing closing `}` at EOF
- Invalid name or value in binding
- `==` operator
- Standalone `!` or `?` characters

No try-to-recover mode. Partial results not returned.

#### Save vs. script files
Script files only (implied by the tool name "interpreter"). No save-file binary support. No zip handling.

#### Edge cases solved
- Inline vs. standalone comments: a `lineNumber == lineOfPreviousToken` test attaches comments to the preceding construct if on the same line, to the next construct if on a preceding line.
- Leading file comments vs. construct comments: a reversed scan of the comment buffer determines whether a comment block is a "file header" (separated from the first construct by a blank line) or attached to the first construct.
- Bare tokens inside clauses: `Token` construct handles standalone values.
- Quoted strings with spaces: preserved through tokenization, unquoted during AST construction.

#### Performance
O(n) tokenization, O(n) interpretation. All strings are plain .NET `string` (no interning). No position information beyond line number. Memory footprint is the full AST tree. For large files this is straightforward but not optimized.

#### Known limitations
- `==` explicitly rejected as a syntax error (wrong for Stellaris).
- `@` embedded in identifier charset, no variable distinction.
- No encoding support.
- No `$PARAM$`, `@[expr]`, `[[PARAM]body]`.
- No comparison operators in key position (e.g., `> = 5` is not handled).
- `ValuePattern` allows only `[a-zA-Z0-9_.:""-]+` — misses `@`, `%`, `!`, numbers-at-start.

#### Uniquely useful
- `Pragma` system: the idea of embedding control annotations in comments is interesting but only affects the serializer, not the parser. Our trivia attachment system can represent similar intent without embedding in comment text.
- The `Interpreter.IsValidToken` function uses `Regex.IsMatch(token, @"\d")` to allow pure-digit tokens — a simple approach to numeric validation.
- Line-number tracking on every token: the `(token, line)` tuple carries line info through to the AST, which is used for error messages.

---

### Parser 3: PDXTools (Java / Kotlin + Java hand-written tokenizer)

**Repository:** `https://github.com/iTitus/PDXTools`
**Key files:** `pdx-tools/src/main/java/io/github/ititus/pdx/pdxscript/PdxScriptParser.java`, `PdxRelation.java`, `PdxConstants.java`, `PdxColor.java`

#### Language and approach
Java with Kotlin higher-level tooling. **Hand-written tokenizer** (inner `Iterator<String>` in `PdxScriptParser.tokenize`) followed by **hand-written recursive descent parser** (`PdxScriptParser.parse`). The tokenizer uses a state-machine approach with `MutableBoolean` state flags for: `openQuotes`, `comment`, `token`, `separator`, `relation`, `mathOperator`. This is cleaner than the character-switch approach but effectively the same algorithm. The parser then consumes the token iterator via an `IteratorBuffer<String>` with 2-token lookahead.

One significant feature: the parser accepts a `PdxPatchDatabase` for patching files at load time. This is a rudimentary version of the conflict-resolution problem we're solving. The `PdxPatch` and `PdxPatchDatabase` classes exist in the source.

#### Token model
The tokenizer emits raw strings, not typed tokens. Type information is resolved at parse time. The PdxRelation enum has 6 operators: `EQUALS, LESS_THAN, GREATER_THAN, NOT_EQUALS, LESS_THAN_OR_EQUALS, GREATER_THAN_OR_EQUALS`. Missing: `==` and `?=`.

Math operators (`+`, `-`, `*`, `/`) are defined in `PdxConstants` but **deliberately disabled** in `isMathOperator()` which always returns `false` with a `// FIXME` comment. There is a TODO comment in the parser: `"TODO: correctly tokenize @[...]"` and `"TODO: Fix tokenizer splitting raw tokens with math symbols in it"`.

PDXTools is uniquely aware of **comma-separated list values**: `a, b, c` as a list separator (not just whitespace). The `COMMA` constant and comma-handling code in the parser handles `value = { 1, 2, 3 }` forms, which appear in some Paradox define files and CK3 DNA sequences.

UTF-8 BOM is explicitly stripped: `if (c == UTF_8_BOM) { continue; }` in the tokenizer.

#### AST shape
4 node types: `PdxScriptObject` (key-value map), `PdxScriptList` (ordered value list), `PdxScriptValue` (scalar), plus a color type `PdxColor`. All implement `IPdxScript`.

`PdxScriptList.Mode` distinguishes between `COMMA`-separated and `SPACE`-separated lists — a nuance no other parser captures.

Value types in `PdxScriptValue` include: `Boolean`, `Integer`, `Long`, `Double`, `LocalDate` (dates parsed via `DateTimeFormatter`), `PdxColor`, and `null` (for the `none` keyword). This is the richest type system of the six parsers. Percent values (`10%`) are automatically converted to `0.1` (fractional form).

`PdxRelation` is attached to each `IPdxScript` node, not stored on a parent — every value knows its own operator. This is semantically clean for comparison operators on values.

Variables (lines beginning with `@`) are collected during parsing into a `variables` map that is threaded through the recursive descent. When a value-position token begins with `@`, it is looked up in `variables` and the resolved value is substituted inline. This is **eager variable resolution at parse time** — inline substitution, no `VAR_REF` nodes in the output AST. If a variable is unresolved at parse time, parsing throws.

#### Encoding handling
Explicit UTF-8 BOM detection and stripping. Encoding is otherwise handled by the caller — the `IOUtil.getCharacterIterator` wrapper handles file I/O and encoding detection (not seen directly but referenced). `PdxConstants.UTF_8_BOM = '\uFEFF'`.

#### Error recovery
Hard throw via `RuntimeException` throughout the parser. No recovery mode. If a variable is unresolved, parsing fails immediately. If braces are unbalanced, parsing fails. No partial results.

#### Save vs. script files
Script files only from the evidence available. The `PdxPatch` system implies script-level patching. No binary save format support.

#### Edge cases solved
- Comma-separated lists: explicit `COMMA` token type and comma-handling in the parser.
- Hex color syntax: `PdxColor.fromRGBHex` handles `#RRGGBBAA` hex tokens before other type checks.
- Percent values: trailing `%` stripped and value divided by 100.
- `none` keyword maps to Java `null`.
- Date parsing with `LocalDate.parse` using a `DateTimeFormatter` for `u.MM.dd` format.
- UTF-8 BOM stripping.
- Variable resolution: `@var` is resolved at parse time.
- Patch database at parse time: modifies the character stream before tokenization.

#### Performance
The tokenizer is implemented as a lazy `Iterator<String>` (pull-based, not push-based). This means tokenization is demand-driven and the full token list is never materialized simultaneously. The parser uses `IteratorBuffer<String>` with 2-token lookahead. This is memory-efficient for large files.

Math operators are disabled in `isMathOperator` precisely because the TODO acknowledges that tokenizing them correctly requires handling `@[...]` blocks — they punted rather than doing it wrong.

#### Known limitations
- `==` and `?=` operators missing.
- Math expressions (`@[...]`) not tokenized — explicitly TODO'd.
- No `$PARAM$`, `[[PARAM]body]`, `event_target:`, `var:` prefix tokens.
- Eager variable resolution at parse time means the AST has no variable reference nodes; downstream consumers cannot distinguish "was this value a @var reference?" vs. "was it a literal?".
- Multiple files parsed into a single merged `PdxScriptObject` (key conflicts are silently overwritten in the builder unless `.addAll()` is used carefully).
- `isMathOperator` is hardcoded to `false` (FIXME comment present).

#### Uniquely useful
- Comma-separated list handling: our grammar spec currently has no provision for comma separators in list contexts. This is a syntax variant that exists in the wild.
- Rich value type system: the distinction between `Integer`/`Long`/`Double`/`LocalDate`/`PdxColor`/`null` is useful for downstream consumers. Our grammar should at minimum emit `DATE` tokens (not just `FLOAT` with 3 dots).
- `PdxScriptList.Mode` (COMMA vs. SPACE): tracking the list delimiter in the AST is useful for faithful round-tripping.
- `PdxPatchDatabase`: precedent for the idea of a patch/overlay database applied at parse time. We are implementing this in a more principled way, but the existence of this concept validates the approach.

---

### Parser 4: jomini (Rust/WASM / tape-based parser)

**Repository:** `https://github.com/nickbabcock/jomini`
**Key files:** `crate/src/lib.rs` (WASM wrapper), `src/jomini.ts` (TS API wrapper)

#### Language and approach
Rust compiled to WebAssembly, with a TypeScript wrapper for the JavaScript API. The core parser is entirely in Rust. The TypeScript side is thin binding glue. **Tape-based architecture**: the parser outputs a `TextTape` — a flat array of `TextToken` values representing the parse result, which is then traversed via `ObjectReader` / `ArrayReader` / `ScalarReader` cursor types without building a tree.

This is the only parser in the survey that uses a **tape model** (similar to what our grammar doc calls "token stream as primary IR"). The key difference: jomini's tape is output of parsing, not the parsing IR — it is already a resolved structural representation with matched `Array{end}` / `Object{end}` indices pre-computed.

#### Token model
`TextToken<'a>` enum (from docs.rs):
- `Array { end: usize, mixed: bool }` — start of array, with index of matching close
- `Object { end: usize, mixed: bool }` — start of object, with index of matching close
- `MixedContainer` — marks where a homogeneous container becomes heterogeneous (an array that also has keyed values, e.g., a block that has both bare values and `key = value` pairs)
- `Unquoted(&'a [u8])` — unquoted scalar; raw bytes, not yet decoded
- `Quoted(&'a [u8])` — quoted scalar; raw bytes
- `Parameter(&'a [u8])` — `[[var] code ]` positive conditional parameter
- `UndefinedParameter(&'a [u8])` — `[[!var] code ]` negative conditional parameter
- `Operator(Operator)` — non-equal operator (equal is implicit in key-value patterns)
- `End(usize)` — matched close of `Array` or `Object`, stores start index
- `Header(&'a [u8])` — the header token before `{`, e.g., `rgb` in `rgb { 100 200 50 }`

`Operator` enum (from write API): `>`, `>=`, `<`, `<=`, `=`. Missing `!=`, `==`, `?=`.

**Key insight**: `Array` and `Object` tokens store their `end` index (index into the tape of the matching `End` token). This means the tape can be traversed in O(1) to skip an entire nested block — no need to recurse or count braces. This is a powerful optimization for partial-document extraction.

**Mixed container**: a fundamental ambiguity in the Clausewitz format is that `{ a b c }` (bare value list) and `{ a = 1 b = 2 }` (key-value map) have identical open-brace tokens. If a block starts as a list but then gets a `key = value`, the `MixedContainer` token is inserted to mark the transition point. This is jomini's explicit handling of the Clausewitz list/object duality.

**`$PARAM$` tokens**: jomini is the only parser in this survey (other than our own design) that explicitly represents inline_script parameter tokens in the parse output. `Parameter` and `UndefinedParameter` carry the parameter body as raw bytes. The WASM wrapper converts them to JSON strings like `[var_name]` and `[!var_name]` respectively for JavaScript consumption.

**`Header` token**: the `rgb`/`hsv`/`LIST` prefix before `{ ... }` blocks is a first-class token. The writer API has `write_header` for these. This validates our grammar spec's `color-space tags` section (§3.7b) — headers are a recognized structural concept.

#### AST shape
No AST. The tape IS the representation. Consumers use cursor readers (`ObjectReader`, `ArrayReader`, `ScalarReader`, `ValueReader`) to traverse it. The WASM wrapper materializes JavaScript objects/arrays from the tape on demand. `Query.at(pointer)` navigates the tape using JSON-pointer-style strings without materializing the full tree.

For JSON output, jomini supports three `duplicateKeyMode` options: `group` (default: merge duplicate keys into arrays), `preserve` (keep all, last wins in JS), `key-value-pairs` (explicit array of `[key, value]` pairs). This is the most sophisticated duplicate-key handling of any parser reviewed.

#### Encoding handling
Explicit two-option model: `utf8` | `windows1252`. The API requires the caller to specify which encoding is used. Raw byte arrays are accepted directly — no forced string conversion. The Rust code uses `Utf8Encoding::new()` or `Windows1252Encoding::new()` passed to the tape readers. No auto-detection.

This is a deliberate design choice in the Rust crate: auto-detection is unreliable for small files. The caller knows the encoding context (vanilla files are 1252, mod files may be either) and should pass it explicitly. For our compiler we've chosen auto-detect (UTF-8 BOM → UTF-8 → 1252 fallback), which is strictly more permissive.

#### Error recovery
The Rust parser throws on unclosed quotes, unbalanced braces, and other structural errors. No partial-tree output. The WASM wrapper returns `null` from operators like `Parameter` and `UndefinedParameter` to the JS side when they are not expected in value position (see `entry_to_js` returning `JsValue::null()` for unexpected tokens). So there is some graceful handling at the JS API boundary, but the Rust core is fail-fast.

#### Save vs. script files
Unique in this survey: jomini explicitly supports **binary Clausewitz save formats** (EU4 ironman, CK3, etc.) via a separate `BinaryTape` and token resolver API. The binary format uses 16-bit integer token identifiers that map to string keys via a game-specific lookup table. This is entirely separate from the text parser. The `skip_header` function in `crate/src/lib.rs` handles the binary-format header that precedes text data in some saves.

#### Performance
>200 MB/s text parsing (per docs). The tape model allocates one `TextToken` per logical token — all `Unquoted`/`Quoted` values are byte slices into the original input buffer, zero-copy. The `Object{end}` / `Array{end}` end-index pre-computation means skipping blocks is O(1). The query API exploits this for 40x speedup on partial-document extraction.

95-99% of parse time (per the README) is materializing JavaScript objects from the tape, not the Rust parsing itself.

#### Known limitations
- `!=`, `==`, `?=` operators not in the operator enum. The write API exposes only `>`, `>=`, `<`, `<=`, `=`.
- No trivia (whitespace/comments) in the tape. Round-trip is lossy.
- No `@var` / `@[expr]` distinction. All unquoted scalars are `Unquoted(&[u8])` bytes — downstream consumers must check for `@` prefix manually.
- No scope chain awareness (`prev.from.owner` is an opaque `Unquoted` token).
- No encoding auto-detection (explicit choice required).

#### Uniquely useful
- **Tape model with pre-computed end indices**: the gold standard for performance. Our token stream can adopt the same `Block.items` precomputed boundary idea at the AST overlay level (block token carries index of matching close token).
- **`MixedContainer` for list/object duality**: an elegant solution to the fundamental ambiguity. Our `BareValue` node in the light AST overlay serves the same purpose.
- **`$PARAM$` as first-class tokens**: the `Parameter` / `UndefinedParameter` tokens confirm this is the right approach. Our `MACRO_PARAM` / `MACRO_PARAM_DEFAULT` token kinds are correct.
- **`Header` token for RGB/HSV/LIST prefixes**: confirms our §3.7b treatment.
- **Duplicate key modes**: the three-mode approach (`group`, `preserve`, `key-value-pairs`) is useful to expose from our compiler's emit phase for downstream consumers that need different semantics.
- **Binary format via separate BinaryTape**: if we ever need to parse save files directly (rather than just script files), the architecture is: same tape model, different token-ID resolver.

---

### Parser 5: klausewitz (Kotlin/Java / ANTLR4)

**Repository:** `https://github.com/LokiSharp/klausewitz`
**Key files:** `src/antlr/Clausewitz.g4`, `src/main/kotlin/moe/slk/clausewitz/parser/Parser.kt`, `src/main/kotlin/moe/slk/clausewitz/parser/TypeVisitors.kt`

#### Language and approach
Kotlin with Java interop. **ANTLR4 grammar-based parser**: the grammar file `Clausewitz.g4` is 34 lines and drives generated Java parser/lexer/visitor classes. Kotlin `TypeVisitors.kt` uses ANTLR's visitor pattern to materialize typed Kotlin objects from parse trees. The `parseClausewitzFile<T>` function uses Kotlin reified generics to map parse tree nodes to arbitrary Kotlin data classes.

The grammar is the entire specification — no hand-written lexer. ANTLR generates both the lexer (from token rules) and the parser (from grammar rules).

#### Token model
From `Clausewitz.g4`:
```antlr
INT: NEGATION?[0-9]+;
PCT: NEGATION?[0-9]+'%';
REAL: NEGATION?[0-9]+'.'[0-9]+;
DATE: [0-9]+'.'[0-9]+'.'[0-9]+;
STRING: STRING_DELIM (~('"' | '\\') | '\\' ('"' | '\\'))* STRING_DELIM;
SYMBOL: [A-Za-z0-9][:@A-Za-z_0-9.%-]*;
OPERATOR: '=' | '<>' | '>' | '<' | '<=' | '>=';
```

Critical observations:
1. **`<>` is listed as a valid operator** — this is NOT real Clausewitz syntax. It does not appear in any game file or cwtools source. This is an error in the grammar.
2. **`!=`, `==`, `?=` are absent** — significant gaps.
3. **`SYMBOL` includes `@` and `:` as valid identifier characters** (via `[:@A-Za-z_0-9.%-]*`) — reproducing the cwtools issue #64 and #73 blind spots.
4. `DATE` (`[0-9]+'.'[0-9]+'.'[0-9]+`) is a genuine first-class token — one of only three parsers to have explicit date token recognition (with PDXTools and jomini).
5. `PCT` (percent values) recognized as a first-class token — consistent with PDXTools.

The grammar has `map` (key-value block) and `array` (bare-value block) as distinct rules, which correctly models the Clausewitz duality — but at the grammar level, they cannot be mixed (the grammar requires `map` to have only `assignment`s and `array` to have only `value`s). Real Clausewitz blocks can contain both (`MixedContainer` in jomini terminology).

#### AST shape
No dedicated AST type — ANTLR generates a parse tree (`ConfigContext`, `AssignmentContext`, `MapContext`, `ArrayContext`, `ValueContext`, etc.). Visitors like `ObjectVisitor<T>` map this parse tree to Kotlin data classes using Kotlin reflection. The `resolveVisitor` function dispatches to typed visitors (`StringVisitor`, `IntegerVisitor`, `RealVisitor`, `BooleanVisitor`, `ListVisitor`, `MapVisitor`, `ObjectVisitor`) based on the Kotlin type of the target field.

This is a **type-directed deserialization** approach: the grammar output drives type inference based on the target Kotlin type. `parseClausewitzFile<Config>()` where `Config` is a Kotlin data class will try to map grammar fields to `Config`'s primary constructor parameters.

The `TypeVisitors` use `English.plural` (from the evo-inflector library) to match Clausewitz snake_case plural field names to Kotlin camelCase parameter names. E.g., a Clausewitz `units = { ... }` block matches a Kotlin parameter named `unit: List<Unit>`.

#### Encoding handling
None visible. The ANTLR `CharStreams.fromFileName(file)` uses the JVM's default charset. For Paradox files, this would typically be wrong unless the JVM is configured for Windows-1252.

#### Error recovery
ANTLR's built-in error recovery (DefaultErrorStrategy). ANTLR automatically inserts or deletes tokens to recover from simple errors. This is more robust than fail-fast but the recovery behavior can be unpredictable for deeply nested structures.

#### Save vs. script files
Script files only. HOI4-specific Kotlin code (`hoi4/`) uses the parser for HOI4 save/config files.

#### Edge cases solved
- `DATE` as a first-class grammar token (not conflated with REAL).
- `PCT` (percent values) as a grammar token.
- Reflection-based field mapping with pluralization.

#### Performance
ANTLR parsers have significant overhead (grammar compilation, ParseTree allocation, visitor dispatch). For a 200 KB script file this is acceptable. For multi-gigabyte save files it would be prohibitive. ANTLR also does not support streaming — the full file must be loaded before parsing.

#### Known limitations
- `<>` operator is wrong — not real Clausewitz syntax.
- Missing `!=`, `==`, `?=`.
- `SYMBOL` lumps all identifiers, `@vars`, dotted chains, `event_target:`, `var:` together.
- Mixed container (list + keyed values in same block) is not grammatically representable.
- No `$PARAM$`, `[[PARAM]body]`, `@[expr]`.
- ANTLR overhead; no streaming.
- No encoding handling.
- The repository description says "TypeScript Clausewitz parser" but the parser is entirely Kotlin/Java/ANTLR4 — misleading.

#### Uniquely useful
- `DATE` as a first-class grammar-level token (not just parsed by semantic lookahead): confirms our §3.3 `DATE` token is the right design.
- `PCT` for percent values: something our grammar spec does not currently have. See gaps section.
- ANTLR grammar as a formal specification: the G4 grammar, despite its bugs, is a useful normative document for what a minimal viable Clausewitz grammar looks like. The 34 lines confirm that the core grammar is small; the complexity is in the identifier character set and the operator set.
- Reflection-based field mapping: the TypeVisitors pattern (map parse tree to typed Kotlin data classes using Kotlin reflection) is one approach to a "typed deserialization" layer above our AST. We don't need this now but it's a valid future direction.

---

### Parser 6: pdxparser (D language / single-file hand-written recursive descent)

**Repository:** `https://github.com/zipsegv/pdxparser`
**Key files:** `source/pdxparser/package.d` (entire parser, 489 lines)

#### Language and approach
D language. **Single-file, single-function recursive descent parser** with no separate tokenization phase. The `parse(string script, int* l)` function takes a position pointer and consumes the source character by character. It uses a `seenSpace` flag to disambiguate whether a whitespace has terminated the current token. This is the most minimal implementation of the six — less than 500 lines total.

D's `Variant` type is used for dynamic typing without needing a tagged union.

#### Token model
No tokens. The parser operates directly on the source character stream. The `get()` inner function classifies a completed token by attempt order: try float (single dot in digit sequence), then int (all digits), then `yes`/`no` booleans, then fall through to string. This is exactly the cwtools/jomini backtracking pattern but without an intermediate token representation.

No operators other than `=`. The parser only handles `key = value` with `=`. It does not handle `>`, `<`, `>=`, etc. in any syntactic position.

#### AST shape
Three node types: `Block` (array of child `Node`s), `Assignment` (string key + `Variant` value), `Value` (standalone `Variant`). The `Variant` type can hold `float`, `int`, `bool`, `string`, or `Block`. The `equals(k, v)` method on `Node` makes it convenient to test key-value pairs directly.

The `NodeType` enum (`ASSIGNMENT`, `BLOCK`, `VALUE`) is used for dispatch. Indexing operators (`opIndex`) provide both positional and key-based access.

All nodes implement `Node` interface. Block is the standard container. Value is a standalone bare value. `toString()` is implemented on all nodes for round-tripping (though without trivia, the output will not preserve comments, whitespace, or blank lines).

#### Encoding handling
D's `parseFromFile` uses two approaches: first tries `readText(filename)` (UTF-8), on `UnicodeException` falls back to reading raw bytes and `transcode`-ing from `Latin1String`. This is the simplest auto-detect of the survey: UTF-8 first, ANSI (Latin-1 / Windows-1252) fallback. No BOM detection.

This is the approach our grammar spec uses as well (§4 encoding section), minus the BOM detection step.

#### Error recovery
Two specific errors thrown:
- `PDXParsingException("Unbalanced braces")` when position exceeds string length
- Implicitly: no check for unclosed strings (would access out-of-bounds in the `while(script[*l] != '"')` loop)

No partial results. No recovery mode. A TODO comment notes that `yes`/`no` inside quotes still becomes `bool` — an acknowledged bug (`value_` check: `"yes"` and `"no"` in quoted context should remain strings).

#### Save vs. script files
Script files only. No binary support. Uses `saveNodeToFile`/`saveNode` for round-trip output.

#### Edge cases solved
- UTF-8 + Latin-1 dual-encoding detection via exception: a practical approach that works without BOM.
- The `seenSpace` flag correctly handles `{ a b c }` — each space terminates the current token and `seenSpace` triggers a `Value` emission.
- Date values (e.g., `1443.3.4`) are correctly left as strings — the `number` detection loop checks for a single `.` but a date has two, so the `dot` variable hits twice and `number` becomes false. Dates fall through to string. This is correct behavior achieved by accident.

#### Performance
Character-by-character with a `string` buffer — D strings are immutable, so `value ~= c` and `buf ~= c` create new strings on every append (or D's compiler may optimize this to array growth). For large files this could be O(n²) in naive D compilers. The `appender!string` in `Block.toString` uses the efficient D `Appender` pattern but the parse path does not.

#### Known limitations
- `=` only. No comparison operators anywhere.
- No `@variables` (a `@` character in a key would be part of the `value` string, since the loop has no special-casing for `@`).
- No `$PARAM$`, `[[PARAM]body]`, `@[expr]`.
- `"yes"`/`"no"` inside quotes becomes `bool` — acknowledged bug.
- No comment preservation. Comments are consumed and discarded with `while(script[*l] != '\n') *l += 1`.
- No position information on nodes.
- `case '"':` loop does not handle escape sequences (a `\"` inside a string would terminate the string early).

#### Uniquely useful
- UTF-8 + Latin-1 fallback without BOM: the simplest working approach. Validate against our own BOM-first strategy.
- The `equals(k, v)` convenience method on `Node` is a useful API design for downstream consumers.
- The brevity (489 lines) demonstrates that a correct-enough parser for simple script files is small. The incremental complexity we're adding (tokens, trivia, variable kinds, arithmetic expressions, scope chains) is the difference between a scripting-tool parser and a compiler frontend.

---

## 3. Comparison Matrix

| Axis | cwtools | eug | clausewitz-interp | PDXTools | jomini | klausewitz | pdxparser |
|---|---|---|---|---|---|---|---|
| **Language** | F# (FParsec) | Java (JFlex) | C# | Java/Kotlin | Rust/WASM + TS | Kotlin/Java/ANTLR4 | D |
| **Approach** | Parser combinators (no token stream) | DFA scanner + RD parser | Hand-written char tokenizer | Hand-written state-machine tokenizer | Tape-based Rust parser | ANTLR4 grammar | Single-pass char RD |
| **Token stream emitted?** | No | Yes (9 types) | Yes (raw strings) | Yes (raw strings) | Tape (`TextToken`) | ANTLR parse tree | No |
| **Operator set** | 8 (`=><>=<=!=== ?=`) | `=` only | 7 (missing `==`) | 6 (missing `==` `?=`) | 5 (missing `!= == ?=`) | 6 (has `<>` WRONG; missing `!= == ?=`) | `=` only |
| **`@var` distinction** | None (`@` in idCharArray) | None | None | None | None (opaque Unquoted) | None (`@` in SYMBOL) | None |
| **`$PARAM$` tokens** | Handled at semantic layer | None | None | None | First-class (`Parameter`/`UndefinedParameter`) | None | None |
| **`[[PARAM]body]`** | Not supported (issue #57) | None | None | None | None | None | None |
| **`@[expr]` arithmetic** | Opaque string (issue #54) | None | None | TODO comment | None | None | None |
| **Date token** | String + semantic parse | String | String | `LocalDate` | `Unquoted` bytes (parsed later) | First-class `DATE` | String (accidental) |
| **Color (rgb/hsv)** | Floats broken (issue #53) | String | String | First-class `PdxColor` | First-class `Header` token | Not in grammar | String |
| **Percent values** | String | String | String | First-class, auto-converted | No | First-class `PCT` | String |
| **Comma list sep** | No | No | No | First-class | No | No | No |
| **Error recovery** | None (empty result) | Configurable (try-recover) | Fail-fast | Fail-fast | Fail-fast | ANTLR default recovery | Fail-fast |
| **Encoding** | Fixed 1252 | Fixed Cp1252 | Caller's responsibility | Explicit BOM strip | Explicit choice (utf8/1252) | JVM default | UTF-8 → Latin1 fallback |
| **Comments preserved** | `CommentStatement` list | Full (NEWLINE + blank-line logic) | Full (attached/leading/trailing) | Discarded | Discarded | Discarded | Discarded |
| **Save file support** | No | Zip unpack | No | No | Yes (binary + text) | No | No |
| **Trivia round-trip** | Optional | Full | Full | No | No | No | No |
| **Variable resolution** | Semantic layer | None | None | Eager at parse time | None | None | None |
| **Mixed container** | Not represented | List-vs-object heuristic | None | None | `MixedContainer` token | Grammar rule split | seenSpace heuristic |
| **Performance** | FParsec (good) | DFA + buffered I/O | O(n) C# | Lazy iterator, good | >200 MB/s | ANTLR overhead | O(n²) possible |
| **Binary format** | No | No | No | No | Yes | No | No |
| **Pragma / pragmatic control** | No | No | Yes (comment-embedded) | No | No | No | No |
| **Patch at load** | No | No | No | PdxPatchDatabase | No | No | No |

---

## 4. Lessons for Our Parser

### Copy directly (don't reinvent)

1. **Jomini's `TextToken` tape model with pre-computed end indices.** Our `Block` AST overlay should carry `close_token_index` in addition to `open_token`. This enables O(1) block skipping during patch application without re-traversing the tree.

2. **Jomini's `MixedContainer` concept.** Our `BareValue` in the light AST overlay handles the same case. Confirm our parser correctly builds `BareValue` nodes when a block starts as key-value but contains standalone values too.

3. **Jomini's `Parameter` / `UndefinedParameter` tokens.** Our `MACRO_PARAM` and `MACRO_PARAM_DEFAULT` names are correct. The `[[PARAM]` and `[[!PARAM]` forms map cleanly to `Parameter(positive)` vs. `UndefinedParameter(negative)`. The jomini naming is slightly confusing but the semantics are identical.

4. **Jomini's `Header` token for rgb/hsv/LIST.** Our grammar's §3.7b color-space treatment (IDENT followed by keyless Block) is validated. The `Header` name is better than our "color-space tag IDENT" description — we should rename internally to clarify.

5. **eug's `NEWLINE` token + newlines-since-comment blank-line counting.** Our trivia attachment (Roslyn-style, next-attached) is the right approach. But eug's blank-line boundary logic for deciding "this comment belongs to the previous section" vs. "this comment belongs to the next construct" is genuinely useful. We should add a `BLANK_LINE_BOUNDARY` concept: if there are 2+ `NEWLINE` trivia tokens in a row, the comment block may be treated as section-heading rather than preceding-node documentation.

6. **PDXTools's comma-separated list handling.** Add `COMMA` as a punctuation token (§3.7). This is present in define files and CK3 DNA syntax. Not adding it now means we silently misparse comma-separated values.

7. **PDXTools's `PdxRelation` attached per-value model.** Our `Assignment.op` token already does this. Confirmed correct.

8. **clausewitz-interpreter's line-number tracking on every token.** Our `Token.line` field already does this. Confirmed necessary.

9. **eug's `ParserSettings` as a configuration object.** Our `ParseOptions` in §4 should mirror the configurable flags: `allow_bare_values`, `try_to_recover`, `preserve_comments`, `strict_operators`.

10. **PDXTools's `skip_header` / UTF-8 BOM detection.** The explicit `UTF_8_BOM` constant and skip is correct. Our auto-detect (§4) does this.

### Avoid (specific patterns proven fragile)

1. **Fusing key and `=` into a single `IDENT` token (eug).** The `HAS_IDENT` two-state DFA produces a token that contains both the identifier name and the `=` sign. This prevents clean key-operator separation and makes it impossible to emit the operator as a separate token. Our lexer must emit `IDENT` and `EQUALS` as separate tokens always.

2. **Eager variable resolution at parse time (PDXTools).** Resolving `@var` to its literal value during parsing means the AST contains no variable reference nodes. Downstream consumers cannot trace "where did this value come from?" Our design (`VAR_DECL_AT` / `VAR_REF_AT` as distinct token kinds) preserves variable structure through the AST. Resolve in a dedicated `GlobalVariableLoader` pass, not during parsing.

3. **`<>` as a valid operator (klausewitz).** This is not Clausewitz syntax. Do not include it.

4. **Discarding trivia (jomini, PDXTools, klausewitz, pdxparser).** Our use case (patch files that must round-trip faithfully) makes trivia preservation mandatory. Jomini's architecture is right for its use case (read-only analytics) but wrong for ours.

5. **`@` in the identifier character set without further distinction (cwtools, clausewitz-interpreter, klausewitz).** This is the universal blind spot. Every parser that embeds `@` in the identifier class fails to distinguish `@var` (scripted variable), `name@SCOPE` (targeted variable), and `@[expr]` (inline arithmetic). Our three-kind approach (`VAR_AT`, `TARGETED_VAR`, `INLINE_EXPR_OPEN`) is the correct fix.

6. **Treating `==` as invalid (clausewitz-interpreter).** Modern Stellaris uses `==` for equality comparison in switch triggers and conditional contexts. Our `OP_EQ_EQ` token is correct.

7. **Failing the entire parse on a single error (almost everyone).** Our `INVALID` token + continue policy is strictly better. eug is the only parser to have a real recovery mode; its `tryToRecover` and `warningsAreErrors` flags are worth modeling.

8. **String-based token lists without type information (clausewitz-interpreter, PDXTools raw tokenizer).** Raw `(string, int)` pairs require every consumer to re-classify token types. Our typed `TokenKind` enum enforces classification once at the lexer boundary.

### Add (missing from ALL existing parsers)

1. **`[[PARAM]body]` conditional parameter blocks.** No parser has this. Our `CONDITIONAL_PARAM_OPEN_POS` / `CONDITIONAL_PARAM_OPEN_NEG` / `CONDITIONAL_PARAM_CLOSE` token kinds are unique. The jomini `Parameter` / `UndefinedParameter` tokens are the closest — they handle `[[var]` as a key position, but we need to handle the full block structure.

2. **Scope chain decomposition.** No parser decomposes `prev.from.owner` into a chain of scope navigation operations at the lexer level. Most treat it as an opaque string. Our `DOT` token + scope chain analysis in the semantic layer is the right fix. Validated by cwtools issue #73.

3. **`TARGETED_VAR` as a distinct token.** `ai_weight@ROOT` is not handled by any parser — they all lump it into the identifier character set or reject it. Our `TARGETED_VAR` token type is genuinely novel.

4. **Inline arithmetic inner grammar (`@[...]`).** Every parser either rejects it (cwtools, klausewitz) or has a TODO (PDXTools). Our `ARITH_EXPR` token with inner grammar (§3.11) is the only correct approach.

5. **`MACRO_PARAM_DEFAULT` (`$NAME|default$` form).** Even jomini only has `Parameter` / `UndefinedParameter` for the `[[PARAM]` form; it does not expose `$NAME|default$` default values as distinct tokens. Our `MACRO_PARAM_DEFAULT` is necessary for faithful inline_script compilation.

6. **Scope-keyword token classification.** None of the parsers distinguishes `SCOPE_THIS`, `SCOPE_ROOT`, `SCOPE_PREV`, `SCOPE_FROM` from general identifiers. Our `SCOPE_*` reserved table is uniquely useful for scope-aware analysis.

7. **Trivia-as-leading-attachment with round-trip guarantee.** Only eug and clausewitz-interpreter preserve comments at all. Neither has the Roslyn-style trivia model where comments are attached to tokens, providing lossless reconstruction. We are unique in this.

8. **`OP_DEFAULT` (`?=`).** Only cwtools and clausewitz-interpreter include `?=`. Others miss it. We have it in §3.2.

### Common challenges across multiple implementations

1. **The list/object duality** (`{ a b c }` vs. `{ a = 1 b = 2 }`): eug uses a heuristic lookahead, jomini uses `MixedContainer`, PDXTools uses context-sensitive parsing, klausewitz splits into separate grammar rules. This is the single hardest problem in Clausewitz parsing. Our `BareValue` / `Assignment` distinction in the `BlockItem` union is the right model.

2. **Date vs. float ambiguity** (`1.5` vs. `1444.11.11`): two dots → date, one dot → float. Most parsers handle this with try-parse fallback. klausewitz handles it with explicit grammar rules. Our `DATE` token (§3.3) with "two dots in digit-only string" rule handles it at lex time.

3. **`@` prefix ambiguity**: universally mishandled. Every parser either (a) includes `@` in identifier chars, (b) ignores `@` entirely, or (c) handles only `@var` at the semantic layer. The three-way distinction (`@var` prefix, `name@SCOPE` internal, `@[expr]` arithmetic) is unique to our design.

4. **Encoding diversity**: vanilla = Windows-1252, modern mods = UTF-8. The universal safe choice is auto-detect; everyone who hard-codes 1252 will break on some mod files.

5. **Comment attachment semantics**: where does a comment between two constructs belong? There is no consensus. eug's blank-line-counting approach is the most principled. Our next-attached trivia model is correct for a compiler (the comment "belongs to" the thing it precedes) but we should expose the blank-line boundary information for tools that need section-comment semantics.

---

## 5. Gaps in Our Grammar Spec

These are syntax features present in existing parsers that our spec does not currently cover.

### G5.1 — Comma as list separator

PDXTools has explicit `COMMA` token handling and `PdxScriptList.Mode.COMMA`. This syntax appears in CK3 DNA strings, `00_defines.txt` math expressions, and CK2/HOI4 files. Our grammar spec (§3.7 Punctuation) does not include `COMMA`. Add:

```
COMMA | `,` | List separator in comma-delimited blocks. Rare in Stellaris but present in CK3 defines and trait DNA values.
```

Add to §3.7 with a note: "Stellaris-rare; present in some defines files. Emit as `COMMA` token; semantic pass decides whether a comma-containing block is valid."

### G5.2 — Percent values

klausewitz has `PCT: NEGATION?[0-9]+'%'` and PDXTools handles `%`-suffixed tokens as auto-converted rationals. Our grammar spec has `%` in the `SYMBOL` char set implicitly but no explicit `PERCENT` or `FLOAT_PCT` token.

This is relevant for some Stellaris defines. Add a note to §3.3 Literals:

```
PERCENT | `10%`, `-5%` | A numeric value followed by `%`. Lexed as a distinct token kind. Value is the numeric portion; semantic pass may auto-convert to decimal (e.g., `10%` → `0.1`) or leave as raw percent for downstream handling.
```

### G5.3 — Zip/PK save-file header

eug detects `PK` magic bytes and unwraps zip saves. Our grammar spec is for text files; this is out of scope for the lexer. But add a note to §4 (Lexer contract):

```
Input encoding note: if the first two bytes are 'P' and 'K', the input is a zip-compressed save file. The lexer does NOT handle decompression; callers must unwrap save files before invoking lex(). A dedicated SaveFileLoader handles zip detection and feeds uncompressed text to lex().
```

### G5.4 — `none` as distinct null token

PDXTools has a dedicated `none` → `null` mapping (separate from `no`). Our grammar spec (§3.3) has `NULL = none` listed. This is correct. Confirm `NULL` is in `RESERVED_TABLE` and is case-insensitive.

### G5.5 — `hidden:` prefix on scope identifiers

cwtools strips `"hidden:"` from effect names before scope resolution. This is a prefix that can appear on effects like `hidden:add_modifier`. Our grammar spec (§3.6) does not mention it. The lexer should treat `hidden:foo` as either:
- A single `IDENT` token (if `:` is in identifier chars — but we explicitly said colon is NOT standalone), OR
- A `HIDDEN_SCOPE_PREFIX` token.

The cwtools review (§7j) documents this. Given that we made `COLON` non-standalone and absorbed it into `EVENT_TARGET_REF` / `VAR_SCOPE_REF` prefix tokens (§3.7), we need a parallel treatment for `hidden:`. Add:

```
HIDDEN_PREFIX | `hidden:foo` | An effect prefixed with `hidden:` — engine hint that the effect is hidden from tooltips. Lexed as a single HIDDEN_PREFIX token where `.text` is the full `hidden:effect_name` and `.suffix` is the effect name alone. Semantically equivalent to the effect without the prefix.
```

---

## 6. Contradictions with Our Grammar Spec

### C6.1 — `<>` operator listed in klausewitz grammar

klausewitz `Clausewitz.g4` includes `<>` as a valid `OPERATOR`. Our grammar spec §3.2 explicitly states "Verified absent (do NOT add): `<>` — appears only in comments / docstrings, not a real operator." We are correct; klausewitz is wrong. No action needed.

### C6.2 — `?=` operator: clausewitz-interpreter throws on standalone `?`

clausewitz-interpreter throws `SyntaxException` on a bare `?` character that is not followed by `=`. Our spec has `OP_DEFAULT = ?=` (§3.2). This is correct. The interpreter's behavior is a bug in the interpreter. No action needed to our spec.

### C6.3 — PDXTools treats `@` as `VARIABLE_PREFIX` but resolves eagerly

PDXTools has `char VARIABLE_PREFIX = '@'` and resolves variable references to literal values during parsing. Our spec treats `@`-prefixed tokens as `VAR_DECL_AT` or `VAR_REF_AT` and defers resolution to the `GlobalVariableLoader` pass (G4). This is the correct design for a compiler. The PDXTools approach (eager resolution) is fine for a read-only tool but loses structural information. Our approach is intentionally different and superior for our use case.

### C6.4 — jomini's Operator enum missing `!=`, `==`, `?=`

Jomini's write API exposes only `>`, `>=`, `<`, `<=`, `=`. This is a limitation of jomini's write side (it was designed for EU4/CK3 save files, which rarely use `!=`/`==`/`?=`). Our spec includes all 8 operators (§3.2). We are correct; jomini is incomplete for Stellaris. No action needed.

### C6.5 — Grammar spec `IDENT` char set omits some valid characters

Our grammar spec §3.1 defines `IDENT` as `[A-Za-z_][A-Za-z_0-9_]*`. The cwtools `idCharArray` includes: `letters, digits, _, :, @, ., ", -, ', [, ], !, <, >, $, ^, &, |`. Our spec is more restrictive and correct for Stellaris specifically — many of those chars are in cwtools to handle CK2/EU4/HOI4 dialects. However, there is one significant omission:

**The `-` (hyphen) in identifier names.** Many Stellaris identifiers include hyphens: `anti-matter`, `tec_psionic-evo-path`, EU4 culture names. Our IDENT rule `[A-Za-z_][A-Za-z_0-9_]*` would split `anti-matter` into `IDENT(anti) OP_MINUS IDENT(matter)` — wrong. Add `-` to the IDENT character continuation set:

```
IDENT: [A-Za-z_][A-Za-z_0-9_\-]*
```

Note the lookahead issue: `-` as identifier continuation vs. `-` as numeric prefix (`-3`). Disambiguation rule: `-` is part of the IDENT if it follows an id-char AND is followed by an id-char or digit. `-` is an arithmetic operator or numeric prefix otherwise.

This is noted in cwtools' char set and confirmed in eug's `ULSTRING` pattern. Our grammar spec must be updated.

### C6.6 — `BOOL` lookahead guard: `yes_please` case

All parsers that recognize `yes`/`no` as booleans (cwtools, clausewitz-interpreter, PDXTools, klausewitz, jomini-via-type-narrowing) handle `yes_please` as a string. Our §3.3 already specifies this correctly: "Lexed as own kind with cwtools-style lookahead guard: `yes` followed by an id-char reverts to `IDENT`." This is validated by all parsers.

### C6.7 — `none` token: case sensitivity

PDXTools handles `none` → null but with exact case. Our spec says `NULL = none` as a reserved word in §3.3. The `RESERVED_TABLE` lookup should be case-insensitive (as with all other reserved words per §3.5). Verify that `None`, `NONE`, `none` all map to `KW_NULL` (or equivalent). PDXTools only handles lowercase `none`.

---

## 7. Open Questions (unresolved across all sources)

### Q1 — `==` semantics: strict equality vs. `=` assign?

Our spec notes `==` is a "distinct token; let semantic pass decide." The community says `==` behavior vs. `=` is debated. clausewitz-interpreter explicitly rejects `==`. cwtools treats it as a distinct operator (EqualEqual = 6uy). No parser in this survey interprets the difference at parse time. **Unresolved.** Our current approach (distinct token, semantic pass decides) is correct as a deferral.

### Q2 — Comma syntax scope in Stellaris specifically?

PDXTools and klausewitz both recognize percent (`%`) and comma (`,`) values. These appear in HOI4 and CK3 define files. Whether they appear in vanilla Stellaris script files is unconfirmed. **Action needed**: scan a vanilla Stellaris `common/defines/` directory for `,` usage before finalizing whether `COMMA` is needed for Stellaris specifically.

### Q3 — `hidden:` prefix: is it Stellaris-specific or universal?

cwtools documents `hidden:` as a scope-key prefix. Whether this appears in Stellaris or only in earlier Paradox games is not confirmed. **Action needed**: grep Stellaris `common/` and `events/` for `hidden:` usage.

### Q4 — Hyphen in identifiers: conflict with unary minus?

As noted in §C6.5: `-` in identifier names creates a lexer ambiguity. Example: `tec_psionic-evo-path` (hypothetical) vs. `tec_psionic - evo_path` (subtraction). Stellaris identifier names do use hyphens (especially EU4/CK2 era mods, less common in Stellaris 3.x). **Action needed**: review Stellaris-specific identifier corpus before updating IDENT char set. If hyphens are absent from Stellaris 3.x vanilla, leave the IDENT rule as-is and document that cross-game portability would require adding `-`.

### Q5 — `@[expr]` scope: does Stellaris support variables from outer scopes?

Our inline expr grammar (§3.11) allows `IDENT` tokens inside `@[...]` as variable references. The exact scoping rules (file-local `@var` only, or can `@[...]` reference script_value names?) are not documented. PDXTools had a TODO about "fix tokenizer splitting raw tokens with math symbols in it" — they encountered real math expressions but couldn't handle them. **Unresolved.** Our permissive inner grammar (accept any IDENT) is the safe choice.

### Q6 — Binary save format for Stellaris multiplayer?

jomini supports binary save formats for EU4 and CK3. Stellaris saves are not plaintext-only (Stellaris ironman saves are binary). Whether our compiler ever needs to process save files (vs. script files only) is unresolved. **Current scope: script files only** (§2). Binary is out-of-scope for G1-G6. Flag as a future capability enabled by jomini's architecture.

### Q7 — Mixed container handling: when is it valid?

jomini's `MixedContainer` marks the transition from homogeneous to heterogeneous block content. PDXTools handles comma-separated list values within keyed objects. Our `BlockItem = Assignment | Block | BareValue` union implicitly allows mixed content. Whether a block with both `Assignment`s and `BareValue`s should produce a warning or is always valid is context-dependent (some blocks like `weights = { 10 20 30 }` are pure `BareValue`, others mix). **Unresolved.** Our current design produces no warning; a future lint could flag suspicious mixed containers.

### Q8 — `NONE` keyword: is it case-insensitive in Stellaris?

PDXTools recognizes lowercase `none` → null. cwtools has `Null` case in Value DU. Whether `NONE` (uppercase) is valid in Stellaris is unverified. **Conservative approach**: add `none` / `NONE` to `RESERVED_TABLE` as `KW_NULL`, case-insensitive per the general rule.

---

*End of research document.*
