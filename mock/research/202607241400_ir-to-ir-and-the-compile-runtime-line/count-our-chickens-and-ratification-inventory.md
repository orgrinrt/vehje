# Count our chickens, and the ratification inventory

**Date:** 2026-07-24
**Author:** main agent (Phase 3 of the op-directed canon sequence)
**Purpose:** (1) review what the panel learned and what we ended up with, before writing canon; (2) enumerate
every canonical claim across the two `canonical_candidate_` documents as a numbered inventory, each with its
expert flag, so op can ratify item by item (Phase 4) and each answer is paper-trailed (Phase 5).

This is not the canon. The canon (positive + negative catalogues, written from scratch, synthesised) is Phase 6,
`research/canon/`, and is written only after op has ratified every item below.

---

## Part 1: what we learned, what we ended up with

### The headline
The two candidate documents (the positive soul/intent enumeration and its inverse) HOLD and are fit to base the
canonical identity on, after a bounded, enumerated set of corrections. Four independent reviewers converged on
this: SPJ (fresh auditor), Wingo (runtime double-take), and both enumeration forks' mutual audits. The
one-sentence soul survived every reader unchanged:

> vehje is one binding-time-directed handler discipline, proven statically and compiled away, projected from one
> signature into two certified artifacts, over a spectrum of outputs with no heap and no collector.

### The compile/runtime "line" question that opened all this
Answered, and the same answer arrived independently from a staging read of the code (Tiark) and from the canon's
own axis: the "line" is not a line, it is the four-point binding-time lattice already in the IR
(`grade.rs:105-115`), {LanguageAuthor, Bundler, HostLoader, Runtime}. AOT lives at LanguageAuthor (the runtime-gen
package + Zig comptime = the first Futamura projection as a build step) and Bundler (the Rust side inlining a known
constant). Load-time compilation, not profile-guided JIT, lives at HostLoader (the "runtime contains a compile
stage" debt the ABI DESIGN already confesses). Interpretation is a Runtime execution form of the already-
specialised residual. Literal const-inlining mandates a specialisation STAGE at the binding time the inputs become
known, never a native step: fully-interpreted is coherent per-stage and incoherent across the whole pipeline. That
an independent read reconstructed the canon's own axis from the code is the strongest corroboration it is right.

### The load-bearing corrections the canon must carry (the panel's adjudicated deltas)
1. **Terminator = binding-time well-foundedness, NOT a fuel cap**, stated in three layers: (a) stage
   well-foundedness terminates the stage dimension; (b) a within-stage per-operation productivity condition (a
   macro may not re-introduce an operation of its own family at the same-or-later binding time) needs a checkable
   home, as data on `vehje-signature::Operation`, not a convention; (c) budget-as-diagnostic (a deterministic
   named error at arena exhaustion) is distinct from cap-as-semantics (which residual you get depends on the cap;
   that is the non-determinism the round killed) and distinct from passthrough at the `FamilyExpand` seam. The
   round already killed cap-based termination (`202607202055:60-70`, PE-as-extraction). [negative fork + SPJ-F6 +
   Wingo-3]
2. **Native ceiling = a RANGE with its baseline index**, not a pinned scalar: roughly 1.5x to 2.0x for scalar
   code depending on interpreter form and dispatch predictability, growing on branchy programs, 10x reserved for
   vectorisation. The committed `carrier_native_ceiling/FINDINGS.md` says ~2.0x is the FLOOR at the interpreter's
   best case; a PMU cell gives 1.49x; the spread is unexplained in the record and owed as a measurement. The
   qualitative claim (native never the driver) is safe under every number. Drop "doubly-superseded" (the 2.0x is
   the switch-baseline number the carrier cell reproduces, not a stale intermediate). [SPJ-F2, Wingo concurs]
3. **The §8 drift test is unify-by-analogy as written.** "The four commitments are shadows of the one handler
   discipline" is the exact move the round is praised for refusing: two of the four (census identity,
   native-never-the-driver) are op's identity call + bench evidence, not corollaries of handler discharge. The
   drift test canonises as §1 + §8 JOINTLY (identity AND mechanism); reword "projections of" to "carried by". The
   one-sentence soul stays as a mnemonic. [SPJ-F3, Wingo sharpens from the mechanism side]
