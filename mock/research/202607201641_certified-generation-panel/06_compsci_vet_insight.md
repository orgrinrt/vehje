# A compiler-theory veteran's take on the seven calls (insight, audit 6)

**Date:** 2026-07-20
**What this is:** the seven decisions the agent flagged as op's calls, answered as if put to a
compiler-theory / PL veteran, one section per call, with justification and references. The preface below is
the agent's chicken-count message verbatim, so the seven questions are readable with the context that produced
them.

---

## Preface: the chicken-count that produced the seven calls (verbatim)

All four audits are in, plus our response. Here's the chicken-count.

### The headline

**The certified-generation resolution survived four independent lenses with its architecture intact. No
expert reopened it.** The framework identity holds throughout: no rustc, no LLVM, no typestate at the
consumer, one pre-compiled per-target binary (and Zig's cross-compilation is an unclaimed win for "one binary
per target"). What the panel produced is not a redesign, it's a stack of precision corrections, two real
cost-cliff fixes, a set of decisions that are genuinely yours, and one clear gate (an experiment plus a short
proof) before any doc CL. That's a strong result: the thing you were reaching for is sound, and now we know
exactly what it costs and where.

### Settled, fold these in (the experts converge)

- **The assurance vocabulary was wrong and is now nailed down.** The design's shape is *partial evaluation by
  a hand-annotated specialiser + per-instance re-type-checking + translation validation* (Jones-Gomard-Sestoft
  for architecture, Pnueli/Necula for assurance), not typed metaprogramming and not verified compilation.
  "Certified" scopes to *structural* well-formedness; semantics stay tested; termination is not claimed. Every
  LMS/typed-TH/Terra citation in the topic moves or goes.
- **The bridge is validated, not verified**, and better than that for the decidable axes: re-*prove* family
  and effect inclusion after the comptime decode (recompute masks, re-derive totality), which removes encoder
  trust for those two entirely. Leases can't be re-run, so their bridge-trust reduces to bit-table integrity.
- **The Zig mechanics stack, with its corrections:** exhaustive-switch totality and comptime `@Type`
  illegal-unrepresentable both hold (production prior art exists); comptime-folding is the strongest single
  idea; optionals and error unions carry non-null and rejection. But the untrusted path must be **panic-free**
  (a Zig panic aborts the host across the C ABI), the kernel must be **iterative** (comptime recursion
  segfaults the compiler, not a clean error), decoding must be **byte-shift not pointer-overlay** (`@embedFile`
  alignment), the kernel takes **caller buffers not an allocator** (vtables are comptime-illegal, and no_alloc
  wants this anyway), allocator-free is convention needing a freestanding-target gate, dispatch is **name-keyed**
  (so a tag can't silently route to the wrong handler), the forcing root must be **owned** (std's churned), a
  **differential-testing gate** runs the kernel at both binding times, and Zig pins to **0.16.0**.
- **The lease axis has a real resolution: the depth-ladder schema.** Leases are lexical body depth; inference
  is depth-min propagation over one link/consume bit per operand; the metatheorem is genuinely provable (it's
  the degenerate LIFO region calculus, easier than what Calcagno-Helsen-Thiemann already proved),
  promotion-to-output is a sound fallback, monotone annotations can't introduce unsoundness, and the MLKit leak
  concern is recalibrated away because vehje tears down per-script. Generation-checked refs are the *armed
  verification harness* (never shipped), and distinct ref types are engine hygiene *detached from the lease
  axis*.
- **One inference, two binding times** (Giesen's overturn of Carmack) stands; its soundness *is* the
  comptime-equals-runtime equivalence, which holds on a stated six-condition subset plus the differential gate.

### Two new architectural fixes (Expert 4's cost cliffs, both with clean fixes)

- **Three loci, not two binding times.** The panel conflated three comptime workloads. Specialising the engine
  to the small language *table* at comptime is cheap and mandatory. But folding bundled *content* (scripts) at
  comptime hits a measured superlinear cliff (comptime is ~20x slower than CPython; a nontrivial JSON parse
  takes *minutes*). The fix: give content validation a **third locus, a native `build.zig` step running the
  identical kernel**, keeping comptime only for the type generation it's uniquely required for. Same kernel
  source, three loci: comptime type-gen from the table, native build-step over content, native at the
  consumer.
- **Spill is a host-lent sink policy, not baked disk I/O.** Your streaming instinct is right, but "spill to
  disk" as literally written breaks sandboxed hosts and the per-frame consumer (ikiuni can't take disk latency
  mid-frame, and `std::fs` is banned in `vehje-runtime-abi` already). The fix: **spill is one instantiation of
  the reserve/commit sink the transfer topic already built**, ikiuni supplies a memory-only, overflow-is-an-error
  backend; batch consumers supply a disk-backed one. One mechanism, host-selected policy.

And the genuinely good news Expert 4 surfaced: **streaming and the depth-ladder lease inference are mutually
reinforcing**, not in tension. The same backward-only post-order invariant that makes the linear scan work is
what lets the kernel spill payload bytes while keeping only a bounded lease-frontier accumulator resident, and
the depth cap (owed anyway for untrusted input) is what keeps that accumulator fixed-size, so **no_alloc
survives streaming**. Your macro-expander problem is resolved in principle by exactly this spine.

### The one thing that moved from footnote to critical path

**The cross-chunk lease lemma** ("a cross-chunk reference is by construction consumed-or-promoted, because
chunk boundaries are whole-subtree boundaries") turns out to be the load-bearing invariant that makes
streaming and linear-scan lease inference compatible *at all*. Theory review showed it's the *same lemma* as
the crux immutability lemma viewed at chunk granularity. Expert 4 says it must be proven or designed-in
**before** committing to the streaming spine, not left trailing. It's the single hardest open item.

### The calls I think are yours

1. **Bless the depth-ladder lease schema** as the debt-2/3 resolution, with the sharpened trusted surface: not
   "eleven rows of link bits" but the link bits of *every family op including the effectful and host-boundary
   rim* (a wrong `consume` on a host-callback operand is the single most realistic unsoundness vector, because
   the retention is invisible in the IR).
2. **Bless the three-loci split** (content validation to a native build step).
3. **Bless spill-as-host-lent-sink-policy.**
4. **Accept the hard depth cap as a first-class runtime limit that no_alloc depends on**, not just a safety
   measure.
5. **Elevate the cross-chunk lemma to build-blocking** (proven or designed-in before the streaming spine
   commits).
6. **Bless the scoped meaning of "certified"** (the per-axis guarantee inventory) for the doc.
7. **Sequence: run the widened experiment first, then write the short proof document, then the doc CL.**

The gate before a doc CL is the widened experiment, the short proof document, and the condition lists committed
as build gates; then the crate taxonomy, which now has to account for the third build locus.

---

## How to read what follows

Put those seven to me as a compiler-theory question and I will answer them the way I would answer a student
defending a thesis chapter: what is settled in the literature, what is genuinely a design choice, and where
the language you are using will get you in trouble with a reader who knows the field. I agree with most of the
panel. Where I push, I push on precision, because in this material imprecision is how sound designs acquire
unsound reputations.

## Call 1 — Bless it; the schema is textbook, the axioms are the whole game

Bless it. The depth-ladder is not a novel calculus that needs its soundness discovered; it is the degenerate,
lexically-nested, strictly-LIFO case of Tofte and Talpin's region calculus ("Region-Based Memory Management,"
*Information and Computation* 132(2), 1997), with no region variables, no region polymorphism, and no
`letregion` under polymorphic recursion. Tofte-Talpin's own soundness proof was hard because it handled all of
that; you have thrown all of it away, which is exactly why the proof gets easy. Soundness for the region
calculus was subsequently given syntactically, in ordinary Wright-Felleisen progress-and-preservation style
(Wright and Felleisen, "A Syntactic Approach to Type Soundness," *Information and Computation* 115(1), 1994),
by Calcagno, Helsen and Thiemann ("Syntactic Type Soundness Results for the Region Calculus," *Information and
Computation* 173(2), 2002). Your metatheorem is a store typing indexed by the live-depth stack plus one
invariant, reachability respects the lease order (every pointee's lease is no deeper than any pointer that
reaches it), and it is provable by structural induction over eleven forms. That is a weekend of proof for
someone fluent, not a research risk.

The part the panel got exactly right, and that you should internalise as the real content of the decision:
this is a *conditional* theorem, and the condition is the per-family link/consume bits. That is not a weakness
peculiar to your design. Every effect and region system in the literature discharges its primitive
signatures as axioms: Tofte-Talpin assumes correct effect annotations on primitives, and LMS's `Summary`
lattice is the same thing in an optimising compiler, where the synthesis material states the failure mode
bluntly, "a miscategorization is a silent correctness bug." So the agent's sharpening, that the trusted
surface is the link bits of *every* family op including the effectful and host-boundary rim, is not pessimism;
it is the correct identification of the axiom set, and it is the single most important sentence in the whole
lease story. A `consume` where a `link` was needed on a host-callback or interner operand is a soundness hole,
and it is invisible in the IR because the retention happens in host memory the analysis cannot see. Name it in
the doc as the axiom set, and treat the armed generational-reference harness (Vale's mechanism; Vale's own
work elides the checks over immutable regions, which is your case) as the executable oracle that discharges
those axioms empirically. That is exactly the catalogue-the-edge-case-as-a-test discipline applied to a proof
obligation, and it is the right instrument.

Two riders. State the *crux immutability lemma* explicitly ("a value created inside body A is reachable
outside A only through A's result value"); it is where immutability does the load-bearing work, and it is
precisely what a mutable store breaks, which is why every mutating and retaining op must declare `link`.
Second, promotion-to-outermost as the fallback is sound by the ordinary abstract-interpretation
over-approximation argument (Cousot and Cousot, POPL 1977): the analysis computes a lower bound on required
lifetime, and moving up the lattice is always safe, so join-to-top can never be unsound, only imprecise. That
is what makes the Lua claim (precision loss, not unsoundness) true, *conditional on the rim declaring `link`*.
Bless the schema; bless it with the axiom set named.

Reading: Tofte-Talpin 1997; Calcagno-Helsen-Thiemann 2002; Wright-Felleisen 1994; Cousot-Cousot 1977; the
Cyclone hybrid (Grossman et al., PLDI 2002) as the map of what you would need *if* you ever relaxed
immutability; de Bruijn 1972 for the level representation.

## Call 2 — Yes; this is binding-time analysis, and it is not optional

Yes, unambiguously, and I would go further: the three-loci split is not a clever optimisation the panel
invented, it is what binding-time analysis *tells* you to do, and the design was quietly violating it. The
whole framework is the first Futamura projection lifted to the language level (Futamura 1971; Jones, Gomard,
Sestoft, *Partial Evaluation and Automatic Program Generation*, 1993): specialise the two-input engine
`E(language, script)` to its static input. But "static" is a spectrum, and binding-time analysis exists
precisely to separate what is known-early (the small language table) from what is known-late (content). Asking
Zig's comptime interpreter to fold content is asking the *compiler's own tree-walking evaluator* to run your
kernel over program data, which is the double-interpretation tax on top of comptime's measured ~20x-slower-than
-CPython base rate. The synthesis material already contains the exact warning: LMS's `compile` invokes the
host compiler at runtime and "there is no analogue for a statically-compiled, no-alloc target"; the disciplined
shape is Terra's `saveobj`, "the generator is dev-time only; only the generated artifact ships." A native
`build.zig` step running the identical kernel over content is the Terra-`saveobj` shape, and keeping comptime
only for `@Type` reification (which genuinely cannot happen anywhere but comptime, because only comptime can
mint a Zig type) is the correct residual. It is also the .NET `[DllImport]` to `[LibraryImport]` lesson the
purity synthesis records verbatim, "resolve the boundary shape ahead of time into inspectable static code;
runtime codegen of the boundary was a mistake."

The one thing I would make the design say out loud: this adds a third element to the trusted computing base,
the `build.zig` content-validation binary, which is *itself* generated/compiled infrastructure that must be
deterministic and version-pinned. You have not removed trust, you have moved it to a place where it is cheap
to audit (a native, inspectable, testable executable) rather than expensive (a comptime interpreter run you
cannot step through). That is a strict improvement, but it is a TCB item, not a free lunch, and the crate
taxonomy owes it a home. Bless the split; account for the third locus in the trust inventory.

Reading: Futamura 1971; Jones-Gomard-Sestoft 1993 (chapters on binding-time analysis and the mix equation);
the staged-metaprogramming synthesis section "Runtime code compilation, and why it usually does not transfer."

## Call 3 — Yes, and it is a correctness property, not a preference

Yes. An embeddable library owning its own I/O policy is a category error, and the field settled this a long
time ago under the banner of capability-passing and dependency injection: the runtime should be *parametric*
over its sink, and the host supplies the capability. Your own value-transfer study is the convergent evidence:
.NET's `IBufferWriter<T>` / `System.IO.Pipelines` with `PauseWriterThreshold` backpressure, and the
WebAssembly component model's `stream<T>` over a caller-supplied buffer, are the same inversion, the consumer
lends the buffer and the producer pulls. Baking `open()` into the runtime breaks it in exactly the two places
that matter: a sandboxed or seccomp host with no filesystem, and the per-frame consumer where an SSD write's
hundreds-of-microseconds-to-milliseconds latency both blows a 16ms frame budget and injects nondeterminism.
Making spill one instantiation of the reserve/commit sink the transfer topic already defined is the correct
abstraction, and it is nearly free because that seam exists.

Two things I would insist on as conditions of the blessing, both of which the theory and systems reviews
already raised. First, bytes read back from a host-lent sink are *untrusted input again*, so the read-back
path re-runs the bounds-and-depth validation; the alternative (declare the spill trusted-local with an
integrity check) is strictly weaker and only appropriate for a process-private temp. Second, determinism is
earned here, not free: spill artifacts are named and reassembled by explicit (stage, chunk-index) or content
hash, never by timestamp, PID, or directory enumeration, because `readdir` order is filesystem-dependent. This
is Dhall's content-addressed semantic-hash discipline applied to spill, and DTVM (arXiv 2504.16552) is the
existence proof that a JIT-shaped runtime can be made deterministic at the cost of a deliberately
deterministic middle IR. Bless the sink policy; make re-validate-on-read-back and content-addressed naming
part of the same decision.

Reading: .NET `System.IO.Pipelines` design notes (backpressure); WebAssembly component model `stream`/`future`;
Dhall semantic hash (the purity synthesis); DTVM, arXiv:2504.16552.

## Call 4 — Yes; the cap is load-bearing three times over, so state the dependency

Yes, and the reason to elevate it is that it is doing three jobs and the design has been crediting it for one.
The lease-frontier accumulator is bounded by open-body nesting depth, nesting depth is attacker-controllable (a
script with a million nested `Let`s), so a fixed depth cap is what turns the frontier into a fixed-size buffer,
which is what lets no_alloc survive streaming. That is job one, and it is the one the panel named. Job two: the
kernel must be *iterative*, because Zig's self-hosted compiler runs comptime calls on its own stack and deep
comptime recursion overflows the compiler process rather than yielding a clean quota error (issue #13724); an
iterative kernel needs an explicit work-stack, and the depth cap is what bounds that stack. This is the
identical mechanism Jsonnet's VM uses, an explicit frame stack with a goto state machine instead of native
recursion, precisely to avoid host-stack overflow on deep input (purity synthesis). Job three: it is the
standard untrusted-input safety limit that every zero-copy format ships, Cap'n Proto's 64-deep pointer limit
and 64 MiB traversal limit, rkyv's `bytecheck`, documented in your value-transfer study as *mandatory*, not
optional.

So the cap is simultaneously an untrusted-input guard, a comptime-stack guard, and the no_alloc-under-streaming
enabler. That is why the agent is right that it must be first-class and its no_alloc dependency stated: if a
later maintainer reads it as "just a safety limit" and makes it configurable-to-unbounded for a trusted batch
consumer, they silently reintroduce unbounded frontier growth and break no_alloc. State the dependency in the
design so the cap can never be relaxed without a review that understands all three roles.

Reading: Cap'n Proto encoding spec (traversal and depth limits); the explicit-frame-stack pattern (Jsonnet
`core/vm.cpp`, in the purity synthesis); Zig issue #13724 (comptime recursion overflows the compiler stack).

## Call 5 — Yes; a lemma the whole architecture's feasibility rests on is a premise, and premises get discharged first

Yes, and this is the one call I feel most strongly about, because it is a methodological point the field
learns and re-learns. The cross-chunk lemma is not a proof footnote; it is the invariant on which the central
efficiency claim of the streaming spine depends. If a cross-chunk backward reference can be an ordinary live
inner-body local rather than a consumed-or-promoted value, then re-reading its chunk is required to complete
its lease, which means either retaining all spilled chunks or random-access re-reading, and either one
collapses the bounded-window benefit that is the entire reason to stream. The theory review's observation that
this is the *same* lemma as the crux immutability lemma at chunk granularity (a subtree escapes only through
its result, and a whole-subtree chunk boundary makes the chunk's escaping references exactly its root) is the
key structural fact: the lemma is true *if and only if* the chunker is constrained so that chunk boundaries
are strictly whole-subtree and emission is at result boundaries.

That gives you the right move, which is stronger than "prove it": make it true by construction. Constrain the
chunker so the boundary condition holds as an invariant of the emission format, and then the lemma is a
property you *maintain* rather than a fact you *hope*. Either way, it discharges before the streaming spine is
committed, because everything downstream of the spine assumes it. This is just the run-the-experiment
discipline applied at the design level: a premise that decides whether an architecture works is verified or
made-true first, not after you have built on it. The forced-spill experiment arm is the empirical half; the
constrained-chunker plus the one-paragraph proof (it lives in the same store-typing induction as the
metatheorem, because both range over the post-order interval structure) is the analytic half. Do both, before
the spine locks.

Reading: the reachability-respects-region-order invariant (Calcagno-Helsen-Thiemann 2002) is the formal home
for the lemma; the run-the-experiment-not-the-argument discipline for the methodology.

## Call 6 — Yes, and this is the one place I will not let imprecision pass

Bless the scoped meaning, and treat it as non-negotiable, because "certified" and "verified" are load-bearing
words in this field and misusing them is how a sound design earns an unsound reputation from the first reader
who knows the difference. Type-checking certifies *well-formedness*, never *functional correctness*: a
well-typed program is not a correct program, and a total, exhaustively-switched, name-keyed dispatch can still
route to a handler whose body computes the wrong thing. The reference for what semantic certification actually
costs is CompCert (Leroy, "Formal Verification of a Realistic Compiler," CACM 2009): a Coq proof of semantic
preservation through every pass, and *even CompCert* trusts its unverified pretty-printer and assembler rim;
CakeML (Kumar, Myreen, Norrish, Owens, POPL 2014) closed that rim only by verifying all the way to machine
code. You are buying neither, correctly, so the doc's guarantee inventory must read per axis and never
collapse to one word:

- family inclusion, effect inclusion, non-null, dispatch totality: *certified structural properties*, by
  construction, and for the two decidable set axes *re-proven* after decode so the encoder is not even trusted
  for them;
- lease safety: *conditionally certified*, on the per-family link-bit axioms and a once-proven algorithm over
  a fixed schema, with the bridge trust reduced to bit-table integrity;
- family semantics: *tested*, never proven;
- termination: *not claimed at all*, because there is no totality axis, and Dhall shows what buying one costs
  (fold-encoded recursion, a real expressiveness tax) which vehje correctly declined.

The assurance-vocabulary correction the panel already adopted is the same discipline at the citation level:
the mechanism is partial evaluation (architecture: Jones-Gomard-Sestoft) plus per-instance re-type-checking
plus translation validation (assurance: Pnueli, Siegel, Singerman, "Translation Validation," TACAS 1998;
Necula, "Translation Validation for an Optimizing Compiler," PLDI 2000), and it is *not* the LMS / typed
-Template-Haskell / Terra "well-typed generator proves the output once" story, because Zig comptime is an
unverified specialiser with no staged type system, so it re-checks each instance rather than proving the
generator once. Get the words right and the design reads as what it is, an honestly-scoped, well-precedented
assurance architecture. Get them wrong and a reviewer stops trusting the whole document at the first
overreach.

Reading: Leroy, CompCert, CACM 2009; Kumar et al., CakeML, POPL 2014; Pnueli-Siegel-Singerman, TACAS 1998;
Necula, PLDI 2000; and, for the distinction itself, any treatment of the difference between validation
(per-run) and verification (once-and-for-all).

## Call 7 — Yes; falsify the schema before you formalise it, but draft the invariant skeleton in parallel

Yes, and the ordering has a specific justification beyond "prototype first." The experiment *shakes the
schema*: the negative arms, the scaling probe, and the forced-spill round-trip will move the design (which link
bits are wrong, whether the chunker constraint holds, where the comptime cost cliff actually sits). Writing the
metatheorem first risks proving the wrong theorem, and a proof you have to redo after the schema shifts is
waste. This is ordinary proof-engineering economy: falsify the design cheaply, formalise the survivor. It is
also the run-the-experiment-not-the-argument rule, which is doubly right here because the load-bearing claims
(the comptime cost curve, the cross-chunk lemma, the panic-free C-ABI behaviour, the ReleaseFast overflow
agreement) are all things a two-node toy passes trivially and only a widened experiment falsifies.

The one refinement I would make, so the sequencing does not idle: the *skeleton* of the proof document does
not depend on what the experiment shakes. The formal small-step semantics of the eleven ratified core forms,
the store typing indexed by the live-depth stack, and the reachability-respects-lease invariant are fixed,
because the forms are ratified. Draft that skeleton in parallel with the experiment; only the per-family axiom
discharge and the exact link/consume table are experiment-dependent, and those slot into a skeleton that is
already standing. So: experiment and semantics-skeleton in parallel, then the metatheorem plus the crux
immutability lemma plus the cross-chunk lemma plus the backward-link-acyclicity totality note against the
surviving schema, then the doc CL that is finally allowed to use the word "proven." That is the fastest sound
order, and it never writes a line of proof it has to throw away.

Reading: the run-the-experiment discipline; Wright-Felleisen 1994 for the shape of the progress-and-preservation
skeleton you can draft ahead of the experiment.

## Closing

I would sign off on all seven, because none of them is a gamble; they are the field's standard answers to
questions the panel posed correctly. The lease schema is a solved calculus in its degenerate form, the
three-loci split is binding-time analysis done right, the sink policy is capability-passing, the depth cap is
the same bound three systems already ship, the cross-chunk lemma is a design premise to discharge before you
build on it, the scoped "certified" is simply the truth stated precisely, and the experiment-first sequencing
is proof economy. The single sentence I would carry out of this: the trusted surface of the whole design is
the per-family link/consume bits including the effectful rim, and the armed generational-reference harness is
the oracle that keeps them honest. Everything else is certified by construction or tested in the open; that
one axiom set is where a real bug will live, so that is where the design's attention and its test budget
belong.
