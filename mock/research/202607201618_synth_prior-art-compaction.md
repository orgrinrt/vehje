# Procedural-docs-arc prior art: compacted, one section per document

**Date:** 2026-07-20
**What this is:** a compaction of the eight prior-art docs in
`202607190650_mockspace-procedural-docs-arc/prior_art/`. One section per document, conveying the whole
picture of each while dropping the exact API names, line counts, and verbatim code. For the specifics, read
the named source; for the cross-cutting domain views, read the five `202607201618_synth_*` domain docs.

## LMS and Delite (`01_lms_delite.md`)

Lightweight Modular Staging is a Scala library that turns ordinary-looking code into a builder of an
intermediate-representation graph, by giving every staged value a special representation type instead of its
ordinary type and overloading its operations so each call constructs an IR node rather than computing a
result. Common-subexpression elimination falls out for free rather than being a pass, because IR nodes are
immutable case-class values built from already-staged operands, so two structurally-equal constructions
compare equal and node construction is memoized on a lookup; the precondition, which is load-bearing, is that
a node must be built only from already-staged operands or the equality silently misses. Side effects are not
kept in emission order; each effectful node carries an explicit dependency edge list computed once at staging
time from its read and write sets, so a scheduler that respects those edges reproduces the total order an
eager interpreter would have produced, while pure computation is free to move to its uses, and each
conditional or loop body captures its own effects locally. A backend is written as one code-emission arm per
IR node type, and backends compose by the host language's trait linearization, so the IR node set and the
backend set each grow independently, which is the expression-problem property the whole design demonstrates.
Delite layers a small family of parallel-loop nodes and a multi-target runtime on the same IR. The elegance is
GC-and-host-dependent: native control flow needs a compiler fork to intercept `if`/`while`, runtime
recompilation depends on a hosted compiler loading generated code back into the process, and CSE, the effect
lattice, and scheduling all run over ambient mutable global state and persistent lists that allocate on every
staged statement; a no-alloc port needs an explicit interner, fixed-capacity graph storage, and a different
dispatch mechanism than trait linearization.

## Jsonnet and Dhall (`02_jsonnet_dhall.md`)

Two configuration languages that both evaluate to a fixed normal form before rendering, taking opposite
positions on totality. Jsonnet is a lazy functional language whose full surface desugars into a small
documented core evaluated by a tree-walking interpreter that produces a value tree, with output formatters for
non-JSON formats written in Jsonnet itself as recursive type-dispatched string-building functions, so adding a
format is a library function rather than an interpreter change, at the cost that each formatter re-derives
type introspection with no typed backend contract and only text output. Its evaluation is lazy throughout
(elements, arguments, and fields are thunks forced once and cached), its objects are a tree resolving
self-and-super references per call frame, and its interpreter uses an explicit frame stack rather than native
recursion to survive deep programs, all of it garbage-collected. Dhall is a total, non-Turing-complete
language whose evaluator strong-normalizes any well-typed expression to a canonical term; totality is a
type-system property, not a runtime check, achieved by forbidding general recursion and expressing recursive
data as its own fold-based eliminator, at a real ergonomic tax (recursive-looking code must be hand-rewritten
into fold form, deep structures cost more, and anything not structurally terminating is inexpressible). The
payoff of totality is a content-addressed semantic hash: normalize, canonicalize bound-variable names, binary-
encode, and hash, giving a value invariant under formatting and naming and sensitive only to semantics, which
makes imports reproducible and pinnable, and which is unusable without guaranteed termination. Dhall's
renderers, unlike Jsonnet's, are host-language code over a decoded native value. The transfer verdict: thunks
and the collector are heap-shaped by necessity and resist no-alloc outright (forcing eager evaluation or an
arena), while the binary encoding and hash are portable.

## Pandoc and Djot (`03_pandoc_djot.md`)

