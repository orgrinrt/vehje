# The soul, and its inverse: what still holds, and what was well-intended but died

Worker-fork deliverable (negative take). Companion to the positive enumeration fork and to
`worker-fork_idealistic-synthesis-...`. My charge has two halves: enumerate the living spirit and
intent of vehje in words, and then map, thoroughly and with provenance, the inverse: what we
declined, killed, found insufficient in benches, or superseded in audits, so the "bad list" and the
"maybe-later" list are as legible as the soul.

## Method, and the one discipline that governs both halves

The load-bearing instruction: this is a **qualitative** audit, not a catalogue of every intent the
history ever held. An intent counts as soul only if it **still holds true today** after the benches
and the expert audits. A call that was well-intended, even strongly stated, but did not survive a
bench or an audit, is not part of the spirit; it is in the inverse. So the same fact ("eqsat is the
genuine research keystone", "copy-and-patch is THE keystone accelerator", "the runtime is one
relational engine under all three analyses") can be a proud line in an early topic and still land in
the bad list, because the measurement or the audit that came later overruled it. I have put each
claim on the side the latest evidence puts it, and named where the evidence is, not where the
enthusiasm was.

Provenance uses round-relative topic ids (`202607…`) and `file:line` where it is a source read.
Chronology governs: a later, more rigorous re-measure or audit beats an earlier statement.

---

# PART A: the living soul (what still holds)

## A0. What vehje is (identity, still canonical)

- **vehje is the framework, not a language.** A grammar plugs into the input side and lowers surface
  syntax into a shared IR; a target plugs into the output side, declares what it supports, and emits.
  No single language is vehje; every language is a consumer, the rich general-purpose one included,
  named later, on equal footing with jomini/w3/ts4/lua/polka/ikiuni/viola/mockspace. Settled
  2026-07-19 (`202607192254:11-17`), re-affirmed in the recenter (`202607202330:33-37`), still the
  CLAUDE.md opening. This survived every subsequent topic untouched.
- **The framework serves a census, equally.** The representative and largest consumers lean to
  authoring and templating (mockspace docs, polka dotfiles), evaluating the build environment away
  and emitting configured data or documents; their speed concern is the compile stage and the emit,
  not a per-frame loop (`202607202330:51-57`). Serving that end does not cap the ambition at the
  other end (native, JIT). Both ends are the same machinery aimed at different points on one output
  spectrum (`:59-65`).
- **The two-artifact model, Rust entirely out of the runtimes.** (1) A Rust dev-time language
  compiler compiles a language spec (grammar, families, effects, lease rules, targets), proves that
  language's disciplines sound, and emits validated DATA plus proofs carried structurally; it never
  sees an end-user script and never ships. (2) A Zig composed runtime: one hand-authored engine,
  comptime-specialised to the Rust-emitted data, certified by the Zig compiler at our build; the
  shipped artifact, running every script. Canonical model fixed in the recenter addendum
  (`202607202330:160-204`).
- **Identity mechanism: the type system is the verification layer, certified generation is the
  vehicle.** Illegal states unrepresentable, total dispatch over the family set, comptime-folded
  inclusion, both certifications at our build and neither at the consumer. Benched WORKS as the
  certified-generation core sketch (`202607240015:98-100`).

## A1. The governing ethos (the "spirit" words, decoded)

These are the qualitative commitments the vague words "spirit" and "intent" point at. Each still
governs.

- **Compose proven pieces until a new shape falls out that supersedes the pieces it was built from.**
  Not obeying tradition, not ignoring it. The clearest statement is the novelty-audit's stance:
  "excellence is not found by obeying tradition and not by ignoring it, but by composing established
  pieces until a new shape falls out that supersedes the pieces it was built from"
  (`202607201854_certgen_panel_continuation/04:6-9`). The corollary the same doc gives: "a design
  that congratulates itself on reaching the field's standard answer has told you exactly where to
  look for the missed idea" (`04:19-20`). This is why almost every piece is borrow-and-adapt from a
  shipping system yet "the composition is ours" (`202607202055:200-203`).
- **But never novelty for its own sake.** The same audit runs a Part 4 fairness pass precisely
  because "novelty for its own sake is the failure mode this persona is one step from" (`04:394-397`).
  The soul is the balance: supersede the frame where a real recombination exists, keep what is
  genuinely right, and say which is which.
- **Design the maximal unified shape fully, before any code.** Reject re-tiering the ambitious
  mechanism off the critical path in favour of a locally cheaper floor. "A hole in a design is
  filled by finishing the design, not by choosing a design with fewer holes because it is easier to
  reach today" (`202607202205:23-24`). This is the workspace's anti-YAGNI stance applied to vehje's
  own architecture (`:27-42`).
- **The Core changes to fit the tech, never the reverse.** The twelve-form re-derivation exists
  because the architecture evolved and the Core was re-derived to fit it, not patched from an older
  census (`202607210120:5-8`).
- **Docs = design = trusted; source = untrusted.** The mockspace concept, load-bearing here: the
  design record is the oracle, the implementation is a transient approximation.
- **Keep the whole shape; do not narrow it and do not over-correct by shrinking it.** The recenter
  rejects both the drift (narrowing to one corner) and the over-compensation (dropping the endgame),
  in one breath: "change the boots, keep walking" (`202607202330:26-30`).

## A2. The load-bearing design commitments that survived

Stated as intent, filtered to what the benches and audits leave standing.

- **One graded (co)modal proof spine.** The three static proof axes (family inclusion, effect
  inclusion, lease/region validity) plus binding time and assurance are unified as one graded
  judgment: binding time a graded necessity modality, effects a graded monad, lease/usage a coeffect
  whose grade is a reachability qualifier, assurance itself a grade. One soundness theorem, the
  per-axis results its corollaries, the axis interactions its typing rules. Adopted from the
  novelty-audit's centerpiece (`04:32-90`), made the recentered identity (`202607202330:40-47`).
  Grounded in real algebra: Katsumata graded monads, Petricek-Orchard-Mycroft coeffects,
  Gaboardi-2016 effect-coeffect combination, Granule as the existence proof, Davies-Pfenning modal
  staging, Green-Karvounarakis-Tannen provenance semirings (`04:353-392`, `202607202055:113-123`).
  Still the spine; still largely unbuilt (the check pass sets binding empty today), so it is a living
  intent, not a shipped fact.
- **The handler discipline: one theory for effects, host-calls, and macros.** An effect is an
  operation; a handler is what services it; a host-call is a runtime-effect operation the host
  handles; macro expansion is a compile-stage-effect operation the compile stage handles. The
  build-versus-runtime split becomes "which stage provides the handler", which is the binding-time
  coordinate. The twelfth Core form `Handle` (Plotkin-Pretnar, adapted no-alloc) carries it
  (`202607210120:42-49`), promoted from the novelty-audit's H7 (`04:296-325`). This is genuine
  vehje novelty that held.
- **CR1: no-alloc bounded multi-shot continuations.** `Handle` supports multi-shot resumption
  without heap, via a host-lent bounded budget, a static fit-proof, and determinism-by-construction
  in the well-behaved case. "No shipping effect system is known to do no-alloc bounded multi-shot
  this way, so it is genuine vehje novelty" (`202607210120:92-112`). Benched feasible
  (`202607240015:104`). Living, and the sharpest original contribution.
- **Inclusion, not coverage.** A program carries a type-level set of the families it uses; a target
  declares the set it supports; emission is bounded on containment; anything outside is refused with
  the construct named, never silently degraded (Pandoc's failure mode). From the founding round
  (`202607192254:42-48`), restated under the handler discipline as unhandled-operation inclusion
  (`202607210120:51-54`). Reused, not reinvented: `hilavitkutin-api`'s `AccessSet` typestate with
  vehje family markers (`202607192254:50-59`). Survived intact.
- **One signature, many projections.** The design's recurring winning pattern: one declarative truth
  projected into consistent views rather than parallel sets kept in sync by hand. The signature
  projects into eliminator IR (fold), introduction value-domain (content-as-values, the Doc family
  the introduction fragment), and compile stencils, plus the assurance-indexed logical relation as a
  fourth functor over the same signature (`202607202055:180-183`, novelty-audit H5 `04:230-255`, D5
  `202607210120:38-39`). The lens move (Foster et al.) is also the right answer to the emitter trust
  seam (H2b, `04:153-159`).
- **Certified generation, doubly certified at our build, no prover shipped.** rustc proves the data
  sound; the Zig compiler proves the specialisation sound; the runtime carries no prover. Rust emits
  DATA, not Zig source (the LMS-hard route was rejected, see B3). Settled at topic 1627, re-fixed in
  the recenter addendum (`202607202330:176-183`).
- **"Certified" is a dial, not a binary.** Assurance is a grade per property (static-certified,
  structurally-certified, re-proven-per-instance, dynamically-checked, tested, unclaimed), composing
  by lattice meet, so proof and the differential test are one object at two grades (novelty-audit H8,
  `04:327-351`; the assurance-indexed gradual logical relation, `202607202055:105-111,195-197`). The
  design's stated "prize" and its highest-value unbuilt item; still living.
- **The output spectrum, with proof-directed strategy selection.** Output is a continuum from
  interpretation (reduce to a value) to transpilation (re-express, reduce nothing); every target sits
  somewhere on it as an internal staging choice, so the framework ships one fold, one target
  contract, one proof obligation. The strategy per region is selected by grade/evidence, which the
  benches confirmed is real: the branch strategy is tier-dependent, so one global choice is wrong
  (`202607240015:80-88`). Living and bench-backed.
- **Native is a real endgame, a method-agnostic point on the spectrum, not a privileged tier.** A
  script may be lowered all the way to machine code; the codegen method is ours and open (emit Zig,
  or LLVM/cranelift IR, or machine code, or copy-and-patch stencils), per target. The tier axis names
  only the internal execution form (interpret-arena vs bytecode); native is off that axis
  (`202607202330:68-88`, `202607240015:288-290`). This survived; the "copy-and-patch is THE keystone"
  framing did not (B5).
- **No-alloc, no-std, both sides; a bounded/budgeted/host-lent resource discipline.** Caller-lent
  arenas; leases as reachability qualifiers with a depth-lease floor and a per-reference generational
  residual; three genuine finite structural bounds (environment-width, nesting-depth, input-length),
  not one cap doing six jobs. The host owns the memory, the framework never allocates.
- **The relational-fixpoint substrate for lease inference.** Semi-naive, no-alloc, cost-bounded;
  benched feasible (8M nodes / 16M edges in ~50 ms), with whole-column OR beating delta semi-naive
  for realistic shallow graphs (`202607240015:121-124`). Living. The broader "one engine under lease
  AND eqsat AND load-verify" claim is narrowed by the benches and the audits (B5, B2).
- **The bench-backed runtime shape.** Register VM baseline (beats stack 1.4-3.1x), fold+CSE+DCE
  always-on middle, NaN-boxed runtime values, a batched-column C ABI entry (W>=2) so the runtime can
  vectorise (vert8 ~4.8x, the single biggest lever). All confirmed in the PMU re-run
  (`benches/results/RUN_SUMMARY.md:51-139`).

## A3. Prior-art posture (respected, half-married, and heeded)

- **Respected but not married (borrow-and-adapt, the composition is ours).** Scala 3
  capture-and-separation checking (lease production floor), reachability types (the sharper aliasing
  variant), slotted and colored e-graphs, simdjson parse-don't-validate (the load decode), Perceus
  exact-meet reference counting at emission, oatlog semi-naive relational engine, AARA potentials,
  Granule graded modal types, Davies-Pfenning modal staging, Katsumata graded monads, coeffects,
  Gaboardi-2016, one-shot/linear continuations, Foster lenses, Plotkin-Pretnar handlers, the Futamura
  projections. Each ships; the joining is the novel part (`202607202055:113-137,187-203`).
- **Half-married, because a bench can overrule any prior-art cite.** These were adopted on a prior-art
  or first-principles argument and then had to yield to measurement: Deegen tail-threading, copy-and-
  patch as keystone, eqsat as the compile-stage keystone, semi-naive-delta for the lease fixpoint, the
  24-byte record. The discipline is that the cite is a hypothesis, not a warrant (all now in B5).
- **Heeded despite not concretely aligning.** CompCert's honesty about naming what is trusted versus
  proven (the emitter TCB seam vehje has and must name, novelty-audit H2 `04:128-167`); the "beat one
  competent thread first" COST discipline behind every bench baseline; the certifying-vs-certified
  distinction (Necula) that names what `Assurance::ReprovenPerInstance` already is.

## A4. The core discipline: bench it, always, before choosing (and audit the bench)

- **Bench-decided forks.** Where the fork is "which shape performs better", the resolver is to build
  the candidates and measure, not to argue from first principles or prior art. The round ran an
  unusually large de-risking effort for exactly this, and several findings flipped a decision the
  design had settled by intuition (`202607240015:20-28`).
- **The harness is trusted; we improved it upstream repeatedly.** Per-variant cdylib subprocess
  isolation (no cross-variant LTO), byte-exact cross-validation against an oracle (a divergent variant
  is rejected, not timed), PMU counters where available. Upstreamed to mockspace: the `total = S + k*I`
  cost-model fit, declarative axis-matrix variant generation, the disassembly duplicate-check fairness
  guard, the reference-floor ratio column, `timed_calibrated!` (`202607240015:186-191`).
- **Audit the benches themselves; the reporting layer lies before the harness core does.** The carrier
  matrix's first run over-claimed on several headlines; a four-expert panel found "every defect lives
  in the reporting layer or in one missing measurement, not in the harness core"
  (`202607230922_.../summary.md:9-14`), and every load-bearing claim was re-verified against primary
  source, two optimizer claims replicated by an independent node-count probe (`summary.md:5-7`). Bench
  honesty is part of the soul: a bench that measures the wrong thing is worse than no bench
  (`202607240015:26-28`).

## A5. Surviving-in-spirit from the round before the canonical one (202607192358)

Even where the substance was later superseded, these founding intents still hold in spirit:

- Framework-not-a-language, and the ten-consumer census that serves it (`202607192254`). Survived
  verbatim.
- Open families; a family is a trait, its dependencies supertraits, so "the Rust trait system is the
  ODS" and the closed-core-plus-a-pass-to-write cost (priced in C++/Scheme terms) does not transfer
  (`202607192254:35-59`). Survived; it is why the framework core stays small while consumers carry
  their richness.
- Inclusion-not-coverage and the single typed escape hatch (Scribble's `convertible?` negotiation
  form) (`202607192254:42-48`). Survived.
- The eleven-form first cut (`202607192330`) is superseded in substance by the twelve forms, but its
  spirit ("a small shared Core every grammar lowers into, in the low tens") is exactly intact; only
  `Handle` was added, and by algebra rather than census (`202607210120`, `04:407-409`).

---

# PART B: the inverse (what was well-intended and did not hold)

Each entry: the call, why it was made, what overruled it, where. These are not soul; several were
proud lines when written.

## B1. Design-adjudication kills (the attack, meta-attack, and Carmack rebuttal)

The debate trail earned explicit mechanism kills that "stay killed" (`202607202205:137-138`):

- **shared-implies-promoted.** Promote every shared value node to the root region. Killed: it is the
  ML Kit region leak the reachability paradigm was adopted to avoid. Replaced by Perceus exact-meet
  (the shared node's lease is the meet of its referrers' reach-sets, by a reversed referrer count
  finalised at emission) (`202607202055:79-87,168-170`).
- **copy-everything (the attack's own fix for sharing).** Copy every leased shared value. Killed:
  exponential blowup on the huge nested values the transfer model exists to carry (`:82-84,170`).
- **PE-as-extraction.** Partial evaluation modelled as e-graph extraction. Killed as a false label:
  extraction is a preference not a well-formedness constraint, the mix equation is an equation not an
  optimisation, and saturating an inlining rule diverges, so a no-alloc cap makes lowering
  non-deterministic. Replaced by a well-founded graded unfold that terminates by binding-time grade
  well-foundedness, NOT by a cap, with determinism from confluence on the static fragment
  (`202607202055:60-70,162`). This one bears directly on today's IR-to-IR fixpoint: the round already
  rejected "terminate the unfold by a cap"; the canonical terminator is binding-time well-foundedness.
- **tnum-for-structure.** Cast the whole load check as one depth-bounded tnum abstract interpretation.
  Killed on both domain and bound: tnum is non-relational and cannot express `child_index <
  arena_size`, and abstract-interpretation cost is program-size not nesting-depth. Replaced by a
  parse-don't-validate typed structural decode (simdjson); tnum kept for the numeric residual only
  (`202607202055:71-78,171-173`).
- **the depth-cap pun.** One depth cap doing six jobs. Killed: two of the six were width or length
  wearing the word "depth". Replaced by three genuine structural bounds, AARA-inferred and folded into
  the graded spine (`202607202055:95-99,176`).
- **the Beck-distributive-law hunt for effect-coeffect.** Killed: no distributive law is owed; a read
  then a write is graded def-use (sequential composition), which is Scala 3 capture-plus-separation
  shipping (`202607202055:53-59`).
- **one-one-shot-coroutine as the whole streaming substrate.** Killed: eqsat is multi-shot and
  re-entrant, so the shared substrate is one level up, a semi-naive differential monotone-fixpoint
  (`202607202055:88-94`). (Note the tension this later created: eqsat itself is now parked, B5.)
- **scannerless PEG by default / a depth-bounded packrat memo.** Killed: PEG's silent shadowing and
  GLL's silent ambiguity are both the wrong default for independently authored family grammars.
  Replaced by inclusion-not-coverage grammar composition with declared-and-detected conflicts via
  Brzozowski derivatives (`202607202055:100-104`).
- **the original full-pipeline proposal's asserted synergies.** "Four synergies asserted, three of
  them puns" (`202607202055:28`). The convergence under independent attack is what replaced assertion
  with construction.

## B2. Conservative "generation-behind" shapes the novelty audit superseded

Well-cited, correct on their own terms, and a generation behind the recombination sitting in the
banked citations (`202607201854_certgen_panel_continuation/04`). Each is dead in favour of its right:

- **unify-by-analogy (a filing system)** superseded by **unify-by-construction (grading)**: "a filing
  system gives you one vocabulary; it does not give you one theorem" (`04:44-46,68-73`).
- **lease-as-effect** superseded by **lease-as-coeffect** (a per-operand usage grade, not a per-op
  effect flag; the effect flag loses per-operand resolution) (`04:94-125`).
- **the differential gate on the kernel** superseded by **translation-validation on the emitter seam**:
  the kernel is one source compiled twice, so the two binding times agreeing is close to a tautology;
  the trust actually leaks at the Rust-to-Zig emitter, which nothing gated (`04:128-167`). The
  intent to close that seam (round-trip validation, or the lens projection, or a self-checking typed
  manifest) is living; the mechanism is open. (This is exactly the layer where the per-slice manifest
  hash unsoundness the panel later flagged lives.)
- **binding-time-as-a-chain** superseded by **binding-time-as-a-lattice** over the four knowledge
  sources {language-author, bundler, host-loader, runtime}, because a partly-bundled partly-arriving
  script is a join, not a point on a chain (`04:169-198`, adopted `202607210120:69-71`).
- **value-kinds as a parallel closed set** superseded by **the introduction-form projection** of the
  one algebra (`04:230-255`).
- **eleven-forms-complete-by-census** superseded by **twelve-forms-complete-by-algebra** (`04:296-325`).
- **certified-as-a-binary** superseded by **certified-as-a-grade** (`04:327-351`).

## B3. Identity errors, both directions, and the locus drift (the recenter)

- **The drift: narrowing vehje to a single-purpose JIT-and-native scripting runtime with a game
  engine as its reason to exist.** Rejected: the tech was right, the identity slipped; vehje serves a
  census, and the authoring/templating end is first-class (`202607202330:18-65`).
- **The over-compensation: dropping the native endgame, demoting JIT and copy-and-patch, treating the
  last three topics as wrong turns.** Rejected "as firmly as the drift it was reacting to"
  (`202607202330:26-30,142-149`). Both directions are dead; the live shape keeps everything and
  removes only the narrow framing.
- **"Rust emits Zig source" (a Rust metaprogram text-printing the specialised engine).** Killed at
  topic 1627 as the hard-to-certify (LMS-hard) route; Rust emits DATA and Zig comptime specialises
  (`202607202330:176-183`).
- **The "dual-locus" framing (a Rust-side per-script analysis locus; one generator emitting both a
  specialised Rust half and a specialised Zig half).** Killed: Rust never runs per-script analyses,
  the runtime does, for all scripts; and the Rust-emits-Zig half is the rejected LMS route. "The two
  Zig things, a runtime and a bridge" dissolves the moment the false split is dropped
  (`202607202330:185-204`).
- **Two stale lines corrected:** 1513's "there is no native endgame" (wrong: there is one, ours and
  method-agnostic) and 1513's "generate Zig and let the Zig compiler own the platforms" (stale: the
  native output path is not locked to Zig) (`202607202330:68-83`).
- **The old canon "vehje is a general-purpose scripting language for game modding".** Corrected
  2026-07-19; that language is one consumer (`202607192254:11-17`).

## B4. The re-tiering / YAGNI-demotion (Carmack), rejected at the frame

Carmack's remedy for every hard piece was the same shape: move the ambitious unified mechanism to a
"later north star" and put a simpler local mechanism on the critical path so the floor ships (one
relational engine becomes three analyses sharing a bitmask; eqsat becomes hash-consing plus a
specializer; the unified logical relation becomes a skeleton; AARA becomes three constants). Rejected
as "YAGNI in sheep's clothing" and "the exact reasoning this workspace exists to reject"
(`202607202205:27-51`). Both halves of the instinct are named so neither is re-trusted: op's own
earlier relay of the re-tier, and Carmack's re-tier itself (`:41-42`). His concrete faults were kept
as the design agenda; his remedy was not. (Note: some of what Carmack wanted to demote, the benches
later demoted anyway, but on evidence, not on YAGNI, see B5. The frame rejection still stands: you do
not demote on "it is more work", you demote on a measurement.)

## B5. Bench-supremacy kills and demotions (measurement overruled the topics)

The sharpest layer, and the one most likely to be missed because the consolidation topic
(`202607240100`) locked several numbers its own round's later, PMU-armed, panel-audited re-run had
already revised. Chronology governs; the PMU re-run (`benches/results/RUN_SUMMARY.md`) and the full-
arc findings (`202607240015`) are the latest word.

- **eqsat, from "the one genuinely original research piece" to OUT.** In the adjudication it was the
  compile stage's keystone and one of three re-scored original-work items (`202607202055:162,187-200`);
  the Carmack rebuttal defended keeping it (`202607202205`). The benches killed it as a shipped
  default: its marginal contribution to the full pipeline is zero-or-negative everywhere, `cse+eqsat`
  is bit-identical in node count to `eqsat` (CSE recovers nothing on top of it), standalone it
  inflates madd 5.6x (the mechanism behind the first run's misreported "142x win", which was eqsat-
  alone pessimization presented as a speedup), and `wideselect` shows an in-pipeline pessimization
  (`RUN_SUMMARY.md:68-78`; `202607230922_.../summary.md:50-58`). Verdict: ship fold+CSE+DCE always-on;
  park eqsat / DAG-aware extraction behind a named trigger. This is the single biggest gap between
  what the topics proudly kept and what the evidence supports.
- **tail-call threading (Deegen, "beats LuaJIT hand-asm by 31%"), flipped.** Imported on the dynamic-
  language literature; measured slower for vehje's ~25-primitive flat IR (switch fastest on straight
  line; threaded ~1.12-1.17x slower) because the switch branch is well-predicted and the threading
  machinery costs more than it saves. It wins ONLY on real control flow (the trace cell 0.44x, retiring
  ~2.25x fewer instructions across basic blocks), so it is reserved for CFG terminator transfers, not
  the default (`202607240015:58-66,198-200`; `RUN_SUMMARY.md:37-44`).
