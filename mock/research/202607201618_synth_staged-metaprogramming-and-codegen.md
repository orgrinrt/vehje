# Staged metaprogramming and code generation: how you generate a runtime from a definition

**Date:** 2026-07-20
**Domain:** generating an interpreter, a backend, or a whole language runtime from a declarative or staged
definition. What crosses the generation boundary, when an unrepresentable value fails, and how a proof or a
totality guarantee can be compiled into generated structure rather than re-checked at run time.
**Synthesised from:** `prior_art/01_lms_delite.md`, `prior_art/06_mlir_ods_nanopass.md`,
`prior_art/07_rust_final_encoding_arenas.md`, `prior_art/08_terra_procmacro_comptime.md`, and the desugaring
mechanism in `prior_art/02_jsonnet_dhall.md`.

## Why this domain matters

Every system here answers one question in a different key: given a description of a language (its operations,
its grammar, its types), how do you produce the machinery that runs or compiles programs in it, without
hand-writing that machinery per language and per target? The answers range from a Scala library that turns
ordinary-looking code into an IR graph (LMS), through build-time record expanders (MLIR ODS, nanopass), to
compile-time evaluators that produce more of the same language (Zig comptime) or a different one (Terra, Rust
proc-macros). Read together they map the design space for "runtime generation": what the generator consumes,
what it emits, what is guaranteed by construction, and what is merely tested.

## Two mechanisms, repeatedly confused as one: staging versus generation

LMS (Lightweight Modular Staging, Rompf and Odersky, GPCE 2010) is the cleanest statement of *staging*. Every
staged value has type `Rep[T]` instead of `T`, and `Rep` is not a wrapper struct but a type alias onto the IR
expression type itself (`type Rep[+T] = Exp[T]`). Overloaded operations build an IR node instead of computing
a result: `Rep[T] + Rep[T]` resolves through an implicit conversion to a smart constructor `numeric_plus`
returning a `NumericPlus` node, and the actual interception point is one implicit, `toAtom`, that coerces any
`Def` into an `Exp` by calling `findOrCreateDefinitionExp`. The load-bearing consequence: common-subexpression
elimination is not a pass. Because every IR node `Def` is a Scala case class built from already-staged
operands, two structurally-equal constructions compare `==`, and `findOrCreateDefinition` reuses the existing
symbol on a hit. **CSE falls out of case-class structural equality plus a lookup on construction, with zero
extra bookkeeping** (`prior_art/01`, "the core representation" and "how staging intercepts operations").

The tax of that elegance is instructive. It requires a GC'd host with cheap structural `equals`/`hashCode`
over immutable trees; a no-alloc port needs an explicit interner (arena plus hash-consing table) to get the
same lookup, and every `Def` must be built only from already-staged operands or two "same" nodes silently
fail to compare equal and CSE misses. Native control flow (`if`/`while`/`var`) additionally needs
`scala-virtualized`, a compiler fork rewriting `if (c) a else b` into `__ifThenElse(c, a, b)` at parse time,
because ordinary operator overloading cannot intercept `if`. This is the seam where "staging as a library"
stops being free.

