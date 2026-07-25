# Sketch: Clause runs in the runtime, with no Rust and no host

**Date:** 2026-07-26
**Outcome: WORKS.**
**Toolchain:** Zig 0.16.0, aarch64-apple-darwin.
**Tasks:** #44, #52

## Hypothesis

The canon puts every per-script analysis in the runtime artifact and states that
the Rust side never sees an end-user script:

> "a dev-time Rust language compiler (emits validated data and structural
> proofs, **never sees an end-user script**, never ships)"
> (`mock/research/canon/the-soul-of-vehje-positive-catalogue.md:45`)

> "**Rust never runs per-script analyses, the runtime does, for all scripts**"
> (`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md:38`, killing
> the dual-locus framing)

So a Clause program's whole path, from source text to a value, belongs inside the
Zig runtime. And per the lead designer's standing call 2, the host configures the
runtime and lends it allocations, and does not supply arithmetic. This sketch
tests both together: can source reach a value with neither Rust nor a host
anywhere in the path.

## Result

Yes. `zig test eval.zig`, 40 tests, all passing. The core of it:

```
1 + 2 * 3                                                   ==> 7
let x = 2; let y = 3; if x < y { x * 10 } else { y * 10 }   ==> 20
if 1 < 2 { let k = 4; k * 3 } else { 0 }                    ==> 12
let x = 2; let x = 9; x                                     ==> 9
x + 1                                                       ==> refused, Unbound

fn double(n) { n * 2 } double(3) + 1                        ==> 7
fn add(a, b) { a + b } add(4, 7)                            ==> 11
fn double(n) { n * 2 }
fn quad(n) { double(double(n)) } quad(5)                    ==> 20
fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } }
fact(5)                                                     ==> 120
fn adder(a) { fn inner(b) { a + b } inner } adder(3)(4)     ==> 7
fn add(a, b) { a + b } add(4)(7)                            ==> 11
```

Three of those are worth naming. `fact` terminates because `fn` lowers to a
binding in scope for its own value, so the call to itself resolves rather than
escaping. `adder(3)(4)` works because a closure names the environment it was
written in, which is why the environment is a linked chain rather than a stack
that pops. And `add(4)(7)` equalling `add(4, 7)` is not a feature: Core
application takes one argument at a time, so a multi-parameter function is nested
lambdas and partial application falls out of the lowering.

The path is: source, lexer, precedence-climbing parser, Core IR written straight
into a lent wire buffer in the layout the runtime already decodes, then
evaluation. No intermediate AST, so there is no second tree to keep in step with
the IR. Every buffer is caller-lent and nothing allocates.

Arithmetic is a family operation, since the Core has no `+`, and its meaning
arrives as a program over the closed primitive vocabulary proved admissible in
`mock/research/sketches/202607260800_four-way-projection/`. `applyOp` dispatches
`inline for` over the operation table, so each arm is specialised at comptime and
the operation's walk disappears. `1 + 2 * 3` is computed by the runtime, not
handed to anyone.

## What this establishes, stated narrowly

The locus is demonstrated, not argued: a language front end runs where the canon
puts it, and the arithmetic that was previously a host callback in a test file is
now computed inside the runtime by data the language definition supplies.

## Data: strings, records, sequences

```
let r = { a: 7, b: 9 }; r.a + r.b                ==> 16
let r = { inner: { deep: 3 } }; r.inner.deep     ==> 3
at([{ v: 9 }, { v: 8 }], 0).v                    ==> 9

let r = { a: 1 }; r.b                            refused, NoSuchField
let n = 1; n.a                                   refused, Mismatch
if 1 < 2 { { a: 1 } } else { { a: "x" } }        refused, Mismatch
len([1, "two"])                                  refused, Mismatch
if { a: 1 } { 1 } else { 2 }                     refused, UnexpectedToken
at([1, 2], 5)                                    refused at run time, OutOfRange
```

