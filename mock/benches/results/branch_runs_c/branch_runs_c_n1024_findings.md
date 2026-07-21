# Branch strategies, cheap-arm, runs: correlated long runs (flip when b<24)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_runs**

## Key findings

- **Baseline (br_branch_c_runs) is the fastest** at 2183.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.33x (fastest 2183.8 ns, slowest 2906.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_runs | 5196ns | 5122ns | 4461ns | 5043ns | 5793ns | base |
| br_lut_c_runs | 5826ns | 5953ns | 5002ns | 5782ns | 6304ns | +12.13% |
| br_mask_c_runs | 5782ns | 5976ns | 5130ns | 5808ns | 6069ns | +11.28% |
| br_predicate_c_runs | 5702ns | 5969ns | 4172ns | 5796ns | 6326ns | +9.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_runs | 2175ns | 1864ns | 2375ns | base | 0.471 |
| br_lut_c_runs | 2815ns | 2422ns | 3044ns | +29.42% | 0.364 |
| br_mask_c_runs | 2803ns | 2451ns | 2951ns | +28.87% | 0.365 |
| br_predicate_c_runs | 2772ns | 2031ns | 3074ns | +27.44% | 0.369 |

## Performance model

- Peak throughput: **0.549 Gops/s** (br_branch_c_runs; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_runs | 0.469 | 85.4% |
| br_lut_c_runs | 0.356 | 64.8% |
| br_mask_c_runs | 0.352 | 64.1% |
| br_predicate_c_runs | 0.353 | 64.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_runs | 5196ns | 5196ns | base |
| br_lut_c_runs | 5826ns | 5826ns | +12.13% |
| br_mask_c_runs | 5782ns | 5782ns | +11.28% |
| br_predicate_c_runs | 5702ns | 5702ns | +9.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_runs | 2184ns | base | --- | [1967, 2375] | --- | --- | --- | --- |
| br_lut_c_runs | 2875ns | +600.4ns (+27.5%) | [+462, +857]ns | [2527, 3044] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_runs | 2907ns | +627.7ns (+28.7%) | [+492, +764]ns | [2553, 2951] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_runs | 2900ns | +624.8ns (+28.6%) | [+279, +887]ns | [2343, 3074] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_runs | br_lut_c_runs | br_mask_c_runs | br_predicate_c_runs |
|---|---|---|---|---|
| 1 | 1864ns | +29.9% | +31.5% | +9.0% |
| 2 | 2070ns | +27.2% | +28.2% | +28.2% |
| 3 | 2506ns | +14.6% | +16.0% | +15.6% |
| 4 | 2245ns | +43.1% | +33.4% | +44.6% |
| 5 | 2238ns | +28.5% | +29.9% | +29.7% |
| 6 | 2129ns | +35.1% | +36.6% | +36.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_runs | 0.098 | ok |
| br_lut_c_runs | 0.316 | moderate+ |
| br_mask_c_runs | 0.406 | moderate+ |
| br_predicate_c_runs | 0.252 | moderate+ |

**Consistency summary:**

- **br_lut_c_runs**: won 0/6, lost 6/6
- **br_mask_c_runs**: won 0/6, lost 6/6
- **br_predicate_c_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_runs | 4.7ns | 2175.4ns | 0.2% |  |
| br_lut_c_runs | 3.6ns | 2815.5ns | 0.1% |  |
| br_mask_c_runs | 4.3ns | 2803.4ns | 0.2% |  |
| br_predicate_c_runs | 4.3ns | 2772.3ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_c_runs (n=6, range 1864.2-2375.4 ns)
   1864.2 |####################
   1889.8 |
   1915.3 |
   1940.9 |
   1966.4 |
   1992.0 |
   2017.6 |
   2043.1 |
   2068.7 |####################
   2094.2 |
   2119.8 |####################
   2145.4 |
   2170.9 |
   2196.5 |
   2222.0 |########################################
   2247.6 |
   2273.2 |
   2298.7 |
   2324.3 |
   2349.8 |
  (0 below, 1 above range)

br_lut_c_runs (n=6, range 2421.7-3044.3 ns)
   2421.7 |#############
   2452.8 |
   2484.0 |
   2515.1 |
   2546.2 |
   2577.4 |
   2608.5 |#############
   2639.6 |
   2670.8 |
   2701.9 |
   2733.0 |
   2764.2 |
   2795.3 |
   2826.4 |
   2857.6 |########################################
   2888.7 |
   2919.8 |
   2951.0 |
   2982.1 |
   3013.2 |
  (0 below, 1 above range)

br_mask_c_runs (n=6, range 2451.2-2950.8 ns)
   2451.2 |#############
   2476.2 |
   2501.2 |
   2526.1 |
   2551.1 |
   2576.1 |
   2601.1 |
   2626.1 |
   2651.1 |#############
   2676.0 |
   2701.0 |
   2726.0 |
   2751.0 |
   2776.0 |
   2801.0 |
   2825.9 |
   2850.9 |
   2875.9 |
   2900.9 |########################################
   2925.9 |
  (0 below, 1 above range)

br_predicate_c_runs (n=6, range 2031.2-3073.9 ns)
   2031.2 |#############
   2083.3 |
   2135.5 |
   2187.6 |
   2239.8 |
   2291.9 |
   2344.0 |
   2396.2 |
   2448.3 |
   2500.4 |
   2552.6 |
   2604.7 |#############
   2656.8 |
   2709.0 |
   2761.1 |
   2813.3 |
   2865.4 |########################################
   2917.5 |
   2969.7 |
   3021.8 |
  (0 below, 1 above range)

```
