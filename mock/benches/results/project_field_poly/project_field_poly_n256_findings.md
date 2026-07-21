# Project field access, polymorphic site: inline-cache hazard vs hash vs linear

3 variants, 6 samples per variant.
Baseline: **project_poly_hash**

## Highlights

Baseline for all deltas below: **project_poly_hash**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_poly_hash dominates: 16% faster than the next best (project_poly_linear)

project_poly_hash (631 ns) leads project_poly_linear (731 ns) by 16%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (project_poly_hash)

The baseline project_poly_hash is the fastest (631 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (project_poly_hash) is the fastest** at 631.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.20x (fastest 631.2 ns, slowest 757.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_poly_hash | 3084ns | 3198ns | 2668ns | 3056ns | 3334ns | base |
| project_poly_ic | 3174ns | 3324ns | 2798ns | 3179ns | 3355ns | +2.92% |
| project_poly_linear | 3163ns | 3218ns | 2751ns | 3064ns | 3517ns | +2.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_poly_hash | 607ns | 521ns | 657ns | base | 0.422 |
| project_poly_ic | 720ns | 620ns | 770ns | +18.65% | 0.356 |
| project_poly_linear | 700ns | 605ns | 763ns | +15.47% | 0.365 |

## Performance model

- Peak throughput: **0.491 Gops/s** (project_poly_hash; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_poly_hash | 0.406 | 82.6% |
| project_poly_ic | 0.338 | 68.8% |
| project_poly_linear | 0.350 | 71.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_poly_hash | 3084ns | 3084ns | base |
| project_poly_ic | 3174ns | 3174ns | +2.92% |
| project_poly_linear | 3163ns | 3163ns | +2.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_poly_hash | 631ns | base | --- | [531, 657] | --- | --- | --- | --- |
| project_poly_ic | 757ns | +110.8ns (+17.6%) | [+92, +136]ns | [632, 770] | YES | 0.0313 | 0.0313 | 0 |
| project_poly_linear | 731ns | +100.0ns (+15.8%) | [+75, +106]ns | [607, 763] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_poly_hash | project_poly_ic | project_poly_linear |
|---|---|---|---|
| 1 | 542ns | +19.1% | +12.5% |
| 2 | 521ns | +19.0% | +16.0% |
| 3 | 635ns | +21.2% | +16.5% |
| 4 | 630ns | +18.8% | +16.2% |
| 5 | 633ns | +21.7% | +15.5% |
| 6 | 680ns | +12.7% | +15.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_poly_hash | 0.335 | moderate+ |
| project_poly_ic | 0.327 | moderate+ |
| project_poly_linear | 0.345 | moderate+ |

**Consistency summary:**

- **project_poly_ic**: won 0/6, lost 6/6
- **project_poly_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_poly_hash | 4.1ns | 606.7ns | 0.7% |  |
| project_poly_ic | 3.5ns | 719.8ns | 0.5% |  |
| project_poly_linear | 2.9ns | 700.5ns | 0.4% |  |

## Distribution (algo ns)

```
project_poly_hash (n=6, range 521.2-657.3 ns)
    521.2 |####################
    528.0 |
    534.8 |
    541.6 |####################
    548.4 |
    555.2 |
    562.0 |
    568.8 |
    575.6 |
    582.4 |
    589.2 |
    596.1 |
    602.9 |
    609.7 |
    616.5 |
    623.3 |####################
    630.1 |########################################
    636.9 |
    643.7 |
    650.5 |
  (0 below, 1 above range)

project_poly_ic (n=6, range 620.0-769.8 ns)
    620.0 |####################
    627.5 |
    635.0 |
    642.5 |####################
    650.0 |
    657.5 |
    664.9 |
    672.4 |
    679.9 |
    687.4 |
    694.9 |
    702.4 |
    709.9 |
    717.4 |
    724.9 |
    732.3 |
    739.8 |
    747.3 |####################
    754.8 |
    762.3 |########################################
  (0 below, 1 above range)

project_poly_linear (n=6, range 604.6-763.4 ns)
    604.6 |########################################
    612.5 |
    620.5 |
    628.4 |
    636.4 |
    644.3 |
    652.2 |
    660.2 |
    668.1 |
    676.0 |
    684.0 |
    691.9 |
    699.9 |
    707.8 |
    715.7 |
    723.7 |####################
    731.6 |########################################
    739.5 |
    747.5 |
    755.4 |
  (0 below, 1 above range)

```
