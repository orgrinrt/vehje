# Canon recheck: verifying the panel's own claims against the round's last topics, and where the changelists and the newer notes drifted from them

**What this is:** op asked for a second pass, specifically against the last topics of
`mock/design_rounds/202607240130/` (not the changelists), to check that my claims and Rompf's actually hold,
to say precisely where a changelist is wrong relative to the topics and how to fix it, and to check whether
that wrongness propagated into the two research notes that already exist adjacent to this panel
(`202607241000_framework-completion-and-deferral-boundary.md`,
`202607241300_deferral-boundary-oracle-audit.md`), since op reads the current state as having drifted from the
canonical round's intent and spirit. I re-read `202607240015_topic.full-arc-bench-and-evidence-findings.md` and
`202607240100_topic.consolidated-design-and-taxonomy.md` in full for this pass (the two closing topics), plus
both changelists, plus the two adjacent notes above, plus the sketch each traces to. Neither adjacent note is
newer than my own first deliverable's mtime, so they are not "new, appearing as I write" in the strict sense op
named earlier, but they sit inside the same round's aftermath and are exactly on point, so I read and review
them here as the second instruction asked.

I found one place where my own first deliverable overclaimed and needs walking back. I state it plainly below
rather than let it stand.

## Method

For each claim under recheck: quote the topic's exact words, quote the changelist's exact words, quote the
newer note's exact words where one exists, and say whether the chain holds or breaks, and where.

## 1. The CFG-of-blocks residual: the topic settles it, the sketch proves a narrower thing than the topic claims, the src CL claims only type surface, and both newer notes miss the gap entirely

**The topic.** `202607240100_topic.consolidated-design-and-taxonomy.md:115-124` (Execute section): "the
interpreter is a CFG of straight-line blocks over the Core forms: a block is a maximal backward-only-child-index
straight-line region evaluated by a forward linear post-order scan... only the terminator touches control flow
(`If` and `Match` transfer to the taken successor, loops are back-edges with block-argument-passed loop-carried
values, calls save a return record on a fixed-capacity frame stack bounded by the depth cap)." Repeated in
"Settled versus open" (`:417`): "the CFG-of-blocks interpreter" is listed among things "settled and not
reopened."

**The evidence the topic rests this on.** `202607240015_topic.full-arc-bench-and-evidence-findings.md:105-106`
lists, among "Feasibilities now proven (assumed or hoped, now WORKS)": "the CFG-of-blocks interpreter
(straight-line blocks, control at terminators, bounded frame stack, running loop plus branch plus call
correctly)". I went to the sketch this cites, `mock/research/sketches/202607210530_sk17_cfg-blocks-interpreter/
findings.md`, in full. It is real and it does what it says: a hand-written Zig register machine, already
authored directly in block-and-terminator form, computing `double(sum(1..5)) = 30` and exercising fallthrough,
conditional branch, loop back-edge, and call/return over a depth-capped frame stack. Its own "Design impact"
section is explicit about scope: "This sketch validates the **execution model** they encode," and: "The wire
format owes the block table plus function table... noted in the solution set; this sketch validates the
execution model they encode." (`findings.md:29-30`, emphasis mine.)

**What this actually proves versus what the topic's citation implies.** The sketch proves an interpreter over
an already-block-shaped program is buildable and correct. It does not touch, mention, or attempt the
transformation from `vehje-ir::Node` (a nested, arbitrarily-deep functional term: `If`, `Match`, `Iter`, `Apply`,
`Lambda`, all of which can appear nested inside each other's slots with no explicit join point) into that
block-and-terminator form. This is exactly the distinction I drew from first principles in my first
deliverable's Finding C (execution versus lowering), and the sketch's own text confirms it precisely: "validates
the execution model," not "validates the lowering." The round's evidence topic cites the sketch as proof the
whole mechanism "WORKS," which overstates what a reader should take from it: the interpreter side is proven;
the construction side was never attempted, not even as a sketch.

**The src CL.** `202607240130/202607240200_changelist.src.lock.md:133-137` claims, for `vehje-runtime-abi`:
"restructure into `wire`/`value`/`sink`/`entry`; add block/function tables, value-arena, sink, batched-column
entry... Verification: `Residual` carries block and function tables." I confirmed this in source:
`wire/residual.rs` does define `Block`, `BlockId`, `BlockTable`, `TerminatorKind`, `Function`, `FunctionTable`,
and `Residual<'img>` carries `blocks: BlockTable<'img>` and `functions: FunctionTable<'img>` as non-optional
fields (`residual.rs:262-265`). The claim is true, narrowly. It is also the only claim the src CL makes about
this piece: nowhere does it claim, or does source provide, a pass that builds a `BlockTable` from a real
`Node` tree. The one encoder that exists and is tested, `FlatArenaEncoder` via `encode()`
(`wire/serialize.rs`, `encode.rs`), assigns a tag per Core form (`Let`, `Lambda`, `If`, `Iter`, and so on,
`serialize.rs:54-68`) and writes a flat, tree-shaped, seven-word-per-node record (`serialize.rs:186-238`), which
`encode.rs`'s own module doc names correctly as IR-preserving, "the IR *as the IR*," explicitly distinct from a
codegen lowering (`encode.rs:1-9`). `grep -n "BlockTable::new\|FunctionTable::new\|Block {"
mock/crates/vehje-runtime-abi/src/` still returns exactly one hit, the struct definition.

