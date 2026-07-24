# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_scalar is an outlier: 608.3x slower than the field

abi_cross_cold_leaf_warm_scalar (1.55 ms) is 608.3x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_leaf_cold_scalar shows alternating (throttle bounce) (autocorr -0.51)

abi_cross_cold_leaf_cold_scalar's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (2.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} (56212% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} with a 56212% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 608.3x the fastest

Fastest abi_cross_cold_leaf_warm_null (2.54 us) to slowest abi_cross_cold_leaf_warm_scalar (1.55 ms): 608.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 2544.6 ns median
- 3 variants significantly slower than baseline
- Spread: 608.33x (fastest 2544.6 ns, slowest 1547947.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5117ns | 5048ns | 4793ns | 4977ns | 5488ns | +4.90% |
| abi_cross_cold_leaf_cold_scalar | 1539670ns | 1539887ns | 1507430ns | 1536053ns | 1561214ns | +31465.91% |
| abi_cross_cold_leaf_warm_null | 4878ns | 4879ns | 4692ns | 4841ns | 5025ns | base |
| abi_cross_cold_leaf_warm_scalar | 1539848ns | 1551590ns | 1499250ns | 1536451ns | 1565242ns | +31469.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 2778ns | 2630ns | 2973ns | +8.85% | 0.006 |
| abi_cross_cold_leaf_cold_scalar | 1536147ns | 1503745ns | 1557708ns | +60083.62% | 0.000 |
| abi_cross_cold_leaf_warm_null | 2552ns | 2489ns | 2621ns | base | 0.006 |
| abi_cross_cold_leaf_warm_scalar | 1536081ns | 1495859ns | 1561124ns | +60081.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 32790.2 | 2890.2 | 2778.2 | n/a |
| abi_cross_cold_leaf_cold_scalar | 73840.5 | 1536843.2 | 1536146.9 | n/a |
| abi_cross_cold_leaf_warm_null | 29292.5 | 2799.0 | 2552.4 | n/a |
| abi_cross_cold_leaf_warm_scalar | 71843.3 | 1537265.1 | 1536081.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.006 | 91.2% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.006 | 97.8% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5117ns | 5117ns | +4.90% |
| abi_cross_cold_leaf_cold_scalar | 1539670ns | 1539670ns | +31465.91% |
| abi_cross_cold_leaf_warm_null | 4878ns | 4878ns | base |
| abi_cross_cold_leaf_warm_scalar | 1539848ns | 1539848ns | +31469.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 2545ns | base | --- | [2491, 2621] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 2728ns | +133.1ns (+5.2%) | [+90, +454]ns | [2633, 2973] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1536263ns | +1533676.4ns (+60271.8%) | [+1511921, +1555186]ns | [1514470, 1557708] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1547947ns | +1545427.6ns (+60733.6%) | [+1496647, +1558513]ns | [1499173, 1561124] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 2489ns | +24.3% | +61557.2% | +62183.4% |
| 2 | 2683ns | +3.9% | +57208.3% | +58050.2% |
| 3 | 2540ns | +5.1% | +59111.9% | +61402.0% |
| 4 | 2493ns | +5.5% | +62208.3% | +60161.0% |
| 5 | 2560ns | +3.0% | +59487.2% | +58341.1% |
| 6 | 2550ns | +11.8% | +61160.0% | +60518.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.179 | ok |
| abi_cross_cold_leaf_cold_scalar | -0.510 | HIGH- (thermal bounce) |
| abi_cross_cold_leaf_warm_null | -0.388 | moderate- |
| abi_cross_cold_leaf_warm_scalar | 0.251 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 123714.5ns | 2778.2ns | 4453.0% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 4682521.7ns | 1536146.9ns | 304.8% | HIGH |
| abi_cross_cold_leaf_warm_null | 119283.2ns | 2552.4ns | 4673.3% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 4687771.4ns | 1536081.5ns | 305.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 2630.0-2973.3 ns)
   2630.0 |########################################
   2647.2 |
   2664.3 |####################
   2681.5 |
   2698.7 |
   2715.8 |
   2733.0 |
   2750.2 |
   2767.3 |
   2784.5 |####################
   2801.7 |
   2818.8 |
   2836.0 |####################
   2853.2 |
   2870.3 |
   2887.5 |
   2904.7 |
   2921.8 |
   2939.0 |
   2956.2 |
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1503745.4-1557707.7 ns)
  1503745.4 |########################################
  1506443.5 |
  1509141.6 |
  1511839.7 |
  1514537.9 |
  1517236.0 |
  1519934.1 |
  1522632.2 |########################################
  1525330.3 |
  1528028.4 |
  1530726.5 |
  1533424.7 |########################################
  1536122.8 |########################################
  1538820.9 |
  1541519.0 |
  1544217.1 |
  1546915.2 |
  1549613.4 |
  1552311.5 |########################################
  1555009.6 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 2489.2-2621.4 ns)
   2489.2 |########################################
   2495.8 |
   2502.4 |
   2509.0 |
   2515.6 |
   2522.3 |
   2528.9 |
   2535.5 |####################
   2542.1 |
   2548.7 |####################
   2555.3 |####################
   2561.9 |
   2568.5 |
   2575.2 |
   2581.8 |
   2588.4 |
   2595.0 |
   2601.6 |
   2608.2 |
   2614.8 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1495859.2-1561124.2 ns)
  1495859.2 |########################################
  1499122.4 |
  1502385.7 |########################################
  1505648.9 |
  1508912.2 |
  1512175.4 |
  1515438.7 |
  1518701.9 |
  1521965.2 |
  1525228.4 |
  1528491.7 |
  1531755.0 |
  1535018.2 |
  1538281.5 |
  1541544.7 |
  1544808.0 |########################################
  1548071.2 |########################################
  1551334.5 |
  1554597.7 |
  1557861.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=4512.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=305.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=4674.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=305.3% of algo (FFI overhead may distort results)
