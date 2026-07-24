# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 56874% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (2.57 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.46 ms) by 56874%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.50 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 584.6x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.50 ms) is 584.6x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (56874% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 56874% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 584.6x the fastest

Fastest abi_cross_scalar_leaf_null_entry (2.57 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.50 ms): 584.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 2570.6 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 584.65x (fastest 2570.6 ns, slowest 1502892.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1538731ns | 1473148ns | 1460462ns | 1469014ns | 1682441ns | +1.79% |
| abi_cross_scalar_leaf_inproc_direct | 1511690ns | 1505917ns | 1490071ns | 1503741ns | 1534422ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1468552ns | 1467546ns | 1458172ns | 1465902ns | 1477718ns | -2.85% |
| abi_cross_scalar_leaf_null_entry | 4870ns | 4874ns | 4642ns | 4804ns | 5083ns | -99.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1535659ns | 1457589ns | 1679022ns | +1.80% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1508577ns | 1487121ns | 1531003ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1465677ns | 1455497ns | 1474872ns | -2.84% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 2570ns | 2472ns | 2663ns | -99.83% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 83401.2 | 1485986.4 | 1535658.9 | n/a |
| abi_cross_scalar_leaf_inproc_direct | 8240.1 | 1510505.1 | 1508577.3 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 7899.7 | 1466676.9 | 1465677.3 | n/a |
| abi_cross_scalar_leaf_null_entry | 28278.2 | 2652.2 | 2569.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.006 | 96.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1538731ns | 1538731ns | +1.79% |
| abi_cross_scalar_leaf_inproc_direct | 1511690ns | 1511690ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1468552ns | 1468552ns | -2.85% |
| abi_cross_scalar_leaf_null_entry | 4870ns | 4870ns | -99.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1502892ns | base | --- | [1491837, 1531003] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1470269ns | no significant difference | [-69607, +182073]ns | [1457685, 1679022] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1464575ns | -33849.3ns (-2.3%) | [-73418, -21433]ns | [1457585, 1474872] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_leaf_null_entry | 2571ns | -1500388.8ns (-99.8%) | [-1528417, -1489218]ns | [2475, 2663] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1487121ns | +25.8% | -0.8% | -99.8% |
| 2 | 1505489ns | -3.2% | -2.1% | -99.8% |
| 3 | 1555228ns | -5.9% | -6.4% | -99.8% |
| 4 | 1506778ns | -1.3% | -3.1% | -99.8% |
| 5 | 1500296ns | -1.6% | -2.2% | -99.8% |
| 6 | 1496552ns | -2.6% | -2.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | -0.071 | ok |
| abi_cross_scalar_leaf_inproc_direct | -0.017 | ok |
| abi_cross_scalar_leaf_inproc_fnptr | 0.159 | ok |
| abi_cross_scalar_leaf_null_entry | -0.376 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 5/6, lost 1/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4564625.1ns | 1535658.9ns | 297.2% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4537665.1ns | 1508577.3ns | 300.8% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4408309.4ns | 1465677.3ns | 300.8% | HIGH |
| abi_cross_scalar_leaf_null_entry | 119323.3ns | 2569.5ns | 4643.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1457589.2-1679022.5 ns)
  1457589.2 |########################################
  1468660.9 |#############
  1479732.5 |#############
  1490804.2 |
  1501875.9 |
  1512947.5 |
  1524019.2 |
  1535090.9 |
  1546162.5 |
  1557234.2 |
  1568305.9 |
  1579377.5 |
  1590449.2 |
  1601520.8 |
  1612592.5 |
  1623664.2 |
  1634735.8 |
  1645807.5 |
  1656879.2 |
  1667950.8 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1487121.2-1531002.7 ns)
  1487121.2 |####################
  1489315.3 |
  1491509.3 |
  1493703.4 |
  1495897.5 |####################
  1498091.6 |
  1500285.6 |####################
  1502479.7 |
  1504673.8 |########################################
  1506867.9 |
  1509061.9 |
  1511256.0 |
  1513450.1 |
  1515644.2 |
  1517838.2 |
  1520032.3 |
  1522226.4 |
  1524420.5 |
  1526614.6 |
  1528808.6 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1455497.1-1474871.9 ns)
  1455497.1 |########################################
  1456465.8 |
  1457434.6 |
  1458403.3 |
  1459372.1 |########################################
  1460340.8 |
  1461309.5 |
  1462278.3 |########################################
  1463247.0 |
  1464215.7 |
  1465184.5 |
  1466153.2 |########################################
  1467122.0 |
  1468090.7 |
  1469059.4 |
  1470028.2 |
  1470996.9 |
  1471965.6 |
  1472934.4 |
  1473903.1 |########################################
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 2472.5-2663.3 ns)
   2472.5 |########################################
   2482.0 |
   2491.6 |
   2501.1 |
   2510.7 |
   2520.2 |
   2529.7 |####################
   2539.3 |
   2548.8 |
   2558.4 |
   2567.9 |
   2577.4 |
   2587.0 |
   2596.5 |
   2606.1 |####################
   2615.6 |
   2625.1 |####################
   2634.7 |
   2644.2 |
   2653.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=300.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=4646.5% of algo (FFI overhead may distort results)