- **copy-and-patch as THE keystone accelerator, demoted.** The recenter kept it as "a keystone among
  keystones" (`202607202330:84-88`); the benches say it is a nice thing among many, not privileged.
  Warm, direct instruction-selection ties it within 1%, and direct isel compiles ~2x cheaper, so for
  short-lived residuals direct isel is the better native tier and predecode dominates when a residual
  runs only a few times (`RUN_SUMMARY.md:80-98`). The native throughput ceiling over the BEST
  interpreter is ~1.5x, not a 10x keystone; the 10x class needs a vectorising JIT, which copy-and-patch
  cannot be (`202607240015:68-78`, `RUN_SUMMARY.md:108-112`).
- **the stack-bytecode middle tier, closed.** Register beats stack every profile (1.41-3.06x); the
  middle tier is redefined as predecoded-register + fold/CSE/DCE (`RUN_SUMMARY.md:51-61,129-131`).
- **the 24-byte three-operand record, washed to a null result.** The earlier sketch flipped 24-over-16
  (avoiding a pool spill); the PMU carrier re-run finds record width within 1% at every size (a density
  choice, pick REC16). Only "operands stay inline" survives, because pool indirection is the measured
  expensive part (`202607240015:44-56`, `RUN_SUMMARY.md:46-49`).
