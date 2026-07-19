Typst, mechanical detail from the `typst/typst` Rust source. Primary sources only.

## What it is, in three sentences

Typst is a markup-and-scripting language whose compiler is a pure-Rust pipeline: parse (`typst-syntax`) to an
AST, evaluate (`typst-eval`) into a tree of type-erased `Content` values, *realize* (`typst-realize`) that
tree by repeatedly applying show/set rules under a style chain until only well-known "base" elements remain,
then *layout* (`typst-layout`) into a `Frame` tree (position/size/paint IR) that per-target backends
(`typst-render`, `typst-svg`, `typst-pdf`, `typst-html`) consume to produce raster, SVG, PDF, or an HTML DOM.
Content whose value depends on the finished document (counters, references, outlines) is handled by
re-running the pipeline up to five times, using the compiler's own incremental-memoization engine (`comemo`)
to detect when introspectable state stops changing and to make re-runs cheap. Everything is heap-based:
content nodes are individually heap-allocated and manually refcounted, style lists are `EcoVec`s,
realization runs inside a `bumpalo` arena.

## The `Content` representation

`content/mod.rs`: `Content` is `#[repr(transparent)] pub struct Content(raw::RawContent)`. The real work is
in `content/raw.rs`. `RawContent` is a hand-rolled, type-erased `Arc`:

```rust
pub struct RawContent {
    ptr: NonNull<Header>,  // fat-pointer-by-hand: Header is the common prefix of Inner<E>
    elem: Element,         // Element = &'static ContentVtable; identifies which E this is
    span: Span,
}
#[repr(C)]
struct Inner<E> { header: Header, data: E }
struct Header { refs: AtomicUsize, meta: Meta, hash: HashLock }
```

