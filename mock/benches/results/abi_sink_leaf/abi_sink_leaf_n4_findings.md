# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.1% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink_decode** at 1448549.9 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1448549.9 ns, slowest 1450240.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1450589ns | 1451501ns | 1444482ns | 1450962ns | 1453082ns | -0.16% |
| abi_sink_leaf_batched_sink_decode | 1451464ns | 1450851ns | 1450124ns | 1450655ns | 1453348ns | -0.10% |
| abi_sink_leaf_null_sink | 1452860ns | 1452501ns | 1451142ns | 1452156ns | 1454774ns | base |
| abi_sink_leaf_per_record_sink | 1453165ns | 1452736ns | 1449885ns | 1451799ns | 1456856ns | +0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1448052ns | 1441938ns | 1450543ns | -0.16% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1449054ns | 1447705ns | 1450848ns | -0.09% | 0.000 |
| abi_sink_leaf_null_sink | 1450364ns | 1448712ns | 1452286ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1450595ns | 1447406ns | 1454126ns | +0.02% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 36425.5 | 1448498.8 | 1448052.5 | 0 |
| abi_sink_leaf_batched_sink_decode | 35277.1 | 1448263.2 | 1449053.7 | n/a |
| abi_sink_leaf_null_sink | 35525.1 | 1450069.3 | 1450363.9 | n/a |
| abi_sink_leaf_per_record_sink | 36689.4 | 1451035.3 | 1450595.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.5% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_leaf_null_sink | 0.000 | 99.4% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1450589ns | 1450589ns | -0.16% |
| abi_sink_leaf_batched_sink_decode | 1451464ns | 1451464ns | -0.10% |
| abi_sink_leaf_null_sink | 1452860ns | 1452860ns | base |
| abi_sink_leaf_per_record_sink | 1453165ns | 1453165ns | +0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1450025ns | base | --- | [1448781, 1452286] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1448954ns | no significant difference | [-6344, +491]ns | [1444660, 1450543] | no | 0.3281 | 0.2188 | 0 |
| abi_sink_leaf_batched_sink_decode | 1448550ns | -1304.7ns (-0.1%) | [-1781, -845]ns | [1447764, 1450848] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_leaf_per_record_sink | 1450241ns | no significant difference | [-3441, +5345]ns | [1447418, 1454126] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1450617ns | -0.0% | -0.1% | -0.2% |
| 2 | 1453180ns | -0.4% | -0.1% | -0.0% |
| 3 | 1449434ns | -0.5% | -0.1% | -0.1% |
| 4 | 1448712ns | +0.1% | -0.1% | +0.3% |
| 5 | 1448849ns | -0.1% | -0.1% | +0.4% |
| 6 | 1451392ns | -0.0% | -0.1% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | -0.285 | moderate- |
| abi_sink_leaf_batched_sink_decode | -0.135 | ok |
| abi_sink_leaf_null_sink | 0.038 | ok |
| abi_sink_leaf_per_record_sink | -0.389 | moderate- |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 3/6, lost 0/6
- **abi_sink_leaf_batched_sink_decode**: won 3/6, lost 0/6
- **abi_sink_leaf_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4385038.5ns | 1448052.5ns | 302.8% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4382895.4ns | 1449053.7ns | 302.5% | HIGH |
| abi_sink_leaf_null_sink | 4387929.6ns | 1450363.9ns | 302.5% | HIGH |
| abi_sink_leaf_per_record_sink | 4391277.9ns | 1450595.0ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1441937.9-1450543.3 ns)
  1441937.9 |####################
  1442368.2 |
  1442798.4 |
  1443228.7 |
  1443659.0 |
  1444089.2 |
  1444519.5 |
  1444949.8 |
  1445380.1 |
  1445810.3 |
  1446240.6 |
  1446670.9 |
  1447101.1 |####################
  1447531.4 |
  1447961.7 |####################
  1448391.9 |
  1448822.2 |
  1449252.5 |
  1449682.8 |########################################
  1450113.0 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1447705.0-1450847.7 ns)
  1447705.0 |########################################
  1447862.1 |####################
  1448019.3 |
  1448176.4 |
  1448333.5 |
  1448490.7 |
  1448647.8 |
  1448804.9 |
  1448962.1 |
  1449119.2 |####################
  1449276.4 |
  1449433.5 |
  1449590.6 |
  1449747.8 |
  1449904.9 |
  1450062.0 |
  1450219.2 |
  1450376.3 |####################
  1450533.4 |
  1450690.6 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1448712.5-1452285.6 ns)
  1448712.5 |########################################
  1448891.2 |
  1449069.8 |
  1449248.5 |
  1449427.1 |####################
  1449605.8 |
  1449784.4 |
  1449963.1 |
  1450141.8 |
  1450320.4 |
  1450499.1 |####################
  1450677.7 |
  1450856.4 |
  1451035.0 |
  1451213.7 |####################
  1451392.4 |
  1451571.0 |
  1451749.7 |
  1451928.3 |
  1452107.0 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1447405.8-1454125.9 ns)
  1447405.8 |########################################
  1447741.8 |
  1448077.8 |
  1448413.8 |
  1448749.8 |
  1449085.8 |
  1449421.8 |
  1449757.8 |
  1450093.8 |
  1450429.8 |
  1450765.8 |
  1451101.8 |
  1451437.8 |
  1451773.8 |
  1452109.8 |
  1452445.8 |#############
  1452781.8 |#############
  1453117.8 |
  1453453.8 |
  1453789.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.6% of algo (FFI overhead may distort results)
