# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_scatter_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_scatter_reused_buffer has the worst median (2.13 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_scatter_null_entry at 2.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_scatter_null_entry dominates: 82411% faster than the next best (abi_residency_scatter_fresh_alloc)

abi_residency_scatter_null_entry (2.58 us) leads abi_residency_scatter_fresh_alloc (2.13 ms) by 82411%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_reused_buffer is an outlier: 825.7x slower than the field

abi_residency_scatter_reused_buffer (2.13 ms) is 825.7x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 825.7x the fastest

Fastest abi_residency_scatter_null_entry (2.58 us) to slowest abi_residency_scatter_reused_buffer (2.13 ms): 825.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 2581.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 825.72x (fastest 2581.4 ns, slowest 2131566.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2138983ns | 2132640ns | 2126268ns | 2131898ns | 2155968ns | -1.15% |
| abi_residency_scatter_null_entry | 4906ns | 4987ns | 4642ns | 4896ns | 5054ns | -99.77% |
| abi_residency_scatter_reused_buffer | 2163800ns | 2134239ns | 2126881ns | 2132571ns | 2229104ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2136368ns | 2123740ns | 2153344ns | -1.14% | 0.000 |
| abi_residency_scatter_null_entry | 2551ns | 2388ns | 2623ns | -99.88% | 0.025 |
| abi_residency_scatter_reused_buffer | 2160960ns | 2124416ns | 2225864ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 42420.8 | 2136649.5 | 2136367.7 | n/a |
| abi_residency_scatter_null_entry | 26888.4 | 2781.0 | 2551.0 | n/a |
| abi_residency_scatter_reused_buffer | 47971.1 | 2153142.9 | 2160959.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.1% |
| abi_residency_scatter_null_entry | 0.025 | 92.5% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2138983ns | 2138983ns | -1.15% |
| abi_residency_scatter_null_entry | 4906ns | 4906ns | -99.77% |
| abi_residency_scatter_reused_buffer | 2163800ns | 2163800ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2131566ns | base | --- | [2125448, 2225864] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2129973ns | no significant difference | [-77117, +6017]ns | [2125786, 2153344] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_scatter_null_entry | 2581ns | -2129075.0ns (-99.9%) | [-2223317, -2122834]ns | [2449, 2623] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2314382ns | -6.1% | -99.9% |
| 2 | 2124416ns | +0.3% | -99.9% |
| 3 | 2126480ns | +0.3% | -99.9% |
| 4 | 2132933ns | -0.1% | -99.9% |
| 5 | 2137347ns | -0.6% | -99.9% |
| 6 | 2130199ns | -0.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.003 | ok |
| abi_residency_scatter_null_entry | -0.235 | moderate- |
| abi_residency_scatter_reused_buffer | -0.070 | ok |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 4/6, lost 2/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6449957.8ns | 2136367.7ns | 301.9% | HIGH |
| abi_residency_scatter_null_entry | 112897.8ns | 2551.0ns | 4425.6% | HIGH |
| abi_residency_scatter_reused_buffer | 6530277.6ns | 2160959.6ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2123739.6-2153343.8 ns)
  2123739.6 |####################
  2125219.8 |
  2126700.0 |####################
  2128180.2 |
  2129660.4 |########################################
  2131140.6 |
  2132620.8 |####################
  2134101.1 |
  2135581.3 |
  2137061.5 |
  2138541.7 |
  2140021.9 |
  2141502.1 |
  2142982.3 |
  2144462.5 |
  2145942.7 |
  2147422.9 |
  2148903.1 |
  2150383.3 |
  2151863.5 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 2387.9-2622.7 ns)
   2387.9 |####################
   2399.6 |
   2411.4 |
   2423.1 |
   2434.9 |
   2446.6 |
   2458.3 |
   2470.1 |
   2481.8 |
   2493.6 |
   2505.3 |####################
   2517.0 |
   2528.8 |
   2540.5 |
   2552.3 |
   2564.0 |
   2575.7 |########################################
   2587.5 |####################
   2599.2 |
   2611.0 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2124416.2-2225864.4 ns)
  2124416.2 |########################################
  2129488.6 |########################################
  2134561.0 |####################
  2139633.4 |
  2144705.8 |
  2149778.2 |
  2154850.7 |
  2159923.1 |
  2164995.5 |
  2170067.9 |
  2175140.3 |
  2180212.7 |
  2185285.1 |
  2190357.5 |
  2195429.9 |
  2200502.4 |
  2205574.8 |
  2210647.2 |
  2215719.6 |
  2220792.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=4409.7% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=302.3% of algo (FFI overhead may distort results)
