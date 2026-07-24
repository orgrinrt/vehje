# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_sink_leaf_null_sink shows alternating (throttle bounce) (autocorr -0.80)

abi_sink_leaf_null_sink's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.3% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1445015.6 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 1445015.6 ns, slowest 1449698.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1449150ns | 1447584ns | 1446337ns | 1447184ns | 1453504ns | -0.09% |
| abi_sink_leaf_batched_sink_decode | 1450273ns | 1449754ns | 1447486ns | 1449278ns | 1453161ns | -0.02% |
| abi_sink_leaf_null_sink | 1450527ns | 1450387ns | 1449114ns | 1450030ns | 1451980ns | base |
| abi_sink_leaf_per_record_sink | 1453213ns | 1452156ns | 1446453ns | 1451689ns | 1458880ns | +0.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1446627ns | 1443906ns | 1450916ns | -0.10% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1447815ns | 1445141ns | 1450605ns | -0.02% | 0.000 |
| abi_sink_leaf_null_sink | 1448042ns | 1446631ns | 1449470ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1450732ns | 1444086ns | 1456234ns | +0.19% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 35749.9 | 1444831.9 | 1446626.7 | 0 |
| abi_sink_leaf_batched_sink_decode | 34735.8 | 1447834.6 | 1447815.2 | n/a |
| abi_sink_leaf_null_sink | 35068.4 | 1448898.7 | 1448041.8 | n/a |
| abi_sink_leaf_per_record_sink | 35241.6 | 1449670.6 | 1450731.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.9% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.8% |
| abi_sink_leaf_null_sink | 0.000 | 99.7% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1449150ns | 1449150ns | -0.09% |
| abi_sink_leaf_batched_sink_decode | 1450273ns | 1450273ns | -0.02% |
| abi_sink_leaf_null_sink | 1450527ns | 1450527ns | base |
| abi_sink_leaf_per_record_sink | 1453213ns | 1453213ns | +0.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1447978ns | base | --- | [1446678, 1449470] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1445016ns | no significant difference | [-5058, +3928]ns | [1443948, 1450916] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_leaf_batched_sink_decode | 1447328ns | no significant difference | [-3078, +2492]ns | [1445513, 1450605] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_leaf_per_record_sink | 1449699ns | no significant difference | [-1500, +6814]ns | [1446263, 1456234] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1446631ns | -0.2% | -0.1% | +0.1% |
| 2 | 1450137ns | -0.4% | -0.3% | +0.4% |
| 3 | 1446724ns | +0.2% | +0.1% | -0.2% |
| 4 | 1448802ns | -0.2% | +0.2% | -0.0% |
| 5 | 1447252ns | +0.3% | -0.1% | +0.3% |
| 6 | 1448704ns | -0.3% | +0.0% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.475 | moderate- |
| abi_sink_leaf_batched_sink_decode | -0.291 | moderate- |
| abi_sink_leaf_null_sink | -0.804 | HIGH- (thermal bounce) |
| abi_sink_leaf_per_record_sink | -0.318 | moderate- |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 4/6, lost 2/6
- **abi_sink_leaf_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_leaf_per_record_sink**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4375486.9ns | 1446626.7ns | 302.5% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4380982.2ns | 1447815.2ns | 302.6% | HIGH |
| abi_sink_leaf_null_sink | 4383621.6ns | 1448041.8ns | 302.7% | HIGH |
| abi_sink_leaf_per_record_sink | 4386583.9ns | 1450731.9ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1443906.2-1450916.2 ns)
  1443906.2 |########################################
  1444256.7 |
  1444607.2 |####################
  1444957.7 |####################
  1445308.2 |
  1445658.7 |
  1446009.2 |
  1446359.7 |
  1446710.2 |
  1447060.7 |
  1447411.2 |
  1447761.7 |
  1448112.2 |
  1448462.7 |
  1448813.2 |
  1449163.7 |
  1449514.2 |
  1449864.7 |####################
  1450215.2 |
  1450565.7 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1445141.2-1450605.0 ns)
  1445141.2 |########################################
  1445414.4 |
  1445687.6 |########################################
  1445960.8 |########################################
  1446234.0 |
  1446507.1 |
  1446780.3 |
  1447053.5 |
  1447326.7 |
  1447599.9 |
  1447873.1 |
  1448146.3 |
  1448419.5 |########################################
  1448692.7 |
  1448965.9 |
  1449239.1 |########################################
  1449512.2 |
  1449785.4 |
  1450058.6 |
  1450331.8 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1446631.2-1449469.6 ns)
  1446631.2 |########################################
  1446773.1 |
  1446915.0 |
  1447057.0 |
  1447198.9 |####################
  1447340.8 |
  1447482.7 |
  1447624.6 |
  1447766.6 |
  1447908.5 |
  1448050.4 |
  1448192.3 |
  1448334.2 |
  1448476.2 |
  1448618.1 |####################
  1448760.0 |####################
  1448901.9 |
  1449043.8 |
  1449185.8 |
  1449327.7 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1444086.2-1456234.1 ns)
  1444086.2 |####################
  1444693.6 |
  1445301.0 |
  1445908.4 |
  1446515.8 |
  1447123.2 |
  1447730.6 |
  1448338.0 |########################################
  1448945.4 |
  1449552.8 |
  1450160.2 |
  1450767.6 |####################
  1451375.0 |
  1451982.4 |
  1452589.8 |
  1453197.2 |
  1453804.6 |
  1454412.0 |
  1455019.4 |
  1455626.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.3% of algo (FFI overhead may distort results)
