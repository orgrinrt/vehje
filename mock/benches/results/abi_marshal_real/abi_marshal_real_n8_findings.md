# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 10232% faster than the next best (abi_marshal_real_aos)

abi_marshal_real_marshal_null (20.87 us) leads abi_marshal_real_aos (2.16 ms) by 10232%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.14 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 104.4x slower than the field

abi_marshal_real_soa_transposed (2.18 ms) is 104.4x the fastest (20.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_real_soa_transposed shows alternating (throttle bounce) (autocorr -0.72)

abi_marshal_real_soa_transposed's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} (10232% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} with a 10232% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 104.4x the fastest

Fastest abi_marshal_real_marshal_null (20.87 us) to slowest abi_marshal_real_soa_transposed (2.18 ms): 104.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 20871.7 ns median (-99.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 104.38x (fastest 20871.7 ns, slowest 2178570.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2153350ns | 2158898ns | 2127282ns | 2150645ns | 2170440ns | base |
| abi_marshal_real_marshal_null | 23033ns | 23150ns | 22427ns | 22973ns | 23427ns | -98.93% |
| abi_marshal_real_soa_native | 2160936ns | 2162149ns | 2147276ns | 2158677ns | 2171154ns | +0.35% |
| abi_marshal_real_soa_transposed | 2181495ns | 2181050ns | 2164081ns | 2178818ns | 2194216ns | +1.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2150845ns | 2124797ns | 2167927ns | base | 0.000 |
| abi_marshal_real_marshal_null | 20752ns | 20238ns | 21063ns | -99.04% | 0.000 |
| abi_marshal_real_soa_native | 2158389ns | 2144780ns | 2168643ns | +0.35% | 0.000 |
| abi_marshal_real_soa_transposed | 2179015ns | 2161660ns | 2191716ns | +1.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 39412.5 | 2151030.0 | 2150844.6 | n/a |
| abi_marshal_real_marshal_null | 27949.8 | 20971.6 | 20751.5 | n/a |
| abi_marshal_real_soa_native | 42764.5 | 2159160.9 | 2158388.6 | n/a |
| abi_marshal_real_soa_transposed | 38929.2 | 2177621.1 | 2179015.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.9% |
| abi_marshal_real_marshal_null | 0.000 | 97.0% |
| abi_marshal_real_soa_native | 0.000 | 0.9% |
| abi_marshal_real_soa_transposed | 0.000 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2153350ns | 2153350ns | base |
| abi_marshal_real_marshal_null | 23033ns | 23033ns | -98.93% |
| abi_marshal_real_soa_native | 2160936ns | 2160936ns | +0.35% |
| abi_marshal_real_soa_transposed | 2181495ns | 2181495ns | +1.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2156454ns | base | --- | [2128153, 2167927] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 20872ns | -2135576.9ns (-99.0%) | [-2146869, -2107833]ns | [20320, 21063] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2159583ns | no significant difference | [-9514, +31430]ns | [2146940, 2168643] | no | 1.0000 | 1.0000 | 0 |
| abi_marshal_real_soa_transposed | 2178570ns | +25266.0ns (+1.2%) | [+20639, +38606]ns | [2166759, 2191716] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2124797ns | -99.0% | +1.7% | +1.7% |
| 2 | 2170945ns | -99.0% | -0.1% | +1.1% |
| 3 | 2131508ns | -99.1% | +1.3% | +1.9% |
| 4 | 2164909ns | -99.0% | +0.2% | +1.1% |
| 5 | 2154902ns | -99.0% | -0.5% | +0.8% |
| 6 | 2158006ns | -99.0% | -0.4% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.638 | HIGH- (thermal bounce) |
| abi_marshal_real_marshal_null | -0.644 | HIGH- (thermal bounce) |
| abi_marshal_real_soa_native | 0.027 | ok |
| abi_marshal_real_soa_transposed | -0.715 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 3/6, lost 3/6
- **abi_marshal_real_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6495507.4ns | 2150844.6ns | 302.0% | HIGH |
| abi_marshal_real_marshal_null | 170579.6ns | 20751.5ns | 822.0% | HIGH |
| abi_marshal_real_soa_native | 6525358.2ns | 2158388.6ns | 302.3% | HIGH |
| abi_marshal_real_soa_transposed | 6575836.8ns | 2179015.1ns | 301.8% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2124797.1-2167926.9 ns)
  2124797.1 |########################################
  2126953.6 |
  2129110.1 |
  2131266.6 |########################################
  2133423.1 |
  2135579.6 |
  2137736.0 |
  2139892.5 |
  2142049.0 |
  2144205.5 |
  2146362.0 |
  2148518.5 |
  2150675.0 |
  2152831.5 |########################################
  2154988.0 |
  2157144.5 |########################################
  2159300.9 |
  2161457.4 |
  2163613.9 |########################################
  2165770.4 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 20238.3-21063.1 ns)
  20238.3 |########################################
  20279.5 |
  20320.8 |
  20362.0 |########################################
  20403.3 |
  20444.5 |
  20485.7 |
  20527.0 |
  20568.2 |
  20609.5 |
  20650.7 |
  20691.9 |########################################
  20733.2 |
  20774.4 |
  20815.7 |
  20856.9 |
  20898.1 |
  20939.4 |
  20980.6 |########################################
  21021.9 |########################################
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2144779.6-2168643.4 ns)
  2144779.6 |########################################
  2145972.8 |
  2147166.0 |
  2148359.2 |########################################
  2149552.4 |
  2150745.5 |
  2151938.7 |
  2153131.9 |
  2154325.1 |
  2155518.3 |
  2156711.5 |
  2157904.7 |########################################
  2159097.9 |########################################
  2160291.0 |
  2161484.2 |
  2162677.4 |
  2163870.6 |
  2165063.8 |
  2166257.0 |
  2167450.2 |########################################
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2161660.4-2191716.0 ns)
  2161660.4 |########################################
  2163163.2 |
  2164666.0 |
  2166168.7 |
  2167671.5 |
  2169174.3 |
  2170677.1 |########################################
  2172179.9 |########################################
  2173682.6 |
  2175185.4 |
  2176688.2 |
  2178191.0 |
  2179693.8 |
  2181196.5 |
  2182699.3 |########################################
  2184202.1 |
  2185704.9 |
  2187207.7 |
  2188710.4 |########################################
  2190213.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=818.5% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)
