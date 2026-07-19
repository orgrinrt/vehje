# Lightweight Modular Staging (LMS) and Delite, in mechanical detail

Source repo examined: `TiarkRompf/virtualization-lms-core` (branch `develop`, org mirror `scala-lms`).
Papers: Rompf & Odersky, GPCE 2010 ("Lightweight Modular Staging"); Rompf et al., POPL 2013
("Optimizing Data Structures in High-Level Programs"); Sujeeth et al., ECOOP/TECS ("Delite: A
Compiler Architecture for Performance-Oriented Embedded DSLs").

## What it is, in three sentences

LMS is a Scala library (not a language extension, beyond one compiler-plugin caveat below) that
turns ordinary-looking Scala code into a builder of an intermediate-representation graph, by giving
every staged value the type `Rep[T]` instead of `T` and overloading its operations so each call
constructs an IR node rather than computing a result. Common subexpression elimination falls out for
free because IR nodes (`Def`) are Scala case classes and node construction is memoized by structural
equality; effect ordering (mutation, I/O) is tracked as an explicit dependency list on each effectful
node so that a backend which only prints nodes in schedule order reproduces the ordering an
interpreter would get from the host language's own evaluation order. Delite is a second layer built
on top of the same `Sym`/`Def`/`Block` IR: it adds a small family of parallel-loop `Def` nodes
(map/zip/reduce/filter) and a runtime that schedules the same per-node code-generation mechanism
across Scala, C++, CUDA, and OpenCL backends.

## The core representation

`src/internal/Expressions.scala` (LMS core):

```scala
abstract class Exp[+T:Typ] {
  def tp: Typ[T @uncheckedVariance] = implicitly[Typ[T]]
  def pos: List[SourceContext] = Nil
}
case class Sym[+T:Typ](val id: Int) extends Exp[T] {
  var sourceContexts: List[SourceContext] = Nil
}
case class Const[+T:Typ](x: T) extends Exp[T]
abstract class Def[+T] {
  override final lazy val hashCode = scala.runtime.ScalaRunTime._hashCode(this.asInstanceOf[Product])
}
case class TP[+T](sym: Sym[T], rhs: Def[T]) extends Stm   // links a symbol to its definition
var globalDefs: List[Stm] = Nil
var globalDefsCache: Map[Sym[Any],Stm] = Map.empty
```

`Exp[T]` is either a bound variable (`Sym`, an integer id) or a compile-time-known literal
(`Const`). `Def[T]` is the operation-node supertype; every IR node (`NumericPlus`, `IfThenElse`,
`ArrayApply`, ...) is a case class extending `Def`. A `Stm` (`TP`) is the pair binding a fresh `Sym`
to the `Def` that computes it; the flat `globalDefs` list plus the `globalDefsCache` map (`Sym ->
Stm`) is the entire graph. There is no separate node-arena type; the graph is this list plus symbol
identity. `type Rep[+T] = Exp[T]` (declared in `BaseExp`), so `Rep` is not a wrapper struct, it is a
type alias onto the IR expression type itself.

`Block[T]` (in `Blocks.scala`, layered over `Expressions`) wraps a result `Exp[T]` plus the set of
effects reified while building it; it is what a "body" (an if-branch, a loop body, a function body)
denotes once effect-reification closes over it.

## How staging intercepts operations

Two independent mechanisms, often confused as one:

1. **Operator overloading + a "smart constructor."** `Rep[T] + Rep[T]` resolves, via an implicit
   conversion, to a method that builds a `Def` and returns it where an `Exp[T]` is expected:
   ```scala
   class NumericOpsCls[T:Numeric:Typ](lhs: Rep[T]){
     def +(rhs: Rep[T])(implicit pos: SourceContext) = numeric_plus(lhs,rhs)
   }
   case class NumericPlus[T:Numeric:Typ](lhs: Exp[T], rhs: Exp[T]) extends DefMN[T]
   def numeric_plus[T:Numeric:Typ](lhs: Exp[T], rhs: Exp[T])(implicit pos: SourceContext): Exp[T] =
     NumericPlus(lhs, rhs)
   ```
   `numeric_plus` returns a `NumericPlus` value where its signature declares `Exp[T]`. That
   coercion is an implicit, `protected implicit def toAtom[T:Typ](d: Def[T])(implicit pos:
   SourceContext): Exp[T] = findOrCreateDefinitionExp(d, List(pos))`. `toAtom` is the actual
   interception point for every operation, not the `+`.
2. **`findOrCreateDefinition` is the CSE step**, and it works because `Def` is a case class:
   ```scala
   def findOrCreateDefinition[T:Typ](d: Def[T], pos: List[SourceContext]): Stm =
     findDefinition[T](d) map { x => x.defines(d).foreach(_.withPos(pos)); x } getOrElse
       { createDefinition(fresh[T](pos), d) }
   ```
   `findDefinition(d: Def[T])` looks for a prior `Stm` whose payload structurally equals `d`
   (case-class `equals`/`hashCode`, since `Def`'s subtypes take already-bound `Exp`/`Sym` values
   as their fields, so two calls with the same already-staged operands produce `==` node values).
   If found, the existing symbol is reused; the new node is never emitted. If not,
   `createDefinition` mints a fresh `Sym` via `fresh[T]` (`nVars += 1`) and appends a `TP(sym, d)`
   to `globalDefs`/`globalDefsCache`. **CSE is a direct consequence of Def being a case class
   plus a lookup on construction, not a separate optimization pass.**
3. **Native control flow** (`if`, `while`, `var`) additionally requires `scala-virtualized`, a fork
   of the Scala compiler that rewrites `if (c) a else b` to `__ifThenElse(c, a, b)` at parse time,
   because ordinary Scala cannot overload `if`/`while`/`var` syntax. `Base extends
   EmbeddedControls`, and `trait IfThenElse extends Base { def __ifThenElse[T:Typ](cond: Rep[Boolean],
   thenp: => Rep[T], elsep: => Rep[T])(...): Rep[T] }` is what the virtualized compiler calls.
   Everything else (arithmetic, method calls, for-comprehensions over staged collections) is
   ordinary operator/implicit overloading and needs no compiler fork.

## The effect system and scheduling

`src/internal/Effects.scala`. Every side-effecting `Def` is wrapped in a `Reflect` node carrying an
explicit dependency list; every reified sub-block is a `Reify`:

```scala
case class Reflect[+A](x:Def[A], summary: Summary, deps: List[Exp[Any]]) extends Def[A]
case class Reify[A](x: Exp[A], summary: Summary, effects: List[Exp[Any]]) extends Def[A]
```

`Summary` classifies an operation along ten booleans/lists:

```scala
case class Summary(maySimple: Boolean, mstSimple: Boolean, mayGlobal: Boolean, mstGlobal: Boolean,
  resAlloc: Boolean, control: Boolean, mayRead: List[Sym[Any]], mstRead: List[Sym[Any]],
  mayWrite: List[Sym[Any]], mstWrite: List[Sym[Any]])
```

`Pure/Simple/Global/Alloc/Control` are named constructors of `Summary` (e.g. `Simple()` sets
`maySimple/mstSimple`, `Global()` sets the global-effect bits). `reflectEffect`, `reflectMutable`,
`reflectWrite(write0: Exp[Any]*)(d)` build a `Reflect` whose `deps` list is computed from the current
`context: State` (a `List[Exp[Any]]`, the ambient ordered list of effectful statements seen so far in
the current block) filtered by `mayRead`/`mayWrite` overlap with prior writers/readers. This is the
whole mechanism: **ordering is not implicit in emission order, it is an explicit edge list per
effectful node**, computed once at staging time from the read/write sets, so a scheduler that
respects those edges reproduces the same total order an eager interpreter would have produced by
running top to bottom. `reifyEffectsHere` runs a thunk with the ambient `context` reset to empty so a
branch's effects are captured locally into its own `Reify`, then restores the outer context; that is
how effects "sink into" the correct block, one `Block` per conditional/loop body.

Scheduling (`Scheduling.scala`, not fully quotable here due to fetch limits) walks backward from a
`Block`'s result symbol, pulls in each dependency (`Def`'s child `Exp`s plus, for `Reflect`, the
`deps` edges) and places each node in the innermost enclosing block where all its dependents live;
this "sinks" a pure node as deep as its uses allow while effectful nodes are additionally pinned by
their recorded ordering edges, so code motion cannot reorder observable effects even though it is
free to reorder or duplicate pure computation.

