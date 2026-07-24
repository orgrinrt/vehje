# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 68187% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (3.14 us) leads abi_lifecycle_scatter_held_handle (2.14 ms) by 68187%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 805.8x slower than the field

abi_lifecycle_scatter_fresh_per_batch (2.53 ms) is 805.8x the fastest (3.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_scatter_null_entry shows warm-up / thermal drift (autocorr +0.52)

abi_lifecycle_scatter_null_entry's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (68187% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 68187% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 805.8x the fastest

Fastest abi_lifecycle_scatter_null_entry (3.14 us) to slowest abi_lifecycle_scatter_fresh_per_batch (2.53 ms): 805.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 3139.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 805.78x (fastest 3139.2 ns, slowest 2529470.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2595044ns | 2531996ns | 2528758ns | 2531441ns | 2723590ns | +20.38% |
| abi_lifecycle_scatter_fresh_per_column | 2152900ns | 2152240ns | 2144000ns | 2149859ns | 2161911ns | -0.13% |
| abi_lifecycle_scatter_held_handle | 2155735ns | 2146199ns | 2124512ns | 2139540ns | 2195640ns | base |
| abi_lifecycle_scatter_null_entry | 5402ns | 5445ns | 5166ns | 5380ns | 5553ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2592273ns | 2526125ns | 2720421ns | +20.40% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2150273ns | 2141654ns | 2159299ns | -0.13% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2153024ns | 2121993ns | 2192582ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 3105ns | 2969ns | 3199ns | -99.86% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 46640.7 | 2586511.4 | 2592272.6 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 37752.4 | 2150290.3 | 2150273.3 | n/a |
| abi_lifecycle_scatter_held_handle | 57166.2 | 2169372.1 | 2153024.0 | n/a |
| abi_lifecycle_scatter_null_entry | 26787.2 | 3153.7 | 3105.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.1% |
| abi_lifecycle_scatter_null_entry | 0.003 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2595044ns | 2595044ns | +20.38% |
| abi_lifecycle_scatter_fresh_per_column | 2152900ns | 2152900ns | -0.13% |
| abi_lifecycle_scatter_held_handle | 2155735ns | 2155735ns | base |
| abi_lifecycle_scatter_null_entry | 5402ns | 5402ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2143629ns | base | --- | [2122861, 2192582] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2529471ns | +399585.0ns (+18.6%) | [+375031, +543130]ns | [2526925, 2720421] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2149569ns | no significant difference | [-37384, +23609]ns | [2141952, 2159299] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_scatter_null_entry | 3139ns | -2140529.0ns (-99.9%) | [-2189435, -2119792]ns | [2978, 3199] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2121993ns | +19.3% | +1.4% | -99.9% |
| 2 | 2151715ns | +17.5% | -0.2% | -99.8% |
| 3 | 2230854ns | +30.3% | -3.2% | -99.9% |
| 4 | 2154310ns | +17.3% | +0.2% | -99.9% |
| 5 | 2135542ns | +18.3% | +0.3% | -99.9% |
| 6 | 2123730ns | +19.3% | +0.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | -0.238 | moderate- |
| abi_lifecycle_scatter_fresh_per_column | 0.183 | ok |
| abi_lifecycle_scatter_held_handle | 0.065 | ok |
| abi_lifecycle_scatter_null_entry | 0.518 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 2/6, lost 4/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 7868537.3ns | 2592272.6ns | 303.5% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6491676.6ns | 2150273.3ns | 301.9% | HIGH |
| abi_lifecycle_scatter_held_handle | 6591227.6ns | 2153024.0ns | 306.1% | HIGH |
| abi_lifecycle_scatter_null_entry | 118843.7ns | 3105.3ns | 3827.2% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2526124.6-2720421.5 ns)
  2526124.6 |########################################
  2535839.4 |
  2545554.3 |
  2555269.1 |
  2564984.0 |
  2574698.8 |
  2584413.7 |
  2594128.5 |
  2603843.3 |
  2613558.2 |
  2623273.0 |
  2632987.9 |
  2642702.7 |
  2652417.6 |
  2662132.4 |
  2671847.2 |
  2681562.1 |
  2691276.9 |
  2700991.8 |
  2710706.6 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2141654.2-2159299.0 ns)
  2141654.2 |########################################
  2142536.4 |
  2143418.7 |
  2144300.9 |
  2145183.2 |
  2146065.4 |
  2146947.6 |
  2147829.9 |####################
  2148712.1 |
  2149594.4 |
  2150476.6 |####################
  2151358.8 |
  2152241.1 |
  2153123.3 |
  2154005.6 |
  2154887.8 |
  2155770.0 |
  2156652.3 |
  2157534.5 |
  2158416.8 |####################
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2121992.9-2192582.1 ns)
  2121992.9 |########################################
  2125522.4 |
  2129051.8 |
  2132581.3 |####################
  2136110.7 |
  2139640.2 |
  2143169.7 |
  2146699.1 |
  2150228.6 |####################
  2153758.0 |####################
  2157287.5 |
  2160817.0 |
  2164346.4 |
  2167875.9 |
  2171405.3 |
  2174934.8 |
  2178464.3 |
  2181993.7 |
  2185523.2 |
  2189052.6 |
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 2969.2-3198.9 ns)
   2969.2 |########################################
   2980.7 |########################################
   2992.2 |
   3003.7 |
   3015.1 |
   3026.6 |
   3038.1 |
   3049.6 |
   3061.1 |
   3072.6 |
   3084.1 |
   3095.6 |
   3107.0 |
   3118.5 |########################################
   3130.0 |
   3141.5 |########################################
   3153.0 |
   3164.5 |########################################
   3176.0 |
   3187.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **abi_lifecycle_scatter_null_entry**: bridge=3784.3% of algo (FFI overhead may distort results)
