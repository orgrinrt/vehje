# Counter-audit: attacking Cluster C's conclusions, with answers and weak points

**Date:** 2026-07-20
**Phase:** research (a counter-audit of the honest-keeper capstone
`202607202250_honest-keeper-capstone-and-diagnostics.md`)
**Task:** attack Cluster C's conclusions specifically. For each hole it raised, find the answer from recent
research, prior art, or the round's own canonical design; and find the weak points in C's arguments where it
overstates a solved problem or reaches for new design work that prior art already supplies. Reads: C, Cluster A
(`202607202210`), Cluster B (`202607202230`), the canonical certified-generation topics (1627, 1845), the
converged shape (2001), and the identity recenter (`202607202330`).

## Verdict

C is a good adversarial reader and three of its catches are real: A's written `Reach` rule is unbounded, A's
"by construction" over-grades a two-backend divergence surface, and B's interpreter is described only for
straight-line dataflow. But C consistently stops at "this is undesigned new work" where the answer already
exists, either in the prior art it did not reach for or in this round's own canonical resolution that A and B
drifted from. Two of its five headline findings dissolve rather than become work: the dual-locus divergence is
an artifact of A departing from the 1627/1845 comptime-specialised-single-engine shape, and returning to that
canonical shape closes it by construction; the interpreter control-flow gap is answered by the Core control
forms that already exist plus textbook structured dispatch, not by new invention. One finding (the `N*W` bound)
is a real bug whose fix C gets half-right, missing the prior-art binder rule and the second bound that no_alloc
tractability actually needs. One (the `preserve_none` risk) is grounded against the wrong Zig version and
overstates the necessity of a convention the winning design does not require. Part 2 (diagnostics) is C's
strongest section and largely survives. Part 3's "the debate secretly adopted Carmack" ledger is correct on the
facts but is superseded by the identity recenter, which reframes the whole ledger as a category error.

## Finding 1: the `N*W` reachability bound. Real bug, half-right fix.

C is correct that A's rule `Reach(p, b) :- Child(p, _, c), Reach(c, b)` is unbounded as written: a binder bound
inside a child's subtree propagates upward, so a node accumulates binders from its whole subtree, `O(N)`, not
`W`. That catch stands.

C's proposed fix, "add the `InScope(p, b)` filter and route the killed-but-still-reached binder into `Escapes`,"
is on the right track but under-specifies the mechanism and, more importantly, only addresses one of the two
bounds no_alloc tractability needs. The prior art that supplies the complete rule is reachability types
themselves (Bao, Wei, Bracevac, Jiang, He, Rompf, "Reachability Types," OOPSLA 2021, and the polymorphic
successor, Wei et al., POPL 2024), which the converged shape already cites as its lease paradigm. In that
calculus the reach set of `let x = e1 in e2` is not the naive union of children's reach sets. It is the reach
set of `e2` with `x` eliminated by substitution: where `e2` reaches `x`, splice in `e1`'s reach set, and drop
`x` itself, because `x` is not observable outside the let. That is the avoidance operation, and it is the
binder rule that both A and C omit. A has no binder rule at all, so it is unbounded. C's fix adds a filter on
the propagation but still states no binder-substitution rule, so as literally written it drops `x` without
splicing `e1`'s reach at the let, which under-approximates the reach set and is unsound in the opposite
direction from A (a value that reaches through `x` into a still-live binder would lose that reach and be judged
safe when it is not). The correct rule fires at the `Let` and `Lambda` node, computes the spliced-and-dropped
reach set there, and then plain child-propagation plus C's `InScope` filter carries the already-scoped set
upward. So the fix is: the reachability-type binder rule at binder nodes (prior art), not a filter on the
generic propagation.

The deeper weak point: the `N*W` bound is only half the tractability story, and C treats it as the whole story.
`N*W` bounds the reach naming set (in-scope binders per node, width at most `W`). It says nothing about the
escape set, the out-of-scope binders a value reaches that must be resolved into regions. That set is what
`Escapes`/`LeaseResidual` carry, and its size is not bounded by `W`; a deeply nested value can reach many dead
binders. no_alloc needs that set bounded too, and the bound comes from region promotion, not from the naming
filter: Tofte-Talpin region inference bounds the number of live regions by the program's let and region
structure, and the converged shape's own shared-implies-promoted discipline (2001) collapses escaping shared
subvalues into a bounded promoted-region set finalised at emission. So the complete answer is two bounds from
two prior-art mechanisms: the reachability binder rule bounds the naming set to `W`, and region promotion
bounds the escape set to the region count. C found the bug, proposed a filter that establishes only the first
bound, and never noticed that the second bound is the one its own `Escapes` routing depends on. The fix is
small, as C says, but it is the binder rule plus the promotion bound, not a filter.