Construction is a family operation and projection is a Core form, which is the
split the census fixes: content-as-values makes construction the signature's
introduction projection, and the Core is the eliminator algebra.

This is where the round's coverage question got answered by building rather than
arguing. The scalar vocabulary that carries arithmetic cannot express a
constructor, because a constructor takes a variable number of operands and yields
a compound. The `TAG_RAW` arm branches on exactly that, and the branch is the
finding made visible. It is one arm of the eventual bench, not a settled choice.

The last refusal is the struct-literal ambiguity, resolved as Rust resolves it:
`{` opens a block after an `if` condition and a record literal elsewhere.

`len` and `at` are recognised by the parser as prelude names in call position.
That is a sketch shortcut: in the finished shape the language definition supplies
them as ordinary operations, and the parser recognises nothing.

## Real Clause code

The first program here that reads as a standard-library function rather than a
demonstration:

```
fn sum_from(s, i) { if i < len(s) { at(s, i) + sum_from(s, i + 1) } else { 0 } }
fn sum(s) { sum_from(s, 0) }
sum([1, 2, 3])                                   ==> 6
```

## Traits, with bounds and coherence

```
trait Doubler { fn dbl(Self) -> Self }
impl Doubler for Int { fn dbl(x) { x * 2 } }
dbl(5)                                           ==> 10

trait Sizer { fn size(Self) -> Int }
impl Sizer for Int { fn size(x) { x + 100 } }
impl Sizer for Str { fn size(s) { 7 } }
size(5)                                          ==> 105
size("anything")                                 ==> 7

fn twice(v) { dbl(dbl(v)) }
twice(5)                                         ==> 20

fn use_it(v) { dbl(v) }  use_it("a string")      refused, NoImpl
impl Doubler for Int { fn dbl(x) { 0 < x } }     refused, Mismatch
impl Doubler for Int { ... } twice               refused, DuplicateImpl
impl Nope for Int { ... }                        refused, UnknownTrait
```

**Dispatch keeps prove-then-erase.** A trait method reference raises an
obligation about the type it was used at, recorded against its node. After
inference, each obligation resolves to the one impl for that trait and type, and
the choice is written into a per-node table. The evaluator reads that table and
never inspects a value's shape, so types still erase and the runtime stays a dumb
evaluator of proven-safe programs.

**An impl must have the type its trait declared**, with `Self` replaced by the
implementing type. Without that check an impl body could be anything and the call
site would still type-check against the signature, which is a hole rather than a
leniency.

**Impl binders are minted from a numeric range disjoint from source names**
rather than by interning a mangled string. A mangled name would have to live
somewhere, and the obvious somewhere is a stack buffer the interner outlives.
Hygiene here is structural rather than a naming convention.

### Several methods per trait

```
trait Num { fn dbl(Self) -> Self  fn trip(Self) -> Self }
impl Num for Int { fn dbl(x) { x * 2 } fn trip(x) { x * 3 } }
dbl(6) + trip(5)                                 ==> 27

impl Num for Int { fn dbl(x) { x * 2 } }         refused, MissingMethod
impl Num for Int { fn nope(x) { x } }            refused, UnknownTrait
trait Num { ... fn name(Self) -> Str }
impl Num for Int { ... fn name(x) { x } }        refused, Mismatch
```

Methods are matched to the trait by name rather than by position, so an impl may
write them in any order, a missing one is caught, and a method the trait never
declared is caught. Each method's signature is checked separately against its own
declaration, and each dispatches independently: two methods of one trait resolve
through the same impl but are separate binders.

### Supertraits

```
trait Base { fn base(Self) -> Int }
trait Derived: Base { fn derived(Self) -> Int }
impl Derived for Int { fn derived(x) { x + 2 } }
impl Base for Int { fn base(x) { x + 1 } }
base(4) + derived(5)                             ==> 12

impl Derived for Int { ... }  (with no Base impl) refused, MissingSuperImpl
trait Derived: Nope { ... }                       refused, UnknownTrait
```

