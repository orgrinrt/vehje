# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 91387% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (2.32 us) leads abi_residency_scatter_reused_buffer (2.13 ms) by 91387%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.12 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 916.0x slower than the field

abi_residency_scatter_fresh_alloc (2.13 ms) is 916.0x the fastest (2.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 916.0x the fastest

Fastest abi_residency_scatter_null_entry (2.32 us) to slowest abi_residency_scatter_fresh_alloc (2.13 ms): 916.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 2324.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 916.02x (fastest 2324.4 ns, slowest 2129187.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2129519ns | 2131909ns | 2118291ns | 2128985ns | 2135935ns | +0.12% |
| abi_residency_scatter_null_entry | 4624ns | 4620ns | 4358ns | 4570ns | 4838ns | -99.78% |
| abi_residency_scatter_reused_buffer | 2127051ns | 2129088ns | 2117215ns | 2126656ns | 2132561ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2126935ns | 2115890ns | 2133350ns | +0.11% | 0.000 |
| abi_residency_scatter_null_entry | 2323ns | 2198ns | 2419ns | -99.89% | 0.014 |
| abi_residency_scatter_reused_buffer | 2124520ns | 2114753ns | 2130054ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 38835.8 | 2126958.0 | 2126935.4 | n/a |
| abi_residency_scatter_null_entry | 26475.1 | 2455.0 | 2323.3 | n/a |
| abi_residency_scatter_reused_buffer | 37489.3 | 2123781.0 | 2124520.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.1% |
| abi_residency_scatter_null_entry | 0.014 | 94.5% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2129519ns | 2129519ns | +0.12% |
| abi_residency_scatter_null_entry | 4624ns | 4624ns | -99.78% |
| abi_residency_scatter_reused_buffer | 2127051ns | 2127051ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2126528ns | base | --- | [2116979, 2130054] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2129187ns | no significant difference | [-5967, +10177]ns | [2118269, 2133350] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_scatter_null_entry | 2324ns | -2124188.5ns (-99.9%) | [-2127785, -2114618]ns | [2227, 2419] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2126991ns | +0.2% | -99.9% |
| 2 | 2130388ns | +0.2% | -99.9% |
| 3 | 2126066ns | -0.5% | -99.9% |
| 4 | 2129720ns | -0.1% | -99.9% |
| 5 | 2114753ns | +0.7% | -99.9% |
| 6 | 2119204ns | +0.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.307 | moderate- |
| abi_residency_scatter_null_entry | 0.245 | moderate+ |
| abi_residency_scatter_reused_buffer | 0.169 | ok |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 1/6, lost 3/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6421000.5ns | 2126935.4ns | 301.9% | HIGH |
| abi_residency_scatter_null_entry | 115490.8ns | 2323.3ns | 4970.9% | HIGH |
| abi_residency_scatter_reused_buffer | 6413338.1ns | 2124520.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2115889.6-2133349.8 ns)
  2115889.6 |########################################
  2116762.6 |
  2117635.6 |
  2118508.6 |
  2119381.6 |
  2120254.6 |########################################
  2121127.7 |
  2122000.7 |
  2122873.7 |
  2123746.7 |
  2124619.7 |
  2125492.7 |
  2126365.7 |
  2127238.7 |########################################
  2128111.7 |
  2128984.8 |
  2129857.8 |########################################
  2130730.8 |
  2131603.8 |########################################
  2132476.8 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 2197.5-2418.6 ns)
   2197.5 |########################################
   2208.6 |
   2219.6 |
   2230.7 |
   2241.7 |
   2252.8 |########################################
   2263.8 |
   2274.9 |
   2285.9 |
   2297.0 |########################################
   2308.0 |
   2319.1 |
   2330.1 |
   2341.2 |########################################
   2352.2 |
   2363.3 |
   2374.3 |
   2385.4 |
   2396.4 |
   2407.5 |########################################
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2114753.3-2130054.3 ns)
  2114753.3 |########################################
  2115518.4 |
  2116283.4 |
  2117048.5 |
  2117813.5 |
  2118578.6 |########################################
  2119343.6 |
  2120108.7 |
  2120873.7 |
  2121638.8 |
  2122403.8 |
  2123168.9 |
  2123933.9 |
  2124699.0 |
  2125464.0 |########################################
  2126229.1 |########################################
  2126994.1 |
  2127759.2 |
  2128524.2 |
  2129289.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=4946.7% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=302.0% of algo (FFI overhead may distort results)
