# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 126% faster than the next best (iterfuse_mat2)

iterfuse_pull2 (22.32 us) leads iterfuse_mat2 (50.48 us) by 126%, a clear separation rather than a photo finish. CV 6.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push2 is an outlier: 3.6x slower than the field

iterfuse_push2 (79.38 us) is 3.6x the fastest (22.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (22.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.6x the fastest

Fastest iterfuse_pull2 (22.32 us) to slowest iterfuse_push2 (79.38 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 22317.9 ns median
- 2 variants significantly slower than baseline
- Spread: 3.56x (fastest 22317.9 ns, slowest 79384.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 53583ns | 52972ns | 43731ns | 51659ns | 61395ns | +116.54% |
| iterfuse_pull2 | 24745ns | 24564ns | 22912ns | 24033ns | 26729ns | base |
| iterfuse_push2 | 79623ns | 82039ns | 67096ns | 78688ns | 87291ns | +221.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 51137ns | 41111ns | 58974ns | +127.53% | 0.040 |
| iterfuse_pull2 | 22475ns | 20779ns | 24287ns | base | 0.091 |
| iterfuse_push2 | 77124ns | 64932ns | 84751ns | +243.15% | 0.027 |

## Performance model

- Peak throughput: **0.099 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.041 | 41.2% |
| iterfuse_pull2 | 0.092 | 93.1% |
| iterfuse_push2 | 0.026 | 26.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 53583ns | 53583ns | +116.54% |
| iterfuse_pull2 | 24745ns | 24745ns | base |
| iterfuse_push2 | 79623ns | 79623ns | +221.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 22318ns | base | --- | [20820, 24287] | --- | --- | --- | --- |
| iterfuse_mat2 | 50485ns | +27385.0ns (+122.7%) | [+22197, +36404]ns | [43954, 58974] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 79385ns | +55765.8ns (+249.9%) | [+44698, +63482]ns | [67236, 84751] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 20862ns | +124.3% | +211.3% |
| 2 | 21759ns | +88.9% | +309.9% |
| 3 | 20779ns | +166.2% | +286.5% |
| 4 | 22877ns | +109.5% | +246.9% |
| 5 | 24214ns | +119.1% | +187.2% |
| 6 | 24360ns | +157.1% | +226.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.014 | ok |
| iterfuse_pull2 | 0.445 | moderate+ |
| iterfuse_push2 | -0.365 | moderate- |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 468.4ns | 51137.4ns | 0.9% |  |
| iterfuse_pull2 | 3.5ns | 22475.2ns | 0.0% |  |
| iterfuse_push2 | 4.0ns | 77124.0ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 41111.2-58974.1 ns)
  41111.2 |########################################
  42004.3 |
  42897.5 |
  43790.6 |
  44683.8 |
  45576.9 |
  46470.1 |########################################
  47363.2 |########################################
  48256.4 |
  49149.5 |
  50042.7 |
  50935.8 |
  51829.0 |
  52722.1 |########################################
  53615.3 |
  54508.4 |########################################
  55401.6 |
  56294.7 |
  57187.9 |
  58081.0 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 20779.2-24287.3 ns)
  20779.2 |########################################
  20954.6 |
  21130.0 |
  21305.4 |
  21480.8 |
  21656.2 |####################
  21831.6 |
  22007.0 |
  22182.4 |
  22357.8 |
  22533.2 |
  22708.7 |####################
  22884.1 |
  23059.5 |
  23234.9 |
  23410.3 |
  23585.7 |
  23761.1 |
  23936.5 |
  24111.9 |####################
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 64932.1-84751.0 ns)
  64932.1 |####################
  65923.0 |
  66914.0 |
  67904.9 |
  68895.9 |####################
  69886.8 |
  70877.8 |
  71868.7 |
  72859.7 |
  73850.6 |
  74841.6 |
  75832.5 |
  76823.5 |
  77814.4 |
  78805.4 |########################################
  79796.3 |####################
  80787.3 |
  81778.2 |
  82769.2 |
  83760.1 |
  (0 below, 1 above range)

```
