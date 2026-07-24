# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 94393% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (2.30 us) leads abi_cross_scalar_scatter_inproc_direct (2.17 ms) by 94393%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 949.8x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.18 ms) is 949.8x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_scatter_null_entry shows alternating (throttle bounce) (autocorr -0.55)

abi_cross_scalar_scatter_null_entry's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (94393% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 94393% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 949.8x the fastest

Fastest abi_cross_scalar_scatter_null_entry (2.30 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.18 ms): 949.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 2295.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 949.78x (fastest 2295.8 ns, slowest 2180552.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2180188ns | 2180789ns | 2177660ns | 2179789ns | 2182050ns | -0.38% |
| abi_cross_scalar_scatter_inproc_direct | 2188485ns | 2172649ns | 2157450ns | 2172140ns | 2228520ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2190653ns | 2183919ns | 2177732ns | 2181939ns | 2210185ns | +0.10% |
| abi_cross_scalar_scatter_null_entry | 4613ns | 4602ns | 4512ns | 4578ns | 4716ns | -99.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2176667ns | 2174100ns | 2178494ns | -0.39% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2185088ns | 2154241ns | 2224792ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2187268ns | 2174446ns | 2206698ns | +0.10% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 2294ns | 2271ns | 2313ns | -99.90% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 69538.1 | 2173508.8 | 2176667.0 | 7 |
| abi_cross_scalar_scatter_inproc_direct | 8410.9 | 2177448.2 | 2185087.7 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 8283.2 | 2187005.2 | 2187267.9 | 0 |
| abi_cross_scalar_scatter_null_entry | 28325.6 | 2448.0 | 2294.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_scatter_null_entry | 0.014 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2180188ns | 2180188ns | -0.38% |
| abi_cross_scalar_scatter_inproc_direct | 2188485ns | 2188485ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2190653ns | 2190653ns | +0.10% |
| abi_cross_scalar_scatter_null_entry | 4613ns | 4613ns | -99.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2169407ns | base | --- | [2161064, 2224792] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2177389ns | no significant difference | [-48315, +16325]ns | [2174118, 2178494] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2180552ns | no significant difference | [-50238, +39897]ns | [2174554, 2206698] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_scatter_null_entry | 2296ns | -2167121.7ns (-99.9%) | [-2222490, -2158769]ns | [2274, 2313] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2154241ns | +1.1% | +1.2% | -99.9% |
| 2 | 2169364ns | +0.2% | +2.4% | -99.9% |
| 3 | 2169451ns | +0.4% | +1.0% | -99.9% |
| 4 | 2203332ns | -1.1% | -1.3% | -99.9% |
| 5 | 2167887ns | +0.4% | +0.6% | -99.9% |
| 6 | 2246251ns | -3.2% | -3.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | -0.171 | ok |
| abi_cross_scalar_scatter_inproc_direct | -0.158 | ok |
| abi_cross_scalar_scatter_inproc_fnptr | 0.033 | ok |
| abi_cross_scalar_scatter_null_entry | -0.549 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6598172.7ns | 2176667.0ns | 303.1% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6539565.2ns | 2185087.7ns | 299.3% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6573741.5ns | 2187267.9ns | 300.5% | HIGH |
| abi_cross_scalar_scatter_null_entry | 117101.2ns | 2294.2ns | 5104.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2174099.6-2178494.4 ns)
  2174099.6 |########################################
  2174319.3 |
  2174539.1 |
  2174758.8 |
  2174978.6 |
  2175198.3 |
  2175418.0 |
  2175637.8 |
  2175857.5 |
  2176077.3 |
  2176297.0 |
  2176516.7 |
  2176736.5 |####################
  2176956.2 |
  2177176.0 |
  2177395.7 |
  2177615.4 |
  2177835.2 |####################
  2178054.9 |####################
  2178274.7 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2154241.2-2224791.9 ns)
  2154241.2 |####################
  2157768.7 |
  2161296.3 |
  2164823.8 |####################
  2168351.3 |########################################
  2171878.9 |
  2175406.4 |
  2178933.9 |
  2182461.5 |
  2185989.0 |
  2189516.5 |
  2193044.1 |
  2196571.6 |
  2200099.1 |####################
  2203626.7 |
  2207154.2 |
  2210681.7 |
  2214209.3 |
  2217736.8 |
  2221264.3 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2174446.2-2206697.9 ns)
  2174446.2 |########################################
  2176058.8 |
  2177671.4 |
  2179284.0 |####################
  2180896.5 |####################
  2182509.1 |
  2184121.7 |
  2185734.3 |
  2187346.9 |
  2188959.5 |
  2190572.1 |####################
  2192184.6 |
  2193797.2 |
  2195409.8 |
  2197022.4 |
  2198635.0 |
  2200247.6 |
  2201860.1 |
  2203472.7 |
  2205085.3 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 2271.2-2313.1 ns)
   2271.2 |########################################
   2273.3 |
   2275.4 |########################################
   2277.5 |
   2279.6 |
   2281.7 |
   2283.8 |
   2285.9 |
   2288.0 |
   2290.1 |########################################
   2292.1 |
   2294.2 |
   2296.3 |
   2298.4 |########################################
   2300.5 |
   2302.6 |
   2304.7 |
   2306.8 |
   2308.9 |
   2311.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=5102.3% of algo (FFI overhead may distort results)
