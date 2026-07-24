# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_scalar is an outlier: 601.1x slower than the field

abi_cross_cold_leaf_warm_scalar (1.50 ms) is 601.1x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null) are a dead heat (<1%)

abi_cross_cold_leaf_warm_null (2.49 us) and abi_cross_cold_leaf_cold_null (2.51 us) differ by 0.73%, inside the noise, even though the wider field spreads 60012.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### abi_cross_cold_leaf_cold_null shows alternating (throttle bounce) (autocorr -0.62)

abi_cross_cold_leaf_cold_null's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (2.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} (59474% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} with a 59474% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 601.1x the fastest

Fastest abi_cross_cold_leaf_warm_null (2.49 us) to slowest abi_cross_cold_leaf_warm_scalar (1.50 ms): 601.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_leaf_cold_null's edge over baseline is significant but tiny (15 ns, 0.60%)

abi_cross_cold_leaf_cold_null differs from baseline abi_cross_cold_leaf_warm_null by 15 ns (0.60%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 2493.9 ns median
- 2 variants significantly slower than baseline
- Spread: 601.13x (fastest 2493.9 ns, slowest 1499182.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 4761ns | 4744ns | 4630ns | 4735ns | 4865ns | -0.03% |
| abi_cross_cold_leaf_cold_scalar | 1500909ns | 1499731ns | 1482289ns | 1494908ns | 1519220ns | +31414.27% |
| abi_cross_cold_leaf_warm_null | 4763ns | 4769ns | 4663ns | 4754ns | 4825ns | base |
| abi_cross_cold_leaf_warm_scalar | 1511197ns | 1502269ns | 1485202ns | 1498406ns | 1543381ns | +31630.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 2500ns | 2440ns | 2547ns | +0.56% | 0.026 |
| abi_cross_cold_leaf_cold_scalar | 1497656ns | 1479306ns | 1515914ns | +60131.10% | 0.000 |
| abi_cross_cold_leaf_warm_null | 2487ns | 2422ns | 2515ns | base | 0.026 |
| abi_cross_cold_leaf_warm_scalar | 1507932ns | 1481969ns | 1539801ns | +60544.36% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 32252.8 | 2717.8 | 2500.3 | n/a |
| abi_cross_cold_leaf_cold_scalar | 64698.9 | 1499411.7 | 1497656.5 | n/a |
| abi_cross_cold_leaf_warm_null | 28272.6 | 2732.3 | 2486.5 | n/a |
| abi_cross_cold_leaf_warm_scalar | 59185.9 | 1507340.3 | 1507932.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.025 | 96.4% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.026 | 97.1% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 4761ns | 4761ns | -0.03% |
| abi_cross_cold_leaf_cold_scalar | 1500909ns | 1500909ns | +31414.27% |
| abi_cross_cold_leaf_warm_null | 4763ns | 4763ns | base |
| abi_cross_cold_leaf_warm_scalar | 1511197ns | 1511197ns | +31630.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 2494ns | base | --- | [2450, 2515] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 2512ns | no significant difference | [-57, +83]ns | [2442, 2547] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1496534ns | +1494067.1ns (+59907.7%) | [+1478012, +1513430]ns | [1480521, 1515914] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1499182ns | +1496698.1ns (+60013.2%) | [+1482353, +1537285]ns | [1484813, 1539801] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 2490ns | +2.2% | +60939.8% | +59725.3% |
| 2 | 2512ns | -0.8% | +59455.4% | +60370.3% |
| 3 | 2498ns | +2.1% | +59112.5% | +59219.1% |
| 4 | 2478ns | -1.6% | +60916.9% | +60786.7% |
| 5 | 2422ns | +4.6% | +61695.1% | +61320.2% |
| 6 | 2518ns | -3.0% | +58738.7% | +61857.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.616 | HIGH- (thermal bounce) |
| abi_cross_cold_leaf_cold_scalar | -0.210 | moderate- |
| abi_cross_cold_leaf_warm_null | -0.203 | moderate- |
| abi_cross_cold_leaf_warm_scalar | -0.376 | moderate- |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 3/6, lost 3/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 119709.7ns | 2500.3ns | 4787.7% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 4563573.8ns | 1497656.5ns | 304.7% | HIGH |
| abi_cross_cold_leaf_warm_null | 114021.3ns | 2486.5ns | 4585.6% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 4589953.8ns | 1507932.0ns | 304.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 2439.6-2547.3 ns)
   2439.6 |########################################
   2445.0 |
   2450.4 |
   2455.8 |
   2461.1 |
   2466.5 |
   2471.9 |
   2477.3 |
   2482.7 |
   2488.1 |####################
   2493.4 |
   2498.8 |
   2504.2 |
   2509.6 |
   2515.0 |
   2520.4 |
   2525.8 |
   2531.1 |####################
   2536.5 |
   2541.9 |####################
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1479306.2-1515914.1 ns)
  1479306.2 |####################
  1481136.6 |####################
  1482967.0 |
  1484797.4 |
  1486627.8 |
  1488458.2 |
  1490288.6 |
  1492119.0 |
  1493949.4 |
  1495779.8 |########################################
  1497610.2 |
  1499440.6 |
  1501271.0 |
  1503101.4 |
  1504931.8 |
  1506762.2 |
  1508592.6 |
  1510423.0 |####################
  1512253.4 |
  1514083.8 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 2422.1-2515.4 ns)
   2422.1 |########################################
   2426.8 |
   2431.4 |
   2436.1 |
   2440.8 |
   2445.4 |
   2450.1 |
   2454.8 |
   2459.4 |
   2464.1 |
   2468.8 |
   2473.4 |
   2478.1 |########################################
   2482.7 |
   2487.4 |########################################
   2492.1 |
   2496.7 |########################################
   2501.4 |
   2506.1 |
   2510.7 |########################################
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1481968.8-1539800.9 ns)
  1481968.8 |########################################
  1484860.4 |########################################
  1487752.0 |########################################
  1490643.6 |
  1493535.2 |
  1496426.8 |
  1499318.4 |
  1502210.0 |
  1505101.6 |
  1507993.2 |########################################
  1510884.8 |
  1513776.4 |
  1516668.0 |########################################
  1519559.6 |
  1522451.2 |
  1525342.8 |
  1528234.4 |
  1531126.0 |
  1534017.6 |
  1536909.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=4785.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=4579.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=304.1% of algo (FFI overhead may distort results)