## Writing a backend

Every backend implements the interface declared in `GenericCodegen` (`~320` lines):

```scala
def emitNode(sym: Sym[Any], rhs: Def[Any]): Unit
def emitValDef(sym: Sym[Any], rhs: String): Unit
def emitBlock(y: Block[Any]): Unit
def quote(x: Exp[Any]): String
def emitSource[A:Typ](args: List[Sym[_]], body: Block[A], className: String,
  stream: PrintWriter): List[(Sym[Any], Any)]
```

A concrete op ships its own small codegen trait, mixed in per target. `ScalaGenIfThenElse` (from
`src/common/IfThenElse.scala`, whose full file including Scala/C/CUDA/OpenCL variants for this one
operator is ~520 lines):

```scala
trait ScalaGenIfThenElse extends ScalaGenEffect with BaseGenIfThenElse {
  override def emitNode(sym: Sym[Any], rhs: Def[Any]) = rhs match {
    case IfThenElse(c,a,b) =>
      stream.println("val " + quote(sym) + " = if (" + quote(c) + ") {")
      emitBlock(a); stream.println(quote(getBlockResult(a)))
      stream.println("} else {")
      emitBlock(b); stream.println(quote(getBlockResult(b)))
      stream.println("}")
    case _ => super.emitNode(sym, rhs)
  }
}
```

