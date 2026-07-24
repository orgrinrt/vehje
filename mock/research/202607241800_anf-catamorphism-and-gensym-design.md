# ANF catamorphism and gensym: the node-creating lowering path

**Date:** 2026-07-24
**Scope:** `vehje-lower` design for the node-creating `Anf` pass, the fresh-binder mechanism, the arena/`Builder` capacity model, and the termination and hygiene arguments. Design deliverable; a subsequent round implements it.
**Source topics:** the macro-hygiene / generative-freshness conviction (op-ratification-answers XVII6), the arena-count fork (XVII7), Rompf finding 4 (gensym through the builder), Fallin's ANF-before-CFG dependency.

The lowering crate ships two shapes of rewrite today, and only one of them exists. The redirecting shape (`Rewrite`, `ConstFold`, `Cse`) records a `NodeRef`-to-`NodeRef` remap in a caller-lent side table and never touches the arena (`vehje-lower/src/lib.rs:36-69`). It terminates because every redirect points at a strictly smaller arena index, so `resolve`'s chain strictly decreases (`vehje-lower/src/lib.rs:55-68`). The node-creating shape does not exist: `Anf::apply` is an unconditional no-op with a `// FIXME:` saying so (`vehje-lower/src/lib.rs:382-393`), because A-normal form hoists each non-trivial subexpression into a fresh `Let` binding, and creating a `Let` is creating a node, which the redirecting model cannot do.

This document designs the node-creating path. It is the prerequisite for two later mechanisms: macro expansion (`MacroExpand`, also a no-op FIXME at `vehje-lower/src/lib.rs:399-408`, which must splice fresh nodes) and the CFG-of-blocks construction, which Fallin showed is downstream of ANF specifically, because a `Node::If` nested inside a `Let` value slot has no well-defined successor block until ANF has bound the branch results by name (`chris_fallin_canon-recheck-and-drift-audit.md:147-179`). ANF is not a nicety here. It is the load-bearing flattening the residual model was designed around.

The crux, and the part with a genuinely open mechanism, is fresh binder names. `Let.name` is an interned `hilavitkutin_str::Str` (`vehje-ir/src/node.rs:91-96`). There is no gensym in the codebase, and the lowering pass has no interner in reach: it takes an `Arena`, not a `StringInterner`, and interning a runtime name requires a host `ArenaInterner` the pass does not hold (`hilavitkutin-str/src/interner.rs:14-19,40-49`). The pass must mint fresh, capture-safe binder identities with no interner and no allocation. That mechanism is designed in full below, with the alternatives weighed and one recommended.

## What the fold produces, and how it differs from the redirecting passes

The redirecting passes are read-only over the arena plus a mutable side table. `fold_consts` and `cse_share` walk the tree, read nodes through `arena.get`, and record redirects; they return nothing structural (`vehje-lower/src/lib.rs:134-178,219-310`). ANF cannot work this way, because its output is a tree that did not exist in the input: the hoisting `Let`s are new nodes. So ANF is a fold from the input tree into freshly appended arena nodes, and it returns the new root as a `NodeRef`.

Three consequences follow, and each is an API-visible design decision.

First, ANF needs mutable arena access. The redirecting passes take `arena: &Arena`; ANF takes a `&mut Builder` (or equivalently a `&mut Arena`), because it calls the smart constructors (`Builder::let_`, `Builder::apply`, `Builder::var`, at `vehje-ir/src/builder.rs:49-51,59-62,44-46`), each of which appends and returns `Maybe<NodeRef>`. The `Maybe::Isnt` case is the no-alloc capacity-exhaustion signal (`vehje-ir/src/arena.rs:45-54`); ANF propagates it, so `Anf::apply` returns `Maybe<NodeRef>`, `Isnt` when the caller-lent arena is too small.