- **eval-all branch strategy under interpretation, worst everywhere.** Up to 4x the field; the
  interpreter never does it. One global branch shape is wrong because the ranking is tier-dependent
  (`202607240015:80-88`).
- **semi-naive delta for the lease fixpoint, beaten by whole-column** on realistic shallow graphs
  (delta bookkeeping not repaid in two rounds) (`202607240015:121-124`).
- **CSE for speed, a non-result.** The cheap-lowering time bench shows no win because LLVM already
  CSEs the recompute; the real, design-relevant benefit is node-count reduction (76% fewer nodes)
  (`202607240015:146-147`). So CSE is kept for node count, not latency.
- **cells deliberately not built, with reasons.** Perfect-hash dispatch degenerates to the function-
  pointer table on a dense opcode set; interned operands cannot win on a value-DAG where operand tuples
  do not repeat. Named absences, because a non-representative cell is worse than a named absence
  (`202607240015:183-184`).

## B6. Bench-integrity self-corrections (how a bench lied, negative precedent about measurement)

Not design kills; precedents about the measurement itself, worth carrying so the same error is not
re-made:

- **The ABI first-pass over-claim.** "W>=8 threshold" was circular (the SoA cell ran its scalar
  remainder below its 8-lane width, so at W<8 the SoA cell IS the scalar path); the "2.4x" was
  dispatch-amortisation plus partial NEON, not an ABI property; the decisive cheap-payload regime was
  never built on the first pass; and ~12% sequential thermal drift confounded the cross-W magnitudes.
  Corrected to: crossing ~9 ns and effectively free, the SoA 2.3-4.1x vectorisation win is the real
  justification, expose one runtime-W batched entry at W>=2 (`202607240015:217-260`).
