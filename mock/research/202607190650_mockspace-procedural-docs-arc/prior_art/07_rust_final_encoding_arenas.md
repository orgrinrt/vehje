# Rust final encoding + no-alloc arena machinery: prior art

Scope: the Rust-specific engineering for (1) tagless-final term encoding across multiple
interpreter backends and (2) arena/interning machinery for a `no_std`, no-heap IR. Reporting
only; no recommendation on whether the system under design should adopt any of it.

## 1. Tagless-final in Rust: real examples, exact trait shapes

Three independent write-ups implement the pattern in Rust and converge on the same shape.

**First-order (no GAT needed).** From "Efficient, Extensible, Expressive: Typed Tagless Final
Interpreters in Rust" (getcode.substack.com):

```rust
trait ExprSym {
    type Repr;
    fn lit(i: i32) -> Self::Repr;
    fn neg(r: Self::Repr) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}

struct Eval;
impl ExprSym for Eval {
    type Repr = i32;
    fn lit(i: i32) -> Self::Repr { i }
    fn neg(r: Self::Repr) -> Self::Repr { -r }
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr { r1 + r2 }
}
```

A term generic over the interpreter is a generic function `fn term<E: ExprSym>() -> E::Repr { E::add(E::lit(1), E::neg(E::lit(2))) }`, instantiated per backend by monomorphization. No enum, no `Box<dyn Expr>`, no match.

**Higher-order (needs a type constructor per node, e.g. for embedded functions).** Same source, and independently in Louy2's "(Ok) Tagless Final in Rust" (dev.to), using a GAT:

```rust
trait ExprSym {
    type Repr<T>;
    fn int(i: i32) -> Self::Repr<i32>;
    fn lam<A, B, F: Fn(Self::Repr<A>) -> Self::Repr<B>>(f: F) -> Self::Repr<Fun<A, B>>;
    fn app<F: Fn(A) -> B, A, B>(f: Self::Repr<F>, arg: Self::Repr<A>) -> Self::Repr<B>;
}
```

and, in the dev.to post's own words, `type Repr<T>` is "the key ingredient in the final style," stable since GATs landed in Rust 1.65 (2022-11; blog.rust-lang.org/2022/10/28/gats-stabilization). Two interpreters shown for the same `Lang` trait: `type Repr<T> = T` (a direct evaluator) and `type Repr<T> = String` (a pretty-printer/codegen backend), both implementing one shared trait. The author's stated payoff: "You actually have to mark the output type correctly as `L::Repr<bool>`! If you pass `L::int(1)` to `L::add` it's a compile time error."

Louy2's companion post "(Not) Finally Tagless in Rust" (dev.to) implements the classic Kiselyov arithmetic-with-conditionals example WITHOUT GATs, using a plain `trait Prim: Sized { fn int(v: i64) -> Self; fn bool(v: bool) -> Self; } trait AddOp { fn add(x: Self, y: Self) -> Self; } trait IfOp { fn leq(x: Self, y: Self) -> Self; fn if_(cond: Self, x: Box<dyn Fn() -> Self>, y: Box<dyn Fn() -> Self>) -> Self; }`. His own conclusion, stated directly: this is NOT truly tagless final in Rust because it cannot, compared to the ML/Haskell original with real HKT, (a) enforce the operators' types as tightly, (b) enforce parametricity of the operators, or (c) avoid runtime tags/allocation for primitives. This is presented as the honest downgrade Rust forces relative to the OCaml/Haskell paper.

## 2. What breaks, and whether GATs help

