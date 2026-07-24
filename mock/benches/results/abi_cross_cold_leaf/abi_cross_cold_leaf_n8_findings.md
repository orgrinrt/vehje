# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_null dominates: 15% faster than the next best (abi_cross_cold_leaf_cold_null)

abi_cross_cold_leaf_warm_null (3.08 us) leads abi_cross_cold_leaf_cold_null (3.54 us) by 15%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_leaf_warm_scalar is an outlier: 512.7x slower than the field

abi_cross_cold_leaf_warm_scalar (1.58 ms) is 512.7x the fastest (3.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_leaf_warm_null shows alternating (throttle bounce) (autocorr -0.52)

abi_cross_cold_leaf_warm_null's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (3.08 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} (43125% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} with a 43125% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 512.7x the fastest

Fastest abi_cross_cold_leaf_warm_null (3.08 us) to slowest abi_cross_cold_leaf_warm_scalar (1.58 ms): 512.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 3078.8 ns median
- 3 variants significantly slower than baseline
- Spread: 512.70x (fastest 3078.8 ns, slowest 1578489.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 6039ns | 5823ns | 5709ns | 5795ns | 6569ns | +10.27% |
| abi_cross_cold_leaf_cold_scalar | 1544579ns | 1532065ns | 1495950ns | 1526626ns | 1595824ns | +28104.11% |
| abi_cross_cold_leaf_warm_null | 5476ns | 5413ns | 5109ns | 5380ns | 5805ns | base |
| abi_cross_cold_leaf_warm_scalar | 1564018ns | 1582695ns | 1498108ns | 1563749ns | 1597375ns | +28459.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 3650ns | 3466ns | 3945ns | +17.06% | 0.002 |
| abi_cross_cold_leaf_cold_scalar | 1540820ns | 1492024ns | 1591846ns | +49317.46% | 0.000 |
| abi_cross_cold_leaf_warm_null | 3118ns | 2933ns | 3307ns | base | 0.003 |
| abi_cross_cold_leaf_warm_scalar | 1560115ns | 1494542ns | 1593367ns | +49936.30% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 35274.7 | 3912.9 | 3649.9 | n/a |
| abi_cross_cold_leaf_cold_scalar | 82810.9 | 1546909.4 | 1540820.0 | n/a |
| abi_cross_cold_leaf_warm_null | 29824.6 | 3219.9 | 3118.0 | n/a |
| abi_cross_cold_leaf_warm_scalar | 77735.6 | 1558926.5 | 1560115.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.002 | 82.9% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.003 | 95.3% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 6039ns | 6039ns | +10.27% |
| abi_cross_cold_leaf_cold_scalar | 1544579ns | 1544579ns | +28104.11% |
| abi_cross_cold_leaf_warm_null | 5476ns | 5476ns | base |
| abi_cross_cold_leaf_warm_scalar | 1564018ns | 1564018ns | +28459.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 3079ns | base | --- | [2968, 3307] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 3536ns | +474.3ns (+15.4%) | [+380, +742]ns | [3468, 3945] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1528553ns | +1525455.8ns (+49547.9%) | [+1498823, +1588827]ns | [1502061, 1591846] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1578490ns | +1575226.2ns (+51164.5%) | [+1505470, +1590295]ns | [1508489, 1593367] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 3141ns | +28.2% | +48778.6% | +50445.4% |
| 2 | 3104ns | +11.8% | +50497.5% | +48952.3% |
| 3 | 3003ns | +15.4% | +50247.9% | +53148.2% |
| 4 | 3473ns | +11.3% | +42861.9% | +45232.3% |
| 5 | 2933ns | +20.4% | +54899.4% | +50850.9% |
| 6 | 3054ns | +15.9% | +49737.1% | +51725.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.309 | moderate- |
| abi_cross_cold_leaf_cold_scalar | -0.465 | moderate- |
| abi_cross_cold_leaf_warm_null | -0.523 | HIGH- (thermal bounce) |
| abi_cross_cold_leaf_warm_scalar | -0.501 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 127949.0ns | 3649.9ns | 3505.5% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 4721138.0ns | 1540820.0ns | 306.4% | HIGH |
| abi_cross_cold_leaf_warm_null | 122004.2ns | 3118.0ns | 3912.9% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 4755781.8ns | 1560115.2ns | 304.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 3465.8-3945.2 ns)
   3465.8 |########################################
   3489.8 |
   3513.7 |####################
   3537.7 |####################
   3561.7 |
   3585.7 |
   3609.6 |
   3633.6 |
   3657.6 |
   3681.5 |
   3705.5 |
   3729.5 |
   3753.4 |
   3777.4 |
   3801.4 |
   3825.3 |
   3849.3 |####################
   3873.3 |
   3897.3 |
   3921.2 |
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1492024.2-1591845.9 ns)
  1492024.2 |########################################
  1497015.3 |
  1502006.4 |
  1506997.4 |
  1511988.5 |########################################
  1516979.6 |########################################
  1521970.7 |
  1526961.8 |
  1531952.9 |########################################
  1536943.9 |
  1541935.0 |
  1546926.1 |
  1551917.2 |
  1556908.3 |
  1561899.4 |
  1566890.4 |########################################
  1571881.5 |
  1576872.6 |
  1581863.7 |
  1586854.8 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 2933.3-3306.9 ns)
   2933.3 |########################################
   2952.0 |
   2970.7 |
   2989.3 |########################################
   3008.0 |
   3026.7 |
   3045.4 |########################################
   3064.0 |
   3082.7 |
   3101.4 |########################################
   3120.1 |
   3138.8 |########################################
   3157.4 |
   3176.1 |
   3194.8 |
   3213.5 |
   3232.1 |
   3250.8 |
   3269.5 |
   3288.2 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1494542.5-1593367.0 ns)
  1494542.5 |########################################
  1499483.7 |
  1504425.0 |
  1509366.2 |
  1514307.4 |
  1519248.6 |########################################
  1524189.9 |
  1529131.1 |
  1534072.3 |
  1539013.5 |
  1543954.8 |
  1548896.0 |
  1553837.2 |
  1558778.5 |
  1563719.7 |
  1568660.9 |
  1573602.1 |########################################
  1578543.4 |########################################
  1583484.6 |########################################
  1588425.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=3589.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=306.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=3955.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=305.1% of algo (FFI overhead may distort results)
