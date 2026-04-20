# Brief — #95 Macros slice 3: user-authored macro declarations

**Status**: APPROVED 2026-04-18. Ready to implement.
**Follows**: slices 1 + 2 (already shipped). See `docs/clause/DESIGN.md` §21.6.
**Supersedes**: earlier draft that proposed a splice DSL (rejected — we don't
invent divergent template languages when our own grammar already covers it).

## Locked decisions

- **Path 1** (interpret Clause IR) — Path 2 (Clause→Python transpile)
  is noted as its own future batch tied to the cross-target IL vision.
- **Declaration: no `!`, invocation: yes `!`**. `macro name(...)` is the
  declaration; `name!(...)` is the invocation.
- **Return type mandatory**. A `macro` with no return type is a parse
  error. Body must return a value compatible with the declared return
  type or it's a type error. Macros are regular Clause fns with extra
  macro wiring; they don't get special "return type optional" treatment.
- **Interpolation**: `$ident` for bare names, `$(expr)` for arbitrary
  expressions.
- **`quote!` and friends live in a `macros` crate, not `std`**. The
  `macros` crate is part of the std ecosystem (shipped, implicitly
  available) but separate from `std` itself — keeps `std` focused on
  runtime/language primitives, `macros` focused on compile-time
  metaprogramming. Applies to any other macro-ecosystem helpers too
  (token builders, quote interpolation, etc.).
- **Visibility fully generalized**: `pub macro`, `pub(crate) macro`,
  `pub(super) macro`, private. No macro-specific visibility rules.

## Why

Slices 1 + 2 ship macro invocation + a Python-backed handler registry.
All macros live in compiler Python today. Slice 3 lets crate authors
**write macros in Clause itself** — macro bodies are real Clause code
that runs at compile time and produces AST for the call site.

## The shape

A user macro:

```clause
pub macro event_family(prefix: Ident, chapters: List[Int]) -> TokenStream {
    let mut out = TokenStream::new();
    for n in chapters {
        let id = ident(prefix, "_", n);
        out.extend(quote! {
            event {
                id = $id
                trigger = { always = yes }
            }
        });
    }
    out
}
```

Everything inside the body is **normal Clause syntax** — `let`, `for`, `if`,
function calls, method calls. No splice DSL. No template-level keywords.
What makes this a macro is the `macro` item kind + the signature that
takes/returns `TokenStream`.

The only primitive that LOOKS template-ish is `quote! { ... }` — and that's
a compiler-provided macro in the stdlib, handled by the same slices-1-2
registry. Other token-building helpers (`ident`, `concat_tokens`, etc.) are
stdlib functions that operate on `TokenStream` / `Ident` / `Literal` — same
as any other stdlib API. No parallel template grammar.

Visibility follows the same rules as every other item: `pub`, `pub(crate)`,
`pub(super)`, or private — unified with `fn`, `struct`, `trait`, `const`.
Importable via `use crate::macros::event_family;`. No macro-specific
visibility exceptions.

## Running the body at compile time

Clause compiles to Clausewitz — which is data, not executable. So macro
bodies can't be "compiled and called" the same way other code is. We need
a compile-time runtime.

Two viable paths:

### Path 1 — Interpret Clause IR in Python

At crate-load, each `macro` item's body is parsed to Clause IR (which we
already do for all Clause source). A new module `macro_interpreter.py`
walks that IR and evaluates it, mapping each construct to an equivalent
Python operation:

- `let x = a + b;` → Python variable assignment with integer/string ops.
- `for t in tokens { ... }` → Python `for` over an iterable.
- `if cond { ... } else { ... }` → Python branch.
- Method calls on stdlib types (`tokens.push(t)`, `ident.to_str()`, etc.)
  → dispatched to a Python-side stdlib shim that implements each type.
- Function calls to stdlib free functions → dispatched to shim registry.
- The `quote! { ... }` invocation is an expression-position macro from
  the slice-1/2 registry; its handler already knows how to turn the
  inner tokens into a TokenStream, including `$ident` interpolation.

The interpreter is a **tree-walking evaluator over Clause IR**. Scope is
only "enough Clause to write macros": arithmetic, string ops, loops,
conditionals, method/function dispatch against a shim registry of
stdlib types (`TokenStream`, `Ident`, `Literal`, `List`, `String`, numerics).
No user-defined structs/traits/impls in macro bodies for v1 — macros
compose via stdlib primitives.

