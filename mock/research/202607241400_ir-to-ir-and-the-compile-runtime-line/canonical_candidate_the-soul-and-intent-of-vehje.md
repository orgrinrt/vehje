# The soul, spirit, and identity of vehje, enumerated

**Date:** 2026-07-24
**Author:** worker fork, soul-and-intent directive
**Method:** read the latter, intent-bearing topics of canonical round `202607240130` in full (identity recenter
`202607202330`, core re-derivation `202607210120`, converged shape `202607202001`, Carmack rebuttal
`202607202205`, three-codegens `202607201513`, value transfer `202607201315`, lease axis `202607201316`,
full-arc bench findings `202607240015`, consolidation `202607240100`), plus the ir-to-ir panel deliverables
(Rompf, Fallin, the three sibling forks, the idealistic synthesis). Then put the vague words "intent" and
"spirit" into concrete, enumerated language, with provenance, avoiding the mechanism-level specifics that date.

**Precedence obeyed** (op, 2026-07-24, plus this directive): the latter topics trump the changelists and the
consolidation where they disagree; hard bench data trumps topics, and therefore trumps any prior-art cite; and
the canonical *spirit* trumps the canonical *letter*, because the round is months of evolution and a literal
line written early may no longer carry the intention behind it. This document is the spirit, stated so future
work can be checked against it rather than against a stale sentence.

**What this is, and is not.** Not a drift audit (the sibling forks did that) and not a forward proposal (the
idealistic synthesis did that). This is the reference: what vehje IS, what it believes, what op has stood on,
what the benches taught, and how tightly we hold each borrowed idea. When someone says "that drifts from the
intent," this is the document that says what the intent was.

## 1. The identity: what vehje is

Vehje is an embeddable, multi-input, multi-output IR framework. It is not a language. A grammar plugs in on the
input side and lowers a surface syntax into a shared IR; a target plugs in on the output side and emits. Vehje
owns the shared Core, the proof axes, the output machinery, and the runtime ABI. No single language is vehje;
every language is a consumer (`202607202330:34-37`).

Its identity, stated as a single proposition: **the type system is the verification layer, and certified
generation is the mechanism** (`202607202330:34`). Everything else is downstream of those two commitments. The
enumerated facets of the identity:

1. **A framework serving a census of consumers, equally.** The representative and largest consumers lean toward
   authoring and templating, not long-lived runtime logic: mockspace's columnar-query plus procedural-document
   DSL, polka-dots' `.polka`, their kin, closer to typst or scribble than to a game's behavior scripts. They
   evaluate the build environment away and emit configured data or documents; their speed concern is the compile
   stage and the emit, so content stacks do not bloat load times. That end is real and first-class
   (`202607202330:51-57`). The flagship game-runtime consumer (ikiuni) is one consumer, not the reason vehje
   exists.
2. **A spectrum, not a single purpose.** The simple templating consumer and the ambitious machine-code consumer
   are the *same machinery aimed at different points* of one output spectrum. Serving the simple end is not a
   reason to abandon the ambitious end, and the ambitious end is one option among many, not the only door
   (`202607202330:59-65`).
3. **Two artifacts, Rust in only one.** A dev-time Rust language compiler (compiles a language definition, proves
   its disciplines sound, emits validated data plus structural proofs, never sees an end-user script, never
   ships) and a shipped Zig composed runtime (a hand-authored general engine, comptime-specialised to the
   Rust-emitted data, runs every script) (`202607202330:166-183`, `202607201513:26-37`).
4. **A metacompiler with a unified graded proof.** Taken whole, vehje is a metacompiler with a unified graded
   proof of three axes, whose lease grade is a reachability qualifier, generating a native output point per
   language from one semantic definition by whatever method benches best (direct instruction selection for the
   short-lived residuals the authoring majority produces), with a load verifier bounded by its own lease-lattice
   height. No shipped system does all of it. The founding sentence (`202607202001:161-164`) wrote that native
   point as "a copy-and-patch JIT per language"; the benches demoted copy-and-patch below direct isel (see 5.viii
   and §6), so the identity keeps the shape (a native output generated per language from one definition) with the
   method left open, per the recenter's "native is method-agnostic" (`202607202330:78-83`). Letting the demoted
   specific ride the identity sentence, even rhetorically, is how a superseded mechanism creeps back by
   familiarity; the sibling negative fork caught the earlier phrasing doing exactly that.

## 2. The soul: the load-bearing convictions

Each is a belief the design holds, stated in words, with where it was set down.

1. **The type system is the verification layer.** This is why vehje is written on this stack at all. The proof
   is the point; the runtime is a dumb evaluator that inherits proven-safe programs (`202607201316:26-32`,
   `202607202205:44-46`).
2. **Certified generation: prove before lowering, then compile the proof away.** The proof is discharged in Rust
   before any lowering, then compiled into a doubly-certified runtime that carries no prover. Rust emits
   validated *data*, not source; the Zig runtime's own compiler specialises to it; both are certified at *our*
   build, neither at the consumer (`202607202330:43-44`, `:166-183`). Rust-emitting-Zig-source (the LMS-hard
   route) was explicitly rejected as hard-to-certify (`202607202330:180`, grounding `1627`).
