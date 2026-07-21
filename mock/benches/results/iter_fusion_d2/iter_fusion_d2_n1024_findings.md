# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 shows warm-up / thermal drift (autocorr +0.56)

iterfuse_pull2's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (11.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader iterfuse_pull2 vs stability leader iterfuse_push2 (+5% speed for 1.1x steadier)

iterfuse_pull2 is fastest (11.51 us, CV 6.7%); iterfuse_push2 gives up 5.2% median for 1.1x lower variance (CV 5.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 11515.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.39x (fastest 11515.0 ns, slowest 15964.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 18811ns | 18268ns | 17445ns | 18017ns | 20686ns | +34.36% |
| iterfuse_pull2 | 14001ns | 13823ns | 13044ns | 13594ns | 15090ns | base |
| iterfuse_push2 | 14679ns | 14317ns | 13761ns | 14231ns | 15810ns | +4.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 16417ns | 15262ns | 17988ns | +40.75% | 0.062 |
| iterfuse_pull2 | 11663ns | 10875ns | 12568ns | base | 0.088 |
| iterfuse_push2 | 12394ns | 11607ns | 13336ns | +6.27% | 0.083 |

## Performance model

- Peak throughput: **0.094 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.064 | 68.1% |
| iterfuse_pull2 | 0.089 | 94.4% |
| iterfuse_push2 | 0.085 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 18811ns | 18811ns | +34.36% |
| iterfuse_pull2 | 14001ns | 14001ns | base |
| iterfuse_push2 | 14679ns | 14679ns | +4.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 11515ns | base | --- | [10908, 12568] | --- | --- | --- | --- |
| iterfuse_mat2 | 15964ns | +4540.0ns (+39.4%) | [+4189, +5531]ns | [15298, 17988] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push2 | 12109ns | +879.3ns (+7.6%) | [+151, +1162]ns | [11738, 13336] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 12299ns | +33.1% | -3.5% |
| 2 | 12837ns | +41.4% | +6.5% |
| 3 | 12076ns | +47.6% | +7.7% |
| 4 | 10954ns | +39.3% | +10.4% |
| 5 | 10940ns | +42.2% | +10.9% |
| 6 | 10875ns | +41.0% | +6.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.322 | moderate+ |
| iterfuse_pull2 | 0.561 | HIGH+ (drift/warm-up) |
| iterfuse_push2 | 0.071 | ok |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 290.6ns | 16416.8ns | 1.8% |  |
| iterfuse_pull2 | 3.5ns | 11663.5ns | 0.0% |  |
| iterfuse_push2 | 3.0ns | 12394.4ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 15261.7-17987.7 ns)
  15261.7 |########################################
  15398.0 |
  15534.3 |####################
  15670.6 |
  15806.9 |
  15943.2 |
  16079.5 |
  16215.8 |
  16352.1 |####################
  16488.4 |
  16624.7 |
  16761.0 |
  16897.3 |
  17033.6 |
  17169.9 |
  17306.2 |
  17442.5 |
  17578.8 |
  17715.1 |####################
  17851.4 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 10875.4-12568.0 ns)
  10875.4 |########################################
  10960.0 |
  11044.7 |
  11129.3 |
  11213.9 |
  11298.5 |
  11383.2 |
  11467.8 |
  11552.4 |
  11637.0 |
  11721.7 |
  11806.3 |
  11890.9 |
  11975.6 |
  12060.2 |#############
  12144.8 |
  12229.4 |#############
  12314.1 |
  12398.7 |
  12483.3 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 11607.1-13336.0 ns)
  11607.1 |########################################
  11693.5 |
  11780.0 |
  11866.4 |########################################
  11952.9 |
  12039.3 |########################################
  12125.8 |########################################
  12212.2 |
  12298.7 |
  12385.1 |
  12471.5 |
  12558.0 |
  12644.4 |
  12730.9 |
  12817.3 |
  12903.8 |
  12990.2 |########################################
  13076.7 |
  13163.1 |
  13249.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **iterfuse_pull2**: autocorrelation=0.56 (measurement drift or warm-up artifact)
