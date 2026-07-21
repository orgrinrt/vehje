# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_switch_v17 dominates: 18% faster than the next best (carrier_disp_fntable_v17)

carrier_disp_switch_v17 (2.22 us) leads carrier_disp_fntable_v17 (2.62 us) by 18%, a clear separation rather than a photo finish. CV 28.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_switch_v17 is fastest but the noisiest (CV 28.2%)

carrier_disp_switch_v17 wins on median (2.22 us) yet has the highest variance (CV 28.2%), while carrier_disp_fntable_v17 is the steadiest (CV 13.6%, 2.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_fntable_v17 shows warm-up / thermal drift (autocorr +0.55)

carrier_disp_fntable_v17's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (402 ns) is smaller than the fastest variant's own run-to-run std-dev (626 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (carrier_disp_switch_v17)

The baseline carrier_disp_switch_v17 is the fastest (2.22 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### carrier_disp_switch_v17 is inconsistent: worst-20% is 1.5x its best-20%

carrier_disp_switch_v17's best 20% of batches run at 2.12 us but its worst 20% at 3.22 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (carrier_disp_switch_v17) is the fastest** at 2215.4 ns median
- Spread: 1.18x (fastest 2215.4 ns, slowest 2617.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 5240ns | 4940ns | 4630ns | 4837ns | 6148ns | +2.78% |
| carrier_disp_switch_v17 | 5098ns | 4494ns | 4298ns | 4430ns | 6501ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2782ns | 2460ns | 3268ns | +10.41% | 0.023 |
| carrier_disp_switch_v17 | 2520ns | 2125ns | 3218ns | base | 0.025 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_disp_switch_v17; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.024 | 81.2% |
| carrier_disp_switch_v17 | 0.029 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 5240ns | 5240ns | +2.78% |
| carrier_disp_switch_v17 | 5098ns | 5098ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 2215ns | base | --- | [2126, 3218] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 2618ns | no significant difference | [-165, +600]ns | [2461, 3268] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 |
|---|---|---|
| 1 | 3881ns | -14.5% |
| 2 | 2554ns | +26.0% |
| 3 | 2203ns | +24.3% |
| 4 | 2125ns | +15.8% |
| 5 | 2228ns | +10.5% |
| 6 | 2128ns | +17.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | 0.552 | HIGH+ (drift/warm-up) |
| carrier_disp_switch_v17 | 0.166 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 55.1ns | 2782.1ns | 2.0% |  |
| carrier_disp_switch_v17 | 58.2ns | 2519.8ns | 2.3% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 2459.6-3267.5 ns)
   2459.6 |########################################
   2500.0 |
   2540.4 |
   2580.8 |
   2621.2 |
   2661.6 |
   2702.0 |#############
   2742.4 |
   2782.8 |
   2823.2 |
   2863.6 |
   2903.9 |
   2944.3 |
   2984.7 |
   3025.1 |
   3065.5 |
   3105.9 |
   3146.3 |
   3186.7 |#############
   3227.1 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 2124.6-3217.5 ns)
   2124.6 |########################################
   2179.2 |########################################
   2233.9 |
   2288.5 |
   2343.2 |
   2397.8 |
   2452.5 |
   2507.1 |####################
   2561.8 |
   2616.4 |
   2671.1 |
   2725.7 |
   2780.3 |
   2835.0 |
   2889.6 |
   2944.3 |
   2998.9 |
   3053.6 |
   3108.2 |
   3162.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_fntable_v17**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **carrier_disp_switch_v17**: CV=24.8% (high variance, measurements may be unstable)
