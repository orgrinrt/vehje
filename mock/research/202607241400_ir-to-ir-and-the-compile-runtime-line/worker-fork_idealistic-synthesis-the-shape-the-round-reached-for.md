# The shape the round was reaching for: an idealistic synthesis truest to the spirit of 202607240130

**Date:** 2026-07-24
**Author:** worker fork, idealistic-synthesis directive
**Method:** read the whole panel (Rompf + addendum, Fallin weval + canon-recheck) and the three sibling
forks (canonical-drift-and-mirror, changelist-drift, retake-under-bench-supremacy), the canonical round's
soul topics (`202607210120` core re-derivation, `202607210045` authoring-sides, `202607202330`
identity-recenter, `202607240100` consolidation), and the bench authority (`RUN_SUMMARY.md`, carrier PMU
re-run). Then ignore the practical and the shipped, and ask one question only: what was the round's spirit
actually reaching for, and what shape realizes it more fully, more soundly, more optimally than what the
literal concretes produced. Precedence obeyed throughout: hard data over topics over changelists (op,
2026-07-24), and canonical spirit over canonical letter (this directive).

This is not a drift audit. The three sibling forks already found the drift and ranked the wounds; I take
their findings as given and cite them. My job is the forward, idealistic half: not "this strayed, wire it
back," but "here is the platonic shape the round kept circling and never quite drew, and it is better than
what the round itself wrote down."

## One-line reading

The round spent twenty topics converging on ONE idea and then wrote it down as five separate mechanisms;
the ideal is to collapse them back into the one idea the round discovered but never named as one:
**binding-time-directed handler discharge**, a single reducer that walks the four-point knowledge lattice
and, at each form, discharges it in the way that form's binding time dictates, emitting IR at a build stage
and producing a value at runtime. Everything the panel argues about (IR-to-IR expansion, const-fold, macro
expansion, the compile/runtime line, the two-artifact split, even the CFG question) is one consequence of
that single mechanism, and the shipped design fragmented it into `ConstFold`, `MacroExpand`, `Interp`
staging, `Handle` discharge, and the `RuleTable` replay, five sites for one operation.

## The soul, distilled to its one sentence

The round's own closing statement gives four coupled commitments (one signature projected; the proof
compiled away; two artifacts, data not source; native never the driver), and the canonical-drift fork
distilled them faithfully (`worker-fork_canonical-drift...:14-22`). But underneath those four is a single
generative idea that `202607210120` states outright and then files under "the pieces to work out":

> "the effect set, the host-call boundary, and macro expansion collapse into one handler discipline. An
> effect is an operation; a handler is what services it. A host-call is a runtime-effect operation the host
> handles. Macro expansion is a compile-stage-effect operation the compile stage handles. So the
> build-versus-runtime split the old effect lattice drew by hand becomes which stage provides the handler,
> which is the binding-time coordinate." (`202607210120:43-49`)

That is the soul. Not four commitments: one. The four are its shadows. "One signature projected" is this
idea on the type axis (one operation set, many projections). "The proof compiled away" is this idea on the
assurance axis (discharge the handler at its stage, erase it). "Two artifacts" is this idea on the artifact
axis (the stages coarsen into two binaries). "Native never the driver" is this idea on the output axis
(runtime discharge is one point on a spectrum, not the telos). The round found the unifying idea and then,
in the consolidation and the crates, split it back apart into separately-sited machinery. The ideal is to
refuse that split.

## The central idealization: one binding-time-directed reducer, discharge polymorphic in its output kind

Tiark's addendum finding 3 (`tiark_rompf...:116-118`) is the load-bearing insight the whole panel produced,
and it deserves to be the spine of the design rather than a numbered finding: **a handler's discharge is
binding-time-polymorphic in its output kind. A build-time handler discharges by emitting IR (generative,
into the arena). A runtime handler discharges by producing a value (reductive, into the value-arena).** The
two are the same act (service an operation) with the output kind chosen by the binding time.

