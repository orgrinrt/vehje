# Core nodes stress-test, the proof-root traits, and the Rust-typestate to Zig-runtime bridge

**Date:** 2026-07-20
**Status:** Worker-fork deliverable, one shot. Muses and surfaces; settles nothing. The parent and op decide.
**Task:** Stress-test the ratified 11 Core forms against the backdrop this arc converged on (certified
generation, staged gradual verification, the lease axis, the value-transfer model, the three-codegens
reframe). Evaluate the shipped typesystem traits that root the proofs, and the metacompile sketch. Then
explore abstracted approaches for the bridge from the Rust typestate to the Zig runtime: what it composes of,
how it is built from the typestate and the plugged-in input and output languages. Spell out the undesigned,
vague, and unmentioned pieces, with several candidate directions for each.
**Reads:** the six round topics (1315, 1316, 1513, 1534, 1627, 1845), the panel `01`..`08b`, the five synth
domain docs, the committed spine round `202607192358/` (core-forms cut, output-side contract, ABI topic), the
metacompile sketch `202607201352`, and the source under `mock/crates/` plus `mock/runtime-zig/`.

**Caveat (op, mid-review):** the code under `mock/crates/` is old and disregarded. This doc therefore treats
those types as an *earlier sketch of the trait direction*, not as the authoritative proof-root, and evaluates
the design direction (which comes from the topics, not the code). Where a finding below rested on the old code
(the `wire.rs` `Tier::Native`, the "shipped" `check_for`, the Zig skeleton), it is re-scoped to the design
artifact that carries the same framing. The design oracle is the topics; the old code is illustrative only.

## One-line verdict

The 11-form evaluation core holds against the new backdrop with no new evaluation form owed. What the backdrop
demands is not a twelfth Core form but four things that sit beside the forms: a separate value-node-kind domain
(op-sequenced, still the largest owed piece), region and effect metadata on the existing forms, an effect
ordering discipline, and a runtime budget. The shipped typestate roots the proof correctly for the static
family and effect axes and is half-built (the runtime-bitmask path and the lease axis are owed). The bridge to
Zig is best read as two boundaries, not one C ABI, and the hybrid data-plus-narrow-behavior shape the
metacompile sketch found is the defensible direction. One live contradiction needs a decision: the committed
ABI topic and the shipped `Tier::Native` encode the program-centric and native-as-tier framings that the
three-codegens reframe explicitly retired.

## The backdrop delta: what changed after the 11 forms were ratified

The forms were ratified 2026-07-19 (`202607192330`), inside the contract spine, under a program-centric read:
the Rust compiler lowers a program to a `Checked` residual that crosses a C ABI to a thin Zig interpreter. The
five things that landed after change the frame the forms are judged against:

1. **Three-codegens reframe (1513).** The Rust side compiles the language, not a script. Runtime generation
   emits the composed runtime once per language; scripts arrive at the composed runtime and are parsed,
   resolved, checked, macro-expanded, and output-generated there. "Native" is one output target, not a tier.
2. **The debts (1534) and certified generation (1627).** The proof is compiled away into certified generated
   Zig, not re-run and not re-shipped. Zig `comptime` is the specialiser over Rust-emitted validated data. The
   guarantee holds by double certification at our build time (rustc types the data and generator, the Zig
   compiler types the specialised engine), none at the consumer.
3. **The lease axis (1316).** A third proof axis, region-validity only, immutable-value, a `Leased` typestate
   gate mirroring `Checked`, region tags emitted into the value image.
4. **The value-transfer model (1315).** The produced value crosses as a flat index-referenced value-arena over
   a `reserve`/`commit` sink, whole-subtree chunk streaming, arena-never-moves.
5. **The staged-gradual-verification frame (08).** Every safety property is discharged at the earliest binding
   time its input is known, across three binding times: language-definition, bundled-content, script-arrival.

The forms were designed for (1)'s predecessor. The stress-test asks whether they survive (2) through (5).

## Stress-test: the 11 Core forms under the new backdrop

The forms: `Lit`, `Var`, `Let`, `Lambda`, `Apply`, `Project`, `If`, `Match`, `Iter`, `Interp`, `Raw`
(`202607192330`; shipped as the `Node` enum and the `NodeTag` set in `wire.rs:54`).

### Per-form pressure

