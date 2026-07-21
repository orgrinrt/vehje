# Incremental compilation: cold rebuild vs warm content-addressed reload after one edit

2 variants, 6 samples per variant.
Baseline: **incr_cold**

## Highlights

Baseline for all deltas below: **incr_cold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (incr_cold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline incr_cold has the worst median (8.39 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest incr_warm at 2.35 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### incr_warm dominates: 256% faster than the next best (incr_cold)

incr_warm (2.35 ms) leads incr_cold (8.39 ms) by 256%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### incr_warm beats baseline by 72% (significant)

incr_warm is -6.04 ms (72%) faster than baseline incr_cold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.6x the fastest

Fastest incr_warm (2.35 ms) to slowest incr_cold (8.39 ms): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: incr_warm** at 2354934.8 ns median (-71.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.56x (fastest 2354934.8 ns, slowest 8385247.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| incr_cold | 8395683ns | 8389621ns | 8340239ns | 8385386ns | 8438851ns | base |
| incr_warm | 2350279ns | 2359087ns | 2286615ns | 2336197ns | 2403234ns | -72.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| incr_cold | 8391402ns | 8335877ns | 8434635ns | base | 0.002 |
| incr_warm | 2346159ns | 2282611ns | 2399142ns | -72.04% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (incr_warm; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| incr_cold | 0.002 | 27.2% |
| incr_warm | 0.007 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| incr_cold | 8395683ns | 8395683ns | base |
| incr_warm | 2350279ns | 2350279ns | -72.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| incr_cold | 8385248ns | base | --- | [8354324, 8434635] | --- | --- | --- | --- |
| incr_warm | 2354935ns | -6036660.8ns (-72.0%) | [-6112038, -5987029]ns | [2284401, 2399142] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | incr_cold | incr_warm |
|---|---|---|
| 1 | 8415887ns | -72.8% |
| 2 | 8335877ns | -72.2% |
| 3 | 8393504ns | -71.5% |
| 4 | 8453382ns | -71.6% |
| 5 | 8372771ns | -71.3% |
| 6 | 8376992ns | -72.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| incr_cold | -0.276 | moderate- |
| incr_warm | 0.139 | ok |

**Consistency summary:**

- **incr_warm**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| incr_cold | 1008.5ns | 8391402.0ns | 0.0% |  |
| incr_warm | 380.4ns | 2346159.4ns | 0.0% |  |

## Distribution (algo ns)

```
incr_cold (n=6, range 8335876.7-8434634.6 ns)
  8335876.7 |########################################
  8340814.6 |
  8345752.5 |
  8350690.4 |
  8355628.3 |
  8360566.2 |
  8365504.1 |
  8370442.0 |########################################
  8375379.9 |########################################
  8380317.8 |
  8385255.7 |
  8390193.5 |########################################
  8395131.4 |
  8400069.3 |
  8405007.2 |
  8409945.1 |
  8414883.0 |########################################
  8419820.9 |
  8424758.8 |
  8429696.7 |
  (0 below, 1 above range)

incr_warm (n=6, range 2282611.2-2399142.3 ns)
  2282611.2 |########################################
  2288437.8 |
  2294264.3 |
  2300090.9 |
  2305917.4 |
  2311744.0 |####################
  2317570.5 |
  2323397.1 |
  2329223.6 |
  2335050.2 |
  2340876.8 |
  2346703.3 |
  2352529.9 |
  2358356.4 |
  2364183.0 |
  2370009.5 |
  2375836.1 |
  2381662.6 |
  2387489.2 |####################
  2393315.7 |####################
  (0 below, 1 above range)

```