## Finding 2: the dual-locus "by construction" identity. Real, but it dissolves into the canonical design.

C's sharpest correct insight is that A's default (a generator emitting specialised semi-naive loops as Rust
source and as Zig source, two backends compiled by two toolchains with independent overflow, evaluation-order,
and UB behavior) makes "by construction" mean deterministic divergence, not agreement, so the differential test
is the real assurance and only the single-shared-object variant truly closes the gap. On the two-backend shape,
C is right.

The weak point is that C treats the single-shared-object variant as an exotic "assurance-maximal fallback"
reached only "if a formal identity guarantee is demanded," when it is in fact this round's own canonical
certified-generation shape, and A drifted from it. Topic 1627's load-bearing resolution, endorsed by the panel
in 1845, is explicit: "Zig's comptime is the specialiser and the certifier, not a Rust metaprogram emitting Zig
source. Rust emits validated data (the family table, effect masks, lease-rule schema, wire layout) as Zig
comptime consts. The general engine is a comptime-parametrised Zig metaprogram that specialises to that data."
That is one hand-authored engine, specialised by Zig comptime to Rust-emitted data, with a single backend. For
the dev-time Rust locus to run the same analysis on bundled scripts, it links the compiled Zig engine over FFI,
which is precisely A's single-shared-object variant. So the divergence-free shape is not a fallback; it is the
canonical resolution, and it is less engineering than A's default (author the engine once, do not maintain two
specialised source emitters). A's "Rust generator emits specialised Rust and specialised Zig source" is itself
the drift the identity recenter warns about: it re-imported the LMS-hard "Rust metaprogram emitting foreign
source" path that 1627 explicitly rejected. The answer to C's finding is therefore not "make the differential
harness a merge gate for a two-backend design"; it is "return to the canonical single-engine comptime-
specialised shape A drifted from," which makes by-construction genuinely mean the same function because it is
the same object. C found a real problem, then prescribed a mitigation for a shape the round already decided not
to build, while the round's own settled design dissolves the problem. The differential harness remains valuable
as defence in depth, but it is not the primary assurance the design is forced to lean on.

## Finding 3: the interpreter control-flow gap. Real omission in B's prose, overstated as "the single largest hole."

C is right that B's `for n in 0..len { dispatch(node[n]) }` describes only a pure expression DAG and is silent
on branches, loops, and script-internal calls, and that the pure forward scan would evaluate both arms of a
conditional (wrong under effects) and cannot iterate a loop. B under-specified this.

The weak point is that C frames a textbook, cheap, already-scaffolded problem as "the single largest hole" and
"undesigned." The design substrate is already present: vehje's eleven Core forms include `If`, `Match`, `Iter`,
`Interp`, and `Apply`, so control flow is not an unmodeled surprise, it is first-class Core the interpreter
dispatches specially. The standard answer, in every bytecode and tree-walking interpreter since the 1970s, is
structured dispatch over those forms. The forward linear scan is the within-basic-block fast path (a
straight-line dataflow region, which is where the children-before-parents arena invariant and the cache story
hold), and control forms redirect the cursor at block boundaries: `If`/`Match` evaluate the scrutinee then set
the cursor to the taken arm's node range and skip the untaken range (so both arms are never evaluated, and the
effectful-branch bug C names is a bug in B's description, not in the arena); `Iter` re-runs a bounded node range
under a loop counter with a back-edge; `Apply` pushes a return cursor on a small bounded control stack. This is
the classic "basic blocks connected by explicit control, straight-line dataflow within each block" shape, and
the tail-threaded dispatch B chose composes with it perfectly: once the successor is a computed cursor rather
than `n+1`, the tail call is a jump to an arbitrary handler, which tail-threading already handles. C even
concedes this in a subordinate clause ("tail-threaded dispatch over an arena is still the right shape") and then
still ranks it the largest hole. The honest weight: the mechanism is standard and cheap, the forms already
exist, and the real owed work is spelling out the control-form dispatch and, as C correctly notes in its
strongest sub-point, re-deriving the per-frame budget on dynamic executed node counts rather than static node
counts. That budget correction is C's genuine contribution here; "the single largest hole" is inflation.

## Finding 4: the `preserve_none` risk. Grounded against the wrong Zig version, and overstated in necessity.

C fetched the Zig 0.14.0 release notes, found no `preserve_none` in the reworked `CallingConvention`, and
concluded B's register-pinned dispatch "does not hold on stock Zig 0.14.0" so "much of the win evaporates."

Two weak points. First, the grounding is against the wrong target: topic 1845 pins the project to Zig 0.16.0
("Zig pins to 0.16.0"), not 0.14.0, and 0.14.0 is precisely the release that made `CallingConvention` an
extensible tagged union, so a later release surfacing more conventions is the expected trajectory, not a
surprise. C's specific "absent in 0.14.0" finding is measured against a version the design does not use.

