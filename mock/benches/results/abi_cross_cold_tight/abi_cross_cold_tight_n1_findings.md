# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_warm_null dominates: 175% faster than the next best (abi_cross_cold_tight_cold_null)

abi_cross_cold_tight_warm_null (5.01 us) leads abi_cross_cold_tight_cold_null (13.78 us) by 175%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_tight_cold_scalar is an outlier: 424.2x slower than the field

abi_cross_cold_tight_cold_scalar (2.13 ms) is 424.2x the fastest (5.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_tight_warm_null)

The baseline abi_cross_cold_tight_warm_null is the fastest (5.01 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (15278% apart)

The field splits into a fast tier {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 15278% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 424.2x the fastest

Fastest abi_cross_cold_tight_warm_null (5.01 us) to slowest abi_cross_cold_tight_cold_scalar (2.13 ms): 424.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_tight_warm_null) is the fastest** at 5012.9 ns median
- 3 variants significantly slower than baseline
- Spread: 424.23x (fastest 5012.9 ns, slowest 2126626.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 16382ns | 16109ns | 15976ns | 16076ns | 17044ns | +119.68% |
| abi_cross_cold_tight_cold_scalar | 2208100ns | 2130594ns | 2117257ns | 2128011ns | 2373655ns | +29509.91% |
| abi_cross_cold_tight_warm_null | 7457ns | 7337ns | 7198ns | 7292ns | 7835ns | base |
| abi_cross_cold_tight_warm_scalar | 2151510ns | 2123066ns | 2110817ns | 2119639ns | 2219663ns | +28751.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 14012ns | 13596ns | 14629ns | +178.84% | 0.000 |
| abi_cross_cold_tight_cold_scalar | 2203895ns | 2113218ns | 2369186ns | +43755.99% | 0.000 |
| abi_cross_cold_tight_warm_null | 5025ns | 4904ns | 5159ns | base | 0.000 |
| abi_cross_cold_tight_warm_scalar | 2147405ns | 2106592ns | 2215455ns | +42631.87% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 35152.7 | 19653.2 | 14012.4 | n/a |
| abi_cross_cold_tight_cold_scalar | 101635.1 | 2192494.0 | 2203895.1 | n/a |
| abi_cross_cold_tight_warm_null | 31694.9 | 5084.6 | 5025.3 | n/a |
| abi_cross_cold_tight_warm_scalar | 92129.8 | 2134519.4 | 2147404.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.000 | 35.6% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_tight_warm_null | 0.000 | 97.8% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 16382ns | 16382ns | +119.68% |
| abi_cross_cold_tight_cold_scalar | 2208100ns | 2208100ns | +29509.91% |
| abi_cross_cold_tight_warm_null | 7457ns | 7457ns | base |
| abi_cross_cold_tight_warm_scalar | 2151510ns | 2151510ns | +28751.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 5013ns | base | --- | [4904, 5159] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 13780ns | +8816.9ns (+175.9%) | [+8502, +9643]ns | [13628, 14629] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_cold_scalar | 2126626ns | +2121467.5ns (+42319.7%) | [+2110919, +2364223]ns | [2115873, 2369186] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2118962ns | +2113925.8ns (+42169.3%) | [+2102720, +2210492]ns | [2107797, 2215455] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 5004ns | +171.7% | +42128.9% | +42155.0% |
| 2 | 4904ns | +181.7% | +43101.8% | +42858.4% |
| 3 | 5068ns | +184.3% | +41840.0% | +41795.7% |
| 4 | 5022ns | +173.7% | +48739.0% | +45109.8% |
| 5 | 4905ns | +202.8% | +46505.8% | +43952.7% |
| 6 | 5249ns | +160.2% | +40432.1% | +40077.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.490 | moderate- |
| abi_cross_cold_tight_cold_scalar | 0.095 | ok |
| abi_cross_cold_tight_warm_null | -0.360 | moderate- |
| abi_cross_cold_tight_warm_scalar | 0.024 | ok |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 148514.6ns | 14012.4ns | 1059.9% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6699364.5ns | 2203895.1ns | 304.0% | HIGH |
| abi_cross_cold_tight_warm_null | 128919.4ns | 5025.3ns | 2565.4% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6525017.8ns | 2147404.7ns | 303.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 13595.8-14629.4 ns)
  13595.8 |########################################
  13647.5 |########################################
  13699.2 |########################################
  13750.8 |
  13802.5 |########################################
  13854.2 |
  13905.9 |
  13957.6 |
  14009.2 |
  14060.9 |
  14112.6 |
  14164.3 |
  14216.0 |
  14267.6 |
  14319.3 |
  14371.0 |########################################
  14422.7 |
  14474.4 |
  14526.0 |
  14577.7 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2113218.3-2369185.8 ns)
  2113218.3 |########################################
  2126016.7 |#############
  2138815.0 |
  2151613.4 |
  2164411.8 |
  2177210.2 |
  2190008.5 |
  2202806.9 |
  2215605.3 |
  2228403.7 |
  2241202.0 |
  2254000.4 |
  2266798.8 |
  2279597.2 |#############
  2292395.5 |
  2305193.9 |
  2317992.3 |
  2330790.7 |
  2343589.0 |
  2356387.4 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 4903.8-5158.8 ns)
   4903.8 |########################################
   4916.5 |
   4929.3 |
   4942.0 |
   4954.8 |
   4967.5 |
   4980.3 |
   4993.0 |####################
   5005.8 |
   5018.5 |####################
   5031.3 |
   5044.0 |
   5056.8 |####################
   5069.5 |
   5082.3 |
   5095.0 |
   5107.8 |
   5120.5 |
   5133.3 |
   5146.0 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2106591.7-2215455.0 ns)
  2106591.7 |########################################
  2112034.9 |####################
  2117478.0 |
  2122921.2 |####################
  2128364.4 |
  2133807.5 |
  2139250.7 |
  2144693.9 |
  2150137.0 |
  2155580.2 |####################
  2161023.4 |
  2166466.5 |
  2171909.7 |
  2177352.8 |
  2182796.0 |
  2188239.2 |
  2193682.3 |
  2199125.5 |
  2204568.7 |
  2210011.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=1062.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=2557.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=305.8% of algo (FFI overhead may distort results)
