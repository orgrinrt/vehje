# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 38839% faster than the next best (abi_boundary_w_tight_soa_per_w)

abi_boundary_w_tight_null_entry (2.48 us) leads abi_boundary_w_tight_soa_per_w (967.08 us) by 38839%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.05 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 826.1x slower than the field

abi_boundary_w_tight_scalar_anchor (2.05 ms) is 826.1x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_zig_runtime_w shows alternating (throttle bounce) (autocorr -0.87)

abi_boundary_w_tight_zig_runtime_w's per-pass series has lag-1 autocorrelation -0.87, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_anchor} (38839% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_anchor} with a 38839% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 826.1x the fastest

Fastest abi_boundary_w_tight_null_entry (2.48 us) to slowest abi_boundary_w_tight_scalar_anchor (2.05 ms): 826.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 2483.6 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 826.06x (fastest 2483.6 ns, slowest 2051573.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4753ns | 4764ns | 4680ns | 4738ns | 4810ns | -99.77% |
| abi_boundary_w_tight_scalar_anchor | 2057284ns | 2054526ns | 2043781ns | 2051791ns | 2072274ns | +0.27% |
| abi_boundary_w_tight_scalar_dispatch | 2045126ns | 2045025ns | 2034108ns | 2042127ns | 2055135ns | -0.33% |
| abi_boundary_w_tight_scalar_per_w | 2046722ns | 2046360ns | 2038213ns | 2044154ns | 2054828ns | -0.25% |
| abi_boundary_w_tight_scalar_runtime_w | 2051840ns | 2051299ns | 2043198ns | 2050090ns | 2058787ns | base |
| abi_boundary_w_tight_soa_dispatch | 978000ns | 979112ns | 970460ns | 976341ns | 984259ns | -52.34% |
| abi_boundary_w_tight_soa_per_w | 973921ns | 969970ns | 960580ns | 968625ns | 988536ns | -52.53% |
| abi_boundary_w_tight_soa_runtime_w | 975526ns | 975046ns | 962883ns | 972057ns | 987052ns | -52.46% |
| abi_boundary_w_tight_zig_runtime_w | 2023900ns | 2025601ns | 2014253ns | 2022714ns | 2030502ns | -1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 2484ns | 2466ns | 2499ns | -99.88% | 0.026 |
| abi_boundary_w_tight_scalar_anchor | 2054075ns | 2040734ns | 2068659ns | +0.28% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2041848ns | 2031108ns | 2051478ns | -0.32% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2043458ns | 2035223ns | 2051241ns | -0.24% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2048349ns | 2039922ns | 2055329ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 975108ns | 967553ns | 981296ns | -52.40% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 970995ns | 957949ns | 985352ns | -52.60% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 972671ns | 960350ns | 983934ns | -52.51% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2020347ns | 2010712ns | 2026984ns | -1.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 28017.6 | 2749.7 | 2484.1 | n/a |
| abi_boundary_w_tight_scalar_anchor | 60441.2 | 2052498.7 | 2054075.1 | 1 |
| abi_boundary_w_tight_scalar_dispatch | 63140.2 | 2039869.6 | 2041847.7 | n/a |
| abi_boundary_w_tight_scalar_per_w | 65223.2 | 2045635.4 | 2043457.6 | n/a |
| abi_boundary_w_tight_scalar_runtime_w | 67250.4 | 2044522.5 | 2048349.4 | n/a |
| abi_boundary_w_tight_soa_dispatch | 48430.3 | 974171.4 | 975108.3 | n/a |
| abi_boundary_w_tight_soa_per_w | 50138.1 | 970627.5 | 970994.9 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 45211.8 | 972536.6 | 972671.0 | n/a |
| abi_boundary_w_tight_zig_runtime_w | 232952.7 | 2019435.9 | 2020347.1 | 6 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.026 | 99.3% |
| abi_boundary_w_tight_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_tight_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_tight_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_tight_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_tight_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4753ns | 4753ns | -99.77% |
| abi_boundary_w_tight_scalar_anchor | 2057284ns | 2057284ns | +0.27% |
| abi_boundary_w_tight_scalar_dispatch | 2045126ns | 2045126ns | -0.33% |
| abi_boundary_w_tight_scalar_per_w | 2046722ns | 2046722ns | -0.25% |
| abi_boundary_w_tight_scalar_runtime_w | 2051840ns | 2051840ns | base |
| abi_boundary_w_tight_soa_dispatch | 978000ns | 978000ns | -52.34% |
| abi_boundary_w_tight_soa_per_w | 973921ns | 973921ns | -52.53% |
| abi_boundary_w_tight_soa_runtime_w | 975526ns | 975526ns | -52.46% |
| abi_boundary_w_tight_zig_runtime_w | 2023900ns | 2023900ns | -1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2047706ns | base | --- | [2042013, 2055329] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 2484ns | -2045222.9ns (-99.9%) | [-2052843, -2039530]ns | [2469, 2499] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2051573ns | no significant difference | [-5713, +17098]ns | [2041993, 2068659] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2041811ns | -5034.8ns (-0.2%) | [-13645, -825]ns | [2032254, 2051478] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2043134ns | no significant difference | [-12195, +5796]ns | [2035998, 2051241] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_dispatch | 976271ns | -1074032.5ns (-52.5%) | [-1078607, -1067084]ns | [967758, 981296] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_per_w | 967079ns | -1081916.2ns (-52.8%) | [-1085891, -1064256]ns | [960554, 985352] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 972152ns | -1072122.7ns (-52.4%) | [-1093402, -1061510]ns | [961927, 983934] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2022096ns | -29961.2ns (-1.5%) | [-31804, -22242]ns | [2011961, 2026984] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2048627ns | -99.9% | -0.4% | -0.1% | -0.6% | -52.8% | -51.5% | -52.5% | -1.1% |
| 2 | 2039922ns | -99.9% | +0.6% | -0.3% | -0.2% | -52.1% | -53.0% | -52.4% | -1.4% |
| 3 | 2059018ns | -99.9% | +0.9% | -0.2% | -0.6% | -52.2% | -52.5% | -53.4% | -1.5% |
| 4 | 2044103ns | -99.9% | +0.8% | -0.6% | +0.2% | -52.7% | -52.5% | -51.6% | -1.5% |
| 5 | 2046786ns | -99.9% | -0.2% | -0.0% | +0.4% | -52.4% | -52.9% | -52.2% | -1.0% |
| 6 | 2051640ns | -99.9% | -0.1% | -0.7% | -0.6% | -52.3% | -53.1% | -53.0% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.153 | ok |
| abi_boundary_w_tight_scalar_anchor | 0.119 | ok |
| abi_boundary_w_tight_scalar_dispatch | -0.848 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_scalar_per_w | 0.179 | ok |
| abi_boundary_w_tight_scalar_runtime_w | -0.629 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_soa_dispatch | -0.298 | moderate- |
| abi_boundary_w_tight_soa_per_w | -0.378 | moderate- |
| abi_boundary_w_tight_soa_runtime_w | -0.238 | moderate- |
| abi_boundary_w_tight_zig_runtime_w | -0.865 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 2/6, lost 3/6
- **abi_boundary_w_tight_scalar_dispatch**: won 4/6, lost 0/6
- **abi_boundary_w_tight_scalar_per_w**: won 4/6, lost 2/6
- **abi_boundary_w_tight_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 113794.6ns | 2484.1ns | 4580.9% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6236554.6ns | 2054075.1ns | 303.6% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6191203.3ns | 2041847.7ns | 303.2% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6196191.0ns | 2043457.6ns | 303.2% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6203085.7ns | 2048349.4ns | 302.8% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 2973268.0ns | 975108.3ns | 304.9% | HIGH |
| abi_boundary_w_tight_soa_per_w | 2963379.8ns | 970994.9ns | 305.2% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 2963693.6ns | 972671.0ns | 304.7% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6380969.0ns | 2020347.1ns | 315.8% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 2466.2-2499.4 ns)
   2466.2 |########################################
   2467.9 |
   2469.5 |
   2471.2 |########################################
   2472.8 |
   2474.5 |
   2476.1 |
   2477.8 |########################################
   2479.5 |
   2481.1 |
   2482.8 |
   2484.4 |
   2486.1 |
   2487.7 |########################################
   2489.4 |
   2491.1 |
   2492.7 |########################################
   2494.4 |
   2496.0 |
   2497.7 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2040733.8-2068658.5 ns)
  2040733.8 |########################################
  2042130.0 |########################################
  2043526.3 |
  2044922.5 |
  2046318.8 |
  2047715.0 |
  2049111.2 |########################################
  2050507.5 |
  2051903.7 |########################################
  2053299.9 |
  2054696.2 |
  2056092.4 |
  2057488.6 |
  2058884.9 |########################################
  2060281.1 |
  2061677.4 |
  2063073.6 |
  2064469.8 |
  2065866.1 |
  2067262.3 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2031107.9-2051478.4 ns)
  2031107.9 |########################################
  2032126.4 |
  2033144.9 |########################################
  2034163.5 |
  2035182.0 |
  2036200.5 |
  2037219.0 |########################################
  2038237.6 |
  2039256.1 |
  2040274.6 |
  2041293.1 |
  2042311.6 |
  2043330.2 |
  2044348.7 |
  2045367.2 |########################################
  2046385.7 |
  2047404.3 |########################################
  2048422.8 |
  2049441.3 |
  2050459.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2035223.3-2051240.6 ns)
  2035223.3 |########################################
  2036024.2 |########################################
  2036825.0 |
  2037625.9 |
  2038426.8 |########################################
  2039227.6 |
  2040028.5 |
  2040829.4 |
  2041630.2 |
  2042431.1 |
  2043232.0 |
  2044032.8 |
  2044833.7 |
  2045634.5 |
  2046435.4 |########################################
  2047236.3 |
  2048037.1 |########################################
  2048838.0 |
  2049638.9 |
  2050439.7 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2039922.5-2055328.9 ns)
  2039922.5 |########################################
  2040692.8 |
  2041463.1 |
  2042233.5 |
  2043003.8 |
  2043774.1 |########################################
  2044544.4 |
  2045314.8 |
  2046085.1 |########################################
  2046855.4 |
  2047625.7 |
  2048396.0 |########################################
  2049166.4 |
  2049936.7 |
  2050707.0 |
  2051477.3 |########################################
  2052247.7 |
  2053018.0 |
  2053788.3 |
  2054558.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 967552.9-981296.4 ns)
  967552.9 |########################################
  968240.1 |
  968927.3 |
  969614.4 |
  970301.6 |
  970988.8 |
  971676.0 |
  972363.1 |
  973050.3 |
  973737.5 |
  974424.7 |####################
  975111.9 |
  975799.0 |
  976486.2 |
  977173.4 |####################
  977860.6 |####################
  978547.7 |
  979234.9 |
  979922.1 |
  980609.3 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 957949.2-985351.9 ns)
  957949.2 |########################################
  959319.3 |
  960689.5 |
  962059.6 |########################################
  963429.7 |########################################
  964799.9 |
  966170.0 |
  967540.1 |
  968910.3 |
  970280.4 |########################################
  971650.6 |
  973020.7 |
  974390.8 |
  975761.0 |
  977131.1 |########################################
  978501.2 |
  979871.4 |
  981241.5 |
  982611.6 |
  983981.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 960350.4-983934.2 ns)
  960350.4 |########################################
  961529.6 |
  962708.8 |########################################
  963888.0 |
  965067.2 |
  966246.3 |
  967425.5 |
  968604.7 |
  969783.9 |########################################
  970963.1 |
  972142.3 |
  973321.5 |########################################
  974500.7 |
  975679.9 |
  976859.1 |
  978038.2 |########################################
  979217.4 |
  980396.6 |
  981575.8 |
  982755.0 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2010711.7-2026983.7 ns)
  2010711.7 |########################################
  2011525.3 |
  2012338.9 |
  2013152.5 |########################################
  2013966.1 |
  2014779.7 |
  2015593.3 |
  2016406.9 |
  2017220.5 |
  2018034.1 |
  2018847.7 |########################################
  2019661.3 |
  2020474.9 |
  2021288.5 |
  2022102.1 |
  2022915.7 |
  2023729.3 |
  2024542.9 |########################################
  2025356.5 |########################################
  2026170.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=4577.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=315.5% of algo (FFI overhead may distort results)
