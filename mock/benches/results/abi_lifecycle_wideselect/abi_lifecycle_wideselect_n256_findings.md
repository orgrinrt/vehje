# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 62869% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (3.26 us) leads abi_lifecycle_wideselect_held_handle (2.05 ms) by 62869%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_column is an outlier: 633.4x slower than the field

abi_lifecycle_wideselect_fresh_per_column (2.07 ms) is 633.4x the fastest (3.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_batch, abi_lifecycle_wideselect_fresh_per_column} (62869% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_batch, abi_lifecycle_wideselect_fresh_per_column} with a 62869% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 633.4x the fastest

Fastest abi_lifecycle_wideselect_null_entry (3.26 us) to slowest abi_lifecycle_wideselect_fresh_per_column (2.07 ms): 633.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 3261.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 633.41x (fastest 3261.2 ns, slowest 2065695.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2070415ns | 2068092ns | 2064241ns | 2067752ns | 2077497ns | +0.29% |
| abi_lifecycle_wideselect_fresh_per_column | 2068806ns | 2068216ns | 2050377ns | 2066274ns | 2081819ns | +0.22% |
| abi_lifecycle_wideselect_held_handle | 2064357ns | 2056094ns | 2036900ns | 2053204ns | 2094815ns | base |
| abi_lifecycle_wideselect_null_entry | 5614ns | 5658ns | 5260ns | 5601ns | 5810ns | -99.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2067880ns | 2061772ns | 2074853ns | +0.30% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2066213ns | 2048078ns | 2079126ns | +0.22% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2061726ns | 2034337ns | 2091990ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 3245ns | 3046ns | 3350ns | -99.84% | 0.079 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 38083.8 | 2068472.4 | 2067879.7 | 1 |
| abi_lifecycle_wideselect_fresh_per_column | 38828.2 | 2066854.0 | 2066212.7 | 1 |
| abi_lifecycle_wideselect_held_handle | 42070.7 | 2057027.4 | 2061725.8 | n/a |
| abi_lifecycle_wideselect_null_entry | 28829.5 | 3258.2 | 3245.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.084 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.1% |
| abi_lifecycle_wideselect_null_entry | 0.078 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2070415ns | 2070415ns | +0.29% |
| abi_lifecycle_wideselect_fresh_per_column | 2068806ns | 2068806ns | +0.22% |
| abi_lifecycle_wideselect_held_handle | 2064357ns | 2064357ns | base |
| abi_lifecycle_wideselect_null_entry | 5614ns | 5614ns | -99.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2053569ns | base | --- | [2039618, 2091990] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2065602ns | no significant difference | [-28283, +32604]ns | [2063185, 2074853] | no | 0.3281 | 0.2188 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2065695ns | no significant difference | [-27866, +28901]ns | [2053817, 2079126] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_wideselect_null_entry | 3261ns | -2050268.1ns (-99.8%) | [-2088708, -2036465]ns | [3125, 3350] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2053202ns | +0.9% | +1.2% | -99.8% |
| 2 | 2053936ns | +0.5% | +0.3% | -99.8% |
| 3 | 2057288ns | +0.2% | -0.4% | -99.8% |
| 4 | 2126692ns | -2.9% | -2.2% | -99.8% |
| 5 | 2044899ns | +1.7% | +0.9% | -99.9% |
| 6 | 2034337ns | +1.5% | +1.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.139 | ok |
| abi_lifecycle_wideselect_fresh_per_column | -0.340 | moderate- |
| abi_lifecycle_wideselect_held_handle | -0.152 | ok |
| abi_lifecycle_wideselect_null_entry | -0.463 | moderate- |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 2/6, lost 4/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 6242636.8ns | 2067879.7ns | 301.9% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6242028.4ns | 2066212.7ns | 302.1% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6253098.2ns | 2061725.8ns | 303.3% | HIGH |
| abi_lifecycle_wideselect_null_entry | 122497.7ns | 3245.4ns | 3774.5% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2061772.5-2074852.7 ns)
  2061772.5 |####################
  2062426.5 |
  2063080.5 |
  2063734.5 |
  2064388.5 |####################
  2065042.6 |########################################
  2065696.6 |
  2066350.6 |
  2067004.6 |
  2067658.6 |
  2068312.6 |
  2068966.6 |
  2069620.6 |
  2070274.6 |####################
  2070928.6 |
  2071582.6 |
  2072236.7 |
  2072890.7 |
  2073544.7 |
  2074198.7 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2048077.5-2079125.6 ns)
  2048077.5 |########################################
  2049629.9 |
  2051182.3 |
  2052734.7 |
  2054287.1 |
  2055839.5 |
  2057391.9 |
  2058944.3 |########################################
  2060496.7 |
  2062049.1 |
  2063601.6 |########################################
  2065154.0 |
  2066706.4 |########################################
  2068258.8 |
  2069811.2 |
  2071363.6 |
  2072916.0 |
  2074468.4 |
  2076020.8 |
  2077573.2 |########################################
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2034337.1-2091990.2 ns)
  2034337.1 |####################
  2037219.8 |
  2040102.4 |
  2042985.1 |####################
  2045867.7 |
  2048750.4 |
  2051633.0 |########################################
  2054515.7 |####################
  2057398.3 |
  2060281.0 |
  2063163.7 |
  2066046.3 |
  2068929.0 |
  2071811.6 |
  2074694.3 |
  2077576.9 |
  2080459.6 |
  2083342.2 |
  2086224.9 |
  2089107.5 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 3045.8-3350.0 ns)
   3045.8 |####################
   3061.0 |
   3076.2 |
   3091.4 |
   3106.6 |
   3121.9 |
   3137.1 |
   3152.3 |
   3167.5 |
   3182.7 |
   3197.9 |####################
   3213.1 |
   3228.3 |
   3243.5 |
   3258.7 |########################################
   3273.9 |
   3289.2 |
   3304.4 |
   3319.6 |
   3334.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=3755.4% of algo (FFI overhead may distort results)
