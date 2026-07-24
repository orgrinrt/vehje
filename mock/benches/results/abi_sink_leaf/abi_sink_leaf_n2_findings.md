# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4.51 us) is smaller than the fastest variant's own run-to-run std-dev (39.60 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1450184.1 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 1450184.1 ns, slowest 1454689.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1469555ns | 1452619ns | 1447476ns | 1451859ns | 1507139ns | +0.83% |
| abi_sink_leaf_batched_sink_decode | 1465887ns | 1457171ns | 1445448ns | 1456960ns | 1489497ns | +0.58% |
| abi_sink_leaf_null_sink | 1457465ns | 1453636ns | 1449990ns | 1453370ns | 1467344ns | base |
| abi_sink_leaf_per_record_sink | 1489134ns | 1455323ns | 1454428ns | 1455043ns | 1557624ns | +2.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1467083ns | 1444960ns | 1504587ns | +0.83% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1463346ns | 1443140ns | 1486713ns | +0.57% | 0.000 |
| abi_sink_leaf_null_sink | 1454991ns | 1447465ns | 1464835ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1486502ns | 1451974ns | 1554681ns | +2.17% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 36825.6 | 1456195.9 | 1467083.2 | n/a |
| abi_sink_leaf_batched_sink_decode | 39481.1 | 1461942.0 | 1463346.0 | n/a |
| abi_sink_leaf_null_sink | 36118.0 | 1457597.2 | 1454990.6 | n/a |
| abi_sink_leaf_per_record_sink | 41167.6 | 1491395.3 | 1486501.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink_decode; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.5% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_leaf_null_sink | 0.000 | 99.4% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1469555ns | 1469555ns | +0.83% |
| abi_sink_leaf_batched_sink_decode | 1465887ns | 1465887ns | +0.58% |
| abi_sink_leaf_null_sink | 1457465ns | 1457465ns | base |
| abi_sink_leaf_per_record_sink | 1489134ns | 1489134ns | +2.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1451186ns | base | --- | [1448951, 1464835] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1450184ns | no significant difference | [-6714, +44105]ns | [1446478, 1504587] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_leaf_batched_sink_decode | 1454689ns | no significant difference | [-7099, +27436]ns | [1448635, 1486713] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_leaf_per_record_sink | 1452809ns | no significant difference | [-3862, +95405]ns | [1452014, 1554681] | no | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1450582ns | -0.4% | -0.5% | +0.1% |
| 2 | 1451790ns | -0.2% | +0.2% | +0.1% |
| 3 | 1447465ns | +0.0% | +0.5% | +0.3% |
| 4 | 1468117ns | +5.9% | +3.0% | +12.6% |
| 5 | 1461554ns | -0.5% | -0.5% | -0.6% |
| 6 | 1450437ns | +0.1% | +0.7% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.203 | moderate- |
| abi_sink_leaf_batched_sink_decode | -0.185 | ok |
| abi_sink_leaf_null_sink | -0.013 | ok |
| abi_sink_leaf_per_record_sink | -0.240 | moderate- |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 3/6, lost 1/6
- **abi_sink_leaf_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_leaf_per_record_sink**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4405249.3ns | 1467083.2ns | 300.3% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4484662.9ns | 1463346.0ns | 306.5% | HIGH |
| abi_sink_leaf_null_sink | 4542751.5ns | 1454990.6ns | 312.2% | HIGH |
| abi_sink_leaf_per_record_sink | 4501052.6ns | 1486501.7ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1444960.4-1504587.3 ns)
  1444960.4 |####################
  1447941.7 |########################################
  1450923.1 |########################################
  1453904.4 |
  1456885.8 |
  1459867.1 |
  1462848.5 |
  1465829.8 |
  1468811.2 |
  1471792.5 |
  1474773.8 |
  1477755.2 |
  1480736.5 |
  1483717.9 |
  1486699.2 |
  1489680.6 |
  1492661.9 |
  1495643.3 |
  1498624.6 |
  1501606.0 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1443140.0-1486713.1 ns)
  1443140.0 |#############
  1445318.7 |
  1447497.3 |
  1449676.0 |
  1451854.6 |
  1454033.3 |########################################
  1456211.9 |
  1458390.6 |
  1460569.3 |#############
  1462747.9 |
  1464926.6 |
  1467105.2 |
  1469283.9 |
  1471462.5 |
  1473641.2 |
  1475819.9 |
  1477998.5 |
  1480177.2 |
  1482355.8 |
  1484534.5 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1447465.0-1464835.2 ns)
  1447465.0 |####################
  1448333.5 |
  1449202.0 |
  1450070.5 |########################################
  1450939.1 |####################
  1451807.6 |
  1452676.1 |
  1453544.6 |
  1454413.1 |
  1455281.6 |
  1456150.1 |
  1457018.6 |
  1457887.1 |
  1458755.7 |
  1459624.2 |
  1460492.7 |
  1461361.2 |####################
  1462229.7 |
  1463098.2 |
  1463966.7 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1451973.8-1554681.5 ns)
  1451973.8 |########################################
  1457109.2 |
  1462244.6 |
  1467379.9 |
  1472515.3 |
  1477650.7 |
  1482786.1 |
  1487921.5 |
  1493056.9 |
  1498192.2 |
  1503327.6 |
  1508463.0 |
  1513598.4 |
  1518733.8 |
  1523869.2 |
  1529004.5 |
  1534139.9 |
  1539275.3 |
  1544410.7 |
  1549546.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.7% of algo (FFI overhead may distort results)
