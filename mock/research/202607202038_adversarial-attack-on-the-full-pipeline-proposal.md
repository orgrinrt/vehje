# Adversarial attack on the full-pipeline proposal

**Date:** 2026-07-20
**Phase:** panel continuation (worker-fork audit, reports once, settles nothing)
**Role taken:** a mathematically-leaning compiler and instruction-set veteran with a novelty knack. The mandate
is to attack the composition proposed in `../../design_rounds/202607202001_topic.converged-architecture-shape.md`
(the "bleeding-edge full-pipeline proposal" section), hard, on paper, and where a claim breaks, to compose a
better novel replacement rather than merely negate. Nothing here reopens the five converged directions as
directions; it attacks the mechanisms and the four claimed synergies, and it separates the one real unification
from the coincidental ones.
**Reads:** the proposal section of topic 202607202001, the five continuation docs 01 to 05, and the arc they
continue.

## One-paragraph verdict

The proposal has exactly one genuine structural unification (the graded spine) and it is padded with three
coincidental ones sold as structure. The graded spine is sound only after a distributive law is constructed,
which the proposal assumes and which does not exist in general. The single most exciting novel move
(partial evaluation as e-graph extraction) trades a theorem for a heuristic and silently forfeits the Futamura
guarantee. Two of the six jobs the depth cap allegedly does are width or length quantities wearing the word
"depth," so the "one number, six jobs" synergy is partly a pun. The memory-model shortcut (shared-implies-promoted)
reintroduces the exact region-leak the reachability paradigm was adopted to avoid. And the "one coroutine, two
stages" synergy is a category error, because a saturating e-graph is multi-shot and re-entrant while a value
emitter is one-shot. What survives is strong and worth keeping: grading as the organising idea (scoped),
reachability-as-coeffect-grade realised as an AccessSet bitmask, copy-and-patch for native, and
effect-constrained extraction for the confluent optimisation fragment. The prize the proposal walks past is a
single graded logical relation that would replace the differential tests with a differential proof.

## The attacks

### A1. The graded spine assumes a distributive law that does not exist in general

The spine combines a graded comonad (the reachability coeffect) with a graded monad (the effect grade) and cites
Gaboardi et al. 2016. That combination is not free: it requires a distributive law, a natural transformation
relating `D T` and `T D` for the comonad `D` and monad `T`, and such laws do not always exist, are not unique
when they do, and the choice fixes the semantics of "an effectful computation under a resource constraint." For
reachability (a comonad over the powerset lattice of in-scope binders) and effects (a monad over the `Permits`
join-semilattice), the distributive law is precisely where "retaining a value through a host boundary (an effect)
interacts with the value's reachability (a coeffect)" is decided, which is call 1's lease-from-effect derivation.
The proposal invokes "the effect-coeffect adjunction derives the bit." An adjunction is not a distributive law,
and the two are being conflated. The lease-from-effect claim rests on a distributive law that must be constructed
and shown to exist, and it demonstrably fails to commute in the general mutable-aliasing case, which is exactly
where effect and coeffect disagree. Severity: high. This is original work mislabelled as grounded, and it is the
mathematical core of call 1.

### A2. Assurance is not a total projection of binding time

The spine's novel move computes the assurance grade as a projection of the modal (binding-time) grade crossed
with the property. This over-reaches. Family semantics are tested regardless of binding time; a family-semantic
property known at language-build time is still only tested, never certified. Termination is outside the system
entirely. So the map from (binding time, property) to assurance is not total: two of its properties (semantics,
termination) are fixed points on the assurance lattice independent of when their inputs are known. The honest
statement is weaker and should be written weaker: assurance is a projection of binding time for the structurally
decidable axes only (family, effect, non-null, and the decidable part of lease), and the non-decidable
properties are constants on the assurance lattice. Selling it as a clean total projection invites the first
reader who knows the difference to distrust the whole inventory. Severity: medium. The fix is a partial map, not
a total one.

### A3. Partial evaluation as e-graph extraction trades a theorem for a heuristic (the sharpest attack)

