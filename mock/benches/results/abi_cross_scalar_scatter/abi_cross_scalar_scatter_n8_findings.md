# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 69597% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (3.10 us) leads abi_cross_scalar_scatter_inproc_direct (2.16 ms) by 69597%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 703.9x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.18 ms) is 703.9x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (69597% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 69597% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 703.9x the fastest

Fastest abi_cross_scalar_scatter_null_entry (3.10 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.18 ms): 703.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 3101.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 703.89x (fastest 3101.3 ns, slowest 2182976.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2209844ns | 2182795ns | 2165252ns | 2180908ns | 2275543ns | +2.17% |
| abi_cross_scalar_scatter_inproc_direct | 2163008ns | 2164722ns | 2148852ns | 2164588ns | 2167717ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2208999ns | 2186344ns | 2181964ns | 2185882ns | 2257192ns | +2.13% |
| abi_cross_scalar_scatter_null_entry | 5431ns | 5401ns | 5318ns | 5380ns | 5563ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2206047ns | 2162010ns | 2271070ns | +2.14% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2159854ns | 2146082ns | 2164602ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2205622ns | 2178770ns | 2253700ns | +2.12% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 3108ns | 3056ns | 3147ns | -99.86% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 75057.0 | 2206123.8 | 2206047.2 | n/a |
| abi_cross_scalar_scatter_inproc_direct | 7713.4 | 2161865.2 | 2159853.9 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 8011.4 | 2214946.3 | 2205622.2 | n/a |
| abi_cross_scalar_scatter_null_entry | 27739.7 | 3175.4 | 3107.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_scatter_null_entry | 0.003 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2209844ns | 2209844ns | +2.17% |
| abi_cross_scalar_scatter_inproc_direct | 2163008ns | 2163008ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2208999ns | 2208999ns | +2.13% |
| abi_cross_scalar_scatter_null_entry | 5431ns | 5431ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2161510ns | base | --- | [2153450, 2164602] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2179246ns | +18140.0ns (+0.8%) | [+5189, +115251]ns | [2167826, 2271070] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2182976ns | +29237.7ns (+1.4%) | [+17553, +90514]ns | [2180190, 2253700] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_scatter_null_entry | 3101ns | -2158435.0ns (-99.9%) | [-2161455, -2150349]ns | [3075, 3147] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2165555ns | +6.2% | +4.2% | -99.9% |
| 2 | 2160817ns | +1.1% | +4.1% | -99.9% |
| 3 | 2163649ns | -0.1% | +0.7% | -99.9% |
| 4 | 2161395ns | +0.6% | +1.0% | -99.9% |
| 5 | 2161625ns | +0.6% | +0.9% | -99.9% |
| 6 | 2146082ns | +4.5% | +1.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.015 | ok |
| abi_cross_scalar_scatter_inproc_direct | -0.027 | ok |
| abi_cross_scalar_scatter_inproc_fnptr | 0.403 | moderate+ |
| abi_cross_scalar_scatter_null_entry | 0.173 | ok |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 0/6, lost 5/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6716207.4ns | 2206047.2ns | 304.4% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6493263.8ns | 2159853.9ns | 300.6% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6648869.4ns | 2205622.2ns | 301.5% | HIGH |
| abi_cross_scalar_scatter_null_entry | 119596.4ns | 3107.6ns | 3848.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2162010.0-2271069.6 ns)
  2162010.0 |####################
  2167463.0 |
  2172916.0 |########################################
  2178368.9 |
  2183821.9 |####################
  2189274.9 |
  2194727.9 |
  2200180.9 |
  2205633.8 |
  2211086.8 |
  2216539.8 |
  2221992.8 |
  2227445.8 |
  2232898.7 |
  2238351.7 |####################
  2243804.7 |
  2249257.7 |
  2254710.7 |
  2260163.6 |
  2265616.6 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2146082.1-2164602.1 ns)
  2146082.1 |####################
  2147008.1 |
  2147934.1 |
  2148860.1 |
  2149786.1 |
  2150712.1 |
  2151638.1 |
  2152564.1 |
  2153490.1 |
  2154416.1 |
  2155342.1 |
  2156268.1 |
  2157194.1 |
  2158120.1 |
  2159046.1 |
  2159972.1 |####################
  2160898.1 |########################################
  2161824.1 |
  2162750.1 |####################
  2163676.1 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2178770.0-2253700.0 ns)
  2178770.0 |########################################
  2182516.5 |########################################
  2186263.0 |
  2190009.5 |
  2193756.0 |
  2197502.5 |
  2201249.0 |
  2204995.5 |
  2208742.0 |
  2212488.5 |
  2216235.0 |
  2219981.5 |
  2223728.0 |
  2227474.5 |
  2231221.0 |
  2234967.5 |
  2238714.0 |
  2242460.5 |
  2246207.0 |####################
  2249953.5 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 3056.2-3147.1 ns)
   3056.2 |####################
   3060.7 |
   3065.3 |
   3069.8 |
   3074.4 |
   3078.9 |
   3083.5 |
   3088.0 |
   3092.5 |########################################
   3097.1 |
   3101.6 |
   3106.2 |####################
   3110.7 |####################
   3115.3 |
   3119.8 |
   3124.3 |
   3128.9 |
   3133.4 |
   3138.0 |
   3142.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=3865.2% of algo (FFI overhead may distort results)
