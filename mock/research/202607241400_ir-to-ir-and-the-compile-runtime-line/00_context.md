# Context: IR to IR expansion and the compile/runtime line

A sequential expert panel. Each expert writes one deliverable file in this directory. Experts
after the first also read every prior deliverable in this directory and may build on, extend,
or dispute it. The maintainer (op) will comment between rounds. This brief is common ground;
it states facts and the question, not a preferred answer.

## The framework in one paragraph

vehje is an embeddable, multi-input multi-output IR framework (NOT a language). A consumer
plugs in a grammar on the input side (surface syntax lowered to IR) and a target on the output
side (declares what it supports, emits). The framework owns a shared IR (a Core substrate of
twelve forms plus a `Raw` family-extension node), the Core passes (resolve, a graded check,
lowering), the output machinery, and a runtime ABI. It has two sides joined by a C ABI defined
in Rust: a Rust dev-time compile side that lowers, checks, discharges every build-environment
effect, and emits a `Checked` residual; and a Zig runtime side that executes the residual
against the runtime environment, small and embeddable, pulling no Rust runtime. `#![no_std]`,
no alloc, on both sides; the IR and passes operate over caller-lent arenas, never a heap.

## The artefact under evaluation

Two coupled questions.

### (A) The IR to IR expansion architecture

Macro expansion and constant folding are being reframed as IR to IR passes. Rather than
mutating one arena in place, each pass reads the current IR and emits an expanded IR into a
fresh arena through the node `Builder` (the same fold-and-emit machinery a target already uses
to fold IR to bytes; here it folds IR to IR). Constant folding inlines a statically-known value
literally into the emitted IR and drops the branch not taken; macro expansion splices a macro's
expansion in place of its call. The passes iterate to a fixpoint: run expansion, and if the IR
changed, run again on the output, until a run produces no change. Only the stable expanded IR
then feeds the rest of the pipeline (check, the cheap lowering subset, the inclusion check,
emit). Two IR-production directions name the shape: input to IR is the grammar; output to IR is
expansion. Both build into caller-lent arenas, so both stay no-alloc.

The current lowering source (`crates/vehje-lower/src/lib.rs`) instead uses a redirecting
`Rewrite` side-table that only remaps existing node indices downward and cannot create nodes;
under it, macro expansion, A-normal-form hoisting, and primitive const folding are no-ops. The
reframing above replaces that vehicle for the node-producing passes with the IR-to-IR emit path.

### (B) The compile/runtime line, and what literal const-inlining commits the framework to

Inlining a constant literally specialises the program to that constant: the residual is no
longer a general program parameterised over that value, it is a residual specialised to it. The
question this raises: where does the compile/runtime line fall, and what does literal
const-inlining (and macro expansion) commit the framework to regarding ahead-of-time
compilation, just-in-time compilation, and interpretation. Concretely: is any part of the
execution model coherently "fully interpreted" once constants are inlined and macros expanded,
or does that specialisation mandate a compile step; if a compile step, where does it live
(the Rust compile side for statically-known programs, a runtime load-time stage for
runtime-arriving scripts, both); and how does that relate to the tier model and the
two-artifact split the design already carries.

## The design oracle (read these; cite file:line)

