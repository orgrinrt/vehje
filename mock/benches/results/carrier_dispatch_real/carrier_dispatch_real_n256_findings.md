# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_real**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_real**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_nullfloor_real dominates: 31% faster than the next best (carrier_disp_ifchainasc_real)

carrier_disp_nullfloor_real (8.25 us) leads carrier_disp_ifchainasc_real (10.78 us) by 31%, a clear separation rather than a photo finish. CV 8.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_nullfloor_real beats baseline by 28% (significant)

carrier_disp_nullfloor_real is -3.22 us (28%) faster than baseline carrier_disp_switch_real, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_ifchainlin_real is an outlier: 2.8x slower than the field

carrier_disp_ifchainlin_real (23.29 us) is 2.8x the fastest (8.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_nullfloor_real, carrier_disp_ifchainasc_real, carrier_disp_switch_real, carrier_disp_ifchain_real, carrier_disp_bittree_real, carrier_disp_threaded_real, carrier_disp_fntable_real} vs {carrier_disp_ifchainlin_real} (62% apart)

The field splits into a fast tier {carrier_disp_nullfloor_real, carrier_disp_ifchainasc_real, carrier_disp_switch_real, carrier_disp_ifchain_real, carrier_disp_bittree_real, carrier_disp_threaded_real, carrier_disp_fntable_real} and a slow tier {carrier_disp_ifchainlin_real} with a 62% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_ifchain_real's edge over baseline is significant but tiny (-23 ns, 0.20%)

