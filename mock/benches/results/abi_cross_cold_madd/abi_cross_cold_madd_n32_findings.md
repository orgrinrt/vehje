# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_scalar is an outlier: 1226.9x slower than the field

abi_cross_cold_madd_warm_scalar (2.76 ms) is 1226.9x the fastest (2.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_madd_cold_null shows alternating (throttle bounce) (autocorr -0.64)

abi_cross_cold_madd_cold_null's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (2.25 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} (114328% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} with a 114328% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1226.9x the fastest

Fastest abi_cross_cold_madd_warm_null (2.25 us) to slowest abi_cross_cold_madd_warm_scalar (2.76 ms): 1226.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 2248.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1226.94x (fastest 2248.6 ns, slowest 2758831.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 4742ns | 4710ns | 4571ns | 4702ns | 4887ns | +4.65% |
| abi_cross_cold_madd_cold_scalar | 2787020ns | 2758983ns | 2747782ns | 2756520ns | 2852388ns | +61406.87% |
| abi_cross_cold_madd_warm_null | 4531ns | 4532ns | 4412ns | 4504ns | 4632ns | base |
| abi_cross_cold_madd_warm_scalar | 2764668ns | 2762629ns | 2746459ns | 2759772ns | 2781118ns | +60913.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 2403ns | 2344ns | 2437ns | +6.54% | 0.013 |
| abi_cross_cold_madd_cold_scalar | 2783171ns | 2744281ns | 2848336ns | +123283.93% | 0.000 |
| abi_cross_cold_madd_warm_null | 2256ns | 2213ns | 2297ns | base | 0.014 |
| abi_cross_cold_madd_warm_scalar | 2760962ns | 2743125ns | 2777248ns | +122299.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 34821.3 | 2497.7 | 2403.1 | n/a |
| abi_cross_cold_madd_cold_scalar | 85534.0 | 2785138.6 | 2783171.3 | n/a |
| abi_cross_cold_madd_warm_null | 26972.3 | 2363.3 | 2255.7 | n/a |
| abi_cross_cold_madd_warm_scalar | 76836.7 | 2759292.5 | 2760962.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.013 | 91.9% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.014 | 98.4% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 4742ns | 4742ns | +4.65% |
| abi_cross_cold_madd_cold_scalar | 2787020ns | 2787020ns | +61406.87% |
| abi_cross_cold_madd_warm_null | 4531ns | 4531ns | base |
| abi_cross_cold_madd_warm_scalar | 2764668ns | 2764668ns | +60913.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 2249ns | base | --- | [2221, 2297] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 2408ns | +148.7ns (+6.6%) | [+78, +215]ns | [2365, 2437] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_cold_scalar | 2755085ns | +2752792.3ns (+122425.2%) | [+2743857, +2846097]ns | [2746093, 2848336] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2758832ns | +2756571.9ns (+122593.3%) | [+2744581, +2774968]ns | [2746808, 2777248] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 2213ns | +9.3% | +132348.7% | +124193.5% |
| 2 | 2330ns | +0.6% | +118243.2% | +119029.4% |
| 3 | 2230ns | +10.1% | +122961.9% | +124530.4% |
| 4 | 2255ns | +6.3% | +121973.6% | +122188.8% |
| 5 | 2242ns | +6.4% | +122481.3% | +122268.1% |
| 6 | 2265ns | +6.8% | +122028.2% | +121756.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.639 | HIGH- (thermal bounce) |
| abi_cross_cold_madd_cold_scalar | -0.001 | ok |
| abi_cross_cold_madd_warm_null | -0.629 | HIGH- (thermal bounce) |
| abi_cross_cold_madd_warm_scalar | 0.139 | ok |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 124929.7ns | 2403.1ns | 5198.6% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8435492.3ns | 2783171.3ns | 303.1% | HIGH |
| abi_cross_cold_madd_warm_null | 116074.2ns | 2255.7ns | 5145.8% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8349391.4ns | 2760962.5ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 2344.2-2436.7 ns)
   2344.2 |########################################
   2348.8 |
   2353.4 |
   2358.1 |
   2362.7 |
   2367.3 |
   2371.9 |
   2376.6 |
   2381.2 |########################################
   2385.8 |
   2390.4 |
   2395.1 |########################################
   2399.7 |
   2404.3 |
   2408.9 |
   2413.6 |########################################
   2418.2 |########################################
   2422.8 |
   2427.4 |
   2432.1 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2744280.8-2848336.0 ns)
  2744280.8 |########################################
  2749483.6 |####################
  2754686.3 |####################
  2759889.1 |
  2765091.8 |####################
  2770294.6 |
  2775497.4 |
  2780700.1 |
  2785902.9 |
  2791105.7 |
  2796308.4 |
  2801511.2 |
  2806713.9 |
  2811916.7 |
  2817119.5 |
  2822322.2 |
  2827525.0 |
  2832727.8 |
  2837930.5 |
  2843133.3 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 2212.9-2297.1 ns)
   2212.9 |########################################
   2217.1 |
   2221.3 |
   2225.5 |
   2229.7 |########################################
   2233.9 |
   2238.2 |########################################
   2242.4 |
   2246.6 |
   2250.8 |
   2255.0 |########################################
   2259.2 |
   2263.4 |########################################
   2267.6 |
   2271.8 |
   2276.1 |
   2280.3 |
   2284.5 |
   2288.7 |
   2292.9 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2743125.0-2777247.7 ns)
  2743125.0 |########################################
  2744831.1 |
  2746537.3 |
  2748243.4 |
  2749949.5 |########################################
  2751655.7 |
  2753361.8 |
  2755067.9 |
  2756774.1 |########################################
  2758480.2 |########################################
  2760186.4 |
  2761892.5 |
  2763598.6 |
  2765304.8 |
  2767010.9 |
  2768717.0 |
  2770423.2 |
  2772129.3 |
  2773835.4 |########################################
  2775541.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=5203.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=5172.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=302.3% of algo (FFI overhead may distort results)
