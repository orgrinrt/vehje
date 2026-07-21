# Speculative appendix: forced PDE cross-pollination (NOT panel-grounded, off the critical path)

**Date:** 2026-07-20
**Status:** SPECULATIVE. This is a deliberately forced cross-pollination between the nonlinear-parabolic-PDE
machinery of Tapio Kurkinen's field (non-divergence-form parabolic equations, Harnack inequalities, quantitative
stability, boundary regularity, the normalized p-Laplacian) and the machinery this design runs on (monotone
fixpoints, lattices, the gradual guarantee, region inference). It is kept in the trail because op asked whether
the two fields could be harnessed to prove something new, and one candidate is real. None of it is on the
critical path, and none of it belongs in the proof document, which wants the standard region-calculus proof.
The rigorous model lives in `08_theory_insights.md`; this file is the labelled side-quest.

**On the name check:** no Juho Kurkinen has any programming-languages footprint; Tapio Kurkinen is a real
researcher but works in pure mathematics (nonlinear parabolic PDE / regularity theory), connected to this design
only by the lexical false-friend "partial differential equations" versus "partial evaluation." So no Kurkinen
citation fits directly. What follows is the machinery of his field, not his specific papers, forced against ours.

## The structural bridges, weakest to strongest

1. **Fixpoints are discrete dissipative flows.** The lease inference is Kleene/Kildall iteration to a least
   fixpoint of a monotone operator on a lattice. A parabolic PDE is a continuous dissipative flow converging to
   its stationary (elliptic) solution, the fixpoint of the spatial operator. The dataflow analysis is a
   discretized parabolic evolution whose stationary solution is the least fixpoint. Real, but half-known to
   everyone in program analysis.
2. **The methodological kinship is comparison principles, and this is the one that matters.** Kurkinen works in
   non-divergence form, the regime with no energy/variational functional, where correctness is proved with the
   maximum principle and comparison principles (viscosity solutions). This design proves with the same tools: the
   gradual guarantee (adding precision never breaks a safe run) is a monotonicity/comparison statement over a
   precision lattice; abstract-interpretation soundness is monotone over-approximation; the region-calculus
   invariant ("reachability respects the lease order") is an order statement. Both fields abandoned energy methods
   and prove by comparison and monotonicity. Same proof philosophy in two domains, not word-matching.
3. **Ranking functions are discrete Lyapunov functions, and vehje declined a totality axis.** PDE stability is
   proved with decreasing Lyapunov functionals; program termination with ranking functions, which are discrete
   Lyapunov functions. vehje explicitly has no totality axis (budgets as containment). If one were ever wanted,
   the natural object is a Lyapunov/ranking functional, and quantitative-stability technique is how you would get
   resource-budget rates (not "it terminates" but "it terminates within this budget as a function of input
   size"), the quantitative constant-carrying flavour rather than the qualitative one.
4. **Harnack is controlled propagation of a nonnegative quantity.** A Harnack inequality says a nonnegative
   solution cannot be large here and tiny nearby; positivity propagates with a controlled constant. The
   lease-depth field propagates a nonnegative quantity (min depth over linking uses) over the post-order
   structure.

## The two forced harnesses that could be genuinely novel (candidates, not results)

**A. A quantitative gradual guarantee via parabolic-stability estimates (the strong one).** The gradual
guarantee is qualitative: precision changes never turn safe into unsafe, a comparison principle with no
constants. Harnack and quantitative stability are exactly the quantitative refinements of comparison principles:
an explicit bound on how much two solutions differ as a function of how their data differ. Imported here: prove a
quantitative gradual guarantee that bounds how much the runtime residual's cost or behaviour changes as a check
moves across the precision lattice or across a binding time, with explicit constants, the way a stability estimate
bounds solution-difference by data-difference. The panel currently handles "how much does moving a check from
static to dynamic cost" empirically, with benches (Expert 4's entire cost-cliff discussion). A quantitative
gradual guarantee would give that a proved bound. Searches into gradual-typing theory did not surface anyone
importing PDE-style quantitative-stability estimates to make the gradual guarantee quantitative. That is a
candidate "nobody has thought of it" theorem, aimed squarely at the thing the panel could only measure. High-risk,
high-novelty: a paper if the discrete structure cooperates, a dead end if it does not.

**B. A Harnack-type regularity bound for the lease frontier, tighter than the depth cap.** Expert 4 made the
depth cap bound the resident lease-frontier accumulator, but it is a crude worst-case-nesting bound. If the
lease-depth field over the post-order arena satisfies a discrete-Harnack regularity, the actually-live frontier
(values that can still receive a linking use) is bounded more tightly than raw depth, giving a smaller provable
no_alloc buffer with a regularity proof instead of a worst-case one. Same structural role Harnack plays in PDE:
turning a crude a-priori bound into a sharp interior one.

**Wildcard (deepest, least defensible).** The normalized p-parabolic work sits on the normalized p-Laplacian,
which has a tug-of-war stochastic-game interpretation (Peres-Schramm-Sheffield-Wilson). Stochastic games are a
semantics for programs (game semantics). So a thread exists: normalized p-Laplacian to tug-of-war games to game
semantics, connecting those exact equations to a semantic model of the IR vehje interprets. No idea if it goes
anywhere; recorded as the wildest possible cross-pollination.

## Verdict

Not on the critical path, and not for the proof document. But the answer to "could it prove something new" is a
qualified yes: the quantitative gradual guarantee via stability estimates is a genuinely novel theorem-shaped
idea that targets exactly the cost question the panel could only bench, and it borrows from the right field
because both prove by comparison principles rather than energy. It is a high-risk, high-novelty side-quest, parked
here so the thread survives the trail without touching the rigorous model in `08`.
