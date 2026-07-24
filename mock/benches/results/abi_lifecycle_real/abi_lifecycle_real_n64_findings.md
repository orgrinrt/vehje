# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 85725% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (2.50 us) leads abi_lifecycle_real_held_handle (2.14 ms) by 85725%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 879.6x slower than the field

abi_lifecycle_real_fresh_per_batch (2.20 ms) is 879.6x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (85725% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 85725% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 879.6x the fastest

Fastest abi_lifecycle_real_null_entry (2.50 us) to slowest abi_lifecycle_real_fresh_per_batch (2.20 ms): 879.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 2498.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 879.62x (fastest 2498.6 ns, slowest 2197767.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2220994ns | 2200323ns | 2191406ns | 2198579ns | 2269410ns | -0.83% |
| abi_lifecycle_real_fresh_per_column | 2175647ns | 2167097ns | 2157112ns | 2165335ns | 2200382ns | -2.86% |
| abi_lifecycle_real_held_handle | 2239639ns | 2146927ns | 2142035ns | 2146163ns | 2428655ns | base |
| abi_lifecycle_real_null_entry | 4789ns | 4797ns | 4586ns | 4735ns | 4971ns | -99.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2218337ns | 2188719ns | 2266514ns | -0.82% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2172960ns | 2154459ns | 2197596ns | -2.85% | 0.000 |
| abi_lifecycle_real_held_handle | 2236787ns | 2139468ns | 2425249ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 2512ns | 2402ns | 2620ns | -99.89% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 44824.9 | 2293597.6 | 2218336.8 | n/a |
| abi_lifecycle_real_fresh_per_column | 43371.5 | 2188289.1 | 2172960.2 | n/a |
| abi_lifecycle_real_held_handle | 50850.7 | 2205464.6 | 2236786.7 | n/a |
| abi_lifecycle_real_null_entry | 27909.2 | 2734.5 | 2511.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_real_held_handle | 0.000 | 0.1% |
| abi_lifecycle_real_null_entry | 0.026 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2220994ns | 2220994ns | -0.83% |
| abi_lifecycle_real_fresh_per_column | 2175647ns | 2175647ns | -2.86% |
| abi_lifecycle_real_held_handle | 2239639ns | 2239639ns | base |
| abi_lifecycle_real_null_entry | 4789ns | 4789ns | -99.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2144378ns | base | --- | [2140733, 2425249] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2197768ns | no significant difference | [-165643, +58223]ns | [2190729, 2266514] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_real_fresh_per_column | 2164366ns | no significant difference | [-232234, +21618]ns | [2156919, 2197596] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_real_null_entry | 2499ns | -2141876.2ns (-99.9%) | [-2422765, -2138184]ns | [2417, 2620] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2147765ns | +2.7% | +1.0% | -99.9% |
| 2 | 2141998ns | +2.6% | +0.6% | -99.9% |
| 3 | 2143901ns | +2.3% | +1.0% | -99.9% |
| 4 | 2702734ns | -13.9% | -17.6% | -99.9% |
| 5 | 2139468ns | +2.7% | +0.9% | -99.9% |
| 6 | 2144855ns | +2.0% | +0.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.248 | moderate- |
| abi_lifecycle_real_fresh_per_column | -0.223 | moderate- |
| abi_lifecycle_real_held_handle | -0.240 | moderate- |
| abi_lifecycle_real_null_entry | 0.170 | ok |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_real_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 6783387.1ns | 2218336.8ns | 305.8% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6575210.9ns | 2172960.2ns | 302.6% | HIGH |
| abi_lifecycle_real_held_handle | 6667193.1ns | 2236786.7ns | 298.1% | HIGH |
| abi_lifecycle_real_null_entry | 113336.7ns | 2511.8ns | 4512.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2188719.2-2266513.5 ns)
  2188719.2 |####################
  2192608.9 |####################
  2196498.6 |########################################
  2200388.4 |
  2204278.1 |####################
  2208167.8 |
  2212057.5 |
  2215947.2 |
  2219836.9 |
  2223726.7 |
  2227616.4 |
  2231506.1 |
  2235395.8 |
  2239285.5 |
  2243175.2 |
  2247065.0 |
  2250954.7 |
  2254844.4 |
  2258734.1 |
  2262623.8 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2154458.8-2197595.9 ns)
  2154458.8 |########################################
  2156615.7 |
  2158772.5 |########################################
  2160929.4 |
  2163086.2 |########################################
  2165243.1 |########################################
  2167399.9 |########################################
  2169556.8 |
  2171713.6 |
  2173870.5 |
  2176027.3 |
  2178184.2 |
  2180341.0 |
  2182497.9 |
  2184654.7 |
  2186811.6 |
  2188968.4 |
  2191125.3 |
  2193282.1 |
  2195439.0 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2139468.3-2425249.4 ns)
  2139468.3 |########################################
  2153757.4 |
  2168046.4 |
  2182335.5 |
  2196624.5 |
  2210913.6 |
  2225202.6 |
  2239491.7 |
  2253780.7 |
  2268069.8 |
  2282358.9 |
  2296647.9 |
  2310937.0 |
  2325226.0 |
  2339515.1 |
  2353804.1 |
  2368093.2 |
  2382382.2 |
  2396671.3 |
  2410960.3 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 2402.1-2620.0 ns)
   2402.1 |########################################
   2413.0 |
   2423.9 |########################################
   2434.8 |
   2445.7 |
   2456.6 |########################################
   2467.5 |
   2478.4 |
   2489.3 |
   2500.2 |
   2511.1 |
   2521.9 |
   2532.8 |########################################
   2543.7 |
   2554.6 |
   2565.5 |
   2576.4 |
   2587.3 |
   2598.2 |########################################
   2609.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=4536.4% of algo (FFI overhead may distort results)
