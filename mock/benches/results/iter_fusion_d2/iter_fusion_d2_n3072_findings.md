# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 40% faster than the next best (iterfuse_mat2)

iterfuse_pull2 (36.05 us) leads iterfuse_mat2 (50.57 us) by 40%, a clear separation rather than a photo finish. CV 15.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push2 is an outlier: 3.8x slower than the field

iterfuse_push2 (135.44 us) is 3.8x the fastest (36.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iterfuse_pull2 is fastest but the noisiest (CV 15.5%)

iterfuse_pull2 wins on median (36.05 us) yet has the highest variance (CV 15.5%), while iterfuse_mat2 is the steadiest (CV 2.3%, 50.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_mat2 shows alternating (throttle bounce) (autocorr -0.59)

iterfuse_mat2's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (36.05 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.8x the fastest

Fastest iterfuse_pull2 (36.05 us) to slowest iterfuse_push2 (135.44 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 36052.9 ns median
- 2 variants significantly slower than baseline
- Spread: 3.76x (fastest 36052.9 ns, slowest 135435.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 53350ns | 53015ns | 51678ns | 52926ns | 54822ns | +30.93% |
| iterfuse_pull2 | 40747ns | 38480ns | 36454ns | 38304ns | 46559ns | base |
| iterfuse_push2 | 136018ns | 137995ns | 124365ns | 134978ns | 143405ns | +233.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 50881ns | 49313ns | 52286ns | +32.80% | 0.060 |
| iterfuse_pull2 | 38314ns | 34106ns | 44072ns | base | 0.080 |
| iterfuse_push2 | 133549ns | 122115ns | 140804ns | +248.57% | 0.023 |

## Performance model

- Peak throughput: **0.090 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 3072

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.061 | 67.4% |
| iterfuse_pull2 | 0.085 | 94.6% |
| iterfuse_push2 | 0.023 | 25.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 53350ns | 53350ns | +30.93% |
| iterfuse_pull2 | 40747ns | 40747ns | base |
| iterfuse_push2 | 136018ns | 136018ns | +233.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 36053ns | base | --- | [34815, 44072] | --- | --- | --- | --- |
| iterfuse_mat2 | 50572ns | +15329.8ns (+42.5%) | [+5712, +16660]ns | [49784, 52286] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| iterfuse_push2 | 135435ns | +98880.4ns (+274.3%) | [+81167, +105659]ns | [124407, 140804] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 50560ns | -0.6% | +141.5% |
| 2 | 35525ns | +42.3% | +277.8% |
| 3 | 34106ns | +48.3% | +320.6% |
| 4 | 36185ns | +46.5% | +281.8% |
| 5 | 37585ns | +31.2% | +263.6% |
| 6 | 35921ns | +43.5% | +252.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.586 | HIGH- (thermal bounce) |
| iterfuse_pull2 | -0.055 | ok |
| iterfuse_push2 | 0.123 | ok |

**Consistency summary:**

- **iterfuse_mat2**: won 1/6, lost 5/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 676.2ns | 50880.6ns | 1.3% |  |
| iterfuse_pull2 | 4.2ns | 38313.5ns | 0.0% |  |
| iterfuse_push2 | 4.0ns | 133548.8ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 49313.3-52286.1 ns)
  49313.3 |####################
  49461.9 |
  49610.6 |
  49759.2 |
  49907.9 |
  50056.5 |
  50205.1 |####################
  50353.8 |
  50502.4 |########################################
  50651.0 |
  50799.7 |
  50948.3 |
  51097.0 |
  51245.6 |
  51394.2 |
  51542.9 |####################
  51691.5 |
  51840.1 |
  51988.8 |
  52137.4 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 34106.2-44072.3 ns)
  34106.2 |########################################
  34604.5 |
  35102.8 |########################################
  35601.1 |########################################
  36099.4 |########################################
  36597.7 |
  37096.0 |########################################
  37594.3 |
  38092.6 |
  38590.9 |
  39089.2 |
  39587.6 |
  40085.9 |
  40584.2 |
  41082.5 |
  41580.8 |
  42079.1 |
  42577.4 |
  43075.7 |
  43574.0 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 122115.4-140804.2 ns)
  122115.4 |########################################
  123049.8 |
  123984.3 |
  124918.7 |
  125853.2 |########################################
  126787.6 |
  127722.0 |
  128656.5 |
  129590.9 |
  130525.4 |
  131459.8 |
  132394.2 |
  133328.7 |########################################
  134263.1 |
  135197.6 |
  136132.0 |########################################
  137066.4 |
  138000.9 |########################################
  138935.3 |
  139869.8 |
  (0 below, 1 above range)

```
