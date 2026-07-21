# Name resolution: linear scope-chain vs flat shadow-stack

2 variants, 6 samples per variant.
Baseline: **hx_resolve__flat**

## Highlights

Baseline for all deltas below: **hx_resolve__flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_resolve__flat dominates: 391% faster than the next best (hx_resolve__linear)

hx_resolve__flat (143 ns) leads hx_resolve__linear (702 ns) by 391%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_resolve__flat)

The baseline hx_resolve__flat is the fastest (143 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest hx_resolve__flat (143 ns) to slowest hx_resolve__linear (702 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_resolve__flat) is the fastest** at 142.9 ns median
- 1 variant significantly slower than baseline
- Spread: 4.91x (fastest 142.9 ns, slowest 701.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_resolve__flat | 2501ns | 2470ns | 2280ns | 2423ns | 2728ns | base |
| hx_resolve__linear | 3114ns | 3056ns | 2781ns | 3014ns | 3429ns | +24.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_resolve__flat | 144ns | 129ns | 157ns | base | 0.446 |
| hx_resolve__linear | 713ns | 622ns | 782ns | +396.93% | 0.090 |

## Performance model

- Peak throughput: **0.495 Gops/s** (hx_resolve__flat; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_resolve__flat | 0.448 | 90.4% |
| hx_resolve__linear | 0.091 | 18.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_resolve__flat | 2501ns | 2501ns | base |
| hx_resolve__linear | 3114ns | 3114ns | +24.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_resolve__flat | 143ns | base | --- | [131, 157] | --- | --- | --- | --- |
| hx_resolve__linear | 702ns | +560.2ns (+392.0%) | [+513, +636]ns | [656, 782] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_resolve__flat | hx_resolve__linear |
|---|---|---|
| 1 | 132ns | +445.6% |
| 2 | 158ns | +431.1% |
| 3 | 153ns | +365.7% |
| 4 | 155ns | +345.1% |
| 5 | 129ns | +434.4% |
| 6 | 133ns | +367.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_resolve__flat | 0.078 | ok |
| hx_resolve__linear | 0.143 | ok |

**Consistency summary:**

- **hx_resolve__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_resolve__flat | 3.8ns | 143.5ns | 2.6% |  |
| hx_resolve__linear | 2.3ns | 713.3ns | 0.3% |  |

## Distribution (algo ns)

```
hx_resolve__flat (n=6, range 129.2-156.9 ns)
    129.2 |####################
    130.6 |
    132.0 |########################################
    133.3 |
    134.7 |
    136.1 |
    137.5 |
    138.9 |
    140.3 |
    141.6 |
    143.0 |
    144.4 |
    145.8 |
    147.2 |
    148.6 |
    149.9 |
    151.3 |
    152.7 |####################
    154.1 |####################
    155.5 |
  (0 below, 1 above range)

hx_resolve__linear (n=6, range 621.7-781.8 ns)
    621.7 |####################
    629.7 |
    637.7 |
    645.7 |
    653.7 |
    661.7 |
    669.7 |
    677.8 |
    685.8 |########################################
    693.8 |
    701.8 |
    709.8 |####################
    717.8 |####################
    725.8 |
    733.8 |
    741.8 |
    749.8 |
    757.8 |
    765.8 |
    773.8 |
  (0 below, 1 above range)

```