- **No higher-kinded types (HKT).** Rust has no `* -> *` type variables. GATs (`type Repr<T>;` on a trait) are the closest approximation: an associated type parameterized by a further type/lifetime/const, landed stable 1.65. Will Crichton's analysis ("Generic associated types encode higher-order functions on types," willcrichton.net) frames GATs as encoding a restricted class of type-level functions, not full HKT: they let a trait carry a type constructor as an associated item, which is precisely what tagless-final needs for the higher-order (function-typed) case, but they do not give quantification over type constructors as a first-class kind the way `forall f. Functor f => ...` does in Haskell.
- **Object safety.** A trait with an associated type is dyn-compatible only if the type is either fixed via `dyn Trait<Assoc = X>` or the trait opts the associated type out via `where Self: Sized`. A trait with a GAT is a harder case: per rust-lang/rust#81823 ("Trait objects do not work with generic associated types") and the Learning Rust dyn-safety guide, GATs "are too new to support type erasing," and making a `dyn Trait` work would require naming the GAT's parameter on the trait-object type itself (a feature that does not exist for GATs as of the sources checked; the escape hatch is the same `where Self: Sized` bound that opts the GAT out of dyn-compatibility entirely, since Rust 1.72). Net effect: a tagless-final `ExprSym`/`Lang`-shaped trait with `type Repr<T>` **cannot** be boxed as `dyn ExprSym` at all if the GAT stays dyn-usable, and if it opts out via `Self: Sized` then it simply cannot be used behind `dyn` under any encoding. This forces monomorphization (a concrete type parameter `E: ExprSym` on every generic function) as the only mechanism to select an interpreter; there is no type-erased "list of interpreters" without a hand-written non-generic wrapper enum.
- **Recursion / conditionals force boxing.** In the non-GAT `(Not) Finally Tagless` example, `if_` takes `Box<dyn Fn() -> Self>` branches, i.e. lazy branches must be heap-boxed closures because Rust cannot express "an unevaluated `Self`-producing thunk" any other way generically over `Self: Sized` unknown size. The GAT posts sidestep this for their own examples (arithmetic/booleans have no laziness requirement) but the higher-order `lam`/`app` case requires `F: Fn(Self::Repr<A>) -> Self::Repr<B>`, a generic closure type per call site; storing a HETEROGENEOUS collection of such terms (e.g. an AST built at runtime from parsing, mixing many different closure/generic-function types under one interpreter) is exactly the case the rust-users forum thread ("Traits vs Haskell typeclasses for tagless final programming") calls out: Haskell can box+erase via typeclass dictionaries; Rust must monomorphize, so "it is very awkward to implement" a runtime-constructed term generic over the interpreter. A forum participant notes the missing piece explicitly: expressing "a function generic over any `E: Exp`" as a value would need higher-ranked trait bounds like `impl for<E: Exp> Fn(&E) -> E::Repr`, which "we don't have yet" (HRTB exists for lifetimes, not for trait-bounded type parameters in this position).
- **Storing a term generic over the interpreter.** Because the interpreter choice is a type parameter, not a value, a term built once and replayed against N interpreters is a generic function (`fn term<E: ExprSym>() -> E::Repr`) called once per interpreter, monomorphized N times at compile time. It is NOT a value you construct once at runtime and hand to different interpreters without recompilation; there is no dynamic "term object" independent of `E` unless `E` is erased behind `dyn`, which (per the object-safety point above) tagless-final's own associated-type/GAT shape forbids for anything past the simplest first-order case.
- **Type inference gaps.** The getcode.substack.com post reports Rust "can't figure out `E` from an `E::Repr`" in several positions, requiring an auxiliary reverse-lookup trait (`HasExprSym`) as boilerplate, and that this workaround itself "does not scale well" to the higher-order case (the post's own April-2026-noted edit).

## 3. Initial vs. final in Rust, and the arena-plus-generic-fold hybrid

The classic tradeoff (enum + `match` per backend vs. trait + associated type per backend) plays out differently in Rust than in ML/Haskell specifically because of the points above:

- **Initial (enum + match).** One concrete `enum Expr { Lit(i32), Neg(Box<Expr>), Add(Box<Expr>, Box<Expr>) }` plus one `fn eval(e: &Expr) -> i32`, one `fn print(e: &Expr) -> String`, etc. per backend. Adding a backend is free (new function, no trait). Adding a new node variant requires editing every existing function (the classic "expression problem" axis Wadler's original framing addresses, cited by name in the getcode post). The value is a first-class runtime value: constructible once, stored, pattern-matched, sent across a channel, cloned, walked by an arbitrary number of backends without recompiling anything, trivially heterogeneous (one `Vec<Expr>` holds any mix of node shapes).
- **Final (trait + assoc type/GAT).** Adding a backend is free (new impl). Adding a node is a breaking trait change (every impl must add the method), i.e. the expression problem's OTHER axis is now the free one. The Rust-specific cost is everything in section 2: no dyn-erased term, forced monomorphization per interpreter, boxed closures for laziness, and (the getcode/dev.to authors' own words) it never reaches the ergonomics of the Haskell/OCaml original because Rust has neither HKT nor the ability to defer/erase a term's interpreter choice to runtime the way a typeclass dictionary does.
- **The hybrid (parse into a concrete arena-held representation once, then fold that representation generically into any interpreter).** No source found in this pass documents this EXACT hybrid under the tagless-final name. What the search surfaced instead, independently, is the mainstream real-world answer to the same underlying problem, and it is the initial encoding, not tagless-final: parse once into a concrete, arena/index-based tree (an enum of node kinds referencing children by index, per Adrian Sampson's "Flattening ASTs" post, section 5 below), then write ordinary generic functions (`fn eval<...>(pool: &Pool, id: NodeId) -> T`) or a `Visitor`/fold trait that walks that fixed representation. This is architecturally cleaner than true final encoding for a shared-arena, multi-backend, no-alloc design because: (a) the arena's node enum is the ONE thing multiple visitor implementations read, so a "backend" is just a function or trait impl closing over the shared arena types, no interpreter-generic type parameter threading through the IR itself; (b) it sidesteps every object-safety/GAT/boxing problem in section 2, because the IR is concrete data, not a family of `Self`-generic operations; (c) it is exactly the "flat array + parent/child by index" shape rustc's HIR, rust-analyzer, oxc, and swc all independently converged on (section 5). Cloudflare's wirefilter engine (section on dispatch below) reports the SAME resolution from a different angle: they moved AWAY from a `trait Expr { fn execute(...) }`-per-node object-dispatch shape and TOWARD compiling the AST once into a `Box<dyn Fn(&ExecutionContext) -> bool>` closure tree, decoupling "the shape that gets built" from "the shape that gets executed" but still choosing concrete, boxed, non-generic values over a tagless-final trait family; they explicitly did not use interning/arenas and reported the extra heap boxing cost as "negligible" in their measurements.
- **Real Rust projects reporting tagless-final for a shipping compiler/interpreter:** none surfaced in this search. Every "real Rust project" found in this pass (rustc, rust-analyzer, oxc, swc, cranelift, wirefilter) uses initial encoding (a concrete enum/struct tree, arena- or index-held), not tagless-final. The tagless-final material that exists is blog-post-scale demonstrations (getcode.substack.com, the two Louy2 dev.to posts) and forum discussion (users.rust-lang.org), not a production compiler. That itself is a data point: no located evidence of tagless-final shipping as the term-representation strategy of a real Rust compiler/interpreter at the scale this research targets.

## 4. `cranelift-entity`: exact API

Source: docs.rs/cranelift-entity (latest), `EntityRef`/`PrimaryMap`/`SecondaryMap`/`EntityList`/`ListPool`/`PackedOption`/`entity_impl!`.

```rust
pub trait EntityRef: Copy + Eq {
    fn new(_: usize) -> Self;
    fn index(self) -> usize;
}
```
"A type wrapping a small integer index should implement `EntityRef` so it can be used as the key of a `SecondaryMap` or `SparseMap`." The `entity_impl!` macro (three call shapes: `entity_impl!($entity)`, `entity_impl!($entity, $display_prefix)`, and a fully custom conversion-function form) generates the boilerplate impl of `EntityRef` (+ `Display`, + `ReservedValue`) for a newtype wrapping `u32`, so a compiler defines `pub struct Inst(u32); entity_impl!(Inst, "inst");` and gets a usable dense key type for free.

```rust
pub struct PrimaryMap<K, V> where K: EntityRef { /* private fields */ }
```
"A primary mapping `K -> V` allocating dense entity references." `push(v) -> K` appends and returns a freshly minted key; `get(k) -> Option<&V>`; `iter()` walks keys+values. Deliberately does NOT implement `Deref`/`DerefMut` to a slice, so the only access path is through the typed key, not a raw index.

```rust
pub struct SecondaryMap<K, V> where K: EntityRef, V: Clone { /* private fields */ }
```
"A mapping `K -> V` for densely indexed entity references." `get(k) -> Option<&V>`, `get_mut(k) -> Option<&mut V>`, `resize(n)` grows the map by filling with `V::default()` (constructors require `V: Default`). A `SecondaryMap` is how a second, independent property gets attached to keys already minted by a `PrimaryMap`, without touching the primary storage.

```rust
pub struct PackedOption<T: ReservedValue>(/* private fields */);
```
"Packed representation of `Option<T>`. This is a wrapper around a `T`, using `T::reserved_value` to represent `None`." (The full `ReservedValue` trait's method signature was not directly quotable from the fetched page, but its role is unambiguous from this description and from `entity_impl!`'s stated behavior: it reserves one sentinel bit-pattern of the underlying integer index to mean "absent," so `Option<EntityKey>` costs zero extra bytes over the bare key, unlike `Option<T>`'s usual tag byte.)

**`EntityList<T: EntityRef + ReservedValue>` + `ListPool<T: EntityRef + ReservedValue>`, the direct answer to "N children, no `Vec`":**

```rust
pub struct EntityList<T: EntityRef + ReservedValue> { /* private fields */ }
pub struct ListPool<T: EntityRef + ReservedValue> { /* private fields */ }
```

- `EntityList::new() -> Self` — an empty list, footprint **4 bytes** (one `u32`) versus **24 bytes** for `Vec<T>` (ptr+len+cap on a 64-bit target). The 4 bytes are "only a 32-bit index into the pool's memory vector, pointing to the first element" of the list's storage; the list itself owns no memory.
- `EntityList::from_slice(slice: &[T], pool: &mut ListPool<T>) -> Self`, `push(&mut self, element: T, pool: &mut ListPool<T>) -> usize`, `extend<I>(&mut self, elements: I, pool: &mut ListPool<T>)`, `as_slice<'a>(&self, pool: &'a ListPool<T>) -> &'a [T]`. Every mutating/reading operation takes the `&ListPool<T>`/`&mut ListPool<T>` explicitly, because the list is only a handle; the actual elements live in the pool's backing store.
- Internally each list occupies "three contiguous parts: 1. the number of elements, 2. the list elements, 3. excess capacity elements," with the total block size kept "a power of two" so the pool can reuse freed blocks by size class rather than doing a raw bump allocation per push (this is closer to a size-class free-list allocator than a pure arena).
- `ListPool::new()`, `with_capacity(len: usize)`, `capacity(&self) -> usize` ("somewhat higher than the total length of lists that can be stored without reallocating"), and the load-bearing operation `clear(&mut self)`: **"Clear the pool, forgetting about all lists that use it. This invalidates any existing entity lists that used this pool."** Memory is retained (not returned to the OS) for reuse; the pool is explicitly a LIFO-style bulk allocator ("after building up a larger data structure with many list references, the whole thing can be discarded quickly by clearing the pool").

This is precisely the "a node has N children with no `Vec`" mechanism: the node struct stores an `EntityList<ChildRef>` (4 bytes) instead of a `Vec<ChildRef>` (24 bytes + a separate heap allocation per node), and every node in the whole IR shares ONE `ListPool` for its child lists, so there is exactly one backing allocation (grown geometrically) for the entire IR's variable-arity edges, not one per node.

## 5. Other arena crates compared

| Crate | Index type | Deletion | Heterogeneous? | `no_std` |
|---|---|---|---|---|
| `la-arena` (rust-analyzer's own, `la_arena`) | `Idx<T>` (a `u32`-backed, phantom-typed index) | no | no, one `T` per `Arena<T>` | not stated as a focus in the fetched docs; crate is a "thin wrapper for `Vec` with `u32` indices" |
| `id-arena` | `Id<T>` (via the `ArenaBehavior` trait, default `DefaultArenaBehavior<T>`) | no | no, one `T` per `Arena<T, A>` | **yes, explicitly**: "supports `no_std` environments that have access to the `alloc` crate" |
| `typed-arena` | plain `&T`/`&mut T` references (no index type; arena owns the allocations and returns references tied to the arena's lifetime) | no (all freed together on arena drop) | no | **yes since 1.4.0**, per the crate's own announcement ("now with `#![no_std]` support") |
| `slotmap` | generational `Key` (index + generation counter) | **yes**, generational (a removed slot's key stays invalid forever even if the slot is reused) | no, one `V` per `SlotMap<K, V>` | yes, but **requires `alloc`** (disable the default `std` feature) |
| `bumpalo` | raw references (`&T`, or `bumpalo::boxed::Box`, `bumpalo::collections::Vec`) | n/a (bump-only, whole-arena reset) | yes, mixed types in one `Bump` | **`no_std` by default**, depends only on `core`+`alloc`; the `std` feature is opt-in for extra trait impls; a matching `allocator_api` feature lets it back std collections directly |

`la-arena`'s own doc framing (from the crate's crates.io page, quoted in the search results): "quite helpful when just a thin wrapper for `Vec` with `u32` indices is needed" — i.e. it is deliberately the minimal case, no generational safety, no deletion, one arena per node type. `Arena<T>`'s field is literally `pub(crate) data: Vec<T>`, `alloc(value: T) -> Idx<T>` pushes and returns the index, `iter()` yields `(Idx<T>, &T)`, and `Index<Idx<T>>` is implemented so `arena[idx]` reads like array indexing. rust-analyzer uses one `Arena<T>` per node kind throughout its `hir`/`base-db` crates (e.g. `Arena<Function>`, `Arena<Struct>`), not one arena for a mixed-kind tree; heterogeneity across node kinds is handled by having several typed arenas side by side plus explicit `Idx<T>`-typed cross-references, not by one arena holding an enum of everything. `indexed_arena` (a newer, separate crate) is noted as offering the same `Arena<T>` API shape as `la-arena` but with an abstracted internal index representation.

`id-arena`'s doc framing: "you allocate objects and get an identifier for that object back, not a reference," useful "for constructing mutable graph data structures," explicitly single-type ("if you need an arena of objects with heterogeneous types, consider another crate"), no deletion "which makes its implementation simple and allocation fast." Confirmed `no_std`+`alloc` support is stated directly on its docs.rs page, which `la-arena`'s fetched page did not state either way.

`rustc_arena` (the compiler's own, in-tree, NOT published to crates.io as a general-purpose dependency): `TypedArena<T>` (single type, drops its contents when the arena drops, implemented as chunk pointer + end pointer + a `Vec` of chunks) and `DroplessArena` (holds MANY different `Copy`/non-`Drop` types in one arena, used for the interner below), composed per-compiler-phase via a `declare_arena!` macro that builds one `Arena` struct with a `dropless: DroplessArena` field plus one `TypedArena<T>` field per registered type (e.g. `layout`, `mir`, `typeck_results`).

## 6. String interning options and `no_std` status

- **rustc's own `Symbol`/`Interner`** (`compiler/rustc_span/src/symbol.rs`): "an 'interner' is a data structure that associates values with `usize` tags and allows bidirectional lookup." Internally `Symbol` is implemented purely as an index; ALL operations (hashing, equality, ordering) act on that index, never on the string bytes. Backing storage is `rustc_arena::DroplessArena` (the string bytes are arena-allocated once, and the interner map + a reverse `Vec` give O(1) name-to-index and index-to-name). Thread-local access goes through `SessionGlobals`/`scoped_thread_local` to avoid a fully global mutable singleton. This is an in-tree, compiler-only design, not a general-purpose crate; the interner's viability rests entirely on the arena, which is `alloc`-based, not `no_std`-without-`alloc`.
- **matklad's "Fast and Simple Rust Interner"** (the didactic minimal version rustc's design descends from): the naive baseline is
  ```rust
  pub struct Interner {
      map: HashMap<String, u32>,
      vec: Vec<String>,
  }
  impl Interner {
      pub fn intern(&mut self, name: &str) -> u32 {
          if let Some(&idx) = self.map.get(name) { return idx; }
          let idx = self.map.len() as u32;
          self.map.insert(name.to_owned(), idx);
          self.vec.push(name.to_owned());
          idx
      }
      pub fn lookup(&self, idx: u32) -> &str { self.vec[idx as usize].as_str() }
  }
  ```
  explicitly flagged by the author as wasteful (every interned string is stored as TWO separate heap allocations, one owned by the `HashMap` key and one by the `Vec`). The post's optimized version consolidates strings into one growable buffer (unsafe, to keep stable `&str` slices into a buffer that may itself reallocate by keeping old, full buffers around rather than moving them) — still fundamentally `alloc`-based, no `no_std`-without-`alloc` claim made.
- **`lasso`**: `Rodeo` (single-threaded) and `ThreadedRodeo` (concurrent, needs the `multi-threaded` feature) both do "O(1) internment and resolution." Default dependency is only `hashbrown` (used for its unstable `raw_entry` API not yet stable in `std`'s own `HashMap`). **The `no-std` feature enables `no_std` + `alloc` support for BOTH `Rodeo` and `ThreadedRodeo`** — stated directly on its docs.rs page. So `lasso` is a `no_std`-with-`alloc` interner, not a heap-free one.
- **`ustr`**: a `Ustr` is "a lightweight handle representing a static, immutable entry in a global string cache," FFI-friendly (each string carries a trailing NUL so it can be passed to C without a `CString` conversion), with the precomputed hash stored alongside the string for fast hashing/comparison. Trade-off stated directly: "no strings are ever freed... they only leak one copy for each unique string the program encounters." No `no_std` claim was found in the fetched material; `ustr` is presented purely as a `std`, global-cache design.
- **`stringleton`** (surfaced by search, not deeply fetched): claimed to "work in `no_std` environments but fundamentally requires allocator support to maintain the global symbol registry" — i.e. `no_std`-with-`alloc`, matching the pattern of every other interner found. The `alloc` feature (default-on) supports building symbols from `String`.
- **No interner found in this pass claims to work with zero `alloc`**, i.e. against a purely fixed-capacity, pre-reserved buffer with no growth. Every interner surveyed (rustc's, matklad's, `lasso`, `ustr`, `stringleton`) either directly depends on `alloc` (`HashMap`/`Vec`/`String`/`hashbrown`) or an arena that itself depends on `alloc`. A heap-free interner would have to be hand-built on top of a fixed-capacity map (e.g. `heapless::FnvIndexMap` or similar, sized at compile time) and a fixed-capacity byte buffer; nothing surfaced in this research is a ready-made "interner with zero allocation, ever" crate.

## 7. `no_std` fixed-capacity collections

- **`heapless`**: "core principle... backed by static memory allocation, with capacity specified via [a] type parameter `N`, allowing instantiation on the stack, in static variables, or even in the heap [if desired]." `Vec<T, N>` (`N` is a const generic capacity), `String<N>`, `Deque`, `BinaryHeap`, `IndexMap`/`IndexSet`. Operations like `push` are stated as genuinely constant-time (not amortized), and the crate explicitly markets "no risk of an uncatchable Out Of Memory condition" because capacity is fixed and checked. This is the actual zero-heap answer for "a collection with a hard compile-time cap," as distinct from every arena/interner crate above which is `alloc`-based even when it also compiles under `no_std`.
- **`bumpalo`**: confirmed **`no_std` by default**, "depends only on the `alloc` and `core` crates." An optional `std` feature adds trait impls for `std`-only types; an `allocator_api` feature lets `Bump` implement the (still-unstable in stable Rust, but present in this ecosystem) `Allocator` trait so `std` collection types can be backed by a `Bump` arena directly. Bumpalo is explicitly a **growing** bump arena (it will request more OS/heap memory as needed, in chunks) rather than a hard fixed-capacity reservation; a genuinely fixed, no-growth version is `fixed-bump` ("depends only on `core` and `alloc`, so it can be used in `no_std` environments that support `alloc`") or `bump-into` (sourced from a caller-provided `&mut [u8]` slice, no allocator dependency at all, positioned explicitly for embedded use).
- **`typed-arena`**: added `#![no_std]` support in its 1.4.0 release per the crate's own announcement post; still `alloc`-based (it grows its backing chunks), single-type, no deletion, returns plain `&T`/`&mut T` tied to the arena's own lifetime rather than an index.
- **`slotmap`**: "supports `no_std` environments but requires the `alloc` crate" (disable the default `std` feature to get this mode). Generational keys, so removed slots stay permanently invalid even under slot reuse; this is the crate to reach for if the IR needs mutation/removal, which none of the other arenas here support.

## 8. Flat tree representations in real Rust compilers

- **rustc HIR** (`rustc_hir`): each "owner" (an item, trait item, or impl item) stores its HIR nodes in `OwnerNodes`, whose `nodes` field is `IndexVec<ItemLocalId, ParentedNode<'tcx>>` — a dense, zero-based, `Vec`-backed index (`ItemLocalId`s "occupy a dense range of integers starting at zero, so a mapping can be implemented by a `Vec` instead of a tree or hash map"), where `ParentedNode<'tcx>` pairs each node with its parent's `ItemLocalId`. This is exactly "flat array + parent pointer by index," scoped per-owner (not one global array for the whole crate) so that incremental recompilation can invalidate one owner's nodes without touching others.
- **rust-analyzer / rowan (`la_arena`'s sibling crate for syntax trees, not the same as the generic `Arena<T>`)**: the "red-green tree" split, originating in Roslyn and adopted by Swift's libsyntax before rust-analyzer's `rowan`. The **green tree** holds position-independent, immutable, structurally-shared syntax nodes (identical subtrees can literally share the same allocation, since green nodes carry no absolute offsets). The **red tree** is a thin secondary layer computed on demand: each red node carries a reference to its parent plus an absolute start offset, so `offset + child-widths-from-the-green-node` gives every node's source range, and parent links let the red tree be walked/re-walked for siblings without the green tree needing any of that information baked in. `cstree` is a documented fork of `rowan` from the same authors, and Biome (the JS/TS toolchain, successor to the Rome project) ships "a custom Rowan definition" of its own (`biome_rowan`), i.e. at least three independent production Rust tools (rust-analyzer, cstree's consumers, Biome) converged on the identical red/green split.
- **oxc** (a from-scratch Rust JS/TS parser+compiler): uses `bumpalo` directly as its AST backing arena. Its own docs report "changing to a memory arena for the AST resulted [in] around 20% performance improvement," attribute part of the win to construction-order-matches-traversal-order linear memory access, and note the AST is built with `bumpalo::collections::Vec`/`bumpalo::boxed::Box` so every AST node carries the arena's lifetime; none of oxc's AST node types implement `Drop`, and this is enforced at compile time (allocating a `Drop` type into the arena is a compile error). oxc's own backlog (`oxc-project/backlog#197`, `oxc-project/oxc#11145`) has an open issue about replacing `bumpalo` with a custom allocator to remove per-allocation alignment overhead, since every node type happens to already be 8-byte aligned (each contains a `Vec`/`Box`/`Atom`/`&str`).
- **swc**: also arena-based (`swc_allocator::allocators::Arena`), and the search material notes its AST is built from ordinary `Statement`/`Expression` enums with "a dozen... enum variants" each containing `Box`es and `Vec`s prior to arena adoption; swc's `Arena` documents an aliasing discipline ("allocations from the arena must not be performed while iterators are alive"; "when reading chunk data, callers must ensure no mutable references exist to previously allocated data") that is the caller's responsibility, not statically enforced.
- **Adrian Sampson's "Flattening ASTs" post** (independent research/teaching post, not a production compiler, but a clean minimal statement of the same pattern all four production tools converge on): before/after is `enum Expr { Binary(BinOp, Box<Expr>, Box<Expr>), Literal(i64) }` becoming `enum Expr { Binary(BinOp, ExprRef, ExprRef), Literal(i64) }` with `struct ExprRef(u32)` indexing into a `struct ExprPool(Vec<Expr>)`. Reported wins: cache locality from contiguity, 50% smaller references (`u32` vs. a 64-bit pointer), bump-style allocation with no per-node `malloc`, and (his measured number) whole-arena deallocation accounting for "38% of runtime" in the pointer-based baseline, disappearing entirely once the whole pool drops as one `Vec`. Overall measured speedup: "2.4x." The stated cost: the "extra-flat" fully-linear-scan interpreter variant assumes children are allocated before parents (bottom-up construction order), which works naturally for a bottom-up parser but constrains how generic, order-independent traversal can be written; a demand-driven (only touch what's needed) traversal is harder to express on the fully flattened, order-dependent variant than on the simpler indexed-but-still-recursive `ExprRef`-based version.
- **Cross-cutting comparison of representations**: rustc HIR and Adrian Sampson's post use flat-array-plus-**parent**-index (good for "walk up," awkward for "get my Nth child" without also storing child ranges or a separate children-list). `cranelift-entity`'s `EntityList`/`ListPool` and oxc/swc's bumpalo-backed `Vec`s use **child-list-by-handle** (good for "get my children," parent tracking is a separate concern the IR must add if needed). rowan's red/green split stores children inline in the green node (structurally, not by separate index) and computes parent/offset information lazily in the red layer, which is the one design here that gives O(1) "get my children" AND O(1) "get my parent" AND structural sharing of identical subtrees, at the cost of maintaining two parallel tree representations instead of one.

## 9. The minimal no-heap toolkit, concretely

For "an arena-held tree with variable-arity nodes, interned strings, and no heap allocation after an initial fixed reservation" the pieces that were confirmed to actually deliver a NO-`alloc` (not merely `no_std`-with-`alloc`) answer are narrower than the full crate list above:

- **Fixed-capacity node storage**: `heapless::Vec<Node, N>` (a hard compile-time `N`), or a hand-rolled equivalent, gives the "arena" itself with zero allocation after the one static/stack reservation. None of `id-arena`/`la-arena`/`typed-arena`/`slotmap` qualify here; all of them grow a `Vec` (i.e. depend on `alloc`), even the ones correctly labeled `no_std`.
- **Variable-arity children without per-node `Vec`**: `cranelift-entity`'s `EntityList<T>` + `ListPool<T>` pattern is the right SHAPE (a 4-byte handle into one shared pool instead of a 24-byte owned `Vec` per node), but `cranelift-entity` itself is built on `PrimaryMap`/`ListPool` internals that were not confirmed `no_std`-without-`alloc` in the fetched pages (the crate's docs describe the pool as backed by "the pool's memory vector," i.e. a `Vec`). Reproducing the SAME index-into-a-shared-pool idea over a `heapless`-style fixed buffer instead of a growing `Vec` would be the no-heap version of this exact mechanism; no ready-made crate combining "`EntityList`-style handles" with "zero-growth backing storage" was found.
- **Interning**: no interner surveyed is `alloc`-free. The closest available building block is `lasso`'s `no-std` feature (`no_std`+`alloc`) or a hand-built map from `heapless::FnvIndexMap`/similar fixed-capacity map plus a fixed-capacity byte arena for the string bytes themselves (unconfirmed as an existing crate; would need to be assembled).
- **Bump allocation**: `bump-into` is the one bump allocator found that takes memory from a caller-supplied `&mut [u8]` with NO allocator dependency at all, i.e. genuinely heap-free; `bumpalo` and `fixed-bump` are `no_std` but both still depend on `alloc` (they request their backing chunks from the global allocator, they merely avoid `std`-specific APIs).

**Summary of the no_std-claims-but-needs-alloc pattern**: every crate in sections 5-7 that advertises `no_std` support (`id-arena`, `typed-arena`, `slotmap`, `lasso`, `stringleton`, `bumpalo`, `fixed-bump`) does so in the weaker sense of "compiles without `std`, still calls into `alloc`" (a global allocator must exist). Only `heapless` and `bump-into` were confirmed to avoid `alloc` entirely, by fixing capacity at compile time or by taking a caller-owned byte slice respectively. Nothing surveyed combines fixed-capacity node storage, `EntityList`-style variable-arity child handles, AND interning in one ready-made crate; assembling that combination from `heapless` (or an equivalent hand-rolled fixed arena) plus a hand-adapted `EntityList`/`ListPool`-shaped indirection plus a hand-built fixed-capacity interner is, per this research, what "no heap after one fixed reservation" concretely requires in Rust today.

## Sources

- [Efficient, Extensible, Expressive: Typed Tagless Final Interpreters in Rust](https://getcode.substack.com/p/efficient-extensible-expressive-typed)
- [(Ok) Tagless Final in Rust](https://dev.to/louy2/ok-tagless-final-in-rust-1aff)
- [(Not) Finally Tagless in Rust](https://dev.to/louy2/not-finally-tagless-in-rust-4kfc)
- [Traits vs Haskell typeclasses (for tagless final programming) — Rust forum](https://users.rust-lang.org/t/traits-vs-haskell-typeclasses-for-tagless-final-programming/33579)
- [Generic associated types encode higher-order functions on types — Will Crichton](https://willcrichton.net/notes/gats-are-hofs/)
- [Generic associated types to be stable in Rust 1.65 — Rust Blog](https://blog.rust-lang.org/2022/10/28/gats-stabilization/)
- [Trait objects do not work with generic associated types — rust-lang/rust#81823](https://github.com/rust-lang/rust/issues/81823)
- [dyn compatibility (object safety) — Learning Rust](https://quinedot.github.io/rust-learning/dyn-safety.html)
- [cranelift_entity — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/)
- [EntityList — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/struct.EntityList.html)
- [ListPool — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/struct.ListPool.html)
- [EntityRef — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/trait.EntityRef.html)
- [PrimaryMap — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/struct.PrimaryMap.html)
- [SecondaryMap — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/struct.SecondaryMap.html)
- [PackedOption — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/struct.PackedOption.html)
- [entity_impl! macro — docs.rs](https://docs.rs/cranelift-entity/latest/cranelift_entity/macro.entity_impl.html)
- [la_arena::Arena — rust-analyzer docs](https://rust-lang.github.io/rust-analyzer/la_arena/struct.Arena.html)
- [la-arena — crates.io](https://crates.io/crates/la-arena)
- [id_arena::Arena — docs.rs](https://docs.rs/id-arena/latest/id_arena/struct.Arena.html)
- [id-arena — crates.io](https://crates.io/crates/id-arena)
- [typed-arena 1.4.0 no_std announcement — Rust forum](https://users.rust-lang.org/t/announcing-typed-arena-version-1-4-0-now-with-no-std-support/18234)
- [slotmap — docs.rs](https://docs.rs/slotmap/)
- [bumpalo — GitHub](https://github.com/fitzgen/bumpalo)
- [bumpalo — docs.rs](https://docs.rs/bumpalo)
- [fixed-bump — crates.io](https://crates.io/crates/fixed-bump/0.1.2)
- [bump-into — docs.rs](https://docs.rs/bump-into/latest/bump_into/)
- [heapless — docs.rs](https://docs.rs/heapless)
- [Fast and Simple Rust Interner — matklad](https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html)
- [lasso — docs.rs](https://docs.rs/lasso/latest/lasso/)
- [lasso — GitHub](https://github.com/Kixiron/lasso)
- [ustr — GitHub](https://github.com/anderslanglands/ustr)
- [rustc_span/src/symbol.rs — rust-lang/rust](https://github.com/rust-lang/rust/blob/main/compiler/rustc_span/src/symbol.rs)
- [rustc_arena/src/lib.rs — rust-lang/rust](https://github.com/rust-lang/rust/blob/master/compiler/rustc_arena/src/lib.rs)
- [Memory management in rustc — rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/memory.html)
- [The HIR (High-level IR) — rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/hir.html)
- [OwnerNodes — nightly rustc docs](https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/hir/struct.OwnerNodes.html)
- [Red Green Syntax Trees - an Overview](https://willspeak.me/2021/11/24/red-green-syntax-trees-an-overview.html)
- [rowan — GitHub, rust-analyzer](https://github.com/rust-analyzer/rowan)
- [Architecture — rust-analyzer book](https://rust-analyzer.github.io/book/contributing/architecture.html)
- [cstree — docs.rs](https://docs.rs/cstree)
- [biome_rowan — docs.rs](https://docs.rs/biome_rowan)
- [AST — Abstract Syntax Tree, oxc docs](https://oxc.rs/docs/learn/parser_in_rust/ast)
- [Pursuit of Performance on Building a JavaScript Compiler — oxc](https://oxc.rs/docs/learn/performance)
- [Custom arena allocator — oxc-project/oxc#11145](https://github.com/oxc-project/oxc/issues/11145)
- [swc_allocator::allocators::Arena — rustdoc.swc.rs](https://rustdoc.swc.rs/swc_allocator/allocators/struct.Arena.html)
- [Flattening ASTs (and Other Compiler Data Structures) — Adrian Sampson](https://www.cs.cornell.edu/~asampson/blog/flattening.html)
- [Building fast interpreters in Rust — Cloudflare blog](https://blog.cloudflare.com/building-fast-interpreters-in-rust/)
