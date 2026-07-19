# Finding the compact form (Inigo Quilez)

**Date:** 2026-07-19. Every count and every source claim below was executed, not inherited.

## The judgement in one line

Nine forms, a cons-list inclusion check, twenty-one constructors, two caches and a Dhall hash pipeline are
five enumerations of things that are all attributes of one table and lanes of one 16-byte summary computed
by the traversal the design already pays for, and the enumerated presentation is what made each of them look
like separate machinery needing separate repairs.

## Where I build on Carmack, and where I part from him

His measurement re-prices the round and I take it as given: 2676 rows (I recount exactly 2676 across the
fourteen namespace directories, plus 10 in `vocab.toml`, so Aaltonen's total verifies and Giesen's 2443
does not), 3148 registry-value references of which **3147 are exactly two segments and zero contain a
paren**, 831 distinct. That is a static address table, not an expression population.

Where I part: he concludes from smallness that most mechanisms should be deleted. Deletion is the right
answer for the incremental machinery and the wrong instinct to generalise, because the brief's ceiling test
is real and several of these mechanisms are cheap. The better move on four of the five is not deletion but
**finding the closed form**, after which the mechanism costs a word and the ceiling goes up rather than
down. His event-stream call I adopt and sharpen; his memo-table deletion I replace with something that
costs one `u32` and keeps the door open.

## The reductions

### 1. The nine forms are three node kinds and one table

Current: `Lit, Ref, Let, Apply, Project, Select, Iter, Seq, Lambda`, a nine-arm enum every fold matches on
(`09_shape.md:54-65`).

Four collapse by standard equalities the design half-uses already. `Let x = e in b` is
`Apply(Lambda([x], b), [e])`. `Iter` is `Apply(map, [source, Lambda([t], body)])`, and the design **already
committed to that spelling** for inline iteration over `blocked_by` (Giesen accepts the value-operation fix,
`04_giesen_synthesis.md:50-52`), so it carries two iteration mechanisms and admits it at `09_shape.md:70-73`.
`Seq` is not semantics at all: it is the n-ary flattening of the content monoid's binary join, a
representation choice, which is exactly why Carmack's stream concatenation replaces it without loss.
`Project(b, n)` is `Apply(sel_n, [b])` where the selector's type carries the namespace-to-row versus
row-to-field rule the design already put in a type rule.

`Select` resists, but only against strict `Apply`. Give each callee a **strictness mask**, one bit per
parameter, and `Select` is `if` with bits `0b110`. `Lambda` is a callee with a fully lazy mask.

Compact form: **node = `{ tag: Lit | Ref | Apply, payload }`**; `Apply` is `(callee: u16, args: EntityList)`;
every former "form" is a row in an operator table carrying `(arity, strictness mask, family bit, stage,
allowed-children mask)`. Nine variants become three tags plus a static table, dispatch becomes a dense
indexed jump rather than a nine-arm match (not slower; denser), and adding a form stops touching the core.

What is lost, stated honestly: a `Preserve` target emitting a Lua `for` must recover the loop from
`Apply(map, [_, Lambda])` rather than seeing `Iter`. That recovery is one callee-id compare, because `map`
is a known row. Nothing is lost; the pattern-match just moves into the renderer that cares. What genuinely
degrades is arity errors, which become table lookups instead of type errors on the constructor.

### 2. The family check is `doc & !tgt == 0`

Current: `AccessSet` cons-lists, `Contains`, `ContainsAll`, plus Giesen's monotone two-level check with
`Checked<T>` carrying an enum of proof routes (`08_decisions.md:64-102`, `04_giesen_synthesis.md:397-416`).

