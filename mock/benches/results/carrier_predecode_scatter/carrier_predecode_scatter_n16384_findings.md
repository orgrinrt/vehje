# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 304% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (422.48 us) leads carrier_pre_scatter_direct (1.70 ms) by 304%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 80% (significant)

carrier_pre_scatter_null is -1.68 ms (80%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 5.6x slower than the field

carrier_pre_scatter_fntable (2.39 ms) is 5.6x the fastest (422.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_scatter_null shows alternating (throttle bounce) (autocorr -0.64)

carrier_pre_scatter_null's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_scatter_null} vs {carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_threaded, carrier_pre_scatter_regcache, carrier_pre_scatter_fntable} (304% apart)

The field splits into a fast tier {carrier_pre_scatter_null} and a slow tier {carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_threaded, carrier_pre_scatter_regcache, carrier_pre_scatter_fntable} with a 304% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest carrier_pre_scatter_null (422.48 us) to slowest carrier_pre_scatter_fntable (2.39 ms): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 422482.5 ns median (-79.9% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.65x (fastest 422482.5 ns, slowest 2386872.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 1702022ns | 1707290ns | 1662483ns | 1704251ns | 1718448ns | -18.84% |
| carrier_pre_scatter_fntable | 2387920ns | 2389177ns | 2378250ns | 2388612ns | 2391716ns | +13.87% |
| carrier_pre_scatter_null | 424701ns | 424767ns | 420210ns | 424379ns | 427430ns | -79.75% |
| carrier_pre_scatter_regcache | 2206898ns | 2209147ns | 2193841ns | 2204916ns | 2216399ns | +5.24% |
| carrier_pre_scatter_switch | 2097066ns | 2100133ns | 2073622ns | 2097253ns | 2108508ns | base |
| carrier_pre_scatter_threaded | 2188147ns | 2193976ns | 2123480ns | 2186783ns | 2222526ns | +4.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 1699680ns | 1660177ns | 1716098ns | -18.86% | 0.010 |
| carrier_pre_scatter_fntable | 2385598ns | 2375832ns | 2389408ns | +13.89% | 0.007 |
| carrier_pre_scatter_null | 422418ns | 418032ns | 425082ns | -79.83% | 0.039 |
| carrier_pre_scatter_regcache | 2204547ns | 2191474ns | 2214047ns | +5.25% | 0.007 |
| carrier_pre_scatter_switch | 2094648ns | 2071345ns | 2105998ns | base | 0.008 |
| carrier_pre_scatter_threaded | 2185820ns | 2121187ns | 2220216ns | +4.35% | 0.007 |

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.010 | 24.5% |
| carrier_pre_scatter_fntable | 0.007 | 17.5% |
| carrier_pre_scatter_null | 0.039 | 98.9% |
| carrier_pre_scatter_regcache | 0.007 | 18.9% |
| carrier_pre_scatter_switch | 0.008 | 19.9% |
| carrier_pre_scatter_threaded | 0.007 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 1702022ns | 1702022ns | -18.84% |
| carrier_pre_scatter_fntable | 2387920ns | 2387920ns | +13.87% |
| carrier_pre_scatter_null | 424701ns | 424701ns | -79.75% |
| carrier_pre_scatter_regcache | 2206898ns | 2206898ns | +5.24% |
| carrier_pre_scatter_switch | 2097066ns | 2097066ns | base |
| carrier_pre_scatter_threaded | 2188147ns | 2188147ns | +4.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 2097724ns | base | --- | [2080223, 2105998] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 1704913ns | -396852.9ns (-18.9%) | [-422699, -365352]ns | [1678030, 1716098] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 2386872ns | +289154.0ns (+13.8%) | [+280653, +303041]ns | [2380513, 2389408] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_null | 422482ns | -1675595.8ns (-79.9%) | [-1680916, -1660179]ns | [419690, 425082] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 2206804ns | +107179.6ns (+5.1%) | [+90729, +131788]ns | [2192791, 2214047] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_threaded | 2191619ns | +99283.8ns (+4.7%) | [+43563, +130667]ns | [2145624, 2220216] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 2098656ns | -18.9% | +13.9% | -79.8% | +5.4% | +6.3% |
| 2 | 2107331ns | -18.9% | +13.3% | -79.9% | +4.0% | +3.0% |
| 3 | 2089102ns | -17.5% | +13.7% | -80.0% | +5.7% | +5.7% |
| 4 | 2104665ns | -19.4% | +13.3% | -79.8% | +4.8% | +3.8% |
| 5 | 2071345ns | -17.6% | +15.2% | -79.6% | +7.0% | +6.2% |
| 6 | 2096792ns | -20.8% | +13.9% | -79.9% | +4.6% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.067 | ok |
| carrier_pre_scatter_fntable | -0.067 | ok |
| carrier_pre_scatter_null | -0.643 | HIGH- (thermal bounce) |
| carrier_pre_scatter_regcache | -0.548 | HIGH- (thermal bounce) |
| carrier_pre_scatter_switch | -0.419 | moderate- |
| carrier_pre_scatter_threaded | -0.283 | moderate- |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 0/6, lost 6/6
- **carrier_pre_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 1762235.4ns | 1699680.4ns | 103.7% | HIGH |
| carrier_pre_scatter_fntable | 2438141.9ns | 2385597.5ns | 102.2% | HIGH |
| carrier_pre_scatter_null | 539701.8ns | 422418.2ns | 127.8% | HIGH |
| carrier_pre_scatter_regcache | 2255542.4ns | 2204547.4ns | 102.3% | HIGH |
| carrier_pre_scatter_switch | 2225537.0ns | 2094648.3ns | 106.2% | HIGH |
| carrier_pre_scatter_threaded | 2234838.3ns | 2185819.5ns | 102.2% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 1660177.1-1716097.9 ns)
  1660177.1 |########################################
  1662973.1 |
  1665769.2 |
  1668565.2 |
  1671361.3 |
  1674157.3 |
  1676953.3 |
  1679749.4 |
  1682545.4 |
  1685341.5 |
  1688137.5 |
  1690933.5 |
  1693729.6 |########################################
  1696525.6 |
  1699321.7 |
  1702117.7 |########################################
  1704913.7 |########################################
  1707709.8 |########################################
  1710505.8 |
  1713301.9 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 2375832.5-2389407.7 ns)
  2375832.5 |####################
  2376511.3 |
  2377190.0 |
  2377868.8 |
  2378547.5 |
  2379226.3 |
  2379905.1 |
  2380583.8 |
  2381262.6 |
  2381941.3 |
  2382620.1 |
  2383298.9 |
  2383977.6 |
  2384656.4 |####################
  2385335.1 |####################
  2386013.9 |
  2386692.7 |
  2387371.4 |
  2388050.2 |########################################
  2388728.9 |
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 418032.5-425082.3 ns)
  418032.5 |########################################
  418385.0 |
  418737.5 |
  419090.0 |
  419442.5 |
  419795.0 |
  420147.4 |
  420499.9 |
  420852.4 |
  421204.9 |########################################
  421557.4 |
  421909.9 |########################################
  422262.4 |
  422614.9 |########################################
  422967.4 |
  423319.8 |
  423672.3 |
  424024.8 |########################################
  424377.3 |
  424729.8 |
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 2191474.2-2214047.3 ns)
  2191474.2 |########################################
  2192602.9 |
  2193731.5 |########################################
  2194860.2 |
  2195988.8 |
  2197117.5 |
  2198246.1 |
  2199374.8 |
  2200503.4 |
  2201632.1 |
  2202760.8 |
  2203889.4 |
  2205018.1 |########################################
  2206146.7 |
  2207275.4 |
  2208404.0 |########################################
  2209532.7 |
  2210661.3 |
  2211790.0 |########################################
  2212918.6 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 2071344.6-2105997.9 ns)
  2071344.6 |########################################
  2073077.3 |
  2074809.9 |
  2076542.6 |
  2078275.3 |
  2080007.9 |
  2081740.6 |
  2083473.3 |
  2085205.9 |
  2086938.6 |
  2088671.2 |########################################
  2090403.9 |
  2092136.6 |
  2093869.2 |
  2095601.9 |########################################
  2097334.6 |########################################
  2099067.2 |
  2100799.9 |
  2102532.6 |
  2104265.2 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 2121187.1-2220215.9 ns)
  2121187.1 |########################################
  2126138.5 |
  2131090.0 |
  2136041.4 |
  2140992.9 |
  2145944.3 |
  2150895.7 |
  2155847.2 |
  2160798.6 |
  2165750.0 |########################################
  2170701.5 |
  2175652.9 |
  2180604.4 |########################################
  2185555.8 |
  2190507.2 |
  2195458.7 |########################################
  2200410.1 |
  2205361.5 |########################################
  2210313.0 |
  2215264.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=103.7% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=127.7% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=102.3% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=106.4% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=102.4% of algo (FFI overhead may distort results)
