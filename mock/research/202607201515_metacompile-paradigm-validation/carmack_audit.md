# The metacompile paradigm: is it sound and good? (an audit)

**Date:** 2026-07-20
**Method:** read the three current-round topics, the value-transfer study and its synthesis, the metacompile-form
sketch, the committed contract spine (`202607192358/`), and the founding procedural-docs arc; cross-checked the
load-bearing prior-art claims against primary sources on partial evaluation, tagless-final embedding, region
inference, and embeddable-VM compile/run splits; inspected the shipped source (`mock/crates`, ~2000 lines) to
keep the "what exists" axis honest against the "what is designed" axis.
**Scope of judgment:** the paradigm as stated in `202607201513_topic.three-codegens-and-compile-levels.md` and
the two sibling topics, read against the spine it partly supersedes.

## Verdict in one paragraph

The paradigm is sound in its core conceptual moves and worth adopting, but it is not finished thinking, and one
of its unfinished parts is load-bearing. The three-way split of "codegen" into runtime generation, macro
expansion, and output generation is correct and is genuine terminology hygiene that removes real, recurring
design errors. Collapsing interpretation and transpilation into one output-generation spectrum, with native
demoted from a tier to an ordinary target, is not a clever reframing but the standard result of partial-evaluation
theory and polymorphic-embedding practice arriving where it always arrives; the design earns credit for stating it
plainly rather than for inventing it. The novelty that remains is real but is composition novelty and
constraint novelty, not invention: nobody has run this exact set of established mechanisms together under
no_std/no_alloc with three parallel proof axes discharged ahead of a collector-free runtime, and the
arena-never-moves observation that deletes the moving-GC handle-table apparatus is a correct and valuable insight.
Against that, three things are underdone. The paradigm silently relocates the entire per-script compile pipeline
out of the dev-time Rust compiler and into the shipped runtime, which contradicts the committed effect-split
derivation and the sibling topics' repeated claim that the proofs are "discharged in Rust before lowering"; that
reconciliation is owed and is not a detail. The lease axis is treated as co-equal with the family and effect axes
when it is a different and much harder kind of proof, and its cited lineage (MLKit, Cyclone) is a cautionary tale
the design under-weights rather than the endorsement it reads as. And the design keeps parking the single hardest
sub-question of each axis behind an optimistic, untested lean. None of this sinks the paradigm. All of it should
be answered before it hardens into a design document.

## Where established theory and practice back it (name the prior art)

The strongest parts of the paradigm are the parts that are not new, and the design is mostly honest about that.

