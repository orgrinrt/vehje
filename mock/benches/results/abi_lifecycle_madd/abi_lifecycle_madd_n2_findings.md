# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 81114% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (3.39 us) leads abi_lifecycle_madd_held_handle (2.75 ms) by 81114%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.75 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1307.5x slower than the field

abi_lifecycle_madd_fresh_per_batch (4.43 ms) is 1307.5x the fastest (3.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (81114% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 81114% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1307.5x the fastest

Fastest abi_lifecycle_madd_null_entry (3.39 us) to slowest abi_lifecycle_madd_fresh_per_batch (4.43 ms): 1307.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 3388.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1307.45x (fastest 3388.3 ns, slowest 4430047.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 4646625ns | 4433598ns | 4372221ns | 4418065ns | 5126666ns | +64.26% |
| abi_lifecycle_madd_fresh_per_column | 2829055ns | 2769040ns | 2758696ns | 2766277ns | 2958400ns | +0.01% |
| abi_lifecycle_madd_held_handle | 2828861ns | 2754969ns | 2745058ns | 2753808ns | 2983342ns | base |
| abi_lifecycle_madd_null_entry | 5764ns | 5630ns | 5511ns | 5609ns | 6122ns | -99.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 4642756ns | 4368717ns | 5122027ns | +64.32% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2825779ns | 2755976ns | 2954382ns | +0.01% | 0.000 |
| abi_lifecycle_madd_held_handle | 2825425ns | 2741970ns | 2979271ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 3459ns | 3336ns | 3651ns | -99.88% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 81144.6 | 4621876.2 | 4642755.8 | n/a |
| abi_lifecycle_madd_fresh_per_column | 65953.0 | 2813013.4 | 2825779.3 | n/a |
| abi_lifecycle_madd_held_handle | 65007.3 | 2794321.9 | 2825425.4 | n/a |
| abi_lifecycle_madd_null_entry | 29276.3 | 3514.7 | 3458.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.001 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 4646625ns | 4646625ns | +64.26% |
| abi_lifecycle_madd_fresh_per_column | 2829055ns | 2829055ns | +0.01% |
| abi_lifecycle_madd_held_handle | 2828861ns | 2828861ns | base |
| abi_lifecycle_madd_null_entry | 5764ns | 5764ns | -99.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2751777ns | base | --- | [2745228, 2979271] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 4430048ns | +1654147.8ns (+60.1%) | [+1496834, +2301010]ns | [4376192, 5122027] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2766074ns | no significant difference | [-149499, +133365]ns | [2756882, 2954382] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_madd_null_entry | 3388ns | -2748134.8ns (-99.9%) | [-2975874, -2741892]ns | [3336, 3651] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 3071428ns | +44.5% | -10.0% | -99.9% |
| 2 | 2748633ns | +60.9% | +0.7% | -99.9% |
| 3 | 2887114ns | +66.3% | +6.7% | -99.9% |
| 4 | 2754921ns | +97.6% | +2.7% | -99.9% |
| 5 | 2748486ns | +59.5% | +0.3% | -99.9% |
| 6 | 2741970ns | +59.3% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.001 | ok |
| abi_lifecycle_madd_fresh_per_column | -0.074 | ok |
| abi_lifecycle_madd_held_handle | -0.183 | ok |
| abi_lifecycle_madd_null_entry | -0.225 | moderate- |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 14093973.2ns | 4642755.8ns | 303.6% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8526796.7ns | 2825779.3ns | 301.8% | HIGH |
| abi_lifecycle_madd_held_handle | 8445120.0ns | 2825425.4ns | 298.9% | HIGH |
| abi_lifecycle_madd_null_entry | 121838.7ns | 3458.5ns | 3522.9% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 4368717.1-5122026.9 ns)
  4368717.1 |########################################
  4406382.6 |########################################
  4444048.1 |
  4481713.6 |
  4519379.1 |
  4557044.5 |
  4594710.0 |
  4632375.5 |
  4670041.0 |
  4707706.5 |
  4745372.0 |
  4783037.5 |####################
  4820703.0 |
  4858368.5 |
  4896034.0 |
  4933699.5 |
  4971364.9 |
  5009030.4 |
  5046695.9 |
  5084361.4 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2755976.2-2954381.9 ns)
  2755976.2 |########################################
  2765896.5 |#############
  2775816.8 |
  2785737.1 |
  2795657.3 |
  2805577.6 |
  2815497.9 |
  2825418.2 |#############
  2835338.5 |
  2845258.8 |
  2855179.1 |
  2865099.3 |
  2875019.6 |
  2884939.9 |
  2894860.2 |
  2904780.5 |
  2914700.8 |
  2924621.0 |
  2934541.3 |
  2944461.6 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2741970.0-2979271.0 ns)
  2741970.0 |########################################
  2753835.0 |#############
  2765700.1 |
  2777565.1 |
  2789430.2 |
  2801295.2 |
  2813160.3 |
  2825025.4 |
  2836890.4 |
  2848755.5 |
  2860620.5 |
  2872485.5 |
  2884350.6 |#############
  2896215.6 |
  2908080.7 |
  2919945.8 |
  2931810.8 |
  2943675.9 |
  2955540.9 |
  2967406.0 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 3335.8-3651.4 ns)
   3335.8 |########################################
   3351.6 |
   3367.4 |
   3383.1 |########################################
   3398.9 |####################
   3414.7 |
   3430.5 |
   3446.3 |
   3462.1 |
   3477.8 |
   3493.6 |
   3509.4 |
   3525.2 |
   3541.0 |
   3556.8 |
   3572.5 |
   3588.3 |
   3604.1 |
   3619.9 |
   3635.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=3504.0% of algo (FFI overhead may distort results)
