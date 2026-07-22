# The vehje bench evidence through the benchmark-honesty lens (Frank McSherry)

## Verdict in one line
The incremental-maintenance core is real and honestly measured; the scaling and parallel *cold-load* headlines are inflated by a strawman delta baseline, a no-op "cache", and a frictionless model reported as a measurement.

## What holds (fair baselines, real wins)
- `incremental-scaling` (incr.zig): edit-one-leaf, walk-ancestors, early-out gives 20 visits vs 1,048,575 full at N=1M. The metric is node-visits (structural, not timing-sensitive), the early-out is genuine differential propagation, and the additive/monotone scope is stated. This is the strongest artefact in the batch.
- `sketches/.../compile-stage-parallelism` (parcompile.zig): 5.64x at 8 threads is a *real* threaded run, ReleaseFast, work-stealing, and honest that threads 5-8 land on efficiency cores. Fair.
- `interner-merge-cost` (merge.zig): 6.19 ms serial tail, and the sketch itself computes the Amdahl-correct net cold win of **4.2x** (24.2 ms vs 102 ms). That is the honest number.
- `interner-intern-hotpath`: committed CSV, 20M interns, ~19 ns, load-factor/hash second-order. Clean.
- `egraph-saturation-scaling`: committed CSV, ~5.1x growth over 4 depths, and the *load-bearing* conclusion is the cap, not the linearity.

## Findings
1. **`reach-fixpoint-scale` "whole-column beats delta" is a strawman.** `deltaSemi` (reach.zig:18-27) scans all 16M edges *every round* and only skips the OR body when the child is clean; it never indexes edges by source. So it does strictly *more* work than `wholeColumn` (full scan + dirty/next bookkeeping), and cannot win on a 2-round graph by construction. The finding admits "a dirty-worklist delta would be faster." True semi-naive (frontier relation, join only new tuples) is never implemented, so the semi-naive question is unanswered, not settled. Fair baseline: a worklist that visits only out-edges of changed nodes. It might still lose at 2 rounds, but this bench cannot show it.
2. **The 8M/50 ms number is single-shot and unreproducible-as-committed.** reach.csv is 0 bytes; the table is hand-transcribed. One `clock_gettime` pair, no warmup, no variance. The throughput columns even mix units (whole = edges*rounds/ms, delta = edges/ms, reach.zig:36 vs 39). The graph (backward window 64, reach.zig:12) is synthetic and *guarantees* shallowness, so "converges in 2 rounds" is baked in, not observed of real programs. 50 ms is plausible and comfortably within budget even at 2x error; the *directional* claim survives, the *precise* one does not.
3. **`mod-stack-load` warm is tautological.** stack.zig:31 is `for(0..N)|i|{ if(i==1234){recompile} }`: the 1,999 "cache hits" are a no-op branch. There is no cache, no content hash compared, no lookup. The 0.003 ms / "1300x" measures "1 unit of work vs 2000", i.e. arithmetic. A real content-addressed warm load must hash every mod's input to *find* the change: inherently O(stack), the one cost this bench elides. Cold 4 ms is a "condensed cost model"; the findings' own full-pipeline estimate is ~54 ms (13x higher). The design conclusion (caching gives O(change)) is standard and probably right, but this bench does not measure it.
4. **The 7.7x cross-mod-DAG speedup is a model, not a run.** depdag.zig:57 computes `finite8_ms += ceil(width/8)*COMPILE_MS_PER_MOD` summed over levels: zero barrier cost, perfect per-level load balance, no contention, no efficiency-core penalty. It "exceeds the independent bench's 5.64x" precisely because it is friction-free arithmetic compared against a real measurement. "DAG parallelises better than independent" is a model-vs-measurement artifact and should not be stated as a property. (The Q2 blast-radius, 19.3 avg / 803 max, is an honest graph traversal.)
5. **The topic over-reports the parallel headline.** It cites "5.6x at 8 threads" and "7.7x" while the merge sketch already discounts these to a **4.2x net** cold win once the 6.2 ms serial interner tail is included. The honest cold number (4.2x) exists in the artefacts and is dropped from the topic's headline.
6. **egraph "5.2x linear" is rule-set-specific.** Commutativity+distributivity self-limit under hashcons; associativity/reassociation, the actual explosion driver, is untested, and the rebuild is simplified (no upward-merge congruence repair). Four data points. The finding says all this; the load-bearing output is correctly "the cap is mandatory," not the linearity.

## The single-thread / fair-baseline question
The correct single-thread baseline for cold load is 101.7 ms serial (parcompile) and the correct *net* parallel figure is 4.2x, not the raw 5.6x/7.7x compile speedups. For "whole-column vs delta," whole-column *is* a fine single-thread baseline; the failure is comparing it to a delta that carries semi-naive's overhead without its work-avoidance. For warm mod-stack, the honest baseline is "recompile the changed mod + O(N) input hashing to detect it," against which 0.003 ms is not a valid measurement. For the incremental fixpoint, the baseline (full recompute) is right and the win is real, bounded by the balanced-tree topology and the additive-only scope.

## Over-claimed or artifact of setup
- mod-stack warm "1300x" / warm-cold 0.00076: artifact of a no-op cache loop.
- 4 ms cold headline: condensed mockup; real ~54 ms admitted.
- 7.7x "beats independent": frictionless model vs real run.
- "whole-column beats delta semi-naive": true only against a crippled delta.
- Empty committed CSVs (reach, stack) and header-only (incr): the scaling numbers are not reproducible from committed data, and none of these five go through the harness's cdylib-isolation / cross-validation / normalise rigor (that applies only to the `hx_*`/`arch_*` interp benches).

## Open questions for the synthesiser
1. Re-run `reach` with a real out-edge worklist semi-naive before the design records "whole-column is faster"? Tradeoff: an hour of work; it may confirm whole-column at 2 rounds or expose the strawman. No preference.
2. Require an actually-threaded dep-DAG run (with barriers) to replace the ceil(width/8) model, or accept the model as an upper bound and delete the "better than independent" claim? Tradeoff: real run costs a sketch; the model is fine as a *ceiling* if relabelled.
3. Accept single-shot standalone Zig benches with empty CSVs as directional, or hold the load-bearing scaling numbers (50 ms, 4 ms, parallel) to the same committed-CSV, multi-run, harness discipline the interp tier uses? Tradeoff: rigor vs the fact that 8M-node sets exceed the 16384-byte harness cap and cannot run in-harness as built.
