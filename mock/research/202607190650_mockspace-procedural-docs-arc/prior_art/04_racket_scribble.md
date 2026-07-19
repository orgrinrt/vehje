# Racket core + Scribble, mechanically

## What each is, in three sentences

Racket is a language-oriented Lisp: source is read into syntax objects, macro-expanded to a small fixed core, and the core is what runs. `#lang` lets a file pick a reader (bytes to syntax objects) and an expander starting point, so "Racket" is a family of languages sharing one expander and core. Scribble is a documentation toolchain built as an ordinary `#lang`: its reader turns `@`-syntax into function calls, its module language folds the body's values into an immutable document tree of structs, and a four-pass pipeline resolves cross-references before a pluggable renderer walks the tree.

## Racket's core forms and how `#lang` plugs in

`reference/syntax-model.html` §1.2.3.1 gives the fully-expanded grammar, expression level:

```
expr = id | (#%plain-lambda formals expr ...+) | (case-lambda (formals expr ...+) ...)
     | (if expr expr expr) | (begin expr ...+) | (begin0 expr expr ...)
     | (let-values ([(id ...) expr] ...) expr ...+) | (letrec-values ([(id ...) expr] ...) expr ...+)
     | (set! id expr) | (quote datum) | (quote-syntax datum) | (quote-syntax datum #:local)
     | (with-continuation-mark expr expr expr) | (#%plain-app expr ...+) | (#%top . id)
     | (#%variable-reference id) | (#%variable-reference (#%top . id)) | (#%variable-reference)
     | (#%foreign-inline datum keyword)
```

15 distinct expression forms (`quote-syntax #:local` and the three `#%variable-reference` arities as variants). Module/top-level-only: `(module id module-path (#%plain-module-begin module-level-form ...))`, `(module* ...)`, `(#%declare declaration-keyword ...)`, `(#%provide raw-provide-spec ...)`, `(begin-for-syntax module-level-form ...)`. Both levels: `(define-values (id ...) expr)`, `(define-syntaxes (id ...) expr)`, `(#%require raw-require-spec ...)`. Top-level-only: `(begin top-level-form ...)`, `(begin-for-syntax top-level-form ...)`.

`#lang` spans a reader layer and an expander layer (`guide/languages.html`). A `#lang name` line resolves to a reader producing `(module anonymous-name lang-module body ...)`; `#lang s-exp module-name` is stated directly as sugar for `(module <derived-name> module-name form ...)`. `#%module-begin` is the entry every language must supply: "the `#%module-begin` form is an implicit form that wraps the body of a module. It must be provided by a module that is to be used as [a] module language" (`guide/module-languages.html`). A language customizes semantics by exporting a replacement that wraps/filters/transforms the body before delegating to the real one; Scribble does this. Minimum for a new `#lang`: a reader (often `syntax/module-reader`, accepting `#:read`/`#:read-syntax` overrides) plus a language exporting at least `#%module-begin`.

Hygiene, `reference/syntax-model.html` §1.2.3.5 verbatim: "Before the expander passes a syntax object to a transformer, the syntax object is extended with a fresh macro-introduction scope...to distinguish syntax objects at the macro's use site from syntax objects that are introduced by the macro; in the result of the transformer the presence of the scope is flipped, so that introduced syntax objects retain the scope, and use-site syntax objects do not have it." Net effect: a macro's template identifiers resolve against its definition context (no capture either direction); use-site identifiers keep their use-site binding. A per-expansion scope-set operation baked into the expander, not a surface convention.

## The `@`-reader

Source: `racket/pkgs/at-exp-lib/scribble/reader.rkt` (672 lines; lives in the `at-exp-lib` package, not `scribble`). Grammar: `@ ‹cmd› [ ‹datum›* ] { ‹text-body›* }`, all parts optional (one required), no space between parts, reading as `(‹cmd› ‹datum›* ‹parsed-body›*)`.

