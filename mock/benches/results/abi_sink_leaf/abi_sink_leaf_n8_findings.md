# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.5% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1445356.2 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 1445356.2 ns, slowest 1452681.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1446795ns | 1448030ns | 1440057ns | 1446640ns | 1450398ns | -0.76% |
| abi_sink_leaf_batched_sink_decode | 1453405ns | 1453861ns | 1451613ns | 1453244ns | 1454541ns | -0.30% |
| abi_sink_leaf_null_sink | 1457831ns | 1453446ns | 1450067ns | 1452980ns | 1468988ns | base |
| abi_sink_leaf_per_record_sink | 1466933ns | 1455189ns | 1452615ns | 1454539ns | 1492684ns | +0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1444274ns | 1437594ns | 1447923ns | -0.76% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1450919ns | 1449204ns | 1452081ns | -0.30% | 0.000 |
| abi_sink_leaf_null_sink | 1455332ns | 1447630ns | 1466356ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1464298ns | 1449978ns | 1489858ns | +0.62% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 34748.3 | 1444234.0 | 1444273.9 | n/a |
| abi_sink_leaf_batched_sink_decode | 35897.8 | 1450431.4 | 1450919.5 | n/a |
| abi_sink_leaf_null_sink | 36909.1 | 1455952.4 | 1455331.6 | n/a |
| abi_sink_leaf_per_record_sink | 39417.8 | 1465081.4 | 1464298.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.5% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.0% |
| abi_sink_leaf_null_sink | 0.000 | 99.1% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1446795ns | 1446795ns | -0.76% |
| abi_sink_leaf_batched_sink_decode | 1453405ns | 1453405ns | -0.30% |
| abi_sink_leaf_null_sink | 1457831ns | 1457831ns | base |
| abi_sink_leaf_per_record_sink | 1466933ns | 1466933ns | +0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1451033ns | base | --- | [1448606, 1466356] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1445356ns | -5950.5ns (-0.4%) | [-24517, -2706]ns | [1439543, 1447923] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_leaf_batched_sink_decode | 1451384ns | no significant difference | [-14561, +1774]ns | [1449293, 1452081] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_leaf_per_record_sink | 1452682ns | no significant difference | [-994, +25799]ns | [1450355, 1489858] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1478169ns | -2.7% | -1.7% | +3.1% |
| 2 | 1454542ns | -0.5% | -0.2% | -0.0% |
| 3 | 1452116ns | -0.2% | -0.0% | -0.1% |
| 4 | 1449582ns | -0.3% | -0.0% | +0.0% |
| 5 | 1449950ns | -0.6% | +0.1% | +0.4% |
| 6 | 1447630ns | -0.2% | +0.1% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.050 | ok |
| abi_sink_leaf_batched_sink_decode | -0.376 | moderate- |
| abi_sink_leaf_null_sink | 0.115 | ok |
| abi_sink_leaf_per_record_sink | -0.012 | ok |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 6/6, lost 0/6
- **abi_sink_leaf_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_leaf_per_record_sink**: won 0/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4369144.8ns | 1444273.9ns | 302.5% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4390394.0ns | 1450919.5ns | 302.6% | HIGH |
| abi_sink_leaf_null_sink | 4406623.4ns | 1455331.6ns | 302.8% | HIGH |
| abi_sink_leaf_per_record_sink | 4480341.3ns | 1464298.3ns | 306.0% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1437593.8-1447922.7 ns)
  1437593.8 |########################################
  1438110.2 |
  1438626.7 |
  1439143.1 |
  1439659.6 |
  1440176.0 |
  1440692.5 |
  1441208.9 |########################################
  1441725.4 |
  1442241.8 |
  1442758.2 |
  1443274.7 |
  1443791.1 |
  1444307.6 |
  1444824.0 |########################################
  1445340.5 |########################################
  1445856.9 |
  1446373.4 |########################################
  1446889.8 |
  1447406.3 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1449204.2-1452080.8 ns)
  1449204.2 |########################################
  1449348.0 |########################################
  1449491.9 |
  1449635.7 |
  1449779.5 |
  1449923.4 |
  1450067.2 |
  1450211.0 |
  1450354.8 |
  1450498.7 |
  1450642.5 |
  1450786.3 |
  1450930.2 |
  1451074.0 |########################################
  1451217.8 |
  1451361.6 |
  1451505.5 |########################################
  1451649.3 |########################################
  1451793.1 |
  1451937.0 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1447630.4-1466355.9 ns)
  1447630.4 |####################
  1448566.7 |
  1449502.9 |########################################
  1450439.2 |
  1451375.5 |####################
  1452311.8 |
  1453248.0 |
  1454184.3 |####################
  1455120.6 |
  1456056.9 |
  1456993.1 |
  1457929.4 |
  1458865.7 |
  1459801.9 |
  1460738.2 |
  1461674.5 |
  1462610.8 |
  1463547.0 |
  1464483.3 |
  1465419.6 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1449977.5-1489858.1 ns)
  1449977.5 |########################################
  1451971.5 |#############
  1453965.6 |#############
  1455959.6 |
  1457953.6 |
  1459947.6 |
  1461941.7 |
  1463935.7 |
  1465929.7 |
  1467923.8 |
  1469917.8 |
  1471911.8 |
  1473905.9 |
  1475899.9 |
  1477893.9 |
  1479888.0 |
  1481882.0 |
  1483876.0 |
  1485870.0 |
  1487864.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.6% of algo (FFI overhead may distort results)