4. **CR1 (no-alloc bounded multi-shot continuation) enters canon carrying its scope**, not flat as married
   novelty. Proven: bounded multi-shot by RE-EXECUTION of a pure defunctionalised continuation (sk2 re-runs a pure
   comptime function per choice; McCarthy-old, unconditionally no-alloc). Open: continuation reinstatement without
   heap (prefix sharing = the actual capture problem, unproven) and multi-shot under other effects in the handled
   body (a soundness obligation, not an efficiency refinement; effect-subtraction is a FIXME). The runtime has NO
   continuation model: the CFG interpreter sketch (sk17) is a plain call-stack register machine with no
   prompt/resumption, and it does not compose with sk2. The handler discipline is ONE type-level unification over
   THREE implementation discharges (compile-time IR emit / runtime value-return / runtime resumable continuation);
   only the third needs the hard machinery and it has no runtime home yet. [Wingo-1,2,4]
5. **Macro hygiene is soul-shaped and absent from both candidates.** It is "illegal states unrepresentable"
   applied to generative discharge (fresh binders, capture structurally impossible because resolve runs after
   expansion). Hygiene and multi-shot-frame-freshness are ONE discipline: generative freshness bounded by a
   host-lent budget, seen at four sites, exhaustion a named diagnostic. Add to §2 + a red capturing-macro test.
   [SPJ-F7, Wingo-3]
6. **op's 2026-07-24 live calls have no durable record**, and topic `202607241330` still states the SUPERSEDED
   two-arena unbounded fixpoint. A supersede-forward topic is OWED before/with the canon: single-arena recursive
   as the default, double-buffer as a bench fork, benches-over-topics-over-changelists precedence. [SPJ-F5]
7. **The single-vs-double arena bench fork carries the `NodeRef` arena-branding decision with it.** "Branding
   dissolves" holds only if single-arena wins; if double-buffer benches better, two same-shaped arenas coexist and
   branding is load-bearing again. One fork, decided in one act. [SPJ note, Wingo]
8. **Corrections already applied to the positive fork by its self-audit** (verified by SPJ against source): the
   depth-cap "six jobs" pun demoted to its three-structural-bounds form; eqsat moved from half-married to
   negative-precedent (already-unseated, not could-be-unseated); the "copy-and-patch JIT" specific lifted out of
   the identity sentence (native stated method-open).
9. **SPJ-F1** (positive §5.i factual): the consolidation locked 1.0-1.5x, never 2.0x; the 2.0x was Fallin's from
   `202607220300`. Fix the misquote.
10. **SPJ-F4** (positive §6 stale citations): register-VM "1.9x" is the superseded composite (corrected 1.41x to
    3.06x per profile); NaN-boxing "married on the evidence" overstates a low-to-medium-confidence cell (say
    bench-leaning + carry the tag); the tnum entry re-widens a killed scope (tnum-for-structure killed, tnum kept
    for the numeric residual only).

### The consolidation-side corrections owed (independent of the soul pair, filed as a doc round)
- The consolidation topic `202607240100` still carries three lines its own round's later evidence retired: eqsat
  as "the one genuinely original research piece" (parked, zero-or-negative marginal), the flat "switch beats
  threading" (threading wins on real control flow / CFG terminators), copy-and-patch as the privileged native path
  (direct isel is ~2x cheaper to compile for short-lived residuals). A canonical-grade rewrite moves these three
  from soul to inverse. [negative B5, 6b24, SPJ]
- The structural-hash identity boundary: `202607240100:267-268` names the manifest + fixpoint dedup as consumers
  of the one structural hash; shipped `vehje-ir/src/hash.rs:1-14` says the word-fold is interner-local and
  cross-artifact identity is the byte-image hash's job. The prose is unsound-as-written (within-stage identity
  structural; cross-stage identity byte-image). [SPJ-F8, Tiark addendum finding 0]
- The CFG-of-blocks residual is locked canon with no lowering and unflagged dead type surface
  (`Block`/`BlockTable`/`Function` defined, never constructed); the sketch it cites (sk17) validates execution,
  not lowering from `vehje-ir::Node` into blocks; no `// FIXME` marks the gap. Downstream of `vehje-lower::Anf`,
  which is downstream of the IR-to-IR emit path (the redirection op is steering is the first domino). [Fallin,
  Wingo-2, 6b24]
- The check-reads-`vehje-signature`-schema mandate is inert prose the impl never honoured (zero `vehje_signature`
  uses in `vehje-typecheck/src/`). [6b24, SPJ verified]

