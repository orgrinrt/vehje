# Soul double-take: the runtime, the handler discipline, and the continuation the soul already banked

**Author lens:** runtime and interpreter implementation, delimited continuations and effect handlers compiled to
real control flow, GC and calling-convention design, no-alloc bounded control.
**Read in full:** the two `canonical_candidate_` documents; SPJ's audit; Rompf's staging deliverable and Fallin's
load-stage deliverable; the CR1 topic (`202607210120`); the check pass, the Core `Node`, the binding-time grade
(`vehje-typecheck/src/lib.rs`, `vehje-ir/src/node.rs`, `grade.rs`); and, the reads nobody in the directory made,
the two sketches that are the operational ground truth for the soul's control-flow claims:
`sketches/202607210430_sk2_bounded-multishot-handle/` and `sketches/202607210530_sk17_cfg-blocks-interpreter/`.

## One-line reading

The identity, ethos, op-calls, bench discipline, and prior-art tiering in the two candidates hold and are fit to
canonise, and SPJ's seven corrections are right; but from the runtime angle the pair canonises one thing as
settled that is operationally an open question: the "no-alloc bounded multi-shot continuation" the soul calls the
sharpest original contribution is proven only in its re-execution form (re-run a pure function per choice, sk2),
the hard half (capturing and reinstating a live delimited continuation without heap, and its interaction with the
CFG call-stack interpreter of sk17 and with the other effects in the handled body) is unbuilt, deferred as
"engineering," and the two runtime sketches that exist do not compose into it, so CR1 must enter canon carrying
its operational scope the way §5's bench numbers carry their caveats, not flat as married novelty.

## Where I agree with the panel, and where I depart