This is the proposal's flagship novelty and it is the weakest claim. Three compounding problems.

First, extraction is the wrong home for staging correctness. Binding-time separation is a well-formedness
property (the two-level type discipline, Nielson-Nielson): a well-staged term is one whose early fragment
provably erases. Making "maximally residualised" an extraction objective means an ill-staged extraction is a
representable element of the e-graph, merely dispreferred. That is backwards against the design's own 1627
principle (illegal states unrepresentable). Binding time must prune the graph as a typing constraint, not weight
it as a preference.

Second, the mix equation is an equation, not an optimisation. `[[mix(p,s)]] = [[p]](s)` states semantic equality
of the specialised and unspecialised programs; specialisation is meaning-preserving by construction. Casting "the
specialised program" as "the extracted lowest-cost equivalent form under a residualisation objective" is a
non-sequitur: the cost-optimal equivalent term is not the residual, and there is no guarantee the extracted form
is even a valid residual unless every rewrite is meaning-preserving and the staging is enforced as an invariant.
You lose the Futamura guarantee the moment extraction is a bounded heuristic.

Third, and concretely fatal for the no-alloc target, partial evaluation via saturation diverges. PE requires
unfolding and inlining, and saturating an inlining rule over a recursive definition does not terminate; egg
survives this only with iteration and node limits. A no-alloc fixed-capacity e-graph that hits its cap
mid-saturation yields a non-canonical, saturation-order-dependent extraction, which breaks the determinism the
design demands everywhere else (content-addressed, reproducible). So the streaming bounded e-graph and PE-as-
extraction are in direct tension: PE wants unbounded saturation, no-alloc wants a hard cap, and hitting the cap
makes lowering non-deterministic. Severity: high, and it is the load-bearing novel binding of the whole
compile-stage story.

### A4. PEG is the wrong default for a plugin grammar, and packrat is not depth-bounded