- **Inverted native cell tags.** The cell tagged `copypatch` was direct instruction-selection; the cell
  tagged `stencil` was the real Xu-Kjolstad copy-and-patch (OOPSLA 2021, not PLDI), and RUN_SUMMARY
  finding 7 inherited the inversion (`202607230922_.../summary.md:59-61`).
- **The cost-model k-sweep, the design's own "central measurement", was never built.** Everything
  shipped is a single warm point at k=16, so every setup cost S is hidden and the tier breakeven k* is
  unknowable (`202607230922_.../summary.md:17-24`).
- **vert8 6x to 4.8x** (the scalar baseline ran the wire interpreter, not the predecoded one, so ~20%
  of the "win" was decode asymmetry) and the native wins measured against the plain rather than the
  best interpreter (`202607230922_.../summary.md:38-48`, `202607240015:207-214`).

---

# PART C: uncertains, with negative precedent (not a definitive no, may return)

These carry a bench-or-audit negative but are not proven-dead; a future consumer, corpus, or measured
breakeven could re-justify them. Named so reaching for one is a deliberate, evidenced choice, not an
accident.

- **eqsat / DAG-aware extraction.** Out as a shipped default, explicitly "park behind a named trigger"
  (`RUN_SUMMARY.md:77-78`). A consumer whose profile the optimiser genuinely helps, with a machine-and-
  effect-aware extraction objective (not pure rewrite-count, which can pessimise), could re-open it. The
  no-alloc bounded slotted-and-colored e-graph remains genuine systems work (`202607202055:198-200`).