### The bench through-line (what the measurements settled)
The value is in the cheap binding-time-directed reducer (fold + CSE + DCE, always-on) and the vertical-SIMD
batched-column ABI (one runtime-W entry, W>=2, ~4.8x the single biggest lever), NOT in the e-graph or the
copy-and-patch stencil. Native is a modest scalar multiplier (a rare compute-bound opt-in), never the driver. The
authoring-and-templating majority is compile-heavy and trivial-execute, so the interpreter's dispatch cost, not
any native multiple, is the number that governs the product. Every architecture call survived bench correction;
five reported numbers did not, which is exactly why "settle which-is-faster forks by bench" is safe.

### The process lesson (worth carrying to the workflow, not the canon)
An audit that enumerates `// FIXME` markers cannot see an UNMARKED gap (my own 2026-07-24 "oracle audit" concluded
"boundary reached" partly on that blindness, and it over-claimed for the CFG mechanism). The second-direction
audit, walk every "settled" bullet against source, is owed. Both live wounds (CFG, signature) are one
CL-verification weakness: surface existence standing in for mandate satisfaction.

### The decision I made about Phase 4's shape (recorded for op to override)
op asked to "go through each and every thing in both lists, one by one." The two lists overlap heavily on the
living soul (both state the identity, the ethos, the convictions). To respect op's time without losing
completeness, Phase 4 walks the UNION of distinct claims: each distinct claim gets one ratification question, with
cross-references where it appears in both lists, and where the two lists disagreed on a claim's status (eqsat,
copy-and-patch) the already-reconciled form is presented. The inverse (Part B, Part C) and the positive-only
sections (prior-art degrees, spirit signals, op-calls) are unique and walked in full. If op wants the literal
double-walk instead, say so and I will re-run the overlapping living-soul items separately.

---

## Part 2: the ratification inventory (the Phase 4 walkthrough list)

Every distinct canonical claim, numbered, with provenance (P = positive doc section, N = negative doc section) and
its expert flag. Phase 4 asks op to ratify each: agree as-is, adopt the flagged/corrected form, amend, or cut.
Phase 5 records op's answer per number.

### Group I: Identity (P§1.1-4, N-A0)
- I1. vehje is a framework, not a language; every language is a consumer (the rich general-purpose one included,
  named later, equal to jomini/w3/ts4/lua/polka/ikiuni/viola/mockspace). [clean; survived every topic]
- I2. Single identity proposition: the type system is the verification layer, and certified generation is the
  mechanism. [clean]
- I3. A framework serving a census of consumers equally; the authoring/templating majority (mockspace docs,
  polka dotfiles) is first-class; ikiuni (game runtime) is one consumer, not the reason vehje exists. [clean]
- I4. A spectrum, not a single purpose: templating end and machine-code end are the same machinery aimed at
  different points of one output spectrum. [clean]
- I5. Two artifacts, Rust in only one: a dev-time Rust language compiler (emits validated data + structural
  proofs, never sees an end-user script, never ships) and a shipped Zig composed runtime (one hand-authored
  engine, comptime-specialised to the Rust-emitted data). [clean]
- I6. Taken whole, vehje is a metacompiler with a unified graded proof, generating a native output point per
  language from one semantic definition by whatever method benches best, with a load verifier bounded by its own
  lease-lattice height. [FLAG: identity sentence must state native as method-OPEN (direct isel for short-lived
  residuals), not "copy-and-patch JIT per language" (that specific is demoted); already corrected in P§1.4]

### Group II: Soul convictions (P§2.1-13, N-A2)
- II1. The type system is the verification layer; the runtime is a dumb evaluator inheriting proven-safe
  programs. [clean]
- II2. Certified generation: prove before lowering, then compile the proof away; Rust emits validated data not
  source; both artifacts certified at our build, neither at the consumer; Rust-emitting-Zig-source (LMS-hard)
  rejected. [clean]
- II3. One graded (co)modal proof spine, unified by CONSTRUCTION not analogy: binding time a graded necessity
  modality, effects a graded monad, lease/usage a coeffect whose grade is a reachability qualifier, assurance
  itself a grade; one soundness theorem, per-axis results its corollaries. [clean; still largely unbuilt (check
  sets binding empty), so a living intent not a shipped fact]
- II4. Inclusion, not coverage: a target declares Supports (families) + Permits (effects); anything outside is
  refused with the construct named, never silently degraded. [clean]
- II5. The one generative idea: effects, host-calls, and macro expansion are one handler discipline; the
  build-vs-runtime split is "which stage provides the handler" = the binding-time coordinate. [FLAG: canon should
  state this as ONE type-level unification over THREE implementation discharges (compile-time IR emit / runtime
  value-return / runtime resumable continuation), only the third needing continuation machinery (Wingo)]