- `.claude/CLAUDE.md` (repo root): the two-side compile/runtime model and the tier statement
  ("the residual crosses tier-tagged: a flat serialized IR arena (baseline), an optimized
  bytecode, or native code ... the tier is orthogonal to correctness; the effect proof is
  discharged in Rust before any lowering"; "the compile-time / runtime split follows the effect
  axis").
- `crates/vehje-codegen/DEEPDIVE_OUTPUT_SPECTRUM.md.tmpl`: output as a spectrum from
  interpretation to transpilation; "proof-directed strategy selection" (interpret / bytecode /
  native chosen per region by evidence); the bench-settled findings (native ceiling 1.0x to
  1.5x over a good interpreter for scalar code, plain switch beats tail-threading ~1.7x, the
  branch strategy is tier-dependent); copy-and-patch as an opt-in accelerator, not a keystone.
- `crates/vehje-codegen/DESIGN.md.tmpl`, `crates/vehje-codegen/src/lib.rs`: `Target`,
  `Checked`, `check_for`, the one generic `fold_core`.
- `crates/vehje-runtime-abi/DESIGN.md.tmpl` + `DEEPDIVE_VALUE_TRANSPORT.md.tmpl`: the residual
  is "a control-flow-graph-of-blocks program, not a flat value ... the debt that the runtime
  contains a compile stage lands" here; `Tier` "narrowed to name only the internal execution
  form (interpret the arena versus run the bytecode), not a native-versus-managed axis. Native
  is a method-agnostic point on the output spectrum, chosen where a per-region grade proves it
  wins, not a tier tag here."
- `crates/vehje-runtime-driver/DESIGN.md.tmpl` + `DEEPDIVE_DISPATCH_AND_DECODE.md.tmpl`: the
  compiler-side dispatch over the ABI and the Zig runtime embedding.
- `crates/vehje-runtime-gen/DESIGN.md.tmpl` + `DEEPDIVE_PACKAGE_MANIFEST.md.tmpl`: the
  validated-data package the Zig runtime is specialised from.
- `crates/vehje-lower/DESIGN.md.tmpl` + `DEEPDIVE_LOWERING.md.tmpl` + `src/lib.rs`: the current
  redirecting-Rewrite lowering and its no-op macro/anf/const-primitive passes.
- `crates/vehje-ir/DESIGN.md.tmpl` + `src/`: the Core twelve forms, `Raw`, the arena/Builder,
  the structural hash, the graded proof spine (effect, reach/lease, binding-time, assurance).
- `crates/vehje-typecheck/src/lib.rs`: the graded check; `Knowledge`/`BindingTime` (the
  binding-time axis of the proof spine) and its currently-deferred discharge.
- `mock/design_rounds/202607241330_topic.ir-to-ir-expansion.md`: the topic that opened this.
- Bench evidence: `mock/design_rounds/202607240800/202607240700_topic.bench-full-run-complete-and-locked-findings.md`
  and `mock/design_rounds/202607240130/202607240015_topic.full-arc-bench-and-evidence-findings.md`.

## Constraints the design holds fixed (not up for the panel to relax)

- `#![no_std]`, no alloc, both sides. Caller-lent arenas. No `dyn`, no `TypeId`, no `std::any`
  in framework code; compile-time composition.
- Two artifacts: a Rust dev-time compile side and a Zig runtime side, joined by a C ABI defined
  in Rust. Rust never runs in the runtime.
- The graded proof (effect + lease/reach + binding-time + assurance) is discharged in Rust
  before any lowering; the tier is orthogonal to correctness.
- The bench findings above are locked evidence, not to be relitigated from intuition (they may
  be extended or reinterpreted, but a claim that contradicts them needs its own evidence).

## The question, stated plainly

Evaluate (A) the IR-to-IR expansion architecture on its own terms (soundness, no-alloc fit,
fixpoint termination, where the passes belong, what the family-extension seam is), and (B) the
compile/runtime line the design must commit to once constants are inlined literally and macros
are expanded: name what is ahead-of-time compilation, what is just-in-time, what is
interpretation, where each lives across the two artifacts and the runtime's own "compile stage"
debt, and whether the tier model and the output spectrum as currently framed are the right
shape for that commitment or need to change. Reach your own conclusion from the source and the
evidence.

## Deliverable

Write your analysis to `<your-slug>_<goal>.md` in this directory. Dense, cite `file:line`,
name real techniques and primary sources, and separate what you are confident in from what is
an open question for the next expert or for op. Structure it however serves the argument, but
include: your one-line reading, the strong points of the current design, your findings
(each: the issue, why it matters, a concrete direction), the compile/runtime-line answer with
the AOT/JIT/interpretation terms pinned for this framework, and the open questions you are
handing forward.
