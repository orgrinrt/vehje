# Weval and the load stage: what the runtime's compile stage actually is, and where the topic already drifts from its own round's canon

**Author lens:** instruction selection and verified lowering, register allocation, e-graph mid-end design,
partial evaluation as an engineering tool (weval).
**Read:** the source cited below, both fully (not summarized), the topic, the round's closing consolidation
topic (`202607240130/202607240100_topic.consolidated-design-and-taxonomy.md`) and its locked doc/src
changelists, the bench-remeasure synthesis, and the prior-art synth docs under `mock/research/`.

## One-line reading

The IR-to-IR reframing is the right vehicle, but the crate that would make its fixpoint safe already exists
(`vehje-fixpoint`) and is not wired to it; the `NodeRef` type that must carry two live arenas at once carries
no arena identity, so the double-buffered emit path is a type hole, not just an ops burden; the "residual"
that is supposed to be a CFG of blocks is, in the only code that actually runs, a flat re-serialization of the
twelve Core forms with the block and function tables present as dead type surface; and the topic under review
right now already contradicts the round's own closing statement, which locked "no unbounded saturation" for
exactly the stratum the topic reopens as an unbounded fixpoint.

## Where I agree with Rompf

The vehicle correction (`Rewrite` cannot create nodes, emit-into-a-fresh-arena can) is right, and his read of
`fold_core` as the shared traversal shape is the right instinct even though (finding 4 below) the concrete
signature does not transfer as-is. The four-point `BindingTime` lattice as the real axis, and the framing that
"the compile/runtime line" is malformed as a single line, is correct and is where I spend the least time
because there is nothing to add: `grade.rs:105-115` is exactly what he says it is. The macro-hygiene finding
(macro expansion needs fresh binders, not just CSE's variable-avoidance) is real and unaddressed anywhere in
the topic or the design; I have nothing to add there either, and it stays the single largest open soundness
item in his list and mine. The tier-narrowing verdict (native off the tier axis, onto the spectrum) is right,
independent of the bench-number correction in Finding 6.

## Where I depart, specifically

**On finding 2 (`tiark_rompf...md:28-31`, the fixpoint and equality saturation are "the same operation at two
precisions").** This undersells what exists. `vehje-fixpoint` (`mock/crates/vehje-fixpoint/`) is not a
deferred idea, it is a committed, shipped crate: `Engine` runs a semi-naive-shaped relational fixpoint to
completion (`engine.rs:149-177`), `Congruence` is a union-find with iterative path halving
(`congruence.rs:29-37`), and `Incremental` does content-addressed differential maintenance
(`incr.rs`). The round's own closing statement names it as the intended home for exactly the equality-saturation
stratum: "the equality-saturation lowering in `vehje-lower` (later)" is client #2 of three
(`202607240130/202607240100_topic...md:280-289`), and `vehje-lower`'s `Cargo.toml` already depends on
`vehje-fixpoint` (`mock/crates/vehje-lower/Cargo.toml:8`). None of that is used: `vehje-lower/src/lib.rs` has
no `use vehje_fixpoint`, and `LowerStrategy::Saturate` (`lib.rs:76-86`) falls through to `Cheap`
(`lib.rs:455-457`) with a bare `FIXME`. So the "same operation at two precisions" framing is correct as a
design intuition but wrong as a statement about what is missing: the missing piece is not a design decision
about whether to unify them, it is wiring an already-designed, already-typed engine to an already-declared
dependency. That is a much smaller and more concrete task than "decide whether to unify the two fixpoints."

But the unification itself needs a caveat Rompf does not draw, and it matters for finding 1. `Engine::evaluate`
terminates because the relations are **finite-height lattices under monotone insertion into caller-bounded
columnar storage** (`engine.rs:6-8`, "the relations are finite-height lattices and the rules are monotone, so
the loop terminates"). A macro-expansion or const-fold fixpoint over an **arena of terms** is not that shape:
it does not grow a bounded set of facts toward a ceiling, it emits a fresh, potentially larger arena each
round, and "smaller" is not guaranteed by anything in the type. Calling both operations "fixpoint" is true in
the pure sense (repeat until stable) and false in the load-bearing sense (bounded by construction). If the
Saturate stratum is built as a `vehje-fixpoint` client, it needs its own relation shape (e-classes as tuples in
a bounded column, congruence merges as the monotone rule) so it inherits the real termination argument; if the
IR-to-IR macro/const-fold fixpoint stays a `Builder`-emitting loop over arenas, it does NOT inherit
`vehje-fixpoint`'s termination guarantee just by being called a fixpoint, and Rompf's fuel-cap direction
(finding 1) is still exactly right for that loop specifically, independent of whatever `vehje-fixpoint` proves
for its own clients.

**On finding 6 (`tiark_rompf...md:48-50`, `FamilyExpand`'s `Maybe<NodeRef>` conflates "passed through" with
"arena full").** Correct, and the same conflation is already shipped one layer down, in the crate meant to
host the saturation stratum: `RelationSet::insert(&mut self, tuple: T) -> Bool` (`vehje-fixpoint/src/engine.rs:66-68`)
returns `Bool::TRUE` for "newly added" with no distinct signal for "the caller's fixed-capacity column is
full and I silently dropped it." The trait contract does not even require an implementer to check capacity
before writing (the test `ArraySet::insert`, `engine.rs:223-234`, indexes `self.data[self.len.0]` with no
bounds check at all, which is fine as a test fixture but is also the only extant model of the contract). If
`vehje-fixpoint` becomes the Saturate stratum's engine, "the e-graph ran out of room" and "the e-graph
converged" become the same observable outcome under the current trait, which is precisely the arena-exhaustion
ambiguity Rompf flags for `FamilyExpand`, one layer earlier in the pipeline than he looked. Fix both at once:
`RelationSet::insert` should return a three-way result (`Inserted` / `Present` / `Full`), matching whatever
shape `FamilyExpand` adopts, because they are the same "caller-bounded growable set" contract with the same
failure mode, and a fixpoint driver built over one and not the other is a fixpoint that reports success on
starvation half the time.

## My findings

### Finding A: `NodeRef` carries no arena identity, which is the actual soundness gap in the two-arena architecture

The topic's whole mechanism (`topic:16-29`) is: read from an input arena, emit into a fresh output arena,
iterate. `NodeRef` (`vehje-ir/src/node.rs:13-31`) is `#[repr(transparent)] struct NodeRef(USize)`, a bare
index with no phantom lifetime, no invariant-lifetime brand (the `generativity`/`GhostCell` trick), no
type-level tag naming which `Arena` it was minted from. `Arena::get` (`arena.rs:74-76`) is
`self.nodes[at.index().0]`, an unchecked slice index, not even the `Maybe`-guarded read `list()` uses two
lines later for pool ranges (`arena.rs:89-97`). Concretely: an expansion pass holding an input arena `A` and
building an output arena `B` through the same `Builder` API can, by a simple bug (forgetting to recurse into a
child and passing the input-arena `NodeRef` straight through to `Builder::let_`/`if_`/`apply`, which all just
take `NodeRef` params with no arena tag) construct a `B` whose child indices happen to be in-bounds for `B`'s
node array but denote a *different, unrelated node* than the one that was in `A`. This does not panic. It does
not return `Maybe::Isnt`. It compiles, and it silently corrupts the output program with a plausible-looking but
wrong node, which is a worse failure than any budget-exhaustion diagnostic in Rompf's finding 1, because it
never surfaces as a diagnosable event at all. This is precisely the class of bug ISLE's typed term
representation and Cranelift's arena-indexed `Value`/`Inst`/`Block` handles were built to make unrepresentable
by construction (each entity kind is its own newtype, and a builder for one function's DFG cannot accept an
entity ref from a different function's DFG without an explicit, checked cross-reference). vehje's `NodeRef` is
one step short of that: it is a newtype over the index (good), but it is not a newtype **per arena instance**
(missing).

Why it matters: this is not a hypothetical. The IR-to-IR expansion is exactly the place where two arenas of
the same node-shape live concurrently and a `NodeRef` must move between them under a fixpoint loop that a
family author's `FamilyExpand` hook also participates in (a third-party implementer, per the framework's own
extension discipline, not just framework-internal code). The harder-to-audit party is exactly the one most
likely to make the mistake.

Direction: brand `NodeRef` (and `NodeList`) with an invariant lifetime tied to the `Arena` that minted it, the
same generativity pattern `GhostCell`/`qcell`/`generativity` use for a single-owner arena, or at minimum a
const-generic or type-level "arena epoch" tag threaded through `Builder`/`Arena` so a `NodeRef` from arena `A`
is a different Rust type than a `NodeRef` from arena `B`. If a full invariant-lifetime brand is too heavy for
the no-alloc, no-generic-const-cost budget, the cheap fallback is a debug-only arena-id field on `NodeRef`
(zero-cost in release, an assert in `Arena::get` in debug) so the bug is at least catchable in the test suite
this project already leans on heavily. Either way, catalogue this now as a red test per the workspace's own
edge-case discipline: construct two same-shaped arenas, mint a `NodeRef` from the first, feed it to a `Builder`
building into the second, and assert the current code either accepts a corrupt result silently (documenting
the hole) or, once branded, refuses to compile.

### Finding B: `fold_core` is a top-down visitor with no return channel; it does not compose as the emit fold the topic describes

`vehje-codegen::fold_core` (`vehje-codegen/src/lib.rs:86-134`) has signature `fn fold_core<F: FnMut(&Node)>(arena:
&Arena<'_>, at: NodeRef, visit: &mut F)`. It calls `visit(&node)` on the way down and recurses into children;
`visit` returns `()`. That is the right shape for "fold IR to bytes" (a `ByteEmitter` sink accumulates as a
side effect of the walk, per `vehje-runtime-abi`'s `encode()`, which is the same top-down shape). It is the
wrong shape for "fold IR to IR," which the topic states as the parallel case (`topic:17-19`, "the same
fold-and-emit machinery the codegen already uses in the output direction"). An IR-to-IR rebuild is necessarily
**bottom-up with a return value threaded upward**: to call `Builder::let_(name, value, body, span)` on the
output arena you need the *already-rebuilt* `NodeRef`s for `value` and `body` in the output arena, which only
exist once their own fold has run and returned a handle. `fold_core`'s `visit: &mut F` where `F: FnMut(&Node)`
has nowhere to put that handle. This is not a nitpick: it means the IR-to-IR expansion pass needs a genuinely
new primitive (a `map_core`-shaped fold: `fn map_core<F: FnMut(&mut Builder<'_>, &Node, /* already-mapped
children */) -> Maybe<NodeRef>>(...) -> Maybe<NodeRef>`, i.e. a catamorphism, not the existing visitor), and
claiming it reuses `fold_core` (as the topic's prose does) will send whoever implements the fixpoint driver
down the wrong path, trying to bolt a return channel onto a signature that structurally cannot carry one
without becoming a different function. Name the new fold now, in the doc CL, as its own function
(`vehje-ir` or `vehje-codegen`, one definition, shared by const-fold, `FamilyExpand`, and `Anf`, all three of
which are catamorphisms over the same twelve forms), rather than discovering three ad hoc bottom-up walks that
each reimplement the same recursion shape `fold_consts`/`cse_share` in `vehje-lower` already show signs of
duplicating (`vehje-lower/src/lib.rs:134-178`, `219-279`, both hand-written bottom-up walks over the same
twelve-form match, structurally identical except for payload). That duplication is exactly what the codebase's
own single-definition discipline (ISLE's whole reason for existing, in miniature: one term-walk, many rules,
not N hand-written walks) says to collapse.

### Finding C: the "residual" is a CFG of blocks in the design and in the round's canon; in the only code that runs, it is a flat re-tagging of the twelve Core forms, and the block/function tables are unreferenced dead type surface

`vehje-runtime-abi/DESIGN.md.tmpl:18-21` and the round's closing consolidation
(`202607240130/202607240100_topic...md:115-124`, "the interpreter is a CFG of straight-line blocks over the
Core forms... only the terminator touches control flow") both commit, explicitly and recently (the
consolidation is dated the same day as the topic under review), to the runtime tier interpreting a
block-and-terminator program, not a tree. `wire/residual.rs` accordingly defines `Block`, `BlockId`,
`BlockTable`, `TerminatorKind`, `Function`, `FunctionTable`, and `Residual<'img>` carries `blocks:
BlockTable<'img>` and `functions: FunctionTable<'img>` as non-optional fields (`residual.rs:262-265`).

None of it is constructed. The one encoder that exists and is tested, `FlatArenaEncoder` via `encode()`
(`vehje-runtime-abi/src/wire/serialize.rs`, `src/encode.rs`), assigns a tag code to each of the twelve Core
forms directly (`serialize.rs:54-68`, `Let => 2`, `Lambda => 3`, `If => 6`, `Iter => 8`, and so on) and writes
each node as a fixed seven-word record with direct child indices (`serialize.rs:186-238`), which is a
flat, tree-shaped, IR-preserving re-serialization, exactly as `encode.rs`'s own module doc says: "An encoder
materializes the IR *as the IR*... distinct from codegen, which abandons the IR by lowering it into a foreign
representation" (`encode.rs:1-9`). There is no block, no terminator, no function anywhere in this path.
`grep -n "BlockTable::new\|FunctionTable::new\|Block {" mock/crates/vehje-runtime-abi/src/` returns exactly one
hit, the struct definition itself. `Tier::Arena` is described in the dispatch deep dive as "the flat serialized
IR arena and the reference semantics every other tier is checked against"
(`vehje-runtime-driver/DEEPDIVE_DISPATCH_AND_DECODE.md.tmpl`, the Dispatch section), which is honest about what
ships, but it is the opposite structural claim from "a CFG of blocks." `Tier::Bytecode` is named, in the same
document, as "an optimized linear form whose encoder is not yet built," which is where a real block/terminator
lowering would have to live.

Why it matters: this is the actual site of the "runtime contains a compile stage" debt, and it is a harder
debt than either the topic or Rompf's deliverable credits. Replaying the shared `RuleTable` (const-fold, CSE)
at `HostLoader` time, which Rompf correctly identifies as the load-time specializer, gets you a re-folded
*tree*. It does not get you a CFG. Going from a recursive, closure-and-lambda-bearing term (`Let`/`Lambda`/
`If`/`Apply`/`Iter`, no explicit jump, no explicit basic block anywhere in `vehje-ir::Node`) to "a maximal
backward-only-child-index straight-line region... terminators... back-edges with block-argument-passed
loop-carried values... a frame stack" (the consolidation's own language) is closure conversion plus explicit
control-flow construction, the front half of a real bytecode compiler. That is categorically different work
from const-fold/CSE/macro-expansion, has no owning crate in the twelve-crate taxonomy (`vehje-runtime-gen`
composes slices, it does not lower a tree to a CFG; `vehje-lower` is scoped to const-fold/CSE/ANF/macro,
none of which produce blocks), and is currently unspecified: no design doc states which binding time performs
it, whether it happens once in Rust and ships as part of the Tier-0 residual, or happens in Zig at load, or is
in fact scoped to `Tier::Bytecode` alone and `Tier::Arena`'s interpreter walks the tree directly (in which case
the `Block`/`Function` fields on `Residual` are simply wrong for `Tier::Arena` and should be `Maybe<BlockTable>`
or absent under that tier, not unconditional fields on every `Residual`).

Direction: this is a design question that needs answering before the IR-to-IR fixpoint driver ships anything
downstream of it, not after. Two honest resolutions, and the design owes a choice between them: (a) `Tier::Arena`
never has real blocks; it is a tree-walking interpreter over `Let`/`If`/`Apply`/`Iter` directly (defunctionalized
into an explicit stack per the citation in the addendum below), and `Block`/`Function`/`TerminatorKind` are
`Tier::Bytecode`-only types that should move out of the unconditional `Residual` struct into a tier-gated
variant, or (b) `Tier::Arena` genuinely does require a CFG, in which case the tree-to-CFG lowering is a
real, named, owned pass (most naturally `vehje-lower`'s job, since it already sits at the point where the
program's final tree shape is fixed, or a new crate if the round's "each stage owns its slice" discipline says
otherwise) and belongs in the taxonomy with the same weight as const-fold. Either way, mark the gap right now:
there is no `// FIXME:` anywhere near `Block`/`BlockTable`/`Function`/`FunctionTable` in `residual.rs` or
`encode.rs` saying "unpopulated, tier-0 does not build these yet," which the workspace's own placeholder
discipline requires at exactly this kind of spot. Add it before the next agent reads `Residual` and assumes the
CFG exists because the field does.

### Finding D: macro expansion's binding-time pin needs a concrete home, and `vehje-signature::Operation` is where it is missing

Rompf's finding on macro binding time (a family macro operation must be pinned to a binding time no later than
`Bundler`, because `FamilyExpand` is Rust code and Rust never runs in the runtime) is right, and it names the
right soundness statement. It needs a place to live as data, not prose. `vehje-signature::Operation`
(`vehje-signature/src/lib.rs:22-30`) already carries `family: FamilyId`, `effect: EffectMask`, `lease:
LeaseRule` per operation, the graded algebraic-signature record the check pass reads. There is no
`binding_time_ceiling: BindingTime` (or equivalent) field here, and there should be: a language author
declaring a macro operation declares, in the same record the effect and lease rule already live in, that this
operation's binding time cannot exceed `Bundler`. That turns Rompf's soundness statement into a checkable fact
at the same layer the effect and lease facts already discharge (`vehje-typecheck::check`, which already reads
`vehje-signature`'s schema per its own module doc, `vehje-typecheck/src/lib.rs:8-9`), rather than a convention
the `FamilyExpand` hook's implementer has to remember unassisted.

### Finding E: the fixpoint driver's fuel/budget question is already answered by a bench, not merely open

`mock/research/202607220300_bench-remeasure-synthesis.md`'s "Eqsat" finding (`sed -n` above,
"scale-runner eqsat") measured a real e-graph, bounded(512 e-nodes) against unbounded, over a reassociation
chain designed to explode it: unbounded runs 2.5k to 399k e-nodes (K=8 to 14) and 9.7ms to 3004ms, hitting a 2M
node ceiling past K=18; bounded stays at ~512 e-nodes and **extracts the identical optimal cost**, 17x to
17000x faster, cross-validated on the extracted value, not just the wall-clock. This directly answers Rompf's
open question 1 (the fuel cap's shape): the mechanism is not a round-count fuel bound (which only bounds the
number of fixpoint iterations, not the size of what each iteration produces), it is a **node-count cap on the
working set itself**, already measured to cost nothing in solution quality on the pathological case built to
break it. The IR-to-IR macro/const-fold fixpoint driver should adopt the same shape: cap the output arena's
node budget (which the arena already has, as a hard capacity, so this is "refuse further emission past the
caller's declared size" rather than a new mechanism) and treat "budget exhausted before the fixpoint settled"
as the diagnosable outcome Rompf's finding 1 wants, with the evidence that a modest bound does not cost
quality already sitting in the repo.

### Finding F: the bench numbers both the topic's authors and Rompf's deliverable would reach for are stale, and one is stale in the round's own closing canon

`vehje-codegen/DEEPDIVE_OUTPUT_SPECTRUM.md.tmpl:81-92` states "the native ceiling is 1.0x to 1.5x over a good
interpreter for scalar code," and Rompf's deliverable cites this figure as settled evidence
(`tiark_rompf...md:82`, `84`). It is retracted. `202607220300_bench-remeasure-synthesis.md`, "1. Native
ceiling," dated two days before the round's own closing consolidation topic, found the old 1.0-1.2x number was
native-vs-native (the interpreter's two-instruction test program was comptime-const-folded away, so the
"interpreter" side was partially native too); re-measured with the program crossing FFI opaque, a switch
interpreter is **~2.0x** a shape-specialized native loop, a function-pointer-table interpreter ~2.2x, stable
across three orders of magnitude, with a cost-model IPC sanity check the doc calls "physically real." The old
1.2x headline is explicitly retracted in that doc's own words. And yet the round's closing consolidation topic,
written two days later and explicitly framed as "settled and not reopened"
(`202607240130/202607240100_topic...md:412-420`), still lists "the native 1.0x-to-1.5x ceiling" as one of "the
bench-backed corrections" it locks in. The retraction is sitting in the same `mock/research/` tree the
consolidation topic cites elsewhere and did not fold in.

This matters for the panel's actual question. A corrected ~2x, cost-model-confirmed ceiling is a meaningfully
stronger case for spending real engineering effort on a per-region native path than a 1.0-1.5x "mostly noise"
number would justify; it does not change the tier-narrowing verdict (native still does not belong as a whole-
program tier tag, per-region selection is still right), but it changes how much weight the "fold more, earlier,
skip native" recommendation (Rompf's open question 6) should carry. The switch-vs-tail-threading number has the
same problem in miniature: the remeasure synthesis's item 3 found the original "switch beats tail-threading
1.7x" comparison was a toolchain-ABI artifact (Zig 0.16's `always_tail` uses the standard calling convention,
not `preserve_none`, so the comparison never tested what Deegen-style dispatch actually claims), and the
corrected range is 1.5x to 4x depending on regime, with a fair Deegen-style test explicitly named as blocked on
`preserve_none` support landing in the toolchain, not settled. Cite the corrected numbers, and file the doc fix
(`DEEPDIVE_OUTPUT_SPECTRUM.md.tmpl` and the round consolidation's "settled" list both need the correction) as
its own small round; a bench evidence retraction that never reaches the design doc that quotes it is a doc
staleness the mockspace `design-doc-source-mismatch` lint has no way to catch, because the mismatch is doc-vs-
evidence, not doc-vs-source.

## The compile/runtime line, pinned

I don't have a disagreement with Rompf's core pinning (AOT at `LanguageAuthor` time via `vehje-runtime-gen` and
the Zig `comptime` specialization; AOT at `Bundler` time for the statically-known Rust compile side; load-time
compilation, not tracing/profiling JIT, at `HostLoader` time for runtime-arriving scripts; interpretation at
`Runtime` time as one point on the output spectrum, orthogonal to when specialization happened). What I add,
from Finding C, is that "load-time compilation at `HostLoader`" is doing more work than the design currently
names. It is not one specializer (the shared `RuleTable`) replayed at a second binding time; it is, at minimum,
that specializer plus an entirely separate, currently-unowned tree-to-CFG lowering if `Tier::Arena` genuinely
needs blocks, or it is nothing more than the tree replay if `Tier::Arena` is (and should be documented as) a
tree-walking interpreter and the CFG only exists for a `Tier::Bytecode` that "is not yet built." Until that
question is answered, "the runtime contains a compile stage" is a debt whose size is not yet known, which is a
different and more urgent status than "known debt, mechanism named." Interpretation is still coherent as a
full end state once constants are inlined (Rompf's core claim survives this addition intact); what changes is
that the AOT-vs-load-time boundary for the *shape* of the residual, not just its content, is still open.

## Open questions handed forward

1. Wire `vehje-fixpoint::Engine`/`Congruence` into `vehje-lower::LowerStrategy::Saturate` as the first real
   client, with a relation shape for e-classes that inherits the finite-height-lattice termination argument
   rather than reusing arena-emission's undecided one (Finding, "where I depart," item 2).
2. Fix `RelationSet::insert`'s three-way conflation alongside `FamilyExpand`'s (both need `Inserted`/`Present`/
   `Full`, not `Bool`/`Maybe`), since a shared engine with two callers each hiding "I ran out of room" differently
   is worse than either alone (Finding, "where I depart," item 2; Rompf's finding 6).
3. Brand `NodeRef`/`NodeList` to their minting `Arena` (Finding A) before the IR-to-IR fixpoint driver ships;
   write the cross-arena-confusion red test now regardless of when the fix lands.
4. Name the actual bottom-up catamorphism primitive (Finding B) in the doc CL rather than reusing `fold_core`'s
   name for a differently-shaped function; unify `fold_consts`/`cse_share`'s duplicated recursion under it.
5. Settle whether `Tier::Arena` ever needs a CFG (Finding C). If yes, name the owning crate and the binding time
   the lowering runs at. If no, move `Block`/`Function`/`TerminatorKind` off the unconditional `Residual` struct
   and mark `Tier::Bytecode` as where they belong, with a `// FIXME:` at the current dead type surface either way,
   today, regardless of which answer wins.
6. Add `binding_time_ceiling` (or equivalent) to `vehje-signature::Operation` so the macro binding-time
   constraint is a checked fact, not a convention (Finding D).
7. Adopt the node-count-capped fixpoint shape the eqsat bench already validates (Finding E) rather than
   re-deriving a fuel policy from first principles.
8. File the doc correction for the retracted native-ceiling and dispatch numbers (Finding F), in both
   `DEEPDIVE_OUTPUT_SPECTRUM.md.tmpl` and the round consolidation's "settled" list, since the second one is a
   locked canonical statement citing evidence its own tree already retracted.

---

## Addendum 1: efficiency and prior-art-driven wins from the synth corpus

Op asked me to mine `mock/research/*_synth_*.md` for citations that answer some of the above with prior art
rather than fresh invention. Five load directly onto this panel's questions.

**CSE by construction, not by pass, is not speculative: it is LMS's actual mechanism, and vehje-ir already has
the one piece it costs.** `202607201618_synth_staged-metaprogramming-and-codegen.md` ("Two mechanisms,
repeatedly confused as one," lines 21-32): in LMS, "CSE is not a pass... because every IR node `Def` is built
from already-staged operands, two structurally-equal constructions compare `==`, and `findOrCreateDefinition`
reuses the existing symbol on a hit. CSE falls out of case-class structural equality plus a lookup on
construction, with zero extra bookkeeping." The doc's own stated tax for a no-alloc port: "an explicit interner
(arena plus hash-consing table)." vehje already has exactly that interner (`vehje-ir::hash_of`, the
`StructuralHash` in `hash.rs`) and `vehje-lower::Cse` already does the lookup, just as a second pass over an
already-built tree rather than as a check inside `Builder::push` itself. This is independent confirmation of
Rompf's open question 4 (collapse CSE into a hash-consing `Builder`): it is not a novel proposal, it is the
one-truth-many-projections pattern this whole prior-art corpus keeps landing on, with a primary citation and a
known cost model (Filliatre and Conchon's hash-consing, which Rompf already named). Fold the hash-cons lookup
into `Builder::let_`/`apply`/`if_`/etc. directly (return the existing `NodeRef` on a structural hit instead of
appending) and `Cse` as a standalone pass becomes unnecessary for anything built through `Builder`, which is
exactly the IR-to-IR emit path this whole topic is about. Bench it before locking (per the workspace's own
bench-decided-fork discipline), but the a priori case from LMS is strong: the cost is one hash-table probe per
`Builder` call, already paid today by `Cse`'s separate walk, just moved earlier and made unconditional.

**The specialization-happens-ahead-of-time argument is not vehje's invention either, and it is the reason the
two-artifact model is the correct shape rather than a compromise.** The same synth doc's "Runtime code
compilation, and why it usually does not transfer" section states LMS's `compile[A,B]` calls the live Scala
compiler at runtime to get a callable back in-process, and flatly: "there is no analogue 'load bytecode back
into the same process' step for a statically-compiled, no-alloc target... the specialization has to happen
ahead of time, and only the specialized artifact ships (the Terra `saveobj` shape, not the LMS `compile`
shape)." This is worth stating explicitly to whoever revisits the two-artifact model later: it is not a
concession vehje makes because Rust-in-the-runtime is forbidden by fiat, it is the same conclusion every
no-alloc, ahead-of-time-shipping staged system in this corpus reaches, for the same structural reason (no
hosted compiler, no classloader, nothing to load bytecode back into). Cite it when the two-artifact model is
next questioned.

**Defunctionalization is the citable, mechanical answer to "how do I turn this recursive pass into the
iterative, no-recursion-on-the-runtime-side kernel," not an ad hoc rewrite done once per pass.**
`202607201854_synth_binding-time-staging-and-type-preserving-compilation.md`'s defunctionalization section
names Reynolds 1972, Danvy and Nielsen's "Defunctionalization at Work" (PPDP 2001), and Ager, Biernacki, Danvy,
and Midtgaard's "A Functional Correspondence between Evaluators and Abstract Machines" (PPDP 2003): closure
conversion, CPS transform, and defunctionalization of the continuation mechanically turn a recursive evaluator
into an explicit-stack abstract machine, with a proof the two compute the same answer. `vehje-fixpoint`'s own
`congruence.rs:1-8` already states the constraint this answers ("the closure is iterative... never recursive,
because recursion is a hard compiler wall on the runtime side") but arrives at the iterative shape by hand,
per mechanism, with no shared derivation discipline. Every future recursive pass this framework writes (the
lease/reachability walk, a future CFG-construction pass from Finding C, the deferred e-graph extractor) is
going to hit the same "no recursion in Zig" wall and needs the same treatment. Naming defunctionalization as
the *standard, citable derivation technique* rather than re-deriving an ad hoc work-stack each time is the
actual efficiency win here: it turns "did we get the iterative version right" from an unaudited per-pass
judgment call into "does this follow the mechanical recipe," and the recipe comes with a 50-year-old
correctness proof attached.

**Certifying compilation, not certified compilation, is the honest name for what the graded proof spine
already does, and it resolves an ambiguity in the `Assurance` lattice.** `202607201854_synth...md`'s "Certifying
versus certified compilation" section (Necula and Lee, PLDI 1998) draws exactly the line vehje needs:
a *certified* compiler (CompCert) is proven correct once, for every input; a *certifying* compiler emits, per
compilation, a checkable certificate that this particular output is correct, re-verified per instance, trusting
the (small) checker rather than the (large) generator. `vehje-ir::Assurance` (`grade.rs:174-189`) already has
`StaticCertified`, `StructurallyCertified`, `ReprovenPerInstance`, `DynamicallyChecked`, `Tested`, `Unclaimed`
as its six variants, and `ReprovenPerInstance` is, precisely, Necula-Lee's certifying-compiler case. Naming this
in the DESIGN doc gives future readers (and the `lambda_veh` proof document the round's consolidation names as
owed) the exact prior-art hook to state what "termination is not claimed" (the consolidation's own honest
scoping, `202607240130/202607240100_topic...md:65`) means without sounding like an oversight: it is the
standard, principled scope of a certifying compiler, which never claims a whole-program metatheorem, only a
per-instance re-checkable one. This is a documentation and framing win, not a code change, but it is the kind
that prevents the next reader from mistaking honest scoping for a gap.

**The lens-law framing gives the shared `RuleTable` a concrete, testable correctness obligation that currently
does not exist anywhere as a stated property.** `202607201854_synth...md`'s bidirectional-transformations
section (Foster, Greenwald, Moore, Pierce, Schmitt, TOPLAS 2007) states the get-put/put-get well-behavedness
laws for one source projected two consistent ways. `RuleTable` (`vehje-lower/src/lib.rs:94-113`) is exactly
this: the same const-fold-plus-CSE rule data crosses to both the Rust compile side and, per its own doc comment
("the same table `vehje-runtime-gen` packages for the runtime's load-time lowering"), the Zig runtime. Nowhere
is there a stated, testable property that says "running the Rust-side `Lower::cheap()` over a program and
running the Zig runtime's replay of the same `RuleTable` over the same program produce structurally-equal
results." That is a lens law (put-get, in this framing: specialize then decode agrees with decoding the
already-specialized form), and it is exactly the kind of property the workspace's own catalogue-edge-cases-as-
tests discipline says to pin now, even if the Zig side does not exist yet to run it against: write the property
as a cross-language differential test target today, red until the Zig replay exists, and it becomes the
concrete acceptance gate for "the two specializers do not drift," which the current design only asserts in
prose ("`RuleTable` being the same data on both sides... keeps the two specializers from drifting," per
Rompf's own reading, correctly, but as an assertion rather than a checked property).

## Addendum 2: auditing `crates/` and the current design state against the round's own canonical close

Op asked for a specific audit: read the last topic files and the changelists of
`mock/design_rounds/202607240130/` (the round whose closing statement, `202607240100_topic.consolidated-
design-and-taxonomy.md`, is the round's own stated canon, locked the same day this panel's topic opened) and
check the current `crates/` and design state, including the topic under review, for drift from it.

**The topic under review already contradicts the round's own locked constraint for the exact stratum it
touches, one topic later.** The consolidation states, as one bullet inside "the full architecture, stage by
stage": "the cheap runtime subset (const-fold plus CSE as a bounded single bottom-up pass with hash-consing, a
fixed rule set, **no unbounded saturation**, no alloc) runs at load for arriving scripts"
(`202607240130/202607240100_topic...md:94-97`, emphasis mine), and repeats the same constraint in "settled and
not reopened" (`:418-419`, "the bench-backed corrections... iterative not recursive kernels" sits beside "the
maximal shape as the target" and the two-strata split as things this round locked). `202607241330_topic.ir-to-
ir-expansion.md`, opened the same day, reframes exactly this stratum ("Core const fold... needs no consumer
family and is built now," `topic:39-41`, i.e. the cheap subset) as an **unbounded fixpoint**: "the passes
iterate to a fixpoint: run expansion, and if it changed the IR, run it again on its own output, **until a run
produces no change**" (`topic:36-38`), with no stated bound anywhere in the topic. This is not a subtle
tension. The round that closed hours earlier locked "no unbounded saturation" for precisely the runtime-load-
time, cheap-subset stratum, and the very next topic reopens unbounded iteration inside that same stratum
without citing or reconciling the constraint it just broke. Per `canonical-design-outranks-intermediate-rounds
.md`, the fresh topic is the thing to correct, not the canon: either the topic needs an explicit, stated fuel/
node-budget bound (which both Rompf's finding 1 and my Finding E already supply the mechanism for) so it stops
being "unbounded" in fact even though the prose says "iterate to a fixpoint," or the topic needs to explain why
the round's own "no unbounded saturation" line does not apply here (I do not see an argument for that reading;
the topic reads as reasoning from the immediate prior audit and op's live correction about the `Rewrite`
vehicle, not from the round's closing statement, exactly the drift pattern the workspace rule describes). The
fix is cheap and is already implied by Rompf's finding 1 and my Finding E: state the bound in the doc CL that
follows this panel, and the contradiction dissolves into "a bounded pass, which happens to iterate a small,
capped number of rounds," which is what the round actually locked.

**The mandated `vehje-typecheck` to `vehje-check` rename is honestly tracked, not silent drift, and is still
outstanding.** The src changelist explicitly attempted and reverted it (`202607240200_changelist.src.lock.md:
36-44`, "the mandated cosmetic dir/package rename was attempted and REVERTED: moving a crate's doc templates...
is blocked by the `changelist-doc-gate` and `changelist-lock` lints in IMPL phase... FLAG FOR OP") and the crate
still lives at `mock/crates/vehje-typecheck/` with its own module doc already narrating the target name
("vehje-typecheck, the framework's graded check pass," `vehje-typecheck/src/lib.rs:1`; `vehje-ir/src/grade.rs:9`
independently refers to "a `vehje-check` pass" as though the rename had already happened). This is not drift in
the sense the round's canon warns about (nobody reasoned past the constraint); it is a known, gate-blocked,
explicitly flagged deferral. I confirm it is still open and note it only so it does not silently fall off a
future audit's radar: it needs a doc-phase round of its own, per the src CL's own note, before the next major
consolidation, or the crate name and the prose describing it will drift further apart with every passing round.

**The CFG-of-blocks interpreter, locked as settled in the same closing statement, has no implementation and no
owning pass, and the type surface for it (`Block`/`Function`/`TerminatorKind`) is unreferenced by the only
encoder that runs.** This is Finding C above, restated in drift terms: the consolidation locks "the interpreter
is a CFG of straight-line blocks" as part of "the full architecture, stage by stage" and again in "settled and
not reopened" ("the CFG-of-blocks interpreter," `:417`). The src changelist's verification for the runtime-abi
crate only claims "`Residual` carries block and function tables" (`202607240200_changelist.src.lock.md:133-137`),
which is true and narrow (the struct has the fields); it does not claim, and nothing in source provides, a pass
that builds a `BlockTable` from a `vehje-ir::Node` tree. `grep -n "BlockTable::new\|FunctionTable::new\|Block {"
mock/crates/vehje-runtime-abi/src/` returns one hit, the struct definition. This is the sharpest drift-from-canon
finding in this audit, because it is not a topic reasoning past a constraint (like the fixpoint bound above) or
a known, flagged deferral (like the rename); it is a locked design commitment with a real type surface built for
it, silently unimplemented, with no `// FIXME:` anywhere marking the gap, so a future reader has no signal that
"the interpreter is a CFG of blocks" is aspiration rather than fact. Correct it two ways at once, cheaply: add
the missing `// FIXME:` at `residual.rs`'s `Block`/`BlockTable`/`Function`/`FunctionTable` definitions and at
`encode.rs`'s module doc, naming the gap (no tree-to-CFG lowering exists; `Tier::Arena`'s only tested path is
the flat re-tag encoder); and force the actual design question (Finding C's two resolutions) into the next
doc-phase round rather than letting the unconditional `Residual` struct keep asserting, by its shape alone, that
the CFG exists.

**Everything else I checked against the canon holds.** `vehje-fixpoint`'s shape, its three named clients, its
`vehje-ir`-freedom, and its dependency wiring into `vehje-lower` and `vehje-typecheck` all match the taxonomy
exactly (`202607240130/202607240100_topic...md:280-289` against `vehje-fixpoint/src/lib.rs`, `vehje-lower/
Cargo.toml:8`, `vehje-typecheck/Cargo.toml`). The twelve-crate count, the dependency layering, and the `Handle`
form's presence in every match arm the src CL names (`node.rs`, `resolve`, `check`, `fold_core`, the runtime-abi
encoder) all check out. The `vehje-codegen` to `vehje-emit` rename is correctly left undone, per the round's own
explicit deferral of it as unmandated cosmetic change (`:313-315`). The soul of the round, structurally, is
intact; the two live wounds are the unbounded-fixpoint contradiction the very next topic introduced and the
unbuilt, unflagged CFG-of-blocks gap, both of which are cheap to correct (state a bound; add two `// FIXME:`
markers and force the design question) and neither of which requires reopening anything the round actually
settled.