- II6. Output generation is a spectrum from interpretation to transpilation; native is a real point on it, not a
  privileged tier. [clean, bench-backed]
- II7. Three named codegens, the bare word "codegen" retired: runtime generation (per language, dev time), macro
  expansion (IR to IR, per script, compile stage), output generation (IR to output, per script). [clean]
- II8. No garbage collector, no null; absence is an explicit Maybe value form, not a nullable reference. [clean]
- II9. No heap, ever, both sides; no_std no alloc including compile-side passes; caller-lent arenas + host-lent
  budgets; the host owns the memory. [clean]
- II10. The host-lent bounded budget is the one universal resource discipline (memory windows, reachability
  leases, no-alloc bounded multi-shot continuations = three uses of one shape). [FLAG: the CR1 use carries a
  re-execution cost and open reinstatement/under-effects cases; do not present it beside the sink and lease as
  the same cheap shape without the scope (Wingo-1)]
- II11. Strict by design: lease-inference failure is a compile error forcing an explicit annotation, never a
  silent fallback. [clean]
- II12. One truth, many projections: one declarative signature is the single source; the Rust typestate and the
  Zig consts are differential-checked lens projections; the node algebra has three projections. [FLAG: sound only
  with the identity boundary stated (within-stage structural hash / cross-stage byte-image); SPJ-F8]
- II13. One shape does the work of many, but count the bounds honestly: hold the aesthetic; the depth-cap "six
  jobs" instance was a pun, retired for three genuine structural bounds (environment-width, nesting-depth,
  input-length). [already corrected in P§2.13]
- II14. (ADD per SPJ-F7 + Wingo-3) Macro hygiene: generative discharge is fresh by construction (capture
  structurally impossible because resolve runs after expansion), bounded by a host-lent budget, exhaustion a
  named diagnostic; one discipline with multi-shot-frame freshness. [NEW; ratify inclusion]

### Group III: Governing ethos (P§3, N-A1)
- III1. Design the unification now, completely, before any code; a hole is filled by finishing the design, not by
  choosing a design with fewer holes because it is easier today. [clean]
- III2. Reject re-tiering as the defer instinct; a sound floor is not a sufficient floor; the floor being
  buildable today is not permission to stop designing above it. [clean]
- III3. The maximal shape is not ambition for its own sake; it is why vehje is on a stack whose thesis is the
  substrate is designed to the bottom before use. [clean]
- III4. Design through the fault, or prove the exact blocking constraint and design the maximal thing that holds.
  [clean]
- III5. Novelty means proven-in-literature, made load-bearing, fitted; excellence is composing established pieces
  until a new shape falls out that supersedes them; never novelty for its own sake. [clean]
- III6. The counterbalance (boots-not-steps): keep the whole shape; reject BOTH the drift (narrowing to one
  corner) and the over-correction (dropping the native endgame). [clean]
- III7. Docs = design = trusted; source = untrusted (the mockspace concept, load-bearing here). [clean; N-A1 only]
- III8. The Core changes to fit the tech, never the reverse. [clean]

### Group IV: op's standing calls (P§4.1-9)
- IV1. The Core changes to fit the tech, never the reverse (re-derived from the evolved architecture). [clean]
- IV2. The type system is vehje's strength, so adopt the proven-but-not-yet-productionised paradigm and implement
  it (reachability types, graded types, algebraic effects). [clean]
- IV3. Reachability types as the primary lease paradigm (op's self-correction from generational-first;
  generational demoted to the dynamic residual at the avoidance boundary). [clean]
- IV4. Bounded multi-shot Handle via a host-lent budget (CR1), named genuine vehje novelty. [FLAG: carry the
  scope per II10/Wingo-1: re-execution proven, reinstatement + under-effects open]
- IV5. The three settled mandates: Handle as the twelfth form (D3), split the effect lattice into effect graded
  monad + binding-time modality (D4), content-as-values as the signature's introduction projection (D5). [clean]
- IV6. Design through the hard parts; reject the floor-first re-tiering (the Carmack verdict rejected at the
  frame, its faults kept as agenda). [clean]
- IV7. The identity recenter: keep the whole shape, fix the narrow framing. [clean]
- IV8. Native is a real endgame point, method-agnostic (not bound to Zig, not a privileged tier). [clean]
- IV9. The live 2026-07-24 calls: macro/const expansion recursive into the single caller-lent arena (not
  two-arena emit); double-buffer a legitimate candidate if it benches better (arena strategy = bench-decided
  fork); precedence hard-data > topics > changelists; this round is itself canonical. [FLAG: no durable record
  yet; topic 202607241330 still states the superseded two-arena unbounded fixpoint; a supersede-forward topic is
  owed (SPJ-F5)]

