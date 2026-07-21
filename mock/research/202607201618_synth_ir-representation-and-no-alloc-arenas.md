# IR representation and no-alloc arenas: what the IR is, and how it is held with no heap

**Date:** 2026-07-20
**Domain:** the shape of a shared intermediate representation and the machinery to hold it without a heap.
Core-form vocabularies and form counts, per-node extensibility, flat-array-plus-index versus
child-list-by-handle versus red/green, hash-consing and CSE, interning, the confirmed no-`alloc` toolkit, and
the relative-offset / position-independent buffer that connects an IR to a wire format.
**Synthesised from:** `prior_art/07_rust_final_encoding_arenas.md` (§4-9),
`prior_art/06_mlir_ods_nanopass.md`, `prior_art/05_typst.md`, `prior_art/01_lms_delite.md`,
`prior_art/03_pandoc_djot.md`, `prior_art/04_racket_scribble.md`, `prior_art/02_jsonnet_dhall.md`, and the
arena/relative-offset material in `runtime-value-transfer-study/zerocopy_formats.md` and
`.../wasm_canonical_abi.md`.

## Two questions this domain answers

First, what is the IR: how many core forms, how open is the node set, and how does a node carry variable-arity
children and attributes. Second, how is it stored under no_std with no allocation after an initial fixed
reservation, which is the standing tax every prior art's transfer section flags. The two questions are
coupled: the representation choice (flat array, child-list-by-handle, red/green) determines what the no-alloc
backing has to provide.

## Core-form vocabularies: the surveyed counts

The corpus gives real, small core-form counts, useful as calibration for "how many forms does a shared
substrate need":

- **Jsonnet** desugars its full surface into roughly 15 core constructs (`null`/`true`/`false`/`self`/`super`/
  string/number, object, object-comprehension, array, index, apply, id, `local`, `if`, binary op, unary op,
  function, error). Nothing survives as itself; object comprehensions rewrite to a single array-comprehension
  form, `+:`/`super` sugar rewrites to generated `InSuper` checks, `self`/`super` introduce generated bindings
  captured via `local`. One evaluator handles the desugared core.
- **Racket** fully-expands to 15 distinct expression forms (`#%plain-lambda`, `case-lambda`, `if`, `begin`,
  `begin0`, `let-values`, `letrec-values`, `set!`, `quote`, `quote-syntax`, `with-continuation-mark`,
  `#%plain-app`, `#%top`, `#%variable-reference`, `#%foreign-inline`), plus a handful of module/top-level
  forms. `#lang` picks a reader and an expander start; a language customizes semantics by exporting a
  `#%module-begin` that wraps the body before delegating. The whole `#lang` mechanism is "reader plus a
  `#%module-begin`."
- **Pandoc** is 14 `Block` and 19 `Inline` constructors; **Djot** is ~13 blocks and ~22 inlines. The
  structural difference that matters: in Pandoc, which nodes carry attributes (`Attr = (id, classes, kv)`) is
  a per-constructor decision baked into the sum type (only `Div`/`Span`/`Header`/`Link`/`Table`/... carry
  `Attr`; `Str`/`Emph`/`Space` carry none). Djot inverts this: `data Node a = Node Pos Attr a` wraps *every*
  node, so "can this carry `{#id .class k=v}`" stops being a per-constructor decision and a new node type gets
  attributes free by being wrapped. This is the cleanest statement of "uniform per-node extensibility versus
  per-constructor extensibility," and Djot's wrapper is the more extensible shape.
- **Typst** has a 28-variant `Value` enum where `Content` and `Styles` are first-class variants alongside
  numbers/strings/functions, and `Dyn(Dynamic)` is the escape hatch. Its `Content` is *not* an enum of
  variants at all (see below).

The calibration: a shared expression/evaluation core lands in the low tens (Jsonnet ~15, Racket ~15), and the
extensibility axis (per-node uniform attributes, an open op set, or a closed enum plus one `Dyn` escape) is a
separate decision from the core-form count.

## Three representation strategies, and their access tradeoffs

