# Iterator fusion (depth 3): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull3**

## Highlights

Baseline for all deltas below: **iterfuse_pull3**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_push3 dominates: 27% faster than the next best (iterfuse_pull3)

iterfuse_push3 (2.90 us) leads iterfuse_pull3 (3.68 us) by 27%, a clear separation rather than a photo finish. CV 9.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push3 beats baseline by 23% (significant)

iterfuse_push3 is -835 ns (23%) faster than baseline iterfuse_pull3, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iterfuse_push3 is fastest but the noisiest (CV 9.3%)

iterfuse_push3 wins on median (2.90 us) yet has the highest variance (CV 9.3%), while iterfuse_mat3 is the steadiest (CV 6.4%, 5.66 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: iterfuse_push3** at 2897.5 ns median (-21.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.95x (fastest 2897.5 ns, slowest 5661.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat3 | 7848ns | 8156ns | 7057ns | 7818ns | 8289ns | +28.12% |
| iterfuse_pull3 | 6126ns | 6036ns | 5519ns | 5936ns | 6712ns | base |
| iterfuse_push3 | 5315ns | 5257ns | 4805ns | 5122ns | 5858ns | -13.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat3 | 5432ns | 4875ns | 5712ns | +45.56% | 0.047 |
| iterfuse_pull3 | 3731ns | 3354ns | 4069ns | base | 0.069 |
| iterfuse_push3 | 2946ns | 2638ns | 3272ns | -21.05% | 0.087 |

## Performance model

- Peak throughput: **0.097 Gops/s** (iterfuse_push3; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat3 | 0.045 | 46.6% |
| iterfuse_pull3 | 0.070 | 71.7% |
| iterfuse_push3 | 0.088 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat3 | 7848ns | 7848ns | +28.12% |
| iterfuse_pull3 | 6126ns | 6126ns | base |
| iterfuse_push3 | 5315ns | 5315ns | -13.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull3 | 3677ns | base | --- | [3449, 4069] | --- | --- | --- | --- |
| iterfuse_mat3 | 5661ns | +1633.1ns (+44.4%) | [+1359, +2109]ns | [4921, 5712] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push3 | 2898ns | -834.8ns (-22.7%) | [-954, -568]ns | [2668, 3272] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull3 | iterfuse_mat3 | iterfuse_push3 |
|---|---|---|---|
| 1 | 3354ns | +67.6% | -18.6% |
| 2 | 4069ns | +40.1% | -19.3% |
| 3 | 4068ns | +40.2% | -24.7% |
| 4 | 3772ns | +51.7% | -13.6% |
| 5 | 3583ns | +38.7% | -24.7% |
| 6 | 3543ns | +37.6% | -25.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat3 | 0.412 | moderate+ |
| iterfuse_pull3 | 0.051 | ok |
| iterfuse_push3 | 0.008 | ok |

**Consistency summary:**

- **iterfuse_mat3**: won 0/6, lost 6/6
- **iterfuse_push3**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat3 | 175.4ns | 5431.5ns | 3.2% |  |
| iterfuse_pull3 | 3.1ns | 3731.5ns | 0.1% |  |
| iterfuse_push3 | 2.8ns | 2945.8ns | 0.1% |  |

## Distribution (algo ns)

```
iterfuse_mat3 (n=6, range 4874.6-5712.1 ns)
   4874.6 |####################
   4916.5 |
   4958.4 |####################
   5000.2 |
   5042.1 |
   5084.0 |
   5125.9 |
   5167.7 |
   5209.6 |
   5251.5 |
   5293.4 |
   5335.2 |
   5377.1 |
   5419.0 |
   5460.9 |
   5502.7 |
   5544.6 |
   5586.5 |####################
   5628.4 |
   5670.2 |########################################
  (0 below, 1 above range)

iterfuse_pull3 (n=6, range 3354.2-4068.6 ns)
   3354.2 |########################################
   3389.9 |
   3425.6 |
   3461.4 |
   3497.1 |
   3532.8 |########################################
   3568.5 |########################################
   3604.2 |
   3639.9 |
   3675.7 |
   3711.4 |
   3747.1 |########################################
   3782.8 |
   3818.5 |
   3854.2 |
   3890.0 |
   3925.7 |
   3961.4 |
   3997.1 |
   4032.8 |########################################
  (0 below, 1 above range)

iterfuse_push3 (n=6, range 2637.5-3272.5 ns)
   2637.5 |########################################
   2669.2 |########################################
   2701.0 |########################################
   2732.8 |
   2764.5 |
   2796.2 |
   2828.0 |
   2859.8 |
   2891.5 |
   2923.2 |
   2955.0 |
   2986.8 |
   3018.5 |
   3050.2 |########################################
   3082.0 |
   3113.8 |
   3145.5 |
   3177.2 |
   3209.0 |
   3240.8 |########################################
  (0 below, 1 above range)

```
