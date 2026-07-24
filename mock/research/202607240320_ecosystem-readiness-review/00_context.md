# Context: vehje crate-ecosystem readiness review

## The artefact

The vehje framework's Rust compile-side crate ecosystem, as landed by design round
`202607240130` (now closed). vehje is an embeddable, multi-input/multi-output IR framework (not a
language). A grammar plugs in on the input side and lowers surface syntax into a shared Core IR; a
target plugs in on the output side, declares what it supports, and emits a checked residual. The
stated identity is type-system-as-verification plus certified generation: the compile side proves a
program's disciplines sound and emits validated data, and a consumer cannot skip the checks.

Twelve crates under `mock/crates/` (all `#![no_std]`, no alloc, reusing arvo / notko / hilavitkutin):

- `vehje-ir`: the shared Core IR. Twelve Core forms (`node.rs`: Lit, Var, Let, Lambda, Apply,
  Project, If, Match, Iter, Interp, Raw, Handle). The grade vocabulary (`grade.rs`: `EffectMask`,
  `ReachMask`, `Knowledge`/`BindingTime`, `Lease`, `Assurance`, `Grade`, `GradeTable`, all over
  `arvo_bitmask::Mask<Bits<64,Hot>>`). Structural hashing (`hash.rs`). The `Family` extension trait
  and effect classification (`family.rs`, `effect.rs`). Arena + builder + spans.
- `vehje-signature`: language-level graded algebraic signature (`Signature`, `Operation`,
  `LeaseRule`, `TargetDecl`, `Projection`, `GrammarHook`).
- `vehje-fixpoint`: generic vehje-ir-free semi-naive relational engine (`Relation`, `Rule`,
  `Engine`, `Delta`, `Congruence`, `Incremental`, `Store`, `ArtifactSet`).
- `vehje-resolve`: Core-level name resolution (`Scope`, `resolve`, `Resolution`, `resolve_into`).
- `vehje-typecheck` (plays the `vehje-check` role): the graded check pass. `check` computes grades
  into `GradeTable`; `Checked<'a,T>` is the checked-program witness with a `pub(crate)` constructor;
  `mint_checked` is the sole sanctioned mint, gated on `Supports: ContainsAll<Families>` and
  `Permits: ContainsAll<Effects>` (the inclusion proof).
- `vehje-codegen`: the output machinery. The `Target` trait (`Supports` family set, `Permits`
  effect set), `check_for` (delegates to `mint_checked`), the one generic `fold_core` walk, `emit`
  consuming a `Checked`.
- `vehje-lower`: lowering stage (`Lower`, `LowerStrategy`, `ConstFold`, `Cse`, `Anf`,
  `MacroExpand`, `RuleTable`).
- `vehje-schedule`: pass DAG (`PassDag`, currently inert).
- `vehje-runtime-gen`: validated-data package + content-addressed manifest (`Package`, `Slice`,
  `Manifest`, `generate`, `DifferentialCheck`).
- `vehje-runtime-abi`: the tier-tagged C ABI + residual serialization (`wire`, `value`, `sink`,
  `entry`; a batched-column entry shape confirmed by the ABI bench).
- `vehje-runtime-driver`: compiler-side dispatch over the ABI (`dispatch`, `Reader`, `DriverError`).
- `vehje`: orchestration (`run`, `compile_language`, `DebugTarget`).

The implementation is at first-light ("M-level"): every DESIGN pub type is real, compiles, and has
smoke tests; deeper frontier mechanisms carry greppable `// FIXME:` markers plus BACKLOG entries
(the full reachability binder rule, fixpoint-routed effect/reach inference, the dev-time eqsat
e-graph, arena-rewriting in lower, extern "C" table population in runtime-abi, the differential
harness).

## The design oracle (what the ecosystem is meant to be)

- `mock/design_rounds/202607240130/202607240100_topic.consolidated-design-and-taxonomy.md` (the
  converged design + the 12-crate taxonomy).
- `mock/design_rounds/202607240130/202607240015_topic.full-arc-bench-and-evidence-findings.md` (the
  bench evidence the design rests on).
- The per-crate `mock/crates/*/DESIGN.md.tmpl` (the shipping contract for each crate) and the
  deepdives: `vehje-typecheck/DEEPDIVE_*` (graded spine, reachability lease), `vehje/DEEPDIVE_*`
  (certified generation), `vehje-runtime-abi/DEEPDIVE_*` (value transport).

## The question

Evaluate whether this landed ecosystem is ready to build a language / DSL consumer on top of (a
grammar plus a family plus a target that plugs into these crates), and what, if anything, still
blocks that. Give particular attention to the verification core, since the framework's stated
identity is type-system-as-verification: the graded proof spine (`grade.rs`), the sealed `Checked`
witness and the `mint_checked` inclusion-proof gate (`vehje-typecheck/src/lib.rs`), and how
`vehje-codegen`'s `Target`/`check_for`/`emit` consume it. Is that core sound as landed, is its
soundness established rather than assumed, and is it actually buildable-upon by a consumer? Reach
your own conclusion from the source and the design oracle.

The implementation being first-light with FIXME'd frontier mechanisms is a known and intended state;
judge readiness-to-build-on against that, distinguishing a genuine soundness or contract gap that
would block a consumer from a frontier mechanism that is merely not yet implemented.