One `Box::into_raw` allocation per node; `clone()` bumps `refs` (relaxed ordering, exactly `Arc`'s scheme),
`drop()` decrements and only deallocates at refcount 0 (release + acquire fence, mirroring `Arc`). The doc
comment states the intent directly: "implement a fat pointer setup similar to `Arc<Inner<dyn Trait>>`, but in
a manual way, allowing us to have a custom [vtable]." Mutation goes through `make_unique`, explicit
clone-on-write: if `refs > 1`, clone before returning `&mut`. So `Content` is single-struct-with-erased-
element, refcounted, cheap to clone (one atomic increment), heap-allocated on construction; there is no
enum-of-variants layer, the element type lives entirely behind the `elem: Element` vtable pointer plus an
`unsafe fn data::<E>()` cast. `Packed<T>` (`content/packed.rs`) is a `#[repr(transparent)]` newtype pairing
`Content` with `PhantomData<T>` so a caller who already checked `content.is::<T>()` gets a statically-typed
`Deref`/`DerefMut` view with no second allocation.

## The element system and `#[elem]`

`crates/typst-macros/src/elem.rs` parses a struct annotated `#[elem(...)]` and, per field attribute
(`#[required]`, `#[positional]`, `#[fold]`, `#[ghost]`, `#[synthesized]`, `#[internal]`, `#[parse(...)]`,
plain), generates: the real struct (required fields stored bare, everything else as `Settable<Self, I>`, an
`Option`-like slot keyed by a per-field const index `I`), an inherent `impl` with a `new(required_fields...)`
constructor and `with_<field>` builders, a `Field<Self, I>` const per field (a zero-sized handle used with
`StyleChain::get`/`set`), a `NativeElement` impl carrying a `'static ContentVtable`, one `FieldVtable` per
field (`content/field.rs` distinguishes `RequiredFieldData`, `SettableFieldData`, `SettablePropertyData`,
`SynthesizedFieldData`, `ExternalFieldData`), and `Construct`/`Set` impls unless opted out. A required field
becomes a plain constructor argument; a non-required field becomes a `Settable<Self, I>` slot also exposed as
a `Field<Self, I>` const, what `set` rules and `StyleChain::get`/`.set()` operate on. That one macro
invocation makes a field simultaneously "a constructor arg at construction" and "a settable property via a
`set` rule further up the tree", without hand-writing either path. Every std-library element and every
user-defined Rust element (`typst-html`'s `HtmlElem`, `FrameElem`) goes through this identical macro.

## The content monoid and `for`

`Content::sequence(iter)` (mod.rs) collapses an iterator to `Self::empty()` if empty, the single item if len
1, else `SequenceElem::new(vec)`. `SequenceElem` (`#[elem(Debug, Repr)]`, one `#[required] pub children:
Vec<Content>`) is the concatenation/identity element; `Content::empty()` is a process-wide `singleton!` of an
empty `SequenceElem`. `Add for Content` special-cases both operands already being `SequenceElem`s (extend
the left's `children` in place, no allocation beyond `Vec::extend`), one side being a sequence
(push/insert), or neither (wrap both in a new two-element `SequenceElem`). `AddAssign` is `*self =
mem::take(self) + rhs`; `impl Sum for Content` is `Self::sequence(iter)` directly, so `#for` and any
`.sum()` funnel through the same flattening constructor. `sequence_recursive_for_each` flattens nested
sequences on the read side. The monoid: identity = empty `SequenceElem`, op = `Content::sequence`/`Add`,
in-place fast path when either operand is already a sequence, cost = a `Vec<Content>` heap allocation per
sequence-shaped subtree.

## The `Value` enum

`crates/typst-library/src/foundations/value.rs`, exact variant list:

```rust
pub enum Value {
    None, Auto, Bool(bool), Int(i64), Float(f64), Length(Length), Angle(Angle),
    Ratio(Ratio), Relative(Rel<Length>), Fraction(Fr), Color(Color), Gradient(Gradient),
    Tiling(Tiling), Symbol(Symbol), Version(Version), Str(Str), Bytes(Bytes),
    Label(Label), Datetime(Datetime), Decimal(Decimal), Duration(Duration),
    Content(Content), Styles(Styles), Array(Array), Dict(Dict), Func(Func),
    Args(Args), Type(Type), Module(Module), Dyn(Dynamic),
}
```

28 variants. `Content` and `Styles` are first-class `Value` variants alongside numbers, strings, and
functions; `IntoValue for T: NativeElement` is `Value::Content(self.pack())`, so any native element is
directly a scripting value. `Dyn(Dynamic)` is the escape hatch for opaque values otherwise unrepresentable.

## Styles, set rules, show rules, `StyleChain`

`Styles` (`foundations/styles.rs`) is `pub struct Styles(EcoVec<LazyHash<Style>>)`, and `Style` is:

```rust
pub enum Style {
    Property(Property),      // from a set rule or constructor
    Recipe(Recipe),           // a show rule
    Revocation(RecipeIndex),  // disables one show recipe (regex recipes only)
}
```

A set rule (from `#[elem]`'s `Set` impl) parses `Args` into a `Styles` list of `Property` entries. A show rule is a `Recipe`: a `Selector` plus a `Transformation` (`Content(Content) | Func(Func) |
Style(Styles)`; replace with content, call a function, or apply set-rule styles as a show-set).
`StyleChain<'a>` is documented as "similar to a linked list": `struct StyleChain<'a> { head: &'a
[LazyHash<Style>], tail: Option<&'a Self> }`. Its doc comment states why it is a chain, not a merged map: "A
style chain allows to combine properties from multiple style lists in a element hierarchy in a
non-allocating way. Rather than eagerly merging the lists, each access walks the hierarchy from the innermost
to the outermost map, trying to find a match and then folding it with matches further up the chain." Each
nested scope pushes one borrowed link (zero-copy) and a lookup walks outward until it matches, optionally
folding (`E::FOLD: Option<fn(T, T) -> T>`) with values further up (how nested `set text(size: ..)` calls
combine, instead of the innermost always winning). Show rules apply during realization (`typst-realize`),
not evaluation, described there as "the process of recursively applying styling and, in particular, show
rules to produce well-known elements"; it runs inside a `bumpalo::Bump` arena, producing `Pair`/`FragmentKind`
values the layout crate consumes.

## Introspection and the convergence loop

`introspection/introspector.rs` defines `#[comemo::track] pub trait Introspector` (`query`, `query_first`,
`query_label`, `query_count_before`, `label_count`, ...), implemented per target (`typst_layout`'s paged
introspector, `typst-html`'s `HtmlIntrospector`). The driving loop, in `crates/typst/src/lib.rs::compile_impl`,
holds `history: ArrayVec<T, { MAX_ITERS - 1 }>` and, each pass, builds a fresh `comemo::Constraint`, runs the
engine with the previous document's introspector tracked against it, calls `T::create` to relayout, then
checks `constraint.validate(document.introspector())`; true breaks the loop as converged, otherwise the
document is pushed to history and the loop repeats until `history.is_full()` (five attempts, `MAX_ITERS = 5`
in `introspection/convergence.rs`).

Convergence reuses `comemo`'s own memoization-invalidation machinery: every introspector read goes through a
tracked call recorded against `constraint`; after producing a new document, `constraint.validate` checks
whether every recorded read still returns the same answer against the new introspector. If yes, the document
is stable; comemo's cache means later iterations mostly reuse prior work. Non-convergence after five full
relayouts produces a diagnostic ("document did not converge within five attempts") rather than an infinite
loop or silent wrong answer; `introspection::analyze` distinguishes "comemo thinks it didn't converge but the
observed introspections did" (no warning) from genuine oscillation (warning per query).

## Export targets, including HTML's limits

Crate-per-backend: `typst-render` (raster), `typst-svg`, `typst-pdf`, `typst-html`, all downstream of
`typst-layout`'s `Frame` tree, the one documented IR between realized content and a concrete output.
`Output::create` produces a target-specific document type; for `Paged`/PNG/SVG/PDF that document is built
from `Frame`s (position+size+paint tree), while `typst-html` converts the *realized* element tree straight
into an `HtmlNode` DOM (`dom.rs`, `convert.rs`), bypassing `Frame` except where `html.frame` asks for
laid-out output embedded as inline SVG. `Target` (`foundations/target.rs`) is a plain enum `Paged | Html |
Bundle`, exposed to show rules via `#[func(contextual)] fn target(context) -> Target { context.styles()?
.get(TargetElem::target) }`, where `TargetElem` "exists solely to host the `target` style chain field. It is
never constructed and not visible to users"; `compile_impl` sets it once at the top so `target()` inside a
show rule is a StyleChain lookup, and `html.frame` locally overrides it back to `Paged` for its embedded
sub-layout. `typst-html`'s `html.elem` (`#[elem(name = "elem")] pub struct HtmlElem { tag: HtmlTag, #[fold]
attrs: HtmlAttrs, body: Option<Content>, ... }`) is a user-constructible native element wrapping an
arbitrary tag/attrs/body; the module doc states the incompleteness directly: "This divergence in the
formats' intents means that Typst cannot simply produce perfect HTML for your existing Typst documents. It
cannot always know what the best semantic HTML representation of your content is." Concretely: no CSS
emission, no HTML-fragment export (always a standalone document), gated behind `--features html` as
experimental (tracking issue `typst/typst#5512`).

## Mechanisms worth naming, with tradeoffs

- **Hand-rolled type-erased `Arc`, not `Arc<dyn Trait>`.** Gains a custom vtable (per-field vtables,
  capability lookup, drop/clone/hash/eq chosen per element) a generic `dyn Trait` cannot express; costs
  unsafe fat-pointer construction and hand-written atomic refcounting a library `Arc` gives for free.
- **Clone-on-write via `make_unique`.** Cheap sharing until first mutation; costs a full re-clone once a
  mutable path is taken on a shared node, same tradeoff as `Arc::make_mut`.
- **`StyleChain` as a borrowed linked list, not a merged map.** Zero allocation per nested scope; costs
  O(depth) per lookup, worse for deep nesting or hot reads, fine for shallow-to-moderate nesting.
- **Convergence reuses memoization-invalidation (`comemo::Constraint`)**, not a dedicated dependency graph.
  Piggybacks existing machinery, at the cost of accuracy being bounded by comemo's tracking discipline (a
  query bypassing tracked accessors can converge while comemo reports otherwise).
- **Realization runs inside a `bumpalo` arena.** Bulk-frees garbage in one drop instead of per-node refcount
  teardown; costs a requirement that realized data not outlive the arena.
- **`target()` as a StyleChain ghost element, not a global.** Lets export target vary *within one
  compilation* (paged, but `paged` again inside `html.frame`) by nesting a style scope, no global mutation.

## Heap and `Arc` dependence

Every mechanism above is heap-based. `RawContent` is one `Box` allocation per node with manual atomic
refcounting modeled on `Arc`; `Styles` is `EcoVec<LazyHash<Style>>` (heap-backed, refcounted, from `ecow`);
`SequenceElem` holds `Vec<Content>`; realization runs inside a `bumpalo::Bump` arena; `Str`/`EcoString` are
heap-backed refcounted strings. There is no stack-only path anywhere; "cheap clone" means cheap relative to
deep-copying a tree, via refcount-sharing a heap allocation, not by avoiding heap allocation. Removing heap
use would require replacing three load-bearing
pieces at once: the type-erased representation (needs *some* indirection for one arity-one `Content` type to
hold arbitrarily-shaped data), the refcounted sharing that makes `+`/`#for` cheap, and the arena-based
realization pass. None is incidental; together they are how cheap cloning of an erased tree node happens.

## Sources

- `crates/typst-library/src/foundations/content/mod.rs`, `raw.rs`, `element.rs`, `packed.rs`, `vtable.rs`,
  `field.rs` — <https://github.com/typst/typst/tree/main/crates/typst-library/src/foundations/content>
- `crates/typst-library/src/foundations/value.rs` — <https://github.com/typst/typst/blob/main/crates/typst-library/src/foundations/value.rs>
- `crates/typst-library/src/foundations/styles.rs` — <https://github.com/typst/typst/blob/main/crates/typst-library/src/foundations/styles.rs>
- `crates/typst-library/src/foundations/target.rs` — <https://github.com/typst/typst/blob/main/crates/typst-library/src/foundations/target.rs>
- `crates/typst-macros/src/elem.rs` — <https://github.com/typst/typst/blob/main/crates/typst-macros/src/elem.rs>
- `crates/typst-realize/src/lib.rs` — <https://github.com/typst/typst/blob/main/crates/typst-realize/src/lib.rs>
- `crates/typst-library/src/introspection/introspector.rs`, `convergence.rs` — <https://github.com/typst/typst/tree/main/crates/typst-library/src/introspection>
- `crates/typst/src/lib.rs` (`compile_impl`) — <https://github.com/typst/typst/blob/main/crates/typst/src/lib.rs>
- `crates/typst-html/src/lib.rs`, `dom.rs`, `convert.rs` — <https://github.com/typst/typst/tree/main/crates/typst-html/src>
- `docs/content/reference/export/html.typ` — <https://github.com/typst/typst/blob/main/docs/content/reference/export/html.typ>
- HTML export tracking issue — <https://github.com/typst/typst/issues/5512>
