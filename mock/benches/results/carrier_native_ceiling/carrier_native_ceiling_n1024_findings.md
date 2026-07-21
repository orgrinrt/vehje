# Native ceiling: switch vs fn-table interp vs shape-specialized native, opaque program (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_ceiling_native**

## Highlights

Baseline for all deltas below: **carrier_ceiling_native**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ceiling_native dominates: 105% faster than the next best (carrier_ceiling_switch)

carrier_ceiling_native (18.55 us) leads carrier_ceiling_switch (37.99 us) by 105%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceiling_fntable is an outlier: 2.2x slower than the field

carrier_ceiling_fntable (41.02 us) is 2.2x the fastest (18.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (carrier_ceiling_native)

The baseline carrier_ceiling_native is the fastest (18.55 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_ceiling_native) is the fastest** at 18549.8 ns median
- 2 variants significantly slower than baseline
- Spread: 2.21x (fastest 18549.8 ns, slowest 41017.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceiling_fntable | 42510ns | 43457ns | 37728ns | 43267ns | 43765ns | +101.40% |
| carrier_ceiling_native | 21107ns | 20968ns | 20726ns | 20966ns | 21509ns | base |
| carrier_ceiling_switch | 40212ns | 40466ns | 37031ns | 40414ns | 41500ns | +90.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceiling_fntable | 40091ns | 35613ns | 41247ns | +114.68% | 0.026 |
| carrier_ceiling_native | 18675ns | 18344ns | 19031ns | base | 0.055 |
| carrier_ceiling_switch | 37768ns | 34747ns | 39002ns | +102.24% | 0.027 |

## Performance model

- Peak throughput: **0.056 Gops/s** (carrier_ceiling_native; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceiling_fntable | 0.025 | 44.7% |
| carrier_ceiling_native | 0.055 | 98.9% |
| carrier_ceiling_switch | 0.027 | 48.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceiling_fntable | 42510ns | 42510ns | +101.40% |
| carrier_ceiling_native | 21107ns | 21107ns | base |
| carrier_ceiling_switch | 40212ns | 40212ns | +90.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceiling_native | 18550ns | base | --- | [18443, 19031] | --- | --- | --- | --- |
| carrier_ceiling_fntable | 41017ns | +22123.5ns (+119.3%) | [+19563, +22562]ns | [38008, 41247] | YES | 0.0313 | 0.0313 | 0 |
| carrier_ceiling_switch | 37993ns | +19258.6ns (+103.8%) | [+17606, +20416]ns | [36309, 39002] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceiling_native | carrier_ceiling_fntable | carrier_ceiling_switch |
|---|---|---|---|
| 1 | 18344ns | +94.1% | +89.4% |
| 2 | 18921ns | +116.9% | +101.4% |
| 3 | 18542ns | +122.3% | +105.1% |
| 4 | 18554ns | +121.0% | +115.0% |
| 5 | 18546ns | +117.9% | +104.2% |
| 6 | 19141ns | +115.6% | +98.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceiling_fntable | -0.060 | ok |
| carrier_ceiling_native | -0.327 | moderate- |
| carrier_ceiling_switch | -0.010 | ok |

**Consistency summary:**

- **carrier_ceiling_fntable**: won 0/6, lost 6/6
- **carrier_ceiling_switch**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceiling_fntable | 182.7ns | 40090.8ns | 0.5% |  |
| carrier_ceiling_native | 4.6ns | 18674.7ns | 0.0% |  |
| carrier_ceiling_switch | 189.2ns | 37768.0ns | 0.5% |  |

## Distribution (algo ns)

```
carrier_ceiling_fntable (n=6, range 35612.9-41247.2 ns)
  35612.9 |#############
  35894.6 |
  36176.3 |
  36458.1 |
  36739.8 |
  37021.5 |
  37303.2 |
  37584.9 |
  37866.6 |
  38148.4 |
  38430.1 |
  38711.8 |
  38993.5 |
  39275.2 |
  39556.9 |
  39838.7 |
  40120.4 |
  40402.1 |#############
  40683.8 |
  40965.5 |########################################
  (0 below, 1 above range)

carrier_ceiling_native (n=6, range 18344.2-19031.0 ns)
  18344.2 |####################
  18378.5 |
  18412.9 |
  18447.2 |
  18481.6 |
  18515.9 |########################################
  18550.2 |####################
  18584.6 |
  18618.9 |
  18653.3 |
  18687.6 |
  18721.9 |
  18756.3 |
  18790.6 |
  18825.0 |
  18859.3 |
  18893.6 |####################
  18928.0 |
  18962.3 |
  18996.7 |
  (0 below, 1 above range)

carrier_ceiling_switch (n=6, range 34747.1-39002.3 ns)
  34747.1 |#############
  34959.9 |
  35172.6 |
  35385.4 |
  35598.1 |
  35810.9 |
  36023.7 |
  36236.4 |
  36449.2 |
  36661.9 |
  36874.7 |
  37087.5 |
  37300.2 |
  37513.0 |
  37725.7 |#############
  37938.5 |########################################
  38151.3 |
  38364.0 |
  38576.8 |
  38789.5 |
  (0 below, 1 above range)

```