Second, ANF reads the input through the redirect view. The redirecting passes run first and record their redirects in `rw`; ANF must see the folded, shared tree, not the raw one. So the fold reads each input child through `rw.resolve` (following a folded `If` to its taken branch, a duplicate subtree to its canonical occurrence). ANF reads from the input index range and writes to the appended range, and those ranges are disjoint (input occupies `[0, k)`, appended nodes occupy `[k, ...)`), so there is no aliasing between the read side and the write side even though both live in one arena.

Third, the returned root is not `rw.resolve(root)`. The redirecting `Lower::lower` returns the input root resolved through the remap (`vehje-lower/src/lib.rs:450`). With ANF in the pipeline, the lowered root is the fresh node ANF built, so `Lower::lower` returns ANF's result. The ordering in `Lower::lower` is already correct (macro-expand, fold, share, then ANF last, `vehje-lower/src/lib.rs:442-449`); only the last step changes from a no-op that discards its result to a builder pass whose returned root becomes the program root.

### The signature

```
impl Anf {
    fn apply(
        &self,
        builder: &mut Builder<'_>,   // node-creating access, not &Arena
        root: NodeRef,               // the input root (indexes the frozen [0,k) region)
        rw: &Rewrite<'_>,            // read the folded/shared view of the input
        fresh: &mut FreshCounter,    // the gensym source, see below
    ) -> Maybe<NodeRef>              // the ANF'd root, Isnt on arena-full
}
```

`Lower::lower` correspondingly takes a `&mut Builder` instead of a `&Arena` for the stretch that runs ANF, and threads one `FreshCounter` through both `MacroExpand` and `Anf` so their fresh-name spaces cannot collide. This signature change to `Lower::lower` is the first maintainer call (Section 8).

## The catamorphism over the twelve forms

ANF is the standard two-function normalization (Flanagan, Sabry, Duba, Felleisen, "The Essence of Compiling with Continuations", PLDI 1993), adapted to build into an append-only arena with no closures and no allocation. Two operations recur over the twelve `Node` forms (`vehje-ir/src/node.rs:83-144`):

`normalize_term(e) -> NodeRef` ANF-normalizes a whole term and returns its new root. It is what runs on a body, a branch, a lambda body: any position that is its own evaluation context.

`atomize(e) -> (atom, binding)` normalizes `e` and guarantees the result is an atom. If the normalized `e` is already an atom (a `Lit` or a `Var`), it returns that atom with no binding. Otherwise it picks a fresh name `t`, and returns the atom `Var(t)` together with the binding `(t, normalized_e)` that the caller must wrap in a `Let` around the enclosing term. This is where hoisting happens.

"Non-trivial" (the thing that gets hoisted) means exactly "not an atom in an operand position." An atom is `Lit` or `Var`. Everything else, appearing where a value is consumed (a function argument, a projection base, an if-condition, an interpolated value, a match scrutinee, an iter sequence), is hoisted to a named `Let` binding so that the operand position holds only an atom and effect order is explicit in the nesting of the `Let`s.

Per form:

- **`Lit`** and **`Var`** are atoms. `normalize_term` returns them unchanged; `atomize` returns them with no binding. Zero new nodes.
- **`Let { rec, name, value, body }`** keeps the user binding. `value` is `normalize_term`'d (a `Let` value is itself an evaluation context that may legitimately be a serious expression), `body` is `normalize_term`'d, and the rebuilt `Let` carries the original `name` and `rec`. ANF adds bindings; it never removes or renames the user's.
- **`Lambda { param, body }`**: `body` is `normalize_term`'d. The body is a fresh evaluation context, so any hoisting inside it stays inside the lambda. The rebuilt `Lambda` keeps `param`.
- **`Apply { callee, args }`**: `callee` is atomized, each element of `args` is atomized. The rebuilt `Apply` has an all-atom callee and an all-atom arg list; the collected bindings wrap it (Section 4 gives the wrap).
- **`Project { base, key }`**: `base` is atomized; the rebuilt `Project` keeps `key`.
- **`If { cond, then_branch, else_branch }`**: `cond` is atomized (the condition value is named before the branch is chosen). The branches are `normalize_term`'d, not atomized. This is the load-bearing case for correctness: a hoist inside `then_branch` must stay inside `then_branch`, or a subexpression that should run only when the condition holds would be evaluated unconditionally. So the branch hoists nest inside each branch term, and only the condition's binding wraps the `If`.
- **`Match { scrutinee, arms }`**: `scrutinee` is atomized; each arm is `normalize_term`'d (an arm is its own context, same reason as an if-branch). M-level `Match` stores arm bodies as a `NodeList` with the pattern representation deferred (`vehje-ir/src/node.rs:112-119`), so ANF treats arms as body terms; it revisits when the pattern node kinds land.
- **`Iter { seq, body }`**: `seq` is atomized; `body` is `normalize_term`'d. The loop body is a repeated evaluation context, so its hoists stay inside the body, not lifted to run once before the loop.
- **`Interp { value }`**: `value` is atomized. Interp is a distinct form because the staging rule attaches to it (`vehje-ir/src/node.rs:120-124`); naming its value keeps the interpolation's effect order explicit, which is the whole point of ANF for the staging pass downstream.
- **`Raw { family, payload }`**: the payload is family-interpreted and opaque (`vehje-ir/src/node.rs:125-130`). ANF cannot know whether a payload child is an eager evaluation position (hoist it) or an opaque handle the family reads structurally (leave it). The conservative default is to `normalize_term` each payload child (so hoisting inside a payload subtree is preserved) but not to atomize payload children (do not hoist across the family boundary, because the family's evaluation semantics are unknown). Whether Raw operands are evaluation positions is a `Family`-hook question (Section 8).
- **`Handle { body, clauses }`**: `body` is `normalize_term`'d and each clause is `normalize_term`'d. `Handle` is a control form (`vehje-ir/src/node.rs:131-143`); hoisting a subexpression out of a handled body past the handler would change which handler is in scope for its effects, so hoists stay inside the body. This is the same containment invariant as the if-branches, applied to the twelfth form, and it is where ANF and the effect discipline meet.

The output root is whatever `normalize_term(root)` returns: either the input root's atom (if the whole program was already atomic) or the outermost hoisting `Let`.

## Building the wrap with no closures and no allocation

Classic ANF expresses the wrap with a continuation closure `k`. In `no_std` with no alloc, closures that capture are not available as heap objects, and the arena is append-only, so the wrap is realized by building inner-to-outer against the arena.

The key observation: a `Var(t)` node can be built before the `Let(t, ...)` that binds it, because both carry the same `Str` name and name resolution (which runs after ANF) matches them by name. So the fold does not need the binding to exist before referencing it.

For a fixed-arity form (say `Project`), the wrap is trivial: `atomize(base)` yields `(atom, binding)`; build `Project(atom, key)`; if `binding` is `Some((t, v))`, the result is `Let(t, v, Project(...))`, else the `Project` itself.

For a list form (`Apply`, `Match`, `Iter`, `Handle`, `Raw`), the operands evaluate left-to-right, so the leftmost operand's `Let` must be the outermost. The fold builds inner-to-outer, which means it processes bindings in reverse. Concretely for `Apply(f, [a0, a1])` normalizing to `let t0 = a0' in let t1 = a1' in f(t0, t1)`:

1. Atomize `callee` and each arg. For an arg already atomic, use the atom directly (no fresh name, no binding). For a compound arg, allocate a fresh name, build the `Var` for the arg-list position, and record the binding `(name, value)`. The recorded bindings live in the caller-lent child pool as a transient scratch region, so no separate buffer is allocated (the pool already exists to hold transient child lists, `vehje-ir/src/arena.rs:58-71`).
2. Build the arg-list `NodeList` from the atom positions (`Builder::alloc_list`), then build `Apply(callee_atom, args)` as the innermost body.
3. Fold the recorded bindings in reverse index order: `body = Let(t_i, v_i, body)` for `i` from last to first. Reverse order makes the first-evaluated binding the outermost `Let`, so scoping and effect order are left-to-right.

The bindings scratch is bounded by the operand count of the node being built, and it is released (the pool cursor rewound, or simply not referenced again) once the wrap is done. The pool's role as scratch is what the capacity bound in Section 5 accounts for.

Recursion depth follows the input tree height. The existing passes recurse (`fold_consts`, `cse_share`), so recursion matches the crate idiom; a deep-tree stack overflow is the same exposure those passes already carry. An explicit work-stack rewrite is a later hardening (the `Saturate` FIXME notes recursion is a hard wall for the e-graph extractor at `vehje-lower/src/lib.rs:82-85`); it is not required for the first landing, and it is called out as a known future item rather than left silent.

## The arena model and the capacity bound

The default is a single caller-lent arena. The input tree occupies node indices `[0, k)` and pool entries `[0, p)`. ANF appends its fresh nodes at `[k, ...)` and its fresh child lists (and transient binding scratch) at `[p, ...)`. One `NodeRef` index space, no branding: a `NodeRef` read from the input and a `NodeRef` built by ANF are values in the same space and compose without a tag. This matches the crate's current single-arena `Builder`/`Arena` shape (`vehje-ir/src/arena.rs:27-41`).

The caller must size the arena for the growth, because ANF creates more nodes than it reads. The growth is linear in the input size, which is what makes it a caller-provisionable bound rather than an open-ended one.

Let `N` be the input node count and `P` the input pool length (total child-list entries). Each compound operand in an operand position contributes at most one fresh `Let` (one node) and one fresh `Var` (one node). The number of operand positions is bounded by the fixed operand slots (a small constant per node) plus the list-form entries (bounded by `P`). Each input node is rebuilt at most once. So:

- fresh `Let` + `Var` nodes: at most `2 * (N + P)`
- rebuilt nodes: at most `N`
- total appended nodes: `nodes_out <= 3*N + 2*P`
- appended pool entries: rebuilt child lists plus fresh binder `Var`s in those lists, at most `2*P`, plus the transient binding scratch (reused, bounded by max operand count), so `pool_out <= 2*P + O(max_arity)`

The simple provisioning rule for the caller: node arena of capacity `>= N + 3*N + 2*P` and child pool of capacity `>= P + 2*P`, rounded to a comfortable `4*N` nodes and `3*P` pool as a safe default. The exact constant is bench-tunable and is a maintainer call (Section 8); the design's obligation is that the bound is linear and stated, so a caller can size the region and an over-run returns `Maybe::Isnt` rather than corrupting memory. The `push`/`alloc_list` full-checks already enforce that (`vehje-ir/src/arena.rs:47-49,60-63`).

### Where the N-buffer variant would change this

Op reframed the arena-count question as N host-configurable buffers, benched, carrying a cross-space `NodeRef` branding decision (`op-ratification-answers.md:69-74`). The single-arena default above is the hypothesis; the design must not foreclose the fork. Under an N-buffer variant, the input tree lives in buffer 0 (read-only during ANF) and ANF appends into buffer 1. Two things change, and only these two:

First, `NodeRef` must distinguish which buffer it indexes, because a fold that reads a child `NodeRef` from buffer 0 and writes a fresh `NodeRef` into buffer 1 now has two index spaces, and a raw index is ambiguous across them. That is the cross-space branding op named: `NodeRef` gains a space tag (or a per-buffer branded newtype), and the fold's invariant becomes "read space R, write space W, R != W," which the type system can enforce so an input ref can never be mistaken for a fresh ref. In the single-arena default this is free because R and W are the same space and the invariant is vacuous.

Second, `Builder`/`Arena` become multi-region (a builder over buffer 1 that reads buffer 0), which is a constructor and accessor change, not a fold-logic change. The catamorphism, the gensym, and the capacity bound are identical; only the read-vs-write region split and the ref branding differ. Keeping ANF's "read input region, write fresh region" invariant explicit in the single-arena design is what makes the later N-buffer bench a localized change rather than a rewrite. The bench can then decide whether a second buffer (independent expansion loops in parallel, or scratch, per op's caveats) pays for the branding cost.

