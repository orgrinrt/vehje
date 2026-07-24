# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_scalar is an outlier: 475.6x slower than the field

abi_cross_cold_leaf_warm_scalar (1.51 ms) is 475.6x the fastest (3.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_leaf_cold_null, abi_cross_cold_leaf_warm_null) are a dead heat (<1%)

abi_cross_cold_leaf_cold_null (3.17 us) and abi_cross_cold_leaf_warm_null (3.19 us) differ by 0.61%, inside the noise, even though the wider field spreads 47456.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {abi_cross_cold_leaf_cold_null, abi_cross_cold_leaf_warm_null} vs {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} (46421% apart)

The field splits into a fast tier {abi_cross_cold_leaf_cold_null, abi_cross_cold_leaf_warm_null} and a slow tier {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} with a 46421% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 475.6x the fastest

Fastest abi_cross_cold_leaf_cold_null (3.17 us) to slowest abi_cross_cold_leaf_warm_scalar (1.51 ms): 475.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_leaf_cold_null's edge over baseline is significant but tiny (-48 ns, 1.49%)

abi_cross_cold_leaf_cold_null differs from baseline abi_cross_cold_leaf_warm_null by -48 ns (1.49%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_leaf_cold_null** at 3173.9 ns median (-0.6% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 475.56x (fastest 3173.9 ns, slowest 1509417.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5527ns | 5477ns | 5310ns | 5429ns | 5783ns | +0.97% |
| abi_cross_cold_leaf_cold_scalar | 1494257ns | 1489074ns | 1476389ns | 1487497ns | 1513331ns | +27196.94% |
| abi_cross_cold_leaf_warm_null | 5474ns | 5480ns | 5416ns | 5463ns | 5519ns | base |
| abi_cross_cold_leaf_warm_scalar | 1513214ns | 1512989ns | 1486152ns | 1506392ns | 1536978ns | +27543.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 3186ns | 3107ns | 3275ns | +0.08% | 0.080 |
| abi_cross_cold_leaf_cold_scalar | 1490827ns | 1473259ns | 1509811ns | +46725.16% | 0.000 |
| abi_cross_cold_leaf_warm_null | 3184ns | 3121ns | 3225ns | base | 0.080 |
| abi_cross_cold_leaf_warm_scalar | 1509739ns | 1482903ns | 1533407ns | +47319.17% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 33091.0 | 3195.6 | 3186.5 | n/a |
| abi_cross_cold_leaf_cold_scalar | 70249.9 | 1490319.6 | 1490827.3 | n/a |
| abi_cross_cold_leaf_warm_null | 29201.5 | 3225.8 | 3183.8 | n/a |
| abi_cross_cold_leaf_warm_scalar | 67335.2 | 1509248.9 | 1509739.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_cross_cold_leaf_cold_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.081 | 97.9% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.080 | 97.3% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5527ns | 5527ns | +0.97% |
| abi_cross_cold_leaf_cold_scalar | 1494257ns | 1494257ns | +27196.94% |
| abi_cross_cold_leaf_warm_null | 5474ns | 5474ns | base |
| abi_cross_cold_leaf_warm_scalar | 1513214ns | 1513214ns | +27543.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 3193ns | base | --- | [3133, 3225] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 3174ns | no significant difference | [-87, +143]ns | [3110, 3275] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1485560ns | +1482394.4ns (+46422.0%) | [+1473918, +1506618]ns | [1477111, 1509811] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1509418ns | +1506284.8ns (+47170.2%) | [+1483182, +1530200]ns | [1486393, 1533407] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 3121ns | +2.4% | +47362.3% | +48492.1% |
| 2 | 3180ns | -2.1% | +47089.1% | +46745.8% |
| 3 | 3206ns | -0.9% | +47271.4% | +47261.8% |
| 4 | 3209ns | -3.2% | +46320.4% | +48145.6% |
| 5 | 3242ns | -2.2% | +45347.1% | +45644.6% |
| 6 | 3144ns | +6.7% | +47001.4% | +47676.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.021 | ok |
| abi_cross_cold_leaf_cold_scalar | 0.252 | moderate+ |
| abi_cross_cold_leaf_warm_null | -0.012 | ok |
| abi_cross_cold_leaf_warm_scalar | -0.289 | moderate- |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 4/6, lost 2/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 127014.3ns | 3186.5ns | 3986.1% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 4544731.4ns | 1490827.3ns | 304.8% | HIGH |
| abi_cross_cold_leaf_warm_null | 122404.2ns | 3183.8ns | 3844.6% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 4601261.4ns | 1509739.4ns | 304.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 3107.1-3275.4 ns)
   3107.1 |########################################
   3115.5 |
   3123.9 |
   3132.3 |
   3140.8 |
   3149.2 |
   3157.6 |
   3166.0 |####################
   3174.4 |####################
   3182.8 |
   3191.2 |####################
   3199.7 |
   3208.1 |
   3216.5 |
   3224.9 |
   3233.3 |
   3241.7 |
   3250.2 |
   3258.6 |
   3267.0 |
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1473259.2-1509811.0 ns)
  1473259.2 |####################
  1475086.8 |
  1476914.4 |
  1478742.0 |
  1480569.6 |########################################
  1482397.1 |
  1484224.7 |
  1486052.3 |
  1487879.9 |
  1489707.5 |####################
  1491535.1 |
  1493362.7 |
  1495190.3 |
  1497017.9 |
  1498845.5 |
  1500673.1 |####################
  1502500.6 |
  1504328.2 |
  1506155.8 |
  1507983.4 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 3121.2-3225.4 ns)
   3121.2 |####################
   3126.4 |
   3131.6 |
   3136.8 |
   3142.0 |####################
   3147.3 |
   3152.5 |
   3157.7 |
   3162.9 |
   3168.1 |
   3173.3 |
   3178.5 |####################
   3183.8 |
   3189.0 |
   3194.2 |
   3199.4 |
   3204.6 |########################################
   3209.8 |
   3215.0 |
   3220.2 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1482903.3-1533407.3 ns)
  1482903.3 |########################################
  1485428.5 |
  1487953.7 |########################################
  1490478.9 |
  1493004.1 |
  1495529.3 |
  1498054.5 |
  1500579.7 |########################################
  1503104.9 |
  1505630.1 |
  1508155.3 |
  1510680.5 |
  1513205.7 |
  1515730.9 |########################################
  1518256.1 |########################################
  1520781.3 |
  1523306.5 |
  1525831.7 |
  1528356.9 |
  1530882.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=4004.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=305.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=3822.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=304.8% of algo (FFI overhead may distort results)
