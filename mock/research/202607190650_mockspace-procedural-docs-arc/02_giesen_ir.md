# One IR Over Two Syntaxes: an IR Construction Read

**Date:** 2026-07-19
**Reads:** `00_seed.md`, `01_baseline.md`, vehje source at `mock/crates/`.

**Verdict:** sound only if "one IR" means a shared expression core plus per-language node families
composed at the type level; `Ir<V>` as a swap of whole vocabularies answers the cost question, leaves
the design question standing, and quietly redefines the "one IR" claim without noticing.

## What is sound

- Building the language instead of growing verbs. "A template is a program whose output is text" is
  correct, and the incremental path was a slow-motion language anyway.
- Dropping fragment-source mapping (baseline §2), and for a stronger reason than the doc gives:
  `Span` already carries `FileId` (vehje-ir/src/span.rs:35-39), and FileId-to-block is a renderer-side
  table. The abstraction they declined already exists in its minimal form.
- Retracting the const-eval interpreter claim. Verified: `Strict4::validate` is an empty body
  (vehje-typecheck/src/strict4.rs:28-33). Constant folding over literals is a fold, not a tree walker.
- The prior-art rejections are operationally right: MLIR is C++ over LLVM with a heap, Racket assumes
  a GC, `rowan` allocates. The niche claim (extensible IR, `no_std`, no alloc, source-to-source) holds.
- Nothing in the current IR needs defending: nine coarse variants, no expression vocabulary at all
  (vehje-ir/src/nodes.rs:17-27). The design space is genuinely open, which is the right time for this
  argument.

## Findings

**1. `Ir<V>` resolves cost, not coherence, and as stated it breaks the "one IR" claim.** If
`VehjeVocab` and `QueryVocab` are two instantiations, a pass written against `Ir<VehjeVocab>` does not
run on `Ir<QueryVocab>`. The sharing that pays (one resolve, one checker, one interpreter) then
requires every shared pass to be generic over `V` with bounds naming the node families it handles.
That is the actual hard design, and neither document mentions it. Alternative: make `Vocabulary`
compositional, not flat. A fixed expression core (literal, call, projection, conditional, binding,
block) both front ends lower into; per-language extensions (vehje's item/fn/module layer, the query
layer's where/select/aggregate) as type-level components; shared passes bounded on the core,
per-language passes on core plus extension. This is MLIR's dialect structure and the nanopass lesson
(passes are the product, vocabularies are cheap) done in Rust generics. It is also the honest form of
"one IR": one core with two extensions, not one union and not two strangers.

**2. `Project { base, name }` is right as the surface lowering and wrong as the checked form.**
Namespace-to-table is static name binding: resolves at check, fails at check, evaluates to nothing.
Row-to-field is a runtime load per row. Table-to-column is a third meaning (vectorised), and
`!enforced_by` inside `where` is a fourth: a field reference under an implicit row binding. One node
before check is fine and standard. But if the checked IR keeps one `Project` whose meaning lives in a
type side table, every consumer (evaluator, backend) re-derives the discrimination. The doc's own
words ("resolves at check time to a concrete access") name the fix: make the concrete access a node
(StaticRef, FieldLoad, ColumnProject), not an annotation. Check rewrites; downstream never guesses.

**3. "There is no host" is true of the type system and false of ownership; keep both truths.**
No-alloc forces the registry bytes to live outside the interpreter arena; a table value is a typed
view into that storage. The Lua-frame rejection is right about typing (no opaque handles) and must not
be read as an ownership claim, or the baseline's "alias rather than copy" degrades to hand-waving. And
layer the value domain the same way as finding 1: if `Value` is one enum with Table and Column
variants, vehje's const folding (in a build that only emits source) carries the relational half in its
type. Scalar core, relational extension, same composition.

**4. "One type system" (baseline §4) is asserted, not argued.** vehje's obligations are nominal:
coherence, orphan rule, bind-target shape (stubs at vehje-typecheck/src/coherence.rs, orphan_rule.rs,
bind_target_shape.rs). The query side needs schema types and the projection rule, which is row
typing. One checker driver and one `Ty` representation can host both; "one piece of work"
underestimates. Budget it as one framework carrying two rule sets.

**5. No-alloc under iteration is mostly answerable now.** Output is emitted in document order, so the
interpreter renders through a caller-provided `impl ByteEmitter` (already the stack idiom;
vehje-codegen/src/registry.rs:17-18 imports it) and needs no text buffer. The residual pressure is
ordering: sort and aggregate need O(rows) index scratch, bounded by registry size, arena-provided.
Decide the sink shape now; only aggregate scratch stays open.

**6. Delete `AstNodeKind`, do not extend it.** nodes.rs:12-14 promises tail extension with stable
discriminants. That is a compatibility promise to zero consumers, contrary to the pre-1.0 no-shims
rule; the vocabulary redesign replaces the enum outright.

## What the documents got wrong

- Reading MLIR as precedent rather than mechanics. The adoptable part is not the C++: it is the
  uniform operation shape (operands, results, attributes) plus trait-based pass applicability, which
  is exactly how one infrastructure serves vocabularies it has never seen without unions. Finding 1 is
  that shape.
- The baseline flags the codegen `dyn` registry as sitting oddly against the no-dyn discipline. Its
  own doc comment records it as the single sanctioned exception per R3 design
  (vehje-codegen/src/registry.rs:1-10), mirroring the validator registry. Not drift; leave it.
- The seed's interpreter-sharing claim, already retracted in the baseline, correctly.

## Open questions I cannot settle alone

- Whether hilavitkutin's `Column<T>` fits a registry of heterogeneous record schemas with string-heavy
  fields, as opposed to the homogeneous records-of-T it was built for. Separability was verified;
  fitness was not.
- Whether the implicit row binding inside `where(...)` is a real binder in the IR (a lambda) or sugar
  resolved at lower. This decides whether the shared core needs closures, the single largest
  complexity fork in the whole vocabulary.
- Baseline question 3 (does the ambition stay honest) is editorial policy, not IR construction; no
  position.