- **copy-and-patch native.** Demoted, not deleted; feasibility proven at the mechanism level
  (`202607240015:108`). It is warm-parity with direct isel and could win where its compile cost is
  amortised over enough runs; the unmeasured k* breakeven decides. The stencil-extraction toolchain is
  deferred, not cancelled.
- **AARA / full potential analysis.** Deprioritised to "three constants" when nothing load-bearing
  needed it, then re-opened by CR1: the bounded-multi-shot budget-fit is the first thing that genuinely
  needs a resource bound (`202607210120:114-118`). Negative-then-positive precedent; live again.
- **Heap / full multi-shot continuations.** Excluded in favour of the no-alloc bounded CR1 form; a
  genuine backtracking need could reopen them only behind an explicit escape (`202607210120:62-65`).
- **tail-threading dispatch.** Rejected as the straight-line default, kept for CFG terminator transfers.
  Negative for one use, positive for another; do not read the straight-line rejection as a blanket one.
- **the native-as-a-tier framing.** Killed; native-as-a-spectrum-point kept. The word "tier" applied to
  native is the negative precedent, the spectrum placement is alive.
- **a specific record width / operand count.** Null on the Python-stdlib proxy corpus; a real census
  corpus, when consumers exist, could re-justify a particular inline count (measure-then-lock is still
  owed) (`202607240015:152-154`).
