# CFG-of-blocks lowering: ANF Core into the residual's block and function tables

**Date:** 2026-07-25
**Scope:** the producer that turns an ANF-normalised `vehje_ir` Core program into the `vehje-runtime-abi` residual's `BlockTable` + `FunctionTable`, the missing step named by the `residual.rs` FIXME and the §11 CFG item. A design deliverable; a subsequent round implements it.
**Source topics:** the residual CFG model (`vehje-runtime-abi/src/wire/residual.rs`), the ANF catamorphism (round 202607250218, which names the join points), op's Zig-runtime decision (2026-07-25), the design's tier model (`Tier::Arena` reference semantics first).

The residual is a control-flow-graph of blocks over a serialized node arena. The node arena, pool, and blob already serialize (`wire/serialize.rs`); the `Block` / `Function` / `BlockTable` / `FunctionTable` records that carry the control flow do not, because no producer builds them yet (`residual.rs:BlockTable` FIXME). Execution needs them: a `Tier::Arena` runtime interprets the serialized nodes by walking the blocks, taking each block's straight-line node range and then its terminator. This memo specifies the ANF-to-blocks lowering that builds those tables, with the first-landing subset pinned and the harder forms sequenced.

The input is ANF, not raw Core, and that is load-bearing. ANF has already named every non-atomic intermediate in a `Let` and made evaluation order explicit as binding order, so a straight-line stretch of the program is exactly a chain of `Let`s whose values are atoms or simple applications, ending in a tail. That chain is one basic block. Control flow appears only where ANF left a form that is its own evaluation context: an `If` (two branches, a join), an `Iter` (a loop), a `Lambda` (a function boundary), a non-atomic `Apply` in tail position (a call), a `Match` (a multiway branch), a `Handle` (an effect frame). Each of those is a terminator plus successor blocks. So the lowering is a walk over the ANF tree that accumulates straight-line nodes into the current block and cuts a new block at each control form.

## What the lowering produces, and where it lives

The producer reads a `vehje_ir::Arena` plus the ANF root and writes, into caller-lent pools (no alloc, the same discipline as the arena and the ANF scratch), three things: a `Block` array (the block records), a successor-id pool (the flat `BlockId` slice the `SuccRange`s index), and a `Function` array. It returns `BlockTable` and `FunctionTable` borrowing those pools. It lives in `vehje-runtime-abi` in a new `cfg` module beside `residual.rs` and `serialize.rs`: it already depends on `vehje-ir` (it names `NodeRef` / `NodeRange`) and it owns the `Block` / `Function` types, so the producer sits with the model it builds. The signature:

```rust
pub fn lower_to_cfg(
    arena: &Arena<'_>,
    root: NodeRef,
    blocks: &mut [Block],       // caller-lent block records
    succ_pool: &mut [BlockId],  // caller-lent successor-id pool
    functions: &mut [Function], // caller-lent function records
) -> Maybe<(BlockTable<'_>, FunctionTable<'_>)>  // Isnt on pool overflow
```

`serialize` then gains the block table + function table as inputs (today it takes only the node/pool/blob images) and writes their records into the wire image, so the `Residual` the runtime reads carries the CFG. The `Tier::Arena` node arena is the same serialized image that already ships; the block table is the control-flow index over it.

## Blocks and the straight-line accumulation

A block is a maximal straight-line region: a `NodeRange` (a contiguous `start` + `len` span of the block-ordered wire node image, see the re-serialization section), a `TerminatorKind`, a `SuccRange` into the successor pool, and a block-argument `Signature`. `NodeRange` is a span, not a node list, because the serializer emits each block's nodes contiguously into the wire image, so the span is exact there even though the same nodes are scattered across the producer's input ANF arena. The lowering walks the ANF tree, assigning each straight-line node to the current block (advancing over each `Let` / atom / simple-`Apply`) until it reaches a control form, at which point it seals the current block with the control form's terminator and its successors, and recurses into the successor regions as new blocks. The serializer later groups the assigned nodes by block to produce the contiguous image.

