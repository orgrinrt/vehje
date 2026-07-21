# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_disp_switch_v17) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_disp_switch_v17 has the worst median (2.55 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_disp_fntable_v17 at 2.15 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_disp_fntable_v17 dominates: 19% faster than the next best (carrier_disp_switch_v17)

carrier_disp_fntable_v17 (2.15 ms) leads carrier_disp_switch_v17 (2.55 ms) by 19%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_disp_fntable_v17** at 2145874.0 ns median (-15.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.19x (fastest 2145874.0 ns, slowest 2551341.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2147884ns | 2149455ns | 2130585ns | 2144521ns | 2161577ns | -15.86% |
| carrier_disp_switch_v17 | 2552626ns | 2554546ns | 2526899ns | 2551134ns | 2567728ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2144518ns | 2127132ns | 2158079ns | -15.88% | 0.008 |
| carrier_disp_switch_v17 | 2549427ns | 2523681ns | 2564593ns | base | 0.006 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_disp_fntable_v17; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.008 | 99.1% |
| carrier_disp_switch_v17 | 0.006 | 83.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 2147884ns | 2147884ns | -15.86% |
| carrier_disp_switch_v17 | 2552626ns | 2552626ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 2551341ns | base | --- | [2532347, 2564593] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 2145874ns | -405077.1ns (-15.9%) | [-426535, -383115]ns | [2129600, 2158079] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 |
|---|---|---|
| 1 | 2541012ns | -15.6% |
| 2 | 2572269ns | -16.1% |
| 3 | 2555355ns | -16.6% |
| 4 | 2547328ns | -15.3% |
| 5 | 2523681ns | -14.9% |
| 6 | 2556916ns | -16.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | -0.431 | moderate- |
| carrier_disp_switch_v17 | -0.154 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2331.2ns | 2144517.8ns | 0.1% |  |
| carrier_disp_switch_v17 | 2402.0ns | 2549426.9ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 2127132.5-2158079.2 ns)
  2127132.5 |########################################
  2128679.8 |
  2130227.2 |
  2131774.5 |########################################
  2133321.8 |
  2134869.2 |
  2136416.5 |
  2137963.8 |
  2139511.2 |
  2141058.5 |
  2142605.9 |########################################
  2144153.2 |
  2145700.5 |
  2147247.9 |########################################
  2148795.2 |
  2150342.5 |
  2151889.9 |
  2153437.2 |
  2154984.5 |
  2156531.9 |########################################
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 2523681.2-2564592.7 ns)
  2523681.2 |########################################
  2525726.8 |
  2527772.4 |
  2529817.9 |
  2531863.5 |
  2533909.1 |
  2535954.7 |
  2538000.2 |
  2540045.8 |########################################
  2542091.4 |
  2544137.0 |
  2546182.5 |########################################
  2548228.1 |
  2550273.7 |
  2552319.2 |
  2554364.8 |########################################
  2556410.4 |########################################
  2558456.0 |
  2560501.6 |
  2562547.1 |
  (0 below, 1 above range)

```