- **single-arena recursive vs double-buffer for the IR-to-IR expansion (op, 2026-07-24, live).**
  Explicitly a bench-decided fork; single-arena recursive is the default hypothesis, double-buffer is a
  live candidate if it benches better. Neither locked. This one has no precedent yet; it is named here
  so the eventual bench is on the record as owed.

---

# The through-line

The soul is "supersede-the-frame-by-composing-proven-pieces, and choose by measurement". Its inverse
is therefore exactly two things: the shapes a better composition superseded (B1, B2, B3, B4), and the
choices a measurement overturned (B5), plus the measurement errors that taught us how a bench lies
(B6). A living intent is one that survived both filters. The most important correction this fork can
hand forward: the consolidation topic's "settled" list still carries three lines the round's own later
evidence retired (eqsat as kept research keystone, the flat "switch beats threading", copy-and-patch as
the privileged native path), and a canonical-grade consolidation must move those three from the soul to
the inverse, on the evidence cited above.

---

# ADDENDUM: audit of the positive enumeration (`worker-fork_the-soul-and-intent-of-vehje.md`)

Owed per op: once the positive soul-and-intent fork settled, audit it from this fork's lens (does it
keep well-intended-but-dead calls out of the living soul, or smuggle any in). Read in full.

## Where it holds (the filter was applied well)