Second, and more important, the design does not need `preserve_none` to get the Deegen result, so "much of the
win evaporates" overstates. The win in tail-threaded interpreters comes primarily from making each op its own
function ended by a guaranteed tail call, which lets the register allocator optimise each handler independently
so a cold op cannot spill the hot path (Haberman's protobuf-parsing-at-2GB/s result, 2021, is driven by
`[[clang::musttail]]` with the hot state passed as explicit arguments, and predates the widespread
`preserve_none`/`preserve_nonecc` convention, which only landed in Clang and LLVM 19 in 2024). Zig's
`@call(.always_tail)` is exactly the guaranteed-tail-call primitive, and if the three walk pointers (node,
results, blob) are passed as the first three explicit arguments of each handler, they occupy argument-passing
registers under every mainstream ABI (six integer arg registers on SysV, eight on AAPCS), and a tail call is a
jump that leaves those argument registers in place, so the hot state stays register-resident across dispatch
under the plain C convention. `preserve_none` is an incremental gain on top: it frees the callee-saved registers
for additional state and drops the callee-save prologue and epilogue. It is worth having and worth the upstream
ask (per the fix-the-stack-upstream discipline), but the register-pinning of the three core pointers, the part
C says evaporates, is already achieved by `@call(.always_tail)` plus explicit-argument passing, which stock Zig
supports today. So B's dispatch is not the tool-gated aspiration C makes it; its core is buildable on stock Zig
now, and `preserve_none` is the optimisation tier, not the foundation. C sided with B's cautious open question
over B's verdict; the honest position is between them, the foundation holds and the optimisation is the ask.

## Finding 5: "the engine remit is narrower than three queries." A relabeling, not a defect.

C treats the structural decode being a linear front-end rather than an engine query as a "narrowing" that
undercuts "the grounded stack's rhetoric." This is the weakest of C's five: every relational engine has a
trusted front-end that materialises the extensional database from input, and parse-don't-validate over untrusted
bytes is exactly that front-end. Naming it and placing it in the loader is the correct architecture, not a
concession that shrinks the engine. The engine's remit (lease inference, eqsat, and the load residuals, all
genuine fixpoints) is the right remit, and A stated it plainly. C is describing standard database architecture
as if it were a retreat. The one substantive thing under this heading is C's correct catch on phrasing: for
untrusted input, children-before-parents is checked by the decode (a complete linear validation), not assumed
"by construction," and A's "by construction" wording for the arriving arena is wrong. That phrasing fix is
worth making; the "narrowing" framing around it is not.

## Part 2: diagnostics. C's strongest section; one refinement, no attack.

The lazy-witness-reconstruction design (provenance carried cheaply on the happy path, the derivation witness
reconstructed on demand over resident tables on the failure path, bounded by derivation depth) is correct and
well-grounded in Souffle's on-demand proof-tree provenance and the provenance-semiring how-provenance polynomial
(Green, Karvounarakis, Tannen, 2007). The six schema additions and the provenance-complete engine ABI are sound
and this is the part that should land in the wire format as C says. One refinement rather than an attack: the
single back-pointer per tuple assumes each rule has one recursive premise, which holds for the reach rules but
not for a join rule with two recursive premises (C notes the record widens to two `premise_key` slots); worth
stating explicitly that the reconstruction then forks and the bounded path buffer becomes a bounded path tree
sized by depth times max-recursive-arity, still finite, still failure-path-only. Minor. Part 2 stands.

## Part 3: the demotion ledger. Factually right, but superseded by the identity recenter.

C's Part 3 concludes the debate held the maximalist line on only about one-and-a-half of Carmack's seven and
adopted the re-tiering on the other five while calling it maximalism, so the "we reject the demotion" framing is
"theatrical." On the raw facts (the logical relation is a north star nothing waits on, AARA is three constants,
the semiring is scoped to the proof, the runtime parser is a table, the speed layer is struck) C is right, and
the arc did over-claim "designed through everything."

But C's ledger is itself trapped in the win-or-loss frame that the identity recenter (`202607202330`, written
after C) dissolves. op's correction is that those five were never ambition-demotions to reject or adopt; they
are correct engineering placement that a maximal design simply includes, and the debate's real error was neither
"rejecting Carmack" nor "secretly adopting him," it was letting the runtime-performance and native corner read
as the whole identity. So scoring the seven as a reject-or-adopt tally, which both the agenda and C do from
opposite ends, is the category error. C's honesty note (do not let a reader think all seven were refuted)
remains a fair rigor correction; its "theatrical" verdict is a truer description than the agenda's "we reject
all seven," but it is still the wrong axis. The right axis is: the engine's buildability is the genuine win, the
placement corrections are just correct placement, and the identity of the whole is the framework serving a
spectrum of consumers, of which the native and JIT corner is one part. C could not have known this; the recenter
postdates it. But a reader should take Part 3 as "the arc over-claimed refutation," not as "the maximal shape
conceded," and should read the whole ledger through the recenter that supersedes it.

