# Theory citation verification (second pass) and adjacent threads to bank

**Date:** 2026-07-20
**Phase:** research continuation (worker-fork deliverable, reports once)
**Scope:** independently re-verify the theory citations carried by the certified-generation arc, chiefly
`202607201641_certified-generation-panel/08_theory_insights.md` and the claims the outcome topic
`202607201845_topic.certified-generation-panel-outcome.md` rests on it, that each source exists, is
reachable, and actually justifies what the doc attaches to it. Then pull the adjacent threads those checks
surface, to bank more relevant theory that could sharpen or strengthen the design.
**What this is not:** it does not reopen the resolution, re-rule the seven calls, or re-derive the frame. It
is a sanity audit of the paper trail plus a survey extension.

## Verdict

Every theory citation in `08_theory_insights.md` checks out. All sixteen reference clusters name real,
reachable papers, attributed to the right authors and venues, and each one supports the specific claim the
document hangs on it. The load-bearing recent citations that were the most falsifiable, "Staged Gradual
Typing" (GPCE 2025) and "Rule-based program specialization to optimize gradually typed code" (KBS 2019),
both exist exactly as cited and say what 08 says they say. The single most load-bearing theoretical move in
the whole arc, the proposal to derive the lease link/consume bit from the operand's declared effect because
"region and effect are one system" (08's final section, and the refinement to call 1 in the outcome topic),
is soundly grounded: Talpin and Jouvelot's type-region-effect unification is real, and two further papers
found in this pass (Marino-Millstein's generic type-and-effect system, and the journal Gradual
Type-and-Effect Systems) strengthen it beyond what 08 cited.

Three attribution-precision notes and one number-provenance caution are worth folding when the proof
document is written. Six adjacent threads are worth banking, one of which (reachability types) is strong
enough to change how the lease axis should be grounded, and one of which (mechanised semantics-preserving
let-insertion, 2025) hands theorem T1 a concrete, machine-checked, effect-aware template that is newer and
sharper than the MetaML citation 08 used.

## Verification ledger

Each row is a claim as stated in `08_theory_insights.md`, the source it cites, whether the source exists and
is reachable, and whether it justifies the claim. "Justifies" means the paper actually establishes the
property the doc uses it for, not merely that a paper of that title exists.

| Claim in 08 | Cited source | Exists / reachable | Justifies the claim |
|---|---|---|---|
| static-where-provable, dynamic cast otherwise, over refinement (subset) types | Flanagan, Hybrid Type Checking, POPL 2006; Knowles-Flanagan, TOPLAS 2009 | yes (soe.ucsc.edu popl06-hybrid.pdf, toplas09.pdf) | yes: λH has refinement types and a cast `⟨T◁S⟩` that statically checks what it can and defers the rest to a run-time cast |
| gradual verification soundly backs partial/missing specs with runtime checking | Bader-Aldrich-Tanter, Gradual Program Verification, VMCAI 2018 | yes (cs.cmu.edu vmcai2018-gradual-verification.pdf) | yes, and stronger than cited: it obtains the dynamic checks of Hoare contracts through the AGT framework, so the "gradual verification composes with AGT" claim is the paper's own construction |
| a gradual system is derived from its static one via a Galois connection, gradual guarantee for free | Garcia-Clark-Tanter, Abstracting Gradual Typing, POPL 2016 | yes (pleiad.cl agt.pdf) | yes: gradual types as sets of static types, static+dynamic semantics derived to satisfy Siek et al's criteria by construction |
| the gradual guarantee stated as criteria | Siek-Vitousek-Cimini-Boyland, Refined Criteria for Gradual Typing, SNAPL 2015 | yes (LIPIcs SNAPL 2015, vol 32, 274-293) | yes: this is the paper that states the gradual guarantee as a refined criterion |
| gradual effects by abstract interpretation over an effect lattice | Bañados Schwerter-Garcia-Tanter, A Theory of Gradual Effect Systems, ICFP 2014 | yes (pleiad.cl banadosAl-icfp2014.pdf) | yes, with nuance (see precision note P2): the 2014 paper predates AGT; the abstract-interpretation-over-a-lattice derivation is fully realised in the journal successor found this pass |
| gradual typing combined with multi-stage run-time code generation (nearest prior art) | "Staged Gradual Typing", GPCE 2025 | yes (Yaguchi-Kameyama et al, GPCE 2025, 94-104) | yes: it is exactly gradual typing plus multi-stage; it is the nearest published point to vehje's staged-plus-gradual shape |
| specialising gradually-typed code removes runtime checks where static info exists (the comptime fold) | Rule-based program specialization to optimize gradually typed code, KBS 2019 | yes (ScienceDirect S0950705119302199; open postprint at digibuo.uniovi.es) | yes: static specialization reduces runtime type checks, consumes no extra runtime memory, gives static safety, which is the comptime-fold analogue |
| the projections and the mix equation `[[E]](L,s) = [[mix(E,L)]](s)` | Futamura 1971; Jones-Gomard-Sestoft 1993 | yes (canonical; Jones-Gomard-Sestoft book online) | yes: the first-projection / mix-equation form is standard and correctly stated |
| the early/late division is itself an abstract interpretation | Consel-Danvy POPL 1993; Palsberg, BTA: Abstract Interpretation vs Type Inference, ICCL 1994 | yes (web.cs.ucla.edu iccl94.pdf) | yes, with attribution fix P1: the ICCL 1994 paper is Palsberg AND Schwartzbach |
| annotation soundness / the erasure property behind "one kernel, two binding times agree" | Taha-Sheard, Multi-Stage Programming with Explicit Annotations, PEPM 1997 | yes (dl.acm 259019) | partly, and better carried elsewhere (P3): MetaML's type system guarantees well-typed programs are correctly staged; the precise semantics-preservation-under-let-insertion result is a 2025 paper (bank B2) that is a cleaner T1 anchor |
| region calculus, LIFO/stack, type-and-effect based, with a soundness proof | Tofte-Talpin, Region-Based Memory Management, I&C 132(2), 1997 | yes (ropas.snu.ac.kr ToTa1997.pdf) | yes: stack of regions, LIFO destroy order, type-and-effect inference, detailed soundness proof |
| syntactic soundness of the region calculus in Wright-Felleisen progress/preservation style | Calcagno-Helsen-Thiemann, I&C 173(2), 2002; Wright-Felleisen, I&C 115(1), 1994 | yes (I&C vol 173, 199-221; cyberleninka copy) | yes: the CHT paper is exactly syntactic type soundness for the region calculus |
| region and effect are one system (basis for deriving the lease bit from the effect) | Talpin-Jouvelot, The Type and Effect Discipline, LICS 1992 / I&C 1994 | yes (semanticscholar) | yes, and the region+effect unification is most explicit in the companion "Polymorphic Type, Region and Effect Inference," JFP 1992 (add per P4) |
| monotone dataflow to a least fixpoint; widening as safe over-approximation | Cousot-Cousot POPL 1977; Kildall POPL 1973 | yes (canonical) | yes: abstract interpretation and the unified dataflow framework; correctly used for the lease inference and promotion-as-widening |
| the bridge is translation validation, validated-not-verified, with PCC re-checking for decidable axes | Pnueli-Siegel-Singerman TACAS 1998; Necula PLDI 2000; Necula PCC POPL 1997 | yes (cornell/washington Necula PCC copies) | yes: PCC's producer-generates / consumer-checks split is precisely the "re-prove after decode" upgrade for the decidable axes |
| folds over an initial algebra are total exactly when every constructor is handled (dispatch totality) | Meijer-Fokkinga-Paterson, Bananas Lenses Envelopes Barbed Wire, FPCA 1991 | yes (springer 3540543961_7) | yes: catamorphism as fold; totality over the coproduct is the universal-property reading |
| what semantic verification costs, why "certified" must scope to structural properties | Leroy, CompCert, CACM 2009; Kumar-Myreen-Norrish-Owens, CakeML, POPL 2014 | yes (xavierleroy.org; cakeml.org icfp16.pdf) | yes qualitatively; see the number-provenance caution below |

## Precision notes (fold when writing the proof document)

- **P1. Palsberg BTA is Palsberg and Schwartzbach.** The ICCL 1994 paper "Binding-time analysis: abstract
  interpretation versus type inference" is co-authored with Michael I. Schwartzbach. 08 lists only Palsberg.
  Separately, the most directly on-point BTA-soundness result for citing "the early/late split is sound" is
  Palsberg's "Correctness of Binding-Time Analysis" (JFP), which is a cleaner anchor for that exact claim
  than the abstract-interpretation-versus-type-inference comparison paper.
- **P2. The gradual-effects abstract-interpretation derivation is the journal paper.** The ICFP 2014 "A
  Theory of Gradual Effect Systems" predates AGT (2016). The systematic derivation over an effect lattice
  that 08 leans on is fully in the journal successor, "Gradual Type-and-Effect Systems," JFP vol 26, 2016
  (19:1-69), which extends Marino-Millstein's generic type-and-effect framework with unknown effects. Cite
  the JFP paper for the derivation and keep ICFP 2014 as the origin.
- **P3. MetaML is not the tightest anchor for T1's erasure property.** 08's "one kernel, two binding times
  agree" is carried better by the 2025 mechanised let-insertion result (bank B2) than by MetaML 1997. Keep
  Taha-Sheard as the origin of typed staging; anchor the actual theorem on the newer, effect-aware,
  machine-checked semantics-preservation work.
- **P4. Anchor the lease-from-effect move on the region+effect inference paper explicitly.** The load-bearing
  novel move needs its strongest citation. That is Talpin-Jouvelot "Polymorphic Type, Region and Effect
  Inference" (JFP 1992), which unifies type, region, and effect inference in one algorithm, plus
  Marino-Millstein (bank B3) for "an effect is a required privilege," plus the gradual type-and-effect JFP
  2016 for the gradual version. With those three the move stops being an appeal to a slogan and becomes a
  cited chain.

**Number-provenance caution (not a doc error, a guard for the proof document).** A general search for
CompCert/CakeML verification cost surfaces a "$350 per source line" figure. That number is the seL4
microkernel verification cost, not CompCert's, and the two are different kinds of artifact. 08 wisely cites
no number; if the proof document ever wants a cost figure to justify scoping "certified" to structural
properties, express it as person-years of proof effort for CompCert/CakeML, and do not import the seL4
per-SLOC number.

## Adjacent threads to bank

These are the threads the verification pass surfaced that are genuinely relevant to vehje and not yet in the
paper trail. Each is stated with what it is, why it bears on the design, and what it strengthens or changes.

### B1. Reachability types: the modern region-plus-effect-plus-aliasing successor, and the right theory for the lease axis

Reachability types (Bao, Wei, Bračevac, Jiang, He, Rompf, "Reachability Types: Tracking Aliasing and
Separation in Higher-Order Functional Programs," OOPSLA 2021) track, per value, the set of variables it can
reach, which is exactly "does this reference escape its scope, and if so through what." The lineage is
active and directly on vehje's axis: polymorphic reachability types (OOPSLA 2024), "Free to Move:
Reachability Types with Flow-Sensitive Effects" (2025), and "Escape with Your Self: Sound and Expressive
Bidirectional Typing with Avoidance for Reachability Types" (PACMPL 2025).

Why it matters more than the 1997 region calculus for this design:

- vehje's lease axis is stated as "prove every reference in a produced value is valid as long as it is used,
  by inferring a per-body region and widening when a reference escapes." That is the reachability/escape
  question in its exact modern form. Reachability types are built for higher-order functional programs with
  escaping closures, which is what a real consumer language produces; Tofte-Talpin was built for a skeletal
  ML and is the older, blunter tool.
- The "avoidance problem" that the 2025 bidirectional-typing paper solves by name IS vehje's
  lease-inference-failure moment: a value escapes a scope, and the analysis must express its type without the
  escaped binder, or fail. Topic 1316 decided that failure is a hard compile error forcing an explicit lease.
  The reachability-types avoidance work is the theory of exactly when that failure is necessary versus when a
  wider lease can be inferred, which is the ergonomics question the Carmack audit (debt 2) flagged as "the
  whole game."
- It addresses debt 2's sharpest objection head-on. Debt 2 noted the immutable-value narrowing that makes
  the lease analysis cheap does not hold for `vehje-lua`'s mutable aliased tables, and Lua is in the census.
  Reachability types track aliasing and separation for exactly the mutable-and-aliased case, so they are the
  principled fallback for the mutable consumer rather than dropping to a collector.

Recommendation to bank: ground the lease axis on the reachability-types lineage (with Tofte-Talpin and CHT
as the LIFO-fragment metatheorem for the immutable core, unchanged), and use the avoidance results as the
theory of the inference-failure boundary. This does not reopen the resolution; it upgrades the citation and
gives the mutable-consumer fallback a name.

### B2. Mechanised, effect-aware, semantics-preserving let-insertion: a concrete template for theorem T1

Two 2025 results give theorem T1 (staging soundness / the mix equation) a machine-checked template stronger
than MetaML:

- "When Do Staging Annotations Preserve Semantics? Mechanizing Typed Semantics-Preserving Multi-Stage
  Programming with Let-Insertion" (arXiv 2606.30854, 2025). It restricts Amin-Rompf's untyped multi-stage
  λ↑↓ (the calculus that models LMS-style automatic let-insertion) to a two-stage typed setting, adds side
  effects, and proves semantics preservation.
- "Mechanised Semantics of Multi-stage Programming" (PACMPL 2025, doi 3798260): a Rocq mechanisation of a
  core calculus for compile-time and run-time MSP with effects, establishing type soundness, elaboration
  soundness, and phase distinction.

Why it matters:

- T1 as 08 stated it ("specialising λ_veh to its early-bound input agrees with running the two-level term
  directly") is precisely a staging-semantics-preservation theorem, and these papers prove that class of
  theorem, mechanised, with effects, in the two-stage setting vehje uses. They are a better proof-document
  anchor than MetaML 1997 (P3).
- Let-insertion is not incidental. It is the mechanism that makes staged code with effects sound, and it is
  what the streaming spine plus macro expander needs when a build-environment reduction has to hoist a bound
  computation out of a subterm. The synth prior-art already carries LMS (`synth_staged-metaprogramming`);
  λ↑↓ is LMS's semantic core, so this closes the loop between the prior art already banked and the T1 target.

Recommendation to bank: make the mechanised λ↑↓ let-insertion result the citation for T1 and for the claim
that macro expansion over effects is sound, and note that a Rocq template exists if T1 is ever to be
machine-checked rather than paper-proved.

### B3. Marino-Millstein generic type-and-effect: the exact frame for vehje's `Permits` axis

"A Generic Type-and-Effect System" (Marino, Millstein, TLDI 2009) interprets an effect system as privilege
checking: each effectful operation requires a privilege, and a type carries the privilege set it needs. That
is vehje's effect axis verbatim: a residual's effect set must be included in the target's `Permits` set. The
framework is parameterised over the privilege vocabulary, which is exactly vehje's "families and effects are
consumer-declared `AccessSet`s" shape.

Why it matters: it gives the effect axis a single named home the way 08 gave the whole design a single
frame. The chain becomes: the effect axis is Marino-Millstein privilege checking (2009); its gradual
counterpart, needed for the runtime-arriving-script case, is Gradual Type-and-Effect Systems (JFP 2016); and
the two together are what let 08 collapse the lease link/consume bit into the effect table (P4). Bank all
three as the effect axis's citation chain.

### B4. Space-efficient / evidence-based gradual typing: narrows novelty item 3 honestly

08's novelty item 3 claims the gradual-typing field has not analysed its dynamic residuals for no-alloc,
single-pass, bounded-memory realizability. That is too strong as stated. Space-efficient gradual typing is a
studied line: Herman-Tomb-Flanagan space-efficient coercions, and "Abstracting Gradual Typing Moving
Forward: Precise and Space-Efficient" (PACMPL 2021), plus the recent "Compiling Gradual Types with Evidence"
(arXiv 2512.22684). The field has bounded the space of gradual casts.

Why it matters: it is both a correction and a gift. The honest positioning of novelty item 3 is narrower:
gradual casts have been made space-efficient, but tying the dynamic-residual memory bound to the region
calculus's LIFO depth (proving the no-alloc frontier bound equals the lattice height, which equals the depth
cap) is the specific unpackaged result. And the space-efficient-coercion techniques are borrowable
machinery for the bounded-frontier dynamic check rather than something to invent. Bank as: soften novelty 3
to the region-depth-bound tie, and cite the space-efficient-coercion line as prior art the streaming-spine
dynamic residual can reuse.

### B5. Region inference does leak, and the sound hybrids that fix it, confirming debt 2 and naming its fallbacks

The debt-2 cautionary claim (pure region inference leaked; MLKit needed a collector; Cyclone shipped a
hybrid) checks out. MLKit exhibited space leaks in the global region with measured memory 2x to 150x the
regions-plus-GC baseline (Hallenberg, Elsman), and Cyclone added unique pointers and reference counting on
top of lexical regions and a GC (Grossman, Morrisett, et al). The modern sound hybrids are named: Elsman's
"Combining Region Inference and Generational Garbage Collection," and "Reference Capabilities for Flexible
Memory Management" (2023).

Why it matters: it validates that the lease axis's compile-error-only stance is a real design commitment
with a known failure history, and it names the two escape valves debt 2 asked vehje to choose between,
should the pure-static lease prove too strict for a mutable consumer. Combined with B1, the recommendation
is coherent: immutable core stays pure-static (Tofte-Talpin LIFO fragment, CHT soundness); the mutable/
escaping fallback is reachability types or a reference-capability discipline, not a collector.

### B6. Further grounding (lighter bank)

"Type, Ability, and Effect Systems: Perspectives on Purity, Semantics, and Expressiveness" (2025) is a
recent survey that situates effect systems, capabilities, and region/reachability tracking in one frame; it
is a convenient one-stop grounding when the proof document introduces the effect and lease axes together.
Glück's "Is There a Fourth Futamura Projection?" and Williams et al's "Revisiting the Futamura Projections:
A Diagrammatic Approach" are clean modern restatements to cite for the projections if the classic
Jones-Gomard-Sestoft reference wants a companion.

## What this changes, and what it does not

Nothing here reopens the certified-generation resolution or the seven calls. The paper trail's theory is
sound. What the pass adds is: four small citation upgrades to fold when the proof document is drafted (P1-P4);
one honesty narrowing of a novelty claim (B4); and two banks that are strong enough to shape the lease axis
and the T1 proof rather than merely decorate them (B1 reachability types as the lease axis's modern theory
and the name for its inference-failure boundary; B2 mechanised effectful let-insertion as T1's template).
The single sharpest outcome: the design's weakest load-bearing axis in the audit, the lease axis, has a
directly-applicable, actively-developed body of theory (reachability types and avoidance) that was not in
the trail and that answers debt 2's mutable-consumer objection without any of the escape valves debt 2
feared.

## References found or confirmed this pass (with reachable locations)

- Flanagan, Hybrid Type Checking, POPL 2006. https://users.soe.ucsc.edu/~cormac/papers/popl06-hybrid.pdf ;
  Knowles-Flanagan, TOPLAS 2009. https://users.soe.ucsc.edu/~cormac/papers/toplas09.pdf
- Bader, Aldrich, Tanter, Gradual Program Verification, VMCAI 2018.
  http://www.cs.cmu.edu/~aldrich/papers/vmcai2018-gradual-verification.pdf
- Garcia, Clark, Tanter, Abstracting Gradual Typing, POPL 2016. https://pleiad.cl/papers/2016/garciaAl-popl2016.pdf
- Siek, Vitousek, Cimini, Boyland, Refined Criteria for Gradual Typing, SNAPL 2015.
  https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SNAPL.2015.274
- Bañados Schwerter, Garcia, Tanter, A Theory of Gradual Effect Systems, ICFP 2014.
  https://pleiad.cl/papers/2014/banadosAl-icfp2014.pdf ; Gradual Type-and-Effect Systems, JFP 26, 2016.
  https://www.cs.ubc.ca/~rxg/gtes.pdf
- Yaguchi, Kameyama et al, Staged Gradual Typing, GPCE 2025 (proceedings pp 94-104).
- Rule-based program specialization to optimize gradually typed code, KBS 2019.
  https://www.sciencedirect.com/science/article/abs/pii/S0950705119302199 (open postprint:
  https://digibuo.uniovi.es/dspace/bitstream/handle/10651/53505/postprint-kbs.pdf)
- Palsberg and Schwartzbach, Binding-Time Analysis: Abstract Interpretation versus Type Inference, ICCL 1994.
  http://web.cs.ucla.edu/~palsberg/paper/iccl94.pdf
- Taha, Sheard, Multi-Stage Programming with Explicit Annotations, PEPM 1997. https://dl.acm.org/doi/10.1145/258993.259019
- Tofte, Talpin, Region-Based Memory Management, I&C 132(2), 1997. http://ropas.snu.ac.kr/lib/dock/ToTa1997.pdf
- Calcagno, Helsen, Thiemann, Syntactic Type Soundness Results for the Region Calculus, I&C 173(2), 2002.
- Talpin, Jouvelot, The Type and Effect Discipline, LICS 1992 / I&C 1994; Polymorphic Type, Region and Effect
  Inference, JFP 1992. https://www.semanticscholar.org/paper/1150f0d17b4c275639e8c03f5581e073830b267a
- Necula, Proof-Carrying Code, POPL 1997. https://homes.cs.washington.edu/~mernst/teaching/6.893/readings/necula-popl97.pdf ;
  Pnueli, Siegel, Singerman, Translation Validation, TACAS 1998; Necula, Translation Validation for an
  Optimizing Compiler, PLDI 2000
- Meijer, Fokkinga, Paterson, Functional Programming with Bananas, Lenses, Envelopes and Barbed Wire, FPCA 1991.
- Leroy, CompCert (CACM 2009, and https://xavierleroy.org/publi/erts2016_compcert.pdf); Kumar, Myreen,
  Norrish, Owens, CakeML, POPL 2014, https://cakeml.org/icfp16.pdf (backend)
- Bao, Wei, Bračevac, Jiang, He, Rompf, Reachability Types, OOPSLA 2021.
  https://dl.acm.org/doi/10.1145/3485516 ; Polymorphic Reachability Types, OOPSLA 2024; Free to Move:
  Reachability Types with Flow-Sensitive Effects, 2025, https://arxiv.org/pdf/2510.08939 ; Escape with Your
  Self: Bidirectional Typing with Avoidance for Reachability Types, PACMPL 2025, https://doi.org/10.1145/3808335
- When Do Staging Annotations Preserve Semantics? Typed Semantics-Preserving MSP with Let-Insertion, 2025,
  https://arxiv.org/pdf/2606.30854 ; Mechanised Semantics of Multi-stage Programming, PACMPL 2025,
  https://doi.org/10.1145/3798260
- Marino, Millstein, A Generic Type-and-Effect System, TLDI 2009. https://web.cs.ucla.edu/~todd/research/tldi09.pdf
- Space-efficient gradual typing: Abstracting Gradual Typing Moving Forward: Precise and Space-Efficient,
  PACMPL 2021, https://dl.acm.org/doi/10.1145/3434342 ; Compiling Gradual Types with Evidence,
  https://arxiv.org/pdf/2512.22684
- Elsman, Combining Region Inference and Generational Garbage Collection; Grossman, Morrisett et al,
  Region-Based Memory Management in Cyclone, https://www.cs.umd.edu/projects/cyclone/papers/cyclone-regions.pdf ;
  Reference Capabilities for Flexible Memory Management, 2023, https://arxiv.org/pdf/2309.02983