I agree with SPJ's verdict (the pair holds, after his corrections) and with all seven of his findings; F6 (the
terminator in three layers) and F7 (hygiene is soul-shaped and absent) are the two that matter most and I
reinforce both below from the execution side. I agree with Rompf's core pinning of the compile/runtime line onto
the four-point binding-time lattice (`grade.rs:105-115`); that an independent staging read reconstructed the
canon's own axis is real corroboration, and I have nothing to subtract from it. I agree with Fallin's Finding C
(the "residual is a CFG of blocks" is locked canon with no implementation and unflagged dead type surface) and
his Finding B (the IR-to-IR rebuild is a catamorphism, `fold_core`'s visitor cannot carry it); both are correct
and both bear directly on my findings.

I depart, or rather extend past all four, on the specific point none of them checked: what the runtime actually
does when a handler resumes. Rompf and Fallin both name macro hygiene as the largest open soundness item and the
CFG lowering as the largest unbuilt gap, and both are right, but they were auditing the compile side and the
residual shape. Neither opened the two sketches that stand behind the soul's control-flow claims, so neither
noticed that the runtime the soul ships has no continuation model at all, and that the novelty the soul is
proudest of is proven by a mechanism (re-execution) that is not a continuation implementation. SPJ audited the
soul documents against the record; his job was fidelity, not implementability, and he flagged (F3) that the
"four commitments are shadows of one idea" is unify-by-analogy. I sharpen F3 from the mechanism side: even the
one handler discipline SPJ grants "by construction" is one theory over three operationally distinct mechanisms,
only one of which needs the hard part, and the soul blurs that.

On the native-ceiling dispute (SPJ F2 versus Fallin F, versus the retake fork), I land with SPJ: canonise the
range with its index (roughly 1.5x to 2x for scalar code depending on interpreter form and dispatch
predictability, growing on branchy programs, 10x reserved for vectorisation), not a pinned scalar. From my angle
the dispute is close to moot for the soul: the authoring-and-templating majority the identity is built on
(`202607202330:51-57`) has a compile-heavy, trivial-execute profile (`202607240015:113-117`), so the number that
governs that majority is the interpreter's own dispatch cost, not any native multiple, and the interpreter is
the product. Native is a rare compute-bound opt-in either way, and the tier narrowing survives every number on
record. I would not spend more canon ink on the exact native multiple than on the dispatch-strategy split, which
is the one that actually prices the majority.

## Findings from the runtime and continuation angle

### 1. The soul's proudest novelty is proven only in its trivial half; the hard half is unbuilt and mislabelled "a refinement"

**The issue.** Both candidates canonise CR1 as settled, load-bearing novelty. Positive §2.10 lists "no-alloc
bounded multi-shot continuations" flat as one of three uses of the host-lent-budget shape (lines 102-106); §4.4
calls it "genuine vehje novelty" (lines 171-173); negative A2 calls it "the sharpest original contribution"
(lines 107-111). The grounding is CR1 (`202607210120:92-112`) and the sketch `sk2`, cited as WORKS
(`202607240015:104`).

I read sk2. `collectAll` (`handle.zig:13-30`) takes a comptime list of choice-domain sizes, a host-lent budget
buffer, and a comptime function `k` that maps a choice vector to a value, then enumerates every combination with
a mixed-radix counter, calling `k(&choices)` per combination and writing the result into the budget. The comptime
check `total > CAP => @compileError` (`handle.zig:20-21`) is the fail-closed budget-fit. The findings are honest
about the scope (`findings.md:32-42`): "this is the defunctionalised bounded realisation," it "does NOT capture
an arbitrary live native stack for general multi-shot," and prefix sharing "is a refinement, not a blocker."

From the continuations chair, that scope is the whole ballgame, and calling the missing half "a refinement" is
where the soul over-claims. What sk2 demonstrates is bounded exhaustive enumeration of a choice space where the
continuation is a **pure, re-runnable function supplied by the caller**. Re-running a pure prefix from the top per
choice has been the no-alloc way to do `amb`/collect-all since McCarthy; it is unconditionally no-alloc and
unconditionally "multi-shot" for any pure `k`, and it is not a continuation representation. The genuinely hard
thing multi-shot delimited continuations demand, the thing that forces either stack copying (heap) or a
segmented/CPS/defunctionalised representation, is capturing the live slice of stack between the prompt and the
operation and **reinstating** it many times without re-running the prefix. sk2 never captures or reinstates
anything; it re-executes. The "prefix-sharing DFS with an explicit fixed-capacity stack" the findings wave at
(`findings.md:38-42`) is not an efficiency tweak on top of what was proven; it is the actual continuation-capture
problem, unproven, and it is where all the difficulty lives.

**Why it matters operationally.** Three concrete bills the re-execution proof hides:

- Cost. Re-execution is O(combos times prefix-length); the whole handled computation re-runs per choice
  combination. For the bounded-search patterns CR1 names (backtracking, collect-all over bounded spaces) that is
  quadratic-ish in the depth of the choice tree, exactly the cost prefix sharing exists to remove. A soul that
  canonises this as the resource-discipline win should say it costs re-execution, not present it beside the
  reserve/commit sink and the reachability lease as if all three were the same cheap shape.
- Soundness under other effects. Re-execution is only sound when the re-run prefix is pure. sk2's `amb` body is
  pure. A general handled body can perform other operations (a runtime host-effect, an I/O), and multi-shot
  resume then re-runs those operations, which is the standard reason production effect systems (OCaml 5) make
  resumption one-shot by default. The interaction between multi-shot `Handle` and the other effects in its body
  is an unaddressed soundness question, not an engineering follow-on. The check pass cannot even see it yet:
  effect subtraction for a handled operation is a FIXME (`vehje-typecheck/src/lib.rs:301-303`).
- Generation from IR. The findings call "generalising from the hand-written `program` continuation to one
  COMPILED from the IR" engineering, not a feasibility gap (`findings.md:35-37`). Generating a pure, re-runnable
  defunctionalised continuation from arbitrary handled IR (with its own binders, its own effects, its own nested
  handlers) is not obviously feasible and is certainly not proven; the `Handle` form itself defers the
  operation-and-resumption encoding entirely (`vehje-ir/src/node.rs:138-143`, "defers the operation-and-
  resumption representation until the bounded-multi-shot continuation encoding lands").

**Direction.** Canonise CR1 with its scope, in the shape §5 already uses for bench caveats: bounded multi-shot by
re-execution of a pure defunctionalised continuation is proven no-alloc and is the well-behaved case; bounded
multi-shot with prefix sharing (continuation reinstatement without heap) and multi-shot in the presence of other
effects in the handled body are open, and the latter is a soundness obligation, not an efficiency refinement. Add
the red test now per the catalogue discipline: a multi-shot handler whose body performs a second effect, asserting
the check either refuses it or the design states the replay semantics. This is the soul's own "one shape does the
work of many, but count the bounds honestly" ethos (positive §2.13) applied to its own novelty: count the
continuation cases as two, one proven and one open, not one married.

### 2. The runtime has no continuation model, and the two control-flow sketches do not compose into CR1

**The issue.** The runtime execution model the round settled is the CFG-of-blocks interpreter, sketched in sk17.
I read it. `cfg.zig` is a register machine over straight-line blocks with control at terminators: branch, loop
back-edge, and call/ret over a fixed-capacity call stack, `Frame = { regs, ret_block, ret_reg, dst_reg }`
(`cfg.zig:17`, `findings.md:16-18`). There is no `Handle`, no effect operation, no prompt, no resumption, no
delimited region anywhere in it; `ret` "pops and resumes the caller" is ordinary subroutine return, not effect
resumption. It is a correct standard call-stack interpreter and nothing more.

So the runtime, as it actually exists in sketch form, is two disjoint pieces that do not meet: a call-stack CFG
interpreter that knows nothing of continuations (sk17), and a multi-shot proof that re-runs a pure comptime
function and never touches a runtime frame (sk2). To get a user-defined resumable multi-shot handler running over
an arbitrary IR body inside the CFG interpreter, the interpreter's frame stack would have to be capturable and
reinstatable as a delimited segment (the classic prompt-to-operation slice), which is the segmented-stack or
CPS-conversion work, and it is in neither sketch. Fallin's Finding C already shows the residual has no CFG builder
at all yet (`Block`/`BlockTable`/`Function` are unconstructed dead type surface); my finding is one layer past
his: even the interpreter sketch that does model a CFG has no seam where a continuation could be reified. This is
the operational content of SPJ's F6 (the terminator is under-specified) pushed onto the runtime: the round settled
"the interpreter is a CFG of blocks" and "Handle supports bounded multi-shot," and the two settled things have no
shared frame model.

**Why it matters.** This is the load-bearing gap the soul's "one handler discipline" thesis rests on and does not
name. The type-level unification (SPJ grants it by construction, one soundness clause) is real. The
implementation is three different mechanisms: a compile-time handler emits IR into a `Builder` (macro expansion,
Rust-side, Rompf addendum finding 3); a runtime host-call handler returns a value once and needs no continuation
(Zig-side); a user-defined in-language handler resumes a continuation, possibly many times, and is the only one
of the three that needs the hard machinery. The soul presents "one handler discipline" as if it were one
mechanism (positive §2.5, §8's one-sentence soul, lines 362-364). It is one typing story over three
operationally distinct discharges, and the third has no runtime home yet. That distinction is exactly the kind
the round's own novelty audit demanded ("a filing system gives you one vocabulary; it does not give you one
theorem," quoted at positive §9.A): one vocabulary for handling is not one implementation of resumption.

**Direction.** State in canon that the handler discipline is a type-level unification with three implementation
discharges (compile-time IR emit, runtime value-return, runtime resumable continuation), and that only the third
needs a continuation representation, which is the open piece. Name the continuation representation as an explicit
design obligation on the CFG interpreter: either the frame stack gains a delimited-segment capture-and-reinstate
operation (segmented, no-alloc, host-lent budget per CR1), or the handled body is CPS/defunctionalised at compile
time so resumption is a data structure the interpreter walks rather than a captured native frame. This is the
deep unblocking piece: every generator, coroutine, event, and resumable-error consumer in the census leans on it,
and its absence taxes every runtime-side effect feature. It should be the next sketch, and it should be built
before the `Handle` clause encoding is locked, because the encoding is downstream of which representation wins.

### 3. Hygiene and the terminator are one discipline, and both are runtime-adjacent, not just compile-side

**The issue.** Rompf finding 4 and SPJ F7 both name macro hygiene as the largest open soundness item, and both
are right. I add the connection to my findings 1 and 2: hygiene (fresh binders so capture is structurally
impossible because resolve runs after expansion) and multi-shot resumption (fresh continuation state per resume so
one resumption cannot corrupt another) are the same discipline, generative freshness, at two binding times. The
soul already contains "illegal states unrepresentable" and "strict by design"; both hygiene and multi-shot-frame
freshness are that conviction applied to generative discharge. Stating them as one discipline (positive §2 should
carry it beside the handler discipline, per SPJ F7) also fixes the terminator (SPJ F6): binding-time
well-foundedness terminates the expansion unfold, and the same freshness-plus-budget discipline bounds the
resumption count; a mis-graded family macro and an unbounded resumption are the same failure (a generative step
that does not reduce), diagnosed the same way (a named error at budget exhaustion, distinct from passthrough).

**Direction.** One canon sentence: generative discharge (macro expansion at build time, continuation resumption at
runtime) is fresh by construction and bounded by a host-lent budget, and exhaustion is a named diagnostic
distinct from a completed expansion or a completed resumption. That single statement closes SPJ F6, SPJ F7,
Rompf finding 4, and my findings 1 and 2 at once, because they are one discipline seen at four sites.

## Judgement on fitness

The two candidate documents are fit to base a canonical identity document on. They are faithful to the latter
topics and to the hard data; the commitment-tier device is the right shape; the negative fork's kill catalogue
and the positive fork's ethos and op-calls are well sourced; and SPJ verified the load-bearing citations against
source. My angle does not overturn that verdict. It adds one condition to SPJ's seven corrections: CR1 must not
enter canon as flat married novelty. Both candidates currently overstate it (positive §2.10, §4.4; negative A2),
and the operational record does not support "settled." The fix is small and is the same move the documents already
make everywhere else: carry the scope. Bounded multi-shot by re-execution of a pure defunctionalised continuation
is proven; continuation reinstatement without heap, and multi-shot under other effects, are open. With that one
scoping applied, and SPJ's seven, the pair is the right foundation. The one-sentence soul (positive §8) survives
my read as it survived SPJ's and both forks'; my only change to it is that "one handler discipline" should be read
as a type-level unification, not an implementation one, which is a gloss on §8, not a wound to it.

## Open questions handed forward to the synthesis

1. The continuation representation for the CFG interpreter: segmented capture-and-reinstate over a host-lent
   budget, versus compile-time CPS/defunctionalisation of the handled body into a data structure the interpreter
   walks. This is the deep unblocking piece and should be the next sketch, before the `Handle` clause encoding
   (`vehje-ir/src/node.rs:138-143`) is locked. Bench-decidable once both candidates exist (per the workspace's
   bench-decided-fork rule): measure capture cost, resume cost, and the ABI shape each forces.

2. Multi-shot resumption under a non-pure handled body: is it refused by the check (a handled body carrying an
   un-handled runtime effect cannot be multi-shot), or is replay semantics defined. Either answer is a canon
   statement and a red test (`vehje-typecheck/src/lib.rs:301-303`, the handled-effect subtraction FIXME, is where
   it lands).

3. CR1's canonical wording: the two continuation cases counted honestly (re-execution proven, reinstatement
   open), in the shape §5 uses for bench caveats, so a future reader does not treat the sharpest contribution as
   more built than it is.

4. Whether the handler discipline is stated in canon as one type-level unification over three implementation
   discharges (compile-time IR emit, runtime value-return, runtime resumable continuation), so the third is not
   assumed to inherit the first two's feasibility.

5. Carried from SPJ and unchanged by me: the superseding topic for op's 2026-07-24 live calls closing the
   contradiction with `202607241330` (SPJ F5); the terminator in three layers (SPJ F6), which my finding 3 folds
   into one generative-freshness discipline with hygiene; the hash-identity boundary (SPJ F8, Rompf addendum
   finding 0); and the native-ceiling reconciliation done as a measurement, with canon carrying the range until
   then (SPJ F2).
