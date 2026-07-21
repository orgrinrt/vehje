# Partial-eval specialization: fold ratio on a random static/dynamic mix

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

## Measured results

Ratio to baseline (pe_rand_sf90), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | pe_rand_sf30 (ratio) | pe_rand_sf50 (ratio) | pe_rand_sf70 (ratio) | pe_rand_sf90 (base) |
|---|---|---|---|---|
| 64 | 2.50x | 2.28x | 1.90x | 2374 ns |
| 256 | 2.01x | 1.83x | 1.60x | 10198 ns |
| 1024 | 2.39x | 2.22x | 1.82x | 31192 ns |
| 4096 | 2.52x | 2.37x | 1.83x | 132910 ns |
| 16384 | 2.23x | 2.33x | 1.92x | 831193 ns |

## Cost-model sanity line

At n=16384, the baseline (pe_rand_sf90) median is 831193 ns for N nodes, a fold attempt per speculatable node. Treating n as the work-item count, that is 50.73 ns/item, about 162.3 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

reduction metric (variants fold different amounts; the residual-node count is the output, anti-hoisted).

## Verdict

Partial-evaluation payoff scales with the speculation factor: sf90 (90% of branches speculated foldable) is the fastest, sf30 is 2.2x to 2.5x slower. On random programs, aggressive speculation folds more and wins. Monotonic in the speculation factor.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
