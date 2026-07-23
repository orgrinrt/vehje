# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), real profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_real_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_real_null dominates: 46% faster than the next best (carrier_cold_real_threaded)

carrier_cold_real_null (1.37 us) leads carrier_cold_real_threaded (2.00 us) by 46%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_real_null beats baseline by 36% (significant)

carrier_cold_real_null is -761 ns (36%) faster than baseline carrier_cold_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_real_fntable is an outlier: 2.2x slower than the field

carrier_cold_real_fntable (2.97 us) is 2.2x the fastest (1.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_real_null} vs {carrier_cold_real_threaded, carrier_cold_real_switch, carrier_cold_real_fntable} (46% apart)

The field splits into a fast tier {carrier_cold_real_null} and a slow tier {carrier_cold_real_threaded, carrier_cold_real_switch, carrier_cold_real_fntable} with a 46% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_real_null** at 1370.2 ns median (-36.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.17x (fastest 1370.2 ns, slowest 2967.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_real_fntable | 5334ns | 5343ns | 5062ns | 5277ns | 5557ns | +19.82% |
| carrier_cold_real_null | 3716ns | 3700ns | 3457ns | 3659ns | 3930ns | -16.54% |
| carrier_cold_real_switch | 4452ns | 4496ns | 4207ns | 4413ns | 4634ns | base |
| carrier_cold_real_threaded | 4321ns | 4242ns | 4148ns | 4225ns | 4550ns | -2.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_real_fntable | 2998ns | 2860ns | 3149ns | +41.51% | 0.021 |
| carrier_cold_real_null | 1354ns | 1256ns | 1403ns | -36.06% | 0.047 |
| carrier_cold_real_switch | 2118ns | 2003ns | 2203ns | base | 0.030 |
| carrier_cold_real_threaded | 2020ns | 1955ns | 2104ns | -4.63% | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_real_fntable | 268400 | 736369 | 0.364 | 0.96× |
| carrier_cold_real_null | 258861 | 1290507 | 0.201 | 0.92× |
| carrier_cold_real_switch | 280028 | 902085 | 0.310 | 1.00× |
| carrier_cold_real_threaded | 272423 | 944078 | 0.289 | 0.97× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.051 Gops/s** (carrier_cold_real_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_real_fntable | 0.022 | 42.3% |
| carrier_cold_real_null | 0.047 | 91.6% |
| carrier_cold_real_switch | 0.030 | 58.6% |
| carrier_cold_real_threaded | 0.032 | 62.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_real_fntable | 5334ns | 5334ns | +19.82% |
| carrier_cold_real_null | 3716ns | 3716ns | -16.54% |
| carrier_cold_real_switch | 4452ns | 4452ns | base |
| carrier_cold_real_threaded | 4321ns | 4321ns | -2.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_real_switch | 2144ns | base | --- | [2008, 2203] | --- | --- | --- | --- |
| carrier_cold_real_fntable | 2968ns | +852.5ns (+39.8%) | [+807, +979]ns | [2876, 3149] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_real_null | 1370ns | -761.1ns (-35.5%) | [-847, -684]ns | [1290, 1403] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_real_threaded | 1996ns | -91.4ns (-4.3%) | [-186, -17]ns | [1961, 2104] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_real_switch | carrier_cold_real_fntable | carrier_cold_real_null | carrier_cold_real_threaded |
|---|---|---|---|---|
| 1 | 2136ns | +50.0% | -33.9% | -8.5% |
| 2 | 2235ns | +38.4% | -38.8% | -8.5% |
| 3 | 2151ns | +37.8% | -38.5% | -6.3% |
| 4 | 2003ns | +44.4% | -37.3% | -1.3% |
| 5 | 2014ns | +42.0% | -31.9% | -2.3% |
| 6 | 2170ns | +36.9% | -35.7% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_real_fntable | 0.459 | moderate+ |
| carrier_cold_real_null | 0.149 | ok |
| carrier_cold_real_switch | 0.208 | moderate+ |
| carrier_cold_real_threaded | -0.225 | moderate- |

**Consistency summary:**

- **carrier_cold_real_fntable**: won 0/6, lost 6/6
- **carrier_cold_real_null**: won 6/6, lost 0/6
- **carrier_cold_real_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_real_fntable | 83705.3ns | 2997.5ns | 2792.5% | HIGH |
| carrier_cold_real_null | 83594.5ns | 1354.3ns | 6172.4% | HIGH |
| carrier_cold_real_switch | 90276.8ns | 2118.2ns | 4262.0% | HIGH |
| carrier_cold_real_threaded | 85100.2ns | 2020.1ns | 4212.6% | HIGH |

## Distribution (algo ns)

```
carrier_cold_real_fntable (n=6, range 2859.6-3149.4 ns)
   2859.6 |####################
   2874.1 |
   2888.6 |####################
   2903.1 |
   2917.6 |
   2932.0 |
   2946.5 |
   2961.0 |########################################
   2975.5 |
   2990.0 |
   3004.5 |
   3019.0 |
   3033.5 |
   3048.0 |
   3062.5 |
   3076.9 |
   3091.4 |####################
   3105.9 |
   3120.4 |
   3134.9 |
  (0 below, 1 above range)

carrier_cold_real_null (n=6, range 1255.8-1402.9 ns)
   1255.8 |####################
   1263.2 |
   1270.5 |
   1277.9 |
   1285.2 |
   1292.6 |
   1299.9 |
   1307.3 |
   1314.6 |
   1322.0 |####################
   1329.3 |
   1336.7 |
   1344.1 |
   1351.4 |
   1358.8 |
   1366.1 |########################################
   1373.5 |
   1380.8 |
   1388.2 |####################
   1395.5 |
  (0 below, 1 above range)

carrier_cold_real_switch (n=6, range 2002.9-2202.7 ns)
   2002.9 |########################################
   2012.9 |########################################
   2022.9 |
   2032.9 |
   2042.9 |
   2052.8 |
   2062.8 |
   2072.8 |
   2082.8 |
   2092.8 |
   2102.8 |
   2112.8 |
   2122.8 |
   2132.8 |########################################
   2142.8 |########################################
   2152.8 |
   2162.7 |########################################
   2172.7 |
   2182.7 |
   2192.7 |
  (0 below, 1 above range)

carrier_cold_real_threaded (n=6, range 1955.0-2103.6 ns)
   1955.0 |########################################
   1962.4 |########################################
   1969.9 |########################################
   1977.3 |
   1984.7 |
   1992.1 |
   1999.6 |
   2007.0 |
   2014.4 |########################################
   2021.8 |
   2029.3 |
   2036.7 |
   2044.1 |########################################
   2051.6 |
   2059.0 |
   2066.4 |
   2073.8 |
   2081.3 |
   2088.7 |
   2096.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_real_fntable**: bridge=2875.4% of algo (FFI overhead may distort results)
- **carrier_cold_real_null**: bridge=6044.6% of algo (FFI overhead may distort results)
- **carrier_cold_real_switch**: bridge=4207.7% of algo (FFI overhead may distort results)
- **carrier_cold_real_threaded**: bridge=4265.0% of algo (FFI overhead may distort results)
