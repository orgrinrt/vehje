# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 344% faster than the next best (resolve_linear)

resolve_flat (95 ns) leads resolve_linear (423 ns) by 344%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 5.0x slower than the field

resolve_hashed (481 ns) is 5.0x the fastest (95 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (95 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest resolve_flat (95 ns) to slowest resolve_hashed (481 ns): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 95.4 ns median
- 2 variants significantly slower than baseline
- Spread: 5.04x (fastest 95.4 ns, slowest 481.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 2645ns | 2687ns | 2182ns | 2682ns | 2820ns | base |
| resolve_hashed | 3000ns | 3072ns | 2593ns | 3068ns | 3102ns | +13.45% |
| resolve_linear | 2912ns | 2998ns | 2512ns | 2938ns | 3071ns | +10.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 94ns | 83ns | 99ns | base | 0.679 |
| resolve_hashed | 478ns | 397ns | 520ns | +406.63% | 0.134 |
| resolve_linear | 424ns | 332ns | 482ns | +350.01% | 0.151 |

## Performance model

- Peak throughput: **0.772 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 0.671 | 86.9% |
| resolve_hashed | 0.133 | 17.2% |
| resolve_linear | 0.151 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 2645ns | 2645ns | base |
| resolve_hashed | 3000ns | 3000ns | +13.45% |
| resolve_linear | 2912ns | 2912ns | +10.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 95ns | base | --- | [88, 99] | --- | --- | --- | --- |
| resolve_hashed | 481ns | +386.2ns (+404.8%) | [+341, +423]ns | [432, 520] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 423ns | +328.3ns (+344.1%) | [+277, +385]ns | [368, 482] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 83ns | +379.0% | +300.6% |
| 2 | 95ns | +448.2% | +411.8% |
| 3 | 94ns | +399.3% | +356.6% |
| 4 | 99ns | +424.2% | +382.7% |
| 5 | 99ns | +370.9% | +306.6% |
| 6 | 96ns | +415.3% | +336.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | 0.113 | ok |
| resolve_hashed | -0.463 | moderate- |
| resolve_linear | -0.397 | moderate- |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 2.9ns | 94.3ns | 3.1% |  |
| resolve_hashed | 4.0ns | 477.8ns | 0.8% |  |
| resolve_linear | 2.2ns | 424.4ns | 0.5% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 82.9-99.2 ns)
     82.9 |########################################
     83.7 |
     84.5 |
     85.3 |
     86.2 |
     87.0 |
     87.8 |
     88.6 |
     89.4 |
     90.2 |
     91.1 |
     91.9 |
     92.7 |
     93.5 |########################################
     94.3 |########################################
     95.1 |########################################
     95.9 |
     96.8 |
     97.6 |
     98.4 |
  (0 below, 2 above range)

resolve_hashed (n=6, range 397.1-520.4 ns)
    397.1 |####################
    403.3 |
    409.4 |
    415.6 |
    421.8 |
    427.9 |
    434.1 |
    440.3 |
    446.4 |
    452.6 |
    458.8 |
    464.9 |########################################
    471.1 |
    477.2 |
    483.4 |
    489.6 |####################
    495.7 |
    501.9 |
    508.1 |
    514.2 |####################
  (0 below, 1 above range)

resolve_linear (n=6, range 332.1-482.5 ns)
    332.1 |########################################
    339.6 |
    347.1 |
    354.7 |
    362.2 |
    369.7 |
    377.2 |
    384.7 |
    392.3 |
    399.8 |########################################
    407.3 |
    414.8 |########################################
    422.3 |########################################
    429.9 |
    437.4 |
    444.9 |
    452.4 |
    459.9 |
    467.5 |
    475.0 |########################################
  (0 below, 1 above range)

```