Two document systems by the same author, contrasting on structural uniformity. Pandoc parses many markup
formats into one shared abstract syntax tree and renders it to many formats; its abstract syntax has a
moderate fixed set of block and inline constructors where which nodes can carry attributes is a
per-constructor decision baked into the sum type. A writer is one function plus one entry appended to a
dispatch table, and the traversal that walks the tree is a hand-instantiated typeclass over every
needle-in-haystack pair, chosen over a generic reflective traversal for speed, whose exhaustive per-constructor
match forces a compile error when a new constructor is added and left unhandled. The cost model surfaces
directly: a standalone writer is a few hundred lines exhaustively matching every constructor, several related
writers cluster under one mode-flagged shared implementation so the marginal format is nearly free, and a
third shape is a thin adapter into a separate library that owns the syntax knowledge. Raw pass-through content
carries a format tag matched by each writer by literal string, falling through to a warning rather than an
error, which is the silent-drop failure mode a closed-universe coverage model forces. Djot is a smaller,
later, linear-time markup language whose abstract syntax is far more uniform: every node carries attributes
through a single wrapper, so extensibility stops being a per-constructor decision and a new node type gets
attributes free, and its filter API is a hand-written mutable-stack walker generalizing over any homogeneous
children array rather than a per-constructor typeclass. Both systems hold and walk a document as an unbounded
recursive sum type, heap-allocated, walked by boxed closures or reflection, with growable collections at every
level; a no-alloc port needs a closed vtable of node kinds or generated per-type-pair recursion.

## Racket and Scribble (`04_racket_scribble.md`)

Racket is a language-oriented Lisp: source is read into syntax objects, macro-expanded to a small fixed core
of expression forms, and the core is what runs, with a language-selection mechanism that lets a file pick a
reader (turning characters into syntax objects) and an expander starting point, so a language is customized by
exporting a replacement for the implicit form that wraps a module body, which can transform the body before
delegating. Hygiene is a per-expansion scope-set operation baked into the expander, so a macro's template
identifiers resolve against its definition context and use-site identifiers keep their use-site binding.
Scribble is a documentation toolchain built as an ordinary Racket language: its reader turns an at-sign syntax
into function calls, its module language folds the body's runtime values into a single immutable document tree
of transparent, serializable structs, and a four-pass pipeline resolves cross-references before a pluggable
renderer walks the tree. The passes are a fixed-point traverse over an immutable persistent hash (iterating
until two successive snapshots are equal, so a feature in one part can depend on a value another part produces
later), a collect pass writing into shared tables, a resolve pass matching references to targets and expanding
deferred elements, and a render pass calling backend leaf methods; deferred content comes in two tiers, one
retried within the traverse pass as state grows and one resolved once after collection settles. A render
backend is a mixin over a shared engine overriding about a dozen leaf methods, so a new output form does not
reimplement the pipeline. The content type is a closed recursive union that admits, as one of its cases,
anything implementing an open conversion protocol, which is the seam letting the model hold a picture or plot
without knowing those types exist. Syntax objects, the persistent-hash fixed point, serialization, dynamic
typing, and unbounded nesting all assume a collector and resist no-alloc.

## Typst (`05_typst.md`)