The join-point mechanism is where ANF pays off a second time. An `If` computes a value in each branch and the code after the `If` uses that value; in a CFG the two branches must converge at a join block, and the converged value is a block argument of the join (the branches pass their result across the edge). ANF already named that value: the `If` sits as the value of a `Let t = if ... in <rest>`, so `t` is the join's block argument and `<rest>` is the join block. The lowering reads the join's argument arity directly from the `Let` binder count at the join, so `Signature { arity }` is populated without a separate dataflow analysis. This is the "names the join points" property the ANF round was built to provide.

## The per-form mapping

Straight-line forms extend the current block:

- `Lit`, `Var`: atoms; part of a block's node range, never a terminator.
- `Let { name, value, body }`: `value` is an atom or a control form. If `value` is straight-line, the `Let` extends the current block and lowering continues into `body`. If `value` is a control form (an `If`, `Iter`, `Match`, `Handle`, or a non-atomic `Apply` in a call position), the current block seals at the `Let`, the control form's successors are lowered, and `body` becomes the join/continuation block with `name` as its block argument.
- `Apply { callee, args }` with an atomic callee and atomic args in a non-tail position: a straight-line call-return whose result is named by its enclosing `Let`; it extends the block, and the terminator is `Call` only when the call is the block's tail transfer (see below). A first landing may model every `Apply` as a `Call` terminator ending its block; the straight-line-call optimisation (a call that returns into the same block) is a later refinement noted in BACKLOG.
- `Project`, `Interp`: straight-line single-operand nodes; extend the block.

Control forms seal the current block and open successors:

