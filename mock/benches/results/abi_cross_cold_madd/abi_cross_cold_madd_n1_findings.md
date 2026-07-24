# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_null dominates: 179% faster than the next best (abi_cross_cold_madd_cold_null)

abi_cross_cold_madd_warm_null (4.89 us) leads abi_cross_cold_madd_cold_null (13.63 us) by 179%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_madd_cold_scalar is an outlier: 569.3x slower than the field

abi_cross_cold_madd_cold_scalar (2.78 ms) is 569.3x the fastest (4.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (4.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} (20152% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} with a 20152% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 569.3x the fastest

Fastest abi_cross_cold_madd_warm_null (4.89 us) to slowest abi_cross_cold_madd_cold_scalar (2.78 ms): 569.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 4890.2 ns median
- 3 variants significantly slower than baseline
- Spread: 569.30x (fastest 4890.2 ns, slowest 2784011.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 15999ns | 15915ns | 15615ns | 15899ns | 16341ns | +118.06% |
| abi_cross_cold_madd_cold_scalar | 2792350ns | 2787774ns | 2773390ns | 2786421ns | 2810725ns | +37957.96% |
| abi_cross_cold_madd_warm_null | 7337ns | 7199ns | 7005ns | 7168ns | 7756ns | base |
| abi_cross_cold_madd_warm_scalar | 2762852ns | 2764926ns | 2752618ns | 2761009ns | 2770734ns | +37555.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 13678ns | 13311ns | 13965ns | +173.21% | 0.000 |
| abi_cross_cold_madd_cold_scalar | 2788626ns | 2769949ns | 2806803ns | +55602.16% | 0.000 |
| abi_cross_cold_madd_warm_null | 5006ns | 4800ns | 5295ns | base | 0.000 |
| abi_cross_cold_madd_warm_scalar | 2758917ns | 2749021ns | 2766532ns | +55008.72% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 35665.8 | 19544.6 | 13678.0 | n/a |
| abi_cross_cold_madd_cold_scalar | 85176.1 | 2788086.3 | 2788626.5 | n/a |
| abi_cross_cold_madd_warm_null | 29306.8 | 5125.7 | 5006.3 | n/a |
| abi_cross_cold_madd_warm_scalar | 85656.4 | 2755791.6 | 2758916.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.000 | 35.2% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_madd_warm_null | 0.000 | 98.1% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 15999ns | 15999ns | +118.06% |
| abi_cross_cold_madd_cold_scalar | 2792350ns | 2792350ns | +37957.96% |
| abi_cross_cold_madd_warm_null | 7337ns | 7337ns | base |
| abi_cross_cold_madd_warm_scalar | 2762852ns | 2762852ns | +37555.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 4890ns | base | --- | [4834, 5295] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 13634ns | +8799.6ns (+179.9%) | [+8248, +8967]ns | [13435, 13965] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_cold_scalar | 2784011ns | +2779177.5ns (+56831.6%) | [+2770175, +2801508]ns | [2775065, 2806803] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2761026ns | +2756120.4ns (+56360.1%) | [+2744213, +2761398]ns | [2749193, 2766532] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 5498ns | +146.6% | +50804.3% | +50164.2% |
| 2 | 4905ns | +184.0% | +56376.5% | +56160.8% |
| 3 | 4868ns | +180.6% | +57054.2% | +56367.8% |
| 4 | 5092ns | +175.0% | +55184.4% | +53892.8% |
| 5 | 4876ns | +173.0% | +56920.0% | +56561.2% |
| 6 | 4800ns | +183.5% | +57937.9% | +57608.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.413 | moderate- |
| abi_cross_cold_madd_cold_scalar | -0.340 | moderate- |
| abi_cross_cold_madd_warm_null | -0.095 | ok |
| abi_cross_cold_madd_warm_scalar | 0.285 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 147175.9ns | 13678.0ns | 1076.0% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8455720.1ns | 2788626.5ns | 303.2% | HIGH |
| abi_cross_cold_madd_warm_null | 126606.3ns | 5006.3ns | 2528.9% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8352884.3ns | 2758916.9ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 13311.2-13965.4 ns)
  13311.2 |########################################
  13343.9 |
  13376.6 |
  13409.3 |
  13442.0 |
  13474.8 |
  13507.5 |
  13540.2 |########################################
  13572.9 |
  13605.6 |########################################
  13638.3 |########################################
  13671.0 |
  13703.7 |
  13736.4 |
  13769.1 |
  13801.9 |
  13834.6 |
  13867.3 |
  13900.0 |########################################
  13932.7 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2769948.8-2806802.7 ns)
  2769948.8 |########################################
  2771791.5 |
  2773634.2 |
  2775476.9 |
  2777319.6 |
  2779162.3 |########################################
  2781005.0 |########################################
  2782847.7 |
  2784690.4 |########################################
  2786533.1 |
  2788375.8 |
  2790218.4 |
  2792061.1 |
  2793903.8 |
  2795746.5 |
  2797589.2 |########################################
  2799431.9 |
  2801274.6 |
  2803117.3 |
  2804960.0 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 4799.6-5294.8 ns)
   4799.6 |########################################
   4824.4 |
   4849.1 |########################################
   4873.9 |########################################
   4898.6 |########################################
   4923.4 |
   4948.2 |
   4972.9 |
   4997.7 |
   5022.4 |
   5047.2 |
   5072.0 |########################################
   5096.7 |
   5121.5 |
   5146.2 |
   5171.0 |
   5195.8 |
   5220.5 |
   5245.3 |
   5270.0 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2749020.8-2766531.7 ns)
  2749020.8 |########################################
  2749896.3 |
  2750771.9 |
  2751647.4 |
  2752523.0 |
  2753398.5 |
  2754274.1 |
  2755149.6 |
  2756025.1 |
  2756900.7 |
  2757776.2 |
  2758651.8 |####################
  2759527.3 |
  2760402.9 |
  2761278.4 |
  2762153.9 |####################
  2763029.5 |####################
  2763905.0 |
  2764780.6 |
  2765656.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=1077.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=2562.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=302.5% of algo (FFI overhead may distort results)