**Where the changelist is wrong, precisely, and the fix.** The changelist is not factually false. It is
incomplete in a way that reads as complete, which is the more dangerous failure. "`Residual` carries block and
function tables" is a claim about a struct's shape. A reader who has not opened `residual.rs` and `encode.rs`
side by side (as I have now done twice) will read that line, and the topic's "settled, CFG-of-blocks, WORKS"
language, and reasonably conclude the mechanism is built. It is not. The fix is two small, mechanical edits, not
a design change: add a `// FIXME:` at `residual.rs`'s `Block`/`BlockTable`/`Function`/`FunctionTable`
definitions and at `encode.rs`'s module doc, each naming the gap plainly ("no tree-to-CFG lowering exists yet;
`Tier::Arena`'s only tested path is the flat per-node re-tag encoder; these types are schema for a lowering pass
that has not been written"), and amend the src CL's verification line (or a follow-up doc-phase note) to say so
in the same words, rather than letting "carries block and function tables" stand alone. This is a
`mark-placeholders-fixme.md`-shaped gap: a spot that is incomplete relative to the design, with no greppable
marker.

**Where the drift compounded, twice, in the two newer notes.** This is the part that answers op's actual
question: did the gap get caught, or did the record quietly re-certify past it?

`202607241000_framework-completion-and-deferral-boundary.md` is dated 2026-07-24 10:00, after both changelists
locked. Its "What is complete" list (`:16-27`) names, for the runtime-abi crate specifically: "The value-arena
wire types with a declared `#[repr(C)]` layout, the bounds-safe accessors, and the typed structural decode
(`Reader::validate`) with the monotone acyclicity check." That is the value side (the produced-value crossing
back), and it is accurately described as complete; I have no dispute with that half. The residual side (the
program crossing out) is never named in this note at all, not in "What is complete," not in any of the four
deferral categories ("waits on a consumer," "waits on a research-grade build," "waits on the Zig runtime,"
"waits on a design decision that is genuinely open"). The closest the note comes is, under "waits on the Zig
runtime": "The `#[no_mangle] extern "C"` batched-column export... and the runtime dispatch in the driver (the
serialize half is doable, the call half needs the runtime)" (`:57-59`). "The serialize half is doable" is the
exact sentence that, read against what I found in `serialize.rs`, is quietly assuming the serializer already
does the thing the topic settled (build a CFG) rather than the thing it actually does (re-tag the tree flat).
The note's own opening claim, "the implementation matches the documented design at every framework-level
mechanism whose design is settled" (`:15-16`), is therefore not true for this one mechanism, and the note has
no category that would have caught it, because its four categories are keyed to *why* something is missing
(needs a consumer, needs research, needs the runtime, needs a decision), not to *whether* every settled
mechanism was checked against source. The CFG-of-blocks residual construction is settled (per the topic), needs
none of the four things the note's own taxonomy names as blockers (it needs no consumer, no unbuilt runtime, no
open research question, and Finding 2 below shows the "genuinely open design decision" framing does not fit
either), and is simply absent, both from source and from this note's accounting of source.

