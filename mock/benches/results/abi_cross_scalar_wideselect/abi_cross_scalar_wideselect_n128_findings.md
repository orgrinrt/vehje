# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 76086% faster than the next best (abi_cross_scalar_wideselect_inproc_direct)

abi_cross_scalar_wideselect_null_entry (2.82 us) leads abi_cross_scalar_wideselect_inproc_direct (2.15 ms) by 76086%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.14 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_inproc_fnptr is an outlier: 770.9x slower than the field

abi_cross_scalar_wideselect_inproc_fnptr (2.17 ms) is 770.9x the fastest (2.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr} (76086% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr} with a 76086% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 770.9x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (2.82 us) to slowest abi_cross_scalar_wideselect_inproc_fnptr (2.17 ms): 770.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 2816.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 770.94x (fastest 2816.1 ns, slowest 2170992.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2186170ns | 2172773ns | 2112857ns | 2170014ns | 2247062ns | +1.12% |
| abi_cross_scalar_wideselect_inproc_direct | 2161868ns | 2149155ns | 2103868ns | 2144432ns | 2217024ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2174809ns | 2174840ns | 2105974ns | 2166066ns | 2222340ns | +0.60% |
| abi_cross_scalar_wideselect_null_entry | 5226ns | 5212ns | 4966ns | 5178ns | 5429ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2182168ns | 2109193ns | 2243015ns | +1.11% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2158180ns | 2100309ns | 2213290ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2171112ns | 2102672ns | 2218577ns | +0.60% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 2833ns | 2710ns | 2970ns | -99.87% | 0.045 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 87796.9 | 2180329.5 | 2182168.0 | n/a |
| abi_cross_scalar_wideselect_inproc_direct | 11292.2 | 2165039.4 | 2158179.9 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 11318.1 | 2167244.0 | 2171112.3 | n/a |
| abi_cross_scalar_wideselect_null_entry | 31988.0 | 2860.7 | 2833.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.047 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_null_entry | 0.045 | 96.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2186170ns | 2186170ns | +1.12% |
| abi_cross_scalar_wideselect_inproc_direct | 2161868ns | 2161868ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2174809ns | 2174809ns | +0.60% |
| abi_cross_scalar_wideselect_null_entry | 5226ns | 5226ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2145448ns | base | --- | [2115802, 2213290] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2168658ns | +20654.8ns (+1.0%) | [+12357, +38952]ns | [2134831, 2243015] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2170992ns | no significant difference | [-3244, +30568]ns | [2123767, 2218577] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_wideselect_null_entry | 2816ns | -2142627.9ns (-99.9%) | [-2210356, -2113055]ns | [2715, 2970] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2255037ns | +0.7% | -0.4% | -99.9% |
| 2 | 2152946ns | +0.9% | +1.8% | -99.9% |
| 3 | 2171542ns | +2.0% | +0.7% | -99.9% |
| 4 | 2137950ns | +1.1% | +0.3% | -99.9% |
| 5 | 2131296ns | +1.6% | +1.1% | -99.9% |
| 6 | 2100309ns | +0.4% | +0.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.027 | ok |
| abi_cross_scalar_wideselect_inproc_direct | 0.089 | ok |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.248 | moderate+ |
| abi_cross_scalar_wideselect_null_entry | 0.416 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6630859.1ns | 2182168.0ns | 303.9% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6503268.0ns | 2158179.9ns | 301.3% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6521813.1ns | 2171112.3ns | 300.4% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 124750.3ns | 2833.5ns | 4402.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2109193.3-2243015.4 ns)
  2109193.3 |########################################
  2115884.4 |
  2122575.5 |
  2129266.6 |
  2135957.7 |
  2142648.8 |
  2149339.9 |
  2156031.0 |########################################
  2162722.1 |########################################
  2169413.2 |########################################
  2176104.4 |
  2182795.5 |
  2189486.6 |
  2196177.7 |
  2202868.8 |
  2209559.9 |########################################
  2216251.0 |
  2222942.1 |
  2229633.2 |
  2236324.3 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2100308.8-2213289.6 ns)
  2100308.8 |########################################
  2105957.8 |
  2111606.9 |
  2117255.9 |
  2122905.0 |
  2128554.0 |########################################
  2134203.0 |########################################
  2139852.1 |
  2145501.1 |
  2151150.2 |########################################
  2156799.2 |
  2162448.2 |
  2168097.3 |########################################
  2173746.3 |
  2179395.4 |
  2185044.4 |
  2190693.4 |
  2196342.5 |
  2201991.5 |
  2207640.6 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2102671.7-2218577.0 ns)
  2102671.7 |########################################
  2108467.0 |
  2114262.2 |
  2120057.5 |
  2125852.8 |
  2131648.0 |
  2137443.3 |
  2143238.6 |########################################
  2149033.8 |########################################
  2154829.1 |
  2160624.4 |
  2166419.6 |
  2172214.9 |
  2178010.2 |
  2183805.4 |########################################
  2189600.7 |########################################
  2195396.0 |
  2201191.2 |
  2206986.5 |
  2212781.8 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 2710.4-2969.8 ns)
   2710.4 |########################################
   2723.4 |
   2736.3 |
   2749.3 |
   2762.3 |####################
   2775.2 |
   2788.2 |
   2801.2 |
   2814.2 |
   2827.1 |
   2840.1 |
   2853.1 |####################
   2866.0 |
   2879.0 |
   2892.0 |
   2905.0 |
   2917.9 |####################
   2930.9 |
   2943.9 |
   2956.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=299.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=4409.4% of algo (FFI overhead may distort results)