An impl of a trait with a supertrait requires an impl of that supertrait for the
same type, and the whole chain is walked so a grandparent is required too. The
requirement is checked over the whole impl table rather than in declaration
order, so the two impls may be written either way round. Doing it in order would
have been easier and would have made a correct program's acceptance depend on
how it was laid out.

### Associated types

```
trait Conv { type Out; fn conv(Self) -> Out }
impl Conv for Int { type Out = Str; fn conv(x) { "from int" } }
impl Conv for Str { type Out = Int; fn conv(s) { 42 } }

conv("a string")                                 ==> 42
conv(7)                                          ==> "from int"
fn bump(v) { conv(v) + 1 }  bump("x")            ==> 43

conv(7) + 1                                      refused, Mismatch
impl Conv for Int { type Out = Str; fn conv(x) { 99 } }
                                                 refused, Mismatch
```

The method's result type depends on which impl applies, which is what makes it
an associated type rather than another parameter. Mechanically it is a second
quantified variable alongside `Self`, left unbound during inference and unified
at discharge with whatever the chosen impl declared. Deferring it is what lets
the result type follow the impl: during inference nothing yet knows which impl
that is.

The two refusals are the ones only an associated type can catch. `conv(7)` is a
`Str` by `Int`'s impl, so adding one to it is a type error rather than a
plausible expression. And an impl whose body disagrees with its own declared
`Out` is refused, so the declaration is a commitment rather than an annotation.

### Specialisation, which is what lifts the bound

```
trait Sized { fn size(Self) -> Int }
impl Sized for Int { fn size(x) { x } }
impl Sized for Str { fn size(s) { 5 } }

fn twice_size(v) { size(v) + size(v) }
twice_size(10) + twice_size("hello")             ==> 30

fn sum_sizes(v, n) { if n < 1 { 0 } else { size(v) + sum_sizes(v, n - 1) } }
sum_sizes(2, 3) + sum_sizes("abc", 3)            ==> 21

dbl_n("no impl for me", 2)                       refused, NoImpl
```

Dispatch is a per-node table, which is what keeps types erased, but one body
means one slot however many times it is used. Two uses at two types therefore
need two bodies, and producing them is monomorphisation.

**Non-recursive bounded functions are inlined.** Each reference becomes a copy of
the value, so the copy's nodes carry their own dispatch slots and its obligation
resolves at that use's type. No binder is introduced, so no scope question
arises.

**Recursive ones get one binding per use site**, because a copy that calls itself
needs a name to call. Each copy is bound under its own specialisation binder with
its self-references rewritten to it, so recursion stays inside the copy rather
than escaping back to the original.

Both leave the original binding dead, and dead is not good enough: its body still
raises an obligation, and with no use left to fix the type that obligation is
unresolvable. The original has to be removed, not merely bypassed. That was the
one thing the pass forced that the design memo had not anticipated.

Types still erase throughout. Nothing type-shaped exists at run time, which is
why this is monomorphisation rather than dictionary passing; the design memo at
`mock/research/202607261000_monomorphisation-shapes.md` prices that fork.

## Patterns and match

```
match 5 { 0 => 1, 5 => 42, _ => 0 }                      ==> 42
match 4 { 0 => 0, n => n + 5 }                           ==> 9
match "a" { "a" => "yes", _ => "no" }                    ==> "yes"
let r = { a: 3, b: 5 }; match r { { a: x, b: y } => x + y }   ==> 8
let r = { a: 0, b: 5 }; match r { { a: 0, b: _ } => 1, _ => 2 } ==> 1

match 1 { 0 => 1, _ => "other" }                  refused, Mismatch
match 1 { "a" => 1, _ => 2 }                      refused, Mismatch
match 1 { { a: x } => x, _ => 2 }                 refused, Mismatch
match 1 { 0 => 1, 2 => 3 }                        refused, NonExhaustive
```