Read op's own live correction through this lens and it stops being a separate rule and becomes a theorem.
op said the macro/const "jit" expansion "should be recursive with the single arena we get," not a two-arena
emit. That is exactly what a build-stage handler discharge IS: it recurses over the term, and where an
operation's handler is available at this stage, it services it by appending the expansion into the same
growing arena, in place, child-before-parent. There is no second arena because there is no separate
"expansion pass"; there is one downward-then-upward walk that discharges every form it can at the current
binding time, and a build-stage discharge appends. The single arena is not an optimization over the
two-arena emit, it is the natural shape of build-stage handler discharge, and the two-arena framing (Tiark
finding 3 of part A, Fallin finding A) was an artifact of treating expansion as a distinct pass rather than
as handler discharge.

So the ideal reducer is one function over the twelve forms:

- `Lit` graded known-now: already a value, keep.
- `If` whose condition's `Knowledge` includes the current stage: discharge to the taken branch (this is
  const-fold, but generalized from "syntactic literal condition" to "condition known at this stage," which
  is Tiark addendum finding 2, the LMS `Rep[T]`-vs-`T` line).
- `Interp` whose value is known-now: inline it into the generated text (the staging rule the round already
  singled out, `node.rs:122-124`); unknown-now, keep it as a runtime interpolation. This is the exact
  splice/escape of a two-level quasiquote, and it is the same discharge.
- `Raw`/family operation whose handler is installed at this stage (a macro): discharge by emitting its
  expansion into the arena (this is `MacroExpand`, but it is not a separate mechanism, it is Handle
  discharge with a build-stage handler and an IR-emitting output kind). This is the `FamilyExpand`-equals-
  build-time-`Handle`-discharge collapse.
- `Handle` of an operation whose handler is available now: discharge it (emit IR if build-stage, produce a
  value if runtime).
- everything else: rebuild it with its already-discharged children (the catamorphism).

const-fold, Interp-inlining, and macro-expansion are three arms of this one match, not three passes
(`ConstFold`, `MacroExpand`, `Interp` handling) sited in three places. Tiark addendum finding 2
(`tiark_rompf...:112-114`) names this collapse; the ideal is to make it the actual architecture. The
"compile/runtime line is the binding-time lattice" (Rompf part B) then stops being a slogan and becomes an
operational fact: there is literally one reducer, parameterized by which stage's handlers are installed, and
running it at `Bundler` with the bundler's handlers is the AOT compile, running it at `HostLoader` with the
loader's handlers is the load-time compile, running it at `Runtime` with the host's handlers is execution.
Same function, three handler environments, three binding times. That is the first Futamura projection made
literal in the framework's own code, not just cited as an analogy.

### Why this is more in spirit than what shipped