Typst is a markup-and-scripting language with a pure-Rust compiler pipeline: parse to a syntax tree, evaluate
into a tree of type-erased content values, realize that tree by repeatedly applying show and set rules under a
style chain until only well-known base elements remain, lay it out into a position-and-size frame tree, and
feed that one documented intermediate representation to per-target backends for raster, vector, print, and an
HTML DOM. Everything is heap-based. Content is a single hand-rolled type-erased reference-counted pointer with
a custom per-element vtable rather than an enum of variants, cheap to clone by a refcount bump and mutated by
explicit clone-on-write, so one arity-one content type holds arbitrarily-shaped data behind the vtable. A
single macro over an annotated struct generates the whole element system, making each field simultaneously a
constructor argument and a settable property that style rules further up the tree can target. Concatenation and
iteration funnel through one flattening sequence element that acts as the identity-and-append monoid. The
scripting value type is a couple dozen variants where content and styles are first-class alongside numbers and
functions, plus a dynamic escape hatch for otherwise-unrepresentable values. Styles are a borrowed linked
chain walked from innermost to outermost per lookup rather than an eagerly merged map, so nested scopes cost
nothing to push and lookups optionally fold matches up the chain. Content whose value depends on the finished
document is handled by re-running the pipeline a bounded number of times, reusing the compiler's own
memoization-invalidation machinery to detect when introspected state stops changing, diagnosing rather than
looping on non-convergence. The HTML target is the honest divergence: it bypasses the frame tree and converts
the realized element tree straight into a DOM, and its own docs state it cannot always know the best semantic
representation, so it is incomplete and experimental. Removing heap use would require replacing the
type-erased representation, the refcounted sharing, and the arena-based realization at once, none of them
incidental.

## MLIR ODS and nanopass (`06_mlir_ods_nanopass.md`)

