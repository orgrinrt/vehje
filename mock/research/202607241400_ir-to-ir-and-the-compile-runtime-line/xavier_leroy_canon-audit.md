# Canon audit (Xavier Leroy)

**Author lens:** fresh independent auditor; verified compilation, mechanized proof, the discipline of stating
exactly what a theorem says and what remains trusted. No continuation of prior panel framing.

## One-line verdict

The canon holds: it is faithful to op's ratifications, internally coherent, honest about proven versus intended
versus shipped in nearly every place, and every shipped-state claim I could check against source checks; it needs
one fidelity correction (the depth-cap backstop is always-on per op, not an else-branch), one state label
(section 2F asserts "no trusted emitter" without marking that the emitter seam is open today), a small set of
ratified items that the synthesis compressed out, and, for a document whose stated lineage is CompCert's
trusted-rim honesty, the one thing that lineage actually demands and the canon never writes down: the explicit
list of what remains trusted.

## Method (what I read, what I verified against source)

I read the canon first, then the authority on what op decided (`op-ratification-answers.md`, batches 1 through 7
plus the frontier list and the net-new STATUS block), then the full numbered inventory
(`count-our-chickens-and-ratification-inventory.md`, Groups I through XVII), then both `canonical_candidate_`
enumerations in full including the negative fork's addendum audit, the SPJ soul-audit (F1 through F8), the Wingo
double-take (findings 1 through 3, opens 1 through 5), Rompf's staging deliverable, Fallin's canon-recheck (all
five findings, including his self-correction on the native ceiling), and `00_context.md`. I re-derived rather
than accepted: every claim below about what op ratified is checked against the ratification doc's own words, not
against the canon's summary of them.

Shipped-state claims verified directly against source at audit time: `binding: Knowledge::empty()` and the
handled-operation effect-subtraction FIXME at `vehje-typecheck/src/lib.rs:311` and `:301`; the sealed `Checked`
witness at `:80`, `mint_checked` at `:158` with `ContainsAll` bounds at `:164` and the caller-supplied-sets FIXME
at `:154`; zero code uses of `vehje-signature` in `vehje-typecheck/src` (one doc-comment mention of the mandate at
`lib.rs:9`, which is the inert prose itself); the `Handle` representation-deferral FIXME at
`vehje-ir/src/node.rs:138-143`; the four-point `BindingTime` lattice at `vehje-ir/src/grade.rs:105-117`; the
interner-local word-fold statement in `vehje-ir/src/hash.rs:1-19`; `vehje-runtime-gen` hashing the byte image
(`xxhash3_64(package.slices[i].bytes)` at `lib.rs:119`, whole-package fold at `:115-120`); the no-op `Anf` and
`MacroExpand` FIXMEs and the `Saturate` deferral in `vehje-lower/src/lib.rs` (at approximately `:385`, `:400`,
`:85` as the canon cites); `Block` at `residual.rs:98`, `Function` at `:111`, `BlockTable` at `:122`, with zero
constructions of any of them outside the defining file (grep across `mock/crates/`); and the superseded topic
`202607241330` stating the unbounded "until a run produces no change" fixpoint at its Decision item 2. Every
line-number citation in the canon that I checked lands on or within two lines of the cited construct.

## What holds

- **Fidelity to the ratification, in the large.** I walked the STATUS block's net-new list item by item:
  recursive macros with the three-layer terminator (2G), the native range with its baseline index (7), the
  joint identity-plus-mechanism drift test (10), CR1 with its scope and the bench-decided representation fork
  (2C, 11), the one-dispatch-three-discharges handler frame with the honest unbuilt-third note (2B), generative
  freshness with the red tests owed (4), the N-space arena reframe with branding riding the fork (2G, 11),
  diagnostics first-class (2J), benches-provisional (2H, 5, 7), the census-as-needs-signal distinction (3), the
  rebalanced fused one-liner (1), and the consolidation corrections plus second-direction audit (11). All
  present, all in op's ratified form except the two defects numbered below.
- **The shipped-state discipline is real, not performative.** Every "state:" label I checked (2D spine unbuilt,
  4 inclusion shipped-in-shape-half-wired, 2C FIXME citations, 11 CFG gap and inert signature) matches source.
  The canon's stated rule that shipped-state claims are verified at authoring time was actually followed.
