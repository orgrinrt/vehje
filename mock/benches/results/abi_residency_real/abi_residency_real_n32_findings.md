# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 91859% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (2.31 us) leads abi_residency_real_reused_buffer (2.13 ms) by 91859%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.13 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 924.4x slower than the field

abi_residency_real_fresh_alloc (2.14 ms) is 924.4x the fastest (2.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 924.4x the fastest

Fastest abi_residency_real_null_entry (2.31 us) to slowest abi_residency_real_fresh_alloc (2.14 ms): 924.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 2315.0 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 924.39x (fastest 2315.0 ns, slowest 2139955.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2221666ns | 2142468ns | 2127452ns | 2139600ns | 2391871ns | +4.11% |
| abi_residency_real_null_entry | 4610ns | 4603ns | 4482ns | 4570ns | 4735ns | -99.78% |
| abi_residency_real_reused_buffer | 2133960ns | 2131405ns | 2122513ns | 2130483ns | 2144899ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2219024ns | 2124858ns | 2388990ns | +4.11% | 0.000 |
| abi_residency_real_null_entry | 2319ns | 2255ns | 2380ns | -99.89% | 0.014 |
| abi_residency_real_reused_buffer | 2131448ns | 2120068ns | 2142330ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 44244.2 | 2150339.3 | 2219023.8 | n/a |
| abi_residency_real_null_entry | 28186.0 | 2431.0 | 2318.6 | n/a |
| abi_residency_real_reused_buffer | 39692.0 | 2132048.2 | 2131448.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.1% |
| abi_residency_real_null_entry | 0.014 | 97.4% |
| abi_residency_real_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2221666ns | 2221666ns | +4.11% |
| abi_residency_real_null_entry | 4610ns | 4610ns | -99.78% |
| abi_residency_real_reused_buffer | 2133960ns | 2133960ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2128859ns | base | --- | [2123156, 2142330] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2139956ns | no significant difference | [-4125, +260114]ns | [2128126, 2388990] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_real_null_entry | 2315ns | -2126479.2ns (-99.9%) | [-2140046, -2120864]ns | [2261, 2380] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2126475ns | +23.9% | -99.9% |
| 2 | 2147764ns | -0.3% | -99.9% |
| 3 | 2136896ns | +0.3% | -99.9% |
| 4 | 2131242ns | +0.4% | -99.9% |
| 5 | 2126243ns | -0.1% | -99.9% |
| 6 | 2120068ns | +0.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | -0.022 | ok |
| abi_residency_real_null_entry | -0.485 | moderate- |
| abi_residency_real_reused_buffer | 0.140 | ok |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 1/6, lost 4/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6516943.3ns | 2219023.8ns | 293.7% | HIGH |
| abi_residency_real_null_entry | 117286.2ns | 2318.6ns | 5058.5% | HIGH |
| abi_residency_real_reused_buffer | 6437074.9ns | 2131448.1ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2124858.3-2388989.5 ns)
  2124858.3 |##########################
  2138064.9 |########################################
  2151271.4 |
  2164478.0 |
  2177684.5 |
  2190891.1 |
  2204097.7 |
  2217304.2 |
  2230510.8 |
  2243717.4 |
  2256923.9 |
  2270130.5 |
  2283337.0 |
  2296543.6 |
  2309750.2 |
  2322956.7 |
  2336163.3 |
  2349369.9 |
  2362576.4 |
  2375783.0 |
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 2255.4-2379.8 ns)
   2255.4 |####################
   2261.6 |####################
   2267.8 |
   2274.1 |
   2280.3 |
   2286.5 |
   2292.7 |
   2298.9 |
   2305.1 |
   2311.4 |########################################
   2317.6 |
   2323.8 |
   2330.0 |
   2336.2 |
   2342.4 |
   2348.7 |
   2354.9 |
   2361.1 |
   2367.3 |####################
   2373.5 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2120068.3-2142329.8 ns)
  2120068.3 |####################
  2121181.4 |
  2122294.4 |
  2123407.5 |
  2124520.6 |
  2125633.7 |########################################
  2126746.8 |
  2127859.8 |
  2128972.9 |
  2130086.0 |
  2131199.0 |####################
  2132312.1 |
  2133425.2 |
  2134538.3 |
  2135651.3 |
  2136764.4 |####################
  2137877.5 |
  2138990.6 |
  2140103.6 |
  2141216.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=5061.8% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=301.8% of algo (FFI overhead may distort results)
