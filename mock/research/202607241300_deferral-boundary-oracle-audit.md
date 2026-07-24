# Deferral-boundary audit against the design oracle (2026-07-24)

Every remaining `// FIXME` in the shipping crate roots, audited one by one against the
locked design docs (DESIGN.md.tmpl + DEEPDIVE_*), to distinguish design-mandated deferral
from defer-instinct. Method: for each, invert the defer instinct (treat it as
implementable-now), then check the oracle. The oracle rules.

Total: 37 FIXMEs across 12 crates. Every one falls into a design-mandated-deferred class.
None is defer-instinct. The framework is at the design-intended M0 first-light state:
surface stable, docs complete, Core-side mechanisms built, consumer/runtime/open-design
wiring deliberately at honest stubs.

## Class 1: consumer-coupled (needs a real consumer family to define/exercise the shape)

- vehje-schedule (3): the pass-DAG BitMatrix adjacency + topo_sort. DEEPDIVE_PASS_DAG is
  explicit: "Both land together with the first consumer family pass... Building the wiring
  ahead of a consumer to exercise it would mean guessing at a shape nothing has tested yet;
  the narrowed remit is itself the product of one such guess turning out wrong once." Design
  canon.
- vehje-lower MacroExpand (1) + family-primitive ConstFold (1): need a consumer family's
  macro ops / payload encoding. "no consumer family defines a macro operation yet."
- vehje-ir Match arm pattern+body pairs (1), Interp family-handle payload (1), Handle clause
  op/handler/resumption (1), Family numeric-id for the runtime bitmask (1): IR shapes whose
  family half a consumer defines.
- vehje-signature attribute-grammar schema (1): "lands with the first grammar-as-data consumer."
- vehje (1): Grammar's source input type "lands with the first consumer grammar."

## Class 2: runtime-coupled (the Zig runtime side, across the C ABI)

- vehje-runtime-abi extern C export (1), per-position value-kind tags (1), relation-kind
  vocab (1), seed relation (1): the wire ABI crossing to Zig.
- vehje-runtime-driver serialize+call (1): drives the residual into the Zig runtime.
- vehje-runtime-gen manifest content-addressing (1) + differential-test harness (1): the
  runtime package format + its census corpus.

## Class 3: open-design mechanism, shape unspecified (building now = guessing)

- vehje-lower Anf (1): needs "the mutable-arena rewrite path once one exists." DEEPDIVE_LOWERING
  is explicit that the redirect-only Rewrite is a deliberate architectural choice ("the arena
  is not the crate's to mutate") and that ANF's node-creating rewrite has an unspecified
  ownership shape. "An unconditional no-op is honest where a partial implementation would
  silently fail." Design prescribes the current no-op.
- vehje-typecheck inference completions (3: Families/Effects derived-not-claimed, Handle
  discharge, binding-time/assurance discharge) + Graded program-derived sets (1): all tied to
  "the runtime-bitmask path (tracked #29)", a specified-but-unbuilt inference path that assigns
  Family numeric ids and accumulates them during the fold. Coupled to Class 1's Family-id.
- vehje-resolve Handle non-lexical binder (1) + handler clause resume binding (1): the Handle
  form's effect-handler resolution, paired with the typecheck discharge above.

## Class 4: research-deferred (building now = faking research / the flagged original)

- vehje-lower Saturate (1): "the no-alloc bounded streaming e-graph is the flagged original
  research."
- vehje-fixpoint (4): base whole-column semi-naive is shipped; WCOJ leapfrog-triejoin,
  general retraction, and re-derivation refinements are the research tail.

## Class 5: op-decision-coupled (recorded for morning review)

- vehje-runtime-abi wire widths (ValueTag discriminant, per-record layout, USize vs
  cross-platform); crate renames (vehje-typecheck to vehje-check, vehje-codegen to vehje-emit);
  census-sized reach mask (the #30 catalogued-red test); free-var-aware CSE refinement.

## Small / cosmetic

- vehje-codegen (1): name the missing family/effect on the inclusion-error variant. Waits on
  the same derived-sets path (Class 3).
- vehje-ir interner cross-arena handle stability (1): interner-integration decision.

## Conclusion

The design oracle prescribes the current state at every remaining FIXME. The deferred wiring
is meant, by the design's own explicit reasoning, to land WHEN the first real consumer (the
mockspace DSL) is built. That is the next arc, and it is exactly the GOAL's stated purpose
("so we can next build the mockspace DSL on the vehje machinery"). The consumer_demo proves
the surface hosts a consumer today. Docs are 100% complete; implementation matches the design
at M0 first-light. Boundary reached, and it is design-mandated, not defer-instinct.