## The fresh-name mechanism

This is the open crux. ANF must mint binder names that are fresh (never equal to any source name, never equal to each other) with no interner and no allocation, and the freshness must make capture structurally impossible because name resolution runs after ANF and after macro expansion.

A `Str` is a 4-byte handle: bit 31 marks const-origin versus runtime-origin, bits 30 through 28 are reserved flags, and bits 27 through 0 are a 28-bit id (`hilavitkutin-str/src/handle.rs:1-25`). Equality on `Str` is integer equality on the whole 32-bit layout (`#[repr(transparent)]` over `Bits<32>`, `hilavitkutin-str/src/handle.rs:27-30`). That equality is exactly what `Let`/`Var` name matching and CSE's `structurally_equal` (`n1 == n2` at `vehje-lower/src/lib.rs:324`) rely on.

### Recommendation: a synthetic-`Str` gensym from a monotonic counter, keyed by a reserved flag bit

Reserve one of the three currently-unused flag bits (bits 30 through 28) to mark a handle as synthetic, and use the 28-bit id field to carry a monotonic counter value directly. A fresh binder is `Str` with the synthetic flag set and id = counter; the pass bumps the counter for each hoist. No interner is consulted, no string is stored, no allocation happens. The `FreshCounter` is an `arvo` fixed-point value (`Uint<28, Hot>`, matching the id field width) threaded through `Anf::apply` (and `MacroExpand::apply`).

