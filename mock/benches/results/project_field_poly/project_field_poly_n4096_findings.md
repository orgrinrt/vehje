# Project field access, polymorphic site: inline-cache hazard vs hash vs linear

3 variants, 6 samples per variant.
Baseline: **project_poly_hash**

## Highlights

Baseline for all deltas below: **project_poly_hash**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_poly_hash dominates: 16% faster than the next best (project_poly_ic)

project_poly_hash (11.08 us) leads project_poly_ic (12.83 us) by 16%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_poly_linear is an outlier: 2.1x slower than the field

project_poly_linear (22.83 us) is 2.1x the fastest (11.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### project_poly_hash is fastest but the noisiest (CV 7.0%)

project_poly_hash wins on median (11.08 us) yet has the highest variance (CV 7.0%), while project_poly_ic is the steadiest (CV 4.8%, 12.83 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (project_poly_hash)

The baseline project_poly_hash is the fastest (11.08 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (project_poly_hash) is the fastest** at 11079.8 ns median
- 2 variants significantly slower than baseline
- Spread: 2.06x (fastest 11079.8 ns, slowest 22834.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_poly_hash | 13482ns | 13586ns | 11784ns | 13448ns | 14381ns | base |
| project_poly_ic | 15743ns | 15265ns | 15178ns | 15256ns | 16756ns | +16.77% |
| project_poly_linear | 25735ns | 25415ns | 24229ns | 25066ns | 27492ns | +90.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_poly_hash | 10999ns | 9541ns | 11790ns | base | 0.372 |
| project_poly_ic | 13213ns | 12719ns | 14048ns | +20.13% | 0.310 |
| project_poly_linear | 23280ns | 21790ns | 25139ns | +111.65% | 0.176 |

## Performance model

- Peak throughput: **0.429 Gops/s** (project_poly_hash; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_poly_hash | 0.370 | 86.1% |
| project_poly_ic | 0.319 | 74.4% |
| project_poly_linear | 0.179 | 41.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_poly_hash | 13482ns | 13482ns | base |
| project_poly_ic | 15743ns | 15743ns | +16.77% |
| project_poly_linear | 25735ns | 25735ns | +90.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_poly_hash | 11080ns | base | --- | [10128, 11790] | --- | --- | --- | --- |
| project_poly_ic | 12825ns | +2015.2ns (+18.2%) | [+1212, +3415]ns | [12766, 14048] | YES | 0.0313 | 0.0313 | 0 |
| project_poly_linear | 22834ns | +11455.2ns (+103.4%) | [+10490, +14896]ns | [21865, 25139] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_poly_hash | project_poly_ic | project_poly_linear |
|---|---|---|---|
| 1 | 9541ns | +33.3% | +175.9% |
| 2 | 11567ns | +18.7% | +101.0% |
| 3 | 10715ns | +34.1% | +104.8% |
| 4 | 10946ns | +17.1% | +118.8% |
| 5 | 11213ns | +14.4% | +99.9% |
| 6 | 12014ns | +6.7% | +81.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_poly_hash | -0.213 | moderate- |
| project_poly_ic | 0.080 | ok |
| project_poly_linear | -0.017 | ok |

**Consistency summary:**

- **project_poly_ic**: won 0/6, lost 6/6
- **project_poly_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_poly_hash | 3.4ns | 10999.3ns | 0.0% |  |
| project_poly_ic | 3.6ns | 13213.3ns | 0.0% |  |
| project_poly_linear | 3.9ns | 23279.6ns | 0.0% |  |

## Distribution (algo ns)

```
project_poly_hash (n=6, range 9540.8-11790.2 ns)
   9540.8 |########################################
   9653.3 |
   9765.7 |
   9878.2 |
   9990.7 |
  10103.1 |
  10215.6 |
  10328.1 |
  10440.6 |
  10553.0 |
  10665.5 |########################################
  10778.0 |
  10890.4 |########################################
  11002.9 |
  11115.4 |########################################
  11227.9 |
  11340.3 |
  11452.8 |
  11565.3 |########################################
  11677.7 |
  (0 below, 1 above range)

project_poly_ic (n=6, range 12718.8-14048.4 ns)
  12718.8 |#############
  12785.3 |########################################
  12851.8 |
  12918.2 |
  12984.7 |
  13051.2 |
  13117.7 |
  13184.1 |
  13250.6 |
  13317.1 |
  13383.6 |
  13450.1 |
  13516.5 |
  13583.0 |
  13649.5 |
  13716.0 |#############
  13782.4 |
  13848.9 |
  13915.4 |
  13981.9 |
  (0 below, 1 above range)

project_poly_linear (n=6, range 21789.6-25139.3 ns)
  21789.6 |########################################
  21957.1 |
  22124.6 |
  22292.1 |####################
  22459.5 |
  22627.0 |
  22794.5 |
  22962.0 |
  23129.5 |####################
  23297.0 |
  23464.5 |
  23632.0 |
  23799.4 |####################
  23966.9 |
  24134.4 |
  24301.9 |
  24469.4 |
  24636.9 |
  24804.4 |
  24971.9 |
  (0 below, 1 above range)

```
