# Project field access, monomorphic site: direct offset vs inline cache vs hash vs linear

4 variants, 6 samples per variant.
Baseline: **project_mono_direct**

## Highlights

Baseline for all deltas below: **project_mono_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_mono_direct dominates: 42% faster than the next best (project_mono_ic)

project_mono_direct (162 ns) leads project_mono_ic (230 ns) by 42%, a clear separation rather than a photo finish. CV 12.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_mono_linear is an outlier: 4.2x slower than the field

project_mono_linear (677 ns) is 4.2x the fastest (162 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### project_mono_direct is fastest but the noisiest (CV 12.7%)

project_mono_direct wins on median (162 ns) yet has the highest variance (CV 12.7%), while project_mono_linear is the steadiest (CV 8.4%, 677 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (project_mono_direct)

The baseline project_mono_direct is the fastest (162 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {project_mono_direct, project_mono_ic} vs {project_mono_hash, project_mono_linear} (156% apart)

The field splits into a fast tier {project_mono_direct, project_mono_ic} and a slow tier {project_mono_hash, project_mono_linear} with a 156% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest project_mono_direct (162 ns) to slowest project_mono_linear (677 ns): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (project_mono_direct) is the fastest** at 161.9 ns median
- 3 variants significantly slower than baseline
- Spread: 4.18x (fastest 161.9 ns, slowest 677.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_mono_direct | 2532ns | 2405ns | 2281ns | 2380ns | 2887ns | base |
| project_mono_hash | 3004ns | 3014ns | 2631ns | 2906ns | 3336ns | +18.60% |
| project_mono_ic | 2681ns | 2611ns | 2372ns | 2555ns | 3024ns | +5.86% |
| project_mono_linear | 3129ns | 3088ns | 2854ns | 3014ns | 3439ns | +23.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_mono_direct | 173ns | 156ns | 201ns | base | 1.477 |
| project_mono_hash | 589ns | 519ns | 654ns | +239.60% | 0.435 |
| project_mono_ic | 237ns | 209ns | 265ns | +36.57% | 1.081 |
| project_mono_linear | 685ns | 615ns | 755ns | +295.24% | 0.374 |

## Performance model

- Peak throughput: **1.643 Gops/s** (project_mono_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_mono_direct | 1.581 | 96.2% |
| project_mono_hash | 0.434 | 26.4% |
| project_mono_ic | 1.113 | 67.7% |
| project_mono_linear | 0.378 | 23.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_mono_direct | 2532ns | 2532ns | base |
| project_mono_hash | 3004ns | 3004ns | +18.60% |
| project_mono_ic | 2681ns | 2681ns | +5.86% |
| project_mono_linear | 3129ns | 3129ns | +23.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_mono_direct | 162ns | base | --- | [158, 201] | --- | --- | --- | --- |
| project_mono_hash | 590ns | +429.6ns (+265.3%) | [+363, +454]ns | [522, 654] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_ic | 230ns | +61.9ns (+38.2%) | [+54, +75]ns | [215, 265] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_linear | 677ns | +519.4ns (+320.8%) | [+462, +554]ns | [624, 755] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_mono_direct | project_mono_hash | project_mono_ic | project_mono_linear |
|---|---|---|---|---|
| 1 | 156ns | +276.6% | +52.7% | +333.2% |
| 2 | 164ns | +260.8% | +35.3% | +285.4% |
| 3 | 159ns | +229.8% | +38.4% | +286.1% |
| 4 | 160ns | +225.3% | +31.1% | +325.6% |
| 5 | 212ns | +220.4% | +31.6% | +271.1% |
| 6 | 189ns | +232.6% | +33.0% | +281.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_mono_direct | 0.223 | moderate+ |
| project_mono_hash | 0.082 | ok |
| project_mono_ic | 0.042 | ok |
| project_mono_linear | 0.398 | moderate+ |

**Consistency summary:**

- **project_mono_hash**: won 0/6, lost 6/6
- **project_mono_ic**: won 0/6, lost 6/6
- **project_mono_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_mono_direct | 3.5ns | 173.3ns | 2.0% |  |
| project_mono_hash | 3.6ns | 588.7ns | 0.6% |  |
| project_mono_ic | 3.6ns | 236.8ns | 1.5% |  |
| project_mono_linear | 3.7ns | 685.1ns | 0.5% |  |

## Distribution (algo ns)

```
project_mono_direct (n=6, range 155.8-200.6 ns)
    155.8 |####################
    158.0 |########################################
    160.3 |
    162.5 |####################
    164.8 |
    167.0 |
    169.3 |
    171.5 |
    173.7 |
    176.0 |
    178.2 |
    180.5 |
    182.7 |
    185.0 |
    187.2 |####################
    189.4 |
    191.7 |
    193.9 |
    196.2 |
    198.4 |
  (0 below, 1 above range)

project_mono_hash (n=6, range 519.2-654.4 ns)
    519.2 |########################################
    526.0 |
    532.7 |
    539.5 |
    546.2 |
    553.0 |
    559.8 |
    566.5 |
    573.3 |
    580.0 |####################
    586.8 |####################
    593.6 |
    600.3 |
    607.1 |
    613.8 |
    620.6 |
    627.4 |####################
    634.1 |
    640.9 |
    647.6 |
  (0 below, 1 above range)

project_mono_ic (n=6, range 209.2-265.4 ns)
    209.2 |########################################
    212.0 |
    214.8 |
    217.6 |########################################
    220.4 |########################################
    223.3 |
    226.1 |
    228.9 |
    231.7 |
    234.5 |
    237.3 |########################################
    240.1 |
    242.9 |
    245.8 |
    248.6 |
    251.4 |########################################
    254.2 |
    257.0 |
    259.8 |
    262.6 |
  (0 below, 1 above range)

project_mono_linear (n=6, range 614.6-754.6 ns)
    614.6 |########################################
    621.6 |
    628.6 |########################################
    635.6 |
    642.6 |
    649.6 |
    656.6 |
    663.6 |
    670.6 |########################################
    677.6 |########################################
    684.6 |
    691.6 |
    698.6 |
    705.6 |
    712.6 |
    719.6 |########################################
    726.6 |
    733.6 |
    740.6 |
    747.6 |
  (0 below, 1 above range)

```
