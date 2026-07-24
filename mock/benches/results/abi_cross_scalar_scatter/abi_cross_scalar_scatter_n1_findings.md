# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 44339% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (4.88 us) leads abi_cross_scalar_scatter_inproc_direct (2.17 ms) by 44339%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 449.8x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.20 ms) is 449.8x the fastest (4.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_scatter_inproc_fnptr shows alternating (throttle bounce) (autocorr -0.60)

abi_cross_scalar_scatter_inproc_fnptr's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (44339% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 44339% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 449.8x the fastest

Fastest abi_cross_scalar_scatter_null_entry (4.88 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.20 ms): 449.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 4883.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 449.85x (fastest 4883.5 ns, slowest 2196845.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2263036ns | 2200315ns | 2175314ns | 2195458ns | 2408264ns | +3.81% |
| abi_cross_scalar_scatter_inproc_direct | 2179948ns | 2173637ns | 2161813ns | 2172494ns | 2200196ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2206136ns | 2200484ns | 2190320ns | 2197947ns | 2226328ns | +1.20% |
| abi_cross_scalar_scatter_null_entry | 7180ns | 7138ns | 6968ns | 7123ns | 7372ns | -99.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2259035ns | 2171898ns | 2403763ns | +3.79% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2176472ns | 2158475ns | 2196483ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2202598ns | 2187361ns | 2222552ns | +1.20% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 4902ns | 4781ns | 5018ns | -99.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 77518.9 | 2252252.7 | 2259035.1 | n/a |
| abi_cross_scalar_scatter_inproc_direct | 8052.1 | 2177883.3 | 2176472.3 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 8391.5 | 2201231.9 | 2202597.7 | n/a |
| abi_cross_scalar_scatter_null_entry | 27776.5 | 4974.3 | 4902.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_scatter_null_entry | 0.000 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2263036ns | 2263036ns | +3.81% |
| abi_cross_scalar_scatter_inproc_direct | 2179948ns | 2179948ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2206136ns | 2206136ns | +1.20% |
| abi_cross_scalar_scatter_null_entry | 7180ns | 7180ns | -99.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2170196ns | base | --- | [2162738, 2196483] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2196306ns | +26142.9ns (+1.2%) | [+14265, +207280]ns | [2177036, 2403763] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2196845ns | +20613.1ns (+0.9%) | [+14380, +43383]ns | [2188396, 2222552] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_scatter_null_entry | 4884ns | -2165321.6ns (-99.8%) | [-2191580, -2157808]ns | [4805, 5018] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2158475ns | +0.6% | +1.6% | -99.8% |
| 2 | 2167001ns | +1.1% | +2.4% | -99.8% |
| 3 | 2167067ns | +0.7% | +0.9% | -99.8% |
| 4 | 2212855ns | +5.8% | +0.6% | -99.8% |
| 5 | 2180110ns | +13.2% | +1.0% | -99.8% |
| 6 | 2173325ns | +1.3% | +0.7% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.141 | ok |
| abi_cross_scalar_scatter_inproc_direct | 0.021 | ok |
| abi_cross_scalar_scatter_inproc_fnptr | -0.600 | HIGH- (thermal bounce) |
| abi_cross_scalar_scatter_null_entry | 0.171 | ok |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6856114.1ns | 2259035.1ns | 303.5% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6535480.4ns | 2176472.3ns | 300.3% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6623118.9ns | 2202597.7ns | 300.7% | HIGH |
| abi_cross_scalar_scatter_null_entry | 124472.3ns | 4902.3ns | 2539.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2171897.9-2403763.1 ns)
  2171897.9 |########################################
  2183491.2 |####################
  2195084.4 |####################
  2206677.7 |
  2218270.9 |
  2229864.2 |
  2241457.5 |
  2253050.7 |
  2264644.0 |
  2276237.2 |
  2287830.5 |
  2299423.8 |
  2311017.0 |
  2322610.3 |
  2334203.5 |####################
  2345796.8 |
  2357390.1 |
  2368983.3 |
  2380576.6 |
  2392169.8 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2158475.0-2196482.9 ns)
  2158475.0 |####################
  2160375.4 |
  2162275.8 |
  2164176.2 |
  2166076.6 |########################################
  2167977.0 |
  2169877.4 |
  2171777.8 |####################
  2173678.2 |
  2175578.6 |
  2177479.0 |
  2179379.3 |####################
  2181279.7 |
  2183180.1 |
  2185080.5 |
  2186980.9 |
  2188881.3 |
  2190781.7 |
  2192682.1 |
  2194582.5 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2187360.8-2222551.7 ns)
  2187360.8 |########################################
  2189120.3 |########################################
  2190879.9 |
  2192639.4 |########################################
  2194399.0 |
  2196158.5 |
  2197918.1 |
  2199677.6 |########################################
  2201437.2 |
  2203196.7 |
  2204956.2 |
  2206715.8 |
  2208475.3 |
  2210234.9 |
  2211994.4 |
  2213754.0 |
  2215513.5 |
  2217273.1 |
  2219032.6 |########################################
  2220792.2 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 4780.8-5018.1 ns)
   4780.8 |########################################
   4792.7 |
   4804.5 |
   4816.4 |
   4828.3 |########################################
   4840.1 |
   4852.0 |
   4863.9 |########################################
   4875.7 |
   4887.6 |
   4899.5 |########################################
   4911.3 |
   4923.2 |
   4935.0 |########################################
   4946.9 |
   4958.8 |
   4970.6 |
   4982.5 |
   4994.4 |
   5006.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=2540.2% of algo (FFI overhead may distort results)
