# Meta-attack: auditing the adversarial audit of the full-pipeline proposal

**Date:** 2026-07-20
**Phase:** panel continuation (worker-fork audit, reports once, settles nothing)
**Role taken:** a novelty-driven computer-science theorist whose method is unconventional synthesis across fields
not normally considered related. The mandate is to attack the adversarial audit itself
(`202607202038_adversarial-attack-on-the-full-pipeline-proposal.md`), not to defend the original proposal
(`../design_rounds/202607202001_topic.converged-architecture-shape.md`, the full-pipeline section). Where the
attack is right I say so and push past its construction; where the attack mis-chose its own abstraction or its
fix reintroduces the problem it escaped, I take it apart and build better. Every item compares my result
directly to both the original and the attack.
**Reads:** the attack in full, the proposal section, the five continuation docs 01 to 05, the synth docs.

## One-paragraph verdict

The attack is a strong demolition and most of its negatives are correct: assurance is not a clean total
projection (A2), the stencil leg of the triple is mislabelled (A5), tnum is non-relational and the load cost is
width not depth (A6), shared-implies-promoted reintroduces the region leak (A7, the sharpest catch), and packrat
memo is not depth-bounded (A4). But the attack has one systemic weakness a novelty reviewer is built to find:
its constructions retreat to the established shape the moment an attack lands, and four of them reintroduce the
very problem they were escaping. C2 (split PE out) trades divergence for the phase-ordering problem eqsat exists
to kill. C3 (copy all leased shared values) trades the leak for exponential blowup on exactly the huge nested
values the design targets. C4 (default to GLL/Earley) trades PEG's silent shadowing for undecidable ambiguity.
And on its own flagship (A1) the attack mis-chose the abstraction: it demands a Beck distributive law for a
phenomenon that is graded read/write dataflow, then cedes the general case (C1) it did not need to cede. The one
place the attack is genuinely visionary (C5, the graded logical relation) it undersells as an aspiration when it
is the load-bearing unification and subsumes the differential tests as its own grade-0 shadow. Below, per item:
the attack's claim, where it or its fix fails, and the better composition, each compared to original and attack.

## Meta-attacks, with constructions

### On A1 (the distributive law): right that the original was sloppy, wrong about the abstraction

The attack correctly kills the original's conflation of an adjunction with a distributive law. But it then imposes
its own wrong frame: it insists the effect-monad and reachability-comonad must combine via a Beck distributive
law `D T -> T D`, proves such laws are not general, and cedes the mutable case to a dynamic residual (C1). That is
mis-abstraction. The phenomenon "does op X retain operand i through the host boundary" is not a monad-comonad
interchange; it is a read/write dependency: reachability is a read (a coeffect, what the value depends on),
retention is a write (an effect, what the op does to host-retained state). A read followed by a write on the same
location is sequential composition, not distribution.

The better composition (cross-domain: categorical graded semantics plus dataflow def-use). Model the interaction
in a single bi-graded structure where a morphism carries both a coeffect grade (in) and an effect grade (out),
composed by ordinary morphism composition, so no distributive law is ever formed. Two concrete, defensible
settings: a graded Freyd category (effects and coeffects as the two projections of one graded morphism), or,
more elementarily, a graded state monad where the host boundary is a graded location, so "retains operand i"
is a graded-state write that depends on operand i's graded read, a def-use fact in one graded-state setting. The
lease-from-effect derivation is then the def-use relation of a graded state pass, which always exists, covering
the general mutable case the attack ceded. "Free to Move" (2025) is then one model of this structure, not the
boundary of where it is defined.

- Versus original: the original's "the adjunction derives the bit" was hand-wave; this names the actual structure
  (graded state/Freyd) and why it needs no distributive law.
- Versus attack: the attack's distributive-law framing is a strawman that forces a retreat to dynamic-elsewhere;
  the graded-state framing keeps the general case static where the dataflow is defined, dynamic only where a genuine
  cyclic def-use makes the reach-set imprecise, which is a smaller residual than C1's whole-mutable-fragment cede.

### On A2 (assurance not total): right, but the fix is worse than the disease

The attack is correct that semantics (always tested) and termination (outside the system) are fixed independent of
binding time, so the assurance map is not the clean total projection the original sold. But its fix, "make it a
partial map," is the wrong repair, because the entire point of assurance-as-a-grade (H8) is that the guarantee
inventory is computed by composing grades, and partial maps do not compose (a partial map into a partial map is a
domain-tracking nightmare).