Two sub-attacks on stage 1. First, PEG ordered choice does not compose. `A / B` commits to `A` on match, and it
is neither associative nor commutative across independently authored rule sets, so two plugged-in families can
silently shadow each other's syntax with no error. For a metacompiler whose entire premise is independently
authored families plugged in on the input side, non-composable ordered choice is a landmine, and it is exactly
the composition case where you need GLL or Earley, not PEG. PEG-by-default inverts the right default. Second, the
claim that the depth cap bounds the packrat memo is a category error. Packrat's memo table is indexed by
(input position, rule), so it is O(input-length times grammar-size), a width and length quantity, not a nesting
depth. The depth cap bounds recursion depth and does nothing to the packrat table. So either you drop
memoisation (and inherit PEG's exponential worst case) or the table is O(n) in input length and not bounded by
the depth cap at all. Severity: medium to high. One of the six alleged depth-cap jobs is punctured here.

### A5. The three-projections symmetry is partly a fiction

The one-source triple claims the value domain is the introduction-form projection of the node algebra and the
stencils are the introduction-forms compiled. Both are imprecise. A produced value is a normal form, an element
of the initial algebra modulo the runtime's equalities (sharing, multiple constructions of one record), so the
value domain is the introduction fragment quotiented by the reachability and sharing structure, not a clean
projection. Worse, the stencils are not the introduction fragment at all: the machine-code fragments you need are
overwhelmingly for eliminators (field access, arithmetic, calls, branches), not constructors. A `Project` stencil
is a compiled eliminator. The stencil set is closer to the whole operation algebra compiled than to the
introduction fragment, so "stencils equal introduction-compiled" is simply wrong. The pleasing symmetry
(eliminators to the IR, introductions to the value domain, introductions-compiled to the stencils) does not
survive contact with what a stencil is. Severity: medium. Keep the value-domain-as-introduction-fragment insight
(scoped as a quotient); drop the stencil leg of the triple.

### A6. The load verifier borrows a proof for a property it does not cover, and its cost is not depth-bounded

Two problems with casting the load verifier as the graded gradual residual over tnum. First, cost. Abstract
interpretation cost is lattice-height times program-points times transfer-cost, and program-points is the program
size (width), not nesting depth. eBPF path explosion is exponential in branch count and merge imprecision, which
the kernel verifier bounds with a total instruction limit and state pruning, not with a nesting-depth cap. A
wide, shallow, branch-heavy program explodes regardless of a depth cap, so "the depth cap kills the verifier's
path explosion" is the depth-versus-width confusion again. Second, and sharper, tnum is a non-relational domain:
it tracks each value's known bits independently and cannot express relations between values such as
`child_index < arena_size`, which is exactly the property a bounds-checked arena traversal needs. eBPF needs tnum
plus a relational range analysis, and even that is famously incomplete. The soundness result cited (CGO 2022) is
for tnum arithmetic, not for the relational reasoning the arena walk requires. The borrowed proof does not cover
the property it is borrowed for. Severity: high. The load check needs a relational domain (or a dedicated
structural-validation argument, simdjson-style, whose correctness is a separate matter), and the cost bound must
be stated in instructions, not depth.

### A7. Shared-implies-promoted reintroduces the region leak the reachability paradigm was chosen to avoid

The memory model keeps single-pass lease inference over a shared DAG by promoting every shared node to the
outermost enclosing region. Promotion is over-approximation, so a shared leaf referenced from many depths is
pinned to the root region and freed last. On a value with pervasive sharing this promotes all shared substructure
to the root, which is precisely the ML Kit region-leak the arc's own debt-2 history warns against: regions that
should free early stay live because one deep reference pins them. Reachability types avoid this by not promoting,
by tracking the actual reach-set, which is the reason they were adopted (point 5) over the older region calculus.
So the memory-model shortcut undoes the reason for the paradigm, on exactly the shared-and-aliased case
reachability types were supposed to handle well. Severity: high. This is an internal contradiction between the
memory model and the lease paradigm, not merely a cost.

### A8. The e-graph and the value emitter are not the same coroutine

The fourth synergy shares one one-shot-continuation linearity theory between the streaming e-graph and the value
transfer. A one-shot continuation is invoked at most once. Equality saturation is iterative to a fixpoint: it
revisits e-classes as unions merge and congruence closure re-canonicalises, which is multi-shot and re-entrant,
the opposite of one-shot. The value emitter is genuinely one-shot (emit a subtree, never revisit). They are
different abstractions, and the frontier-boundedness proof for value emission (linear continuation) does not
transfer to the e-graph, which needs a bounded-working-set fixpoint argument closer to semi-naive Datalog
evaluation with a window. The claimed shared theory is false. Severity: medium to high. The synergy should be
struck.

### A9. The meta-attack: coincidence sold as structure

Four "one X does N" synergies. The mathematically-suspicious null hypothesis, when a design claims one parameter
governs six unrelated quantities, is equivocation: "depth" is being used for nesting depth, lattice height,
work-stack size, traversal bound, memo size, and bitmask width, quantities that share a word. A4 and A6 already
show two of the six are width or length. Real unification would prove each of the six is bounded by a monotone
function of one parameter; the proposal asserts it. Likewise "one signature, three projections" is partly fiction
(A5) and "one coroutine, two stages" is false (A8). One of the four synergies, the graded spine, is a real
structural unification, and even it is conditional (A1). The taste verdict: keep the one real unification, strip
the three puns, and stop counting a shared name as a shared theorem.

## The better compositions (where an attack lands, build past it)

### C1. Scope the distributive law to the flow-sensitive fragment, and name it

The reachability-coeffect and effect-monad do have a canonical interaction, but not in general: it is exactly the
construction in "Free to Move: Reachability Types with Flow-Sensitive Effects" (2025). Cite that as the
distributive law and accept its restriction (flow-sensitive effects) as the boundary of where lease-from-effect
is defined. Outside that fragment the derivation is undefined and the generational residual fires. This turns A1
from a hole into a precise scoping: lease-from-effect is sound on the flow-sensitive fragment, dynamic elsewhere,
which is the graded-gradual story told honestly.

### C2. Split the compile stage: type-directed PE, then effect-constrained eqsat on the residual

Fixing A3 without losing the good part. Do binding-time specialisation as a separate, provably-correct,
type-directed pass (the two-level discipline), whose output is a well-staged residual by construction and carries
the Futamura guarantee. Then run equality saturation only on the confluent, terminating fragment (const-fold,
CSE, algebraic identities) where extraction-under-a-cost-model is sound optimisation, with the LMS effect edges
as egglog extraction constraints. The novelty that survives is real and worth having: effect-constrained
extraction is a genuine and unclaimed composition. What dies is "PE as extraction," which was never PE. Bounded
no-alloc eqsat is still owed, but now it optimises a fixed residual (bounded input, terminating rules), so the
cap no longer makes lowering non-deterministic.

### C3. Share only lease-free interned values; keep leased values as trees

Fixing A7. Permit sharing only for immutable, region-free, interned values (constants), which have no lease and
live forever in a never-freed intern pool. Any value carrying a non-trivial reachability lease is unshared (a
tree, copied). Then leases are genuinely tree-synthesised (single pass, no promotion, no leak), and sharing
exists exactly where it is free. This is strictly better than shared-implies-promoted: it keeps the single-pass
property and the reachability paradigm's whole point at once, and it matches the well-known discipline that only
immutable values are safely shared.

### C4. Default the input side to GLL or Earley, reserve PEG for leaf families, and bound by instructions

Fixing A4. For composed, independently authored family grammars the default must be a general context-free
parser (GLL or Earley) whose union is well-defined; PEG's ordered choice is acceptable only inside a single
family's self-contained leaf grammar where the author controls the choice order. State the parser's resource
bound in input length and grammar size (its real complexity), not in depth, and size its working memory as a
host-lent capability like every other buffer, rather than pretending the depth cap covers it.

### C5. The prize: one graded logical relation, replacing the differential tests with a differential proof

The proposal walks past the theorem worth chasing. If the design is graded anyway, build a step-indexed graded
logical relation (the Ahmed and Iris and RustBelt lineage, indexed by the assurance grade) that simultaneously
proves the lens projections agree, the copy-and-patch stencil computes what the interpreter arm computes (the
Deegen differential test, proven not tested, for the decidable fragment), and the gradual residual is sound. One
semantic model, graded, subsuming three separate assurance mechanisms. This is frontier and hard (graded
logical relations are recent), but it is the real unification the four coincidental synergies were gesturing at,
and it is worth more than all four. If any single piece of original work deserves the effort, it is this one.

## The fair pass: what survives the attack intact

- Grading as the organising idea, scoped by C1. The one real unification.
- Reachability-as-coeffect-grade realised as a fixed-capacity AccessSet bitmask reusing the family and effect
  machinery. Genuinely sound and elegant, and independent of every attack above.
- The degenerate depth-lease as the proven floor under the reachability grade. Sound and the right de-risking.
- Copy-and-patch as the native tier for arriving scripts. Proven in CPython 3.13 and Deegen; the attacks touch
  the stencil-as-introduction mislabel (A5), not the mechanism.
- Effect-constrained extraction for the confluent optimisation fragment (the surviving half of A3 via C2).
- The load-verifier direction, once it gains a relational domain and an instruction-count bound (A6).
- Assurance-as-projection for the decidable axes only (A2).

## Closing

Strip the three coincidental synergies, scope the one real one, and the proposal is still a strong and
genuinely bleeding-edge shape: a graded metacompiler whose lease grade is a reachability qualifier, generating a
copy-and-patch runtime, with a compile stage that specialises by typing and optimises by effect-constrained
saturation. The single sharpest correction is A3 (do not call cost-optimal extraction partial evaluation); the
single sharpest contradiction is A7 (shared-implies-promoted versus the reason reachability was chosen); and the
single most valuable thing the proposal did not reach for is C5, the graded logical relation that turns the
differential tests into a proof. Everything here is an attack with a construction attached; none of it reopens
the five converged directions, and the parts that survive are the parts that were never puns.
