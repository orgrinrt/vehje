# Interpreter model: recursive tree-walk vs CFG-of-blocks linear scan

2 variants, 6 samples per variant.
Baseline: **hx_cfg__cfgblock**

## Highlights

Baseline for all deltas below: **hx_cfg__cfgblock**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_cfg__cfgblock dominates: 796% faster than the next best (hx_cfg__treewalk)

hx_cfg__cfgblock (425 ns) leads hx_cfg__treewalk (3.81 us) by 796%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cfg__cfgblock is fastest but the noisiest (CV 6.9%)

hx_cfg__cfgblock wins on median (425 ns) yet has the highest variance (CV 6.9%), while hx_cfg__treewalk is the steadiest (CV 3.6%, 3.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_cfg__cfgblock)

The baseline hx_cfg__cfgblock is the fastest (425 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.0x the fastest

Fastest hx_cfg__cfgblock (425 ns) to slowest hx_cfg__treewalk (3.81 us): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_cfg__cfgblock) is the fastest** at 425.2 ns median
- 1 variant significantly slower than baseline
- Spread: 8.96x (fastest 425.2 ns, slowest 3807.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 2891ns | 2921ns | 2482ns | 2887ns | 3102ns | base |
| hx_cfg__treewalk | 6386ns | 6354ns | 6129ns | 6296ns | 6649ns | +120.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cfg__cfgblock | 418ns | 361ns | 444ns | base | 0.613 |
| hx_cfg__treewalk | 3850ns | 3702ns | 4028ns | +821.99% | 0.066 |

## Performance model

- Peak throughput: **0.709 Gops/s** (hx_cfg__cfgblock; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cfg__cfgblock | 0.602 | 84.9% |
| hx_cfg__treewalk | 0.067 | 9.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cfg__cfgblock | 2891ns | 2891ns | base |
| hx_cfg__treewalk | 6386ns | 6386ns | +120.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 425ns | base | --- | [384, 444] | --- | --- | --- | --- |
| hx_cfg__treewalk | 3808ns | +3424.2ns (+805.3%) | [+3285, +3588]ns | [3714, 4028] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cfg__cfgblock | hx_cfg__treewalk |
|---|---|---|
| 1 | 361ns | +971.4% |
| 2 | 438ns | +826.9% |
| 3 | 445ns | +732.6% |
| 4 | 406ns | +822.2% |
| 5 | 413ns | +802.3% |
| 6 | 443ns | +803.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cfg__cfgblock | -0.188 | ok |
| hx_cfg__treewalk | -0.147 | ok |

**Consistency summary:**

- **hx_cfg__treewalk**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cfg__cfgblock | 4.2ns | 417.5ns | 1.0% |  |
| hx_cfg__treewalk | 3.5ns | 3849.8ns | 0.1% |  |

## Distribution (algo ns)

```
hx_cfg__cfgblock (n=6, range 361.2-443.8 ns)
    361.2 |########################################
    365.3 |
    369.5 |
    373.6 |
    377.7 |
    381.8 |
    386.0 |
    390.1 |
    394.2 |
    398.3 |
    402.5 |########################################
    406.6 |
    410.7 |########################################
    414.9 |
    419.0 |
    423.1 |
    427.2 |
    431.4 |
    435.5 |########################################
    439.6 |########################################
  (0 below, 1 above range)

hx_cfg__treewalk (n=6, range 3701.7-4027.7 ns)
   3701.7 |########################################
   3718.0 |########################################
   3734.3 |########################################
   3750.6 |
   3766.9 |
   3783.2 |
   3799.5 |
   3815.8 |
   3832.1 |
   3848.4 |
   3864.7 |########################################
   3881.0 |
   3897.3 |
   3913.6 |
   3929.9 |
   3946.2 |
   3962.5 |
   3978.8 |
   3995.1 |########################################
   4011.4 |
  (0 below, 1 above range)

```
