# Sketch: where the data/behavior line falls in the composed runtime (L3)

## Hypothesis

The composed language runtime (L3) can be all-data-driven: one general engine (L2) that interprets
per-language data, with no per-language behavior code. If that is false, this trace should show precisely
which stage resists, so the "beyond data" need is localized rather than assumed everywhere.

## Framing: the real axis

op dissolved the false half of the fork. "Data as a Zig struct" versus "data as a serialized blob" is a matter
of taste, because the metacompiler can emit per-language data as Zig consts and structs just as well as a blob.
That distinction is not the fork. The fork is whether a stage is a general engine reading per-language data, or
whether it is per-language behavior that has to exist as logic. So every stage below is classified on one axis:
ENGINE (general, authored once in L2), DATA (per-language, emitted by L1, whatever its serialized shape), or
BEHAVIOR (per-language logic that is not reducible to data over a fixed engine).

## The traced script

A tiny illustrative language L, one script that exercises the whole pipeline: a binding, a runtime-environment
read, a literal, a family node, a record value, a lease question, and an output.

```
let user = { name: read_input(), score: 42 }
emit(user)
```

`read_input` performs a runtime-environment effect and may fail (so its result is a `Maybe`). `{ ... }` builds
a record value. `emit` sends a value out through the value-arena and sink. `user` is bound in the top-level
body. This is small but it touches parse, resolve, mask checking, lease inference, the compile stage, codegen,
and execution.

## The stage-by-stage trace

**0. Parse (text to IR).** A table-driven parser is the classic all-data shape: the parse engine is general,
the grammar is a table. ENGINE reads DATA (the grammar) and produces IR. The one place this strains is
context-sensitive lexing (significant indentation, custom literal syntaxes), which can want a small
per-language lexer hook. That hook is bounded BEHAVIOR, and only some languages need it. Verdict: DATA plus a
general ENGINE, with a narrow optional lexer-hook the only behavior pressure.

**1. Resolve (names to slots, scope chain).** The walk is a general traversal maintaining a scope stack, which
is ENGINE. What each node form does to scope (a `Let` introduces a binding whose scope is its body) is a
per-node rule, which is DATA. Slot assignment is general. Verdict: all-data over a general engine.

**2. Check, the family and effect masks.** Each node form carries a family id and an effect classification; the
script's used sets are the union of those; the check is used-set is a subset of the language's supported and
permitted sets. The masks are DATA. The union and the subset test are a general bitmask operation, ENGINE. This
is the archetypal all-data stage, and it is exactly the "mask version" op named. Verdict: pure DATA plus a
general bitmask ENGINE, no behavior at all.

**3. Lease and region inference.** `user` is bound in the top-level body, so its default region is the program
body. `read_input()`'s result flows into `user.name` in the same region, no escape. `emit(user)` consumes
`user`; its lease is the emit, and the value crosses into the output region. The inference algorithm (assign a
default region per binder, propagate, detect an escape when a reference flows to a longer-lived consumer, unify
or raise a compile error) is a general algorithm, ENGINE. What varies per language is the region rules (a
`Let`'s default region, `emit`'s region requirement, whether the language allows an explicit annotation), which
is DATA. The algorithm is uniform across languages precisely because we chose region inference without
borrow-exclusivity; only the rules change. Verdict: all-data rules over a general algorithm. The one way this
would grow behavior is a language wanting genuinely different lifetime semantics (linear or borrowing), which
would vary the algorithm itself. None of the census languages want that.

**4. The compile stage: macros and const-eval (build-environment effects).** If the script had a macro or a
const expression, the compile stage evaluates it. Const-eval is the general interpreter (stage 6) run over the
build-environment subgraph. A macro is itself IR evaluated by that same interpreter, which is the existing
"macros are proc-driven, scheduler-pass-driven, typechecked" design. So macro expansion reduces to "the general
interpreter runs some IR," ENGINE over DATA. The only behavior pressure is a macro that needs host or native
capability rather than staying in-language, which is rare and is a deliberate boundary. Verdict: reduces to the
general interpreter, all-data.

