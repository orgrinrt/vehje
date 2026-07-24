# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_null dominates: 30% faster than the next best (abi_cross_cold_wideselect_cold_null)

abi_cross_cold_wideselect_warm_null (3.99 us) leads abi_cross_cold_wideselect_cold_null (5.18 us) by 30%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_wideselect_cold_scalar is an outlier: 529.1x slower than the field

abi_cross_cold_wideselect_cold_scalar (2.11 ms) is 529.1x the fastest (3.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (3.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} (40549% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} with a 40549% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 529.1x the fastest

Fastest abi_cross_cold_wideselect_warm_null (3.99 us) to slowest abi_cross_cold_wideselect_cold_scalar (2.11 ms): 529.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 3986.7 ns median
- 3 variants significantly slower than baseline
- Spread: 529.10x (fastest 3986.7 ns, slowest 2109330.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 7992ns | 7459ns | 7248ns | 7427ns | 9211ns | +26.92% |
| abi_cross_cold_wideselect_cold_scalar | 2112895ns | 2112888ns | 2105322ns | 2111214ns | 2119202ns | +33456.57% |
| abi_cross_cold_wideselect_warm_null | 6297ns | 6294ns | 6226ns | 6287ns | 6346ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2116857ns | 2108495ns | 2104810ns | 2108213ns | 2135847ns | +33519.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 5325ns | 5007ns | 5748ns | +33.69% | 0.001 |
| abi_cross_cold_wideselect_cold_scalar | 2109211ns | 2101639ns | 2115489ns | +52850.01% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 3983ns | 3930ns | 4019ns | base | 0.001 |
| abi_cross_cold_wideselect_warm_scalar | 2113003ns | 2101076ns | 2131753ns | +52945.21% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 37149.4 | 7452.2 | 5325.4 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 84066.4 | 2111985.6 | 2109210.7 | n/a |
| abi_cross_cold_wideselect_warm_null | 29928.3 | 4121.7 | 3983.4 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 79957.1 | 2113643.5 | 2113003.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.001 | 75.9% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_wideselect_warm_null | 0.001 | 98.6% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 7992ns | 7992ns | +26.92% |
| abi_cross_cold_wideselect_cold_scalar | 2112895ns | 2112895ns | +33456.57% |
| abi_cross_cold_wideselect_warm_null | 6297ns | 6297ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2116857ns | 2116857ns | +33519.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 3987ns | base | --- | [3945, 4019] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 5178ns | +1209.5ns (+30.3%) | [+1055, +1762]ns | [5050, 5748] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2109330ns | +2105315.9ns (+52809.1%) | [+2098829, +2111537]ns | [2102813, 2115489] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2104754ns | +2100785.4ns (+52695.5%) | [+2098524, +2127750]ns | [2102502, 2131753] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 4007ns | +29.3% | +52347.9% | +52443.9% |
| 2 | 3975ns | +56.6% | +53078.2% | +52908.8% |
| 3 | 3930ns | +31.6% | +53771.5% | +53442.9% |
| 4 | 3960ns | +26.4% | +53036.3% | +53034.9% |
| 5 | 4031ns | +26.4% | +52305.9% | +53393.0% |
| 6 | 3998ns | +31.9% | +52584.8% | +52454.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | -0.129 | ok |
| abi_cross_cold_wideselect_cold_scalar | -0.351 | moderate- |
| abi_cross_cold_wideselect_warm_null | 0.165 | ok |
| abi_cross_cold_wideselect_warm_scalar | -0.324 | moderate- |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 120207.0ns | 5325.4ns | 2257.2% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6418175.9ns | 2109210.7ns | 304.3% | HIGH |
| abi_cross_cold_wideselect_warm_null | 124473.1ns | 3983.4ns | 3124.8% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6430322.7ns | 2113003.1ns | 304.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 5006.7-5748.4 ns)
   5006.7 |####################
   5043.8 |
   5080.9 |####################
   5117.9 |
   5155.0 |########################################
   5192.1 |
   5229.2 |
   5266.3 |####################
   5303.4 |
   5340.4 |
   5377.5 |
   5414.6 |
   5451.7 |
   5488.8 |
   5525.9 |
   5562.9 |
   5600.0 |
   5637.1 |
   5674.2 |
   5711.3 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2101638.8-2115489.2 ns)
  2101638.8 |########################################
  2102331.3 |
  2103023.8 |
  2103716.4 |########################################
  2104408.9 |
  2105101.4 |
  2105793.9 |########################################
  2106486.4 |
  2107179.0 |
  2107871.5 |
  2108564.0 |
  2109256.5 |
  2109949.0 |
  2110641.6 |
  2111334.1 |
  2112026.6 |########################################
  2112719.1 |
  2113411.6 |########################################
  2114104.2 |
  2114796.7 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 3929.6-4018.9 ns)
   3929.6 |########################################
   3934.1 |
   3938.5 |
   3943.0 |
   3947.5 |
   3951.9 |
   3956.4 |########################################
   3960.9 |
   3965.3 |
   3969.8 |
   3974.3 |########################################
   3978.7 |
   3983.2 |
   3987.7 |
   3992.1 |
   3996.6 |########################################
   4001.1 |
   4005.5 |########################################
   4010.0 |
   4014.5 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2101076.2-2131753.3 ns)
  2101076.2 |####################
  2102610.1 |########################################
  2104143.9 |####################
  2105677.8 |
  2107211.6 |####################
  2108745.5 |
  2110279.3 |
  2111813.2 |
  2113347.0 |
  2114880.9 |
  2116414.8 |
  2117948.6 |
  2119482.5 |
  2121016.3 |
  2122550.2 |
  2124084.0 |
  2125617.9 |
  2127151.7 |
  2128685.6 |
  2130219.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=2272.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=3119.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=304.3% of algo (FFI overhead may distort results)
