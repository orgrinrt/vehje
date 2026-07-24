# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 99009% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (2.74 us) leads abi_lifecycle_madd_held_handle (2.72 ms) by 99009%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.72 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1003.5x slower than the field

abi_lifecycle_madd_fresh_per_batch (2.75 ms) is 1003.5x the fastest (2.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (99009% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 99009% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1003.5x the fastest

Fastest abi_lifecycle_madd_null_entry (2.74 us) to slowest abi_lifecycle_madd_fresh_per_batch (2.75 ms): 1003.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 2744.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1003.52x (fastest 2744.3 ns, slowest 2754010.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2757330ns | 2757295ns | 2750103ns | 2755241ns | 2764079ns | +1.19% |
| abi_lifecycle_madd_fresh_per_column | 2747786ns | 2745087ns | 2740371ns | 2744019ns | 2757143ns | +0.83% |
| abi_lifecycle_madd_held_handle | 2725037ns | 2723093ns | 2716945ns | 2721997ns | 2733644ns | base |
| abi_lifecycle_madd_null_entry | 5058ns | 5082ns | 4818ns | 5041ns | 5204ns | -99.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2754087ns | 2747088ns | 2760673ns | +1.19% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2744535ns | 2737301ns | 2753512ns | +0.84% | 0.000 |
| abi_lifecycle_madd_held_handle | 2721694ns | 2713862ns | 2729945ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 2752ns | 2641ns | 2849ns | -99.90% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 66473.8 | 2761849.7 | 2754087.3 | 0 |
| abi_lifecycle_madd_fresh_per_column | 66906.2 | 2750850.3 | 2744535.4 | 0 |
| abi_lifecycle_madd_held_handle | 67031.7 | 2720954.3 | 2721694.2 | n/a |
| abi_lifecycle_madd_null_entry | 27874.9 | 2792.5 | 2752.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.047 | 96.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2757330ns | 2757330ns | +1.19% |
| abi_lifecycle_madd_fresh_per_column | 2747786ns | 2747786ns | +0.83% |
| abi_lifecycle_madd_held_handle | 2725037ns | 2725037ns | base |
| abi_lifecycle_madd_null_entry | 5058ns | 5058ns | -99.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2719886ns | base | --- | [2715252, 2729945] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 2754010ns | +31062.1ns (+1.1%) | [+24998, +41120]ns | [2747579, 2760673] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2742071ns | +23078.8ns (+0.8%) | [+17672, +27773]ns | [2738023, 2753512] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_null_entry | 2744ns | -2717139.1ns (-99.9%) | [-2727282, -2712406]ns | [2663, 2849] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2713862ns | +1.7% | +1.0% | -99.9% |
| 2 | 2718836ns | +1.4% | +0.7% | -99.9% |
| 3 | 2716642ns | +1.2% | +1.0% | -99.9% |
| 4 | 2720936ns | +1.0% | +0.6% | -99.9% |
| 5 | 2731482ns | +1.1% | +1.0% | -99.9% |
| 6 | 2728407ns | +0.9% | +0.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.169 | ok |
| abi_lifecycle_madd_fresh_per_column | -0.100 | ok |
| abi_lifecycle_madd_held_handle | 0.418 | moderate+ |
| abi_lifecycle_madd_null_entry | 0.180 | ok |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 8346548.0ns | 2754087.3ns | 303.1% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8315031.1ns | 2744535.4ns | 303.0% | HIGH |
| abi_lifecycle_madd_held_handle | 8227406.8ns | 2721694.2ns | 302.3% | HIGH |
| abi_lifecycle_madd_null_entry | 120101.0ns | 2752.0ns | 4364.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 2747087.9-2760673.2 ns)
  2747087.9 |########################################
  2747767.2 |########################################
  2748446.4 |
  2749125.7 |
  2749805.0 |
  2750484.2 |
  2751163.5 |
  2751842.7 |########################################
  2752522.0 |
  2753201.3 |
  2753880.5 |
  2754559.8 |
  2755239.1 |########################################
  2755918.3 |
  2756597.6 |
  2757276.8 |
  2757956.1 |
  2758635.4 |########################################
  2759314.6 |
  2759993.9 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2737300.8-2753512.1 ns)
  2737300.8 |########################################
  2738111.4 |########################################
  2738921.9 |
  2739732.5 |########################################
  2740543.1 |
  2741353.6 |
  2742164.2 |
  2742974.8 |
  2743785.3 |########################################
  2744595.9 |
  2745406.5 |
  2746217.0 |
  2747027.6 |########################################
  2747838.1 |
  2748648.7 |
  2749459.3 |
  2750269.8 |
  2751080.4 |
  2751891.0 |
  2752701.5 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2713862.1-2729944.6 ns)
  2713862.1 |########################################
  2714666.2 |
  2715470.4 |
  2716274.5 |########################################
  2717078.6 |
  2717882.7 |
  2718686.9 |########################################
  2719491.0 |
  2720295.1 |########################################
  2721099.2 |
  2721903.4 |
  2722707.5 |
  2723511.6 |
  2724315.7 |
  2725119.9 |
  2725924.0 |
  2726728.1 |
  2727532.2 |
  2728336.4 |########################################
  2729140.5 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 2640.8-2848.8 ns)
   2640.8 |########################################
   2651.2 |
   2661.6 |
   2672.0 |
   2682.4 |########################################
   2692.8 |
   2703.2 |
   2713.6 |########################################
   2724.0 |
   2734.4 |
   2744.8 |
   2755.2 |
   2765.6 |########################################
   2776.0 |########################################
   2786.4 |
   2796.8 |
   2807.2 |
   2817.6 |
   2828.0 |
   2838.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=4390.6% of algo (FFI overhead may distort results)
