# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_null dominates: 221% faster than the next best (abi_cross_cold_leaf_cold_null)

abi_cross_cold_leaf_warm_null (6.03 us) leads abi_cross_cold_leaf_cold_null (19.37 us) by 221%, a clear separation rather than a photo finish. CV 21.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_leaf_cold_scalar is an outlier: 333.2x slower than the field

abi_cross_cold_leaf_cold_scalar (2.01 ms) is 333.2x the fastest (6.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (6.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} (10212% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} with a 10212% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 333.2x the fastest

Fastest abi_cross_cold_leaf_warm_null (6.03 us) to slowest abi_cross_cold_leaf_cold_scalar (2.01 ms): 333.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_leaf_cold_null is inconsistent: worst-20% is 1.5x its best-20%

abi_cross_cold_leaf_cold_null's best 20% of batches run at 17.72 us but its worst 20% at 27.27 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 6033.4 ns median
- 3 variants significantly slower than baseline
- Spread: 333.20x (fastest 6033.4 ns, slowest 2010299.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 24933ns | 22402ns | 20495ns | 22033ns | 31500ns | +156.70% |
| abi_cross_cold_leaf_cold_scalar | 2058979ns | 2016155ns | 1915902ns | 1983075ns | 2244372ns | +21099.09% |
| abi_cross_cold_leaf_warm_null | 9713ns | 8832ns | 8189ns | 8768ns | 11891ns | base |
| abi_cross_cold_leaf_warm_scalar | 2077250ns | 2003301ns | 1855855ns | 1971642ns | 2346360ns | +21287.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 21543ns | 17720ns | 27271ns | +231.77% | 0.000 |
| abi_cross_cold_leaf_cold_scalar | 2053135ns | 1910381ns | 2238298ns | +31518.72% | 0.000 |
| abi_cross_cold_leaf_warm_null | 6493ns | 5569ns | 7701ns | base | 0.000 |
| abi_cross_cold_leaf_warm_scalar | 2071203ns | 1850674ns | 2339754ns | +31796.97% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 61172.5 | 27214.9 | 21543.2 | n/a |
| abi_cross_cold_leaf_cold_scalar | 125756.9 | 2035074.6 | 2053135.1 | n/a |
| abi_cross_cold_leaf_warm_null | 42400.1 | 6666.8 | 6493.4 | n/a |
| abi_cross_cold_leaf_warm_scalar | 123562.5 | 2072414.2 | 2071203.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.000 | 28.8% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.3% |
| abi_cross_cold_leaf_warm_null | 0.000 | 92.3% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 24933ns | 24933ns | +156.70% |
| abi_cross_cold_leaf_cold_scalar | 2058979ns | 2058979ns | +21099.09% |
| abi_cross_cold_leaf_warm_null | 9713ns | 9713ns | base |
| abi_cross_cold_leaf_warm_scalar | 2077250ns | 2077250ns | +21287.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 6033ns | base | --- | [5745, 7701] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 19369ns | +13392.0ns (+222.0%) | [+11948, +19809]ns | [17989, 27271] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_cold_scalar | 2010299ns | +2004314.4ns (+33220.6%) | [+1903114, +2232497]ns | [1910808, 2238298] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1997418ns | +1991609.6ns (+33010.0%) | [+1870458, +2332061]ns | [1876436, 2339754] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 5569ns | +341.9% | +39211.2% | +34940.3% |
| 2 | 5922ns | +241.9% | +32507.0% | +32022.5% |
| 3 | 6034ns | +193.7% | +31573.4% | +30569.7% |
| 4 | 6048ns | +201.9% | +34452.6% | +33686.4% |
| 5 | 6032ns | +206.5% | +37815.9% | +41384.4% |
| 6 | 9355ns | +220.0% | +20321.0% | +23170.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.017 | ok |
| abi_cross_cold_leaf_cold_scalar | -0.226 | moderate- |
| abi_cross_cold_leaf_warm_null | -0.012 | ok |
| abi_cross_cold_leaf_warm_scalar | 0.336 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 198155.6ns | 21543.2ns | 919.8% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 6247622.7ns | 2053135.1ns | 304.3% | HIGH |
| abi_cross_cold_leaf_warm_null | 147466.7ns | 6493.4ns | 2271.0% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 6399492.8ns | 2071203.0ns | 309.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 17720.0-27271.2 ns)
  17720.0 |####################
  18197.6 |########################################
  18675.1 |
  19152.7 |
  19630.2 |
  20107.8 |####################
  20585.4 |
  21062.9 |
  21540.5 |
  22018.1 |
  22495.6 |
  22973.2 |
  23450.8 |
  23928.3 |
  24405.9 |####################
  24883.4 |
  25361.0 |
  25838.6 |
  26316.1 |
  26793.7 |
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1910380.8-2238297.5 ns)
  1910380.8 |########################################
  1926776.6 |####################
  1943172.5 |
  1959568.3 |
  1975964.1 |
  1992360.0 |
  2008755.8 |
  2025151.6 |
  2041547.5 |
  2057943.3 |
  2074339.1 |####################
  2090735.0 |
  2107130.8 |
  2123526.7 |
  2139922.5 |
  2156318.3 |
  2172714.2 |
  2189110.0 |####################
  2205505.8 |
  2221901.7 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 5569.2-7701.4 ns)
   5569.2 |#############
   5675.8 |
   5782.4 |
   5889.0 |#############
   5995.6 |########################################
   6102.3 |
   6208.9 |
   6315.5 |
   6422.1 |
   6528.7 |
   6635.3 |
   6741.9 |
   6848.5 |
   6955.2 |
   7061.8 |
   7168.4 |
   7275.0 |
   7381.6 |
   7488.2 |
   7594.8 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1850673.7-2339754.4 ns)
  1850673.7 |########################################
  1875127.7 |
  1899581.8 |########################################
  1924035.8 |
  1948489.8 |########################################
  1972943.9 |
  1997397.9 |
  2021851.9 |########################################
  2046306.0 |
  2070760.0 |
  2095214.0 |
  2119668.1 |
  2144122.1 |
  2168576.1 |########################################
  2193030.2 |
  2217484.2 |
  2241938.2 |
  2266392.3 |
  2290846.3 |
  2315300.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: CV=20.4% (high variance, measurements may be unstable)
- **abi_cross_cold_leaf_cold_null**: bridge=957.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=305.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=2293.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=304.8% of algo (FFI overhead may distort results)
