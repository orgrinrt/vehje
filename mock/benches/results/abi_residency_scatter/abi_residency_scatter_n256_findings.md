# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 64832% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (3.28 us) leads abi_residency_scatter_reused_buffer (2.13 ms) by 64832%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.12 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 650.7x slower than the field

abi_residency_scatter_fresh_alloc (2.13 ms) is 650.7x the fastest (3.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 650.7x the fastest

Fastest abi_residency_scatter_null_entry (3.28 us) to slowest abi_residency_scatter_fresh_alloc (2.13 ms): 650.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 3276.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 650.70x (fastest 3276.9 ns, slowest 2132276.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2134297ns | 2134945ns | 2123331ns | 2131755ns | 2143592ns | -3.17% |
| abi_residency_scatter_null_entry | 5577ns | 5589ns | 5291ns | 5549ns | 5762ns | -99.75% |
| abi_residency_scatter_reused_buffer | 2204177ns | 2130368ns | 2127648ns | 2130010ns | 2353692ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2131702ns | 2120876ns | 2140978ns | -3.17% | 0.000 |
| abi_residency_scatter_null_entry | 3259ns | 3081ns | 3358ns | -99.85% | 0.079 |
| abi_residency_scatter_reused_buffer | 2201389ns | 2125070ns | 2350508ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 41670.3 | 2130282.4 | 2131701.7 | n/a |
| abi_residency_scatter_null_entry | 27041.1 | 3267.4 | 3258.8 | n/a |
| abi_residency_scatter_reused_buffer | 46859.7 | 2214292.6 | 2201388.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.1% |
| abi_residency_scatter_null_entry | 0.078 | 94.0% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2134297ns | 2134297ns | -3.17% |
| abi_residency_scatter_null_entry | 5577ns | 5577ns | -99.75% |
| abi_residency_scatter_reused_buffer | 2204177ns | 2204177ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2127757ns | base | --- | [2125900, 2350508] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2132276ns | no significant difference | [-228402, +14073]ns | [2121851, 2140978] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_scatter_null_entry | 3277ns | -2124552.5ns (-99.8%) | [-2347213, -2122624]ns | [3142, 3358] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2572275ns | -17.5% | -99.9% |
| 2 | 2128741ns | +0.6% | -99.8% |
| 3 | 2128230ns | -0.3% | -99.9% |
| 4 | 2126731ns | +0.2% | -99.8% |
| 5 | 2125070ns | +0.8% | -99.8% |
| 6 | 2127284ns | +0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.449 | moderate- |
| abi_residency_scatter_null_entry | -0.197 | ok |
| abi_residency_scatter_reused_buffer | -0.029 | ok |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6434811.6ns | 2131701.7ns | 301.9% | HIGH |
| abi_residency_scatter_null_entry | 120954.0ns | 3258.8ns | 3711.6% | HIGH |
| abi_residency_scatter_reused_buffer | 6676339.9ns | 2201388.5ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2120875.8-2140978.0 ns)
  2120875.8 |########################################
  2121880.9 |########################################
  2122886.0 |
  2123891.1 |
  2124896.2 |
  2125901.3 |
  2126906.4 |
  2127911.6 |
  2128916.7 |
  2129921.8 |
  2130926.9 |########################################
  2131932.0 |########################################
  2132937.1 |
  2133942.2 |
  2134947.3 |
  2135952.4 |
  2136957.5 |
  2137962.6 |
  2138967.7 |
  2139972.8 |########################################
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 3080.8-3357.8 ns)
   3080.8 |####################
   3094.6 |
   3108.5 |
   3122.3 |
   3136.2 |
   3150.0 |
   3163.9 |
   3177.7 |
   3191.6 |####################
   3205.4 |
   3219.3 |####################
   3233.1 |
   3247.0 |
   3260.8 |
   3274.7 |
   3288.5 |
   3302.4 |
   3316.2 |########################################
   3330.1 |
   3343.9 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2125069.6-2350507.9 ns)
  2125069.6 |########################################
  2136341.5 |
  2147613.4 |
  2158885.3 |
  2170157.3 |
  2181429.2 |
  2192701.1 |
  2203973.0 |
  2215244.9 |
  2226516.8 |
  2237788.8 |
  2249060.7 |
  2260332.6 |
  2271604.5 |
  2282876.4 |
  2294148.3 |
  2305420.2 |
  2316692.2 |
  2327964.1 |
  2339236.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=3699.3% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)
