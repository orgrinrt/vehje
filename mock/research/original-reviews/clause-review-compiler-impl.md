# Clause Compiler — Implementation Feasibility Review

**Date:** 2026-04-16  
**Reviewer:** compiler implementation perspective  
**Scope:** Full design doc (`2026-04-16-mod-language-design-draft.md`, §1–31),
supported by `rust-traits-research.md`, `cwtools-formalization-review.md`,
`typed-dsl-prior-art.md`, `2026-04-16-clausewitz-grammar.md`, and the existing
Python compiler codebase.

---

## 1. Executive Summary

**Overall verdict: YELLOW — ambitious but feasible, with three red-zone risks.**

The design is coherent, well-researched, and clearly informed by real prior art.
The language surface is thoughtfully constrained (no `dyn`, no lifetimes, no HKT,
monomorphize-everything) in ways that directly reduce implementation complexity.
The decision to emit tokens rather than text, and to build on the existing pass
framework rather than replace it, is architecturally sound.

Three areas are red-zone from an implementation standpoint:

**Red 1 — Transpile scope-stack tracking.** The mapping from Rust-ish bodies to
Clausewitz is not compositional for all combinations in §24.4. Nested closures
(`for x in iter { match ... { Some(y) => for z in ... { } } }`) require a scope
stack that tracks both the Clause type context and the in-flight Clausewitz emit
context simultaneously. The design says "scope-stack tracking is the novel core
challenge" without specifying the representation. Getting this wrong means the
transpiler either generates structurally invalid Clausewitz or generates valid
Clausewitz with wrong scope semantics. This is the hardest single piece.

**Red 2 — Type checker coherence under patch chains.** The design says "multiple
un-marked impls → compile error" but doesn't specify how the coherence checker
handles the patch-chain ordering problem: when 4,500 patches in the megapatch
crate each carry `#[patch(target = "...")]`, the checker must verify (a) no two
patches overlap on the same `(Trait, Type)` pair without explicit consolidation,
and (b) the `super.foo()` shadow-rename chain is total — every level resolves.
Naively, this is O(n²) in the number of patches. The right algorithm (constraint
graph with topological resolution) is not described.

**Red 3 — stellaris-vanilla-spec ingest fidelity.** The spec crate is the
foundation everything else type-checks against. If the ingest pipeline produces
wrong or incomplete declarations (wrong scope, wrong parameter names, missing
effects), every type error the compiler emits downstream is a lie. The design
correctly identifies this as multi-source-with-cross-validation, but the
resolution procedure for conflicts between `trigger_docs`, `cwtools-config`, and
observed vanilla usage is underspecified. This is not primarily a hard algorithm
problem — it is a data-quality and maintenance process problem that will consume
ongoing effort.

Everything else is yellow or green. The lexer, parser, module resolver, and
codegen are well-specified with clear prior art. The manifest system is
engineering-intensive but the scope is bounded. The CLI is the easiest part.

Estimated effort to Phase 2 acceptance (one subsystem in Clause, byte-identical
output): **8–14 weeks of focused development**, at the upper end if the
transpiler scope-stack proves harder than expected. The design's own estimate of
6–11 weeks is reasonable for an expert implementer who has already read all the
prior art; add 30–50% buffer for a first-timer.

---

## 2. Per-M-Task Implementation Assessment

### M2 — Clause Lexer

**Difficulty: LOW. Estimated effort: 1–2 weeks.**

The token taxonomy in §29 and the G2 Clausewitz grammar doc together give an
unusually complete specification. The Clause lexer is distinct from the G2
Clausewitz lexer: it handles Rust-ish syntax (`struct`, `trait`, `impl`, `fn`,
`:`, `::`, `?`, `|>`, etc.) not Clausewitz tokens. The two lexers share no
grammar. This is a clean boundary.

Gotchas in a Python implementation:

- **String interning costs.** Python's `str` is already interned for short
  literal strings, but `id_text.lower()` for every IDENT token creates a new
  object. Replicate the cwtools `StringTokens` pattern: a `dict[str, InternedId]`
  intern table keyed on lowercased text, value is an int. Each token carries an
  `InternedId` not a `str`. At 100k tokens per large file, this matters.
- **Encoding.** `.cw` files are author-controlled; declare UTF-8 only and fail
  loudly on non-UTF-8 input. No Windows-1252 fallback needed (unlike the G2
  Clausewitz lexer which must handle vanilla's encoding). Saves the auto-detect
  complexity.
- **The `:` overloading (§30 open question 2).** In lexer terms this is trivial:
  `:` is always emitted as `COLON`. The parser assigns meaning. The overloading
  problem is entirely in the parser, not the lexer.
- **Position packing.** The G2 grammar doc's bit-packed `Span` is worth copying
  for the Clause lexer too. At 4,500 patch files the token count will be
  substantial. Use `(file_id: u16, start: u32, end: u32)` packed into an int64.
  Python's `struct.pack` is fine for this.
- **`super` as both keyword and method prefix.** `super.foo()` means "call the
  base impl" (§23.1); `super::Thing` is a module path prefix (§20.5). Both are
  valid Clause. The lexer should emit a single `KW_SUPER` token in both cases;
  the parser distinguishes by what follows (`.IDENT(` vs `::IDENT`).

No blocking issues. This is the safest starting point.

---

### M3 — Clause Parser + AST

**Difficulty: MEDIUM. Estimated effort: 2–3 weeks.**

Recursive-descent is the right choice and all the grammar is effectively
specified inline across §§6–17. The `:` character overloading deserves careful
attention.

**The `:` parsing problem in detail.** The design says `:` means "bind target"
on structs and "supertrait" on traits. Both appear in the same syntactic position
(after the name, before the body). Context at parse time:

```
struct Foo: Bar { ... }   -- bind
trait Foo: Bar { ... }    -- supertrait
```

The parser already knows it is parsing a `struct` or `trait` declaration (keyword
was seen), so disambiguating `:` is a matter of remembering the current
declaration kind. This is straightforward one-token-of-context, not backtracking.
No ambiguity. **The `:` concern is overstated at the parser level.**

The genuinely tricky grammar points:

1. **Method bodies contain raw Clausewitz fragments.** An `impl` block's method
   bodies can embed Clausewitz directly (`any_pop = { limit = { ... } }`). The
   parser needs to handle arbitrary Clausewitz key-value syntax inside fn bodies
   without treating it as Clause syntax. The boundary is: inside a `fn { }` body,
   switch to a "body parser" that applies the §18 rewrite table rather than the
   full Clause grammar. This is not ambiguous, but it is a significant parser
   context switch. Design the parser as two modes: Clause declaration mode and
   body expression mode.

2. **Lambda-like scope-opening syntax.** `name(|p| { p.foo })` (§24.4) is a
   closure-like lambda that opens a Clausewitz scope block. The `|p|` is a
   parameter pattern. This is different from Rust closures (`|x| x + 1`) in that
   the body is a Clausewitz block, not an expression. The parser needs to handle
   `IDENT(|IDENT| BLOCK)` as a scope-opening call. Not ambiguous; just an
   unusual production. Add it to the method call expression grammar explicitly.

3. **`event` declarations inside `impl` blocks.** An `event Foo for Bar { }` can
   appear at top level or inside an `impl` block (§15). Inside an `impl`, `for
   Self` has special semantics. The parser must accept `event` as a valid item
   inside an `impl` block body, not only at module level. This means the impl
   body parser needs to handle both `fn` and `event` item declarations.

4. **`..super` struct-spread syntax.** Inside a `#[patch]` trait_definition body,
   `..super` spreads base fields (§23.2). This borrows from Rust's struct update
   syntax (`..other_struct`). The parser must handle `..super` as a special
   expression inside data definition bodies, and `..super.field` for nested
   sub-struct updates. This is a non-trivial production that doesn't exist in
   standard Rust. Define it explicitly in the grammar.

5. **Error recovery.** The design calls for "partial AST with error nodes; don't
   fail-fast" (§29/M3). This is the right call but doubles the parser development
   effort. A parser without recovery is 2 weeks. A parser with useful error
   recovery (resync to next `}` or next statement boundary) is 3–4 weeks.
   Prioritize recovery early — it pays dividends in the type checker and transpiler
   error quality.

**AST node representation.** Use dataclasses with `frozen=True`. Every node
carries a `SourceSpan`. Use a tagged union for expression variants (Python's
`dataclasses` + `typing.Union` or a dedicated enum-of-dataclasses pattern). Avoid
generic `dict`-based ASTs — the type checker needs structured access, not string
keys.

---

### M4 — Module Resolver

**Difficulty: MEDIUM. Estimated effort: 1–2 weeks.**

The `mod foo;` chain walking is Rust-standard. The interesting part is
`expect`/`actual` cross-crate resolution.

**`expect`/`actual` satisfaction detection.** The design says "runtime
load-ordering verifies the actual impl exists." For compile-time purposes, the
module resolver must determine whether an `actual` in crate B satisfies an
`expect` in crate A. The matching rule is:

- The `expect` declares a name and a type shape: `expect fleet_command:
  FleetCommand`. The `actual` provides `actual impl FleetCommand for Country`.
- The resolver must match: does `actual` in crate B produce something of type
  `FleetCommand`? This requires name resolution across crate boundaries, which
  means the workspace dep graph must be walked.

For workspace members (where all crates are local), this is straightforward: load
all crates' resolver tables, then cross-reference. For external crates (future
mod ecosystem), this requires shipping pre-built resolver indices with each crate.

