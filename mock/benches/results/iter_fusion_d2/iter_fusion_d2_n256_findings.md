# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_push2 dominates: 10% faster than the next best (iterfuse_pull2)

iterfuse_push2 (2.73 us) leads iterfuse_pull2 (3.01 us) by 10%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push2 is fastest but the noisiest (CV 10.1%)

iterfuse_push2 wins on median (2.73 us) yet has the highest variance (CV 10.1%), while iterfuse_mat2 is the steadiest (CV 7.9%, 4.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_pull2 shows warm-up / thermal drift (autocorr +0.56)

iterfuse_pull2's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### iterfuse_push2's edge over baseline is significant but tiny (-42 ns, 1.39%)

iterfuse_push2 differs from baseline iterfuse_pull2 by -42 ns (1.39%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: iterfuse_push2** at 2729.8 ns median (-9.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.49x (fastest 2729.8 ns, slowest 4077.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 6596ns | 6336ns | 6072ns | 6288ns | 7320ns | +22.20% |
| iterfuse_pull2 | 5398ns | 5425ns | 4901ns | 5254ns | 5861ns | base |
| iterfuse_push2 | 5197ns | 4901ns | 4733ns | 4888ns | 5892ns | -3.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 4243ns | 3896ns | 4671ns | +41.00% | 0.060 |
| iterfuse_pull2 | 3010ns | 2735ns | 3283ns | base | 0.085 |
| iterfuse_push2 | 2882ns | 2600ns | 3264ns | -4.25% | 0.089 |

## Performance model

- Peak throughput: **0.098 Gops/s** (iterfuse_push2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.063 | 63.8% |
| iterfuse_pull2 | 0.085 | 86.5% |
| iterfuse_push2 | 0.094 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 6596ns | 6596ns | +22.20% |
| iterfuse_pull2 | 5398ns | 5398ns | base |
| iterfuse_push2 | 5197ns | 5197ns | -3.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 3006ns | base | --- | [2740, 3283] | --- | --- | --- | --- |
| iterfuse_mat2 | 4078ns | +1253.5ns (+41.7%) | [+1032, +1416]ns | [3982, 4671] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push2 | 2730ns | -41.7ns (-1.4%) | [-328, -14]ns | [2651, 3264] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 2856ns | +42.7% | -3.5% |
| 2 | 2744ns | +42.0% | -1.5% |
| 3 | 2735ns | +49.2% | -1.1% |
| 4 | 3155ns | +28.9% | -17.6% |
| 5 | 3348ns | +44.4% | -1.2% |
| 6 | 3219ns | +40.0% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.314 | moderate+ |
| iterfuse_pull2 | 0.555 | HIGH+ (drift/warm-up) |
| iterfuse_push2 | 0.284 | moderate+ |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 126.1ns | 4243.3ns | 3.0% |  |
| iterfuse_pull2 | 4.1ns | 3009.5ns | 0.1% |  |
| iterfuse_push2 | 3.6ns | 2881.5ns | 0.1% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 3895.8-4670.6 ns)
   3895.8 |#############
   3934.5 |
   3973.3 |
   4012.0 |
   4050.8 |########################################
   4089.5 |
   4128.2 |
   4167.0 |
   4205.7 |
   4244.5 |
   4283.2 |
   4321.9 |
   4360.7 |
   4399.4 |
   4438.2 |
   4476.9 |#############
   4515.6 |
   4554.4 |
   4593.1 |
   4631.9 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 2735.0-3283.3 ns)
   2735.0 |########################################
   2762.4 |
   2789.8 |
   2817.3 |
   2844.7 |####################
   2872.1 |
   2899.5 |
   2926.9 |
   2954.3 |
   2981.8 |
   3009.2 |
   3036.6 |
   3064.0 |
   3091.4 |
   3118.8 |
   3146.3 |####################
   3173.7 |
   3201.1 |####################
   3228.5 |
   3255.9 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 2599.6-3263.8 ns)
   2599.6 |####################
   2632.8 |
   2666.0 |
   2699.2 |########################################
   2732.4 |####################
   2765.6 |
   2798.8 |
   2832.1 |
   2865.3 |
   2898.5 |
   2931.7 |
   2964.9 |
   2998.1 |
   3031.3 |
   3064.5 |
   3097.7 |
   3130.9 |
   3164.1 |
   3197.3 |####################
   3230.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **iterfuse_pull2**: autocorrelation=0.56 (measurement drift or warm-up artifact)
