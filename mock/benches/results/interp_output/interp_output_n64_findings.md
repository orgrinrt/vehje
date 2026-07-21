# Interp output-building: format-to-temp+copy vs in-place vs span-list

3 variants, 6 samples per variant.
Baseline: **interp_out_inplace**

## Highlights

Baseline for all deltas below: **interp_out_inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### interp_out_inplace dominates: 12% faster than the next best (interp_out_temp)

interp_out_inplace (14.88 us) leads interp_out_temp (16.72 us) by 12%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### interp_out_spanlist is an outlier: 2.8x slower than the field

interp_out_spanlist (41.17 us) is 2.8x the fastest (14.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (interp_out_inplace)

The baseline interp_out_inplace is the fastest (14.88 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (interp_out_inplace) is the fastest** at 14882.0 ns median
- 2 variants significantly slower than baseline
- Spread: 2.77x (fastest 14882.0 ns, slowest 41173.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| interp_out_inplace | 17309ns | 17306ns | 15430ns | 17279ns | 18291ns | base |
| interp_out_spanlist | 43897ns | 43700ns | 39469ns | 43212ns | 47141ns | +153.62% |
| interp_out_temp | 19478ns | 19120ns | 18909ns | 19079ns | 20361ns | +12.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| interp_out_inplace | 14874ns | 13253ns | 15709ns | base | 0.004 |
| interp_out_spanlist | 41319ns | 37200ns | 44317ns | +177.79% | 0.002 |
| interp_out_temp | 17039ns | 16505ns | 17847ns | +14.55% | 0.004 |

## Performance model

- Peak throughput: **0.005 Gops/s** (interp_out_inplace; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| interp_out_inplace | 0.004 | 89.1% |
| interp_out_spanlist | 0.002 | 32.2% |
| interp_out_temp | 0.004 | 79.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| interp_out_inplace | 17309ns | 17309ns | base |
| interp_out_spanlist | 43897ns | 43897ns | +153.62% |
| interp_out_temp | 19478ns | 19478ns | +12.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| interp_out_inplace | 14882ns | base | --- | [14032, 15709] | --- | --- | --- | --- |
| interp_out_spanlist | 41173ns | +26034.2ns (+174.9%) | [+24435, +28866]ns | [38468, 44317] | YES | 0.0313 | 0.0313 | 0 |
| interp_out_temp | 16721ns | +2011.0ns (+13.5%) | [+1391, +3092]ns | [16549, 17847] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | interp_out_inplace | interp_out_spanlist | interp_out_temp |
|---|---|---|---|
| 1 | 15912ns | +196.6% | +12.8% |
| 2 | 15506ns | +167.2% | +7.0% |
| 3 | 14811ns | +168.3% | +13.4% |
| 4 | 14953ns | +176.9% | +18.6% |
| 5 | 14810ns | +176.5% | +11.4% |
| 6 | 13253ns | +180.7% | +25.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| interp_out_inplace | 0.172 | ok |
| interp_out_spanlist | 0.035 | ok |
| interp_out_temp | -0.315 | moderate- |

**Consistency summary:**

- **interp_out_spanlist**: won 0/6, lost 6/6
- **interp_out_temp**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| interp_out_inplace | 3.9ns | 14874.2ns | 0.0% |  |
| interp_out_spanlist | 5.8ns | 41319.3ns | 0.0% |  |
| interp_out_temp | 3.6ns | 17039.0ns | 0.0% |  |

## Distribution (algo ns)

```
interp_out_inplace (n=6, range 13253.3-15709.0 ns)
  13253.3 |####################
  13376.1 |
  13498.9 |
  13621.6 |
  13744.4 |
  13867.2 |
  13990.0 |
  14112.8 |
  14235.6 |
  14358.3 |
  14481.1 |
  14603.9 |
  14726.7 |########################################
  14849.5 |####################
  14972.3 |
  15095.0 |
  15217.8 |
  15340.6 |
  15463.4 |####################
  15586.2 |
  (0 below, 1 above range)

interp_out_spanlist (n=6, range 37200.0-44317.2 ns)
  37200.0 |####################
  37555.9 |
  37911.7 |
  38267.6 |
  38623.4 |
  38979.3 |
  39335.2 |
  39691.0 |####################
  40046.9 |
  40402.8 |
  40758.6 |####################
  41114.5 |########################################
  41470.3 |
  41826.2 |
  42182.1 |
  42537.9 |
  42893.8 |
  43249.7 |
  43605.5 |
  43961.4 |
  (0 below, 1 above range)

interp_out_temp (n=6, range 16505.0-17847.1 ns)
  16505.0 |########################################
  16572.1 |########################################
  16639.2 |########################################
  16706.3 |
  16773.4 |########################################
  16840.5 |
  16907.6 |
  16974.7 |
  17041.8 |
  17108.9 |
  17176.0 |
  17243.2 |
  17310.3 |
  17377.4 |
  17444.5 |
  17511.6 |
  17578.7 |
  17645.8 |
  17712.9 |########################################
  17780.0 |
  (0 below, 1 above range)

```