carrier_disp_ifchain_real differs from baseline carrier_disp_switch_real by -23 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_nullfloor_real** at 8249.8 ns median (-28.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.82x (fastest 8249.8 ns, slowest 23287.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_bittree_real | 15549ns | 15574ns | 14035ns | 15197ns | 16835ns | +15.16% |
| carrier_disp_fntable_real | 16357ns | 16891ns | 14685ns | 16156ns | 17494ns | +21.14% |
| carrier_disp_ifchain_real | 13610ns | 14320ns | 12014ns | 13559ns | 14485ns | +0.80% |
| carrier_disp_ifchainasc_real | 13173ns | 13190ns | 11865ns | 12782ns | 14412ns | -2.44% |
| carrier_disp_ifchainlin_real | 25107ns | 25809ns | 21745ns | 24881ns | 27126ns | +85.94% |
| carrier_disp_nullfloor_real | 10237ns | 10859ns | 8782ns | 10278ns | 10904ns | -24.18% |
| carrier_disp_switch_real | 13502ns | 13998ns | 12056ns | 13353ns | 14450ns | base |
| carrier_disp_threaded_real | 15337ns | 15855ns | 13323ns | 15529ns | 16056ns | +13.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_bittree_real | 13114ns | 11841ns | 14184ns | +18.59% | 0.020 |
| carrier_disp_fntable_real | 13924ns | 12502ns | 14894ns | +25.92% | 0.018 |
| carrier_disp_ifchain_real | 11141ns | 9834ns | 11872ns | +0.74% | 0.023 |
| carrier_disp_ifchainasc_real | 10786ns | 9728ns | 11803ns | -2.46% | 0.024 |
| carrier_disp_ifchainlin_real | 22617ns | 19618ns | 24375ns | +104.53% | 0.011 |
| carrier_disp_nullfloor_real | 7782ns | 6675ns | 8293ns | -29.63% | 0.033 |
| carrier_disp_switch_real | 11058ns | 9854ns | 11850ns | base | 0.023 |
| carrier_disp_threaded_real | 12815ns | 11136ns | 13389ns | +15.89% | 0.020 |

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_disp_nullfloor_real; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_bittree_real | 0.019 | 50.8% |
| carrier_disp_fntable_real | 0.018 | 46.4% |
| carrier_disp_ifchain_real | 0.022 | 57.0% |
| carrier_disp_ifchainasc_real | 0.024 | 61.9% |
| carrier_disp_ifchainlin_real | 0.011 | 28.7% |
| carrier_disp_nullfloor_real | 0.031 | 80.9% |
| carrier_disp_switch_real | 0.022 | 58.2% |
| carrier_disp_threaded_real | 0.019 | 50.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_bittree_real | 15549ns | 15549ns | +15.16% |
| carrier_disp_fntable_real | 16357ns | 16357ns | +21.14% |
| carrier_disp_ifchain_real | 13610ns | 13610ns | +0.80% |
| carrier_disp_ifchainasc_real | 13173ns | 13173ns | -2.44% |
| carrier_disp_ifchainlin_real | 25107ns | 25107ns | +85.94% |
| carrier_disp_nullfloor_real | 10237ns | 10237ns | -24.18% |
| carrier_disp_switch_real | 13502ns | 13502ns | base |
| carrier_disp_threaded_real | 15337ns | 15337ns | +13.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_real | 11464ns | base | --- | [9861, 11850] | --- | --- | --- | --- |
| carrier_disp_bittree_real | 13145ns | +2302.9ns (+20.1%) | [+1145, +2720]ns | [12014, 14184] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_fntable_real | 14375ns | +2825.6ns (+24.6%) | [+2402, +3371]ns | [12504, 14894] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_ifchain_real | 11712ns | no significant difference | [-98, +368]ns | [9838, 11872] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_ifchainasc_real | 10780ns | no significant difference | [-1093, +335]ns | [9776, 11803] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_ifchainlin_real | 23288ns | +11826.2ns (+103.2%) | [+9320, +13530]ns | [20188, 24375] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_nullfloor_real | 8250ns | -3220.1ns (-28.1%) | [-4353, -2256]ns | [6802, 8293] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_threaded_real | 13262ns | +1598.8ns (+13.9%) | [+1336, +2336]ns | [11794, 13389] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_real | carrier_disp_bittree_real | carrier_disp_fntable_real | carrier_disp_ifchain_real | carrier_disp_ifchainasc_real | carrier_disp_ifchainlin_real | carrier_disp_nullfloor_real | carrier_disp_threaded_real |
|---|---|---|---|---|---|---|---|---|
| 1 | 11870ns | +2.7% | +25.4% | -1.3% | -17.2% | +65.3% | -43.8% | +11.8% |
| 2 | 11751ns | +21.4% | +18.5% | -0.3% | -0.7% | +104.9% | -29.8% | +14.9% |
| 3 | 9854ns | +23.8% | +26.9% | -0.1% | +0.4% | +144.3% | -16.1% | +26.4% |
| 4 | 9868ns | +20.0% | +26.7% | -0.3% | -1.4% | +110.4% | -29.8% | +12.9% |
| 5 | 11830ns | +19.1% | +25.3% | +1.6% | -0.2% | +108.6% | -29.7% | +12.2% |
| 6 | 11178ns | +26.1% | +33.3% | +4.9% | +5.6% | +101.3% | -26.2% | +18.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_bittree_real | -0.189 | ok |
| carrier_disp_fntable_real | 0.240 | moderate+ |
| carrier_disp_ifchain_real | 0.126 | ok |
| carrier_disp_ifchainasc_real | -0.128 | ok |
| carrier_disp_ifchainlin_real | -0.430 | moderate- |
| carrier_disp_nullfloor_real | -0.312 | moderate- |
| carrier_disp_switch_real | 0.073 | ok |
| carrier_disp_threaded_real | 0.026 | ok |

**Consistency summary:**

- **carrier_disp_bittree_real**: won 0/6, lost 6/6
- **carrier_disp_fntable_real**: won 0/6, lost 6/6
- **carrier_disp_ifchain_real**: won 4/6, lost 2/6
- **carrier_disp_ifchainasc_real**: won 4/6, lost 2/6
- **carrier_disp_ifchainlin_real**: won 0/6, lost 6/6
- **carrier_disp_nullfloor_real**: won 6/6, lost 0/6
- **carrier_disp_threaded_real**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_bittree_real | 89.4ns | 13114.3ns | 0.7% |  |
| carrier_disp_fntable_real | 88.7ns | 13924.4ns | 0.6% |  |
| carrier_disp_ifchain_real | 89.0ns | 11140.6ns | 0.8% |  |
| carrier_disp_ifchainasc_real | 84.9ns | 10786.4ns | 0.8% |  |
| carrier_disp_ifchainlin_real | 87.9ns | 22617.0ns | 0.4% |  |
| carrier_disp_nullfloor_real | 86.6ns | 7781.8ns | 1.1% |  |
| carrier_disp_switch_real | 86.7ns | 11058.3ns | 0.8% |  |
| carrier_disp_threaded_real | 90.6ns | 12815.2ns | 0.7% |  |

## Distribution (algo ns)

```
carrier_disp_bittree_real (n=6, range 11841.2-14184.2 ns)
  11841.2 |####################
  11958.4 |
  12075.5 |####################
  12192.7 |####################
  12309.8 |
  12427.0 |
  12544.1 |
  12661.2 |
  12778.4 |
  12895.6 |
  13012.7 |
  13129.9 |
  13247.0 |
  13364.2 |
  13481.3 |
  13598.5 |
  13715.6 |
  13832.8 |
  13949.9 |
  14067.1 |########################################
  (0 below, 1 above range)

carrier_disp_fntable_real (n=6, range 12501.7-14894.2 ns)
  12501.7 |########################################
  12621.3 |
  12740.9 |
  12860.6 |
  12980.2 |
  13099.8 |
  13219.4 |
  13339.1 |
  13458.7 |
  13578.3 |
  13697.9 |
  13817.5 |####################
  13937.2 |
  14056.8 |
  14176.4 |
  14296.0 |
  14415.7 |
  14535.3 |
  14654.9 |
  14774.5 |########################################
  (0 below, 1 above range)

carrier_disp_ifchain_real (n=6, range 9833.7-11871.9 ns)
   9833.7 |##########################
   9935.6 |
  10037.5 |
  10139.4 |
  10241.3 |
  10343.2 |
  10445.2 |
  10547.1 |
  10649.0 |
  10750.9 |
  10852.8 |
  10954.7 |
  11056.6 |
  11158.5 |
  11260.4 |
  11362.4 |
  11464.3 |
  11566.2 |
  11668.1 |########################################
  11770.0 |
  (0 below, 1 above range)

carrier_disp_ifchainasc_real (n=6, range 9727.5-11803.1 ns)
   9727.5 |########################################
   9831.3 |####################
   9935.1 |
  10038.8 |
  10142.6 |
  10246.4 |
  10350.2 |
  10454.0 |
  10557.7 |
  10661.5 |
  10765.3 |
  10869.1 |
  10972.9 |
  11076.6 |
  11180.4 |
  11284.2 |
  11388.0 |
  11491.8 |
  11595.5 |####################
  11699.3 |####################
  (0 below, 1 above range)

carrier_disp_ifchainlin_real (n=6, range 19618.3-24374.6 ns)
  19618.3 |####################
  19856.1 |
  20093.9 |
  20331.7 |
  20569.6 |####################
  20807.4 |
  21045.2 |
  21283.0 |
  21520.8 |
  21758.6 |
  21996.4 |
  22234.3 |
  22472.1 |####################
  22709.9 |
  22947.7 |
  23185.5 |
  23423.3 |
  23661.2 |
  23899.0 |########################################
  24136.8 |
  (0 below, 1 above range)

carrier_disp_nullfloor_real (n=6, range 6675.0-8293.3 ns)
   6675.0 |#############
   6755.9 |
   6836.8 |
   6917.7 |#############
   6998.7 |
   7079.6 |
   7160.5 |
   7241.4 |
   7322.3 |
   7403.2 |
   7484.1 |
   7565.1 |
   7646.0 |
   7726.9 |
   7807.8 |
   7888.7 |
   7969.6 |
   8050.6 |
   8131.5 |
   8212.4 |########################################
  (0 below, 1 above range)

carrier_disp_switch_real (n=6, range 9853.8-11849.8 ns)
   9853.8 |########################################
   9953.6 |
  10053.4 |
  10153.2 |
  10253.0 |
  10352.8 |
  10452.6 |
  10552.4 |
  10652.2 |
  10752.0 |
  10851.8 |
  10951.6 |
  11051.4 |
  11151.2 |####################
  11251.0 |
  11350.8 |
  11450.6 |
  11550.4 |
  11650.2 |
  11750.0 |########################################
  (0 below, 1 above range)

carrier_disp_threaded_real (n=6, range 11136.2-13389.1 ns)
  11136.2 |#############
  11248.8 |
  11361.5 |
  11474.1 |
  11586.8 |
  11699.4 |
  11812.1 |
  11924.7 |
  12037.4 |
  12150.0 |
  12262.7 |
  12375.3 |#############
  12488.0 |
  12600.6 |
  12713.3 |
  12825.9 |
  12938.6 |
  13051.2 |
  13163.9 |########################################
  13276.5 |
  (0 below, 1 above range)

```