- **`Lit`** carries the two-tier value domain (scalars inline via arvo, text/arrays as `(offset, len)` into the
  generation-time byte arena). The pressure: this generation-time byte arena is a *different* arena from the
  runtime value-arena the transfer model produces. A `Lit`'s bytes live in the compile-time image (serialised
  in `wire.rs`'s trailing blob); a produced value's bytes live in the runtime value-arena over the sink. Two
  arenas, two lifetimes, two producers. The form holds; the two-arena reconciliation is an owed piece (below).
- **`Var`** is a name pre-resolution, a binder handle post-resolution. Under the lease axis it is the carrier
  of a reference whose lease is inferred. It holds; the lease metadata attaches here.
- **`Let`** and **`Lambda`** are the region openers. The lease default region is a `Let`/`Lambda`/body scope
  (1316, confirmed by the metacompile sketch stage 3). `Lambda` is also the region-escape hazard: a closure
  captures values and can outlive the scope that built them. The immutable-value narrowing that makes the lease
  analysis cheap (drop aliasing, keep region-validity) does **not** remove closure escape, because a closure is
  immutable yet still captures and outlives. This is debt 2 (1534) made precise: the hard case of the
  Tofte-Talpin lineage is exactly higher-order capture, and `Lambda` puts it first-class in the vocabulary. The
  form holds, but it is the load-bearing proof obligation for T3 (Calcagno-Helsen-Thiemann), not a free case.
