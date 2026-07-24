# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 81242% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (2.52 us) leads abi_lifecycle_wideselect_held_handle (2.05 ms) by 81242%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.04 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 835.4x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (2.10 ms) is 835.4x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (81242% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 81242% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 835.4x the fastest

Fastest abi_lifecycle_wideselect_null_entry (2.52 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (2.10 ms): 835.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 2516.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 835.38x (fastest 2516.9 ns, slowest 2102571.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2105027ns | 2105221ns | 2098087ns | 2102971ns | 2111582ns | +2.70% |
| abi_lifecycle_wideselect_fresh_per_column | 2068958ns | 2065978ns | 2063976ns | 2065794ns | 2076194ns | +0.94% |
| abi_lifecycle_wideselect_held_handle | 2049692ns | 2049699ns | 2040662ns | 2048145ns | 2056529ns | base |
| abi_lifecycle_wideselect_null_entry | 4778ns | 4754ns | 4563ns | 4728ns | 4961ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2102428ns | 2095468ns | 2109012ns | +2.70% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2066415ns | 2061290ns | 2073669ns | +0.94% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2047180ns | 2038068ns | 2053947ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 2506ns | 2382ns | 2596ns | -99.88% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 39694.9 | 2101862.4 | 2102427.9 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 37770.2 | 2067244.9 | 2066415.0 | 0 |
| abi_lifecycle_wideselect_held_handle | 38507.7 | 2046772.7 | 2047180.5 | n/a |
| abi_lifecycle_wideselect_null_entry | 27578.4 | 2776.0 | 2506.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.1% |
| abi_lifecycle_wideselect_null_entry | 0.025 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2105027ns | 2105027ns | +2.70% |
| abi_lifecycle_wideselect_fresh_per_column | 2068958ns | 2068958ns | +0.94% |
| abi_lifecycle_wideselect_held_handle | 2049692ns | 2049692ns | base |
| abi_lifecycle_wideselect_null_entry | 4778ns | 4778ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2047288ns | base | --- | [2040306, 2053947] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2102571ns | +53324.2ns (+2.6%) | [+46208, +66210]ns | [2095701, 2109012] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2063471ns | +19550.4ns (+1.0%) | [+10006, +28147]ns | [2062105, 2073669] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_wideselect_null_entry | 2517ns | -2044819.8ns (-99.9%) | [-2051494, -2037709]ns | [2405, 2596] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2043089ns | +2.6% | +1.0% | -99.9% |
| 2 | 2052550ns | +2.1% | +0.6% | -99.9% |
| 3 | 2038068ns | +3.5% | +1.2% | -99.9% |
| 4 | 2042544ns | +3.0% | +0.9% | -99.9% |
| 5 | 2055345ns | +2.6% | +0.4% | -99.9% |
| 6 | 2051488ns | +2.4% | +1.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.116 | ok |
| abi_lifecycle_wideselect_fresh_per_column | -0.007 | ok |
| abi_lifecycle_wideselect_held_handle | -0.133 | ok |
| abi_lifecycle_wideselect_null_entry | 0.039 | ok |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 6350911.5ns | 2102427.9ns | 302.1% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6248353.1ns | 2066415.0ns | 302.4% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6182465.8ns | 2047180.5ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_null_entry | 113451.0ns | 2506.1ns | 4527.0% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2095467.9-2109011.7 ns)
  2095467.9 |########################################
  2096145.1 |
  2096822.3 |
  2097499.5 |
  2098176.7 |
  2098853.9 |
  2099531.0 |
  2100208.2 |####################
  2100885.4 |
  2101562.6 |
  2102239.8 |
  2102917.0 |
  2103594.2 |
  2104271.4 |####################
  2104948.6 |
  2105625.8 |
  2106302.9 |
  2106980.1 |
  2107657.3 |
  2108334.5 |####################
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2061290.4-2073668.8 ns)
  2061290.4 |####################
  2061909.3 |
  2062528.2 |####################
  2063147.2 |########################################
  2063766.1 |
  2064385.0 |####################
  2065003.9 |
  2065622.8 |
  2066241.7 |
  2066860.7 |
  2067479.6 |
  2068098.5 |
  2068717.4 |
  2069336.3 |
  2069955.2 |
  2070574.2 |
  2071193.1 |
  2071812.0 |
  2072430.9 |
  2073049.8 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2038067.5-2053947.3 ns)
  2038067.5 |########################################
  2038861.5 |
  2039655.5 |
  2040449.5 |
  2041243.5 |
  2042037.4 |########################################
  2042831.4 |########################################
  2043625.4 |
  2044419.4 |
  2045213.4 |
  2046007.4 |
  2046801.4 |
  2047595.4 |
  2048389.4 |
  2049183.4 |
  2049977.4 |
  2050771.3 |########################################
  2051565.3 |
  2052359.3 |########################################
  2053153.3 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 2382.1-2596.4 ns)
   2382.1 |####################
   2392.8 |
   2403.5 |
   2414.3 |
   2425.0 |####################
   2435.7 |
   2446.4 |
   2457.1 |
   2467.8 |
   2478.6 |
   2489.3 |
   2500.0 |####################
   2510.7 |
   2521.4 |########################################
   2532.1 |
   2542.9 |
   2553.6 |
   2564.3 |
   2575.0 |
   2585.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=4477.7% of algo (FFI overhead may distort results)
