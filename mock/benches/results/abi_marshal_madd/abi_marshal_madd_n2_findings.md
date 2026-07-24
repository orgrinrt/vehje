# abi_marshal (madd)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_madd_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_madd_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_madd_marshal_null dominates: 21800% faster than the next best (abi_marshal_madd_soa_native)

abi_marshal_madd_marshal_null (12.21 us) leads abi_marshal_madd_soa_native (2.68 ms) by 21800%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_madd_marshal_null beats baseline by 100% (significant)

abi_marshal_madd_marshal_null is -2.67 ms (100%) faster than baseline abi_marshal_madd_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_madd_soa_transposed is an outlier: 220.3x slower than the field

abi_marshal_madd_soa_transposed (2.69 ms) is 220.3x the fastest (12.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_madd_marshal_null shows alternating (throttle bounce) (autocorr -0.65)

abi_marshal_madd_marshal_null's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_madd_marshal_null} vs {abi_marshal_madd_soa_native, abi_marshal_madd_aos, abi_marshal_madd_soa_transposed} (21800% apart)

The field splits into a fast tier {abi_marshal_madd_marshal_null} and a slow tier {abi_marshal_madd_soa_native, abi_marshal_madd_aos, abi_marshal_madd_soa_transposed} with a 21800% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 220.3x the fastest

Fastest abi_marshal_madd_marshal_null (12.21 us) to slowest abi_marshal_madd_soa_transposed (2.69 ms): 220.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_madd_marshal_null** at 12214.8 ns median (-99.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 220.32x (fastest 12214.8 ns, slowest 2691147.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2678863ns | 2681298ns | 2661366ns | 2679032ns | 2687359ns | base |
| abi_marshal_madd_marshal_null | 14511ns | 14482ns | 13919ns | 14377ns | 15008ns | -99.46% |
| abi_marshal_madd_soa_native | 2676193ns | 2677561ns | 2669212ns | 2675786ns | 2680294ns | -0.10% |
| abi_marshal_madd_soa_transposed | 2692847ns | 2693793ns | 2679999ns | 2691788ns | 2700858ns | +0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2676321ns | 2658831ns | 2684673ns | base | 0.000 |
| abi_marshal_madd_marshal_null | 12239ns | 11756ns | 12654ns | -99.54% | 0.000 |
| abi_marshal_madd_soa_native | 2673601ns | 2666635ns | 2677610ns | -0.10% | 0.000 |
| abi_marshal_madd_soa_transposed | 2690241ns | 2677410ns | 2698217ns | +0.52% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_madd_aos | 42041.3 | 2677975.0 | 2676321.1 | n/a |
| abi_marshal_madd_marshal_null | 27789.2 | 12396.9 | 12238.6 | n/a |
| abi_marshal_madd_soa_native | 39976.2 | 2677129.7 | 2673600.9 | n/a |
| abi_marshal_madd_soa_transposed | 42671.3 | 2689291.4 | 2690241.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_madd_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_madd_aos | 0.000 | 0.4% |
| abi_marshal_madd_marshal_null | 0.000 | 96.2% |
| abi_marshal_madd_soa_native | 0.000 | 0.4% |
| abi_marshal_madd_soa_transposed | 0.000 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_madd_aos | 2678863ns | 2678863ns | base |
| abi_marshal_madd_marshal_null | 14511ns | 14511ns | -99.46% |
| abi_marshal_madd_soa_native | 2676193ns | 2676193ns | -0.10% |
| abi_marshal_madd_soa_transposed | 2692847ns | 2692847ns | +0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2678868ns | base | --- | [2665422, 2684673] | --- | --- | --- | --- |
| abi_marshal_madd_marshal_null | 12215ns | -2666720.2ns (-99.5%) | [-2672320, -2653207]ns | [11847, 12654] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_madd_soa_native | 2675044ns | no significant difference | [-11204, +5653]ns | [2668149, 2677610] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_madd_soa_transposed | 2691148ns | +13793.3ns (+0.5%) | [+6979, +20987]ns | [2681359, 2698217] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_madd_aos | abi_marshal_madd_marshal_null | abi_marshal_madd_soa_native | abi_marshal_madd_soa_transposed |
|---|---|---|---|---|
| 1 | 2688622ns | -99.5% | -0.5% | +0.1% |
| 2 | 2680725ns | -99.6% | -0.1% | +0.9% |
| 3 | 2672012ns | -99.5% | +0.1% | +0.5% |
| 4 | 2679715ns | -99.6% | -0.1% | +0.4% |
| 5 | 2678021ns | -99.5% | -0.3% | +0.5% |
| 6 | 2658831ns | -99.5% | +0.3% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_madd_aos | -0.007 | ok |
| abi_marshal_madd_marshal_null | -0.649 | HIGH- (thermal bounce) |
| abi_marshal_madd_soa_native | 0.318 | moderate+ |
| abi_marshal_madd_soa_transposed | -0.196 | ok |

**Consistency summary:**

- **abi_marshal_madd_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_madd_soa_native**: won 3/6, lost 2/6
- **abi_marshal_madd_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_madd_aos | 8076370.2ns | 2676321.1ns | 301.8% | HIGH |
| abi_marshal_madd_marshal_null | 144908.4ns | 12238.6ns | 1184.0% | HIGH |
| abi_marshal_madd_soa_native | 8064205.3ns | 2673600.9ns | 301.6% | HIGH |
| abi_marshal_madd_soa_transposed | 8113016.8ns | 2690241.0ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_marshal_madd_aos (n=6, range 2658831.2-2684673.4 ns)
  2658831.2 |####################
  2660123.3 |
  2661415.4 |
  2662707.5 |
  2663999.6 |
  2665291.7 |
  2666583.8 |
  2667876.0 |
  2669168.1 |
  2670460.2 |
  2671752.3 |####################
  2673044.4 |
  2674336.5 |
  2675628.6 |
  2676920.7 |####################
  2678212.8 |
  2679504.9 |########################################
  2680797.0 |
  2682089.1 |
  2683381.2 |
  (0 below, 1 above range)

abi_marshal_madd_marshal_null (n=6, range 11755.8-12653.8 ns)
  11755.8 |########################################
  11800.7 |
  11845.6 |
  11890.5 |
  11935.4 |########################################
  11980.3 |
  12025.2 |########################################
  12070.1 |
  12115.0 |
  12159.9 |
  12204.8 |
  12249.7 |
  12294.6 |
  12339.5 |
  12384.4 |########################################
  12429.3 |
  12474.2 |
  12519.1 |########################################
  12564.0 |
  12608.9 |
  (0 below, 1 above range)

abi_marshal_madd_soa_native (n=6, range 2666635.0-2677610.5 ns)
  2666635.0 |########################################
  2667183.8 |
  2667732.5 |
  2668281.3 |
  2668830.1 |
  2669378.9 |########################################
  2669927.6 |
  2670476.4 |
  2671025.2 |
  2671574.0 |
  2672122.7 |
  2672671.5 |
  2673220.3 |
  2673769.0 |
  2674317.8 |########################################
  2674866.6 |
  2675415.4 |########################################
  2675964.1 |
  2676512.9 |
  2677061.7 |########################################
  (0 below, 1 above range)

abi_marshal_madd_soa_transposed (n=6, range 2677410.4-2698216.6 ns)
  2677410.4 |########################################
  2678450.7 |
  2679491.0 |
  2680531.3 |
  2681571.6 |
  2682612.0 |
  2683652.3 |
  2684692.6 |########################################
  2685732.9 |
  2686773.2 |
  2687813.5 |
  2688853.8 |
  2689894.1 |########################################
  2690934.5 |########################################
  2691974.8 |########################################
  2693015.1 |
  2694055.4 |
  2695095.7 |
  2696136.0 |
  2697176.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_madd_aos**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_marshal_madd_marshal_null**: bridge=1188.7% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_native**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_transposed**: bridge=301.5% of algo (FFI overhead may distort results)
