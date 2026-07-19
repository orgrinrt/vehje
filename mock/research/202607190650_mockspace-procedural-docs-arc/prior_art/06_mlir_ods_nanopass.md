## What each is, in three sentences

**MLIR ODS** is a TableGen-based DSL: an operation is written as a `.td` record specifying its arguments, results, traits, and (optionally) an assembly format and verifier; `mlir-tblgen` expands that record at build time into a C++ `mlir::Op` template specialization with accessors, builders, and verification hooks. Dialects, types, attributes, and interfaces use the same record-and-generate mechanism. Generic compiler passes then operate over any op by querying traits/interfaces rather than switching on concrete op identity.

**nanopass** is an embedded Scheme DSL with two forms, `define-language` and `define-pass`. `define-language` declares a context-free grammar for an intermediate language (terminals + nonterminals + productions, optionally as a diff against a prior language via `extends`); `define-pass` writes a transformation over that grammar where the compiler writer supplies only the clauses that change, and the framework generates record types, predicates, and pass-through clauses for everything else.

## Defining an MLIR op in ODS

Complete, real `Toy_Op` example (`mlir/examples/toy/Ch2/include/toy/Ops.td`, LLVM tree):

```tablegen
def AddOp : Toy_Op<"add"> {
  let summary = "element-wise addition operation";
  let description = [{
    The "add" operation performs element-wise addition between two tensors.
    The shapes of the tensor operands are expected to match.
  }];

  let arguments = (ins F64Tensor:$lhs, F64Tensor:$rhs);
  let results = (outs F64Tensor);
  let hasCustomAssemblyFormat = 1;

  let builders = [
    OpBuilder<(ins "Value":$lhs, "Value":$rhs)>
  ];
}
```

Ten non-comment lines. Compare ODS's own hand-written C++ equivalent for `ConstantOp` (`docs/Tutorials/Toy/Ch-2.md`): a class with zero fields that still spells out the CRTP base, `getOperationName`, an accessor, `verifyInvariants`, and three `build` overloads, roughly 35 lines of declarations alone, none of the `.cpp` bodies. From the *empty* ODS form `def ConstantOp : Toy_Op<"constant"> {}`: "Missing here from our C++ definition are the `ZeroOperands` and `OneResult` traits; these will be automatically inferred based upon the `arguments` and `results` fields." `OpDefinitionsGen` (`-gen-op-decls`/`-gen-op-defs`) generates per op: named accessors per operand/attribute/result (`getLhs()`, replacing `getOperand(3)`), an **operand adaptor** class for name-based access from a bare `Value` array in generic templates, multiple builder overloads, `getOperationName()`, and, when requested, `verify()`/`verifyRegions()`/`fold()`/`getCanonicalizationPatterns()`. The tree does not commit generated `.inc` output, so no exact line-count exists from a shipped artifact; the motivation is killing "repetitive string comparisons" and "generic/error prone `getOperand(3)` vs self-documenting `getStride()`" accessors.

## Declaring a dialect: every file required

Minimal in-tree example, the Toy Ch2 tutorial dialect (four files):

- `include/toy/Ops.td` — one file holds both the dialect record and every op: `def Toy_Dialect : Dialect { let name = "toy"; let cppNamespace = "::mlir::toy"; }` plus `class Toy_Op<string mnemonic, list<Trait> traits = []> : Op<Toy_Dialect, mnemonic, traits>;`.
- `include/toy/Dialect.h` — includes the two generated headers: `#include "toy/Dialect.h.inc"` then `#define GET_OP_CLASSES` / `#include "toy/Ops.h.inc"`.
- `mlir/Dialect.cpp` — `#include "toy/Dialect.cpp.inc"` plus any hand-written verifier bodies (`ConstantOp::verify()`, etc.) the `.td` declared via `hasVerifier = 1`.
- `include/toy/CMakeLists.txt` — the tablegen invocation, verbatim:
  ```cmake
  set(LLVM_TARGET_DEFINITIONS Ops.td)
  mlir_tablegen(Ops.h.inc -gen-op-decls)
  mlir_tablegen(Ops.cpp.inc -gen-op-defs)
  mlir_tablegen(Dialect.h.inc -gen-dialect-decls)
  mlir_tablegen(Dialect.cpp.inc -gen-dialect-defs)
  add_public_tablegen_target(ToyCh2OpsIncGen)
  ```