What shipped: `vehje-lower` has `ConstFold` (redirect-only, syntactic-literal `If` only), `MacroExpand`
(no-op), `Anf` (no-op), a separate `Rewrite` side-table, and `Cse` (a second pass); `Handle` discharge lives
in `vehje-typecheck` behind a FIXME; `Interp` staging is prose. Five sites, four of them stubs, one
soundness discipline (`Rewrite`'s monotone-decrease) that structurally cannot express the generative half at
all (`DEEPDIVE_LOWERING:41-47`). The round's own soul says these are one handler discipline. The shipped code
is the fragmentation the round explicitly set out to unify, frozen at the pre-unification stage. The ideal
is not new invention: it is finishing the unification the round already decided on and then didn't build.

## The single source, actually projected (soul 1, idealized past its literal form)

The canonical-drift fork's sharpest finding is that `vehje-signature` projects nothing and both its
consumers ignore it (`worker-fork_canonical-drift...:26-33`): the round's deepest commitment, inert. The
literal repair is "wire `generate()` to read the signature." The ideal is stronger and it reframes the whole
crate taxonomy: **the signature is not a crate that other crates read, it is the single object of which every
other artifact is a projection, and the projection functor should be the framework's one reusable spine.**

`vehje-signature::Projection` already names the three (`Eliminator` / `Introduction` / `IntroductionCompiled`,
`202607240100` and the sibling's `:28`). The ideal makes this literal: there is one `project` operation,
parameterized by target projection, and
- the eliminator projection is the IR (the twelve forms plus the family operations as `Raw`),
- the introduction projection is the value-domain (content-as-values, the Doc family, `202607210120:38-40,75-77`),
- the introduction-compiled projection is the stencil / native form.
These are not three unrelated outputs; they are one signature viewed three ways, exactly the "one truth,
many projections" pattern the synth corpus keeps landing on (LMS, `202607201618_synth_staged...`). The
idealization: `vehje-runtime-gen` does not "package slices," it RUNS the projection functor at
`LanguageAuthor` time to produce the introduction-compiled projection as validated data; `vehje-check` reads
the eliminator projection's per-operation schema; the value-arena is the introduction projection at runtime.
The crate that "packages" is not a crate, it is the projection functor applied at a binding time. This is
Tiark addendum finding 3's "the value-arena producer and the expansion emitter are the runtime-stage and
build-stage instances of one emit discipline" (`tiark_rompf...:118`) carried up to the signature level: emit
discipline and projection are the same functor, indexed by binding time on one axis and by projection kind
on the other.

Corollaries the ideal forces, each an improvement over shipped:
- **The manifest hash must be the byte-image hash, not the structural hash** (Tiark addendum finding 0,
  `tiark_rompf...:104-106`). A projection crosses the interner boundary between artifacts; its identity is
  cross-artifact; the structural word-fold is interner-local and unsound for that (`hash.rs:1-14`). The
  ideal states the rule once: within-stage identity uses the structural hash (hash-consing in the reducer),
  cross-stage identity uses the byte-image hash (the manifest). This is the same within-stage-vs-cross-stage
  distinction as the tier-vs-binding-time split, landed in the hash layer, and getting it right is a
  soundness precondition for the whole projection story.
- **Certifying, not certified** (Fallin weval addendum 1, `chris_fallin_weval...:352-365`). The inclusion
  proof is not one witness that crosses the artifact boundary; it is one obligation discharged two ways (a
  Rust typestate `Checked<T>` at `Bundler`, a re-derived bitmask subset-test at `HostLoader`), Tiark addendum
  finding 4 (`tiark_rompf...:120-122`). `Assurance::ReprovenPerInstance` (`grade.rs`) already IS Necula-Lee's
  certifying-compiler variant; naming it makes "termination is not claimed" (`202607240100:65`) the principled
  scope of a certifying compiler rather than an apology. The ideal: the proof projection is per-instance
  re-checkable, and that is a feature (small trusted checker) not a weakness.

## Illegal states unrepresentable, applied to the framework's OWN types

The round's certified-generation ethos is "make illegal states unrepresentable in the generated Zig"
(`202607240100:60-65`). The ideal turns that lens inward, onto vehje's own types, where three illegal states
are currently representable:

- **Thermometer-encode `Knowledge`** (Tiark addendum finding 1, `tiark_rompf...:108-110`). Binding-time
  availability is monotone (known-early implies known-late), so the grade is a thermometer over the four
  ordered sources, exactly as `EffectMask` already is (`grade.rs:22-24`). As a raw subset mask it admits
  `{LanguageAuthor, Runtime}` without the middle, on which "earliest stage all inputs are known" (the
  multi-cogen discharge rule, `202607240100:104`) is ill-defined. Thermometer makes the four stages a genuine
  chain, illegal sets unrepresentable, and "earliest known" the lowest set bit. The one reducer above NEEDS
  this: "discharge at the earliest stage the inputs are known" is only well-defined on a chain.
- **The catamorphism return channel** (Fallin weval finding B, `chris_fallin_weval...:117-140`). The reducer
  is bottom-up with a handle threaded upward (`map_core`), not `fold_core`'s top-down `FnMut(&Node)` visitor.
  This is not a nitpick against reuse; it is that the one reducer is a catamorphism over the twelve forms and
  must be typed as one, with const-fold, macro-discharge, and Interp-inline as its algebra. Naming it once
  (as the projection functor's eliminator arm) subsumes the duplicated hand-written walks (`fold_consts`,
  `cse_share`) the shipped code already shows.
- **`binding_time_ceiling` on `Operation`** (Fallin weval finding D, `chris_fallin_weval...:197-209`). A family
  macro's handler is Rust code and Rust never runs in the runtime, so a macro operation carries binding time
  no later than `Bundler`; the effect proof then guarantees no unexpanded macro crosses the ABI. The ideal
  makes this a field the check pass reads, not a convention, which also forces the signature-read wiring (soul
  1) in the same stroke. Under the one-reducer model this is not a special rule for macros: it is the general
  statement that an operation's handler-availability binding time is part of its signature, and the reducer
  refuses to reach a stage past which an operation can never be discharged.

Note the single-arena recursive correction (op) dissolves Fallin's finding A (`NodeRef` arena-branding) for
the expansion path specifically: with one growing arena there is no second arena to confuse a `NodeRef` with.
The branding concern survives only where two arenas genuinely coexist (if any projection ever needs a fresh
target arena); for the build-stage in-place discharge, op's correction is the sounder shape and removes the
hole rather than guarding it.

## CSE is not a pass; sharing falls out of the reducer

Rompf finding 5 and Fallin weval addendum 1 (`chris_fallin_weval...:305-321`) both land on LMS's actual
mechanism: once every form is built through the one `Builder`/reducer, a hash-consing constructor returns the
existing node on a structural hit, and CSE is a property of construction, not a second pass over a built
tree. vehje already has the one interner (`hash_of`, `StructuralHash`) and already does the lookup, just as a
separate `Cse` walk. The ideal folds the hash-cons probe into the reducer's node-construction so that
const-fold, macro-discharge, Interp-inline, and sharing are all one downward-upward walk that emits into a
hash-consing arena, and `Cse`-as-a-pass disappears. This is canon's own "one structural-hash definition,
several consumers" (`202607240100:267-268`) realized more directly than the shipped separate-pass form. It is
the LMS collapse again, on the sharing axis this time.

## What the hard data says the ideal should be oriented around (bench supremacy)

The retake-under-bench-supremacy fork is authoritative here (op's precedence rule), and it reshapes the
frontier the ideal should aim at. Three corrections matter for the idealistic shape, not just as doc fixes:

- **fold + CSE + DCE is the optimizer; the e-graph is not the frontier** (`worker-fork_retake...:27,33`;
  `RUN_SUMMARY.md` finding 6, "eqsat is OUT... marginal contribution zero-or-negative... park behind a named
  trigger"). This is the single largest reorientation, and it is GOOD news for the ideal: the one-reducer
  cheap subset (fold + macro-discharge + CSE-by-construction + DCE) IS the endorsed optimizer, so the ideal
  aims exactly where the data says value is. The round's framing of the e-graph as "the one genuinely original
  research piece" (`202607240100:101`) is the drift; both panelists inherited it (Rompf finding 2, Fallin
  open-question 1 spend design attention on wiring `Saturate`). The ideal de-emphasizes the e-graph to a
  reserved conditional seam behind a trigger and does NOT make `LowerStrategy::Saturate` the goal. Crucially,
  the ideal should add **DCE as the third always-on cheap arm** of the reducer (dead-let elimination,
  unreachable-branch drop after const-fold), which the shipped `Lower` does not have as a named pass at all,
  and which the bench names as part of the optimizer triad.
- **The runtime is a CFG-of-blocks register interpreter, and threading wins on its own control-flow path**
  (`worker-fork_retake...:26,47`; `RUN_SUMMARY.md` findings 2, 4). The shipped design dismissed threading
  ("plain switch beats threading") on a straight-line micro-bench, while the runtime's own execution model is
  a CFG with terminators and back-edges, exactly the regime where the data says threading wins (`trace` runs
  0.44x of `switch` on `carrier_cfg`). The ideal settles the CFG shape and the dispatch strategy TOGETHER,
  from the bench: a CFG-of-blocks residual dispatched by preserve-none threading on the control-flow path,
  switch inside straight-line blocks. This also answers Fallin finding C's open question in the direction the
  data prefers: `Tier::Arena` executes a real CFG (threading-dispatched), which means the tree-to-CFG lowering
  IS a real, owned pass (the reducer's final projection arm, most naturally the last thing the one reducer
  emits: blocks are the eliminator projection's control-flow-explicit form). Under the one-reducer model, CFG
  construction is not a separate unowned pass (Fallin's worry); it is the eliminator projection at the
  `Bundler`/`HostLoader` boundary, defunctionalized (Fallin weval addendum 1, Reynolds/Danvy) so the runtime
  walks an explicit block/terminator machine with no recursion.
- **Native is direct isel + predecode, not the copy-and-patch stencil** (`worker-fork_retake...:28,48`;
  `RUN_SUMMARY.md` findings 7, 8: direct isel compiles ~2x cheaper at equal warm speed; predecode dominates
  for few-run). The authoring/templating majority produces short-lived residuals, so the ideal's native point
  is direct isel, and the deferred `vehje-stencil` copy-and-patch crate is de-prioritized. The native ceiling
  is ~1.5x over the best interpreter (VINDICATED, `worker-fork_retake...:20`; Fallin's 2.0x is doubly-stale),
  which confirms native stays a per-region point, never the driver (soul 4 intact). And the batched-column
  `extern "C"` ABI is vindicated by vert8 at 4.8x, the one ABI-shaping result (`worker-fork_retake...:21`).

The through-line: the round's own hard data says the value is in the cheap binding-time-directed reducer
(fold/macro/CSE/DCE) and the vertical-SIMD batched ABI, not in the e-graph or the copy-and-patch stencil. The
ideal orients the entire frontier budget on making the one reducer excellent and the batched ABI wide, and
treats eqsat and stencils as reserved seams. This is more in spirit than the shipped taxonomy, which gave the
e-graph and the stencil crate first-class seats the data does not support.

## Termination, idealized: bounded by construction, not by a bolted-on fuel cap

The IR-to-IR topic (`202607241330`) reopened an "iterate to a fixpoint, until no change" loop that
contradicts the round's locked "no unbounded saturation" (Fallin canon-recheck, `worker-fork_retake...:58`).
The literal fix is a fuel cap (Rompf finding 1) or a node-count cap (Fallin finding E, already bench-
validated: bounded 512 e-nodes extracts identical optimal cost). The ideal is stronger: **the one reducer
terminates by construction for the cheap subset, and the cap is only the backstop for the family-macro arm.**
Reasoning: const-fold, Interp-inline, DCE, and CSE-by-construction are all size-non-increasing (they drop
branches, inline known values, share); the only size-increasing arm is macro discharge, and a macro's
`binding_time_ceiling` plus a per-operation "a macro may not re-introduce an operation of its own family at
the same or later binding time" well-foundedness condition bounds recursion structurally, the way the reach
lease's depth-cap bounds recursion elsewhere. The node-count cap the eqsat bench validated is the honest
backstop for the case the structural argument cannot cover (mutually recursive family macros a third-party
author writes), reported as a named diagnostic, not a silent arena exhaustion. So the ideal is: bounded by
construction where the framework controls the arm, capped-and-diagnosed where a family author does not, and
never "unbounded until it stops." That is exactly the round's "no unbounded saturation" spirit, realized as a
structural property of the reducer rather than a constraint bolted onto a general fixpoint.

## Macro hygiene: an obligation the discharge contract carries

Every reader flagged hygiene as the one unaddressed soundness hole in canon and impl (Rompf finding 4). The
ideal states it as part of the handler-discharge contract, not an add-on: a build-stage handler that emits
binders emits them with fresh interned names from a monotonic gensym threaded through the reducer, and
because resolve runs after discharge, capture is structurally impossible (`tiark_rompf...:38-42`). Under the
one-reducer model this is one line of the discharge contract ("generative discharge emits fresh binders"),
not a macro-specific subsystem. Catalogue the capturing-macro test now, red until it lands, per the
workspace's own discipline.

## The genuine novelty the ideal must protect

One piece of the round is not a unification or a bench-endorsed cheap path but real invention, and the ideal
must keep it central rather than let the bench-supremacy reorientation flatten it: **no-alloc bounded
multi-shot continuations** (`202607210120:92-112`, CR1). A host-lent bounded budget holds the continuation
state, a static validation proves the multi-shot footprint fits, and the well-behaved bounded-choice case is
deterministic and statically computable. This is the reach-lease / value-transfer-sink / depth-cap discipline
applied to continuations, and "no shipping effect system is known to do no-alloc bounded multi-shot this way"
(`:110`). It is the same host-lent-budget discipline as the value-transfer reserve/commit sink and the
reachability lease: one discipline, three uses (memory windows, reachability, continuations). The ideal names
that unification too (host-lent bounded budget is the framework's universal resource shape) and keeps bounded
multi-shot as the framework's headline original contribution, orthogonal to the bench-parked e-graph.

## Net: what would have been better, in one paragraph

The round discovered that effects, host-calls, macros, staging, and the compile/runtime split are one thing,
binding-time-directed handler discharge, and then wrote it down as five separately-sited mechanisms and built
four of them as stubs. The better shape is the one the round already found and then fragmented: a single
catamorphic reducer over the twelve forms that discharges each form at the earliest binding time its inputs
are known, emitting IR at a build stage (op's single-arena recursive expansion) and producing a value at
runtime, with const-fold, Interp-inlining, macro-expansion, and CSE-by-construction as arms of that one walk,
not passes; `Knowledge` thermometer-encoded so "earliest known" is well-defined and illegal binding-time sets
are unrepresentable; the signature as the one object every artifact projects from, cross-stage identity by
byte-image hash and within-stage by structural hash; the proof certifying (per-instance re-checkable), not
certified; the frontier budget aimed by the hard data at the cheap reducer (fold + macro + CSE + DCE) and the
vertical-SIMD batched ABI rather than the e-graph and the copy-and-patch stencil; the runtime a CFG-of-blocks
register machine threading-dispatched on its control-flow path; and the host-lent bounded budget named as the
one resource discipline behind memory windows, reachability leases, and bounded multi-shot continuations. The
single highest-leverage idealization is the first one: build the one reducer. It re-lands the graded-proof
soul (the binding-time axis stops being inert), it answers the panel's compile/runtime question from the
canon's own axis (the line is the lattice the reducer walks), it collapses four stubbed passes into one real
mechanism, and it is exactly where the hard data says the value is. Everything else in this document is a
consequence of, or a precondition for, that one reducer.

## Addendum: evaluation of late additions to the panel dir

Per op's instruction to re-check for new or grown files at delivery time and evaluate them. State at write:

- `worker-fork_retake-under-bench-supremacy.md` grew to 82 lines with the "Corrected bottom line and repairs"
  section (7 repairs). Evaluated and folded into this deliverable's bench-supremacy section above: its three
  authoritative corrections (dispatch/threading, eqsat-out, direct-isel-over-copy-and-patch) and the vindicated
  ~1.5x native ceiling are load-bearing to the frontier-orientation argument, and its precedence rule (benches
  over topics over changelists) is the rule I applied to reconcile the native-ceiling dispute against Fallin's
  stale 2.0x. This addition strengthens rather than changes the synthesis: it confirms the ideal's cheap-reducer
  orientation is where the hard data points and de-prioritizes the e-graph the round over-valued.
- `chris_fallin_canon-recheck-and-drift-audit.md` (284 lines) and `worker-fork_changelist-drift-and-claim-
  verification.md` (76 lines): drift-audit-oriented, their findings carried forward by the retake fork's summary
  (`worker-fork_retake...:52-58`) and the canonical-drift-mirror's ranked wounds; I cite those and do not
  duplicate. No idealistic content in them that changes the synthesis; they establish the drift the forward
  proposals here address.

If further files land after this write, they should be read against this deliverable's one-reducer thesis: the
test for any late addition is whether it strengthens, bounds, or contradicts the claim that the round's five
mechanisms are one binding-time-directed handler discharge. Nothing read so far contradicts it; the panel and
the forks converge on it from four independent directions (staging theory, compiler-backend engineering, drift
audit, bench supremacy), which is the strongest signal that it is the shape the round was reaching for.