**Implementation approach.** Two-phase resolution:
1. Phase 1: resolve all within-crate `use` chains, producing a per-crate symbol
   table `{fully_qualified_name → Declaration}`.
2. Phase 2: for each `expect` declaration, search all declared `actual`s in
   dependent crates. Satisfy an `expect E: T` if there exists `actual impl T for
   X` where `X` is the expected binding target. Report unsatisfied expects as
   warnings (not errors — the `#[cfg(mod = "X")]` path may legitimately have no
   actual for the absent mod).

**`pub(in path)` visibility.** This requires the resolver to track module paths
and validate that `pub(in heritage::internal)` only appears within the
`heritage::internal` path. Standard Rust module resolution; copy the algorithm.
Nothing novel.

**Dead module detection.** Walk the `mod` declaration tree from `lib.cw`.
Anything on disk not reachable gets warned. This is a simple file-system vs
declaration-tree diff. Straightforward.

---

### M5 — Type Checker

**Difficulty: HIGH. Estimated effort: 4–8 weeks. This is the long pole.**

This is the most complex task in the M-batch. Five sub-problems with distinct
implementation strategies.

#### M5a — Trait coherence + orphan rule enforcement

The orphan rule check is: for each `impl Trait for Type`, verify that either
`Trait` was declared in the current crate or `Type` was declared in the current
crate. For `#[patch]`-annotated impls, bypass orphan but require the annotation.

Algorithm:
```
for each impl_decl in crate:
  trait_crate = resolver.declaration_crate(impl_decl.trait_ref)
  type_crate = resolver.declaration_crate(impl_decl.type_ref)
  if trait_crate != CURRENT_CRATE and type_crate != CURRENT_CRATE:
    if not impl_decl.has_attr("patch"):
      error("orphan rule violation: ...")
```

This is O(n) in number of impls per crate — cheap. The hard part is building the
full impl universe for the coherence overlap check.

**Overlap prohibition.** Two impls of the same `(Trait, Type)` pair that are
both non-`#[patch]` → compile error. Build a `dict[(trait_id, type_id) →
list[ImplDecl]]`. Any entry with len > 1 and not all marked `#[patch]` → error.
For blanket impls (`impl<T: Foo> Bar for T`), overlap detection requires checking
whether the type parameter bounds can be simultaneously satisfied — this is the
hard coherence case in Rust (rustc uses `coherence.rs` with unification). For
Clause v1, restrict blanket impls to the stdlib; require concrete types in user
impls. This avoids implementing a unification-based overlap checker and gets 95%
of the value.

#### M5b — Patch chain super-call resolution

The chain resolution algorithm: for `super.foo()` inside a `#[patch]`-annotated
impl of `(Trait, Type)`, find the "base" impl.

A patch chain for `(Trait, Type)` is built as follows:
1. Collect all impls of `(Trait, Type)` across the workspace, sorted by their
   `#[patch(target = "...")]` attribute chain.
2. The base is the impl with no `#[patch]` attribute, or the `extern impl` from
   the vanilla spec.
3. Each successive patch's `super.foo()` resolves to the previous level.

