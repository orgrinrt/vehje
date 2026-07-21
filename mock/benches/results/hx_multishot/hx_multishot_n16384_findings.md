# Bounded multi-shot enumeration vs single-shot

2 variants, 6 samples per variant.
Baseline: **hx_multishot__single**

## Highlights

Baseline for all deltas below: **hx_multishot__single**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_multishot__single dominates: 395% faster than the next best (hx_multishot__multishot)

hx_multishot__single (2.39 us) leads hx_multishot__multishot (11.83 us) by 395%, a clear separation rather than a photo finish. CV 6.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_multishot__single is fastest but the noisiest (CV 6.8%)

hx_multishot__single wins on median (2.39 us) yet has the highest variance (CV 6.8%), while hx_multishot__multishot is the steadiest (CV 5.6%, 11.83 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_multishot__single)

The baseline hx_multishot__single is the fastest (2.39 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest hx_multishot__single (2.39 us) to slowest hx_multishot__multishot (11.83 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_multishot__single) is the fastest** at 2389.6 ns median
- 1 variant significantly slower than baseline
- Spread: 4.95x (fastest 2389.6 ns, slowest 11833.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_multishot__multishot | 14158ns | 14256ns | 12575ns | 14252ns | 14808ns | +190.15% |
| hx_multishot__single | 4880ns | 4972ns | 4165ns | 4896ns | 5213ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_multishot__multishot | 11757ns | 10430ns | 12308ns | +405.04% | 1.394 |
| hx_multishot__single | 2328ns | 1979ns | 2449ns | base | 7.038 |

## Performance model

- Peak throughput: **8.280 Gops/s** (hx_multishot__single; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_multishot__multishot | 1.385 | 16.7% |
| hx_multishot__single | 6.856 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_multishot__multishot | 14158ns | 14158ns | +190.15% |
| hx_multishot__single | 4880ns | 4880ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_multishot__single | 2390ns | base | --- | [2145, 2449] | --- | --- | --- | --- |
| hx_multishot__multishot | 11834ns | +9482.9ns (+396.8%) | [+8742, +10062]ns | [11129, 12308] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_multishot__single | hx_multishot__multishot |
|---|---|---|
| 1 | 2311ns | +351.3% |
| 2 | 1979ns | +498.0% |
| 3 | 2435ns | +386.1% |
| 4 | 2463ns | +380.1% |
| 5 | 2390ns | +400.2% |
| 6 | 2389ns | +429.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_multishot__multishot | 0.040 | ok |
| hx_multishot__single | -0.030 | ok |

**Consistency summary:**

- **hx_multishot__multishot**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_multishot__multishot | 3.5ns | 11756.6ns | 0.0% |  |
| hx_multishot__single | 4.3ns | 2327.9ns | 0.2% |  |

## Distribution (algo ns)

```
hx_multishot__multishot (n=6, range 10430.4-12307.5 ns)
  10430.4 |#############
  10524.3 |
  10618.1 |
  10712.0 |
  10805.8 |
  10899.7 |
  10993.5 |
  11087.4 |
  11181.2 |
  11275.1 |
  11369.0 |
  11462.8 |
  11556.7 |
  11650.5 |
  11744.4 |########################################
  11838.2 |
  11932.1 |#############
  12025.9 |
  12119.8 |
  12213.6 |
  (0 below, 1 above range)

hx_multishot__single (n=6, range 1978.8-2448.9 ns)
   1978.8 |####################
   2002.3 |
   2025.8 |
   2049.3 |
   2072.8 |
   2096.3 |
   2119.8 |
   2143.4 |
   2166.9 |
   2190.4 |
   2213.9 |
   2237.4 |
   2260.9 |
   2284.4 |
   2307.9 |####################
   2331.4 |
   2354.9 |
   2378.4 |########################################
   2401.9 |
   2425.4 |####################
  (0 below, 1 above range)

```
