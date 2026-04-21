# Clause — EBNF reference

Normative grammar for the Clause authoring language (the `.cse` source
that compiles to Clausewitz). Uses ISO/IEC 14977 EBNF notation with
these conventions:

- `foo` — a literal keyword / punctuation token (written as source).
- `Nonterm` — a non-terminal production.
- `[ X ]` — optional (zero or one).
- `{ X }` — zero or more repetitions.
- `X | Y` — alternative.
- `( X )` — grouping.
- `"…"` — literal string matched case-sensitively.
- `IDENT`, `INT`, `FLOAT`, etc — lexical terminals defined in §1.

Each production is paired with the parser function that implements it
(``parser.py:_parse_X``) so tool authors can cross-reference shipped
code against the spec. Discrepancies are DRIFT and should be fixed in
code or reflected in this doc — see `LANG_AUDIT_2026_04.md` for the
tracking register.

---

## 1. Lexical grammar

### 1.1 Terminals

```
IDENT        ::= id_start { id_continue }
INT          ::= decimal_int | hex_int | binary_int | octal_int
                 [ int_suffix ]
FLOAT        ::= decimal_int "." { digit } [ exponent ] [ float_suffix ]
                | decimal_int exponent [ float_suffix ]
STRING       ::= '"' { string_char | escape } '"'
RAW_STRING   ::= 'r' { '#' } '"' { any_char_not_matching_close } '"' { '#' }
CHAR         ::= "'" ( char_byte | escape ) "'"
BOOL         ::= "true" | "false"
```

Identifier starts are `[A-Za-z_]`; continuations add digits.
`r"..."` / `r#"..."#` raw strings match hash counts on both sides
(Rust convention). Numeric suffixes are free-form idents (``42u32``,
``1.5f64``) carried through for later passes.

### 1.2 Keywords (reserved)

```
KW         ::= "mod" | "use" | "pub" | "struct" | "trait" | "impl"
            | "fn" | "let" | "mut" | "const" | "static"
            | "extern" | "expect" | "actual"
            | "event" | "enum" | "type" | "macro" | "sealed"
            | "if" | "else" | "match" | "for" | "while" | "loop"
            | "return" | "break" | "continue" | "in"
            | "self" | "Self" | "super" | "crate"
            | "as" | "where" | "dyn" | "move"
            | "unsafe" | "async" | "await"
```

`dyn`, `unsafe`, `async`, `await` are reserved for future use but not
accepted in any production today.

### 1.3 Punctuation

Single-char: ``+ - * / % = < > ! & | ^ ~ . , ; : ?``
Multi-char: ``-> => :: .. ..= << >> == != >= <= && || += -= *= /= %= &= |= ^= <<= >>=``
Brackets: ``( ) { } [ ]``
Attribute openers: ``#`` (outer) and ``#!`` (inner).
Macro interpolation sigil: ``$`` (meaningful only inside a ``quote!``
body; tokenized as ``DOLLAR`` elsewhere).

### 1.4 Trivia