**Interpretation and transpilation as one operation aimed at different targets** is the tagless-final / polymorphic
embedding result, and it is exactly right. Carette, Kiselyov and Shan's "Finally Tagless, Partially Evaluated"
(JFP 2009) and Hofer, Ostermann, Rendel and Moors' "Polymorphic Embedding of DSLs" (GPCE 2008, Most Influential
Paper 2018) both establish the same property the topic leans on: a term written once against an interface can be
handed to an instance that evaluates it, an instance that emits code, or an instance that pretty-prints it, without
the term changing. The choice of what a target does is a property of the instance, not of the source. The
procedural-docs arc already grounded this correctly in `06_prior_art.md` ("The mechanism for evaluate-versus-emit
is final encoding, not staging") and `07_toolbox.md`. So "output generation is a spectrum, interpret and transpile
are its poles" is a faithful restatement of a settled result.

**The deeper reason the poles are the same operation** is partial evaluation, and this is worth naming because it
underwrites the topic's most contentious claim, that "native is not a concept, it is a target." Futamura's
projections (Futamura 1971; Jones, Gomard and Sestoft, *Partial Evaluation and Automatic Program Generation*, 1993)
establish that a compiler is a specialized interpreter: specializing an interpreter with respect to a source
program yields a compiled program (first projection). There is no ontological category "native code" separate from
"interpreted"; there is only how much of the computation you residualize versus perform. The topic's own
formulation, "reduce-everything is the interpreting end, preserve-control-flow is the transpiling end," is precisely
the binding-time division that offline partial evaluation computes: what is static gets reduced, what is dynamic
gets residualized. So demoting native from a tier at the top of a ladder to one ordinary point on the
interpret-to-transpile axis is not a taste call; it is what the theory says, and the committed spine's "tier-3
native with the runtime collapsing to plumbing" framing was the thing that was slightly wrong, exactly as the topic
claims.

**Runtime generation as specialization of a language-generic engine** has two lineages, and both back it. Read one
way, it is the language-workbench idea: a language definition (grammar, families, effects, regions, targets) fed to
a generator that emits the language's tooling. That is Spoofax (Kats and Visser, OOPSLA 2010), JetBrains MPS,
Rascal, and Melange, all of which generate parsers, checkers, and interpreters from declarative language
definitions. Read the other way, and more precisely, runtime generation is the first Futamura projection lifted
from the program level to the language level: the general engine is an interpreter parametrized over (language,
script), and specializing it with respect to the language yields a single-language runtime. The metacompile-form
sketch's central finding is exactly the partial-evaluation reading made concrete: it traces the pipeline and shows
every stage is "a general engine reading per-language data" (which specializes away to data) except family runtime
semantics (which is residual behavior). That is what specializing an interpreter with respect to its static
language parameter looks like, and the sketch reaching it by hand is a good sign the mental model is coherent.
Truffle/GraalVM is the nearest live system to point at, but note the difference and state it precisely so it is not
oversold: Truffle partially evaluates a language interpreter with respect to a specific guest *program's* AST at
*run time* to get machine code (Würthinger et al.; the "practical first Futamura projection" line). Vehje
specializes with respect to the *language* at *dev time*. Same theorem, different static input, different stage.
The design is on solid ground; it should cite the projection rather than imply the split is novel.

**The two-language compile/run split with a dumb runtime evaluator** is the embeddable-VM mainstream, and one
instance is directly in the author's lineage. QuakeC (1996) is precisely this shape: a dev-time compiler (`qcc`)
produces a flat bytecode image (`progs.dat`) that a tiny switch-based VM inside the engine interprets, with the
engine never containing the compiler. Lua is the same split at a finer grain (`luac` or the built-in compiler
produces bytecode; `lua_State` executes it), as are mruby (`mrbc` to RiteVM), Wren, and Guile's bytecode. The
committed spine's "residual crosses, runtime executes it" is this pattern, and it is well-trodden. What the new
paradigm changes about it is discussed under failures, because that is where the change bites.

**The value-transfer model is the best-grounded part of the whole design, and the grounding is real triangulation.**
The five-runtime study converges because five disjoint systems independently reached the same shape, not because
they shared a frame: the flat, index-referenced, self-describing buffer is Cap'n Proto's arena, FlatBuffers'
`uoffset` tables, rkyv's `RelPtr` archived types, V8's `ValueSerializer` linear stream, and the WebAssembly
Canonical ABI's contiguous record layout. The reserve/commit sink is `System.Buffers.IBufferWriter<T>` and
`wasm stream.write`; backpressure-bounds-residency is `PipeWriter` with `PauseWriterThreshold`. The untrusted-path
bounds-checked traversal is Cap'n Proto's traversal and pointer-depth limits and rkyv's `bytecheck`. All of that is
cited correctly. The single sharpest insight in the study is also correct and is the one I would keep above all the
others: the moving-collector handle-table apparatus that every dynamic runtime carries (V8 `Local`/`HandleScope`,
Lua's stack-by-index, JNI critical regions, .NET pinning) exists *only* because a compacting collector relocates
live objects, and vehje's arena never moves, so vehje inherits none of it. That is a genuine structural
simplification, correctly derived from first principles rather than borrowed.

**Inclusion-not-coverage, checked before emission**, is grounded in four independent lineages by the arc's own
Wyman file: SPIR-V `OpCapability` with `spirv-val --target-env`, `javac --release` against `ct.sym`, Rust
`#[target_feature]` (RFC 2045), and browserslist with `doiuse`. The mechanism is established; its application to a
multi-target IR framework is where the design sits ahead of the surveyed document-conversion systems, and the arc
already narrowed the claim honestly from "nobody checks before emitting" (false against SPIR-V) to "no document
converter checks before emitting" (true).

## Where it is genuinely novel, and whether the novelty is defensible

The novelty is composition and constraint, and it is defensible as long as it is stated as composition and
constraint. The arc's own claims ledger (`panel4/04_hoffman_ledger.md`) already forced this correction on the
parent project: "it is no longer 'we are building something nobody has built', it is 'we are applying established
mechanisms to a domain that lacks them, and composing them in a way nobody has composed them.'" The paradigm topic
inherits that posture and mostly keeps it.

Three things are new in the composition sense, and all three survive scrutiny.

The **three-axis proof unification** is the real design contribution: families, effects, and now leases are three
parallel uses of one `AccessSet`-style typestate, each a `residual_set ⊆ declared_set` inclusion, and the ambition
is to discharge all three ahead of a collector-free runtime. Treating an effect set as a perfect mirror of a family
set, both checked by the same `ContainsAll`, is clean and I have not seen it in exactly this shape. That is worth
having.

The **arena-never-moves deletion of the GC apparatus** is novel as an applied insight even though each half is old.
Nobody ships a dynamic-value runtime that gets to skip the entire handle/pin/root machinery, because nobody else
built the value store to never relocate. Vehje can, and the study correctly saw that this is the single biggest
thing it gets for free.

The **whole-stack no_alloc constraint on a generator-and-runtime** is novel in the "nobody has done it" sense, and
this is where the design should be most careful, because "novel because unprecedented" and "novel because
infeasible" look identical from the inside. The parent arc's ledger classified "a heap-free generator has no
precedent" as U1, unsupported in both directions: no precedent found and no search performed to establish there is
none. That entry is still open, and the paradigm makes the constraint harder, not easier (see failures). Defensible
novelty requires that the hardest instance of the constraint actually be built and shown to hold, not asserted.

The disambiguation itself (three codegens, native-as-target) is not novel and should not be claimed as such; it is
clarification. Its value is real but it is the value of naming a confusion correctly, which the topic states as its
own purpose. That is the right framing.

## Where it fails, or hits known pitfalls (the honest failure analysis)

**1. The paradigm relocates the per-script compiler into the shipped runtime and does not reconcile that with the
committed spine. This is the finding that most needs attention.**

The committed contract spine is explicit and repeated: the Rust compiler "reduces a program to a `Checked`
residual," "reduce every `BuildEnv`-parametrized effect away," and "what crosses is the residual"; the Zig runtime
"only ever executes the residual" and "never implements build-environment semantics." The lease topic restates it:
"All three are discharged in Rust before lowering, so the runtime inherits proven-safe programs." The value-transfer
context restates it: "The compile side pre-resolves `Var` to indices; the residual carries indices."

The three-codegen topic says something different, and it is a genuinely different architecture, not a rewording.
The Rust language compiler "is never shipped" and "never sees an end-user script." End-user scripts "arrive at run
time" at the composed runtime, which "lowers each script to a shared IR, runs the analyzers (name resolution, and
the family/effect/lease checks as per-script evidence), does macro expansion (IR to IR), and does output
generation." So under the new paradigm the per-script pipeline (parse, resolve, the three evidence checks, macro
expansion, const-eval, output) runs inside the shipped composed runtime, at script time, in the generated Zig. The
Rust side only ever compiled the *language*.

That is internally coherent and is arguably the better architecture; it is how Lua, Python, and JS actually ship,
a compiler-plus-runtime for the language rather than a dev-time-only compiler. But it contradicts the spine on
three concrete points that the topic does not work out:

- **Where the per-script proofs run.** The spine says Rust, before lowering. The paradigm says the composed
  runtime, at script load. The lease topic's "discharged in Rust before lowering" is true only of the language-level
  proof (the analysis is sound); the script-level lease is proven by the shipped runtime's analyzer pass. These are
  two different proofs at two different times, and the topics are written as if there is one. A reader cannot tell,
  from the current text, whether a script's family/effect/lease evidence is a Rust-side or a runtime-side check, and
  the answer changes what the C ABI carries and what the runtime must contain.

- **Whether the runtime implements build-environment semantics.** The spine's clean payoff was "the Zig runtime
  never implements build-env semantics, because the effect proof guarantees no such construct reaches it." But the
  paradigm has the composed runtime do macro expansion and const-eval per script, which *are* build-environment
  reductions by the spine's own definition (a macro expands against build state; a const folds at build time). So
  the composed runtime now spans both effect environments. The narrow, one-const-`Permits` runtime contract that
  the effect model was designed to produce may no longer hold in the same form. The topic flags "macro expansion
  runs inside the composed runtime during the script's compile stage" without noticing it has just re-crossed the
  line the effect split was built to keep clean.

- **What the effect-derived compile/runtime split even means now.** The spine derived the whole
  compile-time/runtime split from the effect axis: BuildEnv is the compiler's, RuntimeEnv is the runtime's. Under
  the paradigm, the composed runtime holds both a compile stage and an output stage, so the split is now *inside*
  the shipped artifact, between two stages of the runtime, not between two artifacts. That may be fine, but it is a
  different derivation and it is not written down.

The honest reading: the paradigm topic is a real clarification that also, as a side effect, supersedes more of the
committed spine than it admits. It claims to retire only "native as a tier" and "the program-centric residual
crossing." It actually also moves the script compiler from dev-time Rust into the shipped runtime, which changes
the meaning of the residual, the ABI, the effect split, and the "proofs discharged in Rust" language in both
sibling topics. That reconciliation is a required next step, not a detail.

**2. The lease axis is the weakest load-bearing claim, and its own cited lineage is the evidence against it.**

The design cites the right sources (Tofte-Talpin region inference, MLKit, Cyclone) and makes one genuinely smart
move: because a produced value is immutable, only region validity is needed and the aliasing-exclusive-of-mutability
half of Rust's borrow checker can be dropped, which is what makes the analysis fusable into the resolve walk. That
narrowing is correct and is the best idea in the topic.

But the lineage it cites is a cautionary tale, and the design reads it as an endorsement. MLKit's pure region
inference *leaked* for real programs: a value escaping into an outer region keeps that region alive longer than the
programmer expects, and the system had to (a) emit escaping-put-effect warnings, (b) add region polymorphism so
recursive calls did not all share one region, and (c) in the end combine region inference *with a copying garbage
collector*, because inference alone was not sufficient in practice (Tofte et al., "A Retrospective on Region-Based
Memory Management," HOSC 2004; the MLKit manual documents the leak warnings). Cyclone, named as "the closest prior
art," did not ship pure static regions either: it shipped a hybrid of lexical regions, unique pointers,
reference-counted objects, and a garbage-collected heap (Grossman et al., PLDI 2002; Hicks et al.), precisely
because static regions alone could not carry every real program. The consistent lesson from both is that static
region/lease discipline is sound but incomplete, and every serious attempt needed an escape valve: GC, refcounting,
or dynamic regions. Vehje's design rules out all three ("no collector and no reference counting," failure is a
compile error). The strict-by-design posture is admirable and consistent with the workspace, but the prior art
predicts that a nontrivial fraction of real programs will hit inference failures, and the make-or-break question is
then the ergonomics of forced explicit annotation, which is exactly what the topic defers to "open threads." The
design has not reckoned with the possibility that the failure rate is high enough that ruling out every escape valve
is a usability cliff.

There is also a concrete soundness gap in the "uniform across languages" claim. The narrowing depends on "a
produced vehje value is immutable once computed," so aliasing can be ignored. That holds for a functional consumer.
It does not hold for `vehje-lua`, which has mutable, aliased tables, and it is unclear for the first-party language
with "events and manifests." The lease topic and the metacompile-form sketch both assert the analysis is uniform
and "only the rules change," and the sketch explicitly waves off mutable/linear semantics with "none of the census
languages want that." Lua is in the census and Lua wants exactly that. For a mutable consumer the dropped
aliasing-exclusivity half is not optional, and the analysis is no longer the cheap region-only pass; it is the
expensive borrow-style analysis the design claimed it could avoid. This is a real hole, not a detail: either the
lease axis is functional-consumer-only (and Lua-family consumers fall back to something the design has not
specified), or the "drop aliasing" simplification does not actually hold framework-wide.

**3. The three proof axes are lumped as "proven the same way" when one of them is a different kind of proof.**

Family inclusion and effect inclusion are decidable set checks: given the declared `Supports`/`Permits` sets and
the supertrait dependency DAG, checking a target is total over its families and the program's set is included is a
`ContainsAll` and an acyclicity check, cheap and total. "The Rust compiler proves the language's family/effect
discipline sound" is a real, runnable meta-check. "The Rust compiler proves the language's *lease* discipline
sound" is a different animal: soundness of a region-inference algorithm for all programs of a language is a
metatheorem (MLKit's authors proved it once, on paper, for their calculus), not something you run per language
definition. If the design means the lease *rules* a language may declare are constrained to a fixed, pre-proven
schema, that is defensible, but then the expressiveness of those rules is bounded by what was pre-proven, and that
bound is undescribed. If the design means an automated soundness check per arbitrary language definition, that is
aspirational. The topic's "proves the language's disciplines sound, hole-free, gating runtime generation" reads as
the latter for all three axes uniformly, and only two of the three can plausibly be that.

**4. No_alloc script compilation in the shipped runtime is the hardest instance of an already-unprecedented
constraint.** Follows from finding 1. If the composed runtime compiles scripts, then parse, resolve, region
inference, and macro expansion all run with no heap, over caller-provided bump arenas whose size must be bounded
before the script is seen. Parsing and resolving into a bump region and failing on overflow is fine. Region
inference is a unification/fixpoint whose working set is data-dependent; doable with a bounded scratch region plus a
"proof did not fit" failure, but not trivial. Macro expansion is the genuinely hard one: it produces new IR from old
IR, potentially much more than its input, at runtime, with no heap, which is the classic staged-metaprogramming
versus fixed-memory tension with no established solution at this combination. The parent arc's U1 (no-heap generator,
no precedent, no search) is still open, and the paradigm moves the generator into the shipped runtime, which makes
U1 more central, not less. This is buildable but it is exactly the kind of load-bearing unknown the design keeps
leaning optimistic on.

## Risks and open questions the design has not yet answered

- **Reconcile the artifact layering with the effect split.** Write the pass that re-derives where each per-script
  proof runs (Rust language-proof versus runtime script-evidence), what the C ABI carries now that scripts arrive
  at the composed runtime rather than residuals arriving from Rust, and whether the runtime spanning both effect
  environments breaks the one-const-`Permits` contract. Until this is written the spine and the paradigm topic
  contradict each other in the record, and `canonical-design-outranks-intermediate-rounds.md` will eventually make
  someone guess which to trust.

- **State the lease axis's failure mode and escape valve, or its scope.** Either specify what a consumer language
  does when inference fails and an annotation cannot be supplied (the MLKit/Cyclone evidence says this path will be
  walked often), or scope the axis to immutable-value consumers and specify what mutable consumers (Lua) use
  instead. "Failure is a compile error, no fallback" is a position, but it needs the annotation ergonomics designed
  before it can be judged, and the prior art says the ergonomics are the whole game.

- **Bound the L2 primitive vocabulary empirically.** The metacompile-form sketch's recommendation rests on "if the
  primitive vocabulary stays bounded, the runtime is effectively all-data; if it sprawls, the generated-behavior
  hatch earns its place." That is the fork the sketch says is the only real one, and it is untested. The sketch's own
  next step (enumerate the candidate primitives from the real census families and see whether the set stays bounded
  or sprawls) is the experiment that decides whether the all-data claim holds. Run it before the all-data framing
  hardens.

- **Do not let "logical trace: WORKS" stand in for "executed."** The metacompile-form sketch is a good analysis and
  it is explicitly a logical trace, not a run. The parent arc's ledger is a 700-line monument to what happens when a
  round accumulates confident intermediate conclusions that nobody executed against (five uncaught count errors, a
  citation wrong through two phases, a determinism promise falsified by shipped code). The culture here is unusually
  honest about that, which is the design's biggest asset. Spend it: the tiny general-engine skeleton for stages 1-3
  over a two-node family that the sketch names as its next step is worth more than another topic.

- **Determinism is a framework property, not just a consumer bug.** The parent arc found the mockspace generator
  non-deterministic (shelled to `date`, wrote a wall-clock stamp into every output). That is a consumer bug, but the
  value-transfer and diff-stability stories assume deterministic output, and the framework has not yet made or
  enforced a determinism claim. Name it as a framework invariant with the standard fix (`SOURCE_DATE_EPOCH`-style
  input, no ambient clock in the reducers) before a consumer bakes in the same failure.

- **Streaming emission order versus value-arena construction.** The depth-first, leaves-before-parents emission that
  makes back-patching unnecessary assumes the whole value is computable in that order without holding unbounded
  intermediate state. For a producer whose child indices depend on parent context, that could force buffering that
  the bounded-residency story assumes away. Minor, but worth a sentence.

## Bottom line for the maintainer

Adopt the paradigm. The three-way disambiguation is correct and removes real errors; keep it. The
interpret-to-transpile spectrum with native as an ordinary target is what partial-evaluation theory and
polymorphic-embedding practice say, and stating it plainly is the right move; cite the first Futamura projection and
tagless-final rather than implying the split is new, and the claim gets stronger, not weaker. The value-transfer
design is the best-grounded piece and I would change little in it; the arena-never-moves insight is the one to
protect. The three-axis proof unification is the real design contribution and it is elegant.

Before this hardens into a design document, three debts must be paid. First and most important, reconcile the fact
that the paradigm has moved the per-script compiler out of dev-time Rust and into the shipped runtime; the committed
spine and the two sibling topics are written against the opposite model and now disagree with the paradigm on where
the proofs run, what the ABI carries, and whether the runtime touches build-environment semantics. Second, the
lease axis is not the co-equal third sibling of families and effects that the topics present; it is a harder proof
whose own cited lineage (MLKit, Cyclone) needed the exact GC/refcounting/dynamic-region escape valves this design
forbids, and it has a mutability blind spot for Lua-family consumers. Give it its own reckoning or scope it down.
Third, stop parking the single hardest sub-question of each axis behind an optimistic untested lean: the bounded
primitive vocabulary, the lease inference failure rate, and the no-heap script compiler are the three places the
design is betting, and the workspace's own rule is to run the smallest experiment that answers the real question
rather than argue it. Run those three. The paradigm is good. It is not yet done, and the parts that are not done
are the parts that decide whether it ships.
