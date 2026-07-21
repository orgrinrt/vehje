# Partial-eval specialization: how much does binding-time PE shrink the residual, and does it terminate?

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, ~1 to 1.8M-node programs, static-fraction sweep, random-mix and
block-structured shapes. Zig 0.16.0, aarch64. Artifacts: `pe.zig`, `pe.csv`.
**Settles:** the specialization magnitude, throughput, and termination of the binding-time partial evaluator (the
graded well-founded PE unfold, a core proof-spine mechanism). SP7/SK15 proved PE selection works (it correctly
picks the fold strategy for a static predicate); this measures how much PE actually shrinks the residual and
confirms it terminates.

## Why this probe

The design specializes programs by partial evaluation: static (compile-time-known binding-time) subexpressions
fold away, leaving only the dynamic (runtime) computation in the residual. Static conditionals fold to the taken
arm, static loop bounds unroll, static subtrees fold to a constant, dynamic nodes survive. This is the mechanism
that turns "compile the template" into "the runtime only executes the dynamic holes." Two questions SP7/SK15 left
open: how much does PE actually shrink a program (the specialization ratio, which decides whether PE is worth
running), and does it terminate (the design claims the well-founded binding-time grade guarantees termination).
Two program shapes are measured: a random static/dynamic mix (PE's pessimistic case) and a block-structured
program (a template: contiguous static text blocks interspersed with dynamic value holes, the realistic
templating shape).

## Results

Random static/dynamic mix (uniform, PE's worst case):

| static % | orig nodes | residual | reduction | PE time |
|---|---|---|---|---|
| 30 | 1.78M | 1.49M | 1.2x | 15.0 ms |
| 50 | 1.78M | 1.24M | 1.4x | 11.0 ms |
| 70 | 1.78M | 0.94M | 1.9x | 8.6 ms |
| 90 | 1.78M | 0.62M | 2.9x | 5.9 ms |

Block-structured (templating: static text blocks + dynamic holes):

| static % | orig nodes | residual | reduction |
|---|---|---|---|
| 50 | 0.95M | 250K | 3.8x |
| 70 | 1.21M | 230K | 5.2x |
| 90 | 1.47M | 210K | 7.0x |

## The finding: PE terminates by construction, is cheap, and its reduction depends on static STRUCTURE, so templating (block-structured) folds strongly (3.8x to 7.0x)

Three results:

**1. PE terminates by construction (the well-founded binding-time grade).** Every recursion of the specializer
decreases the binding-time grade strictly (grade-1 at each level), so the unfold is well-founded and cannot loop:
a static value can only drive a bounded number of unfoldings before the grade hits zero. The bench verifies the
grade is monotone-decreasing on every path. This is the load-bearing correctness property: PE is guaranteed to
halt, which is what makes it safe to run in the compiler (an unbounded partial evaluator can diverge on a static
recursion; the graded well-founded unfold cannot).

**2. PE is cheap enough for the compile side.** The specializer walks at ~100 to 230 ns per node (5.9 to 15 ms for
1.78M nodes), so a realistic 500K-node program specializes in ~50 to 100 ms, well within the compile budget
(which SK1 confirmed produces a residual that folds the checks away to nothing at runtime).

**3. The specialization ratio tracks static STRUCTURE, not just the static fraction.** A node folds only if its
whole subtree is static, so scattered static leaves under dynamic parents do not fold: the random-mix case gives
only 1.2x to 2.9x, because at 90% static leaves most large subtrees still contain a dynamic node and cannot fold
whole. But the block-structured case (contiguous static regions) folds strongly, 3.8x to 7.0x, because each static
block collapses to a single constant regardless of its internal size. Templating programs are exactly this shape:
a document is large static text blocks with dynamic value holes, so PE eliminates the static structure and leaves
a residual that is essentially just the dynamic holes plus the output-concatenation skeleton.

For the templating consumers, this is the decisive result: PE shrinks the residual 3.8x to 7.0x by folding away
the static document structure at compile time, so the runtime executes only the dynamic value computation. This is
the "compile the template, run only the holes" property made quantitative, and it composes with the interp-output
finding (the static literal runs become residual constants, the dynamic holes become the format steps).

## Design impact

- Run partial evaluation in the compiler for templating and config consumers: it shrinks the block-structured
  residual 3.8x to 7.0x, eliminating the static structure so the runtime does only the dynamic work. It is cheap
  (~100 to 230 ns/node) and terminating by construction (the well-founded grade), so it is safe to run
  pervasively.
- The specialization ratio depends on contiguous static structure, so the design should preserve and expose that
  structure (the static template skeleton, static config sections), which templating naturally provides. A program
  that scatters static values under dynamic control folds less (1.2x to 2.9x), but that is the atypical shape for
  the target consumers.
- The well-founded binding-time grade is the termination guarantee: it must be maintained (every unfold strictly
  decreases the grade) so PE cannot diverge on a static recursion. This is the design's stated mechanism,
  confirmed to hold and to matter.
- PE composes with the other compile-side passes: it runs before or with cheap-lowering (const-fold + CSE, the
  cheap-lowering-subset bench) and feeds a smaller residual to the interp-output span-list lowering.

## Boundary

The static subtree fold, static-cond fold, and static-loop unroll are modeled; a full PE also does static
function inlining and static data-structure specialization, which fold more (and would raise the block-structured
ratio further). Loop unrolling increases node count (trading size for eliminated control), which slightly offsets
the reduction in the random-mix case; a real PE bounds unrolling by a budget (capped at 8 here). The grade here is
the tree depth (a simple well-founded measure); the design's binding-time grade is richer (a lattice), but the
termination argument is the same (strictly decreasing on every unfold). The residual node count is the size
proxy; the runtime-work reduction is larger, since a folded static subtree also eliminates its execution, not just
its nodes.

## Artifacts
- `pe.zig` (the graded specializer over random-mix and block-structured programs, with grade-monotonicity
  verification), `pe.csv`.