### Group V: bench discipline + what the benches taught (P§5.i-viii, N-A4)
- V1. Bench it before choosing; a which-is-faster fork is resolved by building candidates and measuring, not by
  first principles or prior-art authority. [clean]
- V2. The harness is a first-class asset we improve upstream (cdylib subprocess isolation, byte-exact
  cross-validation, PMU counters, the upstreamed cost-model fit / axis-matrix generation / disassembly guard /
  reference-floor column / timed_calibrated). A bench that measures the wrong thing is worse than no bench.
  [clean]
- V3. (taught) Native is a modest scalar multiplier, not a keystone. [FLAG: state as a RANGE with baseline index
  ~1.5x-2.0x, growing on branchy programs, 10x reserved for vectorisation; reconcile by measurement; drop
  "doubly-superseded" and the bare-scalar pin; SPJ-F1, SPJ-F2]
- V4. (taught) Dispatch is tier-and-shape-dependent: plain switch fastest for the lean ~25-primitive straight-line
  IR; preserve-none threading wins on real control flow (CFG terminators). The flat "switch beats threading 1.7x"
  is the stale form. [clean; corrects the consolidation]
- V5. (taught) The optimiser is fold + CSE + DCE, not the e-graph; DCE is a named third arm; eqsat is a reserved
  conditional seam (zero-or-negative marginal), not the frontier. [clean; corrects the consolidation]
- V6. (taught) The batched-column C ABI is the one ABI-shaping result: vertical/SoA SIMD ~4.8x, survives the FFI
  boundary byte-exact (2.3-4.1x), crossing ~9 ns; expose one runtime-W entry W>=2, no per-W symbol zoo. [clean]
- V7. (taught) Recursion is a hard compiler wall; the kernels are iterative (defunctionalised work-stack);
  promotes defunctionalization from principled route to only route. [clean]
- V8. (taught) The wire format keeps operands inline (pool indirection is the expensive part); the exact inline
  count is a measure-then-lock decision against a real census corpus, open. [clean]
- V9. (taught) The pieces compose with predictable additive cost: a 50k-node script runs end to end in ~3.4 ms,
  compile-stage-dominated with a cheap runtime (127 us); incremental 4 ms cold / 0.003 ms warm / 7.7x parallel.
  [clean]
- V10. (taught) Native refined: direct instruction selection compiles ~2x cheaper than copy-and-patch at equal
  warm speed; the authoring majority produces short-lived residuals, so direct isel is the native point and
  copy-and-patch is de-prioritised. [clean; corrects the consolidation]

### Group VI: prior art by degree of commitment (P§6, N-A3)
- VI1. MARRIED (load-bearing spine, adopted by construction): graded (co)modal types (Davies-Pfenning, Katsumata,
  Petricek-Orchard-Mycroft, Gaboardi 2016, Granule); reachability types (Bao et al. 2021 + successors); algebraic
  effects/handlers (Plotkin-Pretnar); the certified-generation lineage (CompCert trusted-rim, PCC/Necula-Lee,
  certifying not certified); the lens principle (Foster et al.); zero-copy value transfer (Cap'n
  Proto/FlatBuffers/Arrow/rkyv, the reserve/commit iteratee, the object-capability window); region inference
  (Tofte-Talpin/MLKit/Cyclone, dropping borrow-exclusivity); defunctionalization (Reynolds/Danvy-Nielsen/Ager);
  ANF (Flanagan et al.); thermometer-encoded grade lattices; notko's fallibility ladder. [clean as a set]
- VI2. HALF-MARRIED (scoped/gated, a bench can unseat): copy-and-patch (demoted below direct isel, still an
  in-tree accelerator whose k* breakeven is unmeasured); Deegen-style triple emission (shape adopted, stencil
  half de-prioritised); the eBPF-verifier-shaped tnum bounded abstract interpretation [FLAG: scope to the
  numeric residual only; tnum-for-structure was killed; SPJ-F4c]; scannerless PEG / GLL+attribute grammars;
  one-shot and linear continuations; QTT for the assurance grade vector; NaN-boxing + register VM [FLAG: register
  VM is 1.41-3.06x per profile not 1.9x composite; NaN-boxing is low-to-medium confidence, say bench-leaning;
  SPJ-F4a, F4b]; CHERI as optional hardware-assurance. [ratify with the two flags applied]
