# Interpreter model: recursive tree-walk vs CFG-of-blocks linear scan

2 variants, 6 samples per variant.
Baseline: **hx_cfg__cfgblock**

## Highlights

Baseline for all deltas below: **hx_cfg__cfgblock**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_cfg__cfgblock dominates: 815% faster than the next best (hx_cfg__treewalk)

hx_cfg__cfgblock (23.45 us) leads hx_cfg__treewalk (214.48 us) by 815%, a clear separation rather than a photo finish. CV 6.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_cfg__cfgblock is fastest but the noisiest (CV 6.0%)

hx_cfg__cfgblock wins on median (23.45 us) yet has the highest variance (CV 6.0%), while hx_cfg__treewalk is the steadiest (CV 0.8%, 214.48 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_cfg__cfgblock)

The baseline hx_cfg__cfgblock is the fastest (23.45 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.1x the fastest

Fastest hx_cfg__cfgblock (23.45 us) to slowest hx_cfg__treewalk (214.48 us): 9.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_cfg__cfgblock) is the fastest** at 23451.0 ns median
- 1 variant significantly slower than baseline
- Spread: 9.15x (fastest 23451.0 ns, slowest 214480.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 25761ns | 25728ns | 24015ns | 25218ns | 27448ns | base |
| hx_cfg__treewalk | 217066ns | 216760ns | 214685ns | 216414ns | 219234ns | +742.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cfg__cfgblock | 23462ns | 21888ns | 24972ns | base | 0.698 |
| hx_cfg__treewalk | 214679ns | 211963ns | 216803ns | +815.00% | 0.076 |

## Performance model

- Peak throughput: **0.749 Gops/s** (hx_cfg__cfgblock; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cfg__cfgblock | 0.699 | 93.3% |
| hx_cfg__treewalk | 0.076 | 10.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cfg__cfgblock | 25761ns | 25761ns | base |
| hx_cfg__treewalk | 217066ns | 217066ns | +742.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 23451ns | base | --- | [21964, 24972] | --- | --- | --- | --- |
| hx_cfg__treewalk | 214480ns | +191820.8ns (+818.0%) | [+188210, +193619]ns | [212753, 216803] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cfg__cfgblock | hx_cfg__treewalk |
|---|---|---|
| 1 | 22267ns | +859.0% |
| 2 | 21888ns | +888.8% |
| 3 | 24825ns | +774.9% |
| 4 | 22040ns | +874.4% |
| 5 | 24635ns | +769.5% |
| 6 | 25120ns | +743.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cfg__cfgblock | -0.162 | ok |
| hx_cfg__treewalk | 0.209 | moderate+ |

**Consistency summary:**

- **hx_cfg__treewalk**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cfg__cfgblock | 3.5ns | 23462.2ns | 0.0% |  |
| hx_cfg__treewalk | 3.6ns | 214678.6ns | 0.0% |  |

## Distribution (algo ns)

```
hx_cfg__cfgblock (n=6, range 21887.5-24972.1 ns)
  21887.5 |########################################
  22041.7 |
  22196.0 |####################
  22350.2 |
  22504.4 |
  22658.7 |
  22812.9 |
  22967.1 |
  23121.3 |
  23275.6 |
  23429.8 |
  23584.0 |
  23738.3 |
  23892.5 |
  24046.7 |
  24200.9 |
  24355.2 |
  24509.4 |####################
  24663.6 |
  24817.9 |####################
  (0 below, 1 above range)

hx_cfg__treewalk (n=6, range 211963.3-216802.7 ns)
  211963.3 |########################################
  212205.3 |
  212447.2 |
  212689.2 |
  212931.2 |
  213173.1 |
  213415.1 |########################################
  213657.1 |
  213899.1 |
  214141.0 |########################################
  214383.0 |
  214625.0 |########################################
  214866.9 |
  215108.9 |
  215350.9 |
  215592.9 |
  215834.8 |
  216076.8 |
  216318.8 |########################################
  216560.7 |
  (0 below, 1 above range)

```
