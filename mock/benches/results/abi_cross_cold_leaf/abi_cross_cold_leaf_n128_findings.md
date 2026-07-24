# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_cold_scalar is an outlier: 559.5x slower than the field

abi_cross_cold_leaf_cold_scalar (1.52 ms) is 559.5x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_leaf_cold_null, abi_cross_cold_leaf_warm_null} vs {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} (53791% apart)

The field splits into a fast tier {abi_cross_cold_leaf_cold_null, abi_cross_cold_leaf_warm_null} and a slow tier {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} with a 53791% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 559.5x the fastest

Fastest abi_cross_cold_leaf_cold_null (2.72 us) to slowest abi_cross_cold_leaf_cold_scalar (1.52 ms): 559.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_leaf_cold_null** at 2718.5 ns median (-2.0% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 559.54x (fastest 2718.5 ns, slowest 1521112.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5187ns | 5044ns | 4790ns | 5009ns | 5652ns | -3.14% |
| abi_cross_cold_leaf_cold_scalar | 1592299ns | 1524573ns | 1463939ns | 1513931ns | 1774030ns | +29634.35% |
| abi_cross_cold_leaf_warm_null | 5355ns | 5211ns | 4803ns | 5128ns | 5972ns | base |
| abi_cross_cold_leaf_warm_scalar | 1675973ns | 1498549ns | 1488320ns | 1495675ns | 2040247ns | +31196.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 2794ns | 2590ns | 3048ns | -2.88% | 0.046 |
| abi_cross_cold_leaf_cold_scalar | 1588497ns | 1460790ns | 1769364ns | +55117.51% | 0.000 |
| abi_cross_cold_leaf_warm_null | 2877ns | 2628ns | 3184ns | base | 0.044 |
| abi_cross_cold_leaf_warm_scalar | 1672055ns | 1485330ns | 2035059ns | +58022.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 34812.2 | 2845.6 | 2794.1 | 27 |
| abi_cross_cold_leaf_cold_scalar | 74880.1 | 1593561.7 | 1588497.2 | n/a |
| abi_cross_cold_leaf_warm_null | 32541.6 | 2972.0 | 2876.8 | n/a |
| abi_cross_cold_leaf_warm_scalar | 72996.8 | 1687817.3 | 1672055.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_leaf_cold_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.047 | 95.3% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.046 | 93.3% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5187ns | 5187ns | -3.14% |
| abi_cross_cold_leaf_cold_scalar | 1592299ns | 1592299ns | +29634.35% |
| abi_cross_cold_leaf_warm_null | 5355ns | 5355ns | base |
| abi_cross_cold_leaf_warm_scalar | 1675973ns | 1675973ns | +31196.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 2775ns | base | --- | [2672, 3184] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 2718ns | no significant difference | [-153, +6]ns | [2616, 3048] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1521113ns | +1518338.3ns (+54722.8%) | [+1472343, +1766180]ns | [1475015, 1769364] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1495251ns | +1492494.2ns (+53791.3%) | [+1483166, +2031875]ns | [1485856, 2035059] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 2628ns | +2.7% | +55487.7% | +56421.6% |
| 2 | 2752ns | -5.9% | +55093.6% | +53909.0% |
| 3 | 2797ns | -2.1% | +54358.0% | +53605.8% |
| 4 | 2716ns | -2.8% | +54728.1% | +54693.4% |
| 5 | 3049ns | -4.2% | +51880.0% | +74079.4% |
| 6 | 3318ns | -4.4% | +58778.2% | +54393.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.288 | moderate+ |
| abi_cross_cold_leaf_cold_scalar | 0.112 | ok |
| abi_cross_cold_leaf_warm_null | 0.306 | moderate+ |
| abi_cross_cold_leaf_warm_scalar | 0.139 | ok |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 5/6, lost 1/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 127014.2ns | 2794.1ns | 4545.8% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 4860404.0ns | 1588497.2ns | 306.0% | HIGH |
| abi_cross_cold_leaf_warm_null | 125194.6ns | 2876.8ns | 4351.9% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 5104536.9ns | 1672055.3ns | 305.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 2590.0-3048.2 ns)
   2590.0 |########################################
   2612.9 |
   2635.8 |########################################
   2658.7 |
   2681.6 |########################################
   2704.5 |
   2727.4 |########################################
   2750.4 |
   2773.3 |
   2796.2 |
   2819.1 |
   2842.0 |
   2864.9 |
   2887.8 |
   2910.7 |########################################
   2933.6 |
   2956.5 |
   2979.4 |
   3002.3 |
   3025.2 |
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1460790.4-1769363.5 ns)
  1460790.4 |########################################
  1476219.1 |########################################
  1491647.7 |
  1507076.4 |########################################
  1522505.0 |########################################
  1537933.7 |
  1553362.3 |
  1568791.0 |
  1584219.7 |########################################
  1599648.3 |
  1615077.0 |
  1630505.6 |
  1645934.3 |
  1661362.9 |
  1676791.6 |
  1692220.3 |
  1707648.9 |
  1723077.6 |
  1738506.2 |
  1753934.9 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 2627.9-3183.8 ns)
   2627.9 |########################################
   2655.7 |
   2683.5 |
   2711.3 |########################################
   2739.1 |########################################
   2766.9 |
   2794.7 |########################################
   2822.4 |
   2850.2 |
   2878.0 |
   2905.8 |
   2933.6 |
   2961.4 |
   2989.2 |
   3017.0 |
   3044.8 |########################################
   3072.6 |
   3100.4 |
   3128.2 |
   3156.0 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1485330.0-2035059.0 ns)
  1485330.0 |########################################
  1512816.4 |
  1540302.9 |
  1567789.3 |
  1595275.8 |
  1622762.2 |
  1650248.7 |
  1677735.1 |
  1705221.6 |
  1732708.0 |
  1760194.5 |
  1787680.9 |##########
  1815167.4 |
  1842653.8 |
  1870140.3 |
  1897626.7 |
  1925113.2 |
  1952599.6 |
  1980086.1 |
  2007572.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=4664.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=4445.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=304.7% of algo (FFI overhead may distort results)
