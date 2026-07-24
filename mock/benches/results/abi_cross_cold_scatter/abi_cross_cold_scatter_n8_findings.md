# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_null dominates: 13% faster than the next best (abi_cross_cold_scatter_cold_null)

abi_cross_cold_scatter_warm_null (3.06 us) leads abi_cross_cold_scatter_cold_null (3.47 us) by 13%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_scatter_cold_scalar is an outlier: 713.7x slower than the field

abi_cross_cold_scatter_cold_scalar (2.18 ms) is 713.7x the fastest (3.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_scatter_warm_null shows alternating (throttle bounce) (autocorr -0.53)

abi_cross_cold_scatter_warm_null's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (3.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} (62827% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} with a 62827% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 713.7x the fastest

Fastest abi_cross_cold_scatter_warm_null (3.06 us) to slowest abi_cross_cold_scatter_cold_scalar (2.18 ms): 713.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 3057.7 ns median
- 3 variants significantly slower than baseline
- Spread: 713.73x (fastest 3057.7 ns, slowest 2182361.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5820ns | 5771ns | 5669ns | 5742ns | 6012ns | +9.93% |
| abi_cross_cold_scatter_cold_scalar | 2185523ns | 2185674ns | 2180965ns | 2184360ns | 2189547ns | +41180.03% |
| abi_cross_cold_scatter_warm_null | 5294ns | 5314ns | 5139ns | 5303ns | 5360ns | base |
| abi_cross_cold_scatter_warm_scalar | 2184385ns | 2184392ns | 2171997ns | 2180824ns | 2195922ns | +41158.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 3483ns | 3407ns | 3569ns | +14.71% | 0.002 |
| abi_cross_cold_scatter_cold_scalar | 2182058ns | 2177440ns | 2185978ns | +71758.58% | 0.000 |
| abi_cross_cold_scatter_warm_null | 3037ns | 2961ns | 3070ns | base | 0.003 |
| abi_cross_cold_scatter_warm_scalar | 2180796ns | 2168675ns | 2192088ns | +71717.05% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 31966.2 | 3678.9 | 3483.4 | n/a |
| abi_cross_cold_scatter_cold_scalar | 74486.1 | 2181725.9 | 2182057.8 | n/a |
| abi_cross_cold_scatter_warm_null | 27802.0 | 3139.3 | 3036.6 | n/a |
| abi_cross_cold_scatter_warm_scalar | 72610.8 | 2259338.2 | 2180796.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.002 | 85.4% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_scatter_warm_null | 0.003 | 96.8% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5820ns | 5820ns | +9.93% |
| abi_cross_cold_scatter_cold_scalar | 2185523ns | 2185523ns | +41180.03% |
| abi_cross_cold_scatter_warm_null | 5294ns | 5294ns | base |
| abi_cross_cold_scatter_warm_scalar | 2184385ns | 2184385ns | +41158.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 3058ns | base | --- | [2982, 3070] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 3466ns | +432.3ns (+14.1%) | [+398, +510]ns | [3415, 3569] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2182361ns | +2179295.0ns (+71272.4%) | [+2174822, +2182946]ns | [2177834, 2185978] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2180940ns | +2177928.8ns (+71227.7%) | [+2166293, +2189057]ns | [2169361, 2192088] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 3004ns | +13.4% | +72712.4% | +72930.2% |
| 2 | 3077ns | +13.0% | +70825.9% | +70431.6% |
| 3 | 2961ns | +15.6% | +73468.9% | +73454.3% |
| 4 | 3062ns | +15.5% | +71000.1% | +71217.0% |
| 5 | 3059ns | +12.9% | +71308.4% | +70790.3% |
| 6 | 3056ns | +17.8% | +71313.7% | +71564.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | -0.279 | moderate- |
| abi_cross_cold_scatter_cold_scalar | 0.108 | ok |
| abi_cross_cold_scatter_warm_null | -0.529 | HIGH- (thermal bounce) |
| abi_cross_cold_scatter_warm_scalar | -0.502 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 124385.1ns | 3483.4ns | 3570.8% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6617569.7ns | 2182057.8ns | 303.3% | HIGH |
| abi_cross_cold_scatter_warm_null | 119951.1ns | 3036.6ns | 3950.2% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6712104.1ns | 2180796.4ns | 307.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 3407.1-3569.4 ns)
   3407.1 |########################################
   3415.2 |########################################
   3423.3 |
   3431.4 |
   3439.6 |
   3447.7 |########################################
   3455.8 |
   3463.9 |
   3472.0 |########################################
   3480.1 |
   3488.2 |
   3496.4 |
   3504.5 |
   3512.6 |
   3520.7 |
   3528.8 |
   3536.9 |########################################
   3545.1 |
   3553.2 |
   3561.3 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2177440.4-2185978.1 ns)
  2177440.4 |####################
  2177867.3 |####################
  2178294.2 |
  2178721.1 |
  2179147.9 |
  2179574.8 |
  2180001.7 |
  2180428.6 |
  2180855.5 |
  2181282.4 |
  2181709.2 |
  2182136.1 |########################################
  2182563.0 |
  2182989.9 |
  2183416.8 |
  2183843.7 |
  2184270.6 |####################
  2184697.4 |
  2185124.3 |
  2185551.2 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 2960.8-3069.6 ns)
   2960.8 |####################
   2966.2 |
   2971.7 |
   2977.1 |
   2982.6 |
   2988.0 |
   2993.4 |
   2998.9 |####################
   3004.3 |
   3009.8 |
   3015.2 |
   3020.6 |
   3026.1 |
   3031.5 |
   3037.0 |
   3042.4 |
   3047.8 |
   3053.3 |####################
   3058.7 |########################################
   3064.2 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2168675.4-2192087.5 ns)
  2168675.4 |########################################
  2169846.0 |########################################
  2171016.6 |
  2172187.2 |
  2173357.8 |
  2174528.4 |
  2175699.0 |
  2176869.6 |########################################
  2178040.2 |
  2179210.8 |
  2180381.5 |
  2181552.1 |
  2182722.7 |
  2183893.3 |########################################
  2185063.9 |
  2186234.5 |
  2187405.1 |
  2188575.7 |
  2189746.3 |########################################
  2190916.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=3592.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=3929.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