Why this is the right shape:

The synthetic handle is a first-class `Str`, so `Node::Let.name`, `Node::Lambda.param`, and `Node::Var` are unchanged, CSE's `n1 == n2` is unchanged, structural equality and the interner boundary are unchanged, and serialization width is unchanged. Nothing in the twelve forms moves.

Freshness and disjointness are structural, not conventional. A synthetic handle has the synthetic flag bit set; every source-interned handle has it clear. So a synthetic `Str` compares unequal to every source `Str` by integer equality, regardless of the counter value, even if the counter is 0 and a source name happens to be the literal text `"$t0"`. Two synthetic handles with different counter values compare unequal. The fresh-name space is provably disjoint from the source-name space and internally collision-free, by construction of the bit layout, not by a naming convention that a pathological source string could violate.

Resolution needs no backing string. Name resolution after ANF binds `Var`s to `Let`s by `Str` identity, and a synthetic `Var(t)` matches exactly the synthetic `Let(t, ...)` that binds it and nothing else. The synthetic handle never needs to resolve to a `&str` for correctness. For diagnostics, a renderer special-cases the synthetic flag and formats `$t{id}` on demand, so the handle is printable without ever being interned.

This is an upstream `hilavitkutin-str` addition: a `Str::__synthetic(counter: Bits<28, Hot>) -> Str` constructor that sets the reserved synthetic bit, and an `is_synthetic(self) -> Bool` predicate, mirroring the existing `__make`/`__runtime`/`is_const` shape (`hilavitkutin-str/src/handle.rs:44-58`). The workspace rule is to fix the substrate upstream rather than hand-roll in the consumer, and the reserved bits exist for exactly this kind of flag, so this is the substrate serving its purpose, not a squat. It needs a small `hilavitkutin-str` design round (Section 8). The interner's `resolve` should also learn the synthetic case (return a synthetic-marker or `Isnt` rather than scanning the const section), so a stray resolve of a synthetic handle is well-defined.

### Alternatives, and why they lose

**A pre-interned synthetic-name pool indexed by a bounded counter.** Generate a fixed array of const-origin handles `str_const!("$t0")`, `str_const!("$t1")`, up to a cap `MAX_FRESH`, and index it by the counter. This works and is no-alloc, but the cap is a hard ceiling on the number of hoists in one program, and ANF's hoist count is linear in program size, so a large input exhausts the pool and the pass fails on a size it should handle. It also burns const-section space on names that are only ever compared by identity, and it still needs a disjointness argument (a source program could contain `$t7` and collide with pool entry 7 unless the pool names are chosen from a reserved lexical space the lexer forbids, which is a lexer coupling the reserved-bit scheme avoids entirely). The reserved-bit scheme has no cap and no lexical-space coupling. Rejected.