`prior_art/07` §8 gives the cross-cutting comparison of how real Rust compilers hold a tree, and it is the
reference for the choice:

1. **Flat array + parent-index.** rustc HIR stores each owner's nodes in `IndexVec<ItemLocalId,
   ParentedNode>`, a dense zero-based `Vec`-backed index where each node pairs with its parent's id; scoped
   per-owner so incremental recompilation can invalidate one owner without touching others. Adrian Sampson's
   "Flattening ASTs" is the minimal statement: `enum Expr { Binary(BinOp, ExprRef, ExprRef), Literal(i64) }`
   with `struct ExprRef(u32)` into `struct ExprPool(Vec<Expr>)`. Measured wins: cache locality from
   contiguity, 50% smaller references (`u32` versus a 64-bit pointer), bump-style allocation with no per-node
   `malloc`, and whole-arena deallocation (the pointer baseline spent "38% of runtime" in deallocation, gone
   once the pool drops as one `Vec`); overall "2.4x." Good for "walk up," awkward for "get my Nth child"
   without also storing child ranges. The fully-flattened linear-scan variant assumes children are allocated
   before parents (bottom-up construction), which constrains order-independent traversal.
2. **Child-list-by-handle.** `cranelift-entity`'s `EntityList<T>` + `ListPool<T>` is the direct answer to "N
   children, no `Vec`": an `EntityList` is 4 bytes (one `u32` index into the pool) versus 24 bytes for a
   `Vec<T>` (ptr+len+cap), and every node shares one `ListPool`, so the whole IR's variable-arity edges have
   exactly one backing allocation grown geometrically, not one per node. Operations take the pool explicitly
   (`push(&mut self, elem, pool)`, `as_slice(&self, pool)`) because the list is only a handle. Internally each
   list block is power-of-two sized so the pool reuses freed blocks by size class; `clear(&mut self)` forgets
   all lists and retains the memory for reuse (LIFO bulk allocator). oxc and swc use the same shape via
   bumpalo-backed `Vec`s. Good for "get my children," parent tracking is a separate concern.
3. **Red/green (rowan).** The green tree holds position-independent, immutable, structurally-shared nodes
   (identical subtrees literally share one allocation, since green nodes carry no absolute offsets); the red
   tree is a thin on-demand layer where each node carries a parent reference plus an absolute start offset, so
   `offset + child-widths` gives every range. This is the one design giving O(1) get-my-children AND O(1)
   get-my-parent AND structural sharing, at the cost of two parallel representations. Three independent
   production tools converged on it (rust-analyzer's rowan, cstree, Biome's `biome_rowan`), itself descended
   from Roslyn and Swift's libsyntax.

The supporting node-key machinery (`prior_art/07` §4): `EntityRef` is a `Copy + Eq` wrapper over a small
integer (`fn new(usize) -> Self`, `fn index(self) -> usize`); `entity_impl!` generates it for a `u32`
newtype. `PrimaryMap<K,V>` allocates dense keys via `push(v) -> K` and deliberately does not `Deref` to a
slice, so access is only through the typed key. `SecondaryMap<K,V>` attaches a second independent property to
existing keys. `PackedOption<T: ReservedValue>` reserves one sentinel bit-pattern of the integer to mean
absent, so `Option<Key>` costs zero extra bytes. These four (`EntityRef`, `PrimaryMap`, `SecondaryMap`,
`PackedOption`) plus `EntityList`/`ListPool` are the vocabulary of an index-based IR.

## Content as one erased type instead of an enum: Typst's alternative

Typst is the counter-shape worth knowing: `Content` is `#[repr(transparent)]` over a hand-rolled type-erased
`Arc` (`RawContent { ptr: NonNull<Header>, elem: Element (a &'static ContentVtable), span }`), one
`Box::into_raw` allocation per node, refcounted exactly like `Arc` (relaxed increment on clone, release/
acquire on drop), mutated through `make_unique` clone-on-write. There is no enum-of-variants layer; the
element type lives entirely behind the `elem` vtable pointer plus an `unsafe fn data::<E>()` cast, and
`Packed<T>` gives a statically-typed view after an `is::<T>()` check with no second allocation. The `#[elem]`
macro generates, per annotated struct, the real struct (required fields bare, others as `Settable<Self, I>`
slots keyed by a const index), a `new` constructor, `with_<field>` builders, a `Field<Self, I>` const per
field, the `'static ContentVtable`, one `FieldVtable` per field, and `Construct`/`Set` impls. So one macro
makes a field simultaneously a constructor argument and a settable property a `set` rule can target. The
content monoid: `SequenceElem` (one `Vec<Content>` child) is identity-plus-concatenation, `Content::empty()`
is a process-wide singleton, `Add for Content` special-cases both operands already being sequences (extend in
place), and `#for` funnels through `Content::sequence`. This is a good reference for "one arity-one content
type holding arbitrarily-shaped data," and its cost is stated bluntly: three load-bearing heap pieces at once
(the type-erased representation needs some indirection, the refcounted sharing makes `+`/`#for` cheap, the
`bumpalo` realization pass), none incidental.