- `‹cmd›` any Racket expression (incl. `` ` ``/`'`/`,`/`@` prefixes); `@foo{blah}` → `(foo "blah")`.
- `‹datum›` in `[...]`, ordinary syntax spliced before body strings: `@foo[#:style 'big]{bar}` → `(foo #:style 'big "bar")`.
- `‹text-body›` in `{...}`: strings plus nested `@`-forms, balanced braces need no escaping (`@foo{f{o}o}` → `(foo "f{o}o")`). Nested `@bar{b}` → `(bar "b")`; bare `@bar` → identifier `bar` spliced as an expression.
- `@"@"` escapes a literal `@` (string merges into surrounding text). `|...|` isolates a Racket expression without producing a string: `@foo{foo@|bar|.}` → `(foo "foo" bar ".")`.
- Unbalanced/literal braces: switch delimiter via `|{ }|` (optional punctuation, e.g. `|<<{ }>>|`); nested `@`-forms inside then use `|@`.
- `@;{...}` nestable block comment; `@;` to end of line.
- Indentation: leading whitespace per line computed from the `{`'s column; the line right after `{` and a lone trailing newline before `}` are dropped.

Deterministic character-stream parsing; depends on Racket's reader-extension protocol and on syntax objects carrying source locations, heap-boxed in the reference implementation.

## `scribble/doclang` to the `doc` binding

`doclang.rkt` (15 lines) replaces `#%module-begin`:

```racket
(define-syntax (*module-begin stx)
  (syntax-case stx ()
    [(_ id post-process exprs . body)
     #'(#%module-begin
        (module configure-runtime racket/base (require scribble/base/lang/configure-runtime))
        (doc-begin id post-process exprs . body))]))
```

`doc-begin` (`private/doc-begin.rkt`, 80 lines) walks body forms via `local-expand`, sorting each into: a string literal (batched), a `require`/`provide`/`define-values`/`define-syntaxes`/`begin-for-syntax`/`module`/`#%require`/`#%provide`/`#%declare` form (passed through), or else (wrapped `(pre-part <expanded> <original>)`, accumulated). At the end: `(define m-id (post-process (decode (list . exprs)))) (provide m-id)`. `decode`'s contract: `(-> (listof pre-part?) part?)` — folds accumulated runtime values into one `part`. `doclang2.rkt` (31 lines) is the keyword version (`#:id`, `#:post-process`, `#:exprs`, `#:begin`; defaults `doc`/`values`). A document module's body thus reduces, after instantiation, to one `doc` value: a `part?`, importable elsewhere as a first-class value (how cross-document hyperlinking sources targets).

## The document model (`scribble/core.rkt`; structs in `private/provide-structs.rkt`)

Verbatim field lists:

```racket
[part ([tag-prefix (or/c #f string? hash?)] [tags (listof tag?)] [title-content (or/c #f content?)]
       [style style?] [to-collect list?] [blocks (listof block?)] [parts (listof (or/c part? traverse-part?))])]
[paragraph ([style style?] [content content?])]
[table ([style style?] [blockss (and/c (listof (listof (or/c block? (one-of/c 'cont)))) same-lengths?)])]
[itemization ([style style?] [blockss (listof (listof block?))])]
[nested-flow ([style style?] [blocks (listof block?)])]
[compound-paragraph ([style style?] [blocks (listof block?)])]
[element ([style element-style?] [content content?])]
[delayed-block ([resolve (any/c part? resolve-info? . -> . block?)])]
[style ([name (or/c string? symbol? #f)] [properties list?])]
[collected-info ([number (listof part-number-item?)] [parent (or/c #f part?)] [info any/c])]
```

`flow` is not a struct, just `(listof block?)` by contract. `block?` verbatim: `(or (paragraph? p) (table? p) (itemization? p) (nested-flow? p) (compound-paragraph? p) (delayed-block? p) (traverse-block? p))`. `content?` verbatim: string, `element?`, a list of `content?`, `delayed-element?`, `traverse-element?`, `part-relative-element?`, `multiarg-element?`, a fixed decorative-symbol set (`nbsp mdash ndash ldquo rdquo rsquo lsquo prime rarr larr alpha infin lang rang`), or anything `convertible?` (`file/convertible`) — a closed recursive union admitting an external open-world protocol as a content citizen. `collect-info`/`resolve-info` (plain `define-struct`): `(collect-info fp ht ext-ht ext-demand parts tags gen-prefix relatives parents)`, `(resolve-info ci delays undef searches)`. All structs are `#:transparent`, serializable, immutable; only pass-local `ht` fields mutate, as write-once accumulators.

## The four-pass pipeline and deferred content

`base-render.rkt` defines `render<%>` with methods `traverse collect resolve render` plus format-hook methods each backend mixin overrides. `render%` (~1251 lines) is the shared implementation of all four passes.

**Traverse** iterates document order "until it obtains a fixed point." Verbatim mechanism:

```racket
(define/public (traverse ds fns)
  (let loop ([fp #hasheq()])
    (define fp2 (start-traverse ds fns fp))
    (if (equal? fp fp2) fp (loop fp2))))
```

`fp` is a persistent immutable hash from a `traverse-*` node to its resolved value or a retry procedure; each visitor threads `fp` through `for/fold` — the tree is never mutated. The loop ends when two successive `fp` snapshots are `equal?`; it iterates because a `traverse-element` in part A may need a value part B only produces after seeing A's contribution. Contracts, verbatim:

```racket
;; element-traverse-procedure/c = ((symbol? any/c -> any/c) (symbol? any/c -> any) -> (or/c element-traverse-procedure/c content?))
(struct traverse-element ([traverse element-traverse-procedure/c]))
(struct delayed-element ([resolve (any/c part? resolve-info? -> content?)] [sizer (-> any)] [plain (-> any)]))
(struct part-relative-element ([collect (collect-info? -> content?)] [sizer (-> any)] [plain (-> any)]))
```

A `traverse-element`'s procedure gets a getter/setter over shared state, returns `content?` (done) or a new procedure (retry). `delayed-element` defers to **resolve**: its `resolve` field runs once, after collection settles, given the enclosing part and a `resolve-info?`; `sizer`/`plain` stand in for width estimation and pre-collect rendering. `part-relative-element` resolves during **collect**, from `collect-info-parents` (tree position only, no global tables).

**Collect** ("globally collects information...that can span documents built at separate times") writes into `collect-info`'s mutable `ht` via `collect-put!`. **Resolve** ("matches hyperlink references with targets and expands delayed elements") reads back via `resolve-get`/`resolve-search`, walking `collected-info-parent` chains, falling through to `ext-ht`/`ext-demand` for cross-document lookups. **Render** walks the resolved tree, calling backend leaf methods.

## The `convertible` protocol

`prop:convertible` holds one procedure `(v request default) -> result`. `convert`: `(convert v request [default]) -> conversion-result`. Standard requests: `'text` (string); `'gif-bytes 'png-bytes 'png@2x-bytes 'ps-bytes 'eps-bytes 'pdf-bytes 'svg-bytes` (bytes); `'*-bytes+bounds` (bytes + width/height/descent/top-space); `'*-bytes+bounds8` (adds left/right/top/bottom pad). Any other symbol is valid; a producer answers what it can, else returns `default` — the seam letting `content?` hold a pict/image/plot without the model knowing those types exist.

## Writing a render backend

A backend is `(render-mixin %) -> (class % ...)` over `render%`, overriding `get-suffix`, `get-substitutions`, `render-part`, `render-flow`, `render-table`, `render-itemization`, `render-paragraph`, `render-content`, `render-other`, `render-nested-flow`. `text-render.rkt` is standalone, complete, **322 lines** (`wc -l`, `racket/scribble` master). `markdown-render.rkt` is **381 lines**, not standalone: a mixin over `text-render`'s mixin, nearly all its lines Markdown escaping/table/link logic, not pipeline plumbing. `base-render.rkt`, the shared engine, is 1251 lines, written once.

## Mechanisms worth naming

- **Persistent-hash fixed point for traverse**: termination + mutation-free, costs an `equal?` walk per round, forces cross-part features into retry-until-stable form.
- **Two-tier deferred content** (`traverse-element` retried in-pass vs `delayed-element` resolved once post-collect): former sees growing state at O(iterations) cost; latter one call, only after everything global is known.
- **`content?` admitting `convertible?`**: decouples the model from any future media type, costs structural undecidability and a `convert`+`#f` fallback everywhere.
- **`doc-begin`'s `local-expand` classification**: lets `define`/`require` sit next to prose in one body, costs an allowlist tracking the expander's own core-form set.
- **One `render<%>` interface, refined per backend by mixin**: a new output form costs "override a dozen leaf methods," not "reimplement traverse/collect/resolve."

## What would not transfer to no_std, no-alloc Rust

- **Syntax objects and macro expansion**: boxed, GC-managed, carrying scope sets and source locations; assumes a collector and unbounded tree allocation. Hygiene's scope-flip allocates a fresh scope token per expansion step.
- **The persistent `#hasheq` driving the traverse fixed point**: HAMT-style structure sharing; each `hash-set` allocates. A no-alloc port needs a fixed-capacity table with generation counters, and cannot reuse whole-hash `equal?` termination verbatim.
- **`prop:serializable`/`racket/serialize`**: struct properties plus a dynamic parameter save compiled docs for cross-build linking; no fixed-size analog.
- **Dynamically-typed `content?`/`any/c` positions**: a runtime union, several fields bare `any/c`. A static port needs a closed enum (loses the `convertible?` hook) or a trait object (regains it, costs dispatch and usually heap allocation).
- **`local-expand` inside `doc-begin`**: partially expanding and matching a live core-form list is expander-time reflection, no compile-time Rust equivalent absent a full macro system.
- **Unbounded nested lists everywhere**: no fixed arity or depth bound; a no-alloc port needs arena/fixed-capacity backing with an explicit cap, changing the model's guarantees.
- **Dynamic-extent parameters** (`current-output-port` reparameterized per table cell): continuation-aware dynamic binding; a Rust port threads state explicitly instead, mechanical but not the same mechanism.

## Sources

- https://docs.racket-lang.org/reference/syntax-model.html (core forms, hygiene)
- https://docs.racket-lang.org/guide/languages.html , https://docs.racket-lang.org/guide/module-languages.html (`#lang`)
- https://github.com/racket/racket/blob/master/pkgs/at-exp-lib/scribble/reader.rkt (`@`-reader)
- https://github.com/racket/scribble/blob/master/scribble-lib/scribble/doclang.rkt , .../doclang2.rkt , .../private/doc-begin.rkt
- https://github.com/racket/scribble/blob/master/scribble-lib/scribble/core.rkt , .../private/provide-structs.rkt , .../decode.rkt ; docs: https://docs.racket-lang.org/scribble/core.html
- https://github.com/racket/scribble/blob/master/scribble-lib/scribble/base-render.rkt , .../text-render.rkt , .../markdown-render.rkt ; docs: https://docs.racket-lang.org/scribble/renderer.html
- https://docs.racket-lang.org/file/convertible.html
