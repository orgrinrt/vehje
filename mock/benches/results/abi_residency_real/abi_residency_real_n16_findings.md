# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 80317% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (2.65 us) leads abi_residency_real_reused_buffer (2.13 ms) by 80317%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.13 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 807.8x slower than the field

abi_residency_real_fresh_alloc (2.14 ms) is 807.8x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 807.8x the fastest

Fastest abi_residency_real_null_entry (2.65 us) to slowest abi_residency_real_fresh_alloc (2.14 ms): 807.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 2650.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 807.81x (fastest 2650.2 ns, slowest 2140857.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2143630ns | 2143337ns | 2131813ns | 2140292ns | 2154545ns | +0.59% |
| abi_residency_real_null_entry | 5020ns | 5030ns | 4869ns | 5011ns | 5109ns | -99.76% |
| abi_residency_real_reused_buffer | 2130968ns | 2133847ns | 2111129ns | 2130035ns | 2142288ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2141126ns | 2129411ns | 2152001ns | +0.60% | 0.000 |
| abi_residency_real_null_entry | 2640ns | 2548ns | 2703ns | -99.88% | 0.006 |
| abi_residency_real_reused_buffer | 2128440ns | 2108732ns | 2139719ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 40585.8 | 2141008.7 | 2141126.0 | n/a |
| abi_residency_real_null_entry | 28027.8 | 2740.4 | 2640.0 | n/a |
| abi_residency_real_reused_buffer | 39145.9 | 2128030.8 | 2128440.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.1% |
| abi_residency_real_null_entry | 0.006 | 96.1% |
| abi_residency_real_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2143630ns | 2143630ns | +0.59% |
| abi_residency_real_null_entry | 5020ns | 5020ns | -99.76% |
| abi_residency_real_reused_buffer | 2130968ns | 2130968ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2131224ns | base | --- | [2114379, 2139719] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2140858ns | no significant difference | [-4139, +27954]ns | [2130519, 2152001] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_real_null_entry | 2650ns | -2128597.1ns (-99.9%) | [-2137120, -2111684]ns | [2567, 2703] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2120025ns | +1.6% | -99.9% |
| 2 | 2144613ns | -0.7% | -99.9% |
| 3 | 2134825ns | +0.8% | -99.9% |
| 4 | 2130550ns | +0.6% | -99.9% |
| 5 | 2108732ns | +1.1% | -99.9% |
| 6 | 2131898ns | +0.3% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | -0.489 | moderate- |
| abi_residency_real_null_entry | 0.070 | ok |
| abi_residency_real_reused_buffer | -0.166 | ok |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6463834.5ns | 2141126.0ns | 301.9% | HIGH |
| abi_residency_real_null_entry | 118095.8ns | 2640.0ns | 4473.4% | HIGH |
| abi_residency_real_reused_buffer | 6425969.9ns | 2128440.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2129411.2-2152000.7 ns)
  2129411.2 |########################################
  2130540.7 |########################################
  2131670.1 |
  2132799.6 |
  2133929.1 |
  2135058.6 |
  2136188.0 |
  2137317.5 |
  2138447.0 |########################################
  2139576.5 |
  2140705.9 |
  2141835.4 |########################################
  2142964.9 |
  2144094.3 |
  2145223.8 |
  2146353.3 |
  2147482.8 |
  2148612.2 |
  2149741.7 |
  2150871.2 |########################################
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 2547.9-2703.1 ns)
   2547.9 |####################
   2555.7 |
   2563.4 |
   2571.2 |
   2578.9 |####################
   2586.7 |
   2594.5 |
   2602.2 |
   2610.0 |
   2617.7 |
   2625.5 |
   2633.3 |
   2641.0 |
   2648.8 |########################################
   2656.5 |
   2664.3 |####################
   2672.1 |
   2679.8 |
   2687.6 |
   2695.3 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2108732.5-2139719.0 ns)
  2108732.5 |####################
  2110281.8 |
  2111831.1 |
  2113380.5 |
  2114929.8 |
  2116479.1 |
  2118028.4 |
  2119577.8 |####################
  2121127.1 |
  2122676.4 |
  2124225.7 |
  2125775.0 |
  2127324.4 |
  2128873.7 |
  2130423.0 |########################################
  2131972.3 |
  2133521.7 |####################
  2135071.0 |
  2136620.3 |
  2138169.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=4474.6% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)
