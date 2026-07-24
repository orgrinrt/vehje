# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_scalar is an outlier: 948.6x slower than the field

abi_cross_cold_scatter_warm_scalar (2.18 ms) is 948.6x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} (89827% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} with a 89827% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 948.6x the fastest

Fastest abi_cross_cold_scatter_warm_null (2.29 us) to slowest abi_cross_cold_scatter_warm_scalar (2.18 ms): 948.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 2294.8 ns median
- 3 variants significantly slower than baseline
- Spread: 948.62x (fastest 2294.8 ns, slowest 2176883.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 4784ns | 4783ns | 4699ns | 4766ns | 4854ns | +2.70% |
| abi_cross_cold_scatter_cold_scalar | 2202069ns | 2173325ns | 2168119ns | 2172265ns | 2263751ns | +47170.43% |
| abi_cross_cold_scatter_warm_null | 4658ns | 4665ns | 4446ns | 4616ns | 4828ns | base |
| abi_cross_cold_scatter_warm_scalar | 2222343ns | 2180445ns | 2169260ns | 2177794ns | 2315708ns | +47605.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 2422ns | 2360ns | 2480ns | +5.41% | 0.013 |
| abi_cross_cold_scatter_cold_scalar | 2198432ns | 2164655ns | 2259660ns | +95597.05% | 0.000 |
| abi_cross_cold_scatter_warm_null | 2297ns | 2228ns | 2356ns | base | 0.014 |
| abi_cross_cold_scatter_warm_scalar | 2218673ns | 2166085ns | 2311603ns | +96478.13% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 33792.9 | 2604.0 | 2421.5 | n/a |
| abi_cross_cold_scatter_cold_scalar | 80470.5 | 2192169.0 | 2198432.5 | n/a |
| abi_cross_cold_scatter_warm_null | 28711.0 | 2425.9 | 2297.3 | n/a |
| abi_cross_cold_scatter_warm_scalar | 70804.8 | 2225281.9 | 2218673.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.013 | 92.3% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_scatter_warm_null | 0.014 | 97.1% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 4784ns | 4784ns | +2.70% |
| abi_cross_cold_scatter_cold_scalar | 2202069ns | 2202069ns | +47170.43% |
| abi_cross_cold_scatter_warm_null | 4658ns | 4658ns | base |
| abi_cross_cold_scatter_warm_scalar | 2222343ns | 2222343ns | +47605.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 2295ns | base | --- | [2241, 2356] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 2413ns | +114.4ns (+5.0%) | [+53, +205]ns | [2371, 2480] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2169905ns | +2167613.2ns (+94457.6%) | [+2163489, +2257304]ns | [2165733, 2259660] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2176883ns | +2174627.0ns (+94763.2%) | [+2165254, +2309247]ns | [2167534, 2311603] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 2228ns | +11.8% | +97140.5% | +97238.0% |
| 2 | 2253ns | +6.6% | +96226.6% | +96644.0% |
| 3 | 2330ns | +1.3% | +92986.2% | +92849.1% |
| 4 | 2259ns | +5.5% | +95715.1% | +96121.4% |
| 5 | 2365ns | +4.4% | +94826.9% | +102998.6% |
| 6 | 2347ns | +3.3% | +96782.0% | +92972.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.040 | ok |
| abi_cross_cold_scatter_cold_scalar | 0.397 | moderate+ |
| abi_cross_cold_scatter_warm_null | 0.068 | ok |
| abi_cross_cold_scatter_warm_scalar | -0.190 | ok |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 123832.3ns | 2421.5ns | 5113.8% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6661617.7ns | 2198432.5ns | 303.0% | HIGH |
| abi_cross_cold_scatter_warm_null | 117546.1ns | 2297.3ns | 5116.7% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6732761.9ns | 2218673.3ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 2359.6-2480.2 ns)
   2359.6 |########################################
   2365.6 |
   2371.7 |
   2377.7 |########################################
   2383.7 |
   2389.8 |
   2395.8 |########################################
   2401.8 |
   2407.8 |
   2413.9 |
   2419.9 |########################################
   2425.9 |
   2432.0 |
   2438.0 |
   2444.0 |
   2450.0 |
   2456.1 |
   2462.1 |
   2468.1 |########################################
   2474.2 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2164654.6-2259659.8 ns)
  2164654.6 |########################################
  2169404.9 |#############
  2174155.1 |
  2178905.4 |
  2183655.6 |
  2188405.9 |
  2193156.1 |
  2197906.4 |
  2202656.7 |
  2207406.9 |
  2212157.2 |
  2216907.4 |
  2221657.7 |
  2226407.9 |
  2231158.2 |
  2235908.5 |
  2240658.7 |#############
  2245409.0 |
  2250159.2 |
  2254909.5 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 2228.3-2356.2 ns)
   2228.3 |########################################
   2234.7 |
   2241.1 |
   2247.5 |########################################
   2253.9 |########################################
   2260.3 |
   2266.7 |
   2273.1 |
   2279.5 |
   2285.9 |
   2292.3 |
   2298.7 |
   2305.1 |
   2311.5 |
   2317.9 |
   2324.3 |########################################
   2330.7 |
   2337.1 |
   2343.5 |########################################
   2349.9 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2166085.4-2311603.1 ns)
  2166085.4 |########################################
  2173361.3 |########################################
  2180637.2 |####################
  2187913.1 |
  2195188.9 |
  2202464.8 |
  2209740.7 |
  2217016.6 |
  2224292.5 |
  2231568.4 |
  2238844.2 |
  2246120.1 |
  2253396.0 |
  2260671.9 |
  2267947.8 |
  2275223.7 |
  2282499.6 |
  2289775.4 |
  2297051.3 |
  2304327.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=5111.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=5114.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