## Two further weak points in C worth naming

C leans on PEP 744's "CPython's own copy-and-patch JIT is currently about as fast as the specialising
interpreter" as evidence the vehje native tier "may come back marginal." This overstates a fair "must bench"
into near-evidence. CPython's JIT in 3.13 is an explicitly first-generation baseline with no inline caches and
no type feedback yet, and PEP 744 itself frames those optimisations as the future work that makes it fast, so
citing its current speed is comparing against a deliberately unfinished JIT. It cuts both ways for vehje (a
statically-checked flat IR means the interpreter is already leaner, so less to gain, but also the JIT has less
to do, so native speedups on the arithmetic that dominates genuine hot loops are cheaper to reach). The correct
statement is C's own "must bench," without the thumb on the scale.

C's "arity-at-most-two is plausibly wrong because scripts are call-heavy" is a reasonable prompt for the bench,
but it slightly misreads the record layout: even a call carries its callee reference and first argument in the
two inline operands, and argument sub-expressions are themselves mostly low-arity arithmetic that the inline
operands serve. The point is genuinely bench-decidable (C agrees), so it is a measurement to run, not a hole in
the design, and C correctly defers it; the framing as a near-refutation of the 16-byte choice is stronger than
the deferral warrants.

## Concessions: where C is right and the design owes work

Three of C's catches are real and the design is better for them. A's written `Reach` rule is unbounded and must
carry the reachability-type binder rule plus the region-promotion bound (Finding 1). A's "by construction" over
a two-backend emitter is deterministic divergence, and the resolution is to return to the canonical single-
engine comptime shape rather than to ship two specialised emitters (Finding 2). B's interpreter must spell out
control-form dispatch and re-derive the per-frame budget on dynamic, not static, executed node counts (Finding
3). The decode's "by construction" phrasing for untrusted input must become "complete linear validation." And
the differential harness is worth keeping as defence in depth even once the single-engine shape makes identity
hold by construction. C earned those.

## Net

C is a competent audit that correctly locates where the design is unfinished, but it reaches for "new,
undesigned work" at points where the prior art (reachability-type binder rules, region-promotion bounds,
structured control-form dispatch, musttail-with-explicit-args) or this round's own canonical resolution
(comptime-specialised single engine) already supplies the answer. Its two headline structural findings do not
become new arcs of work; one is closed by a prior-art rule A omitted, the other dissolves by returning to the
1627/1845 shape A drifted from. Its interpreter and `preserve_none` findings are real omissions in B's prose but
are standard and buildable-on-stock-Zig respectively, not the tool-gated near-impossibilities C's phrasing
implies. Its diagnostics design is genuinely strong and should land. Its Part 3 is factually accurate and
usefully honest about over-claiming, but its scoring frame is superseded by the identity recenter. The net
correction to C: the design is closer to whole than C's five-surviving-problems tally suggests, because three of
the five are answered by things already in the prior art or the canonical design rather than by work not yet
done.

## Sources and prior art

Reachability types and the binder/avoidance rule: Bao, Wei, Bracevac, Jiang, He, Rompf, "Reachability Types,"
OOPSLA 2021; Wei et al., "Polymorphic Reachability Types," POPL 2024. Region-count bounds: Tofte and Talpin,
region inference. Datalog borrow/reach inference: Polonius over Datafrog (heap-backed, as A notes). Tail-call
interpreter dispatch without a preserve-none convention: Haberman, "Parsing Protobuf at 2+GB/s: musttail," 2021
(musttail plus explicit-argument register residency); `preserve_none`/`preserve_nonecc` is Clang and LLVM 19,
2024, an incremental gain. Generated tail-threaded interpreters beating hand-written assembly: Deegen (LuaJIT
Remake), arXiv:2411.11469. Provenance: Souffle on-demand proof-tree provenance; Green, Karvounarakis, Tannen,
"Provenance Semirings," PODS 2007. Certified generation via Zig comptime as the single-engine shape: this
round's topics 1627 and 1845. The Zig pin (0.16.0, not the 0.14.0 C measured against) and the eleven Core forms
including `If`/`Match`/`Iter`/`Apply`: topics 1845 and the framework identity in `.claude/CLAUDE.md`. PEP 744
(CPython 3.13 baseline JIT, first-generation, interpreter-speed today with the optimising work named as future)
qualifies but does not settle the native-tier bench.