## Hash-consing and CSE for free, and its precise precondition

LMS (`prior_art/01`) gives the mechanism a no-alloc IR wants to replicate: CSE is not a pass. Every IR node
`Def` is a case class built from already-staged operands; `findOrCreateDefinition` looks for a prior statement
whose payload structurally equals `d` (case-class `equals`/`hashCode`) and reuses its symbol on a hit,
otherwise mints a fresh `Sym`. **The precondition is exact and load-bearing:** every `Def` must be built from
already-staged operands (no closures or mutable state inside), or two "same" nodes will not compare equal and
CSE silently misses. The no-alloc port of this is an explicit interner: an arena for the node bytes plus a
hash-consing table keyed by structural hash. The effect-ordering half of LMS is covered in the
purity/effects synthesis doc.

## Interning, and the honest no-alloc status

`prior_art/07` §6 is the reference on interning, and its finding is negative and useful: **no interner
surveyed works with zero `alloc`.** rustc's `Symbol`/`Interner` is a pure index with all operations on the
index, backed by a `DroplessArena` (alloc-based); matklad's minimal `Interner { map: HashMap<String,u32>, vec:
Vec<String> }` is explicitly wasteful (two heap allocations per string); `lasso` offers `no_std`+`alloc`;
`ustr` never frees; `stringleton` requires an allocator for its global registry. Every one either depends on
`alloc` directly or on an arena that does. A heap-free interner must be hand-built on a fixed-capacity map
(e.g. `heapless::FnvIndexMap`) plus a fixed-capacity byte buffer; nothing ready-made exists.

## The confirmed no-`alloc` toolkit

`prior_art/07` §9 is the single most actionable table in the corpus, because it separates "compiles under
`no_std`" from "actually avoids `alloc`," and almost everything advertised as `no_std` is the weaker sense:

- **Fixed-capacity node storage:** `heapless::Vec<Node, N>` (hard compile-time `N`, genuinely constant-time
  `push`, no uncatchable OOM) is the real zero-heap arena. `id-arena`, `la-arena`, `typed-arena`, `slotmap`
  all *grow a `Vec`* and therefore depend on `alloc` even when labeled `no_std`.
- **Variable-arity children without per-node `Vec`:** the `EntityList`/`ListPool` *shape* is right (4-byte
  handle into one shared pool), but `cranelift-entity` itself is `Vec`-backed; reproducing the same
  index-into-a-shared-pool idea over a `heapless`-style fixed buffer is the no-heap version, and no ready-made
  crate combines "`EntityList`-style handles" with "zero-growth backing."
- **Bump allocation:** `bump-into` is the one bump allocator that takes memory from a caller-supplied `&mut
  [u8]` with no allocator dependency at all (genuinely heap-free, positioned for embedded); `bumpalo` and
  `fixed-bump` are `no_std` but still call `alloc` for their backing chunks.
- **Interning:** hand-built only (see above).

The blunt summary the doc lands on: only `heapless` and `bump-into` were confirmed to avoid `alloc` entirely;
"no heap after one fixed reservation" in Rust today means assembling `heapless` (or a hand-rolled fixed arena)
plus a hand-adapted `EntityList`/`ListPool` indirection plus a hand-built fixed-capacity interner. There is no
one crate.

## The IR that is also the wire format: relative offsets and position independence

The value-transfer corpus adds the piece that connects an in-memory IR to a serialized one, and it is the same
index-based discipline pushed one step further. `zerocopy_formats` documents the recurring shape: a flat,
self-describing buffer whose internal links are *relative offsets* read in place under bounds checks. Cap'n
Proto packs a 30-bit signed word offset "relative to the end of the pointer word itself," so a message can be
mmap'd or relocated with no pointer fixups and read by pure pointer-chasing; rkyv's `RelPtr` is the same idea
expressed as Rust types (`ArchivedVec<T>`, root-at-the-end layout because the root is written last once child
offsets are known); FlatBuffers uses `uoffset_t`/`soffset_t` forward offsets plus a vtable for
forward/backward compatibility. The load-bearing insight for an IR: an index-into-a-pool (the `EntityList`
shape) and a relative-offset-into-a-buffer (the Cap'n Proto shape) are the same position-independent link, one
in element units, one in byte units, and choosing indices makes the in-memory IR and the wire form the same
bytes. The cost is uniform across all of them: you give up relocation-and-grow-in-place freedom (layout must
be finalized before handing over, hence back-to-front construction), alignment becomes a contract not a
detail, and an untrusted buffer needs a bounds-checked traversal (Cap'n Proto's mandatory 64 MiB traversal and
64-deep pointer limits, rkyv's `bytecheck`). WebAssembly's linear-memory model (`wasm_canonical_abi`) is the
same fact at the boundary: anything not a scalar crosses as an offset into a shared flat memory, genuinely
zero-copy within one memory, and the Canonical ABI lays nested records contiguously with list/string fields
as `(ptr, len)` pairs, walked by recursive descent over offsets. That structural-descent-over-offsets is
exactly what a flat IR walk is.

## Consolidated: the representation decision

For a no-alloc shared IR, the corpus points at: a small closed core (low tens of forms) plus an open
extensibility axis chosen deliberately (uniform per-node attributes like Djot, an open op set with trait
dispatch like MLIR, or a closed enum plus one `Dyn` escape like Typst); nodes stored in a fixed-capacity arena
(`heapless`-style) addressed by `u32` keys (`EntityRef`/`PackedOption` discipline); variable-arity children as
`EntityList`-style handles into one shared pool over that fixed backing; interning hand-built over a
fixed-capacity map plus byte buffer; and, if the IR is also the wire form, relative-index links so the
in-memory and serialized shapes are one, with a bounds-checked traversal on the untrusted path. Red/green is
the richer alternative when both parent and child access plus structural sharing are needed, at the cost of
two representations.

## Sources

Primary synthesised docs:
`202607190650_mockspace-procedural-docs-arc/prior_art/07_rust_final_encoding_arenas.md` (§4-9, the arena/
interner/flat-tree survey and the confirmed no-alloc toolkit), `.../06_mlir_ods_nanopass.md` (op/dialect
model, grammar-driven records), `.../05_typst.md` (`Content`/`Value`, the `#[elem]` macro), `.../01_lms_delite.md`
(CSE-by-structural-equality), `.../03_pandoc_djot.md` (per-node versus per-constructor extensibility),
`.../04_racket_scribble.md` (core forms), `.../02_jsonnet_dhall.md` (desugared core);
`202607201227_runtime-value-transfer-study/zerocopy_formats.md` and `.../wasm_canonical_abi.md`
(relative-offset buffers, linear memory).

Key underlying citations: `cranelift-entity` docs (`EntityRef`/`EntityList`/`ListPool`/`PackedOption`);
`heapless`, `bump-into`, `bumpalo` docs; rustc HIR and `rustc_arena`/`Symbol`; rowan/cstree/`biome_rowan`;
Adrian Sampson, "Flattening ASTs"; oxc and swc arena reports; Cap'n Proto encoding spec; rkyv; the WebAssembly
core spec and Component Model Canonical ABI.
