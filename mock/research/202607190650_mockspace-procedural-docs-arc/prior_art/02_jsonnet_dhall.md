Jsonnet and Dhall, mechanical detail. Primary sources only.

## What each is, in three sentences

Jsonnet is a lazy, functional configuration language whose full surface syntax desugars into a small
documented core, evaluated by a tree-walking interpreter (C++ `jsonnet`, independent Go `go-jsonnet`) that
produces a JSON value tree, serialized to JSON/YAML/other via functions written in Jsonnet itself. Dhall is
a total, non-Turing-complete functional language whose evaluator strong-normalizes any well-typed
expression to a canonical term; the whole language (normalization, CBOR encoding, type inference) is
formal judgement rules in a standalone standard repository, independent of any implementation. Both
evaluate to a fixed normal form before rendering: Jsonnet's is the JSON tree plus whatever didn't get
forced; Dhall's is a fully beta/alpha-normal AST term, and only Dhall's has a standardized binary encoding
and content hash.

## Jsonnet's core language after desugaring

The spec (`jsonnet.org/ref/spec.html`) gives the desugared grammar verbatim:

```
e ∈ Core ::= null | true | false | self | super | string | number
  | { { assert e } { [ e ] h e } }
  | { [ e ] : e for id in e }
  | [ { e } ]
  | e [ e ]
  | e ( { e } { id = e } )
  | id
  | local id = e { id = e } ; e
  | if e then e else e
  | e binaryop e
  | unaryop e
  | function ( { id = e } ) e
  | error e
```

Desugaring removes `!=`/`==`/`%`/`in` as binary operators, array slices `[::]`, and expression/object-level
assert messages. Nothing survives as itself: object comprehensions rewrite to an array comprehension of
`[key, capture...]` pairs in the single core comprehension form; `+:`/`+::`/`+:::` sugar rewrites to an
`if` testing `super`-membership via a generated `InSuper` check, then `super[f] + rhs` or bare `rhs`;
function defaults desugar recursively but bind at call time (not a compiled error-guard). `self`/`super`
desugaring introduces generated bindings (`$outer_self`, per-use `$outer_super_indexN`, `$outer_in_super`)
captured via `local` at the object boundary so a nested object's own `self`/`super` doesn't shadow the
outer one; original sub-expressions are recorded in a `SuperVars` list and rebound at that scope.
(Source: `core/desugarer.cpp`.)

## Jsonnet's evaluator and manifestation-as-library

Values are a tagged union: `Value { Type t; union { bool b; double d; HeapEntity *h; } }` with heap-backed
variants for STRING/ARRAY/FUNCTION/OBJECT. Evaluation is lazy throughout: array elements, function
arguments, and object field values are all thunks. A `HeapThunk` holds `{ AST *body; BindingFrame upValues;
bool filled; Value content; HeapObject *self; unsigned offset; }`; forcing it evaluates once and caches into
`content`. The VM (`core/vm.cpp`) runs an explicit frame stack with a forward/unwind goto state machine
rather than native recursion, avoiding host stack limits on deep Jsonnet recursion. Objects are a binary
tree of `HeapSimpleObject` (leaves) and `HeapExtendedObject` (`+` nodes); field lookup carries an `offset`
counter for how many `super` levels deep the reference is, so `self`/`super` resolve per-call-frame rather
than via a fixed vtable. Memory is a stop-the-world mark-and-sweep collector, triggered on allocation
growth. `go-jsonnet` mirrors the laziness contract with a `potentialValue`/`getValue` interface and a
`cachedThunk` wrapper, kept out of the core value interface "to avoid easily overevaluating."

Manifestation to non-JSON formats is ordinary Jsonnet, written in `std.jsonnet`, not the host language.
`std.manifestYamlDoc` (~160 lines) is a recursive `aux(v, path, cindent)` dispatching on `std.type(v)`,
hand-formatting YAML indentation; it needs only recursion, string concatenation, `std.type`/`std.isX`
introspection, comprehensions. `std.manifestIni` (~20 lines) is a comprehension over
`std.objectFields(body)` joined with `std.join`. `std.manifestPython` (~25 lines) is type-dispatched
recursive string-building. `std.manifestXmlJsonml` (~20 lines) walks a JsonML array (`[tag, {attrs?},
children...]`) via slicing and `std.join`. Pattern: one recursive function, runtime type-tag dispatch,
string concatenation as the only "codegen" primitive. The limit: runtime type introspection plus unbounded
recursion over an already-forced tree; no typed backend contract, ad hoc format validation, and laziness
already fully discharged by the time manifestation runs.

