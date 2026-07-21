# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_switch_v17 dominates: 31% faster than the next best (carrier_disp_fntable_v17)

carrier_disp_switch_v17 (36.38 us) leads carrier_disp_fntable_v17 (47.67 us) by 31%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_disp_switch_v17)

The baseline carrier_disp_switch_v17 is the fastest (36.38 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_disp_switch_v17) is the fastest** at 36377.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.31x (fastest 36377.8 ns, slowest 47672.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 50153ns | 49962ns | 49623ns | 49866ns | 50849ns | +30.17% |
| carrier_disp_switch_v17 | 38530ns | 38769ns | 37359ns | 38570ns | 39056ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 47775ns | 47395ns | 48241ns | +31.93% | 0.021 |
| carrier_disp_switch_v17 | 36212ns | 35155ns | 36705ns | base | 0.028 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_disp_switch_v17; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.021 | 73.7% |
| carrier_disp_switch_v17 | 0.028 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 50153ns | 50153ns | +30.17% |
| carrier_disp_switch_v17 | 38530ns | 38530ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 36378ns | base | --- | [35554, 36705] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 47672ns | +11470.4ns (+31.5%) | [+10936, +12280]ns | [47411, 48241] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 |
|---|---|---|
| 1 | 35155ns | +35.2% |
| 2 | 36442ns | +33.4% |
| 3 | 36314ns | +31.7% |
| 4 | 36852ns | +28.7% |
| 5 | 35953ns | +31.8% |
| 6 | 36558ns | +30.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | -0.082 | ok |
| carrier_disp_switch_v17 | -0.231 | moderate- |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 160.8ns | 47774.6ns | 0.3% |  |
| carrier_disp_switch_v17 | 164.5ns | 36212.4ns | 0.5% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 47395.0-48240.8 ns)
  47395.0 |########################################
  47437.3 |
  47479.6 |
  47521.9 |####################
  47564.2 |
  47606.4 |
  47648.7 |
  47691.0 |
  47733.3 |
  47775.6 |####################
  47817.9 |####################
  47860.2 |
  47902.5 |
  47944.8 |
  47987.1 |
  48029.4 |
  48071.6 |
  48113.9 |
  48156.2 |
  48198.5 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 35155.4-36705.2 ns)
  35155.4 |########################################
  35232.9 |
  35310.4 |
  35387.9 |
  35465.4 |
  35542.8 |
  35620.3 |
  35697.8 |
  35775.3 |
  35852.8 |
  35930.3 |########################################
  36007.8 |
  36085.3 |
  36162.8 |
  36240.3 |########################################
  36317.8 |
  36395.2 |########################################
  36472.7 |
  36550.2 |########################################
  36627.7 |
  (0 below, 1 above range)

```