`202607241300_deferral-boundary-oracle-audit.md`, dated 2026-07-24 13:00, three hours later, is a mechanical
audit with a stated method: "Every remaining `// FIXME` in the shipping crate roots, audited one by one against
the locked design docs... Total: 37 FIXMEs across 12 crates. Every one falls into a design-mandated-deferred
class. None is defer-instinct... Boundary reached, and it is design-mandated, not defer-instinct" (`:1-11`,
`:69-76`). The method is exactly "grep every `// FIXME:`, then classify it." I confirmed by direct grep
(`mock/research/202607241400.../chris_fallin_weval-and-the-load-stage.md`, Addendum 2, and reconfirmed here)
that `vehje-runtime-abi/src/wire/residual.rs` and `encode.rs` carry zero `// FIXME:` markers anywhere near
`Block`/`BlockTable`/`Function`/`FunctionTable`, and the crate's actual FIXME count (`entry.rs:58`, and four in
`residual.rs` for unrelated fields: arity placeholder, relation-kind vocabulary, seed relation, provenance
arrays) does not include one for this gap. A method that audits FIXMEs is structurally unable to find a gap
that was never marked with one. This is not a criticism of the audit's execution (I have no reason to doubt it
correctly classified the 37 it found); it is a limit on what the method can certify. "Boundary reached" is true
relative to the marked boundary and false relative to the design's actual boundary, and the audit's own
Class 2 heading ("runtime-coupled... the wire ABI crossing to Zig") lists exactly the crate where the unmarked
gap lives, one paragraph away from where it should have been caught, had the residual side carried the marker
the value side's honest FIXMEs already model.

**The compounding pattern, named plainly.** Topic settles the mechanism and cites a sketch that proves a
narrower thing than the topic implies. Src CL ships the type surface, states a true-but-incomplete claim, adds
no FIXME. A same-day completeness note reads the src CL's claim, does not re-derive from the topic or from
`serialize.rs` directly, and certifies the crate complete on the strength of the untested assumption that
"serialize" already does what "settled" says it should. A second same-day audit reads the completeness note's
premise (or independently greps FIXMEs, which has the same blind spot either way) and certifies the whole
framework "boundary reached." Three artifacts in sequence, each trusting the one before it instead of the
canonical topic and the actual source, is precisely the failure `canonical-design-outranks-intermediate-rounds
.md` names: intermediate rounds copy each other's framing, and mutual agreement among them is not corroboration,
it is shared drift. Fix: add the two FIXMEs now (cheap, immediate), and treat "boundary reached" in the oracle
audit as provisional until a second audit pass specifically checks every "settled and not reopened" bullet in
the consolidation topic against source, not just every existing FIXME against its design doc. The second check
is the one this incident shows the first check cannot do by construction.

## 2. The Anf dependency the audit did not connect, and why it changes how hard the fix actually is

Having pinned the gap precisely, the more useful question is how hard it is to close, and here I need to
correct the framing in my own first deliverable rather than repeat it.

I originally wrote that going from the tree to a CFG "is closure conversion plus explicit control-flow
construction, the front half of a real bytecode compiler," and left it an open, unowned question. That is not
wrong, but it understates a connection the round's own material already makes for a sibling problem, and it
misses that the round's own deferred-and-marked `Anf` is a real prerequisite, not an independent nicety.

