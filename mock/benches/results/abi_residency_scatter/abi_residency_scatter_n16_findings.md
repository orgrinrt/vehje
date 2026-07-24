# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 82235% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (2.58 us) leads abi_residency_scatter_reused_buffer (2.13 ms) by 82235%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.12 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 824.9x slower than the field

abi_residency_scatter_fresh_alloc (2.13 ms) is 824.9x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 824.9x the fastest

Fastest abi_residency_scatter_null_entry (2.58 us) to slowest abi_residency_scatter_fresh_alloc (2.13 ms): 824.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 2583.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 824.87x (fastest 2583.9 ns, slowest 2131433.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2137632ns | 2133910ns | 2128424ns | 2132605ns | 2149777ns | +0.33% |
| abi_residency_scatter_null_entry | 4871ns | 4919ns | 4618ns | 4849ns | 5032ns | -99.77% |
| abi_residency_scatter_reused_buffer | 2130625ns | 2130048ns | 2121190ns | 2128700ns | 2138231ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2135059ns | 2125830ns | 2147023ns | +0.33% | 0.000 |
| abi_residency_scatter_null_entry | 2563ns | 2450ns | 2638ns | -99.88% | 0.006 |
| abi_residency_scatter_reused_buffer | 2128072ns | 2118608ns | 2135649ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 39274.9 | 2192491.5 | 2135059.4 | n/a |
| abi_residency_scatter_null_entry | 26858.0 | 2662.9 | 2562.9 | n/a |
| abi_residency_scatter_reused_buffer | 38555.5 | 2127338.1 | 2128071.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.1% |
| abi_residency_scatter_null_entry | 0.006 | 94.8% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2137632ns | 2137632ns | +0.33% |
| abi_residency_scatter_null_entry | 4871ns | 4871ns | -99.77% |
| abi_residency_scatter_reused_buffer | 2130625ns | 2130625ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2127505ns | base | --- | [2121061, 2135649] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2131433ns | no significant difference | [-6003, +21158]ns | [2126722, 2147023] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_scatter_null_entry | 2584ns | -2124905.9ns (-99.9%) | [-2133182, -2118438]ns | [2467, 2638] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2123514ns | +1.3% | -99.9% |
| 2 | 2118608ns | +0.7% | -99.9% |
| 3 | 2130253ns | -0.2% | -99.9% |
| 4 | 2136101ns | +0.4% | -99.9% |
| 5 | 2135198ns | -0.4% | -99.9% |
| 6 | 2124757ns | +0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.212 | moderate- |
| abi_residency_scatter_null_entry | 0.090 | ok |
| abi_residency_scatter_reused_buffer | 0.305 | moderate+ |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6520554.6ns | 2135059.4ns | 305.4% | HIGH |
| abi_residency_scatter_null_entry | 116381.1ns | 2562.9ns | 4541.1% | HIGH |
| abi_residency_scatter_reused_buffer | 6424051.7ns | 2128071.7ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2125830.4-2147023.1 ns)
  2125830.4 |########################################
  2126890.0 |########################################
  2127949.7 |########################################
  2129009.3 |
  2130068.9 |
  2131128.6 |
  2132188.2 |
  2133247.8 |########################################
  2134307.5 |
  2135367.1 |
  2136426.8 |
  2137486.4 |
  2138546.0 |
  2139605.7 |
  2140665.3 |
  2141724.9 |
  2142784.6 |
  2143844.2 |########################################
  2144903.8 |
  2145963.5 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 2450.0-2637.7 ns)
   2450.0 |########################################
   2459.4 |
   2468.8 |
   2478.2 |########################################
   2487.5 |
   2496.9 |
   2506.3 |
   2515.7 |
   2525.1 |
   2534.5 |
   2543.8 |########################################
   2553.2 |
   2562.6 |
   2572.0 |
   2581.4 |
   2590.8 |
   2600.2 |
   2609.5 |########################################
   2618.9 |########################################
   2628.3 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2118608.3-2135649.4 ns)
  2118608.3 |########################################
  2119460.4 |
  2120312.4 |
  2121164.5 |
  2122016.5 |
  2122868.6 |########################################
  2123720.6 |
  2124572.7 |########################################
  2125424.7 |
  2126276.8 |
  2127128.8 |
  2127980.9 |
  2128832.9 |
  2129685.0 |########################################
  2130537.0 |
  2131389.1 |
  2132241.1 |
  2133093.2 |
  2133945.2 |
  2134797.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=4503.6% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=301.8% of algo (FFI overhead may distort results)