Larger dialects add a separate `Dialect.td`, `Types.td`, `Attrs.td`, and shard op defs (`-op-shard-count`); the floor is one `.td`, one `.h`, one `.cpp`, one `CMakeLists.txt`.

## Traits and interfaces: the generic-pass mechanism

Traits are the op's third template argument (`Op<Toy_Dialect, "add", [Pure, Commutative]>`), backing a `mlir::OpTrait::TraitBase<ConcreteType, MyTrait>` C++ class. A generic pass queries one without knowing the concrete op: `if (op->hasTrait<MyTrait>() || op->hasTrait<MyParametricTrait<10>::Impl>()) ...`. `Commutative`: "this trait adds the property that the operation is commutative, i.e. X op Y == Y op X." Interfaces go further: `def ExampleOpInterface : OpInterface<"ExampleOpInterface"> { let methods = [InterfaceMethod<[{...}], "void", "nonStaticMethod">]; }` declares hook methods; an op opts in via `[DeclareOpInterfaceMethods<ExampleOpInterface>]` and supplies bodies. A generic pass calls the interface, not the op: `if (ExampleOpInterface example = dyn_cast<ExampleOpInterface>(op)) example.nonStaticMethod();` or `op->getInterface<ExampleOpInterface>()`. This is the mechanism by which canonicalization, CSE, DCE, or bufferization operate over an open, ever-growing set of op kinds: dispatch on trait/interface, never on a closed enum.

## Declarative rewrite rules, and where they run out

A complete DRR pattern (`docs/DeclarativeRewrites.md`):

```tablegen
def createArrayAttr : NativeCodeCall<"createArrayAttr($_builder, $0, $1)">;

def : Pat<(TwoAttrOp $attr1, $attr2),
          (OneAttrOp (createArrayAttr $attr1, $attr2))>;
```

`NativeCodeCall` is DRR's own escape hatch: used whenever "the captured arguments are not exactly what we want so they cannot be directly fed in as arguments to build the new op." Full drop to hand-written C++ `RewritePattern` subclasses is required for what DRR cannot express: "matching and generating ops with regions," "matching and generating ops with block arguments," "matching multi-result ops in nested patterns," "matching and generating variadic operand/result ops in nested patterns." **PDL/PDLL** is the newer declarative successor (not TableGen-based), matching IR structure directly: `Pattern { let root = op<toy.reshape>(op<toy.reshape>(arg: Value)); replace root with op<toy.reshape>(arg); }`. MLIR frames the DRR-to-PDLL move as forced by that same awkwardness: multi-result handling and constraint placement that "obscure readability" under TableGen.

## `unrealized_conversion_cast`, exact role

Inserted by the dialect conversion driver when a rewrite changes an operand's type but the driver cannot yet safely rewrite every user of that value: "Simply swapping out the operand of `"test.bar"` during the `replaceOp` call would be unsafe, because that would change the type of operand and, therefore, potentially the semantics of the operation. Instead, the dialect conversion driver (conceptually) inserts a `builtin.unrealized_conversion_cast` op that connects the newly created `"test.qux"` op with the `"test.bar"` op, without changing the types of the latter one." It is a type-safe bridge between already-converted and not-yet-converted IR mid-pass; the driver's API does not even guarantee a caller sees the "real" value, since "operands may contain results of transitory `builtin.unrealized_conversion_cast` ops that were inserted by the conversion driver but typically fold away again throughout the conversion process." The dedicated `--reconcile-unrealized-casts` pass walks chains of these casts post-conversion, folding a matched pair away when chain endpoints agree in type; anything unreconciled surfaces via a `remainingCastOps` list, so survival past that pass is diagnosable, not silently tolerated.

## nanopass: `define-language` and `define-pass`, complete examples

From Keep & Dybvig, ICFP 2013 (verbatim, Figure 1):

