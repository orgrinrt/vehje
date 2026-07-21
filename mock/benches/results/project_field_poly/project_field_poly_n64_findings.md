# Project field access, polymorphic site: inline-cache hazard vs hash vs linear

3 variants, 6 samples per variant.
Baseline: **project_poly_hash**

## Highlights

Baseline for all deltas below: **project_poly_hash**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_poly_hash dominates: 16% faster than the next best (project_poly_linear)

project_poly_hash (178 ns) leads project_poly_linear (207 ns) by 16%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_poly_hash is fastest but the noisiest (CV 7.9%)

project_poly_hash wins on median (178 ns) yet has the highest variance (CV 7.9%), while project_poly_ic is the steadiest (CV 5.6%, 212 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (project_poly_hash)

The baseline project_poly_hash is the fastest (178 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (project_poly_hash) is the fastest** at 178.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.19x (fastest 178.2 ns, slowest 211.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_poly_hash | 2700ns | 2755ns | 2338ns | 2701ns | 2880ns | base |
| project_poly_ic | 2900ns | 2901ns | 2754ns | 2858ns | 3036ns | +7.40% |
| project_poly_linear | 2799ns | 2777ns | 2437ns | 2776ns | 3014ns | +3.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_poly_hash | 176ns | 152ns | 190ns | base | 0.364 |
| project_poly_ic | 210ns | 194ns | 222ns | +19.05% | 0.305 |
| project_poly_linear | 208ns | 178ns | 225ns | +17.95% | 0.308 |

## Performance model

- Peak throughput: **0.420 Gops/s** (project_poly_hash; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_poly_hash | 0.359 | 85.6% |
| project_poly_ic | 0.302 | 72.1% |
| project_poly_linear | 0.310 | 73.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_poly_hash | 2700ns | 2700ns | base |
| project_poly_ic | 2900ns | 2900ns | +7.40% |
| project_poly_linear | 2799ns | 2799ns | +3.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_poly_hash | 178ns | base | --- | [160, 190] | --- | --- | --- | --- |
| project_poly_ic | 212ns | +29.7ns (+16.7%) | [+14, +57]ns | [195, 222] | YES | 0.0313 | 0.0313 | 0 |
| project_poly_linear | 207ns | +27.8ns (+15.6%) | [+17, +50]ns | [191, 225] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_poly_hash | project_poly_ic | project_poly_linear |
|---|---|---|---|
| 1 | 167ns | +16.2% | +6.2% |
| 2 | 181ns | +8.3% | +13.4% |
| 3 | 179ns | +18.1% | +16.5% |
| 4 | 152ns | +41.0% | +45.4% |
| 5 | 178ns | +28.9% | +14.8% |
| 6 | 199ns | +6.5% | +15.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_poly_hash | -0.080 | ok |
| project_poly_ic | 0.422 | moderate+ |
| project_poly_linear | -0.033 | ok |

**Consistency summary:**

- **project_poly_ic**: won 0/6, lost 6/6
- **project_poly_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_poly_hash | 2.6ns | 176.0ns | 1.5% |  |
| project_poly_ic | 3.3ns | 209.5ns | 1.6% |  |
| project_poly_linear | 3.7ns | 207.6ns | 1.8% |  |

## Distribution (algo ns)

```
project_poly_hash (n=6, range 152.5-190.0 ns)
    152.5 |########################################
    154.4 |
    156.2 |
    158.1 |
    160.0 |
    161.9 |
    163.8 |
    165.6 |########################################
    167.5 |
    169.4 |
    171.2 |
    173.1 |
    175.0 |
    176.9 |########################################
    178.8 |########################################
    180.6 |########################################
    182.5 |
    184.4 |
    186.2 |
    188.1 |
  (0 below, 1 above range)

project_poly_ic (n=6, range 194.2-221.9 ns)
    194.2 |####################
    195.6 |####################
    197.0 |
    198.4 |
    199.7 |
    201.1 |
    202.5 |
    203.9 |
    205.3 |
    206.7 |
    208.1 |
    209.4 |
    210.8 |########################################
    212.2 |
    213.6 |
    215.0 |####################
    216.4 |
    217.7 |
    219.1 |
    220.5 |
  (0 below, 1 above range)

project_poly_linear (n=6, range 177.5-225.4 ns)
    177.5 |########################################
    179.9 |
    182.3 |
    184.7 |
    187.1 |
    189.5 |
    191.9 |
    194.3 |
    196.7 |
    199.1 |
    201.5 |########################################
    203.9 |########################################
    206.3 |########################################
    208.7 |
    211.1 |
    213.5 |
    215.9 |
    218.3 |
    220.7 |########################################
    223.1 |
  (0 below, 1 above range)

```
