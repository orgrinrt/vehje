# Interpreter model: recursive tree-walk vs CFG-of-blocks linear scan

2 variants, 6 samples per variant.
Baseline: **hx_cfg__cfgblock**

## Highlights

Baseline for all deltas below: **hx_cfg__cfgblock**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_cfg__cfgblock dominates: 690% faster than the next best (hx_cfg__treewalk)

hx_cfg__cfgblock (124 ns) leads hx_cfg__treewalk (981 ns) by 690%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_cfg__cfgblock)

The baseline hx_cfg__cfgblock is the fastest (124 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.9x the fastest

Fastest hx_cfg__cfgblock (124 ns) to slowest hx_cfg__treewalk (981 ns): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_cfg__cfgblock) is the fastest** at 124.2 ns median
- 1 variant significantly slower than baseline
- Spread: 7.90x (fastest 124.2 ns, slowest 980.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 2655ns | 2703ns | 2538ns | 2650ns | 2722ns | base |
| hx_cfg__treewalk | 3464ns | 3494ns | 3156ns | 3445ns | 3646ns | +30.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cfg__cfgblock | 124ns | 116ns | 130ns | base | 0.515 |
| hx_cfg__treewalk | 969ns | 870ns | 1021ns | +680.54% | 0.066 |

## Performance model

- Peak throughput: **0.553 Gops/s** (hx_cfg__cfgblock; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cfg__cfgblock | 0.515 | 93.2% |
| hx_cfg__treewalk | 0.065 | 11.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cfg__cfgblock | 2655ns | 2655ns | base |
| hx_cfg__treewalk | 3464ns | 3464ns | +30.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 124ns | base | --- | [118, 130] | --- | --- | --- | --- |
| hx_cfg__treewalk | 981ns | +858.1ns (+690.9%) | [+783, +894]ns | [906, 1021] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cfg__cfgblock | hx_cfg__treewalk |
|---|---|---|
| 1 | 116ns | +651.6% |
| 2 | 130ns | +622.5% |
| 3 | 130ns | +692.5% |
| 4 | 124ns | +716.6% |
| 5 | 121ns | +697.1% |
| 6 | 124ns | +703.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cfg__cfgblock | -0.122 | ok |
| hx_cfg__treewalk | 0.194 | ok |

**Consistency summary:**

- **hx_cfg__treewalk**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cfg__cfgblock | 3.2ns | 124.2ns | 2.6% |  |
| hx_cfg__treewalk | 2.8ns | 969.2ns | 0.3% |  |

## Distribution (algo ns)

```
hx_cfg__cfgblock (n=6, range 115.8-130.0 ns)
    115.8 |####################
    116.5 |
    117.2 |
    117.9 |
    118.6 |
    119.3 |
    120.1 |
    120.8 |####################
    121.5 |
    122.2 |
    122.9 |
    123.6 |########################################
    124.3 |
    125.0 |
    125.7 |
    126.5 |
    127.2 |
    127.9 |
    128.6 |
    129.3 |####################
  (0 below, 1 above range)

hx_cfg__treewalk (n=6, range 870.4-1020.6 ns)
    870.4 |########################################
    877.9 |
    885.4 |
    892.9 |
    900.4 |
    908.0 |
    915.5 |
    923.0 |
    930.5 |
    938.0 |########################################
    945.5 |
    953.0 |
    960.5 |########################################
    968.1 |
    975.6 |
    983.1 |
    990.6 |
    998.1 |########################################
   1005.6 |
   1013.1 |########################################
  (0 below, 1 above range)

```