Zig `comptime` is staging without any of that machinery, and it is worth holding as the opposite pole. It is
not a language, not a token facility: it is an evaluation-time qualifier on ordinary Zig code, so the same
functions and control flow run either at compile time (via the compiler's own interpreter) or at run time.
Generics are just functions taking `comptime T: type` and returning a `type` value; `@typeInfo`/`@Type`
reflect a type as an ordinary comptime tagged union. There is no quasi-quote, no `Rep[T]`, no macro system,
because generics, constant folding, and conditional specialization all reduce to comptime-parameterized
functions plus `inline for`. What that buys and what it gives up is stated flatly in `prior_art/08`: comptime
"only ever produces more Zig," there is no way to emit surface syntax the host grammar does not already
express, and no cross-language target. A comptime-known value becomes runtime data at any downstream point
where nothing forces it to stay compile-time-known; there is no lower/splice operator. The one guard rail is
the backwards-branch budget (default 1000, `@setEvalBranchQuota(n)` to raise), which ratchets only upward and
must be set from the top of the governing comptime stack.

## What crosses the generation boundary, and when a bad value fails

The sharpest cross-source comparison in `prior_art/08` is the table of "generation-time value into generated
output." It is the reference for anyone designing a metacompiler seam:

- **Terra** stages Terra-code construction in Lua: functions, types, and quotes are first-class Lua values.
  A `` `expr `` backtick yields one unevaluated Terra expression; `quote ... in expr end` groups statements
  into an expression-valued block; `[luaexpr]` splices a Lua expression's result "where any expression or
  statement normally appears." What crosses is a Terra term, converted via LuaJIT's FFI both ways. A value
  outside the subset of Lua values that are Terra terms fails at *specialization* time, which is eager, so it
  is a compile-time error at definition, never a runtime failure (rule `SESC`). Crucially, "Terra executes in
  a separate environment: Terra code runs independently of the Lua runtime," and `terralib.saveobj` emits a
  standalone `object|asm|bitcode|llvmir|executable` artifact linkable into a normal C executable. The stager
  (Lua) is not present in the generated artifact. This is the precise shape of "the generator is dev-time
  only; only the generated artifact ships."
- **Rust proc-macros** map `TokenStream -> TokenStream` with no type information, no view of other items'
  bodies, no semantic analysis. What crosses is tokens, never a live object graph; interpolation via `quote!`
  writes any `ToTokens` value with `#var`, and a value with no `ToTokens` impl fails to compile the
  *macro-authoring* crate, never the generated code. Hygiene is a `Span` choice: `call_site` (resolve as if
  written at the call), `mixed_site` (`macro_rules!`-style), `def_site` (nightly). The macro runs as an
  ordinary build-time process with full I/O access, so non-determinism during expansion is unconstrained by
  anything structural; the only outputs rustc accepts back are Rust tokens, but the input language and side
  effects are unconstrained (real cases: `naga-to-tokenstream` emits a Rust module from a WGSL shader;
  `wgsl_preprocessor` runs C-style preprocessing over WGSL inside a macro).
- **Zig comptime**: nothing crosses; one evaluator, two times.

The pattern across all three: a generator that emits code should make the unrepresentable-value failure a
*generation-time* error (Terra's eager specialization, the missing `ToTokens` bound), not a runtime one, and
the generated artifact should carry no dependency on the generator (Terra's separate environment, the
proc-macro's produced tokens). This is the disciplined version of "runtime generation": the proof or the
type-check happens while generating, and only the checked output ships.

## Generation from a declarative record: ODS and nanopass

MLIR's Operation Definition Specification (ODS) and the nanopass framework are the build-time-generator end of
the space, and both are about killing per-definition boilerplate.

ODS is a TableGen DSL: an operation is a `.td` record naming its arguments, results, traits, and optionally an
assembly format and verifier; `mlir-tblgen` expands it into a C++ `mlir::Op` specialization with named
accessors, builders, and verification hooks. The measured contrast in `prior_art/06`: the `AddOp` example is
ten non-comment lines of `.td`; the hand-written C++ equivalent for a *zero-field* `ConstantOp` is roughly 35
lines of declarations alone before any `.cpp` bodies, and from the empty ODS form the `ZeroOperands`/
`OneResult` traits "will be automatically inferred based upon the `arguments` and `results` fields." The
generator emits named accessors (`getLhs()` replacing `getOperand(3)`), an operand adaptor for name-based
access from a bare `Value` array, multiple builder overloads, and, when requested, `verify()`/`fold()`/
`getCanonicalizationPatterns()`. A dialect's floor is four files (one `.td`, one `.h`, one `.cpp`, one
`CMakeLists.txt` with the `mlir_tablegen` invocations).

Nanopass is the same idea for whole compiler passes. `define-language` declares a context-free grammar
(terminals, nonterminals, productions), optionally as a diff against a prior language via `extends` with `-`
and `+` clauses; it generates a record type per production, a predicate, constructors, and an unparser.
`define-pass` writes a transformation supplying only the clauses that change, and the framework autogenerates
pass-through clauses for every unmentioned production, recursing into subterms. The load-bearing ergonomics
claim, verbatim from the user guide: "Passes often contain boilerplate code to recur through otherwise
unchanging language forms," and the mechanism removing it is grammar-driven default-clause generation (the
catamorphism support credited to Erik Hilsdale). The measured validation: replacing 5 of Chez Scheme's 10
back-end passes with "over 50 nanopasses" across roughly 35 intermediate languages produced code 15-27% faster
at compile times 1.64-1.75x the original; the framework's own two macros are "approximately 4600 lines."

Both carry a warning worth transplanting. Nanopass's own TODO file names two structural gaps: removal patterns
must match variable names "EXACTLY... without the error is a very rough edge," and output forms "need to match
original language forms very closely," with a concrete failure where a `(begin e* ... e)` production cannot be
built from a list because the framework "sees this as a single form instead of a list," forcing a hand-written
`reverse`/`car`/`cdr` workaround. MLIR's DRR (declarative rewrite rules) has an explicit unsupported-construct
list (regions, block arguments, multi-result nested patterns, variadic nested patterns) and an escape hatch
`NativeCodeCall` dropping to C++, which the docs frame as the reason for the move to PDLL. The lesson: a
grammar-driven or record-driven generator handles the regular 90% and needs a clearly-marked escape hatch for
the rest, and the escape hatch's existence is not a defect but the honest boundary of the declarative form.

## Typed generation: certify the generator once, trust the output

The most important idea for a metacompiler that must preserve a proof is that a *typed* generator can make its
output correct by construction, certified once when the generator itself is type-checked. Three sources carry
pieces of it.

`Rep[T]` in LMS is the archetype: the generator manipulates typed representations, so `L::add(L::int(1),
L::int(2))` type-checks and `L::add` applied to a `bool` does not. The Rust tagless-final write-ups
(`prior_art/07` §1) make the same point in Rust: with a GAT `type Repr<T>`, "if you pass `L::int(1)` to
`L::add` it's a compile time error," and the interpreter is selected by monomorphization (`fn term<E:
ExprSym>() -> E::Repr`), one instantiation per backend, no runtime tag. Terra's `symbol()` mints a fresh
unique name per call to violate lexical scoping safely, and specialization mints a fresh symbol per bound
variable (rules `LTDEFN`/`SLET`) to prevent capture; hygiene is a property the generator guarantees, not a
runtime check.

The synthesis for "compile a proof into generated code" reads directly off this material: the generator is
written so that a well-typed generator run can only emit correct output (the `Rep[T]`/`ToTokens` discipline,
Terra's eager specialization), and the correctness is then a property established when the *generator* is
checked, at generation time, with nothing to re-check at run time. This is the Lightweight-Modular-Staging and
typed-quasiquote lineage. Its honest limit, stated across `prior_art/07` and `08`: Rust has no higher-kinded
types, so the fully general tagless-final term generic over its interpreter cannot be stored as a value and
replayed against N interpreters without monomorphizing N times, and a heterogeneous runtime-built term generic
over the interpreter "is very awkward to implement" (the users.rust-lang.org thread notes the missing
`impl for<E: Exp> Fn(&E) -> E::Repr` HRTB-over-type-parameters). The escape hatch every real Rust compiler
took instead is in the next domain doc: parse into a concrete arena-held tree once, then fold it generically.

## Runtime code compilation, and why it usually does not transfer

LMS's `compile[A,B]` invokes the real Scala compiler at run time (`scala.tools.nsc.Global`, a
`VirtualDirectory`, an `AbstractFileClassLoader`) to turn generated source back into a callable in the same
process. Convenient for benchmarking staged code against native in one run, and entirely dependent on a hosted
compiler plus a classloader. `prior_art/01`'s transfer section is blunt: there is no analogue "load bytecode
back into the same process" step for a statically-compiled, no-alloc target. This is the concrete reason a
no-Rust-at-the-embed-site design cannot lean on the Futamura-projection-at-runtime trick that Truffle/GraalVM
uses; the specialization has to happen ahead of time, and only the specialized artifact ships (the Terra
`saveobj` shape, not the LMS `compile` shape).

## The recurring tradeoff table

| Mechanism | What it generates from | What it emits | Bad-value failure | No-alloc transfer |
|---|---|---|---|---|
| LMS `Rep[T]` staging | overloaded Scala over `Rep` | IR graph, then per-backend text | typed at stage time | needs explicit interner + fixed arenas; CSE-by-equality needs hash-consing |
| Zig comptime | ordinary Zig, `comptime` qualifier | more Zig | op-level rejection (e.g. deref of comptime ptr) | native; one evaluator, no marshalling |
| Terra quote/escape | Lua staging of Terra terms | standalone `.o`/asm/bitcode | compile-time, eager specialization | generated artifact is alloc-free; the Lua stager is not shipped |
| Rust proc-macro | `TokenStream` | Rust tokens (any input lang) | compile-time, missing `ToTokens` on macro crate | generator is build-time; `TokenStream` is `Rc<Vec>`-backed, allocates during expansion only |
| MLIR ODS | `.td` record | C++ op class | build-time tblgen | generation transfers; the generated runtime object model does not without an arena/interner redesign |
| nanopass | grammar + changed clauses | record types + pass | rough edges (exact-name match, list-vs-form) | Scheme homoiconicity does not transfer; the record+predicate+default-clause idea maps to a derive |

## Sources

Primary synthesised docs:
`202607190650_mockspace-procedural-docs-arc/prior_art/01_lms_delite.md`,
`.../06_mlir_ods_nanopass.md`, `.../07_rust_final_encoding_arenas.md`,
`.../08_terra_procmacro_comptime.md`, `.../02_jsonnet_dhall.md` (desugar-to-small-core).

Key underlying citations carried by those docs: Rompf and Odersky, "Lightweight Modular Staging," GPCE 2010;
Rompf et al., "Optimizing Data Structures in High-Level Programs," POPL 2013; Sujeeth et al., "Delite," TECS;
DeVito et al., "Terra: A Multi-Stage Language for High-Performance Computing," PLDI 2013; Keep and Dybvig,
"A Nanopass Framework for Commercial Compiler Development," ICFP 2013; MLIR ODS/Traits/Interfaces/DRR/PDLL
docs at mlir.llvm.org; the Rust `proc_macro`/`quote`/`syn` reference; the Zig language reference on comptime.
