# Project field access, monomorphic site: direct offset vs inline cache vs hash vs linear

4 variants, 6 samples per variant.
Baseline: **project_mono_direct**

## Highlights

Baseline for all deltas below: **project_mono_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_mono_direct dominates: 40% faster than the next best (project_mono_ic)

project_mono_direct (576 ns) leads project_mono_ic (807 ns) by 40%, a clear separation rather than a photo finish. CV 7.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_mono_linear is an outlier: 4.6x slower than the field

project_mono_linear (2.67 us) is 4.6x the fastest (576 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (project_mono_direct)

The baseline project_mono_direct is the fastest (576 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {project_mono_direct, project_mono_ic} vs {project_mono_hash, project_mono_linear} (170% apart)

The field splits into a fast tier {project_mono_direct, project_mono_ic} and a slow tier {project_mono_hash, project_mono_linear} with a 170% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest project_mono_direct (576 ns) to slowest project_mono_linear (2.67 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (project_mono_direct) is the fastest** at 575.8 ns median
- 3 variants significantly slower than baseline
- Spread: 4.64x (fastest 575.8 ns, slowest 2674.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_mono_direct | 2907ns | 2876ns | 2698ns | 2824ns | 3137ns | base |
| project_mono_hash | 4673ns | 4541ns | 4202ns | 4437ns | 5263ns | +60.74% |
| project_mono_ic | 3166ns | 3143ns | 2871ns | 3064ns | 3467ns | +8.91% |
| project_mono_linear | 5083ns | 5113ns | 4528ns | 5012ns | 5467ns | +74.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_mono_direct | 596ns | 555ns | 651ns | base | 1.719 |
| project_mono_hash | 2233ns | 2018ns | 2501ns | +274.90% | 0.459 |
| project_mono_ic | 815ns | 735ns | 896ns | +36.90% | 1.256 |
| project_mono_linear | 2641ns | 2352ns | 2824ns | +343.47% | 0.388 |

## Performance model

- Peak throughput: **1.845 Gops/s** (project_mono_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_mono_direct | 1.778 | 96.4% |
| project_mono_hash | 0.470 | 25.5% |
| project_mono_ic | 1.268 | 68.7% |
| project_mono_linear | 0.383 | 20.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_mono_direct | 2907ns | 2907ns | base |
| project_mono_hash | 4673ns | 4673ns | +60.74% |
| project_mono_ic | 3166ns | 3166ns | +8.91% |
| project_mono_linear | 5083ns | 5083ns | +74.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_mono_direct | 576ns | base | --- | [560, 651] | --- | --- | --- | --- |
| project_mono_hash | 2178ns | +1616.2ns (+280.7%) | [+1445, +1850]ns | [2019, 2501] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_ic | 807ns | +210.4ns (+36.5%) | [+167, +282]ns | [743, 896] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_linear | 2675ns | +2049.4ns (+355.9%) | [+1858, +2229]ns | [2425, 2824] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_mono_direct | project_mono_hash | project_mono_ic | project_mono_linear |
|---|---|---|---|---|
| 1 | 555ns | +304.7% | +61.9% | +412.9% |
| 2 | 569ns | +271.0% | +29.2% | +339.2% |
| 3 | 628ns | +282.5% | +32.2% | +345.4% |
| 4 | 583ns | +246.3% | +28.9% | +338.3% |
| 5 | 565ns | +257.5% | +38.9% | +316.2% |
| 6 | 674ns | +286.0% | +32.4% | +315.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_mono_direct | -0.205 | moderate- |
| project_mono_hash | -0.336 | moderate- |
| project_mono_ic | -0.371 | moderate- |
| project_mono_linear | -0.423 | moderate- |

**Consistency summary:**

- **project_mono_hash**: won 0/6, lost 6/6
- **project_mono_ic**: won 0/6, lost 6/6
- **project_mono_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_mono_direct | 3.0ns | 595.6ns | 0.5% |  |
| project_mono_hash | 2.6ns | 2232.8ns | 0.1% |  |
| project_mono_ic | 3.6ns | 815.3ns | 0.4% |  |
| project_mono_linear | 3.8ns | 2641.2ns | 0.1% |  |

## Distribution (algo ns)

```
project_mono_direct (n=6, range 555.0-650.9 ns)
    555.0 |####################
    559.8 |
    564.6 |########################################
    569.4 |
    574.2 |
    579.0 |####################
    583.8 |
    588.5 |
    593.3 |
    598.1 |
    602.9 |
    607.7 |
    612.5 |
    617.3 |
    622.1 |
    626.9 |####################
    631.7 |
    636.5 |
    641.3 |
    646.1 |
  (0 below, 1 above range)

project_mono_hash (n=6, range 2018.3-2501.2 ns)
   2018.3 |########################################
   2042.4 |
   2066.6 |
   2090.7 |####################
   2114.9 |
   2139.0 |
   2163.2 |
   2187.3 |
   2211.5 |
   2235.6 |####################
   2259.8 |
   2283.9 |
   2308.1 |
   2332.2 |
   2356.4 |
   2380.5 |####################
   2404.7 |
   2428.8 |
   2453.0 |
   2477.1 |
  (0 below, 1 above range)

project_mono_ic (n=6, range 735.0-895.6 ns)
    735.0 |########################################
    743.0 |
    751.1 |########################################
    759.1 |
    767.1 |
    775.1 |
    783.2 |########################################
    791.2 |
    799.2 |
    807.3 |
    815.3 |
    823.3 |########################################
    831.4 |
    839.4 |
    847.4 |
    855.4 |
    863.5 |
    871.5 |
    879.5 |
    887.6 |########################################
  (0 below, 1 above range)

project_mono_linear (n=6, range 2351.7-2823.8 ns)
   2351.7 |########################################
   2375.3 |
   2398.9 |
   2422.5 |
   2446.1 |
   2469.7 |
   2493.3 |########################################
   2516.9 |
   2540.5 |########################################
   2564.1 |
   2587.7 |
   2611.3 |
   2634.9 |
   2658.5 |
   2682.1 |
   2705.7 |
   2729.3 |
   2752.9 |
   2776.5 |########################################
   2800.1 |########################################
  (0 below, 1 above range)

```
