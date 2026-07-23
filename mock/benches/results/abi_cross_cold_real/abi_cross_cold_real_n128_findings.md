# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_scalar is an outlier: 808.7x slower than the field

abi_cross_cold_real_warm_scalar (2.16 ms) is 808.7x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (77054% apart)

The field splits into a fast tier {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 77054% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 808.7x the fastest

Fastest abi_cross_cold_real_cold_null (2.67 us) to slowest abi_cross_cold_real_warm_scalar (2.16 ms): 808.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_real_cold_null** at 2665.0 ns median (-4.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 808.74x (fastest 2665.0 ns, slowest 2155304.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4934ns | 4911ns | 4806ns | 4884ns | 5074ns | -2.37% |
| abi_cross_cold_real_cold_scalar | 2155894ns | 2154645ns | 2150559ns | 2153584ns | 2162027ns | +42556.34% |
| abi_cross_cold_real_warm_null | 5054ns | 5095ns | 4815ns | 5069ns | 5151ns | base |
| abi_cross_cold_real_warm_scalar | 2157947ns | 2158249ns | 2153356ns | 2157322ns | 2161182ns | +42596.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2668ns | 2624ns | 2715ns | -3.25% | 0.048 |
| abi_cross_cold_real_cold_scalar | 2152926ns | 2147681ns | 2158855ns | +77967.25% | 0.000 |
| abi_cross_cold_real_warm_null | 2758ns | 2634ns | 2801ns | base | 0.046 |
| abi_cross_cold_real_warm_scalar | 2155016ns | 2150546ns | 2158115ns | +78043.05% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 30960.6 | 2709.0 | 2668.2 | 33 |
| abi_cross_cold_real_cold_scalar | 61150.1 | 2155717.5 | 2152925.6 | n/a |
| abi_cross_cold_real_warm_null | 27973.4 | 2761.2 | 2757.8 | n/a |
| abi_cross_cold_real_warm_scalar | 54185.8 | 2155585.4 | 2155015.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_real_cold_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.048 | 98.5% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.046 | 94.1% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 4934ns | 4934ns | -2.37% |
| abi_cross_cold_real_cold_scalar | 2155894ns | 2155894ns | +42556.34% |
| abi_cross_cold_real_warm_null | 5054ns | 5054ns | base |
| abi_cross_cold_real_warm_scalar | 2157947ns | 2157947ns | +42596.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2789ns | base | --- | [2683, 2801] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2665ns | -112.9ns (-4.0%) | [-126, -30]ns | [2625, 2715] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2151775ns | +2148999.6ns (+77054.1%) | [+2145434, +2156070]ns | [2148146, 2158855] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2155304ns | +2152544.5ns (+77181.2%) | [+2148916, +2155314]ns | [2151628, 2158115] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2634ns | -0.3% | +81430.7% | +81539.4% |
| 2 | 2793ns | -1.8% | +77032.9% | +77121.5% |
| 3 | 2810ns | -4.3% | +76389.5% | +76750.9% |
| 4 | 2787ns | -4.6% | +77513.2% | +77199.4% |
| 5 | 2732ns | -3.9% | +78770.4% | +78832.4% |
| 6 | 2791ns | -4.2% | +76889.1% | +77036.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.168 | ok |
| abi_cross_cold_real_cold_scalar | -0.264 | moderate- |
| abi_cross_cold_real_warm_null | -0.122 | ok |
| abi_cross_cold_real_warm_scalar | -0.134 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 6/6, lost 0/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 123000.7ns | 2668.2ns | 4609.9% | HIGH |
| abi_cross_cold_real_cold_scalar | 6526889.2ns | 2152925.6ns | 303.2% | HIGH |
| abi_cross_cold_real_warm_null | 120305.8ns | 2757.8ns | 4362.4% | HIGH |
| abi_cross_cold_real_warm_scalar | 6523990.9ns | 2155015.9ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2624.2-2714.8 ns)
   2624.2 |########################################
   2628.7 |
   2633.3 |
   2637.8 |
   2642.3 |
   2646.8 |
   2651.4 |
   2655.9 |####################
   2660.4 |
   2665.0 |
   2669.5 |####################
   2674.0 |
   2678.6 |
   2683.1 |
   2687.6 |####################
   2692.2 |
   2696.7 |
   2701.2 |
   2705.7 |
   2710.3 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2147680.8-2158855.5 ns)
  2147680.8 |####################
  2148239.5 |####################
  2148798.3 |####################
  2149357.0 |
  2149915.7 |
  2150474.5 |
  2151033.2 |
  2151591.9 |
  2152150.7 |
  2152709.4 |
  2153268.1 |
  2153826.9 |
  2154385.6 |########################################
  2154944.3 |
  2155503.1 |
  2156061.8 |
  2156620.5 |
  2157179.3 |
  2157738.0 |
  2158296.7 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2634.2-2801.4 ns)
   2634.2 |####################
   2642.6 |
   2650.9 |
   2659.3 |
   2667.6 |
   2676.0 |
   2684.4 |
   2692.7 |
   2701.1 |
   2709.5 |
   2717.8 |
   2726.2 |####################
   2734.5 |
   2742.9 |
   2751.3 |
   2759.6 |
   2768.0 |
   2776.4 |
   2784.7 |########################################
   2793.1 |####################
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2150546.2-2158115.2 ns)
  2150546.2 |########################################
  2150924.7 |
  2151303.1 |
  2151681.6 |
  2152060.0 |
  2152438.5 |########################################
  2152816.9 |
  2153195.4 |
  2153573.8 |
  2153952.2 |
  2154330.7 |########################################
  2154709.2 |
  2155087.6 |
  2155466.1 |
  2155844.5 |########################################
  2156223.0 |
  2156601.4 |
  2156979.9 |########################################
  2157358.3 |
  2157736.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=4599.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=4313.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=302.7% of algo (FFI overhead may distort results)