- VI3. MOVED to negative-precedent (already unseated, not merely could-be): equality saturation (marginal
  zero-or-negative, parked behind a trigger). [already corrected P§6; see XVI]
- VI4. Tail-threading is scoped-not-blanket: negative for the straight-line default, positive for CFG terminator
  transfers. [clean]
- VI5. HEEDED, not concretely aligned (respect the lesson, refuse the mechanism): oatlog / relational-engine-as-
  proc-macro (heed the one-engine idea, refuse the heap-using proc macro); Carmack's floor-first verdict (faults
  are the agenda, remedy rejected); LMS on the emission axis (anti-pattern to route around, while LMS's staging
  insight is married on the reducer axis); the full Rust borrow checker (drop aliasing-exclusivity, values
  immutable); Iris-scale logical relations / MLIR-IREE-SPIR-V / Slang multi-target (existence proofs + audience
  framing, not mechanisms adopted whole); AARA (deprioritised to three constants, re-opened by the CR1
  budget-fit). [clean as a set]

### Group VII: surviving-in-spirit from before the canonical round (P§7, N-A5)
- VII1. A small closed Core + open families, two orthogonal AccessSet axes under inclusion-not-coverage
  (superseded forward to twelve forms via Handle, but the shape is unchanged). [clean]
- VII2. Supersede forward, never rewrite a locked topic (the audit-trail discipline, itself a load-bearing
  intent). [clean]
- VII3. The Core is grounded in the real consumer census (re-derived from the tech, but the census's grounding
  role survives). [clean]
- VII4. Open families as traits (the Rust trait system is the ODS); the framework core stays small while consumers
  carry their richness. [clean; N-A5]
- VII5. The committed ABI and the 1845 panel calls (spill-as-capability, the depth cap is finite lattice height),
  re-voiced and not reopened. [clean]

### Group VIII: the one-sentence soul + shadows (P§8)
- VIII1. The one-sentence soul (quoted at the top of Part 1). [clean; survived every reviewer]
- VIII2. The "four commitments are shadows of the one idea" framing. [FLAG: unify-by-analogy; canonise the drift
  test as §1 + §8 JOINTLY; reword "projections of" to "carried by"; SPJ-F3, Wingo]

### Group IX: spirit signals mined from prior panels (P§9.A-F)
- IX1. Unify by construction, not by analogy; the field's standard answer is a smell pointing at the missed
  recombination. [clean]
- IX2. Make illegal states unrepresentable, push the check into generated structure. [clean]
- IX3. Everything borrowed is heap-and-GC-shaped; the no-alloc port is the standing tax paid per adopted
  mechanism. [clean]
- IX4. A handle is an index, never a pointer, whenever memory can move or must be shared (delivers
  zero-copy-equals-wire). [clean]
- IX5. Guarantees are type-system properties discharged before runtime, no separate analysis pass; content-
  addressed determinism is the payoff of totality. [clean]
- IX6. Bench honesty: audit your own benches adversarially; the framework's own soul (the Futamura projection)
  can hide inside its own benchmark, so cross the opacity boundary or be fooled; the ten distilled bench-writing
  rules. [clean]

### Group X: inverse, design-adjudication kills (N-B1) [ratify each as correctly dead]
- X1. shared-implies-promoted (the ML Kit region leak; replaced by Perceus exact-meet). [dead]
- X2. copy-everything (exponential blowup; killed). [dead]
- X3. PE-as-extraction (a no-alloc cap makes lowering non-deterministic; replaced by well-founded graded unfold
  terminating by binding-time well-foundedness). [dead; THIS is the terminator precedent, XVII1]
- X4. tnum-for-structure (non-relational, wrong cost basis; replaced by parse-don't-validate typed decode; tnum
  kept for the numeric residual only). [dead]
- X5. the depth-cap pun (six jobs; replaced by three genuine structural bounds). [dead]
- X6. the Beck-distributive-law hunt for effect-coeffect (no distributive law owed; graded def-use is sequential
  composition). [dead]
- X7. one-one-shot-coroutine as the whole streaming substrate (replaced by semi-naive differential fixpoint).
  [dead; FLAG: its justification leaned on eqsat being a live multi-shot consumer, and eqsat is now parked, so
  re-justify the substrate on lease-inference + load-verify alone; positive-fork addendum]
- X8. scannerless PEG by default / depth-bounded packrat (silent shadowing/ambiguity; replaced by
  inclusion-not-coverage grammar composition with Brzozowski-derivative conflict detection). [dead]
- X9. the original full-pipeline proposal's asserted synergies (four asserted, three puns; replaced by
  construction under independent attack). [dead]

