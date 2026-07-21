# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 442% faster than the next best (resolve_linear)

resolve_flat (1.12 us) leads resolve_linear (6.09 us) by 442%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 6.4x slower than the field

resolve_hashed (7.16 us) is 6.4x the fastest (1.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (1.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.4x the fastest

Fastest resolve_flat (1.12 us) to slowest resolve_hashed (7.16 us): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 1125.0 ns median
- 2 variants significantly slower than baseline
- Spread: 6.37x (fastest 1125.0 ns, slowest 7164.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 3634ns | 3706ns | 3228ns | 3705ns | 3729ns | base |
| resolve_hashed | 9641ns | 9836ns | 8079ns | 9808ns | 10171ns | +165.34% |
| resolve_linear | 8556ns | 8681ns | 7634ns | 8642ns | 8890ns | +135.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 1101ns | 978ns | 1128ns | base | 0.930 |
| resolve_hashed | 7037ns | 5934ns | 7420ns | +538.95% | 0.146 |
| resolve_linear | 6026ns | 5388ns | 6300ns | +447.17% | 0.170 |

## Performance model

- Peak throughput: **1.048 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 0.910 | 86.9% |
| resolve_hashed | 0.143 | 13.6% |
| resolve_linear | 0.168 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 3634ns | 3634ns | base |
| resolve_hashed | 9641ns | 9641ns | +165.34% |
| resolve_linear | 8556ns | 8556ns | +135.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 1125ns | base | --- | [1051, 1128] | --- | --- | --- | --- |
| resolve_hashed | 7164ns | +6037.5ns (+536.7%) | [+5476, +6294]ns | [6526, 7420] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 6094ns | +4967.1ns (+441.5%) | [+4634, +5174]ns | [5684, 6300] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 978ns | +507.0% | +451.2% |
| 2 | 1124ns | +533.4% | +432.2% |
| 3 | 1124ns | +536.4% | +436.8% |
| 4 | 1127ns | +562.9% | +458.7% |
| 5 | 1126ns | +554.6% | +459.9% |
| 6 | 1130ns | +535.1% | +444.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | -0.019 | ok |
| resolve_hashed | 0.103 | ok |
| resolve_linear | 0.245 | moderate+ |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 2.7ns | 1101.3ns | 0.2% |  |
| resolve_hashed | 3.7ns | 7036.9ns | 0.1% |  |
| resolve_linear | 3.8ns | 6026.2ns | 0.1% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 977.5-1128.3 ns)
    977.5 |##########
    985.0 |
    992.6 |
   1000.1 |
   1007.7 |
   1015.2 |
   1022.8 |
   1030.3 |
   1037.8 |
   1045.4 |
   1052.9 |
   1060.5 |
   1068.0 |
   1075.6 |
   1083.1 |
   1090.6 |
   1098.2 |
   1105.7 |
   1113.3 |
   1120.8 |########################################
  (0 below, 1 above range)

resolve_hashed (n=6, range 5933.7-7420.2 ns)
   5933.7 |####################
   6008.0 |
   6082.3 |
   6156.7 |
   6231.0 |
   6305.3 |
   6379.6 |
   6454.0 |
   6528.3 |
   6602.6 |
   6676.9 |
   6751.3 |
   6825.6 |
   6899.9 |
   6974.2 |
   7048.6 |####################
   7122.9 |########################################
   7197.2 |
   7271.6 |
   7345.9 |####################
  (0 below, 1 above range)

resolve_linear (n=6, range 5387.9-6300.2 ns)
   5387.9 |########################################
   5433.5 |
   5479.1 |
   5524.7 |
   5570.4 |
   5616.0 |
   5661.6 |
   5707.2 |
   5752.8 |
   5798.4 |
   5844.1 |
   5889.7 |
   5935.3 |########################################
   5980.9 |
   6026.5 |########################################
   6072.1 |
   6117.7 |########################################
   6163.4 |
   6209.0 |
   6254.6 |########################################
  (0 below, 1 above range)

```