The chain is linear by construction: `#[patch]` requires the target to already
exist (otherwise it's an orphan). Cycles are impossible if the target is always
a declared symbol from a different crate or module. Detect the "no base" case
(a patch with no non-patched impl beneath it) as a compile error: "super.foo()
has no base to delegate to."

Shadow-rename codegen: the base impl's auto-generated name (`trait_type_method`)
becomes `trait_type_method__base`, the first patch's becomes the canonical name.
If there are three levels: `...base`, the level-1 patch becomes
`...patch_1__base`, the level-2 patch becomes the canonical. The shadow names
must be unique and stable across builds (derive from the crate+version of each
patch, not from build order). Important: build determinism requires a canonical
ordering of patches when multiple patches from different crates target the same
`(Trait, Type)`. Dependency order (topological sort of `mod.toml` deps) is the
right tiebreaker.

**O(n²) risk for 4,500 patches.** Building the chain for every `(Trait, Type)`
pair in megapatch: if each of 4,500 patches has a `super.foo()`, and the compiler
walks the chain from scratch for each, the worst case is O(n²) in chain length.
Mitigation: build the chain once per `(Trait, Type)` key, cache in a dict. The
number of distinct `(Trait, Type)` pairs is bounded by the number of impls, so
this is O(n log n) with the dict.

#### M5c — Kind inference

The kind inference table (§9.4) is the most tractable part of M5. It is a
tree-walk over the method body AST, collecting "kind votes" from each node:

```python
def infer_kind(body_ast: BodyNode) -> Kind:
    votes = set()
    for node in walk(body_ast):
        if node.type in EFFECT_CONSTRUCTS:    # set_variable, add_modifier, etc.
            votes.add(EFFECT)
        elif node.type in TRIGGER_CONSTRUCTS:  # any_*, count_*, etc.
            votes.add(TRIGGER)
        elif node.type in VALUE_CONSTRUCTS:    # math, if/else with values
            votes.add(VALUE)
    if len(votes) == 1:
        return votes.pop()
    elif votes == {TRIGGER, VALUE}:
        return VALUE  # value blocks can contain trigger tests
    else:
        return AMBIGUOUS  # triggers explicit #[prefer] or error
```

The `EFFECT_CONSTRUCTS` / `TRIGGER_CONSTRUCTS` tables are populated from the
vanilla spec. This is O(n) in body size. The 5% ambiguous cases surface as
`#[prefer(...)]` errors at a precise source location — easy to fix. No deep
algorithm needed.

#### M5d — Generic monomorphization bookkeeping

For Clause v1, full monomorphization is not needed at type-check time — only at
codegen. The type checker needs to:
1. Verify that generic parameters are used consistently (a `T: HasFleets + Owned`
   constraint is satisfied at each call site).
2. Track which concrete monomorphizations exist (e.g., `Modifier<Unity>`,
   `Modifier<Energy>`) for the codegen to emit.

This is the same algorithm as Rust's monomorphization, simplified by the lack of
trait objects and lifetimes. Implementation: a `MonoTable: dict[(type_id,
type_args) → mono_id]` populated during type-checking. Each call site that uses
a generic type records its instantiation. Codegen iterates `MonoTable` to emit
each concrete instance.

**Const generics.** `Array<T, const N: int>` requires the type checker to track
constant values as part of the type. At type-check time, verify that `N` is a
compile-time integer literal or constant reference. At codegen, substitute `N`
inline. This is bounded: const generics in Clause are only integers used for
array/vector capacities. No complex const evaluation needed.

#### M5e — Pattern match exhaustiveness

For `Option<T>` and `Result<T, E>`: two variants each. Exhaustiveness is a
two-arm check: `Some(_)` + `None` = exhaustive, missing either = error. Simple.

For user-defined enums with generic type parameters: more complex. `match val { Ok(x: T) => ..., Err(e: E) => ... }` requires knowing that `T` and `E` are bound in context. The exhaustiveness algorithm:
1. Build a "pattern matrix" from the match arms.
2. For each enum variant, check that at least one arm covers it.
3. For each arm, verify the binding pattern is type-compatible with the variant's
   payload type.

The complexity comes from nested patterns (`Ok(Some(x))`) and wildcard patterns
that cover multiple variants. For Clause v1, the enum variants will primarily be
`Option` and `Result` plus user-defined enums. Restrict nested match patterns in
v1 to depth 1 (no nested `Ok(Some(x))`). This covers all the design's examples
and avoids the O(2^depth) worst case of full pattern matching exhaustiveness
checking.

#### M5 — Effort summary

Kind inference: 1 week. Pattern matching (restricted v1): 1 week. Generic
monomorphization bookkeeping: 1–2 weeks. Coherence + orphan: 1 week. Patch chain
resolution: 1–2 weeks. Total: 5–8 weeks. The type checker is the largest single
task and the one where the design has the most implicit specifications to fill in.

---

### M6 — Transpile Phase

**Difficulty: HIGH. Estimated effort: 3–5 weeks. Contains the single hardest
algorithmic problem in the whole compiler.**

#### The scope-stack representation

The transpiler must simultaneously track:
1. **Clause type context**: what type is the receiver (`self` is a `Bloodline`
   bound to `Registry<Self>`), what are local variable bindings and their types.
2. **Clausewitz emit context**: what scope is currently active in the emitted
   Clausewitz, what indentation level are we at, are we in trigger mode or effect
   mode.

These two contexts are distinct and interact. A `for x in iter` in Clause
requires knowing the Clause type of `iter` (to pick the right `every_*` keyword)
AND knowing the current Clausewitz scope (to know what `every_pop` vs
`every_country` means here).

**Proposed representation:**

```python
@dataclass
class TranspileFrame:
    clause_scope: TypeRef          # the Clause type of `self` in this frame
    cw_scope: CWScope              # the Clausewitz scope (Country, Pop, etc.)
    emit_mode: EmitMode            # EFFECT | TRIGGER | VALUE
    local_bindings: dict[str, TypeRef]   # let/let mut bindings
    is_lambda_frame: bool          # whether we entered via |p| { }

class TranspileStack:
    frames: list[TranspileFrame]
    
    def push(self, frame: TranspileFrame): ...
    def pop(self) -> TranspileFrame: ...
    def current_cw_scope(self) -> CWScope: ...
    def lookup_local(self, name: str) -> TypeRef | None: ...
```

The stack is pushed on every scope-opening construct (`for x in iter { }`,
lambda `|p| { }`, nested method body) and popped on exit. The emit mode is
inherited but can be overridden (an `if` inside an effect body is still in
effect mode; an `any_*` call inside a trigger body switches into trigger sub-mode
for the `limit` block but returns to trigger mode after).

**Composability of rewrite table.** The §24.4 table entries mostly compose
cleanly, but three combinations need special attention:

1. **`match` inside a `for`.** A `for x in iter { match foo { ... } }` emits
   `every_X = { <match emit> }`. The match desugars to an `if/else if/else`
   chain inside the `every_X` block. This is fine — the emit is just nested
   Clausewitz `if` blocks inside the `every_X` body.

2. **`for` inside a `match` arm.** `match foo { Ok(x) => { for y in iter { } } }`.
   This requires: check the Ok flag, enter the if = { limit = { ok flag check } }
   body, then emit the `every_*` loop inside the if body. The scope stack handles
   this correctly if the match arm is emitted as an if body and the for pushes a
   new frame inside it.

3. **`?` operator inside a `for`.** `for x in iter { let result = some_op(x)?; }`.
   The `?` operator emits "if error, set the outer result to Err and return." But
   "return" inside a `for` loop (which emits as `every_*`) means "break out of
   the loop AND set the return variable." Clausewitz has no `break` in `every_*`
   — it runs to completion. This is a **genuine semantic gap**: the `?` operator's
   early-return semantics are not directly expressible inside a `for` loop in
   Clausewitz. Resolution options:
   - Emit a "continue flag" variable that subsequent iterations check at the top
     of the loop body: `if check_variable(continue_flag = 0) { <body> }`. This
     emulates early-return with a continue guard. It generates ugly Clausewitz
     but is correct.
   - Make `?` inside `for` a compile error in v1, with a suggestion to use an
     explicit match arm instead.
   - Option b is cleaner for v1; add the continue-flag workaround later.

The design should explicitly specify that `?` inside `for` is a v1 restriction.

#### Option/Result codegen: sentinel collision

The sentinel strategy (`-1` for `None`) can collide when the inner type has `-1`
as a valid value. Concrete case: a `PopId` newtype wrapping an `int`. If the
game ever uses pop ID 4294967295 (max int - sentinel), sentinel detection fails.

Resolution: the design says "numeric inner (Pop, int, pop_id): sentinel value
(-1)." For ID types, use the flag+value strategy instead of the sentinel, because
IDs are engine-assigned and the engine may theoretically use any non-negative
integer. The decision rule:

- `int` fields that are user-controlled bounded values (indices, counts): sentinel
  is safe if the declared max (e.g., `MAX_SIZE = 100`) is well below the sentinel.
  Validate at the type-checker level: if the field has a declared max `N` and `N
  < 2^31 - 1`, sentinel is safe.
- ID types (`PopId`, `CountryId`, etc.): always use flag+value, never sentinel.
  IDs are engine-controlled; the sentinel value may be a valid engine ID.
- `bool`-returning functions: `None` as "not applicable" needs flag+value (bools
  can't carry a sentinel).

Document this decision in the transpiler. The "numeric inner" category in §12.1
is too broad; it should be "bounded-numeric inner with declared max < sentinel."

#### `super.foo()` dispatch: identifying the base

The transpiler needs to emit the shadow name for `super.foo()`. At transpile time,
it consults the type checker's patch chain table: "for `(Trait, Type, method_name)`
in the current crate's patch, what is the shadow name of the level below?"

This is a lookup, not a computation, as long as M5b built the chain table
correctly. The transpiler asks `PatchChainTable.shadow_name_for(trait_id,
type_id, method_name)` and emits it. Clean separation.

#### Source-span tracking through rewrites

Every emitted token must carry the source span of the `.cw` expression that
produced it. For nested rewrites this means threaded span propagation: when `if
cond { a } else { b }` emits `if = { limit = { <cond> } <a> } else = { <b> }`,
the inner tokens from `<cond>`, `<a>`, `<b>` carry their original spans, and the
synthetic tokens (`if = {`, `limit = {`, etc.) carry the span of the containing
`if` expression.

Python implementation: pass `source_span: SourceSpan` through every emit
function. Synthetic tokens get the enclosing expression's span. Do not track at
character-level granularity for synthetic tokens — line-level is sufficient for
the source map. The `context` string in the `source_map.json` entry
("fn add_member > let new_idx = self.size") is the breadcrumb; the exact span
just needs to be close.

---

### M7 — Codegen

**Difficulty: MEDIUM. Estimated effort: 1–2 weeks.**

#### EmitPlan stream vs. tree

The design proposes a stream of `EmitPlan` records. The alternative (a unified
tree with backend slicing) has one advantage: a tree can be post-processed by
passes after all items are known (e.g., deduplicate common sub-expressions across
the whole output). The stream cannot do cross-item analysis without materializing.

However, for v1 the existing megapatch backend already processes items as a
stream (the pass framework operates item-by-item). The EmitPlan stream plugs
directly into that framework. A tree IR would require the backend to change.

**Recommendation: keep the stream, add a `pre_emit_analysis` pass that
accumulates all EmitPlans before writing.** This pass can detect collisions and
reorder for load-order compliance. The backend's existing pass framework handles
this naturally.

#### Collision detection

The design requires detecting two items with the same `(output_path,
item_name)` pair as a compile error. Data structure:

```python
emitted: dict[tuple[Path, str], SourceSpan] = {}

for plan in emit_plans:
    key = (plan.emit_path, plan.item_name)
    if key in emitted:
        error(f"collision: {key} already emitted from {emitted[key]}, "
              f"now also from {plan.source_span}")
    emitted[key] = plan.source_span
```

O(1) per item with a dict. Total: O(n) in number of emitted items. This is
trivially fast even for 4,500 patches.

#### Token type for G2-compatible emit

The existing G2 token type (from the Clausewitz grammar) is the IR. The Clause
frontend emits `list[Token]` where `Token` is the G2 token type from
`clausewitz/lex.py`. This is not yet implemented (M2 implements the Clause lexer,
which is a different lexer than G2), but the design correctly identifies that the
Clause transpiler's output should be G2-format tokens, not Clause tokens.

The codegen layer bridges: it takes the Clause IR (typed AST + transpiled body
tokens) and emits G2-compatible tokens. The `Token` type for G2 should be
available from the G2 lexer once that lands. Until then, the Clause codegen can
use a thin shim.

#### Source map accuracy

The `source_map.json` promise (`source_line`, `source_col`) can be achieved at
statement-level accuracy (each statement maps to a line range) but not at exact
expression-level accuracy after all the transpile rewrites. The `context` string
field compensates: "fn add_member > self.size += 1" tells the author what they
wrote even if the column is an approximation.

Realistic accuracy estimate: line-accurate for top-level statements, expression-
level for simple constructs, approximated for complex nested rewrites. This is
sufficient for the stated use case (finding where in `.cw` source to look when
the emitted Clausewitz is wrong).

---

### M8 — Manifest Management

**Difficulty: MEDIUM-HIGH. Estimated effort: 2–4 weeks.**

#### Git-diff rename detection

The design says "diff current source vs last-tagged-version source." Python
implementation:

```python
import subprocess

def diff_against_tag(tag: str) -> list[DiffEntry]:
    result = subprocess.run(
        ["git", "diff", f"{tag}...HEAD", "--name-status", "--find-renames=80%"],
        capture_output=True, text=True
    )
    return parse_diff_output(result.stdout)
```

Edge cases:

1. **No prior tag.** First build after tagging v0.1.0 — no previous tag to diff
   against. Resolution: on first build, treat all items as "new" (no aliases to
   create). Document this: tag before shipping, not after.

2. **Uncommitted changes.** The compiler runs with uncommitted changes in the
   workspace. `git diff TAG...HEAD` only covers committed changes; uncommitted
   changes (working tree) are invisible. This is by design per §16.3 ("commit-
   tagged renames produce manifest entries on the next build"). The consequence:
   if an author renames a symbol and doesn't commit, the manifest is never
   updated until they commit. This is acceptable — it's the stated design. But
   document it explicitly so authors don't expect the compiler to track
   uncommitted renames.

3. **Rebased history.** If commits were rebased after tagging, `git diff
   tag...HEAD` may see different changes than the author's working intent. No
   mitigation; this is a standard git rebase hazard. The `#[supersedes]`
   annotation is the escape hatch for cases where git diff gives wrong answers.

4. **Multiple commits between builds.** The diff accumulates all changes since
   the last tag. If the author renamed a symbol and renamed it back in two
   separate commits, the diff sees no change. This is correct behavior — no
   manifest entry needed.

#### "Clean rename" detection

The design describes: "same shape, same body, different name." Algorithm:

```python
def is_clean_rename(old_decl, new_decl) -> bool:
    # Same kind (struct/trait/fn/event)
    if old_decl.kind != new_decl.kind:
        return False
    # Same field structure (for structs)
    if old_decl.kind == "struct":
        return old_decl.field_types == new_decl.field_types
    # Same method signature (for traits)
    if old_decl.kind == "trait":
        return old_decl.method_signatures == new_decl.method_signatures
    # Same body (for functions, approximate)
    if old_decl.kind == "fn":
        return _body_similarity(old_decl.body, new_decl.body) > 0.95
    return False
```

Body similarity: compare the two ASTs structurally, ignoring the declaration
name itself. Use a simple tree-edit-distance comparison or hash-based comparison
(hash the AST with names normalized). 0.95 threshold is arbitrary; false positives
(claiming a rename when it's not) produce unnecessary sham handlers. False
negatives (missing a rename) produce a build warning prompting `#[supersedes]`.
**Err toward false negatives** (emit a warning, let the author annotate) rather
than false positives (silently write a wrong sham). Set the threshold at 1.0
initially (exact structural match ignoring name) and loosen later based on
observed false-negative rate.

#### Sham handler auto-scaffolding

A sham handler for a variable rename is:
```
<crate>_migrate_<old>_to_<new> = {
    if = {
        limit = {
            check_variable = { which = @<old_var> value > -1 }
            NOT = { check_variable = { which = @<new_var> value > -1 } }
        }
        set_variable = { which = @<new_var> value = trigger:@<old_var> }
        clear_variable = @<old_var>
        set_variable = { which = <crate>_sham_done_<old>_to_<new> value = 1 }
    }
}
```

The auto-scaffold generates this for the trivial case (variable rename, same
type). The author can extend the sham body for non-trivial migrations. The sham
file lives in `clause-manifest/shams/` and is referenced from the manifest.

Confidence level of auto-scaffold: **high for variable renames, moderate for
scripted_effect renames, low for structural type changes**. A scripted_effect
rename sham must redirect callers, which requires knowing all callers (a
cross-reference pass). Document that non-trivial shams require manual review
and that the auto-scaffold is a starting point, not a finished migration handler.

#### Cross-crate manifest

The design doesn't explicitly answer: if Heritage renames something that megapatch
depends on, where does the alias entry live?

**Resolution:** the alias lives in the crate that declares the symbol — Heritage's
manifest. Megapatch, which depends on Heritage, sees the alias when it resolves
Heritage's declarations. The megapatch type checker, when it sees a use of the
deprecated name, emits a warning pointing to Heritage's manifest entry. The
megapatch author must update their use of the deprecated name. This follows Rust's
own model: the crate declaring the deprecated item owns the deprecation notice;
users of the item see the warning.

---

### M9 — stellaris-vanilla-spec Ingest

**Difficulty: HIGH (data quality) + MEDIUM (implementation). Estimated effort:
3–6 weeks, ongoing.**

This is the task that will quietly consume the most calendar time, because it is
not primarily a one-time implementation — it needs to be re-run and re-validated
on every Stellaris patch.

#### trigger_docs log parsing

The trigger_docs format is produced by Stellaris' own debug console. It is
internally consistent but its format has changed between major Stellaris versions.
The format from Stellaris 3.x differs from 4.x (scope enumeration, parameter
documentation, added effect categories). Risk: **medium**. The format is not
documented by Paradox and is inferred from the output. A Stellaris patch can
change it without notice.

Mitigation: build the parser with strict mode and verbose mode. Strict mode fails
on any unrecognized line format; verbose mode warns and skips. Always run in strict
mode after a Stellaris update and review the failures before regenerating the spec.

What the parser must extract per entry: `(name, kind, params, scopes, returns,
description)`. The trigger_docs format encodes most of this; the gaps are where
cross-validation with cwtools-config and vanilla scripts fills in.

#### cwtools-config import

The CWT format has 40+ field type variants (per the cwtools review). The relevant
subset for vanilla spec generation:

Required: `scope[type]`, `scope_group[name]`, `push_scope`, `replace_scope`,
`cardinality`, `alias[class:name]`, `alias_name[X]`, `alias_match_left[X]`,
`bool`, `int`, `float`, `scalar`, `<type_name>` (game object references),
`enum[name]`, `localisation`.

Sufficient for the scope/type information the Clause type checker needs:
`scope[type]`, `scope_group[name]`, `push_scope`, `replace_scope` are the
essential four. The rest are for cardinality and value validation, which the
Clause compiler doesn't fully need in v1 (it defers to cwtools for those).

**Which subset is "enough"?** The spec crate needs to answer: "what scope is this
effect valid in? what does it push/replace?" These are `push_scope` and
`replace_scope`. A 10-field subset is enough for type checking. The remaining 30+
fields are for more detailed validation (cardinality, localisation keys) which can
be added incrementally.

Risk if the subset is wrong: the spec says "effect X is valid in Country scope"
but the game says "actually it requires Fleet scope." Every use of effect X in
Clause gets an incorrect type-check pass, and the error surfaces only at game
load. **Cross-validation against vanilla scripts is the safety net.**

#### Cross-validation algorithm

"Parse vanilla scripts and compare claims." Concretely:

1. For each effect/trigger in the merged spec, find all occurrences in vanilla
   scripts.
2. For each occurrence, check: is the scope in which it appears consistent with
   the declared valid scopes?
3. Flag discrepancies: "spec says effect X is valid in Country, but vanilla uses
   it in Pop scope at `events/pop_events.txt:142`."

Implementation:
- Parse all of `common/` and `events/` from the vanilla cache using the G2
  Clausewitz lexer (already available or being built).
- For each trigger/effect use site, track the enclosing scope context (using the
  scope stack from G2's scope validator).
- Build a `usage_map: dict[str, set[CWScope]]` of "this name appears in these
  scopes."
- Compare `usage_map` against the spec's `valid_scopes` claim.
- Discrepancies are `(name, claimed_scope, observed_scope, file, line)` records
  surfaced for review.

How much vanilla to parse: **all of `common/`, `events/`, `map/`**. This is
~60MB of text. With the G2 lexer it should parse in seconds. Parse everything;
the cross-validator is only run during spec regeneration (`clause spec ingest`),
not during normal compilation.

#### Conflict resolution procedure

When trigger_docs says "scope: country" but vanilla shows it used from pop scope:

1. Check if the vanilla usage is in a PREV/FROM context (the pop event may have
   `owner = { ... }` establishing a Country scope before the effect is called).
2. If the vanilla usage is direct (not through a scope transition), trust vanilla
   over trigger_docs — trigger_docs can be stale or wrong.
3. If ambiguous, surface as a `SPEC_CONFLICT` item in `sources/spec_conflicts/`.
   The curated overlay must resolve it before the spec is generated.

**Document this procedure explicitly in the spec crate README.** It will come up
on every Stellaris update.

---

### M10 — Clause CLI

**Difficulty: LOW-MEDIUM. Estimated effort: 1–2 weeks.**

`clause build`, `clause check`, `clause fmt`, `clause test`, `clause doc`,
`clause spec` are all straightforward wrappers around the compiler pipeline.
The interesting one is `clause explain`.

#### `clause explain` index organization

To answer both "what's at source line N?" and "what's at emit line M?" in
constant-ish time, maintain two sorted arrays:

```python
@dataclass
class SourceMapIndex:
    # Source-to-emit: sorted by (source_file, source_line)
    by_source: list[MapEntry]  # sorted, binary-searchable
    # Emit-to-source: sorted by (emit_file, emit_line)
    by_emit: list[MapEntry]    # sorted, binary-searchable
```

After writing `source_map.json`, build these two sorted structures. Each lookup
is a binary search: O(log n) for n entries. With 4,500 patches each producing
~100 emit lines, n ≈ 450,000. `bisect.bisect_left` handles this in microseconds.

The explain CLI loads the index at startup and answers point queries directly.
No database needed for the index; memory-map the sorted arrays.

#### Incremental builds

The design mentions caching "at per-crate granularity." The existing pass
framework has a SQLite-based cache with fingerprint-based invalidation. Clause
should use the same mechanism. Per-crate fingerprints: hash of all `.cw` source
files in the crate. If the fingerprint matches the last build, skip the entire
frontend (lex → typecheck) and re-use the cached `EmitPlan` stream. Codegen
still runs (to re-route and collision-detect), but it is O(cached_items) not
O(parsed_items).

Per-item granularity (only re-compile changed items) is achievable but requires
tracking inter-item dependencies within a crate (if `Bloodline.add_member` changes
and `NewHeir` event calls it, both must re-compile). This is a dependency graph
within the crate. V1 can skip this and use per-crate granularity; add per-item
incremental in v2 when crate sizes justify it.

---

## 3. Critical Implementation Gaps

These are design elements where the specification relies on an algorithm but
doesn't provide one. Each gets a minimum viable implementation proposal.

### Gap 1 — Patch chain total ordering across workspace crates

**Where:** §23.1, §10.1.  
**Problem:** Multiple crates can each have a `#[patch(target = "...")]` impl for
the same `(Trait, Type, method)`. The design says "a third crate must write the
merged patch explicitly" — but how does the compiler determine which crate is the
"third" (consolidating) one? If megapatch is a crate-level patch consolidator,
and two sub-crates both patch the same target, there must be exactly one `#[patch]`
without any competing peers. The definition of "competing" and "consolidated" is
implicit.  
**Minimum viable:** A patch is in conflict if two `#[patch]` impls for the same
`(Trait, Type, method)` exist in sibling crates in the dependency graph (i.e.,
neither is a dependency of the other). Error out, require the author to write a
consolidating impl in a crate that depends on both.  
**Ideal:** Detect this automatically and scaffold the consolidating impl as a
code generation suggestion.

### Gap 2 — `let` binding codegen with mutation across scope boundaries

**Where:** §24.4, §10.2.  
**Problem:** `let x = self.size; if cond { x = x + 1; }` — a `let` binding that
is mutated inside a nested scope. In Clause, `let` is mutable if it reads from a
mutable source. In Clausewitz, there are no local variables; `let mut` must emit
a scratch variable. But when the scratch variable is mutated inside a nested
scope (inside an `if` block), the scratch variable must be accessible in the
enclosing scope after the block. How is the scratch variable name allocated and
scoped?  
**Minimum viable:** Allocate a unique scratch variable per `let mut` binding:
`_scratch_<fn_name>_<binding_name>_<uid>`. The variable is live for the entire
method body. No scope-based cleanup (Clausewitz doesn't have scope-local variable
declarations anyway). Use a per-method `uid` counter.  
**Edge case:** `let mut x` inside a loop (`for x in iter { let mut y = 0; ... }`)
would allocate the same scratch variable name on each iteration, which is
actually correct in Clausewitz (the loop body re-executes with the same scratch
var). Verify this is the intended semantics.

### Gap 3 — `Cached<T>` compile hook generation

**Where:** §14.6.  
**Problem:** `Cached<int, refresh = "monthly">` must emit a hook in the
`on_actions/monthly_pulse` file that recomputes the cached value. But the
compiler doesn't have a single emit site for this — the `Cached<T>` field is
declared in the struct, and the recompute logic is in the `get_or_compute`
closure body. The compiler must:
1. Recognize `Cached<T, refresh = "X">` fields.
2. Extract the closure body of `get_or_compute(|| { ... })`.
3. Emit that closure body as a `scripted_effect` or `scripted_trigger`.
4. Emit an `on_action` entry that calls that effect on each `monthly_pulse`.  
**Minimum viable:** During codegen, for each `Cached<T>` field, emit a companion
`scripted_effect` named `<crate>_<struct>_<field>_recompute` containing the
closure body, and emit an `on_action` entry that calls it. The `invalidate()`
method sets a flag variable that the recompute effect checks before running.  
**Gap:** The `on_action` emit requires the codegen to write to the `on_actions/`
output directory, which is a different routing target than the struct's own
`scripted_effects/` output. The codegen's routing logic (§22) must handle
multi-file output from a single struct declaration.

### Gap 4 — `Iter<T>` codegen and scope push

**Where:** §14.2, §24.4.  
**Problem:** `for x in self.bloodline.iter()` in effect context emits
`every_pop = { ... }`. But how does the compiler know `Iter<Pop>` maps to
`every_pop` and not `every_country` or some other iterator? The mapping is
scope-type-dependent: `Iter<Pop>` in a Country scope emits `every_owned_pop`;
`Iter<Pop>` in a Planet scope emits `every_pop` (scoped to the planet). The
`every_*` keyword depends on BOTH the element type AND the current Clausewitz scope.  
**Minimum viable:** Maintain a lookup table `(element_type, cw_scope) →
every_keyword`. This table is populated from the vanilla spec. Build it as a
`dict[(CWScope, CWEntityType), str]` mapping to the Clausewitz keyword. For
cases where no entry exists (e.g., `Iter<Leader>` in a Pop scope), emit a
type error: "cannot iterate over Leader in Pop scope."  
**Note:** This table has ~50–100 entries covering the common cases. Most
gaps are uncommon scopes and can be added on demand.

### Gap 5 — `extern` body-kind classification

**Where:** §13, §25.  
**Problem:** `extern impl CountryActions for Country` declares that the engine
provides the implementation. But kind inference (§9.4) needs to know whether
`create_pop` is an effect, `total_resources` is a trigger, etc. This metadata
must come from the vanilla spec — but the spec declares the interface only as
method signatures, not as effect/trigger/value kinds.  
**Minimum viable:** Add a `kind: EffectKind | TriggerKind | ValueKind` annotation
to each extern method declaration in the vanilla spec. Populate from the trigger_docs
`kind` field. During kind inference, `extern` method calls inherit the kind of
their declaration in the spec.  
**Gap if unresolved:** Every call to an extern method will trigger the "ambiguous
emit" error without a `#[prefer(...)]` annotation, because the kind inferrer has
no information about the callee's kind. This would make writing impl bodies that
call vanilla effects extremely painful.

---

## 4. Architectural Concerns

### Concern 1 — Python performance at 4,500-patch scale

The design targets 4,500 patches in the megapatch crate. Each patch has at least
one `impl` block with at least one method. Type checking 4,500 `impl` blocks
with coherence checking, patch chain resolution, and generic monomorphization in
Python will be slow.

Rough estimate: if type-checking one impl takes 1ms (conservative for Python with
dict lookups, span tracking, and inference), 4,500 impls = 4.5 seconds for the
type checker alone. Add the transpiler (~0.5ms per method body, ~10,000 method
bodies) = 5 seconds. Add lexing and parsing (~200ms for all source files). **Total
cold build: 10–15 seconds.** This is borderline acceptable for a full build but
will feel slow for `clause check`.

Mitigation strategies in order of impact:
1. **Per-crate incremental caching.** If the heritage crate hasn't changed, don't
   re-type-check it. This alone brings `clause check` latency to under 2 seconds
   for incremental builds.
2. **Parallel crate compilation.** The crates in the workspace have a dependency
   DAG. Type-check independent crates in parallel using `multiprocessing`. With
   4 workers, wall-clock time halves.
3. **PyPy for development.** The Python compiler running on PyPy typically gets
   3–5x speedup on tight loops. The existing compiler already uses `spawn` for
   multiprocessing, which is PyPy-compatible.
4. **Defer to Rust/mypyc.** Post-v1, if performance is a real problem, the inner
   loops (lexer, type checker coherence check, transpiler) can be compiled with
   `mypyc` (the MyPy ahead-of-time compiler) for 2–10x speedup without rewriting.

For Phase 1–2 acceptance, Python performance is not a blocking issue. Revisit
after Phase 3 (Heritage full migration) when the full codebase size is known.

### Concern 2 — Transpiler correctness for complex combinations

The transpile rewrite table (§24.4) is specified entry-by-entry, not as a
compositional algebra. This means the correctness of nested combinations
(`match` inside `for` inside `match`, `?` inside `match` inside `for`) is not
provable from the table — it must be tested empirically. The risk is discovering
a combination that produces syntactically valid Clausewitz with wrong semantics,
which would only surface during gameplay testing.

Mitigation: establish a suite of "composition fixtures" in M6's tests — one
fixture per complex combination from §24.4. Each fixture has: Clause source,
expected emitted Clausewitz, and a note on what the semantic guarantee is. Run
the fixture suite on every build. Add new fixtures when a new combination is used
in Heritage migration (M11).

### Concern 3 — vanilla spec freshness

The compiler's type guarantees are only as good as the vanilla spec. An outdated
spec (e.g., Stellaris 4.1 adds a new effect that the spec says doesn't exist)
causes the compiler to reject valid code. An incorrect spec (wrong scope
declaration) causes the compiler to accept invalid code that fails at game load.

This is not an algorithmic problem; it is a maintenance process problem. The spec
crate needs:
1. A CI check that runs `clause spec validate` after every Stellaris update, flags
   changes.
2. A documented workflow for updating the spec: ingest → diff → review conflicts →
   merge curated overlays → regenerate → CI green.
3. Versioning: Heritage 0.x depends on `stellaris-vanilla-spec = "4.0"`. When
   spec 4.1 ships (after a Stellaris update), Heritage must opt in by bumping its
   dependency version.

Without this process, the spec will drift within 3–6 months of active Stellaris
development.

### Concern 4 — The `Bind` sealed trait requires a fixed set of carriers

The `Bind` sealed trait (§8.2, §14.0) is sealed via the private-supertrait
pattern. Only stdlib + the fixed set of declared carriers can satisfy it. This
means adding a new vanilla scope type to the spec requires the stdlib to also
add a `Bind` impl for it. This creates a coupling between the vanilla spec and
the stdlib that is not immediately obvious.

When Stellaris adds a new scope (it does this with DLCs and updates), the spec
must declare it, AND the stdlib must add `impl Bind for NewScope {}`. If the
stdlib doesn't add the impl, code that tries to use `struct Foo: NewScope {}` 
will fail with an opaque sealed-trait error. Add a check to the spec regeneration
pipeline: "for each scope declared in the spec, verify that a `Bind` impl exists
in stdlib." Fail the spec CI if any scope is missing its `Bind` impl.

### Concern 5 — The megapatch crate has ~4,500 patches; `#[patch]` by design requires consolidation

The current megapatch YAML patches represent individual file-level overrides,
many of which will become `#[patch]` impls in Clause. The design says two patches
on the same `(Trait, Type)` from different crates → compile error forcing
consolidation. But the megapatch IS the consolidation crate — it's designed to
consolidate patches from 448 source mods.

The issue: within the megapatch crate, there will be cases where the compiler
sees two impls targeting the same vanilla `(Trait, Type)` from two different
source mods' YAML patches. These shouldn't be a compile error — they should be
the subject of consolidation. But the orphan rule as described would flag them.

Resolution: within the megapatch crate, impls that consolidate upstream mods'
patches are exactly the `#[patch]` use case. Every megapatch impl of a vanilla
`(Trait, Type)` carries `#[patch]` by definition (it's patching vanilla). Two
`#[patch]` impls for the same target WITHIN the megapatch crate → that is the
consolidation conflict that requires explicit merging into one impl. This is the
design intent (§23.3: "two patches of the same target → compile error forcing
consolidation").

The practical question: how does the megapatch author write the consolidated impl?
They write ONE `impl` with `#[patch]` that references all the source mods'
contributions, with `..super` spreads or method-override composition. The
compiler's role is to ensure this ONE impl is the only one. This is correct and
achievable; the concern is that migrating 4,500 YAML patches to Clause impls will
surface many latent consolidation conflicts that YAML's "last file wins" behavior
silently resolved. Plan for this during Phase 4.

---

## 5. Recommended Sequencing Changes

### Current M-batch order: M2 → M3 → M4 → M5 → M6 → M7 → M8 → M9 → M10 → M11

### Recommended changes:

**1. Start M9 (vanilla spec) in parallel with M2–M4.**

The type checker (M5) and transpiler (M6) cannot be properly tested without
vanilla declarations. The vanilla spec ingest is also the longest-calendar-time
task because it requires Stellaris to be running to dump trigger_docs. Start M9
immediately: dump trigger_docs from the Steam Deck installation, import
cwtools-config, begin hand-curating the top 10 scopes. This can proceed in
parallel with M2–M4 because M9 produces static data files (`generated/*.cw`),
not compiler code.

**2. Insert M5a (coherence) and M5e (exhaustiveness) before full M5.**

Coherence checking and pattern match exhaustiveness can be built and tested
independently of kind inference and generics. Build them first as standalone
passes with clear input/output contracts. This makes M5 parallelizable: one
contributor works on coherence, another on kind inference.

**3. M11 should start after M6, not after M10.**

M11 (first migration: Bloodline) is the acceptance test for the whole frontend.
Starting it after M6 (transpiler) rather than waiting for M10 (CLI) means you get
end-to-end validation earlier. The CLI can be built around the working compiler;
the working compiler should not wait for the CLI.

**4. Split M8 into M8a (git diff detection) and M8b (sham scaffolding).**

These are independent pieces. M8a (detecting renames via git diff) can ship as
soon as M5 is stable. M8b (sham handler generation) requires the codegen layer
(M7) to be working. Running them sequentially adds calendar time unnecessarily.

**5. Defer M12 (LSP) until Phase 3 is at 50%.**

The design already marks M12 as post-bootstrap. Affirm this explicitly and resist
any pressure to start it earlier. LSP requires a stable type checker with reliable
incremental updates — that infrastructure doesn't exist until Phase 3 is well
underway.

**Revised order:**

```
Phase 1a (parallel): M2, M3, M9-start
Phase 1b (sequential): M4, M5a+M5e (parallel), M5b+M5c+M5d
Phase 1c: M6, M7 (parallel), M8a
Phase 1d: M8b, M9-finish, M10, M11
Phase 2+: M12
```

---

## 6. Dependencies and External Risks

### Python version

The existing compiler uses `mp.get_context("spawn")` (Python 3.12+) and
`graphlib.TopologicalSorter` (Python 3.9+). Clause should target Python 3.12
minimum for the spawn behavior and to enable type-checking annotations on the
compiler itself. No new dependencies needed for the core language; only stdlib.

**Optional dependencies** that provide significant value:
- `lark` or `textX`: if the recursive-descent parser proves unwieldy, a parser
  generator reduces risk. Not necessary if the grammar stays simple, which is
  achievable given Clause's constrained syntax.
- `mypy`: type the compiler itself. The existing codebase has partial typing;
  enforce strict mode on new Clause files.

### trigger_docs format stability

**Risk: medium.** The trigger_docs output format has changed between major
Stellaris versions. The spec ingest parser must be versioned and capable of
handling both the current format and fallback-to-manual-review when format changes
are detected.

Mitigation: snapshot the raw trigger_docs output in `sources/trigger_docs/<version>/`
alongside the parsed results. If the format changes, the old snapshots remain
parseable, and the diff between old and new snapshots reveals what changed.

### cwtools-stellaris-config maintenance

**Risk: medium.** cwtools-stellaris-config is community-maintained and lags
Stellaris by weeks to months on new content. The cross-validation step catches
cases where the CWT config is behind vanilla.

Mitigation: the spec crate's curated overlay provides a mechanism for hand-fixing
cases where cwtools-config is wrong or missing. This is the designed escape hatch.
If cwtools-stellaris-config were abandoned, the spec crate would fall back entirely
to trigger_docs + vanilla scripts cross-validation, which are both first-party
Stellaris artifacts.

### Stellaris modding API stability

**Risk: medium-low.** Paradox occasionally renames effects, removes triggers, or
changes scope semantics in patches. This breaks the vanilla spec's declarations.
When this happens: the type checker starts rejecting previously-valid Clause code.
The error messages are confusing until the spec is updated.

Mitigation: Tenet 12 (refactoring safety) is specifically designed for this case.
When the vanilla spec is updated, the compiler detects that `extern impl`
declarations have changed and reports which Clause code uses the changed APIs.
This is the correct behavior. The risk is the gap between a Stellaris patch and a
spec update — typically days to weeks depending on community activity.

### Single biggest unknown-unknown

**The transpiler's semantic correctness for Clausewitz scope semantics.** The
design abstracts away Clausewitz's scope model behind Clause's type system, but
the transpiler must faithfully emit Clausewitz that the engine will execute with
the correct scope semantics. The scope transition rules in Clausewitz are complex
(PREV/FROM/ROOT semantics, scope push/pop in iterators, scope inheritance across
scripted_effect calls). If the transpiler gets any of these wrong, the emitted
Clausewitz compiles and loads without error but executes with wrong scope context
at runtime, producing incorrect game behavior that is very hard to debug.

This is unknown-unknown territory because the scope semantic bugs are:
- Not caught by the Clausewitz file validator (valid syntax, wrong semantics)
- Not caught by the G2 linter passes (no scope-semantic checking yet)
- Only visible during gameplay in specific game states (the wrong-scope effect
  fires but produces no observable effect because the scope doesn't have the
  expected entity)

The mitigation is the Phase 2 acceptance gate: byte-identical output vs
pre-migration baseline for the Bloodline subsystem. If the transpiler gets scope
semantics wrong, the output will differ from the hand-authored Clausewitz that
is known-good. This is the right gate, but it requires that the pre-migration
baseline is itself correct (which it is, per the existing gameplay validation).

---

## 7. Minimum Viable Product Sketch

The full design is a complete language. The MVP is the smallest subset that
delivers the three core value propositions: (1) type-checked method bodies, (2)
compiler-routed output, (3) patch-chain conflict detection.

### MVP scope

**In:**
- Clause lexer (M2) — complete.
- Clause parser (M3) — `struct`, `trait`, `impl`, `fn` declarations, method
  bodies as raw Clausewitz with `self.field` expansion. `event` declarations.
  No generics. No `expect`/`actual`. No `match`/`for`/`while` desugaring (just
  passthrough to raw Clausewitz in bodies).
- Module resolver (M4) — `mod foo;` chain, `use` imports within a workspace.
  No cross-crate `expect`/`actual`.
- Type checker (M5) — orphan rule, patch chain detection (no `super.foo()`
  resolution yet — just detect conflicts). Kind inference for top-level method
  declarations. No generics.
- Transpile (M6) — `self.field` expansion only. No control-flow desugaring
  (bodies are emitted as-is). This is the "Clause syntax sugar on top of raw
  Clausewitz bodies" MVP.
- Codegen (M7) — routing per §22, collision detection, source map (line-level).
- vanilla spec (M9) — hand-curated for 10 core scopes, no ingest pipeline.
  Enough to type-check Heritage's Bloodline subsystem.
- CLI (M10) — `clause build`, `clause check` only.

**Out of MVP:**
- `match`/`for`/`while` desugaring.
- `Option<T>`/`Result<T,E>` codegen.
- `?` operator.
- Generic monomorphization.
- `expect`/`actual`.
- `Cached<T>`.
- Manifest management / sham handlers.
- Full vanilla spec ingest pipeline.
- `clause explain` / source map query.
- LSP.

### What the MVP proves

With this scope, a `.cw` file can declare a `struct Bloodline`, an `impl
Bloodline` with method bodies that are essentially Clausewitz with `self.field`
sugar, and an `event NewHeir for Country`. The compiler routes the output to the
correct files, validates the struct's field references, enforces orphan rule on
impls, and detects duplicate impls of the same `(Trait, Type)`.

This is not the full Clause language but it IS a compiler that provides value:
- Field reference validation catches `self.sized` typos.
- Compiler-routed output eliminates hand-managed file decisions.
- Orphan rule catches duplicate patches at compile time.

Estimated effort for MVP: **4–6 weeks** of focused work. The Phase 1
acceptance gate ("a trivial `.cw` file compiles end-to-end") is achievable at
MVP scope. Phase 2 ("Bloodline in Clause, byte-identical output") requires adding
`self.field` expansion and event routing on top of MVP — another 2–3 weeks.

### Decomposition into sequential MVPs

1. **MVP-0 (1 week):** Lexer + passthrough parser. `clause build` reads a `.cw`
   file and emits it unchanged as Clausewitz tokens to the output path. No type
   checking. Proves the pipeline end-to-end.
2. **MVP-1 (2 weeks):** Add struct parsing, `self.field` expansion, and routing.
   `struct Bloodline { mut size: int }` emits to the correct output file with
   `@heritage_bloodline_size`. Proves field expansion and routing.
3. **MVP-2 (2 weeks):** Add orphan rule, patch chain detection. Two conflicting
   impls → compile error. Proves conflict detection.
4. **MVP-3 (2 weeks):** Add kind inference, event declaration routing. Proves
   type-directed codegen.
5. **Phase 2 gate:** Port Bloodline subsystem to `.cw`. Byte-identical output.

Each MVP-N is independently shippable and provides incrementally more value.
None require the full M-batch to be complete.

---

*End of review.*
