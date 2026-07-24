# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 3.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 36946% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (3.98 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.47 ms) by 36946%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.50 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 377.9x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.50 ms) is 377.9x the fastest (3.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_leaf_inproc_direct shows alternating (throttle bounce) (autocorr -0.54)

abi_cross_scalar_leaf_inproc_direct's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (36946% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 36946% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 377.9x the fastest

Fastest abi_cross_scalar_leaf_null_entry (3.98 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.50 ms): 377.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 3975.8 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 377.90x (fastest 3975.8 ns, slowest 1502455.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1548187ns | 1484472ns | 1464529ns | 1480177ns | 1692030ns | +2.88% |
| abi_cross_scalar_leaf_inproc_direct | 1504814ns | 1505474ns | 1490480ns | 1504705ns | 1512146ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1480654ns | 1475676ns | 1462655ns | 1472840ns | 1501374ns | -1.61% |
| abi_cross_scalar_leaf_null_entry | 6222ns | 6197ns | 6156ns | 6184ns | 6311ns | -99.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1544847ns | 1461516ns | 1688215ns | +2.87% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1501811ns | 1487392ns | 1509248ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1477612ns | 1459710ns | 1498022ns | -1.61% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 3970ns | 3901ns | 4029ns | -99.74% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 64528.7 | 1657684.7 | 1544847.5 | n/a |
| abi_cross_scalar_leaf_inproc_direct | 8248.9 | 1500257.8 | 1501811.1 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 8246.1 | 1478083.9 | 1477612.1 | n/a |
| abi_cross_scalar_leaf_null_entry | 27512.6 | 4120.2 | 3969.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.3% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.3% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.3% |
| abi_cross_scalar_leaf_null_entry | 0.001 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1548187ns | 1548187ns | +2.88% |
| abi_cross_scalar_leaf_inproc_direct | 1504814ns | 1504814ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1480654ns | 1480654ns | -1.61% |
| abi_cross_scalar_leaf_null_entry | 6222ns | 6222ns | -99.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1502455ns | base | --- | [1493730, 1509248] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1481231ns | no significant difference | [-35815, +188795]ns | [1465096, 1688215] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1472888ns | no significant difference | [-47322, +4292]ns | [1461925, 1498022] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_leaf_null_entry | 3976ns | -1498551.1ns (-99.7%) | [-1505271, -1489702]ns | [3904, 4029] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1504668ns | +25.7% | -3.0% | -99.7% |
| 2 | 1500068ns | -2.6% | +0.3% | -99.7% |
| 3 | 1501754ns | -2.2% | -1.6% | -99.7% |
| 4 | 1503157ns | -1.2% | -2.3% | -99.7% |
| 5 | 1487392ns | -0.6% | +0.3% | -99.7% |
| 6 | 1513827ns | -2.0% | -3.3% | -99.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | -0.069 | ok |
| abi_cross_scalar_leaf_inproc_direct | -0.541 | HIGH- (thermal bounce) |
| abi_cross_scalar_leaf_inproc_fnptr | -0.532 | HIGH- (thermal bounce) |
| abi_cross_scalar_leaf_null_entry | -0.216 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 5/6, lost 1/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 4/6, lost 2/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4820610.9ns | 1544847.5ns | 312.0% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4514417.1ns | 1501811.1ns | 300.6% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4442243.3ns | 1477612.1ns | 300.6% | HIGH |
| abi_cross_scalar_leaf_null_entry | 122149.1ns | 3969.7ns | 3077.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1461515.8-1688215.2 ns)
  1461515.8 |########################################
  1472850.8 |########################################
  1484185.7 |####################
  1495520.7 |
  1506855.7 |
  1518190.7 |
  1529525.6 |
  1540860.6 |
  1552195.6 |
  1563530.5 |
  1574865.5 |
  1586200.5 |
  1597535.4 |
  1608870.4 |
  1620205.4 |
  1631540.4 |
  1642875.3 |
  1654210.3 |
  1665545.3 |
  1676880.2 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1487392.5-1509247.7 ns)
  1487392.5 |########################################
  1488485.3 |
  1489578.0 |
  1490670.8 |
  1491763.5 |
  1492856.3 |
  1493949.1 |
  1495041.8 |
  1496134.6 |
  1497227.3 |
  1498320.1 |
  1499412.9 |########################################
  1500505.6 |
  1501598.4 |########################################
  1502691.1 |########################################
  1503783.9 |########################################
  1504876.7 |
  1505969.4 |
  1507062.2 |
  1508154.9 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1459709.6-1498022.5 ns)
  1459709.6 |########################################
  1461625.2 |
  1463540.9 |########################################
  1465456.5 |
  1467372.2 |########################################
  1469287.8 |
  1471203.5 |
  1473119.1 |
  1475034.8 |
  1476950.4 |########################################
  1478866.1 |
  1480781.7 |
  1482697.3 |
  1484613.0 |
  1486528.6 |
  1488444.3 |
  1490359.9 |########################################
  1492275.6 |
  1494191.2 |
  1496106.9 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 3901.2-4029.0 ns)
   3901.2 |########################################
   3907.6 |
   3914.0 |
   3920.4 |
   3926.8 |
   3933.1 |
   3939.5 |
   3945.9 |
   3952.3 |
   3958.7 |####################
   3965.1 |
   3971.5 |
   3977.9 |
   3984.3 |
   3990.7 |########################################
   3997.1 |
   4003.4 |
   4009.8 |
   4016.2 |
   4022.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=3067.5% of algo (FFI overhead may distort results)
