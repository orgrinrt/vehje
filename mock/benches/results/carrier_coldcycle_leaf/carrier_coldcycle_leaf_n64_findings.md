# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), leaf profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_leaf_null dominates: 27% faster than the next best (carrier_cold_leaf_threaded)

carrier_cold_leaf_null (1.05 us) leads carrier_cold_leaf_threaded (1.34 us) by 27%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_leaf_null beats baseline by 30% (significant)

carrier_cold_leaf_null is -462 ns (30%) faster than baseline carrier_cold_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_leaf_fntable is an outlier: 2.7x slower than the field

carrier_cold_leaf_fntable (2.87 us) is 2.7x the fastest (1.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_leaf_null, carrier_cold_leaf_threaded, carrier_cold_leaf_switch} vs {carrier_cold_leaf_fntable} (88% apart)

The field splits into a fast tier {carrier_cold_leaf_null, carrier_cold_leaf_threaded, carrier_cold_leaf_switch} and a slow tier {carrier_cold_leaf_fntable} with a 88% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_leaf_null** at 1053.3 ns median (-31.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.73x (fastest 1053.3 ns, slowest 2872.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_leaf_fntable | 5261ns | 5198ns | 5099ns | 5168ns | 5481ns | +38.00% |
| carrier_cold_leaf_null | 3362ns | 3337ns | 3191ns | 3333ns | 3490ns | -11.82% |
| carrier_cold_leaf_switch | 3812ns | 3836ns | 3575ns | 3797ns | 3953ns | base |
| carrier_cold_leaf_threaded | 3639ns | 3630ns | 3470ns | 3583ns | 3808ns | -4.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_leaf_fntable | 2925ns | 2779ns | 3105ns | +91.99% | 0.022 |
| carrier_cold_leaf_null | 1058ns | 1003ns | 1098ns | -30.55% | 0.060 |
| carrier_cold_leaf_switch | 1524ns | 1426ns | 1591ns | base | 0.042 |
| carrier_cold_leaf_threaded | 1340ns | 1281ns | 1393ns | -12.06% | 0.048 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_leaf_fntable | 242613 | 667981 | 0.363 | 0.82× |
| carrier_cold_leaf_null | 265255 | 1388554 | 0.191 | 0.90× |
| carrier_cold_leaf_switch | 296286 | 1238128 | 0.239 | 1.00× |
| carrier_cold_leaf_threaded | 266942 | 1307818 | 0.204 | 0.90× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.064 Gops/s** (carrier_cold_leaf_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_leaf_fntable | 0.022 | 34.9% |
| carrier_cold_leaf_null | 0.061 | 95.2% |
| carrier_cold_leaf_switch | 0.042 | 65.6% |
| carrier_cold_leaf_threaded | 0.048 | 75.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_leaf_fntable | 5261ns | 5261ns | +38.00% |
| carrier_cold_leaf_null | 3362ns | 3362ns | -11.82% |
| carrier_cold_leaf_switch | 3812ns | 3812ns | base |
| carrier_cold_leaf_threaded | 3639ns | 3639ns | -4.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_leaf_switch | 1529ns | base | --- | [1451, 1591] | --- | --- | --- | --- |
| carrier_cold_leaf_fntable | 2873ns | +1349.8ns (+88.3%) | [+1293, +1562]ns | [2798, 3105] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_leaf_null | 1053ns | -461.8ns (-30.2%) | [-531, -403]ns | [1023, 1098] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_leaf_threaded | 1338ns | -171.0ns (-11.2%) | [-252, -129]ns | [1289, 1393] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_leaf_switch | carrier_cold_leaf_fntable | carrier_cold_leaf_null | carrier_cold_leaf_threaded |
|---|---|---|---|---|
| 1 | 1476ns | +90.9% | -29.4% | -9.1% |
| 2 | 1426ns | +94.8% | -29.7% | -10.2% |
| 3 | 1540ns | +87.5% | -31.9% | -12.8% |
| 4 | 1566ns | +97.6% | -32.2% | -7.8% |
| 5 | 1519ns | +105.1% | -25.2% | -14.7% |
| 6 | 1615ns | +77.0% | -34.5% | -17.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_leaf_fntable | 0.332 | moderate+ |
| carrier_cold_leaf_null | 0.162 | ok |
| carrier_cold_leaf_switch | 0.141 | ok |
| carrier_cold_leaf_threaded | -0.264 | moderate- |

**Consistency summary:**

- **carrier_cold_leaf_fntable**: won 0/6, lost 6/6
- **carrier_cold_leaf_null**: won 6/6, lost 0/6
- **carrier_cold_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_leaf_fntable | 76277.7ns | 2925.2ns | 2607.6% | HIGH |
| carrier_cold_leaf_null | 85237.2ns | 1058.1ns | 8055.6% | HIGH |
| carrier_cold_leaf_switch | 95921.8ns | 1523.6ns | 6295.7% | HIGH |
| carrier_cold_leaf_threaded | 85010.0ns | 1339.9ns | 6344.5% | HIGH |

## Distribution (algo ns)

```
carrier_cold_leaf_fntable (n=6, range 2778.7-3104.8 ns)
   2778.7 |########################################
   2795.0 |
   2811.3 |########################################
   2827.6 |
   2843.9 |########################################
   2860.2 |
   2876.5 |########################################
   2892.8 |
   2909.1 |
   2925.4 |
   2941.8 |
   2958.1 |
   2974.4 |
   2990.7 |
   3007.0 |
   3023.3 |
   3039.6 |
   3055.9 |
   3072.2 |
   3088.5 |########################################
  (0 below, 1 above range)

carrier_cold_leaf_null (n=6, range 1002.9-1098.3 ns)
   1002.9 |########################################
   1007.7 |
   1012.4 |
   1017.2 |
   1022.0 |
   1026.8 |
   1031.5 |
   1036.3 |
   1041.1 |########################################
   1045.8 |########################################
   1050.6 |
   1055.4 |########################################
   1060.1 |########################################
   1064.9 |
   1069.7 |
   1074.5 |
   1079.2 |
   1084.0 |
   1088.8 |
   1093.5 |
  (0 below, 1 above range)

carrier_cold_leaf_switch (n=6, range 1426.2-1590.6 ns)
   1426.2 |########################################
   1434.4 |
   1442.6 |
   1450.9 |
   1459.1 |
   1467.3 |
   1475.5 |########################################
   1483.7 |
   1492.0 |
   1500.2 |
   1508.4 |
   1516.6 |########################################
   1524.8 |
   1533.1 |########################################
   1541.3 |
   1549.5 |
   1557.7 |
   1565.9 |########################################
   1574.2 |
   1582.4 |
  (0 below, 1 above range)

carrier_cold_leaf_threaded (n=6, range 1281.2-1393.1 ns)
   1281.2 |####################
   1286.8 |
   1292.4 |####################
   1298.0 |
   1303.6 |
   1309.2 |
   1314.8 |
   1320.4 |
   1326.0 |
   1331.6 |####################
   1337.2 |########################################
   1342.7 |
   1348.3 |
   1353.9 |
   1359.5 |
   1365.1 |
   1370.7 |
   1376.3 |
   1381.9 |
   1387.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_leaf_fntable**: bridge=2653.0% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_null**: bridge=8067.7% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_switch**: bridge=6262.8% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_threaded**: bridge=6376.6% of algo (FFI overhead may distort results)
