# Reachability fixpoint: whole-column OR vs delta semi-naive

2 variants, 6 samples per variant.
Baseline: **hx_reach__whole**

## Highlights

Baseline for all deltas below: **hx_reach__whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reach__whole dominates: 24% faster than the next best (hx_reach__delta)

hx_reach__whole (211 ns) leads hx_reach__delta (262 ns) by 24%, a clear separation rather than a photo finish. CV 6.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_reach__whole)

The baseline hx_reach__whole is the fastest (211 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reach__whole) is the fastest** at 210.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 210.8 ns, slowest 262.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reach__delta | 2789ns | 2831ns | 2335ns | 2828ns | 2956ns | -2.45% |
| hx_reach__whole | 2859ns | 2852ns | 2602ns | 2828ns | 3033ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reach__delta | 259ns | 218ns | 276ns | +21.26% | 63.308 |
| hx_reach__whole | 213ns | 194ns | 232ns | base | 76.764 |

## Performance model

- Peak throughput: **84.541 Gops/s** (hx_reach__whole; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reach__delta | 62.522 | 74.0% |
| hx_reach__whole | 77.705 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reach__delta | 2789ns | 2789ns | -2.45% |
| hx_reach__whole | 2859ns | 2859ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reach__whole | 211ns | base | --- | [198, 232] | --- | --- | --- | --- |
| hx_reach__delta | 262ns | +44.3ns (+21.0%) | [+26, +66]ns | [238, 276] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reach__whole | hx_reach__delta |
|---|---|---|
| 1 | 194ns | +12.2% |
| 2 | 202ns | +29.5% |
| 3 | 234ns | +12.4% |
| 4 | 230ns | +12.5% |
| 5 | 214ns | +31.9% |
| 6 | 208ns | +30.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reach__delta | 0.071 | ok |
| hx_reach__whole | 0.266 | moderate+ |

**Consistency summary:**

- **hx_reach__delta**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reach__delta | 3.6ns | 258.8ns | 1.4% |  |
| hx_reach__whole | 4.0ns | 213.4ns | 1.9% |  |

## Distribution (algo ns)

```
hx_reach__delta (n=6, range 217.5-276.4 ns)
    217.5 |########################################
    220.4 |
    223.4 |
    226.3 |
    229.3 |
    232.2 |
    235.2 |
    238.1 |
    241.1 |
    244.0 |
    247.0 |
    249.9 |
    252.9 |
    255.8 |########################################
    258.8 |########################################
    261.7 |########################################
    264.7 |
    267.6 |########################################
    270.6 |
    273.5 |
  (0 below, 1 above range)

hx_reach__whole (n=6, range 193.8-231.7 ns)
    193.8 |########################################
    195.7 |
    197.6 |
    199.5 |
    201.4 |########################################
    203.3 |
    205.2 |
    207.1 |########################################
    209.0 |
    210.9 |
    212.8 |########################################
    214.6 |
    216.5 |
    218.4 |
    220.3 |
    222.2 |
    224.1 |
    226.0 |
    227.9 |########################################
    229.8 |
  (0 below, 1 above range)

```