The positive fork obeyed the same qualitative discipline and, mostly, correctly. It keeps the
bench-demoted mechanisms OUT of the living convictions and files them honestly: copy-and-patch as
half-married and demoted below direct isel (its §5.viii, §6), eqsat as revised to zero-or-negative and
a reserved seam (§5.iii, §6), tail-threading as bench-rejected for straight-line and re-scoped to CFG
transfers (§5.ii, §6), and it explicitly calls the consolidation's ~2.0x native figure and Fallin's
inherited 2.0x "doubly-superseded", pinning ~1.5x over the best interpreter (§5.i). It heeds the LMS /
Rust-emits-Zig-source anti-pattern and the re-tiering-rejected-at-the-frame lesson (§6). Its §8
one-sentence soul carries no dead specific. On the central question this fork was created to check,
the positive enumeration does not present a bench-killed call as living soul, with the exceptions below.
Its §9 prior-panel mining and §5 bench-honesty section are additive and correct.

## Three places a superseded shape still rides as living conviction

1. **§2.13, "one number does many jobs (the depth cap)", is a killed pun presented as a living
   conviction.** The specific example is exactly the "depth-cap-does-six-jobs" shape the meta-attack
   killed: two of the six jobs were width or length wearing the word "depth", replaced by three genuine
   structural bounds (environment-width, nesting-depth, input-length), AARA-inferred and folded into the
   graded spine (`202607202055:95-99,176`; my B1). The living form is "three real bounds", not "one
   depth cap doing six". The underlying *aesthetic* ("find the shape where one thing does the work of
   many") survives and is genuine soul; the depth-cap instance of it is dead. The positive fork's §2.13
   should carry the correction inline, or it re-canonises a pun the round already retired. This is the
   clearest instance of the failure mode this audit exists to catch.

