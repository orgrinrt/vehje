# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_null dominates: 38% faster than the next best (abi_cross_cold_leaf_cold_null)

abi_cross_cold_leaf_warm_null (4.16 us) leads abi_cross_cold_leaf_cold_null (5.74 us) by 38%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_leaf_cold_scalar is an outlier: 400.4x slower than the field

abi_cross_cold_leaf_cold_scalar (1.67 ms) is 400.4x the fastest (4.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_leaf_warm_null shows alternating (throttle bounce) (autocorr -0.65)

abi_cross_cold_leaf_warm_null's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (4.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} (28701% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} with a 28701% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 400.4x the fastest

Fastest abi_cross_cold_leaf_warm_null (4.16 us) to slowest abi_cross_cold_leaf_cold_scalar (1.67 ms): 400.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 4160.0 ns median
- 3 variants significantly slower than baseline
- Spread: 400.39x (fastest 4160.0 ns, slowest 1665613.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 8132ns | 8314ns | 7438ns | 8046ns | 8607ns | +22.63% |
| abi_cross_cold_leaf_cold_scalar | 1664136ns | 1669532ns | 1549745ns | 1642608ns | 1753624ns | +24995.42% |
| abi_cross_cold_leaf_warm_null | 6631ns | 6538ns | 6322ns | 6473ns | 7023ns | base |
| abi_cross_cold_leaf_warm_scalar | 1652101ns | 1658138ns | 1546800ns | 1630381ns | 1737331ns | +24813.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 5633ns | 5168ns | 5977ns | +33.73% | 0.001 |
| abi_cross_cold_leaf_cold_scalar | 1660177ns | 1546180ns | 1749356ns | +39310.11% | 0.000 |
| abi_cross_cold_leaf_warm_null | 4213ns | 4010ns | 4460ns | base | 0.001 |
| abi_cross_cold_leaf_warm_scalar | 1647790ns | 1543053ns | 1732578ns | +39016.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 37536.7 | 7745.4 | 5633.5 | n/a |
| abi_cross_cold_leaf_cold_scalar | 85206.7 | 1683668.3 | 1660177.1 | n/a |
| abi_cross_cold_leaf_warm_null | 30880.7 | 4334.2 | 4212.6 | n/a |
| abi_cross_cold_leaf_warm_scalar | 87382.4 | 1664192.4 | 1647789.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.001 | 69.8% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.001 | 96.4% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 8132ns | 8132ns | +22.63% |
| abi_cross_cold_leaf_cold_scalar | 1664136ns | 1664136ns | +24995.42% |
| abi_cross_cold_leaf_warm_null | 6631ns | 6631ns | base |
| abi_cross_cold_leaf_warm_scalar | 1652101ns | 1652101ns | +24813.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 4160ns | base | --- | [4018, 4460] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 5742ns | +1365.8ns (+32.8%) | [+1163, +1734]ns | [5181, 5977] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1665613ns | +1661577.2ns (+39941.8%) | [+1561102, +1745214]ns | [1565562, 1749356] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1653819ns | +1649800.4ns (+39658.7%) | [+1552727, +1728205]ns | [1556974, 1732578] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 4026ns | +28.4% | +44404.5% | +41980.7% |
| 2 | 4432ns | +34.9% | +35660.5% | +35343.6% |
| 3 | 4062ns | +47.1% | +39981.2% | +37890.3% |
| 4 | 4258ns | +36.5% | +39987.6% | +40864.8% |
| 5 | 4010ns | +29.5% | +42370.8% | +40134.2% |
| 6 | 4487ns | +26.4% | +34358.3% | +38248.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.113 | ok |
| abi_cross_cold_leaf_cold_scalar | -0.289 | moderate- |
| abi_cross_cold_leaf_warm_null | -0.650 | HIGH- (thermal bounce) |
| abi_cross_cold_leaf_warm_scalar | -0.328 | moderate- |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 122619.1ns | 5633.5ns | 2176.6% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 5108546.0ns | 1660177.1ns | 307.7% | HIGH |
| abi_cross_cold_leaf_warm_null | 126111.2ns | 4212.6ns | 2993.7% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 5083321.7ns | 1647789.9ns | 308.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 5167.9-5977.1 ns)
   5167.9 |########################################
   5208.4 |
   5248.8 |
   5289.3 |
   5329.7 |
   5370.2 |
   5410.7 |
   5451.1 |
   5491.6 |
   5532.0 |
   5572.5 |
   5613.0 |
   5653.4 |####################
   5693.9 |
   5734.3 |
   5774.8 |####################
   5815.3 |
   5855.7 |
   5896.2 |
   5936.6 |####################
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1546180.4-1749356.0 ns)
  1546180.4 |####################
  1556339.2 |
  1566498.0 |
  1576656.7 |####################
  1586815.5 |
  1596974.3 |
  1607133.1 |
  1617291.9 |
  1627450.6 |####################
  1637609.4 |
  1647768.2 |
  1657927.0 |
  1668085.8 |
  1678244.5 |
  1688403.3 |
  1698562.1 |########################################
  1708720.9 |
  1718879.7 |
  1729038.4 |
  1739197.2 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 4010.4-4459.6 ns)
   4010.4 |########################################
   4032.9 |
   4055.3 |####################
   4077.8 |
   4100.2 |
   4122.7 |
   4145.2 |
   4167.6 |
   4190.1 |
   4212.5 |
   4235.0 |
   4257.5 |####################
   4279.9 |
   4302.4 |
   4324.8 |
   4347.3 |
   4369.8 |
   4392.2 |
   4414.7 |####################
   4437.1 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1543053.3-1732577.5 ns)
  1543053.3 |########################################
  1552529.5 |
  1562005.7 |########################################
  1571481.9 |
  1580958.1 |
  1590434.4 |
  1599910.6 |
  1609386.8 |########################################
  1618863.0 |
  1628339.2 |
  1637815.4 |
  1647291.6 |
  1656767.8 |
  1666244.0 |
  1675720.2 |
  1685196.4 |########################################
  1694672.7 |
  1704148.9 |
  1713625.1 |########################################
  1723101.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=2157.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=3016.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=305.4% of algo (FFI overhead may distort results)