### Group XI: inverse, generation-behind shapes superseded (N-B2) [ratify each superseded]
- XI1. unify-by-analogy (filing system) -> unify-by-construction (grading). [superseded]
- XI2. lease-as-effect -> lease-as-coeffect (per-operand usage grade). [superseded]
- XI3. differential gate on the kernel -> translation-validation on the emitter seam (the trust leaks at the
  Rust-to-Zig emitter; this is where the per-slice manifest-hash unsoundness lives). [superseded; intent to close
  the seam is living, mechanism open]
- XI4. binding-time-as-a-chain -> binding-time-as-a-lattice over the four knowledge sources. [superseded]
- XI5. value-kinds as a parallel closed set -> the introduction-form projection of the one algebra. [superseded]
- XI6. eleven-forms-complete-by-census -> twelve-forms-complete-by-algebra. [superseded]
- XI7. certified-as-a-binary -> certified-as-a-grade. [superseded]

### Group XII: inverse, identity errors both directions + locus drift (N-B3) [ratify each dead]
- XII1. The drift: narrowing vehje to a single-purpose JIT-and-native scripting runtime with a game engine as its
  reason to exist. [dead]
- XII2. The over-compensation: dropping the native endgame, demoting JIT/copy-and-patch, treating the last three
  topics as wrong turns. [dead, rejected as firmly as the drift]
- XII3. "Rust emits Zig source" (LMS-hard, hard-to-certify; killed at 1627). [dead]
- XII4. The "dual-locus" framing (a Rust-side per-script analysis locus; Rust-emits-Zig half). [dead, dissolved]
- XII5. Two stale 1513 lines: "there is no native endgame" and "generate Zig and let Zig own the platforms".
  [dead/stale]
- XII6. The old canon "vehje is a general-purpose scripting language for game modding". [dead, corrected
  2026-07-19]

### Group XIII: inverse, re-tiering / YAGNI-demotion rejected at the frame (N-B4)
- XIII1. Carmack's remedy (move the ambitious unified mechanism to a "later north star", ship a simpler local
  floor) rejected as YAGNI in sheep's clothing; his concrete faults kept as the agenda, his remedy not; both op's
  earlier relay and Carmack's re-tier named so neither is re-trusted. Demote on a measurement, never on "it is
  more work". [dead as a method; the frame rejection stands]

### Group XIV: inverse, bench-supremacy kills and demotions (N-B5) [ratify each]
- XIV1. eqsat from "the one genuinely original research piece" to OUT (marginal zero-or-negative; cse+eqsat
  bit-identical node count; the misreported "142x" was eqsat-alone pessimization; park behind a named trigger).
  The single biggest gap between what the topics kept and what the evidence supports. [demoted]
- XIV2. tail-threading flipped (imported on the dynamic-language literature; measured slower for the lean IR;
  wins only on real control flow; reserved for CFG terminators). [demoted/scoped]
- XIV3. copy-and-patch as THE keystone, demoted (direct isel ties warm within 1% and compiles ~2x cheaper; native
  ceiling ~1.5x over the best interpreter, not 10x; 10x needs a vectorising JIT copy-and-patch cannot be).
  [demoted]
- XIV4. the stack-bytecode middle tier, closed (register beats stack 1.41-3.06x; middle tier =
  predecoded-register + fold/CSE/DCE). [dead]
- XIV5. the 24-byte three-operand record, washed to null (record width within 1% at every size; pick REC16; only
  "operands stay inline" survives). [dead]
- XIV6. eval-all branch strategy under interpretation, worst everywhere (up to 4x the field; tier-dependent
  ranking). [dead for interpretation]
- XIV7. semi-naive delta for the lease fixpoint, beaten by whole-column on realistic shallow graphs. [demoted]
- XIV8. CSE for speed, a non-result (LLVM already CSEs the recompute); kept for node-count reduction (76% fewer),
  not latency. [rescoped]
- XIV9. cells deliberately not built, with reasons (perfect-hash dispatch degenerates on a dense opcode set;
  interned operands cannot win on a value-DAG). [named absence]

### Group XV: inverse, bench-integrity self-corrections (N-B6) [ratify each as measurement precedent]
- XV1. The ABI first-pass over-claim (circular W>=8 threshold; "2.4x" was dispatch-amortisation + partial NEON;
  cheap-payload regime never built; thermal drift; corrected to crossing ~9 ns + SoA 2.3-4.1x, expose W>=2).
  [correction]