`vehje-lower::Anf` (`vehje-lower/src/lib.rs:382-393`) hoists every non-trivial subexpression into a fresh `Let`
binding: "name every intermediate so effect order is explicit in the residual and no separate sequencing form
is needed." It is a no-op today, correctly marked `// FIXME:` and correctly classified in the oracle audit's
Class 3 ("open-design mechanism, shape unspecified... DEEPDIVE_LOWERING is explicit that the redirect-only
`Rewrite` is a deliberate architectural choice... An unconditional no-op is honest where a partial
implementation would silently fail," `deferral-boundary-oracle-audit.md:38-42`). That classification is correct
on its own terms. What it misses is that `Anf` is not only a general IR-hygiene nicety, it is the specific
prerequisite the CFG-of-blocks construction needs and does not yet have: a `Node::If` sitting nested three
levels deep inside a `Let`'s value slot has no well-defined "successor block" until the surrounding expression
has been flattened so the if's result is bound by name at a fixed point, which is exactly what ANF gives for
free (Flanagan, Sabry, Duba, Felleisen, "The Essence of Compiling with Continuations," PLDI 1993, the citation
Rompf's addendum-worthy prior-art thread and my own first deliverable's Addendum 1 both already name for a
different reason). Once every branch's result is ANF-bound, the "block equals a maximal straight-line run
ending at the first control-transferring node" definition the consolidation topic states (`:117-118`) becomes
well-defined over the flattened term, and building the block table from it is much closer to a naming and
packaging pass than a fresh analysis: this is Appel's observation in "SSA is Functional Programming" (Lisp and
Symbolic Computation, 1998) that a direct-style term with named, explicit join continuations is already
isomorphic to block-arguments SSA (no phi nodes, block parameters instead, which is exactly the representation
family the consolidation topic's "block-argument-passed loop-carried values" language names, and the same
family Cranelift's CLIF and MLIR use). So the honest, corrected version of my Finding C is: the CFG-of-blocks
lowering is not an independent, unscoped piece of work sitting beside the framework's other deferrals. It is
downstream of `Anf` specifically, its shape is more tractable than "closure conversion from scratch" once `Anf`
lands (name the join points, and the block table is close to falling out), and neither the consolidation topic,
the src CL, nor either of the two newer notes states this dependency anywhere. Op's frustration that the state
"has drifted in intent and spirit" is, at this one spot, precisely this: the round settled a mechanism (the
CFG-of-blocks interpreter) whose construction path depends on another mechanism the same round explicitly and
correctly deferred (`Anf`), and nothing in the taxonomy says so, so the gap reads as two unrelated deferrals
instead of one causal chain with a known next step.

**Fix, concretely.** Add a line to `Anf`'s FIXME and BACKLOG entry naming the CFG construction as the
consumer that needs it (not just "the mutable-arena rewrite path" in the abstract), and add the residual-side
FIXME from Finding 1 with an explicit "blocked on `Anf` landing" note. This turns two disconnected gaps into one
named dependency chain, which is what a reader six months out actually needs.

## 3. The unbounded fixpoint in the topic under review: rechecked against the round's own stated safety mechanism for the sibling case, and it holds up stronger than I first argued

My first deliverable's Addendum 2 argued the `202607241330_topic.ir-to-ir-expansion.md` (today's topic, the one
this whole panel evaluates) contradicts the round's closing statement's "no unbounded saturation" line for the
cheap stratum. Rechecking against the full text of `202607240015` (which I had not read in full when I wrote
that addendum, only cited), the case is stronger than I first made it, not weaker.