**A `Binder` sum type distinct from `Str`.** Change `Node::Let.name`, `Node::Lambda.param`, and `Node::Var` from `Str` to `enum Binder { Source(Str), Fresh(Uint<28, Hot>) }`. This is the most type-honest option and it satisfies "illegal states unrepresentable" at the type level: a fresh binder is a different variant, not a flagged `Str`. But it changes three of the twelve Core forms, every match on a binder name, CSE's `structurally_equal`, the interner boundary, and serialization, a much larger blast radius for a distinction the reserved bit already encodes. It is the right answer only if the reserved bits turn out to be needed for something else, in which case the sum type is the fallback. Named as the alternative; not recommended for the first landing because the reserved-bit scheme buys the same disjointness for a fraction of the churn.

**Threading an interner or name-source into the pass.** Give `Anf::apply` a `&mut StringInterner<A>` and intern `"$t{n}"` per hoist. This pulls a host `ArenaInterner` (a runtime-arena allocator, `hilavitkutin-str/src/interner.rs:12-19`) into a pure compile-side pass, allocates arena space per fresh name, and runs a linear const-section scan per intern (`hilavitkutin-str/src/interner.rs:40-49`). It is more machinery, it allocates (the arena grows), and it makes a compile-time pass depend on a runtime interning concern. The gensym should be self-contained. Rejected.

**A de Bruijn or positional fresh-id scheme.** Convert binders to de Bruijn indices during ANF so freshness is positional. This fights the representation: the IR is name-based pre-resolution (`Var(Str)`, `Let { name: Str }`), and resolve runs after ANF and expects names, not indices (`vehje-ir/src/node.rs:87-96`). Converting to de Bruijn is a different IR and a different resolve, a far larger change than the pass this document scopes. Rejected as out of representation.

The recommendation is the reserved-bit synthetic `Str`. It is no-alloc, has no cap, needs no lexer coupling, changes no Core form, and makes capture structurally impossible by a bit-layout disjointness argument rather than a convention.

## Termination and correctness

Termination is structural and distinct from the redirect-chain argument. The redirecting passes terminate because the redirect chain strictly decreases in arena index (`vehje-lower/src/lib.rs:62-64`). ANF has no redirect chain; it terminates because the fold is structural recursion over the finite input tree. The decreasing measure is the size of the input subterm being normalized: `normalize_term` and `atomize` recurse only into strictly-smaller input subterms, and `atomize` calls `normalize_term` on its argument once and then wraps, adding no new recursion into the input. There is no iteration to a fixpoint and no convergence loop: ANF is a single pass, so the size-of-input-subterm measure decreasing to zero is the whole termination argument. The fresh-name counter is monotonic and never read back as a loop bound, so it cannot cause non-termination.

The fresh-node count is bounded (Section 5), so the append cannot run forever against an infinite arena; against a finite caller-lent arena it stops early with `Maybe::Isnt`, which is the correct no-alloc exhaustion behavior, not a hang.

Correctness rests on three invariants:

Effect order is preserved. Operands are atomized left-to-right and their `Let`s wrap in evaluation order (leftmost outermost, Section 4), so the residual evaluates subexpressions in the same order the input's strict left-to-right reading did. This is the property ANF exists to make explicit ("name every intermediate so effect order is explicit in the residual", `vehje-lower/src/lib.rs:382-383`).

Control containment is preserved. Hoists inside an if-branch, a match arm, an iter body, a lambda body, and a handled body stay inside that context (Section 3), so a subexpression that must run conditionally, repeatedly, under a binder, or under a handler is not floated to a context where it would run unconditionally, once, unbound, or unhandled. The branch/body cases are normalized as terms, not atomized, precisely to hold this line.

