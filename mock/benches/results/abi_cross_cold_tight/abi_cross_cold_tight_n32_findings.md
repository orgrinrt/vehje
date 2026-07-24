# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_cold_scalar is an outlier: 922.0x slower than the field

abi_cross_cold_tight_cold_scalar (2.26 ms) is 922.0x the fastest (2.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null) are a dead heat (<1%)

abi_cross_cold_tight_cold_null (2.45 us) and abi_cross_cold_tight_warm_null (2.46 us) differ by 0.14%, inside the noise, even though the wider field spreads 92096.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### abi_cross_cold_tight_warm_scalar shows alternating (throttle bounce) (autocorr -0.78)

abi_cross_cold_tight_warm_scalar's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (91264% apart)

The field splits into a fast tier {abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 91264% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 922.0x the fastest

Fastest abi_cross_cold_tight_cold_null (2.45 us) to slowest abi_cross_cold_tight_cold_scalar (2.26 ms): 922.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_tight_cold_null's edge over baseline is significant but tiny (-11 ns, 0.43%)

abi_cross_cold_tight_cold_null differs from baseline abi_cross_cold_tight_warm_null by -11 ns (0.43%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_tight_cold_null** at 2453.1 ns median (-0.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 921.96x (fastest 2453.1 ns, slowest 2261663.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 4963ns | 4952ns | 4723ns | 4883ns | 5204ns | -1.34% |
| abi_cross_cold_tight_cold_scalar | 2289116ns | 2266425ns | 2183935ns | 2243898ns | 2409534ns | +45401.12% |
| abi_cross_cold_tight_warm_null | 5031ns | 4873ns | 4725ns | 4843ns | 5466ns | base |
| abi_cross_cold_tight_warm_scalar | 2282644ns | 2249503ns | 2170212ns | 2224157ns | 2426589ns | +45272.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 2469ns | 2394ns | 2544ns | +0.24% | 0.013 |
| abi_cross_cold_tight_cold_scalar | 2283919ns | 2178851ns | 2403517ns | +92629.77% | 0.000 |
| abi_cross_cold_tight_warm_null | 2463ns | 2338ns | 2555ns | base | 0.013 |
| abi_cross_cold_tight_warm_scalar | 2276948ns | 2163593ns | 2420283ns | +92346.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 36565.3 | 2564.2 | 2469.0 | n/a |
| abi_cross_cold_tight_cold_scalar | 115630.3 | 2280825.3 | 2283918.9 | n/a |
| abi_cross_cold_tight_warm_null | 32327.1 | 2623.9 | 2463.0 | n/a |
| abi_cross_cold_tight_warm_scalar | 116053.9 | 2287681.8 | 2276948.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.013 | 95.3% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_tight_warm_null | 0.013 | 95.2% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 4963ns | 4963ns | -1.34% |
| abi_cross_cold_tight_cold_scalar | 2289116ns | 2289116ns | +45401.12% |
| abi_cross_cold_tight_warm_null | 5031ns | 5031ns | base |
| abi_cross_cold_tight_warm_scalar | 2282644ns | 2282644ns | +45272.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 2456ns | base | --- | [2378, 2555] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 2453ns | no significant difference | [-113, +141]ns | [2410, 2544] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_cold_tight_cold_scalar | 2261664ns | +2259272.5ns (+91973.1%) | [+2184132, +2400963]ns | [2186576, 2403517] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2244319ns | +2241941.5ns (+91267.5%) | [+2163761, +2417754]ns | [2166243, 2420283] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 2418ns | +1.2% | +89998.4% | +95537.6% |
| 2 | 2495ns | -2.8% | +95415.0% | +86617.2% |
| 3 | 2614ns | -6.0% | +92621.9% | +95002.4% |
| 4 | 2468ns | +1.5% | +88799.3% | +87769.9% |
| 5 | 2445ns | -2.1% | +90831.2% | +96210.2% |
| 6 | 2338ns | +10.5% | +98313.9% | +92983.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.460 | moderate- |
| abi_cross_cold_tight_cold_scalar | -0.088 | ok |
| abi_cross_cold_tight_warm_null | 0.153 | ok |
| abi_cross_cold_tight_warm_scalar | -0.776 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 3/6, lost 3/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 127405.9ns | 2469.0ns | 5160.3% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6974833.2ns | 2283918.9ns | 305.4% | HIGH |
| abi_cross_cold_tight_warm_null | 122835.8ns | 2463.0ns | 4987.3% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6957839.5ns | 2276948.5ns | 305.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 2393.8-2544.2 ns)
   2393.8 |########################################
   2401.3 |
   2408.8 |
   2416.4 |
   2423.9 |########################################
   2431.4 |
   2438.9 |
   2446.4 |########################################
   2453.9 |########################################
   2461.5 |
   2469.0 |
   2476.5 |
   2484.0 |
   2491.5 |
   2499.0 |########################################
   2506.6 |
   2514.1 |
   2521.6 |
   2529.1 |
   2536.6 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2178850.8-2403517.3 ns)
  2178850.8 |########################################
  2190084.1 |########################################
  2201317.4 |
  2212550.8 |########################################
  2223784.1 |
  2235017.4 |
  2246250.8 |
  2257484.1 |
  2268717.4 |
  2279950.7 |
  2291184.0 |########################################
  2302417.4 |
  2313650.7 |
  2324884.0 |
  2336117.3 |
  2347350.7 |
  2358584.0 |
  2369817.3 |
  2381050.6 |########################################
  2392284.0 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 2337.5-2554.6 ns)
   2337.5 |########################################
   2348.4 |
   2359.2 |
   2370.1 |
   2380.9 |
   2391.8 |
   2402.6 |
   2413.5 |########################################
   2424.3 |
   2435.2 |########################################
   2446.1 |
   2456.9 |
   2467.8 |########################################
   2478.6 |
   2489.5 |########################################
   2500.3 |
   2511.2 |
   2522.0 |
   2532.9 |
   2543.7 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2163592.9-2420283.0 ns)
  2163592.9 |########################################
  2176427.4 |
  2189261.9 |
  2202096.4 |
  2214930.9 |
  2227765.4 |
  2240599.9 |
  2253434.4 |
  2266268.9 |
  2279103.4 |
  2291937.9 |
  2304772.4 |#############
  2317606.9 |
  2330441.4 |
  2343275.9 |#############
  2356110.4 |
  2368944.9 |
  2381779.4 |
  2394613.9 |
  2407448.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=5161.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=306.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=4901.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
