# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 54188% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (5.01 us) leads abi_lifecycle_madd_held_handle (2.72 ms) by 54188%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.71 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1205.9x slower than the field

abi_lifecycle_madd_fresh_per_batch (6.04 ms) is 1205.9x the fastest (5.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (54188% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 54188% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1205.9x the fastest

Fastest abi_lifecycle_madd_null_entry (5.01 us) to slowest abi_lifecycle_madd_fresh_per_batch (6.04 ms): 1205.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 5006.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1205.87x (fastest 5006.5 ns, slowest 6037187.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 6093029ns | 6040771ns | 6014276ns | 6033283ns | 6222023ns | +122.41% |
| abi_lifecycle_madd_fresh_per_column | 2832547ns | 2747398ns | 2729517ns | 2742110ns | 3019718ns | +3.39% |
| abi_lifecycle_madd_held_handle | 2739552ns | 2721046ns | 2711956ns | 2719567ns | 2783326ns | base |
| abi_lifecycle_madd_null_entry | 7337ns | 7325ns | 7017ns | 7234ns | 7653ns | -99.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 6089434ns | 6010909ns | 6218379ns | +122.56% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2829155ns | 2726618ns | 3015760ns | +3.40% | 0.000 |
| abi_lifecycle_madd_held_handle | 2736136ns | 2708962ns | 2779209ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 5016ns | 4794ns | 5239ns | -99.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 83542.8 | 6059124.5 | 6089434.2 | n/a |
| abi_lifecycle_madd_fresh_per_column | 70348.2 | 2845300.3 | 2829154.9 | n/a |
| abi_lifecycle_madd_held_handle | 66422.4 | 2733170.5 | 2736135.5 | n/a |
| abi_lifecycle_madd_null_entry | 28772.9 | 5150.4 | 5015.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.2% |
| abi_lifecycle_madd_null_entry | 0.000 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 6093029ns | 6093029ns | +122.41% |
| abi_lifecycle_madd_fresh_per_column | 2832547ns | 2832547ns | +3.39% |
| abi_lifecycle_madd_held_handle | 2739552ns | 2739552ns | base |
| abi_lifecycle_madd_null_entry | 7337ns | 7337ns | -99.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2717941ns | base | --- | [2711256, 2779209] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 6037187ns | +3305550.8ns (+121.6%) | [+3261488, +3492857]ns | [6012737, 6218379] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2744081ns | no significant difference | [-28990, +290238]ns | [2727624, 3015760] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_madd_null_entry | 5006ns | -2713055.0ns (-99.8%) | [-2773995, -2706310]ns | [4802, 5239] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2720977ns | +121.6% | +0.3% | -99.8% |
| 2 | 2708962ns | +121.9% | +0.7% | -99.8% |
| 3 | 2820925ns | +114.3% | -2.3% | -99.8% |
| 4 | 2713550ns | +134.0% | +15.0% | -99.8% |
| 5 | 2737493ns | +122.4% | +6.4% | -99.8% |
| 6 | 2714906ns | +121.5% | +0.7% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.045 | ok |
| abi_lifecycle_madd_fresh_per_column | 0.098 | ok |
| abi_lifecycle_madd_held_handle | -0.424 | moderate- |
| abi_lifecycle_madd_null_entry | -0.163 | ok |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 18303068.7ns | 6089434.2ns | 300.6% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8626774.9ns | 2829154.9ns | 304.9% | HIGH |
| abi_lifecycle_madd_held_handle | 8270736.5ns | 2736135.5ns | 302.3% | HIGH |
| abi_lifecycle_madd_null_entry | 126450.2ns | 5015.8ns | 2521.0% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 6010908.8-6218378.8 ns)
  6010908.8 |########################################
  6021282.3 |####################
  6031655.8 |
  6042029.3 |####################
  6052402.8 |
  6062776.3 |
  6073149.8 |
  6083523.3 |####################
  6093896.8 |
  6104270.3 |
  6114643.8 |
  6125017.3 |
  6135390.8 |
  6145764.3 |
  6156137.8 |
  6166511.3 |
  6176884.8 |
  6187258.3 |
  6197631.8 |
  6208005.3 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2726617.9-3015760.0 ns)
  2726617.9 |########################################
  2741075.0 |#############
  2755532.1 |
  2769989.2 |
  2784446.3 |
  2798903.4 |
  2813360.5 |
  2827817.6 |
  2842274.7 |
  2856731.8 |
  2871189.0 |
  2885646.1 |
  2900103.2 |#############
  2914560.3 |
  2929017.4 |
  2943474.5 |
  2957931.6 |
  2972388.7 |
  2986845.8 |
  3001302.9 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2708962.5-2779209.0 ns)
  2708962.5 |####################
  2712474.8 |########################################
  2715987.1 |
  2719499.5 |####################
  2723011.8 |
  2726524.1 |
  2730036.4 |
  2733548.8 |
  2737061.1 |####################
  2740573.4 |
  2744085.7 |
  2747598.0 |
  2751110.4 |
  2754622.7 |
  2758135.0 |
  2761647.3 |
  2765159.7 |
  2768672.0 |
  2772184.3 |
  2775696.6 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 4794.2-5239.4 ns)
   4794.2 |########################################
   4816.5 |
   4838.7 |
   4861.0 |
   4883.2 |
   4905.5 |
   4927.8 |
   4950.0 |####################
   4972.3 |
   4994.5 |
   5016.8 |
   5039.1 |####################
   5061.3 |
   5083.6 |####################
   5105.8 |
   5128.1 |
   5150.4 |
   5172.6 |
   5194.9 |
   5217.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=2479.5% of algo (FFI overhead may distort results)
