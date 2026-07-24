# abi_marshal (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_leaf_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_leaf_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_leaf_marshal_null dominates: 6804% faster than the next best (abi_marshal_leaf_aos)

abi_marshal_leaf_marshal_null (21.15 us) leads abi_marshal_leaf_aos (1.46 ms) by 6804%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_leaf_marshal_null beats baseline by 99% (significant)

abi_marshal_leaf_marshal_null is -1.44 ms (99%) faster than baseline abi_marshal_leaf_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_leaf_soa_transposed is an outlier: 70.4x slower than the field

abi_marshal_leaf_soa_transposed (1.49 ms) is 70.4x the fastest (21.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_leaf_marshal_null} vs {abi_marshal_leaf_aos, abi_marshal_leaf_soa_native, abi_marshal_leaf_soa_transposed} (6804% apart)

The field splits into a fast tier {abi_marshal_leaf_marshal_null} and a slow tier {abi_marshal_leaf_aos, abi_marshal_leaf_soa_native, abi_marshal_leaf_soa_transposed} with a 6804% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 70.4x the fastest

Fastest abi_marshal_leaf_marshal_null (21.15 us) to slowest abi_marshal_leaf_soa_transposed (1.49 ms): 70.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_leaf_marshal_null** at 21154.6 ns median (-98.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 70.40x (fastest 21154.6 ns, slowest 1489354.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1465227ns | 1462843ns | 1460124ns | 1462674ns | 1471607ns | base |
| abi_marshal_leaf_marshal_null | 23284ns | 23497ns | 22311ns | 23371ns | 23639ns | -98.41% |
| abi_marshal_leaf_soa_native | 1468148ns | 1467741ns | 1462548ns | 1467118ns | 1472491ns | +0.20% |
| abi_marshal_leaf_soa_transposed | 1505129ns | 1491828ns | 1488563ns | 1490781ns | 1534935ns | +2.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1462725ns | 1457471ns | 1469010ns | base | 0.000 |
| abi_marshal_leaf_marshal_null | 20965ns | 20081ns | 21280ns | -98.57% | 0.000 |
| abi_marshal_leaf_soa_native | 1465596ns | 1460097ns | 1469835ns | +0.20% | 0.000 |
| abi_marshal_leaf_soa_transposed | 1502564ns | 1486170ns | 1532142ns | +2.72% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 37328.5 | 1463278.5 | 1462724.9 | n/a |
| abi_marshal_leaf_marshal_null | 28563.8 | 21174.7 | 20965.2 | n/a |
| abi_marshal_leaf_soa_native | 37831.6 | 1464940.8 | 1465595.8 | n/a |
| abi_marshal_leaf_soa_transposed | 42107.5 | 1500654.5 | 1502564.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_leaf_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_leaf_aos | 0.000 | 1.4% |
| abi_marshal_leaf_marshal_null | 0.000 | 94.9% |
| abi_marshal_leaf_soa_native | 0.000 | 1.4% |
| abi_marshal_leaf_soa_transposed | 0.000 | 1.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_leaf_aos | 1465227ns | 1465227ns | base |
| abi_marshal_leaf_marshal_null | 23284ns | 23284ns | -98.41% |
| abi_marshal_leaf_soa_native | 1468148ns | 1468148ns | +0.20% |
| abi_marshal_leaf_soa_transposed | 1505129ns | 1505129ns | +2.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1460480ns | base | --- | [1458685, 1469010] | --- | --- | --- | --- |
| abi_marshal_leaf_marshal_null | 21155ns | -1439468.4ns (-98.6%) | [-1447730, -1438081]ns | [20461, 21280] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_leaf_soa_native | 1465231ns | no significant difference | [-5568, +11106]ns | [1461721, 1469835] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_leaf_soa_transposed | 1489354ns | +28436.8ns (+1.9%) | [+19664, +71418]ns | [1486197, 1532142] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_leaf_aos | abi_marshal_leaf_marshal_null | abi_marshal_leaf_soa_native | abi_marshal_leaf_soa_transposed |
|---|---|---|---|---|
| 1 | 1457471ns | -98.6% | +1.0% | +2.0% |
| 2 | 1459900ns | -98.6% | +0.3% | +7.8% |
| 3 | 1459988ns | -98.5% | +0.5% | +1.9% |
| 4 | 1464412ns | -98.5% | +0.2% | +1.5% |
| 5 | 1473608ns | -98.6% | -0.7% | +1.2% |
| 6 | 1460971ns | -98.6% | -0.1% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_leaf_aos | 0.103 | ok |
| abi_marshal_leaf_marshal_null | 0.042 | ok |
| abi_marshal_leaf_soa_native | -0.035 | ok |
| abi_marshal_leaf_soa_transposed | -0.269 | moderate- |

**Consistency summary:**

- **abi_marshal_leaf_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_leaf_soa_native**: won 1/6, lost 4/6
- **abi_marshal_leaf_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 4429489.9ns | 1462724.9ns | 302.8% | HIGH |
| abi_marshal_leaf_marshal_null | 171623.1ns | 20965.2ns | 818.6% | HIGH |
| abi_marshal_leaf_soa_native | 4432463.5ns | 1465595.8ns | 302.4% | HIGH |
| abi_marshal_leaf_soa_transposed | 4615795.5ns | 1502564.5ns | 307.2% | HIGH |

## Distribution (algo ns)

```
abi_marshal_leaf_aos (n=6, range 1457471.2-1469009.6 ns)
  1457471.2 |####################
  1458048.1 |
  1458625.0 |
  1459202.0 |
  1459778.9 |########################################
  1460355.8 |
  1460932.7 |####################
  1461509.6 |
  1462086.6 |
  1462663.5 |
  1463240.4 |
  1463817.3 |
  1464394.2 |####################
  1464971.2 |
  1465548.1 |
  1466125.0 |
  1466701.9 |
  1467278.8 |
  1467855.8 |
  1468432.7 |
  (0 below, 1 above range)

abi_marshal_leaf_marshal_null (n=6, range 20080.8-21280.0 ns)
  20080.8 |########################################
  20140.8 |
  20200.7 |
  20260.7 |
  20320.6 |
  20380.6 |
  20440.6 |
  20500.5 |
  20560.5 |
  20620.4 |
  20680.4 |
  20740.4 |
  20800.3 |########################################
  20860.3 |
  20920.2 |
  20980.2 |
  21040.2 |########################################
  21100.1 |
  21160.1 |########################################
  21220.0 |########################################
  (0 below, 1 above range)

abi_marshal_leaf_soa_native (n=6, range 1460096.7-1469835.4 ns)
  1460096.7 |########################################
  1460583.6 |
  1461070.6 |
  1461557.5 |
  1462044.4 |
  1462531.4 |
  1463018.3 |########################################
  1463505.2 |########################################
  1463992.2 |
  1464479.1 |
  1464966.0 |
  1465453.0 |
  1465939.9 |
  1466426.9 |########################################
  1466913.8 |
  1467400.7 |########################################
  1467887.7 |
  1468374.6 |
  1468861.5 |
  1469348.5 |
  (0 below, 1 above range)

abi_marshal_leaf_soa_transposed (n=6, range 1486170.0-1532142.3 ns)
  1486170.0 |########################################
  1488468.6 |#############
  1490767.2 |#############
  1493065.8 |
  1495364.5 |
  1497663.1 |
  1499961.7 |
  1502260.3 |
  1504558.9 |
  1506857.5 |
  1509156.1 |
  1511454.8 |
  1513753.4 |
  1516052.0 |
  1518350.6 |
  1520649.2 |
  1522947.8 |
  1525246.5 |
  1527545.1 |
  1529843.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_leaf_aos**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_marshal_null**: bridge=811.2% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_native**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_transposed**: bridge=302.9% of algo (FFI overhead may distort results)