The better composition: keep the map total by enriching the codomain. The assurance lattice gains two absorbing
elements, a "tested" fixed point and an "unclaimed/out-of-system" bottom; semantics maps to the tested absorber,
termination to the bottom, and every decidable axis maps by the binding-time projection. A total map into a
lattice with absorbing elements composes cleanly, which is exactly what the mechanised inventory needs.

- Versus original: the original claimed a total projection onto a plain lattice, which is false for two axes.
- Versus attack: the attack's partial map is honest but breaks composition; the enriched-lattice total map is
  both honest and composable, which is strictly better for the mechanisation A2 was trying to protect.

### On A3 (PE as e-graph extraction): the attack's best negative, and its fix reintroduces phase-ordering

The attack's three sub-points are correct and this is its sharpest negative: extraction is a preference not a
well-formedness constraint, the mix equation is an equation not an optimisation, and saturating an inlining rule
diverges so a no-alloc cap makes extraction saturation-order-dependent and non-deterministic. All true. But C2's
fix, "do type-directed PE as a separate pass, then eqsat the confluent residual," retreats to the classic
two-phase compiler and thereby reintroduces the phase-ordering problem that equality saturation exists to solve:
PE and CSE and algebraic simplification interact, and sequencing them forecloses optima, which was the whole
reason to want one stage.

The better composition (cross-domain: online-PE termination theory plus modern e-graphs-with-binders). Keep one
graded rewrite stage, but ground its termination in the right place. PE terminates not because of a node cap but
because the static values are finite and the binding-time analysis is well-founded (Jones-Gomard-Sestoft). Cast
that well-foundedness as the coeffect/grade bound: the unfold-and-inline rule is graded, fires only under the
static binding-time grade, and terminates because the static fragment is a well-founded order, not because a cap
cut it off. Then determinism comes from confluence of the meaning-preserving rules over the static fragment, not
from saturation order, so the no-alloc bound never makes lowering non-deterministic. The binding-time grade prunes
the graph as a typing constraint (an ill-staged term is unrepresentable, honouring 1627), and eqsat runs over the
confluent optimisation identities in the same graph, with slotted/colored e-graphs (2024 to 2025 work on e-graphs
with binders) handling the variable-capture problem classical eqsat could not. Partial evaluation is a
well-founded graded rule inside the shared graph, not a heuristic and not a separate pass.

- Versus original: the original called cost-optimal extraction "partial evaluation," which it is not; this makes
  PE a well-founded graded unfold that actually preserves meaning and terminates by grade, not by cap.
- Versus attack: C2 kills the divergence but pays the phase-ordering tax; the graded-well-founded-unfold keeps one
  stage (no phase ordering) and gets determinism from confluence-on-the-static-fragment, so it beats C2 on the
  very axis (phase ordering) C2 silently sacrifices.

### On A4 (PEG and packrat): the packrat catch is right, both the PEG critique and its GLL fix miss the vehje-native answer

The attack is correct that the packrat memo is O(position times rule), a width-and-length quantity the depth cap
does not bound; that puncture stands. But on grammar composition, both the attack (PEG ordered choice does not
compose) and C4 (default to GLL or Earley) miss the target. PEG's ordered choice silently shadows across
independently authored families; GLL/Earley's general context-free union is ambiguous, and disambiguating a
general CF union is undecidable in general, so C4 trades silent shadowing for silent ambiguity. Both are the wrong
default for a metacompiler whose premise is independently authored families.

