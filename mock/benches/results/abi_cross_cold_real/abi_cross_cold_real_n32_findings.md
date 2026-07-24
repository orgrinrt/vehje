# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_scalar is an outlier: 973.7x slower than the field

abi_cross_cold_real_warm_scalar (2.20 ms) is 973.7x the fastest (2.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (2.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (89892% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 89892% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 973.7x the fastest

Fastest abi_cross_cold_real_warm_null (2.26 us) to slowest abi_cross_cold_real_warm_scalar (2.20 ms): 973.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 2262.3 ns median
- 3 variants significantly slower than baseline
- Spread: 973.69x (fastest 2262.3 ns, slowest 2202773.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4826ns | 4758ns | 4714ns | 4745ns | 5004ns | +5.50% |
| abi_cross_cold_real_cold_scalar | 2187275ns | 2188074ns | 2175324ns | 2186580ns | 2194293ns | +47716.43% |
| abi_cross_cold_real_warm_null | 4574ns | 4537ns | 4391ns | 4519ns | 4749ns | base |
| abi_cross_cold_real_warm_scalar | 2213434ns | 2206468ns | 2183188ns | 2201557ns | 2246372ns | +48288.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2450ns | 2407ns | 2509ns | +6.58% | 0.013 |
| abi_cross_cold_real_cold_scalar | 2183707ns | 2172250ns | 2190640ns | +94909.86% | 0.000 |
| abi_cross_cold_real_warm_null | 2298ns | 2213ns | 2404ns | base | 0.014 |
| abi_cross_cold_real_warm_scalar | 2209643ns | 2179690ns | 2242047ns | +96038.32% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 35128.9 | 2651.6 | 2449.7 | n/a |
| abi_cross_cold_real_cold_scalar | 75141.4 | 2181893.1 | 2183706.7 | n/a |
| abi_cross_cold_real_warm_null | 28826.4 | 2422.3 | 2298.4 | n/a |
| abi_cross_cold_real_warm_scalar | 77651.3 | 2210303.9 | 2209643.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.013 | 91.2% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.014 | 97.8% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 4826ns | 4826ns | +5.50% |
| abi_cross_cold_real_cold_scalar | 2187275ns | 2187275ns | +47716.43% |
| abi_cross_cold_real_warm_null | 4574ns | 4574ns | base |
| abi_cross_cold_real_warm_scalar | 2213434ns | 2213434ns | +48288.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2262ns | base | --- | [2229, 2404] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2427ns | +162.1ns (+7.2%) | [+72, +219]ns | [2412, 2509] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2184384ns | +2181980.8ns (+96449.7%) | [+2173842, +2188402]ns | [2176096, 2190640] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2202773ns | +2200544.0ns (+97270.2%) | [+2181743, +2239747]ns | [2184109, 2242047] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2246ns | +8.0% | +96625.0% | +97974.2% |
| 2 | 2262ns | +6.4% | +96285.1% | +97514.8% |
| 3 | 2339ns | +3.4% | +93126.1% | +97229.4% |
| 4 | 2468ns | +2.7% | +88560.0% | +88207.4% |
| 5 | 2263ns | +9.8% | +96771.5% | +96613.4% |
| 6 | 2213ns | +9.8% | +98827.9% | +99452.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.165 | ok |
| abi_cross_cold_real_cold_scalar | 0.449 | moderate+ |
| abi_cross_cold_real_warm_null | 0.100 | ok |
| abi_cross_cold_real_warm_scalar | -0.227 | moderate- |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 124785.5ns | 2449.7ns | 5094.0% | HIGH |
| abi_cross_cold_real_cold_scalar | 6629106.7ns | 2183706.7ns | 303.6% | HIGH |
| abi_cross_cold_real_warm_null | 118485.8ns | 2298.4ns | 5155.1% | HIGH |
| abi_cross_cold_real_warm_scalar | 6710351.3ns | 2209643.0ns | 303.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2407.1-2509.2 ns)
   2407.1 |########################################
   2412.2 |
   2417.3 |########################################
   2422.4 |########################################
   2427.5 |########################################
   2432.6 |
   2437.7 |
   2442.8 |
   2447.9 |
   2453.0 |
   2458.1 |
   2463.3 |
   2468.4 |
   2473.5 |
   2478.6 |
   2483.7 |########################################
   2488.8 |
   2493.9 |
   2499.0 |
   2504.1 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2172250.4-2190639.8 ns)
  2172250.4 |####################
  2173169.9 |
  2174089.3 |
  2175008.8 |
  2175928.3 |
  2176847.8 |
  2177767.2 |
  2178686.7 |
  2179606.2 |########################################
  2180525.6 |
  2181445.1 |
  2182364.6 |
  2183284.0 |
  2184203.5 |
  2185123.0 |
  2186042.4 |
  2186961.9 |
  2187881.4 |####################
  2188800.9 |####################
  2189720.3 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2212.9-2403.6 ns)
   2212.9 |####################
   2222.4 |
   2232.0 |
   2241.5 |####################
   2251.0 |
   2260.6 |########################################
   2270.1 |
   2279.6 |
   2289.2 |
   2298.7 |
   2308.2 |
   2317.8 |
   2327.3 |
   2336.8 |####################
   2346.4 |
   2355.9 |
   2365.4 |
   2375.0 |
   2384.5 |
   2394.0 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2179690.4-2242046.9 ns)
  2179690.4 |####################
  2182808.2 |
  2185926.0 |####################
  2189043.9 |
  2192161.7 |
  2195279.5 |
  2198397.4 |
  2201515.2 |########################################
  2204633.0 |
  2207750.8 |####################
  2210868.7 |
  2213986.5 |
  2217104.3 |
  2220222.1 |
  2223340.0 |
  2226457.8 |
  2229575.6 |
  2232693.4 |
  2235811.3 |
  2238929.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=5127.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=5175.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
