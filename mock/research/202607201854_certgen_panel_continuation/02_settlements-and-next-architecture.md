# Settling the seven calls, and the architecture that falls out

**Date:** 2026-07-20
**Phase:** panel continuation (post-audit)
**Role taken:** compiler-theory veteran with a mathematics leaning, settling the seven open questions and seeding
every follow-on talk the settlements unblock.
**Reads on:** the certified-generation panel (`../202607201641_certified-generation-panel/01`..`08b`, chiefly the
veteran take `06` and the theory model `08`), the five open-round topics (`../../design_rounds/202607201315`,
`1316`, `1513`, `1534`, `1627`) and the consolidation `1845`, the eight prior-art synthesis docs
(`../202607201618_synth_*`), and the paradigm validation (`../202607201515_metacompile-paradigm-validation/`).

This document does two things. Part I settles the seven calls with a decision and a justification per call, folding
in the three refinements `08` landed on them, and adds the sharpenings the mathematics makes available. Part II is
the forward half: with the seven settled, a set of follow-on talks becomes designable now, before the widened
experiment runs. Each is seeded with what to design, a proposed direction, a justification, and a devil's-advocate
pass. Part III banks the related theory the panel has not yet named and cross-references the theory it has.

## Part I: the seven calls, settled

The panel already established the important thing: the certified-generation resolution survived four lenses and
three forks with its architecture intact, and none of the seven is a gamble. They are the field's standard answers
to questions the panel posed correctly. So each settlement below is a bless, and the work is in stating precisely
what is blessed and where a reader who knows the field would otherwise catch imprecision.

### Call 1: bless the depth-ladder lease schema, with the trusted surface named and shrunk

Settled: bless it, and adopt `08`'s sharpening that shrinks the axiom set rather than merely naming it.

The schema is not a new calculus. It is the degenerate, lexically-nested, strictly-LIFO fragment of the
Tofte-Talpin region calculus (region-based memory management, Information and Computation 132(2), 1997) with no
region variables, no region polymorphism, no `letregion` under polymorphic recursion. Tofte-Talpin's soundness
proof was hard because it carried all of that. Throwing all of it away is exactly what makes the metatheorem a
weekend of structural induction over the eleven ratified forms, in the ordinary Wright-Felleisen progress-and-
preservation style that Calcagno, Helsen and Thiemann already applied to the full region calculus (syntactic type
soundness for the region calculus, Information and Computation 173(2), 2002). The invariant is one line:
reachability respects the lease order, every pointee's lease depth no greater than any pointer that reaches it.

The content of the decision is the trusted surface. The metatheorem is conditional, and the condition is the
per-family link/consume bit on every family op, including the effectful and host-boundary rim. A `consume` where a
`link` was needed on a host-callback or interner operand is a soundness hole invisible in the IR, because the
retention happens in host memory the analysis cannot see. That is the correct identification of the axiom set, and
it is the single most load-bearing sentence in the lease story.

The sharpening, and the reason this is a shrink rather than just a warning: retention through a host boundary is
itself an effect. In vehje's own parametrised effect vocabulary it is a write into host-retained state
(`Writes<Host>` in the notation Part II fixes). Type-and-effect systems and region systems are one system, not two
(Talpin and Jouvelot, the type and effect discipline, LICS 1992). So the link/consume obligation is not an
independent bit to be gotten right by hand alongside the effect classification; it is a monotone function of the
operand's declared effect. An operand an op writes into host-retained state is, by the effect classification the
design already computes, an operand whose lease must be `link`. This collapses the two axioms the veteran flagged
(the effect table and the lease bit table) into one already-tested surface, the effect table, and it is the single
place the mathematics reaches an answer the panel had not. It is a proposal until tested against the census
consumers' host-boundary ops (Part II, talk 2), but it is the right default: derive the bit, do not re-declare it.

