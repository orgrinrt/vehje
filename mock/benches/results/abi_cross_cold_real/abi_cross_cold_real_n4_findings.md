# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 20% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (4.12 us) leads abi_cross_cold_real_cold_null (4.95 us) by 20%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_warm_scalar is an outlier: 529.8x slower than the field

abi_cross_cold_real_warm_scalar (2.18 ms) is 529.8x the fastest (4.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (4.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (43958% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 43958% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 529.8x the fastest

Fastest abi_cross_cold_real_warm_null (4.12 us) to slowest abi_cross_cold_real_warm_scalar (2.18 ms): 529.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 4122.3 ns median
- 3 variants significantly slower than baseline
- Spread: 529.78x (fastest 4122.3 ns, slowest 2183911.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 7197ns | 7200ns | 6979ns | 7185ns | 7325ns | +11.85% |
| abi_cross_cold_real_cold_scalar | 2189777ns | 2185951ns | 2180513ns | 2184870ns | 2201769ns | +33930.58% |
| abi_cross_cold_real_warm_null | 6435ns | 6450ns | 6194ns | 6392ns | 6619ns | base |
| abi_cross_cold_real_warm_scalar | 2186326ns | 2187248ns | 2182411ns | 2186588ns | 2187889ns | +33876.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4945ns | 4802ns | 5033ns | +20.92% | 0.001 |
| abi_cross_cold_real_cold_scalar | 2186412ns | 2177358ns | 2198190ns | +53362.08% | 0.000 |
| abi_cross_cold_real_warm_null | 4090ns | 3924ns | 4197ns | base | 0.001 |
| abi_cross_cold_real_warm_scalar | 2183019ns | 2178957ns | 2184508ns | +53279.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33423.7 | 11035.6 | 4945.3 | n/a |
| abi_cross_cold_real_cold_scalar | 69233.9 | 2184577.4 | 2186411.9 | n/a |
| abi_cross_cold_real_warm_null | 29012.9 | 4227.2 | 4089.7 | n/a |
| abi_cross_cold_real_warm_scalar | 63461.1 | 2183713.5 | 2183019.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.001 | 79.2% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_real_warm_null | 0.001 | 95.2% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 7197ns | 7197ns | +11.85% |
| abi_cross_cold_real_cold_scalar | 2189777ns | 2189777ns | +33930.58% |
| abi_cross_cold_real_warm_null | 6435ns | 6435ns | base |
| abi_cross_cold_real_warm_scalar | 2186326ns | 2186326ns | +33876.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 4122ns | base | --- | [3949, 4197] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 4954ns | +820.7ns (+19.9%) | [+775, +972]ns | [4849, 5033] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2182722ns | +2178708.1ns (+52851.8%) | [+2174266, +2193992]ns | [2178324, 2198190] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2183911ns | +2179808.1ns (+52878.4%) | [+2176516, +2180464]ns | [2180638, 2184508] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 3924ns | +22.4% | +55527.1% | +55576.3% |
| 2 | 4141ns | +20.4% | +52483.0% | +52602.8% |
| 3 | 4163ns | +18.2% | +52612.4% | +52367.7% |
| 4 | 4104ns | +19.3% | +53088.4% | +52996.1% |
| 5 | 3975ns | +26.8% | +54724.9% | +54832.7% |
| 6 | 4231ns | +18.8% | +51937.4% | +51522.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.070 | ok |
| abi_cross_cold_real_cold_scalar | -0.332 | moderate- |
| abi_cross_cold_real_warm_null | -0.312 | moderate- |
| abi_cross_cold_real_warm_scalar | -0.396 | moderate- |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 101737.8ns | 4945.3ns | 2057.2% | HIGH |
| abi_cross_cold_real_cold_scalar | 6634014.1ns | 2186411.9ns | 303.4% | HIGH |
| abi_cross_cold_real_warm_null | 123390.8ns | 4089.7ns | 3017.1% | HIGH |
| abi_cross_cold_real_warm_scalar | 6616712.9ns | 2183019.0ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 4802.5-5032.9 ns)
   4802.5 |########################################
   4814.0 |
   4825.5 |
   4837.1 |
   4848.6 |
   4860.1 |
   4871.6 |
   4883.1 |
   4894.7 |########################################
   4906.2 |
   4917.7 |########################################
   4929.2 |
   4940.7 |
   4952.3 |
   4963.8 |
   4975.3 |
   4986.8 |########################################
   4998.3 |
   5009.9 |
   5021.4 |########################################
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2177357.5-2198189.5 ns)
  2177357.5 |####################
  2178399.1 |####################
  2179440.7 |
  2180482.3 |
  2181523.9 |
  2182565.5 |########################################
  2183607.1 |
  2184648.7 |
  2185690.3 |
  2186731.9 |
  2187773.5 |
  2188815.1 |
  2189856.7 |
  2190898.3 |
  2191939.9 |
  2192981.5 |
  2194023.1 |####################
  2195064.7 |
  2196106.3 |
  2197147.9 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 3923.8-4197.2 ns)
   3923.8 |########################################
   3937.5 |
   3951.1 |
   3964.8 |########################################
   3978.5 |
   3992.2 |
   4005.8 |
   4019.5 |
   4033.2 |
   4046.9 |
   4060.5 |
   4074.2 |
   4087.9 |
   4101.5 |########################################
   4115.2 |
   4128.9 |########################################
   4142.6 |
   4156.2 |########################################
   4169.9 |
   4183.6 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2178956.7-2184507.5 ns)
  2178956.7 |####################
  2179234.2 |
  2179511.8 |
  2179789.3 |
  2180066.9 |
  2180344.4 |
  2180621.9 |
  2180899.5 |
  2181177.0 |
  2181454.6 |
  2181732.1 |
  2182009.6 |
  2182287.2 |####################
  2182564.7 |
  2182842.3 |
  2183119.8 |
  2183397.3 |####################
  2183674.9 |
  2183952.4 |
  2184230.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=2069.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=2981.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.0% of algo (FFI overhead may distort results)
