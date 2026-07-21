# Partial-eval specialization: fold ratio on a random static/dynamic mix

Scaffold. Numbers are filled by the harness run (`vehje-benches pe_random`).
Companion: `pe_structured` (the block-structured template shape).

## What this measures (a reduction metric, not a strategy race)

Binding-time partial evaluation: static (compile-time-known) subexpressions fold away,
leaving only the dynamic (runtime) computation in the residual. The scientific payload
is the FOLD RATIO = original node count / residual (live) node count, a REDUCTION
metric, not a speed comparison. The harness times the specialize PASS (a legitimate
throughput measurement, ~100-230 ns/node in prior work) and records an output whose
high bits carry the residual (live) and original node counts, so the ratio is
recoverable; the low bits carry an input-seeded checksum of the folded residual so the
pass cannot hoist.

Program shape: a random static/dynamic mix built as a reduction tree over N leaves
(N = harness size). A combine node folds only if its WHOLE subtree is static, so
scattered static leaves under dynamic parents do not fold. This is PE's pessimistic
case. Variants sweep the static-leaf fraction: `pe_rand_sf30` / `sf50` / `sf70` /
`sf90`. They produce different residuals, so the bench is `may_differ = true` (no
cross-variant byte comparison; each variant is still checked deterministic per input).
The program is built once via `OnceLock` at the process's size; the pass runs REP = 16
full walks per timed call, seeded from the input.

## Expected result (from the design)

The fold ratio is expected to rise only modestly with the static fraction, roughly
1.2x at 30% static to about 2.9x at 90%, because a subtree folds whole only when every
leaf under it is static, which stays rare even at high static fractions in a random
mix. This is the number to compare against `pe_structured`, where contiguous static
structure folds far harder (3.8x to 7.0x). The finding is that the specialization
ratio tracks static STRUCTURE, not just the static fraction.

## Termination

The specialize walk is well-founded by construction: every node is built before its
parents, so children carry strictly smaller ids, and the walk only ever descends to
strictly smaller ids (asserted in debug builds). This is the concrete stand-in for the
design's binding-time grade strictly decreasing on every unfold: a static value can
drive only a bounded number of unfoldings before the grade bottoms out, so PE cannot
diverge on a static recursion. Termination is therefore guaranteed, which is what
makes PE safe to run in the compiler.

## Cost-model sanity (fill from run)

The pass is O(nodes) per walk; time should scale close to linearly with the program
size (N) and with REP, confirming the timed work is the walk and not a hoisted
constant. Per-node cost in the low hundreds of nanoseconds is the expected band.

## Boundary

Modeled folds: whole-static-subtree collapse. A full PE also folds static conditionals
to the taken arm, unrolls static loop bounds, and inlines static calls, all of which
fold MORE (raising the ratio); they are not separately modeled here, matching the
prior bench's boundary. The residual node count is a size proxy; the runtime-work
reduction is larger, since a folded static subtree also eliminates its execution.
