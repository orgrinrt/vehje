# Strategy showdown (skew2): all strategies x tiers (native/interp) same footing

12 variants, 6 samples per variant.
Baseline: **sd_bintree_int_skew2**

## Key findings

- **Fastest: sd_jumptable_int_skew2** at 386.6 ns median (-84.7% vs baseline)
- 9 variants significantly faster than baseline
- Spread: 6.53x (fastest 386.6 ns, slowest 2524.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 4922ns | 5120ns | 4350ns | 4873ns | 5281ns | base |
| sd_bintree_nat_skew2 | 2856ns | 2931ns | 2514ns | 2794ns | 3121ns | -41.97% |
| sd_chain_int_skew2 | 5037ns | 5059ns | 4360ns | 4894ns | 5590ns | +2.35% |
| sd_chain_nat_skew2 | 2867ns | 3002ns | 2504ns | 2873ns | 3040ns | -41.75% |
| sd_chain_rev_int_skew2 | 4819ns | 4854ns | 4189ns | 4703ns | 5308ns | -2.09% |
| sd_chain_rev_nat_skew2 | 2822ns | 2836ns | 2513ns | 2737ns | 3102ns | -42.67% |
| sd_evalall_int_skew2 | 3908ns | 3965ns | 3335ns | 3840ns | 4298ns | -20.59% |
| sd_evalall_nat_skew2 | 3872ns | 3840ns | 3460ns | 3721ns | 4303ns | -21.33% |
| sd_jumptable_int_skew2 | 2778ns | 2801ns | 2519ns | 2719ns | 2995ns | -43.56% |
| sd_jumptable_nat_skew2 | 2873ns | 2998ns | 2612ns | 2871ns | 3008ns | -41.62% |
| sd_profiled_int_skew2 | 4486ns | 4604ns | 4027ns | 4414ns | 4826ns | -8.85% |
| sd_profiled_nat_skew2 | 2840ns | 2890ns | 2501ns | 2778ns | 3104ns | -42.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 2446ns | 2189ns | 2616ns | base | 0.105 |
| sd_bintree_nat_skew2 | 393ns | 348ns | 428ns | -83.94% | 0.652 |
| sd_chain_int_skew2 | 2564ns | 2194ns | 2898ns | +4.83% | 0.100 |
| sd_chain_nat_skew2 | 403ns | 342ns | 440ns | -83.50% | 0.635 |
| sd_chain_rev_int_skew2 | 2357ns | 2026ns | 2618ns | -3.61% | 0.109 |
| sd_chain_rev_nat_skew2 | 401ns | 352ns | 430ns | -83.60% | 0.638 |
| sd_evalall_int_skew2 | 1421ns | 1172ns | 1603ns | -41.88% | 0.180 |
| sd_evalall_nat_skew2 | 1348ns | 1228ns | 1473ns | -44.89% | 0.190 |
| sd_jumptable_int_skew2 | 385ns | 349ns | 414ns | -84.25% | 0.665 |
| sd_jumptable_nat_skew2 | 398ns | 360ns | 416ns | -83.72% | 0.643 |
| sd_profiled_int_skew2 | 2072ns | 1860ns | 2246ns | -15.29% | 0.124 |
| sd_profiled_nat_skew2 | 383ns | 344ns | 413ns | -84.32% | 0.668 |

## Performance model

- Peak throughput: **0.748 Gops/s** (sd_chain_nat_skew2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sd_bintree_int_skew2 | 0.101 | 13.5% |
| sd_bintree_nat_skew2 | 0.637 | 85.2% |
| sd_chain_int_skew2 | 0.102 | 13.6% |
| sd_chain_nat_skew2 | 0.617 | 82.5% |
| sd_chain_rev_int_skew2 | 0.108 | 14.5% |
| sd_chain_rev_nat_skew2 | 0.623 | 83.2% |
| sd_evalall_int_skew2 | 0.182 | 24.3% |
| sd_evalall_nat_skew2 | 0.191 | 25.6% |
| sd_jumptable_int_skew2 | 0.662 | 88.5% |
| sd_jumptable_nat_skew2 | 0.624 | 83.4% |
| sd_profiled_int_skew2 | 0.122 | 16.3% |
| sd_profiled_nat_skew2 | 0.654 | 87.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sd_bintree_int_skew2 | 4922ns | 4922ns | base |
| sd_bintree_nat_skew2 | 2856ns | 2856ns | -41.97% |
| sd_chain_int_skew2 | 5037ns | 5037ns | +2.35% |
| sd_chain_nat_skew2 | 2867ns | 2867ns | -41.75% |
| sd_chain_rev_int_skew2 | 4819ns | 4819ns | -2.09% |
| sd_chain_rev_nat_skew2 | 2822ns | 2822ns | -42.67% |
| sd_evalall_int_skew2 | 3908ns | 3908ns | -20.59% |
| sd_evalall_nat_skew2 | 3872ns | 3872ns | -21.33% |
| sd_jumptable_int_skew2 | 2778ns | 2778ns | -43.56% |
| sd_jumptable_nat_skew2 | 2873ns | 2873ns | -41.62% |
| sd_profiled_int_skew2 | 4486ns | 4486ns | -8.85% |
| sd_profiled_nat_skew2 | 2840ns | 2840ns | -42.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 2525ns | base | --- | [2196, 2616] | --- | --- | --- | --- |
| sd_bintree_nat_skew2 | 402ns | -2115.8ns (-83.8%) | [-2195, -1847]ns | [349, 428] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_int_skew2 | 2518ns | no significant difference | [-70, +282]ns | [2276, 2898] | no | 0.2188 | 0.2188 | 0 |
| sd_chain_nat_skew2 | 415ns | -2107.1ns (-83.5%) | [-2178, -1841]ns | [355, 440] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_rev_int_skew2 | 2362ns | no significant difference | [-223, +22]ns | [2091, 2618] | no | 0.2188 | 0.2188 | 0 |
| sd_chain_rev_nat_skew2 | 411ns | -2094.6ns (-83.0%) | [-2205, -1835]ns | [362, 430] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_int_skew2 | 1406ns | -1030.4ns (-40.8%) | [-1156, -887]ns | [1255, 1603] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_nat_skew2 | 1338ns | -1077.1ns (-42.7%) | [-1275, -942]ns | [1232, 1473] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_int_skew2 | 387ns | -2138.8ns (-84.7%) | [-2203, -1839]ns | [355, 414] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_nat_skew2 | 410ns | -2111.3ns (-83.6%) | [-2202, -1828]ns | [368, 416] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_int_skew2 | 2104ns | -376.9ns (-14.9%) | [-450, -295]ns | [1866, 2246] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_nat_skew2 | 392ns | -2111.9ns (-83.6%) | [-2224, -1850]ns | [346, 413] | YES | 0.0382 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sd_bintree_int_skew2 | sd_bintree_nat_skew2 | sd_chain_int_skew2 | sd_chain_nat_skew2 | sd_chain_rev_int_skew2 | sd_chain_rev_nat_skew2 | sd_evalall_int_skew2 | sd_evalall_nat_skew2 | sd_jumptable_int_skew2 | sd_jumptable_nat_skew2 | sd_profiled_int_skew2 | sd_profiled_nat_skew2 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2537ns | -83.7% | -5.7% | -83.4% | -11.1% | -83.4% | -44.2% | -51.6% | -85.8% | -83.8% | -10.7% | -83.9% |
| 2 | 2189ns | -84.1% | +0.2% | -84.4% | -7.5% | -83.0% | -46.5% | -43.5% | -84.1% | -82.9% | -14.5% | -84.1% |
| 3 | 2644ns | -83.3% | +15.1% | -84.3% | +3.3% | -84.2% | -34.3% | -46.9% | -84.4% | -84.5% | -15.8% | -84.8% |
| 4 | 2513ns | -83.9% | +5.2% | -83.5% | -1.7% | -82.6% | -41.5% | -42.3% | -83.8% | -83.5% | -19.2% | -83.4% |
| 5 | 2203ns | -84.1% | +7.0% | -83.3% | -2.1% | -84.0% | -39.3% | -42.3% | -83.4% | -83.6% | -15.6% | -84.4% |
| 6 | 2587ns | -84.6% | +6.3% | -82.2% | -3.2% | -84.4% | -46.0% | -42.2% | -84.0% | -83.9% | -15.9% | -85.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sd_bintree_int_skew2 | -0.567 | HIGH- (thermal bounce) |
| sd_bintree_nat_skew2 | -0.477 | moderate- |
| sd_chain_int_skew2 | -0.273 | moderate- |
| sd_chain_nat_skew2 | -0.463 | moderate- |
| sd_chain_rev_int_skew2 | -0.298 | moderate- |
| sd_chain_rev_nat_skew2 | -0.446 | moderate- |
| sd_evalall_int_skew2 | -0.373 | moderate- |
| sd_evalall_nat_skew2 | -0.090 | ok |
| sd_jumptable_int_skew2 | -0.124 | ok |
| sd_jumptable_nat_skew2 | -0.593 | HIGH- (thermal bounce) |
| sd_profiled_int_skew2 | -0.564 | HIGH- (thermal bounce) |
| sd_profiled_nat_skew2 | -0.430 | moderate- |

**Consistency summary:**

- **sd_bintree_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_int_skew2**: won 1/6, lost 5/6
- **sd_chain_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_rev_int_skew2**: won 5/6, lost 1/6
- **sd_chain_rev_nat_skew2**: won 6/6, lost 0/6
- **sd_evalall_int_skew2**: won 6/6, lost 0/6
- **sd_evalall_nat_skew2**: won 6/6, lost 0/6
- **sd_jumptable_int_skew2**: won 6/6, lost 0/6
- **sd_jumptable_nat_skew2**: won 6/6, lost 0/6
- **sd_profiled_int_skew2**: won 6/6, lost 0/6
- **sd_profiled_nat_skew2**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sd_bintree_int_skew2 | 5.0ns | 2445.6ns | 0.2% |  |
| sd_bintree_nat_skew2 | 5.6ns | 392.8ns | 1.4% |  |
| sd_chain_int_skew2 | 5.1ns | 2563.8ns | 0.2% |  |
| sd_chain_nat_skew2 | 4.7ns | 403.5ns | 1.2% |  |
| sd_chain_rev_int_skew2 | 4.7ns | 2357.3ns | 0.2% |  |
| sd_chain_rev_nat_skew2 | 3.8ns | 401.0ns | 0.9% |  |
| sd_evalall_int_skew2 | 6.0ns | 1421.2ns | 0.4% |  |
| sd_evalall_nat_skew2 | 5.0ns | 1347.6ns | 0.4% |  |
| sd_jumptable_int_skew2 | 4.6ns | 385.1ns | 1.2% |  |
| sd_jumptable_nat_skew2 | 4.7ns | 398.2ns | 1.2% |  |
| sd_profiled_int_skew2 | 5.0ns | 2071.6ns | 0.2% |  |
| sd_profiled_nat_skew2 | 4.1ns | 383.5ns | 1.1% |  |

## Distribution (algo ns)

```
sd_bintree_int_skew2 (n=6, range 2189.2-2615.6 ns)
   2189.2 |########################################
   2210.5 |
   2231.8 |
   2253.2 |
   2274.5 |
   2295.8 |
   2317.1 |
   2338.5 |
   2359.8 |
   2381.1 |
   2402.4 |
   2423.7 |
   2445.1 |
   2466.4 |
   2487.7 |
   2509.0 |####################
   2530.4 |####################
   2551.7 |
   2573.0 |####################
   2594.3 |
  (0 below, 1 above range)

sd_bintree_nat_skew2 (n=6, range 348.3-427.5 ns)
    348.3 |########################################
    352.3 |
    356.2 |
    360.2 |
    364.1 |
    368.1 |
    372.1 |
    376.0 |
    380.0 |
    383.9 |
    387.9 |
    391.9 |
    395.8 |####################
    399.8 |
    403.7 |####################
    407.7 |
    411.7 |####################
    415.6 |
    419.6 |
    423.5 |
  (0 below, 1 above range)

sd_chain_int_skew2 (n=6, range 2193.8-2897.5 ns)
   2193.8 |########################################
   2229.0 |
   2264.2 |
   2299.4 |
   2334.5 |########################################
   2369.7 |########################################
   2404.9 |
   2440.1 |
   2475.3 |
   2510.5 |
   2545.7 |
   2580.8 |
   2616.0 |########################################
   2651.2 |
   2686.4 |
   2721.6 |########################################
   2756.8 |
   2791.9 |
   2827.1 |
   2862.3 |
  (0 below, 1 above range)

sd_chain_nat_skew2 (n=6, range 342.1-440.4 ns)
    342.1 |####################
    347.0 |
    351.9 |
    356.8 |
    361.8 |
    366.7 |####################
    371.6 |
    376.5 |
    381.4 |
    386.3 |
    391.2 |
    396.2 |
    401.1 |
    406.0 |
    410.9 |########################################
    415.8 |
    420.7 |####################
    425.7 |
    430.6 |
    435.5 |
  (0 below, 1 above range)

sd_chain_rev_int_skew2 (n=6, range 2025.8-2618.3 ns)
   2025.8 |########################################
   2055.4 |
   2085.1 |
   2114.7 |
   2144.3 |########################################
   2173.9 |
   2203.6 |
   2233.2 |########################################
   2262.8 |
   2292.4 |
   2322.1 |
   2351.7 |
   2381.3 |
   2411.0 |
   2440.6 |########################################
   2470.2 |
   2499.8 |########################################
   2529.5 |
   2559.1 |
   2588.7 |
  (0 below, 1 above range)

sd_chain_rev_nat_skew2 (n=6, range 351.7-430.2 ns)
    351.7 |####################
    355.6 |
    359.6 |
    363.5 |
    367.4 |
    371.3 |####################
    375.2 |
    379.2 |
    383.1 |
    387.0 |
    391.0 |
    394.9 |
    398.8 |
    402.7 |####################
    406.7 |
    410.6 |
    414.5 |
    418.4 |########################################
    422.4 |
    426.3 |
  (0 below, 1 above range)

sd_evalall_int_skew2 (n=6, range 1172.1-1602.7 ns)
   1172.1 |########################################
   1193.6 |
   1215.2 |
   1236.7 |
   1258.2 |
   1279.8 |
   1301.3 |
   1322.8 |########################################
   1344.3 |
   1365.9 |
   1387.4 |########################################
   1408.9 |########################################
   1430.5 |
   1452.0 |########################################
   1473.5 |
   1495.0 |
   1516.6 |
   1538.1 |
   1559.6 |
   1581.2 |
  (0 below, 1 above range)

sd_evalall_nat_skew2 (n=6, range 1227.9-1472.9 ns)
   1227.9 |########################################
   1240.2 |
   1252.4 |
   1264.7 |####################
   1276.9 |
   1289.2 |
   1301.4 |
   1313.7 |
   1325.9 |
   1338.2 |
   1350.4 |
   1362.7 |
   1374.9 |
   1387.2 |
   1399.4 |####################
   1411.7 |
   1423.9 |
   1436.2 |
   1448.4 |####################
   1460.7 |
  (0 below, 1 above range)

sd_jumptable_int_skew2 (n=6, range 348.7-413.8 ns)
    348.7 |########################################
    352.0 |
    355.2 |
    358.5 |########################################
    361.7 |
    365.0 |########################################
    368.2 |
    371.5 |
    374.7 |
    378.0 |
    381.2 |
    384.5 |
    387.7 |
    391.0 |
    394.2 |
    397.5 |
    400.7 |
    404.0 |
    407.2 |########################################
    410.5 |########################################
  (0 below, 1 above range)

sd_jumptable_nat_skew2 (n=6, range 360.4-416.2 ns)
    360.4 |########################################
    363.2 |
    366.0 |
    368.8 |
    371.6 |
    374.4 |########################################
    377.2 |
    379.9 |
    382.7 |
    385.5 |
    388.3 |
    391.1 |
    393.9 |
    396.7 |
    399.5 |
    402.3 |
    405.1 |
    407.9 |########################################
    410.7 |########################################
    413.5 |########################################
  (0 below, 1 above range)

sd_profiled_int_skew2 (n=6, range 1860.0-2245.6 ns)
   1860.0 |########################################
   1879.3 |
   1898.6 |
   1917.8 |
   1937.1 |
   1956.4 |
   1975.7 |
   1995.0 |
   2014.3 |####################
   2033.5 |
   2052.8 |
   2072.1 |
   2091.4 |
   2110.7 |
   2130.0 |
   2149.2 |
   2168.5 |####################
   2187.8 |
   2207.1 |
   2226.4 |####################
  (0 below, 1 above range)

sd_profiled_nat_skew2 (n=6, range 344.2-412.9 ns)
    344.2 |########################################
    347.6 |
    351.1 |
    354.5 |
    357.9 |
    361.4 |
    364.8 |
    368.2 |
    371.7 |
    375.1 |
    378.5 |####################
    382.0 |
    385.4 |
    388.9 |
    392.3 |
    395.7 |
    399.2 |####################
    402.6 |
    406.0 |####################
    409.5 |
  (0 below, 1 above range)

```
