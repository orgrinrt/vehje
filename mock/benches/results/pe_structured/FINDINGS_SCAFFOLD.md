# Partial-eval specialization: fold ratio on a block-structured template

Scaffold. Numbers are filled by the harness run (`vehje-benches pe_structured`).
Companion: `pe_random` (the random static/dynamic mix shape).

## What this measures (a reduction metric, not a strategy race)

The same binding-time partial evaluation as `pe_random`, on the realistic templating
shape: a document is a right-leaning concatenation of N blocks, each either a
contiguous static text block (an 8-node static chain that folds whole to one constant)
or a dynamic value hole (survives into the residual). The payload is the FOLD RATIO =
original / residual node count, a REDUCTION metric. The harness times the specialize
pass and records the residual and original counts in the output high bits (ratio
recoverable) plus an input-seeded checksum in the low bits (no hoist).

Variants sweep the static-block fraction: `pe_struct_sf50` / `sf70` / `sf90`. They
produce different residuals, so the bench is `may_differ = true` (per-variant
determinism still checked). The program is built once via `OnceLock` at the process's
size (N blocks); the pass runs REP = 16 full walks per timed call, seeded from input.

## Expected result (from the design)

The fold ratio is expected to be strong and to climb with the static-block fraction,
roughly 3.8x at 50% static to about 7.0x at 90%, far above `pe_random`'s 1.2x-2.9x at
the same fractions. The reason is structural: each contiguous static block collapses to
a single constant regardless of its internal size, so the static skeleton disappears
and the residual is essentially the dynamic holes plus the concatenation skeleton. This
is the decisive result for templating and config consumers: "compile the template, run
only the holes" made quantitative. It composes with the interp-output span-list
lowering (static literal runs become residual constants; dynamic holes become the
format steps).

## Termination

Well-founded by construction, identical argument to `pe_random`: children carry
strictly smaller node ids than their parents (asserted in debug), so the walk descends
on a strictly decreasing measure and cannot loop. This is the concrete form of the
design's binding-time grade strictly decreasing on every unfold, the property that lets
PE run pervasively in the compiler without risk of divergence.

## Cost-model sanity (fill from run)

O(nodes) per walk; time should scale close to linearly with N and REP. Note the
residual is much smaller than the input, so most of the walk time is spent reaching and
folding static blocks, then walking the (smaller) dynamic skeleton.

## Boundary

Static blocks are modeled as static add chains that fold whole to one node; a real
template also has static conditionals and static loops that fold further (raising the
ratio). The residual node count is a size proxy; the runtime-work reduction is larger,
since folded static structure also eliminates its execution, not just its nodes.
