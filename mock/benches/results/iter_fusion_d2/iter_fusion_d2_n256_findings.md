# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (iterfuse_push2, iterfuse_pull2) are a dead heat (<1%)

iterfuse_push2 (3.22 us) and iterfuse_pull2 (3.24 us) differ by 0.50%, inside the noise, even though the wider field spreads 45.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### iterfuse_push2's edge over baseline is significant but tiny (-16 ns, 0.50%)

iterfuse_push2 differs from baseline iterfuse_pull2 by -16 ns (0.50%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: iterfuse_push2** at 3220.8 ns median (-0.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.46x (fastest 3220.8 ns, slowest 4689.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 7079ns | 7273ns | 5950ns | 7232ns | 7412ns | +24.77% |
| iterfuse_pull2 | 5673ns | 5807ns | 4915ns | 5775ns | 5900ns | base |
| iterfuse_push2 | 5660ns | 5790ns | 4934ns | 5774ns | 5852ns | -0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 4555ns | 3849ns | 4754ns | +44.00% | 0.056 |
| iterfuse_pull2 | 3163ns | 2749ns | 3303ns | base | 0.081 |
| iterfuse_push2 | 3146ns | 2757ns | 3265ns | -0.52% | 0.081 |

## Performance model

- Peak throughput: **0.093 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.055 | 58.6% |
| iterfuse_pull2 | 0.079 | 84.9% |
| iterfuse_push2 | 0.079 | 85.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 7079ns | 7079ns | +24.77% |
| iterfuse_pull2 | 5673ns | 5673ns | base |
| iterfuse_push2 | 5660ns | 5660ns | -0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 3237ns | base | --- | [2949, 3303] | --- | --- | --- | --- |
| iterfuse_mat2 | 4689ns | +1446.2ns (+44.7%) | [+1254, +1474]ns | [4221, 4754] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push2 | 3221ns | no significant difference | [-39, +5]ns | [2953, 3265] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 2749ns | +40.0% | +0.3% |
| 2 | 3265ns | +43.1% | -1.3% |
| 3 | 3256ns | +44.5% | -1.0% |
| 4 | 3149ns | +45.8% | +0.0% |
| 5 | 3341ns | +43.4% | -1.0% |
| 6 | 3218ns | +46.6% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.021 | ok |
| iterfuse_pull2 | -0.119 | ok |
| iterfuse_push2 | -0.055 | ok |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 132.5ns | 4554.5ns | 2.9% |  |
| iterfuse_pull2 | 4.7ns | 3162.9ns | 0.1% |  |
| iterfuse_push2 | 4.4ns | 3146.4ns | 0.1% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 3848.8-4753.8 ns)
   3848.8 |####################
   3894.0 |
   3939.3 |
   3984.5 |
   4029.8 |
   4075.0 |
   4120.3 |
   4165.5 |
   4210.8 |
   4256.0 |
   4301.3 |
   4346.5 |
   4391.8 |
   4437.0 |
   4482.3 |
   4527.5 |
   4572.8 |####################
   4618.0 |
   4663.3 |########################################
   4708.5 |####################
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 2748.8-3302.9 ns)
   2748.8 |####################
   2776.5 |
   2804.2 |
   2831.9 |
   2859.6 |
   2887.3 |
   2915.0 |
   2942.7 |
   2970.4 |
   2998.1 |
   3025.9 |
   3053.6 |
   3081.3 |
   3109.0 |
   3136.7 |####################
   3164.4 |
   3192.1 |####################
   3219.8 |
   3247.5 |########################################
   3275.2 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 2757.1-3265.2 ns)
   2757.1 |#############
   2782.5 |
   2807.9 |
   2833.3 |
   2858.7 |
   2884.1 |
   2909.5 |
   2934.9 |
   2960.3 |
   2985.7 |
   3011.1 |
   3036.6 |
   3062.0 |
   3087.4 |
   3112.8 |
   3138.2 |#############
   3163.6 |
   3189.0 |
   3214.4 |########################################
   3239.8 |
  (0 below, 1 above range)

```