`202607240015_topic...md:90-94`, "the comptime kernel must be iterative" finding: "Recursive comptime SIGSEGVs
the Zig compiler at about 2500 depth and segfaults rather than erroring, a hard wall... This promotes the
defunctionalization owed-piece from 'the principled route' to 'the only route,' and the same discipline binds
the load verifier and the e-graph extractor (recursive extraction hangs on the cycles congruence closure
creates; it must be an iterative cost fixpoint)." And `:124-126`, "the eqsat explosion is tamed": "a proper
e-graph saturated with commutativity and distributivity grows a consistent 5.2x linear (not exponential) in
four rounds regardless of depth... which validates eqsat as dev-time-only and **the bounded streaming window as
a hard termination-and-memory safety mechanism**" (emphasis mine). This is the round's own evidence topic,
locked the same day as the consolidation, stating in its own words that a bounded window is a *hard safety
mechanism*, not an optimization, for exactly the class of growing-arena fixpoint the e-graph is. The IR-to-IR
macro-expansion-plus-const-fold fixpoint the topic under review proposes (`202607241330_topic...md:36-38`, "run
expansion, and if it changed the IR, run it again on its own output, until a run produces no change," no stated
bound anywhere) is the same class of mechanism: an arena-emitting fixpoint whose growth is not obviously bounded
by a finite-height lattice the way `vehje-fixpoint::Engine`'s relational fixpoint is (my first deliverable's
"where I depart" section already draws this distinction; I restate it here because it is the operative fact,
not a side note). The round's own evidence explicitly names the fix as mandatory for the sibling mechanism, and
the very next topic omits it for this one, without discussion. This is not me proposing a fuel bound from
first principles and Rompf proposing one independently; it is the round's own prior evidence already having
established, in writing, that this exact class of pass needs one, one topic before the one that skips stating
it.

## 4. A correction to my own first deliverable: the native-ceiling number is not settled at ~2x either, and I should not have said so as if it were

My first deliverable's Finding F said the round's closing consolidation "still lists 'the native 1.0x-to-1.5x
ceiling' as one of the bench-backed corrections it locks in," citing `202607220300_bench-remeasure-synthesis.md`
as the correction that supersedes it (a switch or function-pointer-table interpreter measured at roughly 2.0x
to 2.2x under an interpreter that a comptime-const-fold could no longer collapse). Having now read
`202607240015` in full, that framing is too clean, and I should say so directly rather than let it stand.

`202607240015_topic...md:68-78` (Part 1, restating the original de-risking sketch's finding) says: "Native is a
ceiling of about 1.0x to 1.2x over a good interpreter, not a 10x keystone... copy-and-patch native buys about
1.0x to 1.2x over a good register interpreter across three regimes." This is, word for word, the pre-retraction
number `202607220300` explicitly calls a "native-vs-native" measurement artifact (the sketch's own two-
instruction test program had been comptime-const-folded into the "interpreter" side too, per `220300`'s own
account). Then `202607240015:207-215` (Part 2, the composition-matrix run, a distinct and later effort with its
own expert-panel review and ISA-level dispatch confirmation) gives a third number: "the native throughput
ceiling is about 1.5x over the good interpreter (slightly above Part 1's 1.0x-to-1.2x scalar figure because the
carrier regimes include cases with more headroom, but the same order)." Neither part of `202607240015`, the
round's own final, chronologically-latest bench topic (whose own stated rule is "chronology governs: where an
earlier bench number was refined by a later, more rigorous re-measure, the later number holds,"
`202607240015:16-17`), cites, mentions, or reconciles with `202607220300` anywhere in its 309 lines, and its
"Grounds on" list (`:14-17`) names the de-risking synthesis, the composition-matrix build record, and the ABI
hole-poke synthesis as its sources, not `220300`.

