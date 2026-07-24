# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.3% of the fastest

All 4 variants sit between 1.45 ms and 1.46 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1454598.3 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 1454598.3 ns, slowest 1459155.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1455961ns | 1456978ns | 1451280ns | 1455898ns | 1458396ns | -4.71% |
| abi_sink_leaf_batched_sink_decode | 1462064ns | 1461530ns | 1458380ns | 1461351ns | 1464974ns | -4.31% |
| abi_sink_leaf_null_sink | 1527930ns | 1459610ns | 1447983ns | 1457363ns | 1673753ns | base |
| abi_sink_leaf_per_record_sink | 1524442ns | 1458130ns | 1453639ns | 1457116ns | 1660832ns | -0.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1453526ns | 1448855ns | 1455860ns | -4.71% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1459605ns | 1455930ns | 1462426ns | -4.31% | 0.000 |
| abi_sink_leaf_null_sink | 1525383ns | 1445615ns | 1671081ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1521694ns | 1451199ns | 1657489ns | -0.24% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 34400.5 | 1452756.9 | 1453525.6 | n/a |
| abi_sink_leaf_batched_sink_decode | 37054.0 | 1460036.1 | 1459604.5 | n/a |
| abi_sink_leaf_null_sink | 37739.2 | 1463827.5 | 1525383.4 | n/a |
| abi_sink_leaf_per_record_sink | 43259.3 | 1498327.1 | 1521693.5 | 1 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_null_sink; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.4% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.1% |
| abi_sink_leaf_null_sink | 0.000 | 99.2% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1455961ns | 1455961ns | -4.71% |
| abi_sink_leaf_batched_sink_decode | 1462064ns | 1462064ns | -4.31% |
| abi_sink_leaf_null_sink | 1527930ns | 1527930ns | base |
| abi_sink_leaf_per_record_sink | 1524442ns | 1524442ns | -0.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1457023ns | base | --- | [1448046, 1671081] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1454598ns | no significant difference | [-219223, +5158]ns | [1450119, 1455860] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_leaf_batched_sink_decode | 1459155ns | no significant difference | [-210910, +11441]ns | [1457232, 1462426] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_leaf_per_record_sink | 1455650ns | no significant difference | [-17546, +7310]ns | [1451942, 1657489] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1882912ns | -23.1% | -22.2% | -1.4% |
| 2 | 1459072ns | -0.3% | +0.0% | -0.5% |
| 3 | 1450477ns | +0.3% | +0.6% | +0.2% |
| 4 | 1445615ns | +0.4% | +1.0% | +0.7% |
| 5 | 1459250ns | -0.2% | -0.2% | -0.3% |
| 6 | 1454974ns | -0.0% | +0.3% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.257 | moderate- |
| abi_sink_leaf_batched_sink_decode | -0.022 | ok |
| abi_sink_leaf_null_sink | -0.018 | ok |
| abi_sink_leaf_per_record_sink | -0.042 | ok |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 3/6, lost 2/6
- **abi_sink_leaf_batched_sink_decode**: won 2/6, lost 3/6
- **abi_sink_leaf_per_record_sink**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4395564.3ns | 1453525.6ns | 302.4% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4421225.8ns | 1459604.5ns | 302.9% | HIGH |
| abi_sink_leaf_null_sink | 4474735.9ns | 1525383.4ns | 293.4% | HIGH |
| abi_sink_leaf_per_record_sink | 4581891.2ns | 1521693.5ns | 301.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1448855.0-1455859.8 ns)
  1448855.0 |####################
  1449205.2 |
  1449555.5 |
  1449905.7 |
  1450256.0 |
  1450606.2 |
  1450956.4 |
  1451306.7 |####################
  1451656.9 |
  1452007.2 |
  1452357.4 |
  1452707.6 |
  1453057.9 |
  1453408.1 |
  1453758.4 |
  1454108.6 |
  1454458.8 |########################################
  1454809.1 |####################
  1455159.3 |
  1455509.6 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1455929.6-1462426.5 ns)
  1455929.6 |####################
  1456254.4 |
  1456579.3 |
  1456904.1 |
  1457229.0 |
  1457553.8 |
  1457878.7 |
  1458203.5 |
  1458528.3 |########################################
  1458853.2 |
  1459178.0 |
  1459502.9 |####################
  1459827.7 |
  1460152.6 |####################
  1460477.4 |
  1460802.2 |
  1461127.1 |
  1461451.9 |
  1461776.8 |
  1462101.6 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1445615.0-1671081.1 ns)
  1445615.0 |########################################
  1456888.3 |##########################
  1468161.6 |
  1479434.9 |
  1490708.2 |
  1501981.5 |
  1513254.8 |
  1524528.1 |
  1535801.4 |
  1547074.7 |
  1558348.0 |
  1569621.3 |
  1580894.6 |
  1592167.9 |
  1603441.2 |
  1614714.5 |
  1625987.8 |
  1637261.1 |
  1648534.4 |
  1659807.7 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1451199.2-1657489.2 ns)
  1451199.2 |########################################
  1461513.7 |
  1471828.2 |
  1482142.7 |
  1492457.2 |
  1502771.7 |
  1513086.2 |
  1523400.7 |
  1533715.2 |
  1544029.7 |
  1554344.2 |
  1564658.7 |
  1574973.2 |
  1585287.7 |
  1595602.2 |
  1605916.7 |
  1616231.2 |
  1626545.7 |
  1636860.2 |
  1647174.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.4% of algo (FFI overhead may distort results)
