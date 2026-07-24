# abi_marshal (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_leaf_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_leaf_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_leaf_marshal_null dominates: 11470% faster than the next best (abi_marshal_leaf_soa_native)

abi_marshal_leaf_marshal_null (12.54 us) leads abi_marshal_leaf_soa_native (1.45 ms) by 11470%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_leaf_marshal_null beats baseline by 99% (significant)

abi_marshal_leaf_marshal_null is -1.44 ms (99%) faster than baseline abi_marshal_leaf_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_leaf_soa_transposed is an outlier: 116.6x slower than the field

abi_marshal_leaf_soa_transposed (1.46 ms) is 116.6x the fastest (12.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_leaf_marshal_null shows warm-up / thermal drift (autocorr +0.51)

abi_marshal_leaf_marshal_null's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_leaf_marshal_null} vs {abi_marshal_leaf_soa_native, abi_marshal_leaf_aos, abi_marshal_leaf_soa_transposed} (11470% apart)

The field splits into a fast tier {abi_marshal_leaf_marshal_null} and a slow tier {abi_marshal_leaf_soa_native, abi_marshal_leaf_aos, abi_marshal_leaf_soa_transposed} with a 11470% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 116.6x the fastest

Fastest abi_marshal_leaf_marshal_null (12.54 us) to slowest abi_marshal_leaf_soa_transposed (1.46 ms): 116.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_leaf_marshal_null** at 12540.6 ns median (-99.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 116.58x (fastest 12540.6 ns, slowest 1462016.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1511756ns | 1457943ns | 1456139ns | 1457349ns | 1621176ns | base |
| abi_marshal_leaf_marshal_null | 14695ns | 14860ns | 14103ns | 14653ns | 15053ns | -99.03% |
| abi_marshal_leaf_soa_native | 1453677ns | 1453428ns | 1448542ns | 1451860ns | 1458970ns | -3.84% |
| abi_marshal_leaf_soa_transposed | 1470583ns | 1464504ns | 1461555ns | 1464387ns | 1484392ns | -2.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1509098ns | 1453635ns | 1618112ns | base | 0.000 |
| abi_marshal_leaf_marshal_null | 12407ns | 11944ns | 12726ns | -99.18% | 0.000 |
| abi_marshal_leaf_soa_native | 1451185ns | 1445947ns | 1456465ns | -3.84% | 0.000 |
| abi_marshal_leaf_soa_transposed | 1468091ns | 1459220ns | 1481801ns | -2.72% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 43742.0 | 1498073.8 | 1509098.4 | n/a |
| abi_marshal_leaf_marshal_null | 27776.0 | 12549.6 | 12406.9 | n/a |
| abi_marshal_leaf_soa_native | 36946.7 | 1451660.1 | 1451185.2 | n/a |
| abi_marshal_leaf_soa_transposed | 38036.9 | 1471216.2 | 1468091.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_leaf_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_leaf_aos | 0.000 | 0.8% |
| abi_marshal_leaf_marshal_null | 0.000 | 95.2% |
| abi_marshal_leaf_soa_native | 0.000 | 0.8% |
| abi_marshal_leaf_soa_transposed | 0.000 | 0.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_leaf_aos | 1511756ns | 1511756ns | base |
| abi_marshal_leaf_marshal_null | 14695ns | 14695ns | -99.03% |
| abi_marshal_leaf_soa_native | 1453677ns | 1453677ns | -3.84% |
| abi_marshal_leaf_soa_transposed | 1470583ns | 1470583ns | -2.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1455488ns | base | --- | [1453695, 1618112] | --- | --- | --- | --- |
| abi_marshal_leaf_marshal_null | 12541ns | -1442833.1ns (-99.1%) | [-1605860, -1441381]ns | [11954, 12726] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_leaf_soa_native | 1450985ns | -7569.1ns (-0.5%) | [-166014, -157]ns | [1446105, 1456465] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_marshal_leaf_soa_transposed | 1462017ns | no significant difference | [-139879, +11000]ns | [1460456, 1481801] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_leaf_aos | abi_marshal_leaf_marshal_null | abi_marshal_leaf_soa_native | abi_marshal_leaf_soa_transposed |
|---|---|---|---|---|
| 1 | 1779445ns | -99.3% | -18.1% | -16.0% |
| 2 | 1455442ns | -99.1% | -0.0% | +0.4% |
| 3 | 1453635ns | -99.1% | +0.0% | +1.1% |
| 4 | 1455534ns | -99.1% | -0.7% | +0.4% |
| 5 | 1456780ns | -99.2% | -0.7% | +0.4% |
| 6 | 1453756ns | -99.2% | -0.4% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_leaf_aos | -0.033 | ok |
| abi_marshal_leaf_marshal_null | 0.507 | HIGH+ (drift/warm-up) |
| abi_marshal_leaf_soa_native | 0.491 | moderate+ |
| abi_marshal_leaf_soa_transposed | -0.102 | ok |

**Consistency summary:**

- **abi_marshal_leaf_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_leaf_soa_native**: won 4/6, lost 0/6
- **abi_marshal_leaf_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 4557895.5ns | 1509098.4ns | 302.0% | HIGH |
| abi_marshal_leaf_marshal_null | 144498.1ns | 12406.9ns | 1164.7% | HIGH |
| abi_marshal_leaf_soa_native | 4394252.4ns | 1451185.2ns | 302.8% | HIGH |
| abi_marshal_leaf_soa_transposed | 4448137.9ns | 1468091.1ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_marshal_leaf_aos (n=6, range 1453634.6-1618112.3 ns)
  1453634.6 |########################################
  1461858.5 |
  1470082.4 |
  1478306.3 |
  1486530.1 |
  1494754.0 |
  1502977.9 |
  1511201.8 |
  1519425.7 |
  1527649.6 |
  1535873.5 |
  1544097.3 |
  1552321.2 |
  1560545.1 |
  1568769.0 |
  1576992.9 |
  1585216.8 |
  1593440.6 |
  1601664.5 |
  1609888.4 |
  (0 below, 1 above range)

abi_marshal_leaf_marshal_null (n=6, range 11944.2-12726.2 ns)
  11944.2 |########################################
  11983.3 |
  12022.4 |
  12061.5 |
  12100.6 |
  12139.7 |
  12178.8 |
  12217.9 |
  12257.0 |
  12296.1 |
  12335.2 |
  12374.3 |
  12413.4 |
  12452.5 |
  12491.6 |
  12530.7 |########################################
  12569.8 |
  12608.9 |
  12648.0 |####################
  12687.1 |
  (0 below, 1 above range)

abi_marshal_leaf_soa_native (n=6, range 1445947.1-1456464.8 ns)
  1445947.1 |########################################
  1446473.0 |
  1446998.9 |
  1447524.8 |
  1448050.6 |####################
  1448576.5 |
  1449102.4 |
  1449628.3 |
  1450154.2 |
  1450680.1 |
  1451205.9 |
  1451731.8 |
  1452257.7 |
  1452783.6 |
  1453309.5 |####################
  1453835.4 |
  1454361.3 |
  1454887.1 |####################
  1455413.0 |
  1455938.9 |
  (0 below, 1 above range)

abi_marshal_leaf_soa_transposed (n=6, range 1459220.4-1481800.6 ns)
  1459220.4 |#############
  1460349.4 |
  1461478.4 |########################################
  1462607.4 |
  1463736.4 |
  1464865.5 |
  1465994.5 |
  1467123.5 |
  1468252.5 |#############
  1469381.5 |
  1470510.5 |
  1471639.5 |
  1472768.5 |
  1473897.6 |
  1475026.6 |
  1476155.6 |
  1477284.6 |
  1478413.6 |
  1479542.6 |
  1480671.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_leaf_aos**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_marshal_null**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **abi_marshal_leaf_marshal_null**: bridge=1147.7% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_native**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_transposed**: bridge=302.5% of algo (FFI overhead may distort results)