- **The corrections the panel forced are carried without dilution.** The 2.0x is no longer "doubly-superseded"
  (7 carries SPJ F2's correction exactly, including the floor-at-best-case caveat and the owed reconciling cell);
  eqsat, flat-switch, and copy-and-patch-privileged are in the inverse with the consolidation fix filed (11);
  the hash identity boundary is stated and source-confirmed (2F, 4); CR1 is scoped, not flat (2C), with Wingo's
  soundness-not-efficiency framing of the under-effects case intact; the drift test refuses the mnemonic as the
  test (10), which is SPJ F3 discharged properly.
- **The inverse is complete against Groups X through XVI.** I walked all 43 inventory items of the kill
  catalogue, the superseded shapes, the identity errors, the bench kills, the bench-integrity corrections, and
  the uncertains against section 9. Every one is present, on the correct side, with the correct status,
  including the subtle ones: X7's re-justification note (the coroutine kill leaned on the now-parked eqsat),
  XVI8 updated to the N-space form per XVII7, XI3's living-intent-open-mechanism split.
- **Internal coherence.** The Supersedes header, section 2G, section 6, and section 11 tell one consistent
  story about the expansion, the terminator, and the forks; the precedence rule is stated once and obeyed; the
  drift test's clauses each trace to a ratified kill or conviction. Section 10 is implementable as written: a
  future reader can apply each clause mechanically.

## What does not hold, over-claims, under-specifies, or omits

1. **The depth-cap backstop is garbled from always-on to else-branch.** Canon 2G: "else by a recursion depth
   cap as the fallback and the always-on safety backstop"; canon 11: "static cyclicity detection on the
   typestate if a sketch proves it feasible, else the configurable depth-cap safety backstop (default 255)."
   op's XVII1 is explicit and different: the cap is "a sensible always-on safety backstop either way," and "if
   yes [static detection is feasible], primary mechanism; the depth cap stays as the always-on safety
   backstop." The canon's "else" makes the cap conditional on static detection failing; op ratified it as
   unconditional. This matters operationally: under the canon's wording, a feasible static detector removes the
   hang-guard, and a mis-graded third-party family macro that evades the detector then hangs the expansion.
   Correction: in 2G and 11, state that the cap is always present as the safety backstop; static cyclicity
   detection, if feasible, is the primary termination mechanism layered above it, not a replacement for it.

2. **Section 2F asserts "no trusted emitter" with no state label, and half of II12's ratified augment is
   missing from the conviction.** 2F: "lens projections of it, differential-checked, so there is no trusted
   emitter" and "the lens projection is what closes it." Both read as achieved properties. The shipped state is
   the opposite: the lens projection is unbuilt, `vehje-runtime-gen` ignores its signature argument
   (`_signature`, verified by SPJ, still true), the interim manifest check is a whole-package byte hash only
   (`lib.rs:79-81` says per-slice hashes land later), and the check pass reads no signature schema. Today the
   emitter seam is trusted, entirely. Batch 4 ratified II12 as "adds the identity boundary + emitter-seam +
   inert-signature"; the canon's section 4 entry carries only the identity boundary. Correction: give 2F and
   the section 4 conviction the same state label 2D has, in words like: "designed closure; today the
   Rust-to-Zig emitter seam is trusted and the lens projection is owed (Section 11)." The design claim stands;
   the present tense does not.

3. **"Both artifacts are certified at our build" never names the certified property or the checker.** 2E and 3
   state double certification without saying what is certified by what: rustc's type system certifies the
   well-formedness, inclusion, and grading of the emitted data; the Zig compiler certifies that the comptime
   specialisation type-checks against that data. Neither certifies semantic preservation of any lowering, and
   nothing certifies the Zig engine's own evaluation logic. "Structural proofs" (2E, 3) is used without
   definition. The assurance-as-a-grade frame (2E) is exactly the right instrument and partially rescues this,
   but the meta-level sentence stays soft. Correction: one sentence in 2E naming the certified property per
   artifact and one naming what the certification does not cover. A certification claim whose perimeter is not
   stated is the one failure mode this document's own lineage exists to prevent.

4. **Confirmed ethos and surviving-intent items compressed out.** Batches 6 and 7 confirmed Groups III, VII,
   and IX wholesale; the canon carries most but drops: III4 (design through the fault, **or prove the exact
   blocking constraint and design the maximal thing that does hold**; the honesty valve on the maximal-shape
   ethos, absent from 5 and from the drift test); IX5 (guarantees are type-system properties discharged before
   runtime with no separate analysis pass, and **content-addressed determinism is the payoff of totality**;
   absent everywhere, though it grounds the manifest and cache story); VII2 (supersede forward, never rewrite a
   locked topic, ratified as itself a load-bearing intent; practised by the document, stated nowhere in it);
   VII4 (open families as traits, the framework core stays small while consumers carry their richness); VII5
   (the committed ABI and the 1845 panel calls, re-voiced and not reopened). Correction: a short addition to
   sections 5 and 3; III4 and IX5 are the two with real load, the others may be one clause each.

5. **The semi-naive relational-fixpoint substrate has no positive entry.** Negative A2 lists it as living
   ("semi-naive, no-alloc, cost-bounded; benched feasible, 8M nodes / 16M edges in about 50 ms"), and op
   confirmed Group II and the A2 material. In the canon it appears only inside the inverse, as the replacement
   in the coroutine kill (9) plus the re-justification note. A living, benched conviction should not exist only
   as the winner of a kill. The omission traces to the inventory's union walk (it was never numbered as a
   distinct claim), so this is inherited, not introduced; the canon is still the place to fix it. Correction:
   one conviction line in section 4: the relational-fixpoint substrate serves lease inference and load
   verification, re-justified on those two alone, whole-column over delta on realistic shallow graphs.

