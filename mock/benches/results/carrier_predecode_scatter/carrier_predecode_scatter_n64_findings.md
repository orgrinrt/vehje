# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 10% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (1.34 us) leads carrier_pre_scatter_direct (1.47 us) by 10%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 35% (significant)

carrier_pre_scatter_null is -701 ns (35%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 2.2x slower than the field

carrier_pre_scatter_fntable (2.93 us) is 2.2x the fastest (1.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_scatter_fntable shows alternating (throttle bounce) (autocorr -0.68)

carrier_pre_scatter_fntable's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} vs {carrier_pre_scatter_fntable} (27% apart)

The field splits into a fast tier {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} and a slow tier {carrier_pre_scatter_fntable} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 1335.7 ns median (-33.2% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.19x (fastest 1335.7 ns, slowest 2929.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 3782ns | 3796ns | 3642ns | 3748ns | 3904ns | -12.62% |
| carrier_pre_scatter_fntable | 5198ns | 5185ns | 5013ns | 5160ns | 5346ns | +20.08% |
| carrier_pre_scatter_null | 3635ns | 3624ns | 3548ns | 3608ns | 3719ns | -16.02% |
| carrier_pre_scatter_regcache | 4576ns | 4583ns | 4443ns | 4541ns | 4695ns | +5.72% |
| carrier_pre_scatter_switch | 4328ns | 4331ns | 4206ns | 4301ns | 4430ns | base |
| carrier_pre_scatter_threaded | 3985ns | 4008ns | 3868ns | 3978ns | 4052ns | -7.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 1473ns | 1412ns | 1525ns | -27.17% | 0.043 |
| carrier_pre_scatter_fntable | 2942ns | 2843ns | 3036ns | +45.45% | 0.022 |
| carrier_pre_scatter_null | 1333ns | 1279ns | 1375ns | -34.09% | 0.048 |
| carrier_pre_scatter_regcache | 2301ns | 2225ns | 2365ns | +13.78% | 0.028 |
| carrier_pre_scatter_switch | 2023ns | 1972ns | 2087ns | base | 0.032 |
| carrier_pre_scatter_threaded | 1711ns | 1660ns | 1747ns | -15.40% | 0.037 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 264744 | 951102 | 0.278 | 0.99× |
| carrier_pre_scatter_fntable | 279363 | 790356 | 0.353 | 1.05× |
| carrier_pre_scatter_null | 267886 | 1337529 | 0.200 | 1.00× |
| carrier_pre_scatter_regcache | 272067 | 1114139 | 0.244 | 1.02× |
| carrier_pre_scatter_switch | 266968 | 894398 | 0.298 | 1.00× |
| carrier_pre_scatter_threaded | 268162 | 1123992 | 0.239 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.050 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.043 | 86.7% |
| carrier_pre_scatter_fntable | 0.022 | 43.6% |
| carrier_pre_scatter_null | 0.048 | 95.7% |
| carrier_pre_scatter_regcache | 0.028 | 55.5% |
| carrier_pre_scatter_switch | 0.032 | 63.9% |
| carrier_pre_scatter_threaded | 0.037 | 74.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 3782ns | 3782ns | -12.62% |
| carrier_pre_scatter_fntable | 5198ns | 5198ns | +20.08% |
| carrier_pre_scatter_null | 3635ns | 3635ns | -16.02% |
| carrier_pre_scatter_regcache | 4576ns | 4576ns | +5.72% |
| carrier_pre_scatter_switch | 4328ns | 4328ns | base |
| carrier_pre_scatter_threaded | 3985ns | 3985ns | -7.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 2001ns | base | --- | [1980, 2087] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 1475ns | -543.7ns (-27.2%) | [-608, -497]ns | [1419, 1525] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 2930ns | +918.1ns (+45.9%) | [+866, +974]ns | [2860, 3036] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_null | 1336ns | -700.8ns (-35.0%) | [-737, -630]ns | [1289, 1375] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 2304ns | +276.2ns (+13.8%) | [+182, +378]ns | [2235, 2365] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_threaded | 1717ns | -277.9ns (-13.9%) | [-418, -239]ns | [1669, 1747] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 2002ns | -25.7% | +45.4% | -36.1% | +16.6% | -13.7% |
| 2 | 2000ns | -24.2% | +43.9% | -34.1% | +12.3% | -12.5% |
| 3 | 2067ns | -30.9% | +47.0% | -34.5% | +7.6% | -19.7% |
| 4 | 1988ns | -29.0% | +43.0% | -34.6% | +15.5% | -14.2% |
| 5 | 2107ns | -27.2% | +44.0% | -35.7% | +9.8% | -20.4% |
| 6 | 1972ns | -25.8% | +49.5% | -29.3% | +21.5% | -11.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.254 | moderate- |
| carrier_pre_scatter_fntable | -0.677 | HIGH- (thermal bounce) |
| carrier_pre_scatter_null | 0.047 | ok |
| carrier_pre_scatter_regcache | 0.203 | moderate+ |
| carrier_pre_scatter_switch | -0.672 | HIGH- (thermal bounce) |
| carrier_pre_scatter_threaded | -0.304 | moderate- |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 0/6, lost 6/6
- **carrier_pre_scatter_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 87111.7ns | 1473.1ns | 5913.6% | HIGH |
| carrier_pre_scatter_fntable | 87669.4ns | 2941.9ns | 2980.0% | HIGH |
| carrier_pre_scatter_null | 86273.7ns | 1333.1ns | 6471.7% | HIGH |
| carrier_pre_scatter_regcache | 87231.7ns | 2301.3ns | 3790.5% | HIGH |
| carrier_pre_scatter_switch | 85772.3ns | 2022.6ns | 4240.7% | HIGH |
| carrier_pre_scatter_threaded | 86105.4ns | 1711.1ns | 5032.2% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 1411.7-1525.2 ns)
   1411.7 |########################################
   1417.4 |
   1423.0 |########################################
   1428.7 |
   1434.4 |
   1440.1 |
   1445.8 |
   1451.4 |
   1457.1 |########################################
   1462.8 |
   1468.5 |
   1474.1 |
   1479.8 |
   1485.5 |########################################
   1491.2 |
   1496.8 |
   1502.5 |
   1508.2 |
   1513.9 |########################################
   1519.5 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 2843.3-3036.1 ns)
   2843.3 |########################################
   2852.9 |
   2862.6 |
   2872.2 |########################################
   2881.9 |
   2891.5 |
   2901.1 |
   2910.8 |########################################
   2920.4 |
   2930.0 |
   2939.7 |########################################
   2949.3 |
   2959.0 |
   2968.6 |
   2978.2 |
   2987.9 |
   2997.5 |
   3007.1 |
   3016.8 |
   3026.4 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 1278.8-1374.6 ns)
   1278.8 |####################
   1283.6 |
   1288.4 |
   1293.2 |
   1298.0 |####################
   1302.8 |
   1307.5 |
   1312.3 |
   1317.1 |####################
   1321.9 |
   1326.7 |
   1331.5 |
   1336.3 |
   1341.1 |
   1345.9 |
   1350.6 |########################################
   1355.4 |
   1360.2 |
   1365.0 |
   1369.8 |
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 2224.6-2365.2 ns)
   2224.6 |########################################
   2231.6 |
   2238.7 |########################################
   2245.7 |
   2252.7 |
   2259.8 |
   2266.8 |
   2273.8 |
   2280.8 |
   2287.9 |
   2294.9 |########################################
   2301.9 |
   2309.0 |########################################
   2316.0 |
   2323.0 |
   2330.0 |########################################
   2337.1 |
   2344.1 |
   2351.1 |
   2358.2 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 1972.1-2086.9 ns)
   1972.1 |########################################
   1977.8 |
   1983.6 |########################################
   1989.3 |
   1995.1 |########################################
   2000.8 |########################################
   2006.5 |
   2012.3 |
   2018.0 |
   2023.8 |
   2029.5 |
   2035.2 |
   2041.0 |
   2046.7 |
   2052.5 |
   2058.2 |
   2063.9 |########################################
   2069.7 |
   2075.4 |
   2081.2 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 1660.0-1747.1 ns)
   1660.0 |########################################
   1664.4 |
   1668.7 |
   1673.1 |
   1677.4 |########################################
   1681.8 |
   1686.1 |
   1690.5 |
   1694.8 |
   1699.2 |
   1703.5 |########################################
   1707.9 |
   1712.3 |
   1716.6 |
   1721.0 |
   1725.3 |########################################
   1729.7 |
   1734.0 |
   1738.4 |
   1742.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=5897.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=2990.7% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=6445.1% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=3785.3% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=4277.7% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=5024.4% of algo (FFI overhead may distort results)