3. **One graded (co)modal proof spine, unified by construction, not by analogy.** Binding time is a graded
   necessity modality, effects a graded monad, lease and usage a coeffect whose grade is a reachability
   qualifier, and assurance itself a grade. One soundness theorem; the per-axis results are its corollaries and
   the axis interactions are its typing rules, not unwritten case analyses (`202607202330:40-46`,
   `202607202001:40-53`, `:242-262`).
4. **Inclusion, not coverage.** A target declares the family set it `Supports` and the effect (operation) set it
   `Permits`; a program using anything outside those declared sets is refused outright, with the offending
   construct named. Honesty about a smaller declared surface, enforced by the type system rather than hoped for
   (`202607210120:51-54`, output-spectrum deepdive).
5. **The one generative idea: effects, host-calls, and macro expansion are one handler discipline.** An effect is
   an operation; a handler is what services it. A host-call is a runtime-effect operation the host handles. Macro
   expansion is a compile-stage-effect operation the compile stage handles. So the build-versus-runtime split is
   not drawn by hand: it is "which stage provides the handler," which is the binding-time coordinate
   (`202607210120:43-49`). This is the single idea the whole round circles; the four commitments below are its
   shadows (see 8).
6. **Output generation is a spectrum from interpretation to transpilation.** Interpreting and transpiling are the
   same operation aimed at different targets, not two mechanisms. A target is a point on the spectrum. "Native"
   is a real point on it (a genuine endgame), but it is *not* a privileged tier above the others
   (`202607201513:63-85`, corrected at `202607202330:67-87`).
7. **Three named codegens, and the bare word retired.** Runtime generation (Rust compiles the language, once per
   language, at dev time), macro expansion (IR to IR, per script, at the compile stage), and output generation
   (IR to output, the spectrum, per script). Collapsing these three into one word "codegen" is what kept
   derailing the design (`202607201513:40-61`).
8. **No garbage collector, no null.** The lease and region proof is discharged statically, so the runtime
   inherits proven-valid, non-null references with no collector and no reference counting. Absence is an explicit
   `Maybe` value form the consumer matches on, not a nullable reference: notko's fallibility ladder applied to
   the runtime value model (`202607201316:26-46`, `:72-74`).
9. **No heap, ever, on both sides.** `no_std`, no alloc, including the compile-side passes; there is no host-side
   dispensation. The IR and the passes operate over caller-lent arenas and host-lent budgets; the framework never
   allocates, the host owns the memory (`202607202330:143`, `202607201315:48-49`).
10. **The host-lent bounded budget is the one universal resource discipline.** Memory windows (the reserve/commit
    value-transfer sink), reachability leases, and no-alloc bounded multi-shot continuations are three uses of
    one shape: a fixed-capacity region the host provides up front, statically validated to fit
    (`202607201315:48-60`, `202607210120:92-112`). This is the stack's determinism-and-boundedness ethos applied
    everywhere it can reach.
11. **Strict by design.** Lease-inference failure is a compile error that forces an explicit annotation, never a
    silent fall back to a nullable, collected value. The strict-by-default posture applied to lifetimes
    (`202607201316:44-46`).
12. **One truth, many projections.** One declarative language definition (a graded algebraic signature carrying
    its own grammar) is the single source; the Rust compile-side typestate and the Zig runtime consts are
    lens-projections of it, differential-checked, so there is no trusted emitter. The node algebra itself has
    three projections (the eliminator IR you fold, the introduction value-domain, the introduction-compiled
    stencils), one declaration viewed three ways (`202607202001:141-151`, `:264-280`).
13. **One shape does the work of many, but count the bounds honestly.** The living aesthetic is to find the
    shape where one decision buys many: one graded judgment with the axis theorems as corollaries, one signature
    with several projections, one continuation-linearity theory serving both streaming ends. That aesthetic is
    soul. Its most-cited instance was retired, and the retirement is itself instructive: the earlier "one depth
    cap doing six jobs" (`202607202001:386-388`) was found on adjudication to be a pun, because two of the six
    were a width or a length wearing the word "depth," and it was replaced by three genuine finite structural
    bounds (environment-width, nesting-depth, input-length), each doing real work, AARA-inferred and folded into
    the graded spine (`202607202055:95-99`, attack A9). Hold the aesthetic; count the bounds as three, not one.
    (Verified against `202607202055`; my first draft carried the superseded six-jobs-one-cap form, which the
    sibling negative fork correctly caught.)

## 3. The governing ethos: the maximal shape, designed through the hard parts

This is the deepest layer of the spirit, and the one most easily lost. Stated in its own words:

- **Design the unification now, completely, before any code.** "A hole in a design is filled by finishing the
  design, not by choosing a design with fewer holes because it is easier to reach today" (`202607202205:24`).
- **Reject re-tiering as the defer instinct.** Taking an ambitious unified mechanism and moving it to a "later
  north star" while a simpler local mechanism ships is "the defer instinct producing a locally-cheaper artifact
  that forecloses the thing we actually set out to build" (`202607202205:27-42`). "A sound floor" and "a
  sufficient floor" are different claims; the floor being buildable today is not permission to stop designing
  above it (`:140-143`).