- **`Apply`** is the consumer of leases (a call consumes its arguments' references) and the site of effects
  (`Reads`/`Writes` ride on the callee's classification). Two `emit`s are two `Apply`s carrying
  `Writes<RuntimeEnv>`; their ordering is not expressed by the form. Effect ordering is an owed discipline
  (below), not a new form.
- **`Project`** is field/member/index access and, per `202607192330`, also the shape of an environment read
  (a `self`-context read is a `Project` into a context binding, a registry query an `Apply`/`Project` into a
  provided binding). So `Project` plus the effect set is how build-env versus runtime-env access is expressed
  without a dedicated form. Holds cleanly; the binding-time split rides the effect axis exactly as 08 wants.
- **`If`** carries the staging-preserved two-arm branch (both arms survive into residual code under an emit
  policy). Under the staged-gradual frame this is a binding-time annotation site (early-folded or late-emitted).
  Holds.
- **`Match`** stays Core because destructuring does not desugar to `If` cleanly. Under leases, a match arm binds
  pattern variables scoped to the arm body, a region opener like `Let`. Holds; the lease inference needs each
  arm's binding scope as data, which it already is.
- **`Iter`** is the monoidal accumulation form. Under the value-transfer streaming spine (panel `05`), `Iter` is
  the natural producer of a streamed sequence: each iteration emits a whole-subtree chunk, and the backward-only
  post-order keeps the frontier bounded. So `Iter` is where the streaming and the lease inference meet. Holds,
  and it is the form the cross-chunk lemma (panel critical path) most directly governs.
- **`Interp`** was kept a distinct form precisely because a staging rule attaches to it (`202607192330`: an
  interpolation hole inside otherwise-emitted code is generation-time by what a hole means). The staged-gradual
  frame formalises that rule: `Interp` is a binding-time boundary, and whether the hole is `Reads<BuildEnv>` or
  `Reads<RuntimeEnv>` decides its fold. The form is exactly the nameable site 08 needs. Holds and is vindicated.
- **`Raw`** is the single open point, and the new backdrop *promotes* it. Under certified generation (1627),
  the generated Zig IR type holds exactly the supported families' node shapes; `Raw` is the seam where those
  per-language family variants plug in. In the generic Rust IR, `Raw` is opaque (`FamilyId` plus a payload
  handle, `wire.rs:234`); in the generated Zig, the certified generator expands each supported family's `Raw`
  into concrete union variants so the exhaustive switch is total. `Raw` is therefore not merely an escape hatch,
  it is the extension point the whole metacompiler hangs the family axis on. This holds and is elegant, but it
  raises the family-fold question (`fold_core` visits `Raw` as a leaf today, `codegen/lib.rs:104` FIXME): how a
  family walks its own sub-structure is undesigned.

### Devil's advocate: is a twelfth form owed?

Every candidate disposes cleanly except one:

- **`Seq` / block:** ruled out (`202607192330`, Carmack). Effect ordering is A-normal-form `Let`-sequencing
  plus effect edges; content sequencing is monoid concat through `Iter`. No form owed.
- **A `Perform` effect form:** effects ride on `Apply`/`Project` plus the effect set (`202607192330`). No form.
- **A `Region` / `scope` form:** inferred from `Let`/`Lambda`/`Iter`/`Match` bodies and emitted as open/close
  markers (1316). No source form.
- **Record / list / variant constructors:** these are value-domain, not evaluation forms. They belong to the
  owed value-node-kind schema plus families, not Core (the metacompile sketch classifies record construction as
  family runtime behavior, stage 6). No Core form.
- **A `Cast` / lease-ascription form:** under staged-gradual the dynamic residual is a check the specialiser
  inserts, not a source form (08). An explicit lease annotation (1316's owed consumer-surface hook) desugars to
  metadata on a binder, not a new form. No Core form.
- **The one plausible twelfth: a `Handle` for resumable, user-defined algebraic effects.** The effect model is
  a static set plus reduction, not a handler calculus; nothing in the current effect design lets a consumer
  language define its own resumable effect handlers (Plotkin-Pretnar algebraic effects and handlers). None of
  the nine census consumers wants this today. It is the single form that would be genuinely new rather than
  desugarable, and it is worth naming as the one future-consumer pressure that could reopen the cut. Not owed
  now; flagged so it is not a surprise later.

**Verdict on the forms:** the 11 hold. The backdrop demands metadata and companion domains around the forms,
not a new form.

### What the backdrop actually owes beside the forms

1. **A value-node-kind schema (the produced-value domain).** The transfer model, the region tags, and certified
   generation all depend on it, and it is deferred (op-sequenced follow-up). This is the largest owed piece and
   is treated on its own below.
2. **Region metadata on the forms.** The lease inference needs each form's default-region rule as data (it
   already is: `Let`/`Lambda`/`Iter`/`Match` bodies). No new form, but the metadata and the `Leased` gate are
   undesigned in code.
3. **Effect ordering.** The synth purity doc's LMS reading is load-bearing: ordering is an explicit
   dependency-edge per effectful node, not emission order, or two `Writes` can reorder past a dependent read.
   The forms do not carry it; the resolve or check pass owes it.
4. **A runtime budget.** No totality axis (08: termination is not in the system, budgets as containment).
   `Let`-recursion plus `Lambda` gives unrestricted recursion; the runtime needs a fuel or step budget. Owed.

## The one live contradiction: the ABI spine versus the reframe

The contradiction is between two *design artifacts*: the committed ABI topic (`202607192340`) encodes two
framings the three-codegens reframe (1513) explicitly retired. (The old `wire.rs` `Tier::{Arena, Bytecode,
Native}` enum encoded the same framing in code; that code is disregarded per op, so it is only corroborating
evidence that the retired framing had propagated into an implementation, not a live finding on its own.) The
two retired framings the ABI topic still carries:

- **"The compiler reduces a program to a residual that crosses to the runtime."** 1513 supersedes this: the
  Rust side compiles a *language*, not a script; what crosses to the shipped runtime is the generated language
  code, once, not a per-program residual. The ABI topic is written entirely around the per-program residual
  crossing a C ABI.
- **Native as the top execution tier** (`202607192340` tier 3). 1513 retired native as a tier: "there is no
  native execution mode, no native tier, no native endgame; native is one output target." The committed ABI
  topic still ladders arena, then bytecode, then native-as-endgame.

This is a design artifact to correct (the ABI topic re-voiced under the reframe), and it is a decision, not a
settled fix, so it goes to op. The reconciliation that
dissolves it, and which I think is right, comes from the three binding times (08):

- **wire.rs (the tier-0 flat arena) is the binding-time-neutral IR image**, not a Rust-to-Zig cross-ABI
  artifact. It is produced by the Rust side for a *bundled* script (a script known at our build time, lowered
  and shipped as a data blob) and by the composed runtime's own parser for an *arriving* script. Same format,
  two producers, one interpreter. The residual does not "cross a C ABI per program"; it is either embedded data
  (bundled) or an internal image the Zig runtime builds from parsing (arriving).
- **The tier axis collapses.** Arena is the IR image. Bytecode is an internal execution-speed optimisation of
  the composed runtime, orthogonal to the output-target spectrum. Native is a *target* (an output-generation
  point that emits machine code, the practical route being generated Zig), not a tier of the interpreter. The
  `Tier` enum conflates an internal execution optimisation (arena versus bytecode) with an output target
  (native). Under the reframe these are two different axes and `Tier::Native` should not exist.

Recommended framing to put to op: keep `wire.rs` as the IR image format, rename or narrow `Tier` to name only
the internal execution form (interpret-the-arena versus run-the-bytecode), and move "native" out of the tier
enum entirely into the output-target set where 1513 put it. The committed ABI topic then owes a re-voicing
under the metacompiler reframe, the same way 1513 re-voiced the transfer and lease topics.

## The proof-root traits: the direction, and what it owes

The proofs are rooted in the family, effect, and lease typestate over `hilavitkutin_api::access`. The old
`mock/crates` code sketches an earlier cut of this direction; it is disregarded per op, so this section
evaluates the *design direction* the topics fix (output-side contract, 1316, 1627) and uses the old code only
to illustrate what a first cut of it looked like.

### The direction is right (as the old cut sketches it)

- **Two orthogonal axes on one machinery.** A `Family` marker and the effect markers `Pure` / `Reads<E>` /
  `Writes<E>` over `BuildEnv` / `RuntimeEnv` both instantiate the same `AccessSet` (`Empty` / `Cons<H, T>`),
  and the inclusion check is the same `ContainsAll` for both (the old cut has this at `family.rs`, `effect.rs`).
  This is the perfect-mirror the output-side contract argued for, and it is the right direction.
- **The `Checked` typestate is the right root.** `Target { type Supports: AccessSet; type Permits: AccessSet }`
  plus a `Checked<'a, T>` witness producible only by a check whose `where T::Supports: ContainsAll<Families>,
  T::Permits: ContainsAll<Effects>` bounds are the inclusion proof (the old cut sketches this as `check_for`).
  This is the static half of the two-stage proof, and it is exactly "the obligation to check is enforced at
  compile time." Reusing `hilavitkutin_api::access` rather than reinventing set-typestate is correct (canon).
- **One generic fold over the closed Core plus declared families** is the right traversal shape: the shared
  walk every target folds over, matching all 11 node shapes, with a family-fold extension for `Raw`
  sub-structure.

### What the direction still owes (the proof is not yet whole in design, let alone code)

1. **The runtime-bitmask half of the two-stage proof is not built.** `check_for` is the static path only
   (compile-time `ContainsAll`). The arriving-script path (compute the family/effect sets as a runtime bitmask
   over parsed nodes, then produce the same `Checked` witness through a checked path) is FIXME'd, and the
   `FamilyId` numeric-id assignment from a `Family` type is FIXME'd (`family.rs:17`). Under 08 this is the
   pure-dynamic end of the gradual spectrum for the two decidable axes, and per panel `06`/`08` it should be a
   *re-prove* (proof-carrying-code witness re-checking) not merely a re-check. Owed, and the design should
   record that the runtime bitmask is the AGT-derived gradual counterpart of the static `ContainsAll`, not an
   independent hand-checker.
2. **The `Leased` third axis is undesigned.** The direction fixes only `Supports`/`Permits` (family and
   effect); the lease proof (1316) has no typestate shape yet. And it cannot be a `where ContainsAll` bound,
   because a lease is not a
   decidable set membership; it is a per-program-inferred region property (08: gradual with a conditional
   dynamic residual). So `Leased<'a, T>` is produced by the region-inference *pass* certifying its result, not
   by a trait bound. Direction: a `Leased` witness that wraps or supersedes `Checked`, minted by the inference
   pass, gating `emit` the way `Checked` does. This differs in kind from the family/effect gate and that
   difference is the whole of debt 3; the code must reflect it rather than pretend leases are a third
   `ContainsAll`.
3. **The family-fold extension is a leaf stub.** `fold_core` visits `Raw` as a leaf (`codegen/lib.rs:104`
   FIXME). How a family walks its own payload sub-structure is the undesigned hook that certified generation
   needs (the generated Zig expands `Raw` into concrete family variants; the fold must recurse into them).
4. **The effect and family *computation* is undesigned.** The markers exist; the mechanism that computes a
   node's effect and family set from what it references (`202607192330`: "computed from what a form references,
   decidably") lives in the resolve or check pass and is not designed. This is where the LMS effect-summary
   discipline (synth purity doc) must land, including the effect-ordering edges.

**Verdict on the traits:** the direction is right and the static family/effect axes are well-specified. The
proof is roughly half-designed: the runtime-bitmask gradual counterpart, the lease axis as a pass-minted
witness, the family-fold extension, and the effect/family computation are the owed pieces, and they are exactly
the pieces the arc's theory (08) tells us how to shape. Building them fresh (the old cut is disregarded) is the
work, with the topics and 08 as the oracle.

## The metacompile sketch: evaluation and the pivotal experiment

`202607201352` traces where the data-versus-behavior line falls in the composed runtime. Its finding is sound
and load-bearing: every pipeline stage is a general engine reading per-language data (parse, resolve,
mask-check, lease inference, and even the codegen backend, which is per-ISA not per-language) *except* family
runtime semantics (stage 6), where behavior is data up to the richness of a general primitive vocabulary and
becomes irreducible behavior beyond it. Its recommendation is the hybrid: all-data for stages 0-5, a bounded L2
primitive vocabulary for family semantics, and a narrow generated-behavior escape for families whose semantics
cannot compose from that vocabulary.

The sketch is a logical trace (`WORKS`), not executed, and per the bench-in-harness and run-the-experiment
rules it owes its follow-up. The single highest-value next experiment it names is **the enumeration of the L2
primitive vocabulary across the real census families** (string ops for lua and jomini, record and list
construction, effect calls, arithmetic), because that enumeration decides the entire bridge shape: if the
vocabulary stays bounded, the runtime is effectively all-data and the escape hatch is rare; if it sprawls, the
generated-behavior hatch earns its place and the bridge carries per-family Zig code. Everything below about the
bridge is conditional on that experiment, and it should run before the bridge is committed.

## The bridge to Zig: what it composes of, and how

The right first move is to stop calling it "the C ABI." Under the reframe there are two boundaries:

- **Build-time codegen handoff (Rust to generated Zig, files on disk).** The Rust language compiler runs once
  per language and emits validated data (the family table, effect masks, the lease-rule schema, the value and
  IR wire layout) as Zig `comptime` consts, plus, for the hybrid, a small per-family lowering table and rare
  per-family behavior functions. The hand-authored general Zig engine is `comptime`-parametric over that data.
  The Zig compiler compiles the specialised engine into the shipped binary. This is where the double
  certification lives: rustc types the data and the generator (certification 1), the Zig compiler types the
  specialised engine and folds the `@compileError`s (certification 2). Neither ships.
- **Runtime embedding boundary (the actual C ABI at the consumer).** The host embeds the composed runtime. The
  earlier Zig skeleton sketches the right shape (disregarded as code, but the shape is sound):
  `vehje_runtime_new` / `vehje_runtime_execute` / `vehje_runtime_free` as `extern "C"`, `VehjeResult` as a plain
  `i32` for calling-convention portability. What crosses here is: (host to runtime) script source for arriving
  scripts or a bundled-residual handle; and
  (runtime to host) the produced value via the `VehjeSink` reserve/commit contract (1315). Panel `05`'s
  correction applies: this path must be panic-free (a Zig panic aborts the host across the C ABI).

### Three approaches for the codegen handoff

**Approach A, data-only.** Rust emits only data tables; the hand-authored comptime Zig engine specialises to
them and generates the IR type, the dispatch, and the checkers. This is 1627's load-bearing proposal.

- Payoff: certification is cleanest (rustc proves the data consistent, the Zig compiler proves the
  specialisation), the general engine is authored once, adding a language is a new data table.
- Cost: family runtime semantics are the one place data does not suffice, so this approach is really Approach C
  unless the L2 primitive vocabulary is rich enough to cover every census family with no escape.

**Approach B, generated Zig source.** Rust emits the whole specialised engine per language as Zig source (the
LMS / typed-Template-Haskell / Terra path).

- Payoff: maximum per-language specialisation.
- Cost: 1627 rejected it. You must prove the Rust generator emits correct Zig *source*, which is the hard
  typed-metaprogramming path, and the certification is on a string output rather than on a data value the Zig
  compiler then types. It is the pole to name and not take.

**Approach C, hybrid (recommended).** Data for stages 0-5, a bounded L2 primitive vocabulary in the
hand-authored engine, and a narrow generated-behavior escape for irreducible family semantics.

- The bridge composes of four artifacts: (1) a hand-authored comptime-parametric Zig general engine (the
  parser over a grammar table, the resolve walk, the mask checker, the region-inference algorithm, the tier-0
  interpreter, and the per-ISA codegen backend if native is ever built); (2) Rust-emitted per-language data
  consts (family table, effect masks, lease schema, wire and value layouts, the grammar table); (3) a
  Rust-emitted per-family lowering table (family node to primitive sequence, data); (4) rare Rust-emitted
  per-family behavior functions (generated Zig fns) for families whose semantics need a primitive the engine
  lacks.
- Certification: rustc proves the data and the escape-fn generation well-typed; the Zig compiler proves the
  specialised engine and the generated fns compile and every exhaustive switch is total.
- This is the sketch's recommendation and it honours op's stated taste ("ideally all data, hybrid if it cannot
  be as optimal"). Recommended, conditional on the L2-vocabulary experiment sizing the escape.

### The typestate-to-Zig chain, mechanism by mechanism

The shipped Rust typestate is the *source* of the proof; the generated Zig *carries it away*. The chain:

1. `Target::Supports` (a `Cons`-list of family markers) is proven consistent by rustc when a distribution
   composes its targets (the `ContainsAll` bounds). Rust emits the supported set as a Zig `comptime` const array
   of family ids.
2. Zig `comptime` builds, from that array, a tagged union holding exactly those families' `Raw`-expanded node
   variants, so an unsupported family is not a representable value (illegal-states-unrepresentable), and an
   exhaustive `switch` over it is compile-checked total (dispatch totality by the coproduct universal property,
   08).
3. `Target::Permits` (the effect `Cons`-list) becomes a `comptime` effect-mask const. The inclusion check is
   `script_mask & ~target_mask == 0`; for a bundled script `script_mask` is also `comptime` and Zig folds the
   whole check to a `@compileError` (the static end); for an arriving script only `target_mask` is `comptime`
   and one runtime bitwise op remains (the gradual dynamic residual). One source, stratified by the compiler
   (1627, 08's mix equation).
4. Non-null (the `Maybe` value form) becomes a Zig optional `?T`, non-null by construction, fully static (08).
5. The lease axis is the hard one: Zig has no lifetime types, so full region-validity is not statically
   Zig-provable. It leans on the Rust-side schema proof, the generated arena structure with region open/close
   markers, the distinct-generated-ref-types-per-region trick (partial, panel `04`/`05` flagged it least
   certain), and a residual runtime region-tag check (the gradual dynamic residual for leases, 08).

The clean statement: the Rust typestate is the proof that the emitted data is *consistent*; the emitted data is
what Zig *specialises on*; neither the typestate nor rustc ships, exactly as a compiled Rust binary is trusted
because rustc proved its source, not because the binary re-checks anything.

### How the two binding times share one arena

Bundled scripts (known at our build time) get the full static inclusion proof (the `Checked` typestate path)
and are pre-lowered by the Rust side into a tier-0 residual embedded as data. Arriving scripts get the runtime
bitmask (owed) and are parsed by the composed runtime into the same tier-0 arena. Both feed the same
interpreter. The tier-0 arena format is the shared image; the producer differs by binding time, the consumer is
one Zig code path. This is the reconciliation that also dissolves the tier contradiction above.

## The undesigned and vague pieces, each with candidate directions

1. **The value-node-kind schema (the produced-value domain).** The biggest owed piece. A produced value is data,
   not an expression, so it is not the 11 Core forms. Candidate kinds: scalar (arvo prim), text or bytes (a blob
   handle), record (named fields as child indices), list (a child-index range), variant or tagged value, and
   `Maybe` (absence, 1316). Directions: (a) a fixed closed value-kind set plus a `Raw`-value escape, mirroring
   the Core-closed family-open split exactly; (b) a fully family-parametric value domain where a family
   contributes value-kinds (jomini a route-tagged block value); (c) reuse the IR node arena format verbatim for
   values, treating a value as a normal-form IR term. I lean (a): closed value-kinds plus a `Raw`-value escape,
   symmetric to the node side, because it keeps the value-arena a closed record layout the certified generator
   can make total the same way it makes the node union total. (c) is tempting for format reuse but conflates
   evaluation forms with data and strains the lease region-tag placement.
2. **Effect ordering.** The forms do not order two `Writes`. Directions: (a) explicit effect-dependency edges per
   effectful node (LMS `Reflect`/`Reify`), computed once in resolve or check from read/write sets, so pure code
   moves freely and effects pin; (b) A-normal form, every effect named-bound so order equals binding order; (c)
   a monadic or CPS lowering. I lean (b) plus (a): A-normal form for the baseline order, LMS edges for the
   reorderable-pure-versus-pinned-effect distinction the synth doc flags as load-bearing (a miscategorised
   effect is a silent reorder bug). This is also what lets the scheduler and any future parallel evaluation
   reorder safely.
3. **The runtime budget (non-termination containment).** No totality axis. Directions: (a) a fuel counter
   decremented per step, a runtime error on exhaustion; (b) a host-set step or wall-clock budget passed through
   the runtime-env interface; (c) Zig's `@setEvalBranchQuota` shape but that is compile-time only and does not
   apply to arriving scripts. I lean (a) plus (b): a host-set step budget checked in the interpreter loop,
   surfaced as a first-class runtime error, consistent with "budgets as containment." For the ikiuni consumer
   (evaluated every frame) this budget is also the per-frame time slice.
4. **The closure lease hard case.** `Lambda` capture is the real T3 obligation. Directions: (a) closure
   conversion that copies captured values into the closure's own region, so nothing escapes (Tofte-Talpin's
   actual mechanism, sound by construction); (b) forbid a closure from escaping its defining region unless the
   inference proves the captured values' leases dominate, else a compile error (the strict-by-design stance
   1316 takes); (c) derive the capture's link-or-consume bit from the captured operand's declared effect (08's
   proposal: retention through a boundary is a `Writes<HostEnv>` effect, so the lease bit is a function of the
   effect, not an independent declaration). I lean (a) as the sound default (copy captures into the closure
   region) with (b) as the optimisation when copy is provably unneeded, and (c) folded in so the effect table is
   the single trusted surface. This is the concrete shape of the debt-2 reckoning the panel left open.
5. **The two-arena reconciliation.** The generation-time byte arena (`Lit` payloads, compile-time, in the IR
   wire blob) versus the runtime value-arena (produced values, over the sink). Directions: unify into one format
   or keep separate. I lean keep separate (different lifetimes and producers) but share the `(offset, len)`
   into-a-blob discipline and the index-not-pointer link rule, so a reader is one code path over two arenas.
6. **No-alloc macro expansion (the debt's hardest instance).** Macro expansion is IR-to-IR at the composed
   runtime's compile stage; the output IR needs arena space not known ahead. Directions: (a) a bump arena over
   host-lent memory, expansion fails on overflow (bounded, the transfer model's host-lent discipline applied to
   the compile stage); (b) streaming expansion, expand-and-emit incrementally over the panel's backward-only
   post-order so the frontier is bounded; (c) a fixed fuel plus fixed arena hard cap. I lean (a) plus (b): a bump
   arena over host-lent memory with the streaming spine keeping the frontier fixed-size, which ties directly to
   the cross-chunk lemma the panel put on the critical path. This is the owed no-heap macro experiment.
7. **The input side under the reframe (grammar as data).** The lexer is not generic (`Lexer<'a>` fixed,
   extension-point topic), and the input-side contract (`202607192300`) predates the metacompiler reframe. Since
   arriving scripts are parsed inside the composed runtime, the parser must be Zig (or generated), not a Rust
   parser combinator. Directions: (a) a declarative grammar table (PEG or LL data) a general Zig parser
   interprets, pure data, with a narrow per-language lexer hook for significant indentation and custom literals
   (the metacompile sketch stage 0 escape); (b) a per-language generated Zig parser (behavior, the Approach-B
   pole for the input side); (c) a Rust-side parser for bundled scripts only, with the runtime parser as a
   separate general engine for arriving scripts. I lean (a): grammar-as-data with a bounded lexer hook, matching
   the all-data finding, so a grammar plug-in is a data table plus at most a small hook. This is the
   least-designed axis under the reframe and owes its own topic.
8. **The bundled-content binding time.** 08 names three binding times but the artifacts mostly discuss two. The
   bundled case (a script known at our build time, lowered by Rust, shipped as a data blob) is where the shipped
   `wire.rs` residual fits cleanly and where the static `check_for` proof already applies. Directions: support
   bundled as a first-class optimisation (full static proof plus pre-lowered residual) or always parse at
   runtime. I lean supporting it: bundled scripts get the strongest proof and skip the runtime parse, arriving
   scripts get the gradual runtime path, both share the arena and the interpreter.

## Related theory to bank

Mentioned in the arc, to reference when relevant:

- **Tofte-Talpin region calculus** and **Calcagno-Helsen-Thiemann** syntactic soundness (08): the lease
  metatheorem's calculus and proof technique; the closure hard case (piece 4) is the part they actually cover.
- **Talpin-Jouvelot type-and-effect** (08): region and effect are one system, the basis for deriving the lease
  bit from the effect (piece 4c).
- **AGT (Garcia-Clark-Tanter), gradual effects (Bañados-Garcia-Tanter), the mix equation
  (Jones-Gomard-Sestoft), MetaML annotation soundness (Taha-Sheard)** (08): the staged-gradual frame under the
  whole bridge; the runtime-bitmask path (owed piece in the traits) is the AGT gradual counterpart of the static
  `ContainsAll`, and the comptime fold is the mix equation.
- **LMS effect summary** (synth purity doc): the effect-ordering edges (piece 2).

Not yet mentioned in the arc, worth banking:

- **A-normal form (Flanagan, Sabry, Duba, Felleisen, "The Essence of Compiling with Continuations," PLDI 1993).**
  The clean way to make effect ordering explicit without a `Seq` form (piece 2): every intermediate is named,
  order is binding order, and pure bindings are free to move. It is the IR discipline that lets the 11 forms
  carry effect ordering for free.
- **Defunctionalization (Reynolds 1972; Danvy-Nielsen 2001).** The theory under "family behavior as a lowering
  to a primitive vocabulary" (the sketch's stage 6, the Approach-C escape): a family's behavior is defunctionalised
  into a first-order primitive sequence plus a data tag, which is exactly the "data up to the vocabulary,
  behavior beyond it" line and the tool for keeping the escape narrow.
- **Closure conversion into regions (Tofte-Talpin; Aiken, Fähndrich, Levien, "Better Static Memory Management,"
  PLDI 1995).** The mechanism for piece 4a (copy captures into the closure region) and the standard treatment of
  region inference for higher-order programs, which is precisely debt 2's hard case.
- **Attribute grammars and tree-walking evaluators (Knuth 1968); GLL / PEG parsing (Scott-Johnstone; Ford
  2004).** The theory for grammar-as-data (piece 7): a declarative grammar table plus a general parser is an
  attribute grammar evaluated by a general engine, and PEG or GLL is the data shape a general Zig parser can
  interpret.
- **Object-capability / capability-safety (Miller 2006).** The right model for the runtime-env host-call
  surface: the effect set a residual carries is exactly the set of capabilities the host lends the runtime, and
  framing the runtime-env interface as an object-capability surface makes "the runtime can only touch what its
  effect set permits" a capability-confinement property rather than a convention. This is a strong frame for the
  embedding boundary the sketched `vehje_runtime_execute` will grow.
- **Sea-of-nodes / RVSDG (Click-Paleczny 1995; Bahmann et al. 2015).** If the effect-edge IR (piece 2) grows
  past A-normal form (for parallel evaluation or aggressive reordering), the regionalised value-state-dependence
  graph is the IR shape that carries effect and region edges as first-class structure, and it composes with the
  lease region nesting.
- **Nielson-Nielson two-level lambda calculus (1992).** 08's `λ_veh` names it for the binding-time annotation;
  banking it explicitly gives the value-node-kind schema and the bundled-versus-arriving split (pieces 1 and 8)
  a two-level typing to hang the binding-time annotation on.

## Summary

The 11 Core forms hold; the backdrop owes companion domains and metadata, not a new form, with a resumable
algebraic-effect `Handle` the only plausible future-consumer twelfth. The proof-root traits are correct and
half-built: the static family and effect axes are real, the runtime-bitmask gradual counterpart and the
`Leased` third axis (a pass-minted witness, not a `ContainsAll` bound) and the family-fold extension and the
effect/family computation are owed. The bridge is two boundaries (a build-time codegen handoff and a runtime
embedding boundary), and the hybrid data-plus-narrow-behavior shape is the defensible direction, conditional on
the L2-primitive-vocabulary enumeration, which is the single pivotal next experiment. One live contradiction
needs op's decision: the committed ABI topic and the shipped `Tier::Native` encode the program-centric and
native-as-tier framings the three-codegens reframe retired, and the three-binding-time reconciliation (wire.rs
as the binding-time-neutral image, native as a target not a tier) dissolves it.
