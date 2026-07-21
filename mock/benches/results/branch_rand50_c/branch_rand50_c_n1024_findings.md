# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Highlights

Baseline for all deltas below: **br_branch_c_rand50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### br_branch_c_rand50 dominates: 24% faster than the next best (br_lut_c_rand50)

br_branch_c_rand50 (2.07 us) leads br_lut_c_rand50 (2.56 us) by 24%, a clear separation rather than a photo finish. CV 14.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### br_branch_c_rand50 is fastest but the noisiest (CV 14.6%)

br_branch_c_rand50 wins on median (2.07 us) yet has the highest variance (CV 14.6%), while br_lut_c_rand50 is the steadiest (CV 10.8%, 2.56 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (br_branch_c_rand50)

The baseline br_branch_c_rand50 is the fastest (2.07 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### br_branch_c_rand50 is inconsistent: worst-20% is 1.6x its best-20%

br_branch_c_rand50's best 20% of batches run at 1.45 us but its worst 20% at 2.27 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (br_branch_c_rand50) is the fastest** at 2068.9 ns median
- 3 variants significantly slower than baseline
- Spread: 1.40x (fastest 2068.9 ns, slowest 2901.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 4792ns | 4994ns | 3603ns | 4826ns | 5336ns | base |
| br_lut_c_rand50 | 5283ns | 5479ns | 4043ns | 5398ns | 5730ns | +10.24% |
| br_mask_c_rand50 | 5549ns | 5778ns | 4336ns | 5571ns | 6124ns | +15.80% |
| br_predicate_c_rand50 | 5344ns | 5458ns | 4055ns | 5269ns | 6102ns | +11.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 1994ns | 1454ns | 2275ns | base | 0.514 |
| br_lut_c_rand50 | 2467ns | 1892ns | 2675ns | +23.74% | 0.415 |
| br_mask_c_rand50 | 2782ns | 2165ns | 3072ns | +39.51% | 0.368 |
| br_predicate_c_rand50 | 2513ns | 1905ns | 2860ns | +26.03% | 0.408 |

## Performance model

- Peak throughput: **0.704 Gops/s** (br_branch_c_rand50; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.495 | 70.3% |
| br_lut_c_rand50 | 0.400 | 56.8% |
| br_mask_c_rand50 | 0.353 | 50.1% |
| br_predicate_c_rand50 | 0.399 | 56.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 4792ns | 4792ns | base |
| br_lut_c_rand50 | 5283ns | 5283ns | +10.24% |
| br_mask_c_rand50 | 5549ns | 5549ns | +15.80% |
| br_predicate_c_rand50 | 5344ns | 5344ns | +11.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 2069ns | base | --- | [1638, 2275] | --- | --- | --- | --- |
| br_lut_c_rand50 | 2558ns | +438.0ns (+21.2%) | [+399, +583]ns | [2168, 2675] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 2901ns | +762.7ns (+36.9%) | [+641, +960]ns | [2372, 3072] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 2570ns | +470.8ns (+22.8%) | [+413, +673]ns | [2109, 2860] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 1454ns | +30.1% | +48.9% | +31.0% |
| 2 | 1822ns | +34.2% | +41.5% | +26.9% |
| 3 | 1902ns | +28.5% | +57.5% | +30.0% |
| 4 | 2236ns | +19.6% | +25.5% | +19.3% |
| 5 | 2242ns | +19.4% | +36.8% | +34.6% |
| 6 | 2307ns | +15.8% | +33.3% | +17.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | 0.409 | moderate+ |
| br_lut_c_rand50 | 0.206 | moderate+ |
| br_mask_c_rand50 | 0.281 | moderate+ |
| br_predicate_c_rand50 | 0.409 | moderate+ |

**Consistency summary:**

- **br_lut_c_rand50**: won 0/6, lost 6/6
- **br_mask_c_rand50**: won 0/6, lost 6/6
- **br_predicate_c_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 4.4ns | 1993.9ns | 0.2% |  |
| br_lut_c_rand50 | 3.7ns | 2467.2ns | 0.1% |  |
| br_mask_c_rand50 | 3.6ns | 2781.7ns | 0.1% |  |
| br_predicate_c_rand50 | 3.6ns | 2512.8ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 1453.8-2274.6 ns)
   1453.8 |####################
   1494.8 |
   1535.9 |
   1576.9 |
   1618.0 |
   1659.0 |
   1700.0 |
   1741.1 |
   1782.1 |####################
   1823.2 |
   1864.2 |####################
   1905.2 |
   1946.3 |
   1987.3 |
   2028.4 |
   2069.4 |
   2110.4 |
   2151.5 |
   2192.5 |
   2233.6 |########################################
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 1891.7-2675.4 ns)
   1891.7 |####################
   1930.9 |
   1970.1 |
   2009.3 |
   2048.4 |
   2087.6 |
   2126.8 |
   2166.0 |
   2205.2 |
   2244.4 |
   2283.6 |
   2322.8 |
   2361.9 |
   2401.1 |
   2440.3 |########################################
   2479.5 |
   2518.7 |
   2557.9 |
   2597.1 |
   2636.3 |########################################
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 2165.0-3071.9 ns)
   2165.0 |########################################
   2210.3 |
   2255.7 |
   2301.0 |
   2346.4 |
   2391.7 |
   2437.1 |
   2482.4 |
   2527.7 |
   2573.1 |########################################
   2618.4 |
   2663.8 |
   2709.1 |
   2754.5 |
   2799.8 |########################################
   2845.1 |
   2890.5 |
   2935.8 |
   2981.2 |########################################
   3026.5 |########################################
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 1905.0-2860.0 ns)
   1905.0 |########################################
   1952.8 |
   2000.5 |
   2048.2 |
   2096.0 |
   2143.8 |
   2191.5 |
   2239.2 |
   2287.0 |########################################
   2334.8 |
   2382.5 |
   2430.2 |########################################
   2478.0 |
   2525.8 |
   2573.5 |
   2621.2 |########################################
   2669.0 |########################################
   2716.8 |
   2764.5 |
   2812.2 |
  (0 below, 1 above range)

```