```scheme
(define-language Lsrc
  (terminals
    (uvar (x))
    (primitive (pr))
    (datum (d)))
  (Expr (e body)
    x
    (quote d)
    (if e0 e1 e2)
    (begin e* ... e)
    (lambda (x* ...) body)
    (let ([x* e*] ...) body)
    (letrec ([x* e*] ...) body)
    (set! x e)
    (pr e* ...)
    (call e e* ...) => (e e* ...)))
```

Extension, diff notation, `-`/`+` clauses (Figure 2):

```scheme
(define-language L1
  (extends Lsrc)
  (terminals
    (- (datum (d)))
    (+ (constant (c))))
  (Expr (e body)
    (- (quote d))
    (+ (quote c))))
```

`define-language` generates a record type per production, a corresponding predicate, constructors, and an unparser back to S-expressions; each terminal clause requires the writer to supply a `<name>?` predicate (`uvar?`, `primitive?`, `datum?`) lexically visible where the language is defined. `extends` states only the delta: `-` removes a terminal or production, `+` adds one; unlisted forms carry over unchanged. A `define-pass` (Figure 3, `convert-complex-datum : Lsrc (x) -> L1 ()`) supplies one transformer clause for the one form it changes, `(quote ,d)` under a `guard`; the framework autogenerates pass-through clauses for every other `Lsrc` production, rewriting each into its `L1` counterpart unchanged: "The `define-pass` macro autogenerates clauses matching the other input-language forms and producing equivalent output-language forms."

## The automatic unchanged-form mechanism

This is the load-bearing ergonomics claim. The user guide states the problem directly: "Passes often contain boilerplate code to recur through otherwise unchanging language forms. For instance, in a pass to remove one-armed `if` expressions, where only the `if` form changes, other forms in the language must be handled explicitly to locate embedded `if` expressions." The mechanism removing it: `define-pass` parses the input language's grammar (already known from `define-language`) and, for every nonterminal/production the pass's clauses do not mention, emits a default clause reconstructing the equivalent output form and recursing into its subterms (the "catamorphism" support the guide credits to Erik Hilsdale). The pass author's source shrinks to exactly the clauses that do something.

## Measured costs, both systems

**nanopass**: the two macros implementing `define-language`/`define-pass` are, in the authors' words, "complex, with approximately 4600 lines of code" for the whole framework. The commercial-compiler validation: replacing 5 multipurpose back-end passes (of the original Chez Scheme compiler's 10) with "over 50 nanopasses" across roughly 35 intermediate languages produced code 15-27% faster at compile times "well within a factor of two" of the original (measured 1.64-1.75x, Table 3), despite the new compiler also adding a slower graph-coloring register allocator. Figure 4 reports no direct correlation found between expanded-source line count and normalized compile time. **MLIR ODS**: no shipped generated-`.inc` artifact or line-count table exists in the primary sources found; the docs' claim is boilerplate elimination (named accessors, generated builders, verification stubs) rather than a measured multiplier. This is an asymmetry in what the primary sources report, not an omission in this research.

## Mechanisms worth naming, each with its tradeoff

- **Trait/interface dispatch (MLIR)**: any pass acts on any op via a marker or vtable-shaped interface; cost is every generic pass needing the op author to opt in (an op forgetting `Pure` is invisible to DCE).
- **NativeCodeCall escape hatch (DRR)**: keeps most of a rewrite declarative, dropping to C++ only for the value transform; cost is a de-facto two-language pattern PDLL was built to replace.
- **Unrealized-cast materialization**: lets a conversion pass rewrite operations one at a time instead of atomically; cost is an extra interim IR-legality state plus a dedicated cleanup pass.
- **Diff-based language extension (`extends`)**: keeps N similar languages from requiring N full grammars; cost is exact-name matching being "too strict" and a renamed field silently breaking downstream clauses until compile time.
- **Grammar-driven default-clause generation (`define-pass`)**: the single mechanism making one-form-change passes cheap; cost is trusting the framework's notion of "unchanged," documented as sometimes wrong below.

## Known limitations, in the authors' own words