Two riders stand. State the crux immutability lemma explicitly (a value created inside body A is reachable outside
A only through A's result value); it is where immutability carries the load and precisely what a mutable store
breaks. Promotion-to-outermost as the inference fallback is sound by the Cousot-Cousot over-approximation argument
(abstract interpretation, POPL 1977): the analysis computes a lower bound on required lifetime, and moving up the
lattice is always safe, imprecise at worst. That is what makes the Lua claim true (precision loss, never
unsoundness), conditional on the rim declaring `link`, which the effect-derivation now makes automatic wherever the
retention is an honestly-declared effect.

A second theoretical home worth recording, because it will matter when a consumer wants mutable aliased values
(`vehje-lua`'s tables): the one-bit-per-operand link/consume distinction is a substructural typing distinction.
`consume` is affine (at-most-once) use of the operand's lease; `link` is unrestricted or borrowed use. The lease
schema is thus simultaneously a region discipline (Tofte-Talpin) and a substructural one (linear logic, Girard,
Theoretical Computer Science 1987; linear types can change the world, Wadler 1990). Rust's own ownership is the
same pairing. Naming both homes now tells us where to reach when immutability is relaxed: not to a garbage
collector, but to affine ownership plus regions, which is the Cyclone hybrid (Grossman et al., PLDI 2002) read as a
substructural extension rather than a retreat.

### Call 2: bless the three-loci split; it is multi-level binding-time analysis done right

Settled: bless it, and correct the framing from two binding times to three, because that correction is itself the
justification.

The panel calls this "one inference, two binding times." It is three, and the literature has the exact tool. The
inputs to the engine become known at three distinct times: the language definition (dev, the language author's
build), the bundled content (our build of the composed runtime), and the arriving script (the consumer's load).
Discharging each property at the earliest time its input is known is multi-level binding-time analysis, and
specialising a generating extension across more than two levels is Glück and Jørgensen's multi-cogen construction
(efficient multi-level generating extensions for program specialization, PLILP '95, LNCS 982). The three-loci split
Expert 4 forced (comptime for the small language table, a native `build.zig` step for content, native at the
consumer) is not an optimisation the panel invented. It is what a three-level BTA tells you to do, and the design
was quietly running a two-level analysis over a three-level problem.

The mechanism-level justification the panel already has is right and should be kept: asking Zig comptime to fold
content is asking the compiler's own tree-walking evaluator to run the kernel over program data, the double-
interpretation tax on top of comptime's measured superlinear base rate. The synthesis material carries the exact
warning (LMS's runtime `compile` "has no analogue for a statically-compiled, no-alloc target"; the disciplined
shape is Terra's `saveobj`, generator dev-time only, only the artifact ships) and the .NET `[DllImport]` to
`[LibraryImport]` lesson (resolve the boundary ahead of time into inspectable static code). The native `build.zig`
content step running the identical kernel is the `saveobj` shape; keeping comptime only for `@Type` reification,
which genuinely cannot happen anywhere else, is the correct residual.

The one thing to say out loud: the `build.zig` content-validation binary joins the trusted computing base. It is
cheap to audit (native, inspectable, testable) rather than expensive (a comptime interpreter run you cannot step),
which is a strict improvement, but it is a TCB item, not a free lunch, and the crate taxonomy owes it a home (talk
9). Bless the split; account for the third locus in the trust inventory.

### Call 3: bless spill as a host-lent sink policy; it is a correctness property, not a preference

Settled: bless it, with the two conditions the theory and systems reviews already raised made part of the decision.

An embeddable library owning its own I/O policy is a category error the field settled long ago under capability-
passing and dependency injection: the runtime is parametric over its sink, the host supplies the capability. The
value-transfer study is the convergent evidence (.NET `IBufferWriter`/`System.IO.Pipelines` with
`PauseWriterThreshold` backpressure; the WebAssembly component model `stream<T>` over a caller-supplied buffer, the
consumer lends and the producer pulls). Baking `open()` in breaks the two cases that matter: a sandboxed host with
no filesystem, and the per-frame consumer (ikiuni) where an SSD write's latency both blows a frame budget and
injects nondeterminism, and where `std::fs` is already banned in `vehje-runtime-abi`. Making spill one
instantiation of the reserve/commit sink the transfer topic already defined is the correct abstraction and nearly
free.

Two conditions of the blessing. First, bytes read back from a host-lent sink are untrusted input again, so the
read-back path re-runs the bounds-and-depth validation; the trusted-local exemption applies only to a
process-private temp with an integrity check. Second, determinism is earned, not free: spill artifacts are named and
reassembled by explicit (stage, chunk-index) or content hash, never by timestamp, PID, or directory enumeration,
because `readdir` order is filesystem-dependent. That is Dhall's content-addressed semantic-hash discipline applied
to spill, with DTVM (arXiv:2504.16552) as the existence proof that a JIT-shaped runtime is made deterministic at
the cost of a deliberately deterministic middle form. Bless the sink policy; make re-validate-on-read-back and
content-addressed naming part of the same decision (design detail in talk 5).

### Call 4: accept the hard depth cap as a first-class runtime limit; it is load-bearing three times over

Settled: bless it as first-class, and state the no_alloc dependency in the design so it can never be silently
relaxed.

The cap does three jobs and the design has been crediting it for one. Job one: the lease-frontier accumulator is
bounded by open-body nesting depth, nesting depth is attacker-controllable, so a fixed cap turns the frontier into
a fixed-size buffer, which is what lets no_alloc survive streaming. Job two: the kernel must be iterative, because
Zig's self-hosted compiler runs comptime calls on its own stack and deep comptime recursion overflows the compiler
process rather than yielding a clean quota error (Zig issue #13724); an iterative kernel needs an explicit
work-stack, and the cap bounds that stack. This is Jsonnet's explicit-frame-stack-with-goto pattern, used precisely
to avoid host-stack overflow on deep input. Job three: it is the standard untrusted-input safety limit every
zero-copy format ships (Cap'n Proto's 64-deep pointer and 64 MiB traversal limits, rkyv's `bytecheck`), documented
as mandatory in the value-transfer study.

The mathematics gives the fourth reading that ties the three together: the cap is the height of the lease lattice.
The lease inference is a monotone dataflow analysis (Kildall, POPL 1973) over the complete lattice of lexical
depths, a finite chain of height equal to the cap, so it converges in one pass and its resident state is bounded by
that height. The no_alloc frontier bound, the work-stack bound, and the untrusted-traversal bound are the same
finite-chain-height fact viewed from three sides. That is why relaxing the cap to unbounded is not a tuning knob; it
removes the finite-height property the single-pass, no-alloc analysis rests on.

This resolves cleanly against the workspace's caps-are-defaults-not-policy rule rather than contradicting it. The
cap is consumer-tunable, but between a floor and a hard framework ceiling, never to unbounded. A batch consumer that
legitimately nests deep raises the cap and pays a larger fixed buffer; the finite-height guarantee survives because
the ceiling stays finite. State the dependency so a later maintainer reading it as "just a safety limit" cannot make
it configurable-to-unbounded and silently break no_alloc.

### Call 5: elevate the cross-chunk lemma to build-blocking, and make it true by construction

Settled: bless it, and adopt the stronger form: do not merely prove the lemma, constrain the chunker so it holds as
an invariant of the emission format.

The cross-chunk lemma (a cross-chunk reference is by construction consumed or promoted, because chunk boundaries are
whole-subtree boundaries) is the invariant the streaming spine's central efficiency claim rests on. If a cross-chunk
backward reference can be an ordinary live inner-body local rather than a consumed-or-promoted value, completing its
lease requires re-reading its chunk, which means retaining all spilled chunks or random-access re-reading, and
either collapses the bounded-window benefit that is the entire reason to stream. The theory review's observation is
the key structural fact: this is the same lemma as the crux immutability lemma viewed at chunk granularity (a
subtree escapes only through its result, and a whole-subtree chunk boundary makes the chunk's escaping references
exactly its root). The lemma is true if and only if the chunker is constrained so boundaries are strictly
whole-subtree and emission is at result boundaries.

So the right move is stronger than "prove it": make it true by construction, constrain the chunker so the boundary
condition is an invariant of the format, and the lemma becomes a property maintained rather than a fact hoped. It
discharges before the streaming spine commits, because everything downstream assumes it. The forced-spill experiment
arm is the empirical half; the constrained-chunker plus a one-paragraph proof (it lives in the same store-typing
induction as the metatheorem, both ranging over the post-order interval structure) is the analytic half. Both,
before the spine locks. The chunker contract this obliges is talk 3, and it carries a real open tension (a single
subtree larger than the window) that the settlement names and the talk resolves.

### Call 6: bless the scoped meaning of "certified"; this is the one place imprecision is not allowed

Settled: bless it, treat it as non-negotiable, and make it mechanical by labelling each axis with its fragment of
the staged-gradual system.

"Certified" and "verified" are load-bearing words, and misusing them is how a sound design earns an unsound
reputation from the first reader who knows the difference. Type-checking certifies well-formedness, never functional
correctness; a total, exhaustively-switched, name-keyed dispatch can still route to a handler whose body computes
the wrong thing. The reference for what semantic certification costs is CompCert (Leroy, CACM 2009): a Coq proof of
semantic preservation through every pass, and even CompCert trusts its unverified pretty-printer and assembler rim;
CakeML (Kumar et al., POPL 2014) closed that rim only by verifying to machine code. Vehje buys neither, correctly,
so the guarantee inventory reads per axis and never collapses to one word:

- family inclusion, effect inclusion: certified structural properties, and for these two decidable set axes
  re-proven after decode (recompute masks, re-derive totality), so the encoder is not even trusted for them. That
  re-proving is proof-carrying-code witness re-checking (Necula, POPL 1997), not mere translation validation.
- non-null, dispatch totality: certified structural properties by construction (Zig's optional type; the exhaustive
  switch over a finite coproduct, whose universal property forces exhaustiveness, the same fact as totality of a
  fold over an initial algebra, Meijer-Fokkinga-Paterson, FPCA 1991).
- lease safety: conditionally certified, on the per-family link-bit axioms (now derived from effects per call 1) and
  a once-proven algorithm over a fixed schema, bridge trust reduced to bit-table integrity.
- family semantics: tested, never proven.
- termination: not claimed, because there is no totality axis, and Dhall shows what buying one costs (fold-encoded
  recursion, a real expressiveness tax) which vehje correctly declined.

The frame that makes this mechanical rather than a hand-curated list is `08`'s: every axis is one instance of staged
gradual verification, labelled by which fragment of the gradual system it occupies (non-null and totality fully
static; family and effect gradual with a decidable re-provable dynamic residual; lease gradual with a
conditionally-sound residual; bounds pure dynamic; semantics outside the system; termination not in the system).
The assurance-vocabulary correction at the citation level is the same discipline: the mechanism is partial
evaluation (architecture: Jones-Gomard-Sestoft) plus per-instance re-type-checking plus translation validation
(assurance: Pnueli-Siegel-Singerman, TACAS 1998; Necula, PLDI 2000), and it is not the LMS / typed-Template-Haskell
/ Terra "well-typed generator proves the output once" story, because Zig comptime is an unverified specialiser with
no staged type system, so it re-checks each instance rather than proving the generator once.

The precise word for what vehje does on the decidable axes deserves banking: it is certifying compilation in the
Necula-Lee sense (each compilation emits a checkable certificate re-verified per instance), distinct from certified
compilation (proven once and for all, CompCert). Typed Assembly Language (Morrisett-Walker-Crary-Glew, TOPLAS 1999)
is the closest structural precedent for the 1627 core move itself: types preserved through compilation into a
low-level form whose structure is checkable without the source-level prover present. The generated Zig's structure
carrying the family/effect proof is TAL's idea lifted from assembly to a specialised runtime.

### Call 7: bless experiment-first, and draft the semantics skeleton in parallel

Settled: bless it, with `08`'s refinement that the form-fixed skeleton drafts in parallel so the sequencing does not
idle.

The ordering has a justification beyond "prototype first": the experiment shakes the schema. The negative arms, the
scaling probe, and the forced-spill round-trip will move the design (which link bits are wrong, whether the chunker
constraint holds, where the comptime cost cliff sits). Writing the metatheorem first risks proving the wrong
theorem, and a proof redone after the schema shifts is waste. This is proof-engineering economy and the
run-the-experiment-not-the-argument rule, doubly right because the load-bearing claims (the comptime cost curve, the
cross-chunk lemma, the panic-free C-ABI behaviour, the ReleaseFast overflow agreement) are all things a two-node toy
passes trivially and only a widened experiment falsifies.

The refinement so sequencing does not idle: the skeleton of the proof document does not depend on what the
experiment shakes. The small-step semantics of the eleven ratified forms, the store typing indexed by the live-depth
stack, and the reachability-respects-lease invariant are fixed because the forms are ratified. Draft that in parallel;
only the per-family axiom discharge and the exact link/consume table are experiment-dependent, and they slot into a
skeleton already standing. So: experiment and semantics-skeleton in parallel (talks 8 and Part II's experiment
seed), then the metatheorem plus the three lemmas plus the backward-link-acyclicity totality note against the
surviving schema, then the doc CL finally allowed to use the word "proven."

## Part II: what now becomes designable, seeded

Settling the seven unblocks a set of talks that could not start while the calls were open. They are ordered by
leverage: the ones the most downstream work waits on come first. Each is a seed, not a settlement. For each: what to
design, a proposed direction, the justification, and a devil's-advocate pass.

### Talk 1: the three binding times, named as a lattice, and the effect axis split to match

What to design. Debt 1 established two build environments (the language build at dev time in Rust, and the script
build at load time in the runtime's compile stage). Add the runtime environment and there are three binding times.
The effect axis's `Reads<BuildEnv>`/`Writes<BuildEnv>` marker conflates the first two because the spine only had one
compile time; it must name which build environment, or split. Design the marker vocabulary as a three-point ordered
set and re-derive the compile/runtime split as internal to the runtime, between its compile stage and its execute
stage.

Proposed direction. Make the binding-time marker a three-point lattice `Lang < Script < Runtime` (language-build
earliest, script-build next, runtime latest), and parametrise the environment effects by level: `Reads<Lang>`,
`Reads<Script>`, `Reads<Runtime>` and the write duals. An effect is discharged (reduced away) at its own level:
`Lang`-level effects vanish during runtime generation, `Script`-level effects vanish during the runtime's compile
stage, `Runtime`-level effects are what actually execute. The certified generation proof is then exactly the
statement that no effect survives below its level.

Justification. This is multi-level binding-time analysis, and the three-level structure is Glück-Jørgensen's
multi-cogen (PLILP '95), not an ad hoc split. It resolves debt 1's four consequences mechanically: "which build
environment does `Reads<BuildEnv>` mean" is answered by the level index; "the compile/runtime split moved from
between two artifacts to between two stages" is the `Script`-versus-`Runtime` boundary inside the composed runtime;
and the certified generator's guarantee (`08`'s central theorem, the mix equation composed with the gradual
guarantee) is stated over three levels rather than two. The lattice ordering is what lets "discharge at the earliest
level where the input is known" be a well-defined rule rather than a case analysis.

Devil's advocate. Is three the right number, or will macros force a fourth? Macro expansion is IR-to-IR at the
script build (level `Script`), so a macro that itself stages could in principle want sub-levels. Glück's relative
binding times (multi-stage specialization with relative binding times) handle arbitrary depth, so the framework is
not boxed in; but committing to exactly three now, with relative binding times named as the escape if a consumer
ever needs staged macros, is the right scope. The counter-risk is over-generality: a full multi-level effect lattice
is more machinery than three concrete levels need. Answer: ship the three concrete levels as an enum, keep the
lattice structure in the proof (where it is free) and out of the runtime types (where it would be cost), which is
the same closed-enum-plus-named-escape discipline the core forms already use.

### Talk 2: the effect-derived lease bit, and the family override

What to design. Call 1 accepted deriving the link/consume bit from the operand's declared effect. Design the retention
-bearing effect, the monotone map from effect set to lease bit, and the escape a family uses when its retention is
genuinely not expressible as a declared effect.

Proposed direction. Introduce a retention effect `Retains<Host>` (equivalently `Writes<Host>` where the host store is
the retaining location) in the parametrised effect vocabulary from talk 1. Define the map: an operand whose op carries
`Retains<Host>` (or any effect in the retention-closed upward set) gets `link`; otherwise `consume`. The map is
monotone on the effect lattice (more effects can only move an operand from `consume` to `link`, never back), which is
what makes promotion sound. A family whose retention is invisible to its declared effects must declare it, and the
only sanctioned way to declare it is to add the effect, not to hand-write a contradicting lease bit; a hand override
is permitted only with a recorded justification that becomes a harness test (talk 7).

Justification. Region systems and effect systems are one system (Talpin-Jouvelot, LICS 1992), so the lease bit as an
effect consequence is not a trick, it is the canonical unification. It collapses the two axioms the veteran named
(the effect table and the lease table) into one already-tested surface. Gradual effect systems (Bañados Schwerter,
Garcia, Tanter, ICFP 2014) give the effect side its own gradual-guarantee derivation, so the lease axis inherits the
effect axis's soundness argument rather than needing a parallel one.

Devil's advocate. Does every retention manifest as a declared effect? No, and that is the danger. The `Raw` escape
family and opaque host callbacks are exactly where a retention can hide with no honest effect declaration. If a
`Raw` op stashes an operand in host memory and declares no effect, the derived bit is `consume` and the schema is
unsound for that op. So the derivation does not remove the axiom, it relocates it: the axiom is now "every retaining
op declares the retention effect," which is the same trusted surface in a single table instead of two. This is
strictly better (one surface, and it is the effect table the design already tests), but it must be stated as such,
not sold as making the axiom disappear. The harness's coverage obligation (talk 7) therefore targets precisely the
effectful rim and the `Raw` escape, and the override path must itself be effect-typed so a hand override cannot
silently under-declare.

### Talk 3: the chunker contract, and the oversized-subtree tension

What to design. Call 5 requires the chunker constrained so cross-chunk references are consumed-or-promoted by
construction. Design the chunk-boundary invariant, the cross-chunk reference discipline, and how the wire format
enforces both. Name and resolve the tension that a single subtree can exceed the streaming window.

Proposed direction. A chunk is a rooted subtree (a whole-subtree boundary), and the only cross-chunk link the format
can express is a reference to an already-emitted chunk root, encoded as a (chunk-id, root-index) pair. The encoder is
structurally unable to emit a boundary that splits a subtree mid-way, so the invariant holds by construction, not by
check. For the oversized-subtree case (one subtree larger than the window), the resolution is an intra-subtree spill
that still preserves the root-escape property: the large subtree is emitted as a chain of chunks whose non-root
chunks are private continuations of the same subtree, reachable only from within it and never referenced across the
outer boundary, so the escaping reference is still exactly the subtree's root. The format distinguishes a
cross-subtree link (root only) from an intra-subtree continuation (private, dropped once its parent consumes it).

Justification. This is the crux immutability lemma at chunk granularity, so making it a format invariant is making
the lemma structural. The post-order interval structure (children before parents, backward-only links) is what makes
"already-emitted chunk root" well-defined, and backward-link acyclicity (backward-only links imply a DAG imply
single-pass termination) is the totality argument the load-time checker needs. The (chunk, index) discipline is
Cap'n Proto's arena-of-segments with cross-segment pointers as the recognised complexity cost, scoped down to
root-only cross-references so the complexity is bounded.

Devil's advocate. The oversized-subtree resolution reintroduces exactly the thing streaming was avoiding: an
intra-subtree continuation chain whose earlier links a later link may reach, which looks like it needs the earlier
chunks retained. The escape is that intra-subtree links are forward-consuming within one post-order walk (a
continuation is consumed by the single parent that spawned it and never re-referenced), so the frontier stays
bounded even inside a giant subtree, but this needs its own one-paragraph proof, and it is the place the streaming
spine is genuinely hard. If that proof does not close, the fallback is a hard limit on subtree size (a subtree larger
than the window is a load error), which is honest but restricts the value shapes a consumer can produce. The talk's
job is to close the continuation-is-consumed proof or accept the size limit, explicitly, before the spine locks.

### Talk 4: the shared validation kernel, one source and three instantiations

What to design. Call 2's three loci run the identical kernel. Design the kernel's signature and structure so one
source compiles as a comptime type generator, as a native `build.zig` content step, and as the native consumer-side
checker, plus the differential gate that proves the instantiations agree.

Proposed direction. Write the kernel once in Zig as an iterative, allocator-free, panic-free, name-keyed traversal
over the value-arena and the IR wire records. Iterative means an explicit work-stack rather than native recursion;
the clean way to get there from the natural recursive walk is defunctionalization (Reynolds, definitional
interpreters, 1972; Danvy and Nielsen, defunctionalization at work, PPDP 2001), which turns the recursive
continuation into an explicit stack of defunctionalized frames, exactly the abstract-machine end of the functional
correspondence (Ager, Biernacki, Danvy, Midtgaard, PPDP 2003). The kernel takes its buffers and its sink as
parameters (caller-buffers, no allocator), so the three loci differ only in what they pass: comptime passes the
language table and asks for a `@Type`; the build step passes bundled content and a memory sink; the consumer passes
the arriving script and the host sink. The differential gate runs the kernel at two binding times over the same
input and asserts identical output.

Justification. The kernel is the generating extension's output, and running one source at multiple binding times is
the mix equation's operational content (`[[kernel]](static, dynamic) = [[mix(kernel, static)]](dynamic)`,
Jones-Gomard-Sestoft). The defunctionalized explicit stack is not just an implementation convenience: it is the
principled route from the recursive specification (easy to prove correct) to the iterative machine (required by call
4's job two, comptime-stack safety), and the functional correspondence is the proof that the two agree. The
differential gate is per-instance translation validation (Pnueli-Siegel-Singerman; Necula), the assurance mechanism
call 6 already blessed, applied to the one place three instantiations could diverge.

Devil's advocate. Three instantiations are three chances to diverge, and the differential gate is the only thing
standing between "one kernel" and "three kernels that happen to share source." If the gate is weak (too few inputs,
or only the happy path), the "one kernel" claim is aspirational. The gate must be a real build-blocking gate with
adversarial inputs (the negative arms), and the ReleaseFast-versus-ReleaseSafe overflow-agreement arm is part of it,
because comptime and native can disagree on integer overflow behaviour. The counter-risk is that a fully general
kernel parametric over everything is slower or larger than three hand-specialised ones; answer: measure it (the
scaling probe is in the experiment), and accept a small constant cost for the single-source guarantee, which is the
whole point.

### Talk 5: the sink capability, generalised across in-process, pipe, and spill

What to design. Call 3 makes spill one sink policy. Design the sink capability so it carries in-process buffer, out-
of-process pipe, memory-spill, and disk-spill under one contract, with re-validate-on-read-back and content-addressed
naming built in.

Proposed direction. The sink is the `#[repr(C)]` reserve/commit pair the transfer topic already defined (two function
pointers plus opaque userdata), and the policy is a host-selected backend behind it: ikiuni supplies a memory-only,
overflow-is-an-error backend; a batch consumer supplies a disk-backed one; the standalone exe uses its stdout pipe.
The runtime is parametric over the capability and never names a concrete backend. Read-back re-runs the bounds-and-
depth validation unless the backend is a process-private temp with an integrity check. Spill artifacts are keyed by
(stage, chunk-index) or content hash, never by timestamp, PID, or directory order.

Justification. This is the object-capability model (Dennis and Van Horn, CACM 1966; Miller, robust composition,
2006): authority to perform I/O is a capability the host holds and lends, not an ambient power the runtime reaches
for. The pull-based reserve/commit sink is an iteratee (Kiselyov, iteratees, FLOPS 2012), the functional
formalisation of a bounded-buffer consumer that drives the producer, which is why backpressure falls out (a full
buffer blocks the producer, the OS pipe buffer is the backpressure for the exe path). Content-addressed naming is
Dhall's semantic-hash discipline and the reason DTVM's deterministic middle form reproduces; it is what makes spill
replay deterministic across machines.

Devil's advocate. Content-addressed naming costs a hash per chunk, and re-validate-on-read-back doubles the
bounds-check cost on the spill path. For a batch consumer moving gigabytes that is real overhead. The answer is that
the overhead is on the spill path only (the common case never spills, the window suffices), and the alternative
(trusting spilled bytes) is unsound the moment the spill backend is anything but a process-private temp. The
trusted-local exemption is the pressure valve: a consumer that controls its own temp directory can declare it trusted
and skip the re-validation, paying an integrity check instead. That keeps the fast path fast and the untrusted path
sound, but it puts a real decision in the host's hands, which the capability model is exactly designed to carry.

### Talk 6: the depth cap, declared as a bounded lattice height

What to design. Call 4 makes the cap first-class. Design where it is declared, how it is threaded through the three
loci, and the invariant that it is tunable up to a ceiling but never to unbounded.

Proposed direction. The cap is a language-declared limit with a hard framework ceiling, exposed to the consumer as a
tunable between a floor and that ceiling. It is threaded as a comptime const at the type-gen locus (so the work-stack
and frontier buffers are fixed-size arrays sized by the cap), and as a checked runtime limit at the content and
consumer loci. The design states, at the declaration site, that the cap is the lease-lattice height and that no_alloc
under streaming depends on it being finite, so no maintainer can relax it to unbounded without a review that
understands all three roles.

Justification. The cap is the finite height of the lease lattice, and finite height is what makes the monotone
dataflow inference converge in one pass (Kildall, POPL 1973; Cousot-Cousot, POPL 1977) with bounded resident state.
The same finite bound serves the work-stack (call 4 job two) and the untrusted traversal (Cap'n Proto's mandatory
depth limit). Sizing the fixed buffers by the cap at the comptime locus is what makes no_alloc a compile-time
property rather than a runtime hope.

Devil's advocate. A legitimate deep-nesting batch consumer (a code generator emitting deeply nested structure) wants
a high cap, and the caps-are-defaults-not-policy rule says do not hardcode consumer policy. The resolution is that
the cap is tunable, just not to infinity: raise it and pay a larger fixed buffer, the finite-height guarantee
survives because the ceiling is finite. The tension is real only if some consumer needs genuinely unbounded depth,
which for a value produced under a residency budget is a contradiction (unbounded depth cannot fit a bounded window),
so the ceiling is not an arbitrary restriction, it is implied by the streaming budget the consumer already accepted.

### Talk 7: the armed generational-reference harness, the lease oracle

What to design. Call 1's axiom set (the per-family link/consume bits, now derived from effects) is discharged
empirically. Design the CI harness that arms Vale-style generational references to falsify those axioms, its coverage
obligation, and its wiring into the differential gate.

Proposed direction. The harness is a CI-only, never-shipped instrumentation that arms generational references (Vale's
mechanism, which itself elides the checks over immutable regions, vehje's case) to detect any dereference into a
region that has been reset, that is, any use-after-region-close a wrong lease bit would permit. It runs the census
consumers' families, and its coverage obligation is census-driven: every family op that carries a retention effect
(the effectful rim and the `Raw` escape from talk 2) must have a harness test that exercises its link/consume bit
under a scenario that would fault if the bit were wrong. It is wired into the differential gate so a schema change
that breaks a bit fails the build.

Justification. This is the catalogue-every-edge-case-as-a-test discipline applied to a proof obligation: the axiom
set is exactly the set of things the metatheorem assumes and cannot prove, so it is the set the tests must cover.
`08` names the harness as the dynamic residual of the lease property in the gradual-verification sense, run under CI
to falsify the static axioms, which is why it belongs in the same frame as the runtime checks rather than being a
bolt-on.

Devil's advocate. A harness falsifies only the bits it exercises. An unexercised rim op stays an unfalsified axiom,
so the harness is only as good as its coverage, and coverage of "every retaining op across every census consumer" is
a moving target as consumers are added. The mitigation is the coverage obligation as a gate: a family op that
declares a retention effect but has no arming test fails CI, so adding a consumer forces adding the tests. That turns
an open-ended assurance into a bounded, checkable one, but it does mean the harness's completeness is a discipline
the census must maintain, not a property proven once. That is the honest cost of an empirically-discharged axiom set,
and it is why the effect-derivation (talk 2) matters: fewer independent axioms means fewer things the harness must
cover.

### Talk 8: the lambda-veh proof-document skeleton, drafted in parallel

What to design. Call 7 drafts the form-fixed skeleton in parallel with the experiment. Design the skeleton: the
two-level core, the region store, the per-axis judgments, the theorems, the lemmas.

Proposed direction. A two-level (staged) core `lambda_veh` over the eleven ratified forms, small-step operational
semantics with an explicit region store indexed by a live-depth stack, and a binding-time annotation on each subterm
in the Nielson-Nielson two-level style (two-level functional languages, 1992), extended to the three levels of talk
1. Per axis, a static judgment plus its AGT-derived gradual judgment (Garcia-Clark-Tanter, POPL 2016). Theorem T1
(staging soundness, the mix equation, by Consel-Danvy and MetaML annotation soundness, Taha-Sheard, PEPM 1997). T2
(the gradual guarantee per axis, by AGT and the refined-criteria formulation, Siek-Vitousek-Cimini-Boyland, SNAPL
2015). T3 (region soundness, progress and preservation, by Calcagno-Helsen-Thiemann 2002 restricted to the LIFO
chain). Plus the crux immutability lemma, the cross-chunk lemma discharged by construction (talk 3), and
backward-link acyclicity as the load-checker totality argument.

Justification. The forms are ratified, so the skeleton is fixed and drafting it now costs nothing later; only the
per-family axiom table is experiment-dependent, and it slots in. `08`'s central contribution is that all five axes
are instances of one frame (staged gradual verification indexed by binding time), so the skeleton proves one
soundness-argument shape (Galois connection composed with the mix equation) once and instantiates it five times,
rather than five separate metatheories. The concrete inference algorithm to cite for the lease pass is the region
inference algorithm (Tofte-Birkedal, TOPLAS 1998), specialised to the LIFO chain where it degenerates to
depth-min propagation.

Devil's advocate. Drafting a proof before the experiment risks proving the wrong theorem if the forms shift. The
forms are ratified, so the risk is low, but "ratified" is a claim about the current design record, and the
value-node-kind schema (op's deferred round) could interact with the core forms in ways that move the region store's
shape. The mitigation is to draft the skeleton over the eleven core forms only and keep the value-node-kind schema
as a parameter the store typing is generic over, so a later schema round instantiates the parameter without
reopening the skeleton. That is the same generic-over-the-open-set discipline the codegen fold already uses.

### Talk 9: the crate taxonomy, with the third locus's home

What to design. Calls 2 and 6 add the shared kernel (compiled three ways), the `build.zig` content step, and the
guarantee inventory to the crate surface. Design where each lives.

Proposed direction. Introduce a kernel crate (working name `vehje-kernel`) that owns the Zig validation kernel source
and its three-loci build wiring, distinct from `vehje-codegen` (the Rust output machinery) and from
`vehje-runtime-abi` (the C ABI and wire formats). The `build.zig` content-validation step is part of `vehje-kernel`'s
build graph. The guarantee inventory (call 6's per-axis labelling) is a document under the design surface, not a
crate. The value-arena format and the sink capability stay in `vehje-runtime-abi`; the safe reader stays in
`vehje-runtime-driver`.

Justification. The kernel is a single artefact with a well-defined boundary (it consumes wire records and the
language table, it produces a verdict), so it earns its own crate rather than being smeared across codegen and
runtime-abi. Separating it from `vehje-codegen` keeps the Rust output machinery (which emits the validated data) on
one side of the language-build boundary and the Zig kernel (which consumes that data at three loci) on the other,
which is exactly the level split talk 1 draws.

Devil's advocate. Does the kernel belong in its own crate, or is it part of `vehje-runtime-abi` (it reads the same
wire records) or `vehje-runtime-driver` (it runs at the consumer)? A separate crate risks a thin crate that mostly
re-exports. The argument for separateness is the three-loci build wiring, which is real infrastructure (comptime
type-gen, a native build step, and a consumer library, from one source) that does not belong in the ABI definition
crate or the driver. If it turns out thin after the experiment, folding it into `vehje-runtime-abi` is a cheap
later move; starting it separate keeps the build wiring from polluting the ABI crate's dependency graph. This is a
low-stakes, reversible layout call, so the direction is "separate crate now, fold later if thin."

### Talk 10: the design document's spine, staged gradual verification

What to design. Call 6 becomes mechanical via `08`'s frame. Design the DESIGN document's organising principle so the
three proof axes, the certified generation, and the guarantee inventory read as one thing rather than three.

Proposed direction. Frame the whole design as staged gradual verification indexed by binding time: a two-input engine
that discharges each safety property at the earliest binding time its input is known, static where early (folded to a
compile error), dynamic where late (a bounded single-pass no-alloc check), with the specialiser as the cast-inserter
and the gradual guarantee composed with the mix equation as the central soundness claim. Each axis is a labelled
instance. The certifying-versus-certified distinction (talk in call 6) and the TAL structural precedent anchor the
"compiled the proof into structure" claim.

Justification. One frame for five axes is the difference between a design a reader trusts and a pile of separate
gambles. It gives the seven blessed calls one shared justification (each is a fact about staged gradual
verification), hands the proof document a formal target with the prior-art technique named per theorem, and locates
the honest guarantee inventory as a labelling exercise rather than a hand-curated list.

Devil's advocate. The frame is dense with jargon (gradual verification, Galois connection, mix equation, binding-time
analysis), and a DESIGN document that leads with it reads as academic posturing, which the workspace vocabulary rules
forbid. The mitigation is that the frame is the skeleton, not the surface prose: the document explains what each axis
guarantees in plain terms and cites the frame as the reason the guarantees compose, rather than opening with the
theory. The frame earns its place in the proof document (talk 8) and in a single "why these compose" section of
DESIGN, not in the surface. That keeps the reader who wants to use the framework unburdened and the reader who wants
to trust its soundness satisfied.

## Part III: related theory, banked

The panel named a strong set of theory. This part banks the theory it has not yet named (with what it is and where
it lands in vehje) and cross-references the theory it has where a talk above leans on it. All new banks were
confirmed reachable and correctly attributed during this pass.

### Newly banked (not previously in the panel or topics)

- Multi-level binding-time analysis and multi-cogen. Glück and Jørgensen, efficient multi-level generating
  extensions for program specialization, PLILP '95, LNCS 982; multi-stage specialization with relative binding
  times. Lands on: talk 1 (three binding times as a lattice), talk 2 (the level at which each effect discharges).
  The panel used two-level framing; vehje is genuinely three-level, and this is the tool for it.
- Definitional interpreters and defunctionalization. Reynolds, definitional interpreters for higher-order
  programming languages, 1972; Danvy and Nielsen, defunctionalization at work, PPDP 2001. Lands on: talk 4, the
  principled route from the recursive lease/validation walk to the iterative explicit-work-stack kernel call 4
  requires.
- The functional correspondence between evaluators and abstract machines. Ager, Biernacki, Danvy, Midtgaard, PPDP
  2003 (also BRICS RS-03-13). Lands on: talk 4, the proof that the recursive specification and the iterative machine
  agree (closure conversion, CPS, defunctionalization of continuations).
- Substructural and linear type theory. Girard, linear logic, TCS 1987; Wadler, linear types can change the world,
  1990. Lands on: call 1 and talk 2, the link/consume bit read as an affine-versus-unrestricted distinction, the
  second theoretical home for the lease schema alongside regions, and the place to reach when immutability is relaxed
  for `vehje-lua`.
- Attribute grammars and one-pass synthesised-attribute evaluation. Knuth, semantics of context-free languages, 1968.
  Lands on: the lease inference as a single bottom-up pass computing a synthesised attribute (the lease depth) over
  the post-order structure, which is why op's "one pass, no extra walk" constraint (topic 1316) is not just an
  optimisation but the natural shape of an L-attributed evaluation.
- Typed Assembly Language and type-preserving compilation. Morrisett, Walker, Crary, Glew, from System F to typed
  assembly language, TOPLAS 21(3), 1999. Lands on: the 1627 core move, types preserved through compilation into a
  low-level form whose structure is checkable without the prover present, the closest structural precedent for "the
  proof compiled into the generated Zig's structure."
- Certifying versus certified compilation. Necula and Lee, the design and implementation of a certifying compiler,
  PLDI 1998. Lands on: call 6, the precise word for what vehje does on the decidable axes (emit a per-instance
  checkable certificate, re-verified after decode), distinct from certified (proven once, CompCert).
- The object-capability model. Dennis and Van Horn, programming semantics for multiprogrammed computations, CACM
  1966; Miller, robust composition, PhD thesis 2006. Lands on: talk 5, the sink as a lent capability the host holds,
  the formal basis for "the runtime owns no I/O policy."
- Iteratees. Kiselyov, iteratees, FLOPS 2012. Lands on: talk 5, the functional formalisation of the pull-based
  bounded-buffer reserve/commit sink and why backpressure falls out of it.
- The region inference algorithm. Tofte and Birkedal, a region inference algorithm, TOPLAS 20(4), 1998. Lands on:
  talk 8, the concrete inference algorithm the lease pass specialises (to depth-min propagation over the LIFO chain),
  the algorithmic complement to Tofte-Talpin's calculus.
- Two-level functional languages. Nielson and Nielson, two-level functional languages, Cambridge 1992. Lands on:
  talk 8, the binding-time annotation style for `lambda_veh`'s subterms, extended to three levels per talk 1.
- Hermetic and reproducible build systems, as engineering precedent. The Nix and Bazel hermetic-build lineage. Lands
  on: talk 4 and call 2, the `build.zig` content-validation step as a deterministic, version-pinned TCB item, the
  engineering practice of a reproducible build step whose output is trusted because its inputs and toolchain are
  pinned.
- Separation logic, as an adjacent proof vehicle noted but not adopted. Reynolds, separation logic, LICS 2002;
  O'Hearn and collaborators. The reachability-respects-lease invariant is a separation/ownership invariant, and
  separation logic is the alternative proof vehicle if the region-calculus route ever strains; the region calculus
  stays the chosen one, this is the recorded fallback.

### Cross-referenced (already named, relied on above)

- Tofte-Talpin region calculus (1997) and Calcagno-Helsen-Thiemann syntactic soundness (2002): the lease
  metatheorem's calculus and proof technique, calls 1 and 5, talk 8.
- Talpin-Jouvelot, the type and effect discipline (LICS 1992): region and effect as one system, the basis for the
  effect-derived lease bit, call 1 and talk 2.
- Futamura projections (1971) and Jones-Gomard-Sestoft partial evaluation (1993): the whole architecture as the
  first Futamura projection lifted to the language level, call 2, talk 4.
- Abstracting Gradual Typing (Garcia-Clark-Tanter, POPL 2016), the gradual guarantee (Siek et al., SNAPL 2015),
  gradual verification (Bader-Aldrich-Tanter, VMCAI 2018), gradual effects (Bañados Schwerter et al., ICFP 2014):
  the staged-gradual-verification frame, call 6, talks 2, 8, 10.
- Cousot-Cousot abstract interpretation (1977) and Kildall monotone dataflow (1973): the lease inference and
  promotion-to-top soundness, and the finite-chain-height convergence, calls 1 and 4, talks 6 and 8.
- Translation validation (Pnueli-Siegel-Singerman, TACAS 1998; Necula, PLDI 2000) and proof-carrying code (Necula,
  POPL 1997): the bridge, validated-not-verified with PCC re-checking for the decidable axes, call 6, talk 4.
- CompCert (Leroy, CACM 2009) and CakeML (Kumar et al., POPL 2014): what semantic certification actually costs and
  why "certified" must scope to structural properties, call 6.
- Initial-algebra folds (Meijer-Fokkinga-Paterson, FPCA 1991): dispatch totality as totality of a fold over a
  coproduct, call 6.
- MetaML (Taha-Sheard, PEPM 1997): the annotation-soundness erasure property behind "one kernel, two (three)
  binding times," talks 4 and 8.
- Vale generational references: the armed-harness mechanism, call 1, talk 7.
- de Bruijn levels (1972): the lease-depth representation, call 1.
- Cyclone (Grossman et al., PLDI 2002): the map of what a relaxation of immutability would cost, call 1, talk 2.

## Closing

The seven settle as seven blesses, because the panel posed them correctly and the field has standard answers. The
one sentence to carry out of Part I: the trusted surface of the whole design is the per-family retention declarations
(the link/consume bits, now derived from effects), and the armed generational-reference harness is the oracle that
keeps them honest; everything else is certified by construction or tested in the open. Part II's ten talks are what
the settlements unblock, and they partition cleanly: talks 1, 2, 3, 6 harden the proof surface (binding-time
levels, the effect-derived lease bit, the chunker invariant, the depth cap); talks 4, 5, 9 build the machinery (the
three-loci kernel, the sink capability, the crate home); talks 7, 8, 10 are the assurance and its framing (the
harness, the proof skeleton, the document spine). The critical path through them is talk 3 (the chunker contract,
because the streaming spine waits on it) and talk 8's cross-chunk lemma discharge, exactly the item call 5 elevated.
Everything here is designable now, before the widened experiment, which is the point: the experiment falsifies the
schema, but the shape the schema lives in is settled, and these talks lay it out.
