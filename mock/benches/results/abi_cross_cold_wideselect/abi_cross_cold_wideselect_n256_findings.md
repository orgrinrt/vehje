# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_cold_scalar is an outlier: 666.9x slower than the field

abi_cross_cold_wideselect_cold_scalar (2.10 ms) is 666.9x the fastest (3.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_wideselect_cold_null, abi_cross_cold_wideselect_warm_null} vs {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} (64996% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_cold_null, abi_cross_cold_wideselect_warm_null} and a slow tier {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} with a 64996% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 666.9x the fastest

Fastest abi_cross_cold_wideselect_cold_null (3.16 us) to slowest abi_cross_cold_wideselect_cold_scalar (2.10 ms): 666.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_wideselect_cold_null's edge over baseline is significant but tiny (-38 ns, 1.19%)

abi_cross_cold_wideselect_cold_null differs from baseline abi_cross_cold_wideselect_warm_null by -38 ns (1.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_wideselect_cold_null** at 3156.2 ns median (-1.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 666.86x (fastest 3156.2 ns, slowest 2104763.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 5444ns | 5410ns | 5305ns | 5382ns | 5607ns | -4.11% |
| abi_cross_cold_wideselect_cold_scalar | 2118228ns | 2108445ns | 2099348ns | 2108327ns | 2142522ns | +37207.86% |
| abi_cross_cold_wideselect_warm_null | 5678ns | 5542ns | 5435ns | 5527ns | 6025ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2169553ns | 2092752ns | 2089736ns | 2091769ns | 2326136ns | +38111.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 3161ns | 3095ns | 3219ns | -5.51% | 0.081 |
| abi_cross_cold_wideselect_cold_scalar | 2114487ns | 2095681ns | 2138656ns | +63097.61% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 3346ns | 3140ns | 3683ns | base | 0.077 |
| abi_cross_cold_wideselect_warm_scalar | 2165927ns | 2086282ns | 2322263ns | +64635.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 33072.2 | 3172.2 | 3161.3 | 7 |
| abi_cross_cold_wideselect_cold_scalar | 83682.2 | 2112359.4 | 2114486.8 | n/a |
| abi_cross_cold_wideselect_warm_null | 31802.2 | 3223.4 | 3345.8 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 74451.6 | 2115124.2 | 2165927.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_cross_cold_wideselect_cold_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.081 | 98.1% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_wideselect_warm_null | 0.080 | 96.4% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 5444ns | 5444ns | -4.11% |
| abi_cross_cold_wideselect_cold_scalar | 2118228ns | 2118228ns | +37207.86% |
| abi_cross_cold_wideselect_warm_null | 5678ns | 5678ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2169553ns | 2169553ns | +38111.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 3209ns | base | --- | [3145, 3683] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 3156ns | -38.1ns (-1.2%) | [-504, -12]ns | [3109, 3219] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2104764ns | +2101570.0ns (+65482.7%) | [+2096813, +2135040]ns | [2100041, 2138656] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2089170ns | +2085533.4ns (+64983.0%) | [+2083136, +2319074]ns | [2086348, 2322263] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 3140ns | -1.4% | +67106.6% | +66602.4% |
| 2 | 3181ns | +0.1% | +65777.1% | +65578.1% |
| 3 | 3238ns | -3.4% | +64906.0% | +78658.4% |
| 4 | 4092ns | -21.9% | +52849.8% | +50949.3% |
| 5 | 3274ns | -1.0% | +64172.2% | +63622.9% |
| 6 | 3150ns | -0.8% | +66732.5% | +66139.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | -0.254 | moderate- |
| abi_cross_cold_wideselect_cold_scalar | -0.203 | moderate- |
| abi_cross_cold_wideselect_warm_null | -0.101 | ok |
| abi_cross_cold_wideselect_warm_scalar | -0.232 | moderate- |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 5/6, lost 1/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 126417.5ns | 3161.3ns | 3998.9% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6422106.9ns | 2114486.8ns | 303.7% | HIGH |
| abi_cross_cold_wideselect_warm_null | 128385.8ns | 3345.8ns | 3837.2% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6427435.6ns | 2165927.1ns | 296.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 3095.4-3218.6 ns)
   3095.4 |########################################
   3101.6 |
   3107.7 |
   3113.9 |
   3120.0 |########################################
   3126.2 |########################################
   3132.3 |
   3138.5 |
   3144.7 |
   3150.8 |
   3157.0 |
   3163.1 |
   3169.3 |
   3175.4 |
   3181.6 |########################################
   3187.8 |
   3193.9 |########################################
   3200.1 |
   3206.2 |
   3212.4 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2095681.2-2138655.9 ns)
  2095681.2 |#############
  2097829.9 |
  2099978.7 |
  2102127.4 |
  2104276.1 |########################################
  2106424.9 |
  2108573.6 |#############
  2110722.3 |
  2112871.1 |
  2115019.8 |
  2117168.5 |
  2119317.3 |
  2121466.0 |
  2123614.7 |
  2125763.5 |
  2127912.2 |
  2130060.9 |
  2132209.7 |
  2134358.4 |
  2136507.1 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 3140.4-3683.1 ns)
   3140.4 |########################################
   3167.5 |####################
   3194.7 |
   3221.8 |####################
   3248.9 |####################
   3276.1 |
   3303.2 |
   3330.4 |
   3357.5 |
   3384.6 |
   3411.8 |
   3438.9 |
   3466.0 |
   3493.2 |
   3520.3 |
   3547.5 |
   3574.6 |
   3601.7 |
   3628.9 |
   3656.0 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2086282.1-2322263.1 ns)
  2086282.1 |########################################
  2098081.1 |
  2109880.2 |
  2121679.2 |
  2133478.3 |
  2145277.4 |
  2157076.4 |
  2168875.5 |
  2180674.5 |
  2192473.6 |
  2204272.6 |
  2216071.6 |
  2227870.7 |
  2239669.8 |
  2251468.8 |
  2263267.9 |
  2275066.9 |
  2286866.0 |
  2298665.0 |
  2310464.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=4003.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=3827.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=303.6% of algo (FFI overhead may distort results)
