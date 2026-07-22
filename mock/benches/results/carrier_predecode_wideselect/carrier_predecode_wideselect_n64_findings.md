# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null beats baseline by 35% (significant)

carrier_pre_wideselect_null is -755 ns (35%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_fntable is an outlier: 2.2x slower than the field

carrier_pre_wideselect_fntable (2.96 us) is 2.2x the fastest (1.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_wideselect_switch shows alternating (throttle bounce) (autocorr -0.78)

carrier_pre_wideselect_switch's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_wideselect_null, carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache} vs {carrier_pre_wideselect_fntable} (27% apart)

The field splits into a fast tier {carrier_pre_wideselect_null, carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache} and a slow tier {carrier_pre_wideselect_fntable} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 1361.9 ns median (-36.0% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.17x (fastest 1361.9 ns, slowest 2956.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 4026ns | 3984ns | 3930ns | 3977ns | 4148ns | -12.60% |
| carrier_pre_wideselect_fntable | 5324ns | 5438ns | 4695ns | 5404ns | 5520ns | +15.58% |
| carrier_pre_wideselect_null | 3827ns | 3832ns | 3752ns | 3814ns | 3884ns | -16.92% |
| carrier_pre_wideselect_regcache | 4819ns | 4803ns | 4712ns | 4791ns | 4916ns | +4.62% |
| carrier_pre_wideselect_switch | 4607ns | 4607ns | 4573ns | 4600ns | 4633ns | base |
| carrier_pre_wideselect_threaded | 4300ns | 4249ns | 4235ns | 4245ns | 4414ns | -6.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1498ns | 1465ns | 1544ns | -29.64% | 0.043 |
| carrier_pre_wideselect_fntable | 2893ns | 2548ns | 2994ns | +35.92% | 0.022 |
| carrier_pre_wideselect_null | 1360ns | 1340ns | 1378ns | -36.09% | 0.047 |
| carrier_pre_wideselect_regcache | 2336ns | 2280ns | 2391ns | +9.74% | 0.027 |
| carrier_pre_wideselect_switch | 2129ns | 2112ns | 2143ns | base | 0.030 |
| carrier_pre_wideselect_threaded | 1808ns | 1786ns | 1843ns | -15.08% | 0.035 |

## Performance model

- Peak throughput: **0.048 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.043 | 90.5% |
| carrier_pre_wideselect_fntable | 0.022 | 45.3% |
| carrier_pre_wideselect_null | 0.047 | 98.4% |
| carrier_pre_wideselect_regcache | 0.028 | 57.7% |
| carrier_pre_wideselect_switch | 0.030 | 62.9% |
| carrier_pre_wideselect_threaded | 0.036 | 74.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 4026ns | 4026ns | -12.60% |
| carrier_pre_wideselect_fntable | 5324ns | 5324ns | +15.58% |
| carrier_pre_wideselect_null | 3827ns | 3827ns | -16.92% |
| carrier_pre_wideselect_regcache | 4819ns | 4819ns | +4.62% |
| carrier_pre_wideselect_switch | 4607ns | 4607ns | base |
| carrier_pre_wideselect_threaded | 4300ns | 4300ns | -6.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 2129ns | base | --- | [2114, 2143] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 1481ns | -649.8ns (-30.5%) | [-668, -575]ns | [1468, 1544] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 2956ns | +826.6ns (+38.8%) | [+589, +878]ns | [2729, 2994] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 1362ns | -755.2ns (-35.5%) | [-799, -751]ns | [1342, 1378] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 2321ns | +191.2ns (+9.0%) | [+166, +264]ns | [2296, 2391] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 1792ns | -327.3ns (-15.4%) | [-344, -291]ns | [1788, 1843] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 2144ns | -31.0% | +18.8% | -37.5% | +8.1% | -12.2% |
| 2 | 2117ns | -26.1% | +39.3% | -35.4% | +7.7% | -15.4% |
| 3 | 2142ns | -30.8% | +38.4% | -35.2% | +7.9% | -15.8% |
| 4 | 2112ns | -30.3% | +42.0% | -35.7% | +10.1% | -15.2% |
| 5 | 2137ns | -31.4% | +36.2% | -37.1% | +14.8% | -16.4% |
| 6 | 2121ns | -28.2% | +40.9% | -35.6% | +9.9% | -15.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.244 | moderate- |
| carrier_pre_wideselect_fntable | -0.029 | ok |
| carrier_pre_wideselect_null | -0.032 | ok |
| carrier_pre_wideselect_regcache | 0.044 | ok |
| carrier_pre_wideselect_switch | -0.783 | HIGH- (thermal bounce) |
| carrier_pre_wideselect_threaded | -0.057 | ok |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 86749.5ns | 1497.7ns | 5792.2% | HIGH |
| carrier_pre_wideselect_fntable | 87180.9ns | 2893.2ns | 3013.3% | HIGH |
| carrier_pre_wideselect_null | 85893.4ns | 1360.4ns | 6314.0% | HIGH |
| carrier_pre_wideselect_regcache | 86497.0ns | 2336.0ns | 3702.7% | HIGH |
| carrier_pre_wideselect_switch | 86082.0ns | 2128.7ns | 4043.9% | HIGH |
| carrier_pre_wideselect_threaded | 86056.5ns | 1807.8ns | 4760.4% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 1465.4-1544.0 ns)
   1465.4 |########################################
   1469.3 |########################################
   1473.3 |
   1477.2 |########################################
   1481.1 |########################################
   1485.0 |
   1489.0 |
   1492.9 |
   1496.8 |
   1500.7 |
   1504.7 |
   1508.6 |
   1512.5 |
   1516.5 |
   1520.4 |########################################
   1524.3 |
   1528.2 |
   1532.2 |
   1536.1 |
   1540.0 |
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 2547.9-2994.4 ns)
   2547.9 |########################################
   2570.2 |
   2592.6 |
   2614.9 |
   2637.2 |
   2659.5 |
   2681.8 |
   2704.2 |
   2726.5 |
   2748.8 |
   2771.1 |
   2793.5 |
   2815.8 |
   2838.1 |
   2860.4 |
   2882.8 |
   2905.1 |########################################
   2927.4 |########################################
   2949.7 |########################################
   2972.1 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 1339.6-1377.5 ns)
   1339.6 |########################################
   1341.5 |
   1343.4 |########################################
   1345.3 |
   1347.2 |
   1349.1 |
   1351.0 |
   1352.9 |
   1354.8 |
   1356.7 |
   1358.5 |########################################
   1360.4 |
   1362.3 |
   1364.2 |########################################
   1366.1 |########################################
   1368.0 |
   1369.9 |
   1371.8 |
   1373.7 |
   1375.6 |
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 2280.4-2391.2 ns)
   2280.4 |########################################
   2285.9 |
   2291.5 |
   2297.0 |
   2302.6 |
   2308.1 |########################################
   2313.7 |########################################
   2319.2 |
   2324.7 |########################################
   2330.3 |########################################
   2335.8 |
   2341.4 |
   2346.9 |
   2352.5 |
   2358.0 |
   2363.5 |
   2369.1 |
   2374.6 |
   2380.2 |
   2385.7 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 2111.7-2142.9 ns)
   2111.7 |########################################
   2113.3 |
   2114.8 |
   2116.4 |########################################
   2117.9 |
   2119.5 |########################################
   2121.1 |
   2122.6 |
   2124.2 |
   2125.8 |
   2127.3 |
   2128.9 |
   2130.4 |
   2132.0 |
   2133.6 |
   2135.1 |
   2136.7 |########################################
   2138.3 |
   2139.8 |
   2141.4 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 1786.2-1843.3 ns)
   1786.2 |####################
   1789.1 |########################################
   1791.9 |####################
   1794.8 |
   1797.6 |
   1800.5 |
   1803.3 |####################
   1806.2 |
   1809.0 |
   1811.9 |
   1814.8 |
   1817.6 |
   1820.5 |
   1823.3 |
   1826.2 |
   1829.0 |
   1831.9 |
   1834.7 |
   1837.6 |
   1840.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=5865.8% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=2949.8% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=6312.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=3713.5% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=4045.3% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=4820.7% of algo (FFI overhead may distort results)