A family set is a set of small tags. The closed form of set inclusion over small tags is a bitmask AND. So:
family set is `const MASK: u64`, inclusion is one expression, and the failure diagnostic is
`doc & !tgt` whose set bits **name exactly the offending families** for free, which the cons-list version
needs a separate mechanism (Giesen's P8 reflection) to produce.

For statically known terms the proof stays compile-time without a trait at all:

```rust
fn emit<D: Doc, T: Target>(d: D) -> Out {
    const { assert!(D::MASK & !T::MASK == 0) };   // post-monomorphisation compile error
    ...
}
```

For parsed documents it is the same expression on a runtime operand, feeding the same `Checked<T>`
typestate with its one constructor. **One operation, both paths.** Giesen's two-level check then is not two
checks: the fast path and the exact path differ only in which traversal produced the operand (all nodes
versus reached nodes). One accumulator, two traversals, no dual-route evidence enum.

The cost side is not a wash, it is lopsided. `Contains` is `#[marker]`
(`hilavitkutin-api/src/access.rs:34`), so this mechanism drags `marker_trait_attr` (a WATCH feature with a
documented history of overlap rough edges) into the extractable no-heap crates, and its own shipped
diagnostic already warns that resolution overflows and asks consumers to raise `recursion_limit` to 1024
(`access.rs:37`). The mask needs no unstable feature and no recursion. Aaltonen's compile-time worry and
Giesen's monomorphisation-axis correction both evaporate rather than needing a bench.

Lost: cons-lists carry type identity, so `Contains<S>` can gate a method. A `u64` cannot, and the const
block reports at monomorphisation rather than at definition, so the diagnostic is worse and fires at the use
site. Ceiling: 64 families, extending to `[u64; N]` at N ANDs. Two families exist today.

### 3. The document algebra is a table, so 21-versus-35 is not a question

The design already took Djot's uniform-attributes decision (`09_shape.md:88-90`) and then did not follow it
through. If attributes are uniform, a constructor is a **tag on a container**, and the difference between
`Para`, `Quote`, `Div`, `Emph`, `Strong` and `Span` is entirely their row: block/inline bit,
allowed-children mask, family bit. `Heading`'s level and `Table`'s column spec are attributes, and
attributes are uniform.

So constructors are rows in the same table as operators, and the count is not designed, it is authored. A
renderer's totality obligation becomes "handle every row whose family bit is in my declared mask", which is
checkable by iterating the table rather than by relying on match exhaustiveness. `Math` arrives as a row
plus arms in the targets that declare it; targets that do not declare it refuse documents using it, which
is decision 2 working exactly as specified. Giesen's Q6 and its red catalogue tests answer a question that
should not have existed.

On tree versus stream I side with Carmack, with one addition he owes: a tree gives structural
well-formedness for free and a stream does not. State the invariant or it is a hidden cost. It is cheap:
one pass, a depth counter and a small kind stack, checking balanced begin/end and each child's kind against
its parent's allowed-children mask, both read off the same table.

### 4. The two caches, the two keys and the fingerprint are one hash and one counter

Current: CallHash, ReadSet, ResultKey, plus a free-variable environment fingerprint on the memo key
(`04_giesen_synthesis.md:361-395`).

The environment fingerprint exists **only because hash-consing shares open terms**. Its closed form is
already known: key the memo by the node's *closed* form, meaning free variables substituted by their bound
ids at the point the memo is taken. For the dominant case that is `hash(green_id, self_id)`, which is what
P4 computes, stated as one value instead of as free-variable summaries plus a fingerprint protocol.

The ReadSet is deletable outright, for a reason stronger than Carmack's cost argument. Within one
generation run the registry is **immutable**. So a read-set fingerprint is constant across the entire run
and carries zero information; the only thing it distinguishes is runs. Replace it with a `u32` registry
generation counter bumped at load. Memo key becomes `(closed hash, generation)`, which inside a run is just
the hash.

Three mechanisms and a plumbing requirement threaded through the evaluator collapse to one hash and one
integer. The ceiling goes *up*: when incremental buys in, the counter becomes per-row and the dependency
edges come from the schema, because with typed reference fields the read set is statically known from the
parse. No evaluator instrumentation is ever needed, so P3's provenance survives at zero runtime cost.

### 5. Variation seeding is one hash, and the proposed one has the bug it was designed to avoid

Dhall normalizes, alpha-normalizes, CBOR-encodes and SHA-256s because its hashes cross machines and
versions as content addresses, and because it hashes arbitrary lambda terms where binder names must not
count. Here the hash never leaves the process and the term is a fragment id plus resolved arguments, which
contain no binders. Alpha-normalization has nothing to normalize and canonical encoding has nothing to
canonicalize that an ordered tuple does not.

Compact: `seed = hash(fragment_slug, arg_slug_0, ..., arg_slug_n)`, then `variants[seed % n]`.

**Load-bearing correction.** Do not hash interned ids. `ArenaInterner::arena_intern` returns
encounter-order ids (`hilavitkutin-str/src/interner.rs:16`) and `Str` is a 28-bit id plus an origin bit
(`handle.rs:28-30`), so ids move when the corpus is reordered. Giesen's CallHash is specified over
"nested references resolved to typed ids" (`04_giesen_synthesis.md:210`). That is the document-path failure
of `05_correction_and_aim.md:89-94` reintroduced one layer down: adding rows above another row in a TOML
file re-rolls phrasings corpus-wide, silently, exactly the thing decision 4 exists to prevent. Hash the
**slug bytes**. Then it is stable under reordering, reformatting and renumbering, and re-rolls only on
rename, which is a semantic change that should re-roll.

## What is irreducible, and why it earned its size

**`Ref` and the scope chain.** `Ref` is the one form that consults the environment; nothing folds it away.
Typst's borrowed `StyleChain` (`09_shape.md:174-181`) is already the compact form: outward-folding, no
merge, no allocation.

**Typed reference fields** (`10_syntax_correction.md`). Not machinery. It is the deletion of a
representation error, and it makes the reference graph schema-known, which is what lets both the read set
and cycle detection become static properties rather than runtime traps. The shipped resolver currently
discovers the graph by parsing values and guards cycles at runtime (`resolve.rs:705-724`); that inverts.

**The target model.** A target is a mask, a policy and a total renderer. Three fields. Already compact.

**Total renderers.** Irreducible and correctly sized: the alternative is Pandoc's silent drop.

## The unification

Yes, and it is concrete. Reference resolution, family checking, staging, memoization and variation are not
five mechanisms. They are **five lanes of one summary, computed bottom-up by the constructor the design
already pays for**, because a hash-consing constructor must already visit children and hash.

```
summary = { fam: u64, hash: u64, free: u32, stage: u8 }        // 24 bytes, 16 if hash and free narrow
```

Each lane is a monoid whose unit and combine operator come from the node's operator-table row:

- `fam` combines by OR with the row's family bit. Root value versus the target's const answers the
  inclusion check, in both the parse-mask and staged-mask readings.
- `hash` combines by mix with the row's callee id. Root value is the variation seed and the memo key.
- `free` combines by OR minus the row's binders. Closes the memo key; the free-variable summary P4 needed.
- `stage` combines by the row's stage under the target policy. Decision 1's staging and its declared
  predicates are a check on this lane.

One traversal, one 24-byte value per node, and the root's summary answers four questions that the round
currently treats as four subsystems with four repairs. Registry rows join the same arena, because
`10_syntax_correction.md` establishes that a prose field *is* a document fragment, so resolution is the same
fold with the same summary and cycle detection is the DAG check on the same edges.

The design's spine was stated as "a target declares, everything else is a proof obligation against that
declaration" (`08_decisions.md:157-160`). The sharper statement: **every static property of a document is a
monoid over the operator table, and a target is a set of bounds on those monoids.**

## What panel one and two got wrong

- **Giesen's row count.** 2676, verified by direct count per namespace. The file that re-verified every
  claim introduced the round's only wrong count. Carmack already caught this; I confirm it independently.
- **Giesen's ResultKey.** Read sets carry no information within an immutable run. The mechanism was sized
  for a mutable-input incremental system this is not.
- **Giesen's CallHash over resolved ids** reintroduces the renumbering fragility decision 4 exists to
  prevent. Nobody in either panel checked what an interner id is stable under.
- **The panel's shared move**, which Carmack named at the expression level and which is broader than he
  said: five separate enumerations were accepted as enumerations. Nobody asked whether any of them was the
  n-ary spelling of one operation with a table.
- **Carmack's memo deletion** is right on cost and gives up a ceiling for nothing. One `u32` keeps it.

## Open provocations for the panellists after me

1. **Kill or confirm the operator table.** Find one thing a nine-arm enum gives that a three-tag node plus a
   const table does not. I claim only arity diagnostics, and that is worth a table lookup.
2. **Price the `#[marker]` dependency.** If the cons-list survives my reduction, someone must own dragging a
   WATCH unstable feature with a documented `recursion_limit` overflow into the extractable crates.
3. **Test the slug hash against a real reorder.** Move `task/band_0.toml`'s rows and confirm zero phrasing
   churn. That is a ten-minute test the round has already been burned by once.
4. **Size the summary.** 24 bytes per node against 2676 rows plus roughly 10K expression nodes is under a
   megabyte. If that is wrong, the whole unification is wrong, and it is arithmetic, not opinion.
