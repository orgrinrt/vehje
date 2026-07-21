# Project field access, monomorphic site: direct offset vs inline cache vs hash vs linear

4 variants, 6 samples per variant.
Baseline: **project_mono_direct**

## Highlights

Baseline for all deltas below: **project_mono_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_mono_direct dominates: 42% faster than the next best (project_mono_ic)

project_mono_direct (59 ns) leads project_mono_ic (84 ns) by 42%, a clear separation rather than a photo finish. CV 18.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_mono_linear is an outlier: 3.5x slower than the field

project_mono_linear (207 ns) is 3.5x the fastest (59 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### project_mono_direct is fastest but the noisiest (CV 18.3%)

project_mono_direct wins on median (59 ns) yet has the highest variance (CV 18.3%), while project_mono_linear is the steadiest (CV 12.3%, 207 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (project_mono_direct)

The baseline project_mono_direct is the fastest (59 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {project_mono_direct, project_mono_ic} vs {project_mono_hash, project_mono_linear} (119% apart)

The field splits into a fast tier {project_mono_direct, project_mono_ic} and a slow tier {project_mono_hash, project_mono_linear} with a 119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.5x the fastest

Fastest project_mono_direct (59 ns) to slowest project_mono_linear (207 ns): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (project_mono_direct) is the fastest** at 59.4 ns median
- 3 variants significantly slower than baseline
- Spread: 3.49x (fastest 59.4 ns, slowest 207.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_mono_direct | 2546ns | 2357ns | 2211ns | 2312ns | 3066ns | base |
| project_mono_hash | 2793ns | 2753ns | 2373ns | 2638ns | 3235ns | +9.68% |
| project_mono_ic | 2568ns | 2513ns | 2226ns | 2448ns | 2918ns | +0.83% |
| project_mono_linear | 2737ns | 2778ns | 2335ns | 2658ns | 3057ns | +7.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_mono_direct | 62ns | 52ns | 74ns | base | 1.029 |
| project_mono_hash | 184ns | 154ns | 213ns | +196.22% | 0.348 |
| project_mono_ic | 87ns | 75ns | 100ns | +39.30% | 0.739 |
| project_mono_linear | 203ns | 169ns | 229ns | +226.51% | 0.315 |

## Performance model

- Peak throughput: **1.228 Gops/s** (project_mono_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_mono_direct | 1.077 | 87.7% |
| project_mono_hash | 0.347 | 28.3% |
| project_mono_ic | 0.761 | 61.9% |
| project_mono_linear | 0.309 | 25.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_mono_direct | 2546ns | 2546ns | base |
| project_mono_hash | 2793ns | 2793ns | +9.68% |
| project_mono_ic | 2568ns | 2568ns | +0.83% |
| project_mono_linear | 2737ns | 2737ns | +7.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_mono_direct | 59ns | base | --- | [53, 74] | --- | --- | --- | --- |
| project_mono_hash | 184ns | +121.4ns (+204.4%) | [+102, +142]ns | [155, 213] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_ic | 84ns | +24.2ns (+40.7%) | [+22, +27]ns | [76, 100] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_linear | 207ns | +144.3ns (+243.0%) | [+120, +158]ns | [173, 229] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_mono_direct | project_mono_hash | project_mono_ic | project_mono_linear |
|---|---|---|---|---|
| 1 | 84ns | +186.7% | +28.4% | +195.2% |
| 2 | 54ns | +189.6% | +38.7% | +228.4% |
| 3 | 52ns | +196.0% | +47.2% | +223.8% |
| 4 | 54ns | +237.5% | +42.3% | +282.8% |
| 5 | 65ns | +188.2% | +41.2% | +226.9% |
| 6 | 65ns | +187.6% | +43.2% | +220.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_mono_direct | -0.041 | ok |
| project_mono_hash | -0.144 | ok |
| project_mono_ic | -0.066 | ok |
| project_mono_linear | -0.088 | ok |

**Consistency summary:**

- **project_mono_hash**: won 0/6, lost 6/6
- **project_mono_ic**: won 0/6, lost 6/6
- **project_mono_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_mono_direct | 3.1ns | 62.2ns | 5.0% | HIGH |
| project_mono_hash | 2.8ns | 184.2ns | 1.5% |  |
| project_mono_ic | 2.7ns | 86.6ns | 3.1% |  |
| project_mono_linear | 3.9ns | 203.0ns | 1.9% |  |

## Distribution (algo ns)

```
project_mono_direct (n=6, range 52.1-74.2 ns)
     52.1 |####################
     53.2 |########################################
     54.3 |
     55.4 |
     56.5 |
     57.6 |
     58.7 |
     59.8 |
     60.9 |
     62.0 |
     63.1 |
     64.2 |########################################
     65.3 |
     66.4 |
     67.5 |
     68.6 |
     69.7 |
     70.8 |
     71.9 |
     73.0 |
  (0 below, 1 above range)

project_mono_hash (n=6, range 154.2-213.1 ns)
    154.2 |########################################
    157.1 |
    160.1 |
    163.0 |
    166.0 |
    168.9 |
    171.9 |
    174.8 |
    177.8 |
    180.7 |####################
    183.6 |########################################
    186.6 |
    189.5 |
    192.5 |
    195.4 |
    198.4 |
    201.3 |
    204.3 |
    207.2 |
    210.2 |
  (0 below, 1 above range)

project_mono_ic (n=6, range 74.6-100.0 ns)
     74.6 |####################
     75.9 |########################################
     77.1 |
     78.4 |
     79.7 |
     80.9 |
     82.2 |
     83.5 |
     84.8 |
     86.0 |
     87.3 |
     88.6 |
     89.8 |
     91.1 |####################
     92.4 |####################
     93.7 |
     94.9 |
     96.2 |
     97.5 |
     98.7 |
  (0 below, 1 above range)

project_mono_linear (n=6, range 168.7-229.1 ns)
    168.7 |####################
    171.7 |
    174.7 |####################
    177.8 |
    180.8 |
    183.8 |
    186.8 |
    189.9 |
    192.9 |
    195.9 |
    198.9 |
    201.9 |
    205.0 |########################################
    208.0 |
    211.0 |####################
    214.0 |
    217.1 |
    220.1 |
    223.1 |
    226.1 |
  (0 below, 1 above range)

```
