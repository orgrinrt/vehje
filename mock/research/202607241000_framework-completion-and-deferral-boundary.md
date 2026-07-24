# Framework completion and the deferral boundary

This note records the state of the vehje framework crate ecosystem at the close of the long design round, and
the precise boundary between what is complete and what is deferred, with the reason each deferred piece is
deferred. It exists so a reader (or the next work session) knows the ecosystem is complete to the extent it can
be at the framework layer, and what each remaining marker is waiting on.

## What is complete

The twelve-crate framework ecosystem under `mock/crates` has, for every crate, a README, a DESIGN, at least
one topic-specific deepdive, and a BACKLOG (a SHAME where a known gap needs one). The documentation models the
full design and carries the bench and research evidence that justifies each crate's shape. The workspace builds
green and all test suites pass.

The implementation matches the documented design at every framework-level mechanism whose design is settled and
whose realisation needs neither a consumer nor a research-grade build nor an unresolved design question:

- The Core IR, the grade vocabulary, the structural hash (leaf-content folding, interner-local), the no-alloc
  arena, and the bounds-checked `GradeTable`.
- The `Resolution` pass including the recursive-`Let` rule, and the reachability binder rule in the graded
  check (a `Var` contributes its binder's reach slot, a `Let`/`Lambda` drops its own).
- The sealed `Checked` witness, the `TargetSets`-derived inclusion mint, and the `Graded` evidence gate.
- The lower cheap stratum: the redirecting `Rewrite`, `If`-condition const-fold, and capture-safe
  variable-free CSE with a structural re-verify.
- The relational engine's whole-column evaluation (the measured winner) and the congruence union-find.
- The value-arena wire types with a declared `#[repr(C)]` layout, the bounds-safe accessors, and the typed
  structural decode (`Reader::validate`) with the monotone acyclicity check.

## The deferral boundary, and why each piece waits

Every remaining `// FIXME:` marker falls into one of four categories. None is deferred for size alone; each is
blocked on something the framework layer cannot supply from itself.

### Waits on a consumer (a grammar plus a family plus a target, which live in separate consumer repos)

The framework owns no consumer, by design. These land with the first consumer that constrains their shape;
building them now would invent a shape no consumer has asked for.

- The family payload IR shapes: `Match` arm pattern-plus-body pairs, `Handle` clause (operation id plus handler
  body plus resumption binder), and the `Raw` family-interpreted payload encoding.
- The family-extension hooks: the `Raw` resolve hook, the `Raw`/`Handle` family-check and handler-discharge
  hooks in the check pass, and the `Raw` sub-structure fold in codegen.
- The family primitive const-fold and the macro-expansion dispatch in lower (both need a family operation).
- The attribute-grammar schema in the signature, the census-corpus harness in runtime-gen, and the grammar
  byte-source seam and `compile_language` in the top crate.

### Waits on a research-grade build the design sequences as a later stratum

- The equality-saturation e-graph (`vehje-lower`'s `Saturate` stratum) with its iterative cost-fixpoint
  extractor, and the congruence re-derivation it needs. The design ships the cheap stratum first and stages
  saturation behind a bounded streaming window.
- The relational engine's semi-naive delta (whole-column is the measured winner on the realistic shallow
  graphs), the multi-relation worst-case-optimal join, and retraction.

### Waits on the Zig runtime, which is a separate artifact not built here

- The `#[no_mangle] extern "C"` batched-column export (also gated on the cdylib panic-runtime story; the crate
  ships as an rlib), and the runtime dispatch in the driver (the serialize half is doable, the call half needs
  the runtime).

### Waits on a design decision that is genuinely open

- The full `run` pipeline interleave (check and lower interleaved in the orchestrator) and what the `Checked`
  witness covers across a lowering rewrite, plus the program-derived family and effect sets (the inclusion half
  that stops trusting caller-supplied sets). These carry real unresolved questions, not just unwritten code.

## Decisions owed to the maintainer (none blocking the framework's completeness)

- The wire-spec details: the `ValueTag` discriminant width, the per-record byte layout at the batched-column
  entry, and host-native versus cross-platform value fields (`USize` versus fixed-width).
- The two deferred crate renames: `vehje-typecheck` to `vehje-check`, and the suggested `vehje-codegen` to
  `vehje-emit`.
- The free-variable-aware CSE refinement (today CSE conservatively shares only variable-free subtrees) and the
  census-sized reach mask (today the reach slot is the binder index modulo 64, the degenerate depth-lease
  floor).
- Whether to build a first consumer next (which unblocks the family-shaped markers) or invest in the
  research-grade e-graph stratum.

## Bottom line

The framework is a ready-to-go crate ecosystem: complete documentation, and an implementation that realises
every framework-level mechanism the design settles without a consumer, a research build, the runtime, or an
open design question. The remaining markers are honest, each naming what it waits on. The mockspace DSL can be
built on this machinery as its first consumer, which is also what defines the family-shaped markers above.
