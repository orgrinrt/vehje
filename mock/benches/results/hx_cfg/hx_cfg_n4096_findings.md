# Interpreter model: recursive tree-walk vs CFG-of-blocks linear scan

2 variants, 6 samples per variant.
Baseline: **hx_cfg__cfgblock**

## Highlights

Baseline for all deltas below: **hx_cfg__cfgblock**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_cfg__cfgblock dominates: 895% faster than the next best (hx_cfg__treewalk)

hx_cfg__cfgblock (5.78 us) leads hx_cfg__treewalk (57.55 us) by 895%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_cfg__cfgblock)

The baseline hx_cfg__cfgblock is the fastest (5.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 10.0x the fastest

Fastest hx_cfg__cfgblock (5.78 us) to slowest hx_cfg__treewalk (57.55 us): 10.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_cfg__cfgblock) is the fastest** at 5781.9 ns median
- 1 variant significantly slower than baseline
- Spread: 9.95x (fastest 5781.9 ns, slowest 57552.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 8171ns | 8020ns | 7747ns | 7993ns | 8649ns | base |
| hx_cfg__treewalk | 59191ns | 60009ns | 53457ns | 59045ns | 62278ns | +624.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cfg__cfgblock | 5891ns | 5578ns | 6231ns | base | 0.695 |
| hx_cfg__treewalk | 56785ns | 51322ns | 59758ns | +863.92% | 0.072 |

## Performance model

- Peak throughput: **0.734 Gops/s** (hx_cfg__cfgblock; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cfg__cfgblock | 0.708 | 96.5% |
| hx_cfg__treewalk | 0.071 | 9.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cfg__cfgblock | 8171ns | 8171ns | base |
| hx_cfg__treewalk | 59191ns | 59191ns | +624.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 5782ns | base | --- | [5660, 6231] | --- | --- | --- | --- |
| hx_cfg__treewalk | 57553ns | +51544.3ns (+891.5%) | [+47383, +53754]ns | [53043, 59758] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cfg__cfgblock | hx_cfg__treewalk |
|---|---|---|
| 1 | 5743ns | +853.5% |
| 2 | 5578ns | +820.2% |
| 3 | 5779ns | +925.3% |
| 4 | 6229ns | +867.4% |
| 5 | 5785ns | +879.4% |
| 6 | 6232ns | +837.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cfg__cfgblock | -0.077 | ok |
| hx_cfg__treewalk | 0.100 | ok |

**Consistency summary:**

- **hx_cfg__treewalk**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cfg__cfgblock | 1.8ns | 5891.1ns | 0.0% |  |
| hx_cfg__treewalk | 2.8ns | 56784.7ns | 0.0% |  |

## Distribution (algo ns)

```
hx_cfg__cfgblock (n=6, range 5577.5-6230.9 ns)
   5577.5 |####################
   5610.2 |
   5642.8 |
   5675.5 |
   5708.2 |
   5740.8 |####################
   5773.5 |########################################
   5806.2 |
   5838.8 |
   5871.5 |
   5904.2 |
   5936.8 |
   5969.5 |
   6002.2 |
   6034.8 |
   6067.5 |
   6100.2 |
   6132.8 |
   6165.5 |
   6198.2 |####################
  (0 below, 1 above range)

hx_cfg__treewalk (n=6, range 51321.7-59758.1 ns)
  51321.7 |########################################
  51743.5 |
  52165.3 |
  52587.2 |
  53009.0 |
  53430.8 |
  53852.6 |
  54274.5 |
  54696.3 |########################################
  55118.1 |
  55539.9 |
  55961.7 |
  56383.6 |########################################
  56805.4 |
  57227.2 |
  57649.0 |
  58070.9 |########################################
  58492.7 |
  58914.5 |########################################
  59336.3 |
  (0 below, 1 above range)

```
