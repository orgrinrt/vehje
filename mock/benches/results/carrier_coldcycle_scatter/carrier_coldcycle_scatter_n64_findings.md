# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), scatter profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_scatter_null dominates: 65% faster than the next best (carrier_cold_scatter_threaded)

carrier_cold_scatter_null (1.24 us) leads carrier_cold_scatter_threaded (2.06 us) by 65%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_scatter_null beats baseline by 41% (significant)

carrier_cold_scatter_null is -872 ns (41%) faster than baseline carrier_cold_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_scatter_fntable is an outlier: 2.4x slower than the field

carrier_cold_scatter_fntable (3.02 us) is 2.4x the fastest (1.24 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_scatter_null} vs {carrier_cold_scatter_threaded, carrier_cold_scatter_switch, carrier_cold_scatter_fntable} (65% apart)

The field splits into a fast tier {carrier_cold_scatter_null} and a slow tier {carrier_cold_scatter_threaded, carrier_cold_scatter_switch, carrier_cold_scatter_fntable} with a 65% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_scatter_null** at 1242.7 ns median (-41.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.43x (fastest 1242.7 ns, slowest 3018.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_scatter_fntable | 5363ns | 5279ns | 5037ns | 5210ns | 5755ns | +20.21% |
| carrier_cold_scatter_null | 3576ns | 3504ns | 3378ns | 3499ns | 3789ns | -19.85% |
| carrier_cold_scatter_switch | 4461ns | 4414ns | 4209ns | 4362ns | 4735ns | base |
| carrier_cold_scatter_threaded | 4357ns | 4361ns | 4116ns | 4291ns | 4578ns | -2.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_scatter_fntable | 3047ns | 2887ns | 3228ns | +42.47% | 0.021 |
| carrier_cold_scatter_null | 1265ns | 1210ns | 1331ns | -40.83% | 0.051 |
| carrier_cold_scatter_switch | 2139ns | 2032ns | 2259ns | base | 0.030 |
| carrier_cold_scatter_threaded | 2056ns | 1975ns | 2131ns | -3.87% | 0.031 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_scatter_fntable | 251124 | 684934 | 0.367 | 0.87× |
| carrier_cold_scatter_null | 268688 | 1425546 | 0.188 | 0.93× |
| carrier_cold_scatter_switch | 287531 | 919508 | 0.313 | 1.00× |
| carrier_cold_scatter_threaded | 271719 | 937472 | 0.290 | 0.95× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.053 Gops/s** (carrier_cold_scatter_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_scatter_fntable | 0.021 | 40.1% |
| carrier_cold_scatter_null | 0.052 | 97.4% |
| carrier_cold_scatter_switch | 0.030 | 57.0% |
| carrier_cold_scatter_threaded | 0.031 | 58.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_scatter_fntable | 5363ns | 5363ns | +20.21% |
| carrier_cold_scatter_null | 3576ns | 3576ns | -19.85% |
| carrier_cold_scatter_switch | 4461ns | 4461ns | base |
| carrier_cold_scatter_threaded | 4357ns | 4357ns | -2.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_scatter_switch | 2122ns | base | --- | [2036, 2259] | --- | --- | --- | --- |
| carrier_cold_scatter_fntable | 3019ns | +914.2ns (+43.1%) | [+841, +969]ns | [2895, 3228] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_scatter_null | 1243ns | -871.9ns (-41.1%) | [-935, -813]ns | [1223, 1331] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_scatter_threaded | 2056ns | no significant difference | [-169, +21]ns | [1980, 2131] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_scatter_switch | carrier_cold_scatter_fntable | carrier_cold_scatter_null | carrier_cold_scatter_threaded |
|---|---|---|---|---|
| 1 | 2177ns | +40.8% | -42.8% | -9.3% |
| 2 | 2068ns | +39.6% | -40.0% | -4.0% |
| 3 | 2032ns | +46.3% | -40.5% | +1.6% |
| 4 | 2247ns | +42.6% | -41.8% | -6.0% |
| 5 | 2271ns | +43.2% | -40.3% | -5.3% |
| 6 | 2039ns | +42.4% | -39.4% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_scatter_fntable | 0.000 | ok |
| carrier_cold_scatter_null | 0.045 | ok |
| carrier_cold_scatter_switch | -0.097 | ok |
| carrier_cold_scatter_threaded | 0.425 | moderate+ |

**Consistency summary:**

- **carrier_cold_scatter_fntable**: won 0/6, lost 6/6
- **carrier_cold_scatter_null**: won 6/6, lost 0/6
- **carrier_cold_scatter_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_scatter_fntable | 78419.0ns | 3047.1ns | 2573.6% | HIGH |
| carrier_cold_scatter_null | 85766.6ns | 1265.5ns | 6777.4% | HIGH |
| carrier_cold_scatter_switch | 92116.2ns | 2138.8ns | 4306.8% | HIGH |
| carrier_cold_scatter_threaded | 84972.1ns | 2056.0ns | 4132.9% | HIGH |

## Distribution (algo ns)

```
carrier_cold_scatter_fntable (n=6, range 2886.7-3227.9 ns)
   2886.7 |########################################
   2903.8 |
   2920.8 |
   2937.9 |
   2954.9 |
   2972.0 |####################
   2989.1 |
   3006.1 |
   3023.2 |
   3040.3 |
   3057.3 |####################
   3074.4 |
   3091.4 |
   3108.5 |
   3125.6 |
   3142.6 |
   3159.7 |
   3176.8 |
   3193.8 |####################
   3210.9 |
  (0 below, 1 above range)

carrier_cold_scatter_null (n=6, range 1210.0-1331.0 ns)
   1210.0 |####################
   1216.1 |
   1222.1 |
   1228.2 |
   1234.2 |########################################
   1240.3 |####################
   1246.3 |
   1252.4 |
   1258.4 |
   1264.5 |
   1270.5 |
   1276.6 |
   1282.6 |
   1288.7 |
   1294.7 |
   1300.8 |
   1306.8 |####################
   1312.9 |
   1318.9 |
   1325.0 |
  (0 below, 1 above range)

carrier_cold_scatter_switch (n=6, range 2032.1-2258.8 ns)
   2032.1 |########################################
   2043.4 |
   2054.8 |
   2066.1 |####################
   2077.4 |
   2088.8 |
   2100.1 |
   2111.4 |
   2122.8 |
   2134.1 |
   2145.4 |
   2156.8 |
   2168.1 |####################
   2179.4 |
   2190.8 |
   2202.1 |
   2213.4 |
   2224.8 |
   2236.1 |####################
   2247.4 |
  (0 below, 1 above range)

carrier_cold_scatter_threaded (n=6, range 1974.6-2131.2 ns)
   1974.6 |########################################
   1982.4 |########################################
   1990.3 |
   1998.1 |
   2005.9 |
   2013.8 |
   2021.6 |
   2029.4 |
   2037.3 |
   2045.1 |########################################
   2052.9 |
   2060.8 |########################################
   2068.6 |
   2076.4 |
   2084.3 |
   2092.1 |
   2099.9 |
   2107.8 |########################################
   2115.6 |
   2123.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_scatter_fntable**: bridge=2709.0% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_null**: bridge=6899.1% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_switch**: bridge=4339.8% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_threaded**: bridge=4133.1% of algo (FFI overhead may distort results)
