# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_scalar is an outlier: 876.9x slower than the field

abi_cross_cold_real_warm_scalar (2.19 ms) is 876.9x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (2.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (79737% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 79737% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 876.9x the fastest

Fastest abi_cross_cold_real_warm_null (2.50 us) to slowest abi_cross_cold_real_warm_scalar (2.19 ms): 876.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 2495.0 ns median
- 3 variants significantly slower than baseline
- Spread: 876.93x (fastest 2495.0 ns, slowest 2187947.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 5033ns | 5010ns | 4870ns | 4980ns | 5195ns | +3.27% |
| abi_cross_cold_real_cold_scalar | 2193722ns | 2188086ns | 2181320ns | 2186105ns | 2211349ns | +44907.58% |
| abi_cross_cold_real_warm_null | 4874ns | 4832ns | 4650ns | 4784ns | 5122ns | base |
| abi_cross_cold_real_warm_scalar | 2197720ns | 2191504ns | 2180269ns | 2189226ns | 2219186ns | +44989.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2737ns | 2675ns | 2798ns | +8.50% | 0.006 |
| abi_cross_cold_real_cold_scalar | 2190173ns | 2177775ns | 2207524ns | +86715.74% | 0.000 |
| abi_cross_cold_real_warm_null | 2523ns | 2468ns | 2600ns | base | 0.006 |
| abi_cross_cold_real_warm_scalar | 2194169ns | 2177038ns | 2215535ns | +86874.13% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33380.2 | 2859.0 | 2737.2 | n/a |
| abi_cross_cold_real_cold_scalar | 77683.8 | 2192417.6 | 2190172.9 | n/a |
| abi_cross_cold_real_warm_null | 27468.2 | 2624.4 | 2522.8 | n/a |
| abi_cross_cold_real_warm_scalar | 73805.1 | 2215093.5 | 2194168.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.006 | 90.2% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.006 | 98.9% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 5033ns | 5033ns | +3.27% |
| abi_cross_cold_real_cold_scalar | 2193722ns | 2193722ns | +44907.58% |
| abi_cross_cold_real_warm_null | 4874ns | 4874ns | base |
| abi_cross_cold_real_warm_scalar | 2197720ns | 2197720ns | +44989.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2495ns | base | --- | [2474, 2600] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2736ns | +242.5ns (+9.7%) | [+96, +304]ns | [2677, 2798] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2184701ns | +2182175.8ns (+87462.0%) | [+2175737, +2205038]ns | [2178294, 2207524] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2187948ns | +2185390.4ns (+87590.8%) | [+2176498, +2213050]ns | [2179024, 2215535] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2503ns | +12.7% | +88663.7% | +88919.3% |
| 2 | 2635ns | +1.7% | +82535.5% | +82801.1% |
| 3 | 2564ns | +5.8% | +85179.1% | +84956.1% |
| 4 | 2479ns | +7.9% | +87797.9% | +88294.4% |
| 5 | 2468ns | +11.8% | +88747.6% | +89137.6% |
| 6 | 2487ns | +11.6% | +87674.0% | +87447.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.143 | ok |
| abi_cross_cold_real_cold_scalar | -0.267 | moderate- |
| abi_cross_cold_real_warm_null | 0.239 | moderate+ |
| abi_cross_cold_real_warm_scalar | -0.182 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 124599.3ns | 2737.2ns | 4552.2% | HIGH |
| abi_cross_cold_real_cold_scalar | 6658210.4ns | 2190172.9ns | 304.0% | HIGH |
| abi_cross_cold_real_warm_null | 118192.2ns | 2522.8ns | 4685.0% | HIGH |
| abi_cross_cold_real_warm_scalar | 6672204.1ns | 2194168.8ns | 304.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2675.0-2797.7 ns)
   2675.0 |########################################
   2681.1 |
   2687.3 |
   2693.4 |
   2699.5 |
   2705.7 |
   2711.8 |####################
   2717.9 |
   2724.1 |
   2730.2 |
   2736.3 |
   2742.5 |
   2748.6 |
   2754.8 |####################
   2760.9 |
   2767.0 |
   2773.2 |####################
   2779.3 |
   2785.4 |
   2791.6 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2177775.0-2207523.5 ns)
  2177775.0 |########################################
  2179262.4 |
  2180749.9 |
  2182237.3 |####################
  2183724.7 |
  2185212.1 |
  2186699.6 |####################
  2188187.0 |
  2189674.4 |
  2191161.8 |
  2192649.3 |####################
  2194136.7 |
  2195624.1 |
  2197111.6 |
  2198599.0 |
  2200086.4 |
  2201573.8 |
  2203061.3 |
  2204548.7 |
  2206036.1 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2468.3-2599.8 ns)
   2468.3 |########################################
   2474.9 |########################################
   2481.5 |########################################
   2488.0 |
   2494.6 |
   2501.2 |########################################
   2507.8 |
   2514.3 |
   2520.9 |
   2527.5 |
   2534.1 |
   2540.6 |
   2547.2 |
   2553.8 |
   2560.4 |########################################
   2566.9 |
   2573.5 |
   2580.1 |
   2586.7 |
   2593.2 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2177038.3-2215535.4 ns)
  2177038.3 |########################################
  2178963.2 |
  2180888.0 |########################################
  2182812.9 |
  2184737.7 |########################################
  2186662.6 |
  2188587.4 |
  2190512.3 |########################################
  2192437.1 |
  2194362.0 |
  2196286.9 |
  2198211.7 |
  2200136.6 |
  2202061.4 |########################################
  2203986.3 |
  2205911.1 |
  2207836.0 |
  2209760.8 |
  2211685.7 |
  2213610.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=4544.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=4709.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.6% of algo (FFI overhead may distort results)
