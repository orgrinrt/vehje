# Iterator fusion (depth 3): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull3**

## Highlights

Baseline for all deltas below: **iterfuse_pull3**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_push3's edge over baseline is significant but tiny (1 ns, 0.13%)

iterfuse_push3 differs from baseline iterfuse_pull3 by 1 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: iterfuse_push3** at 782.9 ns median (-4.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.80x (fastest 782.9 ns, slowest 1405.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat3 | 4054ns | 4100ns | 3368ns | 3873ns | 4668ns | +24.41% |
| iterfuse_pull3 | 3258ns | 3322ns | 2859ns | 3187ns | 3565ns | base |
| iterfuse_push3 | 3197ns | 3100ns | 2889ns | 3045ns | 3578ns | -1.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat3 | 1405ns | 1212ns | 1595ns | +72.68% | 0.046 |
| iterfuse_pull3 | 813ns | 702ns | 896ns | base | 0.079 |
| iterfuse_push3 | 795ns | 703ns | 877ns | -2.21% | 0.080 |

## Performance model

- Peak throughput: **0.091 Gops/s** (iterfuse_pull3; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat3 | 0.046 | 50.0% |
| iterfuse_pull3 | 0.078 | 85.8% |
| iterfuse_push3 | 0.082 | 89.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat3 | 4054ns | 4054ns | +24.41% |
| iterfuse_pull3 | 3258ns | 3258ns | base |
| iterfuse_push3 | 3197ns | 3197ns | -1.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull3 | 819ns | base | --- | [725, 896] | --- | --- | --- | --- |
| iterfuse_mat3 | 1406ns | +567.1ns (+69.3%) | [+488, +719]ns | [1213, 1595] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push3 | 783ns | no significant difference | [-59, +4]ns | [726, 877] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull3 | iterfuse_mat3 | iterfuse_push3 |
|---|---|---|---|
| 1 | 857ns | +81.8% | -0.4% |
| 2 | 895ns | +82.2% | -12.8% |
| 3 | 702ns | +72.8% | +0.1% |
| 4 | 748ns | +62.0% | +0.2% |
| 5 | 781ns | +71.2% | +0.5% |
| 6 | 896ns | +64.5% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat3 | 0.230 | moderate+ |
| iterfuse_pull3 | 0.033 | ok |
| iterfuse_push3 | 0.162 | ok |

**Consistency summary:**

- **iterfuse_mat3**: won 0/6, lost 6/6
- **iterfuse_push3**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat3 | 128.9ns | 1404.5ns | 9.2% | HIGH |
| iterfuse_pull3 | 3.6ns | 813.4ns | 0.4% |  |
| iterfuse_push3 | 3.3ns | 795.4ns | 0.4% |  |

## Distribution (algo ns)

```
iterfuse_mat3 (n=6, range 1212.5-1594.6 ns)
   1212.5 |########################################
   1231.6 |
   1250.7 |
   1269.8 |
   1288.9 |
   1308.0 |
   1327.1 |####################
   1346.2 |
   1365.3 |
   1384.4 |
   1403.5 |
   1422.7 |
   1441.8 |
   1460.9 |####################
   1480.0 |
   1499.1 |
   1518.2 |
   1537.3 |
   1556.4 |####################
   1575.5 |
  (0 below, 1 above range)

iterfuse_pull3 (n=6, range 702.5-895.8 ns)
    702.5 |########################################
    712.2 |
    721.8 |
    731.5 |
    741.2 |########################################
    750.8 |
    760.5 |
    770.2 |
    779.8 |########################################
    789.5 |
    799.1 |
    808.8 |
    818.5 |
    828.1 |
    837.8 |
    847.5 |########################################
    857.1 |
    866.8 |
    876.5 |
    886.1 |########################################
  (0 below, 1 above range)

iterfuse_push3 (n=6, range 703.3-876.8 ns)
    703.3 |########################################
    712.0 |
    720.7 |
    729.3 |
    738.0 |
    746.7 |########################################
    755.4 |
    764.0 |
    772.7 |########################################
    781.4 |########################################
    790.1 |
    798.8 |
    807.4 |
    816.1 |
    824.8 |
    833.5 |
    842.1 |
    850.8 |########################################
    859.5 |
    868.2 |
  (0 below, 1 above range)

```