6. **I6's load-verifier clause vanished in the fold-in.** op's batch 4 answer on I6 was "holds as a starting
   point, iterate, de-pretentious-ify, and make it address everything." The canon's fusion (1) is a genuine
   improvement on the pretension axis, but one concrete element of I6 was dropped rather than iterated: "a load
   verifier bounded by its own lease-lattice height." The load verifier appears in 7 (the recursion wall binds
   it) and obliquely in 8 (tnum scoped to the numeric residual), but its bound is stated nowhere. Correction:
   one clause, most naturally in 3 or 4, restoring the verifier and its bound, or an explicit note that op
   retired the clause (I found no such retirement in the record).

7. **The ratified fresh-expert-audit plan step is absent from section 11.** Batch 5: "NEW PLAN STEP: after
   ratification (and the canon draft), dispatch a fresh expert to audit everything decided + ratified here."
   Section 11 lists the other process items (the consolidation doc round, the second-direction audit) but not
   this one. It is being discharged by this very document, so the practical cost is nil; the paper-trail cost
   is that a future reader of section 11 cannot see the step was owed. Correction: one line in 11, marked
   discharged by this audit.

8. **The half-married NaN-boxing clause risks smearing the confidence tag onto the register-VM number.**
   Section 8: "NaN-boxing plus register VM (bench-leaning, low-to-medium confidence, register VM 1.41x to 3.06x
   per profile not a 1.9x composite)." Per SPJ F4a/F4b and the ratified VI2 flags, low-to-medium confidence
   attaches to the NaN-boxing cell only; the register-VM range is a corrected, PMU-confirmed measurement
   (section 7 itself states "register beats stack every profile"). Correction: split the clause so the
   confidence tag is unambiguous.