Whitespace, newlines, line comments (``// …``), and block comments
(``/* … */``, nestable) are preserved as trivia attached to the next
token for lossless round-trip. Doc comments (``/// …``) are line
comments with a special prefix — consumed by `clause doc` but not
distinguished at the token level.

---

## 2. File structure

```
File         ::= { InnerAttribute } { Item }
InnerAttribute ::= "#!" "[" AttrArgs "]"
```

*Implemented by*: `parser._parse_file`, `parser._parse_inner_attrs`.

---

## 3. Items

```
Item         ::= { OuterAttribute } [ Visibility ] [ "sealed" ]
                 ItemKind
ItemKind     ::= ModDecl | UseDecl | StructDecl | EnumDecl
               | TraitDecl | ImplBlock | FnDecl | MacroDecl
               | EventDecl | TypeAlias | ConstDecl | StaticDecl
               | ExpectItem | ActualItem | ExternItem
               | MacroInvokeItem
OuterAttribute ::= "#" "[" AttrArgs "]"
Visibility   ::= "pub" [ "(" ( "crate" | "super" | "in" Path ) ")" ]
```

The ``sealed`` modifier may appear either before or after ``pub``
(either order is legal). Its placement is rejected on every
``ItemKind`` other than ``StructDecl`` and ``TraitDecl`` — the
parser emits ``PARSE_CL250`` for unsupported positions and
``PARSE_CL251`` for duplicate modifiers.

*Implemented by*: `parser._parse_item`.

### 3.1 Modules and `use`

```
ModDecl      ::= "mod" Path ( ";" | "{" { Item } "}" )
UseDecl      ::= "use" UseTree ";"
UseTree      ::= Path [ "::" "{" UseTreeList "}" ]
              |  Path [ "as" IDENT ]
              |  Path "::" "*"
              |  "{" UseTreeList "}"
UseTreeList  ::= [ UseTree { "," UseTree } [ "," ] ]
Path         ::= [ "::" ] PathSeg { "::" PathSeg } [ PathGenerics ]
PathSeg      ::= IDENT | "self" | "Self" | "super" | "crate"
PathGenerics ::= "<" TypeArgList ">" | "::" "<" TypeArgList ">"
TypeArgList  ::= TypeExpr { "," TypeExpr } [ "," ]
```

### 3.2 Struct, enum, trait, impl

```
StructDecl   ::= "struct" IDENT [ Generics ] [ ":" TypeExpr ]
                 [ WhereClause ] "{" [ StructFieldList ] "}"
StructFieldList ::= StructField { "," StructField } [ "," ]
StructField  ::= { OuterAttribute } [ Visibility ] [ "const" ] [ "mut" ]
                 IDENT ":" TypeExpr [ "=" Expr ]

EnumDecl     ::= "enum" IDENT [ Generics ] [ WhereClause ]
                 "{" [ EnumVariantList ] "}"
EnumVariantList ::= EnumVariant { "," EnumVariant } [ "," ]
EnumVariant  ::= { OuterAttribute } IDENT [ "(" TypeList ")" ]
TypeList     ::= TypeExpr { "," TypeExpr } [ "," ]

TraitDecl    ::= "trait" IDENT [ Generics ] [ ":" BoundList ]
                 [ WhereClause ] "{" { Item } "}"

ImplBlock    ::= "impl" [ Generics ] TypeExpr [ "for" TypeExpr ]
                 [ WhereClause ] "{" { Item } "}"
```

The `: TypeExpr` tail of `StructDecl` is the bind-target colon syntax;
the type after the colon must impl `std::bind::Bind`. The `": " BoundList`
tail of `TraitDecl` is the supertrait list — every impl of the
declared trait must also have an impl of each supertrait for the same
self-type (``CL_MISSING_SUPERTRAIT_IMPL``).

### 3.3 Fn, macro, event

```
FnDecl       ::= "fn" IDENT [ Generics ] "(" [ FnParamList ] ")"
                 [ "->" TypeExpr ] [ WhereClause ]
                 ( Block | ";" )
FnParamList  ::= FnParam { "," FnParam } [ "," ]
FnParam      ::= SelfParam | TypedParam
SelfParam    ::= "self" | "mut" "self" | "&" "self" | "&" "mut" "self"
TypedParam   ::= IDENT [ ":" TypeExpr ]

MacroDecl    ::= "macro" IDENT [ Generics ] "(" [ FnParamList ] ")"
                 "->" TypeExpr [ WhereClause ] Block

EventDecl    ::= "event" IDENT "for" TypeExpr Block
```

Fn bodies can be `;` (signature-only, for trait items and extern fns).
Macro bodies are mandatory per ``CL_MACRO_DECL_RETURN_REQUIRED`` plus
``PARSE_CL234`` (missing body).

### 3.4 Type alias, const, static

```
TypeAlias    ::= "type" IDENT [ "=" TypeExpr ] ";"
ConstDecl    ::= "const" IDENT ":" TypeExpr [ "=" Expr ] ";"
StaticDecl   ::= "static" [ "mut" ] IDENT ":" TypeExpr "=" Expr ";"
```

`TypeAlias` without `= T` is legal only inside a trait body (assoc
type declaration); at top level or inside an impl body it's rejected
with ``CL_ASSOC_TYPE_WITHOUT_TRAIT`` / ``CL_ASSOC_TYPE_NO_DEFINITION``.
Same split for `ConstDecl` without `= expr` (`CL_CONST_DECL_NEEDS_INIT`
at module level, `CL_ASSOC_CONST_WITHOUT_TRAIT` inside an inherent
impl, `CL_ASSOC_CONST_NO_DEFINITION` inside a trait impl).

### 3.5 Expect / actual

```
ExpectItem   ::= "expect" ItemKind
ActualItem   ::= "actual" ItemKind
```

Used for `expect fn`, `expect struct`, `actual impl`, etc. — multi-
platform / multi-crate contract bindings.

### 3.6 Extern

```
ExternItem   ::= "extern" ExternBody
ExternBody   ::= "scope" IDENT ";"                 (engine-provided scope)
               | "struct" IDENT ";"                 (engine-provided struct)
               | "trait" IDENT [ Generics ] "{" { Item } "}"
               | "impl" [ Generics ] TypeExpr "for" TypeExpr ( ";" | "{" { Item } "}" )
               | "fn" IDENT [ Generics ] "(" [ FnParamList ] ")" [ "->" TypeExpr ] ";"
```

### 3.7 Macro invocations at item position

```
MacroInvokeItem ::= Path "!" MacroDelimitedBody [ ";" ]
MacroDelimitedBody ::= "(" TokenStream ")"
                     | "{" TokenStream "}"
                     | "[" TokenStream "]"
```

Semicolon is required for `(`/`[` delimiters; `{`-delimited
invocations are self-terminating.

---

## 4. Generics and where

```
Generics     ::= "<" [ GenericParam { "," GenericParam } [ "," ] ] ">"
GenericParam ::= TypeParam | ConstParam
TypeParam    ::= IDENT [ ":" BoundList ]
ConstParam   ::= "const" IDENT ":" TypeExpr

BoundList    ::= TypeExpr { "+" TypeExpr }

WhereClause  ::= "where" [ WherePredicate { "," WherePredicate } [ "," ] ]
WherePredicate ::= TypeExpr ":" BoundList
```

`ConstParam`'s type is retained on the AST; enforcement at
instantiation sites rides with T3 monomorphization (#121).
`BoundList` retains every supplied bound on the AST. Instantiation-
site enforcement rides with #160 G7b.

---

## 5. Type expressions

```
TypeExpr     ::= RefType | TupleType | PathType | ErrorType
RefType      ::= "&" [ "mut" ] TypeExpr
TupleType    ::= "(" ")"                                (unit)
              | "(" TypeExpr ")"                         (peeled)
              | "(" TypeExpr "," ")"                     (1-tuple)
              | "(" TypeExpr "," TypeExpr { "," TypeExpr } [ "," ] ")"
PathType     ::= Path
```

References (`&T`, `&mut T`) are accepted anywhere a type is valid;
they erase to their pointee at every downstream layer (§10.3 DESIGN).
`(T)` without a trailing comma is NOT a tuple — it's the inner type
with parens dropped. `(T,)` is the 1-tuple spelling.

---

## 6. Statements

```
Stmt         ::= LetStmt | ExprStmt | ItemStmt
LetStmt      ::= "let" [ "mut" ] Pattern [ ":" TypeExpr ]
                 [ "=" Expr ] ";"
ExprStmt     ::= Expr [ ";" ]
ItemStmt     ::= Item
```

An `ExprStmt` without a trailing `;` is the block's trailing
expression (the block's value). `LetStmt` with no initializer
declares an uninitialized binding — allowed today; real
definite-assignment analysis rides with type inference (#119).

---

## 7. Expressions

Expressions follow a standard precedence-climbing chain. Highest
binding first:

```
Primary      ::= Literal | PathExpr | BlockExpr | GroupOrTuple
              | IfExpr | MatchExpr | ForExpr | WhileExpr | LoopExpr
              | ReturnExpr | BreakExpr | ContinueExpr
              | ClosureExpr | MacroInvokeExpr
              | StructLiteralExpr        (only when allowed by context)
GroupOrTuple ::= "(" ")"                 (unit)
              | "(" Expr ")"              (paren)
              | "(" Expr "," ")"          (1-tuple)
              | "(" Expr "," Expr { "," Expr } [ "," ] ")"

Postfix      ::= Primary { PostfixOp }
PostfixOp    ::= "." IDENT                                (field access)
              | "." IDENT "(" [ CallArgList ] ")"         (method call)
              | "(" [ CallArgList ] ")"                   (call)
              | "[" Expr "]"                              (index)
              | "?"                                       (question)

Unary        ::= ( "-" | "!" | "&" [ "mut" ] ) Unary
              | Postfix

MulExpr      ::= Unary { ( "*" | "/" | "%" ) Unary }
AddExpr      ::= MulExpr { ( "+" | "-" ) MulExpr }
ShiftExpr    ::= AddExpr { ( "<<" | ">>" ) AddExpr }
BitAndExpr   ::= ShiftExpr { "&" ShiftExpr }
BitXorExpr   ::= BitAndExpr { "^" BitAndExpr }
BitOrExpr    ::= BitXorExpr { "|" BitXorExpr }
CmpExpr      ::= BitOrExpr { ( "==" | "!=" | "<" | ">" | "<=" | ">=" ) BitOrExpr }
AndExpr      ::= CmpExpr { "&&" CmpExpr }
OrExpr       ::= AndExpr { "||" AndExpr }
RangeExpr    ::= OrExpr [ ( ".." | "..=" ) [ OrExpr ] ]
                | ( ".." | "..=" ) [ OrExpr ]
AssignExpr   ::= RangeExpr [ AssignOp Expr ]
AssignOp     ::= "=" | "+=" | "-=" | "*=" | "/=" | "%="
              | "&=" | "|=" | "^=" | "<<=" | ">>="
Expr         ::= AssignExpr
```

### 7.1 Block

```
BlockExpr    ::= "{" { Stmt } [ Expr ] "}"
```

Inside a block, struct literals ARE permitted (the flag re-enables
inside block bodies). The trailing `Expr` (no semicolon) is the
block's value.

### 7.2 If / match / for / while / loop

```
IfExpr       ::= "if" ExprNoStructLit BlockExpr [ "else" ( BlockExpr | IfExpr ) ]
MatchExpr    ::= "match" ExprNoStructLit "{" { MatchArm "," } "}"
MatchArm     ::= Pattern [ "if" ExprNoStructLit ] "=>" Expr
ForExpr      ::= "for" Pattern "in" ExprNoStructLit BlockExpr
WhileExpr    ::= "while" ExprNoStructLit BlockExpr
LoopExpr     ::= "loop" BlockExpr
```

``ExprNoStructLit`` is an expression parsed with the
`_allow_struct_literals` flag set to `False` — disambiguating
``if Name { … }`` between a struct-literal condition and a path
condition with a block body. Rust uses the same rule.

### 7.3 Return / break / continue

```
ReturnExpr   ::= "return" [ Expr ]
BreakExpr    ::= "break" [ Expr ]
ContinueExpr ::= "continue"
```

### 7.4 Closure

```
ClosureExpr  ::= [ "move" ] "||" Expr
              | [ "move" ] "|" [ ClosureParamList ] "|" Expr
ClosureParamList ::= ClosureParam { "," ClosureParam } [ "," ]
ClosureParam ::= IDENT [ ":" TypeExpr ]
```

### 7.5 Struct literal

```
StructLiteralExpr ::= Path "{" [ StructLiteralBody ] "}"
StructLiteralBody ::= StructLiteralField { "," StructLiteralField } [ "," ]
                    [ "," ".." Expr ]
StructLiteralField ::= IDENT [ ":" Expr ]
```

A shorthand field (`Name { x, y }`) uses the ident as both field name
and value binding. The `..base` tail copies unspecified fields from
`base`. Struct literals are context-restricted: disallowed in the
head of `if`/`while`/`for`/`match`; re-allowed inside block bodies.

### 7.6 Macro invocation (expression position)

```
MacroInvokeExpr ::= Path "!" MacroDelimitedBody
```

### 7.7 Call argument lists

```
CallArgList  ::= CallArg { "," CallArg } [ "," ]
CallArg      ::= [ IDENT ":" ] Expr
```

Keyword args (`IDENT : Expr`) are parsed but currently ignored by
downstream passes (positional-only matching at call resolution).

---

## 8. Patterns

```
Pattern      ::= OrPattern
OrPattern    ::= SinglePattern { "|" SinglePattern }
SinglePattern ::= WildPat | LiteralPat | IdentPat | PathPat
              | TupleStructPat | TuplePat | RangePat
              | RefPat | RestPat

WildPat      ::= "_"
LiteralPat   ::= INT | FLOAT | STRING | RAW_STRING | CHAR | BOOL
                 [ ( ".." | "..=" ) RangeEndpoint ]
IdentPat     ::= IDENT                      (bare binding)
PathPat      ::= Path                       (multi-seg or leading-colon)
TupleStructPat ::= Path "(" [ Pattern { "," Pattern } [ "," ] ] ")"
TuplePat     ::= "(" ")"                    (unit)
              | "(" Pattern ")"              (peeled)
              | "(" Pattern "," ")"          (1-tuple)
              | "(" Pattern "," Pattern { "," Pattern } [ "," ] ")"
RangePat     ::= [ Expr ] ( ".." | "..=" ) [ Expr ]
RefPat       ::= "&" [ "mut" ] SinglePattern
RestPat      ::= ".."
```

Match exhaustiveness on enum variants flattens `OrPattern` into
its alternatives and peels `RefPat` to its inner (#161 slice).
Range + struct pattern exhaustiveness is still #161.

---

## 9. Attribute arguments

```
AttrArgs     ::= Path [ "(" TokenStream ")" ]
```

The `TokenStream` inside `(...)` is stored as raw text on the
attribute node (``Attribute.arg_span_text``). Attribute-specific
semantics (``#[patch(target="…")]``, ``#[repr(flag)]``, etc.) are
parsed out lazily by the consumers, not by the top-level grammar.

Outer (`#[…]`) attributes attach to the next item. Inner
(`#![…]`) attributes attach to the enclosing module or file.

---

## 10. Reserved corners not yet productive

The following tokens lex but have no production today:

- `dyn Trait` — reserved; no parser path. Rejected wherever a type
  expression is expected.
- `unsafe`, `async`, `await` — reserved; treated as `PARSE_CL010` in
  every position.
- Double-reference `&&T` at type position — must spell as `& &T` with
  spacing because `&&` lexes as `AMP_AMP`.
- `@` (bind-pattern sigil) — not a token; `BindPat` AST stub exists
  without a parser path.

---

## 11. Known ambiguities and their resolutions

- `Name { … }` in expression position — struct literal vs block body.
  Resolved by a context flag set False at the heads of
  `if`/`while`/`for`/`match`, re-enabled inside nested blocks. Inside
  those heads, `Name { … }` always parses as path + block, never as a
  struct literal.
- `::<T>` vs `<T>` for generic args — type position uses `<T>` bare;
  expression position (turbofish) requires `::<T>` so the parser can
  distinguish from the less-than operator.
- `|x| …` closure vs bitwise-or — closure only in primary-expression
  position; bitwise-or only in binary-operator position (both
  operands already parsed).

---

*This document is normative. If the shipped parser diverges from a
production above, treat the divergence as DRIFT and either fix the
parser or amend this document — see `LANG_AUDIT_2026_04.md`.*