## Dhall's standardized normal form and semantic hash

The standard (`dhall-lang/dhall-lang/standard/`) is thirteen documents in judgement-rule notation
(`Γ ⊢ t : T`, reduction `⇥`, equivalence `≡`, shift `↑`, De Bruijn `x@n`): `beta-normalization.md` (~20 rule
groups covering constants through imports), `alpha-normalization.md` (renames bound variables to De Bruijn
indices), `type-inference.md` (~25 sections, System Fω-style typing), plus `binary.md`, `equivalence.md`,
`function-check.md`, `imports.md`, `multiline.md`, `record.md`, `shift.md`, `substitution.md`, `syntax.md`,
`versioning.md`. Sample rules, verbatim: `t ⇥ True   l₀ ⇥ l₁ / if t then l₀ else r ⇥ l₁`;
`f ⇥ λ(x : A) → b₀ ... b₂ ⇥ b₃ / f a₀ ⇥ b₃`. Beta-normalization is beta-reduction only; alpha-normalization
is a separate, later pass.

Binary encoding is CBOR (RFC 7049); every expression variant is a CBOR array tagged by an integer label:
`0` = application (`[0, f₁, a₁, b₁, …]`), `1` = lambda (`[1, A₁, b₁]` or `[1, "x", A₁, b₁]` named), `2` =
Pi/forall, `3` = binary operators (opcode 0-13), `4` = List, `6` = merge, `7`/`8` = record types/literals,
`24` = imports, `25` = `let` (flattened across nested binders), `26` = type annotation.

The semantic hash: resolve imports, beta-normalize (`e₁ ⇥ e₂`), alpha-normalize (`e₂ ↦ e₃`), CBOR-encode,
SHA-256, base16-encode. Invariant under comments/formatting (stripped at parse), bound-variable naming
(killed by alpha-normalization), and which expression/import set produced the same normal form; sensitive
to actual semantics and to `Text` literal content pulled `as Text`. Cache filenames use a multihash prefix
(`1220<base16Hash>`). An import can be pinned with `sha256:<hash>`; the resolver refuses a mismatch, giving
reproducible, mirror-independent, content-addressed imports.

## Dhall's totality and its cost