- **The maximal shape is not ambition for its own sake.** It is the reason vehje is written in a language whose
  type system is the verification layer, on a stack whose thesis is that the substrate is designed to the bottom
  before it is used (`202607202205:44-46`).
- **Design through the fault, or prove the exact blocking constraint.** Where a piece genuinely cannot be built
  under the constraints (no-alloc, dual-locus, no-JIT-permission), prove that concretely with the exact blocking
  constraint and design the maximal thing that *does* hold, never a smaller architecture chosen for being easier
  (`202607202205:148-152`).
- **Novelty means proven-in-literature, made load-bearing, fitted.** Not novelty for its own sake: a paradigm
  proven in the literature but not yet in a production runtime, made load-bearing and implemented, fitted to the
  other calls rather than taken as-is. "Excellence is composing established pieces until a new shape falls out
  that supersedes them" (`202607202001:26-27`, `:239`).

**The counterbalance, equally load-bearing (the boots-not-steps correction).** The maximal-shape ethos is not a
license to narrow onto one exciting corner. The design once drifted into reading as a single-purpose JIT-and-
native scripting runtime for one game; op corrected it ("the walk was correct, the boots were wrong: change the
boots and keep walking"), and *also* rejected the over-correction that tried to drop the native endgame and demote
the JIT (`202607202330:19-31`, `:141-149`). Both errors are rejected. Keep the whole shape; remove only the
framing that makes one part read as the whole. The ambitious end and the templating end are both first-class.

## 4. op's standing design calls (intent over specifics; op self-corrects)

These are the calls op has stood on, stated as intent rather than as the specific mechanism that carried them at
the time. The caveat op gave for this document: even op can be wrong and correct it later, so where op has
revised, the *latest* revision holds.

1. **The Core changes to fit the tech, never the reverse** (`202607210120:6`). The Core was re-derived from the
   evolved architecture, not patched to preserve an earlier census cut.
2. **The type system is vehje's strength, so adopt the proven-but-not-yet-productionised paradigm and implement
   it.** This is the call that made reachability types the primary lease paradigm, graded types the spine, and
   algebraic effects the Core's twelfth form (`202607202001:86-88`).
3. **Reachability types as the primary lease paradigm** (op overriding an earlier "ship generational references,
   aspire to reachability" lean, `202607202001:84-88`). A recorded self-correction: op moved from
   generational-first to reachability-first, with generational references demoted to the dynamic residual at the
   avoidance boundary.
4. **Bounded multi-shot `Handle` via a host-lent budget** (CR1, op 2026-07-20, `202607210120:92-112`): not
   one-shot-only and not heap-multi-shot, but full multi-shot expressiveness on a host-lent, statically-validated
   budget. Named as genuine vehje novelty.
5. **The three settled mandates** that drove the Core re-derivation: adopt `Handle` as the twelfth form (D3),
   split the effect lattice into an effect graded monad plus a binding-time modality (D4), and treat
   content-as-values as the signature's introduction projection (D5) (`202607210120:30-40`).
6. **Design through the hard parts, reject the floor-first re-tiering** (op's rejection of the Carmack verdict at
   the frame, keeping its faults as the agenda, `202607202205` throughout).
7. **The identity recenter** (op's 2026-07-20 correction after re-reading the round in order): keep the whole
   shape, fix the narrow framing (`202607202330:11`).
8. **Native is a real endgame point, method-agnostic** (op correcting the stale `202607201513` lines "there is no
   native endgame" and "generate Zig for native"): native is a genuine destination on the output spectrum, ours
   to build by whatever method serves, not bound to Zig and not a privileged tier (`202607202330:67-87`).
9. **The live 2026-07-24 calls, which hold as the latest word:** macro/const "jit" expansion is recursive into
   the single caller-lent arena (not a two-arena emit); *and* double-buffering (two arenas) is a legitimate
   candidate if it benches better, so the arena strategy is a bench-decided fork, not an assertion. The
   precedence rule op set the same day: hard data over topics over changelists. And the framing that this
   consolidation round is itself canonical, to be worded to the same bar as `202607240130`.

The pattern across these: op's calls are directional and self-correcting. The intent is stable (the type system
is the verification layer; the maximal shape; boundedness; certified generation), while the specific vehicle
(generational-vs-reachability, single-vs-double arena, copy-and-patch-vs-direct-isel) is negotiable and often
settled by bench rather than by decree. Read op's calls as intent; let the latest correction and the bench
override the earlier specific.

## 5. The core discipline: bench it, always

The single working discipline under everything above: **bench it before choosing.** A design fork about
which-is-faster or which-shape-is-cheaper is not resolved by first principles or prior-art authority; it is
resolved by building the candidates and measuring. The full-arc bench topic (`202607240015`) is the record.

**The harness is a first-class asset, and we improve it.** Each variant compiles as its own cdylib in subprocess
isolation so no cross-variant LTO or inlining can blur a comparison; every variant is cross-validated byte-exact
against an oracle so a variant computing a different answer is rejected, not timed; timing uses hardware counters;
known LLVM defeats (if-conversion of a cheap branch, copy elision, ICF, auto-vectorisation of a naive baseline)
are handled explicitly (`202607240015:30-35`, `:263-278`). We have pushed features upstream to mockspace
repeatedly: the cost-model regression fit `total = S + k*I`, declarative axis-matrix variant-crate generation,
the disassembly duplicate-check fairness guard, the reference-floor ratio column, and the `timed_calibrated!`
auto duration-floor macro (`202607240015:186-191`). The harness should be reached for actively to support a call
we want to make, not treated as ceremony. **A bench that measures the wrong thing is worse than no bench**: the
ABI arc over-claimed on its first pass and was corrected under adversarial review, recorded in full rather than
smoothed (`202607240015:26-28`, `:217-234`).

**What the benches taught (the settled results, with their honest caveats). The later, more rigorous re-measure
governs where it refined an earlier number.**

i. **Native is a modest multiplier for scalar code, not a keystone.** A ~1.0x to 1.5x ceiling over a good
   interpreter; the interpreter's dispatch is small enough that an out-of-order core hides it. The transformative
   order-of-magnitude win needs auto-vectorisation, which copy-and-patch cannot do, so the 10x class belongs to a
   vectorising JIT for the rare genuinely compute-bound consumer (`202607240015:68-78`, `:212-214`). This
   vindicates "native is never the driver." (The consolidation's stale ~2.0x, and Fallin's inherited 2.0x, are
   doubly-superseded; ~1.5x over the best interpreter is the honest ceiling.)
ii. **Dispatch is tier-and-shape-dependent, not a flat winner.** A plain switch is fastest for vehje's lean
    ~25-primitive straight-line IR (the switch branch is well-predicted; tail-threading's machinery costs more
    than it saves for so few op kinds). *But* preserve-none threading wins on real control flow (the CFG-of-blocks
    regime, loops and branches), where the trace cell runs ~0.44x of switch. So: switch inside straight-line
    blocks, threading at the CFG terminator transfers (`202607240015:58-66`, `:198-200`). The consolidation's flat
    "switch beats threading 1.7x" is the stale form of this; the tier-dependent split is the truth.
iii. **The optimiser is fold + CSE + DCE, not the e-graph.** Equality saturation grows a tame linear ~5.2x (not
    exponential), which validated it as dev-time-only behind a bounded window; but the later re-measure put its
    marginal contribution at zero-or-negative and parked it behind a named trigger. The cheap subset
    (const-fold, CSE, and DCE) is the endorsed optimiser; DCE belongs as a named third arm. The e-graph is a
    reserved conditional seam, not the frontier (`202607240015:126-127`, plus the bench-supremacy retake). The
    consolidation's framing of the e-graph as "the one genuinely original research piece" is drift.
iv. **The batched-column C ABI is the one ABI-shaping result.** Vertical/SoA SIMD is the biggest single win
    (~4.8x on a symmetric baseline), and it survives the FFI boundary byte-exact (2.3x to 4.1x). The crossing
    itself is nearly free (~9 ns warm). So the runtime exposes one runtime-W batched column entry taking W >= 2,
    no per-W symbol zoo and no crossing-amortisation cleverness, because the runtime, not the ABI, owns the SIMD
    width (`202607240015:217-260`, `:293`).
v. **Recursion is a hard compiler wall; the kernels are iterative.** Recursive Zig comptime SIGSEGVs at ~2500
   depth; the iterative defunctionalised work-stack folds 100k nodes clean. This promotes defunctionalization
   from "the principled route" to "the only route," and binds the comptime kernel, the load verifier, and the
   e-graph extractor alike (`202607240015:90-94`).
vi. **The wire format keeps operands inline; the exact count is still open.** Pool indirection is the measured
    expensive part (10.7 vs 4.4 ns/node), so operands stay inline; the specific inline count washed to a null
    result at scale and is a measure-then-lock decision against a real census corpus (`202607240015:44-57`).
vii. **The pieces compose with predictable additive cost.** The capstone: a 50,000-node script runs the full
    pipeline end to end in 3.4 ms, compile-stage-dominated (3.24 ms) with a cheap runtime (127 us), which matches
    the authoring-and-templating majority's compile-heavy, trivial-execute profile exactly; incremental scaling
    answers the load-bloat concern (4 ms cold, 0.003 ms warm, 7.7x parallel) (`202607240015:113-117`, `:132-142`).
viii. **Native, refined:** direct instruction selection compiles ~2x cheaper than the copy-and-patch stencil at
    equal warm speed, and the authoring majority produces short-lived residuals, so direct isel is the native
    point and the copy-and-patch stencil is de-prioritised (bench-supremacy retake; the consolidation privileged
    copy-and-patch, which the data does not support).

The through-line of the evidence: the value is in the cheap binding-time-directed reducer and the vertical-SIMD
batched ABI, not in the e-graph or the copy-and-patch stencil. The bench is the authority that says so.

## 6. Prior art, by degree of commitment

The design cites a great deal of prior art. What matters for the spirit is *how tightly we hold each one*. The
governing rule: **a bench trumps any prior-art cite.** A paradigm we imported on a paper's authority stays only
as long as the measurement agrees.

**Married (load-bearing spine; adopted by construction).**
- Graded (co)modal type systems: binding-time as graded necessity modality (Davies-Pfenning), effects as graded
  monad (Katsumata), lease/usage as coeffect (Petricek-Orchard-Mycroft), combined by Gaboardi et al. 2016, with
  Granule as the implementability evidence (`202607202001:44-45`, `:242-249`). This is the proof spine.
- Reachability types (Bao-Wei-Bracevac-Jiang-He-Rompf, OOPSLA 2021, and the flow-sensitive-effect and avoidance
  successors, "Free to Move" 2025) as the primary lease paradigm (`202607202001:84-91`).
- Algebraic effects and handlers (Plotkin-Pretnar, ESOP 2009) as the semantic foundation under the twelfth Core
  form `Handle`, unifying the effect set, host-calls, and macro expansion (`202607210120:35`, `202607202001:209-214`).
- The certified-generation lineage: CompCert's trusted-rim framing (naming the one uncertified step), proof-
  carrying-code (Necula-Lee) on the data blob, and certifying (per-instance re-checkable, `Assurance::
  ReprovenPerInstance`) rather than certified (`202607202001:56-64`).
- The lens principle (Foster et al.): both language-facing sides as projections of one signature (`202607202001:62-64`).
- Zero-copy value transfer (Cap'n Proto, FlatBuffers, Arrow, rkyv converged on the same arena shape), the
  reserve/commit sink as an iteratee (Kiselyov), the host-lent window as an object-capability (Miller)
  (`202607201315:20-37`, `202607202001:359-360`).
- Region inference (Tofte-Talpin, MLKit, Cyclone) for the lease axis, deliberately dropping the borrow-
  exclusivity half because produced values are immutable (`202607201316:34-38`).
- Defunctionalization (Reynolds 1972; Danvy-Nielsen) with the functional correspondence (Ager et al.) as the
  recursive-to-iterative route, promoted by the bench to the only route (`202607202001:183-187`).
- A-normal form (Flanagan et al. 1993) as the effect-ordering normal form; thermometer-encoded grade lattices so
  a join is a bitwise OR (`202607202001:173-176`, `202607240015:127-129`).
- notko's fallibility ladder (absence as an explicit `Maybe`, no null) (`202607201316:72-74`).

**Half-married (adopted-leaning, but scoped, gated, or a bench can unseat it).**
- Copy-and-patch (Xu-Kjolstad; CPython 3.13; Deegen's baseline JIT): once framed a keystone, demoted by the
  bench to an opt-in accelerator and further to second place behind direct isel for short-lived residuals
  (`202607202001:66-81`, `202607240015:68-78`, bench-supremacy). The clearest case of "the bench re-scoped a
  cite."
- Equality saturation (egg, egglog) as the unified compile stage: revised by the bench to zero-or-negative
  marginal value, now a reserved seam behind a trigger, not the frontier (`202607202001:193-197`, bench-supremacy).
- Deegen-style triple emission (interpreter plus stencil plus differential test from one definition): the shape
  is adopted, the stencil half de-prioritised per the native bench (`202607202001:148`, `:331-339`).
- Tail-call threading (Deegen's "beats LuaJIT hand-asm by 31%"): imported, then bench-rejected for the straight-
  line lean IR and re-scoped to the CFG terminator transfers only (`202607240015:58-66`). The textbook case of a
  prior-art cite the measurement overrode.
- The eBPF-verifier-shaped bounded abstract interpretation over tristate numbers (Vishwanathan et al., CGO 2022)
  for the untrusted-load check; scannerless PEG (Ford 2004) or GLL plus attribute grammars (Knuth 1968) for the
  input side; one-shot and linear continuations (Bruggeman-Waddell-Dybvig; Berdine et al.) for the streaming
  spine and resumption; quantitative type theory (Atkey; McBride) for the assurance grade vector
  (`202607202001:152-153`, `:198-203`, `:188-192`, `:204-208`).
- NaN-boxing and the register VM for the runtime value model and residual encoding (bench-confirmed, ~1.9x over a
  stack VM), which are married on the evidence even though listed among the runtime-representation choices
  (`202607240015:203`, `:140-142`).
- CHERI as an optional hardware-assurance grade for the lease axis: additive, not core (`202607202001:378-380`).

A distinction the sibling negative fork sharpened, and I adopt: "half-married" means a bench *could* unseat it or
it is scoped/gated, which is a different state from "a bench *already* unseated it and it now waits behind a
trigger." Two of the entries above are past that line and belong at the negative-precedent end. Equality
saturation was already unseated: its marginal contribution measured zero-or-negative and it is parked behind a
named trigger, so it is really an "uncertain, may return on a machine-and-effect-aware objective," not merely
adopted-leaning. Tail-threading is negative for the straight-line default but positive for the CFG terminator
transfers, so read its rejection as scoped, not blanket. Copy-and-patch, by contrast, is genuinely half-married:
demoted below direct isel but still an in-tree accelerator whose compile-vs-warm breakeven (`k*`) is unmeasured,
so a run-many-times residual could still justify it. For the full already-killed and parked-with-negative-
precedent catalogue, see the sibling `worker-fork_soul-and-its-inverse-what-still-holds-and-what-died.md` (its
Part B and Part C); this positive enumeration deliberately does not duplicate that inverse, and defers to it on
the exact status of every demoted mechanism.

**Heeded, but not concretely aligned (respect the lesson, refuse the mechanism).**
- oatlog and the relational-engine-as-proc-macro: heed the idea (one semi-naive relational engine under lease
  inference and load verification) but not the tool (a Rust proc macro emitting heap-using Rust, illegal under
  no-alloc dual-locus). Design our own no-alloc relational-engine generator emitting into both loci
  (`202607202205:57-69`).
- Carmack's own floor-first verdict: its concrete faults are the agenda; its re-tiering remedy is rejected at the
  frame (`202607202205` throughout). The lesson is heeded; the conclusion is not.
- LMS on the emission axis: heeded as the anti-pattern to route around (a Rust metaprogram emitting Zig source is
  the LMS-hard, hard-to-certify route, rejected at `1627`), even while LMS's staging *insight* (`Rep[T]` vs `T`,
  construction-time CSE) is married on the reducer axis. A deliberate split verdict (`202607202330:180`).
- The full Rust borrow checker: heeded but explicitly not aligned; the aliasing-exclusivity half is dropped
  because produced values are immutable (`202607201316:34-38`).
- Iris-scale unified logical relations, MLIR/IREE/SPIR-V `OpCapability`/Slang multi-target IR: heeded as
  existence proofs that the output-spectrum and inclusion-not-coverage abstractions ship at production scale, and
  as the systems-audience framing, not as mechanisms adopted whole (`202607202001:204-208`, `:215-219`).
- AARA / potential-amortised analysis: deprioritised to "three constants," re-opened only because the bounded-
  multi-shot budget-fit is the first thing that genuinely needs a resource bound; the mechanism is still open
  (`202607210120:114-118`).

## 7. Intents surviving from before the canonical round

Substance can be superseded while the intent behind it survives. These predate or open `202607240130` and are
carried in spirit even where the specifics moved:

- **A small closed Core plus open families, with two orthogonal `AccessSet` axes under inclusion-not-coverage**
  (the ratified `202607192330`/`2358` cut). Superseded forward (an eleventh-to-twelfth form via `Handle`, the
  effect lattice split into effect-plus-binding-time), but the *shape* (a small closed vocabulary the framework
  owns, families the consumers extend, inclusion enforced) is unchanged (`202607210120:30`, `:83-88`).
- **Supersede forward, never rewrite a locked topic.** The old topics stand as the path that led here; the
  evolution is recorded on top. This is the workspace's audit-trail discipline, and it is itself a load-bearing
  intent (`202607210120:83-88`).
- **The Core is grounded in the real consumer census.** The Core was re-derived *from the tech* rather than from
  the 2026-07-19 ten-consumer census, but the census's role (the Core exists to serve the actual census of
  consumers, not an abstract ideal) survives as the grounding constraint (`202607210120:6-7`).
- **The committed ABI and the 1845 panel calls** ("spill as capability," "the depth cap is finite lattice
  height"), re-voiced under the three-codegens reframe and not reopened (`202607202001:31-38`, `:424-425`).

## 8. The one-sentence soul, and how the identity's four commitments are its shadows

Distilled to a single sentence: **vehje is one binding-time-directed handler discipline, proven statically and
compiled away, projected from one signature into two certified artifacts, over a spectrum of outputs with no heap
and no collector.**

The round's four closing commitments are four projections of that one idea (`202607210120:43-49`, read through the
idealistic synthesis):
- "One signature projected" is the idea on the *type* axis: one operation set, many projections.
- "The proof compiled away" is the idea on the *assurance* axis: discharge each handler at its stage, then erase it.
- "Two artifacts, data not source" is the idea on the *artifact* axis: the stages coarsen into two binaries, and
  the coarsening (four binding times, two artifacts) is exactly why one artifact, the runtime, contains a compile
  stage.
- "Native never the driver" is the idea on the *output* axis: runtime discharge is one point on the spectrum, not
  the telos.

The intent, then, is not four separate ambitions held together by taste. It is one idea (an operation is
discharged by a handler at the earliest binding time its inputs are known, emitting IR at a build stage and a
value at runtime) held with maximal-shape conviction (design the whole thing, through the hard parts, before
code), grounded on a census of real consumers (templating first-class, not the game runtime alone), verified by
the type system, bounded by host-lent budgets, and settled, wherever it is a question of which-is-faster, by the
bench rather than by decree.

## 9. Spirit signals mined from the prior panels (per op)

Op asked that the earlier panels and their synthesis be read too, because small details in the prior work carry
the spirit even where the concrete claim is stale or superseded. Six signals surface that sharpen the sections
above; each is cited so the provenance is explicit, and none is taken as a live concrete where a later bench or
topic moved it.

A. **Unify by construction, not by analogy; the standard answer is a smell.** The graded-spine's deepest origin
   is a stance, stated in the novelty audit: "excellence is not found by obeying tradition and not by ignoring
   it, but by composing established pieces until a new shape falls out that supersedes the pieces it was built
   from," and, sharper, "a design that congratulates itself on reaching the field's standard answer has told you
   exactly where to look for the missed idea" (`202607201854_certgen_panel_continuation/04:8`, `:19`). The three
   axes were nearly unified *by analogy* ("each is an instance of one frame"); the audit rejected that with "a
   filing system gives you one vocabulary; it does not give you one theorem" and forced the unification *by
   construction*, one soundness theorem (`04:43-46`). This deepens §3 (the maximal-shape ethos) and §2.3: the
   graded spine is unified by construction because unify-by-analogy was caught and refused. When a design reaches
   the field's default, treat it as the signpost to the missed recombination, not as arrival.

B. **Make illegal states unrepresentable, and push the check into generated structure.** This recurs across every
   prior-art domain: staged generation, exhaustive writers that fail to compile on an unhandled case, typed
   generation certified once (`202607201618_synth_00_index.md`, cross-cutting threads). It is broader than the
   inclusion check of §2.4: it is the pervasive aesthetic that a guarantee should be a shape the type system
   refuses to let you violate, not a runtime check or a convention. The idealistic synthesis turns this lens
   inward onto vehje's own types (thermometer-encode `Knowledge`, brand arenas, a `binding_time_ceiling` field);
   that inward turn is the same signal applied to the framework itself.

C. **Everything borrowed is heap-and-GC-shaped; the no-alloc port is the standing tax.** Every prior art in the
   corpus carries a "what would not transfer to no_std, no-alloc" section, and read together those are the real
   constraint map (`synth_00_index`, thread 2). Concretely: thunks and laziness are heap-shaped by necessity, so
   a no-alloc runtime either evaluates eagerly or accepts an arena as its heap, there is no free lazy evaluation
   (`202607201618_synth_purity-totality-effects-and-the-compile-runtime-split.md:67-73`). This sharpens §2.9: the
   no-heap conviction is not a preference, it is the tax every adopted mechanism pays, and paying it (porting the
   heap-shaped idea to a bounded arena or a host-lent budget) is the recurring real design work, not an
   afterthought.

D. **A handle is an index, never a pointer, whenever memory can move or must be shared** (`synth_00_index`,
   thread 3). The operational form of §2.9 and §2.10: the value-arena's relative-index links, the residual's
   child-index pool, and the never-relocating arena all follow from this one rule, which also delivers the
   zero-copy-equals-wire property (the same bytes are the in-process representation and the pipe form).

E. **Guarantees are type-system properties discharged before runtime, needing no separate analysis pass; and
   content-addressed determinism is the payoff of that discipline.** Totality, effect-safety, and lease validity,
   when made type properties, "become guarantees with no separate analysis pass rather than runtime checks, at a
   stated ergonomic cost" (`synth_purity...:19-20`). Content-addressed determinism (a semantic hash invariant
   under formatting and bound-variable naming, sensitive only to semantics) is *a payoff of totality*, because it
   requires a normal form that always exists (`synth_purity...:42-54`). This deepens §2.2 and §2.8: "prove before
   lowering, the runtime is a dumb evaluator" is one instance of the general conviction that a guarantee lives in
   the type system and is discharged statically, and reproducible content-addressed output (the manifest, the
   cache) is what that conviction buys. It also grounds §2.11 (strict-by-design): the ergonomic tax is accepted
   deliberately in exchange for the language-wide guarantee.

F. **Bench honesty: audit your own benches adversarially; a bench that measures the wrong thing is worse than
   none.** The design's own first evidence pass was independently audited and found "not dependable as written":
   five load-bearing numbers were, respectively, an optimizer partial-evaluation artifact, a no-op loop, an
   arithmetic model, a strawman comparison, and a false methodology claim, yet *every architecture call survived*
   (`202607211539_bench-evidence-audit-panel/summary.md:9-14`, `:94-100`). The correction rebuilt the evidence as
   the carrier matrix, whose two structural wins are the discipline made mechanical: FFI-borne programs make the
   partial-evaluation artifact impossible by construction, and one-axis-off-one-operating-point makes numbers
   compose instead of being asserted additive (`summary.md:80-87`). The ten distilled bench-writing rules are the
   spirit of §5 in operational form (`summary.md:88-92`): workload data must cross an opacity boundary; assert
   every named layout; put a cost-model sanity line in every findings file; the committed CSV is the bench; label
   models versus measurements versus proxies at every citation; a comparative claim needs the strongest
   good-faith opponent; the mechanism under test must exist in the code; single-shot numbers do not conclude;
   ranking stops at the noise floor; every caveat survives summarization. The load-bearing spirit note: the
   architecture is robust to bench correction (the direction survived; five numbers did not), which is exactly why
   §4's "settle which-is-faster forks by bench" is safe. A beautiful detail worth keeping as a caution: the naive
   native-ceiling bench measured the framework's own central mechanism (partial evaluation, the first Futamura
   projection) as a *measurement artifact*, because the compiler specialised the interpreter into native and the
   bench compared native to native; the honest re-measure required forcing the workload across an opacity boundary
   (`summary.md:16-24`). The framework's own soul can hide inside its own benchmark; cross the opacity boundary or
   be fooled by it.

## How to use this document

When a proposal, a doc, or a piece of code is questioned as "drifting from the intent," check it against §2 (does
it keep the convictions), §3 (does it take the maximal shape or quietly re-tier), §4 (does it honour op's
latest call, not a superseded one), §5 (is a which-is-faster fork being asserted instead of benched, and does it
respect what the benches already taught), and §6 (is a demoted prior-art cite being treated as still-married). The
one-sentence soul in §8 is the fastest test: does the thing serve the one binding-time-directed handler discipline,
or does it fragment it. That question, more than any specific line in any topic, is the spirit.

## 10. Audit of the sibling negative enumeration (per op)

Op asked, once the negative fork settled, that I audit it from this fork's perspective, aligned or not. I read
`worker-fork_soul-and-its-inverse-what-still-holds-and-what-died.md` in full, including its own addendum auditing
this document. Verdict up front: the two enumerations agree on the identity, the ethos, op's standing calls, and
the bench discipline; the negative fork's inverse catalogue (its Part B kills, Part C uncertains) is the half my
directive did not cover and is well-sourced; and its four findings against this document are correct, two of them
verified by me against source. I applied all four to the body above. Details.

### Its catches against this document: conceded, and where I verified them

1. **The depth-cap pun (its finding 1) is correct, and I verified it.** My original §2.13 presented "one depth
   cap doing six jobs" as living conviction. `202607202055:95-99` (attack A9) explicitly retired that as a pun
   (two of the six were a width or a length wearing the word "depth") and replaced it with three genuine
   structural bounds. I read `202607202055` directly to confirm, not on the negative fork's say-so. Fixed in
   §2.13, keeping the living aesthetic and correcting the dead instance.
2. **The terminator reconciliation (its cross-fork tension) is correct, and I verified it, with one nuance.**
   `202607202055:60-66` (attack A3) states the canonical terminator for the partial-evaluation unfold is
   "binding-time grade well-foundedness, not a cap," because "a no-alloc cap makes lowering non-deterministic,"
   with determinism from "confluence on the static fragment." So the negative fork is right that Rompf's fuel-cap
   proposal (`tiark_rompf_staging-and-the-line.md:20-26`) did not confront the round's own recorded kill of
   cap-termination. The nuance I add: the prior kill was of partial-evaluation-as-e-graph-*extraction*
   specifically; today's op-corrected mechanism is single-arena recursive *handler discharge* (a catamorphism), a
   different mechanism. But the *principle* the kill established (well-foundedness for determinism, not a cap) is
   general and governs the recursive discharge too, so the reconciliation holds. And the convergence is worth
   naming: the idealistic synthesis already landed on the right shape ("bounded by construction, a cap only as the
   backstop for a mis-graded family macro"), it just did not cite the prior kill; the negative fork's unique
   contribution is supplying that provenance, which elevates the terminator from a panelist's fresh instinct to
   the round's recorded position. The canonical round should state the terminator as binding-time
   well-foundedness, cite `202607202055`, and treat a fuel cap as at most a non-canonical defensive backstop for a
   mis-graded third-party family macro, reported as a diagnostic. This is the single most load-bearing item the
   panel and both soul forks converge on.
3. **eqsat mis-filed as half-married (its finding 2) is correct.** "A bench could unseat it" and "a bench already
   unseated it, and it waits behind a trigger" are different states; eqsat is the latter. Fixed in §6 with the
   distinction, deferring to the negative fork's Part C for the exact status.
4. **The copy-and-patch specific in the identity sentence (its finding 3) is correct.** Fixed in §2.4 by lifting
   the demoted mechanism out and stating native as a method-open output point per language.

None of the four touches this document's §8 one-sentence soul, which the negative fork also confirms stands.

### Where I extend or lightly push on the negative fork

The negative fork is sound; I have one substantive addition, not a correction. Its Part B lists "one-one-shot
coroutine as the whole streaming substrate" as killed, replaced by a semi-naive differential monotone-fixpoint,
and the kill rested explicitly on "equality saturation is multi-shot and re-entrant" (`202607202055:88-94`). But
eqsat is now parked (its own Part B/C). So that *particular justification* for lifting the substrate one level up
is now partly moot: the live multi-shot consumer it invoked has been demoted. The semi-naive relational-fixpoint
substrate still stands, but on the surviving consumers (lease inference `202607240015:121-124`, and load
verification), not on eqsat; and the one-shot/linear continuation machinery still stands on its own for the
value-transfer streaming and the CR1 bounded multi-shot continuations. The negative fork noted this tension
parenthetically; I sharpen it to an owed action for the canonical round: re-justify the relational-fixpoint
substrate on lease-inference-and-load-verify alone, and do not let a justification that leaned on a now-parked
consumer ride into canon unexamined. This is the same failure mode the whole exercise guards against (a demoted
thing propping up a live claim), applied to a kill's rationale rather than to a soul entry.

### Net

The positive enumeration (this document, after the four fixes) and the negative enumeration are consistent and
complementary: this one states what the soul is, the sibling states what the soul is not and what a measurement
or a better composition retired. Read together, they are the intended pair. The one place the canonical
consolidation must be most careful, on the combined evidence, is the unfold terminator: binding-time
well-foundedness, grounded in `202607202055`, not a cap.
