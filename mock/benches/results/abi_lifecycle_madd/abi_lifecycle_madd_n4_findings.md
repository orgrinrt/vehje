# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 69802% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (3.91 us) leads abi_lifecycle_madd_held_handle (2.73 ms) by 69802%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.73 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 913.5x slower than the field

abi_lifecycle_madd_fresh_per_batch (3.57 ms) is 913.5x the fastest (3.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (69802% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 69802% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 913.5x the fastest

Fastest abi_lifecycle_madd_null_entry (3.91 us) to slowest abi_lifecycle_madd_fresh_per_batch (3.57 ms): 913.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 3908.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 913.52x (fastest 3908.3 ns, slowest 3570338.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 3626093ns | 3573907ns | 3540749ns | 3564836ns | 3760652ns | +32.62% |
| abi_lifecycle_madd_fresh_per_column | 2884266ns | 2752570ns | 2751285ns | 2752247ns | 3148785ns | +5.49% |
| abi_lifecycle_madd_held_handle | 2734116ns | 2735272ns | 2727767ns | 2734353ns | 2736935ns | base |
| abi_lifecycle_madd_null_entry | 6144ns | 6113ns | 6062ns | 6100ns | 6250ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 3622580ns | 3537825ns | 3756639ns | +32.65% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2881051ns | 2748392ns | 3145266ns | +5.50% | 0.000 |
| abi_lifecycle_madd_held_handle | 2730902ns | 2724643ns | 2733752ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 3920ns | 3880ns | 3970ns | -99.86% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 76881.7 | 3587618.8 | 3622580.3 | n/a |
| abi_lifecycle_madd_fresh_per_column | 78269.8 | 2851955.7 | 2881051.5 | n/a |
| abi_lifecycle_madd_held_handle | 63094.5 | 2729427.9 | 2730902.4 | n/a |
| abi_lifecycle_madd_null_entry | 26832.3 | 4043.3 | 3920.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.001 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 3626093ns | 3626093ns | +32.62% |
| abi_lifecycle_madd_fresh_per_column | 2884266ns | 2884266ns | +5.49% |
| abi_lifecycle_madd_held_handle | 2734116ns | 2734116ns | base |
| abi_lifecycle_madd_null_entry | 6144ns | 6144ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2732031ns | base | --- | [2726924, 2733752] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 3570338ns | +838831.5ns (+30.7%) | [+807689, +1028514]ns | [3540764, 3756639] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2749478ns | +18649.0ns (+0.7%) | [+14658, +417141]ns | [2748410, 3145266] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_null_entry | 3908ns | -2728148.5ns (-99.9%) | [-2729803, -2722995]ns | [3882, 3970] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2729204ns | +30.8% | +0.7% | -99.9% |
| 2 | 2731608ns | +43.0% | +29.6% | -99.9% |
| 3 | 2733809ns | +30.6% | +0.5% | -99.9% |
| 4 | 2724643ns | +32.4% | +0.9% | -99.9% |
| 5 | 2733696ns | +29.4% | +0.5% | -99.9% |
| 6 | 2732454ns | +29.7% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.210 | moderate- |
| abi_lifecycle_madd_fresh_per_column | -0.235 | moderate- |
| abi_lifecycle_madd_held_handle | -0.498 | moderate- |
| abi_lifecycle_madd_null_entry | -0.454 | moderate- |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 10974422.3ns | 3622580.3ns | 302.9% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8564220.0ns | 2881051.5ns | 297.3% | HIGH |
| abi_lifecycle_madd_held_handle | 8247785.8ns | 2730902.4ns | 302.0% | HIGH |
| abi_lifecycle_madd_null_entry | 121212.8ns | 3920.3ns | 3091.9% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 3537825.0-3756639.4 ns)
  3537825.0 |########################################
  3548765.7 |
  3559706.4 |########################################
  3570647.2 |
  3581587.9 |
  3592528.6 |
  3603469.3 |####################
  3614410.0 |
  3625350.8 |
  3636291.5 |
  3647232.2 |
  3658172.9 |
  3669113.6 |
  3680054.4 |
  3690995.1 |
  3701935.8 |
  3712876.5 |
  3723817.2 |
  3734758.0 |
  3745698.7 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2748392.1-3145266.5 ns)
  2748392.1 |########################################
  2768235.8 |
  2788079.5 |
  2807923.3 |
  2827767.0 |
  2847610.7 |
  2867454.4 |
  2887298.1 |
  2907141.8 |
  2926985.6 |
  2946829.3 |
  2966673.0 |
  2986516.7 |
  3006360.4 |
  3026204.1 |
  3046047.9 |
  3065891.6 |
  3085735.3 |
  3105579.0 |
  3125422.7 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2724643.3-2733752.5 ns)
  2724643.3 |########################################
  2725098.8 |
  2725554.2 |
  2726009.7 |
  2726465.1 |
  2726920.6 |
  2727376.1 |
  2727831.5 |
  2728287.0 |
  2728742.4 |
  2729197.9 |########################################
  2729653.4 |
  2730108.8 |
  2730564.3 |
  2731019.7 |
  2731475.2 |########################################
  2731930.7 |
  2732386.1 |########################################
  2732841.6 |
  2733297.0 |########################################
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 3879.6-3970.2 ns)
   3879.6 |########################################
   3884.1 |########################################
   3888.7 |
   3893.2 |
   3897.7 |
   3902.2 |########################################
   3906.8 |########################################
   3911.3 |
   3915.8 |
   3920.4 |
   3924.9 |
   3929.4 |
   3934.0 |
   3938.5 |
   3943.0 |
   3947.5 |########################################
   3952.1 |
   3956.6 |
   3961.1 |
   3965.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=3109.5% of algo (FFI overhead may distort results)