**5. Codegen (IR to bytecode or native), for JIT and AOT.** This is where the useful surprise is. The codegen
backend (instruction selection, register allocation, IR to machine code) is per-target-ISA and
language-agnostic: an x86 or ARM backend is the same regardless of which language produced the IR. So the
backend is a general L2 component, authored once per ISA, heavy but shared across every language. The only
per-language part of codegen is the lowering of a family node to IR operations, which is a lowering table,
DATA. Verdict: a general per-ISA backend (ENGINE) plus per-language family-lowering DATA. The language-specific
part is all data; the heavy part is general and shared.

**6. Execute (interpret the IR or the codegen output).** Dispatch over node or opcode is a general switch,
ENGINE. The eleven Core forms have fixed semantics that live in L2, ENGINE. The crux is family-node runtime
behavior: what `concat`, `read_input`, `emit`, and record-construction actually do at runtime. This can be DATA
if a family node lowers to a composition of general primitives the engine already has (build a value-arena
record, perform an effect call, a string operation, arithmetic). It becomes BEHAVIOR only when a family needs a
primitive L2 does not provide. Verdict: family behavior is DATA (a lowering to a general primitive vocabulary)
up to the richness of that vocabulary, and BEHAVIOR beyond it. This is the single genuine locus of the fork.

## The finding

The whole pipeline is cleanly all-data (general L2 engines reading per-language data) at every stage except
one. Parse, resolve, mask-checking, lease inference, the compile stage, and even the codegen backend are
general engines parameterized by data, and the codegen backend is not even per-language, it is per-ISA. The
only place behavior pressure concentrates is family runtime semantics, and even there the behavior is data (a
lowering to general primitives) until a family needs a primitive the engine lacks.

So op's tension is real but it is localized to one place, not spread across the runtime. The choice is not
"all-data versus generated-Zig" for the whole engine; it is a narrow sub-question at family semantics: give L2
a rich enough general primitive vocabulary that families compose from it as data (all-data families, some
growth in L2's primitive set), or generate per-family behavior code (a lean L2, per-language behavior). op's
worry about "a generic data-processing engine at the head bloating" is exactly the first option's cost, and it
applies only to the primitive vocabulary, not to the whole engine.

## Recommendation

All-data for stages 0 through 5: general engines in L2 (the parser, the resolve walk, the mask checker, the
region-inference algorithm, the per-ISA codegen backend) reading per-language data emitted by L1. For stage 6
family semantics, a general primitive vocabulary in L2 that families compose from as data (a family node lowers
to a primitive sequence), with generated per-family behavior as a bounded, principled escape hatch used only
for a family whose semantics genuinely cannot compose without forcing L2's primitive set to grow past what a
minimal engine should carry.

This honors "ideally all data" (the pipeline is all-data), respects "some things are beyond data" (family
semantics can need a primitive L2 lacks), and minimizes bloat by keeping L2 a set of general engines plus a
bounded primitive vocabulary rather than a do-everything interpreter. It also lines up with the ratified
Core-closed, family-open split: Core forms are fixed engine behavior in L2, and a family extends the language
through a data lowering to primitives, with narrow generated behavior only where a primitive is genuinely
missing.

The "Zig struct versus blob" question then follows the composition choice, not the data-versus-behavior line.
When L3 is compiled together, the per-language data is Zig consts and structs; when a prebuilt engine loads a
language, the same data is a blob. Both are data, which confirms op's taste framing and folds that question
into the composition fork rather than the form fork.

## Outcome

WORKS as a logical trace: all-data is viable for the entire pipeline except family runtime semantics, where a
bounded L2 primitive vocabulary keeps it all-data with a narrow generated-behavior escape hatch. The fork
collapses from "how is the whole engine emitted" to "how rich is L2's primitive vocabulary, and where is the
escape hatch for irreducible family behavior."

## Next steps this unblocks

A code sketch that stands up a tiny general engine skeleton for stages 1 through 3 (resolve, mask-check, lease)
over a two-node toy family, to confirm the general-engine-over-data shape compiles and the region algorithm is
genuinely uniform. And an enumeration of the candidate L2 primitive vocabulary drawn from the real census
families (string operations for lua and jomini, record and list construction, effect calls, arithmetic), to
test whether that vocabulary stays bounded or sprawls. If it stays bounded, the escape hatch is rarely needed
and the runtime is effectively all-data; if it sprawls, the generated-behavior hatch earns its place for the
sprawling families.
