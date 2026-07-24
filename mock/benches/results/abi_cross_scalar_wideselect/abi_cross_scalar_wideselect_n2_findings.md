# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 62301% faster than the next best (abi_cross_scalar_wideselect_inproc_direct)

abi_cross_scalar_wideselect_null_entry (3.48 us) leads abi_cross_scalar_wideselect_inproc_direct (2.17 ms) by 62301%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_inproc_fnptr is an outlier: 630.4x slower than the field

abi_cross_scalar_wideselect_inproc_fnptr (2.19 ms) is 630.4x the fastest (3.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr} (62301% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr} with a 62301% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 630.4x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (3.48 us) to slowest abi_cross_scalar_wideselect_inproc_fnptr (2.19 ms): 630.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 3481.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 630.37x (fastest 3481.2 ns, slowest 2194490.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2230514ns | 2180483ns | 2111572ns | 2176680ns | 2370734ns | +2.66% |
| abi_cross_scalar_wideselect_inproc_direct | 2172793ns | 2176251ns | 2105797ns | 2172912ns | 2206114ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2257809ns | 2198443ns | 2119135ns | 2186990ns | 2433373ns | +3.91% |
| abi_cross_scalar_wideselect_null_entry | 5838ns | 5815ns | 5683ns | 5791ns | 5985ns | -99.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2226218ns | 2107794ns | 2366222ns | +2.64% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2168979ns | 2102334ns | 2202168ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2253820ns | 2115503ns | 2429174ns | +3.91% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 3502ns | 3408ns | 3598ns | -99.84% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 94085.8 | 2229415.9 | 2226218.0 | n/a |
| abi_cross_scalar_wideselect_inproc_direct | 11376.6 | 2168246.8 | 2168979.0 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 12701.5 | 2195803.5 | 2253820.5 | n/a |
| abi_cross_scalar_wideselect_null_entry | 31479.2 | 3650.0 | 3502.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_null_entry | 0.001 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2230514ns | 2230514ns | +2.66% |
| abi_cross_scalar_wideselect_inproc_direct | 2172793ns | 2172793ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2257809ns | 2257809ns | +3.91% |
| abi_cross_scalar_wideselect_null_entry | 5838ns | 5838ns | -99.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2172348ns | base | --- | [2132420, 2202168] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2176113ns | no significant difference | [-32362, +196749]ns | [2136319, 2366222] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2194490ns | no significant difference | [-26069, +259701]ns | [2137797, 2429174] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_null_entry | 3481ns | -2168839.6ns (-99.8%) | [-2198652, -2128939]ns | [3428, 3598] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2227898ns | -2.5% | -1.7% | -99.8% |
| 2 | 2171207ns | +0.4% | +1.3% | -99.8% |
| 3 | 2173489ns | -0.4% | -0.6% | -99.8% |
| 4 | 2176439ns | +13.7% | +20.9% | -99.8% |
| 5 | 2162507ns | +4.4% | +2.9% | -99.8% |
| 6 | 2102334ns | +0.3% | +0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.069 | ok |
| abi_cross_scalar_wideselect_inproc_direct | 0.069 | ok |
| abi_cross_scalar_wideselect_inproc_fnptr | -0.188 | ok |
| abi_cross_scalar_wideselect_null_entry | 0.352 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6802336.5ns | 2226218.0ns | 305.6% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6516810.6ns | 2168979.0ns | 300.5% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6684593.8ns | 2253820.5ns | 296.6% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 124155.3ns | 3502.2ns | 3545.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2107794.2-2366222.0 ns)
  2107794.2 |####################
  2120715.6 |
  2133637.0 |
  2146558.4 |
  2159479.8 |########################################
  2172401.2 |####################
  2185322.6 |
  2198243.9 |
  2211165.3 |
  2224086.7 |
  2237008.1 |
  2249929.5 |####################
  2262850.9 |
  2275772.3 |
  2288693.7 |
  2301615.1 |
  2314536.5 |
  2327457.9 |
  2340379.3 |
  2353300.7 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2102333.8-2202168.4 ns)
  2102333.8 |####################
  2107325.5 |
  2112317.3 |
  2117309.0 |
  2122300.7 |
  2127292.4 |
  2132284.2 |
  2137275.9 |
  2142267.6 |
  2147259.3 |
  2152251.1 |
  2157242.8 |
  2162234.5 |####################
  2167226.3 |####################
  2172218.0 |########################################
  2177209.7 |
  2182201.4 |
  2187193.2 |
  2192184.9 |
  2197176.6 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2115502.9-2429174.4 ns)
  2115502.9 |########################################
  2131186.5 |
  2146870.0 |########################################
  2162553.6 |
  2178237.2 |########################################
  2193920.8 |########################################
  2209604.4 |
  2225287.9 |########################################
  2240971.5 |
  2256655.1 |
  2272338.6 |
  2288022.2 |
  2303705.8 |
  2319389.4 |
  2335072.9 |
  2350756.5 |
  2366440.1 |
  2382123.7 |
  2397807.2 |
  2413490.8 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 3407.9-3597.5 ns)
   3407.9 |########################################
   3417.4 |
   3426.9 |
   3436.3 |
   3445.8 |########################################
   3455.3 |
   3464.8 |########################################
   3474.3 |
   3483.7 |########################################
   3493.2 |
   3502.7 |
   3512.2 |
   3521.7 |
   3531.1 |
   3540.6 |
   3550.1 |
   3559.6 |
   3569.1 |########################################
   3578.5 |
   3588.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=300.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=3565.6% of algo (FFI overhead may distort results)