nanopass's own TODO file ("Nanopass Annoyances") states two structural gaps directly: (1) "Removal of patterns is too strict matching EXACTLY the variable names... without the error is a very rough edge," and (2) output forms constructed from a language "need to match original language forms very closely," giving a concrete failure: a `(begin e* ... e)` production cannot be built directly from a list via `` `(begin (set! ,x0 (var ,tmp*)) ...) `` because the framework "sees this as a single form instead of a list," forcing a hand-written `reverse`/`car`/`cdr` workaround. The paper separately notes the new compiler does not implement block allocation of closures (an optimization the original had): restructuring into nanopasses does not guarantee feature parity for free. MLIR's docs record no analogous unsound corner; the closest is DRR's explicit unsupported-construct list above, framed as "use PDLL or C++" rather than a defect.

## What would not transfer to no_std, no-alloc Rust

- **MLIR/TableGen**: ODS itself is a build-time generator, independent of GC/RTTI/alloc at codegen time. But the *generated C++* leans on `llvm::dyn_cast` (hand-rolled RTTI), `SmallVector`/`ArrayRef` (heap-backed growable storage), and MLIR's `Operation*`/`Value` object model, reference-counted/arena-allocated under an `MLIRContext` owning uniqued storage. None of that is `no_std`-compatible as-is; a Rust port needs its own arena/interning story, and `dyn_cast`-style dispatch maps to Rust trait objects or an enum-of-witnesses, not a direct translation. Rust macros (declarative or proc-macro) can express an ODS-equivalent generator, so *generation* transfers; the context-owned, GC-adjacent *runtime object model* does not without a redesign.
- **nanopass**: depends on Scheme's dynamic, homoiconic S-expression representation and hygienic macros operating over it, a uniform pair/vector/symbol data model absent in Rust. `define-language` performs checks and codegen as one macro-expansion-time computation building predicate closures referenced by lexical visibility, more dynamic than Rust proc-macro sandboxing (token streams only, no runtime values) permits directly. Rust expresses the *record-plus-predicate-plus-constructor* generation (what `enum`-plus-derive already does) and the *default-clause-for-unmatched-productions* mechanism (a derive emitting a default per-variant method unless overridden), but `extends` diff-based grammars and the S-expression pattern/template DSL (nested `...`-quantified matching against records) have no Rust host primitive to lean on; building them is a bigger lift than TableGen's record translation.

## Sources

- MLIR ODS reference: https://mlir.llvm.org/docs/DefiningDialects/Operations/
- Toy tutorial Ch.2 (hand-written vs. ODS `ConstantOp`, dialect-loading walkthrough): https://mlir.llvm.org/docs/Tutorials/Toy/Ch-2/
- Toy Ch2 source tree (`Ops.td`, `Dialect.h`, `Dialect.cpp`, `CMakeLists.txt`): https://github.com/llvm/llvm-project/tree/main/mlir/examples/toy/Ch2
- MLIR Traits: https://mlir.llvm.org/docs/Traits/
- MLIR Interfaces: https://mlir.llvm.org/docs/Interfaces/
- MLIR Declarative Rewrite Rules (DRR): https://mlir.llvm.org/docs/DeclarativeRewrites/
- MLIR PDLL: https://mlir.llvm.org/docs/PDLL/
- MLIR Dialect Conversion (`unrealized_conversion_cast`): https://mlir.llvm.org/docs/DialectConversion/
- `ReconcileUnrealizedCasts` pass source: https://mlir.llvm.org/doxygen/ReconcileUnrealizedCasts_8cpp_source.html
- Keep & Dybvig, "A Nanopass Framework for Commercial Compiler Development", ICFP 2013 (preprint PDF): https://andykeep.com/pubs/np-preprint.pdf
- Keep, "Nanopass Framework Users Guide" (extracted from dissertation ch.2): https://guenchi.github.io/Scheme/doc/Nanopass%20Framework%20Users%20Guide.pdf
- Keep, dissertation, Indiana University 2013: https://andykeep.com/pubs/dissertation.pdf
- nanopass-framework-scheme repository (ReadMe, TODO): https://github.com/nanopass/nanopass-framework-scheme
