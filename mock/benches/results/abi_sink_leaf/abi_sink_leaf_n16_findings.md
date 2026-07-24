# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_sink_leaf_batched_sink is fastest but the noisiest (CV 11.0%)

abi_sink_leaf_batched_sink wins on median (1.45 ms) yet has the highest variance (CV 11.0%), while abi_sink_leaf_per_record_sink is the steadiest (CV 0.2%, 1.45 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.31 us) is smaller than the fastest variant's own run-to-run std-dev (159.89 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_leaf_batched_sink vs stability leader abi_sink_leaf_per_record_sink (+0% speed for 65.1x steadier)

abi_sink_leaf_batched_sink is fastest (1.45 ms, CV 11.0%); abi_sink_leaf_per_record_sink gives up 0.2% median for 65.1x lower variance (CV 0.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.2% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1447844.5 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 1447844.5 ns, slowest 1450153.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1520572ns | 1450289ns | 1442633ns | 1449642ns | 1665935ns | +4.76% |
| abi_sink_leaf_batched_sink_decode | 1451032ns | 1450624ns | 1438162ns | 1449612ns | 1459596ns | -0.03% |
| abi_sink_leaf_null_sink | 1451475ns | 1452119ns | 1444054ns | 1451706ns | 1454840ns | base |
| abi_sink_leaf_per_record_sink | 1453215ns | 1452711ns | 1449673ns | 1452528ns | 1456016ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1518052ns | 1440308ns | 1663168ns | +4.77% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1448522ns | 1435781ns | 1456856ns | -0.03% | 0.000 |
| abi_sink_leaf_null_sink | 1448987ns | 1441498ns | 1452363ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1450750ns | 1447204ns | 1453634ns | +0.12% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 37964.9 | 1458609.3 | 1518051.7 | n/a |
| abi_sink_leaf_batched_sink_decode | 37929.1 | 1448623.6 | 1448521.8 | 5 |
| abi_sink_leaf_null_sink | 35522.5 | 1449443.9 | 1448986.7 | n/a |
| abi_sink_leaf_per_record_sink | 35463.2 | 1449852.1 | 1450749.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink_decode; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.2% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.1% |
| abi_sink_leaf_null_sink | 0.000 | 99.0% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1520572ns | 1520572ns | +4.76% |
| abi_sink_leaf_batched_sink_decode | 1451032ns | 1451032ns | -0.03% |
| abi_sink_leaf_null_sink | 1451475ns | 1451475ns | base |
| abi_sink_leaf_per_record_sink | 1453215ns | 1453215ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1449731ns | base | --- | [1444866, 1452363] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1447845ns | no significant difference | [-7935, +217410]ns | [1443143, 1663168] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_leaf_batched_sink_decode | 1448216ns | no significant difference | [-8632, +7301]ns | [1440494, 1456856] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_leaf_per_record_sink | 1450153ns | no significant difference | [-1866, +6733]ns | [1448462, 1453634] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1449998ns | -0.1% | -0.2% | +0.0% |
| 2 | 1449463ns | -0.2% | +0.5% | +0.0% |
| 3 | 1448235ns | -0.5% | -0.2% | -0.1% |
| 4 | 1450017ns | +29.3% | -1.0% | +0.4% |
| 5 | 1454710ns | -0.5% | +0.2% | -0.2% |
| 6 | 1441498ns | +0.6% | +0.5% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.247 | moderate- |
| abi_sink_leaf_batched_sink_decode | -0.316 | moderate- |
| abi_sink_leaf_null_sink | -0.410 | moderate- |
| abi_sink_leaf_per_record_sink | -0.232 | moderate- |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 3/6, lost 2/6
- **abi_sink_leaf_batched_sink_decode**: won 3/6, lost 3/6
- **abi_sink_leaf_per_record_sink**: won 1/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4414368.6ns | 1518051.7ns | 290.8% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4385987.6ns | 1448521.8ns | 302.8% | HIGH |
| abi_sink_leaf_null_sink | 4385184.5ns | 1448986.7ns | 302.6% | HIGH |
| abi_sink_leaf_per_record_sink | 4387810.4ns | 1450749.7ns | 302.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1440308.3-1663167.7 ns)
  1440308.3 |########################################
  1451451.3 |
  1462594.2 |
  1473737.2 |
  1484880.2 |
  1496023.2 |
  1507166.1 |
  1518309.1 |
  1529452.1 |
  1540595.0 |
  1551738.0 |
  1562881.0 |
  1574023.9 |
  1585166.9 |
  1596309.9 |
  1607452.9 |
  1618595.8 |
  1629738.8 |
  1640881.8 |
  1652024.7 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1435781.2-1456855.8 ns)
  1435781.2 |########################################
  1436834.9 |
  1437888.7 |
  1438942.4 |
  1439996.1 |
  1441049.9 |
  1442103.6 |
  1443157.3 |
  1444211.0 |########################################
  1445264.8 |
  1446318.5 |
  1447372.2 |########################################
  1448426.0 |########################################
  1449479.7 |
  1450533.4 |
  1451587.1 |
  1452640.9 |
  1453694.6 |
  1454748.3 |
  1455802.1 |########################################
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1441497.9-1452363.1 ns)
  1441497.9 |####################
  1442041.2 |
  1442584.4 |
  1443127.7 |
  1443670.9 |
  1444214.2 |
  1444757.5 |
  1445300.7 |
  1445844.0 |
  1446387.3 |
  1446930.5 |
  1447473.8 |
  1448017.0 |####################
  1448560.3 |
  1449103.6 |####################
  1449646.8 |########################################
  1450190.1 |
  1450733.4 |
  1451276.6 |
  1451819.9 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1447203.8-1453633.8 ns)
  1447203.8 |########################################
  1447525.3 |
  1447846.8 |
  1448168.3 |
  1448489.8 |
  1448811.3 |
  1449132.8 |
  1449454.3 |########################################
  1449775.8 |########################################
  1450097.3 |########################################
  1450418.8 |
  1450740.3 |
  1451061.8 |
  1451383.3 |
  1451704.8 |########################################
  1452026.3 |
  1452347.8 |
  1452669.3 |
  1452990.8 |
  1453312.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.5% of algo (FFI overhead may distort results)
