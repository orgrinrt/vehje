# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_null dominates: 175% faster than the next best (abi_cross_cold_scatter_cold_null)

abi_cross_cold_scatter_warm_null (4.94 us) leads abi_cross_cold_scatter_cold_null (13.59 us) by 175%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_scatter_warm_scalar is an outlier: 454.1x slower than the field

abi_cross_cold_scatter_warm_scalar (2.24 ms) is 454.1x the fastest (4.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_scatter_warm_null shows alternating (throttle bounce) (autocorr -0.61)

abi_cross_cold_scatter_warm_null's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (4.94 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} (16401% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} with a 16401% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 454.1x the fastest

Fastest abi_cross_cold_scatter_warm_null (4.94 us) to slowest abi_cross_cold_scatter_warm_scalar (2.24 ms): 454.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 4940.9 ns median
- 3 variants significantly slower than baseline
- Spread: 454.12x (fastest 4940.9 ns, slowest 2243750.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 15957ns | 15915ns | 15723ns | 15865ns | 16210ns | +118.92% |
| abi_cross_cold_scatter_cold_scalar | 2271335ns | 2245982ns | 2236010ns | 2243788ns | 2330319ns | +31061.92% |
| abi_cross_cold_scatter_warm_null | 7289ns | 7329ns | 7134ns | 7281ns | 7378ns | base |
| abi_cross_cold_scatter_warm_scalar | 2255459ns | 2247565ns | 2233301ns | 2244217ns | 2283402ns | +30844.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 13606ns | 13335ns | 13812ns | +175.10% | 0.000 |
| abi_cross_cold_scatter_cold_scalar | 2267320ns | 2232319ns | 2325860ns | +45744.12% | 0.000 |
| abi_cross_cold_scatter_warm_null | 4946ns | 4869ns | 5013ns | base | 0.000 |
| abi_cross_cold_scatter_warm_scalar | 2251425ns | 2229175ns | 2279114ns | +45422.73% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 35440.8 | 20205.3 | 13605.6 | n/a |
| abi_cross_cold_scatter_cold_scalar | 90257.2 | 2263069.2 | 2267320.1 | n/a |
| abi_cross_cold_scatter_warm_null | 28825.1 | 5010.6 | 4945.7 | n/a |
| abi_cross_cold_scatter_warm_scalar | 84594.6 | 2257705.6 | 2251425.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.000 | 35.8% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_scatter_warm_null | 0.000 | 98.5% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 15957ns | 15957ns | +118.92% |
| abi_cross_cold_scatter_cold_scalar | 2271335ns | 2271335ns | +31061.92% |
| abi_cross_cold_scatter_warm_null | 7289ns | 7289ns | base |
| abi_cross_cold_scatter_warm_scalar | 2255459ns | 2255459ns | +30844.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 4941ns | base | --- | [4883, 5013] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 13588ns | +8597.5ns (+174.0%) | [+8510, +8872]ns | [13416, 13812] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2242105ns | +2237169.1ns (+45279.0%) | [+2229035, +2320919]ns | [2233995, 2325860] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2243751ns | +2238790.0ns (+45311.8%) | [+2226527, +2274121]ns | [2231411, 2279114] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 5002ns | +171.0% | +44661.4% | +46110.0% |
| 2 | 4897ns | +182.9% | +48790.9% | +45511.6% |
| 3 | 4897ns | +175.7% | +45488.2% | +45684.6% |
| 4 | 5025ns | +171.0% | +44394.5% | +44591.5% |
| 5 | 4869ns | +173.9% | +46010.2% | +45681.1% |
| 6 | 4985ns | +176.3% | +45189.1% | +44974.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | -0.502 | HIGH- (thermal bounce) |
| abi_cross_cold_scatter_cold_scalar | -0.305 | moderate- |
| abi_cross_cold_scatter_warm_null | -0.615 | HIGH- (thermal bounce) |
| abi_cross_cold_scatter_warm_scalar | -0.133 | ok |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 148079.5ns | 13605.6ns | 1088.4% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6904232.7ns | 2267320.1ns | 304.5% | HIGH |
| abi_cross_cold_scatter_warm_null | 125448.0ns | 4945.7ns | 2536.5% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6860673.1ns | 2251425.2ns | 304.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 13335.0-13812.5 ns)
  13335.0 |########################################
  13358.9 |
  13382.8 |
  13406.6 |
  13430.5 |
  13454.4 |
  13478.2 |########################################
  13502.1 |
  13526.0 |
  13549.9 |########################################
  13573.8 |
  13597.6 |########################################
  13621.5 |
  13645.4 |
  13669.2 |
  13693.1 |
  13717.0 |
  13740.9 |
  13764.8 |########################################
  13788.6 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2232318.8-2325860.2 ns)
  2232318.8 |########################################
  2236995.9 |####################
  2241672.9 |####################
  2246350.0 |
  2251027.1 |
  2255704.1 |####################
  2260381.2 |
  2265058.3 |
  2269735.4 |
  2274412.4 |
  2279089.5 |
  2283766.6 |
  2288443.6 |
  2293120.7 |
  2297797.8 |
  2302474.9 |
  2307151.9 |
  2311829.0 |
  2316506.1 |
  2321183.1 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 4869.2-5013.4 ns)
   4869.2 |####################
   4876.4 |
   4883.6 |
   4890.8 |########################################
   4898.0 |
   4905.2 |
   4912.4 |
   4919.7 |
   4926.9 |
   4934.1 |
   4941.3 |
   4948.5 |
   4955.7 |
   4962.9 |
   4970.1 |
   4977.3 |
   4984.5 |####################
   4991.7 |
   4998.9 |####################
   5006.1 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2229175.4-2279114.3 ns)
  2229175.4 |########################################
  2231672.3 |########################################
  2234169.3 |
  2236666.2 |
  2239163.2 |
  2241660.1 |########################################
  2244157.1 |########################################
  2246654.0 |########################################
  2249151.0 |
  2251647.9 |
  2254144.9 |
  2256641.8 |
  2259138.8 |
  2261635.7 |
  2264132.7 |
  2266629.6 |
  2269126.6 |
  2271623.5 |
  2274120.5 |
  2276617.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=1082.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=2522.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=304.3% of algo (FFI overhead may distort results)