Two systems that generate compiler machinery from a declarative record so the writer does not hand-write it
per definition. MLIR's operation-definition specification is a build-time generator: an operation is written
as a record naming its arguments, results, traits, and optionally an assembly format and verifier, and a tool
expands it into a class with named accessors, builders, and verification hooks, inferring structural traits
from the argument and result fields and killing the repetitive hand-written boilerplate. The mechanism that
lets generic passes work over an open, ever-growing set of operation kinds is trait and interface dispatch: a
pass never switches on concrete operation identity but queries a marker trait or casts to an interface, so
canonicalization, dead-code elimination, and the like operate over operations that did not exist when the pass
was written, with dependencies composing as supertraits, at the cost that an operation forgetting to declare a
trait is invisible to the pass that needs it. Declarative rewrite rules cover the regular cases with a native-
code escape hatch and an explicit list of constructs they cannot express, and a special interim cast operation
bridges partially-converted intermediate representation mid-pass with a dedicated later cleanup that makes any
survivor diagnosable. Nanopass is the same record-to-code idea for whole passes: a grammar-definition form
declares an intermediate language and generates its record types, predicates, and constructors, optionally as
a diff against a prior language, and a pass form lets the writer supply only the clauses that change while the
framework autogenerates pass-through clauses that reconstruct every unmentioned form and recurse into its
subterms, which is the load-bearing ergonomics that makes a one-form-change pass cheap. It was validated on a
commercial compiler, replacing a few large passes with many small ones across many intermediate languages at a
modest compile-time cost, and its authors document its own rough edges honestly (exact-name matching on
extension diffs, and output forms that must closely match input forms). The generation idea transfers to Rust
macros and derives; the runtime object models (hand-rolled runtime type information, context-owned arena
storage, Scheme's homoiconic data model) do not without redesign.

## Rust final encoding and no-alloc arenas (`07_rust_final_encoding_arenas.md`)

Two engineering surveys for a no-heap Rust intermediate representation. The first is tagless-final encoding,
where a term is written once against a trait whose associated representation type each backend chooses, so one
interpreter evaluates and another pretty-prints or emits code without the term changing. Multiple independent
write-ups converge on the same shape and the same honest downgrade: Rust lacks higher-kinded types, so past
the simplest first-order case the encoding cannot erase a term behind dynamic dispatch, forcing monomorphized
selection of the interpreter as a type parameter, boxed closures for laziness, and inference workarounds, and
it never reaches the ergonomics of the ML or Haskell original because a term generic over its interpreter
cannot be stored as a value and replayed against several interpreters without recompiling. The load-bearing
empirical finding is that no real Rust compiler ships tagless-final; every production tool surveyed uses
initial encoding, parsing once into a concrete arena- or index-held tree and walking it with ordinary generic
functions or a visitor, which sidesteps every object-safety and boxing problem and is the pragmatic hybrid.
The second part is the arena and interning machinery. A vocabulary of typed integer keys, dense primary maps
that hand out keys, secondary maps that attach extra properties to existing keys, and a packed optional that
reserves a sentinel bit-pattern gives an index-based representation; variable-arity children are stored as a
small handle into one shared pool rather than an owned growable vector per node, so the whole representation's
edges have one backing allocation. Real compilers converged on three flat-tree strategies (flat array with a
parent index, child-list by handle, and a red-green split giving both parent and child access plus structural
sharing at the cost of two representations). The interning survey's finding is negative and useful: no
surveyed interner avoids allocation entirely; every one depends on an allocator directly or through an arena.
The confirmed no-allocation toolkit is therefore narrow: only fixed-capacity collections and a bump allocator
that takes a caller-supplied byte slice truly avoid the allocator, and "no heap after one fixed reservation"
means hand-assembling a fixed-capacity arena, a hand-adapted handle-into-a-shared-pool indirection, and a
hand-built fixed-capacity interner, because no single crate combines them.

## Terra, Rust proc-macros, and Zig comptime (`08_terra_procmacro_comptime.md`)

Three compile-time metaprogramming systems compared on what crosses from generation time into generated
output. Terra is a statically-typed low-level language embedded in a scripting host that stages its
construction (functions, types, and quoted fragments are first-class host values sharing one lexical scope),
while the generated Terra code executes with no dependency on the host runtime and can be saved as a
standalone object, assembly, or executable artifact. Its quote, escape, and fresh-symbol mechanics splice a
host value into a Terra term; specialization is eager, so a host value is captured by value when the term is
built and an unrepresentable spliced value fails at compile time, never at run time, and hygiene is guaranteed
by minting a fresh unique name per bound variable. Rust procedural macros are a separate build-time crate that
maps a token stream to a token stream with no access to type information, other items' bodies, or semantic
analysis; hygiene is chosen per identifier through span tiers, the only output the compiler accepts back is
Rust tokens, but the input language and the side effects during expansion are unconstrained, so a macro can
consume a shader or another language and emit Rust, or run arbitrary build-time input and output. Zig's
compile-time evaluation is neither a separate language nor a token facility but an evaluation-time qualifier on
ordinary code, so the same functions and control flow run either at compile time via the compiler's
interpreter or at run time; generics are just functions taking and returning types, reflection is an ordinary
compile-time value, and a compile-time value becomes runtime data wherever nothing forces it to stay
compile-time-known, with a backwards-branch budget as the termination guard. The consequence is that Zig has
no macro system and no cross-language target: compile-time evaluation only ever produces more Zig, giving up
the ability to emit surface syntax the grammar does not express or to target another language, in exchange for
one uniform mechanism covering generics, constant folding, and specialization. The comparison the document
centers is the table of what crosses (a host value, a token, or nothing), how an unrepresentable value fails
(all at compile time), and what constrains the output language; the generators themselves allocate, but the
emitted artifacts can be allocation-free.

## Sources

The eight compacted documents:
`202607190650_mockspace-procedural-docs-arc/prior_art/01_lms_delite.md`, `.../02_jsonnet_dhall.md`,
`.../03_pandoc_djot.md`, `.../04_racket_scribble.md`, `.../05_typst.md`, `.../06_mlir_ods_nanopass.md`,
`.../07_rust_final_encoding_arenas.md`, `.../08_terra_procmacro_comptime.md`. Each carries its own primary
citations (the LMS/Delite papers and source; the Jsonnet spec and Dhall standard; pandoc-types and djot; the
Racket reference and Scribble source; the Typst source tree; MLIR ODS/traits/interfaces and the nanopass
paper; the tagless-final write-ups and the cranelift-entity/heapless/arena crate docs; the Terra paper, the
Rust proc-macro reference, and the Zig language reference).