9. **`hash.rs`'s own header contradicts the boundary the canon says the shipped hash confirms, and nobody has
   flagged it.** New finding, source-side. `vehje-ir/src/hash.rs:3-4`: "One definition, two consumers:
   hash-consing in `vehje-lower` and the manifest in `vehje-runtime-gen`." The manifest is not a consumer of
   this word-fold: `vehje-runtime-gen` imports only `arvo_hash::{xxhash3_64, ContentHash}` and hashes slice
   byte images. The body of the same doc comment states the correct boundary (interner-local, cross-artifact is
   the byte-image's job), so the file contradicts itself in its own header, in the exact direction of the
   consolidation prose SPJ F8 flagged. The canon's claim (2F, 4) is true at the code level; the citation it
   rests on carries an unfixed stale sentence. Correction: add the `hash.rs` header fix to the XVII9(b)
   consolidation doc round in section 11.

10. **Three small wording items.** (a) 2G says the round "already killed cap-based termination for this exact
    unfold"; the kill (`202607202055`, X3) was of PE-as-e-graph-extraction; the positive fork's own audit note
    states the nuance correctly (a different mechanism, the principle generalises and governs). "This exact
    unfold" overstates by a hair; say "for this class of unfold, and the principle governs the recursive
    discharge." (b) 2G's parenthetical explains `fold_core`'s inadequacy via the redirecting `Rewrite`, which
    is a different construct in a different crate; the record's phrasing (the visitor shape cannot create
    nodes) is cleaner and avoids conflating `vehje-codegen::fold_core` with `vehje-lower::Rewrite`. (c) 4's
    generative-freshness entry says "one discipline seen at four sites" without naming the four (hygiene at
    expansion, freshness at resumption, the terminator's within-stage layer, the exhaustion diagnostic); under
    the XVII3 maximal-explicitness mandate, name them.

## Honesty of the proof/certification claims

Mostly exemplary, with two soft spots already numbered above (findings 2 and 3) and one register issue.

What is stated correctly: the graded spine is labelled a living intent with the exact source line that proves
it unbuilt (2D, 4); inclusion is labelled shipped-in-shape, half-wired-in-derivation, with the caller-supplied
FIXME cited, which is precisely the honest statement (the witness machinery exists; the obligation it
discharges is weaker than the design's until derivation lands); CR1 carries its scope in both directions
(re-execution demonstrated, reinstatement and under-effects open, the latter named a soundness obligation);
the third handler discharge is flagged unbuilt at every mention; the CFG residual gap and the inert signature
read are stated as gaps with the exact type surface named; the bench numbers carry their caveats and the
benches-provisional principle is applied to the document's own evidence. The precedence and the
design-versus-state axis separation (preamble, 12) are stated cleanly and observed.

The register issue: the canon uses "proven" in two senses without marking the difference. In 2C, "proven"
means a sketch ran and produced the expected result (WORKS); in 2D and 2E, "proven sound" gestures at typing
theorems that do not exist yet anywhere but in design intent. Neither sense is a mechanized or even a
pen-and-paper proof. The in-house usage is consistent with the record ("feasibilities now proven"), and the
scope-carrying mostly disambiguates, but a canonical document read years from now should fix the ladder once:
proven (a checked theorem), demonstrated (a sketch outcome), measured (a bench cell), intended (design). One
sentence in the preamble would do it. This is the same discipline as assurance-as-a-grade, applied to the
canon's own prose.

The structural gap: the trusted base is never assembled. The document invokes CompCert's trusted-rim honesty
as married lineage (2E, 8) and scatters the actual trusted elements across five sections: the Zig compiler
(the entire second certification), rustc, the C ABI marshalling, the byte-image hash function, the open
Rust-to-Zig emitter seam, the caller-supplied family and effect sets until derivation lands, and the load-time
verifier's own decode logic. The lineage it cites is precisely the practice of writing that list down in one
place. Until the list exists, "doubly certified, no prover shipped" is a true statement with an unstated
perimeter. Recommendation: a short "what remains trusted" subsection, in the canon or in the XVII9 doc round,
enumerating the trusted elements and pointing at the owed work that shrinks each (the lens projection, the
set derivation, the verifier). This is the single highest-value addition available to the document.

## Anything worth carrying forward

- **Rompf finding 5 fell through the inventory's mesh and will bite when the emit path lands.** Once every
  productive pass emits through a `Builder`, a hash-consing constructor subsumes the redirect-table CSE, and
  keeping both is two sharing mechanisms for one meaning. This was never inventoried, never ratified, and is
  not in section 11's owed list. It is a bench-decidable fork (fold CSE into the emitting Builder versus keep
  the redirect table for the no-copy case) and should be filed with the arena bench, since both touch the same
  emit machinery.
- **Fallin's Anf-to-CFG causal chain deserves its name in section 11.** The canon says the CFG lowering is
  "downstream of the IR-to-IR emit path"; Fallin's recheck showed it is specifically downstream of `Anf`
  (a nested `If` has no successor block until ANF names the join points; Appel's block-arguments observation
  makes the block table nearly fall out afterward). Naming `Anf` turns two disconnected deferrals into one
  dependency chain, which is what he correctly said a reader six months out needs.
- **Fallin's three-number record on the native ceiling.** His recheck holds a third live cell the canon's
  range excludes numerically: `202607220300` at roughly 2.0x to 2.2x. The canon's 1.5x-to-2.0x is the ratified
  range and "growing on branchy programs" admits more qualitatively, but the owed reconciling measurement
  (11) should explicitly include the 220300 cell in its scope rather than treating 2.0x as the recorded
  maximum; otherwise the reconciliation can quietly clamp at the range the canon happened to print.
- **Wingo's product framing is the right deflator and the canon carries it; keep it prominent.** The
  authoring-and-templating majority is compile-heavy and trivial-execute, so the interpreter's dispatch cost
  governs the product and the native multiple is nearly moot for the identity. Section 2H states this; it is
  the single best inoculation against the next native-number relitigation.
- **The compounding-drift mechanism Fallin named (surface existence standing in for mandate satisfaction) is
  correctly generalised in 11.** The second-direction audit is the check the FIXME-walk cannot do by
  construction; finding 9 above (the `hash.rs` header) is a fresh instance of the same species, which
  suggests the second-direction audit should also walk doc-comment claims in source headers, not only design
  bullets.

## Open questions handed back

1. Confirm the finding-1 correction with op: the depth cap as always-on backstop even when static cyclicity
   detection proves feasible. The canon's current wording ratifies a weaker guard than op stated.
2. Where does the trusted-base list live: a subsection of this canon, or the XVII9 consolidation doc round?
   Either is fine; its absence from both is not.
3. Were VII5's 1845 calls (spill-as-capability, the depth cap as finite lattice height) deliberately
   compressed out as re-voiced-elsewhere, or should the canon restate them? The second half interacts with
   the retired depth-cap pun and would benefit from one disambiguating sentence either way.
4. Was I6's "load verifier bounded by its own lease-lattice height" retired, or dropped by accident in the
   one-liner fold-in? I found no retirement in the record.
5. Should the proven/demonstrated/measured/intended register be fixed in the canon's preamble now, or in the
   doc round? It is one sentence, and every future state-label reads through it.
6. The Rompf-finding-5 CSE-versus-hash-consing-Builder fork: file it alongside the arena bench, or as its own
   owed item? It is currently in neither list.