The better composition (the design's own axis, applied to syntax). Grammar composition is inclusion-not-coverage,
the same discipline as `Supports`/`Permits` everywhere else in vehje. Each family declares its syntactic surface;
a conflict between two families' rules is a declared-and-detected compile error, never silent shadowing (PEG) and
never silent ambiguity (GLL). The mechanism can be Brzozowski derivatives over a family lattice (parsing with
derivatives composes because derivatives are compositional) or a scannerless modular formalism (the SDF3 lineage)
with explicit conflict detection. The parser's resource bound is stated in input length and grammar size, its
real complexity, and its working memory is a host-lent capability like every other buffer.

- Versus original: the original defaulted to scannerless PEG and wrongly claimed the depth cap bounds the memo.
- Versus attack: the attack swaps PEG for GLL and inherits ambiguity; inclusion-with-conflict-detection is the
  vehje-native answer that makes a cross-family syntax clash an error, which neither PEG nor GLL gives, and it
  reuses the inclusion axis already proven elsewhere in the design.

### On A5 (the three projections): right that the stencil leg is mislabelled, wrong to drop it

The attack correctly catches that stencils are mostly compiled eliminators (field access, arithmetic, calls),
not introduction forms, so "stencils equal introduction-compiled" is false. But its fix, "drop the stencil leg,"
discards the load-bearing insight the triple was reaching for: that all views are generated from one signature
and therefore cannot drift.

The better composition: one signature, three functors, of which two are total and one is a restriction. The term
functor gives the IR (the whole algebra as terms), the compile functor gives the stencils (the whole algebra
compiled, eliminators included), and the value-domain functor is the introduction-fragment restriction quotiented
by sharing and reachability. Not a symmetric triple, but a single algebraic signature with three functorial
views, which preserves the real prize (generation-from-one-source prevents drift) while being honest that the
compile functor is total over operations and only the value functor is an intro restriction.

- Versus original: the original's pleasing symmetry does not survive what a stencil is.
- Versus attack: the attack throws out the anti-drift unification with the mislabel; three functors over one
  signature keeps the unification and states the asymmetry correctly.

### On A6 (the load verifier): right that tnum and depth are wrong, wrong that the fix is a relational domain

The attack is correct twice: tnum is non-relational and cannot express `child_index < arena_size`, and abstract-
interpretation cost is program-size not nesting depth, so the depth cap does not bound it. Both stand. But the
implied fix, "add a relational range analysis," keeps the design inside abstract interpretation, which is exactly
where eBPF is famously incomplete and expensive.

The better composition (cross-domain: LangSec parse-don't-validate plus refinement types). The structural safety
of the arena walk (`child_index < arena_size`, acyclicity, depth-boundedness) is not a numeric relational property
to be recovered by abstract interpretation; it is a typing property of the wire format. Give the arena a
refinement-typed format where an index has type `Fin(arena_size)` and the load check is a linear-time typed
decoder that either produces a well-typed arena or fails (parse, don't validate; Alexis King's discipline lifted
to the untrusted-load boundary). There are no paths to explode because there is no abstract interpretation; there
is a single structural decode, complete (not eBPF-incomplete) and honestly linear in input length. tnum is then
retained only for the genuinely numeric residual (script arithmetic overflow), a separate and smaller property.
Two mechanisms for two properties the attack lumped: typed decoding for structure, tnum for arithmetic.

- Versus original: the original cast the whole load check as one tnum abstract interpretation bounded by depth,
  wrong on both the domain and the bound.
- Versus attack: the attack's relational-domain fix is still incomplete abstract interpretation; typed decoding is
  complete and linear, and it is a stronger guarantee at lower cost, which is the direction the attack pointed at
  but did not take.

### On A7 (shared-implies-promoted): the attack's sharpest catch, and its fix is too blunt

The attack is right, and this is the single most important thing in its audit: promoting every shared node to the
root region is the ML Kit region leak, undoing the reason reachability types were adopted (track the exact
reach-set, do not promote). But C3, "copy all leased shared values into trees, share only lease-free constants,"
is too blunt: forcing every leased shared subvalue to be duplicated causes exponential blowup on exactly the
huge, deeply nested, legitimately-shared values the transfer model (1315) exists to carry.

The better composition (cross-domain: reference counting as a compile-time finalisation, not a runtime
collector). Keep sharing, and give a shared node the exact meet of its referrers' reach-sets, which is what
reachability promised and what avoids both the leak and the copy. Compute it by a reversed referrer count on
shared roots only: a shared node's lease finalises when its last referrer is emitted (a count-down, dropped at
zero), which yields the exact least-region without promotion and without a collector, because the count is a
compile-time and emission-time bound on shared roots only, itself bounded by the number of shared roots. This
weakens the single-pass promise to single-pass plus a bounded finalisation on shared roots, which is the honest
cost, and it is far better than a leak (A7) or a blowup (C3).

- Versus original: the original's shared-implies-promoted is the leak the attack correctly identified.
- Versus attack: C3 avoids the leak by forbidding leased sharing, paying exponential duplication on the target
  workload; exact-meet-by-reversed-referrer-count keeps sharing and exactness at the cost of a bounded finalise,
  delivering the reachability paradigm's actual promise that both original and attack walked away from.

### On A8 (one coroutine, two stages): right that it is not one-shot, wrong that there is no shared theory

The attack correctly shows equality saturation is multi-shot and re-entrant (it revisits e-classes under union
and congruence) while value emission is one-shot, so "one one-shot coroutine" is false. But its conclusion,
"strike the synergy," misses that a real shared abstraction is one level up, and the attack itself names it
("semi-naive Datalog with a window") without noticing it is the unification.

The better composition (cross-domain: differential dataflow from the database and stream-processing world).
Both stages, and two more, are bounded-frontier monotone-fixpoint computations: the lease inference (a monotone
dataflow to a least fixpoint), the equality saturation (a monotone congruence fixpoint), the load verifier (a
monotone abstract-interpretation fixpoint), and the value stream (the degenerate one-shot fixpoint). The shared
substrate is semi-naive, differential fixpoint evaluation over a monotone lattice with a bounded working set
(Abadi-McSherry-Murray differential dataflow, and semi-naive Datalog). Value emission is the trivial instance;
eqsat is the general instance; the same bounded-frontier engine runs all four. That is a bigger and genuine
unification than the original's coroutine claim, and it is the frame the attack gestured at and dropped.

- Versus original: the original claimed one one-shot-continuation theory, which is false for eqsat.
- Versus attack: the attack struck the synergy entirely; differential-dataflow-as-the-monotone-substrate restores
  a real unification across four passes, strictly more than the original claimed and the opposite of "strike it."

### On A9 (coincidence sold as structure): right standard, and it exposes a real correction the attack half-saw

The attack's epistemic standard is good: do not count a shared word as a shared theorem, prove each quantity is a
monotone function of one parameter or admit a pun. Applying that standard honestly, the depth cap is not one
parameter doing six jobs, but it is also not six puns. Adopting reachability (point 5) silently changed the
governing quantity: a reachability qualifier is a set of in-scope binders, so the lease-lattice height is the
number of simultaneously-live binders, an environment-width, not a nesting depth. There are three genuine finite
structural bounds, and each does real work: environment-width (the lease frontier, the reachability-bitmask
width, the live-region count), nesting-depth (the iterative work-stack, the arena traversal), and input-length
(the packrat memo, the load-verifier program-points). The honest design carries three named bounds, each doing
two jobs, which is still a strong no-alloc-and-termination story, just not the pun of one.

- Versus original: the original's "one number, six jobs" is two puns (A4, A6) over a silently-shifted quantity.
- Versus attack: the attack demolished the pun but did not name the replacement; three finite structural bounds
  (environment-width, nesting-depth, input-length) is the constructive correction, and it is exactly the shift
  adopting reachability forced.

### On C5 (the graded logical relation): the attack's one visionary move, undersold; sharpen it into the spine

C5 is the attack's best construction and it is right that a step-indexed graded logical relation (Ahmed, Iris,
RustBelt) that proves the lens projections agree, the stencil computes what the interpreter computes (the Deegen
differential test as a proof), and the residual is sound, in one model, is the real prize. But the attack files
it as a hard aspiration and frames it as replacing the differential tests. The sharper synthesis: index the
logical relation by the assurance grade (my A2 enriched lattice), so it is a gradual logical relation that holds
as a proof where assurance is high and degrades to its operational shadow where assurance is low. The differential
test is then not replaced by the proof; it is the grade-0 instance of the same relation, run where the proof is
not affordable, the same object at a lower assurance grade. And because the language is generated per family, the
relation is parametric in the graded algebraic signature (relational parametricity over a generated signature),
which makes it the fourth functor over the one signature from the A5 fix: term, compile, value, and relational,
where the relational functor's fundamental lemma is exactly "compile agrees with interpret." One signature, one
gradual logical relation, subsuming the lens check, the differential test, and the residual soundness on one
assurance scale.