- XV2. Inverted native cell tags (copypatch cell was direct isel; stencil cell was the real copy-and-patch;
  OOPSLA not PLDI; RUN_SUMMARY finding 7 inherited it). [correction]
- XV3. The cost-model k-sweep (the design's own "central measurement") was never built; everything is a single
  warm point at k=16, so setup cost S is hidden and tier breakeven k* is unknowable. [correction; owed
  measurement]
- XV4. vert8 6x -> 4.8x (scalar baseline ran the wire interpreter not the predecoded one, ~20% was decode
  asymmetry); native wins measured against the plain not the best interpreter. [correction]

### Group XVI: uncertains with negative precedent (N-C) [ratify each as "not a definitive no, may return"]
- XVI1. eqsat / DAG-aware extraction (park behind a named trigger; a machine-and-effect-aware objective could
  re-open; the no-alloc slotted-and-colored e-graph remains genuine systems work). [uncertain]
- XVI2. copy-and-patch native (demoted not deleted; feasibility proven; the unmeasured k* breakeven decides).
  [uncertain]
- XVI3. AARA / full potential analysis (deprioritised, re-opened by CR1; live again). [uncertain, live again]
- XVI4. heap / full multi-shot continuations (excluded for CR1's no-alloc bounded form; a genuine backtracking
  need could reopen behind an explicit escape). [uncertain]
- XVI5. tail-threading dispatch (negative for straight-line, positive for CFG terminators; do not read the
  rejection as blanket). [uncertain/scoped]
- XVI6. native-as-a-tier framing (killed; native-as-a-spectrum-point kept). [dead framing, live placement]
- XVI7. a specific record width / operand count (null on the proxy corpus; a real census corpus could re-justify;
  measure-then-lock owed). [uncertain]
- XVI8. single-arena recursive vs double-buffer for the IR-to-IR expansion (explicitly a bench-decided fork;
  single-arena default, double-buffer if it benches better; no precedent yet). [FLAG: this fork CARRIES the
  NodeRef arena-branding decision; decide branding in the same act; XVII7]

### Group XVII: cross-cutting adjudicated opens (the panel's load-bearing deltas; op's call most valuable here)
- XVII1. Terminator = binding-time well-foundedness in three layers (stage well-foundedness / within-stage
  per-operation productivity as signature data / budget-as-diagnostic distinct from cap-as-semantics and from
  passthrough). NOT a fuel cap. [negative fork + SPJ-F6 + Wingo-3]
- XVII2. Native ceiling = a range with its baseline index, reconciled by measurement; canon carries the range
  until then. [SPJ-F2]
- XVII3. The drift test = §1 + §8 jointly (identity AND mechanism); the shadows framing reworded to "carried by".
  [SPJ-F3]
- XVII4. CR1 enters canon carrying its scope (re-execution proven; reinstatement-without-heap + multi-shot-under-
  effects open); the runtime has no continuation model yet; the two sketches do not compose. [Wingo-1,2]
- XVII5. The handler discipline is one type-level unification over three implementation discharges. [Wingo-4]
- XVII6. Macro hygiene + multi-shot-frame freshness = one generative-freshness discipline, bounded by a host-lent
  budget, exhaustion a named diagnostic; add to canon + red tests. [SPJ-F7, Wingo-3]
- XVII7. The single-vs-double arena bench fork carries the NodeRef arena-branding decision; one fork. [SPJ, Wingo]
- XVII8. op's 2026-07-24 live calls need a supersede-forward topic closing the contradiction with 202607241330
  (single-arena default, double-buffer bench fork, benches>topics>changelists). [SPJ-F5]
- XVII9. Consolidation-side corrections (filed as a doc round, not the canon): move eqsat + flat-switch-beats-
  threading + copy-and-patch-privileged from soul to inverse; fix the structural-hash identity boundary; mark +
  resolve the CFG-of-blocks residual gap (with FIXMEs); wire check -> signature. [negative B5, SPJ-F8, Fallin, 6b24]
- XVII10. Process: the second-direction audit (walk every "settled" bullet against source) is owed; the
  FIXME-enumeration audit is blind to unmarked gaps. [Fallin, 6b24]

---

## Part 3: how Phase 4 runs

For each numbered item above, op is asked (via AskUserQuestion, <=4 per call, in inventory order, starting with
Group XVII since op's input is most valuable on the adjudicated opens, then Groups I-XVI): agree as-is / adopt the
flagged form / amend (free text) / cut. Every answer is recorded in Phase 5's paper-trail doc keyed by item
number. Only after every item is answered does Phase 6 write the two canon catalogues from op's ratified answers.
