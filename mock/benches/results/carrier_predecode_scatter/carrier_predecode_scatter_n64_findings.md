# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null beats baseline by 34% (significant)

carrier_pre_scatter_null is -730 ns (34%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 2.2x slower than the field

carrier_pre_scatter_fntable (3.18 us) is 2.2x the fastest (1.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_scatter_threaded shows alternating (throttle bounce) (autocorr -0.53)

carrier_pre_scatter_threaded's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} vs {carrier_pre_scatter_fntable} (31% apart)

The field splits into a fast tier {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} and a slow tier {carrier_pre_scatter_fntable} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 1437.9 ns median (-33.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.21x (fastest 1437.9 ns, slowest 3183.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 4012ns | 4012ns | 3881ns | 4000ns | 4094ns | -12.16% |
| carrier_pre_scatter_fntable | 5505ns | 5643ns | 4794ns | 5634ns | 5668ns | +20.55% |
| carrier_pre_scatter_null | 3893ns | 3888ns | 3856ns | 3884ns | 3925ns | -14.76% |
| carrier_pre_scatter_regcache | 4934ns | 4935ns | 4897ns | 4927ns | 4962ns | +8.03% |
| carrier_pre_scatter_switch | 4567ns | 4631ns | 4142ns | 4620ns | 4700ns | base |
| carrier_pre_scatter_threaded | 4301ns | 4310ns | 4238ns | 4301ns | 4334ns | -5.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 1538ns | 1478ns | 1576ns | -28.06% | 0.042 |
| carrier_pre_scatter_fntable | 3104ns | 2711ns | 3193ns | +45.19% | 0.021 |
| carrier_pre_scatter_null | 1440ns | 1427ns | 1452ns | -32.66% | 0.044 |
| carrier_pre_scatter_regcache | 2446ns | 2433ns | 2466ns | +14.42% | 0.026 |
| carrier_pre_scatter_switch | 2138ns | 1938ns | 2204ns | base | 0.030 |
| carrier_pre_scatter_threaded | 1821ns | 1797ns | 1834ns | -14.83% | 0.035 |

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.042 | 92.6% |
| carrier_pre_scatter_fntable | 0.020 | 44.8% |
| carrier_pre_scatter_null | 0.045 | 99.2% |
| carrier_pre_scatter_regcache | 0.026 | 58.5% |
| carrier_pre_scatter_switch | 0.030 | 65.9% |
| carrier_pre_scatter_threaded | 0.035 | 78.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 4012ns | 4012ns | -12.16% |
| carrier_pre_scatter_fntable | 5505ns | 5505ns | +20.55% |
| carrier_pre_scatter_null | 3893ns | 3893ns | -14.76% |
| carrier_pre_scatter_regcache | 4934ns | 4934ns | +8.03% |
| carrier_pre_scatter_switch | 4567ns | 4567ns | base |
| carrier_pre_scatter_threaded | 4301ns | 4301ns | -5.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 2164ns | base | --- | [2047, 2204] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 1540ns | -619.6ns (-28.6%) | [-698, -482]ns | [1498, 1576] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 3184ns | +999.0ns (+46.2%) | [+867, +1033]ns | [2936, 3193] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_null | 1438ns | -730.0ns (-33.7%) | [-758, -607]ns | [1430, 1452] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 2439ns | +285.0ns (+13.2%) | [+232, +408]ns | [2434, 2466] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_threaded | 1822ns | -334.6ns (-15.5%) | [-395, -222]ns | [1807, 1834] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 1938ns | -18.1% | +39.9% | -25.8% | +25.9% | -6.3% |
| 2 | 2163ns | -28.8% | +46.2% | -34.0% | +14.5% | -14.9% |
| 3 | 2178ns | -28.2% | +45.9% | -34.0% | +11.7% | -17.5% |
| 4 | 2229ns | -31.8% | +43.1% | -34.8% | +9.4% | -18.3% |
| 5 | 2164ns | -31.7% | +47.6% | -33.0% | +12.5% | -15.8% |
| 6 | 2156ns | -28.5% | +48.1% | -33.6% | +13.9% | -15.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | 0.078 | ok |
| carrier_pre_scatter_fntable | 0.017 | ok |
| carrier_pre_scatter_null | 0.181 | ok |
| carrier_pre_scatter_regcache | -0.335 | moderate- |
| carrier_pre_scatter_switch | 0.047 | ok |
| carrier_pre_scatter_threaded | -0.534 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 0/6, lost 6/6
- **carrier_pre_scatter_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 86831.9ns | 1538.1ns | 5645.3% | HIGH |
| carrier_pre_scatter_fntable | 87026.8ns | 3104.2ns | 2803.5% | HIGH |
| carrier_pre_scatter_null | 85686.0ns | 1439.7ns | 5951.5% | HIGH |
| carrier_pre_scatter_regcache | 87336.0ns | 2446.2ns | 3570.3% | HIGH |
| carrier_pre_scatter_switch | 86322.1ns | 2138.0ns | 4037.5% | HIGH |
| carrier_pre_scatter_threaded | 86445.8ns | 1820.9ns | 4747.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 1477.5-1576.0 ns)
   1477.5 |####################
   1482.4 |
   1487.4 |
   1492.3 |
   1497.2 |
   1502.1 |
   1507.1 |
   1512.0 |
   1516.9 |####################
   1521.8 |
   1526.8 |
   1531.7 |
   1536.6 |########################################
   1541.6 |
   1546.5 |
   1551.4 |
   1556.3 |
   1561.3 |####################
   1566.2 |
   1571.1 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 2710.8-3192.7 ns)
   2710.8 |#############
   2734.9 |
   2759.0 |
   2783.1 |
   2807.2 |
   2831.3 |
   2855.4 |
   2879.5 |
   2903.6 |
   2927.7 |
   2951.8 |
   2975.8 |
   2999.9 |
   3024.0 |
   3048.1 |
   3072.2 |
   3096.3 |
   3120.4 |
   3144.5 |#############
   3168.6 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 1426.7-1451.7 ns)
   1426.7 |########################################
   1428.0 |
   1429.2 |
   1430.5 |
   1431.7 |########################################
   1433.0 |
   1434.2 |
   1435.5 |
   1436.7 |########################################
   1438.0 |########################################
   1439.2 |
   1440.4 |
   1441.7 |
   1442.9 |
   1444.2 |
   1445.4 |
   1446.7 |
   1447.9 |
   1449.2 |########################################
   1450.4 |
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 2432.9-2466.1 ns)
   2432.9 |########################################
   2434.6 |########################################
   2436.2 |########################################
   2437.9 |
   2439.5 |########################################
   2441.2 |
   2442.8 |
   2444.5 |
   2446.2 |
   2447.8 |
   2449.5 |
   2451.1 |
   2452.8 |
   2454.4 |########################################
   2456.1 |
   2457.8 |
   2459.4 |
   2461.1 |
   2462.7 |
   2464.4 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 1937.9-2203.6 ns)
   1937.9 |####################
   1951.2 |
   1964.5 |
   1977.7 |
   1991.0 |
   2004.3 |
   2017.6 |
   2030.9 |
   2044.2 |
   2057.4 |
   2070.7 |
   2084.0 |
   2097.3 |
   2110.6 |
   2123.9 |
   2137.1 |
   2150.4 |########################################
   2163.7 |####################
   2177.0 |####################
   2190.3 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 1797.1-1834.3 ns)
   1797.1 |########################################
   1799.0 |
   1800.8 |
   1802.7 |
   1804.5 |
   1806.4 |
   1808.3 |
   1810.1 |
   1812.0 |
   1813.9 |
   1815.7 |########################################
   1817.6 |
   1819.4 |########################################
   1821.3 |########################################
   1823.2 |
   1825.0 |
   1826.9 |########################################
   1828.8 |
   1830.6 |
   1832.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=5641.2% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=2739.0% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=5980.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=3576.2% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=4010.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=4746.8% of algo (FFI overhead may distort results)
