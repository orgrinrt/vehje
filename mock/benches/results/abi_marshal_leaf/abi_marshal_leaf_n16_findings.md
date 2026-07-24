# abi_marshal (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_leaf_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_leaf_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_leaf_marshal_null dominates: 5228% faster than the next best (abi_marshal_leaf_aos)

abi_marshal_leaf_marshal_null (27.53 us) leads abi_marshal_leaf_aos (1.47 ms) by 5228%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_leaf_marshal_null beats baseline by 98% (significant)

abi_marshal_leaf_marshal_null is -1.44 ms (98%) faster than baseline abi_marshal_leaf_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_leaf_soa_transposed is an outlier: 55.1x slower than the field

abi_marshal_leaf_soa_transposed (1.52 ms) is 55.1x the fastest (27.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_leaf_marshal_null} vs {abi_marshal_leaf_aos, abi_marshal_leaf_soa_native, abi_marshal_leaf_soa_transposed} (5228% apart)

The field splits into a fast tier {abi_marshal_leaf_marshal_null} and a slow tier {abi_marshal_leaf_aos, abi_marshal_leaf_soa_native, abi_marshal_leaf_soa_transposed} with a 5228% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 55.1x the fastest

Fastest abi_marshal_leaf_marshal_null (27.53 us) to slowest abi_marshal_leaf_soa_transposed (1.52 ms): 55.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_leaf_marshal_null** at 27532.1 ns median (-98.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 55.15x (fastest 27532.1 ns, slowest 1518285.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1469438ns | 1469477ns | 1467082ns | 1468697ns | 1471726ns | base |
| abi_marshal_leaf_marshal_null | 29581ns | 29778ns | 28780ns | 29476ns | 30139ns | -97.99% |
| abi_marshal_leaf_soa_native | 1479036ns | 1479415ns | 1475373ns | 1478376ns | 1481858ns | +0.65% |
| abi_marshal_leaf_soa_transposed | 1519591ns | 1520840ns | 1513360ns | 1519152ns | 1523365ns | +3.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1466936ns | 1464593ns | 1469184ns | base | 0.000 |
| abi_marshal_leaf_marshal_null | 27317ns | 26574ns | 27812ns | -98.14% | 0.001 |
| abi_marshal_leaf_soa_native | 1476526ns | 1472954ns | 1479325ns | +0.65% | 0.000 |
| abi_marshal_leaf_soa_transposed | 1517072ns | 1510936ns | 1520818ns | +3.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 36786.1 | 1467217.8 | 1466936.2 | n/a |
| abi_marshal_leaf_marshal_null | 27792.9 | 27817.2 | 27317.0 | n/a |
| abi_marshal_leaf_soa_native | 38112.4 | 1475572.4 | 1476526.0 | n/a |
| abi_marshal_leaf_soa_transposed | 37973.0 | 1515951.7 | 1517072.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_marshal_leaf_marshal_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_leaf_aos | 0.000 | 1.8% |
| abi_marshal_leaf_marshal_null | 0.001 | 96.5% |
| abi_marshal_leaf_soa_native | 0.000 | 1.8% |
| abi_marshal_leaf_soa_transposed | 0.000 | 1.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_leaf_aos | 1469438ns | 1469438ns | base |
| abi_marshal_leaf_marshal_null | 29581ns | 29581ns | -97.99% |
| abi_marshal_leaf_soa_native | 1479036ns | 1479036ns | +0.65% |
| abi_marshal_leaf_soa_transposed | 1519591ns | 1519591ns | +3.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1466984ns | base | --- | [1464640, 1469184] | --- | --- | --- | --- |
| abi_marshal_leaf_marshal_null | 27532ns | -1439452.1ns (-98.1%) | [-1441984, -1437421]ns | [26607, 27812] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_leaf_soa_native | 1476848ns | +8525.2ns (+0.6%) | [+6372, +13872]ns | [1473405, 1479325] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_leaf_soa_transposed | 1518286ns | +51122.3ns (+3.5%) | [+45035, +54251]ns | [1512112, 1520818] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_leaf_aos | abi_marshal_leaf_marshal_null | abi_marshal_leaf_soa_native | abi_marshal_leaf_soa_transposed |
|---|---|---|---|---|
| 1 | 1467750ns | -98.1% | +0.6% | +3.6% |
| 2 | 1468806ns | -98.1% | +0.3% | +3.3% |
| 3 | 1469562ns | -98.2% | +0.5% | +2.8% |
| 4 | 1464688ns | -98.2% | +1.0% | +3.7% |
| 5 | 1466219ns | -98.1% | +0.9% | +3.7% |
| 6 | 1464593ns | -98.1% | +0.6% | +3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_leaf_aos | 0.173 | ok |
| abi_marshal_leaf_marshal_null | 0.202 | moderate+ |
| abi_marshal_leaf_soa_native | -0.045 | ok |
| abi_marshal_leaf_soa_transposed | -0.228 | moderate- |

**Consistency summary:**

- **abi_marshal_leaf_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_leaf_soa_native**: won 0/6, lost 6/6
- **abi_marshal_leaf_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 4440611.9ns | 1466936.2ns | 302.7% | HIGH |
| abi_marshal_leaf_marshal_null | 191602.4ns | 27317.0ns | 701.4% | HIGH |
| abi_marshal_leaf_soa_native | 4467518.3ns | 1476526.0ns | 302.6% | HIGH |
| abi_marshal_leaf_soa_transposed | 4587455.0ns | 1517072.1ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_leaf_aos (n=6, range 1464592.9-1469184.0 ns)
  1464592.9 |########################################
  1464822.5 |
  1465052.0 |
  1465281.6 |
  1465511.1 |
  1465740.7 |
  1465970.2 |
  1466199.8 |####################
  1466429.3 |
  1466658.9 |
  1466888.4 |
  1467118.0 |
  1467347.5 |
  1467577.1 |####################
  1467806.6 |
  1468036.2 |
  1468265.7 |
  1468495.3 |
  1468724.8 |####################
  1468954.4 |
  (0 below, 1 above range)

abi_marshal_leaf_marshal_null (n=6, range 26573.7-27812.3 ns)
  26573.7 |########################################
  26635.6 |########################################
  26697.6 |
  26759.5 |
  26821.4 |
  26883.4 |
  26945.3 |
  27007.2 |
  27069.1 |
  27131.1 |
  27193.0 |
  27254.9 |
  27316.9 |########################################
  27378.8 |
  27440.7 |
  27502.7 |
  27564.6 |
  27626.5 |
  27688.4 |########################################
  27750.4 |########################################
  (0 below, 1 above range)

abi_marshal_leaf_soa_native (n=6, range 1472954.2-1479325.4 ns)
  1472954.2 |########################################
  1473272.8 |
  1473591.3 |########################################
  1473909.9 |
  1474228.4 |
  1474547.0 |
  1474865.6 |
  1475184.1 |
  1475502.7 |
  1475821.2 |
  1476139.8 |########################################
  1476458.4 |
  1476776.9 |
  1477095.5 |########################################
  1477414.0 |
  1477732.6 |
  1478051.2 |
  1478369.7 |
  1478688.3 |
  1479006.8 |########################################
  (0 below, 1 above range)

abi_marshal_leaf_soa_transposed (n=6, range 1510936.2-1520818.4 ns)
  1510936.2 |########################################
  1511430.3 |
  1511924.4 |
  1512418.5 |
  1512912.6 |########################################
  1513406.7 |
  1513900.8 |
  1514395.0 |
  1514889.1 |
  1515383.2 |
  1515877.3 |
  1516371.4 |
  1516865.5 |
  1517359.6 |########################################
  1517853.7 |
  1518347.8 |
  1518841.9 |########################################
  1519336.0 |
  1519830.1 |
  1520324.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_leaf_aos**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_marshal_null**: bridge=700.2% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_native**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_transposed**: bridge=302.4% of algo (FFI overhead may distort results)
