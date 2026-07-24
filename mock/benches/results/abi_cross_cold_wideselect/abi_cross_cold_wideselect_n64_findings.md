# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_cold_scalar is an outlier: 848.5x slower than the field

abi_cross_cold_wideselect_cold_scalar (2.10 ms) is 848.5x the fastest (2.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_wideselect_warm_scalar shows alternating (throttle bounce) (autocorr -0.85)

abi_cross_cold_wideselect_warm_scalar's per-pass series has lag-1 autocorrelation -0.85, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (2.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} (82778% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} with a 82778% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 848.5x the fastest

Fastest abi_cross_cold_wideselect_warm_null (2.47 us) to slowest abi_cross_cold_wideselect_cold_scalar (2.10 ms): 848.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 2471.4 ns median
- 2 variants significantly slower than baseline
- Spread: 848.45x (fastest 2471.4 ns, slowest 2096908.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 4852ns | 4835ns | 4733ns | 4803ns | 4985ns | +2.10% |
| abi_cross_cold_wideselect_cold_scalar | 2100661ns | 2100502ns | 2094742ns | 2100054ns | 2104532ns | +44102.58% |
| abi_cross_cold_wideselect_warm_null | 4752ns | 4704ns | 4611ns | 4695ns | 4909ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2099326ns | 2098775ns | 2086792ns | 2096896ns | 2109238ns | +44074.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 2544ns | 2481ns | 2611ns | +1.82% | 0.025 |
| abi_cross_cold_wideselect_cold_scalar | 2097150ns | 2091454ns | 2100973ns | +83851.46% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 2498ns | 2413ns | 2584ns | base | 0.026 |
| abi_cross_cold_wideselect_warm_scalar | 2095785ns | 2083332ns | 2105572ns | +83796.85% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 33885.9 | 2741.0 | 2543.5 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 79960.6 | 2098145.8 | 2097149.5 | n/a |
| abi_cross_cold_wideselect_warm_null | 29318.3 | 2731.2 | 2498.1 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 72468.1 | 2096422.4 | 2095785.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.025 | 95.4% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_wideselect_warm_null | 0.026 | 97.6% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 4852ns | 4852ns | +2.10% |
| abi_cross_cold_wideselect_cold_scalar | 2100661ns | 2100661ns | +44102.58% |
| abi_cross_cold_wideselect_warm_null | 4752ns | 4752ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2099326ns | 2099326ns | +44074.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 2471ns | base | --- | [2438, 2584] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 2528ns | no significant difference | [-93, +140]ns | [2491, 2611] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2096908ns | +2094431.5ns (+84745.0%) | [+2091110, +2098413]ns | [2093567, 2100973] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2095251ns | +2092694.4ns (+84674.8%) | [+2084064, +2103103]ns | [2086532, 2105572] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 2468ns | +4.9% | +84632.6% | +84967.6% |
| 2 | 2645ns | -6.2% | +79247.6% | +78934.6% |
| 3 | 2464ns | +3.5% | +85055.4% | +85457.6% |
| 4 | 2523ns | -0.8% | +82956.2% | +82463.8% |
| 5 | 2475ns | +6.3% | +84878.8% | +84890.5% |
| 6 | 2413ns | +3.8% | +86753.2% | +86506.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | -0.629 | HIGH- (thermal bounce) |
| abi_cross_cold_wideselect_cold_scalar | -0.363 | moderate- |
| abi_cross_cold_wideselect_warm_null | -0.276 | moderate- |
| abi_cross_cold_wideselect_warm_scalar | -0.848 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 2/6, lost 4/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 121845.1ns | 2543.5ns | 4790.4% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6379372.2ns | 2097149.5ns | 304.2% | HIGH |
| abi_cross_cold_wideselect_warm_null | 114935.5ns | 2498.1ns | 4601.0% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6364966.8ns | 2095785.2ns | 303.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 2480.8-2611.1 ns)
   2480.8 |####################
   2487.3 |
   2493.8 |
   2500.3 |########################################
   2506.9 |
   2513.4 |
   2519.9 |
   2526.4 |
   2532.9 |
   2539.4 |
   2545.9 |####################
   2552.4 |
   2559.0 |
   2565.5 |
   2572.0 |
   2578.5 |
   2585.0 |####################
   2591.5 |
   2598.0 |
   2604.5 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2091453.7-2100973.4 ns)
  2091453.7 |########################################
  2091929.7 |
  2092405.7 |
  2092881.6 |
  2093357.6 |
  2093833.6 |
  2094309.6 |
  2094785.6 |
  2095261.6 |########################################
  2095737.5 |########################################
  2096213.5 |
  2096689.5 |
  2097165.5 |
  2097641.5 |########################################
  2098117.5 |
  2098593.4 |########################################
  2099069.4 |
  2099545.4 |
  2100021.4 |
  2100497.4 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 2412.9-2584.4 ns)
   2412.9 |########################################
   2421.5 |
   2430.0 |
   2438.6 |
   2447.2 |
   2455.8 |########################################
   2464.3 |########################################
   2472.9 |########################################
   2481.5 |
   2490.1 |
   2498.6 |
   2507.2 |
   2515.8 |########################################
   2524.3 |
   2532.9 |
   2541.5 |
   2550.1 |
   2558.6 |
   2567.2 |
   2575.8 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2083331.7-2105572.5 ns)
  2083331.7 |########################################
  2084443.7 |
  2085555.8 |
  2086667.8 |
  2087779.9 |
  2088891.9 |########################################
  2090003.9 |########################################
  2091116.0 |
  2092228.0 |
  2093340.1 |
  2094452.1 |
  2095564.1 |
  2096676.2 |
  2097788.2 |
  2098900.3 |########################################
  2100012.3 |
  2101124.3 |
  2102236.4 |########################################
  2103348.4 |
  2104460.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=4850.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=4656.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=304.0% of algo (FFI overhead may distort results)