- ➕ Minimal surface: we already have the Clause parser and AST; the
     interpreter is a ~500-line visitor over existing IR nodes.
- ➕ Errors point at Clause source with proper spans (the IR carries
     them through).
- ➕ Extensible: the shim registry is how stdlib types expose ops to
     macros; adding a new op is adding a method handler to the shim.
- ➖ Interpreter means macro performance is slow-ish. Fine for macro
     expansion; not a concern at codegen scale.
- ➖ Semantics divergence risk: the interpreter's `+` has to match the
     type-checker's `+`. Testable; not design-blocking.

### Path 2 — Transpile Clause → Python, then `exec`

Alternative: treat Clause as a general intermediate language whose
grammar framework can target multiple outputs (Clausewitz being the
primary one, Python being another). A new transpile pass emits Python
source for each macro body; at crate-load we `exec` that Python in a
sandboxed namespace populated with the stdlib shims, register the
resulting callable as a handler.

- ➕ Unlocks the broader "Clause as cross-target IL" story — once
     we can emit Python from Clause, the same framework could target
     other script grammars for other PDX / non-PDX games.
- ➕ Execution speed matches native Python (no interpreter overhead).
- ➖ Much larger v1: designing a Clause→Python grammar pairing is its
     own chunk of work. Spans are indirect (Python traceback → map
     back to Clause source).
- ➖ Security: `exec`-ing generated Python from untrusted crate source
     needs real sandboxing. Dodgy.

## Recommendation

**Path 1 (interpret Clause IR)** for slice 3 v1. It's the smaller,
more surgical step, and everything about it aligns with the existing
compiler architecture (we already walk IR in a dozen passes). Path 2
is a genuinely interesting future direction (the "Clause as
general-purpose IL" vision) and should be its own batch, designed in
context of what the second/third target grammar would be.

Either path lands at the same user-facing outcome: a `macro` item,
body is normal Clause code, visibility is unified with other items, no
DSL.

## Scope of slice 3 v1 (path 1 chosen)

In:
- Parse `[vis] macro NAME(PARAMS) -> TYPE { BODY }` as a new
  `MacroDecl` item. Visibility parses like any other item.
- Type checker validates the signature: params must be recognized
  stdlib types (Ident, Literal, TokenStream, List<T>, String, Int,
  Bool, etc.); return type must be `TokenStream` (what gets spliced
  at the call site).
- `macro_interpreter.py` — tree-walking evaluator over Clause IR.
  Evaluates bodies against a per-call environment. Dispatches method
  and function calls to a shim registry.
- Macro-runtime types + `quote!` macro all live in a new `macros` crate
  (part of the std ecosystem, separate crate from `std`):
  - `TokenStream`: `new()`, `push(tok)`, `extend(other)`, `len()`, iteration.
  - `Ident`: `from_parts(a, b, ...)`, `to_string()`, equality.
  - `Literal`: string/int/bool constructors, `to_string()`.
  - Compiler-provided `quote! { ... }` macro with `$name` /
    `$(expr)` interpolation. Registered in the slice-1/2 registry so
    it's invoked like any other macro.
  - `List<T>`, `String`, `Int`, `Bool` — ops are in `std`.
- Crate-load: each `MacroDecl` is registered into the slice-1/2
  macro registry, binding name → a closure that invokes the
  interpreter on the body with the call's tokens bound to the
  declared params.
- Unified visibility: `pub macro`, `pub(crate) macro`, private. Use
  declarations can import macros like any other item.

Out (deferred beyond v1):
- User-defined structs/traits/impls inside macro bodies.
- Path 2 transpile-to-Python approach.
- Qualified macro paths at the call site (`crate::macros::foo!`).
  Slice-1/2 already rejects these; still rejected in slice 3.
- Recursive macros. A follow-up can add recursion with a depth cap.
- Macros that emit other macros. (Type-checker doesn't block it; we
  just haven't exercised the path.)

## Open questions — all resolved 2026-04-18 (see Locked decisions above)

## Verification

- Parse `pub macro foo(t: TokenStream) -> TokenStream { t }` → `MacroDecl`.
- Minimal runtime test: identity macro — invoked, returns its input
  tokens unchanged.
- Stdlib shim tests: `Ident::from_parts`, `TokenStream::push`, etc.
- Integration: a fixture crate declares `event_family!`, invokes it
  with real data, `clause build` emits the expected event items.
- Error paths: macro body references an undefined identifier; param
  type mismatch at call; quote! interpolation of missing `$name`.
