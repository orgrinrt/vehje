# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Key findings

- **Baseline (br_branch_c_rand50) is the fastest** at 1691.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.48x (fastest 1691.2 ns, slowest 2496.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 4121ns | 4283ns | 3650ns | 4098ns | 4392ns | base |
| br_lut_c_rand50 | 4590ns | 4529ns | 4050ns | 4397ns | 5150ns | +11.37% |
| br_mask_c_rand50 | 4847ns | 4997ns | 4332ns | 4801ns | 5173ns | +17.61% |
| br_predicate_c_rand50 | 4475ns | 4408ns | 4077ns | 4348ns | 4865ns | +8.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 1647ns | 1434ns | 1792ns | base | 0.622 |
| br_lut_c_rand50 | 2148ns | 1882ns | 2440ns | +30.38% | 0.477 |
| br_mask_c_rand50 | 2424ns | 2159ns | 2583ns | +47.17% | 0.422 |
| br_predicate_c_rand50 | 2077ns | 1901ns | 2270ns | +26.05% | 0.493 |

## Performance model

- Peak throughput: **0.714 Gops/s** (br_branch_c_rand50; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.605 | 84.8% |
| br_lut_c_rand50 | 0.487 | 68.2% |
| br_mask_c_rand50 | 0.410 | 57.4% |
| br_predicate_c_rand50 | 0.500 | 70.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 4121ns | 4121ns | base |
| br_lut_c_rand50 | 4590ns | 4590ns | +11.37% |
| br_mask_c_rand50 | 4847ns | 4847ns | +17.61% |
| br_predicate_c_rand50 | 4475ns | 4475ns | +8.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 1691ns | base | --- | [1459, 1792] | --- | --- | --- | --- |
| br_lut_c_rand50 | 2102ns | +513.9ns (+30.4%) | [+285, +703]ns | [1901, 2440] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 2497ns | +786.4ns (+46.5%) | [+653, +891]ns | [2194, 2583] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 2046ns | +476.4ns (+28.2%) | [+233, +578]ns | [1914, 2270] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 1780ns | +9.7% | +35.5% | +8.2% |
| 2 | 1710ns | +31.7% | +51.0% | +32.7% |
| 3 | 1434ns | +33.9% | +55.4% | +37.4% |
| 4 | 1484ns | +26.8% | +45.5% | +28.1% |
| 5 | 1803ns | +34.9% | +43.2% | +17.7% |
| 6 | 1672ns | +46.4% | +54.4% | +35.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | 0.069 | ok |
| br_lut_c_rand50 | 0.077 | ok |
| br_mask_c_rand50 | 0.012 | ok |
| br_predicate_c_rand50 | -0.213 | moderate- |

**Consistency summary:**

- **br_lut_c_rand50**: won 0/6, lost 6/6
- **br_mask_c_rand50**: won 0/6, lost 6/6
- **br_predicate_c_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 4.8ns | 1647.4ns | 0.3% |  |
| br_lut_c_rand50 | 4.2ns | 2147.9ns | 0.2% |  |
| br_mask_c_rand50 | 2.7ns | 2424.4ns | 0.1% |  |
| br_predicate_c_rand50 | 2.2ns | 2076.5ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 1434.2-1791.7 ns)
   1434.2 |########################################
   1452.1 |
   1469.9 |########################################
   1487.8 |
   1505.7 |
   1523.6 |
   1541.4 |
   1559.3 |
   1577.2 |
   1595.1 |
   1612.9 |
   1630.8 |
   1648.7 |
   1666.5 |########################################
   1684.4 |
   1702.3 |########################################
   1720.2 |
   1738.0 |
   1755.9 |
   1773.8 |########################################
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 1882.1-2440.2 ns)
   1882.1 |########################################
   1910.0 |########################################
   1937.9 |########################################
   1965.8 |
   1993.7 |
   2021.6 |
   2049.5 |
   2077.4 |
   2105.3 |
   2133.2 |
   2161.1 |
   2189.1 |
   2217.0 |
   2244.9 |########################################
   2272.8 |
   2300.7 |
   2328.6 |
   2356.5 |
   2384.4 |
   2412.3 |########################################
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 2159.2-2582.7 ns)
   2159.2 |####################
   2180.4 |
   2201.5 |
   2222.7 |####################
   2243.9 |
   2265.1 |
   2286.2 |
   2307.4 |
   2328.6 |
   2349.8 |
   2370.9 |
   2392.1 |####################
   2413.3 |
   2434.5 |
   2455.6 |
   2476.8 |
   2498.0 |
   2519.2 |
   2540.3 |
   2561.5 |########################################
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 1900.8-2269.6 ns)
   1900.8 |########################################
   1919.2 |########################################
   1937.7 |
   1956.1 |########################################
   1974.6 |
   1993.0 |
   2011.4 |
   2029.9 |
   2048.3 |
   2066.8 |
   2085.2 |
   2103.6 |
   2122.1 |########################################
   2140.5 |
   2159.0 |
   2177.4 |
   2195.8 |
   2214.3 |
   2232.7 |
   2251.2 |########################################
  (0 below, 1 above range)

```
