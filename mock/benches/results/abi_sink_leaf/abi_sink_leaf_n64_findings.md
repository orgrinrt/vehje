# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_leaf_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_leaf_null_sink has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_leaf_batched_sink at 1.45 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7.12 us) is smaller than the fastest variant's own run-to-run std-dev (40.15 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.5% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1446023.6 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1446023.6 ns, slowest 1453146.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1467172ns | 1448422ns | 1446905ns | 1448168ns | 1505810ns | -1.27% |
| abi_sink_leaf_batched_sink_decode | 1452719ns | 1450496ns | 1443422ns | 1449057ns | 1462860ns | -2.24% |
| abi_sink_leaf_null_sink | 1486071ns | 1455739ns | 1449966ns | 1453966ns | 1552280ns | base |
| abi_sink_leaf_per_record_sink | 1454827ns | 1453189ns | 1450056ns | 1452600ns | 1460554ns | -2.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1464603ns | 1444302ns | 1502993ns | -1.27% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1450264ns | 1441168ns | 1460326ns | -2.24% | 0.000 |
| abi_sink_leaf_null_sink | 1483452ns | 1447586ns | 1549453ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1452322ns | 1447584ns | 1457895ns | -2.10% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 38311.5 | 1459702.2 | 1464602.6 | n/a |
| abi_sink_leaf_batched_sink_decode | 35002.6 | 1451171.4 | 1450263.5 | n/a |
| abi_sink_leaf_null_sink | 39538.1 | 1469337.5 | 1483452.1 | n/a |
| abi_sink_leaf_per_record_sink | 37410.9 | 1452803.9 | 1452322.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink_decode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.7% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_leaf_null_sink | 0.000 | 99.2% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1467172ns | 1467172ns | -1.27% |
| abi_sink_leaf_batched_sink_decode | 1452719ns | 1452719ns | -2.24% |
| abi_sink_leaf_null_sink | 1486071ns | 1486071ns | base |
| abi_sink_leaf_per_record_sink | 1454827ns | 1454827ns | -2.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1453146ns | base | --- | [1447758, 1549453] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1446024ns | no significant difference | [-50191, +1441]ns | [1444792, 1502993] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_leaf_batched_sink_decode | 1448025ns | -6089.2ns (-0.4%) | [-92718, -759]ns | [1442439, 1460326] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_sink_leaf_per_record_sink | 1450763ns | no significant difference | [-91785, +551]ns | [1448309, 1457895] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1639704ns | -5.2% | -10.7% | -10.7% |
| 2 | 1459202ns | -1.0% | -0.6% | -0.6% |
| 3 | 1454618ns | -0.6% | +0.2% | -0.3% |
| 4 | 1451674ns | -0.4% | -0.4% | -0.0% |
| 5 | 1447929ns | -0.1% | -0.5% | -0.0% |
| 6 | 1447586ns | +0.3% | -0.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.050 | ok |
| abi_sink_leaf_batched_sink_decode | 0.163 | ok |
| abi_sink_leaf_null_sink | 0.008 | ok |
| abi_sink_leaf_per_record_sink | 0.032 | ok |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 4/6, lost 1/6
- **abi_sink_leaf_batched_sink_decode**: won 5/6, lost 1/6
- **abi_sink_leaf_per_record_sink**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4418024.4ns | 1464602.6ns | 301.7% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4394688.0ns | 1450263.5ns | 303.0% | HIGH |
| abi_sink_leaf_null_sink | 4489085.2ns | 1483452.1ns | 302.6% | HIGH |
| abi_sink_leaf_per_record_sink | 4394712.5ns | 1452322.3ns | 302.6% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1444302.5-1502992.7 ns)
  1444302.5 |########################################
  1447237.0 |
  1450171.5 |##########
  1453106.0 |
  1456040.5 |
  1458975.1 |
  1461909.6 |
  1464844.1 |
  1467778.6 |
  1470713.1 |
  1473647.6 |
  1476582.1 |
  1479516.6 |
  1482451.1 |
  1485385.6 |
  1488320.1 |
  1491254.7 |
  1494189.2 |
  1497123.7 |
  1500058.2 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1441167.9-1460326.4 ns)
  1441167.9 |########################################
  1442125.8 |
  1443083.8 |########################################
  1444041.7 |
  1444999.6 |
  1445957.5 |########################################
  1446915.5 |
  1447873.4 |
  1448831.3 |
  1449789.2 |########################################
  1450747.2 |
  1451705.1 |
  1452663.0 |
  1453621.0 |
  1454578.9 |
  1455536.8 |
  1456494.7 |########################################
  1457452.7 |
  1458410.6 |
  1459368.5 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1447585.8-1549452.9 ns)
  1447585.8 |########################################
  1452679.2 |#############
  1457772.5 |#############
  1462865.9 |
  1467959.2 |
  1473052.6 |
  1478145.9 |
  1483239.3 |
  1488332.6 |
  1493426.0 |
  1498519.4 |
  1503612.7 |
  1508706.1 |
  1513799.4 |
  1518892.8 |
  1523986.1 |
  1529079.5 |
  1534172.8 |
  1539266.2 |
  1544359.5 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1447583.8-1457895.4 ns)
  1447583.8 |####################
  1448099.4 |
  1448615.0 |####################
  1449130.5 |
  1449646.1 |
  1450161.7 |
  1450677.3 |########################################
  1451192.9 |####################
  1451708.4 |
  1452224.0 |
  1452739.6 |
  1453255.2 |
  1453770.8 |
  1454286.3 |
  1454801.9 |
  1455317.5 |
  1455833.1 |
  1456348.7 |
  1456864.2 |
  1457379.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.3% of algo (FFI overhead may distort results)