- Versus original: the original had differential tests as a separate assurance mechanism with no unifying proof.
- Versus attack: C5 proposes the relation but leaves it aspirational and test-replacing; indexing it by assurance
  makes test and proof the same object at different grades, and making it parametric over the generated signature
  ties it to the three-functor structure, so it becomes the design's spine rather than a distant stretch goal.

## The cross-domain imports this pass introduces (novelty ledger)

- Differential dataflow and semi-naive evaluation (databases, stream processing) as the one monotone-fixpoint
  substrate for lease, eqsat, load-verify, and value-stream (meta-A8).
- Parse-don't-validate plus refinement-typed wire formats (LangSec, functional-programming discipline) as the
  linear complete load check, displacing abstract interpretation (meta-A6).
- Reference counting as a compile-time and emission-time finalisation (a runtime GC technique moved to
  generation time) for the exact reachability meet on shared roots (meta-A7).
- Graded Freyd category and graded state (frontier categorical semantics) displacing the distributive-law framing
  for the effect-coeffect interaction (meta-A1).
- Online-PE well-foundedness as a coeffect/grade termination bound, keeping PE a graded rule in the shared
  e-graph (meta-A3).
- Assurance-indexed gradual logical relation with the differential test as its grade-0 shadow, parametric over
  the generated signature (meta-C5).
- Inclusion-not-coverage with conflict detection applied to grammar composition (meta-A4).