2. **eqsat is filed "half-married" where it belongs in "uncertain, with negative precedent".** The
   positive fork's half-married bucket is defined as "adopted-leaning, but a bench can unseat it" (§6).
   eqsat was already unseated: the bench put its marginal contribution at zero-or-negative and removed
   it from the default, parking DAG-aware extraction behind a named trigger (`RUN_SUMMARY.md:68-78`; my
   B5/Part C). "A bench could unseat it" and "a bench already unseated it, and it waits behind a trigger"
   are different states; conflating them reads eqsat as more alive than it is. It is Part C
   (negative-precedent, may return on a consumer profile with a machine-aware objective), not
   half-married. copy-and-patch, by contrast, is correctly half-married: demoted but still an in-tree
   accelerator whose k* breakeven is merely unmeasured. The bucket needs the split.

3. **§2.4 keeps "copy-and-patch JIT per language" inside the identity sentence.** It quotes
   `202607202001:161-164` ("generating a copy-and-patch JIT per language from one semantic definition")
   as "the shape of the sentence is the identity", noting the demotion in the same breath. The
   half-note is not enough: the living identity is "a native output *point* per language, by whatever
   method benches best (direct isel for the short-lived residuals the authoring majority produces)". Letting
   the dead specific ("copy-and-patch JIT") ride the identity sentence, even rhetorically, is how a
   demoted mechanism creeps back into canon by familiarity. Restate the identity with the method left
   open, per the recenter's own "native is method-agnostic" (`202607202330:78-83`).

## One cross-fork tension neither positive fork nor the panel reconciled

The canonical terminator for the partial-evaluation / expansion unfold is **binding-time
well-foundedness, not a fuel cap.** The meta-attack killed "PE-as-extraction" specifically because
saturating an inlining rule diverges and "a no-alloc cap makes lowering non-deterministic", replacing
it with a well-founded graded unfold that terminates by binding-time grade well-foundedness and is
deterministic by confluence on the static fragment (`202607202055:60-70,162`; my B1). Rompf's panel
deliverable proposes terminating today's IR-to-IR fixpoint with an explicit fuel `Cap`
(`tiark_rompf_staging-and-the-line.md:20-26`), and the idealistic synthesis endorses "bounded by
construction". Neither reconciled the fuel-cap proposal against the round's own recorded rejection of
cap-based termination for this exact unfold. The canonical-grade consolidation must state the
terminator as binding-time well-foundedness (a fuel cap is at most a defensive backstop for a
mis-graded family macro, not the mechanism), or explicitly re-open and justify overturning
`202607202055`'s kill. This is the single most load-bearing place where the living design (op's
single-arena recursive expansion) meets a still-standing negative precedent, and it should not be
settled by a panelist's fresh fuel-cap instinct without confronting the prior kill.

## Verdict

The positive enumeration is sound and safe to build the consolidation on, after three corrections
(the depth-cap pun demoted from §2.13 to its three-bounds form, eqsat moved from half-married to
negative-precedent, and the copy-and-patch specific lifted out of the identity sentence) and one
reconciliation (the unfold terminator is binding-time well-foundedness, not a cap). None of these
touches its §8 one-sentence soul, which stands. The two enumerations agree on the identity, the ethos,
op's standing calls, and the bench discipline; they differ only on how alive three specific demoted
shapes are, which is exactly the seam a negative audit is for.
