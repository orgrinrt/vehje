# op's ratification answers (Phase 5 paper trail)

Per op: every item's ratification answer recorded here, keyed by inventory number, verbatim intent, for the
canon (Phase 6) to be written from. Not the forks' homework; op's actual calls. Appended batch by batch as
Phase 4 walks the inventory.

---

## Batch 1 (Group XVII, items 1-4) - answered 2026-07-24

### XVII1 Terminator -> ADOPT AS STATED, with one rule corrected (recursive macros are wanted)
op adopts binding-time well-foundedness + the 3-layer statement AS THE MAIN SHAPE, BUT corrects sub-rule (b):
- **The within-stage rule as I phrased it ("a macro may not re-introduce an operation of its own family at the
  same-or-later binding time") is WRONG: it bans recursive macro expansion, which op explicitly WANTS.**
- Recursive macro expansion is SUPPORTED. Its termination is handled by, in preference order:
  1. **Static cyclicity detection (preferred)** on the Rust typestate, IF it can be proven statically. op favours
     this if feasible; it makes the hang concern moot.
  2. **A recursion DEPTH CAP (fallback, and a sensible always-on safety backstop either way)** so we never hang
     on a recursion loop. Default **255**, **host-configurable** (a host may allow only 2, or 99, or the full
     255, as it needs).
- This depth cap is DISTINCT from the two killed caps: it is a safety backstop against infinite recursion
  (diagnosed as a named error), NOT cap-as-semantics (X3, which residual you get depends on the cap) and NOT the
  six-jobs depth-cap pun (X5). op's cap is a hang-guard, compatible with the well-foundedness semantics.
- **Full diagnostics ALWAYS, everywhere, not just here.** op wants a full diagnostic set: informative errors AND
  actionable suggestions to go with them, with span links that integrate with LSP etc. (This is a new standing
  canonical requirement, broader than the terminator: DIAGNOSTICS ARE A FIRST-CLASS CONVICTION.)
- Open design task this creates: investigate whether macro-expansion cyclicity is statically detectable on the
  Rust typestate (a sketch). If yes -> primary mechanism; the depth cap stays as the always-on safety backstop. If
  no -> the configurable depth cap (default 255) is the mechanism, plus well-foundedness for the stage dimension.

### XVII2 Native range -> ADOPT THE RANGE
Carry ~1.5x to 2.0x with its baseline index + the owed reconciling measurement; native is never the driver.
Clean adoption.

### XVII3 Drift test -> ADOPT DIRECTION; op DELEGATES the exact wording to me, demanding MAXIMAL EXPLICITNESS
op: "this is what everyone in future will read and draw conclusions from, so it can't be ambiguous. I leave the
exact wording and shape to you, since you are the kind of agent that will read this in future; whatever is the
most explicit about what we actually mean there, is the way it should be spelled out too." So: adopt the §1+§8
joint drift-test direction, and in Phase 6 craft the canonical wording to be MAXIMALLY EXPLICIT and unambiguous
about what we mean (identity AND mechanism together), not the analogy shorthand. My call on the precise phrasing,
optimised for a future agent-reader's zero-ambiguity comprehension.

### XVII4 CR1 scope -> NEEDS A BENCH, not a blind decision
op: "This sounds like it needs a bench, not a decision made blindly." So: canon states CR1 WITH ITS SCOPE
(re-execution proven; reinstatement-without-heap + multi-shot-under-effects open) AND flags the continuation
REPRESENTATION as a BENCH-DECIDED FORK (Wingo's open #1: segmented capture-and-reinstate over a host-lent budget
vs compile-time CPS/defunctionalisation of the handled body; measure capture cost, resume cost, ABI shape). Build
the candidates in mock/benches/ and measure; do not settle it in canon by decree. The scope-carrying is adopted;
the resolution of the open half is owed to a bench.

## Batch 2 (Group XVII, items 5-8) - answered 2026-07-24

### XVII5 Handler = 1 type, 3 discharges -> ADOPT (but see XVII6: find the unifying frame)
State the handler discipline as one type-level unification over three implementation discharges (compile-time IR
emit / runtime value-return / runtime resumable continuation); #3 is the open hard one and must not be assumed to
inherit #1/#2's feasibility.

### XVII6 Generative freshness + the handler framing -> ADOPT SUBSTANCE; op wants the UNIFYING framing (delegated to me)
op: "Handles are one discipline, that's not in debate. But whether we frame the three as distinct, or find the
unifying layer within (all of them are impls of Handle dispatch for example, which the system itself has no
opinion on?) or something." So: adopt generative-freshness (hygiene + resume-freshness) as a canonical conviction
+ red tests, AND frame BOTH this and the three-discharges (XVII5) via the UNIFYING LAYER, not as three siloed
distinct things. The frame: Handle is ONE dispatch discipline the SYSTEM ITSELF IS AGNOSTIC ABOUT (has no opinion
on which discharge realizes a handler); the three discharges are impl realizations of that one dispatch, differing
only in operational feasibility (#3 open); generative freshness is a PROPERTY of generative discharge under that
one discipline. Wording delegated to me, maximal explicitness (same mandate as XVII3): surface the unifying layer,
keep the honest note that #3 is unbuilt. My call on precise phrasing.

### XVII7 Arena strategy -> REFRAMED by op: NOT single-vs-double, but N host-configurable buffers, benched
op rejects the binary. Key points:
- Do NOT arbitrarily limit to one or two arenas/buffers/allocation-spaces. Explore N.
- A second (or Nth) host-provided buffer could let us run independent expansion/recursion loops IN PARALLEL (if
  they don't touch each other), or serve as scratchpad / temporary-value storage. (Caveat op raised: this may be
  redundant if the processing DAG already parallelises properly; even then extra buffers are useful for scratch.)
- OUR job now: find HOW several buffers/arenas would work for us, whether they are actually good for perf and
  efficiency, and PINPOINT where multi-buffer wins most. BENCH IT (op's gut: multi-buffer helps, but unsure).
- ONLY AFTER we confirm the wins do we expose host CONFIGURATION (buffer/arena count), and our end just ADAPTS to
  whatever the host provides (if the host gives 255 arenas, maximise utilisation). Do NOT make the count decision
  FOR the host; each host differs.
- The NodeRef branding coupling holds and generalises: if we use N arenas, cross-arena refs need branding; the
  branding decision rides the multi-arena bench. So canon: arena strategy = adapt to N host-provided allocation
  spaces, benched to find where multi-buffer wins, host-configurable once proven, branding decided with it. This
  supersedes the earlier "single-arena recursive default vs double-buffer" binary (which itself superseded the
  two-arena emit). The live-call precedence still: bench decides.

### XVII8 Supersede-forward topic -> DO NOT write a separate topic now
op: "we are already writing the live calls in one place, let's keep that one place as the target (the
op-ratification-answers). We will later synthesise the full two canonical docs + my ratifications and others that
are good to mention or summarise, in a topic file, as a cohesive bundle. So don't worry about that yet." So: THIS
doc (op-ratification-answers.md) is the record of the live calls for now. No separate supersede-forward topic. The
eventual canonical TOPIC BUNDLE (after the two canon catalogues) folds in: the two canonical docs + op's
ratifications + other panel findings worth summarising, as one cohesive canonical topic. SPJ-F5's contradiction
with 202607241330 is resolved there (in the bundle), not by a standalone topic now.

## Batch 3 (Group XVII 9-10, Group I, Group II part 1) - answered 2026-07-24

### XVII9 Consolidation fixes -> ADOPT ALL FOUR
(a) move eqsat-as-keystone + flat-switch-beats-threading + copy-and-patch-privileged from soul to inverse;
(b) fix the structural-hash identity boundary (within-stage structural / cross-stage byte-image);
(c) mark+resolve the CFG-of-blocks residual gap with FIXMEs;
(d) wire check -> vehje-signature. Done as a doc round AFTER the canon.

### XVII10 Audit lesson -> ADOPT (do the audit)
Carry the FIXME-enumeration-is-blind-to-unmarked-gaps lesson + run the second-direction audit (walk every
"settled" bullet against source). Both live wounds are one CL-verification weakness.

### Group I Identity -> I1/I3/I4/I5 FINE; I2 + the one-liner OVER-FOCUS on the type system; I6 needs CONFIRM
- op: "the identity hyperfocuses on the type system alone, should not be the only highlight to mention in the
  one-liner." So the identity proposition (I2) and the one-sentence soul must NOT make the type system the ONLY
  highlight. Broaden: the census/consumer-serving, the two-artifact certified-generation model, the handler
  discipline, the bench-driven discipline, no-heap, the output spectrum are all co-equal identity highlights. The
  one-liner needs rebalancing (delegated to me, max-explicit per XVII3).
- I6 (metacompiler with a unified graded proof, native output point per language by whatever method benches best,
  load verifier bounded by lease-lattice height): op wants to CONFIRM this is still the intended shape. Flag for
  explicit re-confirmation (I will re-verify against current state + surface it).
- I1 (framework not a language), I3 (census served equally), I4 (spectrum), I5 (two artifacts, Rust in one):
  ratified.

### Group II part 1 (convictions II1-II7) -> DO NOT RUBBER-STAMP; RE-VERIFY + AUGMENT (META-INSTRUCTION)
op: "All these should be checked whether they are still correct statements about vehje, after all the benches and
research (could be, just a double-check). A lot of these sound kind of old, like very old things we wrote down
when we didn't have any benches, and had barely any concrete settled about the design. They might still hold, but
worth just confirming. Might also be this needs a lot more to go with it, since we couldn't have predicted very
well where we'd end up (here) back then."
IMPLICATION (applies to ALL the soul/conviction/identity/ethos groups, not just II1-7): the canon convictions are
NOT ratified as-written. I must CRITICALLY RE-VERIFY each against the current settled state (the benches, the
research, the panel's converged conclusions), CONFIRM what still holds, UPDATE what is stale, and AUGMENT where
the evolved design outgrew the old pre-bench statement. The old candidates capture the early intent; the canon
must be the CURRENT, COMPLETE synthesis. This is the real canon-writing work (Phase 6's "from scratch,
synthesised") pulled forward: I do the re-derivation, then present op the DELTAS (holds / updated / added) for
confirmation, rather than asking op to bless stale text. Next action: re-verification pass over the soul, written
as analysis, deltas surfaced to op.

## Batch 4 (identity one-liner, I6, convictions re-verified) - answered 2026-07-24

### One-liner rebalance -> op wants a REAL SEMANTIC DISTILLATION, not a list
op: "a lot of those are just restating the soul entries too. There's a real semantical way to rephrase the whole
thing as a one-liner, instead of listing things written elsewhere already." So the one-liner must be a genuine
synthesis/distillation (a single semantic statement of what vehje IS), NOT an enumeration of the pillars/soul
entries. My call on the phrasing (max-explicit per XVII3), but it must READ as one coherent idea, not a list.

### I6 metacompiler shape -> HOLDS as a starting point, but iterate + de-pretentious-ify + make it address everything
op: "Might hold, but still, doesn't feel like it addresses everything, and also, perhaps words things a bit too
pretentiously. I think this needs some iterating on, but I guess it's a starting point." So I6 is provisional:
iterate it, cut the pretentious wording, and make it cover the full identity (not just the metacompiler framing).
Fold into the one-liner/identity rework.

### II1-II7 re-verify -> augmentations accepted IN GENERAL, but VERIFY every shipped-state claim against source
op: "Par the statements about 'is now implemented' or 'as shipped', in general, accurate. Needs confirming any and
all the details this claims though, that I'm not sure about." So before the canon asserts II4-is-shipped or
II3-is-inert or any concrete state claim, I must VERIFY each against actual source (the sealed Checked witness
exists; check sets binding empty; etc.). No shipped-state claim goes into canon unverified. (cl-claim discipline.)

### II8-II14 + new convictions -> CONFIRM ALL
II9 expands to N-allocation-spaces; II10 carries CR1 scope; II12 adds the identity boundary + emitter-seam +
inert-signature; NEW: N1 diagnostics-first-class, N2 generative-freshness, N3 N-buffer resource adaptivity.

### MID-TURN STEER (op, 2026-07-24): RE-FOCUS on the frontier, do not foreground the old identity boilerplate
op: "I feel we might be dropping all the genuinely exciting, tricky, even novel, things we learnt in the past few
days (and the one long ass canonical design round before), to background, and highlighting the old identity
statements and such. We really need to get some re-focus or at least confirmation that we aren't highlighting
only a small subset, or the wrong things even."
IMPLICATION: the canon must FOREGROUND the genuinely novel/hard-won/exciting discoveries (the binding-time
lattice + Futamura-staging answer, the one handler discipline, CR1 bounded no-alloc multi-shot, the graded
proof spine, certified-generation-data-not-source, one-signature-many-projections, the IR-to-IR staged
expansion, what the benches actually taught + the bench-honesty meta-lesson, the maximal-shape/unify-by-
construction ethos, and op's fresh N-buffer + diagnostics convictions). The old foundational identity statements
(framework-not-a-language, type-system-verification-layer, no-null, no-heap) are the BASE those rest on, not the
highlight. Next action: surface the full frontier set, confirm with op it is the right foreground + nothing
missing/mis-weighted, THEN structure the canon to lead with it.

## Batch 5 (frontier re-focus + centre of gravity) - answered 2026-07-24

### Frontier set A-J -> CONFIRMED as the right foreground; + a fresh EXPERT AUDIT of all ratifications is owed
op: "This sounds good to me, but worth having an expert later audit all we've decided and ratified here. It
sounds good to me, but I'm not in the thick of it, unlike you or any expert poking around." So: frontier A-J is
the confirmed foreground (old identity statements = the base). NEW PLAN STEP: after ratification (and the canon
draft), dispatch a fresh expert to audit everything decided + ratified here, since op reviewed from outside the
detail and wants an in-the-weeds expert check.

### Centre of gravity -> a FUSION, keep iterating
op: "A fusion, keep iterating." The one-liner/centre is a fusion (the binding-time-directed handler discipline +
certified-generation-data-not-source + the graded proof spine, at least), not a single pillar. Keep iterating the
distillation; do not settle a single-pillar centre. My call on the phrasing, iterated, max-explicit, de-
pretentious (per the one-liner + I6 answers).

The frontier A-J (verbatim for the canon's foreground):
A. compile/runtime line = the four-point binding-time lattice; const-inline/macro-expand = partial evaluation =
   the first Futamura projection, staged (AOT at LanguageAuthor+Bundler, load-time compile at HostLoader,
   interpretation a Runtime form; fully-interpreted coherent per-stage, incoherent across the pipeline).
B. one handler discipline (effects + host-calls + macros; discharge at earliest binding time; twelfth Core form
   Handle; system agnostic over three discharges).
C. CR1 no-alloc bounded multi-shot continuations via host-lent budget, carried with scope (re-execution proven;
   reinstatement + under-effects open; bench owed).
D. the graded (co)modal proof spine, unified by construction into one soundness theorem; reachability types the
   primary lease.
E. certified generation, doubly-certified, data-not-source; Futamura-as-build-step; certifying not certified;
   assurance a dial.
F. one signature, many projections; differential-checked; no trusted emitter (trust seam is the emitter, closed
   by the lens).
G. the IR-to-IR staged expansion (single-arena recursive or N-buffer, benched; recursive macros; binding-time
   well-foundedness + a configurable recursion-depth safety cap).
H. what the benches taught (vertical-SIMD batched-column ABI = biggest lever ~4.8x; cheap reducer fold+CSE+DCE
   not the e-graph; native never the driver; tier-split dispatch; the bench-honesty meta-lesson).
I. the maximal-shape / unify-by-construction ethos (design through the hard parts; compose until a new shape
   supersedes; the standard answer is a signpost to the missed recombination; reject re-tiering).
J. op's fresh 07-24 convictions: N-buffer host-lent resource adaptivity; diagnostics first-class (informative +
   actionable + LSP-linked, everywhere).

## Batch 6 (ethos, op-calls, benches, prior art) - answered 2026-07-24
- Group III ethos + Group IX spirit signals -> CONFIRM (the maximal-shape / unify-by-construction ethos, verbatim
  the frontier-I foreground).
- Group IV op's standing calls (IV1-IV8) -> CONFIRM.
- Group V what the benches taught (V1-V10, the frontier-H current evidence) -> CONFIRM.
- Group VI prior art by commitment (with SPJ-F4 flags: tnum scoped to numeric residual; register VM 1.41-3.06x
  per profile; NaN-boxing bench-leaning low-to-medium confidence; eqsat moved to negative-precedent; tail-threading
  scoped) -> CONFIRM.

## Batch 7 (surviving, kill catalogue, bench kills, uncertains) - answered 2026-07-24
- Group VII surviving + VIII direction -> CONFIRM, with a KEY DISTINCTION on VII3: the consumer census means we
  EXPECT and MATCH the consumers' needs, NOT that we design vehje FOR them specifically. "It should only mean
  that we should expect their needs and match those, not that we design vehje for them specifically. Important
  distinction." So: the census is a NEEDS-SIGNAL that anticipates what must be served; the DESIGN is driven by
  benches + prior-art + the maximal shape, never tailored to a specific consumer. VII3 must not conflict with the
  bench-and-prior-art-driven choices. (Reconciles the census-grounding with the identity that vehje is not any
  one consumer's tool.)
- Groups X-XIII the kill catalogue -> CONFIRM ALL DEAD.
- Groups XIV-XV bench-supremacy kills + bench-integrity corrections -> CONFIRM, with a NEW CANONICAL PRINCIPLE op
  wants added: **benches are PROVISIONAL and LIMITED.** "Don't blindly trust done benches or their claims, but
  trust the data itself for what it is. The benches might be limited and we shouldn't extrapolate any further
  proofs from them than they truly do prove. These are in flux always, since we can always do better, more
  complex, more representative, better designed benches in future, and that we should strive to do too." So the
  bench discipline (frontier H / conviction) gains: trust the DATA for exactly what it measured, no
  extrapolation beyond what it proves; current benches are rudimentary; keep building better/more-representative
  benches. Epistemic humility as a standing principle, not just "benches decide."
- Group XVI uncertains-with-negative-precedent -> CONFIRM.

## Batch 8 (post-expert-audit calls) - answered 2026-07-24 (vacation directive)

After the two fresh expert audits op ordered (Lattner infra-lens + the Torvalds systems double-take), the "count our chickens take 2" review surfaced a consolidated set of open decisions. op answered them in two AskUserQuestion rounds. Recorded verbatim-intent for the canon.

### PE1 vehje-schedule -> DISSOLVE (verified no use)
op: "it is more of a runtime concern, not really something vehje needs rust side, so yeah, I guess it's redundant? We can add it back later if we need it for something. Unless we already have a use for it, just not implemented?" Verified: no `vehje_schedule::` use anywhere in source, the canon never mentions the crate, only `vehje-runtime-driver/Cargo.toml` declares an unused dep. So there is no designed-or-implemented use. DECISION: dissolve the crate. Delete `vehje-schedule`, hardcode the 4-pass compile sequence (resolve/check/lower/emit) in `vehje::run`, drop the unused dep. Pass scheduling, if ever needed, is a RUNTIME concern (Zig/engine side), re-addable later. This is an IMPL round after the canon.

### PE2 CR1 sequencing -> RUN THE MULTI-SHOT REPRESENTATION BENCH-FORK NOW
op chose "run the bench-fork now" over one-shot-first. So: build the XVII4 candidates (segmented capture-and-reinstate over a host-lent budget vs compile-time CPS/defunctionalisation of the handled body) under `mock/benches/` now, measure capture/resume cost + ABI shape, and settle the continuation representation. The multi-shot novelty claim stays on schedule; canon carries CR1 with the representation as an actively-being-benched fork (not a deferred one). The scope note (re-execution proven; reinstatement-without-heap + multi-shot-under-effects were the open halves) still frames it until the bench lands.

### PE3 compile-side no-alloc -> AXIOM + MANDATE MITIGATIONS
Keep no-alloc on BOTH sides as a stack-wide axiom (per the no-heap thesis and vehje soul 2.9); no dev-side allocator dispensation. The canon names the costs and MANDATES the fixes: `must_use` / size-refusing constructors for grade regions so the illegal state (a region smaller than the arena) cannot be built; defunctionalised (non-recursive) compile passes. This directly fixes the two source bugs Torvalds found (the discarded `GradeTable::set` failure at `typecheck:314`; recursion-on-IR-depth in `infer`/`structurally_equal`). Both land as catalogued red tests then fixes in the IMPL rounds.

### PE4 spirit-over-letter precedence -> NARROW THE CLAUSE
Spirit-over-letter governs reading STALE INTERMEDIATE artifacts only (its original purpose: a stale early line does not override the evolved intent). The canon's LETTER governs and is amendable solely by a superseding round; the drift test (canon Section 10) is the letter. This removes the 20-year relitigation hazard Torvalds named and aligns with the workspace's `canonical-design-outranks-intermediate-rounds` rule. The canon preamble is reworded accordingly.

### PE5 first consumer / sequencing -> NOT "FASTEST", "MOST IDEAL AND OPTIMAL"
op rejected the framing: "we aren't about 'fastest', we are about 'most ideal and optimal'." So the experts' consumer-first recommendation (build mockspace-docs early to drive/decide the §11 wiring, because it is the fastest resolver) is declined as a speed optimisation. The §11 wiring is done the ideal, design-first, maximal-shape way (the reject-re-tiering ethos: do not take the locally-cheaper consumer-driven shortcut). mockspace-docs enters as the first consumer when vehje is genuinely ready, per the standing goal. Bench-decided forks still resolve by bench (PE2), not by rushing a consumer to drive them.

### PE6 centre-of-gravity distillation -> HYBRID OF ALL THREE CANDIDATES
op: "a hybrid of all of these, they are all good actually." The canon's central one-line distillation fuses all three offered directions (discharge/prove/erase/ship-data; the surviving one-sentence soul; language-definition-becomes-certified-data) into one coherent, de-pretentious semantic statement. My call on the exact wording (per the standing delegation), synthesising the three. The I6 metacompiler paragraph is iterated + de-pretentiousified to match and to address the full identity, not just the type system.

### Agent's own calls (low-stakes, recorded not asked)
- Banner the two `canonical_candidate_` docs as superseded-by-`202607241545` (one line each); they remain readable panel artifacts but a reader is pointed at the canon so the pre-correction defects are not re-inherited.
- Extend the assurance verb ladder with the measured-artifact distinction: "measured (prototype)" vs "measured (shipped crate)", so a standalone-Zig-prototype number never certifies a shipped-crate claim unqualified (the 8M/50ms fixpoint figure is prototype-measured).
- Adopt "every canon-grade claim carries a catalogued red test or bench cell at authoring" as the standing discipline (applying `catalogue-edge-cases-as-tests` harder); catalogue the two new bugs and the second-direction audit's 23 rows as red tests during the IMPL rounds (the workspace had exactly one catalogued red test against 23 rows).
- Record as owed design work (roadmap, not blocking the canon): the embedder-author API is the least-designed surface and needs author-clearable scaffolding before the census can broaden past first-party; diagnostics-first-class (N1) needs its own host-lent budget discipline on the no-alloc substrate; the runtime C ABI is the one interface that outlives every mechanism, so its versioning + refusal-of-a-newer-residual compatibility policy is design work owed before the first external consumer, not after.

## STATUS: ratification COMPLETE. Every inventory group (I-XVII) ratified with op's nuances.
Net new / augmented beyond the candidates, to bake into the canon:
- Recursive macros SUPPORTED; termination = binding-time well-foundedness (stage) + static cyclicity detection
  if feasible ELSE a host-configurable recursion-depth cap (default 255) as fallback + always-on safety backstop;
  full diagnostics always (informative + actionable + LSP-linked). [XVII1]
- Native ceiling = a range with baseline index, reconciled by measurement. [XVII2]
- Drift test = identity + mechanism jointly; one-liner is a real fusion distillation, not a list; de-pretentious.
  [XVII3, batch4]
- CR1 carries its scope + the continuation representation is a bench-decided fork. [XVII4]
- Handler discipline = one dispatch the system is agnostic about, three discharges (#3 open); find the unifying
  frame, not three silos. [XVII5/6]
- Generative freshness (hygiene + resume-freshness) as one bounded-diagnosed conviction + red tests. [XVII6]
- Arena strategy = adapt to N host-provided allocation spaces, benched to find where multi-buffer wins,
  host-configurable once proven, NodeRef branding decided with it. [XVII7]
- DIAGNOSTICS FIRST-CLASS as a standing conviction. [XVII1/N1]
- BENCHES ARE PROVISIONAL: trust the data for what it proves, no over-extrapolation, keep improving them. [batch7]
- Consumer census = expect+match needs, NOT design-for-them. [VII3]
- Identity one-liner rebalanced across pillars, type-system among not above; the frontier A-J is the foreground,
  old identity statements the base.
- Consolidation-side doc-round corrections (eqsat/switch/copy-patch soul->inverse; hash identity boundary; CFG
  FIXMEs; check->signature wiring) + the second-direction audit, both AFTER the canon.
- NEW PLAN STEP: a fresh expert audits all decided+ratified here (op's ask), after the canon draft.
Before asserting any shipped-state claim (II4 implemented, II3 inert), VERIFY against source (cl-claim discipline).
