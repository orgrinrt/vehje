# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 275% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (455.55 us) leads carrier_pre_real_direct (1.71 ms) by 275%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 78% (significant)

carrier_pre_real_null is -1.64 ms (78%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 5.2x slower than the field

carrier_pre_real_fntable (2.39 ms) is 5.2x the fastest (455.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_real_null} vs {carrier_pre_real_direct, carrier_pre_real_switch, carrier_pre_real_threaded, carrier_pre_real_regcache, carrier_pre_real_fntable} (275% apart)

The field splits into a fast tier {carrier_pre_real_null} and a slow tier {carrier_pre_real_direct, carrier_pre_real_switch, carrier_pre_real_threaded, carrier_pre_real_regcache, carrier_pre_real_fntable} with a 275% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest carrier_pre_real_null (455.55 us) to slowest carrier_pre_real_fntable (2.39 ms): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_real_null** at 455554.8 ns median (-78.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.24x (fastest 455554.8 ns, slowest 2387537.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 1708382ns | 1708683ns | 1693377ns | 1707644ns | 1716993ns | -18.57% |
| carrier_pre_real_fntable | 2381739ns | 2389803ns | 2325379ns | 2385516ns | 2404254ns | +13.53% |
| carrier_pre_real_null | 457997ns | 457806ns | 449409ns | 457216ns | 463462ns | -78.17% |
| carrier_pre_real_regcache | 2210722ns | 2213393ns | 2182218ns | 2210432ns | 2225410ns | +5.38% |
| carrier_pre_real_switch | 2097873ns | 2098698ns | 2088655ns | 2095592ns | 2105905ns | base |
| carrier_pre_real_threaded | 2199551ns | 2202574ns | 2155872ns | 2199133ns | 2222017ns | +4.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 1706030ns | 1690851ns | 1714720ns | -18.58% | 0.010 |
| carrier_pre_real_fntable | 2379214ns | 2322630ns | 2401640ns | +13.54% | 0.007 |
| carrier_pre_real_null | 455712ns | 447155ns | 461123ns | -78.25% | 0.036 |
| carrier_pre_real_regcache | 2208265ns | 2179565ns | 2222961ns | +5.38% | 0.007 |
| carrier_pre_real_switch | 2095440ns | 2085895ns | 2103540ns | base | 0.008 |
| carrier_pre_real_threaded | 2197089ns | 2153622ns | 2219392ns | +4.85% | 0.007 |

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.010 | 26.2% |
| carrier_pre_real_fntable | 0.007 | 18.7% |
| carrier_pre_real_null | 0.036 | 98.2% |
| carrier_pre_real_regcache | 0.007 | 20.2% |
| carrier_pre_real_switch | 0.008 | 21.3% |
| carrier_pre_real_threaded | 0.007 | 20.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 1708382ns | 1708382ns | -18.57% |
| carrier_pre_real_fntable | 2381739ns | 2381739ns | +13.53% |
| carrier_pre_real_null | 457997ns | 457997ns | -78.17% |
| carrier_pre_real_regcache | 2210722ns | 2210722ns | +5.38% |
| carrier_pre_real_switch | 2097873ns | 2097873ns | base |
| carrier_pre_real_threaded | 2199551ns | 2199551ns | +4.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 2096356ns | base | --- | [2086424, 2103540] | --- | --- | --- | --- |
| carrier_pre_real_direct | 1706314ns | -392323.1ns (-18.7%) | [-400212, -375695]ns | [1697056, 1714720] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_fntable | 2387537ns | +287217.9ns (+13.7%) | [+255295, +308808]ns | [2348464, 2401640] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_null | 455555ns | -1641203.1ns (-78.3%) | [-1646897, -1631085]ns | [450457, 461123] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_regcache | 2210950ns | +113921.3ns (+5.4%) | [+101862, +122691]ns | [2190883, 2222961] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_threaded | 2200116ns | +105053.1ns (+5.0%) | [+79675, +120219]ns | [2171760, 2219392] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 2094438ns | -17.9% | +13.4% | -78.3% | +5.3% | +4.7% |
| 2 | 2102268ns | -19.0% | +13.1% | -78.4% | +5.8% | +5.3% |
| 3 | 2104813ns | -19.1% | +14.0% | -78.2% | +5.3% | +5.7% |
| 4 | 2098273ns | -18.5% | +14.6% | -78.2% | +5.9% | +4.4% |
| 5 | 2086954ns | -19.0% | +11.3% | -77.8% | +4.4% | +5.7% |
| 6 | 2085895ns | -18.1% | +15.0% | -78.6% | +5.6% | +3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | -0.315 | moderate- |
| carrier_pre_real_fntable | -0.432 | moderate- |
| carrier_pre_real_null | -0.396 | moderate- |
| carrier_pre_real_regcache | -0.014 | ok |
| carrier_pre_real_switch | 0.459 | moderate+ |
| carrier_pre_real_threaded | -0.090 | ok |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 1777521.2ns | 1706030.1ns | 104.2% | HIGH |
| carrier_pre_real_fntable | 2437110.0ns | 2379213.8ns | 102.4% | HIGH |
| carrier_pre_real_null | 596452.1ns | 455711.6ns | 130.9% | HIGH |
| carrier_pre_real_regcache | 2264592.6ns | 2208264.7ns | 102.6% | HIGH |
| carrier_pre_real_switch | 2232020.8ns | 2095440.1ns | 106.5% | HIGH |
| carrier_pre_real_threaded | 2251676.8ns | 2197089.2ns | 102.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 1690851.2-1714720.0 ns)
  1690851.2 |####################
  1692044.6 |
  1693238.1 |
  1694431.5 |
  1695625.0 |
  1696818.4 |
  1698011.8 |
  1699205.3 |
  1700398.7 |
  1701592.2 |
  1702785.6 |########################################
  1703979.0 |
  1705172.5 |
  1706365.9 |
  1707559.4 |
  1708752.8 |########################################
  1709946.2 |
  1711139.7 |
  1712333.1 |
  1713526.6 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 2322629.6-2401640.0 ns)
  2322629.6 |####################
  2326580.1 |
  2330530.6 |
  2334481.2 |
  2338431.7 |
  2342382.2 |
  2346332.7 |
  2350283.2 |
  2354233.8 |
  2358184.3 |
  2362134.8 |
  2366085.3 |
  2370035.8 |
  2373986.4 |########################################
  2377936.9 |
  2381887.4 |
  2385837.9 |
  2389788.4 |
  2393739.0 |
  2397689.5 |########################################
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 447155.0-461123.3 ns)
  447155.0 |########################################
  447853.4 |
  448551.8 |
  449250.3 |
  449948.7 |
  450647.1 |
  451345.5 |
  452043.9 |
  452742.3 |
  453440.8 |########################################
  454139.2 |########################################
  454837.6 |
  455536.0 |
  456234.4 |########################################
  456932.8 |
  457631.3 |
  458329.7 |########################################
  459028.1 |
  459726.5 |
  460424.9 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 2179565.4-2222961.2 ns)
  2179565.4 |########################################
  2181735.2 |
  2183905.0 |
  2186074.8 |
  2188244.6 |
  2190414.4 |
  2192584.1 |
  2194753.9 |
  2196923.7 |
  2199093.5 |
  2201263.3 |########################################
  2203433.1 |########################################
  2205602.9 |
  2207772.7 |
  2209942.5 |
  2212112.2 |
  2214282.0 |########################################
  2216451.8 |
  2218621.6 |
  2220791.4 |########################################
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 2085895.0-2103540.4 ns)
  2085895.0 |########################################
  2086777.3 |########################################
  2087659.5 |
  2088541.8 |
  2089424.1 |
  2090306.4 |
  2091188.6 |
  2092070.9 |
  2092953.2 |
  2093835.4 |########################################
  2094717.7 |
  2095600.0 |
  2096482.2 |
  2097364.5 |
  2098246.8 |########################################
  2099129.0 |
  2100011.3 |
  2100893.6 |
  2101775.9 |########################################
  2102658.1 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 2153622.5-2219391.6 ns)
  2153622.5 |########################################
  2156911.0 |
  2160199.4 |
  2163487.9 |
  2166776.3 |
  2170064.8 |
  2173353.2 |
  2176641.7 |
  2179930.2 |
  2183218.6 |
  2186507.1 |
  2189795.5 |########################################
  2193084.0 |########################################
  2196372.4 |
  2199660.9 |
  2202949.4 |
  2206237.8 |########################################
  2209526.3 |
  2212814.7 |########################################
  2216103.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=104.2% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=102.4% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=130.8% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=102.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=106.6% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=102.5% of algo (FFI overhead may distort results)