Binding identity is preserved. ANF never renames a source `Var`; it only introduces fresh names for anonymous intermediates. A hoisted subexpression that mentions `Var("x")` still refers to the same `x` binder it referred to in the input, because the source name is copied unchanged, and the fresh `t{n}` that names the intermediate is disjoint from every source name by the reserved-bit argument.

## Hygiene: why capture is structurally impossible

The panel flagged macro hygiene as the largest open soundness item (`tiark_rompf_staging-and-the-line.md:36-42`, `simon_peyton_jones_soul-audit.md:155-158`), and op adopted generative freshness (hygiene plus resume-freshness) as a canonical conviction (`op-ratification-answers.md:58-67`). ANF is the first client of that conviction; macro expansion is the second, and they share the mechanism.

The property: a fresh binder introduced by ANF (or by macro expansion) can never capture a source variable, and can never be captured by a source binder. The argument is direct from the design:

Resolve runs after ANF and after expansion. Name resolution binds `Var`s to `Let`/`Lambda` binders by `Str` identity, and it runs on the already-flattened, already-expanded residual, so it sees the fresh names as ordinary names and binds them by the same identity rule as source names.

The fresh-name space is provably disjoint from the source-name space. A synthetic `Str` has the reserved synthetic bit set; every source `Str` has it clear; integer equality on the 32-bit layout therefore never equates a synthetic handle with a source handle (Section 6). So a fresh `t{n}` cannot equal any source name `x`, which means a `Let t{n}` cannot shadow-capture a use-site `x`, and a use-site `x` cannot resolve to a macro-introduced `t{n}`.

The fresh-name space is internally collision-free. The counter is monotonic within one lowering, and both ANF and macro expansion draw from the same `FreshCounter`, so no two fresh binders across the two passes share an id, and one macro's fresh binder cannot capture another's.

Capture is thus impossible by construction, not by a renaming pass that could be forgotten or a convention a pathological input could violate. This is the same generative-freshness discipline Wingo connected to multi-shot resumption freshness (`andy_wingo_soul-double-take.md:154-160`): fresh binders so capture is structurally impossible, at compile time here and at each resumption there, one discipline at two binding times. ANF realizing it for the anonymous-intermediate case is what proves the mechanism before macro expansion depends on it.

CSE already carries a weaker version of the same reasoning: it refuses to share any subtree containing a `Var`, because two syntactic `Var("x")` can resolve to different binders and sharing would capture (`vehje-lower/src/lib.rs:199-203,227-238`). Macro expansion needs the stronger property (fresh binders, not just var-avoidance), which the panel noted CSE's discipline does not supply (`chris_fallin_weval-and-the-load-stage.md:25-26`). The gensym mechanism is that stronger property.

## Strict tests to write first

Per the fail-first discipline, these are written before the implementation, assert the intended behavior, and stay red until ANF builds. The capture-safety and containment tests are the theorem boundaries and are the most load-bearing.

1. **`anf_names_a_nested_apply_argument`.** Build `f(g(x))`. Assert the ANF root is a `Let` whose value is `Apply(g, [x])` and whose body is `Apply(f, [Var t])`, with `t` a synthetic binder. Pins the core hoist.

2. **`anf_leaves_atoms_untouched`.** Normalize `x` and `1`. Assert the returned root is the input node and the arena node count is unchanged (no appends). Pins the "non-trivial" predicate: atoms are not hoisted.

3. **`anf_is_idempotent`.** Assert `ANF(ANF(e))` is structurally equal to `ANF(e)`. Pins that already-atomic operands are not re-hoisted, so a second pass is a no-op.

4. **`anf_preserves_left_to_right_effect_order`.** Build `f(a(), b())`. Assert the residual is `let t0 = a() in let t1 = b() in f(t0, t1)` with `t0`'s `Let` outermost. Pins effect order.

5. **`anf_hoist_stays_inside_if_branch`.** Build `if c then h(y) else z`. Assert the `Let` binding `h(y)` sits inside the then-branch term and does not wrap the `If`. Pins control containment; a regression here is an unconditional-evaluation soundness bug.

