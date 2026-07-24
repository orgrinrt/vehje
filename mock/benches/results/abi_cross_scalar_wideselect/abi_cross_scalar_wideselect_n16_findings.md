# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 82889% faster than the next best (abi_cross_scalar_wideselect_inproc_direct)

abi_cross_scalar_wideselect_null_entry (2.58 us) leads abi_cross_scalar_wideselect_inproc_direct (2.14 ms) by 82889%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.14 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_ffi_batched_scalar is an outlier: 853.4x slower than the field

abi_cross_scalar_wideselect_ffi_batched_scalar (2.20 ms) is 853.4x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_ffi_batched_scalar} (82889% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_ffi_batched_scalar} with a 82889% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 853.4x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (2.58 us) to slowest abi_cross_scalar_wideselect_ffi_batched_scalar (2.20 ms): 853.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 2579.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 853.42x (fastest 2579.8 ns, slowest 2201649.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2276426ns | 2205725ns | 2153966ns | 2190894ns | 2465953ns | +6.88% |
| abi_cross_scalar_wideselect_inproc_direct | 2129974ns | 2144588ns | 2087005ns | 2125677ns | 2157905ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2166032ns | 2149088ns | 2095881ns | 2143200ns | 2235356ns | +1.69% |
| abi_cross_scalar_wideselect_null_entry | 4977ns | 4962ns | 4864ns | 4952ns | 5072ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2272281ns | 2149979ns | 2461597ns | +6.86% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2126462ns | 2083835ns | 2154269ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2162492ns | 2092328ns | 2231717ns | +1.69% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 2582ns | 2545ns | 2613ns | -99.88% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 96854.2 | 2217532.9 | 2272281.3 | n/a |
| abi_cross_scalar_wideselect_inproc_direct | 11109.4 | 2134116.8 | 2126461.7 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 11196.7 | 2166794.1 | 2162492.2 | n/a |
| abi_cross_scalar_wideselect_null_entry | 30577.2 | 2691.2 | 2581.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_null_entry | 0.006 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2276426ns | 2276426ns | +6.88% |
| abi_cross_scalar_wideselect_inproc_direct | 2129974ns | 2129974ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2166032ns | 2166032ns | +1.69% |
| abi_cross_scalar_wideselect_null_entry | 4977ns | 4977ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2140942ns | base | --- | [2084175, 2154269] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2201649ns | +88226.2ns (+4.1%) | [+9302, +339931]ns | [2153598, 2461597] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2145643ns | +22284.6ns (+1.0%) | [+3156, +82651]ns | [2110116, 2231717] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_wideselect_null_entry | 2580ns | -2138367.4ns (-99.9%) | [-2151688, -2081584]ns | [2552, 2613] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2084514ns | +7.0% | +0.4% | -99.9% |
| 2 | 2083835ns | +20.9% | +2.1% | -99.9% |
| 3 | 2159497ns | +11.3% | +5.6% | -99.9% |
| 4 | 2142332ns | +1.4% | -0.1% | -99.9% |
| 5 | 2139552ns | +0.5% | +0.5% | -99.9% |
| 6 | 2149040ns | +0.4% | +1.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.300 | moderate+ |
| abi_cross_scalar_wideselect_inproc_direct | 0.251 | moderate+ |
| abi_cross_scalar_wideselect_inproc_fnptr | -0.199 | ok |
| abi_cross_scalar_wideselect_null_entry | -0.164 | ok |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 0/6, lost 5/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6770557.7ns | 2272281.3ns | 298.0% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6407348.1ns | 2126461.7ns | 301.3% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6495623.9ns | 2162492.2ns | 300.4% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 119879.3ns | 2581.7ns | 4643.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2149978.8-2461596.7 ns)
  2149978.8 |########################################
  2165559.7 |####################
  2181140.6 |
  2196721.5 |
  2212302.4 |
  2227883.3 |####################
  2243464.2 |
  2259045.1 |
  2274626.0 |
  2290206.9 |
  2305787.8 |
  2321368.6 |
  2336949.5 |
  2352530.4 |
  2368111.3 |
  2383692.2 |
  2399273.1 |####################
  2414854.0 |
  2430434.9 |
  2446015.8 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2083835.4-2154268.5 ns)
  2083835.4 |########################################
  2087357.1 |
  2090878.7 |
  2094400.4 |
  2097922.0 |
  2101443.7 |
  2104965.3 |
  2108487.0 |
  2112008.7 |
  2115530.3 |
  2119052.0 |
  2122573.6 |
  2126095.3 |
  2129616.9 |
  2133138.6 |
  2136660.3 |####################
  2140181.9 |####################
  2143703.6 |
  2147225.2 |####################
  2150746.9 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2092327.9-2231717.1 ns)
  2092327.9 |########################################
  2099297.4 |
  2106266.8 |
  2113236.3 |
  2120205.7 |
  2127175.2 |########################################
  2134144.7 |########################################
  2141114.1 |
  2148083.6 |########################################
  2155053.0 |
  2162022.5 |
  2168992.0 |
  2175961.4 |########################################
  2182930.9 |
  2189900.3 |
  2196869.8 |
  2203839.3 |
  2210808.7 |
  2217778.2 |
  2224747.6 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 2545.4-2613.1 ns)
   2545.4 |########################################
   2548.8 |
   2552.2 |
   2555.6 |########################################
   2558.9 |
   2562.3 |
   2565.7 |
   2569.1 |########################################
   2572.5 |
   2575.9 |
   2579.2 |
   2582.6 |
   2586.0 |
   2589.4 |########################################
   2592.8 |
   2596.2 |
   2599.6 |
   2602.9 |
   2606.3 |
   2609.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=4653.8% of algo (FFI overhead may distort results)
