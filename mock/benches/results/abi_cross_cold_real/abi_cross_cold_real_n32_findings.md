# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_cold_scalar is an outlier: 959.2x slower than the field

abi_cross_cold_real_cold_scalar (2.15 ms) is 959.2x the fastest (2.24 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_real_cold_null shows alternating (throttle bounce) (autocorr -0.54)

abi_cross_cold_real_cold_null's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (2.24 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (88390% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 88390% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 959.2x the fastest

Fastest abi_cross_cold_real_warm_null (2.24 us) to slowest abi_cross_cold_real_cold_scalar (2.15 ms): 959.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 2244.6 ns median
- 3 variants significantly slower than baseline
- Spread: 959.20x (fastest 2244.6 ns, slowest 2152966.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4743ns | 4763ns | 4570ns | 4737ns | 4838ns | +3.81% |
| abi_cross_cold_real_cold_scalar | 2168909ns | 2155951ns | 2145166ns | 2153944ns | 2203227ns | +47374.95% |
| abi_cross_cold_real_warm_null | 4569ns | 4536ns | 4452ns | 4517ns | 4704ns | base |
| abi_cross_cold_real_warm_scalar | 2220909ns | 2155806ns | 2147702ns | 2155328ns | 2355885ns | +48513.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2421ns | 2327ns | 2468ns | +6.31% | 0.013 |
| abi_cross_cold_real_cold_scalar | 2165832ns | 2142400ns | 2199820ns | +94992.04% | 0.000 |
| abi_cross_cold_real_warm_null | 2278ns | 2210ns | 2364ns | base | 0.014 |
| abi_cross_cold_real_warm_scalar | 2217801ns | 2144902ns | 2352294ns | +97273.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 32871.6 | 2498.8 | 2421.4 | n/a |
| abi_cross_cold_real_cold_scalar | 61322.9 | 2165662.8 | 2165832.2 | n/a |
| abi_cross_cold_real_warm_null | 27788.0 | 2379.2 | 2277.6 | n/a |
| abi_cross_cold_real_warm_scalar | 59922.4 | 2266829.6 | 2217800.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.013 | 90.8% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.014 | 98.5% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 4743ns | 4743ns | +3.81% |
| abi_cross_cold_real_cold_scalar | 2168909ns | 2168909ns | +47374.95% |
| abi_cross_cold_real_warm_null | 4569ns | 4569ns | base |
| abi_cross_cold_real_warm_scalar | 2220909ns | 2220909ns | +48513.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2245ns | base | --- | [2224, 2364] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2433ns | +146.5ns (+6.5%) | [+81, +204]ns | [2364, 2468] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2152966ns | +2150652.5ns (+95816.6%) | [+2142473, +2197538]ns | [2144711, 2199820] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2152913ns | +2150605.6ns (+95814.6%) | [+2145951, +2350013]ns | [2148196, 2352294] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2352ns | +5.4% | +91914.5% | +108211.9% |
| 2 | 2210ns | +8.6% | +101031.1% | +97481.5% |
| 3 | 2238ns | +9.7% | +95632.6% | +96153.3% |
| 4 | 2238ns | +4.0% | +95822.0% | +95727.3% |
| 5 | 2376ns | +3.1% | +90532.5% | +90455.2% |
| 6 | 2251ns | +7.4% | +95524.8% | +95487.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.542 | HIGH- (thermal bounce) |
| abi_cross_cold_real_cold_scalar | -0.144 | ok |
| abi_cross_cold_real_warm_null | -0.309 | moderate- |
| abi_cross_cold_real_warm_scalar | -0.019 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 122865.4ns | 2421.4ns | 5074.1% | HIGH |
| abi_cross_cold_real_cold_scalar | 6568146.3ns | 2165832.2ns | 303.3% | HIGH |
| abi_cross_cold_real_warm_null | 117088.3ns | 2277.6ns | 5140.8% | HIGH |
| abi_cross_cold_real_warm_scalar | 6764355.5ns | 2217800.9ns | 305.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2327.1-2467.7 ns)
   2327.1 |########################################
   2334.1 |
   2341.2 |
   2348.2 |
   2355.2 |
   2362.2 |
   2369.3 |
   2376.3 |
   2383.3 |
   2390.4 |
   2397.4 |########################################
   2404.4 |
   2411.5 |########################################
   2418.5 |
   2425.5 |
   2432.5 |
   2439.6 |
   2446.6 |########################################
   2453.6 |########################################
   2460.7 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2142400.4-2199819.6 ns)
  2142400.4 |####################
  2145271.4 |####################
  2148142.3 |
  2151013.3 |########################################
  2153884.2 |
  2156755.2 |
  2159626.2 |
  2162497.1 |####################
  2165368.1 |
  2168239.0 |
  2171110.0 |
  2173981.0 |
  2176851.9 |
  2179722.9 |
  2182593.8 |
  2185464.8 |
  2188335.8 |
  2191206.7 |
  2194077.7 |
  2196948.6 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2210.0-2364.3 ns)
   2210.0 |####################
   2217.7 |
   2225.4 |
   2233.2 |########################################
   2240.9 |
   2248.6 |####################
   2256.3 |
   2264.0 |
   2271.7 |
   2279.5 |
   2287.2 |
   2294.9 |
   2302.6 |
   2310.3 |
   2318.0 |
   2325.8 |
   2333.5 |
   2341.2 |
   2348.9 |####################
   2356.6 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2144902.5-2352294.4 ns)
  2144902.5 |########################################
  2155272.1 |##########
  2165641.7 |
  2176011.3 |
  2186380.9 |
  2196750.5 |
  2207120.1 |
  2217489.7 |
  2227859.3 |
  2238228.9 |
  2248598.5 |
  2258968.0 |
  2269337.6 |
  2279707.2 |
  2290076.8 |
  2300446.4 |
  2310816.0 |
  2321185.6 |
  2331555.2 |
  2341924.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=5057.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=5208.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=302.9% of algo (FFI overhead may distort results)
