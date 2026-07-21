# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 38% faster than the next best (iterfuse_mat2)

iterfuse_pull2 (49.99 us) leads iterfuse_mat2 (69.19 us) by 38%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push2 is an outlier: 3.8x slower than the field

iterfuse_push2 (188.08 us) is 3.8x the fastest (49.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iterfuse_pull2 is fastest but the noisiest (CV 10.1%)

iterfuse_pull2 wins on median (49.99 us) yet has the highest variance (CV 10.1%), while iterfuse_push2 is the steadiest (CV 2.9%, 188.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (49.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.8x the fastest

Fastest iterfuse_pull2 (49.99 us) to slowest iterfuse_push2 (188.08 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 49991.1 ns median
- 2 variants significantly slower than baseline
- Spread: 3.76x (fastest 49991.1 ns, slowest 188080.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 71730ns | 71445ns | 65884ns | 70726ns | 76159ns | +35.95% |
| iterfuse_pull2 | 52762ns | 52230ns | 46395ns | 50712ns | 59020ns | base |
| iterfuse_push2 | 190095ns | 190573ns | 181558ns | 188681ns | 196484ns | +260.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 69428ns | 63488ns | 73789ns | +37.60% | 0.059 |
| iterfuse_pull2 | 50457ns | 44220ns | 56494ns | base | 0.081 |
| iterfuse_push2 | 187608ns | 179197ns | 193833ns | +271.82% | 0.022 |

## Performance model

- Peak throughput: **0.093 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.059 | 63.9% |
| iterfuse_pull2 | 0.082 | 88.5% |
| iterfuse_push2 | 0.022 | 23.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 71730ns | 71730ns | +35.95% |
| iterfuse_pull2 | 52762ns | 52762ns | base |
| iterfuse_push2 | 190095ns | 190095ns | +260.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 49991ns | base | --- | [44886, 56494] | --- | --- | --- | --- |
| iterfuse_mat2 | 69193ns | +19843.8ns (+39.7%) | [+13074, +23994]ns | [65301, 73789] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 188081ns | +137168.5ns (+274.4%) | [+132922, +141361]ns | [180910, 193833] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 47588ns | +50.0% | +276.6% |
| 2 | 45552ns | +47.3% | +316.1% |
| 3 | 58092ns | +31.2% | +234.0% |
| 4 | 44220ns | +54.8% | +313.0% |
| 5 | 52394ns | +21.2% | +256.2% |
| 6 | 54897ns | +27.4% | +252.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.263 | moderate- |
| iterfuse_pull2 | -0.487 | moderate- |
| iterfuse_push2 | -0.207 | moderate- |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 716.4ns | 69427.9ns | 1.0% |  |
| iterfuse_pull2 | 2.3ns | 50457.3ns | 0.0% |  |
| iterfuse_push2 | 3.9ns | 187608.1ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 63487.9-73789.2 ns)
  63487.9 |########################################
  64003.0 |
  64518.0 |
  65033.1 |
  65548.2 |
  66063.2 |
  66578.3 |
  67093.4 |########################################
  67608.4 |
  68123.5 |########################################
  68638.6 |
  69153.6 |
  69668.7 |########################################
  70183.7 |
  70698.8 |
  71213.9 |########################################
  71728.9 |
  72244.0 |
  72759.1 |
  73274.1 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 44220.4-56494.4 ns)
  44220.4 |########################################
  44834.1 |
  45447.8 |########################################
  46061.5 |
  46675.2 |
  47288.9 |########################################
  47902.6 |
  48516.3 |
  49130.0 |
  49743.7 |
  50357.4 |
  50971.1 |
  51584.8 |
  52198.5 |########################################
  52812.2 |
  53425.9 |
  54039.6 |
  54653.3 |########################################
  55267.0 |
  55880.7 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 179197.1-193833.1 ns)
  179197.1 |########################################
  179928.9 |
  180660.7 |
  181392.5 |
  182124.3 |########################################
  182856.1 |
  183587.9 |
  184319.7 |
  185051.5 |
  185783.3 |
  186515.1 |########################################
  187246.9 |
  187978.7 |
  188710.5 |
  189442.3 |########################################
  190174.1 |
  190905.9 |
  191637.7 |
  192369.5 |
  193101.3 |########################################
  (0 below, 1 above range)

```
