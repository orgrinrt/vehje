# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 324% faster than the next best (resolve_linear)

resolve_flat (94 ns) leads resolve_linear (400 ns) by 324%, a clear separation rather than a photo finish. CV 9.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 4.8x slower than the field

resolve_hashed (450 ns) is 4.8x the fastest (94 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### resolve_flat is fastest but the noisiest (CV 9.4%)

resolve_flat wins on median (94 ns) yet has the highest variance (CV 9.4%), while resolve_linear is the steadiest (CV 6.9%, 400 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (94 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest resolve_flat (94 ns) to slowest resolve_hashed (450 ns): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 94.4 ns median
- 2 variants significantly slower than baseline
- Spread: 4.76x (fastest 94.4 ns, slowest 449.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 2563ns | 2585ns | 2236ns | 2474ns | 2861ns | base |
| resolve_hashed | 2844ns | 2809ns | 2544ns | 2723ns | 3176ns | +10.96% |
| resolve_linear | 2797ns | 2869ns | 2505ns | 2759ns | 3002ns | +9.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 92ns | 78ns | 101ns | base | 0.698 |
| resolve_hashed | 453ns | 391ns | 499ns | +394.10% | 0.141 |
| resolve_linear | 402ns | 357ns | 433ns | +338.17% | 0.159 |

## Performance model

- Peak throughput: **0.817 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 0.678 | 82.9% |
| resolve_hashed | 0.142 | 17.4% |
| resolve_linear | 0.160 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 2563ns | 2563ns | base |
| resolve_hashed | 2844ns | 2844ns | +10.96% |
| resolve_linear | 2797ns | 2797ns | +9.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 94ns | base | --- | [80, 101] | --- | --- | --- | --- |
| resolve_hashed | 450ns | +358.8ns (+380.0%) | [+326, +400]ns | [411, 499] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 400ns | +299.4ns (+317.2%) | [+287, +344]ns | [372, 433] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 101ns | +399.8% | +296.0% |
| 2 | 82ns | +437.6% | +414.1% |
| 3 | 78ns | +399.6% | +355.6% |
| 4 | 92ns | +367.8% | +321.2% |
| 5 | 100ns | +358.6% | +298.0% |
| 6 | 97ns | +408.9% | +361.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | 0.172 | ok |
| resolve_hashed | 0.185 | ok |
| resolve_linear | -0.060 | ok |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 3.7ns | 91.7ns | 4.0% |  |
| resolve_hashed | 3.5ns | 453.2ns | 0.8% |  |
| resolve_linear | 3.1ns | 401.9ns | 0.8% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 78.3-100.8 ns)
     78.3 |########################################
     79.4 |
     80.5 |
     81.7 |########################################
     82.8 |
     83.9 |
     85.0 |
     86.2 |
     87.3 |
     88.4 |
     89.6 |
     90.7 |
     91.8 |########################################
     92.9 |
     94.1 |
     95.2 |
     96.3 |########################################
     97.4 |
     98.6 |
     99.7 |########################################
  (0 below, 1 above range)

resolve_hashed (n=6, range 391.2-499.0 ns)
    391.2 |########################################
    396.6 |
    402.0 |
    407.4 |
    412.8 |
    418.1 |
    423.5 |
    428.9 |########################################
    434.3 |########################################
    439.7 |
    445.1 |
    450.5 |
    455.9 |########################################
    461.2 |
    466.6 |
    472.0 |
    477.4 |
    482.8 |
    488.2 |########################################
    493.6 |
  (0 below, 1 above range)

resolve_linear (n=6, range 356.7-433.4 ns)
    356.7 |####################
    360.5 |
    364.4 |
    368.2 |
    372.0 |
    375.9 |
    379.7 |
    383.5 |
    387.4 |####################
    391.2 |
    395.0 |
    398.9 |########################################
    402.7 |
    406.5 |
    410.4 |
    414.2 |
    418.0 |####################
    421.9 |
    425.7 |
    429.5 |
  (0 below, 1 above range)

```
