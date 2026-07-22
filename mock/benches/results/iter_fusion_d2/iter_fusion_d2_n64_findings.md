# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_push2 is fastest but the noisiest (CV 27.8%)

iterfuse_push2 wins on median (910 ns) yet has the highest variance (CV 27.8%), while iterfuse_mat2 is the steadiest (CV 14.0%, 1.30 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_pull2 shows warm-up / thermal drift (autocorr +0.56)

iterfuse_pull2's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### iterfuse_push2 is inconsistent: worst-20% is 1.7x its best-20%

iterfuse_push2's best 20% of batches run at 805 ns but its worst 20% at 1.34 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iterfuse_push2** at 910.5 ns median (-2.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.43x (fastest 910.5 ns, slowest 1298.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 4266ns | 4188ns | 3802ns | 4071ns | 4791ns | +7.74% |
| iterfuse_pull2 | 3959ns | 3853ns | 3407ns | 3708ns | 4613ns | base |
| iterfuse_push2 | 4159ns | 3821ns | 3247ns | 3681ns | 5332ns | +5.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 1351ns | 1174ns | 1555ns | +38.51% | 0.047 |
| iterfuse_pull2 | 976ns | 842ns | 1145ns | base | 0.066 |
| iterfuse_push2 | 1024ns | 805ns | 1342ns | +5.00% | 0.062 |

## Performance model

- Peak throughput: **0.080 Gops/s** (iterfuse_push2; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.049 | 62.0% |
| iterfuse_pull2 | 0.068 | 86.0% |
| iterfuse_push2 | 0.070 | 88.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 4266ns | 4266ns | +7.74% |
| iterfuse_pull2 | 3959ns | 3959ns | base |
| iterfuse_push2 | 4159ns | 4159ns | +5.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 936ns | base | --- | [846, 1145] | --- | --- | --- | --- |
| iterfuse_mat2 | 1299ns | +370.0ns (+39.5%) | [+293, +465]ns | [1200, 1555] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push2 | 910ns | no significant difference | [-51, +218]ns | [821, 1342] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 1106ns | +22.8% | +38.0% |
| 2 | 1184ns | +45.6% | -2.2% |
| 3 | 1022ns | +35.6% | -5.7% |
| 4 | 842ns | +39.5% | +1.8% |
| 5 | 850ns | +44.2% | -1.7% |
| 6 | 850ns | +45.8% | -5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.230 | moderate+ |
| iterfuse_pull2 | 0.564 | HIGH+ (drift/warm-up) |
| iterfuse_push2 | 0.369 | moderate+ |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 102.2ns | 1351.3ns | 7.6% | HIGH |
| iterfuse_pull2 | 5.2ns | 975.6ns | 0.5% |  |
| iterfuse_push2 | 5.5ns | 1024.4ns | 0.5% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 1174.2-1555.0 ns)
   1174.2 |########################################
   1193.2 |
   1212.3 |########################################
   1231.3 |########################################
   1250.4 |
   1269.4 |
   1288.4 |
   1307.5 |
   1326.5 |
   1345.5 |########################################
   1364.6 |
   1383.6 |########################################
   1402.7 |
   1421.7 |
   1440.7 |
   1459.8 |
   1478.8 |
   1497.8 |
   1516.9 |
   1535.9 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 841.7-1145.0 ns)
    841.7 |########################################
    856.9 |
    872.0 |
    887.2 |
    902.4 |
    917.5 |
    932.7 |
    947.9 |
    963.0 |
    978.2 |
    993.4 |
   1008.5 |#############
   1023.7 |
   1038.8 |
   1054.0 |
   1069.2 |
   1084.3 |
   1099.5 |#############
   1114.7 |
   1129.8 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 805.0-1342.1 ns)
    805.0 |####################
    831.9 |########################################
    858.7 |
    885.6 |
    912.4 |
    939.3 |####################
    966.1 |
    993.0 |
   1019.8 |
   1046.7 |
   1073.5 |
   1100.4 |
   1127.2 |
   1154.1 |####################
   1180.9 |
   1207.8 |
   1234.6 |
   1261.5 |
   1288.3 |
   1315.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **iterfuse_pull2**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **iterfuse_push2**: CV=24.7% (high variance, measurements may be unstable)
