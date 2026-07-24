# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_leaf_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_leaf_null_sink has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_leaf_batched_sink_decode at 1.45 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_sink_leaf_batched_sink_decode is fastest but the noisiest (CV 11.0%)

abi_sink_leaf_batched_sink_decode wins on median (1.45 ms) yet has the highest variance (CV 11.0%), while abi_sink_leaf_batched_sink is the steadiest (CV 0.6%, 1.45 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.60 us) is smaller than the fastest variant's own run-to-run std-dev (159.58 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_leaf_batched_sink_decode vs stability leader abi_sink_leaf_batched_sink (+0% speed for 18.4x steadier)

abi_sink_leaf_batched_sink_decode is fastest (1.45 ms, CV 11.0%); abi_sink_leaf_batched_sink gives up 0.0% median for 18.4x lower variance (CV 0.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.4% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink_decode** at 1446042.2 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1446042.2 ns, slowest 1451639.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1453114ns | 1448702ns | 1445768ns | 1448539ns | 1463648ns | -1.85% |
| abi_sink_leaf_batched_sink_decode | 1520011ns | 1448496ns | 1447146ns | 1448264ns | 1664063ns | +2.67% |
| abi_sink_leaf_null_sink | 1480471ns | 1454112ns | 1445084ns | 1452970ns | 1539416ns | base |
| abi_sink_leaf_per_record_sink | 1466437ns | 1450754ns | 1449409ns | 1450714ns | 1498536ns | -0.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1450616ns | 1443484ns | 1461055ns | -1.85% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1517490ns | 1444781ns | 1661380ns | +2.67% | 0.000 |
| abi_sink_leaf_null_sink | 1477972ns | 1442770ns | 1536819ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1463829ns | 1446954ns | 1495715ns | -0.96% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 35174.2 | 1448429.0 | 1450616.1 | n/a |
| abi_sink_leaf_batched_sink_decode | 37417.5 | 1457554.9 | 1517489.9 | n/a |
| abi_sink_leaf_null_sink | 36952.2 | 1460675.7 | 1477971.9 | n/a |
| abi_sink_leaf_per_record_sink | 39363.4 | 1464699.5 | 1463829.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_null_sink; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.8% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.8% |
| abi_sink_leaf_null_sink | 0.000 | 99.4% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1453114ns | 1453114ns | -1.85% |
| abi_sink_leaf_batched_sink_decode | 1520011ns | 1520011ns | +2.67% |
| abi_sink_leaf_null_sink | 1480471ns | 1480471ns | base |
| abi_sink_leaf_per_record_sink | 1466437ns | 1466437ns | -0.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1451639ns | base | --- | [1445457, 1536819] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1446164ns | -4313.3ns (-0.3%) | [-77590, -165]ns | [1444629, 1461055] | YES (adj: no) | 0.6563 | 0.2188 | 0 |
| abi_sink_leaf_batched_sink_decode | 1446042ns | no significant difference | [-10458, +130223]ns | [1445048, 1661380] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_leaf_per_record_sink | 1448264ns | no significant difference | [-44117, +5819]ns | [1447508, 1495715] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1449435ns | -0.3% | -0.1% | -0.1% |
| 2 | 1457232ns | -0.3% | -0.8% | -0.6% |
| 3 | 1442770ns | +0.2% | +0.2% | +0.4% |
| 4 | 1616407ns | -9.1% | +16.0% | -4.9% |
| 5 | 1453844ns | -0.5% | -0.6% | -0.5% |
| 6 | 1448144ns | -0.3% | -0.1% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.344 | moderate- |
| abi_sink_leaf_batched_sink_decode | -0.238 | moderate- |
| abi_sink_leaf_null_sink | -0.267 | moderate- |
| abi_sink_leaf_per_record_sink | -0.265 | moderate- |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 5/6, lost 1/6
- **abi_sink_leaf_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_leaf_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4380296.5ns | 1450616.1ns | 302.0% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4418278.1ns | 1517489.9ns | 291.2% | HIGH |
| abi_sink_leaf_null_sink | 4484317.7ns | 1477971.9ns | 303.4% | HIGH |
| abi_sink_leaf_per_record_sink | 4435222.5ns | 1463829.0ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1443483.8-1461055.0 ns)
  1443483.8 |####################
  1444362.4 |
  1445240.9 |########################################
  1446119.5 |####################
  1446998.0 |
  1447876.6 |
  1448755.2 |
  1449633.7 |
  1450512.3 |
  1451390.8 |
  1452269.4 |
  1453148.0 |####################
  1454026.5 |
  1454905.1 |
  1455783.6 |
  1456662.2 |
  1457540.8 |
  1458419.3 |
  1459297.9 |
  1460176.4 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1444781.2-1661379.6 ns)
  1444781.2 |########################################
  1455611.1 |
  1466441.0 |
  1477271.0 |
  1488100.9 |
  1498930.8 |
  1509760.7 |
  1520590.6 |
  1531420.6 |
  1542250.5 |
  1553080.4 |
  1563910.3 |
  1574740.2 |
  1585570.2 |
  1596400.1 |
  1607230.0 |
  1618059.9 |
  1628889.8 |
  1639719.8 |
  1650549.7 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1442770.4-1536819.4 ns)
  1442770.4 |####################
  1447472.8 |########################################
  1452175.3 |####################
  1456877.8 |####################
  1461580.2 |
  1466282.6 |
  1470985.1 |
  1475687.5 |
  1480390.0 |
  1485092.4 |
  1489794.9 |
  1494497.3 |
  1499199.8 |
  1503902.2 |
  1508604.7 |
  1513307.1 |
  1518009.6 |
  1522712.0 |
  1527414.5 |
  1532116.9 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1446953.8-1495714.8 ns)
  1446953.8 |########################################
  1449391.9 |
  1451829.9 |##########
  1454267.9 |
  1456706.0 |
  1459144.1 |
  1461582.1 |
  1464020.2 |
  1466458.2 |
  1468896.2 |
  1471334.3 |
  1473772.4 |
  1476210.4 |
  1478648.4 |
  1481086.5 |
  1483524.6 |
  1485962.6 |
  1488400.7 |
  1490838.7 |
  1493276.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.8% of algo (FFI overhead may distort results)
