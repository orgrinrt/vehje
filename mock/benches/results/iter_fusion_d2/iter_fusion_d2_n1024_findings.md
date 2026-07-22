# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 44% faster than the next best (iterfuse_mat2)

iterfuse_pull2 (10.69 us) leads iterfuse_mat2 (15.39 us) by 44%, a clear separation rather than a photo finish. CV 9.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push2 is an outlier: 2.4x slower than the field

iterfuse_push2 (25.61 us) is 2.4x the fastest (10.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iterfuse_pull2 is fastest but the noisiest (CV 9.7%)

iterfuse_pull2 wins on median (10.69 us) yet has the highest variance (CV 9.7%), while iterfuse_mat2 is the steadiest (CV 7.8%, 15.39 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_push2 shows warm-up / thermal drift (autocorr +0.53)

iterfuse_push2's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (10.69 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 10690.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.40x (fastest 10690.9 ns, slowest 25610.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 18238ns | 17602ns | 17052ns | 17431ns | 20040ns | +34.27% |
| iterfuse_pull2 | 13583ns | 12849ns | 12608ns | 12819ns | 15218ns | base |
| iterfuse_push2 | 27538ns | 27914ns | 23672ns | 27158ns | 30043ns | +102.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 15934ns | 14859ns | 17529ns | +41.19% | 0.064 |
| iterfuse_pull2 | 11286ns | 10451ns | 12639ns | base | 0.091 |
| iterfuse_push2 | 25238ns | 21510ns | 27650ns | +123.63% | 0.041 |

## Performance model

- Peak throughput: **0.098 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.067 | 67.9% |
| iterfuse_pull2 | 0.096 | 97.8% |
| iterfuse_push2 | 0.040 | 40.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 18238ns | 18238ns | +34.27% |
| iterfuse_pull2 | 13583ns | 13583ns | base |
| iterfuse_push2 | 27538ns | 27538ns | +102.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 10691ns | base | --- | [10527, 12639] | --- | --- | --- | --- |
| iterfuse_mat2 | 15391ns | +4386.9ns (+41.0%) | [+3974, +5585]ns | [14883, 17529] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 25611ns | +13564.6ns (+126.9%) | [+11926, +16366]ns | [22452, 27650] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 10772ns | +40.5% | +123.6% |
| 2 | 10602ns | +40.6% | +102.9% |
| 3 | 10451ns | +42.2% | +123.9% |
| 4 | 10610ns | +47.5% | +157.7% |
| 5 | 11958ns | +51.3% | +133.8% |
| 6 | 13320ns | +27.4% | +103.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.443 | moderate+ |
| iterfuse_pull2 | 0.370 | moderate+ |
| iterfuse_push2 | 0.533 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 275.7ns | 15934.4ns | 1.7% |  |
| iterfuse_pull2 | 3.8ns | 11285.6ns | 0.0% |  |
| iterfuse_push2 | 3.6ns | 25237.8ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 14859.2-17528.9 ns)
  14859.2 |########################################
  14992.7 |
  15126.2 |####################
  15259.7 |
  15393.1 |
  15526.6 |####################
  15660.1 |
  15793.6 |
  15927.1 |
  16060.6 |
  16194.1 |
  16327.6 |
  16461.0 |
  16594.5 |
  16728.0 |
  16861.5 |####################
  16995.0 |
  17128.5 |
  17262.0 |
  17395.5 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 10450.8-12639.1 ns)
  10450.8 |####################
  10560.2 |########################################
  10669.6 |####################
  10779.1 |
  10888.5 |
  10997.9 |
  11107.3 |
  11216.7 |
  11326.1 |
  11435.6 |
  11545.0 |
  11654.4 |
  11763.8 |
  11873.2 |####################
  11982.6 |
  12092.1 |
  12201.5 |
  12310.9 |
  12420.3 |
  12529.7 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 21510.0-27650.0 ns)
  21510.0 |########################################
  21817.0 |
  22124.0 |
  22431.0 |
  22738.0 |
  23045.0 |
  23352.0 |########################################
  23659.0 |
  23966.0 |########################################
  24273.0 |
  24580.0 |
  24887.0 |
  25194.0 |
  25501.0 |
  25808.0 |
  26115.0 |
  26422.0 |
  26729.0 |
  27036.0 |########################################
  27343.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **iterfuse_push2**: autocorrelation=0.53 (measurement drift or warm-up artifact)