- `If { cond, then_branch, else_branch }`: the current block ends with `TerminatorKind::Branch`; its `SuccRange` is `[then_block, else_block]`; `cond` (an atom, by ANF) is the scrutinee. `then_branch` and `else_branch` lower as their own blocks, each ending in a `Jump` to the join block (the `If`'s enclosing-`Let` body), passing their result as the join's block argument.
- `Iter { seq, body }`: a loop. A header block tests the iteration and ends in `Branch` to `[body_block, exit_block]`; `body_block` lowers and ends in a `Jump` back-edge to the header; the loop-carried accumulator is the header's block argument (arity from the `Iter`'s accumulator binding). `exit_block` is the continuation.
- `Lambda { param, body }`: a function boundary. It emits a `Function` record (its `entry` is `body`'s first block, its `args` signature is the parameter arity, its `results` signature the tail arity), and `body` lowers as that function's blocks, its tail block ending in `Return`.
- `Match { scrutinee, arms }`: a multiway `Branch` whose `SuccRange` is the arm blocks; each arm `Jump`s to the join. (Arm patterns are deferred with `Match`'s own pattern-representation FIXME; the first landing handles the arm-body control flow, not pattern binding.)
- `Handle { body, clauses }`: an effect frame. `body` lowers as the handled blocks; a clause is a handler entry reached by the effect operation's transfer. This is the most involved form and is deferred past the first landing (it needs the resumption/frame model the runtime's effect discipline defines); the first landing lowers `Handle` as its `body` with the clauses recorded but not yet wired, marked FIXME.

`Raw` is opaque: it lowers as a straight-line node in the current block (its family-interpreted payload is not control flow the Core lowering reasons about).

## The node image is re-serialized in block order (a correction)

An earlier draft claimed a block's nodes are already a contiguous span of the ANF arena. That is wrong, and the error is worth stating so the implementation does not inherit it. The ANF catamorphism builds inner-to-outer: it emits the innermost body first and wraps `Let`s outward, so the arena's index order is roughly reverse-evaluation order, and the nodes of one straight-line block are scattered across the arena, not contiguous. `NodeRange { start, len }` is a contiguous span by contract, so the span cannot index the ANF arena directly.

The resolution: serialization emits a fresh node image ordered by block, and `NodeRange` indexes that wire image, not the ANF arena. The producer and the serializer are therefore coupled: `lower_to_cfg` determines the block structure and the per-block node membership, and the serializer walks the blocks in order, emitting each block's nodes contiguously into the wire node image while recording each block's `NodeRange` as its `[start, start+len)` in that image. A node referenced by more than one block (a value used across a branch) is not duplicated; it is emitted once at its definition block and referred to by the wire index, the same way the source arena refers by `NodeRef`. So the wire `Residual.nodes` image is block-ordered, its `NodeRange`s are exact spans over it, and the ANF arena is the producer's input only, never the runtime's image. This makes the block/serialize coupling explicit: the two land together, and the `cfg_serialize_round_trips_the_tables` test covers the block-ordered emission, not just the table records.

This also fixes the block-membership question the straight-line accumulation raised: a node belongs to the block whose straight-line region defines it (its enclosing `Let`), determined by the tree walk, not by an arena-index range. The walk assigns each node a block; the serializer then groups by block to produce the contiguous image.

## The first landing and the sequence

The first landing produces a correct CFG for the straight-line + `If` + `Lambda`/`Call` + `Return` subset: a program of let-chains, conditionals, function definitions, and calls lowers to blocks and functions the Zig `Tier::Arena` interpreter can walk. `Iter` (loops), `Match` (multiway), and `Handle` (effects) land in follow-on rounds, each adding its terminator shape and its block-argument story; the block/function/successor pools and the walk are unchanged, so the follow-ons are additive. This mirrors the ANF round's shape (a correct core subset first, the harder forms sequenced) and keeps every landing a real, testable increment.

The no-alloc exhaustion contract matches the arena's: `lower_to_cfg` returns `Maybe::Isnt` when the block, successor, or function pool overflows, never truncating. The caller sizes the pools linearly in the program's control-form count (one block per branch/loop/arm plus the join, one function per lambda), a caller-provisionable bound the same way the ANF node bound is.

## Termination and correctness

The lowering is structural recursion over the finite ANF tree: each control form recurses only into strictly-smaller subterms (its branches, body, arms). There is no fixpoint. A block's node range is a contiguous span over the block-ordered wire image the serializer emits (see the re-serialization section above), not over the ANF arena, so the `start`/`len` representation is exact by construction rather than by assuming the ANF arena's order. The join block arguments come from the ANF binders, so the CFG's dataflow at joins is exactly the source program's, with no separate SSA-construction phase and no phi-insertion pass.

## Strict tests to write first

1. `cfg_straight_line_is_one_block`: a let-chain of atoms lowers to a single block ending in `Return`, arity zero.
2. `cfg_if_is_a_branch_with_a_join`: `let t = if c then a else b in t` lowers to a `Branch` block with two `Jump` successors converging on a join block whose argument arity is one.
3. `cfg_lambda_is_a_function`: a `Lambda` emits a `Function` whose `entry` is its body's first block, its tail block ending in `Return`.
4. `cfg_pool_overflow_returns_isnt`: too-small a block or successor pool returns `Maybe::Isnt`, no truncation.
5. `cfg_serialize_round_trips_the_tables`: the produced `BlockTable` / `FunctionTable` serialize into the wire image and decode back structurally identical (the runtime's decode contract).

## Downstream

Once `lower_to_cfg` + the serialize extension land, the residual carries a real CFG, and the Zig runtime's `vehje_runtime_execute` (today a skeleton in `mock/runtime-zig/src/runtime.zig`) decodes the residual, walks the entry function's blocks executing each node range and taking each terminator, and produces a value marshalled back across the C ABI. The bench Zig interpreters (`mock/benches/cfg-interp-throughput/cfg.zig`, `carrier-zig/interp.zig`, `zig-dispatch/dispatch.zig`) are the reference for that walk. That is the step that makes a vehje program genuinely runnable.
