# Grammar Framework — Shared Lexer/Parser Primitives

> **Status:** v1 landed as G0 (task #79). Implementation at `tools/clause/compiler/grammar/`. Both Clausewitz (G2) and Clause (M2/M3) build on this.

---

## 1. Purpose

Two languages live in this project:
- **Clausewitz** — the engine's script format (.txt, .gui, .asset, …). Read-only from our POV: we tokenize it to analyze, patch, and emit back. Spec in `docs/clause/GRAMMAR.md`.
- **Clause** — our authoring language. Rust-ish, compiles down to Clausewitz. Spec in `docs/clause/DESIGN.md` §18, §24.

Both need: lexing, parsing, error recovery, span tracking, lossless round-trip, diagnostics. Rather than implement all of that twice, the framework factors out everything language-neutral. Each language's `tokens.py`, `lexer.py`, `parser.py`, `ast.py` provides only what differs.

This is the same split Rust's compiler uses (`rustc_lexer` + `rustc_parse` are the reusable cores, each language edition plugs into them).

## 2. Module map

```
compiler/grammar/
    span.py         SourceFile(path, text, line_starts); Span(file, start, end)
    trivia.py       Trivia(kind, span); TriviaKind { WHITESPACE, NEWLINE, LINE_COMMENT, BLOCK_COMMENT }
    token.py        Token[TK](kind, span, leading_trivia, trailing_trivia); .full_text
    stream.py       TokenStream[TK]: peek(n), advance, eat(kind), mark(), rewind()
    lexer.py        LexerBase[TK]: CharCursor, scan_one_trivia, scan_next_token, tokenize,
                    emit_diagnostic, + check_round_trip / reconstruct_source helpers
    parser.py       ParserBase[TK]: expect, eat, check, recover_to, error/warning/note;
                    ParseResult[N] wrapper
    ast.py          AstNode(span); Visitor (visit_<TypeName> dispatch)
    diagnostic.py   Diagnostic(level, code, message, span, notes); DiagnosticLevel

    clausewitz/     (G2 — not yet implemented)
    clause/         (M2/M3 — not yet implemented)
```

## 3. Invariants (enforced by the framework)

### 3.1 Lossless round-trip

For any tokenize output `tokens` on source `src`:
```
"".join(t.full_text for t in tokens) == src
```

Every byte of source is accounted for: either in a token's text or in one of its attached trivia spans. Comments, blank lines, indentation — all preserved through any compile pass. The test suite property-checks this on a dozen mixed-trivia inputs.

Why it matters: patches need to diff cleanly against source. Code-generation passes need to re-emit files that look like input. Round-trip is the difference between "a parser" and "a parser that can round-trip edits."

### 3.2 Trivia attachment (Roslyn/swift-syntax convention)

- Trivia before a token, up through (and including) the newline that ends the previous line, become the *next* token's `leading_trivia`.
- Trivia immediately after a token, on the same line, up to (not including) the terminating newline, become that token's `trailing_trivia`.

Consequence: a standalone line comment attaches to whatever follows it (semantically "the comment is about X"). A trailing `// X` comment stays with its line's token. This matches intuition — and what every refactoring tool that preserves comments needs.

### 3.3 Progress guarantee

Every iteration of the `LexerBase.tokenize` loop must advance the cursor. A `scan_next_token` implementation that emits a token without consuming input would loop forever; the base class turns that into a `RuntimeError` at the buggy offset rather than letting the process hang.

### 3.4 Spans map to (file, line, col)

Every `Span` carries a SourceFile back-pointer plus `[start, end)` byte offsets. `span.start_line_col` / `span.end_line_col` give (line, col) via the SourceFile's precomputed line-start table — O(log lines). Diagnostics and source-map emission depend on this.

### 3.5 Error accumulation, never fail-fast

The lexer and parser accumulate `Diagnostic` objects and continue. User-input errors do NOT raise. A pipeline coordinator (the compiler driver) checks accumulated severities at its discretion and decides whether to halt.

Lexer emits `INVALID(text)` tokens on unrecognized bytes; parser emits diagnostics on unexpected tokens and uses `recover_to(kinds)` to resync at the next synchronisation point. Neither throws.

## 4. How to add a new language

1. `grammar/<lang>/tokens.py` — define a `TokenKind` enum with the language's token categories + a distinguished EOF and INVALID kind.
2. `grammar/<lang>/lexer.py` — subclass `LexerBase[LangTokenKind]`:
   - Set `LINE_COMMENT_STARTS` and `BLOCK_COMMENT_DELIMS` class attrs (or override `scan_one_trivia` for non-standard trivia).
   - Implement `scan_next_token() -> Token | None`: produce one token, advancing the cursor. Return `None` if the current byte is not handled (base class emits INVALID + advances 1).
   - Implement `eof_kind()` and `invalid_kind()`.
3. `grammar/<lang>/parser.py` — subclass `ParserBase[LangTokenKind]`. Build recursive-descent rules using `peek`, `expect`, `eat`, `recover_to`. Return a `ParseResult[YourAst]`.
4. `grammar/<lang>/ast.py` — subclass `AstNode` for each node type. Add language-specific node classes.
5. `tests/unit/test_grammar_<lang>.py` — per-token-kind positive tests, edge cases, round-trip invariant check.

## 5. Non-goals

- **Declarative grammar specification** (PEG, BNF, ANTLR). Hand-written lexers and recursive-descent parsers give us precise control over error recovery, lookahead, and language-specific disambiguation (Clausewitz `yes` vs `yes_please`, Clause `:` bind-syntax vs operator colon). A grammar DSL would obscure more than it reveals at this scale.
- **Incremental lexing.** Tokenize is whole-file. Incremental re-tokenization for LSP would layer on top (task M12); not here.
- **Unicode beyond ASCII.** Clausewitz files are ASCII + Latin-1 in practice; Clause source is ASCII. Non-ASCII appears only inside string literals, where we pass through bytes unchanged. No grapheme-cluster work.
- **Byte-level performance optimization.** Python implementation; we trade a ~5× CPU cost for clarity and stdlib-only deps. A later Rust port can match C-level speeds when performance bounds become real.

## 6. Evolution

New token kinds, trivia kinds, or language-level features may require framework changes. Rules:
- The framework changes are backward-compatible with existing `grammar/<lang>` impls unless a bump is strictly necessary.
- A framework change that breaks one language's invariants (e.g. trivia attachment rules) requires a coordinated update of both languages' test suites.
- Utility functions used only by one language belong in that language's module, not the framework.

## 7. Testing

`tools/clause/compiler/tests/unit/test_grammar_core.py` — 39 tests covering SourceFile line tables, Span operations, Trivia attachment splits, TokenStream cursor semantics, ParserBase expect/recover, Visitor dispatch, Diagnostic formatting, and the lossless round-trip property (10+ input shapes including CRLF, empty files, whitespace-only, unterminated comments).

Each concrete language will add its own `test_grammar_<lang>.py` with per-token positive/negative tests.