Dhall types with a restricted System Fω: every well-typed expression provably terminates, a type-system
property, not a runtime check. There is no general recursion; a self-referential `let` or function is a
type error, and the only built-in recursive structure is `List`, consumed only through
structurally-terminating primitives like `List/fold`. Recursive data (trees, JSON-like ASTs) uses
Church/Boehm-Berarducci encoding: a recursive value is its own eliminator (one case-handler per
constructor), finite by construction because every consumption is a fold, never a `Y`-combinator fixpoint.
Cost: recursive-looking code must be hand-transformed into fold form (docs.dhall-lang.org, "How to
translate recursive code to Dhall"), deep structures cost more than a native recursive type, and nothing
whose termination isn't structurally evident this way is expressible. In exchange, type-checking and
evaluation both provably complete in finite time, the property the semantic hash and import cache lean on.

## Backends: dhall-to-json / dhall-to-yaml

Both are Haskell (`dhall-json` package, `Dhall.JSON` module), not Dhall itself. The normalized expression
converts to an intermediate `aeson` `Value`; `dhall-to-yaml` renders that same `Value` through `yaml`/
`HsYAML` (JSON's `Value` is a strict subset of YAML, so the YAML backend inherits JSON's ceiling).
Structural opposite of Jsonnet: Dhall's renderers are host-language code over a decoded native value;
Jsonnet's are guest-language code over the same guest value tree the evaluator built.

## Mechanisms worth naming (with their tradeoff)

- **Desugar-to-small-core (Jsonnet).** One evaluator handles ~15 constructs. Tradeoff: the desugarer is a
  second, informally-specified pass needing its own testing; errors can point at desugared code.
- **Judgement-rule standard independent of implementation (Dhall).** Checkable prose lets independent
  implementations converge on bit-identical behavior. Tradeoff: ~13 formal documents, ongoing
  spec-engineering cost, steep read for non-PL-theory audiences.
- **Content-addressed imports via semantic hash (Dhall).** Reproducible, mirror-independent trust.
  Tradeoff: requires normalization to be tractable before hashing; unusable under non-termination.
- **Manifestation written in the guest language (Jsonnet).** Adding a format is a library function, not an
  interpreter change. Tradeoff: every function re-derives type introspection from scratch, no typed backend
  contract, only text output possible.
- **Church-encoded recursion under strong normalization (Dhall).** Termination becomes a language-wide
  guarantee with no separate analysis pass. Tradeoff: no native recursive types; fold encodings are a real
  ergonomic tax, deep values costly to normalize.
- **Frame-stack (non-native-recursive) evaluator (Jsonnet C++ VM).** Deep recursion doesn't blow the host
  stack. Tradeoff: goto-based control flow is harder to read than a naive recursive `eval`.

## What would not transfer to no_std, no-alloc Rust

- **Jsonnet's thunks** (`HeapThunk.filled`/`.content`) are heap-shaped by necessity: aliased from multiple
  bindings, forced exactly once, needing a boxed, GC-tracked object with identity. No no-alloc mapping
  exists without eager evaluation (losing laziness) or an arena standing in for the heap.
- **Jsonnet's mark-and-sweep GC and object model** (`HeapExtendedObject`/`HeapSimpleObject` tree,
  `offset`-based super tracking) assume arbitrary cyclic, pointer-linked graphs (`super` chains, closures
  over arbitrary environments); no stack-only substitute exists for runtime self-reference.
- **Dhall's Church/Boehm-Berarducci encoding** is allocation-light in spirit (a recursive value is a
  closure), but a real normalizer still builds new AST nodes per reduction, and existing implementations
  allocate freely. Transfer depends on statically bounding closure/fold depth, which general Dhall programs
  do not guarantee.
- **CBOR and SHA-256** are no_std-compatible in principle, depending on neither GC nor laziness; the most
  portable part of Dhall's design to no-alloc, modulo a fixed-capacity buffer instead of a growable one.

## Sources

- [Jsonnet Specification](https://jsonnet.org/ref/spec.html)
- [google/jsonnet core/desugarer.cpp](https://github.com/google/jsonnet/blob/master/core/desugarer.cpp)
- [google/jsonnet core/vm.cpp](https://github.com/google/jsonnet/blob/master/core/vm.cpp)
- [google/jsonnet stdlib/std.jsonnet](https://github.com/google/jsonnet/blob/master/stdlib/std.jsonnet)
- [google/go-jsonnet thunks.go](https://github.com/google/go-jsonnet/blob/master/thunks.go)
- [google/go-jsonnet value.go](https://github.com/google/go-jsonnet/blob/master/value.go)
- [dhall-lang standard/beta-normalization.md](https://github.com/dhall-lang/dhall-lang/blob/master/standard/beta-normalization.md)
- [dhall-lang standard/alpha-normalization.md](https://github.com/dhall-lang/dhall-lang/blob/master/standard/alpha-normalization.md)
- [dhall-lang standard/type-inference.md](https://github.com/dhall-lang/dhall-lang/blob/master/standard/type-inference.md)
- [dhall-lang standard/binary.md](https://github.com/dhall-lang/dhall-lang/blob/master/standard/binary.md)
- [dhall-lang standard/imports.md](https://github.com/dhall-lang/dhall-lang/blob/master/standard/imports.md) (semantic integrity check section)
- [Dhall Safety guarantees discussion](https://docs.dhall-lang.org/discussions/Safety-guarantees.html)
- [Dhall: How to translate recursive code to Dhall](https://docs.dhall-lang.org/howtos/How-to-translate-recursive-code-to-Dhall.html)
- [dhall-json / Dhall.JSON on Hackage](https://hackage.haskell.org/package/dhall-json)
- [dhall-haskell dhall-json README](https://github.com/dhall-lang/dhall-haskell/blob/main/dhall-json/README.md)
