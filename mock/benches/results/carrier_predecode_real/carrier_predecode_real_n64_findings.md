# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 15% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (1.39 us) leads carrier_pre_real_direct (1.60 us) by 15%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 34% (significant)

carrier_pre_real_null is -721 ns (34%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 2.2x slower than the field

carrier_pre_real_fntable (3.03 us) is 2.2x the fastest (1.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_pre_real_null** at 1393.5 ns median (-34.3% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.17x (fastest 1393.5 ns, slowest 3026.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 4050ns | 4039ns | 4011ns | 4032ns | 4095ns | -10.85% |
| carrier_pre_real_fntable | 5385ns | 5436ns | 4793ns | 5405ns | 5651ns | +18.53% |
| carrier_pre_real_null | 3821ns | 3807ns | 3791ns | 3806ns | 3859ns | -15.89% |
| carrier_pre_real_regcache | 4932ns | 4934ns | 4826ns | 4909ns | 5021ns | +8.57% |
| carrier_pre_real_switch | 4543ns | 4549ns | 4496ns | 4543ns | 4568ns | base |
| carrier_pre_real_threaded | 4334ns | 4320ns | 4298ns | 4315ns | 4381ns | -4.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 1608ns | 1597ns | 1625ns | -23.96% | 0.040 |
| carrier_pre_real_fntable | 2998ns | 2669ns | 3149ns | +41.76% | 0.021 |
| carrier_pre_real_null | 1398ns | 1378ns | 1418ns | -33.90% | 0.046 |
| carrier_pre_real_regcache | 2476ns | 2432ns | 2505ns | +17.06% | 0.026 |
| carrier_pre_real_switch | 2115ns | 2084ns | 2126ns | base | 0.030 |
| carrier_pre_real_threaded | 1886ns | 1870ns | 1905ns | -10.84% | 0.034 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_real_direct | 249992 | 852892 | 0.293 | 1.00× |
| carrier_pre_real_fntable | 263536 | 769608 | 0.342 | 1.06× |
| carrier_pre_real_null | 251881 | 1308265 | 0.193 | 1.01× |
| carrier_pre_real_regcache | 255143 | 1028726 | 0.248 | 1.02× |
| carrier_pre_real_switch | 249746 | 851377 | 0.293 | 1.00× |
| carrier_pre_real_threaded | 249771 | 1014471 | 0.246 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.046 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.040 | 86.0% |
| carrier_pre_real_fntable | 0.021 | 45.5% |
| carrier_pre_real_null | 0.046 | 98.8% |
| carrier_pre_real_regcache | 0.026 | 55.5% |
| carrier_pre_real_switch | 0.030 | 65.0% |
| carrier_pre_real_threaded | 0.034 | 73.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 4050ns | 4050ns | -10.85% |
| carrier_pre_real_fntable | 5385ns | 5385ns | +18.53% |
| carrier_pre_real_null | 3821ns | 3821ns | -15.89% |
| carrier_pre_real_regcache | 4932ns | 4932ns | +8.57% |
| carrier_pre_real_switch | 4543ns | 4543ns | base |
| carrier_pre_real_threaded | 4334ns | 4334ns | -4.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 2121ns | base | --- | [2098, 2126] | --- | --- | --- | --- |
| carrier_pre_real_direct | 1602ns | -511.7ns (-24.1%) | [-524, -485]ns | [1597, 1625] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_fntable | 3027ns | +907.0ns (+42.8%) | [+699, +1044]ns | [2819, 3149] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_null | 1394ns | -721.0ns (-34.0%) | [-742, -688]ns | [1382, 1418] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_regcache | 2483ns | +362.0ns (+17.1%) | [+321, +399]ns | [2440, 2505] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_threaded | 1880ns | -233.3ns (-11.0%) | [-254, -201]ns | [1872, 1905] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 2125ns | -22.9% | +25.6% | -35.2% | +17.3% | -9.3% |
| 2 | 2125ns | -24.8% | +46.5% | -34.2% | +15.2% | -12.0% |
| 3 | 2117ns | -24.6% | +40.3% | -33.8% | +16.9% | -11.2% |
| 4 | 2084ns | -23.2% | +52.8% | -33.5% | +20.3% | -9.8% |
| 5 | 2127ns | -24.1% | +42.3% | -34.7% | +17.7% | -11.9% |
| 6 | 2112ns | -24.1% | +43.3% | -32.1% | +15.2% | -10.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | -0.153 | ok |
| carrier_pre_real_fntable | -0.255 | moderate- |
| carrier_pre_real_null | -0.124 | ok |
| carrier_pre_real_regcache | -0.169 | ok |
| carrier_pre_real_switch | -0.273 | moderate- |
| carrier_pre_real_threaded | -0.202 | moderate- |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 86413.8ns | 1608.1ns | 5373.5% | HIGH |
| carrier_pre_real_fntable | 87826.3ns | 2998.1ns | 2929.4% | HIGH |
| carrier_pre_real_null | 86724.8ns | 1397.8ns | 6204.2% | HIGH |
| carrier_pre_real_regcache | 87669.9ns | 2475.8ns | 3541.1% | HIGH |
| carrier_pre_real_switch | 85277.4ns | 2114.9ns | 4032.3% | HIGH |
| carrier_pre_real_threaded | 86702.6ns | 1885.6ns | 4598.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 1596.7-1625.4 ns)
   1596.7 |########################################
   1598.1 |
   1599.6 |
   1601.0 |########################################
   1602.4 |
   1603.9 |
   1605.3 |
   1606.7 |
   1608.2 |
   1609.6 |
   1611.1 |
   1612.5 |####################
   1613.9 |
   1615.4 |
   1616.8 |
   1618.2 |
   1619.7 |
   1621.1 |
   1622.5 |
   1624.0 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 2669.2-3148.6 ns)
   2669.2 |####################
   2693.2 |
   2717.1 |
   2741.1 |
   2765.1 |
   2789.0 |
   2813.0 |
   2837.0 |
   2860.9 |
   2884.9 |
   2908.9 |
   2932.8 |
   2956.8 |####################
   2980.8 |
   3004.7 |########################################
   3028.7 |
   3052.7 |
   3076.6 |
   3100.6 |####################
   3124.6 |
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 1377.5-1418.2 ns)
   1377.5 |########################################
   1379.5 |
   1381.6 |
   1383.6 |
   1385.6 |########################################
   1387.7 |########################################
   1389.7 |
   1391.7 |
   1393.8 |
   1395.8 |########################################
   1397.8 |
   1399.9 |########################################
   1401.9 |
   1403.9 |
   1406.0 |
   1408.0 |
   1410.0 |
   1412.1 |
   1414.1 |
   1416.1 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 2432.1-2504.8 ns)
   2432.1 |########################################
   2435.7 |
   2439.4 |
   2443.0 |
   2446.6 |########################################
   2450.3 |
   2453.9 |
   2457.5 |
   2461.2 |
   2464.8 |
   2468.4 |
   2472.1 |########################################
   2475.7 |
   2479.4 |
   2483.0 |
   2486.6 |
   2490.3 |########################################
   2493.9 |
   2497.5 |
   2501.2 |########################################
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 2084.2-2125.8 ns)
   2084.2 |####################
   2086.3 |
   2088.4 |
   2090.4 |
   2092.5 |
   2094.6 |
   2096.7 |
   2098.8 |
   2100.9 |
   2102.9 |
   2105.0 |
   2107.1 |
   2109.2 |
   2111.3 |####################
   2113.4 |
   2115.4 |####################
   2117.5 |
   2119.6 |
   2121.7 |
   2123.8 |########################################
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 1869.6-1905.0 ns)
   1869.6 |########################################
   1871.4 |
   1873.1 |########################################
   1874.9 |
   1876.7 |
   1878.4 |########################################
   1880.2 |########################################
   1882.0 |########################################
   1883.8 |
   1885.5 |
   1887.3 |
   1889.1 |
   1890.8 |
   1892.6 |
   1894.4 |
   1896.2 |
   1897.9 |
   1899.7 |
   1901.5 |
   1903.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=5393.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=2911.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=6192.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=3532.0% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=4019.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=4608.4% of algo (FFI overhead may distort results)
