# Record update (20% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s20**

## Highlights

Baseline for all deltas below: **rec_reuse_s20**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 149% faster than the next best (rec_reuse_s20)

rec_mut (16.48 us) leads rec_reuse_s20 (41.02 us) by 149%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 58% (significant)

rec_mut is -23.85 us (58%) faster than baseline rec_reuse_s20, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 7.1x slower than the field

rec_copy (116.52 us) is 7.1x the fastest (16.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.1x the fastest

Fastest rec_mut (16.48 us) to slowest rec_copy (116.52 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 16476.1 ns median (-59.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.07x (fastest 16476.1 ns, slowest 116521.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 118721ns | 118828ns | 114638ns | 117652ns | 122364ns | +168.38% |
| rec_mut | 19192ns | 18752ns | 18241ns | 18688ns | 20424ns | -56.61% |
| rec_reuse_s20 | 44236ns | 43342ns | 41495ns | 42901ns | 47607ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 116410ns | 112409ns | 119986ns | +177.71% | 0.035 |
| rec_mut | 16854ns | 16019ns | 17931ns | -59.79% | 0.243 |
| rec_reuse_s20 | 41918ns | 39319ns | 45169ns | base | 0.098 |

## Performance model

- Peak throughput: **0.256 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 13.7% |
| rec_mut | 0.249 | 97.2% |
| rec_reuse_s20 | 0.100 | 39.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 118721ns | 118721ns | +168.38% |
| rec_mut | 19192ns | 19192ns | -56.61% |
| rec_reuse_s20 | 44236ns | 44236ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s20 | 41022ns | base | --- | [39563, 45169] | --- | --- | --- | --- |
| rec_copy | 116522ns | +73611.9ns (+179.4%) | [+73011, +76854]ns | [112723, 119986] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 16476ns | -23849.7ns (-58.1%) | [-28098, -23244]ns | [16154, 17931] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s20 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 39807ns | +184.0% | -59.8% |
| 2 | 39319ns | +185.9% | -57.9% |
| 3 | 44565ns | +163.7% | -60.2% |
| 4 | 41843ns | +187.3% | -56.7% |
| 5 | 40201ns | +187.4% | -59.5% |
| 6 | 45772ns | +161.7% | -64.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.129 | ok |
| rec_mut | 0.174 | ok |
| rec_reuse_s20 | -0.224 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21636.4ns | 116410.0ns | 18.6% | HIGH |
| rec_mut | 22177.0ns | 16853.8ns | 131.6% | HIGH |
| rec_reuse_s20 | 23224.9ns | 41917.8ns | 55.4% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 112408.7-119985.6 ns)
  112408.7 |########################################
  112787.5 |########################################
  113166.4 |
  113545.2 |
  113924.1 |
  114302.9 |
  114681.8 |
  115060.6 |
  115439.5 |########################################
  115818.3 |
  116197.1 |
  116576.0 |
  116954.8 |
  117333.7 |########################################
  117712.5 |
  118091.4 |
  118470.2 |
  118849.1 |
  119227.9 |
  119606.8 |########################################
  (0 below, 1 above range)

rec_mut (n=6, range 16019.2-17931.2 ns)
  16019.2 |########################################
  16114.8 |
  16210.4 |########################################
  16306.0 |
  16401.6 |########################################
  16497.2 |########################################
  16592.8 |
  16688.4 |
  16784.0 |
  16879.6 |
  16975.2 |
  17070.8 |
  17166.4 |
  17262.0 |
  17357.6 |
  17453.2 |
  17548.8 |
  17644.4 |########################################
  17740.0 |
  17835.6 |
  (0 below, 1 above range)

rec_reuse_s20 (n=6, range 39318.8-45168.6 ns)
  39318.8 |########################################
  39611.3 |########################################
  39903.8 |
  40196.3 |########################################
  40488.8 |
  40781.2 |
  41073.7 |
  41366.2 |
  41658.7 |########################################
  41951.2 |
  42243.7 |
  42536.2 |
  42828.7 |
  43121.1 |
  43413.6 |
  43706.1 |
  43998.6 |
  44291.1 |########################################
  44583.6 |
  44876.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=18.6% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=132.1% of algo (FFI overhead may distort results)
- **rec_reuse_s20**: bridge=56.5% of algo (FFI overhead may distort results)
