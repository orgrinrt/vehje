# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_switch_v4 dominates: 31% faster than the next best (carrier_disp_fntable_v4)

carrier_disp_switch_v4 (34.28 us) leads carrier_disp_fntable_v4 (45.07 us) by 31%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_disp_switch_v4)

The baseline carrier_disp_switch_v4 is the fastest (34.28 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_disp_switch_v4) is the fastest** at 34278.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.31x (fastest 34278.1 ns, slowest 45065.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 47654ns | 47341ns | 45973ns | 46998ns | 49479ns | +29.54% |
| carrier_disp_switch_v4 | 36787ns | 36520ns | 35402ns | 36376ns | 38096ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 45349ns | 43801ns | 47040ns | +31.41% | 0.023 |
| carrier_disp_switch_v4 | 34509ns | 33189ns | 35726ns | base | 0.030 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_switch_v4; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.023 | 73.6% |
| carrier_disp_switch_v4 | 0.030 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 47654ns | 47654ns | +29.54% |
| carrier_disp_switch_v4 | 36787ns | 36787ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 34278ns | base | --- | [33524, 35726] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 45065ns | +10874.6ns (+31.7%) | [+9927, +11717]ns | [43940, 47040] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 |
|---|---|---|
| 1 | 34546ns | +26.8% |
| 2 | 33189ns | +32.8% |
| 3 | 34010ns | +31.2% |
| 4 | 33859ns | +34.4% |
| 5 | 34701ns | +31.3% |
| 6 | 36751ns | +32.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.237 | moderate+ |
| carrier_disp_switch_v4 | 0.166 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 155.5ns | 45348.6ns | 0.3% |  |
| carrier_disp_switch_v4 | 152.0ns | 34509.3ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 43800.8-47040.2 ns)
  43800.8 |####################
  43962.8 |####################
  44124.7 |
  44286.7 |
  44448.7 |####################
  44610.7 |
  44772.6 |
  44934.6 |
  45096.6 |
  45258.5 |
  45420.5 |########################################
  45582.5 |
  45744.4 |
  45906.4 |
  46068.4 |
  46230.3 |
  46392.3 |
  46554.3 |
  46716.3 |
  46878.2 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 33188.8-35726.0 ns)
  33188.8 |########################################
  33315.7 |
  33442.5 |
  33569.4 |
  33696.2 |
  33823.1 |########################################
  33950.0 |########################################
  34076.8 |
  34203.7 |
  34330.5 |
  34457.4 |########################################
  34584.3 |########################################
  34711.1 |
  34838.0 |
  34964.8 |
  35091.7 |
  35218.6 |
  35345.4 |
  35472.3 |
  35599.1 |
  (0 below, 1 above range)

```
