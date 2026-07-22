# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 17% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (1.40 us) leads carrier_pre_real_direct (1.63 us) by 17%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 35% (significant)

carrier_pre_real_null is -750 ns (35%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 2.2x slower than the field

carrier_pre_real_fntable (3.07 us) is 2.2x the fastest (1.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} vs {carrier_pre_real_fntable} (25% apart)

The field splits into a fast tier {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} and a slow tier {carrier_pre_real_fntable} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_real_null** at 1397.7 ns median (-34.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.20x (fastest 1397.7 ns, slowest 3071.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 4121ns | 4107ns | 4050ns | 4091ns | 4200ns | -10.52% |
| carrier_pre_real_fntable | 5435ns | 5565ns | 4808ns | 5540ns | 5593ns | +18.02% |
| carrier_pre_real_null | 3885ns | 3870ns | 3830ns | 3863ns | 3946ns | -15.64% |
| carrier_pre_real_regcache | 4903ns | 4895ns | 4892ns | 4894ns | 4922ns | +6.47% |
| carrier_pre_real_switch | 4605ns | 4613ns | 4552ns | 4596ns | 4646ns | base |
| carrier_pre_real_threaded | 4415ns | 4396ns | 4355ns | 4388ns | 4486ns | -4.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 1637ns | 1603ns | 1669ns | -23.49% | 0.039 |
| carrier_pre_real_fntable | 3006ns | 2669ns | 3090ns | +40.50% | 0.021 |
| carrier_pre_real_null | 1402ns | 1387ns | 1419ns | -34.47% | 0.046 |
| carrier_pre_real_regcache | 2455ns | 2443ns | 2464ns | +14.73% | 0.026 |
| carrier_pre_real_switch | 2140ns | 2115ns | 2156ns | base | 0.030 |
| carrier_pre_real_threaded | 1907ns | 1882ns | 1937ns | -10.89% | 0.034 |

## Performance model

- Peak throughput: **0.046 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.039 | 84.8% |
| carrier_pre_real_fntable | 0.021 | 45.1% |
| carrier_pre_real_null | 0.046 | 99.2% |
| carrier_pre_real_regcache | 0.026 | 56.5% |
| carrier_pre_real_switch | 0.030 | 64.7% |
| carrier_pre_real_threaded | 0.034 | 73.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 4121ns | 4121ns | -10.52% |
| carrier_pre_real_fntable | 5435ns | 5435ns | +18.02% |
| carrier_pre_real_null | 3885ns | 3885ns | -15.64% |
| carrier_pre_real_regcache | 4903ns | 4903ns | +6.47% |
| carrier_pre_real_switch | 4605ns | 4605ns | base |
| carrier_pre_real_threaded | 4415ns | 4415ns | -4.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 2145ns | base | --- | [2118, 2156] | --- | --- | --- | --- |
| carrier_pre_real_direct | 1635ns | -505.4ns (-23.6%) | [-518, -485]ns | [1608, 1669] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_fntable | 3071ns | +939.2ns (+43.8%) | [+704, +956]ns | [2857, 3090] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_null | 1398ns | -749.8ns (-35.0%) | [-751, -712]ns | [1390, 1419] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_regcache | 2453ns | +316.9ns (+14.8%) | [+292, +336]ns | [2447, 2464] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_threaded | 1898ns | -231.2ns (-10.8%) | [-260, -208]ns | [1884, 1937] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 2143ns | -23.6% | +24.5% | -35.0% | +14.4% | -12.2% |
| 2 | 2121ns | -24.4% | +45.4% | -32.8% | +15.6% | -10.7% |
| 3 | 2150ns | -24.1% | +43.4% | -34.8% | +14.1% | -8.8% |
| 4 | 2162ns | -21.6% | +40.8% | -34.7% | +13.0% | -12.0% |
| 5 | 2146ns | -23.5% | +44.3% | -35.0% | +15.1% | -10.9% |
| 6 | 2115ns | -23.8% | +44.7% | -34.4% | +16.2% | -10.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | 0.007 | ok |
| carrier_pre_real_fntable | -0.063 | ok |
| carrier_pre_real_null | -0.218 | moderate- |
| carrier_pre_real_regcache | -0.325 | moderate- |
| carrier_pre_real_switch | -0.025 | ok |
| carrier_pre_real_threaded | -0.178 | ok |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 85416.5ns | 1637.0ns | 5218.0% | HIGH |
| carrier_pre_real_fntable | 87387.8ns | 3006.1ns | 2907.0% | HIGH |
| carrier_pre_real_null | 86171.3ns | 1402.1ns | 6145.9% | HIGH |
| carrier_pre_real_regcache | 86030.6ns | 2454.7ns | 3504.7% | HIGH |
| carrier_pre_real_switch | 86071.0ns | 2139.6ns | 4022.7% | HIGH |
| carrier_pre_real_threaded | 86604.1ns | 1906.6ns | 4542.3% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 1602.9-1668.6 ns)
   1602.9 |########################################
   1606.2 |
   1609.5 |########################################
   1612.7 |
   1616.0 |
   1619.3 |
   1622.6 |
   1625.9 |
   1629.2 |########################################
   1632.4 |
   1635.7 |########################################
   1639.0 |########################################
   1642.3 |
   1645.6 |
   1648.9 |
   1652.1 |
   1655.4 |
   1658.7 |
   1662.0 |
   1665.3 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 2669.2-3090.0 ns)
   2669.2 |####################
   2690.2 |
   2711.3 |
   2732.3 |
   2753.4 |
   2774.4 |
   2795.4 |
   2816.5 |
   2837.5 |
   2858.6 |
   2879.6 |
   2900.6 |
   2921.7 |
   2942.7 |
   2963.8 |
   2984.8 |
   3005.8 |
   3026.9 |####################
   3047.9 |####################
   3069.0 |########################################
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 1386.7-1419.0 ns)
   1386.7 |########################################
   1388.3 |
   1389.9 |
   1391.5 |########################################
   1393.2 |########################################
   1394.8 |
   1396.4 |
   1398.0 |
   1399.6 |########################################
   1401.2 |
   1402.8 |
   1404.4 |
   1406.0 |
   1407.7 |
   1409.3 |
   1410.9 |########################################
   1412.5 |
   1414.1 |
   1415.7 |
   1417.3 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 2442.9-2463.9 ns)
   2442.9 |####################
   2444.0 |
   2445.0 |
   2446.1 |
   2447.1 |
   2448.2 |
   2449.2 |
   2450.3 |
   2451.3 |########################################
   2452.4 |
   2453.4 |####################
   2454.5 |
   2455.5 |
   2456.6 |####################
   2457.6 |
   2458.7 |
   2459.7 |
   2460.8 |
   2461.8 |
   2462.9 |
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 2115.0-2156.1 ns)
   2115.0 |########################################
   2117.1 |
   2119.1 |
   2121.2 |########################################
   2123.2 |
   2125.3 |
   2127.3 |
   2129.4 |
   2131.4 |
   2133.5 |
   2135.5 |
   2137.6 |
   2139.6 |
   2141.7 |########################################
   2143.7 |
   2145.8 |########################################
   2147.8 |
   2149.9 |########################################
   2151.9 |
   2154.0 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 1882.5-1937.1 ns)
   1882.5 |########################################
   1885.2 |########################################
   1888.0 |
   1890.7 |
   1893.4 |########################################
   1896.2 |
   1898.9 |
   1901.6 |########################################
   1904.3 |
   1907.1 |
   1909.8 |########################################
   1912.5 |
   1915.3 |
   1918.0 |
   1920.7 |
   1923.4 |
   1926.2 |
   1928.9 |
   1931.6 |
   1934.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=5257.8% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=2873.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=6161.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=3509.8% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=3998.9% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=4562.4% of algo (FFI overhead may distort results)
