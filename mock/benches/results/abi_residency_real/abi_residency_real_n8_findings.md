# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 71654% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (2.97 us) leads abi_residency_real_reused_buffer (2.13 ms) by 71654%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.13 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 720.6x slower than the field

abi_residency_real_fresh_alloc (2.14 ms) is 720.6x the fastest (2.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 720.6x the fastest

Fastest abi_residency_real_null_entry (2.97 us) to slowest abi_residency_real_fresh_alloc (2.14 ms): 720.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 2972.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 720.57x (fastest 2972.7 ns, slowest 2142037.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2144105ns | 2144631ns | 2132608ns | 2142625ns | 2152074ns | +0.39% |
| abi_residency_real_null_entry | 5282ns | 5212ns | 5102ns | 5189ns | 5512ns | -99.75% |
| abi_residency_real_reused_buffer | 2135730ns | 2135550ns | 2124885ns | 2133823ns | 2144014ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2141528ns | 2129946ns | 2149558ns | +0.39% | 0.000 |
| abi_residency_real_null_entry | 3016ns | 2922ns | 3151ns | -99.86% | 0.003 |
| abi_residency_real_reused_buffer | 2133124ns | 2122348ns | 2141221ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 40345.6 | 2141178.8 | 2141528.0 | 0 |
| abi_residency_real_null_entry | 27126.2 | 3110.5 | 3016.2 | n/a |
| abi_residency_real_reused_buffer | 40587.7 | 2133336.4 | 2133123.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.1% |
| abi_residency_real_null_entry | 0.003 | 98.3% |
| abi_residency_real_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2144105ns | 2144105ns | +0.39% |
| abi_residency_real_null_entry | 5282ns | 5282ns | -99.75% |
| abi_residency_real_reused_buffer | 2135730ns | 2135730ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2133020ns | base | --- | [2125129, 2141221] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2142038ns | no significant difference | [-1374, +20459]ns | [2132988, 2149558] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_real_null_entry | 2973ns | -2130076.7ns (-99.9%) | [-2138070, -2122175]ns | [2925, 3151] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2130190ns | +0.5% | -99.9% |
| 2 | 2122348ns | +1.3% | -99.9% |
| 3 | 2143045ns | +0.0% | -99.9% |
| 4 | 2127910ns | +0.1% | -99.9% |
| 5 | 2139397ns | -0.2% | -99.8% |
| 6 | 2135851ns | +0.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | ok |
| abi_residency_real_null_entry | -0.180 | ok |
| abi_residency_real_reused_buffer | -0.480 | moderate- |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 1/6, lost 3/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6464521.5ns | 2141528.0ns | 301.9% | HIGH |
| abi_residency_real_null_entry | 119103.4ns | 3016.2ns | 3948.7% | HIGH |
| abi_residency_real_reused_buffer | 6442811.5ns | 2133123.5ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2129945.8-2149558.2 ns)
  2129945.8 |########################################
  2130926.4 |
  2131907.0 |
  2132887.7 |
  2133868.3 |
  2134848.9 |
  2135829.5 |########################################
  2136810.1 |
  2137790.7 |
  2138771.4 |
  2139752.0 |########################################
  2140732.6 |
  2141713.2 |
  2142693.8 |########################################
  2143674.4 |
  2144655.1 |
  2145635.7 |
  2146616.3 |
  2147596.9 |
  2148577.5 |########################################
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 2922.1-3150.8 ns)
   2922.1 |########################################
   2933.5 |
   2945.0 |
   2956.4 |####################
   2967.8 |
   2979.3 |####################
   2990.7 |
   3002.1 |
   3013.6 |
   3025.0 |####################
   3036.4 |
   3047.9 |
   3059.3 |
   3070.8 |
   3082.2 |
   3093.6 |
   3105.1 |
   3116.5 |
   3127.9 |
   3139.4 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2122347.9-2141221.2 ns)
  2122347.9 |########################################
  2123291.6 |
  2124235.2 |
  2125178.9 |
  2126122.6 |
  2127066.2 |########################################
  2128009.9 |
  2128953.6 |
  2129897.2 |########################################
  2130840.9 |
  2131784.6 |
  2132728.2 |
  2133671.9 |
  2134615.6 |
  2135559.2 |########################################
  2136502.9 |
  2137446.6 |
  2138390.2 |
  2139333.9 |########################################
  2140277.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=3993.4% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=301.9% of algo (FFI overhead may distort results)
