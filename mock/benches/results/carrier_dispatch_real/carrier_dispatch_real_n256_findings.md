# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 44% faster than the next best (carrier_disp_real_ifchainasc)

carrier_disp_real_nullfloor (7.11 us) leads carrier_disp_real_ifchainasc (10.21 us) by 44%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 29% (significant)

carrier_disp_real_nullfloor is -2.97 us (29%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 2.9x slower than the field

carrier_disp_real_ifchainlin (20.74 us) is 2.9x the fastest (7.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_switch shows alternating (throttle bounce) (autocorr -0.85)

carrier_disp_real_switch's per-pass series has lag-1 autocorrelation -0.85, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_ifchain, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} vs {carrier_disp_real_ifchainlin} (56% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_ifchain, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} and a slow tier {carrier_disp_real_ifchainlin} with a 56% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 7106.0 ns median (-31.4% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 2.92x (fastest 7106.0 ns, slowest 20739.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 14573ns | 14158ns | 13992ns | 14118ns | 15546ns | +14.47% |
| carrier_disp_real_fntable | 15773ns | 15900ns | 14731ns | 15529ns | 16661ns | +23.90% |
| carrier_disp_real_ifchain | 13127ns | 13373ns | 12231ns | 13030ns | 13721ns | +3.11% |
| carrier_disp_real_ifchainasc | 12631ns | 12603ns | 12160ns | 12477ns | 13099ns | -0.78% |
| carrier_disp_real_ifchainlin | 23407ns | 23115ns | 22568ns | 23023ns | 24402ns | +83.86% |
| carrier_disp_real_nullfloor | 9712ns | 9463ns | 9276ns | 9417ns | 10372ns | -23.72% |
| carrier_disp_real_switch | 12731ns | 12746ns | 12190ns | 12597ns | 13203ns | base |
| carrier_disp_real_threaded | 13968ns | 13845ns | 13307ns | 13699ns | 14701ns | +9.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 12232ns | 11792ns | 12992ns | +18.34% | 0.021 |
| carrier_disp_real_fntable | 13305ns | 12540ns | 14105ns | +28.72% | 0.019 |
| carrier_disp_real_ifchain | 10625ns | 10011ns | 11106ns | +2.79% | 0.024 |
| carrier_disp_real_ifchainasc | 10257ns | 9971ns | 10573ns | -0.76% | 0.025 |
| carrier_disp_real_ifchainlin | 21013ns | 20338ns | 21916ns | +103.29% | 0.012 |
| carrier_disp_real_nullfloor | 7298ns | 6956ns | 7794ns | -29.39% | 0.035 |
| carrier_disp_real_switch | 10336ns | 9890ns | 10709ns | base | 0.025 |
| carrier_disp_real_threaded | 11567ns | 10996ns | 12187ns | +11.90% | 0.022 |

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.022 | 58.5% |
| carrier_disp_real_fntable | 0.019 | 52.5% |
| carrier_disp_real_ifchain | 0.024 | 64.7% |
| carrier_disp_real_ifchainasc | 0.025 | 68.1% |
| carrier_disp_real_ifchainlin | 0.012 | 33.5% |
| carrier_disp_real_nullfloor | 0.036 | 97.9% |
| carrier_disp_real_switch | 0.025 | 67.2% |
| carrier_disp_real_threaded | 0.022 | 60.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 14573ns | 14573ns | +14.47% |
| carrier_disp_real_fntable | 15773ns | 15773ns | +23.90% |
| carrier_disp_real_ifchain | 13127ns | 13127ns | +3.11% |
| carrier_disp_real_ifchainasc | 12631ns | 12631ns | -0.78% |
| carrier_disp_real_ifchainlin | 23407ns | 23407ns | +83.86% |
| carrier_disp_real_nullfloor | 9712ns | 9712ns | -23.72% |
| carrier_disp_real_switch | 12731ns | 12731ns | base |
| carrier_disp_real_threaded | 13968ns | 13968ns | +9.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 10354ns | base | --- | [9947, 10709] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 11897ns | +1914.1ns (+18.5%) | [+1431, +2342]ns | [11807, 12992] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_fntable | 13259ns | +2937.5ns (+28.4%) | [+2513, +3455]ns | [12551, 14105] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 10748ns | +349.3ns (+3.4%) | [+39, +477]ns | [10020, 11106] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_real_ifchainasc | 10209ns | no significant difference | [-586, +534]ns | [9989, 10573] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_real_ifchainlin | 20740ns | +10701.7ns (+103.4%) | [+10121, +11208]ns | [20384, 21916] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 7106ns | -2969.2ns (-28.7%) | [-3269, -2876]ns | [6995, 7794] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_threaded | 11487ns | +1233.4ns (+11.9%) | [+964, +1494]ns | [11026, 12187] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 10635ns | +24.6% | +34.6% | +2.9% | -6.2% | +92.1% | -27.3% | +14.6% |
| 2 | 10073ns | +18.8% | +24.7% | -0.6% | -0.6% | +105.3% | -30.1% | +9.2% |
| 3 | 10665ns | +19.4% | +30.2% | +3.8% | -2.9% | +104.8% | -32.7% | +9.4% |
| 4 | 9890ns | +19.5% | +30.7% | +1.4% | +9.1% | +105.6% | -28.9% | +11.8% |
| 5 | 10752ns | +9.7% | +26.4% | +3.7% | -4.7% | +104.5% | -26.9% | +13.4% |
| 6 | 10003ns | +18.2% | +25.4% | +5.5% | +1.7% | +108.0% | -30.5% | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.130 | ok |
| carrier_disp_real_fntable | -0.638 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchain | -0.826 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchainasc | 0.213 | moderate+ |
| carrier_disp_real_ifchainlin | -0.580 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | -0.503 | HIGH- (thermal bounce) |
| carrier_disp_real_switch | -0.848 | HIGH- (thermal bounce) |
| carrier_disp_real_threaded | -0.659 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 0/6, lost 6/6
- **carrier_disp_real_fntable**: won 0/6, lost 6/6
- **carrier_disp_real_ifchain**: won 1/6, lost 5/6
- **carrier_disp_real_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 93279.4ns | 12232.2ns | 762.6% | HIGH |
| carrier_disp_real_fntable | 92131.7ns | 13304.7ns | 692.5% | HIGH |
| carrier_disp_real_ifchain | 89550.3ns | 10624.8ns | 842.8% | HIGH |
| carrier_disp_real_ifchainasc | 90970.7ns | 10257.4ns | 886.9% | HIGH |
| carrier_disp_real_ifchainlin | 100557.1ns | 21013.2ns | 478.5% | HIGH |
| carrier_disp_real_nullfloor | 89800.0ns | 7298.3ns | 1230.4% | HIGH |
| carrier_disp_real_switch | 90097.7ns | 10336.4ns | 871.7% | HIGH |
| carrier_disp_real_threaded | 89139.1ns | 11566.8ns | 770.6% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 11791.7-12992.3 ns)
  11791.7 |########################################
  11851.7 |
  11911.8 |#############
  11971.8 |
  12031.8 |
  12091.9 |
  12151.9 |
  12211.9 |
  12271.9 |
  12332.0 |
  12392.0 |
  12452.0 |
  12512.1 |
  12572.1 |
  12632.1 |
  12692.1 |#############
  12752.2 |
  12812.2 |
  12872.2 |
  12932.3 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 12540.0-14104.5 ns)
  12540.0 |########################################
  12618.2 |
  12696.5 |
  12774.7 |
  12852.9 |####################
  12931.1 |
  13009.4 |
  13087.6 |
  13165.8 |
  13244.0 |
  13322.3 |
  13400.5 |
  13478.7 |
  13557.0 |####################
  13635.2 |
  13713.4 |
  13791.6 |
  13869.9 |####################
  13948.1 |
  14026.3 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 10011.2-11106.2 ns)
  10011.2 |########################################
  10066.0 |
  10120.7 |
  10175.5 |
  10230.2 |
  10285.0 |
  10339.7 |
  10394.5 |
  10449.2 |
  10504.0 |####################
  10558.7 |
  10613.5 |
  10668.2 |
  10723.0 |
  10777.7 |
  10832.5 |
  10887.2 |####################
  10942.0 |
  10996.7 |
  11051.5 |####################
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 9971.2-10573.3 ns)
   9971.2 |########################################
  10001.3 |########################################
  10031.4 |
  10061.5 |
  10091.6 |
  10121.7 |
  10151.8 |########################################
  10182.0 |
  10212.1 |
  10242.2 |########################################
  10272.3 |
  10302.4 |
  10332.5 |########################################
  10362.6 |
  10392.7 |
  10422.8 |
  10452.9 |
  10483.0 |
  10513.1 |
  10543.2 |
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 20338.3-21916.1 ns)
  20338.3 |########################################
  20417.2 |########################################
  20496.1 |
  20575.0 |
  20653.8 |########################################
  20732.7 |########################################
  20811.6 |
  20890.5 |
  20969.4 |
  21048.3 |
  21127.2 |
  21206.1 |
  21285.0 |
  21363.8 |
  21442.7 |
  21521.6 |
  21600.5 |
  21679.4 |
  21758.3 |
  21837.2 |########################################
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 6955.8-7793.8 ns)
   6955.8 |####################
   6997.7 |########################################
   7039.6 |
   7081.5 |
   7123.4 |
   7165.3 |####################
   7207.2 |
   7249.1 |
   7291.0 |
   7332.9 |
   7374.8 |
   7416.7 |
   7458.6 |
   7500.5 |
   7542.4 |
   7584.3 |
   7626.2 |
   7668.1 |
   7710.0 |####################
   7751.9 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 9890.0-10708.5 ns)
   9890.0 |####################
   9930.9 |
   9971.9 |####################
  10012.8 |
  10053.7 |####################
  10094.6 |
  10135.6 |
  10176.5 |
  10217.4 |
  10258.3 |
  10299.3 |
  10340.2 |
  10381.1 |
  10422.1 |
  10463.0 |
  10503.9 |
  10544.8 |
  10585.8 |
  10626.7 |########################################
  10667.6 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 10996.2-12187.3 ns)
  10996.2 |########################################
  11055.8 |########################################
  11115.3 |
  11174.9 |
  11234.4 |
  11294.0 |########################################
  11353.5 |
  11413.1 |
  11472.6 |
  11532.2 |
  11591.8 |
  11651.3 |########################################
  11710.9 |
  11770.4 |
  11830.0 |
  11889.5 |
  11949.1 |
  12008.6 |
  12068.2 |
  12127.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=788.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=697.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=834.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=890.7% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=491.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=1267.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=870.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=770.5% of algo (FFI overhead may distort results)
