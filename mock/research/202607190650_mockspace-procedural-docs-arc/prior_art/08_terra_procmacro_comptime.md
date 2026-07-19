# Terra, Rust proc macros, Zig comptime: generation-time to output-value

## What each is, in three sentences

**Terra** is a statically-typed, C-like low-level language embedded in Lua: Lua stages Terra's
construction (functions, types, quotes are first-class Lua values), Lua and Terra share one lexical
environment, and Terra executes in a separate environment with no runtime dependency on Lua. **Rust
procedural macros** are a separate `proc-macro`-type crate, dynamically loaded into rustc, that maps
`TokenStream -> TokenStream` with no access to type information, no view of other items' bodies, and no
semantic analysis of the surrounding crate. **Zig `comptime`** is not a separate language or a token
facility: it is an evaluation-time qualifier on ordinary Zig code, so the same functions and control flow
run either at compile time (via the compiler's own interpreter) or at runtime, and generics are just
functions returning `type` values evaluated at comptime.

## Terra: quote / escape / symbol mechanics

Sources: terralang.org (getting-started, api.html); DeVito et al., PLDI'13.

- **Backtick, short quote**: `` `terraexpr `` yields one unevaluated Terra expression as a Lua value.
- **`quote ... end`**: groups statements; an `in expr end` tail makes the block expression-valued:
  `quote var a = foo() in a end`.
- **`[expr]` escape**: splices a Lua expression's result "where any expression or statement normally
  appears." Shapes: expression escape, statement escape, identifier escape (`var [luaexpr] = 4`), and
  multi-value escape (returning a Lua array splices multiple values).
- **`symbol(typ, [name])`**: "abstract representations of Terra identifiers," a controlled, explicit
  violation of lexical scoping producing a fresh unique name each call; used as `var [a] = 3` where
  `a = symbol()`.
- **Shared lexical scope, hygienic by default**: "Terra ensures that variable references are hygienic,"
  and "Lua and Terra... share the same lexical scope." The formal model (Terra Core) evaluates Lua via
  `e ->^L v`; a Terra term triggers specialization `->^S` ("analogous to macro expansion in LISP that
  evaluates escapes... to produce concrete Terra terms"), then execution `->^T`. Rules `LTDEFN`/`SLET` mint
  a fresh symbol per bound variable during specialization specifically to prevent capture.
- **Eager specialization, lazy typecheck/link**: "we perform specialization eagerly... while we perform
  typechecking and linking lazily (only when a function is called...)." A worked example shows Lua-side
  mutation between definition and use is captured by value at specialization time, not live reference.

## What crosses the Terra/Lua boundary, and failure modes

- **Lua -> Terra**: arguments "are converted into Terra types," following "LuaJIT's foreign-function
  interface" rules.
- **Terra -> Lua**: "primitive types like `double`... converted to their respective Lua type," aggregates
  "boxed in a LuaJIT `ctype`."
- **Unrepresentable escape value**: escapes are evaluated during specialization (rule `SESC`: "splices the
  result... if the resulting value is in the subset of values that are Terra terms"). A value outside that
  subset fails at specialization/typecheck time, i.e. a compile-time error at definition, since
  specialization is eager, never a runtime failure.
- **No residual Lua dependency**: "Terra executes in a separate environment: Terra code runs independently
  of the Lua runtime. It can run in a different thread, or... on accelerators." `terralib.saveobj(filename,
  filetype, functiontable, ...)` with `filetype` one of `object|asm|bitcode|llvmir|executable` emits a
  standalone artifact; the paper links a saved `.o` "to a normal C executable." Type reflection
  (`type:isprimitive()`, `type:ispointer()`, `terralib.types.pointer(typ)`) is entirely Lua-side; nothing
  equivalent exists inside compiled Terra.

## Rust proc macros: execution model and hygiene

Sources: doc.rust-lang.org/reference/procedural-macros.html; proc_macro::Span docs.

- **Compiled artifact**: "must be defined in the root of a crate with the crate type of `proc-macro`"
  (`proc-macro = true` in `[lib]`); "may not be used from the crate where they are defined."
- **Invocation, all token-to-token**: function-like macros get "what is inside the delimiters" as input
  and replace the whole invocation with the output; attribute macros get the attribute's own args plus
  "the rest of the item"; derive macros get the annotated item's tokens and must return "a (possibly empty)
  set of items" appended alongside the original.
- **`TokenStream`**: "roughly equivalent to `Vec<TokenTree>`," "cheap to clone" (the `proc_macro2` reimpl
  used by syn/quote is concretely `Rc<Vec<TokenTree>>`).
- **Hygiene via `Span`**: "procedural macros are unhygienic... behave as if the output token stream was
  simply written inline." `Span::call_site()` (stable 1.29): identifiers "resolved as if... written
  directly at the macro call location." `Span::mixed_site()` (stable 1.45): `macro_rules!`-style,
  resolves "local variables, labels, `$crate`" at def-site and "everything else" at call-site.
  `Span::def_site()` (nightly, `proc_macro_def_site`): resolves fully at definition site.
- **Why token-to-token, and what's unreachable**: macros "operate over token streams instead of AST nodes,
  a far more stable interface... for both the compiler and for procedural macros." No type information, no
  other items' bodies, no semantic model, because input is exactly the invocation's token span and macro
  expansion runs before full semantic analysis. As a function it "must either return syntax, panic, or loop
  endlessly"; panics become compiler errors, infinite loops hang the compiler unguarded. It runs as an
  ordinary process at build time with full compiler-level resource access (stdin/stdout/file access), so
  arbitrary I/O or non-determinism during expansion is unconstrained by anything structural.

## `quote!` / `syn`: interpolation and `ToTokens`

- **Syntax**: `#var` ("similar to `$var` in `macro_rules!`"). Any `ToTokens` value can be interpolated,
  "most Rust primitive types as well as most of the syntax tree types from Syn."
- **Trait** (docs.rs/quote):
  ```rust
  pub trait ToTokens {
      fn to_tokens(&self, tokens: &mut TokenStream);
      fn to_token_stream(&self) -> TokenStream { ... }
      fn into_token_stream(self) -> TokenStream where Self: Sized { ... }
  }
  ```
- **No-impl case**: fails to compile the macro-authoring crate itself at expansion (missing trait bound on
  `#var`), never the downstream generated code; no stringification fallback exists.
- **Non-Rust artifacts, real cases**: `naga-to-tokenstream` takes a WGSL/GLSL shader and emits a Rust
  module of types + bind-group metadata; `wgsl_preprocessor`/`wgpu-pp` run C-style (`#include`/`#define`)
  preprocessing over WGSL text inside macro/`build.rs` codegen. Output is still Rust `TokenStream` (rustc
  accepts nothing else back), but input language and side effects (writing files, invoking `naga`) are
  unconstrained.

## Zig comptime: model and limits

Sources: ziglang.org/documentation/master; corroborating community docs for points the fetch truncated.

- **Same syntax, different evaluation time**: "a local variable... qualified with `comptime`... causes
  the variable's value to be comptime-known, and all loads and stores... happen during semantic analysis...
  rather than at runtime." Variables inside a `comptime` block are implicitly comptime; a `const` bound to
  a comptime-known initializer is itself comptime-known.
- **Generics as comptime-parameterized functions returning `type`**:
  ```zig
  fn List(comptime T: type) type {
      return struct { items: []T };
  }
  ```
  Types are ordinary comptime `type` values, computable and passable like any other value; this one
  mechanism substitutes for both generics and a macro system.
- **Reflection**: `@typeInfo` returns a tagged union describing a type's shape as a normal comptime value,
  inspectable with ordinary control flow; `@Type` constructs a type from such a description.
- **Branch quota**: comptime evaluation runs under a backwards-branch budget, default 1000; exceeding it
  errors "evaluation exceeded 1000 backwards branches," directing use of `@setEvalBranchQuota(n)`. The
  quota only ratchets upward (a smaller call is ignored) and must be called from the top of the governing
  comptime stack, a placement rule several tracked Zig issues call unintuitive.
- **Comptime pointers**: "Zig is able to preserve memory addresses in comptime code, as long as the pointer
  is never dereferenced" — carry and compare, never read through, since the memory may not exist yet.
  Beyond this the langref does not centralize one "forbidden at comptime" list; anything depending on
  not-yet-existing runtime layout or true I/O is rejected case by case, not tabulated.
- **Comptime value into runtime data**: no lower/splice operator; a comptime-known value simply becomes
  runtime data at any point downstream where nothing forces it to stay compile-time-known (stored into a
  runtime field, passed as a non-comptime argument, baked into a monomorphized length).
- **Why no macro system, and the tradeoff**: generics, constant folding, and conditional specialization all
  reduce to comptime-parameterized functions plus `if`/`inline for` over comptime values, so Zig has no
  `macro_rules!`, no quasi-quoting, no token-level AST surface. Given up: no way to emit surface syntax the
  host grammar doesn't already express, and no cross-language target; comptime only ever produces more Zig.

## Compared: generation-time value into generated output

| | Terra | Rust proc macro | Zig comptime |
|---|---|---|---|
| Crossing unit | Lua value via `[expr]` splice into a Terra term | `ToTokens` value written via `#var` | Nothing crosses; one evaluator, two times |
| Representation | Terra term (LuaJIT FFI conversion both ways) | Tokens only, never a live object graph | N/A |
| Unrepresentable-value failure | Compile-time, at specialization (eager) | Compile-time, missing `ToTokens` on the macro crate | N/A; op-level rejection (e.g. deref) |
| Hygiene | `symbol()` renaming, formalized in Terra Core | `Span` tiers: call-site / mixed-site / def-site | Not applicable, no expansion step |
| Output language constraint | Only Terra terms/types | Only Rust tokens back to rustc (input/side-effects unconstrained) | Only more Zig |

## Generator allocation

Terra's generator is Lua/LuaJIT: tables, closures, and boxed `ctype` values are heap objects by
construction, incidental to Terra's own semantics (only the *emitted* Terra/machine code is
allocation-free). Rust's `proc_macro`/`proc_macro2` `TokenStream` is `Vec`- or `Rc<Vec<_>>`-backed
(maintainers note the `Rc<Vec<_>>` shape "makes twice as many allocations as it should"); every invocation
builds new token vectors, so the process allocates continuously, incidental to the token model, not
required by it. Zig's comptime interpreter runs inside the compiler's own semantic-analysis memory
management; it is not documented allocation-free, but it is the one case where a value never leaves the
interpreter's world to cross a serialization boundary, so no separate marshalling allocation sits on top.

## Sources

terralang.org/getting-started.html, terralang.org/api.html, terralang.org/publications.html; DeVito et al.,
*Terra: A Multi-Stage Language for High-Performance Computing*, PLDI'13
(cs.stanford.edu/~zdevito/pldi071-devito.pdf); doc.rust-lang.org/reference/procedural-macros.html;
doc.rust-lang.org/proc_macro/struct.{Span,TokenStream}.html; docs.rs/quote (macro.quote.html,
trait.ToTokens.html), docs.rs/proc-macro2; ziglang.org/documentation/master (comptime, builtins);
github.com/ziglang/zig issues #1767/#11996/#12624/#19525; lib.rs entries for naga-to-tokenstream,
wgsl_preprocessor, wgpu-pp.
