# Reachability fixpoint: whole-column OR vs delta semi-naive

2 variants, 6 samples per variant.
Baseline: **hx_reach__whole**

## Highlights

Baseline for all deltas below: **hx_reach__whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reach__whole dominates: 30% faster than the next best (hx_reach__delta)

hx_reach__whole (201 ns) leads hx_reach__delta (262 ns) by 30%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_reach__whole)

The baseline hx_reach__whole is the fastest (201 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reach__whole) is the fastest** at 200.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.30x (fastest 200.8 ns, slowest 261.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reach__delta | 2769ns | 2839ns | 2343ns | 2783ns | 2962ns | +2.08% |
| hx_reach__whole | 2713ns | 2710ns | 2439ns | 2676ns | 2905ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reach__delta | 259ns | 219ns | 279ns | +29.24% | 3.952 |
| hx_reach__whole | 200ns | 182ns | 215ns | base | 5.107 |

## Performance model

- Peak throughput: **5.636 Gops/s** (hx_reach__whole; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reach__delta | 3.910 | 69.4% |
| hx_reach__whole | 5.098 | 90.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reach__delta | 2769ns | 2769ns | +2.08% |
| hx_reach__whole | 2713ns | 2713ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reach__whole | 201ns | base | --- | [186, 215] | --- | --- | --- | --- |
| hx_reach__delta | 262ns | +62.1ns (+30.9%) | [+38, +76]ns | [237, 279] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reach__whole | hx_reach__delta |
|---|---|---|
| 1 | 190ns | +15.4% |
| 2 | 192ns | +41.3% |
| 3 | 212ns | +26.2% |
| 4 | 210ns | +22.2% |
| 5 | 182ns | +39.9% |
| 6 | 218ns | +31.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reach__delta | -0.200 | ok |
| hx_reach__whole | -0.383 | moderate- |

**Consistency summary:**

- **hx_reach__delta**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reach__delta | 4.1ns | 259.1ns | 1.6% |  |
| hx_reach__whole | 4.3ns | 200.5ns | 2.2% |  |

## Distribution (algo ns)

```
hx_reach__delta (n=6, range 219.2-278.8 ns)
    219.2 |########################################
    222.2 |
    225.2 |
    228.1 |
    231.1 |
    234.1 |
    237.1 |
    240.0 |
    243.0 |
    246.0 |
    249.0 |
    252.0 |########################################
    254.9 |########################################
    257.9 |
    260.9 |
    263.9 |
    266.8 |########################################
    269.8 |########################################
    272.8 |
    275.8 |
  (0 below, 1 above range)

hx_reach__whole (n=6, range 181.7-214.8 ns)
    181.7 |########################################
    183.4 |
    185.0 |
    186.7 |
    188.3 |
    190.0 |########################################
    191.6 |########################################
    193.3 |
    194.9 |
    196.6 |
    198.2 |
    199.9 |
    201.6 |
    203.2 |
    204.9 |
    206.5 |
    208.2 |
    209.8 |########################################
    211.5 |########################################
    213.1 |
  (0 below, 1 above range)

```