6. **`anf_hoist_stays_inside_handled_body`.** Build a `Handle` whose body contains a compound effectful operand. Assert the hoist stays inside the handled body, not lifted past the handler. Pins the twelfth-form containment where ANF meets the effect discipline.

7. **`anf_fresh_binder_is_disjoint_from_source_names`.** Construct a program with a source binding whose name is the literal `str_const!("$t0")`, then run ANF so its first fresh binder has counter 0. Assert the synthetic binder's `Str` is not equal to the source `"$t0"` handle. This is the capture-safety theorem boundary: it must pass by the reserved-bit disjointness, not by lexical luck.

8. **`anf_returns_isnt_on_arena_full`.** Provide an arena too small for the ANF growth. Assert `apply` returns `Maybe::Isnt` and does not panic, truncate, or corrupt the input region. Pins the no-alloc exhaustion contract.

9. **`anf_output_within_capacity_bound`.** For a representative input, assert `nodes_out <= 3*N + 2*P` and `pool_out <= 2*P + max_arity`. Pins the stated bound so a regression that blows linear growth is caught, which is what lets callers size the region.

10. **`macro_expansion_fresh_binder_does_not_capture` (catalogue-red).** A capturing-macro scenario: a macro body binds `t` and the call site also has `t` in scope; assert the expanded fresh binder is disjoint from the call-site `t`. Marked `#[ignore = "catalogue: macro hygiene; blocked on MacroExpand gensym; tracked #<id>"]`, left red per the catalogue-edge-cases discipline. It is the same mechanism as test 7 at a different binding time, and the panel already named it as owed (`simon_peyton_jones_soul-audit.md:211`).

## Calls the maintainer must make

1. **`hilavitkutin-str` synthetic-`Str` API.** The recommended gensym adds `Str::__synthetic` and `is_synthetic` and claims one reserved flag bit (`hilavitkutin-str/src/handle.rs:22-24`). This is a small upstream `hilavitkutin-str` design round. Confirm the reserved bit is available for vehje's use and that the interner's `resolve` learns the synthetic case. If the reserved bits are wanted for something else, fall back to the `Binder` sum type (Section 6), accepting the larger blast radius.

2. **`Lower::lower` signature.** The node-creating stratum requires mutable arena access, so `Lower::lower` takes a `&mut Builder` for the ANF stretch instead of a `&Arena`, and threads one `FreshCounter` through `MacroExpand` and `Anf` (`vehje-lower/src/lib.rs:432-459`). Confirm the orchestration layer can hand ANF a builder over the same arena the redirecting passes read.

3. **`Raw` payload evaluation semantics.** Whether a `Raw` payload child is an eager evaluation position (atomize and hoist it) or an opaque family handle (leave it) is family-dependent (`vehje-ir/src/node.rs:125-130`). The conservative default is opaque (normalize inside, do not hoist across the boundary). If families need eager `Raw` operands ANF'd, this becomes a `Family` hook. Decide the default or add the hook.

4. **The capacity constant.** The bound is linear (`3*N + 2*P` nodes, `2*P + max_arity` pool); the exact constant a caller provisions is bench-tunable. Pick a safe default (the design suggests `4*N` nodes, `3*P` pool) and let a later bench tighten it.

5. **Pass ordering: a second CSE after ANF.** ANF introduces new named structure that a CSE pass could share further. Running CSE after ANF still terminates (CSE only redirects to smaller indices). Decide whether the extra sharing is worth a second CSE pass, or whether ANF-last is the final order (`vehje-lower/src/lib.rs:442-449`).

6. **N-buffer arena fork.** The single-arena default is the hypothesis; the N-buffer variant carries the `NodeRef` cross-space branding decision (`op-ratification-answers.md:69-74`). This design keeps the single-arena default and localizes the branding seam (Section 5) so the later bench is a contained change. Confirm the single-arena default is the right first landing and the bench is deferred, not foreclosed.