This is the actual shape and cost of adding a target: for every `Def` node type your DSL uses, write
one `case ... =>` arm per backend that prints (or recursively `emitBlock`s) the right text; `case _
=> super.emitNode(...)` chains to whatever the trait is mixed after, so backends compose by trait
linearization, and a brand-new backend for an existing DSL is "one codegen trait per existing IR
node type, each a few lines, plus `emitSource`'s file/function wrapper." There is no central
dispatch table; the `match` chain through `super` calls is the whole extension mechanism, which is
exactly why the IR node set and the backend set can each grow independently (the "expression
problem" solution LMS is built to demonstrate).

## Mechanisms worth naming

- **CSE-by-case-class-equality.** Zero extra bookkeeping, but every `Def` must be built from
  already-staged operands (no closures/mutable state inside), or two "same" nodes won't compare
  equal and CSE silently misses.
- **Effect summaries as read/write-set edges**, not a monolithic total order. Lets pure code move
  freely and only effectful code pay the ordering cost; the cost is a nontrivial `Summary` lattice
  every new effectful primitive must classify correctly, and getting it wrong is a silent
  correctness bug (a write miscategorized as pure can be reordered past a dependent read).
- **`compile[A,B]` invokes the real Scala compiler at runtime** (`scala.tools.nsc.Global`,
  `VirtualDirectory`, `AbstractFileClassLoader`) to turn generated source back into a callable
  `A=>B` in the same process. Convenient for benchmarking staged code against native Scala in one
  run; entirely dependent on a hosted compiler and a classloader.
- **Struct splitting / SoA** (POPL 2013 paper): a staged `struct[T](tag, elems: Seq[(String,
  Rep[Any])])` node (`SimpleStruct`) is kept symbolic through the IR; a later transformation
  rewrites an array of structs into a struct of arrays by pushing the `struct`/`field` nodes through
  array-allocation and indexing nodes. Because struct construction is itself just another `Def`
  (not a host-language object), the optimization is a graph rewrite, not a separate escape-analysis
  pass; the cost is that `field()` access on a struct that was never split still needs to fall back
  to a materialized representation.
- **Delite's DeliteOp layer** adds `Def` subclasses for map/zip/reduce/filter that carry the same
  `Reflect`-style effect summaries as ordinary nodes, so parallel loops interleave correctly with
  effects; scheduling to Scala/C++/CUDA/OpenCL is a separate runtime concern (the Delite Execution
  Graph) layered on top of, not instead of, the LMS schedule.

## What would not transfer to no_std, no-alloc Rust

- **Runtime code compilation via a hosted `scala.tools.nsc` compiler and JVM classloading**
  (`compile[A,B]`) has no analogue without a JVM; there is no equivalent "load bytecode back into
  the same process" step available to a statically-compiled, no-alloc target.
- **Implicit conversions and operator overloading as the interception mechanism.** `toAtom`, the
  `Numeric`/`Typ` implicit evidence parameters, and `NumericOpsCls` all lean on Scala's implicit
  resolution to make `Rep[T] + Rep[T]` silently build a graph node; Rust has no implicit-conversion
  analogue (`From`/`Into` are explicit-call or narrow coercions, not silent method rewriting), so
  the same surface would need explicit builder calls or macro-based operator overloading.
  Rust operator overloading (`impl Add for Rep<T>`) can replicate the surface syntax, but the
  smart-constructor body still must exist explicitly per type.
- **`scala-virtualized`'s compiler-fork rewriting of `if`/`while`/`var` syntax** has no Rust
  counterpart; Rust macros (`proc_macro`, `syn`/`quote`) could intercept syntax at the token level,
  but that is a fundamentally different (and heavier, compile-time-only) mechanism than a modified
  parser emitting method calls.
- **Case-class structural equality driving CSE** assumes cheap, GC-backed structural `equals`/
  `hashCode` over immutable trees; a no-alloc Rust IR would need an explicit interner (arena +
  hash-consing table) to get the same lookup, which is buildable but is added machinery, not free.
- **Global mutable `var globalDefs: List[Stm] = Nil` / `globalDefsCache: Map[...]`** as ambient,
  unsynchronized, heap-allocated (linked-list, immutable `Map`) global state assumes a single-
  threaded, GC'd host; a no-alloc port needs the graph storage to be an explicit, capacity-bounded
  arena passed around rather than ambient mutable statics.
- **Effect classification and scheduling run over immutable persistent lists/maps** (`List[Exp[Any]]`
  for `context`, `deps`, `effects`), reallocated on every append; this is idiomatic, cheap-enough
  Scala but is heap allocation on every single staged statement, which a no-alloc target would need
  to replace with fixed-capacity vectors or a different dependency-encoding scheme entirely.
- **Reflection-adjacent dynamic dispatch in codegen chaining** (`case _ => super.emitNode(...)`
  through Scala trait linearization) relies on the JVM's virtual dispatch and multiple-trait
  inheritance (traits stacked via `with`), which Rust does not have in the same form; the same
  open per-node extension would need an explicit trait-object vtable or a generated big match,
  either of which changes the "add a case, anywhere, and it composes" property LMS gets for free
  from trait linearization.

## Sources

- Rompf, Odersky. "Lightweight Modular Staging: A Pragmatic Approach to Runtime Code Generation and
  Compiled DSLs." GPCE 2010. https://infoscience.epfl.ch/record/150347/files/gpce63-rompf.pdf
- Rompf, Sujeeth, Amin, Brown, Jovanovic, Lee, Jonnalagedda, Odersky, Olukotun. "Optimizing Data
  Structures in High-Level Programs." POPL 2013. https://ppl.stanford.edu/papers/popl13_rompf.pdf
- Sujeeth et al. "Delite: A Compiler Architecture for Performance-Oriented Embedded Domain-Specific
  Languages." TECS. https://ppl.stanford.edu/papers/tecs14-sujeeth.pdf
- Brown, Sujeeth, Lee, et al. "A Heterogeneous Parallel Framework for Domain-Specific Languages" /
  "Implementing Domain-Specific Languages for Heterogeneous Parallel Computing."
  https://ieeexplore.ieee.org/document/6113791/
- LMS source: https://github.com/TiarkRompf/virtualization-lms-core (branch `develop`):
  `src/internal/Expressions.scala`, `src/internal/Effects.scala`, `src/internal/GenericCodegen.scala`,
  `src/internal/ScalaCodegen.scala`, `src/internal/ScalaCompile.scala`, `src/common/Base.scala`,
  `src/common/NumericOps.scala`, `src/common/IfThenElse.scala`, `src/common/Structs.scala`.
- LMS tutorials: https://scala-lms.github.io/ and https://github.com/scala-lms/tutorials
  (`03_compiler.scala`, `dslapi.scala`, `fft.scala`, `stencil.scala`, `scanner.scala`).
- Delite project: https://stanford-ppl.github.io/Delite/