So there are three candidate numbers on record for the same question, from three efforts, none of which
explicitly stands down for another: 1.0x-1.2x (the original sketch, restated uncritically in `240015` Part 1
despite `220300` having named its specific measurement flaw), roughly 2.0x-2.2x (`220300`, the standalone
remeasure that caught the flaw and fixed it), and about 1.5x (`240015` Part 2, the composition-matrix run, which
does appear to use the same anti-artifact discipline `220300` used, cdylib subprocess isolation and opaque
crossings, per `240015`'s own "measurement discipline" section, `:264-278`). My first deliverable picked the
middle one and stated it as the correction that should replace the doc's stale figure. That was premature. The
honest status, which I should have reached the first time: **all three efforts agree on the direction and the
order of magnitude** (native is a modest multiplier, somewhere north of the original 1.0-1.2x floor and nowhere
near a 10x keystone), and disagree on the exact figure by a factor of roughly 1.3x to 1.5x between the two
post-correction efforts, with the original pre-correction number now doubly suspect (named an artifact once,
never independently reconfirmed by either later effort). The number to cite going forward is not "1.0x-1.5x"
(what the doc and the consolidation currently say, which keeps the retracted floor alive) and not "~2x" (what I
said, cherry-picking one of two non-reconciled later efforts). It is "the pre-retraction 1.0x-1.2x figure is
confirmed wrong by two independent later efforts; the honest range pending reconciliation is roughly 1.5x to
2.2x, and reconciling `220300` against the composition-matrix's own native-ceiling cell (do they cross the FFI
boundary the same way, is the workload shape the same, which one, if either, needs to yield) is itself an owed
piece of work, not a closed question." File that reconciliation as its own small research task; do not let
either number stand alone as "the" corrected figure, including the one I gave.

## 5. What still checks out on reread

Everything else in both my first deliverable and Rompf's holds against this closer reading. `vehje-fixpoint`'s
existence, its three named clients, and its non-use in `vehje-lower` and the partial use in `vehje-typecheck`
are exactly as described (`202607240100_topic...md:280-289` against `vehje-fixpoint/src/lib.rs`,
`vehje-lower/Cargo.toml:8`). The `vehje-typecheck` to `vehje-check` rename is honestly gate-blocked and flagged
in the src CL (`:36-44`), not silent drift, and both newer notes correctly list it as an owed maintainer
decision rather than mis-certifying it. `NodeRef`'s missing arena brand (my Finding A) is not addressed by
anything in either topic or either newer note; it remains the sharpest unaddressed soundness item alongside
macro hygiene, and it is not implicated in either compounding-drift chain traced above, it is simply still
open. The twelve-crate taxonomy, the dependency layering, and the `Handle` form's coverage across every match
site the src CL names all check out against source as claimed.

## Open items handed forward, superseding my first deliverable's list where they overlap

1. Add the two `// FIXME:` markers named in Finding 1 (`residual.rs`'s Block/Function types, `encode.rs`'s
   module doc), naming the gap as blocked on `Anf` per Finding 2, today, regardless of when the construction
   pass itself is scheduled.
2. Update `202607241000_framework-completion-and-deferral-boundary.md`'s "waits on the Zig runtime" bullet: "the
   serialize half is doable" should read "the serialize half ships the flat tree encoder; the CFG-of-blocks
   construction the design settled is a distinct, unbuilt pass blocked on `Anf`."
3. Re-run the `202607241300_deferral-boundary-oracle-audit.md`-style pass a second time, this time walking every
   "settled and not reopened" bullet in `202607240100`'s closing list against source directly, not against the
   FIXME population, since this incident shows the FIXME-population method has a structural blind spot for an
   unmarked gap.
4. State an explicit fuel or node-budget bound in the doc CL that follows this panel for the IR-to-IR
   macro/const-fold fixpoint, citing `202607240015`'s own "bounded streaming window as a hard termination-and-
   memory safety mechanism" line as the round's own precedent for why this is mandatory, not optional (Finding
   3).
5. Reconcile `202607220300_bench-remeasure-synthesis.md` against `202607240015`'s composition-matrix native-
   ceiling cell explicitly: same crossing discipline, same or different workload shape, which number wins, and
   correct `DEEPDIVE_OUTPUT_SPECTRUM.md.tmpl` and the consolidation's "settled" list to the reconciled figure,
   not to either standalone number (Finding 4, superseding my first deliverable's Finding F).