```
match 2 { 1 | 2 | 3 => 1, _ => 0 }                       ==> 1
match "e" { "a" | "e" | "i" => "vowel", _ => "other" }   ==> "vowel"
match 3 { 0..5 => 1, _ => 0 }                            ==> 1
match 5 { 0..5 => 1, _ => 0 }                            ==> 0
match 5 { 0..=5 => 1, _ => 0 }                           ==> 1
match 8 { n if 5 < n => 1, _ => 0 }                      ==> 1
let r = { a: 3, b: 5 };
match r { { a: x, b: y } if x < y => x * y + 5, _ => 0 } ==> 20

match 1 { n if 0 < n => 1 }              refused, NonExhaustive
match 1 { n if n => 1, _ => 0 }          refused, Mismatch
match 1 { 1 | n => n, _ => 0 }           refused, BindingInAlternative
match "s" { 0..5 => 1, _ => 0 }          refused, Mismatch
```

Patterns are literals, bindings, `_`, records of sub-patterns, alternatives,
and integer ranges, with optional guards.

**Alternatives may not bind.** Both sides then agree on the empty set of
bindings by construction rather than by a check that their binding sets match,
which is the cheaper way to be sound about it.

**A guard makes an arm refutable however irrefutable its pattern is**, since the
condition may be false, so a guarded arm cannot be the last one alone. The guard
types under the arm's bindings, so it may test what the pattern just bound, and
it must be a condition rather than merely a value. Every pattern
must type against the scrutinee, so a match over the wrong shape is a static
error rather than an arm that silently never fires, and all arms must agree on a
result type.

**Exhaustiveness is approximated by requiring an irrefutable last arm.** That is
sound and checkable without a usefulness algorithm, and it refuses some programs
a real exhaustiveness check would accept. A record pattern counts as irrefutable
when all its sub-patterns are, because a record has exactly the fields it has and
there is no other shape for the match to fall through to.

A bare record literal cannot be a scrutinee, since `match r {` cannot tell the
record from the arm block. Rust has the same restriction and the same two ways
out: bind it first, or parenthesise it.