## Fair pass: where the attack is simply correct and stands

- The packrat memo is not depth-bounded (A4 first half). Correct puncture.
- Stencils are not the introduction fragment (A5). Correct catch; only the fix changes.
- tnum is non-relational and the load cost is program-size (A6). Correct on both; only the fix changes.
- Shared-implies-promoted is the ML Kit leak (A7). The single most valuable catch in the attack.
- Equality saturation is multi-shot, not one-shot (A8). Correct; the shared theory is one level up, not absent.
- Grading is the one real structural unification, and it is conditional (A1, A9). Correct, and the condition is a
  graded-state construction, not a ceded fragment.

## Defence of the original, where the attack overreached

Not reflexive, and not a given. On three concrete mechanism failures the original is simply wrong and the attack
is unquestionable: the packrat memo is not depth-bounded (A4), tnum cannot express the arena relation the walk
needs (A6), and shared-implies-promoted is a genuine internal contradiction with the reachability paradigm (A7).
Those are mechanism errors, not naming errors, and no defence is offered. The "PE equals extraction" label (A3)
also has no defence and is dropped. But on the structural intuitions the attack repeatedly mistook the original's
bad naming for the absence of a real structure, and there the original holds and the attack overreached:

- A1. The attack escalated the obligation to a Beck distributive law, found none in general, and ceded the
  mutable fragment. The original's underlying stance, that the effect-coeffect combination is available without a
  new exotic construction, is the more correct one (the graded-state or Freyd reading needs no distributive law
  at all). The original named it badly ("adjunction"); the attack demanded a law that was never owed.
- A2. The original's instinct that assurance is a computed, total, mechanical projection is recoverable and
  superior to the attack's "make it partial." The enriched-codomain total map vindicates the totality instinct;
  the attack's partial-map repair is the weaker of the two. The original over-reached only in the plain-lattice
  codomain, not in claiming a total map.
- A5. The load-bearing insight (generation from one source prevents drift) is entirely correct, and the attack's
  "drop the stencil leg" discards it. The original over-stated the introduction-form symmetry; it did not
  over-state the anti-drift unification, which stands intact as three functors over one signature.
- A8. The original's claim that the streaming stages share structure is right; "strike the synergy" is the
  overreach. A shared substrate genuinely exists (the monotone bounded-frontier fixpoint, differential dataflow);
  the original mis-named it a one-shot coroutine, but its structural intuition beat the attack's dismissal, and
  the attack even named the correct substrate ("semi-naive with a window") while concluding there was none.
- A9. The original correctly saw that a small set of finite structural bounds carries the whole no-alloc,
  termination, and verification story. The attack's "mostly puns" under-credits a real consolidation: four of the
  six jobs are genuine functions of two real bounds (environment-width and nesting-depth). The original miscounted
  one versus three; it did not invent the consolidation.
- A3, partial. The PE-equals-extraction label is indefensible, but the original's bounded-frontier-streaming
  instinct for the compile stage was sounder than the attack credited: windowed, stratified saturation is a real
  construction (the same differential-dataflow substrate), so the tension the attack drew between "bounded
  e-graph" and the compile stage was overstated. The label was wrong; the streaming instinct partly survives.

The pattern: the attack is unanswerable on the concrete mechanism failures (A4, A6, A7) and the one false label
(A3), and it overreaches on the structural intuitions (A1, A2, A5, A8, A9), where the original saw a real
unification and named it poorly. A fair scoring keeps the attack's mechanism corrections and the original's
structural instincts, which is what the meta-attacks above already do.

## Closing

The attack did the harder half well: it found the real leak (A7), the real mislabels (A5, A6), and the real
pun (A9), and it named the real prize (C5). Its systemic flaw is that its constructions retreat to the
established two-phase, tree-only, general-parser shapes the moment a mechanism breaks, and three of them
(C2, C3, C4) reintroduce the exact problem they escaped, while its flagship (A1) mis-chose a distributive law for
what is graded dataflow. The better result, compared to both original and attack throughout, keeps every
unification the attack conceded was real and recovers three the attack discarded: graded-state (not
distributive-law) for the effect-coeffect interaction, differential-dataflow (not a struck synergy) as the
monotone substrate, and exact-meet-by-reversed-referrer-count (not copy-everything) for sharing. The single
sharpest correction to the attack: its own prize (C5) is not a distant stretch goal that replaces the tests, it
is the assurance-indexed gradual logical relation that makes proof and test one object and, made parametric over
the generated signature, becomes the fourth functor and the spine of the whole design. Nothing here reopens the
five converged directions; it sharpens the mechanisms one level past where the attack left them.
