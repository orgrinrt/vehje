# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 62067% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (3.47 us) leads abi_residency_real_reused_buffer (2.16 ms) by 62067%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.15 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 622.7x slower than the field

abi_residency_real_fresh_alloc (2.16 ms) is 622.7x the fastest (3.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 622.7x the fastest

Fastest abi_residency_real_null_entry (3.47 us) to slowest abi_residency_real_fresh_alloc (2.16 ms): 622.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 3469.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 622.68x (fastest 3469.8 ns, slowest 2160579.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2168611ns | 2163221ns | 2145115ns | 2162509ns | 2189513ns | -0.49% |
| abi_residency_real_null_entry | 5752ns | 5737ns | 5590ns | 5713ns | 5891ns | -99.74% |
| abi_residency_real_reused_buffer | 2179242ns | 2159711ns | 2149435ns | 2158948ns | 2224586ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2166009ns | 2142683ns | 2186821ns | -0.48% | 0.000 |
| abi_residency_real_null_entry | 3482ns | 3386ns | 3567ns | -99.84% | 0.001 |
| abi_residency_real_reused_buffer | 2176532ns | 2146823ns | 2221725ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 43366.0 | 2165786.1 | 2166008.8 | n/a |
| abi_residency_real_null_entry | 27363.7 | 3570.8 | 3481.8 | n/a |
| abi_residency_real_reused_buffer | 46425.8 | 2191805.6 | 2176531.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.2% |
| abi_residency_real_null_entry | 0.001 | 97.6% |
| abi_residency_real_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2168611ns | 2168611ns | -0.49% |
| abi_residency_real_null_entry | 5752ns | 5752ns | -99.74% |
| abi_residency_real_reused_buffer | 2179242ns | 2179242ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2157078ns | base | --- | [2150792, 2221725] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2160579ns | no significant difference | [-49969, +16277]ns | [2150626, 2186821] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_real_null_entry | 3470ns | -2153627.3ns (-99.8%) | [-2218297, -2147225]ns | [3408, 3567] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2284269ns | -3.8% | -99.9% |
| 2 | 2159181ns | +0.8% | -99.8% |
| 3 | 2146823ns | +0.7% | -99.8% |
| 4 | 2158402ns | +0.0% | -99.8% |
| 5 | 2155753ns | -0.6% | -99.8% |
| 6 | 2154762ns | +0.2% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.371 | moderate+ |
| abi_residency_real_null_entry | -0.190 | ok |
| abi_residency_real_reused_buffer | 0.001 | ok |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6541396.9ns | 2166008.8ns | 302.0% | HIGH |
| abi_residency_real_null_entry | 120425.3ns | 3481.8ns | 3458.7% | HIGH |
| abi_residency_real_reused_buffer | 6582102.8ns | 2176531.7ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2142682.9-2186821.2 ns)
  2142682.9 |####################
  2144889.8 |
  2147096.7 |
  2149303.7 |
  2151510.6 |
  2153717.5 |
  2155924.4 |
  2158131.3 |########################################
  2160338.2 |####################
  2162545.2 |
  2164752.1 |
  2166959.0 |
  2169165.9 |
  2171372.8 |
  2173579.7 |
  2175786.7 |####################
  2177993.6 |
  2180200.5 |
  2182407.4 |
  2184614.3 |
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 3386.2-3567.4 ns)
   3386.2 |####################
   3395.3 |
   3404.3 |
   3413.4 |
   3422.4 |####################
   3431.5 |
   3440.6 |
   3449.6 |
   3458.7 |
   3467.8 |########################################
   3476.8 |
   3485.9 |
   3494.9 |
   3504.0 |
   3513.1 |####################
   3522.1 |
   3531.2 |
   3540.3 |
   3549.3 |
   3558.4 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2146823.3-2221725.0 ns)
  2146823.3 |####################
  2150568.4 |
  2154313.5 |########################################
  2158058.5 |########################################
  2161803.6 |
  2165548.7 |
  2169293.8 |
  2173038.9 |
  2176784.0 |
  2180529.0 |
  2184274.1 |
  2188019.2 |
  2191764.3 |
  2195509.4 |
  2199254.5 |
  2202999.5 |
  2206744.6 |
  2210489.7 |
  2214234.8 |
  2217979.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=3444.6% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)
