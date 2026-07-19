# Baseline after the maintainer's rulings and a source check

**Date:** 2026-07-19
**Supersedes:** the six open questions in `00_seed.md`. The seed's settled decisions still stand.

Four of the six are answered, two are reshaped, and one claim in the seed was wrong. This is the state an
independent reader should start from.

## Answered

**1. hilavitkutin's storage contracts are separable from its scheduler.** Verified in source rather than
assumed: `hilavitkutin-api`, which holds `Column`, `store` and `context`, depends on `arvo`,
`arvo-bitmask`, `arvo-tensor`, `hilavitkutin-ctx` and `notko`. It does not depend on the engine crate at
all, and nothing in the workspace depends on the engine to use the contracts.

The maintainer adds the part that matters for design: **the scheduler is wanted eventually.** So the
interpreter's execution should be shaped so that hilavitkutin's scheduler can drop in when it is ready,
rather than merely borrowing its storage types. Until then we run a minimal runner fitted to this use
case. That makes the runner's contract, not just its data model, a design constraint.

**2. No fragment sources, and no offset mapping.** The seed proposed a source abstraction carrying an
origin and mapping spans back to a host document's line and column. That is over-built. The input is taken
verbatim from the code block, interpreted and evaluated with no mapping, and a diagnostic anchors to *the
block* rather than to the file. The renderer already knows where the block began, so reporting "in the
query block beginning at line 400, at line 2" needs no threading of host offsets through the span layer.

**P2 collapses** from an abstraction to a caller convention.

**4. One type system, not two.** The DSL wants exactly the guarantees canonical vehje wants: static
analysis, typestate, the same statically-analysed grammar and IR. It differs only in surface expression.
The maintainer's framing: doing to a dynamically interpreted IR what Rust did to C. So `P7` is one piece
of work, and the DSL is not a weaker dialect that needs a weaker checker.

**5. Arena capacity is controlled, not discovered.** If the DSL outgrows its preallocation, the
preallocation grows. The sharper point: if that ever becomes a real problem, the answer is to alias
existing allocations rather than to copy the registry into the interpreter. That is a design instruction,
not a fallback.

## Reshaped

**3. The efficiency objection dissolves; the soundness question stands.** The seed worried that one IR
serving two syntaxes means carrying nodes one side never emits. With the vocabulary as a compile-time type
parameter, monomorphisation erases unused paths entirely, so there is no runtime cost to answer for. What
is still worth an independent read is whether one IR over both is *sound and sensible*, as opposed to
merely cheap.

**6. The seed's claim was wrong.** It asserted that vehje needs an interpreter anyway for const
evaluation, and that mockspace's interpreter would therefore be shared work. Source says otherwise: vehje
has no const evaluator. The only trace is `STRICT4`, a validator for compile-time integer overflow that
returns an empty diagnostic vec and is itself marked a skeleton. That wants constant folding over literals,
which is much less than a tree-walking interpreter over the IR.

**The interpreter is mockspace's need.** Whether vehje wants one is a design question for the maintainer,
not a fact to be discovered, and the seed should not have leaned on it.

## Found while checking

`vehje-codegen` already carries a backend registry: `lookup(name) -> Maybe<&'static dyn CodegenTarget>`,
with `emit_for` dispatching to a named target. The output half of "grammar plugins on both sides" is
therefore already plugin-shaped rather than speculative. Worth noting that it uses `dyn` dispatch, which
sits oddly against the workspace's no-dyn discipline and may want revisiting on its own terms.

## What is genuinely open

Reduced to four, and none of them is answerable by reading more source.

1. **Is one IR over two syntaxes sound?** Not the cost, which is answered. Whether a query-and-expression
   shaped language and a statement-and-item shaped language lowering to one vocabulary produces a
   coherent IR, or an IR that is the union of two designs and the home of neither.
2. **The value domain, columnar natively.** The seed's direction, now that `hilavitkutin-api` is
   confirmed available. What a value is, how records and columns are first-class without an alloc, and
   whether the projection type rule holds across scalars, rows and tables.
3. **Does the ambition stay honest?** Procedural documents are meant to stop a document restating what a
   row already says. A language with iteration makes it mechanically easy for a document to become a
   rendering of the table it loops over, which is the same drift arrived at from the other side. What is a
   document still for once it can generate itself.
4. **No-alloc under iteration and text output.** A loop over thousands of rows producing text has to put
   the text somewhere. Caller-provided sink, fixed arena, or streaming changes the interpreter's shape,
   and interacts with the instruction to alias rather than copy.