**This is input to the Core round, not a decision it can skip.** The framework's
pattern representation is still undecided (task #43); patterns here live in the
same node arena as expressions with their own tags, which is a sketch choice that
works and is exactly the kind of thing that round should weigh rather than
inherit.

## Loops, without mutation

```
let mut acc = 0;
let mut i = 0;
while i < 5 { acc += i; i += 1; }
acc                                              ==> 10

let x = 1; ... x = 5;                            refused, NotMutable
let mut acc = 0; ... acc = "text";               refused, Mismatch
```

`while` needs no new Core form. It becomes a recursive function of exactly the
mutable locals in scope:

```
while c { body } rest
  =>  let rec L = \x1..\xn. if c { body; L(x1..xn) } else { rest } in L(x1..xn)
```

Assignments inside the body are `let` rebindings that shadow the parameters, so
the tail call reads the updated values with no renaming at all: **shadowing is
the state update.** That is the census's resolution of `mut` made operational.
A local reassignment is rebinding; only a genuine place, a field someone else can
observe, would need an effect, and this subset has none.

Two consequences worth naming. The loop's continuation lives in the `else`
branch, so the whole thing is one tail-recursive function rather than a loop plus
a follow-on. And the state keeps its type across iterations, so reassigning a
counter to a string is refused rather than quietly widening.

`for` is the same desugaring with an index the parser supplies:

```
let mut total = 0;
for x in [1, 2, 3, 4] { total += x; }
total                                            ==> 10

for x in map(add2(10), range(3)) { total += x; } ==> 33
for x in ["a", "b"] { total += x; }              refused, Mismatch
for x in 5 { total += x; }                       refused, Mismatch
```

The sequence is bound once so it is not re-evaluated per turn, and the index
joins the mutable locals so the loop function carries it alongside whatever the
enclosing scope was already threading. The increment is emitted rather than
parsed, so a body that never mentions the index still advances.

**The index and the sequence come from a synthetic binder range** that no source
name can reach, so a program using the names `i` or `s` neither captures them nor
is captured by them. Hygiene here is structural rather than a naming convention,
the same as impl-method binders.

**Loops are bounded by the evaluator's stack.** Each iteration is a call, and the
evaluator does not yet trampoline (task #49), so a long loop overflows where a
real one would not. The desugaring is right; the evaluator has not caught up.

## Modules

```
mod M { fn double(n) { n * 2 } fn bump(n) { n + 1 } }
M::bump(M::double(5))                            ==> 11

mod M { fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } } }
M::fact(5)                                       ==> 120

use M::double;  double(5)                        ==> 10
let d = M.double;  d(4)                          ==> 8

M::missing(5)                                    refused, NoSuchField
M::double("not a number")                        refused, Mismatch
```

```
mod M { fn double(n) { n * 2 } pub fn quad(n) { double(double(n)) } }
M::quad(5)                                       ==> 20
M::double(5)                                     refused, NoSuchField

mod Outer { pub mod Inner { pub fn six() { 6 } } }
Outer::Inner::six()                              ==> 6

#[inline] fn double(n) { n * 2 }  double(5)      ==> 10
```

**Items are private by default**, as in Rust. A private item is still bound
inside the module so its siblings can call it; it simply does not become a field
of the record. Reaching it from outside then fails as a missing field, which is
the right answer arrived at without a second mechanism. Modules nest, and a
private inner module is private the same way.

Attributes parse and erase. They are metadata for a later stage and nothing
downstream has asked for them yet, so dropping them is the honest treatment
rather than a shortcut.

**A module is a record of its items.** `mod M { ... }` binds `M` to a record
whose fields are the items; `M::f` is a projection; `use M::f;` is an ordinary
binding of `f` to that projection. So modules are first-class values that can be
bound and passed, and none of it is a second namespace mechanism bolted beside
the first. Items inside a module see each other, and may recurse, because they
are ordinary recursive bindings that the record closes over.

The limit this exposes is real and recorded: **projecting off a function
parameter is refused**, because the parameter's type would have to mean "some
record with a `double` field", which is a row type. Passing a module into a
function and reaching into it there needs row polymorphism, which this does not
have. Inferring a concrete record instead would be wrong, so it refuses.

## Macros, discharged at the compile stage

```
macro double(n) -> Int { n * 2 }
double!(3) + double!(10)                         ==> 26

macro pick(a, b) -> Int { if a < b { b } else { a } }
pick!(7, 3)                                      ==> 7

macro ten() -> Int { 10 }
fn triple(n) { n * 3 }  triple(ten!())           ==> 30

fn f(x) { double!(x) }                  refused, NotConstant
nope!(1)                                refused, UnknownMacro
double!(1, 2)                           refused, WrongMacroArity
macro bad(n) -> Int { unknown_name + n } bad!(1) refused, NotConstant
```

The census's reading made operational: **a macro is a function evaluated at a
compile stage**, and which stage services an operation is a binding-time
coordinate rather than a separate mechanism. Here that stage is the parser, so a
macro's body is evaluated there and the call site keeps only the constant it
produced. Nothing of the macro survives into the residual, which is the point of
discharging it at that stage.

A macro declares its result type, which is what makes it a typed function at a
stage rather than a token rewriter.

Division is the one scalar primitive with a runtime refusal, since a zero divisor
is not a type error and the checker cannot prove it absent.

**A macro given a runtime value is refused rather than deferred**, because there
is no later stage for it to fall back to. That refusal is the binding-time
discipline showing: the argument's value is not known when the handler runs, so
the operation cannot be discharged at all.

Arguments are evaluated at the same stage, so a macro may take another macro's
expansion but not a function parameter.

**This is the constant-evaluation half of macros, not the syntactic half.** The
normative grammar's macros take a token stream with `$` interpolation, and
nothing here does that: there is no token-stream representation, no splicing of
syntax, and no hygiene question yet because a constant has no names in it. The
binding-time axis is what this demonstrates; the token-stream story is separate
and unbuilt.

## What it does not establish, stated plainly

**This is a slice of the grammar, and the bar is the whole grammar.** It has
integers, strings, names, `let`, `fn` with recursion and closures and currying,
`if`/`else`, four operators, records with field access, sequences with a
three-operation prelude, single-method traits with impls, coherence, inferred
bounds, associated types, and pattern matching. It does not have multi-method
real exhaustiveness checking,
syntactic macros over token streams, `loop`, row polymorphism, re-exported modules,
a separate resolve pass, or monomorphisation, nor the
rest of the surface the
normative grammar (`mock/research/original-docs/CLAUSE_EBNF.md`) requires.
Nothing is done until the full intended language is expressible.

## Inference, in the runtime, before evaluation

`check.zig` is Hindley-Milner over the same image, with generalisation at binding
sites. It refuses what the untyped evaluator accepted:

```
if 1 { 1 } else { 2 }                      refused, Mismatch
if 1 < 2 { 1 } else { 2 < 3 }              refused, Mismatch (branches disagree)
1 + (2 < 3)                                refused, Mismatch
fn double(n) { n * 2 } double(1 < 2)       refused, Mismatch
fn f(x) { x(x) } 0                         refused, Occurs
x + 1                                      refused, Unbound

fn id(x) { x }
if id(1 < 2) { id(1) } else { 0 }          ==> int
```

The last one is the load-bearing test. `id` is generalised, so its two uses do
not constrain each other and one definition serves both `Bool` and `Int`. That is
what a generic *is* in inference terms, and it is the seed the bounds and
associated types grow from: a bound is a constraint carried alongside the
quantifier, not a different mechanism.

Two smaller points worth keeping. A recursive binding stays monomorphic inside
its own body, because generalising before the value is inferred would let a
recursive call take a type the definition has not earned. And the arithmetic
family's operation *types* come from the same signature data that carries the
operation bodies, so the checker reads what the operations mean rather than
knowing what arithmetic is.

The first refusal above is the one that matters most: booleans were `0` and `1`
in an `i64` because nothing demanded better, and now something does.

The evaluator here is a subset evaluator in the sketch, not a change to
`mock/runtime-zig/src/runtime.zig`, which still routes every family operation to
a host. That change is the source changelist of round
`202607260500_topic.the-languageauthor-specialisation-stage.md`, and this sketch
exists to prove the shape before that changelist implements it.

At runtime a boolean is still `0` or `1` in an `i64`, because the comparison
operation returns what its program computes. The difference the checker makes is
that this is no longer observable: a program that treats one as the other is
refused before it runs. The representation remains a subset artifact and is not
the intended one.

## Reproducing

```
zig test eval.zig     # 40 tests, the whole pipeline plus the checker's
zig test check.zig    # the inference tests alone
```

## Still owed on the checker itself

Inference runs, but nothing yet forces it to: `eval.zig`'s `run` evaluates
without calling `check`, which is the same prove-then-erase gap the shipped
pipeline has (`serialize` documents itself as taking a checked program while its
signature takes a bare arena). Wiring the gate so an unchecked program cannot be
evaluated is owed, and is the smaller half of the junction question.

There is also no separate resolve pass. Names are looked up during inference and
again during evaluation, by the same backward scan over a linked scope. That
works for a subset with no modules, paths, or imports, and stops working the
moment `use` and multi-segment paths arrive.

Bounds, associated types, and coherence are not started. The generalisation
machinery above is their foundation, not a partial version of them.
