# Reachability fixpoint: whole-column OR vs delta semi-naive

2 variants, 6 samples per variant.
Baseline: **hx_reach__whole**

## Highlights

Baseline for all deltas below: **hx_reach__whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reach__whole dominates: 26% faster than the next best (hx_reach__delta)

hx_reach__whole (193 ns) leads hx_reach__delta (242 ns) by 26%, a clear separation rather than a photo finish. CV 9.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_reach__whole is fastest but the noisiest (CV 9.1%)

hx_reach__whole wins on median (193 ns) yet has the highest variance (CV 9.1%), while hx_reach__delta is the steadiest (CV 7.1%, 242 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_reach__whole)

The baseline hx_reach__whole is the fastest (193 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reach__whole) is the fastest** at 192.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.26x (fastest 192.7 ns, slowest 242.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reach__delta | 2632ns | 2658ns | 2365ns | 2588ns | 2833ns | +1.60% |
| hx_reach__whole | 2591ns | 2554ns | 2319ns | 2479ns | 2894ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reach__delta | 241ns | 214ns | 258ns | +25.96% | 1.064 |
| hx_reach__whole | 191ns | 168ns | 210ns | base | 1.340 |

## Performance model

- Peak throughput: **1.525 Gops/s** (hx_reach__whole; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reach__delta | 1.057 | 69.3% |
| hx_reach__whole | 1.328 | 87.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reach__delta | 2632ns | 2632ns | +1.60% |
| hx_reach__whole | 2591ns | 2591ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reach__whole | 193ns | base | --- | [170, 210] | --- | --- | --- | --- |
| hx_reach__delta | 242ns | +47.9ns (+24.9%) | [+46, +54]ns | [221, 258] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reach__whole | hx_reach__delta |
|---|---|---|
| 1 | 168ns | +27.3% |
| 2 | 172ns | +32.6% |
| 3 | 183ns | +25.9% |
| 4 | 205ns | +23.5% |
| 5 | 202ns | +26.0% |
| 6 | 215ns | +21.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reach__delta | 0.441 | moderate+ |
| hx_reach__whole | 0.481 | moderate+ |

**Consistency summary:**

- **hx_reach__delta**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reach__delta | 4.6ns | 240.6ns | 1.9% |  |
| hx_reach__whole | 4.2ns | 191.0ns | 2.2% |  |

## Distribution (algo ns)

```
hx_reach__delta (n=6, range 213.7-258.4 ns)
    213.7 |########################################
    215.9 |
    218.2 |
    220.4 |
    222.6 |
    224.9 |
    227.1 |########################################
    229.3 |########################################
    231.6 |
    233.8 |
    236.0 |
    238.3 |
    240.5 |
    242.7 |
    245.0 |
    247.2 |
    249.4 |
    251.7 |########################################
    253.9 |########################################
    256.1 |
  (0 below, 1 above range)

hx_reach__whole (n=6, range 167.9-210.2 ns)
    167.9 |########################################
    170.0 |
    172.1 |########################################
    174.2 |
    176.4 |
    178.5 |
    180.6 |
    182.7 |########################################
    184.8 |
    186.9 |
    189.1 |
    191.2 |
    193.3 |
    195.4 |
    197.5 |
    199.6 |
    201.7 |########################################
    203.9 |########################################
    206.0 |
    208.1 |
  (0 below, 1 above range)

```
