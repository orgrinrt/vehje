# Node record width: 16B pool-spill vs 24B inline operands

2 variants, 6 samples per variant.
Baseline: **hx_recwidth__rec24**

## Highlights

Baseline for all deltas below: **hx_recwidth__rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_recwidth__rec24 dominates: 228% faster than the next best (hx_recwidth__rec16)

hx_recwidth__rec24 (17.04 us) leads hx_recwidth__rec16 (55.84 us) by 228%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_recwidth__rec24)

The baseline hx_recwidth__rec24 is the fastest (17.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest hx_recwidth__rec24 (17.04 us) to slowest hx_recwidth__rec16 (55.84 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_recwidth__rec24) is the fastest** at 17036.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.28x (fastest 17036.5 ns, slowest 55840.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_recwidth__rec16 | 57901ns | 58057ns | 52247ns | 57680ns | 61059ns | +196.34% |
| hx_recwidth__rec24 | 19539ns | 19216ns | 18959ns | 19187ns | 20356ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_recwidth__rec16 | 55601ns | 49980ns | 58605ns | +221.02% | 0.295 |
| hx_recwidth__rec24 | 17320ns | 16815ns | 18037ns | base | 0.946 |

## Performance model

- Peak throughput: **0.974 Gops/s** (hx_recwidth__rec24; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_recwidth__rec16 | 0.293 | 30.1% |
| hx_recwidth__rec24 | 0.962 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_recwidth__rec16 | 57901ns | 57901ns | +196.34% |
| hx_recwidth__rec24 | 19539ns | 19539ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_recwidth__rec24 | 17036ns | base | --- | [16888, 18037] | --- | --- | --- | --- |
| hx_recwidth__rec16 | 55841ns | +38369.0ns (+225.2%) | [+35100, +41374]ns | [52359, 58605] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_recwidth__rec24 | hx_recwidth__rec16 |
|---|---|---|
| 1 | 16981ns | +226.6% |
| 2 | 16961ns | +222.7% |
| 3 | 17557ns | +184.7% |
| 4 | 17092ns | +228.9% |
| 5 | 18517ns | +206.6% |
| 6 | 16815ns | +259.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_recwidth__rec16 | 0.137 | ok |
| hx_recwidth__rec24 | -0.439 | moderate- |

**Consistency summary:**

- **hx_recwidth__rec16**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_recwidth__rec16 | 5.3ns | 55601.4ns | 0.0% |  |
| hx_recwidth__rec24 | 2.5ns | 17320.3ns | 0.0% |  |

## Distribution (algo ns)

```
hx_recwidth__rec16 (n=6, range 49979.6-58604.8 ns)
  49979.6 |########################################
  50410.9 |
  50842.1 |
  51273.4 |
  51704.6 |
  52135.9 |
  52567.2 |
  52998.4 |
  53429.7 |
  53860.9 |
  54292.2 |
  54723.5 |########################################
  55154.7 |########################################
  55586.0 |
  56017.2 |########################################
  56448.5 |########################################
  56879.8 |
  57311.0 |
  57742.3 |
  58173.5 |
  (0 below, 1 above range)

hx_recwidth__rec24 (n=6, range 16814.6-18036.7 ns)
  16814.6 |####################
  16875.7 |
  16936.8 |########################################
  16997.9 |
  17059.0 |####################
  17120.1 |
  17181.2 |
  17242.3 |
  17303.4 |
  17364.5 |
  17425.7 |
  17486.8 |
  17547.9 |####################
  17609.0 |
  17670.1 |
  17731.2 |
  17792.3 |
  17853.4 |
  17914.5 |
  17975.6 |
  (0 below, 1 above range)

```
