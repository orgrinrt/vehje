# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 61448% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (3.35 us) leads abi_lifecycle_wideselect_held_handle (2.06 ms) by 61448%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 1098.5x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (3.68 ms) is 1098.5x the fastest (3.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.55)

abi_lifecycle_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (61448% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 61448% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1098.5x the fastest

Fastest abi_lifecycle_wideselect_null_entry (3.35 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (3.68 ms): 1098.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 3349.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1098.50x (fastest 3349.3 ns, slowest 3679255.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 3678878ns | 3682002ns | 3660445ns | 3676616ns | 3691488ns | +77.42% |
| abi_lifecycle_wideselect_fresh_per_column | 2085906ns | 2081684ns | 2070874ns | 2079778ns | 2102613ns | +0.60% |
| abi_lifecycle_wideselect_held_handle | 2073544ns | 2063907ns | 2043605ns | 2060752ns | 2107701ns | base |
| abi_lifecycle_wideselect_null_entry | 5685ns | 5564ns | 5494ns | 5547ns | 5989ns | -99.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 3676184ns | 3657682ns | 3688809ns | +77.51% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2083330ns | 2068584ns | 2099892ns | +0.60% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2071004ns | 2041228ns | 2104960ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 3415ns | 3309ns | 3573ns | -99.84% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 45911.2 | 3672673.7 | 3676183.7 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 39987.5 | 2083753.5 | 2083330.2 | 0 |
| abi_lifecycle_wideselect_held_handle | 41408.6 | 2071977.1 | 2071003.8 | n/a |
| abi_lifecycle_wideselect_null_entry | 27804.1 | 3513.9 | 3414.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.2% |
| abi_lifecycle_wideselect_null_entry | 0.001 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 3678878ns | 3678878ns | +77.42% |
| abi_lifecycle_wideselect_fresh_per_column | 2085906ns | 2085906ns | +0.60% |
| abi_lifecycle_wideselect_held_handle | 2073544ns | 2073544ns | base |
| abi_lifecycle_wideselect_null_entry | 5685ns | 5685ns | -99.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2061442ns | base | --- | [2046609, 2104960] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 3679256ns | +1611433.2ns (+78.2%) | [+1569024, +1635082]ns | [3660487, 3688809] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2079150ns | no significant difference | [-6887, +26030]ns | [2070949, 2099892] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_wideselect_null_entry | 3349ns | -2057952.1ns (-99.8%) | [-2101611, -2043204]ns | [3322, 3573] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2051989ns | +78.5% | +0.8% | -99.8% |
| 2 | 2061581ns | +77.4% | +1.0% | -99.8% |
| 3 | 2064546ns | +78.1% | +0.6% | -99.8% |
| 4 | 2061304ns | +79.0% | +0.9% | -99.8% |
| 5 | 2041228ns | +80.4% | +1.6% | -99.8% |
| 6 | 2145375ns | +71.9% | -1.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.455 | moderate+ |
| abi_lifecycle_wideselect_fresh_per_column | -0.170 | ok |
| abi_lifecycle_wideselect_held_handle | -0.232 | moderate- |
| abi_lifecycle_wideselect_null_entry | -0.548 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 11188270.0ns | 3676183.7ns | 304.3% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6286262.4ns | 2083330.2ns | 301.7% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6257264.7ns | 2071003.8ns | 302.1% | HIGH |
| abi_lifecycle_wideselect_null_entry | 120940.8ns | 3414.8ns | 3541.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 3657681.7-3688808.5 ns)
  3657681.7 |########################################
  3659238.0 |
  3660794.4 |
  3662350.7 |########################################
  3663907.1 |
  3665463.4 |
  3667019.8 |
  3668576.1 |
  3670132.4 |
  3671688.8 |
  3673245.1 |
  3674801.5 |########################################
  3676357.8 |
  3677914.2 |
  3679470.5 |
  3681026.8 |########################################
  3682583.2 |
  3684139.5 |
  3685695.9 |
  3687252.2 |########################################
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2068584.2-2099892.3 ns)
  2068584.2 |########################################
  2070149.6 |
  2071715.0 |
  2073280.4 |########################################
  2074845.8 |
  2076411.2 |########################################
  2077976.6 |
  2079542.0 |########################################
  2081107.4 |########################################
  2082672.8 |
  2084238.2 |
  2085803.7 |
  2087369.1 |
  2088934.5 |
  2090499.9 |
  2092065.3 |
  2093630.7 |
  2095196.1 |
  2096761.5 |
  2098326.9 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2041228.3-2104960.2 ns)
  2041228.3 |####################
  2044414.9 |
  2047601.5 |
  2050788.1 |####################
  2053974.7 |
  2057161.3 |
  2060347.9 |########################################
  2063534.5 |####################
  2066721.1 |
  2069907.7 |
  2073094.2 |
  2076280.8 |
  2079467.4 |
  2082654.0 |
  2085840.6 |
  2089027.2 |
  2092213.8 |
  2095400.4 |
  2098587.0 |
  2101773.6 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 3308.7-3572.9 ns)
   3308.7 |####################
   3321.9 |
   3335.1 |########################################
   3348.3 |####################
   3361.5 |
   3374.8 |
   3388.0 |
   3401.2 |
   3414.4 |
   3427.6 |
   3440.8 |
   3454.0 |
   3467.2 |
   3480.4 |
   3493.6 |####################
   3506.8 |
   3520.1 |
   3533.3 |
   3546.5 |
   3559.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=3630.2% of algo (FFI overhead may distort results)
