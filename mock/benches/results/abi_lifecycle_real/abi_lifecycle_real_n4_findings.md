# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 53436% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (4.02 us) leads abi_lifecycle_real_held_handle (2.15 ms) by 53436%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.15 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 729.6x slower than the field

abi_lifecycle_real_fresh_per_batch (2.93 ms) is 729.6x the fastest (4.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (53436% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 53436% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 729.6x the fastest

Fastest abi_lifecycle_real_null_entry (4.02 us) to slowest abi_lifecycle_real_fresh_per_batch (2.93 ms): 729.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 4015.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 729.61x (fastest 4015.4 ns, slowest 2929666.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2930877ns | 2932350ns | 2912966ns | 2929170ns | 2942393ns | +35.57% |
| abi_lifecycle_real_fresh_per_column | 2168066ns | 2165674ns | 2159305ns | 2164855ns | 2177264ns | +0.29% |
| abi_lifecycle_real_held_handle | 2161850ns | 2152355ns | 2142632ns | 2151315ns | 2187261ns | base |
| abi_lifecycle_real_null_entry | 6295ns | 6301ns | 6085ns | 6264ns | 6448ns | -99.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2928192ns | 2910438ns | 2939564ns | +35.62% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2165437ns | 2156848ns | 2174471ns | +0.29% | 0.000 |
| abi_lifecycle_real_held_handle | 2159106ns | 2140152ns | 2184151ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 4025ns | 3905ns | 4138ns | -99.81% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 45095.6 | 2927711.8 | 2928192.5 | n/a |
| abi_lifecycle_real_fresh_per_column | 41516.9 | 2165917.9 | 2165437.3 | 0 |
| abi_lifecycle_real_held_handle | 43870.3 | 2158762.9 | 2159105.6 | n/a |
| abi_lifecycle_real_null_entry | 27766.8 | 4162.5 | 4025.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_real_held_handle | 0.000 | 0.2% |
| abi_lifecycle_real_null_entry | 0.001 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2930877ns | 2930877ns | +35.57% |
| abi_lifecycle_real_fresh_per_column | 2168066ns | 2168066ns | +0.29% |
| abi_lifecycle_real_held_handle | 2161850ns | 2161850ns | base |
| abi_lifecycle_real_null_entry | 6295ns | 6295ns | -99.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2149700ns | base | --- | [2143465, 2184151] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2929666ns | +771700.0ns (+35.9%) | [+753031, +782529]ns | [2915348, 2939564] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2163012ns | no significant difference | [-25322, +28422]ns | [2158829, 2174471] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_real_null_entry | 4015ns | -2145747.7ns (-99.8%) | [-2180066, -2139428]ns | [3922, 4138] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2140152ns | +36.0% | +1.2% | -99.8% |
| 2 | 2152258ns | +36.2% | +0.8% | -99.8% |
| 3 | 2147143ns | +36.0% | +1.5% | -99.8% |
| 4 | 2146778ns | +36.6% | +0.7% | -99.8% |
| 5 | 2206056ns | +33.6% | -2.2% | -99.8% |
| 6 | 2162247ns | +35.4% | -0.1% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.103 | ok |
| abi_lifecycle_real_fresh_per_column | 0.230 | moderate+ |
| abi_lifecycle_real_held_handle | -0.025 | ok |
| abi_lifecycle_real_null_entry | -0.037 | ok |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 1/6, lost 4/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 8843645.4ns | 2928192.5ns | 302.0% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6541470.8ns | 2165437.3ns | 302.1% | HIGH |
| abi_lifecycle_real_held_handle | 6522835.9ns | 2159105.6ns | 302.1% | HIGH |
| abi_lifecycle_real_null_entry | 122375.7ns | 4025.0ns | 3040.4% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2910438.3-2939563.5 ns)
  2910438.3 |####################
  2911894.6 |
  2913350.8 |
  2914807.1 |
  2916263.3 |
  2917719.6 |
  2919175.9 |####################
  2920632.1 |
  2922088.4 |
  2923544.6 |
  2925000.9 |
  2926457.2 |####################
  2927913.4 |
  2929369.7 |
  2930825.9 |########################################
  2932282.2 |
  2933738.5 |
  2935194.7 |
  2936651.0 |
  2938107.2 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2156847.9-2174470.6 ns)
  2156847.9 |####################
  2157729.0 |
  2158610.2 |
  2159491.3 |
  2160372.4 |########################################
  2161253.6 |
  2162134.7 |
  2163015.9 |
  2163897.0 |
  2164778.1 |####################
  2165659.3 |
  2166540.4 |
  2167421.5 |
  2168302.7 |
  2169183.8 |####################
  2170065.0 |
  2170946.1 |
  2171827.2 |
  2172708.4 |
  2173589.5 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2140151.7-2184151.2 ns)
  2140151.7 |####################
  2142351.7 |
  2144551.7 |
  2146751.6 |########################################
  2148951.6 |
  2151151.6 |####################
  2153351.6 |
  2155551.5 |
  2157751.5 |
  2159951.5 |
  2162151.5 |####################
  2164351.5 |
  2166551.4 |
  2168751.4 |
  2170951.4 |
  2173151.4 |
  2175351.3 |
  2177551.3 |
  2179751.3 |
  2181951.3 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 3905.4-4138.1 ns)
   3905.4 |########################################
   3917.0 |
   3928.7 |########################################
   3940.3 |
   3951.9 |
   3963.6 |########################################
   3975.2 |
   3986.8 |
   3998.5 |
   4010.1 |
   4021.8 |
   4033.4 |
   4045.0 |
   4056.7 |########################################
   4068.3 |
   4079.9 |
   4091.6 |
   4103.2 |########################################
   4114.8 |
   4126.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=3037.9% of algo (FFI overhead may distort results)
